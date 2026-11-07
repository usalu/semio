# 🔀️ Bounded per-turn document-archive merge (prepared, NOT applied)

Status: v2 DESIGN below is authoritative (all-or-nothing load, bounded follow). v1 code files (`cursor.rs`, `archive-merge.rs`, `laws.rs`, `apply.py`) still implement the superseded replay-dropping variant and are NOT to be landed as is; `follow-children-step.rs` is the new v2 snippet. v1 status: patch written and anchor-checked (`python3 apply.py dry` -> 8 files modified, 3 added, all anchors match once). UNCOMPILED. Land after the activation freeze on the coordinator's word.

## Problem
`merge_document_archive` runs `store.merge_persisted_history(&pack, history).await` inside one host poll: aussuchen measured 34,693 CPU units in a single poll. That call is four unbounded stages: (1) `parse_decoded_document_spr` decodes every edit and then replay-validates the whole foreign document (`settle_parsed_envelope`) only to throw the snapshot away, (2) `event_log()` clones every envelope of the live store to build a `known` set, (3) a full sort of the transitions, (4) one `ingest_remote` per unknown event, each an atomic merge that may replay. `follow_derivable_children` follows in the same poll.

## Design
A resumable merge cursor in the VCS store (`PersistedMergeCursor`, `ArtifactStore::persisted_merge_open/persisted_merge_step`). The old one-call `merge_persisted_history` stays untouched as the independent oracle for the laws and for non-poll callers (space members, tests).

Phases (a cursor visits them in order; `PersistedMergePhase`):
1. Preflight / PreflightTransitions: decode each persisted edit once with the borrowed decoder `decode_history_edit_record` (same logic as `parse_decoded_document_spr`, kept next to it), build its envelopes, record their ids in `offered`, retire the scratch edit. This keeps "a malformed pair is refused before anything is taken" (set `preflightDecode:false` to skip when decode errors may surface mid-ingest).
2. Census / CensusTransitions: page over the live ledger collecting `known` ids without building envelopes (`event_log_edit_ids_page`, `event_log_transition_ids_page`); `ahead` is counted on the fly (`known \ offered`) so no second pass.
3. Order: resumable heapsort of the transition indices by `(hlt cmp_key, id)`, one sift per unit.
4. IngestEdits / IngestTransitions: re-decode edit i (so the cursor holds no envelopes), ingest the unknown ones through the unchanged atomic `ingest_remote`, resume mid-edit via `(index, offset)`.
5. Complete: `outcome()` = `PairMerge{merged, ahead}`.

Cursor state is ephemeral and local-only; it owns the decoded `HistoryLog` (already `RetireOwned`) plus id sets, `Vec<u64>` keys and `Vec<u32>` order. Nothing is persisted; the store's atomic ingests are the only durable effect. Schema-first: the price list (`persisted-merge-pricing.json`) and the progress record (`persisted-merge-progress.schema.json`) are authored JSON; a law keeps `PERSISTED_MERGE_PRICING` equal to the pricing file.

## Accounting and bounds
`persisted_merge_step(cursor, budget) -> RetainedWorkOutcome<PersistedMergeProgress, VcsError>`: every unit is priced up front (decode: base + per op; census: per id; order: per comparison; ingest: base + per dependency). A step starts a unit only if `work + price <= budget`, except the first unit of a step, so every step makes progress and overspends by at most one indivisible unit. `work` is the units really performed and the host consumes it with `cx.consume_fuel(turn.work)`; the budget is `cx.fuel_remaining()`. Prices are tunable data; measured units from aussuchen should calibrate `ingestBase` (the dominant, indivisible cost: one `ingest_remote`).

