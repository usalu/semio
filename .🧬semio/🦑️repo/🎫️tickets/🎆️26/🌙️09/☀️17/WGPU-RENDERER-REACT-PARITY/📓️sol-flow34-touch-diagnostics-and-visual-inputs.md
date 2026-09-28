# Touch, Accepted-Frame Diagnostics, and Concrete Forest Inputs

## Scope

This packet establishes the neutral two-contact lifecycle shared by Board2d and TiledMap, proves the existing React implementations against it, and adds a diagnostics-gated accepted-frame receipt for physical React/WGPU projection comparison. Rust pointer transport and native surface gesture ownership remain a subsequent checkpoint because the WGPU artifact build was intentionally kept on a coherent pre-touch source boundary.

## Neutral touch authority

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🤏️surface-pinch/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🤏️surface-pinch/🔣️.json`

The strict schema records an 800×600 viewport and two camera laws:

- separation 200→400 maps `{x:0,y:0,zoom:1}` to `{x:0,y:0,zoom:2}`;
- a rigid +40,+20 pair translation maps it to `{x:-40,y:-20,zoom:1}`.

It also records the surface ownership distinction:

- Board2d transfers the first contact by cancelling area selection and emitting one synthetic pointer-up at second-contact admission;
- TiledMap clears marquee/pan ownership without a synthetic map pointer-up;
- neither surface sends pointer-move actions during the pinch;
- either lift is excluded from the former single-pointer lane;
- final lift publishes exactly one settled camera;
- no selection is published;
- the next contact starts a fresh single-pointer lane.

## Actual React oracles

`🧱️elements/🖥️Board2dHost/🧪️tests/🤏️pinch-gesture/🟦️.tsx` now validates the shared schema and replays the fixture through the mounted real host. It verifies Board's transfer-time synthetic pointer-up, silent camera movement, one final settled camera, absence of release/selection actions during both lifts, and a fresh successor contact.

`🧱️elements/🧭️TiledMapHost/🧪️tests/🤏️pinch-gesture/🟦️.tsx` mounts the real host with only the Map WASM session port replaced by a deterministic session double. It verifies both shared camera cases, zero synthetic map pointer-up on transfer, no per-move action, one final camera publication, no selection on lift, and a fresh successor contact.

Focused proof:

```text
Test Files  3 passed (3)
Tests       15 passed (15)
Duration    21.25s
```

The three files were Board pinch, TiledMap pinch, and the World3dHost component diagnostic law. The command used the explicit `@semio-tech/framework-renderer-react` Nx project with excluded task dependencies and one Vitest worker. Receipt: `🗑️generated/sol-flow34-touch/react-hosts-2.log`. JSDOM's existing canvas `getContext` notices were non-fatal; every assertion passed.

## React accepted-frame receipt

`🧱️elements/🌐️World3dHost/🟦️.tsx` now mounts `WorldAcceptedFrameDiagnostics` inside the actual `WorldLodBridge`. When the shared renderer runtime diagnostics authority is enabled, one changed-value receipt is emitted:

```text
[DEBUG] react-world-frame { ... }
```

The receipt reads the live `useThree` camera and controls after the projection/framing and LOD frame runners. It carries:

- surface and window instance identity;
- complete projection spec;
- camera kind, position, target, up, quaternion, FOV, zoom, near, and far;
- viewport dimensions/aspect;
- content bounds;
- visible reference ids, origins, and resolved widths;
- automatic/depth-variable LOD inputs, orbit and matched LOD distance, selected LOD, grid factor, and grid step.

The mounted component law proves disabled diagnostics emit nothing, enabled diagnostics emit the exact accepted record, unchanged frames deduplicate, and a changed camera produces one new record.

## Concrete Forest expected inputs

The selected Puzzle3d example authors:

- one instance at `[0,0,0]`;
- `ref-masterarbeit` at `[7,0,0.01]`, width `50`;
- the reference image is 2275×2560, so its accepted plane is `50×56.2637362637`.

The GLB position-accessor union is raw Y-up `[0,0,-4.676537]..[10.800011,3,0]`. React and WGPU both apply the same +90° X world frame, giving:

```text
mesh bounds  [0,0,0] .. [10.800011,4.676537,3]
mesh center  [5.4000055,2.3382685,1.5]
mesh radius  6.0726896000
```

The projection-content framer deliberately uses instance positions plus visible reference footprints rather than mesh vertices:

```text
center      [7,0,0.005]
halfExtent  [25,25,0.5]
```

For Top, padding 1.35 produces `zoom = min(viewportWidth, viewportHeight) / 67.5`. The later producer fit lane is enabled with padding 1.25 and no explicit bounds, so the resident mesh AABB should move the accepted target to `[5.4000055,2.3382685,1.5]` while preserving the Top zoom. Its accepted eye distance is approximately `6.0726896 / sin(25°) × 1.25 = 17.96`.

Orthographic automatic LOD is driven by `viewportHeight / (2 × zoom × tan(25°))`, not the eye distance. At this framing the selected grid step must be `10` world units for the authored grid factor 10. A 6.18 px/world Top zoom therefore yields a 61.8 px grid pitch. The new paired receipts distinguish a zoom/LOD discrepancy from a grid raster discrepancy.

The reference transform and actual WGPU textured vertex buffer are both centered XY quads, matching Three's `PlaneGeometry`. No source-supported translation or scale correction exists.

## Curvilinear physical interpretation

The WGPU and React fragment formulas agree, including the `tan(radius × halfFov)` mapping, source-UV bounds guard, FOV, strength, and aspect. The WGPU sampler clamps; it does not repeat. At FOV 120° on a wide viewport, corner radii can cross the tangent singularity and fold back into valid source UVs. If the paired React receipt reports an orthographic camera while the title says Curvilinear, React's postpass is inactive. If both receipts report a perspective camera with the same FOV and strength, the out-of-domain radius behavior is a shared shader contract issue and needs one schema-first radius-domain rule in both implementations.

## Native touch implementation boundary

The browser input wire's complete `PointerInfo` now stays intact through Winit dispatch and `AppInteractionState`. A generic Rust recognizer peer consumes the same neutral vectors as TypeScript. Board2d and TiledMap share contact/pinch/camera math while retaining their distinct transfer semantics: Board cancels area selection and synthesizes its one transfer-time pointer-up; TiledMap clears marquee/pan without a map pointer-up. Both publish silent move cameras, one final settled camera, suppress both pinch lifts from the prior single-pointer lane, and admit the next contact fresh. Mouse primary/middle and the existing exact PointerCancel ownership route remain unchanged. Native compilation and focused retained-surface execution remain pending the parent's current Cargo gate boundary.

## Late reference mount and camera remount ownership

The physical Top discrepancy was a React lifecycle defect. `WorldReferencePlaneItem` returned `null` until its media resolved, while its pose effect had already run against a null group ref and did not depend on media. The eventual plane therefore rendered at world origin despite carrying authored origin `[7,0,0.01]`. The plane now applies pose through its actual callback ref. A Three `Object3D` mounted after the initial null phase proves both its local position and `matrixWorld` translation are `[7,0,0.01]`.

Projection-family and focus remounts exposed a second identity defect. The camera seed watched only a boolean controls-ready value. React could replace controls A with controls B in one batched commit while that boolean stayed true, so B retained its default target `[0,0,0]`. The seed now owns the actual camera and controls identities. Its mounted ReactDOM law uses real Three perspective cameras and proves the same accepted seed reapplies position and nonzero target for camera A→B and controls A→B, while an unchanged tuple stays idle.

Focused receipt:

```text
Test Files  1 passed (1)
Tests       103 passed (103)
Duration    5.86s
```

Receipt: `🗑️generated/sol-flow34-touch/r3f-reference-camera-2.log`.

## Authored camera framing policy

The renderer-neutral viewport schema now declares `projectionFrame: "content" | "preserveCamera"`. Omission resolves to `content`. A delivered `content` spec owns the initial/external one-shot frame. `preserveCamera` accepts the complete projection spec for family/FOV matrix construction while keeping the delivered position, target, up, and zoom; it clears pending content-frame debt and does not rearm on later delivered camera changes. A local projection selection switches policy ownership back to `content` while retaining the already accepted view, matching React's mounted viewport owner rather than replaying the external content framer.

The shared fixture carries both admitted values and hostile spellings. Ajv/TypeScript, Rust serde/Pack, React camera parsing, and the native World scene bridge consume the same taxonomy. The native law sends a real `camera_json` through the retained bridge, verifies the authored orbit survives content bounds, then selects a local projection and verifies the accepted pose remains owned locally.

## Page-visible WGPU accepted-frame receipt

The frame Worker now projects the diagnostics-gated `dumpMeshStats` result after each accepted tick to the fields needed for paired visual comparison: surface/pane identity, viewport rect, published bounds, wire camera, and live camera. `liveCamera` already carries complete projection, framing policy/debt, viewport, content bounds, references, and grid LOD inputs. The Worker deduplicates the serialized accepted receipt and posts only changes. `BrowserFrameTransport` forwards the structured `world3d-accepted-frame` message to the page, where browser boot emits:

```text
[DEBUG] wgpu world3d-accepted-frame generation=… frame=… {…}
```

The channel is absent when diagnostics are not stamped and cannot fault rendering if introspection is unavailable or malformed.

## Neutral World presentation and viewport compositing

`World3dScene` now carries a separately paged `presentationJson` lane. Its typed cross-language value owns `showGrid`, `showGizmo`, `interactive`, `viewportMask`, `clear`, and optional `sourceAspect`; omitted fields resolve to the regular interactive World presentation. The Icon producer publishes a noninteractive presentation with grid and gizmo disabled. `sourceAspect` lets projection use the authored request extent while the finished image is stretched into React's independently measured inner frame.

The language-neutral lane fixture covers defaults, the complete Icon value, invalid aspect/type/enum/unknown-field records, the 22-lane manifest, split/reassembly, byte count, and digest. Focused proof:

```text
2 pass
0 fail
21 expect() calls
```

The Rust scene carrier also completed a scoped `@semio-tech/ui-scene-rs:check`. The check emitted only the pre-existing unused-parameter warning in projection framing; that parameter was subsequently marked intentionally unused.

Ellipse presentation is owned by `ScenePass3d`, after every 3D scalar. The prepared cursor now snapshots the composite before the pass, optionally paints an authored scene clear, emits shadow/opaque/material/texture/grid/line/translucent content, snapshots the completed scene, and runs one terminal postprocess. Inside the normalized ellipse it samples the completed scene. Outside it restores the pre-scene backdrop. Curvilinear source coordinates outside the source viewport retain their established opaque-black behavior. Rectangle/no-remap passes emit no extra cursor. Explicit background remains scene-owned and is therefore masked; transparent Icon presentation emits no early fallback rectangle.

Native WGPU compilation was queued behind another repository Cargo owner and intentionally cancelled before acquiring the Cargo lock. The prepared-cursor, shader-validation, World-presentation, and local-projection Rust assertions therefore remain pending the next coordinated native gate.

## Local projection transition ownership

Physical React confirms Top → Curvilinear retains the accepted target, eye distance, zoom, and actual camera controls while replacing family and FOV. Native local projection selection now mirrors that rule: same-orientation changes preserve yaw, pitch, up, target, distance, and zoom; orientation changes alone apply the new look; parallel-family entry computes the matched orthographic zoom from the prior perspective view; and local changes never arm content-frame debt. Delivered/initial specs retain their separate frame-once path.

## Asset transport failure isolation

The browser fetch lane now classifies ordinary HTTP status, network `TypeError`, short-read, and reference decode failures as one unavailable asset; it returns the exact checked-out owner through a bounded reject path, records that URL in the existing World/shared miss authority, wakes the next asset poll, and leaves the shell alive. AbortError follows the existing cancellation return and records no miss. Capacity, protocol, and renderer-credit errors remain `asset-stream-fault`.

Native transport uses the same distinction. File/HTTP/network/body availability failures retain the handoff until the exact owner authority can record its miss, then close and return the owner. Interaction-lock contention is back-pressure. Cancellation remains miss-free; fixed-capacity and malformed-page failures still record a frame fault. The shared v9 fixture covers both transports. The focused browser worker/transport receipt is GREEN **60/60**; the native exact-owner law is authored but remains unrun while the coordinated WASM/native queues own Cargo.

## Bounded GLB outline implementation

React derives one-degree `EdgesGeometry` per primitive, scales the outline locally by `1.001`, then applies the primitive/node/root transforms. The existing GLB decoder already owns the correct bounded/cancellable publication boundary, so outline derivation belongs inside `GlbMaterializeCursor` between sealed BIN validation and `Mesh3d` allocation:

1. Extend the neutral outline fixture with stroke visibility/color resolution. Keep the existing actual Three oracle for welded boundary/crease segments.
2. Add a retained edge cursor that consumes exactly one planned triangle per step, reproduces Three's four-decimal position welding and directed sibling/tombstone rule, and caps its table/output against the existing 16 MiB semantic-output budget.
3. Materialize the accepted outline segments into the same mesh claim before its single seal. Transform each endpoint as `nodeMatrix × (primitiveLocal × 1.001)`, so translations are not inflated. Closing aborts the partial mesh and drops only fixed-capacity numeric storage; no paint-time triangle traversal exists.
4. Keep derived outline edges presentation-only by leaving `edgeIds` empty. Component registries/pickers count only edges with exact semantic ids, while the render overlay may consume every outline segment.
5. Carry `material.stroke` through the existing World environment record. Regular World falls back to its neutral line color; Icon explicitly resolves absent stroke to black and `none`/`transparent` to disabled. Mesh and outline publish atomically, so a cancelled derivation cannot expose partial geometry.

The packet is now implemented at that ownership boundary. `GlbMaterializeCursor` derives at most one triangle per step before it begins the `Mesh3d` claim, flushes at most one unmatched edge per step, writes at most one outline segment per step, and seals the mesh and outline together. The conservative mesh-plus-outline byte count is checked against the existing 16 MiB semantic-output budget before the fixed-capacity lookup, record, and segment stores are reserved. Closing still aborts the sole partial mesh claim, while pre-allocation cancellation drops unpublished numeric state.

Outline endpoints are multiplied by `1.001` in primitive-local space and then transformed through the planned node/world matrix. Their exact published bounds enter the mesh AABB. The generated rows deliberately have `edgeIds = 0`: World registries, marquee consistency, selection overlays, and direct edge picking admit edges only when `edgeIds == edges`; the visual overlay can consume every edge. The Icon environment carries the material stroke, including absent → black and trimmed `none`/`transparent` → disabled. The regular World fallback remains its theme border token.

The language-neutral fixture is version 2 and carries both geometry and stroke cases. The production React resolver and actual Three `EdgesGeometry` oracle are GREEN:

```text
Test Files  1 passed (1)
Tests       4 passed (4)
Duration    8.51s
```

The Rust accumulator law consumes the same fixture and asserts the scaled canonical segment set, but is intentionally unrun while the parent's WASM25 build owns Cargo. Five pre-existing World overlay unit callers were updated to pass the canonical theme after the overlay gained the material-stroke authority.

## Board touch compile correction

The earlier native Board failure was real rather than a stale dependency snapshot. `BoardHost` had `set_camera` and `set_camera_silent`, while the apparent `camera` method belonged to `BoardWheelPlan`. The parent added the missing documented read-only `BoardHost::camera` beside those owning methods before WASM25. Any earlier note treating that diagnostic as stale is superseded by this correction.

## Icon SVG lighting profile

Actual Three `SVGRenderer` does not use WebGL's PBR path for `MeshStandardMaterial`. It computes one flat face color from ambient RGB plus directional RGB × intensity × the face-normal dot product, multiplies that by the material base color, adds emissive RGB, then applies the sRGB transfer function. Ambient intensity, metalness, roughness, and emissive intensity do not participate. Back faces are omitted. Line strokes are emitted as direct sRGB colors.

The neutral `framework.icon-render.svg-lighting/v1` fixture records front, back-light, and oblique-light cases plus a pair that differs only in those ignored PBR/intensity fields. The installed Three `SVGRenderer` produces the fixture's four exact CSS colors. `ScenePass3d` now carries an explicit `RasterPbr | SvgFlatLit` render profile. Icon SVG selects `SvgFlatLit`; regular World and PNG remain `RasterPbr`. The profile tag is carried in `World3dGlobals`, and the canonical mesh/line WGSL selects the SVG flat-light/direct-sRGB path only for that tag. The ordinary PBR BRDF, shadow, material, and ACES path is unchanged.

The focused browser proof executes the actual production `WORLD3D_SHADER` in Chromium WebGPU with the production 240-byte globals, 48-byte vertex, 96-byte instance, and shadow bind-group layouts. All four GPU readbacks match the installed Three `SVGRenderer` RGB values within one byte, including the derivative face-normal/sign calculation. The ignored-parameter pair produces byte-identical SVG pixels while the same two records produce different pixels under the unchanged `RasterPbr` tag.

```text
Test Files  1 passed (1)
Tests       3 passed (3)
Duration    5.82s
```

The focused command used explicit project `@semio-tech/framework-renderer-react`, excluded task dependencies, one Vitest worker, and the repository Playwright browser cache. Native profile/state assertions remain owned by the parent's current native/WASM gates.
