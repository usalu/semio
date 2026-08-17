# W4e HOT LAST — done

**Ticket:** `2026/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`  
**Scope:** `🌊️flow`, `🌀️procedural`, `🧩️puzzle`, `🧱️block`, `📐️cad`, `🌿️vcs`  
**Driver:** `handcraft-w4e-hot.mjs` (ticket temp; 33 grammar facets updated)

## What changed

Replaced bulk `statement*` / `family-geo` / wrong `grammar 2d.*` stubs with artifact-shaped `.grammar.semio` bodies aligned to each facet’s `🦀️component.rs` / `DslDocument` / `DslOps` surface:

| Plugin | Family / shape | Notes |
|--------|----------------|-------|
| **flow** | `family-graph` + document sections | `camera` / `widgets` / `layout` / `synapses` table; synapse rows use fused `EDGEARROW` wire literals |
| **procedural** (2d/3d) | graph + generation tables | Flow fixture + `generations` / `selected-generation` / `preview-text` |
| **puzzle** (2d/3d/5d) | document + SoA tables | `grammar puzzle.puzzle{N}d`, extension `puzzle{N}d`; camera/meta/nodes/edges (or 3d/5d tables) — not generic graph `document = statement*` |
| **block** (2d/3d/5d) | `family-catalog` + kind tables | `block.block{N}d` / `block{N}d`; node/object/part kind blocks, handle/vortex/grip catalogs |
| **cad** | `family-scene` + scene sections | `cad.document`; pane `*-geometry`, `*-objects`, `nodes` per `CadProjection` example DSL |
| **vcs** | document fields | `vcs.document` / `vcsdemo`; counter/title/notes/status/tags — removed erroneous `family-geo` |

**Graph-family** (`flow`, `procedural`): `edge-arrow` productions retain `EDGEARROW` (notation style guide).

**Protocols** (`🎒️pack` / `📡️spr`): unchanged — already dag-aligned framing; no drift found vs encoders in facet `🦀️component.rs`.

**CAD interaction-spec:** `🎬️interaction-spec/🦀️component.rs` exists; taxonomy has no interaction grammar facet under apps — no spec added.

## Verification

- Manual cross-check against canonical example DSL fixtures (flow, procedural2d, puzzle2d, block2d, cad default, vcs demo).
- `cargo test` / recognizer sweep not run on this host (out of scope for this wave slice).

## Files touched (plugin)

33 × `📖️component.grammar.semio` under:

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/{🗣️dsl,🔧️op,🔺️diff}/`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/{🌀️procedural2d,🧊️procedural3d}/{🗣️dsl,🔧️op,🔺️diff}/`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/{🗣️dsl,🔧️op,🔺️diff}/`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/{🗣️dsl,🔧️op,🔺️diff}/`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/{🗣️dsl,🔧️op,🔺️diff}/`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/{🗣️dsl,🔧️op,🔺️diff}/`

## Ticket temps

- `handcraft-w4e-hot.mjs` (generator)
- `wave-w4e-hot-done.md` (this file)
