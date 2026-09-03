# Artifact Bootstrap Protocol

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`  
Scope: P1 only — replication wire contract, bounded assembler, and language-neutral vectors.

## Outcome

Rust and TypeScript now expose one explicit, format-versioned, descriptor-bound `ArtifactBootstrap` transfer for the canonical `(pack, spr)` pair. The new public transfer is independent from the existing database-private `Bootstrap::Snapshot`, `SnapshotChunk`, and `SnapshotDone` frames; those variants retain their existing tags, fields, and byte encoding.

The implementation adds no runtime dependency and does not add `serde` derives to protocol leaf types. Rust uses the existing first-party `semio_framework_hash::Sha256`; TypeScript uses host Web Crypto. The TypeScript tests independently validate fixture hashes through Node `createHash` and validate the neutral fixture through the already-installed AJV 2020 implementation.

## Wire Contract

Existing bootstrap tags remain `None = 0`, `Snapshot = 1`, and `Tail = 2`. The additive public tag is:

- `Bootstrap::ArtifactBootstrap = 3`

Its canonical body is:

```text
format_version varint
descriptor_hash [32]
artifact_schema str
artifact_kind str
pack_schema_hash [32]
baseline_frontier FrontierSummary
pack_hash [32]
spr_hash [32]
pack_length varint
spr_length varint
chunk_count varint
aggregate_hash [32]
required_tail_frontier FrontierSummary
inline bool
[pack bytes, spr bytes] when inline
```

The additive server-frame tags are:

- `10`: `ArtifactBootstrapChunk { descriptor_hash, index, bytes }`
- `11`: `ArtifactBootstrapDone { descriptor_hash, chunk_count }`

Chunked content is the exact `pack || spr` byte stream. The declared pack length is the only split boundary. The aggregate hash is SHA-256 of the exact `pack || spr` stream, in addition to the independent pack and SPR SHA-256 values.

The format version is `1`. Default ceilings are 64 MiB total content, 16,384 chunks, and 4,096 bytes per chunk. Caller-selected budgets may be smaller but cannot raise the wire chunk ceiling.

## Assembly And Integrity Invariants

Both implementations enforce before completion:

- nonzero descriptor, pack-schema, pack, SPR, and aggregate hashes;
- nonempty pack and SPR with checked total length;
- nonempty bounded artifact schema and kind;
- one document across baseline and required-tail frontiers, with tail ordinals not preceding baseline;
- exact descriptor binding at transfer start, on every chunk, and on the done frame;
- ordered, unique, nonempty chunks with exact declared count and total length;
- bounded allocation before payload acceptance;
- per-part SHA-256 and aggregate SHA-256;
- cancellation and deadline checks during construction, every push, and finish;
- a second finish guard after hashing and before ownership transfer;
- monotonic received-byte/chunk progress;
- retirement of staged storage after cancel, deadline, malformed input, hash failure, explicit abort, or successful ownership transfer.

TypeScript validates every public `readonly number[]` hash, inline payload, and pushed chunk element as an integer byte before any `Uint8Array.set`, preventing silent modulo coercion. `finish` returns the pair only after all validation. A failed or late-cancelled finish returns no completion value and leaves `retainedBytes == 0`. Rust provides the same success-only ownership boundary and `retained_bytes() == 0` retirement observation.

P1 does not emit `ArtifactEvent::SnapshotReplaced`; that store event belongs to P3. The P1 test therefore proves no pair/completion escapes a failed assembler, then creates a fresh assembler and completes the reconnect transfer.

## Language-Neutral Fixture

The shared fixture is at `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/🧫️artifact-bootstrap/🔣️.json`, with its strict draft-2020-12 schema beside it at `🔣️.schema.json`.

It fixes:

- pack bytes `PACK:v1:alpha`, length 13;
- SPR bytes `SPR:v1:edit-1\nedit-2`, length 20;
- three chunks with lengths 12, 12, and 9;
- pack SHA-256 `169695f1e91fb6462785847e507dae4321c0c393170703c8faf3f144906f8565`;
- SPR SHA-256 `e49050e06c4b0df5dca1d29013512af40f7edb637e52c2eb5fce1dc1167dff59`;
- aggregate SHA-256 `ece72f1894405c3968eb0ae1969289749a8b97dd1034a56d129e78175602c440`;
- exact inline welcome, chunked welcome, three chunk frames, and done-frame bytes.

Rust and TypeScript independently encode those frames to the fixture bytes and decode/re-encode every vector byte-for-byte.

## Test-First Evidence

Initial red state:

- TypeScript focused run: 4 P1 tests failed, 0 P1 tests passed, 1 existing pattern-skipped test. Failures proved the new bootstrap variant, assembler, hash API, and canonical vectors did not exist.
- Rust focused attempt: compilation failed before test execution because the new protocol types/APIs and fixture did not exist; 0 tests executed.
- Inline direct-byte hardening follow-up: 3 P1 tests passed, 1 P1 test failed, 1 existing test was pattern-skipped. The failure proved a direct inline `[256]` byte was silently accepted before the constructor validation was added.
- Terminal-progress hardening follow-up: the focused TypeScript and Rust runs each had 3 P1 tests pass and 1 P1 test fail; the failures proved retired assemblers released storage but reset their terminal progress counters. The counters are now retained while storage is released.

Final focused verification:

1. `bun nx run @semio-tech/framework-replication:test --skip-nx-cache --testNamePattern='artifact bootstrap protocol'`
   - 4 passed, 0 failed, 1 existing test pattern-skipped; 1 test file passed.
2. `CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/artifact-bootstrap-target" bun nx run @semio-tech/framework-replication-rs:test --skip-nx-cache -- artifact_bootstrap`
   - 4 passed, 0 failed, 0 ignored, 238 filtered out.
