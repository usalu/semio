# Runtime Dependency Inventory (derived, not hand-written)

Produced by the new `plugin-dependency/parity` gate in the root `📜️script.ts` on 2026-08-16.
Every row is a real Cargo dependency on a sibling plugin crate that must gain a matching
`.depends_on(<id>, <version-req>)` declaration in its plugin/extension builder. The gate holds
these at `medium` until the runtime API lands, then they shrink to zero as lanes adopt it.

Total: 61 declarations to add across 40 owners.

| Owner | Must declare |
|---|---|
| `✏️s/🔌️plugins/✒️writer` | `stdio`, `trinity` |
| `✏️s/🔌️plugins/➗️mathematical` | `stdio` |
| `✏️s/🔌️plugins/🌀️procedural` | `flow-extension-brep`, `flow-extension-dictionary`, `flow-extension-list`, `flow-extension-logic`, `flow-extension-math`, `flow-extension-primitive`, `flow-extension-text`, `stdio` |
| `✏️s/🔌️plugins/🌊️flow` | `stdio` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep` | `stdio` |
| `✏️s/🔌️plugins/🌍️gis` | `stdio` |
| `✏️s/🔌️plugins/🌿️vcs` | `stdio` |
| `✏️s/🔌️plugins/🎞️animate` | `stdio` |
| `✏️s/🔌️plugins/🎥️shooting` | `stdio` |
| `✏️s/🔌️plugins/🎪️demonstrator` | `cad`, `gis`, `procedural`, `process`, `puzzle`, `sourcing`, `stdio` |
| `✏️s/🔌️plugins/🎬️sequence` | `imperative-control`, `imperative-effect`, `imperative-math`, `imperative-text`, `stdio` |
| `✏️s/🔌️plugins/🏗️fem` | `stdio` |
| `✏️s/🔌️plugins/🏛️architect` | `stdio` |
| `✏️s/🔌️plugins/🏭️process` | `stdio` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal` | `process` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic` | `process` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete` | `process` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood` | `process` |
| `✏️s/🔌️plugins/💠️lowpoly` | `cad`, `stdio` |
| `✏️s/🔌️plugins/💡️reasoning` | `stdio` |
| `✏️s/🔌️plugins/📋️forms` | `stdio` |
| `✏️s/🔌️plugins/📏️layout` | `stdio` |
| `✏️s/🔌️plugins/📐️cad` | `stdio` |
| `✏️s/🔌️plugins/📕️norm` | `fem`, `stdio` |
| `✏️s/🔌️plugins/📖️playbook` | `stdio` |
| `✏️s/🔌️plugins/📜️imperative` | `stdio` |
| `✏️s/🔌️plugins/📸️remodel` | `stdio` |
| `✏️s/🔌️plugins/🔋️energy` | `stdio` |
| `✏️s/🔌️plugins/🔱️trinity` | `stdio` |
| `✏️s/🔌️plugins/🕸️dag` | `stdio` |
| `✏️s/🔌️plugins/🖍️draw` | `draw-fsm`, `stdio` |
| `✏️s/🔌️plugins/🖨️raster` | `stdio` |
| `✏️s/🔌️plugins/🗒️note` | `stdio` |
| `✏️s/🔌️plugins/🧩️puzzle` | `stdio` |
| `✏️s/🔌️plugins/🧱️block` | `stdio` |
| `✏️s/🔌️plugins/🪐️space` | `stdio` |
| `✏️s/🔌️plugins/🪵️sourcing` | `stdio` |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs` | `sourcing` |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows` | `sourcing` |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams` | `sourcing` |
