# Hub Artifact Authority Adapter — P2-A2

## Outcome

P2-A2 now has real hub-owned adapters for the live plugin manifest graph, the process document-codec registry, and the database payload CAS. The publication orchestrator accepts only a hub-derived `CheckpointCandidate`, recomputes every public SHA-256 identity, stages both immutable blobs, reads both back, verifies exact bytes plus hashes and lengths, replaces only the private database locator, and then crosses one explicit durable publication commit point.

The project-owned ports remain `TrustedArtifactCatalog`, `TrustedArtifactCodec`, `ImmutableArtifactBlobStore`, and `VerifiedCheckpointPublisher`; no plugin-host, codec-registry, or database-driver type leaks through them. P2-B directory event/projection design and P2-C WebSocket/bootstrap production are outside this packet.

## Trusted live catalog

`PluginHostTrustedArtifactCatalog::load` resolves the existing production surfaces directly:

- `semio_framework_plugin_host::PluginGraph::manifest(plugin_id)` for the loaded manifest;
- an explicit `LivePluginPackageBinding` for the distinct plugin id, package id, and package hash;
- `directory::os_store::document_codec(artifact_schema)` for the registered executable codec;
- the manifest's exact version, artifact-kind id and artifact schema;
- the registered codec's exact schema and nonzero pack-schema fingerprint.

The immutable catalog snapshot is a fixed `Box<[PluginHostArtifactCodec]>`. Load rejects empty/zero identities, duplicate plugin or package bindings, missing manifests, mismatched manifest ownership, empty artifact identities, absent codecs, zero/mismatched pack-schema fingerprints, duplicate exact identities, more than 4,096 packages, or more than 16,384 codecs. Each package boundary checks cancellation/deadline and reports bounded progress. Resolution requires exact equality of all seven descriptor-owned fields: plugin id, package id, version, package hash, artifact kind, artifact schema, and pack-schema hash.

The codec wrapper calls the registry's real `print_mirror` and `apply_ops_binary` functions asynchronously. It preserves executor yielding, checks the operation context around each call, and caps retained mirror/ops text at 64 MiB. Diagnostics are formatted directly into a UTF-8-safe 4,096-byte buffer; the adapter never first constructs an unbounded `Display::to_string()` value.

The language-neutral fixture registers a real global `ArtifactCodec`, calls its executable `print_mirror` and `apply_ops_binary` functions, and resolves it through a real `PluginGraph` plus `PackageRef`. It exercises independent mutation of every exact identity field. This is the strongest trustworthy catalog proof available before the audited headless loader exists; it is not presented as a loaded production-package proof.

### Trust limit

`PackageRef` currently contains only a package id and hash, and `PluginGraph` associates manifests with plugin ids but not with a compiled component handle or retained package bytes. `LivePluginPackageBinding::from_host` proves the explicit distinct plugin-to-package mapping; it does **not** prove that the supplied hash names the bytes loaded into an executable handle. The headless audit found no trustworthy production startup composition and only one of the expected 59 WASM packages. Heavy Block/DAG dev dependencies and their synthetic package hashes were therefore deliberately removed rather than mislabeled as loader proof. A headless registry/loader must next bind `{compiled handle, manifest plugin id, package id, hash of loaded package bytes}` as one trusted record.

## Immutable database blob adapter

`DbImmutableArtifactBlobStore` wraps the existing `DbBackend::payload()` / `PayloadStorage::{put,len,get}` API. Public integrity remains SHA-256 and byte length. The private locator is replaced after `put` with `db-payload/blake3/<actual database CAS hash>` and decoded strictly as 64 lowercase hexadecimal digits. A candidate's earlier `sha256/...` placeholder is never treated as a database locator and never reaches publication.

The adapter rejects an empty blob, a caller/hash/length mismatch, or a blob larger than the current cross-backend payload ceiling of 496 KiB before page allocation. After `put`, it verifies the backend-reported length. Read verifies locator shape, length, exact reconstructed byte count, and yields/checks the operation context between page fragments. Page leases always complete deterministic asynchronous cleanup even after cancellation or a prior close error. The fixed cleanup budget is derived from `DB_IO_OPERATION_PAGES` plus the owner shell and result handback (`64 + 2` steps), rather than an unrelated numeric maximum.

The memory-backend runtime law proves round-trip bytes and SHA-256 integrity while the actual locator is BLAKE3-addressed and differs from the candidate placeholder.

## Failure-atomic publication and commit point

`CheckpointPublicationOrchestrator::publish_candidate` performs these ordered phases:

1. recompute pack/SPR SHA-256 and lengths, aggregate SHA-256, and checkpoint id;
2. require exact equality with the candidate metadata;
3. stage pack, then stage SPR;
4. read pack, then read SPR;
5. require byte-for-byte equality and independently revalidate each staged hash and length;
6. replace only `pack.storage_key` and `spr.storage_key` with actual backend locators;
7. perform the last fallible cancellation/deadline/progress boundary;
8. call `VerifiedCheckpointPublisher::publish` exactly once.

The publisher contract requires every possible error to occur before durable commit and requires `Ok(())` after commit. After a successful publish the orchestrator performs only best-effort, nonfallible final progress reporting and returns the committed checkpoint. A cancel-at-commit law makes the fake publisher set cancellation immediately after committing and proves the caller still receives success.

