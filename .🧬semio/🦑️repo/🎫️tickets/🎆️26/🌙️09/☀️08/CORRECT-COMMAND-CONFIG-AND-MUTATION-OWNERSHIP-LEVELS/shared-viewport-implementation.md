# Shared Viewport Implementation

## Ownership Decision

Navigation values belong to a renderer-neutral UI contract shared by commands, exact-window configs and scene echo. The new `semio-framework-ui-viewport` package owns direct `Viewport2d` and `Viewport3dOrbit` records under `🧰️framework/🔨️modules/🖱️ui/🪟️viewport`. It depends only on the existing first-party replication package. Serde and serde_json are development-only independent test oracles. There is no new external runtime dependency.

This corrects three recommendations in `viewport-contract-dependency-audit.md`: the five facets are actual Rust, TypeScript, JSON Schema, GraphQL and Protobuf representations; the new leaf package does not add Serde at runtime; and native or scene consumers must not add legacy camera-string admission. The renderer's existing 2D flat command envelope is not being silently changed while introducing the value record. The nested 3D gesture payload remains the producer contract for the subsequent command migration.

The 2D record has finite x/y and positive finite zoom. The 3D orbit record has fixed three-coordinate position/target arrays, positive finite zoom and an optional finite three-coordinate up vector. Projection remains a separate semantic field, and authored glTF/document cameras remain document data. Each concrete app chooses its initial 3D pose explicitly; the shared record does not invent a universal scene orientation.

## Evidence

The neutral corpus has 18 valid/invalid cases including renderer gesture coordinates, missing/foreign fields, zero or negative zoom, opaque strings, wrong vector arity and projection mixed into navigation. TypeScript admission agrees with Ajv; addressed window replacement agrees with fast-json-patch while preserving another window, authored document cameras and OS locale. Additional native and TypeScript checks reject nonfinite values. Native decoding also rejects duplicate object keys.

Native1 passed the runtime oracle but strict TypeScript rejected the test assertion's undefined predicate argument. The assertion now specifies TypeError. Native2 exited 0: neutral/Ajv/JSONPatch, strict TypeScript and both native laws passed on 2 MiB. Native values and round trips agree with independent Serde records. Evidence: `🗑️generated/framework-viewport-native-2.log`.

This is foundational value-contract evidence only. Kernel DSL binding, FEM command/window adoption, typed scene echo and the renderer integration remain in progress. GraphQL and Protobuf declarations have not been compiled; no compiler success is claimed.

## Files

Owner files under `🧰️framework/🔨️modules/🖱️ui/🪟️viewport`:

- `🦀️.rs` and `🟦️.ts`: shared validation helpers and native module exports.
- `◻️2d/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto}`.
- `🧊️3d/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto}`.
- `📦️packages/🦀️rust/Cargo.toml`.
- `🧪️tests/🧫️fixtures/🪟️poses/🔣️.json`.
- `🧪️tests/🪟️poses/{🦀️.rs,🟦️.ts}`.

Workspace registration: root `Cargo.toml`, `📜️script.ts`, `📋️project.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (orders `311.206` and `311.207`), and ticket `validation/📜️script.ts` and `validation/project.json`.

## Kernel Binding Preparation

The OS DSL now implements its local `DslField` trait directly for the two shared viewport types. It exposes fixed field IDs and tuple arity, validates decoded scalars through the original viewport record, and rejects foreign record fields. The kernel reexports the actual shared types for command/config consumers. The neutral package still has no OS dependency; the kernel has no UI scene dependency.

Two additional kernel laws exercise all four valid neutral poses through inline text, document text and Pack and reject invalid zoom, wrong vector arity and foreign fields. These laws are authored and added to the native route but have not run. App/kernel validation remains coordinated with the concurrent retained-history source changes.

Additional files: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🪟️viewport/🦀️.rs`, its `🧪️tests/🪟️poses/🦀️.rs`, DSL root module include, and kernel Rust package manifest/root reexport.

Kernel native3 subsequently exited 0: both leaf laws and both kernel binding laws passed on 2 MiB. All four valid neutral poses round-trip through inline text, document text and Pack; invalid zoom, vector arity and unknown record fields are rejected. Evidence: `🗑️generated/framework-viewport-native-3.log`.

## FEM3D Command And Window Adoption

FEM3D's SetCamera payload now directly contains the shared Viewport3dOrbit. Both Model and Results window configs use that same type and reference its shared schema; the local FemCamera string wrapper and document-schema reexport are removed. The command validates even programmatically constructed poses before emitting an addressed window mutation. Editor/viewer window defaults use one explicit FEM initial pose. Their current scene codec serializes the exact shared value without the former empty-object fallback.

All five config representations and neutral fixture cameras are updated. Invalid cases include the former opaque object, short vectors and zero zoom. The native window law now decodes the existing World3dHost completed-gesture fixture and checks the rendered scene's pose against its exact window config. These native changes are authored but unrun. The updated FEM3D oracle plus strict TypeScript exited 0 in `🗑️generated/fem3d-viewport-neutral-1.log`, including the mesh, child-outcome and mounted stiffness oracle prelude.

Global World3dScene still has its existing encoded string boundary, including separate projection data used by other products. Replacing that global scene field requires all scene producers and renderer consumers to move together; this remains open. No alternate legacy camera input was added. Drawing's app-to-window migration independently adopts the shared two-dimensional record.

## FEM3D Native16 And Story Ownership Follow-Up

Native16 exited before any selected FEM test: the shared plugin invoked the newly named persisted-history constructor during the interval before its SPR implementation was written. The archive owner then confirmed source coherence. This run proves no native FEM behavior.

