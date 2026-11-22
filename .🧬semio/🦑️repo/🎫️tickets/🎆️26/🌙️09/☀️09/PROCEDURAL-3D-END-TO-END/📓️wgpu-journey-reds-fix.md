# 🧭️ wgpu journey reds: re-audit against current source (2026-11-22)

Method: every Sept red (verification 2026-09-14 §5.1-5.6, regressions sweep 09-15, readiness audit 09-13) was
re-read against the current source. No browser (6118 is down) and no Rust run: `repo cargo test -p
semio-os-infinite-world-host` stops at the source-boundary guard
`semio-os-infinite-world-interaction (active lines 4084/4000)` before the compiler (not this lane's file).
No source was changed by this lane.

| row | still present? | evidence |
|---|---|---|
| world3d-editor wheel | Cause removed. `AppWheel` accumulator no longer exists. | Wheel is per event: `handle_pointer_wheel` -> `scene_pointer_target_at` -> `apply_scene_wheel` (frame-actions/interaction-state); ScrollRegion refusal fixed by W12c (`wgpu-wheel-and-escape-routing` law). Needs a browser re-run of h7 in edit mode. |
| world3d-viewer translateSelection | Fixed. | `WorldSessionState.declared_action_ids` published by `ShellDockState::sync_world3d_declared_actions` from the window kind; `world3d_offers_transform_gumball` gates gumball/handles; viewer kind declares no transform verb. Fixture `World3dHost/tests/fixtures/surface-verbs.json` + Rust law `world host tests/surface-verbs` (not run, build blocked). |
| picker / modes / roles | Fixed. | `shell_example_control`, `shell_mode_controls`, `shell_role_controls` paint; `shell-hit` arms for `playground.navbar.fixture|roles.*|modes.*` and `shell.example.<id>` dispatch `setActiveExample`; navbar chord `mod+alt+e/v`, mode step. |
| status-a11y-i18n | Fixed in source. | Mirror refresh moved to frame channel and spans all windows (a11y-status-i18n-runtime §2, mirror code now in `wgpu/accessibility-mirror`); German via palette proven in that lane; producer now publishes a non-idle chain phase (`preview_progress_status_json_for`: `chain.working` -> `computing`, `cancellable` via the `toolRunStart` re-arm fix of 09-15) and `world3d_status_pill_for` paints it. Needs browser re-run of `status:pill-while-computing`. |
| port-fit Fit camera | Real gap, not fixed. | Flakiness cause (input starved by `settle_ui_chain`) was closed by the gesture law in settle-pump. But the consumer is gone: `node_graph_fit_camera` (EngineCanvas node-graph) has zero callers, the `shell node-graph fit` log and the `F` arm (`keyboard_fit_surface_id`) no longer exist, and `nodeGraph.fitGraph` is a label with no control in wgpu or React. The battery step tests a removed feature. |

## Recommended follow-up for port-fit
Add one shared declared action (schema first) `nodeGraph.fitGraph` that both hosts dispatch: wgpu via
`node_graph_fit_camera` + the same `graph_interaction_snapshot` persist path `node_graph_wheel_into` uses; React via the
existing `dagFitCamera`. Not done blind without compile access.

## Browser checks needed
h7 wheel in edit mode (8 examples); status pill while an example computes; German via palette; viewer shift-click on
gumball area (no `translateSelection` fault); picker/mode/role clicks; port-fit after the fit control exists.