## Resumability, cancellation, retirement
- Resume: the host calls the step once per poll while `awaits_merge()`; progress (`completed/total` of the current phase) is mirrored into the load's status. Between steps the store may change: the document identity is re-checked per step, ingests are idempotent (`known` plus the DAG's duplicate handling), and a law covers a local edit between steps.
- Cancel: cancellation takes effect between steps (a step is bounded). `ActiveDocumentArchiveLoad` owns the cursor (`merge_cursor`); the existing terminal ladder retires it through `close_step` exactly like `history_custody` (`admit_owned_retirement` into `retained`), `inputs_are_empty`/Drop/`retirement_demands` include it. Events ingested before the cancel stay (like any remote batch); a fresh cursor over the same pair completes the rest (law).
- Failure: a refused ingest leaves the cursor at the refused unit and the load requests a Fault terminal; the retirement path above drains the cursor.

## Semantic deltas to decide before landing
1. The whole-foreign-document replay validation (`settle_parsed_envelope`) is no longer run before merging. Per-event validation happens at ingest; a pair that decodes but would fail the replay now surfaces as an ingest refusal after the prefix was taken (the documented behaviour of any remote batch). If the old all-or-nothing replay refusal must stay, it needs a cursorable replay phase (not available today).
2. `ahead` is the census at the time the Census phase ran (events the store gained later are not counted).
3. Status `completed/total` is per phase (indeterminate across phase changes).
4. `follow_derivable_children` is still one unbounded call in the finishing poll (isolated from the merge cost); a per-child step API is the follow-up.
5. The census walks the ledger with `skip(from)`: O(n) per page; a `ledger.iter_from(key)` would make it linear.

## Laws (viewer-head, region PairMergeCursorLaws)
chunked == one-call for budgets 1,2,5,13,89,unbounded (outcome, event log, live==replay, per-step work bounded, bigger budget never needs more steps); cancel mid-merge keeps a prefix and a fresh cursor completes; another document refused at open and the history returned; local edit between steps; price constant == authored file.

## Land checklist (nothing here could be compiled)
1. `python3 apply.py dry` then `write`; copy nothing else.
2. Verify at compile time: `semio_os_spr::HistoryEdit` is exported (decoder), `HistoryEdits::get(i)` exists (checked: PagedList::get), `MutationId`/`HashSet` are in scope of the lifted store module (used by merge_persisted_history), `HistoryLane`/`stamp_edit_semantics`/`mutation_meta_from_history_op_meta` are in scope of the text composition file (used by parse_decoded_document_spr), `HistoryOpMeta: Clone`, `StepContext::{fuel_remaining,consume_fuel}`.
3. `repo cargo ... check --target wasm32-wasip2` for the store engine, the plugin instance crate and archive-load crate; then `repo cargo test -p semio-os-store-tests-<viewer-head bin> -- pair_merge cursor`.
4. Runtime lane (ab5d53c73baeac213): confirm `cx.fuel_remaining()` is the per-turn budget in the same units as the 34,693 measurement and that the poll caller tolerates a merge spanning many polls (`awaits_merge` stays true until Complete).


---

# v2 (coordinator requirement: all-or-nothing load, bounded child follow)

## A. Whole-document validation stays; the load stays all-or-nothing
Invariant: no event of a foreign pair reaches the live store unless the whole foreign document replayed clean. Mechanism (no new validation code, only reuse):
1. The ladder's existing `HydrateParent` phase (`RetainedPersistedDocumentHydration`, `Target::Envelope`) is the cursorable validation: it already runs `project_envelope_history -> validate_durable_history -> replay_loaded_history` in bounded, cancellable steps and yields a validated `ArtifactEnvelope` of the foreign pair. A merge load therefore enters `HydrateParent` (instead of `MergeParent` straight from decode) with the merge pair as its source and stores the result in `merge_foreign: Option<ArtifactEnvelope>`.
2. Ingest runs only after `HydrateParent` succeeded. The cursor is built over `merge_foreign` (its edit/transition envelopes already exist, so the v1 Preflight/PreflightTransitions/Order phases and every re-decode disappear: the envelopes are ordered by the validated replay). Remaining phases: Census, CensusTransitions, IngestEdits, IngestTransitions, Complete.
3. Failure before ingest (decode, projection, validation, replay) leaves the live store untouched. The only partial state possible is a refusal of a single ingest after validation passed; since the foreign document replayed clean and `ingest_remote` is the same atomic gate the old one-call merge used, that case equals the old behaviour and stays a Fault terminal. A law pins this: a pair whose last edit breaks replay yields `merged == 0` and an unchanged event log across the whole stepped load, for budgets 1, 13 and unbounded.
4. Staging alternative (not chosen): a scratch ledger committed atomically needs a second live store per load (memory x2 and a commit primitive the store does not have); hydration-as-validation reuses what exists.

