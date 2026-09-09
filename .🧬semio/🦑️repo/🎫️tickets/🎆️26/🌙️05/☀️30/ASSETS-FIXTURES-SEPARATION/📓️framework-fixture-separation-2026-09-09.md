# Framework Fixture and Asset Separation

## Scope and plan

This wave covers `🧰️framework` except `🛍️products/🦑️repo` and every `✏️s` subtree. It classifies actual consumers before moving data, removes unconditional Rust production compilation of testing examples, moves fixtures out of test-case folders to the nearest semantic owner, moves production media to `🖼️assets`, and moves schemas and test support source to their own semantic facets. The parent task owns the platform URI resolver and the ticket lifecycle.

The initial scoped inventory contained 738 fixture-named files. The first bounded audit selected direct production `include_str!` dependencies, direct fixture/schema files below test cases, production media served from fixture paths, and a public MCP fixture support module compiled by normal library builds.

## Findings and classification

- `🌉️abi`, UI host, UI WebGPU, OS host codec, and four Flow ledger/limit/trace constants were only consumed by tests. Their includes now live in the tests; normal modules no longer compile the examples.
- Flow's ABI JSON is a real schema contract. It moved from `🧫️fixtures` to `🧬️schema`; the production protocol include remains because the schema is production support.
- Infinite's PNG/JPG/PDF files are served runtime content. They moved byte-for-byte to `🖼️assets`, and authored routes now use `/infinite-assets`.
- OS media projection and plugin runner/completion JSON schemas are contracts for test examples. The schemas moved to owner `🧬️schema`; the example JSON moved to owner `🧫️fixtures`.
- MCP's `🧫️fixtures/🦀️.rs` was public and unconditionally compiled. It is test support, not example data, so it moved to `🧪️testkit` and the crate mounts it only under `cfg(test)`. Its nested test moved to the owner `🧪️tests`, while its natural-language evaluation rows remain data under owner `🧫️fixtures`.
- The OS scale workspace crate and JCO probe guest are executable test support and generator inputs, rather than fixture data. Their authored source, package manifests, generated executable artifact, and tests moved to OS `🧪️testkit`. The scale registry/catalog example rows remain under OS `🧫️fixtures`.
- Store durable-group's fixture loader is gated by the explicit `testkit` feature and is only called by test law suites. Plugin retained-command's helper is under `cfg(test)`. These are not default production dependencies, but the broader feature graph still needs a dedicated follow-up audit.

## Byte identity evidence

Every move in `oldPaths`/ `newPaths` below was checked with SHA-256 immediately before and after the move. All pairs matched. The physical move count at this report revision is 55 files. Fixture/example payloads were not rewritten. Two moved schemas subsequently received semantic `$id` path corrections; their move itself was hash-identical.

## Exact paths

`oldPaths`

```json
[
  "🧰️framework/🔨️modules/🌉️abi/🧪️fixtures/📊️.tsv",
  "🧰️framework/🔨️modules/🌉️abi/🧪️fixtures/📐️limits.tsv",
  "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🤝️mutation-leaf-contract/🧫️fixtures/🔣️.json",
  "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧭️mutation-leaf-source-contract/🧫️fixtures/🔣️.json",
  "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🕸️web-mercator-tile-oracle/🧫️fixtures/🔣️.json",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🧊️gltf-codec/🧫️fixtures/✅️expected-single-triangle.json",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🧊️gltf-codec/🧫️fixtures/🔗️external-buffer.gltf",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🧊️gltf-codec/🧫️fixtures/🔢️external-buffer.bin",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🧊️gltf-codec/🧫️fixtures/🔺️single-triangle-embedded.gltf",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🧊️gltf-codec/🧫️fixtures/🧊️single-triangle-embedded.glb",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️fixtures/📊️.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️fixtures/📐️browser-host-limits.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️fixtures/🧪️browser-host-framing/📊️.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧪️fixtures/📊️.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧪️fixtures/📐️surface-port-limits.tsv",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/✏️sketch/🖼️.png",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🏛️rathaus-ahlen-grundriss/🖼️.png",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🗺️site.pdf",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/📡️abi.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🪪️mutation-leaf-descriptor/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔣️mutation-leaf-json/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/✨️mutation-leaf-derive/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🏛️mutation-aggregate-source-authority/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🏛️mutation-aggregate-source-authority/🧫️fixtures/🧩️sources.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🧫️fixtures/🧭️domains.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🏷️mutation-attributes/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🧪️tests/🎚️canonical-os-ui-preferences/🧫️fixtures/🔁️event-replay.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🪪️artifact-admission/🧪️fixture/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🕸️media-projection/🧪️fixture/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🕸️media-projection/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner/🧪️fixture/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/🧪️fixture/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🧪️tests/🔬️quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🧪️eval/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🎭️profile/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🎭️profile/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/.nx-artifact.json",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📋️project.json",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.lock",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/🧩️component/🦀️.rs"
]
```

