# Wave W4b / W4c / W4d Done

**Ticket:** `2026/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`  
**Scope:** Remaining waves **not** covered by W4a (graph) / W4e (HOT).  
**ASCII `combos` paths:** none found under ticket/plugins (nothing deleted).

## Ownership (disjoint)

| Wave | Plugins | Artifacts |
|------|---------|-----------|
| **W4b** | 📕️norm | 15 norms (`din4108`…`en1990`, `vdi3805`, `iso16757`) |
| **W4c** | 🖨️raster 🎞️animate 💠️lowpoly 🖍️draw 📏️layout 🎥️shooting 📸️remodel 🗒️note | `raster`, `present`, `lowpoly`, `draw`, `layout`, `shooting`, `remodel`, `note` |
| **W4d** | 🏗️fem 🏛️architect 🏭️process 📖️playbook 🌍️gis 📋️forms 📜️imperative 🪐️space 🪵️sourcing ✒️writer | `2d`/`3d`, `program`, `process3d`, `playbook`, `gismap`/`gisterrain`, `forms`, `imperative`, `home`, `curate`, `writer` |


## What landed

- Every owned facet has artifact-specific `📖️component.grammar.semio` / `📡️component.protocol.semio` (not bulk family stubs alone).
- Keywords baked from sibling `🦀️component.rs` (`#[dsl(keyword|key|table|block)]`, `DslOps` / document fields) and example fixtures.
- Dialect headers retained on all grammars.
- Pack/spr protocols carry `schema <id>` + `start frame|record` framing (aligned with W4d fem pack/spr).
- Graph-family EDGEARROW not applicable on this ownership set (no graph-wire artifacts here); W4a owns EDGEARROW wires.

### Finish pass (this agent)

Closed remaining gaps after W4b/W4c/W4d wave agents:

1. **✒️writer** — replaced generic `family-embed` `field*` stubs with projection keywords `schema` / `id` / `language-id` / `uri` / `text`(+fence via `lang_from`), ops `set-text` / `set-document`, diff `text` / `document`.
2. **W4c scene + writer protocols** — injected `schema` + `start frame|record` on pack/spr for raster, present, lowpoly, draw, layout, shooting, remodel, note, writer.

Drivers: `handcraft-w4bcd-finish.mjs`, prior `handcraft-w4d-eng.mjs`, `🔧️refine-w4c-scene.mjs`, W4b norm refine.

## File counts (facet specs under ownership)

| Kind | Count |
|------|------:|
| Grammar `.semio` (`dsl`/`op`/`diff`) | 105 |
| Protocol `.semio` (`pack`/`spr`) | 70 |
| **Total facet specs** | **175** |

### Per plugin

| Wave | Plugin | Artifacts (g+p) | Files |
|------|--------|-----------------|------:|
| W4b | 📕️norm | 15 × (3g+2p) | 75 |
| W4c | 🖨️raster | raster | 5 |
| W4c | 🎞️animate | present | 5 |
| W4c | 💠️lowpoly | lowpoly | 5 |
| W4c | 🖍️draw | draw | 5 |
| W4c | 📏️layout | layout | 5 |
| W4c | 🎥️shooting | shooting | 5 |
| W4c | 📸️remodel | remodel | 5 |
| W4c | 🗒️note | note | 5 |
| W4d | 🏗️fem | 2d, 3d | 10 |
| W4d | 🏛️architect | program | 5 |
| W4d | 🏭️process | process3d | 5 |
| W4d | 📖️playbook | playbook | 5 |
| W4d | 🌍️gis | gismap, gisterrain | 10 |
| W4d | 📋️forms | forms | 5 |
| W4d | 📜️imperative | imperative | 5 |
| W4d | 🪐️space | home | 5 |
| W4d | 🪵️sourcing | curate | 5 |
| W4d | ✒️writer | writer | 5 |
| | | **Sum** | **175** |

### Session delta (finish pass)

**21** facet files rewritten this pass (3 writer grammars + 18 pack/spr protocols) — see `🧪w4bcd-changed-files.txt`.

## Verification

- Cross-checked writer keywords against `WriterProjection` / `WriterOperation` / `WriterDiff` and jack DSL fixture.
- Protocol schema ids taken from artifact `*_SCHEMA` constants in `🦀️component.rs`.
- `cargo test` / recognizer sweep **not run** on this host.
