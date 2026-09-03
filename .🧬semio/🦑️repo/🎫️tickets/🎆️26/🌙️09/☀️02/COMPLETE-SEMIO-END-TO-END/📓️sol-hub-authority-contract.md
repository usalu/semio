# Hub Artifact Authority Contract — P2-A1

## Outcome

P2-A1 is implemented as a bounded foundation packet. The shared OS directory contract now owns structural document scope and artifact checkpoint vocabulary in Rust, TypeScript, and JSON Schema. The hub consumes that shared scope through its service and SQLite, PostgreSQL, and Neo4j directory implementations. A hub-local authority port now validates a fully pinned trusted codec identity, validates and materializes an ordered operation range under hard resource/cancellation/deadline controls, derives all content identities, and transfers ownership only through a successful `CheckpointCandidate`.

This packet does not publish a checkpoint, mutate directory retention, send `ArtifactBootstrap`, or wire a real plugin host/catalog/store. Those remain separate packets.

## Shared structural vocabulary

The shared directory schema adds:

- `DocumentScope { space_id, document_id }`
- `ArtifactHash([u8; 32])` and `CheckpointId`
- `ArtifactFrontier`
- `ArtifactBlobRef`
- `ArtifactCheckpoint`
- `ArtifactRetention`
- `descriptor_digest_v1`

The Rust value decoder and JSON Schema reject fixed hashes that are not exactly 32 bytes, byte values outside `0..=255`, zero hashes where identity is required, zero byte lengths where a blob is required, and integers beyond JSON's exact safe range. The neutral fixture covers empty scope components, non-ASCII text, colon-containing components, a short hash, a byte value of 256, an all-zero hash, and an unsafe integer. TypeScript declarations and exports match the Rust names and shapes.

### Descriptor digest v1

`descriptor_digest_v1` is SHA-256 from the existing `semio_framework_hash::Sha256`. It never serializes JSON. Its preimage is:

1. literal UTF-8 domain `semio.document-descriptor.digest.v1\0`;
2. every immutable descriptor leaf in declaration order;
3. each leaf framed as `u64_be(payload byte length) || payload`;
4. text encoded as UTF-8;
5. package, pack-schema, and bootstrap-snapshot hashes parsed from canonical lowercase 64-digit hexadecimal into 32-byte payloads;
6. `bootstrapVersion` encoded as fixed-width big-endian `u32`;
7. the three nested bootstrap-frontier integers encoded as fixed-width big-endian `u64`.

The exact field order is `spaceId`, `documentId`, `artifactKind`, `artifactSchema`, `owner.pluginId`, `owner.packageId`, `owner.version`, `owner.packageHash`, `packSchemaHash`, `bootstrapVersion`, `bootstrapFrontier.headSeq`, `bootstrapFrontier.commitSeq`, `bootstrapFrontier.epoch`, `bootstrapSnapshotHash`.

For the colon/non-ASCII fixture the exact digest is `03504036d9aecf653a38d807d2a852f18d13bcf612cb9d7e612c46540c430cac`. Rust, Web Crypto, and independent Node `crypto.createHash("sha256")` agree on the bytes.

## Scope and backend migration

The hub-local `SpaceDocumentId` duplicate is removed. `DocumentScope` is re-exported from the shared directory package and flows through the directory port, records, service, and all three database backends. Share-token and descriptor semantics remain exact composite `(space_id, document_id)` semantics.

The hub's former ambiguous `format!("{space}:{document}")` flat key is replaced by named `document_scope_key_v1`. Its encoding is:

`v1:<space UTF-8 byte length>:<document UTF-8 byte length>:<space bytes><document bytes>`

There is no legacy decoder or fallback. The vectors prove that `(a:b, c)` and `(a, b:c)` do not alias, non-ASCII lengths are byte lengths, and equal document IDs in `space-a` and `space-b` remain distinct. Database artifact IDs and fanout keys derive from this single encoding.

## Canonical authority port

The hub-local module exposes only project-owned types and ports:

- `TrustedArtifactCatalog`
- `TrustedArtifactCodec`
- `CanonicalArtifactAuthority`
- `ValidatingCanonicalArtifactAuthority`
- `TrustedArtifactIdentity`
- `AcceptedArtifactOperation`
- `ArtifactPair`
- `CheckpointRequest`
- `CheckpointCandidate`
- `AuthorityLimits`
- bounded/cancellable `OperationContext`, progress stages, validation stages, and errors

Before either validation or materialization the authority requires the catalog to resolve an identity exactly equal to the descriptor's owner plugin/package/version/package hash, artifact kind/schema, and pack-schema hash. It validates the input pair through that trusted codec, applies each accepted operation one at a time in strict contiguous frontier order, checks the pair budget after every application, validates the output pair, and derives pack, SPR, aggregate, descriptor, and checkpoint hashes itself.

