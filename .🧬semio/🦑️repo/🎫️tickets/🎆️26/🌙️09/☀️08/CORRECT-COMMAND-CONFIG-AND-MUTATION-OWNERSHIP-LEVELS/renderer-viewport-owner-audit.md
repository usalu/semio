# Renderer Viewport Owner Audit

## Scope and decision

This is a read-only audit of the relocated scene contract, native producers, and React renderer consumers. It covers `World3dScene.cameraJson`, `NodeGraphViewport`, and similarly named camera types. No source files, generated files, Cargo manifests, or git state were changed.

`World3dScene.cameraJson` is **not** intentional opaque transport for a single camera concept. It currently combines at least a shared orbit navigation pose, a lens value, and a world-projection taxonomy in an unvalidated JSON string. The shared navigation part already has the correct owner: `Viewport3dOrbit`. It needs to replace the scene's `cameraJson` field. Lens and projection must remain separate from that shared pose until their own explicit contracts are represented.

`NodeGraphViewport` is a duplicate of the shared `Viewport2d` value and should be removed. Its surrounding `viewportJson` action edge is also an untyped duplicate and must be changed with the scene type; retaining a JSON adapter would retain the ownership defect.

The React target `Camera`, Three's `Camera`, tutorial cameras, authored cameras, `IconRenderCamera`, and renderer math cameras have distinct meanings. None should be merged into a viewport contract because their names overlap.

## Existing shared contract

`🧰️framework/🔨️modules/🖱️ui/🪟️viewport` is already the real owner of reusable navigation pose:

| Contract | Shape | Evidence |
| --- | --- | --- |
| `Viewport2d` | `x`, `y`, `zoom` | `◻️2d/🧬️schema/{🔣️.json,🛰️.proto,🔗️.graphql,🟦️.ts,🦀️.rs}` |
| `Viewport3dOrbit` | `position`, `target`, `zoom`, optional `up` | `🧊️3d/🧬️schema/{🔣️.json,🛰️.proto,🔗️.graphql,🟦️.ts,🦀️.rs}` |
| Rust protocol boundary | finite coordinates, positive finite zoom, closed-record decoding | `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🦀️.rs` |
| TypeScript boundary | matching strict parsers | `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🟦️.ts` and each schema `🟦️.ts` |
| Interoperability checks | Serde, JSON, unknown/duplicate fields, non-finite values, exact replacement | `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🪟️poses/{🦀️.rs,🟦️.ts}` |
| OS DSL boundary | inline/document/pack records and rejection cases | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🪟️viewport/🧪️tests/🪟️poses/🦀️.rs` |

The dependency audit in `viewport-contract-dependency-audit.md` correctly keeps `WorldProjectionConfig` outside `Viewport3dOrbit`: a projection mode or orientation cannot be represented losslessly as an orbit pose. That boundary must remain in the repair.

## World scene camera evidence

### Current stored field and codec

`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` declares `World3dScene.cameraJson: string`. `cameraJson` is in the scene spine, not in the pageable `WORLD3D_SCENE_LANES` list, so replacing it does not require a new lane transport.

`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` mirrors this as `camera_json: String`, including the pack wire and `ToValue`/`FromValue` conversions. Current world-scene tests in `🧪️tests/🔬️scenes-value-round-trip/🦀️.rs` and `🧪️tests/🔬️scenes-unit/🦀️.rs` use values such as `"{}"`; they prove string retention, not a camera shape, cross-language contract, finite values, or required fields.

### Real producer variants

There is more than one actual shape behind the one string:

| Producer and path | Produced camera portion | Defaults or semantic data |
| --- | --- | --- |
| `world3d_camera_json` in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` | `position`, `target`, `up`, `fov` | canonical helper root, but no explicit `zoom` |
| `world3d_default_camera` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | the helper form | `[4,-4,3]`, `[0,0,0]`, 45° lens |
| `world3d_camera_projection_json` in the same plugin module | `position`, `target`, `zoom`, optional `up`, `projection` | projection is `WorldProjectionConfig` serialized as mode × orientation |
| CAD editors under `✏️s/🔌️plugins/📐️cad` | projection-helper form | editor-specific projection selection |
| Block, puzzle, low-poly, and shooting scene producers under `✏️s/🔌️plugins/{🧱️block,🧩️puzzle,🎥️shooting}` | helper-derived JSON | shooting additionally owns session/capture lens and projection state |

