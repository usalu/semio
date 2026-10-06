# Generic Option Syntax Review

Read-only semantic handoff to the existing mutation/inference owner. The hand-authored source-owned map and twelve exact source hashes are retained under `📥️authored-inputs/generic-option-syntax-map`. It is an authored review input, not a runtime module or generated permanent script.

## Owning Scopes

| Authored Object | Actual Native Owner |
|---|---|
| Scale options | semio / viz / scale, 18 stored controls including unknown |
| Coordinate options | semio / viz / coordinate, 19 keys |
| Transform options | semio / viz / transform, shorthand plus explicit generic extension at line2100 |
| Layout options | Registered table algorithm selected first; exact24 authored-layout vocabularies |
| Layer options | semio / viz / plot plus forwarded semio / viz / mark, with plot-owned controls taking precedence |
| Guide options | axis or legend; the generic grid inference lowers to an invisible axis, while native standalone grid owns only x/y/minor/ticks |

All24 layout maps were checked against the actual literal vocabulary, with width/height as explicit common additions. They are not inferred from unrelated globally matching key names.

## Encoding Semantics

Identifier values name actual table/column/scale/algorithm/enum tokens and must preserve underscores. Composite column lists (columns/groupby/keys/stages), coordinates and record grammars require structure-preserving encoding, not literal text escaping. Native floating-point slots consume expressions (including pi and owned numeric macros); bare numbers also remain valid. Boolean/int slots and comparison literals are scalar values. Text fields are literal captions or formatter specs; TeX fonts, native dimension styles and TikZ styles require their explicit style grammar. Paint values use the existing paint owner. The exact schema contract should preserve type validation independently of syntax encoding; neither type coercion nor global key-name heuristics is authorized by this review.

## Owner-Specific Distinctions

The generic transform value key is a comparison operand, while catalogue shorthand value names a column. Catalog family figure/encoding/grammar reset defaults own their local samples/swatches even when shared mark/guide keys use the same name. Axis at is a single crossing coordinate; legend at is an x/y pair. Layout contour/density thresholds is a numeric target-count control, whereas bin thresholds accepts directive/explicit-list records. Coordinate axes accepts either an integer count or a comma-separated identifier list. Scalar enum anchors may contain spaces (north west); do not apply the restricted table token validator to every enum string.

The actual scale stored unknown-value control is distinct from no-op unknown handlers. It must preserve the existing typed Undefined/Null/String/Bool sentinels and uses paint projection only for actual paint scales. Ordinal/band/point domains require typed cells. A source-string false, boolean false, empty string, null and omitted row cell are distinct.

## Required Runtime Controls

Execute mutation-to-TS/Rust inference and the actual emitted compiler for an underscored field, a live owned numeric macro/expression, grouped piecewise records, key lists, styles, paints and numeric lists. Typed table and ordinal-domain vectors must decode the distinct null/omitted/empty/string/bool cases and compare physical/numeric output against an independent oracle. No emitter or compiler PASS is inferred from this source map.