Story helpers now own typed shared 2D/3D navigation and serialize the 3D pose only at the existing scene wire boundary. They admit the nested 3D renderer command and flat 2D command, preserve window camera/result settings and explicit OS locale during document replacement, and use the registered demo id. Artifact setLocale emulation and opaque string admission are removed. Story red1 reproduced both invalid 3D default and preference reset failures; neutral2 passes both new Vitest laws plus existing independent FEM oracles and contract TypeScript checks. Expanded TypeScript in neutral3 revealed an unnecessary mesh runtime import; precise inferred scene records remove that import. Neutral4 is pending at this checkpoint.

Exact FEM3D adoption files follow.
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/🔬️unit/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🪟️viewport/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔣️.json)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🟦️.ts)
- [🔗️.graphql](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔗️.graphql)
- [🛰️.proto](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🛰️.proto)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧪️tests/🔬️unit/🦀️.rs)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔣️.json)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts)
- [🔗️.graphql](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔗️.graphql)
- [🛰️.proto](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🛰️.proto)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧪️tests/🔬️unit/🦀️.rs)

## FEM2D Shared Navigation Adoption

Both exact window configs now use kernel Viewport2d across five formats. The flat renderer command envelope remains exact, with shared finite/positive validation before mutation. Document re-export of app-local camera is removed. New zero/negative zoom fixture laws precede the migration; native validation is pending.

- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🧪️tests/🔬️unit/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔣️.json)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🟦️.ts)
- [🔗️.graphql](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🔗️.graphql)
- [🛰️.proto](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🛰️.proto)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔣️.json)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🟦️.ts)
- [🔗️.graphql](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🔗️.graphql)
- [🛰️.proto](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🛰️.proto)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts)

Story neutral4 passes both Vitest runtime laws, all independent FEM 3D/NumPy oracles, and strict TypeScript including the new story tests/helper. The helper returns the exact inferred scene payload, avoiding a dependency on the renderer or the monolithic mesh/manifest runtime solely for its return annotation.

Additional story and validation files:

- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📖️stories/🧭️coordination/🧫️fixtures/🧫️scene/🟦️.ts)
- [🟦️.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts)
- [vitest.config.ts](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/vitest.config.ts)
- [📜️script.ts](/Users/ueli/Documents/semio/📜️script.ts)
- [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts)
- [🧪️.story.tsx](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📖️stories/🎭️2d-model/🧪️.story.tsx)
- [🧪️.story.tsx](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📖️stories/🎭️3d-model/🧪️.story.tsx)
- [🧪️.story.tsx](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📖️stories/🎭️3d-results/🧪️.story.tsx)
- [🧪️.story.tsx](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📖️stories/🎭️3d-viewer/🧪️.story.tsx)

FEM2D neutral2 is GREEN: both exact-window schemas, new zero/negative zoom cases, independent Ajv/JSONPatch checks and strict TypeScript pass. Neutral1 stopped because the Results validator did not register the new shared schema; registration is corrected. Native FEM2D remains pending.

## Command Contract Ownership

The FEM3D retained-command fixture schema was an unused definition inside the document schema, with stale application-config lanes. It now lives under the editor command schema and admits the current 18 routes, including HostOnly document replacement and WindowConfig camera/result commands. Ajv validates the real fixture and rejects a Config-lane substitution; the native route law is included in the selected FEM window filter. Dead application-config base/step/preparation metadata is removed. The retained command byte measure now uses the actual fixed shared pose instead of a removed JSON string field, and its command-value ceiling is explicitly window-scoped. Neutral4 passes both story laws and all contract/oracle/strict TypeScript checks. Native verification remains pending.

Additional files:

- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔣️.json)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧬️schema/🚧️retained-limits/🔣️.json)
- [🔣️.json](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs)
- [🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs)

Combined engine neutral5 is GREEN: two story runtime laws, full/free candidate refusal/retry oracle, 1537-entry NumPy CSR hash/action oracle, command-owned schema, all existing FEM contracts and strict TypeScript. FEM17 has now started under root Cargo ownership. No native adoption success is claimed until that route exits.

FEM17 stopped before tests on one engine paged-close return type; the engine owner corrected the missing Option wrapper. FEM18 then compiled the new FEM2D/engine production and reached FEM3D test compilation, exposing the viewport JSON helper calling the JsonValue serializer with a DslValue. The helper now calls the canonical to_json_string serializer. FEM19 is running. These two failed compiler attempts prove no selected native tests; combined neutral checks continued to pass.

FEM19 also exited before selected tests: compilation read concurrent OS Store open-operation edits with a stale Edit type path and incomplete Phase matches. Root requested a coherent shared-source boundary and a stable interval for the next full FEM route; the archive owner can continue fixtures and the next staged slice during that interval. No FEM native assertion success is attributed to run19.

## GraphQL Source Syntax Check

The existing `framework-viewport-ownership` root and ticket routes now also parse and check the two shared GraphQL SDL sources with the installed Prettier GraphQL parser. The first run found formatting differences only; both description blocks were formatted. `viewport-graphql-2.log` exits0 with both GraphQL sources accepted, all18 neutral/nonfinite admission vectors, exact-window JSONPatch parity and strict TypeScript. This is GraphQL syntax/format evidence, not schema linking, execution, or Proto compiler evidence.

Additional changed files: both shared viewport `◻️2d/🧬️schema/🔗️.graphql` and `🧊️3d/🧬️schema/🔗️.graphql` facets; root `📜️script.ts`; ticket `validation/📜️script.ts`.
