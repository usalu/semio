# World Projection Neutral Values Implementation

## Outcome

The bounded neutral-value slice is implemented at the shared `ui/viewport/3d` owner without activating `World3dScene` or migrating any producer, helper, React host, or WGPU consumer.

`Viewport3dProjectionPreferences` retains the complete fifteen-field editable preset bank. `Viewport3dProjectionSpec` represents one active mathematical mode and an independent cardinal, corner, or free orientation. The types contain no renderer object, renderer flag, window identity, authored camera value, or compatibility projection kind.

The preference bank deliberately has no hemisphere field. Active specs accept omitted, upper, and lower corner hemispheres; preference derivation emits upper for the current axonometric preference behavior.

## Schema And Range Decisions

The source UI control limits were inspected before validation was implemented. They are retained on the editable preference bank:

| Field | Inclusive range |
|---|---:|
| `axonometricAngleA`, `axonometricAngleB` | 5–75 |
| `obliqueAngle` | 5–90 |
| `obliqueDepth` | 0.05–1 |
| shared perspective `fov` | 15–120 |
| `twoPointShift` | -1–1 |
| `curvilinearFov` | 60–160 |
| `curvilinearStrength` | 0–1 |

The stored `axonometricAngleA` range remains 5–75 even when the selected variant is dimetric. This preserves an inactive angle previously edited under trimetric; the dimetric control may expose a narrower active editing range without making the retained bank undecodable.

The active transport has a different admission contract: every numeric mode field must be finite, but it does not inherit the preference controls' slider limits. This split is required by the inspected sources:

- Lowpoly `worldCameraFov`, Process3d `cameraFov`, and Generation3d preview camera `fov` are plain JSON numbers and their generated TypeScript guards apply `Number.isFinite` without a range. Their Rust facets use `f64`.
- `worldProjectionGoalMatrix` passes the supplied perspective FOV directly to Three's `PerspectiveCamera`; oblique angle/depth and two-point vertical shift are direct finite matrix inputs.
- the current curvilinear renderer intentionally accepts an active FOV 200 in its test and caps the effective capture FOV to 160 in `worldProjectionPerspectiveFov` and `worldCurvilinearUnproject`. Therefore 160 is a renderer capture/control decision, not the active transport's upper bound.
- native `Camera3d` computes its perspective matrix directly from `fov_y` and has no 15–120 admission range.

The shared active corpus consequently includes finite values outside every relevant preference control: axonometric 80/4, oblique 135/depth 1.5, perspective FOV 130, vertical shift -1.25, and curvilinear FOV 200/strength 1.25. Strict TypeScript and Rust codecs, Ajv, and Pack admit those values unchanged while rejecting every non-finite active scalar.

This slice validates a lossless renderer-neutral transport. It does not claim that every possible finite combination yields a usable projection matrix. In particular, axonometric corner pose trigonometry has cross-field constraints that the current renderer does not validate. The later renderer/math boundary must reject unusable finite values explicitly; it must not silently substitute a default.

The JSON schema remains rooted at `Viewport3dOrbit` for existing consumers and adds closed `$defs` for the preferences, active modes, orientations, and spec. GraphQL and Proto facets describe the same enums, fifteen preference fields, seven modes, and three orientation shapes.

## Behavioral Laws

The language-neutral corpus contains seven representative active modes and twenty orientations: seven cardinal faces, four corners with omitted hemisphere, four upper corners, four lower corners, and free. Their cross product exercises 140 valid active specs in TypeScript, Rust Serde/FromValue, and the scene Pack codec.

Ten derivation vectors cover the seven active kinds plus isometric, configured dimetric, and configured trimetric details. The laws are:

- isometric derives 30/30;
- dimetric derives configured A/A;
- trimetric derives configured A/B;
- military oblique derives plan while cabinet/cavalier derive front;
- one-point X/Y/Z derives left/front/top;
- two-point, three-point, and curvilinear derive free;
- default preferences preserve all inactive values and derive three-point/free/FOV 50.

The rejection corpus covers non-object inputs, unknown and missing fields, wrong tags and enums, mode-only field leakage, bare orientation strings, explicit-null hemisphere, every stored preference range, and dynamic NaN/positive-infinity/negative-infinity values for every preference and active numeric field. Rust adds duplicate-field tests for Serde and `DslValue`.

Independent Ajv validation agrees with the TypeScript parsers. `fast-json-patch` constructs derivation and retention states independently and proves curvilinear settings survive orthographic selection and later recall. Three.js produces finite perspective and oblique matrices from the representative outside-control values, including the current FOV-160 capture of a transported curvilinear FOV 200. Rust compares strict Serde and native `FromValue` results. The existing scene Pack implementation round-trips the full bank and all 140 spec combinations, its strict decode rejects a nonfinite active lens, and the native scene math produces a finite projection matrix after Pack preserves FOV 130.

## Exact File Ledger

Created:

- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🧫️fixtures/📐️projection/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/📐️projection/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/📐️projection/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/world-projection-neutral-values-implementation.md`

Updated:

- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔗️.graphql`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🛰️.proto`
- `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️pack-unit/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`
- `📜️script.ts`
- `📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

The shared root files already contained concurrent work. The ledger identifies the narrow routes and launch entries added by this slice, not exclusive authorship of those files.

## Verification

Executed successfully through the ticket validation facade:

```text
bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts framework-viewport-projection
[DEBUG] Shared viewport projection values matched Ajv, fast-json-patch, Three.js finite matrices, 140 mode-orientation pairs including outside-control active values, 10 derivations, and strict rejection laws
All matched files use Prettier code style!
TypeScript strict compilation exited 0.
```

Additional successful read-only checks:

```text
fixture JSON parse: passed
schema JSON parse: passed
root and ticket project JSON parse: passed
scene Pack fixture include path: resolved
git diff --check on the bounded ledger: passed
```

The canonical `bun nx run workspace:framework-viewport-projection` route reached Nx project-graph construction but made no progress beyond the two repository plugins for sixty seconds, matching the known unrelated graph hang. It was interrupted, and the ticket facade ran the exact registered implementation directly.

Native verification remains pending the coordinated Cargo lane. The queued native facade runs only:

```text
cargo test -p semio-framework-ui-viewport --lib viewport_projection_ -- --nocapture
cargo test -p semio-framework-ui-scene --lib viewport_projection_pack_ -- --nocapture
```

This report must be updated with the native result before the bounded slice is handed back.

## Intentional Limits

This slice does not change `World3dScene.cameraJson`, projection helpers, renderer parsing, React camera selection, WGPU camera transport, window configurations, or app actions. It does not attempt to settle the later framing behavior for arbitrary helper FOV values. The verified current mismatch between helper/parser FOV 45 or configured values and the React default rig FOV 50 remains a source-atomic scene activation concern.
