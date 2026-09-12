# Reusable Viewport Contract And Dependency Audit

## Decision

Create a small framework-owned `semio-framework-ui-viewport` package beside `ui-scene`. It owns two direct navigation-value records: `Viewport2d` and `Viewport3dOrbit`. It depends on `protocol` and `serde`, never on OS kernel. `ui-scene` consumes those values for scene echo; OS kernel consumes them to implement its local `DslField` trait. Product window configs and native `setCamera` commands use the same records directly.

This is the smallest dependency graph which keeps the existing UI scene boundary kernel-free and avoids making the OS kernel depend on the broader UI scene package:

```text
ui-viewport ──> protocol, serde
ui-scene    ──> ui-viewport
os-kernel   ──> ui-viewport     (only to own DslField implementations)
FEM/plugin  ──> ui-viewport + os-kernel + ui-scene
renderer    <── ui-scene contract
```

The kernel-side implementations are valid under Rust coherence because `DslField` is local to OS kernel. They bind the actual foreign `Viewport2d` and `Viewport3dOrbit` types; they do not create FEM or renderer wrapper records. A package-local implementation is not possible because neither the trait nor the type would be local there.

Do **not** place these records in `ui-scene` and then add `os-kernel -> ui-scene`: that reverses the intended core-to-presentation layering and drags scene math/codecs into the kernel. Do **not** move `DslField`, `Shape`, or `FieldValue` into a new foundation in this slice: the derive macro and the existing DSL record graph are built around their OS-kernel ownership, so that would be a reflexive whole-DSL migration rather than a viewport change.

## Five Reusable Contract Facets

The initial reusable contract has exactly these five facets. Projection is deliberately outside the set.

| Facet | Contract | Owner and use |
| --- | --- | --- |
| 1. Two-dimensional pose | `Viewport2d { x: f64, y: f64, zoom: f64 }` | Navigation value for Canvas2d, infinite canvas/board, FEM2D, Note, Sequence, and Drawing. `zoom` must be finite and positive. |
| 2. Three-dimensional orbit pose | `Viewport3dOrbit { position: [f64; 3], target: [f64; 3], zoom: f64, up: Option<[f64; 3]> }` | Navigation value emitted after an orbit/pan/zoom gesture. All supplied scalars must be finite; `zoom` is finite and positive. |
| 3. Typed action envelope | `SetViewport2d { camera: Viewport2d }` / `SetViewport3d { camera: Viewport3dOrbit }` | The native producer/consumer wire shape. A product may retain the `setCamera` action id, but its payload must nest `camera`; it is not a flat camera object and never a JSON string. |
| 4. Exact-window retention | A window config field of the corresponding pose type | Each supported window instance retains its own navigation state. A `setCamera` handler reads the concrete window id, replaces that field, and emits one addressed `window_config_mutation`; it emits no artifact mutation or app-wide config mutation. |
| 5. Typed scene echo | `Canvas2dScene` and `World3dScene` expose the same pose type to the renderer | Scene construction reads exact-window config and echoes it. The renderer reads it as external scene state; a local gesture remains local until the native command produces that echo. No `String` JSON camera survives on this path. |

`WorldProjectionConfig` is not a sixth generic viewport facet. It is a product-specific projection taxonomy with orthographic, axonometric, oblique, one-, two-, three-point, and curvilinear parameters. The renderer's `WorldCameraState.projection` is only a family hint, while its `projectionSpec` carries the taxonomy. Current native handlers receive granular `setProjection` and `setProjectionParam` actions, and the renderer intentionally does not dispatch a projection in `setCamera`. A future shared projection contract needs its own producer/consumer parity and command semantics; adding a lossy family string to `Viewport3dOrbit` would recreate the current drop-on-decode failure.

The geometry `Camera3d` in `ui-scene` is also excluded: it is renderer math (`position`, `target`, mandatory `up`, `fov_y`, `near`, `far`), not a persistent orbit navigation state.

## Actual Gesture To Echo Trace

### FEM2D is the working ownership pattern

1. The renderer's `Canvas2dHost` holds `{ x, y, zoom }`; panning and wheel-zoom call `onCameraChange`, then trailing-debounce `setCamera`.
2. FEM2D's `SetCamera` accepts those three scalar fields. Its window handler requires a concrete view/window id, replaces the camera in the matching model or results config, and returns `window_config_mutations`.
3. The model/results renderers read that exact config and construct `Canvas2dScene { camera_x, camera_y, zoom, .. }`.
4. `Canvas2dHost` receives the scene echo. The camera therefore belongs to a window instance, never the FEM document or app configuration.

Primary source paths:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🪟️windows/🧱️model/🎚️config/🧬️schema/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🪟️windows/📊️results/🎚️config/🧬️schema/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🪟️windows/🧱️model/🦀️.rs`

### FEM3D has correct ownership but an invalid boundary type

`WorldOrbitGated` completes a user gesture into a structured `WorldCameraState`. `World3dHost` locally adopts it, then after 120 ms dispatches `setCamera` with the exact window id and a nested `camera` object. Its `buildWorldCameraDispatchArgs` includes `position`, `target`, `zoom`, and optional `up`; it intentionally omits projection.

FEM3D's native handler correctly selects the exact model/results window and emits one addressed window-config mutation. Its failure is only the field type: both command and retained `FemCamera` accept `json: String`, then the renderer is fed the opaque string. The `{}` sentinel subsequently selects a fallback default camera. Thus successful local orbit motion does not prove native command decoding or retained echo.

The first FEM3D implementation must replace this `String` on all three boundaries in one change: `SetCamera.camera`, the two window config `camera` fields, and `World3dScene.camera`. It must remove the `{}` fallback rather than support it as a compatibility input.

Primary source paths:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🪟️windows/🧱️model/🎚️config/🧬️schema/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🪟️windows/📊️results/🎚️config/🧬️schema/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`

