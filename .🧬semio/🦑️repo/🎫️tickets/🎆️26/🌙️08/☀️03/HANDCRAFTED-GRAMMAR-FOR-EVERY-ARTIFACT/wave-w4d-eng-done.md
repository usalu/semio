# W4d engineering/spatial — done

**Ticket:** `2026/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`  
**Scope (ownership):** `🏗️fem`, `🏛️architect`, `🏭️process`, `📖️playbook`, `🌍️gis`, `📋️forms`, `📜️imperative`, `🪐️space`, `🪵️sourcing`
**Driver:** `handcraft-w4d-eng.mjs` (+ `🔧️w4d-fix-op-keywords.mjs` for OpText keyword alignment)

## What changed

Replaced bulk family-template stubs (`statement*`, generic `stock*`, `feature*`, wrong graph ops) with artifact-shaped `.grammar.semio` bodies keyed off sibling `🦀️component.rs` / `DslDocument` / `DslOps` mirrors (and example DSL fixtures for section order). Refined pack/spr `.protocol.semio` to dag-aligned `schema` + `start frame|record` framing.

| Plugin | Artifact | Family / shape | Document keywords (from `component.rs` / fixtures) | Op keywords |
|--------|----------|----------------|----------------------------------------------------|-------------|
| **fem** | `◻2d` | `family-sheet` | `elements`, `analysis`, `nodes`, `regions`, `materials`, `sections`, `supports`, `load-cases`, `combinations` | `set-node`…`set-document` |
| **fem** | `🧊️3d` | `family-sheet` | + `solids`; load/element tags `frame`/`nodal`/`area`/`solid` | + `set-solid`/`remove-solid` |
| **architect** | `🏛️program` | `family-catalog` | `schema`/`meta`/`project`/`governance` + register tables (`stakeholders`…`traces`) | JSON-line `map*` (`OpText` is serde JSON, not kebab DslOps) |
| **process** | `process3d` | `family-recipe` | `resolved-up-to`, `workshop`, `stock`, `steps` | `steps-*`, `machines-*`, `stock`, `cursor`, `document` |
| **playbook** | `playbook` | `family-recipe` | `schema`/`id`/`version`/`title`, `steps` (+ block/condition) | `add-step`…`update-playbook` |
| **forms** | `forms` | `family-recipe` | same playbook shape (`FormSpec` = `PlaybookSpec`) | same as playbook |
| **gis** | `gismap` | `family-geo` | `positions`/`routes`/`regions` tables | `add-position`…`patch-region`, `set-document` |
| **gis** | `gisterrain` | `family-geo` | `gisterrain` (+ `exaggeration`/`imported-features-json`), `origin`/`position` | `set-exaggeration`, `set-imported-features`, `set-document` |
| **imperative** | `imperative` | document fields | `schema`, `steps`, `seed`; step kinds `state.set`/`log.print`/… | `add`/`remove`/`move`/`patch` (`ImperativeOperationDsl`) |
| **space** | `home` | document fields | `schema`, `gen` | `no-operation`, `set-catalog-generation` |
| **sourcing** | `curate` | `family-catalog` | `stock` list + `curated` table; geometry `box`/`frame`/`slab`/`mesh` | `set-document` / `document` |

## Protocols

All owned pack/spr facets now carry:

- pack: `schema <doc-schema>`, `start frame`, unchanged SPK framing
- spr: `schema <op-schema>`, `start record`, unchanged record framing

## Verification

- Cross-checked keywords against artifact `🦀️component.rs` (`#[dsl(keyword|key|table|block)]`, `DslOps` / `*OperationDsl` mirrors) and bundled example `.dsl.semio` fixtures.
- `cargo test` / recognizer sweep not run on this host (out of scope for this wave slice).

## Files touched (plugin)

**55** facet files (33 grammar + 22 protocol) under the nine owned plugins only.

## Ticket temps

- `handcraft-w4d-eng.mjs`
- `🔧️w4d-extract.mjs` / `🔧️w4d-fix-op-keywords.mjs`
- `🧪w4d-*.txt` keyword/example extracts
- `wave-w4d-eng-done.md` (this file)
