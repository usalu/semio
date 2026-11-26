# WGPU BIM browser validation

The standalone adapter probe ran in Chrome 158.0.8127.89 on this host and obtained an Apple Metal 3 WebGPU device with core features and limits. This establishes the browser graphics prerequisite.

The captured server uses the BIM playground's normal WGPU compositor and supplied native catalog. It reads the linked renderer bindings, transpiled linked BIM guest, emitted descriptors, and the existing shard, vendor, and font assets. It verifies that the linked BIM component, transpiled module and emitted descriptors have the same component digest before opening. The captured server listens on port 16402. The managed server retains its normal port 6402. The captured server skips the managed activation-file watcher and creates no activation receipt.

The shared captured module route returned HTTP 200 for the installed shard worker and font pack. The route reported 48,381 bytes for the worker and 8,757,072 bytes for the fonts. [Shared module routes](bim-shell-modules.ts).

The browser runner waits for the renderer modules, BIM bridge, and compiled descriptor through the server before testing the English editor and German viewer. It checks the live BIM readiness beacon, selected locale, accepted accessibility nodes, the House mesh and instances, nonuniform pixels inside that world surface, and the command palette opening and closing through a browser keyboard interaction. [Browser law](bim-wgpu-browser.mjs), [Captured server](serve-wgpu-runtime.ts), [Server and resource watcher](watch-wgpu-shell.mjs).

The English editor and German viewer tests are waiting for the linked renderer and guest. The adapter probe does not establish either application result.

## Changed files

```text
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🕸️model-graph/📡️session/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/📐️geometry/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/📐️geometry/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🔗️pairing/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🔗️pairing/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🧵️xml/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🧵️xml/🌐️view/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/📤️pages/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️constructions/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️document/🌐️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️document/📇️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️document/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️materials/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️names/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️openings/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️surfaces/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🎟️zones/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎛️chrome/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-model/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-model/🌿️gbxml/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-model/🧪️tests/🔬️unit/🌿️gbxml.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🪑️families/🧪️tests/🔬️unit/📦️packages/🦀️rust/🪞️mirror/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖥️play-app/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗃️io-and-retained-commands/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🎮️command-and-argument-labels/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🌿️gbxml/🖥️mounted/🔣️.json
✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/🌡️energy-envelope/🏠️box/🎚️io-control/🔣️.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/gbxml-control-native.rs
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/gbxml-text-replay.rs
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/verify-gbxml-control.py
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/replay-mounted-gbxml.py
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/run-gbxml-control.mjs
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/inspect-wasi-capture.py
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/wasi-paged-gbxml.py
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/bim-shell-modules.ts
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/bim-wgpu-browser.mjs
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/serve-wgpu-runtime.ts
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/watch-wgpu-shell.mjs
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/🔬️gbxml-io.md
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/🔬️wasi-capture.md
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/🔬️wgpu-live.md
```
