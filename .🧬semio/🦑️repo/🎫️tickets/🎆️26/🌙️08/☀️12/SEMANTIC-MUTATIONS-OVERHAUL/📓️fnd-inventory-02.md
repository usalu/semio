# FND-INVENTORY-02 Evidence

## Scope

Implemented the bounded mutation inventory packet in the root workflow, its repository-library regression block, and a separate neutral consumer/assignment fixture plus output schema. No production mutation apply was run. `compose/**` was excluded before the source-index walker entered a directory; the fixture contains only a virtual opaque compose path.

## Inventory Facts

- Inventory schema version 2 separates structural shape (`state`/`structuralState`) from execution assignment state (`executionState`) and explicit evidence.
- The canonical digest now includes the selected mutation roots, an ordered no-follow source/evidence roster, and the exact supplied or ticket-ledger bytes. An edit in an external command changes the inventory digest.
- Per-leaf records contain resolved consumer edges, only their own language/schema files, inbound command/editor/viewer/catalog/registry/test/oracle consumers, outgoing foreign-leaf dependencies, mounted aggregate evidence, helper targets, and unresolved mutation-like imports.
- Assignment rows are schema validated and keyed by the exact mutation-root plus direct-leaf directory. Missing, conflicting, and invalid rows remain unresolved; no agent name is invented. Inline ledger input deliberately has no filesystem-path provenance.
- Inventory work accepts scope, progress, and cancellation; source content reads are limited to selected mutation-root bytes plus source/schema/manifest/test evidence required for cross-owner consumers. Cancellation and ledger paths are root-contained, compose-excluding, and no-follow.

## Resolution Boundary

Rust module/use facts now come from the existing tokenizing discovery parser, including nested block comments, raw strings, lifetimes, `pub(crate) use`, and lexical inline-module scopes. The graph starts only at verified `Cargo.toml` `[lib] path` entries or conventional `lib.rs`/`main.rs`, follows actual `mod` and `#[path]` mounts, handles inline-module path accumulation, and maps named foreign crates only from their manifest package/lib names. A resolved edge therefore has a concrete mounted target source path; no last-token, leaf-alias, or helper-filename lookup remains.

The neutral fixture exercises conventional `lib.rs` → `mutations.rs` → direct leaf and `command.rs` `use crate::mutations::insert_page::Mutation`, a Cargo `[lib] path = "📦️glue.rs"` with nested inline scopes, a foreign crate with an explicit `[lib] name`, a lifetime before `use`, and a positive canonical emoji `#[path]` target. Unknown/dynamic/external imports remain unresolved at their actual source path, including sources outside mutation leaves. This remains deliberately bounded: crates without an indexed entrypoint, unsupported macro-generated modules, and ambiguous duplicate crate names are unresolved rather than claimed complete.

## Test Evidence

Initial focused runs were red while the taxonomy filename transition exposed an invalid JSON component-kind lookup and while outgoing cross-owner edges were incorrectly attached only to their destination. Both were fixed before the final focused run.

```text
bun nx run @semio-tech/repo-lib:test-quick -- -t 'resolves mutation consumers and schema-validated assignment evidence'
PASS: 1 test, 20 expectations, 0 failures
```

```text
bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️inventory-resolution-regressions/📜️script.ts
PASS: 0 failed regression groups (same-name, nested-comment, raw-string, and missing-ledger controls)
```

```text
SEMIO_TEST_BUDGET_MS=180000 bun nx run @semio-tech/repo-lib:test-quick -- -t 'direct mutation ownership' --timeout 30000
PASS: 15 tests, 286 expectations, 0 failures (18.53 seconds)
```

```text
bun nx run @semio-tech/repo-lib:test-quick -- -t 'requires a fresh clean terminal verification'
PASS: 1 test, 21 expectations, 0 failures
```

The broader direct-mutation filter exceeded its existing 30-second quick-test budget after unrelated codec coverage; its log is retained as `🧪️fnd-inventory-02-direct-suite.log`. It is not reported as a pass.

## Final Graph Replay

The graph caches one tokenized module/use fact packet per indexed Rust source and carries its source path through the module queue. Non-mutation unresolved imports are not plan blockers; mutation-like unresolved imports remain explicit at their real source path.

```text
bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️inventory-module-graph-regressions/📜️script.ts
PASS: inline-path-override, ordinary-child-module-directory, crate-prefix-must-not-use-child-local-mount, self-prefix-must-not-escape-inline-scope (4/4 compiler-oracled cases)

bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️inventory-inline-path-compiler/📜️script.ts
PASS: rustc inline nested `#[path = "."]` mount fixture (exit 0)
```

Replay input SHA-256: module-graph harness `98ef01d803fe9c6d9492ab222baf549b4b15a80fb65f0f66c37c37877d7a9833`; no-guess harness `c3cf6f3b588636cbca124b779973e1eb837456bcf95b03505940fc5ef2a82a5c`; inline compiler harness `ea529c6e4c28bb99e05f3883501114c54d1fd8070bf4261a2b6af456e9066d5d`; neutral fixture `49e60a98d4a61e98fff5e78c293b06e0ba93e41551752a7250699d433fea1157`.

## Files

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧫️fixtures/🔣️consumers.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🛂️schema/🔣️inventory.json`