The renderer accepts further implicit variants in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`:

* preferred vector form: `position`, `target`, optional `up`, optional `zoom`, optional `fov`;
* coordinate alias form: `x`, `y`, `z`, optional target and zoom;
* `projection` as a full object, or the string families `perspective` / `orthographic`;
* malformed JSON or missing members, which fall back to position `[4,-4,3]`, origin target, zoom `1`, perspective, and fov `45`.

`parseCameraState` therefore performs parsing, coercion, defaulting, and union discrimination at renderer runtime. Its output also controls auto-fit and attachment. The renderer compares raw `cameraJson` text when deciding whether to reattach a viewport, so textual formatting can affect behavior even when the pose is unchanged.

`WorldOrbitGated` eventually dispatches only `{ position, target, zoom, up? }` under `setCamera`. `buildWorldCameraDispatchArgs` deliberately excludes projection because a bare family string does not decode as the native full projection configuration. This is direct evidence that the stored JSON has conflated independent contracts, rather than evidence that the string is a valid transport boundary.

### World repair

Make this one bounded scene-contract migration, with projection work explicitly separated:

1. Replace `cameraJson` with required `camera: Viewport3dOrbit` in both scene owners:
   * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`
   * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`
   * its scene pack/value codecs and `World3dScene::base` call sites.
2. Move lens and world projection into separately named, structurally typed scene fields before deleting the string. A shared viewport must not acquire `fov`, projection family, or `WorldProjectionConfig`. The existing native owner is `WorldProjectionConfig` and `world3d_projection_spec_json` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`; it needs a dedicated cross-runtime projection/lens owner and audit. The React `WorldProjectionSpec` is renderer-side only and its comment records that it has no lossless mapping to the native granular action model, so it cannot simply be copied into the scene schema.
3. Update `World3dHost/🟦️.tsx` to consume `scene.camera` directly and to parse neither a pose nor a union. Its display-only fallback may exist only for a no-scene shell or test harness; a received `World3dScene` must always contain a valid orbit pose. Keep the existing renderer-local tutorial, projection-pane, and auto-fit behavior separate.
4. Replace the helper roots above and every `World3dScene` producer atomically. Producers must carry their own initial pose, lens, and projection values explicitly, so a CAD, shooting, or other domain default does not silently become the shared viewport default.
5. Replace the current string round-trip coverage with tests for a valid typed scene camera, invalid/non-finite/zero zoom rejection, unknown-field rejection, exact native/TypeScript serialization, renderer attach/echo behavior, and preservation of each domain's lens and projection values.

The immediate files for this repair are the scene type and Rust codec above, `World3dHost/🟦️.tsx`, the WGPU helper, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, and the two existing scene test suites. The producer search must be completed at implementation time against `World3dScene` construction and the helper roots; leaving one JSON-producing adapter would reintroduce the same untyped perimeter.

## Node graph viewport evidence

`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` defines:

```ts
type NodeGraphViewport = { readonly x: number; readonly y: number; readonly zoom: number };
```

This is structurally and semantically the shared `Viewport2d` camera: a pan origin and zoom for a node graph. The Rust mirror in `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` is a second codec. It accepts omitted coordinates through serde defaults and supplies zoom `1`, unlike the shared strict contract. Its scene tests only round-trip a record.

The React renderer's `🧱️elements/🕸️NodeGraph/🟦️.tsx` imports the duplicate public type and starts its local session with `JSON.stringify(scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT)`. It intentionally treats the scene value as an initial attach value after which live renderer state takes ownership. That behavior is sound; the JSON session encoding and action shape are not.

