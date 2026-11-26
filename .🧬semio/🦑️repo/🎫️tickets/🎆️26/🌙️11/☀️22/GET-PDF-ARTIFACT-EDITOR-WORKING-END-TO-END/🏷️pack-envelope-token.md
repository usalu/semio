# Pack envelope token: `stdio.semio.<subset>`, not `s.stdio.semio.<subset>`

## Finding
- Truth: the schemas. `#[artifact_schema(id = "stdio.semio.animation")]` and `STDIO_SEMIO*_DOCUMENT_SCHEMA = "stdio.semio.*"` (Rust), the exported `record-spec` JSON (`envelope`, hence `token = <envelope>.pack v1`), the TS and Python twins (token read from the spec). The Rust `envelope_id()` is that schema id, so Rust, TS and Python all emit/accept `stdio.semio.<subset>`.
- Rename: commit 4f04c7f3398 (2026-11-04) dropped the namespace `s.` from schema/envelope ids (`s.stdio.semio.kit` -> `stdio.semio.kit`, `.mesh`). `s.stdio.semio` stays the artifact KIND (dialect `artifact_kind`, component id); only the schema/envelope id lost it. 10 of 17 subsets already used the new spelling; 7 (animation, document, graph, image, presentation, table, text) were left half-renamed.
- Stale spelling also lived in the document `schema` value inside the 17 packs' bodies (not only the envelope), the text DSL preamble `semio s.stdio.semio.X.dsl v1` + hex `schema=` field of 16 `.dsl.semio` files, hardcoded tokens in 8 native/sqlite Rust encoders (`token=match ...`, `encode_admitted(...)`), 6 Python mutation tests (`DSL_PREAMBLE`), 5 `.ksy`, `.ebnf/.g4/.abnf/.spicy/.grammar.semio` docs, the sqlite `prefix/encoding.json` and a dsl grammar fixture.

## Sweep (one pass, no compat)
- Text: `(?<![\w.-])s\.stdio\.semio\.<sub>\.(dsl|pack)` -> drop `s.` in 49 files (perl); hex `schema=` of `s.stdio.semio.<sub>` -> `stdio.semio.<sub>` in 16 `.dsl.semio`.
- Packs: `retoken-packs.ts` (TS twin `unwrapBinary/wrapBinary`) retokenised 17; `reschema-packs.ts` decodes each through its record spec, sets `schema`, re-encodes with the twin `encodePack` (17 changed, 22 already right). Run with `repo tools exec bun -- run ...`.
- Deliberately untouched: the `deflate` rows of the record-document fixture (raw->deflate vectors, independent of tokens); `s.stdio.semio.<sub>.mutations|snapshot|tool-command` in docs/ids, `capability:schema:schema:s.stdio.semio.*` in `artifact-definition.json`, `s.stdio.semio` artifact kinds.

## Results
- TS `record-document`: 19/19 (was 17/19). Python `record-document`: 18/18 (was 16/18).
- Not run: the Python `mutate-semio-*` Cucumber suites (need the repo harness `semio_repo_test`).
- Rust: `native-cargo.sh test -p semio-s-artifact-stdio-semio-{text,animation,base,base-examples}` ran (`🗑️generated/token-rust.log`) but did not reach the tests: `semio-plugin-time-travel` (missing `SupersededInput` in `semio_framework_time_travel`) and `semio-plugin-runtime-ops-document` (missing `PLUGIN_RUNTIME_INSTANCE_SLOTS`) fail to compile, concurrent edits by others, unrelated to the token sweep. Rerun once those build.