## B. Retirement and ownership
`merge_foreign` (an `ArtifactEnvelope`) is owned by `ActiveDocumentArchiveLoad`, retired by the existing terminal ladder through `retire_document_archive_envelope` (the same call HydrateParent's result uses), listed in `retirement_demands`, `inputs_are_empty` and the Drop guard. The cursor itself (id sets) is plain heap and retires with the load's `close_step` custody branch as in v1. Cancel: the ladder cancels between steps; events already ingested stay (as any remote batch), the foreign envelope and cursor are retired by the close path, a fresh load completes the rest (law kept from v1).

## C. Budget rule (runtime)
Each merge step takes at most half of `cx.fuel_remaining()` (`allowance = (remaining/2).max(first unit)`), always at least one indivisible unit (forward progress), and the host charges `cx.consume_fuel(turn.work)` with the work really performed. The same half-rule is used by the child-follow step.

## D. follow_derivable_children is bounded per child (`follow-children-step.rs`)
`ChildFollowPlan` (field `child_follow_plan: Option<ChildFollowPlan>` on `ChildComposition`, ephemeral local-only, initialised `None` in `new`/`initialize_original_at`) drains one unit per child: stage 0 projects one declared child per unit (genesis pack derivation is priced `genesis_per_child`), stage 1 reclaims retirements once, stage 2 retires one child per unit, stage 3 opens one genesis child per unit (`open_child` is the indivisible unit, `open_per_child` = 64). The generation is committed only at the end of a clean, non-deferred pass; a generation change mid-pass restarts the plan (checked when the plan is created, and the finishing step re-compares). The finishing poll calls the step with the leftover fuel until `done`. The old name remains as a loop over the step for tests.
Debt, recorded: the projection is rebuilt (`ChildRestoreProjection::from_snapshot`) per stage-0 unit, O(children^2) per pass; a `projection.iter_from(at)` or caching the projection in the plan would make it linear. Until the plan exists in code the one-shot `follow_derivable_children` stays the next unbounded call in the finishing poll; its cost for demonstrator fixtures is measured by the law below.

## E. Laws added
- all-or-nothing (A.3) with a replay-breaking last edit, budgets 1, 13, unbounded.
- finishing poll under budget: for every demonstrator fixture the work of the poll that completes the merge (last ingest + follow step with half-fuel) is <= `INTERACTIVE_TURN_FUEL` (2_000_000); a fixture exceeding it names itself in the failure.
- follow step: chunked == one-shot for budgets 1, 5, unbounded (same children opened, same `followed_parent_generation`); a parent edit between steps restarts the plan.
- retirement: after cancel at every phase the load's retirement demands drain to zero (`merge_foreign`, cursor, plan).

## F. Still to do before landing (not done)
Wire v2 into apply.py (ladder anchors in `🪜️maintenance-ladder`, `document-archive-load` fields `merge_foreign`, terminal retirement, demands/inputs_are_empty/Drop), replace `cursor.rs` by the 5-phase cursor over `merge_foreign`, port the laws in `laws.rs`, wire the `child_follow_plan` field. All UNCOMPILED. Coordinate the freeze with the coordinator before any `write`.

## G. Runtime confirmation (ab5d53c73baeac213)
- Budget rule confirmed: per step at most `floor(cx.fuel_remaining()/2)`, charge exactly the reported work via `cx.consume_fuel(turn.work)`.
- Landing conditions: a refused validation or ingest must leave the load operation's terminal fault intact and retire the cursor, `merge_foreign` and the follow plan through the load's close path; the finishing-poll law must read `lane_defaults::budget_for(Interactive).fuel` (do not copy 2_000_000); bounded follow is required before landing.
- Hold until the coordinator opens a patch window.
