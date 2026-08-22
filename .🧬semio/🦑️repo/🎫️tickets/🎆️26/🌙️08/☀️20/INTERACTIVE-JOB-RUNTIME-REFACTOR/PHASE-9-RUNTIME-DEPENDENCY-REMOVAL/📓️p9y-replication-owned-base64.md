# P9y Replication-Owned Base64

## Outcome

The replication crate no longer declares or calls the external `base64` crate. Its presence-pack JSON boundary now uses a strict owned RFC 4648 standard-alphabet codec while preserving the existing padded representation.

This packet removes the direct dependency from replication only. Other workspace consumers remain separate Phase 9 work and the global dependency identity is therefore not yet retired.

## Implementation

- `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`
  - adds a padded RFC 4648 standard encoder;
  - adds a strict decoder that rejects whitespace, invalid bytes, misplaced padding, non-multiple-of-four lengths, and non-canonical unused bits;
  - exposes a repository-owned `Base64Error` without leaking external types;
  - covers RFC vectors, all byte values and remainder shapes, plus malformed/non-canonical inputs.
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️component.rs`
  - routes presence-pack serialization and deserialization through the owned codec.
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml`
  - removes the direct `base64` dependency row.

## Differential Parity

The ticket-local `🧪️p9y-base64-differential` binary links the changed production replication crate and the external reference implementation. It compares:

- 4,097 deterministic payload lengths (`0..=4096`), both encode and decode;
- seven malformed or non-canonical vectors, which both implementations reject.

Result:

```text
[DEBUG] p9y base64 differential parity: 4097 payloads and 7 malformed vectors
```

The differential executable exited zero.

## Compiler And Test Gates

All commands used ticket-local target directories.

| Gate | Result |
|---|---|
| native debug tests | 187 passed, 0 failed |
| native release tests | 187 passed, 0 failed |
| `wasm32-unknown-unknown` build | success |
| `wasm32-wasip2` build | success |
| differential harness | success |

Commands:

```text
bun nx run @semio-tech/framework-replication-rs:test
bun nx run @semio-tech/framework-replication-rs:test -- --release
bun nx run @semio-tech/framework-replication-rs:build -- --target wasm32-unknown-unknown
bun nx run @semio-tech/framework-replication-rs:build -- --target wasm32-wasip2
cargo run --manifest-path .../🧪️p9y-base64-differential/Cargo.toml
```

## Dependency And Diff Census

- A scoped source/manifest search for `base64 =`, `base64.workspace`, `use base64`, and `base64::` returns zero hits in the replication tree.
- `cargo tree -p semio-framework-replication -e normal --depth 1` contains no direct `base64` row.
- Scoped `git diff --check` exits zero.

## Boundary Note

The codec is portable and public, but making replication the universal dependency of unrelated plugins would violate the repository's curated plugin dependency surface. Any broad migration must first place or reexport the same owned contract from an already-sanctioned neutral boundary; consumers must not add ad-hoc replication dependencies merely to reach this implementation.
