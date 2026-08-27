# FND-PUBLIC-LEAF-DERIVE-11

## Declared Write Set

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`
- the derive crate `Cargo.toml` for an owned compile-time hash dependency only
- OS DSL façade macro re-export only
- authority-local fixtures, schema, tests, and ticket harness evidence

## Frozen Boundary

The derive accepts exactly `#[mutation_leaf(contract = ::absolute::path)]`. It emits only the frozen lower `MutationLeaf` metadata implementation, using the existing validated source authority, strict sibling descriptor parser, workspace provenance token, and explicit caller contract path. It does not alter lower metadata/core traits, aggregate mutation behavior, registry logic, production leaves, or TypeScript policy.

## Coordination

The lower-contract path and trait/type names are supplied by `terra_test_presence`; this packet will consume them read-only after confirmation.
