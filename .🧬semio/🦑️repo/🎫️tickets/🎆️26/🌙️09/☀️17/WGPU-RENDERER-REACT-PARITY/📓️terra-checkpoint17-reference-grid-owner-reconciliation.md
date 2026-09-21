# Checkpoint 17b Reference Placement and Grid Owner Reconciliation

## Scope and Evidence

This is a read-only review of the fresh, already captured activation. It did not drive a browser, change source, or run Cargo/tests. It compares:

- `🗑️generated/astra-runtime/checkpoint-17-full/react/02-dismiss-tour.png`;
- `🗑️generated/astra-runtime/checkpoint-17-full/wgpu/02-dismiss-tour.png`;
- the paired camera records in `🗑️generated/astra-runtime/checkpoint-17-full/cameras.md`;
- the current React/WGPU renderer paths and the installed `@react-three/drei` implementation.

The fresh paired images still show the floor-plan/slab relationship as non-congruent and show WGPU grid strokes substantially darker and more jagged. They also still lack mesh outlines, but outlines are deliberately outside this review and remain owned by `📓️astra-sol-world-glb-outline.md`.

At `dismiss-tour`, the two live camera records are close enough to reject a camera-seed explanation for the Top-plane observation: maximum position/target difference is `0.00001` and zoom difference is `0.00049` (the latter accompanies a `0.06562` CSS-pixel viewport difference). Perspective has `0.00005` maximum position difference and `0.00001` target difference. This says nothing about reference payload delivery, because the WGPU checkpoint diagnostics do not expose reference records, decoded dimensions, or the emitted textured-instance model.

## Reference Plane: Equivalent Live Source, No Proven Placement Owner

The actual React host filters hidden references and passes only this reachable shape into `WorldReferenceLayer`: `id`, image URL, `origin`, `widthWorld`, `locked`, and `opacity`.

- React host publication: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:7583-7597`.
- React plane size is `[widthWorld, widthWorld / naturalAspect]`; the group pose is applied before the plane is drawn: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3940-3943`, `4018-4025`.
- The retained WGPU record has exactly the reachable fields, including `id`, `origin`, `width_world`, `locked`, `opacity`, and `hidden`: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:576-594`.
- The actual WGPU render path retains visible records, derives `[width, width / aspect]`, and sends an identity-rotation `TexturedInstance3d` translated by the authored origin and scaled by that size: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:12891-12928`, `14535-14537`.

The generic R3F reference type also has orientation and scale facilities, but they are **not reachable from the real `World3dHost` mapping above**. They cannot explain this checkpoint’s React/WGPU difference; adding them to WGPU would create a non-parity path.

The existing shared fixture already defines the production geometry law: natural `2275 × 2560`, authored width `50`, and origin `[7, 0, 0.01]` produce a centred `50 × 56.2637362637` plane with the four stated corners. See `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🖼️reference-visual/🔣️.json:31-70`. The native unit law verifies the size and an equivalent model matrix at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:5013-5080`.

That is strong source-equivalence evidence, but it is not an end-to-end placement witness: the native law builds its own matrix instead of inspecting the `TexturedInstance3d` from the actual `render_world_3d` pass. The fresh WGPU record also omits the comparable live reference input and raster descriptor. Therefore the visual delta is real, while a reference-transform production defect is **not established**.

### Required Fail-First Reference Law

Extend the existing `framework.world3d.reference-visual/v1` fixture law in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` to call `render_world_3d` with the fixture’s actual JSON and decoded raster descriptor. Inspect the emitted `ScenePass3d.textured_draws[0].instances[0].model`, transform the four canonical plane corners, and require equality with `geometry.expectedCorners`; require the texture key and decoded aspect as well. Mutating the draw-path origin, model scale, or aspect source must fail that law.

The checkpoint’s WGPU world diagnostic must additionally publish, for every visible reference, the accepted `id`, URL, origin, width, decoded `[width,height]`, and emitted model/corners. The paired `dismiss-tour` check must compare those rows to the React-host reference rows before a screenshot can be used to assign a placement owner. This observes the real delivery boundary; it does not create a second geometry authority.

**Reference confidence: high** that the present source construction matches the reachable React contract; **high** that the checkpoint does not contain enough reference telemetry to identify an owner; **low** that an origin/scale source change would fix the observed image.

## Grid: Confirmed Live Rendering-Policy Divergence

