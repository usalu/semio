# FEM 3D results playback (F9): Play/Pause, Step, reload

Status: panic NOT reproduced natively; pressed-state fix landed (3D + 2D); browser re-probe on a fresh wasm still owed.

## Findings
- The alias assertion is `SharedOriginalStorage::drop` (`value/retirement/allocation-return/shared/original`, line 152): a counted alias dropped without retire/transfer. The FEM crates own no `SharedOriginalStorage` (grep: none). Playback logic (`set_result_animation`, `result_animation_tick`, transient clock) lives once in the 2D crate and 3D reuses it; it holds no aliases.
- Framework owners of such aliases on the tick path: tool-run `ToolRunEntry.entities/payload`, legacy `ToolRunView` (derives `Clone`, so every clone is a counted alias; its constructors `ToolRunEntry::view`, `ToolRunLedger::views/view/view_for` have no non-test caller). Not changed: another agent owns arc-custody (`fix-seven-tests.md`).
- `resultAnimationTick` was declared a Migrated retained job in the manifest after the 12:30 wasm; the audit notes this was never verified in the browser.

## Native laws (3D editor_app_2, `C:\t\pk`, logs `🗑️generated/pk/law-a*.log`)
- Existing, all pass (a8): 6 `result_animation_tick::*` laws (advance+rearm, park when stopped, once-run stop, tagged gestures, config-lane ledger, playback presses).
- New `pause_over_an_inflight_tick_retires_every_alias_and_steps_still_work` (a9): Play, tick, open a tick WITHOUT settling, Pause lands, settle twice, straggler tick, Step +/-, close runtime. PASS.
- New `panel_wire_play_pause_and_step_leave_no_alias_behind` (a11): same through the panel's wire args (`field`/`value`/`windowId`) with the host's received view state. PASS.
- New panel law `results_panel_play_control_is_a_pressed_toggle_that_follows_the_transport` (a12, see below).
- Helper change: tick unit file `open()` and `transport_wire()` now use `receive_view_fixture` as the 2D twin does.

## Fix: button never pressed (`aria-pressed` null)
The play control was a plain button whose label/icon swapped. Now a `ui::toggle` (button appearance) with `on == playing`, Change dispatches `setResultAnimation{field:"playing"}`.
Files: 3D and 2D `editor/panels/results/🦀️.rs` (`playing_toggle`), 3D panel unit law.

## Framework functions changed
None.

## Open
- Browser panic is not reproduced by any native law; it needs a wasm rebuilt from the current tree (coordinator) and a fresh probe: Play, Pause, Step +, reload.
- Four `results_window` laws from the coordinator's full run (mounted_results_frames_never_invoke_synchronous_solvers, playback_phase_moves_the_deformed_scene, results_scene_captions_name_mode_and_factor_3d, results_window_surfaces_solver_error_without_panicking_3d): panic text was not in the log; not yet reproduced.
