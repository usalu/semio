# grid2d editor status

- Build/test: `repo cargo --store=isolated --configuration=prototype --parallelism=2 -- test --manifest-path <grid2d>/Cargo.toml --workspace --no-fail-fast` queued behind `repo` capacity admission (needs 3355 MB). Log: `🗑️generated/grid2d-repo-isolated-test.log`. Plain cargo is refused by the compiler gate; `--store=shared` was stopped after 3+ min capacity wait.
- Editor: there is no separate editor Cargo crate. The editor (`standards/1/subsets/any/editor`, ~4k lines: Grid2dEditor, window, modes/edit with fill tool, grid and preview windows) is path-included into `semio-s-artifact-wfc-grid2d-v1` and re-exported through the artifact crate `editor::grid2d`. 2d has per-concern editor crates (config, transient); grid2d does not.
- TS: `📦️packages/🟦️typescript` (`@semio-tech/wfc-grid2d`) has only build/check/test scripts via `repo run`; no vitest files found. Fixture `🧪️tests/🧫️grid2d-action-arguments`.
- Wiring: hub composition `🌎️hub/🧩️compositions/🀄️wfc/🦀️.rs` declares `Grid2dEditor`/`Grid2dViewer` apps, artifact, inference factory and mutation rosters; component manifest `🧩️component/🔣️.json` has `s.wfc.grid2d@1/*#editor` with windows `wfc-grid2d-grid` and `wfc-grid2d-preview`; bridge `🏭️bridge/🦀️.rs` lists `s.wfc.grid2d`.

## Snapshot API port (done, not yet compiled)

Edited (grid2d): `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (host delegates to the text native with the `output` slot; dropped `admit_sqlite_values`) and its tests (`decode_native`/`encode_native` helpers), `🚪️io/💾️binary/📸️snapshot/🦀️.rs` (removed `decode_into`), `🧪️tests/🔬️store-fixture/🦀️.rs` (removed decode-in-place tests), `🔨️modules/🏠️host/💡️inferences/🦀️.rs` (`close_effect`, `?` patterns), the editor fill tool `close_step`, and `🧬️schema/🧬️mutations/🦀️.rs` (`original_pack_value_operation_receiving!`). The text native snapshot was already ported. Build is blocked by the window-config 262144 byte crate ceiling (infrastructure); rerun `repo cargo --store=isolated ... check --workspace --tests` once unblocked.
Open risks: the SQLite tests at lines 37, 39 and 41 still call `provider.import/export` and `io_export_sqlite_snapshot(...,native_owner)` with the old owner shape; only the compiler will show whether those need `original_owners::import_sqlite/export_sqlite`.

## Triage of `grid2d-test-final8.log` (232 passed, 32 failed) and static fixes

- Shared-owned (not touched): 17 tests fail with `grid2d-inference-headless-teardown-retains-original-owner` (inferences x8, examples x2, fill x3, preview x5, `the_commit_fill_command_writes_solve_json`), `aborting_mid_run_cancels_and_never_dispatches_commit_fill` ("need a live partial", same in wfc3d/grid3d), `history_edits_end_to_end` and `conflict_history_edits_end_to_end` (terminal close "original native producer omitted observed native work"), and `the_inference_job_publishes_a_twenty_five_byte_preview` (no preview, same headless path). The SQLite native frontier test is the wfc2d agent's decode-cancel fix (`fundamental-fixes.md`).
- `store_fixture::authored_seed_and_pins_survive_actual_pack_spr_replay`: "terminal ticket has no explicit native header lifecycle" was the framework lifecycle declaration fix already logged in `fundamental-fixes.md`; expected green after rebuild.
- Fixed (grid2d tests/code):
  - `🧬️schema/🧬️mutations/🧪️tests/🔬️unit`: the two cascade inverse laws apply the inverse list reversed (`into_iter().rev()`).
  - `🔨️modules/🏠️host/💡️inferences/🧪️tests`: the descriptor leaves test resolves each format with `resolved_leaf_body` (the descriptor serves four leaves from repository documents, an empty body defers).
  - Action-forms contract: the app builder's `action_args` only searches app-scope actions, so window-kind actions had no args. `windows/🔲️grid` now owns `pub fn action_args` attached in its own definition and the preview window reuses it (`set-camera`, `setActiveExample`); the chained builder calls were removed from the editor.
  - Editor tests: `retire_run` destructures `ToolRunView` and retires `provisional_entities` and `payload` (editor unit test, preview unit test; the stale second run in the mounted preview test is retired too); the armed-utility test retires `active_utility_by_window_id` via `retire_cold`.
  - Removed the previous agent's `[DEBUG]` print from `terminal_is_empty` in `💡️inferences`.
- Run: `🗑️generated/grid2d-test-01.log`.
- Coordinator rule: runs after grid2d-test-01 use --store=shared (SEMIO_OFFLOAD=never kept); 01 was started isolated and is left to finish.

## grid2d-test-01.log (isolated): 240 passed, 24 failed
- All my static fixes held (inverse laws, descriptor leaves, action forms, alias retire, armed utility no longer fail).
- Shared-owned (22): headless teardown x17 (inferences 8, examples 2, fill 3, preview 5 incl. commit-fill), abort "need a live partial" x1 and "no preview published" x1 (headless), history x2 ("original fold retirement requires its actual Source decoder").
- Other (2): sqlite `encode_native` with `max_rows: 2` returns Ok at tests line 81 (wfc2d agent's sqlite area; decode cancel now passes); `store_fixture` replay now got past the lifecycle error and failed with "snapshot root admission was not funded" because the quoted close grant was too small; fixed by using the siblings' generous grant (`1 << 20` items, `1 << 30` bytes, depth 256). Verification run: grid2d-test-02.log (shared store, filtered).

## Full shared-store Rust suite after the solver and fixture fixes

`grid2d-rust-continuation-01.log` records 262 passes and two failures across the eight workspace members. The failures are `history_edits_end_to_end` and `conflict_history_edits_end_to_end`. Both now reach an archive hydration owner with an unreceived original failure during teardown. The targeted diagnostic run is queued in `[DEBUG] grid2d-history-first-cause.log`. The SQLite frontier, store replay, fill, preview, and inference suites passed.
