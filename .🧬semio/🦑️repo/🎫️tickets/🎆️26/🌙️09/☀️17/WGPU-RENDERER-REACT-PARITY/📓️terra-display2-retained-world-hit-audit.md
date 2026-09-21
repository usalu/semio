# Display2 Retained World Hit Audit

## Scope

Read-only follow-up to the two Native136 Display2 failures. This report reflects the source after the fixture corrections already made by the owner. No build or runtime probe was run here.

## Disposition

Both original failures were fixture mismatches, not established production defects.

- The expandable Display-template test used `apply_tree`, then sealed and acknowledged input. That bypassed the retained document ingress/reconcile path that creates the next candidate after a disclosure action. The corrected fixture now uses `panel_ui_records`, a `UiDocumentLease`, and `step_document_reconcile` before each paint/ACK. Production already routes chevron activation through the retained EventRouter and marks/reconciles the tree.
- The created-window test expected `ComponentScene` for a World scene. Canonical input classification explicitly maps `SurfaceKind::World3d` to `HitKind::World3d`; the stale `ComponentScene` expectation was the direct cause. The updated assertion uses the live contract.

## Real Created-Window World3d Path

The new instance has a concrete, bounded route from its published document to the physical hit:

1. Shell's phase 3 gives the dock body a `ScrollRegion`; phase 4 takes the retained lease and calls `render_ui_document_step` with the solved window rect. [Shell wgpu](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23717)
2. The interpreter admits document pages, reconciles, lays out until that window is clean, and only then advances retained paint. [Interpreter wgpu](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2637)
3. A scene slot mounts its runtime identity and calls `render_component_scene_step`; the World branch creates/gets the host state and emits the actual 3-D scene pass. [Interpreter scene host](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2470) [Scenes wgpu](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3259)
4. The engine walks hits only after painting and publishes its candidate registry before returning `Ready`. [UI engine](../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2644)
5. Shell then reads that published registry, clips it to the solved body, and stages it above the body `ScrollRegion`. [Shell registry bridge](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12970) [Interpreter clip bridge](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:833)
6. Classification is unambiguous: World3d publishes a `World3d` hit, whereas generic component scenes publish `ComponentScene`. [UI input](../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:712)

The fixture's root is itself the World surface. Its default vertical Stack layout does not leave a nested scene with zero growth: the engine assigns the root viewport to that root. There is no source evidence that layout growth drops this instance's hit.

## If the Corrected Target Is Still Red

Do not change the World3d classifier or add an immediate hit fallback. The smallest diagnostic law should assert the handoff in this order for `main-2`:

1. `render_ui_document_step` completes without a terminal paint fault.
2. `Ui::window_hit_targets("main-2")` contains exactly one registration with `kind == World3d` and a nonempty rect.
3. `register_clipped_retained_hit_targets("main-2", window_rect, ...)` retains that registration after clipping.
4. Shell's staged and ACKed input both retain that exact World3d registration.

A failure at 1 is document/scene paint; at 2 it is retained layout or scene-slot publication; at 3 it is concrete body geometry; at 4 it is Shell staging/ACK. This keeps the regression on the actual presented path and distinguishes a zero-sized rect from an absent scene host.

## Confidence

High for the two original root causes and the source route. No pass claim: the corrected targeted Native run remains the required runtime evidence.

