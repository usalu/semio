# Wave W4a graph — done

## Scope (disjoint)

- `✏️s/🔌️plugins/🕸️dag` / artifact `🕸️dag`
- `✏️s/🔌️plugins/💡️reasoning` / artifact `🔌️wires`
- `✏️s/🔌️plugins/🎬️sequence` / artifact `🎬️sequence`
- `✏️s/🔌️plugins/➗️mathematical` / artifact `➗️mathematical`
- `✏️s/🔌️plugins/🔱️trinity` / artifacts `🔌️jack`, `♻️rewrite` (facets only)

## Landed

- Replaced bulk `family-graph` statement templates with artifact-specific `📖️component.grammar.semio` bodies keyed off sibling `🦀️component.rs` / DSL mirrors (document tables/blocks, op variant keywords, diff slots).
- Kept `dialect grammar`, `use family-graph` (or `family-embed` for `rewrite` document), and fused `EDGEARROW` wire productions on every graph-family facet that carries `WIRE` cells.
- Refined `📡️component.protocol.semio` for pack/spr: `schema`, `start frame|record`, unchanged magic/record framing aligned with dag pilot.
- Updated `🟦️component.ts` per facet: `parseDsl`/`printDsl`, `parseOp`/`printOp`, `parseDiff`/`printDiff`, `encode`/`decode` — all throw until WASM bindgen wires in.

## Apply script

`w4a-apply-graph-grammars.mjs` in this ticket folder (re-runnable).

## Verification (not run on agent host)

- `cargo test -p semio-s-plugin-dag` (and sibling plugin crates) facet round-trip laws
- Grammar recognizer sweep when fixture-sweep entry exists per artifact

## File change count

**60** plugin artifact facet files updated in the apply pass (+ this report).
