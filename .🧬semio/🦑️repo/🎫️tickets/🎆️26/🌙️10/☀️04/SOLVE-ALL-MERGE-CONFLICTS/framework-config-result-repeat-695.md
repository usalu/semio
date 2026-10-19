# Framework Command Configuration Preservation, Repeat 695

Current neutral dispatch and targets combine with incoming target additions using recursive object/array source semantics. No scalar conflicts were found in26 existing project documents. Incoming-only held config/script sources use exact stash blobs. Fresh-byte guarded atomic writes; no builds, compilers, tests, dependency resolvers or runtimes. Remaining script body merges are tracked separately.

## Files

- `🧰️framework/📦️packages/🟦️typescript/📋️project.json`: incoming exact.
- `🧰️framework/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json`: structural JSON three-way.
- `🧰️framework/🔨️modules/🏃️process/📋️project.json`: structural JSON three-way.
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json`: structural JSON three-way.
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust/📜️script.ts`: incoming exact.
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`: structural JSON three-way.
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/📜️script.ts`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/📜️script.ts`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/📜️script.ts`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`: structural JSON three-way.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📋️project.json`: structural JSON three-way.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/📜️script.ts`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📋️project.json`: incoming exact.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`: structural JSON three-way.

## Remaining Scripts

- `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`.
- `🧰️framework/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts`.
- `🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🛍️products/🐾️pets/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`.
