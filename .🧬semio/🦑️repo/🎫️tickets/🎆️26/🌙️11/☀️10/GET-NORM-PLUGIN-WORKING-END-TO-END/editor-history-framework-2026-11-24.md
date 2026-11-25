# Editor history framework failures (2026-11-24)

Reproduced on en1990 (`semio-s-artifact-norm-en1990-editor --test integration`, script `run-editor-history.sh`, logs in `🗑️generated/h-*.log`).

## Root causes

### 1. `undo_redo_round_trips_through_the_wrapper` (4098 turns, stage 18, zero demand)

Measured with a temporary `[DEBUG]` dump of every `maintenance_ladder_settled` clause: the only unsettled clause is `tool_cancellations.active_operation_count() == 0`. One keyed scope (operation 128, a reserved framework route) is registered, not finished and not cancelled, and the registry `Arc` has a second strong holder. `cleanup_slot` (stage 18) only frees finished scopes and `live_cleanup_awaits_custody` answers `AwaitingInput`, so the stage correctly makes no demand and no progress. The lease belongs to `pending.pending_reserved`: `dispatch_framework_reserved_action` (`undo`/`redo` are `framework_reserved_job!` routes) admits the permit and returns `Effect::SpawnJob`, and the host must run that job.

The settle loop was therefore not a framework liveness bug but a test that dropped the effect: the norm editor tests called `dispatch_fixture_action("undo")` and then plain `settle`. The framework law `artifact_app_laws::settle_history_verb` exists exactly for this (admission, `settle_framework_reserved_admission`, then publication).

Fix (test driver, no assertion changed): in every Norm editor unit test `undo_redo_round_trips_through_the_wrapper` the `dispatch_fixture_action` plus `context::settle` pair is replaced by `settle_history_verb(&mut app.0, "undo"|"redo", ...)`.

### 2. `history_edits_end_to_end`: "original report requires its installed semantic and retirement issuers"

Measured at `🔭️history-preview/🦀️.rs:270` (`[DEBUG]` print of the owner flags): `replay_preparation` and `one_item_preparation` absent, snapshot/initial/mutation issuers present. The default `ArtifactEditor::build_document_store_owners` is `bounded_artifact_store_owners` (no semantic issuer). Apps with replay opt in to `cold_replay_document_store_owners` (generation3d does). The Norm editors never did.

Fix: `norm_exact_store_ownership!` (both arms) now overrides `document_store_owners_source_demands` and `build_document_store_owners` with the shared cold replay catalog.

### 3. `conflict_history_edits_end_to_end`: "archive load did not settle ... DecodeForward: Replay: truncated at offset N"

Measured with a `[DEBUG]` print in `step_native_mutation`: the stored op bytes are `[1, 6, '{"newReferencePeriodYears":1.0}']` (`format`, `tag`, canonical JSON, the frame of `payload_op_binary`), but the receiving was `original_pack_value_operation_receiving!`, a compile-fix placeholder (`wave-en0-6.md`) that parses a Pack wire value. It read the `{` byte (123) as a length and ran off the end. Every Norm family has its own hand-written `OpBinary` frame (payload JSON, tagged DSL, varint+JSON), so none of them matches the Pack receiver.

Fix: new framework receiving `OriginalDecodedOperationReceiving` (`🗣️dsl/🫴️receiving/🧾️decoded`), macro `original_decoded_operation_receiving!`, re-exported by `semio_os_kernel`. It keeps the funded protocol (paged copy quote of `len + size_of::<Option<M>>()`, one decode step through the authored `OpBinary::decode_op`, take, paid close) and uses the authored codec as the only decoder. All 15 Norm mutation binary files use it instead of the Pack placeholder.

### 4. After 2 and 3: "original operation has no authored conflict path issuer"

The wire installed by `cold_replay_document_store_owners` is `ArtifactOperationWire::canonical()` without a conflict target. The Norm owners now install `semantic::<Snapshot>().with_batch_digest().with_original_target(whole_document_target)` (the documented explicit declaration, same as semio graph, jack and rewriting).


