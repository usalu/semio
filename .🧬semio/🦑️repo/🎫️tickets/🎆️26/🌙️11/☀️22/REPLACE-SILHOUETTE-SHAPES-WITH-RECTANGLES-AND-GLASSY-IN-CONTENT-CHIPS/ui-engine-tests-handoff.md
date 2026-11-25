# semio-renderer-wgpu-ui-engine-tests: handoff (59/59)

Start: 43 passed / 16 failed (`shell-chrome5-select-handoff.md`, run r13). End: **59 passed / 0 failed** (`repo cargo --store=shared --configuration=prototype -- test -j 3 --no-fail-fast -p semio-renderer-wgpu-ui-engine-tests --lib -- --test-threads=2`, log `scratchpad/uiengine/b6.txt`). The crate has no doc tests of its own; the `--lib` flag was used only because the doc-test target forced a world rebuild at every iteration.

All 16 failures were stale fixtures. No product code was changed.

## Root causes per class

1. Work loan (9 laws, `original work loan callback replaced its initiating context or refilled its work`): the paint helpers drove `render_ui_document_step` through `FixtureTurnV1::turn`, which quotes one fuel unit. Fixed with `turn_funded(1 << 20, ..)` (pattern 2 of `shell-all-handoff.md`).
2. Retained paint then never completed (ceiling of 2^20 opportunities, 150 s): `FixtureTurnV1::new` grants the narrow `UI_WORKER_RETIREMENT_POLICY`, the paint demand exceeded it and `render_ui_document_step` returned without progress. Fixed with `FixtureTurnV1::wide(policy, 1)` (one wide caller at generation 1 as in production).
3. `original fixture root closure never reached terminal` (the same 9 laws, once 1 and 2 were fixed): every paint helper created its own caller, finished it while the window was still mounted, and a later helper closed the window with another caller. A layout session can only be driven and retired by the caller that birthed it. Fix in `🧰️surface-fixtures/🦀️.rs`: one long-lived surface caller per law (`take_surface_caller`, `keep_surface_caller`, `with_surface_caller`), owning every window it paints; `retire_surface_caller_window` pays the close of one window on it (serving the host component close after each turn) and finishes the caller once it owns no more windows. `close_retained_surface_fixture` is now that function; `paint_retained_map_document` and `paint_retained_surface_in_window` use the same caller; the sibling-rebase law closes through it instead of its own caller. `FixtureTurnV1` gained `disown_surface` and `owns_surfaces`. The caller wrapper finishes under `catch_unwind` if a failing law leaves it in thread-local storage.
4. `original world resources dropped before paid close` (5 laws): `paint_scene` (world3d) built a `World3dBuildContext` per frame and dropped it unpaid, and never prepared its slot pages (`original slot page remains owned` once the drop was fixed). It now owns a wide caller, prepares the context with `prepare_step` in funded turns, and retires it through `FixtureTurnV1::retire_world` before the context drops.
5. `layout never settled` (`a_painted_chrome_window_reports_draw_calls_quads_and_glyphs`): the test drove `Ui::step_layouts` with an ad hoc `StepContext`, which is not the original caller the layout worker binds to. Now uses the testkit `UiFixture` (`settle_layout`), the same fixture the engine's own laws use. The unused `UNBOUNDED_STEP_GRANT` const was removed.
6. `40.0` vs `40` (`map_device_loss_recovery_restores_the_live_camera_on_retry`): compared `Number(40.0)` with the JSON integer; now compares typed `Option<[f64; 3]>`. Same assertion, no weakening.

## Further fixture changes found on the way

- `FixtureTurnV1::retire_world` (`🧪️tests/🎟️original-turn/🦀️.rs`): the close step now runs in a funded turn (`CLOSE_TURN_WORK`); with one unit the world close ladder failed with `original effect performed more work than its ceiling` in every world-owning law (benefits all Shell fixtures that call it).
- `unchanged_production_frames_advance_selection_intent_and_released_capture_cancels`: the helper republished a generation-1 document for every paint, so a changed scene (the selection cancellation authority) never reached the retained tree. Surface helpers now publish a successor generation exactly when the scene content changes (the mounting host id, which the law echoes back from the first paint, is ignored); the same scene repainted stays the same document (the tiled-map law asserts the same host for the same generation).
- `🏠️os-host/🧰️fixtures/🦀️.rs` `finish_cpu_only_fixture_component_close`: its kind whitelist now includes `Paint2d`, whose retired scene (replaced by a successor generation) hands the host a component close request; production `advance_component_surface_close` accepts any kind.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧿️ui-engine-tests/🧰️surface-fixtures/🦀️.rs` (surface caller, generations, funded and wide turns)
- `…/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/{🗃️fixtures-and-retained-map-records,♻️retained-map-gesture-lifecycle-and-bridge,🚪️engine-close-and-world3d-fixtures}/🦀️.rs`
- `…/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs`
- `…/🧑‍🎨engine/🧪️tests/🎟️original-turn/🦀️.rs`
- `…/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🧰️fixtures/🦀️.rs`

Product bugs: none. Fixture fixes: all of the above.

## Not verified

- The Shell test crates were not re-run after the shared `FixtureTurnV1::retire_world` and os-host fixture whitelist edits; both are strictly more permissive (funded turn, one more allowed kind).
- Runs were repeatedly delayed by other agents' in-flight compile errors and by one `exit status: 137` (rustc killed at the 3.9 GiB budget), all cleared on retry.
