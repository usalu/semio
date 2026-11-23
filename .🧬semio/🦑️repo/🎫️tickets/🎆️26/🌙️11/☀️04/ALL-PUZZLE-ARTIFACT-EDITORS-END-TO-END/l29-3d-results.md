# L29 3D Native Test Results (partial, run 3d-r3, 2026-11-23)

Built cleanly (5g tmpfs, all 36 `semio-s-artifact-puzzle-3d*` crates spilled to `/dev/shm`). The low-memory reaper stopped the run at 13:0x during `editor-gestures`, so the later targets (`editor-panels`, `editor-surface`, …) did not run. Log: `🗑️generated/l23-3d-r3.log`. Failed names: `🗑️generated/l23-3d-r3-failed.txt`. Some names appear twice because the same test lives in two targets.

| target | result |
|---|---|
| puzzle-3d unit | 10/10 ok |
| puzzle-3d snapshot-sqlite | 14/14 ok (the L17b scale/blank-SQL fix is confirmed) |
| 3d-editor unit | 1/1 ok |
| editor-action-producer | 0 passed, 2 failed |
| editor-budgets | 64 passed, 9 failed |
| editor-defects | 42 passed, 11 failed |
| editor-gestures | partial: about 46 ok, 6 failed before the stop |

## Failure clusters (by first message line)
1. **Import wire budget (L17b fix not effective):** "the plugin declares its import wire budget", plus "a product-sized import inside the budget is admitted: Fault …", "every chunk of a product-sized pick is admitted: Fault …" and "an over-budget import is an ANSWER, never a fault: Fault …". These are the import/pick/over-budget tests and `target_volume_action…`.
2. **Close custody / terminal-empty:** "context root retains its original projection or Store issuer", "original Puzzle work remains in close custody", "registered fixture did not reach its exact terminal-empty witness after 4145 turns". These are `context::closes_mid_reserved_commit` and `context::conflict_history_edits_end_to_end`. L24's `TimeTravelDiscardCursor` fix did not clear them.
3. **Interactive ceiling (nakagin):** `accept_suggestion_*`, `open_vortex_suggestions_*` and `set_active_example_*` each "stays below the interactive ceiling". A `[DEBUG] unposted-origin …` probe from another agent is in this build.
4. **Clipboard:** "paste must emit one mutation edit: []" (`copy_then_paste…`), and `gesture_kinds::cancelling_clipboard_admission_*`.
5. **Source-string laws:** `importSnapshot`/`exportSnapshot` "must leftover-commit through Puzzle3dWindowCommandWork, not BoundedFirstStep".
6. **Perf:** `mutation_latency::one_mutation_publishes…` (L21) and `selection_scale::one_pick_builds_the_interaction_topology_once…` (L17b topology counter).
7. `settlement_work_ledger_is_finite_and_progresses` and `leftover_export_snapshot_downloads_the_boot_example_json`.