### Native producer law

The existing renderer fixture is the producer-side regression input. Its completed orbit has window `procedural-preview:1` and must dispatch exactly one `setCamera` containing this object after the renderer supplies window context:

```json
{
  "windowId": "procedural-preview:1",
  "camera": {
    "position": [8, -3, 5],
    "target": [0, 0, 0],
    "zoom": 1.25
  }
}
```

This is valid because it is a complete nested orbit pose; omitting `up` is valid. These inputs are invalid for the new FEM3D decoder:

```json
{ "windowId": "procedural-preview:1", "position": [8, -3, 5], "target": [0, 0, 0], "zoom": 1.25 }
{ "windowId": "procedural-preview:1", "camera": "{\\"position\\":[8,-3,5]}" }
{ "windowId": "procedural-preview:1", "camera": { "position": [8,-3,5], "target": [0,0,0], "zoom": 1.25, "projection": "orthographic" } }
{ "windowId": "procedural-preview:1", "camera": { "position": [8,-3], "target": [0,0,0], "zoom": 1.25 } }
```

They respectively flatten the envelope, restore the opaque wrapper, inject a lossy projection family, or violate vector arity. Missing `position`, `target`, or `zoom`, zero/non-finite zoom, and non-finite vector coordinates are likewise rejected. Incoming scene parsing may choose display defaults for absent legacy data; native command decoding must not.

## Comparable Ownership Evidence

| Area | Existing representation | Correct ownership finding |
| --- | --- | --- |
| Infinite canvas and board | Both carry `Camera { x, y, zoom }`; board has `set_camera`. | Confirms the 2D value shape. These operational renderer types should consume the shared record or convert at their own boundary, not become a second public schema. |
| Note | `SetCamera { camera: NoteCamera }` replaces `NoteCompositeWindowConfig.camera` through an addressed window mutation. Tests assert no document mutations and scene output reflects config. | Reference implementation for action/config/scene ownership. |
| Sequence | `SequenceCamera { x, y, zoom }` lives in `SequenceMainWindowConfig` and maps to the node-graph scene. | Same navigation semantic; a later direct-type conversion can follow the shared owner without changing document mutations. |
| Drawing | `DrawingCamera { x, y, zoom }` is action/config state, but its current `DrawingConfig` is app-level. | It confirms the direct nested command shape but is not a template for exact-window lifetime. Move it only in a dedicated ownership slice. |
| glTF | `GltfDocument.cameras: Vec<GltfCamera>` and nodes can reference cameras; camera create/delete/move/reorder are document mutations. | Authored camera objects are semantic document state. They must never be replaced with navigation viewport records. |

Primary comparison paths:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/🎥️camera/🧪️tests/🎥️mutate-gltf-2-0-camera/🦀️.rs`

## Cargo And Derive Consequences

`ui-scene` currently depends on `ui-contract`, `protocol`, `serde`, and geometry, with no OS-kernel edge. Its Cargo manifest documents why `#[derive(ToValue, FromValue)]` cannot be used there: expansion names `::semio_framework_os_kernel`. The DslRecord proc macro likewise emits `impl ::dsl::DslField` using kernel-owned `Shape`, `FieldValue`, and `RecordValue`.

Therefore the new neutral package must use manual `protocol::value::{ToValue, FromValue}` implementations, as `ui-scene` already does. OS kernel adds a direct dependency on the new package and adds a narrow viewport binding module that hand-implements `DslField` for the two actual records. The neutral package adds no runtime dependency. Do not derive `DslRecord` in the neutral package; do not expose kernel types from it.

Relevant implementation paths:

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`

## Minimal Coherent First Implementation Set

1. Add the neutral `ui-viewport` package, its two value records, manual protocol codecs, finite/positive validation at native decode, and schema/value tests.
2. Add only the two OS-kernel `DslField` implementations plus their textual and Pack codec laws. This preserves native direct typing without moving the trait family.
3. Change `Canvas2dScene` and `World3dScene` to retain typed camera values, and update `Canvas2dHost`/`World3dHost` to read those direct fields. This removes the `World3dScene.camera_json` string from the viewport path; scene encoding is the boundary codec, not a retained opaque wrapper.
4. Convert FEM3D's command, `FemCamera` retention fields, both exact window schemas, render construction, and regression tests together. The native test must dispatch the existing renderer fixture value, assert one addressed model/results config change, no FEM document mutation, and a matching typed World3d scene echo.
5. Convert FEM2D in the same schema slice only where it consumes the new 2D record. Inventory Note, Sequence, Drawing, and the infinite implementations for follow-up; do not expand this first slice into their lifecycle migrations.

Projection configs remain in their current exact product window owners. The renderer's full taxonomy is defined by `WorldProjectionConfig` and serialized by `world3d_camera_projection_json`; it must remain separate from the new camera-pose decoder until a complete typed projection action is designed.

No source, manifest, generated asset, launch configuration, or Cargo command was changed/run for this audit.
