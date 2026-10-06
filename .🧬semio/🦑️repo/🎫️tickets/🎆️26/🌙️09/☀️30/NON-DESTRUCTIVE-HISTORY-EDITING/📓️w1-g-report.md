# 📓️ W1-G — Store core: report

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Executor W1-G. Scope: design §3 items 1–9, plus the coordinator
add-ons (TransactionRef slot, `commit_finished_replay`) and W2-E's two hub requests (check-in strictness, `observed`).

Aliases: `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`, `ST` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`.

## Outcome

- **Implemented and verified.** The store suite is 652/653 green. The one failure was already failing in the
  baseline (`retained_clone::tests::unit_generic_enum_and_recursive_records_remain_grant_bounded`, a stack
  overflow). There are no new failures, and all 14 new laws pass.
- The TS oracle (Ajv + fast-json-patch) passes 2/2.
- `cargo check` is clean for `semio-framework-os-kernel` (lib, and `--tests --features sync`),
  `semio-framework-plugin`, `semio-framework-os`, `semio-framework-os-flow`, `semio-framework-os-mcp`,
  `semio-s-artifact-puzzle-2d` and `semio-s-artifact-writer-writer` (all with `--tests`).
- The taxonomy report is clean for the three new directories.

## What changed (by design §3 item)

1. **Effective forwards everywhere.**
   - New pieces: `EffectiveSupersessions`, `EffectiveOperation` (a decoded replacement retires itself),
     `effective_operation`, `effective_forwards` and `fold_effective_edit` (region `🔖️EffectiveForwards` in `S`).
   - Used by:
     - both `fold_history` (free fn and `Self::`);
     - `materialize_document_snapshot`, which derives the map from the envelope's fold, so
       `assert_live_equals_replay` is an independent check;
     - `project_applied`, including the one-edit path;
     - the replay engine;
     - load (`settle_parsed_envelope`);
     - `ingest_remote` and `accept_quarantined_events`, and therefore `replay_envelopes_onto_pair` / hub check-in;
     - the retained initializer `BoundedStoreInitializationPhase`. It gains a new `FoldSupersessions` phase and
       `ApplyForward` reads the effective op; withdrawn ops are skipped and decoded ops retired.
   - Every fold site now uses keep-and-record semantics (`fold_operation` = `MutationOutcome::apply_to` without
     dropping a projection): an apply refusal is a Fatal message and folds as a no-op.
   - The `.spr` loader's separate file-order "validation replay" is removed. Settle folds the applied history
     with effective forwards instead.
2. **`EditReplay` stepper** (region `🔖️EditReplay`).
   - Modes: `ReplayMode::{Merge(policy), Report}`. Methods: `step(edits, deadline)` → `ReplayStep` with
     `ReplayProgress{done,total}`; `finish()` → `EditReplayResult`; `cancel()` (drop retires everything).
   - The running state is an `Arc<P>`, and scratch aliases retire via `retire_shared_projection`. There is no
     per-edit `P::clone` in either mode.
   - Report mode:
     - never quarantines and never aborts;
     - records rebased inverses and a `MutationReplayOutcome` per op;
     - withdrawn ops yield an outcome with `withdrawn`.
   - `replay_suffix_partitioned` is now a drive-to-completion wrapper around a `Merge` replay. Its old loop is
     deleted.
   - Convergence early exit: `ReplayConvergence` checkpoints (ring entries, tail snapshot, head) are checked from
     `horizon` on (`Arc::ptr_eq`, then `PartialEq` via `enable_convergence_early_exit()`, which needs
     `P: PartialEq`). When it converges, the previous head is spliced in, and `replay_report` splices the durable
     outcomes of the tail.
3. **Prefix-snapshot ring.**
   - New store field `prefix_ring`, capacity 8, adaptive stride `ceil(len/8)`.
   - Entries are keyed by `forward_prefix_digests`: ids, forward counts and effective supersession digests, never
     inverses or metadata.
   - Filled by the full-fold `project_applied`, by Report replays (`.recording(stride)`) and by `state_before`
     (live prefixes only).
   - `prune_prefix_ring` runs in `bump`. Eviction goes through the snapshot factory when this is the last alias and
     only drops the count otherwise.
   - Closing uses a new disposer phase `PrefixSnapshots`. The Drop and terminal witnesses include the ring.
4. **Commands.**
   - `ArtifactCommand::Supersede { scope, inputs: Vec<SupersedeInput<M>> }` (ordinal 17) and
     `CreateAlternativeWithSupersede { name, inputs }` (ordinal 18).
   - `SupersedeInput { target, replacement: Option<M> }` is a named struct, not the design's tuple. The semantics
     are the same.
   - Codecs:
     - ToValue/FromValue: derived.
     - Text: header line plus indented `replace <id>` + 4-space op line, or `withdraw <id>`.
     - Binary: yes.
   - Validation (`supersede_inputs`): the target is an applied op; neither the target nor the replacement may
     `may_emit_foreign_steps`; the replacement must re-encode canonically; the scope must be a known alternative;
     `TransitionSupersede::validate`.
   - One authoring path, `author_supersession`:
     - builds the transitions (`NewAlternative`: pending commit + `Branch` at the head + scoped `Supersede`);
     - folds the prospective log;
     - dry-runs Report (or adopts a supplied finished replay);
     - refuses `Rejected { policy: Normal, messages }` when the report `blocks_finalize()`, changing nothing;
     - installs everything with `install_transitions` in one outbound batch, **without replaying again**
       (`reproject_replayed`).
   - The Supersede envelope's `dependencies` are its targets. Its `target` is the common conflict-target prefix of
     the originals and replacements, re-derived on load. `observed` = the newest foreign op (W2-E request).
5. **`reproject`** now detects supersession changes (it compares the fold map with the store's `supersessions`
   field).
   - `replay_supersessions` Report-replays from the first position whose effective input or order changed
     (`replay_window`).
   - It writes rebased inverses and `replace_edit_messages` (all-or-nothing byte check first), resets the tail
     cache, and sets `current`, the ring, `revision_dirty_from` and cause `Replay`.
   - Remote supersessions converge through the same path. `admit_remote_transitions` validates the schema,
     decode and canonical form of remote Supersede payloads.
   - `reproject` returns the replayed `EditMessages`. `ingest_remote` puts them in its `MergeReport`, and
     `replay_envelopes_onto_pair` refuses `Normal.rejects(worst)` as `Rejected { Normal }`.
6. **Read API.** `supersessions()`, `replay_edits()`, `mutation_ops()` (`AppliedMutation` with id, edit id,
   position, op index, op, meta, `transaction`, supersession), `mutation_outcomes()` (durable, per mutation),
   `state_before(id, drafts)` (O(stride)), `begin_report_replay(drafts, from)` (stamped with generation, revision
   and drafts), `replay_report(result)`.
   - Also `commit_finished_replay(result, HistoryFinalization::{Overwrite, Alternative{name}})`. It checks
     freshness: a generation or revision mismatch returns the new `VcsError::Stale { expected_generation,
     generation }` (in `🌿️vcs`). It refuses a blocking report and installs atomically, using the same install as
     the command path.
7. **Revision.** `CursorRevisionAccumulator::reconcile` takes the supersession map and a `dirty_from`. The edit
   digest is extended with the effective supersession digest, and records are rebuilt from the first rewritten
   position. `ingest_remote` also marks `k` dirty, which fixes the old blindness to interior inverses.
8. **Persistence.**
   - `.spr` `HistoryOpMeta.transaction` (presence bit 7, dict-interned id and tool) in `📡️spr/📜️history/🦀️.rs`,
     plus retirement for it.
   - The `.ops` `supersede` line was already landed by W1-A.
   - `HistoryLog::fold` resolves supersessions (tested through the `.spr` round trip).
   - On load with supersessions, `replay_superseded_history` recomputes persisted inverses and messages with one
     Report replay (in-place ledger replacement, matching the live order).
9. **Metadata.**
   - `semantic_kind` (`<schema>#<kind>`) and `label` (the descriptor `display_name`) are stamped by
     `replay_mutations`, `fold_batch_item`, remote ingest, accept-quarantined, and both loaders
     (`stamp_edit_semantics`).
   - `transaction`:
     - `ArtifactCommand::Apply`/`ApplyInLane` gain `transaction` in all three codecs;
     - `AmendLast` keeps the tail edit's transaction;
     - `begin_apply_batch`/`begin_outbound_apply_batch` take a trailing `transaction: Option<TransactionRef>`,
       stamped in `fold_batch_item`, with `ArtifactStoreBatchPublication::transaction()`.

## Files

- **Store, spr and vcs sources.**
  - `S`
  - `ST/🔄️sync/🦀️.rs` (HistoryOpMeta.transaction from the envelope)
  - `ST/🧩️composition/🚪️open/🦀️.rs` (retirement of `HistoryOpMeta.transaction` and `TransactionRef`)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs` and its `🧪️tests/🔬️unit/🦀️.rs`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs` (`VcsError::Stale`)
- **Plugin and host.**
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: retained initializer region only.
  - `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`: a one-line bound (`materialize_backbone_snapshot` needs
    `OpBinary`).
- **Codemod.** `🧪️w1-g/codemod-transaction.py` added `transaction: None` at 283 `ArtifactCommand::Apply{..}` /
  `ApplyInLane{..}` literals and a trailing `None` at 42 `begin_apply_batch` / `begin_outbound_apply_batch` calls
  across 98 files (store, plugin, hub, os host, flow, mcp, run, ~45 s-plugins). The file list is in the dry-run
  output.
- **Tests.**
  - `ST/🧪️tests/🧪️supersede-replay/🦀️.rs`: 14 laws, mounted from `ST/🧪️tests/🔬️unit/🦀️.rs`, which also has
    the ghost-edit test adapted.
  - `ST/🧪️tests/🧪️supersede-replay/🟦️.ts`: bun test, Ajv + independent fast-json-patch replay.
  - `ST/🧫️fixtures/🧫️supersede-replay/🔣️.json`: 7 cases, including withdraw, blocking delete and coalesced
    interior op.
  - `ST/🧬️schema/🔣️supersede-replay/🔣️.json`
- **Ticket inputs (kept).** `🧪️w1-g/{apply-blocks.py, blocks-01..19-*.txt, codemod-transaction.py,
  run-each-test.sh}`.

## Commands and exact results

| Command | Result |
|---|---|
| Baseline: `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w1g cargo test -p semio-framework-os-kernel --lib --features sync --no-run`, then `🧪️w1-g/run-each-test.sh` over `os_store:: os_vcs:: os_spr::history os_spr::protocol_laws os_spr::materialize` (one process per test; a stack overflow aborts a plain `cargo test` run) | **638 ok / 1 FAIL** (639) |
| After the core change, same runner | 638 ok / 1 FAIL (identical set) |
| Final, same runner (653 tests incl. 14 new) | **652 ok / 1 FAIL**; the only FAIL is the baseline's `retained_clone::…remain_grant_bounded` |
| `cargo test … --features sync supersede_replay_tests` | 14 passed, 0 failed |
| `bun test ./ST/🧪️tests/🧪️supersede-replay/🟦️.ts` | 2 pass, 0 fail (8 expects) |
| `cargo check -p semio-framework-os-kernel --lib --tests --features sync` | Finished, only pre-existing warnings |
| `cargo check -p semio-framework-plugin` | Finished (95 warnings, pre-existing) |
| `cargo check -p semio-framework-os -p semio-framework-os-flow --tests` | Finished |
| `cargo check -p semio-s-artifact-puzzle-2d -p semio-s-artifact-writer-writer -p semio-framework-os-mcp --tests` | Finished |
| `cargo check -p semio-hub --tests` | **1 error in W2-E's new file** (see open items) |
| `bun ./📜️script.ts verify taxonomy report --scope <new dir>` ×3 | clean=true errors=0 warnings=0 |

The prior ticket's "51 pre-existing failures" figure did not reproduce under per-test isolation. The baseline had
one failure.

## Open items for W2 and peers

- **Hub (W2-E), messaged.** `🌎️hub/🗿️artifact-authority/📌️check-in/🧪️tests/🔬️unit/🦀️.rs:232` needs
  `transaction: None`. The file was written after the codemod ran.
- **Unchecked s-plugin crates.** About 45 s-plugin crates with codemodded test files were not compiled here
  (build budget). A sample of 3 compiled clean, and the transform is uniform.
- **W2-A (plugin runtime).**
  - Call `store.enable_convergence_early_exit()` for `VcsArtifactApp` stores; snapshots are `PartialEq`.
  - Drive `begin_report_replay` + `step(replay_edits(), deadline)` as the latest-wins job, then call
    `commit_finished_replay`. `Stale` means replay again.
  - Pass `Emit.transaction` via `begin_apply_batch(…, Some(tx))`.
  - Localize `MutationMeta.label` (the descriptor `display_name`) through `semantic_kind` when rendering.
- **Retained initializer.** It now reads effective forwards, but no plugin-level test exercises it with a
  supersession. It still refuses Fatal ops, while the store's folds keep-and-record them; that inconsistency
  predates this work.
- **Stale durable messages.** Messages recorded before an interior undo/redo/checkout are not recomputed by
  `project_applied`. That was already true, but convergence early exit splices durable tail outcomes, so "early
  exit == full replay" relies on durable messages being replay-consistent.
- **Edit boundaries.** The per-edit message ledger is 4 KiB/entry. A long superseded suffix with many messages is
  refused rather than truncated.
- **`apply_to`.** `protocol::MutationOutcome::apply_to` drops the displaced projection with a plain drop. The store
  uses its own retiring twin, `fold_operation`. Suggest that W1-A return the displaced value.

## Follow-up: audit-core fixes (2026-09-30)

Scope: every finding of `📓️audit-core.md` that touches W1-G's files (F-C1, F-M1..F-M6, the owned minors), plus the
audit §3 test gaps. Aliases as in the audit: `S` = `🏪️store/🦀️.rs`, `HYD` = `🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`,
`P` = `🔌️plugin/🦀️.rs` (initializer region only), `SY` = `🏪️store/🔄️sync/🦀️.rs`. Patch inputs are kept as
`🧪️w1-g/blocks-20..25-*.txt`, `🧪️w1-g/reproject-region.rs.txt`, and the emoji tools `🧪️w1-g/doc-emoji-{audit,rename}.py`.

### One supersede law, total at every fold site (F-M2, F-m6)

- `admit_replacement(original, replacement, schema)` (`S`, region `🔖️EffectiveForwards`) is now the only law.
  - The original must plan no foreign steps.
  - A replacement must be an operation of the document's schema, in its canonical encoding, and must plan no foreign steps.
  - It is used by authoring (`supersede_input`), by the fold (`effective_operation`), and by load (`supersession_address`).
  - `validate_remote_supersede` is deleted: remote ingest no longer refuses a supersession.
- `effective_operation(…, schema, supersessions)` is total. A supersession that breaks the law folds its target as a
  no-op with one `Fatal` `mutation.invariant` message: target = the mutation id, `op_index` set, exposed by
  `EffectiveOperation::fault()`.
  - `EditReplay` records the fault in the outcome, which has `superseded` = true and `withdrawn` = false.
  - Folds without messages skip the operation.
  - `EditReplay::new` and `replay_suffix_partitioned` take the document schema.
- Semantic kind is not constrained: history may replace a leaf by another kind (the severity and demo corpora do this).
  - `AppliedMutation.effective` exposes the descriptor of the leaf that actually folds (kind and display name).
  - Recorded `meta` is never restamped.
- Folding code is deduplicated.
  - One `fold_effective_edit(&mut impl FoldProjection, edit, schema, supersessions, take)` serves `fold_history` (both),
    `materialize_document_snapshot`, `fold_recorded` and `state_before`.
  - `FoldProjection` is implemented for `ReplayProjection` and `Arc<P>`.
  - `fold_operation` is `pub` and is the keep-and-record step every site takes, the retained initializer included.

### Hydration and the retained initializer (F-C1, F-M1)

- `HYD`: `Begin` calls `runtime.set_supersessions(fold.supersessions)`.
  - The raw file-order `ValidateReplay`, `RetireValidation` and the raw `ReplayCursor` fold are gone.
  - `ReplayCursor` now steps the load's Report replay (`loaded_history_replay`) under the job deadline, spending one
    unit of fuel per operation.
  - It then adopts the replay through the same `adopt_loaded_replay` that `.pack`/`.spr` and `.ops` loads use:
    state, rebased inverses, messages, supersession targets.
  - `FinishEdit` stamps edit semantics as the parse path does, so hydration and parse build the same content revision.
- `ArtifactStoreInitializationRuntime::push_{applied,redo}_edit` digest effective edits, so runtime-built revisions
  equal reconciled ones.
- `P` `ApplyForward` (initializer region only) reads `store::effective_operation` and folds with `store::fold_operation`.
  - A faulted, withdrawn, Fatal or apply-refused operation is a no-op.
  - Only an encoding fault still refuses the load.

### Transactional reproject, bounded messages, interior replays (F-M3, F-M6, F-m1, F-m4)

- `bound_edit_messages(edit_id, &mut messages)` (`S`, beside `ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES`) replaces every "exceeds
  its exact byte authority" refusal. It is deterministic:
  - worst level first, ties in recorded order, kept messages stay in recorded order;
  - the rest become one `mutation.cascade` info, "N more messages", at the first dropped operation;
  - a single oversize worst message keeps its code, with its text cut.
- It is applied in `EditReplay::close_edit`, which also rewrites that edit's outcomes so early exit still equals a
  full replay, and in `record_edit_messages`/`replace_edit_messages`. Loads get it through the replay.
- The `mutation.cascade` summary code follows the coordinator's "allowed code" rule. A dedicated code would need
  W1-A's replay-report schema and W2-A's label table.
- `reproject` works in three steps: `reprojection_replay` (pure), `preflight_reprojection`, then `adopt_reprojection`.
  - The preflight checks everything before anything is adopted: fact vacancies per ledger (new
    `ArtifactHistoryLedger::vacancies`), pin URIs, message-ledger slots, and displaced-owner capacity
    (`REPROJECTION_OWNER_SLOTS`).
  - The ghost alternative/checkpoint left by a failed `CreateAlternativeWithSupersede` can no longer occur.
- Every reprojection that is not a pure tail removal is now a Report replay from the first differing position: interior
  undo/redo/checkout, a redo at the tail, and supersession changes.
  - Durable messages and inverses therefore stay replay-consistent, and the convergence splice is exact.
  - A pure tail removal keeps the tail-snapshot / prefix-ring fast path (`prefix_projection`).
  - An append of one edit at the tail keeps its tail snapshot (`adopt_report_replay(result, tail)`).
- `report_replay(…, stride)` seeds the ring with the prefixes the base was folded through, plus the base itself.
- Redo revision records are rebuilt whenever a replay dirtied the history (`dirty_from.map(|_| 0)`).
- Report mode retires the rebased inverse of an operation that failed to apply.
- Removed: `project_applied`, `replay_supersessions`, `adopt_fold_replaying`, `take_tail_snapshot_for_current`,
  `disable_convergence_early_exit`, `EditReplay{,Result}::from_position`, `EffectiveOperation::original`.
  `evict_prefix_snapshot_reserved` uses `Arc::try_unwrap`.

### Check-in and the sync actor (F-M4, F-M5)

- `replay_envelopes_onto_pair` no longer refuses at an intermediate blocking report.
  - If the ledger changed effective inputs, it judges the folded history once, via `refuse_blocking_history` (a
    Report replay of the whole applied history), and refuses with `Rejected { Normal }` only if that final report blocks.
  - A quarantine conflict or a pending dependency still refuses as before.
  - `commit_finished_replay` refuses a blocking finished replay before it validates inputs, so an invalid draft reads
    as `Rejected` with its `mutation.invariant` message.
- `SY`, decided per "author stamps are canonical": both actors (native and browser) now send the authored `(hlc, id)`
  unchanged. They no longer stamp a send-time clock.
  - The actor string is still rewritten to the socket subject: the hub refuses "socket subject actor mismatch", and
    share sockets get an ephemeral actor per connection.
  - This rewrite is fold-neutral. Order is `(hlc, id)`, supersession has no ownership rule, digests exclude the actor,
    and Revert ownership is consistent per replica.
  - `next_timestamp` and `now_ms` are native-only now; they only stamp clock-less external folder envelopes.

### Persistence and load consistency (F-m5, F-m14, F-m2, F-m3)

- Every load (`settle_parsed_envelope`) runs the load Report replay, supersessions or not:
  - an operation that no longer applies is recorded as a Fatal no-op;
  - `fill_supersession_targets` runs for every load and is itself total.
- `observed` is persisted:
  - `.spr` `HistoryTransitionRecord.observed` (binary format 2 appends it; format 1 = none; text field `observed=`);
  - the `.ops` `supersede` line (`observed=`).
- The `.ops` metadata records no longer carry the derived `semantic_kind`/`label`, matching `.spr`. The config
  retained loader stamps semantics like every other load, which fixes the plugin `window_config` byte-exact
  reopen tests.
- The config retained loader refuses a fold with supersessions.
- `expected_mutation_message_level` admits `mutation.apply.*` at `Fatal`.
- The 64-edit synchronous replay budget is documented on `reprojection_replay` and `dry_run` (F-m10).
- F-m12: docstring emojis are unique across the five owned regions (111 blocks, 111 distinct).

### Tests (19 new or changed laws)

- `🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs`, region `🧪️FoldSiteLaws`:
  - `a_member_document_with_supersessions_hydrates_to_its_superseded_state` — F-C1, plus a `.ops` supersede round trip.
  - `an_input_breaking_the_supersede_law_folds_as_a_fatal_no_op_at_every_site` — F-M2. Remote garbage and a foreign
    schema across ingest, outcomes, `state_before`, `.spr`, `.ops`, `materialize` and check-in; the ledger is refused,
    then repaired.
  - `oversize_replay_messages_are_bounded_deterministically_everywhere` — F-M3 at apply, install, branch, remote
    ingest and load.
  - `check_in_judges_the_folded_ledger_not_its_intermediate_states` — F-M4.
  - `an_interior_revert_keeps_the_early_exit_equal_to_the_full_replay` — F-M6, including an interior redo.
  - `a_supersession_of_a_redo_edit_keeps_revisions_order_independent` — F-m1.
- `🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs`: `concurrent_supersessions_of_one_operation_converge_through_the_hub_socket`
  — F-M5. Two stores and two native actors on loopback hub sockets relay in reverse order; the wire carries the
  authored `(hlc, id)`, and both replicas fold the same winner.
- `🌿️vcs/🧪️tests/🔬️unit/🦀️.rs`: `fixed_history_ledger_vacancies_count_every_admissible_insert`.
- `🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs`:
  `a_whole_document_archive_with_supersessions_loads_its_superseded_state` — F-C1 on the Envelope target plus F-M1.
  It covers a replacement, a withdrawal and a remote garbage supersession through hydration and the bounded
  initializer.
- Corpus (language-agnostic):
  - new case `an-input-breaking-the-supersede-law-folds-as-a-fatal-no-op`;
  - `invalidInput` added to `🧬️schema/🔣️supersede-replay`;
  - `🟦️.ts` oracle and the Rust harness updated.
- Fixture fix: `canonical_revision_distinguishes_interior_aba_across_load_and_reset` now builds its changed history
  with its real inverses (`RestoreN`). Load now recomputes derived inverses, so the old stale fixture no longer held.
- Sync test `DemoMutation` declares `may_emit_foreign_steps = false`.
- `.spr` literals gain `observed`: spr and plugin composition tests. `compile_ops_decompile_ops_round_trip` now
  round-trips an observed transition.

### Commands and counts

