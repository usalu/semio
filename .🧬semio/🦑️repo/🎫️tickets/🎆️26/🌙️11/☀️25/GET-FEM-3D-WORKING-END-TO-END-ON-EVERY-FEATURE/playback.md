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

## 2D twin
`result_animation_tick` laws in the 2D editor_app_2 target: 7 passed (a13); panel toggle ported to the 2D results panel (not yet run).

## results_window laws (test-side fixes, no render/solve path edits)
- `playback_phase_moves_the_deformed_scene` (PASS, c1): asserted lowercase `phase 0.42`; the caption is `Phase 0.42 · ▶`. Assertion corrected.
- `results_scene_captions_name_mode_and_factor_3d`, `results_window_surfaces_solver_error_without_panicking_3d`: rendered once and saw `Analysis queued` (results solve in async jobs). Now drive the jobs with the existing `pump_results` and close them with new `drain_results` (cancel and step to bounded close) so a pending caption fails with its text instead of aborting on `original registry job retained work after its bounded close`. First run (b3) showed that abort in the modal case: the modal solve did not settle (browser: `Mode 1: 0.000 Hz`, `Computing modes`), owned by the modal agent. Re-run pending (builds blocked by other agents' framework edits).
- `mounted_results_frames_never_invoke_synchronous_solvers`: owned by the F3 agent.

## Final laws and status (native, `C:	\pk`)
- 3D `result_animation_tick::*` existing 6 laws PASS; new `pause_over_an_inflight_tick_retires_every_alias_and_steps_still_work` PASS; new `panel_wire_play_pause_and_step_leave_no_alias_behind` PASS.
- 2D `result_animation_tick` 7 PASS; 2D panels lib 6 PASS; 3D panels lib 9 PASS (incl. new `results_panel_play_control_is_a_pressed_toggle_that_follows_the_transport`).
- `playback_phase_moves_the_deformed_scene` PASS (caption assertion corrected).
- `results_scene_captions_name_mode_and_factor_3d`, `results_window_surfaces_solver_error_without_panicking_3d`: still FAIL. Pumped with `pump_until_settled` (200-round chunks, 20 min cap): both stay on `Preparing analysis` for 600 rounds, about 2.2 s per round (1423 s and 1290 s), even the empty document. Per-stage count: one stage, `Preparing analysis` x 3 chunks. Needs the F3/modal owners; no solver or render path touched.

## Root cause of the F9 panic
NOT FOUND. No native law reproduces `shared original alias must retire or transfer` (play, tick, pause over an in-flight tick, wire route with received view state, step, close all pass). Candidates on the framework side: alias owners on the tool-run entry/view path and the sealed-source handback (`arc-custody/sealed`, owned by another agent, not changed). Needs a wasm built from the current tree and a fresh browser probe (static case: Play, Pause, Step +, reload); build 3 predates the toggle and ran in Modal mode.

## Open
- Browser re-probe on fresh wasm.
- The two results_window laws above.