The authority has immutable ceilings independent of caller configuration:

- 16,384 operations
- 64 MiB total accepted-operation bytes
- 64 MiB for the current pack/SPR pair

Caller limits must be nonzero and cannot exceed those ceilings. Input and output pair failures preserve distinct `ArtifactValidationStage::Input` and `ArtifactValidationStage::Output` classification. Deadline and cancellation checks bracket catalog lookup, validation, every operation, derivation, and the success handoff. Progress is monotonic and exposes the resolved/validated/applying/output-validated/derived stages.

The success return owns its pack and SPR bytes. No publication or store capability exists in this API, so every failure returns an error before there is any publishable candidate or ownership transfer. The deterministic codec/catalog/control implementation is compiled only under `#[cfg(test)]`; it is evidence for the port, not claimed runtime wiring.

### Checkpoint identity v1

The checkpoint ID is SHA-256 over domain `semio.hub.artifact-checkpoint.v1\0` followed by individually `u64`-length-prefixed fields in this order:

`scope.spaceId`, `scope.documentId`, `parentCheckpointId`, `descriptorDigestV1`, `baselineFrontier.documentId`, `baselineFrontier.headEditOrdinal`, `baselineFrontier.headEditId`, `baselineFrontier.lastCommitSeq`, `baselineFrontier.chainHash`, `pack.sha256`, `pack.byteLength`, `spr.sha256`, `spr.byteLength`, `aggregateSha256`.

Integer field payloads are fixed-width unsigned big-endian. Storage keys and server time are deliberately excluded from identity. The language-neutral vector yields:

- pack SHA-256 `3fdba35f04dc8c462986c992bcf875546257113072a909c162f7e470e581e278`
- SPR SHA-256 `ec51adbbbb7bc6c781fa2bf8c8ddf1c0edbdcb7a1376509bc2cbf6774ee591d8`
- aggregate SHA-256 `e29f9287b42b0642d71995e2d00cc27312b9a568f1d0a2f87fbb6e369632ad35`
- checkpoint ID `480a3b0f144e38a0527e4f6341af445098284a18a53017ba5e9a8c1bcdd60d33`

Independent Node crypto reproduced the fake codec output pair and all four SHA-256 identities from the fixture's binary checkpoint preimage.

## Explicit boundary

P2-A1 bounds the expensive operation list, operation payload sum, and materialized pair. It does not introduce maximum lengths for descriptor, scope, or frontier strings; consequently the public digest/checkpoint encoders can allocate in proportion to those already-owned strings. This report does not claim all caller-owned text is bounded. Defining shared textual maxima and enforcing them at descriptor ingestion is a separate schema-policy decision.

No external plugin/store type leaks through the hub API, and no real plugin host or blob store has been invented. No checkpoint publication event, retention event, backend projection, bootstrap WebSocket sender, or runtime plugin-host adapter was added.

## TDD evidence

The first focused TypeScript run failed because the descriptor digest encoding export did not exist. The first authority Rust run failed because the authority module did not exist. The Rust fixture harness subsequently caught that the generic `u8` decoder narrowed JSON `256`; the final fixed-hash decoder performs explicit exact-range validation. The first AJV run caught duplicate `$id` registration caused by validating multiple target refs in one AJV instance; the final harness creates one instance per target.

Final commands and observed results:

```text
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-contract-descriptor-target" bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/📜️script.ts' test
```

Result: 3 passed, 0 failed, 12 filtered; warnings only.

```text
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-contract-descriptor-target" bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/📜️script.ts' check
```

Result: all-feature hub source check passed across SQLite, PostgreSQL, and Neo4j; warnings only.

```text
bun nx run @semio-tech/framework-os:test-quick --skip-nx-cache --testNamePattern='document descriptor|document.announced|descriptor digest'
```

Result: 3 passed, 221 skipped; one test file passed and two were skipped.

```text
SEMIO_OS_MCP_BIN=/usr/bin/true bun './📜️script.ts' test quick --testNamePattern='structural artifact authority'
```

Run from `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript`. Result: 1 passed, 41 skipped; one test file passed and four were skipped. `/usr/bin/true` bypassed only the script's top-level executable-existence gate for this filtered, AJV-only test; the test starts no MCP binary. The full Nx producer path was attempted first but remained blocked on concurrent shared Cargo package/build locks and was stopped.

```text
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-contract-target" bun nx run os-hub:test-quick --skip-nx-cache -- canonical_authority_contract
```

Result: 4 passed, 42 skipped, all-feature hub build.

