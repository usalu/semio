# L1a Rust 2D test call-site migration

Result: `cargo check --workspace --all-targets --keep-going` of the Puzzle 2D workspace (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d`) exits 0 with zero errors (log `🗑️generated/l1a-check-8.log`, `exit=0`). Down from the 88 sites in 24 files across 9 test crates left by L1.

## Runner

`l1a-session.sh 2d` (this folder): the L1 tmpfs unshare runner with `--jobs 2`, kept alive as one session so the tmpfs build dir is reused. Each check or test is requested by writing `🗑️generated/l1a-req` (first line: log name, then cargo args). Quit with a `quit` request. Fresh per-run builds (`l1-native.sh`) rebuild the whole closure every time (about 10 minutes); the session makes re-checks incremental.

## Fixes (root causes, no shims)

- Unit tests of the root crate (`◻️2d/🧪️tests/🔬️unit/🦀️.rs`): records are imported from `semio_s_artifact_puzzle_2d_records` (the carve moved them); the 4 per-name imports became one import list with all catalog and anchor types.
- `protocol::Mutation::<Puzzle2dSnapshot>::diff/inverse` (recorder test, `🧪️recorded-kinds`, `🧪️select-tool`): `Mutation` no longer carries `diff`/`inverse`; they live on `protocol::ColdMutation`. Tests call `ColdMutation::<..>::diff/inverse` and import `ColdMutation as _`.
- `Puzzle2dCommand` (`✏️editor/📸️snapshot/🧵️command-variants/🦀️.rs`): `from_action` was `cfg(test)` only, invisible to the carved test crate; now `cfg(any(test, feature = "testkit"))`. Tests import `crate::editor::puzzle2d::snapshot::Puzzle2dCommand` (`🔬️unit`, `🧪️recorded-kinds`, `🧪️select-tool-transactions`).
- `puzzle2d_manifest_for_tests` in `🧰️testkit/🦀️.rs` made `pub` (used by `history_edit_acceptance_law!` from the carved crate).
- `BODY_KEY` in `🎯️board-event-and-ui-scope-laws`: wrong path; uses `selection::BODY_KEY` of the selection window.
- Board helpers in `🧪️select-tool-transactions` and `🧪️select-tool-history`: `set_camera_silent`, `set_transform_flags`, `set_selection_ids_silent` are now free functions in `engine::board_host::testkit` (previous L1 re-export); the history test passes `(scene, focus, outbox, ids)` instead of calling the removed `BoardHost` method.
- `puzzle2d_dispatch_emit` is generic over the editor: tests pass `::<Puzzle2dPlayApp>`.
- `ArtifactEditor::command_from_action(action, &mut Option<DslValue>, &mut Option<Fault>)`: gumball and presence tests updated; `DslValue::from(&json!)` replaced by `semio_framework_pack_json::to_dsl_value`.
- Presence law (`🔬️unit/🦀️.rs`, `mounted_camera_and_interaction_presence_...`): `assert_presence_round_trip` no longer takes a command/view/interaction. Puzzle 2D's app presence is empty (`Puzzle2dPresence {}`), so the test now publishes the camera and node through `dispatch_fixture_action`, asserts no document edit, and runs the generic law with a unit probe. Runtime behaviour of this reduced law is unverified (the law asserts the ephemeral lane carries a non-empty presence; with an empty app presence this may fail at runtime).
- `VcsArtifactApp` deref in `🧮️operations-and-command-envelope-laws`: `&mut *reopened.0` is `&mut reopened.0`.
- `load_board_snapshot_json` takes the board state parts: helper `load_into_default_host` in `🧩️engine-parse-and-handle-suggestion-laws`.
- Ingest idempotence test: envelopes are `PagedBytes`; decoded with `protocol::decode_envelopes` and re-encoded with `protocol::encode_envelopes` before `ingest_operations`.
- Sqlite snapshot test (13 sites): migrated to `semio_framework_test_original_snapshot_owners` wrappers `decode_native`, `encode_native`, `export_sqlite`, `import_sqlite` (native decode/encode now take an output slot and return `RetainedWorkOutcome`). Script `l1a-migrate-sqlite-owners.py`. Added the `semio-framework-value` dependency to `◻️2d/📦️packages/🦀️rust/Cargo.toml` (test uses `semio_framework_value`).
- `issuer.generation().0` of the owned export test: `generation()` is a plain `u64`.
- Lookup test (`🧬️schema/📸️snapshot/🔎️lookup`): `Puzzle2dLookupCursor::advance(source, target, effect)` lost its body parameter.
- Derived retained-clone cursors (`🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone`): no `advance`; new helper `test_source_custody::advance(cursor, source, body, maximum, grant)` (in `◻️2d/🧪️tests/🔗️source-custody/🦀️.rs`) drives `advance_into` through a real original reply, with recovery re-entry.
- Text preparation tests (`mut-any-5`): private cursor fields read from another crate. Added testkit-gated accessors `Puzzle2dTextPreparationCursor::child_cursors_terminal_is_empty()` and `Puzzle2dTextCandidateCursor::phase()`. Message compose cursor still takes a grant; the earlier effect migration of its test is reverted.
- Canonical mutation test: `RetainedCloneSource::admit_owned` and `project_owned` are gone. New testkit helpers `test_source_custody::admit_owner` (owned source admission under the effect bank) and `test_source_custody::project` (retained owned projection via `RetainedOwnedProjectionAdmission::admit/take`); cursor `advance(bytes, effect)` and `close_step(4096, effect)` through `run` and `close_turn`. The per-step heap receipt equality for the close step was dropped (the driven turn already asserts the physical receipt).
- Brush fill tests (`⚙️engine/🖌️brush/🧪️tests/🔬️unit`): capture `step(construction, scene, style, brush, cx)` with a persistent `OriginalConstructionAdmission`; close closures return `RetainedWorkOutcome`; `admit_pending(config, &mut output, cx)`; mounted sessions are pumped on the original caller via `pump_one_for_interactive_turn` (the worker pool submit protocol is gone, pool creation and shutdown removed); acknowledge via `acknowledge_checked_out_outcome(cx)`; cancel alias return through `cx.with_original_effect`; stale-job law uses `OriginalFixture::step_job`. Removed two obsolete laws that exercised `WorkerPool::try_submit` refusal/saturation (`board_fill_worker_saturation_returns_exact_session_for_close` and the refusal half of the refusal test, renamed `board_fill_unclaimed_complete_closes_exact_owners`). Script `l1a-migrate-brush.py`.
- `semio-s-plugin-puzzle-jobs` (`🎮️commands/📤️jobs`): a concurrent edit added `step_job_observed` using `semio_framework_trace` in the `testing` module while the crate only had it as dev-dependency; made it an optional dependency enabled by the `testing` feature.

## Tests

Not run to completion. `cargo test --no-fail-fast` for the 5 crates that formerly failed to compile (`schema`, `mutations`, `mut-any-5`, `text-inverse`, `connect-handles-preparation`) exhausted the 7 GB tmpfs during dependency codegen (`failed to build archive ... No space left on device` in `semio-s-artifact-puzzle-3d-mutations-catalog`; the 2D workspace links the 3D crates). Pass and fail counts: 0 executed. /home is full, so the build dir cannot move to disk; a rerun needs freed disk or a larger tmpfs.

## Remaining

- No compile errors in the 2D workspace.
- Unverified at runtime: the reduced presence law, the heap-receipt assertions of the canonical test and of the derived-cursor test (they now go through effect-bank driven turns), and the brush mounted-fill laws on the original caller.
- Logs: `🗑️generated/l1a-check-*.log`, `l1a-test-1.log`.
