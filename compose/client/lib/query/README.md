# @semio-tech/compose-query

Cypher-inspired **architect** query language for compose. Parses `MATCH` / `WITH` / `UNWIND` / `CALL` / `RETURN`, plans hand-crafted GraphQL documents against the compose schema, and executes them end-to-end via an injected async transport (native tests or WASM host).

## Commands

- `bun ../../../📜️script.ts query build` — release lib + wasm pkg
- `bun ../../../📜️script.ts query test` — `cargo test`
- `bun ../../../📜️script.ts query wasm` — wasm-pack only
