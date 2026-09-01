# FEM Serde Conversion Wave

The value derive supports `tag` plus `content`; FEM contains only supported `rename_all`, `default`, and internally tagged enum attributes. The previous additive derive conversion is therefore retained and serialization call sites can move directly to `ToValue`/`FromValue` and `json::{to_json_string, from_json_str}`.

The two standalone FEM JSON generator manifests also declared `serde_json`. They now use the owned `pack::json::Value` carrier. Its required mutable object/array accessors are first-party additions in the JSON codec.

Fixture adapters decode JSON text straight through `from_json_str`, retain canonical-object comparison against `DslValue`, and encode values through `ToValue::to_value`; no JSON text round trip is introduced for in-memory mutation values.

## Verification

- `rg 'serde_json|serde::|#\[serde'` over FEM Rust sources: zero matches.
- `rg 'serde(_json)?\s*='` over FEM manifests: zero matches.
- Both standalone `fem2d-1-any-json-engine` and `fem3d-1-any-json-engine` manifests pass `cargo check --manifest-path … --message-format=short`.
- `cargo check -p semio-s-plugin-fem --message-format=short` reaches `semio-framework-os-kernel` and stops on 48 pre-existing `ToValue`/`FromValue` errors in the store space-alternative/checkpoint mutations before compiling FEM.
- The pack mutable-accessor test compiles. Its first exact-filter invocation selected zero tests; the corrected invocation remained queued behind unrelated root-target builds and was cancelled to avoid another long-lived Cargo waiter, so runtime execution is not claimed.
- `cargo fmt -p semio-s-plugin-fem -- --check` parses the complete FEM crate without a syntax failure, then exits nonzero because the shared FEM/glue sources contain extensive outstanding formatting differences. A bulk formatter was not run because it would rewrite concurrent work outside this conversion.
