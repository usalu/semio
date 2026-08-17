# Coverage matrix (updated)

| Facet files | Count | Status |
|-------------|-------|--------|
| Handcrafted .semio (grammar+protocol) | 260 | family-based productions (no TEXT*/varint* stubs) |
| TS facades scaffolded | 260 | WASM wire pending — POLICY_TS_FACADE_ALLOWLIST |
| Grammar/protocol allowlists | empty | forcing function armed |

P5 note: `DocumentDsl`/`OpText` derive emission remains until every artifact ships a handcrafted Rust parser that does not call `dsl::__rt` text path. Specs are normative now; rust parsers migrate per wave.

Pilots refined: dag, fem2d, note (+ wave agents refining others).
