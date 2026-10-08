# Marker-free Fixture Schema Discovery Hole — 2026-10-08

Actual root fixture-boundary receipt Nx0/1m5s modules4349/scopes3539/findings0 is structurally incomplete for companion schemas still present. Read-only inspection establishes the exact hole; no source edits or rerun tests here.

## Exact Dataflow

Discovery `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:3052–3054` schemaCollectionContractPath only recognizes directory segments 🧬️schema/🛂️schema below collection. Filename-only 📐️schema.json/🔣️schema.json companions outside these directories therefore take the non-facet branch.

Inventory lines3506–3518 iterates sorted walked files and correctly recognizes collection ancestry. For non-facet JSON, line3512 rootContract requires an explicit string $schema URL starting json-schema.org. All ten companions have actual root JSON Schema type/properties/required but no $schema marker, so rootContract=false. line3513 falls back to schemaEmbeddedExampleAuthority.

Embedded detector lines3241–3250 does not classify the subject root as a schema. It only notices child keys ending schema whose child value carries a schema keyword; companion root properties/type/required are not such child keys. These documents therefore produce no diagnostic despite actual test readers compiling them. This is a content/physical authority classification hole, not module walk exclusion for these ten observed paths.

## Proposed Owner Fix and Neutral Cases

Main should enhance collection authority classification in discovery inventory and the independent harness implementation in `repo/🔨️modules/🧪️test/🟦️.ts` (schemaFixtureIsolationDiagnostics), preserving independent agreement rather than copying a marker-only rule.

- Reject explicit schema authority filenames (📐️schema.json, 🔣️schema.json, 🧬️schema.json and declared schema facet leaves) beneath actual collection ancestry even without dialect marker. Physical authority identity is sufficient; ordinary parser input must use an ordinary declared data filename.
- Recognize root JSON Schema shape without $schema: valid type string/list, properties map of schema objects/booleans, required string array, items schema, $ref string, combinators schema arrays, enum/const plus appropriate supporting schema grammar. Do not use mere keyword presence as a general expected-data detector.
- Test marker-free object schema with cases/limits, per-row setup+expectedVolume schema, root $ref/oneOf/boolean schemas in explicit authority files, nested schema under custom authority key, dialect markers with and without http URL, and format-only ordinary authority declarations.
- Preserve expected input payload with property named properties represented as an array (current Semio Graph payload), ordinary output with type/properties textual domain fields, rows containing schema strings/IDs, explicit ordinary inert parser input/expected emitted-schema declarations, real Fixture-named per-value report schema in an actual domain owner, and module-member test carve-outs.
- Test removal of $schema/$id/title/description cannot evade fixture authority diagnosis. Explicit inert declarations must not waive actual contract facet/authority file identity; actual inert expected parser data has ordinary filenames and declared data role.

Add cases exercising actual inventory temporary tree, not only direct synthetic detector calls, to prove walked placement reaches the new content/filename classification. Validate ordinary payload counterexamples independently using actual JSON parser/schema input discrimination, retaining existing 68 cases plus new cases and root current inventory receipt.

Raw ten-path observed ledger: `🗑️generated/oct8-boundary-auditor/fresh-remaining-physical-collection-schema-paths.json` includes one explicitly documented ordinary Graph false-positive excluded from ten count.

## Independent Harness Minimal Fix Boundary

Harness `repo/🔨️modules/🧪️test/🟦️.ts:4596–4606` isJsonSchemaDefinition only recognizes dialect URL; readSchemaDefinition4630 consumes it. Fixture isolation4828–4834 therefore independently shares the marker-free root miss. Physical authority helper4840 also only detects directory segments. Fix both independent entrypoints; changing discovery alone cannot make harness agree.

Concrete minimal recognition proposal:

1. First apply exact declared authority path/filename identity under collection ancestry; no content needed. No inert exemption for that identity.
2. For ordinary named JSON files, recognize explicit meta-schema URL OR a coherent root schema form: string $ref; valid schema type plus at least one grammar-valid constraint; properties must be object-map (not array/string), required must be array of strings, combinators must be arrays of schema objects/booleans, items must be object/boolean/allowed tuple depending supported dialect. For {enum}/{const}/boolean-only shapes, require declared authority identity or explicit schema definition role; ordinary data true/false/enum business payload cannot be globally condemned by shape alone.
3. Explicit ordinary inert data-role declaration remains the source of parser-input/expected-schema allowance. Generic data property named schema/properties/type does not establish authority by itself. Root algorithm should be shared conceptually with independent harness tests, not silently reused as sole oracle.

Neutral RED examples (expected findings are recommendations; auditor did not run new tests):

| Case | Path and JSON | Expected |
|---|---|---|
| marker-free companion | `owner/🧫️fixtures/📐️schema.json`, `{ "type":"object", "required":["cases"], "properties":{"cases":{"type":"array","items":{"type":"object"}}} }` | fixture-authority finding |
| marker-free named root | `owner/🧫️fixtures/contract.json`, `{ "type":"object", "additionalProperties":false, "properties":{"count":{"type":"integer"}} }` | finding unless explicitly declared inert ordinary data |
| definition-only ref companion | `owner/🧫️fixtures/🔣️schema.json`, `{ "$ref":"#/$defs/Row", "$defs":{"Row":{"type":"string"}} }` | finding |
| boolean authority companion | `owner/🧫️fixtures/📐️schema.json`, `true` | finding by path identity |
| Graph properties payload | `owner/🧫️fixtures/🔣️.json`, `{ "properties":[{"key":"integer","value":{"kind":"int","lexeme":"1"}}] }` | no finding |
| schema-instance declaration | ordinary config `{ "$schema":"https://schemas.example/domain.json", "type":"document", "properties":{"name":"literal"} }` | no meta-schema assumption; payload counterexample preserved |
| ordinary boolean expected | `owner/🧫️fixtures/expected.json`, `true` | no finding |
| declared parser specimen | ordinary `parser-input.json` coherent schema and explicit inertSchemaData declaration | no finding; same bytes moved to schema authority filename must fail |
| admitted facet bypass | noncollection `owner/🧬️schema/🌳️ownership/🔣️.json` with actual Process corpus preimage | schema authority census sees it despite facet placement exclusion |
| root controls without cases | Symbol decode/Drawing retirement/Producer actual preimages | whole-law authority finding from declared role/reader dataflow, not vocabulary name alone |
| real policy/report | DirectoryAccessPolicyV1 grants and actual produced ValueRefusal/Report contracts | preserve |

Actual temporary-tree inventory tests must create these physical paths so walker/ancestry/admitted-facet gaps are covered. A direct isJsonSchemaDefinition unit test alone cannot verify root check coverage.
