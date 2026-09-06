# Authenticated Checkpoint Publication Command Design

## Existing authority seam

The Hub already owns the trusted publication pipeline: `ValidatingCanonicalArtifactAuthority` validates a descriptor-selected codec and canonical Pack/SPR pair, `CheckpointPublicationOrchestrator` reserves and stages the exact CAS ownership plan and verifies readback, and `DirectoryService::publish_reserved_artifact_checkpoint` atomically consumes the reservation with the storage-key-free `artifact.checkpoint-published` event. The authenticated blob `PUT` only lands immutable bytes and is not publication.

No existing public document command enters this pipeline. The new command therefore reuses these owners rather than extending `DirectoryCommand` with private locators or adding a backend selector.

## Public contract

`POST /spaces/{spaceId}/documents/{documentId}/checkpoint-publications` accepts exact canonical `semio.hub.checkpoint-publication-command/v1` JSON containing:

- a 32-lower-hex correlation id;
- the expected descriptor SHA-256;
- the expected DB document frontier;
- a required `none | active` current-checkpoint expectation;
- an expected artifact frontier; and
- Pack and SPR SHA-256/byte-length references already present in the authenticated Hub payload store.

The request contains no scope, descriptor, checkpoint id, aggregate hash, storage locator, path, backend, actor, publication time, or event body. All request frontier values are comparison inputs only. They never become authority by validating each other.

## Actor-owned identity and writer sequencing

The DB artifact actor owns a synchronous `CheckpointPublicationSnapshot` query that returns its exact generation, committed frontier, and retained tip mutation id. Replay updates the retained tip for every committed command, including commands below the materialized snapshot floor; a non-genesis reopen therefore cannot project an empty head id.

The Hub acquires live session, membership, and per-document writer gates, reads that actor snapshot, and requires the request expectations to equal it before reading blobs. It server-constructs the artifact frontier from the actor snapshot. Ordinary websocket writes acquire the same per-document gate immediately before actor submission. Every production writer, including the GIS approval committer, must use this one sequencing contract before it can advance the actor.

Expensive blob reads, codec validation, CAS reservation, staging, and readback run after releasing admission locks under one deadline/cancellation context. A request-specific publisher reacquires live session and membership gates, then the same document writer gate, reads the actor snapshot again, and compares the exact generation/frontier/tip, durable descriptor/digest, and required active-checkpoint expectation while holding the gate through the atomic directory publication. Revocation monitoring cancels the operation without holding an administrative gate across expensive work.

## Durable retry identity

Before expensive work, the Hub durably claims `(authenticated author id, correlation id, SHA-256 of the exact canonical command)` in the directory. A substituted command under the same author/correlation conflicts. The successful checkpoint publication and the claim completion carrying its exact checkpoint id must share the directory's final atomic append transaction. A completed retry reads that exact retained checkpoint and returns the same public receipt after repeating current authorization. Pending requests never execute concurrently; failure before the atomic append releases the owned claim. This closes the lost-response window without putting private locators in the public receipt.

## Acceptance

1. Rust, TypeScript, and JSON Schema share the closed command grammar. AJV plus Node/WebCrypto SHA-256 independently reject unknown fields, unsafe integers, uppercase/zero hashes, private locators, route scope injection, and invalid `none | active` expectations.
2. Actor laws prove exact tip identity before and after reopen, a queued write invalidates the first snapshot, and cancellation publishes nothing.
3. Native HTTP laws prove Author-only admission, exact descriptor/scope/frontier/current fences, bounded pair reads, final revocation and queued-write rejection, durable identical retry, and digest-substitution conflict.
4. A process law publishes a genuine small GIS pair through the public blob and publication routes, then reads `semio://workspace/scopes/{space}/{document}/checkpoint` from a credential-FD MCP child and compares exact bytes and hashes.

Source, native, and process receipts are reported separately; no source-only gate is a runtime claim.
