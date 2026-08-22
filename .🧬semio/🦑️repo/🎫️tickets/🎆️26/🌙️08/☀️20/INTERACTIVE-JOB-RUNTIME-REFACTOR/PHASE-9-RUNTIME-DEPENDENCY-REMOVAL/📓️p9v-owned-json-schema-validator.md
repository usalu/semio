# Phase 9v — Owned JSON Schema Validator

## Scope

Retire direct `jsonschema` dependencies from `semio-framework-schema` and `semio-framework-os-mcp` without exposing a third-party JSON representation. The replacement is a deterministic, bounded, cooperatively cancellable validator behind the framework schema contract.

## Exercised product contract

The source and generated-schema census found these exercised families: boolean schemas; local JSON Pointer `$ref`; `$defs`/`definitions`; `type` including type arrays; `properties`; `required`; schema-valued and boolean `additionalProperties`; `enum`; `const`; `items`; item and string bounds; `uniqueItems`; numeric inclusive/exclusive bounds; `multipleOf`; `allOf`/`anyOf`/`oneOf`/`not`; annotation keywords; and `x-*` extensions. Unsupported keywords fail compilation explicitly.

The public boundary accepts and returns owned strings, `SchemaError`, `ValidationProgress`, and `ValidationControl`; it does not expose the internal pack JSON tree or any former validator type. Object and property iteration order is deterministic. Expensive compilation, recursive validation, combinator traversal, and uniqueness comparisons count nodes against a configurable bound and observe a shared cancellation flag.

## Differential proof before dependency deletion

- `📝️p9v-jsonschema-schema-differential-2.txt`: supported baseline corpus, 1 passed.
- `📝️p9v-jsonschema-schema-differential-4.txt`: every exercised keyword family plus deterministic diagnostics/progress/cancellation, 3 passed.
- `📝️p9v-jsonschema-mcp-differential-1.txt`: the MCP seven-schema comparison was compiled against both implementations, but the MCP lib-test target stopped before this test on ten unrelated missing plugin-host/protocol symbols. Exact external errors: missing `semio_framework_plugin_host::poll_ready` at workspace lines 345/419; missing `ArtifactChannel`, `AppCommand`, `AppFrame`, and `Fault` in dispatch; missing `SearchHit`, `RevisionStamp`, `PreparedActionReport`, and `InvocationReport` in protocol.

After the differential corpus passed, both direct dependency rows and all source references were removed. The same corpora remain as fixed golden outcomes.

## Final gates

- `📝️p9v-owned-validator-native-debug.txt`: exit 0; 3 passed, 0 failed.
- `📝️p9v-owned-validator-native-release.txt`: exit 0.
- `📝️p9v-owned-validator-wasm.txt`: exit 0 for `wasm32-unknown-unknown`.
- `📝️p9v-mcp-native-debug.txt`: blocked by the ten external symbols listed above; zero schema-validator diagnostics.
- `📝️p9v-mcp-native-release.txt`: blocked by the identical ten external symbols; zero schema-validator diagnostics.
- `📝️p9v-mcp-wasm.txt`: blocked in the existing native Tokio networking feature graph because `mio` does not support `wasm32-unknown-unknown`; the framework-owned validator itself is WASM-clean per the preceding focused gate.
- `📝️p9v-dependency-ratchet.txt`: exit 0, 209 current versus 238 baseline; `rust:jsonschema` is in the 29-dependency removed set.
- Global Rust/Cargo/Cargo.lock `jsonschema` census: zero.

## Files

- `🧰️framework/🔨️modules/🧬️schema/🦀️validator.rs`
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️conformance/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml`
- `Cargo.lock`
