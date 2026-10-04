# Node Graph Scene Lane Transport — 2026-10-02

The procedural Flow pane's 49,792-byte payload exceeded the framework's fixed 32 KiB scene document. `NodeGraphScene` now uses the existing scene spine and independently paged retained carriers, shared by React and wgpu.

## Implementation

The language-neutral `ui/scene/🧬️schema/🚚️node-graph-scene-lanes/🔣️.json` and matching fixture declare sixteen carriers. Six carry JSON-encoded typed vectors: nodes, edges, document operators, find items, selection and highlights. Ten carry opaque strings: preview, LOD, controls, clusters, computing, status, capabilities, host snapshot, presence and evaluation. Viewport, editability, complete interaction domain, hover and the byte/hash manifest remain in the spine.

Rust uses the existing first-party `semio-framework-pack-json` writer/parser for typed carriers. No external runtime library was added. The native value and pack codecs preserve the manifest. TypeScript extends the existing generic `SceneLane`/`sceneFromLanes` path with declared JSON encoding, retaining spine values when a typed carrier is malformed. React registers `node-graph` in its existing paged routing table, and wgpu projects node graphs through its existing retained lane merge.

The neutral fixture includes a full field sample and an oversized 4,096-node Unicode graph with an oversized valid JSON host snapshot. Native tests compare first-party typed carrier output through `serde_json` as the independent library oracle. TypeScript validates the same declaration with Ajv and tests every carrier plus oversized reconstruction. React's actual UiDocumentStore fixture and wgpu's actual UiDocumentTree projection exercise paged Unicode transport.

## Verification

- Test-driven failure observed before implementation: the new TypeScript test failed because `NODE_GRAPH_SCENE_LANES` was absent.
- `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/ui-scene-js:test --skip-nx-cache`: **80 passed, 0 failed**, 38,242 assertions.
- `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache -- long '../../../../🧱️elements/🗣️Interpreter/🟦️.tsx' -t 'node graph retained carriers|paged surface routing'` with the same Nx environment: **2 passed**, 185 unrelated tests filtered. Both paged UiDocumentStore assembly and routing passed.
- First native attempt hit filesystem exhaustion during dependency compilation. Space was recovered by shared workspace activity.
- Native scene crate then compiled; its full test run reached **105 passed** and one unrelated existing failure in `typed_scene_neutral_catalog_matches_native_serde_contracts`, whose `board-2d` fixture lacks the current native scene contract. The scoped new native law then **passed (1 passed, 159 filtered)**, including full field restoration, oversized spine admission and the serde_json oracle. A `--nocapture` runtime run confirmed `[DEBUG] node graph lane transport: nodes=4096, full=588841B, spine=446B, carriers=7`.
- Native wgpu fixture construction initially attempted cloning linear owners, which the contract correctly forbids. Fixture construction now preserves ownership. Its runtime correctly rejected a synthetic 1,024-byte text leaf against the 512-byte contract; the fixture now uses the existing 33-slice packed text leaves (`value` plus ascending `dataAttributes`) and balanced branches, preserving the 128-node document budget. The scoped projection law then **passed (1 passed, 770 filtered)**. Its `--nocapture` runtime log confirmed `[DEBUG] wgpu node graph lane projection restores 4096 nodes through 48 records`.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/🚚️node-graph-scene-lanes/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️node-graph-scene-lanes/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🚚️node-graph-scene-lanes/🟦️.test.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️scenes-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-reconcile-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx`

Existing VS Code ui-scene and renderer test launch entries already cover these suites; no executable command was added. No Git mutation or worktree was used. Generated command logs were deleted after this report captured their evidence. The parent task owns browser confirmation after a fresh plugin build and ticket closure.
