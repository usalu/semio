# Wave W4c Scene Done

Refined artifact-specific `.semio` facet specs + TS WASM facade stubs for geometry/media scene plugins (note uses embed).

## Ownership

| Plugin / Artifact | Id | Family | Facets |
|-------------------|----|--------|--------|
| 🖨️raster / 🖨️raster | `raster` | family-scene | 5 grammars/protocols + 5 TS stubs |
| 🎞️animate / 🎬️present | `present` | family-scene | 5 grammars/protocols + 5 TS stubs |
| 💠️lowpoly / 💠️lowpoly | `lowpoly` | family-scene | 5 grammars/protocols + 5 TS stubs |
| 🖍️draw / 🖍️draw | `draw` | family-scene | 5 grammars/protocols + 5 TS stubs |
| 📏️layout / 📏️layout | `layout` | family-scene | 5 grammars/protocols + 5 TS stubs |
| 🎥️shooting / 🎥️shooting | `shooting` | family-scene | 5 grammars/protocols + 5 TS stubs |
| 📸️remodel / 📸️remodel | `remodel` | family-scene | 5 grammars/protocols + 5 TS stubs |
| 🗒️note / 🗒️note | `note` | family-embed | 5 grammars/protocols + 5 TS stubs |

## What changed

- Replaced generic `layer = IDENT "@" …` family-scene stubs with productions keyed from each artifact's `🦀️component.rs` (`#[dsl(keyword)]`, op enum variants, document fields).
- `🗒️note` switched to `use family-embed` (blocks + `tex` fence).
- Dialect headers kept; pack/spr framing shared with artifact-specific protocol ids.
- Facet `🟦️component.ts` stubs updated; package barrels re-export them (animate keeps React present export).

## File count

| Kind | Count |
|------|------:|
| `.semio` facet specs | 40 |
| Facet `🟦️component.ts` stubs | 40 |
| Package `📦️index.ts` barrels | 8 |
| **Total** | **88** |