| Command | Result |
|---|---|
| Baseline (before the follow-up), `run-each-test.sh` over `os_store:: os_vcs:: os_spr::history os_spr::protocol_laws os_spr::materialize`, `--features sync` | **653 ok / 1 FAIL** (654) |
| After, same filter set (sync build) | **661 ok / 1 FAIL** (662 = 654 + 8 new); the only FAIL is the pre-existing `retained_clone::…remain_grant_bounded` |
| After, widened to `os_store:: os_vcs:: os_spr::` | 858 ok / 1 FAIL (same pre-existing one) |
| Final, widened filter, after the `.ops` metadata / config-load fix (peers added tests meanwhile) | **868 ok / 1 FAIL** (same pre-existing one) |
| `supersede_replay_tests` | 20 passed, 0 failed |
| `bun test ./🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts` | 2 pass, 0 fail (9 expects) |
| `cargo check -p semio-framework-plugin --tests` | Finished (before W2-A's in-flight time-travel test edit) |
| Plugin suite, per test, `RUST_MIN_STACK=268435456` | 879 ok / 13 FAIL. 2 were mine (`window_config` reopen byte-exactness, derived `.ops` metadata). They are fixed in the store; the rerun is pending because the plugin test build currently fails in W2-A's in-flight `🧪️time-travel` test edit (`time_travel_input_at` arity). The other 11 are unrelated peer work: `merge_ui_values` ×4 and `tool_run` ×4 (UInt vs Float), `window_kits` ×2 (`textEdit`), `command_ingress_terminal` ×1 (reactor source scan) |
| `a_whole_document_archive_with_supersessions_loads_its_superseded_state` | 1 passed |
| `cargo test -p semio-hub --lib --no-run` | blocked by a peer's in-flight stdio plugin edit (`unsupported mutation_leaf attribute`); hub check-in tests not rerun |

### Open items

- **F-M5 actor string.** The authored actor becomes canonical once hosts dispatch under the hub actor
  (`ArtifactEvent::Session.actor`, W2-A/host); the sync rewrite is then the identity. Until then, a transition
  authored under a local actor carries a content-addressed id naming that actor. Peers see the socket actor, and a
  peer's `.ops` reload refuses such a line. This is pre-existing for every transition kind. It needs either
  authoring under the hub actor or a content address that excludes the actor string (W1-A).
- **F-m8 (store part) is not done.** No backbone message lets the actor ask the store to retract a refused local
  transition. It needs a protocol/backbone seam (W1-A/W2-E). The hub's `RebootstrapRequired` still covers the
  checkpoint case.
- **F-m11b.** `read_supersede_inputs` still drops already-decoded ops when a later input fails, like `read_command_ops`
  does. The command codec is P-agnostic, and `retire_cold` needs `Mutation<P>`.
- **Config Supersede at dispatch.** `ConfigStore` is an alias of `ArtifactStore`, with no kind flag. A config
  Supersede (like a config Commit/Branch) is refused at load, not at dispatch.
- **Not owned, noted:**
  - F-m7 (schema ceilings, TS decoder);
  - F-m9 (hub target predicate);
  - F-m13 (`encode_history_transition` validation, base64 `.ops` payload; the `.spr` op-meta presence byte is full,
    so the next field needs a second byte).
- **Reruns pending on peers:** the plugin build after W2-A's time-travel test edit lands; the hub check-in tests after
  the stdio plugin edit lands.

## Follow-up 2: the trunk as a first-class, switchable alternative (2026-09-30)

Problem: after "finalize as new alternative", the original trunk was not listed and could not be switched back to.
That defeated "a new alternative preserves the original".

### Model

- The implicit root line is an alternative with a deterministic id.
  - Id: `trunk_alternative_id(document_id) = trunk-{hex16(blake3(str "semio.history.trunk" | str document_id))}`.
  - Implemented in Rust (`R/🔗️causal/🔀️transition/🦀️.rs`) and the TS twin (`trunkAlternativeId`). The Python generator
    emits `trunks` vectors in the history-transition corpus; the Rust test also cross-checks them against a
    third-party `blake3`.
- The log never names the trunk. The fold resolves it:
  - `fold_history(document_id, edits, transitions, excluded)` now takes the document id and sets `HistoryFold.trunk`.
  - Commits made while no branched alternative is active grow the trunk's chain.
  - The trunk is listed first in `alternatives`, with an empty name (the UI localizes "Main"/"Hauptlinie"), once
    its chain is non-empty. A document without checkpoints (a genesis or config store) still lists no alternatives.
  - A `Checkout` naming the trunk, or naming none, activates it. `HistoryFold.alternative` stays `None` on the trunk.
  - A supersession applies when its scope is `None` or equals the active line (the trunk's id on the trunk), so
    trunk-scoped supersessions work.
  - A `Branch` claiming the trunk id is a fold error. Repin re-identifies trunk checkpoints too.
  - The TS `foldSupersessions(documentId, operations, events)` twins this and returns `trunk`.
- Callers updated:
  - `HistoryLog::fold`;
  - `store::fold_event_log(document_id, …)` and its three store callers;
  - `🖥️host` `BackboneDocument::applied_edit_ids` and the host test helper;
  - the `HistoryFold` retire struct.

### Store

- `trunk_alternative_id()` and `active_line_id()` (the active branched alternative, or the trunk).
- `SwitchAlternative { trunk }` and `CheckoutCheckpoint` of the trunk head write `Checkout { alternative_id: None }`.
- `Supersede { scope: Some(trunk) }` is accepted.
- The history columns keep the trunk on lane 0 through `branched_alternatives`.
- W2-A was sent the exact API (`ac84cd80529822d7f`): pass `Some(store.active_line_id())` as the current alternative.

### Laws and tests

- **Replication (Rust):**
  - `the_trunk_is_a_first_class_alternative` covers chain growth, listing, checkout by id or none, trunk-scoped
    supersessions, switching back to the branch, branch refusal, and an unlisted empty trunk.
  - The corpus test checks trunk vectors, first-party and third-party.
  - The supersede-fold fixture gained 6 steps plus 1 refusal: a trunk-scoped supersession on the trunk, dropped on the
    branch, and restored by a checkout naming the trunk id; a branch claiming the trunk is refused.
  - `branch_commit_and_repin_track_alternative_chains` now expects the trunk first.
- **Store:** `a_new_alternative_preserves_the_trunk_it_branched_from` checks that:
  - `CreateAlternativeWithSupersede` lists the trunk first, and switching to the trunk gives the original positions;
  - the log never names the trunk;
  - a trunk-scoped supersession applies on the trunk only;
  - switching back gives the edited positions, and switching again gives the trunk's;
  - two replicas (reverse and authoring arrival order) list the same alternatives, stand on the same line, and share
    one content revision;
  - `.spr` and `.ops` reloads keep all of it, and a reloaded store switches.
- **Existing store and spr tests** that indexed `alternatives[0]` as the branch now pick the branch explicitly and assert
  the trunk.

### Counts

| Command | Result |
|---|---|
| `cargo test -p semio-framework-replication --lib` | 309 passed, 0 failed |
| `bun ./📜️script.ts test` (replication TS) | 17 passed (history-transition corpus incl. trunk vectors, supersede fold twin incl. trunk steps) |
| `python3 🧪️tests/🧪️history-transition/🐍️.py` | exit 0 (corpus regenerated with `--write`, then verified) |
| Kernel per-test runner, `os_store:: os_vcs:: os_spr::` | **878 ok / 1 FAIL** (the pre-existing `retained_clone::…remain_grant_bounded`) |
| `semio-framework-os-flow` (host) per test | 241 ok / 1 FAIL (`wasm_session::…hostile_source_census…`, a JS bundle census, unrelated) |
| Plugin per test (`RUST_MIN_STACK=268435456`) | 918 ok / 13 FAIL, none from this work. The `window_config` reopen tests and the archive-supersession test now pass. The 13 are peer work: `merge_ui_values` ×4, `tool_run` ×4, `window_kits` ×2, `command_ingress_terminal`, `activated_tool_factory_keys…` (`hostEvent`), `neutral_checked_diff_boundaries…` (job-test code rename) |

### Notes

- The transition id no longer includes the actor string (`history_transition_id(timestamp, payload)` changed
  meanwhile). The F-M5 open item about the actor-string content address is therefore resolved: the socket-subject
  actor rewrite is fully fold-neutral, including `.ops` reload.
- E2E R2-2 (folder reload collapsing history) is routed to W2-B. It has not asked about the archive yet. If it does, the
  store side is:
  - the `.spr` persists every edit and every transition;
  - load (`settle_parsed_envelope`) re-folds the whole log and Report-replays it;
  - hydration now does the same.

## Follow-up 3: config shape, retraction, all-or-nothing decode, concurrent supersede (2026-09-30 / 10-01)

### 1. Config stores refuse history transitions at dispatch

- **Law, schema-first.** Replication (`TR`) gains `HistoryTransitionKind` (`ALL`, `name()`), `HistoryTransition::kind()`
  and `HistoryShape { Document, Config }::admits(kind)`: a config history holds `revert` and `reinstate` only.
  - The language-agnostic corpus (`🔗️causal/🧫️fixtures/🧫️history-transition`, schema requires `shapes`) carries the
    table, generated by the Python generator (`🐍️.py`).
  - The Rust unit test checks the table against `admits` and that every kind is exercised. The TS twin
    (`HISTORY_TRANSITION_KINDS`, `historyShapeAdmits`) checks the same table.
- **Store.**
  - `ArtifactEnvelopeOwners.history_shape` is set to `Config` by `create_config_envelope` and by the retained config loader.
    The loader now refuses by the same law, per transition kind, instead of checking fold facts.
  - `ArtifactCommand::history_transition_kinds()` names the kinds a command authors. `dispatch_inner` refuses an
    inadmissible command first, with the typed `VcsError::HistoryShape { shape, kind }`: nothing runs and every typed
    replacement retires.
  - `install_transitions` (every local authoring path, Repin included) and `admit_remote_transitions` (ingest) apply the
    same guard.
- **Law** `a_config_store_holds_undo_and_redo_only`:
  - every non-undo command is refused typed, with the store untouched;
  - a remote Supersede is refused;
  - undo and redo work;
  - the refused kinds are exactly those the corpus table excludes.
- **Note (W2-A).** The plugin's interaction store is built with `create_document_envelope`. If it is config-shaped, it
  should use `create_config_envelope`.

### 2. F-m8: a refused local transition is retracted; `for_good` is a named predicate

- **Backbone.** `BackboneMessage::Retract { mutation_ids }` (ordinal 4, actor to store).
- **Store** (`retract_transitions`):
  - removes every named transition this replica authored, plus every local transition causally depending on one
    (`transition_retraction_closure`), then folds again;
  - is transactional: if the log would not fold, every transition is restored;
  - retracts nothing for an unknown name or a transition another replica authored;
  - the history ledgers now follow a shrinking fold: `ArtifactHistoryLedger::extract_if`, and `adopt_history_facts`
    drops changes, checkpoints and alternatives the fold no longer holds.
- **Persisted log.** `retract_history_transitions_from_spr` applies the same closure, so a folder reload never
  resurrects a refused step.
- **Actors.**
  - Native, wasm and TS-worker actors no longer let the author stay ahead. On `Rejected` they roll operations back by
    inverse, as before. They now also retract the batch's transitions: Rust `retract_refused_transitions` and
    `deliver_retraction`; TS `refusedBatchCorrection`.
  - The same applies to a `Transformed` batch and to the wasm actor's locally refused oversized batch.
  - The native actor rewrites its persisted spr (`persist_retraction`).
  - The typed `history.transition-refused` refusal is still raised, now saying the steps are withdrawn.
- **Twins and admission.**
  - TS codec: `retract` is tag 4; the Rust-only `member` tag 3 is refused.
  - `parseInboundDocumentBackboneMessage` admits mutations or retract on actor-to-shell events.
  - Shell: `receiveDocumentBackbone` in ShellHost admits `retract`; W2-B was told.
  - Every egress path (guest to actor) refuses a retract: plugin binding TS, mcp workspace, wgpu shell, both Rust actors.
- **Hub.** `DbError::is_transient()` covers Unavailable, Timeout, Closed, Fenced, StaleGeneration and Io. `for_good` and
  `messages_for_error` (the transient resend code) both read it, so the hub's rebuild decision and the client's resend
  cannot drift.
  - The hub's refusal rebootstrap still runs in the checkpoint case. With retraction it is redundant for refused
    transitions; that is the hub owner's call.
- **Laws:**
  - store: `a_refused_local_transition_retracts_and_the_author_converges_with_the_hub`. It covers a refused Supersede and
    a refused finalize-as-alternative, plus a dependent checkout that retracts with them. Afterwards the author has the
    hub's event log, snapshot, supersessions, alternatives and outcomes, and its own pre-refusal revision. Re-, unknown
    and foreign retractions are no-ops, and the persisted spr retracts the same closure.
  - sync: `a_refused_supersession_is_retracted_as_a_typed_refusal`.
  - parity scenario `ack-rejected-transition-retracts`, run by the Rust and TS drivers. Schema additions:
    `queueTransition` and `retractedMutationIds`.
  - Retract hex vectors (`01040001000c00`, `010401016101000c010600`) in the Rust store unit test and the TS
    backbone-envelope-io test.
  - the Vigilant arm of the hub law below.

### 3. F-m11b: command decode is all-or-nothing

- `ArtifactCommand` is no longer an `OpBinary`, because decoding must name `P` to retire what a refused command decoded.
  It now has:
  - `encode_command`;
  - `decode_command::<P>(bytes) -> Result<_, CommandDecodeError>`, where the error is
    `Layout(ProtocolError) | Operation { index, error }`.
- Decoding behaviour:
  - every operation and supersede replacement decoded before a failure retires through `Mutation::<P>::retire_cold`;
  - bytes past the end refuse the whole command, which then retires;
  - a hostile count never allocates beyond the input; `read_command_ops` used to `with_capacity(count)`.
- The text twin `parse_command::<P, Op>` does the same. `ApplyInLane` and `AmendInLane` now parse the lane before the
  operations, so nothing can fail after an operation was parsed.
- Callers updated: `dispatch_binary`, `dispatch_text`, `test_support::assert_command_text_binary_equivalence::<P, Op>`,
  the plugin config-command decode (one line in P, W2-A told), and the tests.
- **Law** `a_refused_command_decode_retires_every_operation_it_decoded`. It uses a `WitnessOp` that counts cold retirements
  and bare drops. The cases are: garbled third op, garbled supersede input, garbled nested undo command, trailing bytes,
  hostile count, and text Apply and Supersede. In every case each decoded op retires and none is dropped.

### 4. Two-replica + hub concurrent supersede

- **Law** `concurrent_supersessions_through_the_hub_converge_both_replicas` (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, region
  `✏️ConcurrentSupersede`) runs real document sockets and GIS Map replicas.
  - The replicas author as their socket's hub actor (`set_local_actor_id`).
  - A seed author's edits are relayed to two authors. Each pair of supersessions is authored before either author saw the
    other's.
- **Normal:**
  - the same operation replaced twice;
  - different operations;
  - withdraw against replace of one operation;
  - trunk-scoped against a new alternative's scoped supersession;
  - then a checkout back to the trunk.

  After each round both replicas hold the same document, effective supersessions, content revision, outcomes,
  alternatives and line. A contested operation folds the last supersession by `(hlc, id)`. The trunk-scoped
  supersession waits on the alternative and takes effect on the trunk.
- **Vigilant:** the later concurrent supersession of the same operation is refused with `mutation.clamped`. Its author
  retracts it (`Retract`), and both converge on the committed one.

### 5. Kernel `retained_clone` failure

- `retained_clone::tests::unit_generic_enum_and_recursive_records_remain_grant_bounded` is pre-existing and stack-only.
  - Its module and test were last changed on 2026-09-28; this ticket never touched `🧬️retained-clone`.
  - It overflows the default 2 MiB test-thread stack with its deep recursive record.
  - `cargo test` passes because `.cargo/config.toml` `[env] RUST_MIN_STACK = 64 MiB` applies.
  - It failed only under my direct binary runner. That runner now exports 32 MiB.
- Not a store or kernel defect; nothing to fix.

### 6. R2-2 (W2-B)

- I sent W2-B the store API: `print_document_pack`/`parse_document_pack`, `print_document_text`/`parse_document_text`,
  the archive codec, the passing laws, and the warning that envelope JSON is a projection without `transitions`.
- W2-B confirmed the store side natively and added the plugin law
  `a_document_archive_round_trip_lists_every_history_row_of_its_source`. The defect it found was its own:
  `retire_displaced_document_rows`.
- The remaining gap, a command's label after reload, needs a locale-neutral verb on the edit. W2-A takes the `Edit.verb`
  store change. I sent it exact pointers and told it the field must also ride the wire envelope.

### F-M5 TS twin (exact relay timestamps)

- `hubWireEnvelope` keeps an exact envelope's authored `(hlc, id)` and only rewrites the actor.
- The TS wire HLC is `number`, while a replica's HLC actor is random u64 entropy past 2^53. The worker therefore now sends
  `Commands` through `encodeClientCommandsFrameExact`, which writes bigint varints.
- One vector checks it in three places: an independent Python oracle (`🧪️w1-g/client-commands-exact-oracle.py`), the Rust
  test `client_frame_commands_keep_exact_hlc_fields`, and the TS test "sends exact envelopes as a Commands frame…". Below
  2^53 it is byte-equal to `encodeClientFrame`.

### Commands and counts

| Command | Result |
|---|---|
| `cargo test -p semio-framework-replication --lib` | 313 passed, 0 failed |
| `bun ./📜️script.ts test` (replication TS) | 20 passed |
| `python3 🔗️causal/…/🧪️history-transition/🐍️.py` | exit 0 |
| Kernel lib tests, `--features sync`, per test (`run-each-test.sh`, `os_store:: os_vcs:: os_spr::`) | **883 ok / 0 fail** (rerun after the fleet reset); includes 24 supersede-replay laws, the sync and parity laws |
| os TS, filtered (`retraction`, `backbone parity`, worker wire) | 28 passed, incl. `backbone parity: ack-rejected-transition-retracts` and the retract vectors |
| os TS, full (before the reset) | 557 passed / 1 failed. The failure is `🏷️schema-vocabulary`: `x-semio-fixture` in a peer's `🌎️hub/🧫️fixtures/🐳️docker-image-v1` |
| os TS, full (after the reset) | blocked: the worker file no longer imports. `💡️inference/🚪️opening/🟦️.ts` imports a missing `🔌️service/🟦️.ts` (peer, inference) |
| `os-hub` bin tests (`--test-threads=4`) | 173 passed / 1 failed. `concurrent_supersessions_through_the_hub_converge_both_replicas` passes. The failure, `a_supersession_is_admitted_whatever_its_author_and_its_refusal_rebootstraps_the_author`, decodes the hub's `RebootstrapRequired` as "rebootstrap control identity is invalid": the baseline frontier's document id does not match the control's (owner: hub rebootstrap / `🛰️lag-rebootstrap`), not mine |
| `cargo test -p semio-hub --lib` | 247 passed / 8 failed, none mine: `auth::…credential_derivation…` (timing under load), inference ×6 (`command`, `runtime`, `sqlite` ×3, `wal`), `trusted_catalog::…one_rule` (energy descriptor `vec3`). The check-in supersede laws and the transient refusal law pass |
| `cargo check --keep-going --tests -p semio-framework-plugin -p semio-framework-os -p semio-framework-os-mcp -p semio-framework-os-renderer-wgpu` (after the workspace loaded again, 02:38) | host, mcp and wgpu renderer (lib tests) and the plugin lib compile: every match over `BackboneMessage`, the plugin's `decode_command` line and the envelope `history_shape` field are consistent. The plugin lib-test target fails 10× E0433 (`crate::os_store` in `🧪️tests/🖥️test-app-mutations-document`, `🔬️app-declarations-fixture` and `🧩️composition`). That is a peer's in-flight sqlite-snapshot codec edit (`ArtifactSqliteSnapshot`), not mine, so the plugin per-test suite cannot be rerun yet |

- After those runs I only made docstring changes: every new docstring emoji is now unique in its region or file. They are
  verified with `cargo check -p semio-framework-os-kernel --features sync` (exit 0). The kernel lib-test target then
  stopped building for a peer reason: `🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json` was deleted while
  `📇️directory/🧬️schema` tests still `include_str!` it (inference owner).
- The worker TS file no longer imports: `💡️inference/🚪️opening/🟦️.ts` imports `../../🔌️service/🟦️.ts`, but the module
  lives at `💡️inference/🔌️service` (inference owner).

### Open items

- The `Edit.verb` store change: W2-A.
- `history.transition-refused` UI text ("rebuilt from the hub") should say "withdrawn": owners of the wgpu `⏪️time-travel`
  strings and `ShellHelpers`.
- The hub refusal rebootstrap for transition refusals is now redundant: hub owner.
- Peer failures from the earlier plugin run, unchanged owners: `merge_ui_values` ×4 and `tool_run` ×4 (UInt vs Float),
  `window_kits` ×2 (`textEdit`), `command_ingress_terminal` (reactor source scan), `activated_tool_factory_keys…`
  (`hostEvent`), `neutral_checked_diff_boundaries…` (job-test code rename).

## Follow-up 4: the retract story end to end (2026-10-01)

### 1. The hub forces no rebuild after a refusal

- `🌎️hub/🏗️bootstrap/🦀️.rs`:
  - removed `ClientFrameStepV1::Rebootstrap`, `after_refusal` and `holds_history_transition`, and `submit_commands` now
    returns no `for_good`;
  - every refusal path answers its `Ack` and keeps the socket live (`after_send`);
  - `rebootstrap_document_socket` serves the lagged socket only, so its `lagged` parameter is gone and it always closes
    `1013 rebootstrap-required`;
  - `DbError::is_transient` still selects the transient resend code in `messages_for_error`.
- The author converges by retraction instead: Rust actors, TS worker and, new, the browser actor.
- **The real defect behind the old test's failure.** A document's creation (genesis) checkpoint has a baseline frontier
  with an empty `head_edit_id`. Both product creation (`🗿️artifact-authority/🌱️creation`) and the test seed produce it.
  The wire `validate_rebootstrap_required` and its TS twin refused every control with an empty head edit. A lagged socket
  on a document not yet checked in therefore received a `RebootstrapRequired` its client could not decode.
  - Fixed in both twins: only a genesis baseline (ordinal 0 and commit 0) may name no head edit; every later baseline
    must.
  - Rust law `rebootstrap_controls_admit_a_genesis_baseline_and_refuse_a_later_one_naming_no_head_edit`, with a TS twin
    in `🧪️artifact-bootstrap-protocol`.
- **Hub test rewritten.** `a_supersession_is_admitted_whatever_its_author_and_its_refusal_rebootstraps_the_author` became
  `…_and_its_refusal_leaves_the_socket_live`. The ghost supersession is still refused `history.unknown-target` and commits
  nothing. Now no rebuild follows: the author's next supersession commits on the same socket, and it is the first thing
  the other author receives.
- **Sync test.** `a_refused_supersession_is_retracted_as_a_typed_refusal` no longer simulates a hub rebuild. It asserts
  that nothing is requeued and nothing rebuilds.

### 2. `history.transition-refused` says the step was withdrawn

- EN "The hub refused a history edit; the step was withdrawn." DE "Der Hub hat eine Verlaufsbearbeitung abgelehnt; der
  Schritt wurde zurückgenommen."
- One atomic edit covers all five pinned places:
  - the wgpu `⏪️time-travel` `HISTORY_REFUSALS` row;
  - the React bundle key `ui.history.refusal.transitionRefused` (EN and DE, normal and beginner);
  - the `🧫️command-rejection` corpus;
  - the `🧫️time-travel-band` corpus.
- W2-C and W2-B were told beforehand. W2-C listed the corpora, and both confirmed the scope.

### 3. Timestamp relay TS twin, and the browser actor

- The TS twin had already landed in follow-up 3: `hubWireEnvelope` keeps the authored `(hlc, id)`, and
  `encodeClientCommandsFrameExact` writes exact 64-bit varints. The shared vector is checked in Python, Rust
  (`client_frame_commands_keep_exact_hlc_fields`) and TS (replication TS, 21 passing).
- New: a mounted browser actor now retracts too.
  - `applyAckCorrection` delivers the `retract` message to the actor's document port
    (`DocumentBrowserActorReservation.receiveBackbone`). This applies when the refused batch holds only history
    transitions.
  - It still rebuilds the actor (`requireArtifactRebootstrap`) when refused operations need rolling back.
  - `receiveBackbone` and the retention before binding admit retract messages (`admitInboundBackbone`). The guest store's
    pump handles the retraction.
  - Law: "a mounted browser actor retracts a refused history step and rebuilds only for refused operations", in
    `🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts`.

### 4. W2-A

- I told W2-A about the interaction store, which is built with `create_document_envelope` and should use
  `create_config_envelope` if config-shaped, together with the `Edit.verb` pointers (follow-up 3 message).

### Commands and counts

| Command | Result |
|---|---|
| `cargo test -p semio-framework-replication --lib` | 313 passed / 1 failed. The new rebootstrap-control law passes. The failure, `causal::tests::envelope_transaction_round_trips_through_binary_and_value`, comes from W2-A's in-flight `Edit.verb` wire-flags change in `🔗️causal/🦀️.rs`, not mine |
| replication TS | 21 passed |
| os TS `👷️worker` | 14 passed: the browser-actor retraction law and the parity scenarios, `ack-rejected-transition-retracts` included |
| renderer-react `test long` `🛠️ShellHelpers/🧪️tests/🧪️command-rejection` and `⏪️time-travel/🧪️tests/🧩️component` | 3/3 and 17/17 passed |

## Session 2 — 2026-10-01

Successor of W1-G (S2-W1G), coordinator `⚪552b484a…`. Scope: design §15 (transaction-scoped amend), the follow-up-4
re-verifications, then §16.6 / G9. **Status (17:55): §15 done and verified (store 8/8, plugin 2/2, TS twin 5/5); G9 store
half done and verified (4/4: deferred remote reprojection with progress + cancel, ring sizing, scoped finalize); runtime
adoption of G9 handed to S2-W2A (below).** A usage cut interrupted 13:30–16:30.

### Repair (rule 21)

- The predecessor's follow-up 5 (§15 store half) was complete and compiling in HEAD (`4e36b2b5012`, 11:16): the three
  commands, codecs, `VcsError::{TransactionOpen, UnknownTransaction}`, the dispatch gate. There were no tests, no fixture, no
  persistence decision and no runtime route. No half-finished edit had to be repaired.
- Three defects in that half were fixed (below): open edits leaked into every persisted form; a remote edit could quarantine
  or reorder the unannounced open edit (lazy `keep_open_edit_at_tail` re-stamp, deleted); appended outcomes carried the op
  index of their tick instead of their position in the edit (found by the N-appends == one-Apply law).

### §15 store (`OS/🏪️store/🦀️.rs`, region `🔖️ToolTransactions` and the touched sites)

- **The open transaction lives on the envelope** (`ArtifactEnvelopeOwners.open_transaction`, store field removed) with
  `holds_committed_edit(edit_id)`. Every persisted form leaves the open edit, its messages and its lane out:
  `print_ops_log` (`.ops`), `print_document_spr` (`.spr`/pack/archive), `event_log` (backbone attach), member
  `announce_tail_edit_payload`. Decision: **a crash or reload mid-transaction leaves zero trace**.
- **Append / commit / abort.** Append offsets every recorded message's `op_index` by the edit's length. Commit
  (`commit_open_transaction`, pub, never suspends) stamps the primary identity of a one-op edit (as `apply_command` does), so
  N appends + commit records exactly the edit one `Apply{transaction}` records, and queues all ops for one announcement
  (`flush_announcements` sends; `dispatch` flushes itself). Abort (`abort_open_transaction`, pub, never suspends) retires the
  edit and its messages and gives back the edit sequence it took. `reproject` delegates to the non-suspending `reproject_now`.
- **Remote edits under an open transaction.** `ingest_remote` is a wrapper around `ingest_remote_merge` (the old body):
  `lift_open_transaction` takes the open edit off the tail (tail-cache fast path), the merge runs without it,
  `restore_open_transaction` re-stamps its ops after everything seen, gives it the next sequence and folds it back with a
  Report replay (keep-and-record, never quarantined). The open edit is therefore always the applied tail, keeps its ids, and
  commits in the order every replica folds. If the ledger cannot take it back, the transaction aborts with zero trace and the
  refusal is answered.
- **Guards.** `apply_command` refuses while open (covers `dispatch_apply_exact`/`apply_one`), `commit_finished_replay`
  refuses while open; the dispatch gate stays as authored.
- **Batched publication route** (migrated typed operations): `ArtifactStoreBatchPublication::set_transaction_open(bool)`,
  `announce_items`. Admission refuses a batch that does not carry the open transaction. `batch_amend_target` targets the
  open edit by transaction id (never a coalesce key); a streamed batch keeps the edit open and unannounced (`outbound`
  cleared, ACK admitted); a closing batch appends, finishes and announces the whole edit once; a streamed batch with no open
  transaction opens one.
- **Typed refusals** (`OS/🌿️vcs/🦀️.rs`): `VcsError::HistoryFull { capacity }` and a hand-written `FaultFrom` giving
  `toolTransaction.open`, `toolTransaction.unknown`, `history.full` their own fault codes (everything else stays
  `module.vcs`). `reserve_edit_history_slot` and the batched preflight answer `HistoryFull`; the puzzle 2d ceiling law
  (`PZ2D/✏️editor/🧪️tests/🔬️unit/🦀️.rs`) asserts the typed code. The paged ledger (other session) has landed, so the 65th edit
  is admitted now and `HistoryFull` only answers address-space exhaustion; that session replaced my full-history law with
  `a_history_past_one_page_keeps_admitting_edits` in my test file (it passes).
- No replication codec changed: committed ops already carry `MutationEnvelope.transaction` (W1-A). There is no TS codec of
  `ArtifactCommand` (the guest→store path is Rust-only), so the TS twin is the law twin below.

### §15 runtime (`OS/🔌️plugin/🦀️.rs`, S2-W2A's file, §15 regions only; `⏪️time-travel/🦀️.rs` one guard)

- `Emit.transaction_phase: TransactionPhase { Commit (default), Stream, Abort }`; `Emit::stream_transaction`,
  `Emit::abort_transaction`; `Emit::commit_transaction` always carries its ref (an empty commit closes a streamed edit).
- `tool_transaction_shape_fault` **refined, not lifted**: still refuses a transaction with a coalesce key; newly refuses a
  stream/abort naming no transaction, a stream/abort with `child_emits`, a stream with foreign steps, an abort carrying
  mutations (`toolTransaction.shape`).
- Unmigrated `dispatch_emit_inner`: `artifact_lane_commands` maps the emission to `AppendTransaction`,
  `Append + CommitTransaction`, `AbortTransaction`, `Apply{transaction}` or `AmendLast`; one row at the first tick, no row
  for an empty stream, an abort retires its row (`retire_displaced_document_rows`).
- Migrated route: `close_streamed_transaction_unit` (abort / empty commit through the non-suspending store calls — the
  one-page publisher's no-suspend contract holds), `open_transaction_admission_fault` (typed `toolTransaction.open` before a
  batch), `set_transaction_open(streams)`, `store_publication_fault` keeps the typed codes of batch refusals; the run loop
  calls `flush_announcements` beside `flush_published_apply_batch`.
- `historyEditBegin` answers `timeTravel.busy` while a transaction is open.

### G9 / design §16.6 (store half, `OS/🏪️store/🦀️.rs` region `🔖️DeferredReprojection`)

- **Resumable remote reprojection.** `defer_remote_replays(Some(operations))` turns the Report replay a remote history
  change needs (supersession, undo, redo of another replica) into a job: `admit_remote_transitions` validates the prospective
  fold at once (a broken change is still refused inside the ingest), then keeps the transitions outside the log
  (`PendingReprojection`) while `step_reprojection()` (also every `tick()`) replays one budget per call; the adoption is
  `reproject_replayed` of exactly the finished replay, so it equals the undeferred one. Progress `reprojection_progress()`
  `{done,total}`; `cancel_reprojection()` drops the running replay (store untouched, change kept, the next step restarts);
  the replay restarts whenever the content revision it started from changed (a local edit, another remote change). `None`
  (default) keeps the synchronous behaviour through the same path.
- **Resumable authored supersessions.** The interactive path already is the session replay (`begin_report_replay` +
  steps + `commit_finished_replay`). New `HistoryFinalization::Scope { alternative_id }` lets the runtime author the undo/redo
  of a finalize (today the synchronous `Supersede` in `⏪️time-travel/🦀️.rs::author_supersede`) the same resumable way. The
  `Supersede` command itself stays the synchronous command API (agents, text, hub).
- **Ring sizing.** The prefix-snapshot ring keeps ⌊√N⌋ projections between 8 and 16 (`prefix_ring_capacity`), stride
  ⌈N / capacity⌉: a `state_before` or replay start folds at most ≈√N edits; memory grows with √N, capped at twice the old 8.
- **65th edit.** Moot: the paged ledger landed. The typed `history.full` refusal stays for exhaustion.

### Tests, fixture, oracle

- `OS/🏪️store/🧪️tests/🧪️tool-transaction/🦀️.rs` (mounted from `🔬️unit`), 8 laws: corpus (every step: refusal, value,
  ledger, open transaction + its tail position, announcements, persisted value from pack AND text, shared/persisted log
  exclusion, ref on every committed op, live == replay); codec vectors; N appends + commit == one Apply (multi-op and one-op);
  abort leaves no trace (state, revision, order, ledgers, outcomes, log, pack bytes, text, announcements, next sequence); open
  transaction refuses every other command (`toolTransaction.open`) and `retire_command` retires every carried op (WitnessOp
  tally, now `pub(super)` in `🧪️supersede-replay`); two-replica convergence with later- and earlier-clocked remote edits
  landing under the open edit; batched streaming / closing / admission refusal / batch abort; the paged-ledger session's
  past-one-page law.
- `OS/🏪️store/🧫️fixtures/🧫️tool-transaction/🔣️.json` (6 cases, 32 steps, 3 command vectors) +
  `🧬️schema/🔣️tool-transaction/🔣️.json`; `🧪️tests/🧪️tool-transaction/🟦️.ts`: Ajv, independent TS twin folded with
  fast-json-patch, xstate lifecycle oracle, independent command encoder, fast-check (500 runs).
- `OS/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs`, 4 laws: every supersede-replay corpus case deferred (1 op/turn)
  adopts exactly what the undeferred replica adopts (and the corpus state, whose oracle is the fast-json-patch TS replay),
  showing the old history while waiting, progress monotonic; cancel after every step leaves the store untouched; a local
  edit or a further remote change restarts the replay; the ring keeps √N strided live prefixes.
- Plugin law `a_streamed_tool_transaction_is_one_row_and_its_abort_leaves_none` (`OS/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`).
- Supersede-replay retract law adapted to per-viewer heads (switching no longer authors a `Checkout`).
- API for S2-STROKES and CLOSURE: `📓️api-transaction-amend.md`.

### Commands and counts

| Command | Result |
|---|---|
| `bun test ./OS/🏪️store/🧪️tests/🧪️tool-transaction/🟦️.ts` | **5 pass, 0 fail** (3444 expects); negative control (abort keeps its ops): 2 fail as expected |
| `cargo check -p semio-framework-os-kernel --lib` (17:20, 17:23) | Finished, 0 errors |
| `cargo check -p semio-framework-plugin --lib` | Finished, 0 errors, no warning in §15 items |
| kernel lib tests, private target `target-nde-s2-w1g`, `-- tool_transaction_tests deferred_reprojection_tests` (17:35) | **12 passed, 0 failed** |
| kernel per-test runner (`🧪️w1-g/run-each-test.sh`, `os_store:: os_vcs:: os_spr::`) (17:26) | 889 ok / 21 FAIL, none from this work: durable-group ×15 (`wire value materialization exceeds max_total_alloc` — peer `🌱️value/🛬️decode` + `🎒️pack` edits after 16:41; 2 of them already failed at 16:41 on `Edit.line` canonical bytes), `os_spr::history::fold_falls_back…` (per-viewer fold), `sync::fixtures_replay_matches_expected_events` (`.ops` edit line format, per-viewer), supersede-replay ×2 (`assert_document_text_round_trip`: `.ops` does not persist the per-viewer head, `REC_VIEWER` is spr-only — PER-VIEWER-ALTERNATIVE-HEAD), and the ring law (fixed since: 12/12 above) |
| plugin lib per test, `RUST_MIN_STACK=268435456` (17:48) | **940 ok / 12 FAIL**, none from this work: `merge_ui_values` ×4, `tool_run` ×4, `window_kits` ×2, `command_ingress_terminal` (baseline), `history_label_reload` (W2-A's new law: `renameDescribed` has no manifest declaration). Both transaction laws and the publisher contract pass |
| `cargo test -p semio-framework-replication --lib` (FU4) | **316 passed, 0 failed** (the FU4 failure `envelope_transaction_round_trips…` is gone) |
| replication TS `bun ./📜️script.ts test` (FU4) | blocked: `Explicit Vitest policy required` (peer change in `🏃️process/🧪️testing/🧪️vitest`) |

### Open items / coordinator

- **S2-W2A (runtime adoption of G9):** call `store.defer_remote_replays(Some(budget))` on the document store of a
  `VcsArtifactApp`; drive `step_reprojection()` per reactor turn (≤ 4 ms, like `step_time_travel_replay`); show
  `reprojection_progress()` in the history band (a remote change replaying) with Cancel → `cancel_reprojection()`; move
  `author_supersede` (undo/redo of a finalize) to `begin_report_replay` + steps + `commit_finished_replay(…,
  HistoryFinalization::Scope { alternative_id: active line } | Overwrite)`.
- **Notice owners (S2-W2A / W2-B / W2-C):** en/de notices for `toolTransaction.open` ("A tool is still recording — finish or
  cancel it first." / "Ein Werkzeug zeichnet noch auf — zuerst abschliessen oder abbrechen.") and `history.full` ("This
  document's history is full ({n} edits)." / "Der Verlauf dieses Dokuments ist voll ({n} Bearbeitungen).").
- **PER-VIEWER-ALTERNATIVE-HEAD overlap:** its `Edit.line`/`HistoryLog.viewer_*`/`.ops` format changes break
  `assert_document_text_round_trip` for non-trunk heads and two kernel fixtures; my retract law was adapted to its
  "switching authors no Checkout". §15/G9 touch `ingest_remote` (wrapper), `admit_remote_transitions` and `tick` only.
- `AmendLast` has the same tick-relative `op_index` defect for amended messages; CLOSURE deletes `AmendLast`, so it is left.
- Hub re-verification of FU4 (`os-hub` bin, `semio-hub --lib`): not rerun yet (build budget); next.
- No descriptor regeneration needed (no new verbs). The runtime change needs the next activation to reach the shells.

### Update 21:55 (after the 18:00–21:30 usage cut)

- Re-read: every §15/G9 edit is intact in `🏪️store/🦀️.rs` (peer write 20:54), `🔌️plugin/🦀️.rs` (19:43) and `🌿️vcs/🦀️.rs` (19:00).
- **G9 checklist:** prefix ring sizing done (⌊√N⌋ in [8, 16], law `the_prefix_ring_grows_with_the_square_root_of_the_history`
  passed 17:35); 65th-edit refusal: the paged ledger landed, so the 65th edit is admitted (law
  `a_history_past_one_page_keeps_admitting_edits` passes) and exhaustion answers the typed `history.full`; notice texts
  proposed to S2-W2A above. Resumable remote reprojection + cancel + progress: done (4 laws). Runtime adoption: S2-W2A is on
  it (`HistoryPatch.remote_replay` in flight at 21:47).
- **Blocked by PER-VIEWER-ALTERNATIVE-HEAD (not ours, not fixed here):**
  `supersede_replay_tests::finished_replays_commit_atomically_and_refuse_stale_or_blocking_ones` and
  `…::scoped_supersessions_follow_their_alternative_and_unscoped_ones_every_alternative` — `.ops` does not persist the
  viewer head (`REC_VIEWER` is spr-only), so `assert_document_text_round_trip` fails on a non-trunk head.
- **Reruns at 21:37–21:51 blocked by peers in flight:**
  - kernel lib-test target: E0509 `cannot move out of type MapMutation, which implements Drop` at the `FromValue` derive
    of the unchanged fixture `🏪️store/🧵️canonical-edit/🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs:22` — caused by the peer's
    uncommitted `🌱️value/🦀️.rs`, `🌱️value/🛬️decode/🦀️.rs` and `🗣️dsl/✨️derive/🦀️.rs` edits (intrinsic bytes). Last green run
    of my laws: 17:35, 12/12.
  - FU4 hub (`cargo test --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml -p semio-hub --bin os-hub`): the plugin lib
    fails E0063 `missing field remote_replay in HistoryPatch` (S2-W2A's G9 adoption in flight).
- FU4 status otherwise: replication 316/0 (the FU4 failure is gone); the kernel sync retract law and the rebootstrap-control
  law pass; replication TS is blocked by the peer vitest policy requirement.
- **Store TS oracles are a permanent target now:** `bun ./📜️script.ts test-store-oracles` (os TS package, nx target
  `test-store-oracles` in `💻️os/📦️packages/🟦️typescript/📋️project.json`) runs the supersede-replay and tool-transaction
  oracles: **7 pass, 0 fail** (3464 expects). The coordinator's central launch.json regeneration should pick the new target up.
- `verify taxonomy report --scope` on `🧪️tests/🧪️tool-transaction`, `🧪️tests/🧪️deferred-reprojection`,
  `🧫️fixtures/🧫️tool-transaction`, `🧬️schema/🔣️tool-transaction`: clean=true errors=0 warnings=0 ×4.

### Update 02:50 (after the 23:00–02:30 usage cut; fleet rule 26 cargo hold active)

- Re-read: all §15 / G9 / oracle-wiring edits intact (store written by peers 02:15, plugin 02:40). S2-W2A has started the G9
  runtime adoption (`defer_remote_replays` used in `🔌️plugin/🦀️.rs`).
- `cargo check -p semio-framework-plugin --lib` (23:14–23:19, before the cut): **Finished**, 0 errors (S2-W2A's
  `remote_replay` had settled).
- The kernel lib-test blocker (`MapMutation` E0509) was fixed by its peer at 22:45 (`#[value(retire_with = "std::mem::drop")]`).
- Pending until "hold lifted": rerun of the kernel laws + per-test suite, the plugin suite, and the FU4 hub bin laws.

## Session 3 — 2026-10-02

Successor S3-W1G, coordinator `⚪b7db773a…`. Focus (rule 29): owed verification → fixes in W1-G regions → §15 proof →
"§15 proven" to `main`. Scratch output: `🗑️generated/s3-w1g/`.

### Repair (rule 28)

- 10:56 re-read: every §15 / G9 item is intact — store regions `🔖️ToolTransactions` (`🏪️store/🦀️.rs` ~20068) and
  `🔖️DeferredReprojection` (~20201), `ArtifactCommand::{AppendTransaction, CommitTransaction, AbortTransaction}` with both
  codecs, `holds_committed_edit`, `set_transaction_open`; plugin `TransactionPhase`, `Emit::{stream,commit,abort}_transaction`,
  `tool_transaction_shape_fault` (`🔌️plugin/🦀️.rs` ~24126) and its two call sites, `BoundedStoreInitializationPhase::FoldSupersessions`.
  Peers wrote `🏪️store/🦀️.rs` at 10:38 and `🔌️plugin/🦀️.rs` at 07:56; `🌿️vcs/🦀️.rs` and `📡️spr/📜️history/🦀️.rs` unchanged since
  session 2. No half-finished W1-G edit found.

### Verification (running log)

| When | Command | Result |
|---|---|---|
| 11:11 | `bun ./📜️script.ts test-store-oracles` (os TS package) | **7 pass, 0 fail** (3521 expects): supersede-replay + tool-transaction oracles (Ajv, fast-json-patch twin, xstate, fast-check) |
| 11:13 | replication TS `bun ./📜️script.ts test --reporter=verbose` (needs `SEMIO_VITEST_POLICY` = `repositoryVitestPolicyV1(cwd)`, `SEMIO_TEST_BUDGET_MS=400000` under load ~75) | **20 pass / 1 fail**. FU4 twins pass: "admits a genesis rebootstrap baseline naming no head edit and refuses a later one", "sends exact envelopes as a Commands frame keeping every HLC field exact"; supersede fold twin ×2, transaction ref ×2, replay report pass. Fail = peer fixture, see Open items (R-1) |
| 11:16 | os TS `bun ./📜️script.ts test 🏪️store/👷️worker` | **15 pass, 0 fail**, incl. "a mounted browser actor retracts a refused history step and rebuilds only for refused operations" and parity `ack-rejected-transition-retracts` |
| 11:18 | renderer-react `bun ./📜️script.ts test long 🧪️command-rejection ⏪️time-travel/🧪️tests/🧩️component` | **39 pass, 0 fail** (FU4 refused-step text en/de: "names the refused history step", band corpus) |
| 11:11–11:21 | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-w1g cargo test -p semio-framework-os-kernel --lib --no-run` | Finished (10 m 03 s), 0 errors |
| 11:23 | kernel lib tests `-- tool_transaction_tests deferred_reprojection_tests supersede_replay_tests dispatch_group_stamps_one_tool_transaction` | **35 passed / 2 failed** — the 2 are the PER-VIEWER-ALTERNATIVE-HEAD blockers (below). §15: 8/8 tool-transaction laws, G9 4/4 |
| 11:24 | per-test runner `🧪️w1-g/run-each-test.sh … os_store:: os_vcs:: os_spr::` | **809 ok / 19 FAIL**: durable-group ×15 (peer pack, R-2), `os_spr::history::tests::fold_falls_back…` (stale for per-viewer heads — fixed, below), supersede-replay ×2 (blocked, below). `sync::fixtures_replay_matches_expected_events` passes again |

| 11:56–12:29 | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-w1g cargo test -p semio-framework-plugin --lib --no-run` (compiled the kernel WITH the N17 + viewer store edits) | Finished (27 m), 0 errors |
| 12:30 | plugin lib, one process per test, `RUST_MIN_STACK=268435456` (959 tests) | **943 ok / 16 FAIL**. §15: `a_streamed_tool_transaction_is_one_row_and_its_abort_leaves_none` ok, `a_committed_tool_transaction_is_one_row…` ok; archive supersession / round-trip / reload laws ok; 35/37 time-travel laws ok. FAIL: baseline peers `merge_ui_values` ×4, `tool_run` ×5, `window_kits` ×2, `command_ingress_terminal` ×1; new peer `declarations::fixture::sqlite_snapshot_covers_*` ×2; S3-W2A in flight (test file + `⏪️time-travel/🦀️.rs` edited 12:24–12:29): `a_long_remote_history_change_replays_over_turns_and_pauses_on_cancel` ("a paused remote replay is no driver work": `has_pending_work` counts `patch_due`/ui-dirty right after the cancel verb) and `a_warning_an_edit_introduces_stays_visible_after_finalize_and_reload` (review body lacks "New since this edit") |
| 12:31 | kernel lib-test rebuild for the new laws | **blocked by a peer** mid-refactor: `🧬️schema/📇️registry/🦀️.rs:349` E0428 duplicate `ArtifactSchemaRegistry` (+27), `📡️replication/🎮️mutation/🦀️.rs:219` unresolved `semio_framework_schema_state` (schema-state crate extraction, files written 12:20–12:24) |

§15 status: **proven** (store 8/8, TS 7/7, plugin 2/2 on today's tree) — "§15 proven" sent to `main` 12:35.

### Fixes

- `OS/📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs` `fold_falls_back_to_positional_mutation_ids_and_zero_clock_without_meta`: under per-viewer
  heads a `Branch` registers the alternative and selects no replica's head (`fold_history_for`, `fold.alternative = None` on the
  trunk). The law now asserts both halves: the sample log without a viewer line folds to the trunk tip (`alternative == None`),
  and the same log viewed on `alt-1` (`viewer_line: Some("alt-1")`) folds to `alt-1` with `applied` / checkpoint chain as before.

### Coordinator message 11:40 (handled first)

1. PER-VIEWER-ALTERNATIVE-HEAD is closed → fix the two blocked supersede-replay laws properly.
2. N17 (P1, `📓️s3-gap.md`): `dry_run` and `reprojection_replay` replay the whole applied history synchronously → resumable
   jobs with progress + cancel (§16.6), API for S3-W2A, law with ≥ 200 mutations.

### The `.ops` text carries the viewer head (11:50; REVERTED 19:00, see "PER-VIEWER laws — corrected fix")

Decision: the text pair (`.dsl` + `.ops`) is a full persisted form ("replaces the JSON envelope as the canonical persisted
form", and the `.ops` mirror of a pack/spr pair), so it carries this replica's head exactly like `.spr`'s `REC_VIEWER`
(persisted local-only; check-in still clears it before printing the shared pair).
- Store (`🏪️store/🦀️.rs`): `OpsHeaderLine::Viewer { line, checkpoint }` (keyword `viewer`, both optional, last variant so no
  ordinal moves); `print_ops_log` prints it after `doc` unless the head is the canonical trunk tip; `replay_ops` admits one
  `viewer` line (a repeat is refused) and sets `active_alternative_id` (trunk id filtered) and `viewer_checkpoint_id` before
  `settle_parsed_envelope` folds.
- History twin (`📡️spr/📜️history/🦀️.rs`): `viewer_spec()` + `parse_ops_text`/`print_ops_text` carry `HistoryLog::viewer_line`
  / `viewer_checkpoint`; law `ops_text_round_trips_the_viewer_head` (no line on the trunk tip; line/checkpoint/both round-trip
  and fold to the right alternative; a repeat is refused).

### N17 — resumable local history steps (`🏪️store/🦀️.rs` region `🔖️DeferredReprojection`, `🌿️vcs/🦀️.rs`)

- `ArtifactStore::defer_local_replays(Some(operations))` (beside `defer_remote_replays`): a local history step whose Report
  replay is needed — interior undo/redo (`install_transitions`), checkout / alternative switch (`move_head`), and a
  supersession (finalize) authored without a finished replay (`author_supersession`, the `dry_run` path) — is admitted as
  `PendingLocalStep { envelopes, head, finalizing }` joined to the one `PendingReprojection` (remote transitions + at most one
  local step + its replay). The dispatch replays one budget and answers; `step_reprojection()` (every `tick()`) steps the
  rest; the adoption (`adopt_pending`) records remote + local transitions, moves the head, adopts the finished replay
  (`reproject_replayed`), seeds the DAG, records the replay report and announces the step's transitions (flushed at once).
  A finalizing step whose report blocks is refused `Rejected { policy: Normal }` with nothing recorded (remote transitions
  wait again). Steps that need no replay (tail undo, commit, branch) stay synchronous. The clock advances past the step's
  transitions at admission (no HLC reuse while it waits).
- While a local step waits: every history-moving command (`ArtifactCommand::moves_history`) and `commit_finished_replay` are
  refused `VcsError::HistoryReplaying` (fault code `history.replaying`); edits and remote changes restart its replay.
- `local_step_pending()`, `discard_local_step()` (zero trace: nothing was logged/announced/shown), `reprojection_progress()`
  covers both kinds, `cancel_reprojection()` keeps pausing (drops the replay only).
- `dry_run` stays the synchronous verdict of the command API when local replays are not deferred (agents, text, hub).
- Laws (`🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs`, region `🧪️DeferredLocalStepLaws`, 240-mutation history, budget 16,
  `CountedOp` counts every fold): `long_local_history_steps_replay_across_turns_and_adopt_what_an_undeferred_dispatch_adopts`
  (interior undo, redo, overwrite finalize, new-alternative finalize, switch back to trunk: ≤ budget + 1 folds per dispatch
  and per turn, nothing shown/logged while waiting, `history.replaying` for other history steps, adoption == undeferred),
  `a_waiting_local_step_is_discarded_with_zero_trace_and_an_edit_restarts_it`,
  `a_deferred_finalize_whose_report_blocks_is_refused_with_nothing_recorded`.

### Blocked (PER-VIEWER-ALTERNATIVE-HEAD ticket — documented, not worked around) — superseded by the fix above

- `supersede_replay_tests::finished_replays_commit_atomically_and_refuse_stale_or_blocking_ones` and
  `…::scoped_supersessions_follow_their_alternative_and_unscoped_ones_every_alternative` panic in
  `assert_document_text_round_trip` (`🏪️store/🦀️.rs:26699`, "document-text round trip lost durable history"): the `.ops` text
  form does not persist the viewer head (`HistoryLog.viewer_line/viewer_checkpoint` ride `REC_VIEWER`, `.spr` only —
  `📡️spr/📜️history/🦀️.rs:1640`), so a store sitting on a non-trunk alternative does not round-trip through text. The fix
  (a `.ops` viewer line or a text-form decision) belongs to PER-VIEWER-ALTERNATIVE-HEAD.

### Open items (routed)

- **R-1 (replication fixture, peer of PER-VIEWER-ALTERNATIVE-HEAD):** `📡️replication/🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json`
  case `trailing-flags-invalid` still encodes flags `0x04`, which became valid (`line`, bit 2) in the 10-01 14:31 auto-commit
  (`🟦️.ts:1491`, `🔗️causal/🦀️.rs:1161` accept `≤ 0b111`). TS decodes a `line` and fails `truncated` instead of `trailing-flags`;
  the Rust law passes only because it does not compare the reason. Fix: flags byte `08` (rawHex `…030405 08`) and make the Rust
  law compare `reason`.
- **R-2 (pack, peer of the `RecordSpecProducer`/materialization refactor):** 15 `os_store::component::durable_group::tests::*` fail
  `Codec("limit exceeded: wire value materialization exceeds max_total_alloc")` (`🎒️pack/🌱️value/🦀️.rs:1982`, uncommitted peer edit
  10-02 03:53): the new 64-byte slot accounting exceeds the durable group's `max_total_alloc = 162 000`
  (`🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2240,2344`). Not a W1-G change.

### Resume 18:40 (usage cut ~13:05, machine reboot ~17:00)

- Re-read: every Session-3 edit is intact (store regions, `.ops` viewer line, vcs `HistoryReplaying`, history twin, the three
  deferred laws, the plugin reload law `a_long_history_reloads_one_operation_per_initializer_step`). No half-finished edit:
  the 19-anchor store script applied atomically at 12:00 and `adopt_state` drops a waiting reprojection (12:40).
- Still in the tree on purpose until the bisect below has run: env-guarded `[DEBUG]` probes (`SEMIO_DEBUG_STORE_UNITS=1`,
  silent otherwise) in `begin_typed_apply_batch`, at the top of `advance_apply_batch`, in
  `take_returned_snapshot_read_retirement` and `ArtifactStore::maintenance_retirements_step`. Remove them after the run.
- 18:42 `cargo check -p semio-framework-os-kernel --lib`: **Finished, 0 errors**.

### PER-VIEWER laws — corrected fix (19:00)

The 11:50 `.ops` `viewer` line contradicted the closed PER-VIEWER-ALTERNATIVE-HEAD design. Its laws
`document_text_round_trips_with_an_active_alternative_and_a_quoted_description` and
`reload_replays_the_event_log_into_the_same_positions` pin "the ops text is the shared log and hydrates to the trunk".
Both failed with the viewer line. So:

- The `viewer` line is **reverted** in both twins (store `OpsHeaderLine`/`print_ops_log`/`replay_ops`; history
  `parse_ops_text`/`print_ops_text`, whose doc now says the text is the shared log). The history law is now
  `ops_text_is_the_shared_log_without_the_viewer_head`: a viewed log prints the trunk-tip text, and the text parses back to
  the trunk tip.
- The real defect was the oracle. `test_support::assert_document_text_round_trip` (`🏪️store/🦀️.rs`) compared the whole
  envelope even for a store viewing an alternative. It now asserts:
  - every edit (`id`, forwards, metadata) and every transition survives the text round trip;
  - the text hydrates to the canonical trunk tip;
  - full envelope + live snapshot equality holds when the store itself is on the trunk tip.
- `reprojection_progress()` now reports `done` 0 when an edit or another change made the waiting replay restart (it showed
  the stale replay's count until the next turn).
- The N17 law harness `settle`s displaced owners between commands, as a runtime maintenance turn does. `CountedOp` names
  its author (`author_id`) so one store holds another actor's edits, and it delegates `may_emit_foreign_steps`.

| When | Command | Result |
|---|---|---|
| 18:42 | `cargo check -p semio-framework-os-kernel --lib` | Finished, 0 errors |
| 19:00 | kernel lib tests (private target) `-- tool_transaction_tests deferred_reprojection_tests supersede_replay_tests dispatch_group_stamps_one_tool_transaction os_spr::history::tests` | **92 passed, 0 failed**. Includes `finished_replays_commit_atomically…` and `scoped_supersessions_follow_their_alternative…` (formerly blocked), the 3 N17 local-step laws, the 4 deferred-remote laws, the 8 §15 laws, `ops_text_is_the_shared_log…` and `fold_falls_back…` |
| 19:02 | per-test runner `os_store:: os_vcs:: os_spr::` | **820 ok / 16 FAIL**. All 16 are `durable_group` (peer pack materialization budget, R-2). Both PER-VIEWER text laws pass |

### Reload: linear edit validation in the retained initializer (19:35, `🔌️plugin/🦀️.rs` region of `BoundedStoreInitializationAuthority`)

`ValidateEditPair { left, right }` checked edit-id uniqueness pairwise, one O(1) step per pair: N²/2 steps per reload, which
is 28 680 for 240 edits and about 50 M for 10 000 (the paged ledger admits that). It is replaced by `ValidateEdit { index }`.
Each step bounds the id length and inserts it into `edit_ids` (a duplicate fails as before). That is N steps, and the set
is released once the phase completes. The refusal code is unchanged. Covered by the reload law and the archive-load laws
(WRITTEN, verification waits for TREE GREEN).

19:55: `FindApplied { position, scan }` and `FindRedo { position, scan }` also scanned the edit list from 0 for every
position, a second N² term. The validation pass now builds `edit_index: HashMap<String, usize>`, so `FindApplied { position }`
and `FindRedo { position }` find their edit in one step, and the map is released before `BuildCandidate`.

**Progress for S3-W2A** (asked directly, design `📓️api-stepped-document-load.md` §2.2):
- `ArtifactStoreInitializationAuthority::progress(&self) -> (u64, u64)` (default `(0, 0)`) gives (operations folded,
  operations to fold).
- On `BoundedStoreInitializationAuthority` it returns `(folded, fold_total)`: `fold_total` is the forwards of the applied
  edits, fixed at `CloneInitial`; `folded` counts every applied forward step.
- `ArtifactStoreInitializationJob::progress()` is `pub(crate)` and forwards to the authority.

The reload law (moved by S3-W2A to `🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs`, mounted in `app`) now also asserts:
- progress is monotonic and ends at `(240, 240)`;
- the step count is a constant per edit (< 16 · 240).

### O(document) publication regression (S3-PUZZLE, coordinator 12:50)

Baseline (ticket 26/09/02 wave B54, 09-13): translate and delete publish in **store=17** units on both Concrete Forest (1
object) and Nakagin (180 objects). Now: 22 vs 1590 (delete 23 vs 1630). Reading of the store side so far: every
`advance_apply_batch` phase (prepare 2 / fold 1 / item close 1 / cursor 1 / preflight 1 / publish 1) and the batch close are
O(1) units per admitted item, and `fold_batch_item` is O(item). The store units therefore scale either with the number of
items the batch admits (the tool emitting O(document) mutations) or with plugin-side units counted as `store` in
`publish_mounted_typed_operation_run` (interaction-write units, inline interaction verbs, early returns while
`reclaim_document_snapshot_read_returns` has not drained). The instrumented run decides which.

**Leading hypothesis (06:50, reading; NOT yet measured).** Commit `6f33e313da9` (09-15, ticket 26/09/13
INTERACTIVE-TOOLS-VISIBLE-PROCESS) landed after the B54 baseline (09-13). It added the gate
`if Artifact(_) && !self.reclaim_document_snapshot_read_returns(PUBLICATION_SNAPSHOT_READ_RECLAIM_STEPS)? { return Ok(()) }` to
`🔌️plugin/🦀️.rs` `publish_mounted_typed_operation_unit`, in front of EVERY artifact publication unit.

Why that gate would scale with the document:
- Every one-item publication reads its base through a `SnapshotRead` of the live root. After `Publishing` replaces `current`,
  that returned read is the last owner of the pre-edit root, and puzzle 3d's `apply` produces a whole new root.
- So the gate has to retire the entire previous document before the ladder may advance. It does so through the bounded
  value retirement (`ReturnedSnapshotReadRetirement` → `initial_snapshot_retirement_factory`), and every unit that returns
  early is counted as `store`.
- That is O(document) units per mutation: a small document pays about +5 (22 vs 17), Nakagin pays about +1570.

What the gate is for: it bounds a multi-item batch to the staged root plus one base, which was the 816-op fill OOM. A
one-item publication does not need it on its critical path.

Root fix to apply once confirmed (plugin region of S3-W2A, typed-operation publication; store side unchanged): drain
returned reads only down to a constant allowance. Either:
- before an item ≥ 2 of the same batch takes its next base (`ArtifactStoreBatchPublication` would expose
  `takes_next_base()`), or
- whenever more than one returned root is outstanding.

The rest retires in the maintenance rotation. Memory stays bounded by a constant number of roots, and the ladder becomes
size-independent again.

Probes for the confirming run (env `SEMIO_DEBUG_STORE_UNITS=1`, puzzle 3d `one_mutation_publishes…` / `b54_measures…` with
`--nocapture`):
- `store-batch begin/advance` (store ladder units);
- `store-read-return` (returned read taken);
- `store-read-return unique root retiring/retired` (a whole pre-edit root retired through the gate);
- `store-displaced-step`.

Confirmation shape: many `store` census units between `unique root retiring` and `retired`, while `store-batch advance`
stays at about 17.

Blocked at 06:33: the `✏️s` workspace does not compile (`semio-s-artifact-stdio-zip`/`-gltf`, peer DSL/`ValueError`
change), so puzzle 3d cannot be built.

### Resume 10-03 05:50 (cut ~21:00; coordinator: TREE GREEN core)

- Re-read: every edit intact. In the store: deferred/local-step regions, `assert_document_text_round_trip`,
  `reprojection_progress` restart, the 4 env-guarded `[DEBUG]` probes. In the plugin initializer: `ValidateEdit`,
  `edit_index`, `progress`, `fold_total`; 0 `ValidateEditPair` left. No interrupted edit.
- The three N17 local-step laws were already green at 19:00 (92/92). Nothing is owed on them except the plugin-side
  reload law.

### Browser host accepts a multi-turn cold-pair `loading` (coordinator 06:36, `📓️api-stepped-document-load.md` §8 (1))

- `🏪️store/👷️worker/🟦️.ts`: `transferColdPair` settles the LAST page through the new exported `settleColdPairLoading(first,
  status, cursor, poll, release, limit = COLD_PAIR_LOADING_TURN_LIMIT = 1 << 20)`.
  - While the guest answers `loading` (its whole-document archive load still folding), the host polls empty turns.
  - Every `loading` must name exactly the last page of this transfer, otherwise "invalid loading receipt".
  - A UI patch on a loading turn throws, as on page turns.
  - Every answer the host moves past is wiped.
  - Cancelling is `assertCurrent` throwing inside a poll; the guest then restores the previous document.
  - Only then does the host require `applied`; anything else throws with the status diagnostic.
- Law `👷️worker/🧪️tests/🧪️cold-pair-loading/🟦️.ts`, registered in the worker's vitest block, covers:
  - the language-agnostic corpus `👷️worker/🧫️fixtures/🧫️cold-pair-loading/🔣️.json` (7 cases: at once, across turns, then
    fault, other page, other transfer, cancelled, past the limit), checked against
    `👷️worker/🧬️schema/🔣️cold-pair-loading/🔣️.json` with Ajv;
  - a fast-check property (200 runs): n loading turns then applied/fault give n polls and n releases.
- 06:39 os TS `bun ./📜️script.ts test 🏪️store/👷️worker`: **17 pass, 0 fail**.
- 06:40 `tsc -p 🧪️s3-w1g-typecheck-cold-pair-loading.tsconfig.json` (worker + law, 1090 files): **0 errors**.
- S3-W2A told directly (guest contract: last-page cursor, no UI patch before `applied`). S3-W2C owns the wgpu equivalent.

### Status

In progress: O(document) bisect, plugin reload law + archive laws, FU4 hub bin, replication lib.
- 06:33: the `✏️s` workspace is red from peers. `semio-s-artifact-stdio-zip`/`-gltf` fail with `ValueError`/`MutationLeaf`
  errors (DSL extraction), so the puzzle 3d build for the bisect cannot link.
- 06:36: `semio-framework-os-kernel` is red again from a peer. `🚪️io/🦀️.rs:2406…` fails with `IoError` having no field
  `message` (IoError reshape), which blocks the plugin lib-test build.

### Session 3 summary (10-03 06:55)

**Verified (ran, saw pass):**

| Check | Result |
|---|---|
| Kernel lib laws (§15 ×8, deferred-remote ×4, N17 local-step ×3, supersede-replay incl. both PER-VIEWER laws, history twin) | 92/92 |
| Kernel per-test `os_store:: os_vcs:: os_spr::` | 820 ok / 16 FAIL, all peer `durable_group` (R-2) |
| Plugin lib per-test, 10-02 12:30, before the initializer change | 943 ok / 16 FAIL. §15 2/2 ok; FAILs are peers or S3-W2A in flight |
| TS store oracles | 7/7 |
| TS replication | 20/21 (R-1 peer fixture); FU4 twins ok |
| TS os worker | 17/17, incl. the new cold-pair-loading law |
| TS React refusal / band | 39/39 |
| tsc worker + law | 0 errors |
| `cargo check -p semio-framework-os-kernel --lib`, 10-02 18:42 | green |

**Written, not yet verified** (a peer break blocks each one):
- plugin initializer: linear `ValidateEdit` + `edit_index` lookups + `progress()`;
- reload law `🧪️bounded-reload` (moved there by S3-W2A, now also asserts progress and linear steps);
- the O(document) bisect run (probes in place);
- FU4 hub bin laws;
- replication lib.

**Blockers right now:**
- kernel red: `🚪️io/🦀️.rs:2406` `IoError` has no field `message` (peer IoError reshape);
- `✏️s` workspace red: `semio-s-artifact-stdio-zip`/`-gltf` `ValueError`/`MutationLeaf` (peer DSL extraction);
- replication lib-test: `🧾️wire/🧪️tests/🔬️unit/🦀️.rs:434` `artifact_inference_catalog_len` missing (peer).

**Open (mine):**
- run the bisect and apply the root fix (hypothesis above, plugin region, coordinate with S3-W2A);
- remove the `[DEBUG]` probes (4 store sites + 2 in `ReturnedSnapshotReadRetirement`) right after that run;
- rerun the plugin suite (reload law, archive laws, initializer);
- FU4 hub bin; replication lib;
- the ring-stride prefix fold at replay start (≤ ⌈N/16⌉ edits for N > 256, see `📓️api-deferred-history-replays.md` §4).

**Routed:**
- R-1 replication fixture `trailing-flags-invalid` (flags `04` → `08`);
- R-2 pack materialization budget (`durable_group` ×16);
- S3-W2A runtime adoption of N17 (`📓️api-deferred-history-replays.md` §3);
- S3-W2C wgpu cold-pair `loading`.

**06:46 update.** S3-W2A applied the root fix in its region (`🔌️plugin/🦀️.rs` ≈31605). The new constant is
`PUBLICATION_RETURNED_ROOT_ALLOWANCE = 1`, and the reclaim gate now drains only when
`returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE`:
- a one-item publication no longer retires the whole pre-edit root on its ladder (the maintenance rotation retires it);
- a multi-item batch drains before item 2, which keeps the 816-op fill OOM bound.

Reviewed: memory stays bounded by a constant number of roots (staged + base + ≤ 2 returned + the one retiring in the pump).
Expected: translate/delete back to store ≈ 17 on both sizes, since the small document's +5 was the same gate.

Confirming run still owed: the puzzle 3d `one_mutation_publishes…` and `b54_measures…` with `SEMIO_DEBUG_STORE_UNITS=1`,
plus the fill memory law. It is blocked because the kernel is still red at 06:46 (`🚪️io/🦀️.rs:2415` `IoError.message`, peer).

**Coordinator actions:** none for W1-G (no descriptor, launch or schema regeneration; no new verbs). The central launch.json
regeneration should keep the `test-store-oracles` row (session 2).

**07:00 update.**
- S3-W2A put the guest cold-pair multi-turn `Loading` on disk (`⚛️reactor/🔄️turn/🦀️.rs` `step_cold_pair_document_load`,
  `⚛️reactor/📥️cold-pair/🦀️.rs`). Every loading answer carries the last-page cursor, there is no patch before `applied`, and
  a guest-side cancel answers Fault `cold-pair.load-cancelled`. That matches `settleColdPairLoading`.
- Told S3-W2A: a host-side cancel sends nothing, so the guest must cancel a stale load when it sees a page with a newer
  `transfer_generation` or when the instance closes.
- Kernel still red at 07:00: 1 error, `🧬️semio/🦀️.rs:4` unresolved `crate::os_dsl::ValueRefusalKind` (peer DSL extraction).
  The publication-gate measurement waits for TREE GREEN.

**10:50 update (after a usage cut at ~07:05).**
- The browser host now resends cold pages on backpressure. S3-W2A's stale-load fix answers `Backpressure(older owner's
  cursor)` to the first page of a new transfer while the old load is cancelled, and the host used to fail that transfer
  ("invalid page receipt").
- `🏪️store/👷️worker/🟦️.ts`: every page goes through the new exported `retryColdPairBackpressure`.
  - A same-lifetime backpressure releases the answer, gives the guest one empty turn and sends a fresh copy of the page.
  - A foreign lifetime is refused, bounded by `COLD_PAIR_LOADING_TURN_LIMIT`.
  - Page bytes are zeroed per attempt.
- Corpus and schema `cold-pair-loading` gain a `backpressure` section (5 cases); the law adds a fast-check property.
- os TS `test 🏪️store/👷️worker`: **18 pass, 0 fail**. tsc worker + law: **0 errors**.
- S3-W2A reports the guest side (`Loading`, cancel, supersede, checkpoint restore over the archive load) compiles, plugin
  `--lib` native + wasm32 at 10:55. The publication-gate measurement still needs puzzle 3d to build.

### Audit majors, 10-03 (from `📓️audit-s3-core.md` §2)

**W1G-1. Probes removed.** All six `SEMIO_DEBUG_STORE_UNITS` sites are deleted (4 in the store ladder, 2 in
`ReturnedSnapshotReadRetirement`); `grep SEMIO_DEBUG_STORE_UNITS` finds 0. The bisect plan is in "O(document) publication
regression" above. Its run (`one_mutation_publishes…`, `b54_measures…` and the 816-op fill law on puzzle 3d, streamed
ticks for W2A-5) still waits for `✏️s` to build (stdio peer).

**W1G-3. O(change) per mutation (§20.14).** Done in `🏪️store/🦀️.rs`, `🌿️vcs/🦀️.rs` and the durable-group adoption.
- *Lookups.*
  - `ArtifactHistoryLedger` has a seek hint (`find_near`, `find_near_position`, O(1) forward `get`).
  - 26 store lookups use it.
  - `HistoryPageIter` has an O(1) `nth`, and `HistoryPageStack::iter_from` seeks to a position, so `cloned_from` and
    `skip` no longer walk the prefix.
- *Accessors for the runtime (W2A-1).* All are O(that edit):
  - `applied_edit_mutations(position)`;
  - `applied_edit_outcomes(position)`;
  - `applied_edit_position(edit_id)` (searched from the tail, so O(rows read) for the newest rows).
  - `mutation_ops`/`mutation_outcomes` are now linear loops over these.
- *No fold on the tail.* `fold_frontier` holds the greatest event key of the log while the live cursor is the proven fold
  of that log.
  - It is set at the end of every full adoption (`adopt_reprojection`).
  - It is cleared by every primitive that changes the log or the cursor: edit insert, transition insert, the
    applied/redo/checkpoint/supersession replacements, reload, durable-group adoption, and the start of every reproject.
  - `Apply` and a transaction's opening append whose edit sorts after the frontier (at the line's tip, no step waiting)
    skip `reproject()`. They apply exactly the fold's rule instead: keep only the redo entries the edit's actor did not
    author (`adopt_folded_tail`).
  - `AppendTransaction` ticks skip `reproject()` while the fold is live: an amend changes no fold input.
- *Mirrors.* `StackMirrorDirt` holds the first applied/redo position whose id changed. It is set by
  `replace_applied/redo_edit_ids_retained` (shared prefix), `push_applied_edit_id`, reload (`ALL`) and group/origin
  restamps (`restamp_applied_tail`).
  - `sync_cursor` rewrites only that suffix in place. It used to clone both stacks and retire the old cursor each bump,
    which was O(E) owners queued per mutation.
  - `reconcile_stack` starts from it instead of SHA-256 hashing every id.
- *Prefix ring.* Pruning no longer recomputes `forward_prefix_digests` over the whole history.
  - `prefix_ring_dirty_from` is set by applied replacement, replay start and a regrown tail record.
  - Pruning evicts entries past it in O(ring).
  - `retain_prefix_snapshots` prunes before it admits.
  - Readers still verify digests.
- *Test-build oracle (`assert_mirrors_are_live`, histories ≤ 256).* Every bump checks:
  - the persisted cursor equals the live stacks;
  - each revision record equals a from-scratch rebuild (first stale index reported);
  - the content revision equals the rebuilt one;
  - a claimed frontier equals `fold_envelope_history`.

  The oracle found two pre-existing staleness cases:
  - `stamp_tail_group_id`/`stamp_tail_origin` change the tail edit after its record was computed. Fixed with
    `restamp_applied_tail`.
  - One law renumbered a live store's edits. It now renumbers a pack round-trip copy.
- *Law* `🧪️tests/⚡️hot-path` + corpus `🧫️fixtures/⚡️hot-path` + schema `🧬️schema/⚡️hot-path`, with a test-only
  `hot_path_census` (folds, prefix digests, mirrored ids, rebuilt records). Measured windows:
  - 1000 Applies after 1000 edits: 0 folds, 0 digests, 1000/1000 mirrored/rebuilt;
  - 400 ticks + commit after 2000 edits: 0/0/1/401;
  - 3 ticks + commit + 200 Applies after 2000 edits: 0/0/201/204.
- *Not O(change) yet:* undo/redo/checkout/supersede still fold the log once per step. They are history steps, not
  mutations, and a long replay is already deferred (N17). The batched publication ladder never folded and still clears
  the frontier, so the first dispatch after a batched edit folds once.

**Verification 11:2x–11:4x** (gated, `target-nde-s3-w1g`):
- `cargo check -p semio-framework-os-kernel --lib`: 0 errors. A transient red at ~11:20 was my in-flight wave, completed
  at once.
- Kernel lib, all tests: **1254 ok / 2 FAIL**. Both failures are peers':
  - `outbound_announcement…exactly_once` asserts `steps >= 22`, but S3-CLOSURE's coalesce removal cut the fixture to 14
    steps (`🧪️tests/📤️outbound-announcement/🦀️.rs` ≈144);
  - `sqlite_snapshot…native_input_retained_materialization…` panics at
    `📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:62`, where the native sqlite work is uncommitted
    peer churn.
- `retained_clone_tests` overflow the default 2 MiB test stack (pre-existing, not mine). With `RUST_MIN_STACK=256MiB`
  they pass 11/11.
- `close_test_store`'s hang guard went from 65 536 to 2²⁴ steps, because a 2000-edit store needs more single-item close
  steps.

## Session 4 — 2026-10-04

Successor S4-STORE (Opus), coordinator `⚪487b04ad…`. Brief: `📓️s4-resume.md` §7 S3-W1G, §4 D7/D8/D10/D22. Scratch output:
`🗑️generated/s4-store/`.

### Repair (rule 34, 02:1x)

- `git diff HEAD --stat` over `🏪️store`, `📡️spr/📜️history`, `🌿️vcs`: 150 files, all staged; the only unstaged file is the
  peer's `📜️space-history/…/🧪️tests/🪶️sqlite/🟦️.ts` (Codex sqlite work, 02:01). Files newer than the session-3 tail are peer
  waves (value/DSL extraction 23:25, sqlite/child owner 00:27–01:30); no half-finished W1G edit found (checked by compiling, below).
- `SEMIO_DEBUG_STORE_UNITS`: 0 sites in store/spr/vcs/plugin (W1G-1 probes stay deleted).
- `cargo check -p semio-framework-os-kernel --lib` (shared target, 02:19–02:21): **0 errors** (587 warnings, peer files).

### W1G-10 host cold-pair wait: wall deadline + backoff (02:30–02:45, `🏪️store/👷️worker/🟦️.ts`)

- `COLD_PAIR_LOADING_TURN_LIMIT` is replaced by `ColdPairWaitPolicy { deadlineMs, maximumTurns, eagerTurns, backoffMs,
  maximumBackoffMs, now, sleep }` and `COLD_PAIR_WAIT_POLICY` (5 min wall, turn cap `1<<20` as the secondary bound,
  4096 eager turns ≈ 20 s of continuous folding, then 1 → 64 ms exponential backoff). `coldPairWaitDelay(policy, turn)` is the
  closed form; `coldPairWaitTurn` refuses at the cap or the deadline and never sleeps past the deadline.
  `settleColdPairLoading` and `retryColdPairBackpressure` take the policy instead of a turn limit; errors name the bound
  (`still loading after 300000 ms (n turns)` / `… after n turns`, `cold page still refused after … attempts`).
- Corpus `👷️worker/🧫️fixtures/🧫️cold-pair-loading/🔣️.json`: `limit` → `policy`, per-case `policy` overrides and fake-clock
  `turnMs`, every expectation pins `sleeps`; new cases `backoff-after-the-eager-turns`, `loading-past-the-wall-deadline`,
  `backoff-never-sleeps-past-the-deadline`, `resends-back-off-after-the-eager-turns`, `refused-past-the-wall-deadline` (17 cases).
  Schema `👷️worker/🧬️schema/🔣️cold-pair-loading/🔣️.json` updated (Ajv strict).
- Law `👷️worker/🧪️tests/🧪️cold-pair-loading/🟦️.ts`: fake clock; new fast-check property (200 runs) — the pauses follow the
  closed-form schedule and a stuck guest is refused within one turn of the wall deadline.
- Verified: os TS `bun ./📜️script.ts test 🏪️store/👷️worker` **19 passed, 0 failed** (was 18; +1 property); `tsc -p
  🧪️s3-w1g-typecheck-cold-pair-loading.tsconfig.json` **0 errors**.

### D7 hold (coordinator relay 02:50)

S4-PUZZLE found the publication unit census (`🔌️plugin/🦀️.rs` region `📊️PublicationUnitCensus`) used process-wide atomics, so
the session-3 "22 vs 1590" may be contamination from parallel laws; the census is now thread_local and PUZZLE re-measures once
puzzle 3d compiles. No D7 root fix until those numbers arrive.

### Verification and store fixes (02:2x–03:15)

**Full kernel lib** (`RUST_MIN_STACK=268435456 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-store cargo test -p
semio-framework-os-kernel --lib`, compiled 02:2x before the edits below): **1190 passed / 7 failed** — all 7 are peers':
`os_dsl … controlled_refusal` (refusal text `value.refused field`), `os_pack … schema_storage_tests::schema_hash…`,
`durable_group ×2` (canonical hash / unbound bytes changed by the value refactor, R-2), `sqlite_snapshot_space_history…preflight`
and `native_encoding_tests::sqlite…` (Codex sqlite work), `interaction_state_pack_matches_first_party_value_and_json_oracle`
(pack bytes of the value refactor). Log: `🗑️generated/s4-store/kernel-lib-all-1.txt`.

**W1G-7** (`🏪️store/🦀️.rs` region `🔖️DeferredReprojection`): `adopt_pending`'s projection-failure arm now takes the remote
transitions back out of the log (`extract_if`) and hands them to `refuse_local_step`, so a refused local step leaves its remote
transitions waiting (they were dropped). `defer_local_step`'s synchronous adoption sets `last_projection_cause = Replay` like
`step_reprojection`.

**W1G-8** (one identity rule): `advance_apply_batch` stamps `stamp_primary_operation_identity` on the staged edit before its
revision digest (`PreflightingCommit`, not amending, not opening a transaction) and on the open edit when a batch closes its
transaction; a one-mutation batched gesture is now named after its edit like `Apply` and `CommitTransaction`, so a remote
replica's one-forward edit resolves transitions naming it. Unit laws updated: `artifact_store_batch_publication_of_one_mutation…`
(one-item id = edit id; a two-item gesture keeps `#0/#1`), `…stamped_publication…` (unstamped one-item = edit id).

**W1G-9**: law `supersede_replay_tests::a_viewer_head_round_trips_through_the_spr_pack` (alternative + explicit checkpoint
survive `.spr` `REC_VIEWER`, reload shows the same projection/supersessions, `.ops` hydrates to the trunk tip);
`test_support::assert_document_pack_round_trip` now asserts the parsed pack's viewer head and compares pack vs text only on the
trunk tip.

**W1G-5** (three §15 scenarios, `🧪️tests/🧪️tool-transaction/🦀️.rs`):
- `an_abort_restores_the_redo_stack_and_the_cursor_on_both_routes` (command + batched): redo stack, persisted cursor, order,
  projection, revision restored; redo/undo step as before. It found a trace: an abort left the open's local actor in place
  (batched authority actor ≠ `Apply` actor → `NothingToRedo`). Fix: `OpenToolTransaction.previous_actor` (set on both open
  routes), restored by `abort_open_transaction`.
- `remote_edits_that_landed_under_an_open_transaction_survive_its_abort` (later + earlier clock; equals a replica of the shared
  log, nothing announced, persisted forms agree, live = replay).
- `a_transaction_streaming_while_a_local_step_waits_never_starves_it`: was a real starvation (every tick restarted the waiting
  replay). Fix: `continue_pending_replay_across_append` (command `append_transaction` and the batched amend publish) re-keys a
  waiting replay started against the pre-tick revision and `EditReplay::absorb_tail_growth` counts the appended ops (the grown
  tail edit is read by id; cached ids reset; the convergence probe, whose head predates the tick, is dropped).

Verified 03:12: `cargo test -p semio-framework-os-kernel --lib -- tool_transaction_tests deferred_reprojection_tests
supersede_replay_tests dispatch_group_stamps_one_tool_transaction os_spr::history::tests artifact_store_batch_publication
artifact_store_stamped_publication hot_path_tests` **103 passed, 0 failed** (`kernel-filters-3.txt`).
Rule 39 check 03:15: `cargo check -p semio-framework-os-kernel --lib` RED from a peer only — `🧰️framework/🔨️modules/🚪️io/🦀️.rs`
2089/2095/2719/2736 E0308 (`IoOutcome<(Cow<[u8]>, ArchiveChildren)>` vs tuple, mtime 03:10); reported to `main`.

### Owed runs, continued (03:15–03:25)

- os TS `bun ./📜️script.ts test-store-oracles`: **7 pass, 0 fail** (3708 expects).
- `cargo test -p semio-framework-replication --lib`: **264 passed, 0 failed**.
- Replication TS (`SEMIO_VITEST_POLICY=repositoryVitestPolicyV1(cwd) SEMIO_TEST_BUDGET_MS=400000 bun ./📜️script.ts test
  --reporter=verbose`): first **20/21** (R-1), then **21/21** after the fix below.
- **R-1 closed**: `🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json` case `trailing-flags-invalid` used flags `04`, which
  is the `line` flag the per-viewer wave added (bit 2), so the TS decoder read a line and answered `truncated`; the first invalid
  flag is `08`. (The Rust twin accepts any `Malformed` for a malformed row; tightening it to the reason showed the Rust varint
  errors use another vocabulary — `Pack(Malformed{varint…})` — so it stays as it was; noted for a separate twin-alignment task.)

### D22 — O(change) undo/redo, stepped replay prefix (03:20–04:15, `🏪️store/🦀️.rs`)

**Tail steps without folding the log.** `install_transitions` first asks `tail_step(envelope)`: a single locally authored
`Revert`/`Reinstate` changes nothing but the applied tail when the fold is live (`fold_is_live`, not detached, line tip, no
open transaction), the transition sorts after every event, and it names exactly the operations of the applied tail (undo) or
of the redo top a tail step just moved there with nothing happening since (redo: `tail_step_generation == generation`), authored
by the edit's own actor — then the fold rule moves exactly that edit between the stacks. `adopt_tail_step` adopts it without
`fold_live_log`: undo projects through the tail snapshot or `live_prefix_state` (nearest pruned ring entry or genesis folded
forward, recording the ring on its stride with digests chained from the base's digest — no whole-history hashing), redo folds
the edit's effective forwards onto the head and keeps the head as the tail snapshot; the edit's inverse and messages stand
(their base is unchanged). New primitives `pop_applied_edit_id`, `push_redo_edit_id`, `pop_redo_edit_id` keep
`StackMirrorDirt`/`prefix_ring_dirty_from` exact. Retirement order matters (first run caught it: a redo that replaced the head
before installing the tail snapshot left an aliased root in an exact retirement → store close stuck): undo replaces the head then
clears the cache; redo installs the cache then replaces the head. The test-build oracle `assert_mirrors_are_live` (≤ 256 edits)
re-checks every tail step against the full fold; `assert_prefix_entry_is_live` (test builds) re-checks every ring entry the
digest-free readers trust.

**Stepped replay prefix** (`📓️api-deferred-history-replays.md` §4, the "ring-stride prefix fold"): `EditReplay::new_after_prefix`
starts from the live base nearest before `from` (`live_base`: head, tail snapshot, pruned ring entry, genesis) and folds the
rest of the prefix itself inside `step` — under the same deadline and recording the ring — so `report_replay` (finalize dry run,
reprojection, deferred steps, `begin_report_replay`) no longer folds up to a stride and hashes the whole history synchronously
before its first budget. The convergence probe uses the pruned ring directly (no `forward_prefix_digests`); a tail-only removal
(`adopt_reprojection`) projects through `live_prefix_state` and `admit_prefix_snapshots` (`prefix_projection` deleted;
`retain_prefix_snapshots` split into digest + `admit_prefix_snapshots`). Every caller's `from` lies at or before the first
position where the replayed order or inputs leave the live history (`replay_window`), so the live prefix is the replayed one.
`forward_prefix_digests` now composes `forward_prefix_genesis` + `forward_prefix_step` (same digests).

**Law** `⚡️hot-path` corpus vector `undo-redo-stream` (schema kinds `undo`/`redo`): 50 undos + 50 redos after 2000 edits —
**folds 0, digested 1875** (one ring seeding: the second undo folds from the genesis past the tail snapshot, recording 15 stride
snapshots; every later undo folds ≤ one stride from the ring and hashes nothing), mirrored 100, rebuilt 100; the law also checks
live = replay at the end.

**Peer-change test pins** (CLOSURE-4 made batch over-declaration refuse with the typed `VcsError::TooLarge`): updated
`artifact_store_batch_fold_refuses_a_one_work_item_declaration…` (`TooLarge { rows: 2, capacity: 1 }`) and
`artifact_store_batch_commit_refuses_an_under_declared_multi_item_gesture…` (`TooLarge { rows: 6, capacity: 3 }`).

Still O(history) per interior step: `fold_live_log` (any transition other than a tail step), `replay_window`'s
`superseded_positions`, and the digest of `retain_prefix_snapshots` at replay adoption (one pass per adopted replay, which is
itself O(suffix)).
- Verified 04:27 (`kernel-lib-all-4.txt`): full kernel lib **1197 passed / 7 failed**, all 7 peers' (dsl refusal text; channel
  `document_identity_wire…v20_fixture` 20 vs 21 = S4-BUMP's channel bump; durable_group ×2; sqlite ×2; interaction-state pack).
  The hot-path law (incl. `undo-redo-stream`) and the two `TooLarge` pins pass. 06:45–07:03 (after the cut): `cargo check -p
  semio-framework-os-kernel --lib` native **0 errors** and `--target wasm32-wasip2` **0 errors**.

### Resume 06:45 (cut 04:15–06:45) and `[DEBUG]` removal

- The stepped-prefix patch had already landed at 04:12 (kernel check green, full lib 04:27 above); nothing was half-written.
- `[DEBUG]` removed from my trees: `🌿️vcs/🧪️tests/🔬️unit/🦀️.rs` (2 `eprintln!`), `🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs`
  (1 `println!`), `📡️spr/🎮️command/🧪️tests/🗣️verb-vocabulary/🟦️.ts` (3 summary lines keep their text without the prefix),
  `🏪️store/🧷️assembly/⏳️lifetime/🧫️fixtures/🧑️client/{explicit-release,valid-order}/🦀️.rs` (fixture output; the law matches the
  substring after the prefix). Left on purpose (active Codex sqlite work, 01:28–02:01 today, not mine):
  `🏪️store/📜️space-history/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/{🦀️.rs:118,429, 🟦️.ts:43,81,97,121}` — coordinator: the owner removes them.

### W1G-2 / W1G-4 — one retained-initializer mechanism (07:00–08:30)

**Finding.** The 8 plugin-owned store initializers (writer, gismap, jack, generation2d, generation3d, process3d, drawing,
raster) folded the ORIGINAL forwards: a superseded or withdrawn operation reloaded with its old input through the plugin's own
initializer (design §3.1 "effective forwards everywhere" — the archive load / persisted replacement path of those apps). The
`ValidateEditPair` copies themselves were already gone (shared `ArtifactStoreInitializationEditIndex::admit`, coverage gate
names updated, 10-03 ~11:57).

**Store (`🏪️store/🦀️.rs`, initializer runtime):** `ArtifactStoreInitializationRuntime::fold_supersession_step(envelope,
transition)` — one transition per job step, `(hlc, id)` order checked, `Supersede` inputs scoped to the document or this
replica's line become effective (later wins), a target no operation carries matches nothing (the store's first full fold refuses
such a log, as before W1G); `effective_forward(edit, index, schema)` — the effective input for initializers that apply ops
themselves; `fold_forward(edit, index, schema, max_bytes)` — keep-and-record fold of the effective input
(`ArtifactStoreInitializationForward::{Exhausted, Folded{displaced, fuel}}`); `ArtifactStoreInitializationEditIndex::retire_step`
(bounded index release).

**Framework initializer (`🔌️plugin/🦀️.rs` `BoundedStoreInitializationAuthority`):** phases CloneInitial → SeedHistory →
FoldSupersessions{transition} (stepped; the whole-log `fold_envelope_history` single step is gone, W1G-4) → CountApplied{position}
(the O(history) `applied_operation_count` sum is gone, W1G-4) → FindApplied → ApplyForward (`fold_forward`) → CommitApplied
(`push_applied_edit`: the store's CANONICAL revision record) → FindRedo → CommitRedo (`push_redo_edit`) → ReleaseEditIndex (64 per
step, W1G-4) → BuildCandidate; the canonical initial digest (`semio_framework_hash::hash(encode_pack)`), so a retained reload names
the same content revision as a whole load. HashInverse/HashRedo phases and `edit_digest` deleted.

**8 plugin initializers** (script `🧪️s4-store-plugin-initializer-supersessions.py`, idempotent, `--check`): each gets
`FoldSupersessions{transition}` after seeding; writer/gismap/jack apply through `fold_forward` (no more refusal of a reload on a
Fatal-outcome op — keep-and-record like every fold site); generation2d/generation3d/process3d keep their in-place appliers fed with
`effective_forward(...).operation()` (withdrawn/faulted → skipped); drawing/raster keep their stepped candidates fed with the
effective input (withdrawn skips the candidate). Their per-op digest scheme is unchanged (local identity only).

**Laws:** `🔌️plugin/🧪️tests/🧪️bounded-reload` — the law op now delegates `may_emit_foreign_steps` (it supersedes; the law had
never run green: "plans foreign steps"), and asserts `reloaded.content_revision_now() == whole.content_revision_now()` (a store
loaded in one piece from the same pair). `🧾️document-archive-load-legs` row labels follow the leaves (`Set count to 1`, …; the
law predated §20.6). Generic reload-after-supersede law for every plugin → S4-AGNOSTIC adds it to `history-edit-acceptance`
(`acceptance_scenario`: reload the overwrite/alternative through `acceptance_reloaded` = the app's own initializer, head == fresh fold).

**Checks:** `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib` (07:55–08:00) **0 errors**; plugin
`--target wasm32-wasip2` (07:10) **0 errors**; batched 8-crate `--lib` native (08:16–08:29): writer, jack, drawing, raster **0
errors**; generation2d (5), generation3d (9), gismap (62) red ONLY from the peer value refactor (`os_dsl::DslValue/ToValue/
FromValue` private, `close_step`/`from_value` signatures) — my arms not reached by type check; process3d not reached (dep
`stdio-ifc` `🚦️native/🦀️.rs:78,81,142,145` TextError vs ValueError). Plugin `--lib` reload/archive laws ran once at 07:49 (5 pass /
3 fail → fixed above); re-run OWED (rule 43 checks only).

### Time-travel routing (coordinator 08:15)

(a) `ValidationFailed("…displaced-owner fixed retirement authority is saturated")` in 3 plugin time-travel laws: the store drained
displaced owners only in runtime maintenance turns, and a burst of changes without turns (600 Applies driven directly, ~3 owners
each) fills the fixed 1024 queue. Fix: `bump()` → `relieve_displaced_pressure()` retires ≤ `DISPLACED_RELIEF_STEPS` (16) one-item
steps of 4 KiB while the queue is under pressure (≥ 256), O(change), a blocked owner stops it; no capacity constant raised. Laws
OWED (rule 43). (b) `a_history_edit_is_its_own_row…` pack reload `revertible` true: the archive load's adopted candidate takes the
local actor from the tail applied edit (every initializer always did; the old `reset` path kept the live actor) — routed to
S4-RUNTIME: carry the replaced store's `local_actor_id()` into the adopted candidate.

### os-host wasip2 red (coordinator 11:35, rule 39)

`🖥️host/🦀️.rs` (crate `semio-framework-os`, wasm-only paths): the two `ArtifactEnvelopeOwners` literals lacked `viewer_checkpoint_id`
and `fold_event_log` lacked its `ViewerHead` — callers of the session-3 per-viewer change. Fixed (+ `🖥️host/🧪️tests/🔬️host-unit`);
repo-wide grep finds no other literal or caller. `cargo check -p semio-framework-os --lib --target wasm32-wasip2` (11:42–12:03)
**0 errors**; native re-check failed only on infra (a swept build dir, disk 2.8 GiB) — OWED.

### F3 — canonical revision in every retained initializer (12:20–13:05, audit `📓️audit-s4-core.md` F3)

- Store: `artifact_initial_digest(&P)` / `artifact_initial_digest_of_pack` — ONE genesis identity for every load path (whole load
  sites + framework initializer + all 8 plugins); `push_applied_admitted(id, edit)` / `push_redo_admitted(id, edit)` beside
  `push_applied_edit` / `push_redo_edit` (canonical effective record, for callers that copied the id under their own grant); the raw
  `push_applied(id, digest)` / `push_redo` are now private (no plugin can name a non-canonical record again).
- 8 plugin initializers (script `🧪️s4-store-plugin-initializer-revision.py`, idempotent, `--check` = pending none): the
  `HashInverse`/`HashRedoForward`/`HashRedoInverse` phases, the `edit_digest`/`initial_digest` fields and their domain digests are
  deleted; commits use the canonical records; the genesis identity is `artifact_initial_digest`. Per family:
  - writer, gismap, jack: clone steppers lose their digest parameter (jack's per-phase observe block deleted);
  - generation2d, generation3d, process3d: copy steppers lose the digest, the `*_observe_*` helper families (≈ 250 / 270 / 120
    lines) and the three tests that only proved those helpers deterministic are deleted;
  - drawing: `HashInitialSchema`/`HashInitialId` deleted, `CommitApplied`/`CommitRedo` carry their edit and use the admitted-id
    pushes (the arena-copied ids stay), the mutation digest prelude of `ApplyForward` deleted (`DrawingMutationDigestAuthority`
    stays: the preflight path uses it);
  - raster: `RasterMutationDigestAuthority` (only the initializer used it) deleted, the snapshot and layer clone steppers lose
    their digest (the candidate authority's two throwaway digests go too), unit tests updated.
- Gates: root `📜️script.ts` envelope gates pinned `…Phase::ValidateEditPair` (stale since 10-03, so `verify interactivity tool-jobs`
  could not pass) → `…Phase::ValidateEdit { index }` + `self.edit_index.admit(&envelope.vcs.edits, index,` (writer, jack ×2,
  gismap, raster, drawing); the raster digest-authority pin dropped there and in the `🔬️tool-job-coverage` fixture.
- Law: the framework `bounded-reload` law asserts reloaded revision == whole-load revision; the per-plugin equivalent rides the
  G12 reload extension (S4-AGNOSTIC: compare `reloaded.store.content_revision_now()` with the source's).
- **OWED (rule 44 cargo freeze):** `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib` (+ wasip2), the batched
  8-crate check native + wasip2, `bun nx run workspace:verify -- interactivity tool-jobs`; tests per rule 43.
- **process3d typed retirement fault (coordinator, folded into F3):** in `💾️binary/🦀️.rs` region `🔖️RetainedConstruction`, the
  inherent `Process3dSnapshotCopyCursor::close_step` now returns `Result<_, semio_framework_value::ValueError>`. In
  `🔖️RetainedStoreInitialization`, its only caller `pump_terminal_retirement` and `pump_active` now return `Result<bool, ValueError>`
  (writer's pattern): the `map_err(ValueError::into_message)` hops are gone, and each String refusal (false terminal ×6, exceeded
  grant, disposer fault) is `ValueError::new(ValueRefusalKind::InvariantViolated, …)`. The 3 call sites (step prologue, the
  `RetireCancelled|RetireFault` arm, `close_step`'s `Fault`) take `into_message()`. Proof so far: a parse check only
  (`rustfmt --edition 2024 --emit stdout` < file, exit 0). The crate check is OWED (rule 44; process3d is also still blocked by
  stdio-ifc).
- **Region release (14:xx):** F3 has released the 8 plugin initializer regions and the framework initializer region, and
  `main` has been told.

### W1G-6: language-agnostic N17 corpus, third-party twin, fast-check (14:xx)

- **Corpus** `🏪️store/🧫️fixtures/🧫️deferred-reprojection/🔣️.json` (12 cases, 22 steps) and **schema**
  `🏪️store/🧬️schema/🔣️deferred-reprojection/🔣️.json`. Each case authors a history (runs of local or other edits) and defers local and
  remote replays by a budget (`null` means undeferred). Each step is one of undo, redo, finalize, finalize as an alternative, trunk
  switch or remote supersession, with an optional interrupt after k turns (discard, cancel, or an edit that lands on the history
  from before the step). Each step expects R (the replay total), `waits`, `turns`, `refused`, the state, applied edits and
  supersessions.
- **Stepping contract, taken from the code** (`EditReplay::step` asks the deadline after every operation; `turn_ends` is
  `replayed >= operations`; closing an edit folds nothing; dispatch and ingest each fold the first turn, see
  `defer_local_step` / `admit_remote_transitions`): a step waits iff R >= B, needs floor(R/B) turns after its dispatch, and
  floor(R'/B)+1 turns once restarted by a cancel or an edit. Edge cases the corpus pins:
  - R == B still waits one closing turn.
  - R == B-1 adopts inside the dispatch.
  - Multi-operation edits close without spending budget.
  - A tail undo needs no replay.
  - Every case changes position 0 or the live head, so no retained prefix can shorten R and the numbers are exact.
- **TS twin** `🏪️store/🧪️tests/🧪️deferred-reprojection/🟦️.ts`:
  - Ajv validates the corpus.
  - An independent history model folds every step with fast-json-patch and steps it with the store's rule.
  - A coverage law checks the corpus has waiting, short, exact-budget, one-under, multi-op, discard, cancel, edit, refused,
    every command kind, remote and undeferred steps.
  - fast-check: over random histories and budgets, the stepped replay ends at the one-shot fold, its turns sum to R, each turn
    folds at most B, and there are floor(R/B) turns after the dispatch.
  - fast-check: every corpus case adopts the same history under any budget.
  - It is registered in the os TS `test-store-oracles` (`📦️packages/🟦️typescript/📜️script.ts` list + `📋️project.json` inputs).
- **Rust corpus law, staged and not saved** (rules 43/44: test code cannot be checked while cargo is frozen):
  - Script `🧪️s4-store-deferred-reprojection-corpus-law.py` is idempotent; `--check` reports `pending: inverse, law`.
  - It inserts `every_deferred_reprojection_corpus_case_waits_turns_and_adopts_as_the_corpus_says` into region
    `🧪️DeferredLocalStepLaws`. The law uses CountedOp authors `local`/`other`, defers both replay kinds, asserts that the view is
    unchanged while waiting, `progress.total == R`, the exact turns, `Rejected { Normal }`, and `assert_live_equals_replay`.
  - It also normalizes the codemod-mangled `CountedOp::inverse` body.
  - The resulting file parses (`rustfmt --emit stdout` on stdin, exit 0).
  - OWED: run the script, then `cargo test -p semio-framework-os --lib every_deferred_reprojection_corpus_case`.
- Verified:
  - `bun test ./🏪️store/🧪️tests/🧪️deferred-reprojection/🟦️.ts`: **5 pass, 0 fail** (1836 expects).
  - Negative control: the stepping rule changed to `replayed > cap` gives **3 fail, 2 pass**, as expected.
  - os TS `bun ./📜️script.ts test-store-oracles`: **12 pass, 0 fail** (5268 expects; was 7).
  - `tsc -p 🧪️s4-store-typecheck-deferred-reprojection.tsconfig.json` (781 files, both twins listed): **0 errors**.

## Session 5 — 2026-10-05

Successor S5-STORE (Opus), coordinator `⚪3f26aaa1…`. Brief: design §22.3/.5/.6/.11, audit `📓️audit-s5-goal.md` gaps 3/6/9/11/14,
`📓️s5-resume.md` §1.4. Scratch: `🗑️generated/s5-store/`. Start state: `landing` + `serve` held by `COORDINATOR-ACTIVATION`
(00:14) → reading, design and staging only.

### P1 design — the viewed alternative is per-replica state (§22.3), written before any code (00:35)

**Finding (read from the code, not from a report): the mechanism is already on disk.** Ticket
`26/10/01/PER-VIEWER-ALTERNATIVE-HEAD` (closed 10-01, its `design.md`) landed it; audit G9.1 and design §9.8 quote the
behaviour from before that ticket. Nothing in the store, the fold or the wire moves another replica's head today.

1. **Where the head lives.** `ViewerHead { line_id, checkpoint_id }` (`📡️replication/🔗️causal/🔀️transition/🦀️.rs:649`), held
   per replica in the envelope (`active_alternative_id`, `viewer_checkpoint_id`), never in the event log. `fold_history` is
   the canonical trunk tip (hub Check In); `fold_history_for(…, head)` is one viewer's projection — `applied`, `checkpoint`,
   `alternative` and which scoped supersessions are effective depend on the head; changes, checkpoints, alternative
   registrations, the redo stack and refusals do not.
2. **Shared vs local.** `Branch` registers the alternative (id, name, root checkpoint) for everyone and selects nothing
   (`:766`). `Checkout` is only validated (`:775`) and no code authors it: `SwitchAlternative` / `CheckoutCheckpoint` call
   `move_head` (store `:21882–21924`), which sets this envelope's head and reprojects. A new-alternative finalize authors
   `[Commit?, Branch, Supersede{scope: alt}]` as one batch and sets only the author's head (`author_supersession`
   `viewing_new`, `:21585`; deferred: `PendingLocalStep.head`).
3. **Persisted local-only.** `.spr` record `REC_VIEWER` (`0x45`, non-critical; `print_document_spr` `:13576`) in the
   replica's own archive (folder binding = `persistedLocalOnly`, browser archive). The shared forms carry no head and hydrate to
   the trunk tip: the `.ops` text and the hub Check In pair (`replay_envelopes_onto_pair` clears the head, `:11518`).
4. **Wire.** Unchanged — no layout or persisted-record change, so nothing rides the channel-22 wave.
5. **What this session adds (proof, schema first).** A language-agnostic two-replica corpus
   `🏪️store/🧫️fixtures/🧫️viewer-head/🔣️.json` + schema `🏪️store/🧬️schema/🔣️viewer-head/🔣️.json`; an independent TS model
   `🏪️store/🧪️tests/🧪️viewer-head/🟦️.ts` (shared event set, per-replica head, fast-json-patch fold, fast-check over arrival
   orders; registered in `test-store-oracles`); the Rust law `🧪️viewer-head/🦀️.rs` over real stores: A finalizes a new
   alternative → B's head, projection, applied order and content revision are unchanged; B lists the alternative; B switches
   locally and A does not move; an `.spr` reload restores each replica's own head while `.ops` hydrates to the trunk; every
   arrival order of the shared log converges (same registrations, equal projections for equal heads).
6. **What is deleted (own wave, after P2/P3, on coordinator GO).** The dead `HistoryTransition::Checkout` (wire tag 4, the
   `.ops` `checkout` line, TS kind, schema + corpus rows, the `co-*` steps of `🧫️supersede-fold`) and its last use as a
   shape tag (`ArtifactCommand::history_transition_kinds` claims `SwitchAlternative`/`CheckoutCheckpoint` author `Checkout`,
   `:3287`; a config store needs a typed head-move refusal instead). No produced byte changes (tags 5 and 6 keep their
   numbers; the admissible set shrinks), so no channel bump; it touches the shared replication crate and three codecs.

Coordinator 00:5x: P1 = this proof; G9.1 / §9.8 marked stale; GO for the `Checkout` deletion as its own wave after P2–P4 and
the foreign-step Withdraw. Correction to item 5: B's content revision is NOT asserted unchanged (A's pending-edit commit moves
the trunk tip checkpoint, which the revision names); the law pins B's head, projection, applied order and supersessions.

### Design note — `Withdrawn` of an operation that plans foreign steps (coordinator decision 00:5x, §22.1), written before code

**Facts read from the code.**
- "Plans foreign steps" is `Mutation::may_emit_foreign_steps()`: `true` for every `#[derive(CompositeMutation)]` kind and for a
  hand-written aggregate that does not override the trait default; `false` for plain derived leaves. Today `admit_replacement`
  (store `:23701`) refuses every supersession of such an operation, a withdrawal included, and `effective_operation` folds the
  refusal as a `Fatal` `mutation.invariant` no-op.
- Foreign steps are never re-planned by a fold: a replay folds the operation's own diff only
  (`foreign_steps_are_excluded_from_fold_plan_diff`). They were materialized when the composite gesture was authored: a gesture
  whose plan has foreign steps applies nothing and becomes a gateway transaction (`TransactionProposal` → `TransactionPrepare` →
  `TransactionCommit`), and every member store — the planner's own included — commits its operations as exactly ONE edit whose
  operations all carry `MutationMeta.group_id = txn_id` (`transaction_commit` → `stamp_tail_group_id`; composed members
  `commit_transaction_group`). A plan without foreign steps applies as an ordinary edit with no `group_id`.
- `group_id` is persisted (`.spr` `HistoryOpMeta.group_id`) and is NOT on the wire: `edit_from_operation_envelope` gives every
  ingested operation `group_id: None`. Product trees contain no `call_foreign` today (only the kernel law fixture and the plugin
  `set-transaction` fixture plan foreign steps); cad `create-building-storey` and hand-written aggregates are "foreign-step"
  operations whose unit is themselves.

**Decision as the store implements it.**
1. **The unit and its address.** The unit of an operation is every applied operation carrying its `group_id`, in every member
   store; an operation without a `group_id` planned nothing foreign and is its own unit. The store answers
   `unit_operations(mutation_id)` (this store's part, the operation itself included, applied order) and `unit_id(mutation_id)`
   (the `group_id` the other member stores are asked by).
2. **Supersede law (`admit_replacement`, every fold site, every replica).** `Withdrawn` is admitted for every operation: a
   withdrawn foreign-step operation folds as a no-op, without a fault. An `Input` replacement of an operation that plans foreign
   steps (or a replacement that itself plans them) stays refused — except the operation's own recorded input
   (`payload == original.encode_op()`), which re-plans exactly what was materialized: that is what the undo of a withdrawal
   authors (`SupersedeLedger::restore` re-authors the previous effective input), so the withdrawal is undoable.
3. **One store: the unit moves together (authoring law, `supersede_inputs` + `draft_inputs`).** A supersession that names an
   operation of a unit must name every applied operation of that unit in this store, all `Withdrawn` or all their recorded
   inputs; anything else is refused at authoring with nothing recorded. So a store's part of a unit is withdrawn by ONE
   `Supersede` (one history row) and restored by one.
4. **Several stores.** Logs are per document, so the unit is withdrawn by one `Supersede` per member log — "one history row" is
   the acting store's row; each member's own history shows its own. Composed members of one instance: the store's
   `CompositionCoordinator` gains `withdraw_group(parent, children, group_id)` beside `undo_group` (two phases: every member
   holding operations of the group dry-runs its withdrawal — a blocking report anywhere refuses the whole step with nothing
   authored — then each member authors its `Supersede`). Members in other instances need the gateway to fan the same two
   phases out (a `TransactionUndo`-shaped verb): a new `AppCommand`, so it rides channel 22 — not in this WP.
5. **What a remote replica folds.** Exactly the authored transitions: a `Supersede` whose inputs are `Withdrawn` for the unit's
   operations of that document. Each folds as a no-op through `effective_operation`; no unit knowledge is needed, and the
   member documents converge per document as their own supersedes arrive (the same per-document arrival the composite gesture's
   member edits had).
6. **Gap that needs the wire (stated, not worked around).** A non-authoring replica holds no `group_id`, so it cannot complete a
   cross-store unit. Until `MutationEnvelope` carries the group (trailing flag bit 3, channel 22), the store refuses, on such a
   replica, to withdraw a foreign-step operation whose plan on its base (`foreign_steps(state_before)`) is not empty
   (`unit not addressable`); an operation that plans nothing foreign there — every product operation today — is its own unit
   and withdraws.
7. **Runtime consequences (S5-RUNTIME).** The Withdraw row action drafts `unit_operations(target)` together; `editable` (input
   editing) is false for an operation inside a unit (`unit_id(..).is_some()`), since rule 3 refuses its input replacement.

**Addendum 01:2x — scope as approved, and two corrections from reading `dispatch_group`.**
- `group_id` is not only the gateway `txn_id`: `CompositionCoordinator::dispatch_group` mints one (`GroupMeta.group_id: None` →
  the invocation id) for EVERY parent+child gesture of one instance. Rules 1 and 3 above therefore apply to a **cross-artifact
  unit** only: an operation that plans foreign steps and whose plan on the state before it is not empty (every replica can
  evaluate `foreign_steps(state_before)`), or an operation whose `MutationMeta.origin` is `Transaction { .. }`. Ordinary
  one-instance gesture groups keep today's per-store editing.
- A unit with foreign steps always has a member in another instance, so rule 4's `withdraw_group` has no subject and is dropped.
- Coordinator scope: the cross-instance fan-out is NOT this session (no product tree calls `call_foreign`). **Explicit limit:**
  the store refuses to withdraw (or restore) an operation of a cross-artifact unit with `VcsError::UnitSpansDocuments` → fault
  code `history.unit-spans-documents` (en "This step also changed other documents, so it cannot be withdrawn on its own yet." /
  de "Dieser Schritt hat auch andere Dokumente geändert und lässt sich daher noch nicht einzeln zurückziehen."), nothing
  recorded; a law pins it. Every product operation today is its own unit and withdraws.
- Read API as S5-RUNTIME was told: `unit_id(mutation_id)` is `Some` for a member of a cross-artifact unit (Withdraw unavailable,
  `editable` false), `unit_operations(mutation_id)` is this store's part of it (the operation alone otherwise).
- Wire: a `MutationEnvelope.unit` field (trailing flag bit 3, one `str` after `[line]`, the gateway transaction id of a
  `Transaction`-origin operation) would let a non-authoring replica of a target document know the unit. **Coordinator decision
  00:5x: NOT added this session** — it has no producer until `transaction_commit` stamps group and origin BEFORE the edit is
  announced (today `dispatch(Apply)` then `stamp_tail_group_id`, plugin `:35292–35294`), and no consumer while cross-artifact
  withdrawal is refused. Wave B is unchanged.
- **OPEN ITEM FOR A LATER TICKET (cross-artifact units):** withdrawal refused (`history.unit-spans-documents`); wire `unit`
  field deferred (layout: `🔗️causal/🦀️.rs:916/:954/:1159`, TS `🟦️.ts:1035/:1060/:1396/:1491`, fixture
  `🧮️document-backbone-batch-v1` `trailing-flags-invalid` `08` → `10`); precondition: a stamped gateway commit; then the gateway
  fan-out verb (two phases per member, `TransactionUndo`-shaped). Until then a non-authoring replica of a TARGET document cannot
  tell a `Transaction`-origin member (its `origin` is not on the wire) and treats it as an ordinary operation.

Laws (store): a withdrawn foreign-step operation folds as a no-op at every site (live, Report replay, `.spr`/`.ops` reload,
replica ingest in any order, retained initializer); its undo restores it; an `Input` replacement other than the recorded input
still folds as the `mutation.invariant` fatal no-op; a partial unit withdrawal is refused with nothing recorded; a whole unit
withdraws as one `Supersede`.

### Staged while `landing` is held by COORDINATOR-ACTIVATION (00:35–01:0x) — nothing saved under `🧰️framework/**` yet

| Wave | Staged where | State |
|---|---|---|
| P1 proof | `🗑️generated/s5-store/stage-p1/🏪️store/{🧬️schema/🔣️viewer-head, 🧫️fixtures/🧫️viewer-head, 🧪️tests/🧪️viewer-head/{🟦️.ts,🦀️.rs}}` | corpus 4 cases / 52 steps (hand-computed expectations); TS twin run from the staging tree: **5 pass / 0 fail, 1972 expects**; negative control (a received `branch` moves the receiver's head): **3 fail / 2 pass**; Rust law parses (`rustfmt`), never compiled |
| FW (foreign-step Withdraw) | script `🧪️s5-store-foreign-withdraw.py` (`--check`: 7 anchors pending, each unique; `--emit` + `rustfmt` parse: store ok, vcs ok); laws `🗑️generated/s5-store/stage-fw/🏪️store/{🧬️schema/🔣️supersede-law, 🧫️fixtures/🧫️supersede-law, 🧪️tests/🧪️supersede-law/{🟦️.ts,🦀️.rs}}` | TS twin: **5 pass / 0 fail, 658 expects** (12 input rows + 12 unit rows, both tables total); Rust law parses, never compiled |
| P2 (inverse refused) | script `🧪️s5-store-inverse-refused.py` (1 anchor pending; parse ok; refuses to apply before the vocabulary holds `mutation.inverse-refused`); law `🗑️generated/s5-store/stage-p2/inverse-refused-laws.rs` (appended to `🧪️supersede-law/🦀️.rs` at landing) | parses; waits for "OUTCOME CODES LANDED" |

**P1 corpus conventions.** Two replicas `a`/`b`; steps `edit`, `commit`, `alternative` (finalize as a new alternative),
`supersede` (`document`/`line`), `switch`, `checkout`, `receive` (`authored`/`reversed`), `reload` (`spr`/`ops`); after every
step both replicas' views are stated (line, explicit checkpoint, state, applied, supersessions, listed alternatives);
`converged` states every alternative's tip for any arrival order of the whole log. Edits carry one operation (a received
multi-operation edit arrives as one edit per operation, so `applied` would differ between author and receiver). Concurrent
edits commute (`addN`), because the Rust order of same-millisecond events of two replicas is decided by random actor ids.

**P2 semantics (decided while reading the fold sites).** A messageless fold site (`fold_effective_edit`: prefix ring,
materialization, retained initializers) never calls `inverse`, so "inverse refused ⇒ no-op" could not hold at every site
without an inverse per fold. Therefore: the operation folds forward exactly as `fold_operation` folds it everywhere, the
Report replay adds one `Fatal` `mutation.inverse-refused` message for that mutation and stages no inverse rows for it; the
report blocks finalizing until the mutation is edited or withdrawn. This also unblocks loading: `project_envelope_history`
replays in Report mode (store `:12858`) and today a document whose replay meets a refused inverse cannot load at all.

### Landed 01:10–01:16 (first `landing` hold, 5 min 22 s)

- **Wave FW** (`python3 T/🧪️s5-store-foreign-withdraw.py`, 7 anchors, one write per file): `🌿️vcs/🦀️.rs` — `VcsError::UnitSpansDocuments
  { mutation_id, unit }`, its `Display`, fault code `history.unit-spans-documents`; `🏪️store/🦀️.rs` — `admit_replacement` (a
  withdrawal is admitted for every operation; a foreign-step operation takes no input but its own recorded one),
  `ArtifactStore::unit_id`, `unit_operations`, private `plans_foreign_steps` (region `🔖️TimeTravelReads`), `supersede_input`
  refuses a unit operation (region `🔖️Supersede`).
  **RAN** (gate v3, 01:11:20–01:15:50): `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib
  --message-format=short` → **exit 0**, `Finished dev in 4m 29s` (plugin lib 283 warnings: type-checked). This is also the F3
  proof's first owed check (kernel + plugin lib).
- **Language-agnostic law halves** (`python3 T/🧪️s5-store-land-laws.py viewer-head|supersede-law`): schema + corpus + TS twin of
  both sets under `🏪️store/{🧬️schema/🔣️<set>, 🧫️fixtures/🧫️<set>, 🧪️tests/🧪️<set>/🟦️.ts}`, registered in the os package
  (`📦️packages/🟦️typescript/📜️script.ts` oracle list, `📋️project.json` nx inputs).
  **RAN**: `bun ./📜️script.ts test-store-oracles` → **22 pass / 0 fail** (5 files, 7948 expects; was 12 pass in 3 files).
- **NOT landed (staged, parse-checked only):** the Rust laws `🧪️viewer-head/🦀️.rs` (2 laws) and `🧪️supersede-law/🦀️.rs` (3 laws) and
  their mounts in `🧪️tests/🔬️unit/🦀️.rs` — the kernel test target does not compile (S5-CHANNEL). Land with
  `python3 T/🧪️s5-store-land-laws.py viewer-head --rust` / `… supersede-law --rust`.
- Rule 55 (01:16): build gate v4 (cargo < 4, rustc < 6, `CARGO_BUILD_JOBS=3`; `cargo test` builds need ≥ 25 GiB free).

### P4 — folder reload (R2-2, R2-4): what is on disk and what it closes (01:05)

- **R2-4** (`local.backbone-scope-mismatch` on every folder-bound batch). Cause (W2-B F3.2): the shell addressed a folder
  document by a runtime counter (`${pluginId}-${instanceId}`) while the guest stamps its store id; and the TS actor demanded a
  hub-verified cold pair of a folder document. On disk: worker `handleLocalMsg` asks for the pair only under a hub binding and
  refuses exactly the envelopes whose `document_id` is not the binding's (`🏪️store/👷️worker/🟦️.ts:6995–7001`); the shell
  attaches by the program's own identity (`syncAttachDocumentIdV1`, SHELLS' tree). Law: `💻️os/🧪️tests/🧪️folder-archive-restore`
  "a folder-bound document admits the edits its program stamps with its own identity, and refuses a batch addressed by a
  runtime counter (e2e R2-4)". **Closes R2-4 at worker level**; the live probe (batch B, running) is the proof.
- **R2-2** (rows, supersede rows and alternatives lost after re-attach). The store pair always carried them (Run 2's archive
  held all 5 edits and 11 transitions). On disk: `retire_displaced_document_rows` after every whole-document replacement
  (plugin `🦀️.rs:26275, 28818, 28866, 35358`; `⏪️time-travel/🦀️.rs:3263`), the worker's own-write echo guard (`folderArchive`,
  `:4138`), and `restoreDocumentArchiveV1` (load → re-read history → refresh). Laws: plugin
  `a_document_archive_round_trip_lists_every_history_row_of_its_source` (edits, an overwrite with its undo and redo, two
  new-alternative finalizes, edits on the alternative; rows and transition count equal after the archive load into an instance
  holding another document) and the worker law "attach, edit, reload and re-attach restores the head and the history rows, with
  no refused edit and no own-write echo".
  **RAN 01:03**: `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🔨️modules/🏪️store/👷️worker/🟦️.ts -t "folder archive
  restore"` → **5 passed** (14 skipped by the filter).
- **Gap in the laws:** neither asserts the alternatives a reloaded instance LISTS, the head it stands on, or the warnings of
  the rows; the worker law drives a TS stand-in program, not the guest. The plugin law is the one that fails on the old
  behaviour (it found the displaced rows). Extension staged below.
- **P4 law extension — STAGED** (`🗑️generated/s5-store/stage-p4/folder-reload-law.rs`, parses appended to
  `🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs`; that file is not in my trees): law
  `a_reattached_folder_document_restores_rows_history_edits_alternatives_and_warnings` — edits, a session-path OVERWRITE whose
  replay introduces a `mutation.no-op` warning, a session-path NEW ALTERNATIVE, `document_archive` → a fresh instance holding
  another document → archive load; asserts projection, edit rows and history-edit rows with their per-mutation outcomes, the
  alternatives list, the head (`active_alternative_id`) and the durable outcomes. Needs the plugin test target to compile.

### P5 — owed verification (01:18–01:30)

| When | Command | Result |
|---|---|---|
| 01:11–01:15 | `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib` (after wave FW) | **exit 0** (F3 kernel + plugin lib proof) |
| 01:18–01:25 | `CARGO_BUILD_JOBS=3 cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-{writer-writer,trinity-jack,draw-drawing,raster-raster,procedural-generation2d,procedural-generation3d,gis-gismap,process-process3d} --lib --keep-going` (`check-8-native-1.txt`) | exit 101. **GREEN (0 errors): writer, drawing, generation2d, generation3d, process3d.** RED: jack 2 (`🧬️schema/📸️snapshot/📝️text/🦀️.rs:30,71` E0425 `attach_bundled_content`, TEXT), raster 1 (`✏️editor/🎮️commands/🖌️paint-stroke/🦀️.rs:381` E0599 `map_err` on `GestureDrive`, TOOLS/STROKES), gismap 42 (inference worker `os_dsl::{DslValue,ToValue}`, io import/export, `💾️binary/🦀️.rs:263` in `🔖️OwnedSprCatalog`; **4 in my region** `🔖️RetainedStoreInitialization` `:830–873`) |
| 01:26 | `python3 T/🧪️s5-store-gismap-initializer-value-error.py` | applied: `pump_active` / `pump_terminal_retirement` answer `ValueError` (writer's pattern), 5 String refusals → `InvariantViolated`, 2 fault sinks take `into_message()`; 9 lines, region-scoped |
| 01:27–01:28 | `cargo check … -p semio-s-artifact-gis-gismap --lib` | **blocked by a peer red in the kernel lib**: `🗣️dsl/🦀️.rs:219:71 E0308 EncodeOptions` (landing held by S5-TOOLS; reported to `main`). gismap fix UNVERIFIED |

OWED after the kernel is green again: the gismap re-check (expect the 4 initializer errors gone), the same 8 crates with
`--target wasm32-wasip2`, `bun nx run workspace:verify -- interactivity tool-jobs`.

### `Checkout` deletion wave — census and plan (coordinator GO 00:5x; own wave after P2–P4; NOT started)

No produced byte changes: no code authors `HistoryTransition::Checkout` since 10-01 and the fold only validates it, so no
persisted document and no wire message of a current build holds tag 4; tags 5 (`Repin`) and 6 (`Supersede`) keep their numbers,
the decoder answers tag 4 as an unknown tag. No channel bump. Sites (`git grep`, 01:30):

| Tree | Sites |
|---|---|
| `FW/📡️replication/🔗️causal/🔀️transition/🦀️.rs` | variant `:125`, `HistoryTransitionKind::Checkout` `:232/:239 (ALL [..; 7] → 6)/:248/:263`, encode `:385`, decode tag 4 `:477`, fold arm `:775`, docs `:523/:670` |
| `…/🔀️transition/🧪️tests/🔬️unit/🦀️.rs` | 10 uses (`commit_and_checkout_materialize_facts_and_positions`, `checkout_governs_concurrent_edits_by_hlc`, …) → heads move by `ViewerHead` |
| `FW/📡️replication/🟦️.ts` | `HISTORY_TRANSITION_KINDS` `:1135`, `SupersessionFoldTransition` `:1181`, docs `:1097/:1177/:1191` |
| `FW/📡️replication/🧪️tests/{🧪️history-transition/{🐍️.py,🟦️.ts}, 🧪️supersede-fold/🟦️.ts}` | generator rows, the TS encoder arm, the `checkout` event shape |
| `FW/📡️replication/🔗️causal/{🧬️schema/🔣️history-transition, 🧬️schema/🔣️supersede-fold, 🧫️fixtures/🧫️history-transition, 🧫️fixtures/🧫️supersede-fold}` | schema `kind` enums, accepted/malformed corpus rows (tag 4 becomes a malformed row), fold steps `co-1/co-2/co-3` (the expectations they precede already depend on `expect.head` only), the `shapes` table |
| `OSM/🏪️store/🦀️.rs` | `.ops` `OpsHeaderLine::Checkout` print `:12784` / parse `:13852`, `ArtifactCommand::history_transition_kinds` `:3281–3287` (`SwitchAlternative`/`CheckoutCheckpoint` claim `Checkout`: replace by a head-move predicate + a typed config-store refusal), doc of `branched_alternative` |
| `OSM/🏪️store/🧪️tests/{🔬️unit, 🧪️supersede-replay}/🦀️.rs` | `a_config_store_holds_undo_and_redo_only` (`Checkout` kind), the trunk law's `Checkout { .. } => false` arm |
| `OSM/📡️spr/📜️history/🦀️.rs` | the `.ops` text twin (`checkout` line) |
| `OSM/🛢️db/🗿️artifact/🦀️.rs`, `OSM/🔌️plugin/🦀️.rs` (+ `🔬️plugin-runtime-plugin-builder-contract`), `🌎️hub/…/🪐️space/…/🧪️tests`, `✏️s/🔌️plugins/🌿️vcs/…/✏️editor/🧪️tests` | one mention each to re-read (several are the `checkoutCheckpoint` ACTION id, which stays) |

Order: schema + corpora + Python generator → Rust codec/fold + unit tests → TS twin + tests → store `.ops` + shape gate →
`cargo check -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin --lib`, replication
`cargo test --lib`, replication TS, os `test-store-oracles`, `python3 …/🧪️history-transition/🐍️.py`.

### More owed runs (01:33)

| Command | Result |
|---|---|
| `python3 T/🧪️s4-store-deferred-reprojection-corpus-law.py --check` | `pending: inverse, law` — anchors still resolve; lands with the first kernel test build |
| `bun ./📜️script.ts verify interactivity tool-jobs` (`verify-tool-jobs.txt`) | **exit 1** at rule `shared-action-fixture`: "descriptor disposition join is not exact across manifest and runtime factory definitions" (`📜️script.ts:5691`) — a manifest/runtime action-descriptor rule (GATES/RUNTIME), raised before the run reports the envelope pins F3 corrected, so those stay unproven by this gate |

**P6 (replay cost) — NOT started.** Notes for the successor: the store has no mutation-id index (`locate_mutation` `:19334`
scans applied edits; `supersede_inputs` builds a whole `HashMap<MutationId, &Mutation>` per call `:21764`; `unit_id` adds one
`locate_mutation` per supersede input); an index must follow `applied_edit_ids` through `StackMirrorDirt` like the cursor
mirror does. The live digest chain is recomputed in `prefix_state_recorded` (`forward_prefix_digests` twice per call).

### P4 — layer split agreed with S5-LOAD (coordinator relay 01:4x), and the hot-attach answer

One law per layer: **mine** = (a) store / `.spr` / vcs reload of rows, supersessions, alternatives and `REC_VIEWER`;
(b) the folder TRANSPORT (Rust actor `🏪️store/🔄️sync`, TS worker). **S5-LOAD** = the route on the program side of the channel
(`PluginApp`, `DocumentArchiveLoadHost`, `ReadHistory`). The plugin-level draft `stage-p4/folder-reload-law.rs` is therefore
LOAD's to take or drop (it drives `PluginApp`); I do not land it.

- **(a) STAGED** in `stage-p1/…/🧪️viewer-head/🦀️.rs`, region `🧪️ReloadLaws`:
  `the_persisted_pair_restores_edits_history_edits_alternatives_the_head_and_warnings` — the probe's sequence at the store
  (three edits; a session-path OVERWRITE whose replay leaves a durable `Warning`; a session-path NEW ALTERNATIVE), then the pair
  a detach persists (`print_document_pack`) → a re-attached store (`parse_document_pack`): every edit with author and
  operations, the 4 history transitions, the alternatives with their chains, the head, both effective supersessions, the
  durable outcomes, the applied order, the projection and the content revision are equal; the `.ops` text restores all but the
  head, at the trunk tip. Parses; never compiled.
- **(b) on disk, RAN**: TS worker `🧪️folder-archive-restore` 5 pass (01:03, above): no batch refused, the archive is written per
  `localDocumentArchive`, the read-back after re-attach replaces the document once, the own write is no echo, an envelope
  addressed by a runtime counter is refused `local.backbone-scope-mismatch`. Rust actor twin (`🏪️store/🔄️sync/🦀️.rs:2652–2681`,
  `:4724–4750`): the same scope check and `persist_archive`; its storage laws
  (`document_archive_event_log_storage_round_trips_complete_owned_bytes`, `folder_event_log_storage_round_trips_*`,
  `document_archive_actor_refuses_malformed_and_over_limit_candidates_without_displacing_live_bytes`) are OWED (kernel test
  target). The transport is content-agnostic: (b) proves the bytes come back, (a) proves what those bytes restore.

**LOAD's finding — "the document port is a HOT attach: a folder holds nothing until the first published batch, and a program
that only receives never writes its folder". Is that intended?** At the transport: yes by construction, and it cannot be
otherwise there — the actor persists what it is handed (`LocalDocumentArchive`) and reads back; it never originates an archive,
because the pack belongs to the program. As product behaviour: **no, it is a gap on the route side.** (1) Binding an existing
document to an empty folder and leaving without another edit leaves the folder empty, so the re-attach restores nothing.
(2) A replica bound to a folder AND a hub that only receives never refreshes its folder archive (stale, recoverable from the
hub; a folder-only binding has no remote peers — `persistedLocalOnly`, `allowsCollaboration: false`). The shell writes the
archive only from its outbound-send path (`ShellHost` `archivePersistence`, `:3221`). Fix for the route owner: persist the
program's archive when the bind's first folder read finds none, and after an ingested batch. The transport accepts
`localDocumentArchive` at any time after `open`; if the route wants a positive "the folder holds no archive" signal instead of
waiting out the read (the identity document races a 2 s timeout for exactly this, `ShellHost:4214`), I add a
`documentArchiveAbsent` event to both actor twins (schema first) — on request.

### Resume 04:24 (usage cut ~02:40–04:22, rule 62) — repair-first

- Nothing of mine was half on disk. `--check` of every wave script: FW `none pending`, gismap fix `none pending`, both law
  landings `none pending`; staged and untouched: PM (`store:pair-merge`), P3 (4 anchors), P2 (1 anchor). The TS twin of the
  supersede law had landed at 01:16. **RAN 04:25**: `bun ./📜️script.ts test-store-oracles` → **22 pass / 0 fail** (5 files,
  8169 expects). A peer added 11 lines to the store at 02:18 (`:6284`); my anchors still resolve.
- 01:50–02:47 before the cut: decisions recorded as design §22.22 (below); four `acquire landing` rounds (S5-LOAD, S5-UI,
  S5-RUNTIME, S5-GATES held it), no landing. 04:24–: `landing` still shows S5-GATES' hold from 02:25 (it verifies an interrupted
  kernel landing); FIFO ticket kept.

### Design §22.22 — a folder read-back merges, it never replaces (coordinator requirement, probe batch H) — STAGED

**Defect (read from the code):** a folder read-back of another writer's bytes is a whole-document replacement
(`documentArchiveReplaced` → archive load → store replacement): (a) the receiver's events the archive lacks are dropped,
(b) `parse_decoded_document_spr` adopts the pair's `REC_VIEWER` (store `:13701`), so the receiver moves onto the WRITER's
alternative, (c) the replacement ends an open time-travel session.

**Store wave PM** (`T/🧪️s5-store-pair-merge.py`, one insertion, region `🔖️PairMerge` after `🔖️ArtifactStore`; parses):
`ArtifactStore::merge_persisted_pair(pack, spr) -> Result<PairMerge { merged, ahead }, VcsError>` — parses the pair, refuses
another document (id, schema or genesis digest differ: `ValidationFailed`, nothing changed), ingests every operation and history
transition the store lacks through `ingest_remote` (head untouched, own edits kept, an open replay sees an ordinary base
move), never reads the pair's viewer head; `ahead` = events the store holds and the pair lacks. A cold construction
(`parse_document_pack` → new store) stays the only reader of `REC_VIEWER`.
- Persist rule (S5-LOAD's route): persist iff `ahead > 0` after a read-back merge; always (coalesced) after a hub ingest. It
  terminates without clocks: A writes the union, B merges it with `ahead == 0` and writes nothing.
- Law (staged in `stage-p1/…/🧪️viewer-head/🦀️.rs`, region `🧪️PairMergeLaws`):
  `a_read_back_pair_merges_its_log_and_never_moves_the_reader` — A on its own alternative with an uncommitted edit, B writes
  the pair with a trunk edit: A's merge = `{ merged: 1, ahead: 4 }` and A's head, projection, applied edits and supersessions
  are unchanged; B's merge of A's union = `{ merged: 4, ahead: 0 }` and B stays on the trunk; a third round merges nothing;
  another document's pair is refused with nothing changed; only a cold load stands where its pair says.
- **Known cost (follow-up):** the merge parses the whole pair (`parse_document_pack` folds the foreign log) synchronously —
  O(pair) per read-back, not stepped. The exact typed envelopes need the typed edits; a stepped variant belongs with the
  stepped document load (`📓️api-stepped-document-load.md`). The ingest half already honours `defer_remote_replays`.

### Two probe items (coordinator relay 04:3x, batch E)

1. **"The History body lists Alternatives / 'Main line' only once the document holds a checkpoint — intended?"** It is not a
   precondition of the main line: the trunk exists from the genesis (`trunk_alternative_id()`, `active_line_id()` and
   `SwitchAlternative { trunk }` all work with zero checkpoints). It IS a store invariant of the LIST: `vcs.alternatives` holds
   checkpoint chains, and consumers index the tip (`switch_local_alternative` `checkpoint_ids.last()`, `SpaceMember::checkout`,
   composition pins, `malformed_alternative_checkpoint_pin_is_rejected_at_construction`), so the fold lists the trunk "as soon
   as a commit made on it gives it a chain" (`🔀️transition/🦀️.rs:852`). I agree with the lean: the main line should be listed
   from the first edit. The fix is in the body (S5-RUNTIME): list the main line from `trunk_alternative_id()` whenever
   `vcs.alternatives` does not hold it, current while `active_alternative_id` is `None` — no store change; an empty-chain entry
   in `vcs.alternatives` would break the tip readers above. On request I add a read `ArtifactStore::history_lines()` (trunk
   first, `tip: Option<checkpoint>`), so the body needs no special case.
2. **O4 — `commitCheckpoint refused: dispatch-failed — actor-document-port.retired` after a detach.** Two layers.
   Shell (SHELLS/LOAD): `AutoCheckinScheduler` is keyed on `[isEditorSession, currentDocumentId, dispatchCheckpoint]`
   (`ShellHost:11177–11187`), not on the document port: a detach leaves its timer armed. It must cancel on detach (key the
   effect on the binding as well) — that is the "cancel the pending automatic checkpoint" the relay asks for.
   Worker (mine, `🏪️store/👷️worker/🟦️.ts:2385–2388`): the browser actor throws `actor-document-port.retired` AFTER the guest ran
   the command, when it hands the guest's outbound messages to a port that is gone. So the checkpoint was NOT refused — the
   guest committed it and only its announcement was dropped; the shell is told "dispatch-failed" about a command that took
   effect. The dropped announcement heals at the next bind (`attach_backbone` announces the whole history; LOAD's
   persist-on-bind / `ahead > 0` rule rewrites the folder). **Open (not started, low priority):** refuse a publishing command
   at admission while the document port is retired (a typed local refusal before the guest runs — an 11th
   `command-rejection` code, schema + both twins + notice), instead of failing after the fact.

### F4 — the two-peer divergence seen live (probe batch H, coordinator relay 04:5x): triage and acceptance law

Evidence `🗑️generated/s5-e2e/run5-react-en-H.final.txt`: B's PUT is fetched by A (GET 200) but A's body never lists B's edit;
A's Accept reviews `ready` without it; A's finalize is never written to the folder; both peers end different, neither is told.

| Symptom | Layer | Reading (from the code) |
|---|---|---|
| A fetches B's bytes, B's edit never becomes a row / a "not applied while editing" row | **store (root)** + route | the fetched pair never reaches A's live store as events: the store had no entry for a fetched pair (only a whole replacement). React routes the read-back to `loadDocumentArchive` (`ShellHost:3319`). The store's remote path that yields the rows, the generation bump and `BaseMoved` is driven by envelopes only — `merge_persisted_pair` produces them |
| A's Accept reviews `ready` without B's edit | store (same root) | the replay covers what the store holds |
| no folder PUT after A's finalize | route / session | the store announces the finalize (`commit_finished_replay` → `install_transitions` → `flush_outbound`; deferred: `adopt_pending` → `flush_outbound`). Either the worker got that batch and the shell's `archivePersistence` did not fire for a publication that is not a command's reply (LOAD), or the session commit's batch never left the guest (RUNTIME) — the ndjson of A's worker at ~130 s decides |
| one `timeTravel.stale` on A at 109.5 s | session | A's session generation moved once at B's write although no row changed — what the guest did with the archive load under an open session is the load route's to answer |

**Acceptance law (staged, `🧪️viewer-head/🦀️.rs` region `🧪️PairMergeLaws`):**
`two_peers_on_one_folder_converge_through_an_open_history_edit` — A edits, B opens the pair cold; A holds a draft and a replay
begun early; B edits and writes; A's merge = `{ merged: 1, ahead: 0 }`, A's generation moves, the early replay commits
`Stale`, `state_before(target, drafts)` is unchanged and B's edit sits downstream; Accept's report contains B's mutation and
reaches draft + B's edit; finalize; merging the folder's (B's) pair = `{ merged: 0, ahead: 1 }` → A persists; B merges
`{ merged: 1, ahead: 0 }`; both show the same state, supersession and applied edits; a last merge = `{ 0, 0 }`.
The merge retires the parsed pair through `ArtifactEnvelope::retire_unadopted` (never a bare drop).

### Wave AA — `documentArchiveAbsent` on both actor twins — STAGED (04:45–05:00)

Script `T/🧪️s5-store-archive-absent.py` (`--check`: 20 edits pending, every anchor unique; `--emit`: the Rust files parse with
`rustfmt`, both TS files transpile). Shape as agreed with S5-LOAD: TS `{ kind: "documentArchiveAbsent" }` (strict parse: no
other key), Rust `ArtifactEvent::DocumentArchiveAbsent`.
- **Rule, both twins** (schema `🏪️store/🔄️sync/🧬️schema/🔣️folder-archive-presence`, 9-case corpus
  `🏪️store/🧫️fixtures/🧫️folder-archive-presence`): the actor remembers the archive the folder is known to hold (last read or
  written; none after a read that found it empty); it announces the absence once per open — when the first ANSWERED read finds
  no archive and nothing was read or written before — and writes an archive only when its bytes differ from the held one.
- TS worker: `folderBootstrapped`, `pollFolderOnce` (a 204/404 that raced a write is ignored; an emptied folder forgets the held
  archive and reports `persisted: false`), `writeFolder` (no PUT for the held bytes). `💻️os/🟦️.ts`: union + strict parse.
- Rust native actor: `folder_held_hash` (separate from `last_written_hash`, whose self-write suppression is unchanged —
  an external archive with the same operation ids never updated it, so it could not carry the de-dup), `folder_bootstrapped`,
  `setup` / `handle_external_change` (a read ERROR is no longer taken for an absent archive) / `persist_write_archive`; test
  seams `setup_test`, `folder_writes_test`. The wasm Rust actor has no folder. **One no-op arm in the wgpu shell's exhaustive
  `ArtifactEvent` match** (S5-WGPU's file; it already persists on an empty folder) and the two test name maps.
- An append-only folder event log cannot be emptied, so the Rust law stages 7 of the 9 cases (stated in the schema).
- Laws to write at landing: TS — the corpus through the real worker inside `folder archive restore` (+ codec round trip and
  the strict-parse refusal); Rust — the corpus through `native_actor::ArtifactActor` over a temp folder (with the other Rust
  laws, behind the kernel test target).

### Wave PM LANDED 05:11:45–05:20:14 — "MERGE PAIR ON DISK"

`python3 T/🧪️s5-store-pair-merge.py` (7 anchors, one write of `🏪️store/🦀️.rs`):
- region `🔖️PairMerge`: `PairMerge { merged, ahead }`; `ArtifactStore::merge_persisted_history(pack, history: HistoryLog)` (the
  decoded entry S5-LOAD's stepped archive machine calls; another document is refused before anything is parsed: doc id,
  schema, `artifact_initial_digest_of_pack(pack)` vs the store's genesis digest) and `merge_persisted_pair(pack, spr)` = decode +
  that. The parsed pair retires through `ArtifactEnvelope::retire_unadopted`.
- `SpaceMember::merge_persisted_envelope(envelope_pack)`: default refuses (`ValidationFailed`), `ArtifactStore` decodes its
  member pack and merges. **Owed by S5-NESTED:** the `space_members!` arm (one delegating line, like `envelope_pack_bytes`).
- `AppliedMutation.unit: Option<String>` (S5-RUNTIME), filled by `applied_edit_mutations`; `applied_edit_units(position)`
  replaces the per-operation `plans_foreign_steps`: no fold unless an operation of the edit may plan foreign steps, then one
  prefix-ring fold per edit.
- **RAN** (gate v5, `CARGO_BUILD_JOBS=3`): `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib` → **exit 0**
  (`Finished dev in 1m 02s`, plugin lib 283 warnings).
- **Follow-up (not done):** `merge_persisted_history` still builds the typed edits AND folds the foreign log once
  (`parse_decoded_document_spr` → `settle_parsed_envelope`) before it ingests: O(pair operations) in one call. Extract the
  typed-edit conversion (`parse_decoded_document_spr` lines of the edit loop) so the merge skips the settle.
- 05:21–05:22 baseline `cargo check -p semio-framework-os-kernel --lib --tests`: the **lib test target type-checks** (898
  warnings); exit 101 only from the peer integration test `sqlite_snapshot_native_admission` (3 × E0432, known remainder).

### Coordinator relay 05:2x — port rebinding, F6

1. **A port rebinding made A's next draft input `timeTravel.stale`.** `attach_hot_backbone` / `attach_backbone` /
   `detach_backbone` bump the generation; the detach bump is a pinned refusal contract (`🏪️store/🔗️backbone/✂️detach` law,
   `🧪️tests/🔗️backbone-detach` twin, cases `full-destination` and `generation-overflow`) and other consumers read the generation
   as "the store changed", so it stays. **Store half — wave RB STAGED** (`T/🧪️s5-store-content-staleness.py`):
   `commit_finished_replay` is stale on the content revision alone. Law staged (`🧪️viewer-head/🦀️.rs` region `🧪️PortLaws`):
   `a_port_rebinding_moves_no_content_and_keeps_a_finished_replay`. **Session half (S5-RUNTIME):** `TimeTravelBase`
   compares `store_generation` and `content_revision` together (`FW/⏪️time-travel/🦀️.rs:98–102, :623`; TT
   `watch_time_travel_base`); the base must be the content revision alone.
2. **F6 ("Loading document: 0 of 1" stays after Cancel).** Not the store's reprojection status: `reprojection_status()` answers
   `kind: Load` from `live_document_load()` → `ActiveDocumentArchiveLoad::status()` before it asks the store. A cancelled load
   stays listed until it is `terminal()`, which needs maintenance steps nobody drives after the cancel; "0 of 1" is the
   machine's own count. The store offers `ArtifactStoreInitializationAuthority::progress()` (operations folded / to fold) for
   the fold phase. Owner: S5-LOAD.

### "RB ON DISK" — RB + P2 + the Rust laws LANDED 05:40:01–05:42:15 (one hold)

- **RB** (`python3 T/🧪️s5-store-content-staleness.py`): `commit_finished_replay` refuses `Stale` only when
  `finished.revision != content_revision`.
- **P2** (`python3 T/🧪️s5-store-inverse-refused.py`, after S5-PUZZLE's "OUTCOME CODES LANDED" 05:35):
  `EditReplay::replay_operation` — Report mode: a refused inverse is that mutation's `Fatal` `mutation.inverse-refused`, the
  forward folds as `fold_operation` folds it at every site, no inverse rows are staged for it, the replay continues; Merge mode
  still answers `VcsError::InverseRefused`.
- **Rust laws** (`python3 T/🧪️s5-store-land-laws.py viewer-head --rust` / `… supersede-law --rust`), mounted in
  `🏪️store/🧪️tests/🔬️unit/🦀️.rs` as `viewer_head_tests` (6 laws) and `supersede_law_tests` (4 laws).
- **RAN:** `cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib` → **exit 0** (49 s);
  `cargo check -p semio-framework-os-kernel --lib --tests` → **exit 0**.
- **RAN 05:42:30–05:44:30:** `RUST_MIN_STACK=268435456 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s5-store
  cargo test -p semio-framework-os-kernel --lib -- viewer_head_tests supersede_law_tests` → **8 passed / 2 failed**
  (`test-laws-1.txt`).

| Law | Result |
|---|---|
| `viewer_head_tests::two_peers_on_one_folder_converge_through_an_open_history_edit` (F4 acceptance) | **pass** |
| `viewer_head_tests::a_read_back_pair_merges_its_log_and_never_moves_the_reader` | **pass** |
| `viewer_head_tests::the_persisted_pair_restores_edits_history_edits_alternatives_the_head_and_warnings` (P4, store layer) | **pass** |
| `viewer_head_tests::a_port_rebinding_moves_no_content_and_keeps_a_finished_replay` (RB) | **pass** |
| `supersede_law_tests::a_refused_inverse_is_one_fatal_mutation_and_the_session_stays_repairable` (P2) | **pass** |
| `supersede_law_tests::every_input_row_is_admitted_and_folded_as_the_table_says` (12 rows) | **pass** |
| `supersede_law_tests::a_withdrawn_planner_folds_as_a_no_op_at_every_site_and_its_recorded_input_restores_it` | **pass** |
| `supersede_law_tests::every_unit_row_names_its_unit_and_refuses_its_supersession_as_the_table_says` (12 rows) | **pass** |
| `viewer_head_tests::the_viewer_head_corpus_matches_two_stores` | **FAIL** — real defect below |
| `viewer_head_tests::a_finalize_as_a_new_alternative_moves_only_its_author` | **FAIL** — real defect below |

### Defect found by the two red laws — the causal DAG applies a dependent before its buffered dependency (05:50)

Both laws pass every step up to their arrival-order sweep and die there in `ingest_remote` with
`ValidationFailed("malformed history fold at offset 0: branch names unknown checkpoint ck-…")` (backtrace `test-laws-2.txt`:
the sweep's `deliver`). **Cause:** `MutationDag::insert` (`FW/📡️replication/🔗️causal/🦀️.rs`) marks an envelope pending only
when a dependency is "wholly unknown"; a dependency that is buffered but itself still pending counts as met. Arrival
`[Commit, Branch, Supersede, operations…]` (a rotation of the log): the `Commit` waits for the operations it covers, the
`Branch` names the buffered `Commit` and is applied at once, and the store folds a `Branch` without its `Commit`. The crate's
own test comments call it "the `insert`-classification quirk"; `advance_ready_one` already requires every dependency to be
applied. **Live meaning:** a replica that receives a new-alternative finalize before the batch with the edits it commits
refuses the batch.
**Fix STAGED** (`T/🧪️s5-store-dag-pending-rule.py`, 4 anchors resolve): `pending` = some dependency is not applied; doc; DAG law
`a_dependency_that_is_buffered_but_pending_keeps_its_dependent_pending` (b, c, a drains a, b, c). Touches the replication
foundation crate (every dependent rebuilds once): sequencing asked of `main`.

### "DAG RULE LANDED" 06:03:22–06:11:17 (coordinator GO 05:5x, before wave B)

`python3 T/🧪️s5-store-dag-pending-rule.py` (re-read right before: `🔗️causal/🦀️.rs` clean vs HEAD, mtime 10-04 18:27):
`MutationDag::insert` — pending while any dependency is not APPLIED (was: while one is wholly unknown) + doc;
`🔗️causal/🧪️tests/🔬️unit/🦀️.rs` — law `a_dependency_that_is_buffered_but_pending_keeps_its_dependent_pending`, one stale
sentence.
- **RAN:** `cargo check -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin --lib` → **exit 0**
  (1 m 16 s). `cargo test -p semio-framework-replication --lib -- causal` → **58 passed / 0 failed** (the new law included).
- **RAN 06:01–06:02 (before the DAG edit), owed kernel store filters:** `cargo test -p semio-framework-os-kernel --lib --
  deferred_reprojection_tests supersede_replay_tests tool_transaction_tests hot_path_tests os_spr::history::tests` →
  **97 passed / 0 failed** (`test-filters-1.txt`; includes the three N17 local-step laws: an interior undo under a foreign tail
  defers).
- **OWED (stand-back for wave B, 06:12: no new cargo):** re-run of the two red laws after the DAG rule —
  `RUST_MIN_STACK=268435456 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s5-store cargo test
  -p semio-framework-os-kernel --lib -- viewer_head_tests supersede_law_tests` (expect 10 / 0).

### S5-RUNTIME's question — the legacy emit route authors every edit as `local` (06:15)

True, and it is a store defect, not only a route one. `replay_mutations` stamps `author_id:
mutation.author_id().unwrap_or(ActorId("local"))`, the edit's actor is that first operation's author
(`edit_actor_from_meta`), and `apply_command` then REPLACES the store's local actor with it — so a plain
`ArtifactCommand::Apply` of an aggregate without `author_id()` is authored `local` whatever `set_local_actor_id(meta.actor)`
said, on every replica. A peer's ingested edit then carries actor `local` too: `edit_is_local` is true for it, `Undo` may take
the peer's tail edit, and the fold's ownership rule (`foreign`) cannot tell the two authors apart. Whether a non-migrated
document verb still reaches the route is the tool-jobs census's to list (the route is live code: `dispatch_emit` → `Apply`);
the store must make it impossible either way. **Fix plan = P3 widened ("every edit names its author", §22.6), one wave after
LOCKS OPEN:** (1) `replay_mutations` authors with `mutation.author_id()`, else the store's `local_actor_id`; with neither the
command is refused before anything is recorded (a typed `VcsError`, no `"local"` literal left in the store); `local_actor()`
for transition envelopes likewise; (2) `edit_is_local` counts authored edits only, `validate_durable_history` and the paged
`.spr` decoder refuse an unauthored edit (staged: `T/🧪️s5-store-authored-edits.py`); (3) S5-RUNTIME: every route sets
`set_local_actor_id(Some(meta.actor))` before a store dispatch — `dispatch_emit` included; (4) the store's test harness names
its local actor at construction and the hand-built unauthored edits (census: 6 test sites) get one; (5) law: two replicas
through plain `Apply` with distinct actors — each undoes only its own edit, the other's is refused by the fold.

### Stand-back for wave B (06:12 →) — state and the exact next steps

Everything below is staged; nothing of mine is half on disk (every wave script answers `pending: none` for what landed).

| Step (in order, at "LOCKS OPEN") | Command | Lock |
|---|---|---|
| 1. re-run the two laws the DAG rule fixes | `RUST_MIN_STACK=268435456 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s5-store cargo test -p semio-framework-os-kernel --lib -- viewer_head_tests supersede_law_tests` (expect 10 / 0) | none |
| 2. wave AA (`documentArchiveAbsent`, both twins, schema + corpus + both laws) | `python3 T/🧪️s5-store-archive-absent.py --rust-law` (22 edits + the Rust law; `--check` first) → `cargo check -p semio-framework-os-kernel -p semio-framework-plugin -p semio-framework-os-renderer-wgpu --lib` and `cargo check -p semio-framework-os-kernel --lib --tests` → os TS `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🔨️modules/🏪️store/👷️worker/🟦️.ts -t "folder archive restore"` (expect 6) + `tsc` of the os package → `cargo test -p semio-framework-os-kernel --lib -- the_folder_archive_presence_corpus` | `landing` then `serve` |
| 3. P3 widened (every edit names its author; no `"local"` fallback) | store: `T/🧪️s5-store-authored-edits.py` (4 anchors: `edit_is_local`, `validate_durable_history`, the paged decoder) + `replay_mutations` (`:22224`) and `local_actor()` (`:20346`) authoring with the store's local actor or refusing typed; harness actor; 6 hand-built test edits; needs S5-RUNTIME's `set_local_actor_id` on every route in the same window | `landing` |
| 4. dead-`Checkout` deletion | census + order in "`Checkout` deletion wave" above | `landing` + `serve` |
| 5. P6 replay cost | notes in "More owed runs" above | `landing` |

Wave AA additions since the first staging: the TS law (the corpus through the real worker inside `folder archive restore`,
the codec round trip and the strict-parse refusal) and the Rust law
(`the_folder_archive_presence_corpus_matches_the_native_actor`, 7 of 9 cases through `native_actor::ArtifactActor` over a temp
folder) are wired into the script (`🗑️generated/s5-store/stage-aa/laws/*`); all 22 anchors still resolve at 06:26 and every
edited Rust file parses, both TS files transpile. Never compiled, never run.

Also open from the brief, not started: 8-crate `wasm32-wasip2` check (gismap initializer fix on disk, unverified:
`cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-gis-gismap --lib`), the `merge_persisted_history` settle-fold
removal, the 11th `command-rejection` code for a retired document port (O4), `history_lines()` for the main-line row.

### Session-5 summary at the stand-back (06:45)

**Landed (4 `landing` holds, each released green):** FW 01:10 · PM 05:11 · RB + P2 + Rust laws 05:40 · DAG rule 06:03.
Files: `🏪️store/🦀️.rs` (regions `🔖️EffectiveForwards`, `🔖️TimeTravelReads`, `🔖️Supersede`, `🔖️PairMerge`, `🔖️EditReplay`,
`SpaceMember`), `🌿️vcs/🦀️.rs` (`UnitSpansDocuments`), `📡️replication/🔗️causal/🦀️.rs` (+ unit tests), gismap initializer region,
new `🏪️store/{🧬️schema/🔣️,🧫️fixtures/🧫️,🧪️tests/🧪️}{viewer-head,supersede-law}`, mounts in `🏪️store/🧪️tests/🔬️unit/🦀️.rs`, os
`test-store-oracles` registration.
**Ran:** kernel + plugin lib checks ×4 exit 0; kernel `--lib --tests` check exit 0; os `test-store-oracles` 22 / 0; worker
`folder archive restore` 5 / 0; kernel store filters 97 / 0; replication `causal` 58 / 0; new store laws 8 / 2 (the 2 reds
= the DAG defect, fixed on disk, re-run owed).
**Not done:** P1's `Checkout` deletion, P3 (staged; widened by the legacy-emit finding), P6, wave AA (staged complete),
8-crate wasip2 check.

**Wave B is rewriting the tree under the staged waves (06:14 →, S5-CHANNEL):** it removed `description` from
`ArtifactCommand::Apply` in my two landed law files (correct; the staged copies are re-synced from the tree, `land-laws --check`
= none pending) and changed the paged edit decoder, so `T/🧪️s5-store-authored-edits.py` no longer resolves its
`decoder-write` anchor (`anchor occurs 0 times`) — **P3 must be re-derived against the post-wave-B store** (it is widened
anyway). The plugin-level draft `stage-p4/folder-reload-law.rs` (S5-LOAD's to take) still passes `description:` to `Apply`.
Wave AA's 22 anchors resolved at 06:45 during wave B; re-run `--check` after "WAVE B LANDED".

### Resume 07:27 ("LOCKS OPEN", build B1 live, channel 22)

- **RAN 07:27:43–07:30:02** the owed re-run: `cargo test -p semio-framework-os-kernel --lib -- viewer_head_tests
  supersede_law_tests` → **9 passed / 1 failed** (`test-laws-3.txt`). `a_finalize_as_a_new_alternative_moves_only_its_author` is
  green with the DAG rule.
- **Second real defect found by `the_viewer_head_corpus_matches_two_stores`** (backtrace `test-laws-4.txt`: `act` → `deliver` →
  `ingest_remote_merge` → `bump` → `assert_mirrors_are_live`: "incremental applied revision records … first stale record
  Some(1), 3 live vs 3 rebuilt"). The merge of a remote edit renumbers every applied edit by its applied position (store
  `:22754`) and marks the revision accumulator dirty from the insertion point `k` only; `sequence_number` is part of an edit's
  revision digest (`edit_digest`: the edit's JSON, or the chained header). Corpus case 2, replica `a` on its alternative:
  step 8 ingests `b`'s trunk edit — the merge orders it into the applied order (numbers 1, 2, 3) and the reproject that
  follows drops it again (not visible at `a`'s head; `a` keeps 1, 3); step 12 ingests `b`'s edit on the alternative at the tail
  (`k = 2`) and renumbers position 1 from 3 to 2 without re-digesting its record. Release builds have no oracle: the live
  content revision silently differs from a rebuilt one.
  **Fix STAGED** (`T/🧪️s5-store-ingest-renumber-dirt.py`, 1 anchor): the renumbering reports the first position it changed,
  the accumulator is dirty from `min(k, that)`; one ledger walk instead of one ledger search per applied edit (O(n²) → O(n)
  per remote ingest). Lands with wave AA.
- **Wider finding, not fixed (design list):** `sequence_number` is replica-local and path-dependent (a live store numbers by
  applied position, a reload by ledger position; a hidden edit keeps the number it got while briefly ordered, so numbers can
  also repeat across applied and hidden edits) and it is inside the edit's revision digest — the content revision is not a
  pure function of (log, head). Taking it out of the digest touches the one-item sealer's canonical edit bytes
  (`🏪️store/🧵️canonical-edit`).
- Wave AA after wave B: `python3 T/🧪️s5-store-archive-absent.py --rust-law --check` → all 22 edits + the Rust law pending,
  every anchor resolves.

### "AA ON DISK" — wave AA + renumber fix LANDED 09:36–10:01 (resume after the 07:45 cut; rules 64–67)

- Applied 09:37 in one hold: `T/🧪️s5-store-archive-absent.py --rust-law` (10 files: `🔄️sync/🦀️.rs`, `👷️worker/🟦️.ts`,
  `💻️os/🟦️.ts`, wgpu shell arm, schema, corpus, three test files) and `T/🧪️s5-store-ingest-renumber-dirt.py`.
- RAN: `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-archive-restore/🟦️.ts` → 6 pass / 0 fail (09:47);
  `tsc` over the three AA TypeScript files (ticket-local tsconfig) → 0 errors; `test-store-oracles` → 22 / 0.
- `serve` released 09:48, `landing` released 10:00:46; restore script `T/🧪️s5-store-restore-aa.py` (hunk-exact inverse,
  `--check` clean); train line `10:01:05 S5-STORE AA+renumber 10 files`. Train first pass GREEN 10:05 (kernel + plugin +
  wgpu + ui `--lib`) on a tree holding both → the coordinator relayed "AA ON DISK" to S5-LOAD.
- Rule 67 (landing train) from here on: a hold is apply-only, no cargo inside it, one line in `coord/train.txt`, the
  framework-closure verdict is read from `coord/train.status`.
- NOT RUN: the Rust law `the_folder_archive_presence_corpus_matches_the_native_actor` — the `sync` module is mounted
  only with `--features sync`, my filter without the feature matched 0 tests. OWED:
  `cargo test -p semio-framework-os-kernel --lib --features sync -- the_folder_archive_presence_corpus`.

### Design §22.28 — `sequence_number` leaves the revision digest (wave C, channel 23) — STAGED 10:19, not landed

**Problem.** `Edit.sequence_number` is where an edit stands in ONE replica's ledger. Three writers disagree about it: a
live store numbers the applied edits by applied position after every remote merge (`ingest_remote_merge`); the history-log
reload numbers every edit by ledger position (`parse_decoded_document_spr`, `sequence_number: index + 1`); an edit of a
line the replica does not stand on keeps the number it arrived with. The member was inside the edit's revision record
(single-operation: inside the hashed JSON; chained: a 4-byte part of `edit-chained`), so the content revision was not a
pure function of (log, head).

**Witnessed, not only argued (10:21).** The staged law, temporarily in the tree, is RED under the current rule: corpus
case `a-new-alternative-moves-only-its-author`, reversed arrival, a replica that stood on the trunk throughout — its own
`.spr` reload names another content revision than the replica showed (`🗑️generated/s5-store/test-laws-5.txt`).

**Rule after wave C** (confirmed with S5-CHANNEL, who reseals fixtures / schema / TS twins):
- single-operation edit: `record("edit", [id, JSON(edit WITHOUT its sequenceNumber member)])`, other members in the
  same order (id, actor?, forwards, inverse, mutationMeta?, verb?, startedAt, finishedAt?, line);
- chained edit: `record("edit-chained", [id, actorTag, actor, startedAt, finishedAtTag, finishedAt, n+chain ×3])` — only
  the 4-byte sequence part is gone; `edit-verb`, `edit-line`, the three chains unchanged;
- the one-item sealer streams exactly that JSON (its Edit field table 10 → 9, child ordinals 6/7/8 = startedAt /
  finishedAt / line), so sealer digest == store digest; its byte count comes from the same traversal;
- `CursorRevisionAccumulator::revision_value(edit)` is the one oracle of "what the identity covers" (store digest and
  the five Rust test oracles use it);
- the remote merge's renumbering dirties revision records from the insertion point `k` only (the 09:37 fix's wider
  dirt range is no longer needed: a renumbered edit keeps its digest);
- `Edit.sequence_number` stays on the type, the wire, `.spr`, `.ops`; `authority_digest` keeps `next_sequence_number`
  (preparation identity). `validate_durable_history` still requires unique non-negative numbers.

**What changes on disk or wire: nothing.** The revision digest is replica-local (session base, staleness, projection
stamp); it is in no `.spr` / `.ops` / wire / hub record. The only pinned bytes are the canonical-edit fixtures
(`🔏️canonical-edit-sealer.json`, `🗺️canonical-borrowed-map.json`: `expectedJson` + `expectedDigest`;
`📖️canonical-reader.json`: `expectedByteLength`, `expectedJsonSha256`; `🔗️edit-digest-chains.json`: every
`expectedDigest`), their schema and two TS twins — S5-CHANNEL's half. The fixtures' `edit` inputs keep `sequenceNumber`.

**Proof law (staged, `viewer_head_tests::a_content_revision_names_a_head_whatever_line_its_replica_stood_on`).** For
every case of the viewer-head corpus a fresh replica takes the whole log in every arrival order (log order, reversed,
every rotation) while standing on each line of the case — it steps onto a line the moment the line is listed, so the
other lines' edits (a peer's trunk edit, for a replica on an alternative) reach it while it shows something else. Then,
per line: the replica's own persisted pair restores exactly the revision it showed (R1), and all replicas name ONE
content revision for the line whatever they stood on and whatever order the events came in (R2). The assertion message
carries each variant's arrival, standing line and ledger positions.

**Limit, recorded (R3).** Author and receiver still hold different records of one edit: a received edit has
`started_at: ""`, `finished_at: None`, `payload_hash: None`, `base_version: 0`, `undo_policy: ExactBaseOnly`, no
`group_id` / `origin` — none of these are on the wire. So an author and a receiver name different content revisions for
the same head. No product path compares revisions across replicas today (the token never leaves its replica); making
R3 hold means either carrying those members on the wire or taking them out of the identity — a later decision.

**Encoder order (coordinator's exposure question, 10:10).** Checked in code:
- revision digest: never calls `encode_wire_value`; hashes `pack_json::to_json_string`, i.e. value order (record members
  in `to_value` declaration order, intrinsic objects in stored order; `pack_json::Object` is an insertion-ordered `Vec`),
  not key-sorted. Its TS twin hashes `JSON.stringify` in document order. It relies on a reload giving intrinsic objects
  back in stored order — the unsorted pack encoder guarantees that, the sorting one did not. The supersession digest
  covers payload bytes as stored;
- sealer: owns its form — explicit field tables for edit / meta / clock / origin / target, payloads walk the mutation's
  own canonical traversal in stored order;
- archive pair: compared as raw bytes only (`folder_held_hash`, `last_written_hash`, TS `folderArchive`;
  `initial_digest` = blake3 of the genesis pack bytes, a re-encode of a decoded genesis is order-preserving). Drift only
  across builds (sorted vs unsorted genesis re-encode → "other genesis" refusal), which the channel bump covers. The TS
  worker's `placeholderPayloadHash` over the sorted TS encoding is never verified by Rust.

**Staged:** `python3 T/🧪️s5-store-revision-digest.py --check` → 13 hunks pending in 6 files
(`🏪️store/🦀️.rs` ×4, `🧵️canonical-edit/🦀️.rs` ×3, `🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs` ×2,
`🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs` ×1, `📖️reader/🧪️tests/📖️reader/🦀️.rs` ×2,
`🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs` ×1); `--only rule|law`, `--revert`, `--emit <dir>`. Emitted copies parse
(`rustfmt` over the four test files and, via stdin, the store and the sealer) and the hunks are rustfmt-stable. The law
compiled in the 10:19 kernel test build (1 m 55 s, 892 warnings). Landing order on "WAVE C GO": this script → S5-NESTED
N2 → S5-CHANNEL's reseal → bump. Until the reseal lands, the Rust laws reading those fixtures are red by construction.
A surrogate probe that predicts the post-change outcome (`🗑️generated/s5-store/debug-revision-surrogate.py`,
`[DEBUG]`, applied and reverted, never run: the gate stayed closed and the disk fell below the test floor) is kept for
the first test window.

### Defect CL — a commit on an alternative did not depend on the alternative's registration (found 10:21, fixed 10:36)

- Symptom (the corpus law, once the renumber fix let it run further): `ingest_remote` →
  `ValidationFailed("malformed history fold at offset 0: commit ck-… names unknown alternative alternative-…")`.
- Cause: `pending_checkpoint_transition` declared the commit's operations and its parent checkpoint as dependencies,
  never the `Branch` that registered `Commit.line_id`. The history fold refuses a commit whose line it does not list
  (`🔗️causal/🔀️transition/🦀️.rs:760`), so a receiver holding the operations and the parent applied the commit before
  the registration and refused the event. Every other fold refusal already has its declared dependency (commit → parent,
  commit → operations, branch → checkpoint, repin → checkpoint); scoped supersessions fold tolerantly.
- Fix LANDED 10:36:37 (apply-only hold, train line): `T/🧪️s5-store-commit-names-its-line.py --only rule` — store
  `alternative_origin(alternative_id)` + one `dependencies.extend(…)` in `pending_checkpoint_transition`. One more
  dependency id on newly authored `Commit` events; no codec, wire or persisted format change.
- Focused law STAGED (`--only law`, `a_commit_on_an_alternative_waits_for_the_alternative`): the commit names the
  registration among its dependencies; reversed and every rotation converge on the author's registrations and projection.
  Not landed: compile-unverified until a test build is allowed.

### Runs and blocks 10:19–10:36

| when | command | result |
|---|---|---|
| 10:19–10:21 | gate → `cargo test -p semio-framework-os-kernel --lib --no-run` (private target dir, jobs 3) | exit 0, 1 m 55 s, 892 warnings |
| 10:21 | `… cargo test -p semio-framework-os-kernel --lib -- viewer_head_tests supersede_law_tests the_folder_archive_presence_corpus` (witness law in the tree) | 9 passed / 2 failed of 11: witness law RED on R1 (by design), corpus law RED on defect CL; archive law matched 0 tests (needs `--features sync`) |
| 10:24–10:34 | same build for the `[DEBUG]` surrogate probe | never started: gate closed 10 min (5 shared cargos at 0 % CPU), moved to background by the tool cap, I killed my own waiting gate; disk 30 → 16 → 13 GiB (test floor 25) |
| 10:35 | revert of probe + witness law (one hold) | tree as before (`--check`: all 13 hunks pending) |

OWED (needs ≥ 25 GiB free and train GREEN through CL):
`zsh T/🚦️gate.sh 3 25 && RUST_MIN_STACK=268435456 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s5-store cargo test -p semio-framework-os-kernel --lib -- viewer_head_tests supersede_law_tests`
— expect 10 / 0 before wave C, 11 / 0 after it (12 / 0 with the CL law).

### Wave C LANDED 10:58:37 (by S5-CHANNEL, inside the coordinator's hold) — and my fix-forward 10:59:56

- S5-CHANNEL ran `python3 T/🧪️s5-store-revision-digest.py` (13 hunks, 6 files) → NESTED's N2 → its own reseal; channel 23.
  FRAMEWORK train GREEN 11:02:30 through wave C (`--lib`).
- **My defect, found by my own test build and fixed forward (test-only):** three of the five Rust test oracles named
  `crate::os_store::CursorRevisionAccumulator`; the store module is `crate::os_store::component` (the accumulator is
  private to it, not re-exported) → `E0433` at `📖️reader/🧪️tests/📖️reader/🦀️.rs:84,339` and
  `🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs:222`. `--lib` cannot see test files, so neither my `rustfmt` parse check nor the
  train caught it. Fix `T/🧪️s5-store-revision-digest-oracle-path.py` (path → `crate::os_store::component::…` in the two
  test files and inside the wave script, which still answers `pending: none`); train line written. Lesson: a hunk in a
  test file is only staged once a TEST build compiled it — the witness law was compiled at 10:19, the oracle hunks were
  not.
- Kernel test builds (private `CARGO_TARGET_DIR` + `CARGO_BUILD_BUILD_DIR`, no shared lock, no gate):
  10:49:59–10:59:09 cold, exit 101 (my 3 × E0433 + 1 foreign error); 11:00:14–11:02:27 warm, exit 101 with ONE error
  left, not mine: `🏪️store/🧪️tests/🔬️unit/🦀️.rs:9712` `ChildDispatch::borrowed(child_ref,&source,&schema,&labels)` →
  `E0599` (test `dispatch_group_borrowed_child_keeps_exact_sources_on_policy_refusal`, fixture `🫳️child-dispatch`; the
  test file changed 10:32, the store has no `ChildDispatch::borrowed`). Reported; the coordinator routed it to S5-NESTED.
  It blocks every kernel `--lib` test build, so NO law of mine ran after wave C.
- CL's focused law (`a_commit_on_an_alternative_waits_for_the_alternative`) is in the tree since 10:47 (test-only hold)
  and COMPILED in both builds above (no error names it); not run.

### P3 → B3 (10:47), shape decided by the coordinator: design §22.34 — the actor is a required constructor argument

Why not the refusal before B2: only the trait default answers `Mutation::author_id()` (`None`; two test types override
it), so every store that applies without a bound actor authors through the two `"local"` literals. A refusal is a
run-time change no `--lib` check sees. Census of `ArtifactStore::new` (227 sites, `git grep`, 11:03):

| crate / area | production | tests |
|---|---|---|
| kernel `🏪️store` | 6 (`:3837 :3858 :11500 :11663 :26775 :28293`) | 164 |
| kernel `🔌️plugin` | 2 (`:25559` bound by wave H, `:38871`) | 12 |
| kernel `🏃️run` / `🪐️space` / `🛢️db` / `🌊️flow` | 1 (`🏗️bootstrap/🦀️.rs:229`) / 1 (`:461`) / 0 / 0 | 0 / 3 / 1 / 1 |
| `🖥️host` | 1 (`:829`) | 2 |
| `✏️s` plugins (draw 8, wfc 4, gis 3, raster 3, shooting 2, fem 2, lowpoly 2, playbook 2, remodel 2, stdio 2, trinity 1, space 1, sourcing 1) | 0 | 33 |

plus 52 `set_local_actor_id(` calls and 13 `.local_actor_id()` reads in 4 files. §22.34 (written into `📋️design.md`):
`ArtifactStore::new(envelope, actor)` and every retained initializer take the actor; the store holds `local_actor:
ActorId`; authoring is total (mutation's author, else the store's); genesis / codec `apply_ops` / plain test stores pass
`LOCAL_ACTOR_ID`; `edit_is_local` authored-only; unauthored edits refused by `validate_durable_history` and the paged
decoder. `cargo check --tests` enumerates every unbound site, so the codemod is fail-closed at compile time.
NOT STAGED YET: the codemod needs a `--tests` check to prove itself (the lesson above) and cargo is closed for
activation B2. The stale `T/🧪️s5-store-authored-edits.py` stays unapplied and is superseded.

### Design §22.35 — a document is identified by its genesis pack's stored bytes (S5-LOAD's finding, 11:05) — decided, not staged

Read in code: `initial_digest` is `blake3(initial_snapshot.encode_pack())` on EVERY route (store `:17769`, `:18046`,
hydration `:306`, eight plugin initializers through `store::artifact_initial_digest`), while the two same-document tests
hash a peer's STORED bytes (`merge_persisted_history` `:23677`, backbone `verify_genesis` `:23031`), and
`print_document_pack` re-encodes the genesis on every persist. The two sides agree only while decode → encode is
byte-stable, which an encoder change (member order of typed records) breaks: one document reads as two, the pair is
loaded instead of merged, the hub refuses a peer's `Genesis`.
Decision (written as §22.35): the genesis pack is encoded once, at creation; the envelope keeps the bytes beside the
decoded snapshot; identity = blake3 of the stored bytes; every load keeps the bytes it decoded from; persist and the
backbone `Genesis` emit them verbatim. Rejected: a store-owned canonical key-ordered form (a second encoder contract —
member order, number forms — that drifts the same way; needs a decode + sort per read-back) and a recorded digest
without the bytes (the stored pack could no longer be checked against it).
Plan (B3, ONE wave with §22.34 — both change `create_document_envelope` / `ArtifactStore::new` and the same 227 sites):
1. `ArtifactVcs` gains the immutable genesis bytes; `create_document_envelope` encodes once; `parse_document_spr`
   keeps the bytes it decoded from; `artifact_initial_digest(initial)` (the re-encoding form) is deleted.
2. `print_document_pack` and the backbone `Genesis` writer emit the kept bytes; hydration and the eight plugin
   initializers hash the bytes they read (incrementally, no second materialisation).
3. Law "two encoders, one document → merge": a test snapshot type with two byte forms of one value; a replica created
   with form A, a replica that cold-loaded A's pair in a process whose `encode_pack` yields form B: B's persisted pack
   is byte-equal to A's, each merges the other's pair (`PairMerge`), both name one content revision, and a pair whose
   genesis bytes differ is refused as another document. TS twin: the worker already treats the archive as opaque bytes.

### Kernel law run after the reboot (16:19–16:35) — everything of mine is green; wave LO landed

Full table, every red with its owner: `T/📓️s5-kernel-law-run.md`. In short:
- `viewer_head_tests supersede_law_tests` **13 / 0** — the §22.28 law, CL's law and the corpus law all pass. This is the
  first run that verifies wave C (the §22.28 law was RED before it: live ≠ own reload), wave CL and the renumber fix.
- canonical-edit laws **25 / 0** (S5-CHANNEL's reseal agrees with the sealer and the store digest); replication `causal`
  **58 / 0** (DAG pending rule); `--features sync` archive-presence law **1 / 0**, whole `os_store::sync::` **88 / 0**
  (the wave AA Rust half, never run before); `os_spr::` **343 / 0**; `the_document_archive_load_host` **1 / 0**.
- `os_store::` 514 / 1 and the whole binary 1188 / 6: five deterministic reds, none in a file a fleet wave changed (four
  are the Codex peer's in-flight object-member-order work in the pack encoder, one is an `os_dsl` record refusal message),
  and one test-isolation flake (passes alone).
- **Wave LO, my red fixed forward (16:25):** a live store listed changes, checkpoints and alternatives in ARRIVAL order
  (`adopt_history_facts` kept known facts in place and appended new ones) while the fold — and a reload — list them in
  log order. `T/🧪️s5-store-ledgers-follow-log-order.py`: `order_ledger_as` after adoption + the focused law
  `the_ledgers_list_their_facts_in_log_order_whatever_order_the_events_arrived_in`. Fourth defect this corpus law has
  surfaced (DAG pending rule, renumber dirt, commit → branch dependency, ledger order), each hidden behind the previous.
- Harness note: a test binary run directly needs `SEMIO_TEST_ARTIFACT_DIR` and the package directory as cwd
  (`os_spr::io::native::…caller_cancellation…` panics without the variable).
- §22.34 + §22.35 remain decided-not-staged; the precondition "this run is green" holds for everything the fleet owns.

## Session 5 — 2026-10-06

### Wave RR — a refused command retires what its replay built (design §23; S5-AGNOSTIC's raster abort) — rule APPLIED 01:45, UNVERIFIED; law STAGED

**Bug (read in code).** `ArtifactStore::replay_mutations` kept its working projection (`pre_snapshot.clone()`), the command's
operations and their inverses as plain locals. Of its four refusal exits only the refused inverse retired them; the encode
failure (`?`), the failing diff (`applied?`) and the policy rejection (`return Err(VcsError::Rejected { .. })`) let them fall
to drop glue — and a projection owning a fail-closed root (raster's owned map) panics there:
`drop_glue::<RasterSnapshot>` ← `replay_mutations` ← `apply_command` ← `dispatch`
(`T/🗑️generated/s5-agnostic/raster-g12-backtrace.txt`). Its three callers had the same shape after a successful replay:
every `?` between the replay and the adoption (`replace_local_actor_retained`, `reserve_edit_history_slot`,
`operation_envelopes`, the open edit's lookup, `record_edit_messages`, …) dropped the same owners bare, and
`replace_current_retained` dropped the projection it was offered when it refused.

**Census of the same shape.** `replay_mutations` was the ONLY replay with bare locals: the folds hold their intermediate in
`ReplayProjection` (retires on drop), the history replays own their state in `EditReplay` / `EditReplayResult` /
`EffectiveOperation` (all `Drop`), `replay_operation` retires `back` on its own refusal, the remote merge retires its batch
through `retire_scratch_edits`.

**Fix — one mechanism** (`T/🧪️s5-store-replay-retires-on-every-exit.py --only rule`, 10 parts, `🏪️store/🦀️.rs`,
+92 / −54): `ScratchOwner<T>` holds a scratch owner and retires it through a given cold retirement on EVERY exit path
until `adopt` hands it to the store.
- `replay_mutations` holds projection / operations / inverses as `ScratchOwner`s and RETURNS them as such (its loop is
  index-based so no reference is held across the `await` — no new `Sync` bound on `Mutation`);
- `apply_command`, `open_transaction_edit`, `append_transaction` adopt each part where the store takes it; the edit
  `apply_command` builds is a `ScratchOwner` until `insert_reserved_edit_history` takes it;
- `replace_current_retained` wraps the offered projection and retires it (`retire_shared_projection`) when it refuses.
Sites: 4 exits in `replay_mutations`, 5 + 3 + 2 early returns in the three callers, 2 in `replace_current_retained`.
Restore: `--revert-rule` (hunks swapped back, the old span from the sidecar, digest-checked before writing).

**Law — STAGED, not applied** (`--only law`: `🏪️store/🧪️tests/🧪️replay-retirement/🦀️.rs`, corpus
`🧫️fixtures/🧫️replay-retirement/🔣️.json` (7 cases), schema, one mount line in the store unit tests):
`replay_retirement_tests::a_refused_apply_retires_everything_its_replay_built` — a store over a demo operation and a demo
diff with a fail-closed owner's discipline (cold retirement and bare drops are counted; the diff's technology counts every
projection it produces and every scratch projection retired through it). For an encode failure, a refused inverse, a
failing diff and an error the merge policy rejects — at the first, a middle and the last operation — the store answers the
refusal the corpus names; every operation of the command and every inverse derived from it retired cold, none reached a
bare `Drop`; projections retired == projections applied + 1 (the working copy); the store shows what it showed; it still
applies an edit and closes to its terminal-empty witness. The fixture counts instead of panicking in `Drop` because the
demo snapshot is shared by every store test; the count is the same proof (zero bare drops, every scratch projection
through `retire_projection`).

**Not verified — said plainly.** No cargo ran: when the wave was ready the disk had fallen from 9 to 3 GiB (6 at 01:46),
swap stood at 19.9 of 21.5 GiB, and the disk keeper had pruned my private target dir (a cold kernel build would be ~9 min
and several GiB). The emitted store and law file parse (`rustfmt`), the corpus and schema are valid JSON, every anchor
matched its exact count and the old `replay_mutations` span matched its digest. The rule waits for the FRAMEWORK train
lane; the law waits for a kernel test build:
`P=…/⚡️cache/cargo/target-nde-s5-store; python3 T/🧪️s5-store-replay-retires-on-every-exit.py --only law && RUST_MIN_STACK=268435456 CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$P CARGO_BUILD_BUILD_DIR=$P cargo test -p semio-framework-os-kernel --lib -- replay_retirement_tests viewer_head_tests supersede_law_tests`
(expect 1 + 9 + 4 = 14 / 0), then S5-AGNOSTIC's reproduction
`cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-raster-raster --lib -- --exact editor::raster::component::unit_tests::context::history_edits_end_to_end`.

### Wave RR verified (02:25–02:33): train GREEN 01:47, law landed and green, raster no longer aborts

- Rule: FRAMEWORK train GREEN 01:47:02 through the RR line (coordinator relay). Law applied 02:25 (apply-only hold, train
  line): `replay_retirement_tests` + corpus + schema + mount.
- RAN (private target + build dir, jobs 3; kernel test build 3 m 15 s, exit 0):
  `replay_retirement_tests viewer_head_tests supersede_law_tests` → **14 / 0** (1 + 9 + 4), outputs `🗑️generated/s5-store/rr-laws.txt`;
  whole `os_store::` → **517 / 1**, the one red is the Codex peer's known
  `…sqlite_snapshot_native_physical_pack_record_preserves_literal_table_and_numeric_tags` (object member order, unchanged).
- The law's discrimination is by construction, not by a run against the old code: before RR every refusal but the refused
  inverse dropped `FailClosedOp`s live (`dropped > 0`) and retired `applied` instead of `applied + 1` projections.
- Product evidence: S5-AGNOSTIC's batch re-ran raster at 02:03:33 on the tree holding RR
  (`🗑️generated/s5-agnostic/family-raster.test.txt`): zero "reached Drop before" panics (the 01:17 run aborted in
  `drop_glue::<RasterSnapshot>`); refused seed edits now come back as `Rejected { policy: Normal, .. }` rows in its census.
  The family is still 3 passed / 1 failed on the acceptance CENSUS (13 of 22 editable leaves exercised; 9 named with
  reasons) — the raster family's own finding, not a store abort. The raster reproduction was therefore not re-run by me.
- NOT RUN: `--features sync -- the_folder_archive_presence_corpus`. The build at 02:30:51 did not compile — 18 errors, all
  in files a peer saved at 02:27:02–02:27:43 (`📡️spr/🦀️.rs:44` unresolved `os_spr::command::{DiffCodec, OpBinary,
  OpText}`, `🗣️dsl/🦀️.rs:210–250` and `🗣️dsl/🧪️tests/🔬️unit/🦀️.rs` "cannot find `binary` in `io`"; the replication crate's
  `🚪️io`, `⚙️codec`, `🎮️mutation` changed in the same minute — the Codex peer's wave in flight). None in a file of mine;
  my non-sync build had compiled replication before those saves. Last green run of that law: 2026-10-05 16:34 (1 / 0,
  whole `os_store::sync::` 88 / 0); wave RR does not touch the sync module. OWED when the tree compiles again:
  `… cargo test -p semio-framework-os-kernel --lib --features sync -- the_folder_archive_presence_corpus`.