The grid has multiple current, source-level differences that directly account for the fresh darker/aliased output. Unlike the reference plane, these are actual implementation gaps.

React’s mounted world explicitly requests the Drei Grid as a transparent double-sided, depth-test-on/depth-write-off procedural plane with `cellThickness={0.6}`, `sectionThickness={0}`, `fadeStrength={1.5}`, `followCamera`, and `infiniteGrid`:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:935-985`.

The installed source that this component invokes makes the pixel policy concrete:

- its vertex shader expands the plane by `1 + fadeDistance` and translates it by the camera position projected onto the grid plane when `followCamera` is set: `node_modules/@react-three/drei/core/Grid.js:25-46`;
- its fragment shader uses `fwidth` to antialias the `cellThickness`, applies radial `distance(worldCamProjPosition, worldPosition)` fade, and then Three tonemapping/color-space chunks: `node_modules/@react-three/drei/core/Grid.js:50-80`.

WGPU instead emits finite CPU `LineVertex3d` pairs. Its grid is anchored to `camera.target` (`grid_placement_anchor(camera.target, datum)`), not the camera’s projected position, then sends a vertex-alpha tent through the raw line pipeline:

- grid invocation and target anchor: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:12799-12804`;
- finite lines and per-vertex alpha: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:7747-7783`;
- actual `LineList` pipeline: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:3071-3077`, `3167-3190`;
- its line shader merely interpolates the vertices and applies the target’s separate `world3d_attachment_output`: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs:571-631`.

The existing native grid law at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:4989-5011` only proves that finite geometry reaches alpha zero at its capped rim. It does not test fragment coverage, line thickness, radial fade, camera-position anchoring, or Three’s color transfer. It therefore cannot establish React visual parity.

The camera phase mismatch is material in the fresh Perspective record. React’s `dismiss-tour` camera position is `[16.6917, -8.9457, 9.7781]` while its target is `[5.4054, 2.3406, 1.5015]`; WGPU intentionally anchors the grid phase to the latter. This cannot be called a reference-plane cause, but it is a concrete grid-following deviation. In the Top record, position and target share XY, so this deviation cannot explain the Top grid’s jaggedness; the raw line versus derivative-fragment policy can.

### Minimal Owner-Level Repair

The owner is the shared World grid producer plus the WGPU world material/pipeline:

1. Replace the `append_lod_grid_lines` grid lane in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` with a distinct procedural grid draw record. Its inputs must be the same policy tuple React supplies: grid-plane pose, camera position projected to that plane, step, `0.6` cell thickness, zero section thickness, live theme element color, fade distance, and fade exponent `1.5`.
2. Add a WGPU grid material in the actual target shader/pipeline paths (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs` and `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`). It needs derivative/pixel-space grid coverage and radial fade, depth test enabled, depth write disabled, transparent blending, and the exact React color-output policy measured through the Three oracle. It must not reuse generic `LineList` geometry.
3. Keep ordinary overlay, engagement, and reference-outline lines on their existing line path. Only the LOD grid is a fragment-grid material in React.

### Required Fail-First Grid Law

Add a `framework.world3d.grid-visual/v1` schema/fixture to the existing scene-shading oracle route. The reference program must invoke the installed Drei `Grid` with the real component values, render both an orthographic Top camera and the fresh Perspective camera, and record pixel samples across a cell centre, a `0.6`-thickness edge, the fade rim, and the perspective phase point. The WGPU program must execute the actual production grid shader and pipeline, not a copied test shader.

The fail-first assertions are:

- moving camera position while holding orbit target fixed moves the grid phase identically in both implementations;
- edge samples have the same derivative-antialiased coverage class as the Three oracle, so changing the WGPU draw back to `LineList` fails;
- radial fade samples match Three’s `pow(distance / fadeDistance, 1.5)` policy, so independent horizontal/vertical vertex fades fail;
- direct light/dark theme samples establish the necessary output transfer before any color transform is changed.

This law should be integrated under the existing `scene-shading-pixel-check` mechanism, whose reference and actual-WGPU variants already live in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎨️world3d-scene-shading/📜️script.ts:922-1492`. It gives one Three authority and one real production GPU execution.

**Grid confidence: high** that raw finite `LineList` geometry cannot implement the installed Drei fragment policy and explains the visible aliasing; **high** that WGPU currently anchors perspective phase to the wrong source value; **medium** that the separate line color transform explains the darkness until the proposed Three/WGPU samples quantify it.