`newPaths`

```json
[
  "🧰️framework/🔨️modules/🌉️abi/🧫️fixtures/📊️.tsv",
  "🧰️framework/🔨️modules/🌉️abi/🧫️fixtures/📐️limits.tsv",
  "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🤝️mutation-leaf-contract/🔣️.json",
  "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🧭️mutation-leaf-source-contract/🔣️.json",
  "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧫️fixtures/🕸️web-mercator-tile-oracle/🔣️.json",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🧊️gltf-codec/✅️expected-single-triangle.json",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🧊️gltf-codec/🔗️external-buffer.gltf",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🧊️gltf-codec/🔢️external-buffer.bin",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🧊️gltf-codec/🔺️single-triangle-embedded.gltf",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🧊️gltf-codec/🧊️single-triangle-embedded.glb",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧫️fixtures/📊️.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧫️fixtures/📐️browser-host-limits.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧫️fixtures/🧪️browser-host-framing/📊️.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧫️fixtures/📊️.tsv",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧫️fixtures/📐️surface-port-limits.tsv",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/✏️sketch/🖼️.png",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🗺️site.pdf",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/📡️abi.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🪪️mutation-leaf-descriptor/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧫️fixtures/🧬️mutation-laws/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🔣️mutation-leaf-json/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/✨️mutation-leaf-derive/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🏛️mutation-aggregate-source-authority/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🏛️mutation-aggregate-source-authority/🧩️sources.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🛂️mutation-source-authority/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🏷️mutation-attributes/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🧫️fixtures/🎚️canonical-os-ui-preferences/🔁️event-replay.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/🪪️artifact-admission/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🖥️host/🧫️fixtures/🕸️media-projection/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🖥️host/🧬️schema/🕸️media-projection/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🏃️runner/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🏃️runner/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/⏳️completion/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/⏳️completion/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️testkit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️testkit-quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🧠️conformance/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/🎭️profile/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/🎭️profile/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/dist/component/.nx-artifact.json",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/📋️project.json",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.lock",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/👽️guest/🧩️component/🦀️.rs"
]
```

`modifiedPaths`

```json
[
  "🧰️framework/🔨️modules/🌉️abi/🦀️.rs",
  "🧰️framework/🔨️modules/🌉️abi/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs",
  "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🔮️oracle/🔣️.json",
  "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🕸️web-mercator-tile-oracle/🐍️.py",
  "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🕸️web-mercator-tile-oracle/🥒️.feature",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🔬️gltf-oracle-differential/🦀️.rs",
  "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📡️event.rs",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🔬️event-unit/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧊️surface_adapter.rs",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-surface-adapter-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔬️protocol-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️composite-attrs/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️mutation-aggregate-source-authority/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️mutation-attrs/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️mutation-leaf-derive/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️mutation-leaf-json/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️mutation-source-authority/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🧪️tests/🎚️canonical-os-ui-preferences/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🪪️artifact-admission/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner-self-tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️subset-macro/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🏃️runner/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️codec-abi-unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️workflow-standalone/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🖥️host/🧬️schema/🕸️media-projection/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️testkit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🧪️tests/🔬️quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️conformance/🧪️tests/🔬️quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔎️search/🧪️tests/🔬️quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧠️context/🧪️tests/🔬️quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs",
  "Cargo.toml",
  "🔒️dependencies.json",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/📋️project.json",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧪️testkit/⚖️scale/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  "🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🧪️destination-cases.json"
]
```

## Validation and remaining limitations

Validation is still in progress. Focused Nx/Bun runtime and compile results will be appended here. The authored WebGPU worker currently contains two stale generated `/infinite-fixture` records even though its source configuration is corrected; the owning generator must be rerun. The full scanner also reports legacy `🧪️fixture` facets and tests nested below fixture/testkit trees outside this first selection.
