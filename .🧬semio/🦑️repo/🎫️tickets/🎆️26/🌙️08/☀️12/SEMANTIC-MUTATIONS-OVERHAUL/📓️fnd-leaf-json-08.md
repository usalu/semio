# FND-LEAF-JSON-08

## Scope

Private compile-time parsing and token emission for the fourteen-field direct-mutation descriptor. No public trait, proc-macro entry point, core contract, registry, or production leaf is changed by this packet.

## Contract Input

The authoritative schema is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json`. Neutral vectors live beside the derive authority under `🧪️tests/🧬️mutation-leaf-json`.

The vectors distinguish schema acceptance from parser acceptance. In particular, JSON Schema cannot observe duplicate raw object keys after JSON parsing; the pending byte-level parser must reject them. Valid integral JSON number spellings (`1.0`, `2e0`, `-0.0`, and `4294967295.0`) are retained as accepted vectors, while fractional and out-of-range values are rejected.

## Source Hold

Root started a real STDIO Cargo selection. The derive sources remain unmodified until root explicitly releases that compiler hold. The ticket oracle currently validates only the fixture and authoritative Ajv behavior; it deliberately does not claim Rust-parser or emitted-token execution.

## Planned Private Boundary

The released implementation will keep an owned descriptor representation private, require exact owner equality from `MutationSourceAuthority`, reject missing/extra/duplicate keys, and emit full static `::semio_framework_os_kernel::MutationLeafDescriptor` tokens. The derive crate will not depend on the kernel; only emitted client tokens name that existing core type.

## Current Evidence and Freeze

The private parser/emitter is now present in both derive mirrors. The fixture has 72 vectors: all fourteen key omissions and wrong types, required string/list failures, enum/binary boundaries, owner mismatch, raw duplicate keys (including Unicode escaping), trailing JSON, and accepted variants covering all emitted enum variants plus null and numeric wire forms. Ajv accepts or rejects all 72 as pinned by the fixture.

The first standalone Rust attempt retained `only metadata stub found` diagnostics because paired `.rmeta` artifacts were not supplied; this was an artifact invocation defect, not incompatible-library evidence. A subsequent attempt found the global target artifacts absent before invoking rustc. Its cause is unestablished. The ticket harness now requires `SEMIO_TEST_COMPILER_ARTIFACT_DIR` (or `CARGO_TARGET_DIR`) and pairs same-hash `.rlib`/`.rmeta` artifacts before it executes the exact private parser region and exact unchanged core descriptor region. Derive sources are frozen pending root's isolated registered gate and retained target directory.
