# DIN V 18599 `dhw.dwellingCount` and the document-contract oracle

## Cause
The DIN V 18599 JSON schema, the Rust model (`dwelling_count: u32`), the TypeScript parser, the GraphQL schema, the field-meta and the SQLite snapshot all require `dhw.dwellingCount` (integer, 0..u32 max). It is semantically required: the residential hot-water profile in DIN V 18599-10 (check `din18599.10.residential-dhw`) weights demand by net floor area and number of dwellings. Non-residential buildings are modelled precisely by the allowed value `0` (minimum 0), so the field is not optional.
The only document that lacked it was the authored valid document in `✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🪪️document-contract/🔣️.json` (the din18599 vector), which the oracle validates against the artifact schema.

## Fix
Fixed the data, not the schema: added `"dwellingCount": 1` to the `dhw` object of the Residential document in the contract fixture. Schema, Rust, DSL and generated twins are unchanged.

## Other documents checked
Every file containing `specificDemandKwhPersonA` was compared with its `dwellingCount` count. All mutation fixtures, sqlite snapshot fixture and cross-checks already carry the field. The remaining mentions without it (Python compliance oracle, sqlite TS reader/writer type, field-meta/check Rust) only read specific fields and are not documents. The oracle covers all Norm editors through `vectors.artifacts`; no other owner reported a mismatch.

## Results
- `s-norm-composition-tests:verify-norm-document-contract`: PASS (212 snapshots, 42 diffs).
- `norm-js:test`: the document-contract oracle test now passes, but the suite still fails with 132 failures unrelated to this change: 122 in `norm wire twins` (din16798 and en1998 mutation fixtures, handled by another agent) and 10 others (schema-sync: 21 optional/bounds mismatches e.g. en1991 `fatigueModels`; twin drift in en1998 graphql/rust/ts/proto; field-meta choices; schema typecheck; insert/rule/payload/catalogue corpora). None mention din18599 or `dwellingCount`.

## Changed files
- `✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🪪️document-contract/🔣️.json`

No Rust/DSL/schema changed, so the browser guest component needs no rebuild.
