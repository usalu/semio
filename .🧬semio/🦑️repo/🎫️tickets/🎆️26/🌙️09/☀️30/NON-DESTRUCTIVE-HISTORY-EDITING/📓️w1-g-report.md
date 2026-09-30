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
