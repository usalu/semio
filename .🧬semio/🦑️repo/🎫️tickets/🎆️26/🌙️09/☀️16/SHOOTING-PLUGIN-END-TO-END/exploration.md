# Shooting Exploration

Read-only source audit of `✏️s/🔌️plugins/🎥️shooting`. No build or runtime execution was performed by this audit agent.

## Scope and Instructions

The parent `✏️s/AGENTS.md` describes the collaborative design OS. The plugin AGENTS.md defines Shooting as a parametric conversion of 3D assets to SVG and PNG icons. No nested AGENTS.md was found below the plugin root.

## Implementation Map

The plugin contains the Shooting artifact at `🗿️artifacts/🎥️shooting`; its standard 1, any subset holds the document schemas, mutations, codecs, examples, viewer, and editor. Rust is the executable editor implementation. TypeScript editor/window files are typed view-model twins and re-export bindings. The artifact root mounts split Rust crates and re-exports the public surface.

The editor dispatch table exposes 35 actions for document/example loading, assets, shots, cameras, scene lighting, transforms, pointer interactions, import and exports. `🔀️args-bridge` translates shell action arguments; `🕹️play-app` implements the editor; `🧵️retained-commands` classifies all 35 actions as migrated bounded first-step jobs and assigns publication lanes. Both scene and icon windows expose config-derived measures and engagement controls. The icon window serializes an icon-render request into the IconRender surface. Export active/all emits IconRenderExport effects with one request per selected shot and faults when no asset/shot can be exported.

## Existing Tests

The TypeScript package test script runs only three example tests: demo document, hexagonal concrete-forest document, and demo editor session. Rust has editor unit tests, window-action contract tests, per-command unit tests, schema/mutation tests, contract vectors, fixtures, and sqlite roundtrip tests. The production mutation bridge derives inventory from runtime mutation descriptors for the document, config and presence surfaces.

## Audit Limits and Follow-up

No TODO/unimplemented/todo macro was found in shooting Rust/TypeScript source. The addAsset action deliberately produces `/mesh/placeholder.<format>` and is documented as a placeholder action; actual file-backed import is separately available. Export request JSON parse failures currently become DslValue::Null through map_or, which can obscure a malformed render request and deserves validation at the request producer/consumer boundary. UI shell and native runtime execution are being investigated by the other agents.

## Export Pipeline Audit

The React host effect `ShellHost/⚡️host-effects/📤️export-effects/🟦️.ts` iterates export items sequentially, calls `IconRenderHost/🚚️request/🟦️.ts`, and downloads each data URL. The request adapter maps mesh URLs through the published asset transport then calls the replaceable iconRenderPort interface. Its default implementation in the React host ports loads GLB geometry, builds scene and camera, renders true SVG through SVGRenderer or PNG through WebGLRenderer, and applies ellipse clipping when requested. These are real implementations, not output stubs. Browser export catches each failure only with console.error and exposes no cancellation or progress contract.

The native shell queues export batches and stages load, prepare, device, render/vector, retire and save phases. It selects GPU PNG or vector SVG rendering based on the request format, supports per-batch cancellation, and exposes counts/progress and translated status chrome. Existing tests cover native SVG output and export batches, request transport, and wire effect decoding.

A definite source type error was found in native shell doc-open icon-export `retire`: the `scene.close_step(cx)` match returns bare `false` in its Ok(false) arm despite the surrounding Result<bool, OriginalTurnFailure> signature. The coordinator was notified to route this to the native agent. This audit did not change production source.
