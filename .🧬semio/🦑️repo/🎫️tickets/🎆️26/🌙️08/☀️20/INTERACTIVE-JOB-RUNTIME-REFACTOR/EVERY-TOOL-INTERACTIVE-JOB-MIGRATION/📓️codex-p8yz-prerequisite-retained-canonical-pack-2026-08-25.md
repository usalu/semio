# P8yz Shared Prerequisite: Retained Canonical Pack

Date: 2026-08-25

## Boundary

This separate shared subpacket adds a domain-neutral retained reader for the existing canonical SPK/1.0 wire. It adds no Procedural2d-only wire, compatibility adapter, alternate encoder, Cargo/script/manifest change, or wire change. The batch decoder remains outside the mounted facade and is structurally unreachable by interactive callers.

## Retained Canonical Layers

- Exact preflight/page/byte admission, producer handback, cancellation, deadline/progress, seal/resume, and incremental terminal-empty close.
- Fixed header/footer/anchor verification for magic, CRC-32C, version, flags, reserved bytes, file length, identity, and manifest span.
- Non-buffering segment framing with minimal lengths, limit checks before payload, CRC-32C, END, exact footer length, and bytewise raw-DEFLATE output.
- Fixed manifest/symbol/chunk registries, incremental UTF-8 ownership, document/schema/field-index events, authoritative span/count reconciliation, and document-content-hash verification.
- A nonrecursive pre-reserved value VM for tags `0x00..0x17`, including scalar/container/DSL/wire/TableSoA/packed/expr shapes, with one admitted byte and one scalar/string/item opportunity per grant.
- Explicit hostile behavior for malformed/minimality, truncation, UTF-8, CRC, depth/count/span, interruption, cancellation, and terminal-empty closure.

## Procedural2d Binding

`P2D2` is consumed before semantic allocation and `P3D3` fails closed. Every following byte is unchanged canonical SPK and flows through `RetainedPackSourceCursor`, anchor, segment/DEFLATE, catalog, value, and fixed typed snapshot/all-14 mutation owners. Mounted code has no `OwnedSchemaHexAuthority`, `decode_pack`, `decode_document`, full byte-vector, or `RecordValue` route.

Direct initializer replay and the real non-empty maintenance replacement/ACK law complete the production binding. The first P8yz-a RED was a principal fixture defect rather than a shared cursor defect; the repaired law appends and exactly verifies a valid non-empty retained synapse and field digest while preserving nested payload coverage.

## Fixtures and Verification

The language-neutral ledger is `🧰️framework/🔨️modules/🎒️pack/🧪️tests/🔣️retained-canonical-pack-laws.json`. It records exact admission, anchors, segments, registries, hostile cases, and all 24 value tags.

Scoped rustfmt, Bun JSON/exact ledger predicates, domain-local retained-route reachability, source-contract checks, scoped diff validation, and the exact census all pass. No Cargo, Nx, Wasm, browser, build, or root verifier ran. Runtime execution remains for the final permitted matrix.