Pack-stage, SPR-stage, pack-read, SPR-read, corrupted-pack, and corrupted-SPR laws prove zero publisher invocation and zero committed checkpoint. A publisher error proves no committed checkpoint. Candidate tampering is rejected before the first store operation. `checkpoint_id` remains unchanged when only private locators are replaced because storage keys are deliberately excluded from its canonical encoding.

The concrete `HubVerifiedCheckpointPublisher` supplied by the concurrent P2-B packet crosses the System-only `DirectoryService::publish_verified_artifact_checkpoint` seam. Its `DirectoryError` mapping now uses the same 4,096-byte bounded formatter, and it performs no fallible context check after the durable service call.

Immutable CAS writes cannot safely be rolled back: a successful pack stage can remain unreferenced if SPR staging/readback or publication later fails, and both pack and SPR can remain unreferenced after a later publication failure. Deleting them here could remove content shared by another reference. Reference tracing plus retention/sweep owns that later cleanup.

## Async and ownership

Catalog lookup, codec validation/application, blob stage/read, and publication are native async calls. There is no sync-to-async poll bridge, `block_on`, thread parking, or busy spin. Every pending future returns control to the caller's executor. Expensive loops have fixed ceilings and operation-context cancellation/deadline/progress boundaries. Database read cleanup deliberately finishes after cancellation so page ownership is never abandoned. Only a fully verified candidate transfers into publication, and only committed success transfers the final checkpoint back to the caller.

## Language-neutral and independent oracle

`🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️authority-adapter/🔣️.json` is consumed by both Rust and the permanent `os-hub-ts` Vitest suite. The Rust laws use the vector for exact distinct plugin/package identity and failure stages. The independent Node `crypto.createHash("sha256")` implementation recomputes the pack, SPR, and concatenated aggregate digests from the same byte arrays and verifies zero prepublication publisher calls.

## TDD and validation evidence

The first Rust compile after the async port was red because the old synchronous doubles and tests no longer implemented the evolved async ports. Subsequent red compiles found a worker-pool ownership mismatch and a shadowed test control. Those source defects were corrected.

Permanent independent oracle:

```text
bun nx run os-hub-ts:test --skip-nx-cache --verbose
```

Result: one test file passed; 3 tests passed and the opt-in live E2E test was skipped. The new Node-crypto authority fixture test passed.

The first focused Rust production-codec attempt was:

```text
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-adapter-target" bun nx run os-hub:test-quick --skip-nx-cache --verbose -- authority_adapter
```

It was not a test result: the repository runner killed `cargo nextest list` at its fixed 1,200,000 ms budget while several concurrent jobs compiled `semio_s_plugin_stdio`; no authority test ran. The filter also did not name a test directly. This failed attempt is retained here to avoid overclaiming it as green.

Production build:

```text
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-authority-adapter-target" bun nx run os-hub:build --skip-nx-cache --verbose
```

Result: green release build in 6m03s, with pre-existing warnings only. This compiled the catalog, database store, orchestrator, and concrete directory publisher production paths.

Focused Rust runtime gates, all driven through Bun and Nx with the same isolated ticket target:

```text
bun nx run os-hub:test-quick --skip-nx-cache --verbose -- plugin_host_catalog
bun nx run os-hub:test-quick --skip-nx-cache --verbose -- publication_orchestrator
RUST_MIN_STACK=16777216 bun nx run os-hub:test-quick --skip-nx-cache --verbose -- database_blob_adapter
bun nx run os-hub:test-quick --skip-nx-cache --verbose -- authority_diagnostics
bun nx run os-hub:test-quick --skip-nx-cache --verbose -- canonical_authority_contract
```

Results: catalog 1/1 passed; publication 2/2 passed; database 1/1 passed; diagnostics 1/1 passed; canonical P2-A1 contract 4/4 passed. The database law's first harness invocation aborted before assertions because the default test thread stack overflowed; the repository's established 16 MiB test-stack setting reached the test. Its first assertion run then exposed a test-only `MemoryStorage::close` generation mismatch (`StaleGeneration expected 1 actual 0`); removing that invalid explicit close and using the backend's established drop/retirement path made the runtime law green.

`git diff --check` passed for the owned authority adapter, fixture, hub manifest, permanent TypeScript oracle, and the bounded publisher mapping. Source inspection found no `block_on`, thread park, busy-poll bridge, duplicate `CatalogResolved` progress report, or authority-owned directory/retention/WebSocket implementation.

The required `repo://goals` read was attempted before implementation, but the repository MCP connection closed during initialization. No ticket, goal, or Git state was changed.

## Exact remaining production seam

The adapters are callable and the directory publisher is concrete, but the headless accepted-operation/checkpoint scheduler still must compose:

`verified loaded-package record → PluginHostTrustedArtifactCatalog → ValidatingCanonicalArtifactAuthority → DbImmutableArtifactBlobStore → HubVerifiedCheckpointPublisher`.

That scheduler must inject the real compiled-handle/package-byte association described above and drive the nonblocking codec future on its executor. P2-B owns checkpoint/retention directory events and projections; P2-C owns production WebSocket/bootstrap delivery. This report does not claim either the composed scheduler or package-byte attestation is already wired.

## Files

- `🌎️hub/🗿️artifact-authority/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️authority-adapter/🔣️.json`
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🌎️hub/📇️directory/🦀️.rs` (bounded publisher diagnostic mapping only; publication event/projection changes are P2-B-owned)
