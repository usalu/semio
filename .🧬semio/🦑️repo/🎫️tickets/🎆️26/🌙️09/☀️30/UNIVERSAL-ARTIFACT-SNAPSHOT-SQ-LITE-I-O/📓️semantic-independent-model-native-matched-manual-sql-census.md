# Model Native Matched Manual SQLite Census

Actual Bun 1.3.14 SQLite execution independently authored semantic cells against current Model SQL. Owner projection/decoder/visitor was not called. Retained inputs are `📥️inputs/semio-model-complete-semantic/handcrafted-native-matched-demand-specifications.json`, sibling closed contract schema and `📜️script.ts`.

| Case | Semantic Rows | Semantic Value Bytes | Public Rows | Public Binary Value Bytes | Public Text Value Bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Full | 99 | 6496 | 100 | 6538 | 6536 |
| Metadata Retaining Empty | 1 | 25 | 2 | 67 | 65 |

Semantic schema accounting is 3230 bytes, nine tables, maximum placement width 31. Public metadata adds 293 schema bytes and one six-column table, giving 3523 bytes/ten tables. Public values add exactly 42 Binary or 40 Text bytes for Model's five-letter subset. All six manually populated databases passed actual SQLite integrity and foreign key checks.

Full fields reproduce current Native fixture constructor: four spatial kinds/site-building-storey-space parent chain; ten element classes including IfcCustom世界; geometry none/brep/mesh cycling with external-geometry literal target; five even elements reference spatial-3 and five odd elements retain null spatialId. Each element owns Pset typed with duplicate key same text/number plus boolean. Text retains Grüße, embedded NUL and 世界. Number is original fixture decimal 1.2345678901234567 captured as exact Binary64 word 3ff3c0ca428c59fb. Fourteen placements contain ten identity Binary64 fields each. Six relations preserve Native camelCase tagged source variants, SQL snake_case labels and Other custom relation. Actual independent relation join returns spatial-0 to element-0 through element-5 in order. SQL hex text query preserves embedded NUL byte identity.

Empty clears spatial/elements/relations while preserving stdio.semio.model schema. Source field nulls are deliberate: actual Model TypeScript interfaces and Source relationship handling require parentId/spatialId null; Rust Option ToValue returns Null. Native derived rename_all camelCase preserves parentId/spatialId, tagged class/geometry/property/relation variants and ordinary spatial-kind strings. No plain-number float is supplied to Source: all 150 full scalar words hydrate to bigint and passed actual framework parseBinary64. Actual framework encodeIeee754Cells admitted all fourteen placement rows at width31 and ten number property rows at width10. Exact hydrate parsed through Bun.Transpiler, and closed constant authority passed strict Ajv2020.

Actual Model schema snapshot TypeScript currently defines interfaces only and exports no parseSemioModelSnapshot. A repository search of the complete Model subtree confirmed this limitation. The unrelated parseSemioModelSnapshotText guards a preamble/body carrier and is not a typed semantic parser. This report does not claim an unavailable Model domain-parser check, owner projection runtime, strict TypeScript compilation, Native compiler, cancellation or retirement qualification. Numeric carrier runtime was verified through actual framework parsers/helpers before authority delivery.