```text
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-contract-target" bun nx run os-hub:test-quick --skip-nx-cache -- document_scope_key_v1_is
```

Result: 1 passed, 45 skipped, all-feature hub build.

```text
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-contract-target" bun nx run os-hub:test-quick --skip-nx-cache -- share_token_lifecycle_and_scope
```

Result: 1 passed, 45 skipped, all-feature hub build.

```text
RUST_MIN_STACK=16777216 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-contract-target" bun nx run os-hub:test-quick --skip-nx-cache -- document_open_rejects_missing_or_conflicting_descriptor_before_db_creation
```

Result: 1 passed, 45 skipped, all-feature hub build.

```text
bun nx run os-hub-ts:test-quick --skip-nx-cache
```

Result: 2 passed, 1 skipped; the live E2E is intentionally skipped without `HUB_E2E`.

```text
jq empty '🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json' '🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️artifact-authority.json' '🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️canonical-authority/🔣️.json' '🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🧬️hub-boundaries/🔣️.json'
```

Result: all four documents parsed.

```text
bun -e 'import assert from "node:assert/strict"; import {createHash} from "node:crypto"; import {readFileSync} from "node:fs"; const a=JSON.parse(readFileSync("🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️canonical-authority/🔣️.json","utf8")); const d=JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️artifact-authority.json","utf8")); const sha=x=>createHash("sha256").update(Buffer.from(x)).digest(); const bytes=x=>Buffer.from(x); let n=Number(Buffer.from(a.input.pack).toString()); let spr=Buffer.from(a.input.spr); for(const op of a.input.orderedOperations){n+=Number(Buffer.from(op).toString()); const size=Buffer.alloc(4); size.writeUInt32BE(op.length); spr=Buffer.concat([spr,size,Buffer.from(op)]);} const pack=Buffer.from(String(n)); assert.deepEqual(pack,bytes(a.outputPack)); assert.deepEqual(spr,bytes(a.outputSpr)); assert.deepEqual(sha(pack),bytes(a.packSha256)); assert.deepEqual(sha(spr),bytes(a.sprSha256)); assert.deepEqual(sha(Buffer.concat([pack,spr])),bytes(a.aggregateSha256)); assert.deepEqual(sha(Buffer.from(a.checkpointIdEncodingHexV1,"hex")),bytes(a.checkpointIdV1)); assert.deepEqual(sha(Buffer.from(d.descriptorEncodingHex,"hex")),bytes(d.descriptorDigestV1)); console.log("authority Node crypto oracle: descriptor, output pair, and 4 authority SHA-256 vectors passed")'
```

Result: the independent Node oracle reproduced the descriptor digest, fake codec output pair, pack hash, SPR hash, aggregate hash, and checkpoint ID. The TypeScript suite also compares Web Crypto directly with the same descriptor encoding.

Final source checks found no remaining `SpaceDocumentId`, old `scope_key`, or colon-interpolated scope key in `🌎️hub`. `git diff --check` passed for the owned tracked files.

### Known harness blocker

The broad `space_scoped_documents_are_isolated` integration test is not recorded as passing. Without a larger main test stack it aborts with stack overflow; one larger-stack attempt reached `Welcome` and then failed transiently, and a retry overflowed on `semio-pool-worker-1`, whose worker stack is not controlled by `RUST_MIN_STACK`. The focused collision vector, share lifecycle, descriptor path, and all three backend source checks are green. This is an existing pool-worker/harness limitation, not evidence of a passing broad integration scenario.

The repo goal/ticket MCP advertised by the repository instructions was not present in the available MCP resources/tools. Therefore no goal or ticket state operation was attempted; this report was written directly into the already-open umbrella ticket requested by the coordinator.

## Owned files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️artifact-authority.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️hygiene.test.ts`
- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/📦️packages/🦀️rust/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🧬️hub-boundaries/🔣️.json`
- `🌎️hub/🗿️artifact-authority/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️canonical-authority/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/shim/Cargo.toml`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/src/lib.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-hub-authority-contract.md`

## Next seams

P2-A2 should implement the real trusted-catalog adapter that resolves the exact descriptor owner package hash and artifact kind/schema/pack-schema hash through the plugin host, plus immutable blob staging, read-back verification, and the publication orchestrator consuming `CheckpointCandidate`. Plugin-host and store types must remain behind hub-owned ports.

P2-B should introduce explicit checkpoint-published and retention-advanced directory events and atomic projections in SQLite, PostgreSQL, and Neo4j, enforcing one lineage head/active checkpoint and retained-floor invariants in the same transactional fold. P2-C can then send committed artifact bootstrap payloads over the WebSocket boundary.