3. `CARGO_TARGET_DIR=... bun nx run @semio-tech/framework-replication-rs:build --skip-nx-cache`
   - 1 Nx build target passed, 0 failed. Existing warnings remain outside this P1 change.
4. `jq empty` over both new fixture documents
   - 2 documents parsed, 0 failed.
5. `git diff --check` over the two implementation files
   - 2 files checked, 0 whitespace errors.

Malformed-path coverage in both languages includes unsupported version, wrong expected descriptor, wrong chunk descriptor, duplicate/out-of-order/missing chunks, total and per-chunk oversize, tail-before-baseline, expired deadline, bad pack hash, bad SPR hash, bad aggregate hash, cancel-at-N, monotonic progress, late cancellation after hashing, no completion on failure, zero retained staging, and successful fresh-transfer restart. TypeScript additionally covers a non-byte chunk element and a non-byte inline element.

## Wider Package Results And Existing Blockers

The focused P1 gates are green. The unfiltered package suites each expose one unrelated current-tree failure:

1. `bun nx run @semio-tech/framework-replication:test --skip-nx-cache`
   - 4 passed, 1 failed.
   - Failure: the pre-existing wire-fixture test opens the flat path `🧫️fixtures/🧫️wire/📦️client-hello.bin`, while the existing fixture is at the nested canonical path `🧫️fixtures/🧫️wire/📦️client-hello/💾️.bin`.
2. `CARGO_TARGET_DIR=... bun nx run @semio-tech/framework-replication-rs:test --skip-nx-cache`
   - 241 passed, 1 failed, 0 ignored, 0 measured, 0 filtered out.
   - Failure: `causal::tests::causal_add_fixture_has_exact_required_descriptor`; emitted `payloadSchema` is `🛂️schema.json`, while the existing fixture expects `../🛂️schema/🔣️.json`.

Neither unrelated failure was changed because its files are outside the P1 boundary.

Downstream hub, DB, native store, wasm store, and TypeScript store integration was intentionally not changed or claimed here. In particular, the DB `.spk` producer still uses the old private snapshot frames, and current store consumers still need the separate P2/P3 work to select, receive, and atomically install this artifact bootstrap. Downstream exhaustive Rust matches must add the new variants as that integration lands.

## Files Changed

- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🟦️.ts`
- `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/🧫️artifact-bootstrap/🔣️.json`
- `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/🧫️artifact-bootstrap/🔣️.schema.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-artifact-bootstrap-protocol.md`