The same file builds `nodeGraphViewportActionArgs(cameraJson)` as `{ viewportJson }` and dispatches serialized camera values. Native and test consumers are present in:

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`;
* its `🧪️tests/🕸️wgpu-node-graph/🦀️.rs` test;
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` and the NodeGraph story;
* real plugin action handlers in `✏️s/🔌️plugins/🌀️procedural/.../🌀️generation2d/.../✏️editor/🦀️.rs`, `✏️s/🔌️plugins/🏛️architect/.../✏️editor/🦀️.rs`, and `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs`.

The broader repository search finds NodeGraph viewport use across graph, flow, equation, procedural, architect, space, and trinity domain implementations. Those consumers are migration participants, but do not indicate distinct camera semantics; they should all receive the one shared `Viewport2d` type.

### Node graph repair

1. Export the shared TypeScript `Viewport2d` parser/type through the UI package boundary used by scene consumers, then replace `NodeGraphViewport` with `Viewport2d` in `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`.
2. Change `NodeGraphScene.viewport` to `Option<Viewport2d>` in `🎬️scenes/🦀️.rs`; delete the duplicate Rust struct, serde defaults, value codec, and re-exports. Do not preserve it as a type alias.
3. Make NodeGraph's renderer and WGPU target accept and emit a nested typed `{ viewport: Viewport2d }` action. Remove `cameraJson`, `viewportJson`, and `JSON.stringify` from `NodeGraph/🟦️.tsx`, `EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, the listed native handlers, and their tests/stories.
4. Retain the renderer's initial-attach/live-session distinction. It should hydrate the typed scene viewport once and preserve live gesture state while an echoed scene value lags.
5. Add graph-specific wiring tests on top of the existing shared pose suite: initial typed scene viewport, pan/wheel emits one typed nested record, valid echo does not reset the live session, and invalid/missing/extra/non-finite fields are rejected at the native and TypeScript action boundaries.

## Camera names that must remain separate

| Name | Owner and meaning | Repair decision |
| --- | --- | --- |
| React target `Camera` | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` defines `position`, `forward`, `up`: an orientation/basis record with no `target`, `zoom`, or lens. | Leave separate; it cannot substitute for an orbit viewport. A rename is a separate API clarity audit. |
| `ThreeCamera` | The same target re-exports Three.js's external runtime `Camera` under the explicit alias. | Leave it external and aliased. |
| `IconRenderCamera` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts` owns a render-request/capture input with pose, optional fov/up, asset, dimensions, and output format. | Keep it as a rendering-port request; it is not persisted navigation state. |
| Tutorial camera/keyframe | `World3dHost/🟦️.tsx` converts runtime state to an orbit playback record that includes fov and timeline semantics. | Keep it authored/timeline-local. Do not use `Viewport3dOrbit` as a tutorial document model. |
| `Camera3d` | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs` owns renderer math: position, target, up, fov, near, far, matrices, and rays. | Keep it math-local. |
| Shooting and other authored cameras | Product config records carry capture/session lens and projection values. | Preserve their domain contracts and initial values while adapting only their navigation pose at the scene boundary. |

## Default preservation

The shared types validate a supplied pose; they do not establish a global domain camera default. Preserve each domain's authored initial values during migration:

* FEM set-camera tests already use `Viewport3dOrbit` in `✏️s/🔌️plugins/🏗️fem/.../🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs`.
* Remodel uses `store::Viewport3dOrbit` in `✏️s/🔌️plugins/📸️remodel/.../🎮️commands/📷️set-camera/🦀️.rs` and its window configuration.
* Drawing uses `store::Viewport2d`; its presence default remains the authored `{ x: 512, y: 512, zoom: 0.75 }` in `✏️s/🔌️plugins/🖍️draw/.../👥️presence/🦀️.rs`.

No FEM, Remodel, or Drawing default should be replaced by a renderer fallback or a `Viewport*` default as part of either repair.

## Validation to require from the repair

Run the existing language-neutral schema tests plus native scene/action and renderer interaction tests after implementation. The acceptance condition is that scene and action boundaries carry typed `Viewport2d`/`Viewport3dOrbit` records; no `cameraJson` or `viewportJson` remains on those paths; malformed input has no renderer fallback that becomes persisted state; and lens, projection, tutorial, authored, icon-render, and math-camera behavior retain their separate owners.

This audit did not run Cargo, test, or build commands.
