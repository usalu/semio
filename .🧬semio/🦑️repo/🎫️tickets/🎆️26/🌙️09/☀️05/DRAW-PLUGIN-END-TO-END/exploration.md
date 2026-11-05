# Draw Architecture and IO Parity Exploration

Read-only exploration, 2026-11-04. No source changes.

Applicable instructions: root AGENTS.md and ✏️s/AGENTS.md; no nested draw AGENTS found. All commands used repo. Direct repo tools exec rg is unavailable (unregistered); use repo tools exec --light node with filesystem APIs or child_process rg.

Owner: ✏️s/🔌️plugins/🖍️draw. Artifact root: 🗿️artifacts/🖍️drawing/🦀️.rs. Its artifact() declares the standard tree and io() declares native conversion registrations. Main subset: 🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any. Editor manifest: ✳️any/✏️editor/🛂️manifest/🦀️.rs. Native owned state/arena/digests: 🗿️artifacts/🖍️drawing/🔨️modules/🏠️host. Mutation inventory bridge: 🏭️bridge/🦀️.rs.

Canvas ✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs already renders prepared scene geometry, images, request-owned selection, gesture previews, node anchors/control points, transform handles, artboard dimensions and camera framing. No actionable TODO/unimplemented body was found in draw Rust/TS; execution gaps require validation.

## Definite IO Parity Gap

✳️any/🚪️io/🟦️.ts exports a locally redeclared primitive-string IoEntryDescriptorMirror and twelve routes, including PDF/PNG/DWG/DXF imports absent from native production registration. Comments call SVG import and foreign exports stubs despite existing implementations. SVG import metadata wrongly says Lossy whereas production is Semantic.

Native artifact root io() registers eight routes, in order: Drawing→SVG Lossy; Drawing→PDF Lossy; Drawing→PNG Lossy; Drawing→JSON Exact; JSON→Drawing Exact; SVG→Drawing Semantic; Drawing→DWG Lossy; Drawing→DXF Lossy. Fidelity constants were checked in ✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/{🎨️svg/🔖️1.1,📖️pdf/🔖️1.4,📷️png/🔖️1.2,🔣️json/🔖️rfc8259,🖊️dwg/🔖️ac1018,📐️dxf/🔖️r12}/✳️any/🦀️.rs and import equivalents at 📥️import/🧩️deserializers/🗿️artifacts/{🔣️json/🔖️rfc8259,🎨️svg/🔖️1.1}/✳️any/🦀️.rs. Inspect shared deserializer trait/entry constructor for sniffs semantics; not verified in this exploration.

Shared schema: 🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts. Public package @semio-tech/framework/io/schema exposes IoEntryDescriptor, parseDialectCoordinate, parseIoEntryDescriptor, ioSchema; export leaf 🧬️schema/📦️packages/🟦️typescript/🟦️.ts. Replace the local mirror with this first-party contract.

Reuse test pattern from ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io: 🟦️.ts uses shared typed dialects; 🧪️tests/🔬️descriptor-parity/🟦️.ts validates neutral fixture with both first-party parser and independent Ajv; 🧪️tests/🧫️fixtures/📇️descriptor-parity.json defines expected/invalid/mismatch cases. Recommend failing neutral fixture/test first, eight-route parity, shared type, actual fidelity/sniffs, test invocation from draw-js script. Prefer native runtime descriptor assertion if lightweight existing route is available.

## Validation Owners

✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📋️project.json exposes @semio-tech/draw-js:test, publication-authority-audit, utility-action-policy. 📜️script.ts has focused selectors and broad scene/schema/raster/tool tests. Existing Ajv 8.20.0 test dependency satisfies independent oracle without runtime dependency.

🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/📋️project.json exposes @semio-tech/draw-drawing-rs:check, test, graph-wire-check, test-inspector-selection-actions, verify-drawing-canvas-window-ownership, test-snapshot-sqlite plus source/native variants. Check/test depend on graph-generate.