## Results (en1990, `semio-s-artifact-norm-en1990-editor --test integration`)

Last completed run (`unit_tests::undo_redo_round_trips_through_the_wrapper`, `history_edits_end_to_end` filter, which also matches the conflict test):

```
test unit_tests::undo_redo_round_trips_through_the_wrapper ... ok
test unit_tests::context::history_edits_end_to_end ... FAILED
test unit_tests::context::conflict_history_edits_end_to_end ... ok
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 48 filtered out; finished in 53.16s
```

- `undo_redo_round_trips_through_the_wrapper`: passes (fix 1).
- `conflict_history_edits_end_to_end`: passes (fixes 2, 3, 4).
- `history_edits_end_to_end`: the earlier errors are gone ("installed semantic and retirement issuers", then "no authored conflict path issuer", then "installed operation wire has no original receiving capability"). It now stops at `operation output payload extent exhausted` (`ownershipLimit`, `ArtifactPreparedOperationOutput::append` in `🏪️store/🧵️operation-wire/📦️output/🦀️.rs:57`). The per-operation payload limit is fixed at `262144` bytes in `🧵️operation-wire/📤️edit/🦀️.rs:68` (`admit_paged(.., 262144, 1<<20, ..)`), so one edit of the history-edit scenario serialises to more than 256 KiB. Not yet known which edit; a `[DEBUG]` backtrace print was added to find it and removed again, because the run that would have printed it never finished.

Not verified: en1997 and en1998 (not run), and the full editor suite after the owner catalog change (the swap of `bounded_artifact_store_owners` for the cold replay catalog and the new wire may change allocation ledgers asserted elsewhere; the earlier note about `(x, y) != (3200, 0)` in `artifact-app-laws` is the first thing to re-check).

The last four cargo invocations (45 minute cap each) never reached the test run: host compile capacity waited 10 to 20 minutes per run, and other agents' edits to framework crates recompiled 200 to 600 units each time, so the last run was cancelled during link. Nothing of mine was left running.

## Open question for the next pass

Find what makes one edit in `history_edits_end_to_end` exceed 262144 bytes. Candidates: an inverse that snapshots a whole subtree, or the edit that carries every operation of an example document. If the edit is legitimately large the limit at `🧵️operation-wire/📤️edit/🦀️.rs:68` is a framework constant to quote from the edit rather than fix; do not raise it blindly.

## Changed files

Framework:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧾️decoded/🦀️.rs` (new: `OriginalDecodedOperationReceiving`, `DecodedOperationRetirement`, `original_decoded_operation_receiving!`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs` (mounts and re-exports the new module)
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📥️mounts/🌱️wp1-leaves/🦀️.rs` (kernel re-export of the macro)

Norm:
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🛡️store-ownership-macros/🦀️.rs` (cold replay owners, `semantic` wire with batch digest, binary receiving and whole-document conflict target, both arms of `norm_exact_store_ownership!`)
- 15 `🚪️io/💾️binary/🧬️mutations/🦀️.rs` files of the Norm artifacts (en1990 to en1999, din4108, din16798, din18599, iso16757, vdi3805): `original_pack_value_operation_receiving!` replaced by `original_decoded_operation_receiving!`
- 13 editor `🧪️tests/🔬️unit/🦀️.rs` files (en1990, en1991, en1992, en1993, en1994, en1995, en1996, en1997, en1998, din16798, din18599, iso16757, vdi3805): undo and redo use `settle_history_verb`

Ticket:
- `run-editor-history.sh` (runs one editor integration test target), this report

All temporary `[DEBUG]` instrumentation of this pass was removed (maintenance ladder clause dump, cancellation scope dump, history preview owner flags, hydration mutation bytes, output payload backtrace, `close_diagnostic` print); `git diff` against the last archivist commit shows only the removals, because that commit captured the debug lines while they were in the tree.
