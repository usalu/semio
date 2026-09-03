# Hub Snapshot And Headless Workspace Cold-Path Audit

**Scope.** Read-only architecture audit for ticket `26/09/02/COMPLETE-SEMIO-END-TO-END`, conducted 2026-09-03. It traces persisted snapshots, hub sync, native/browser store clients, authenticated directory discovery, MCP resources, and inference. The audit read the repository and applicable OS instructions. It did not modify production files, execute a build, or claim runtime success: shared DB/Cargo work is contended and presently broken elsewhere, so a broad build would not provide attributable evidence.

## Decision

Do **not** make OS clients decode the database snapshot. The byte stream called `Bootstrap::Snapshot` today is an internal DB `.spk` generation, not an artifact pack plus SPR history. For normal OS schemas it also lacks semantically materialized artifact state. The public cold/reconnect contract must instead carry a distinct, versioned, authenticated **artifact bootstrap** containing the canonical `(pack, spr)` pair, its schema identity, its frontier, and integrity hashes. DB `.spk` remains a server-private compaction/recovery acceleration format.

Until that boundary exists, a reconnect after the retained command floor cannot be correct: all deployed sync clients intentionally discard the snapshot frames, then accept the server frontier as live.

## Verified Current Path

| Boundary | Actual contract and evidence | Result |
| --- | --- | --- |
| DB snapshot writer | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️.rs` writes a `.spk`: content-addressed `KIND_CHUNK` state pages followed by a `KIND_SNAPSHOT` descriptor. The descriptor includes document, generation, head/commit/epoch, protocol/VCS metadata, root hashes, and new page hashes. `🗄️storage/🦀️.rs` explicitly defines `SnapshotStorage` as storage for those complete `.spk` generation bytes. | Internal DB materialized-state serialization; **not** an OS artifact file. |
| DB bootstrap decision | `🛢️db/🔄️sync/🦀️.rs` has `BootstrapPlan::Snapshot { generation, pages: DbIoPages, pack_hash }`; it reads the stored generation and hashes the DB pages. `DatabaseSyncHello` lowers the production result to `Bootstrap::Snapshot { pack_hash, inline: None }`, then `DatabaseSyncHelloFollowUp` emits fixed chunks. `lower_bootstrap_plan` can make an inline frame, but it is test-only. | Production hub path is chunked DB pages only; “inline and chunked in production” is not true today. |
| Replication wire | `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` defines `Bootstrap::Snapshot { pack_hash, inline }`, `SnapshotChunk`, and `SnapshotDone`; each chunk has a 4 KiB backing limit. `🧰️framework/🔨️modules/📡️replication/🟦️.ts` mirrors this byte-for-byte. The name `pack_hash` is therefore misleading: it hashes DB snapshot pages, not the OS `.pack` artifact. | A protocol naming/design defect, not evidence of artifact-pack transfer. |
| DB semantic state | `🛢️db/📄️artifact/🦀️.rs` materializes `DocumentState` as a path map. Its non-`DB_PATHMAP_SCHEMA` diff path preserves/relays foreign envelopes but yields no touched semantic state. The OS store emits application-schema operation payloads, not DB path-map payloads. | Even a client that decoded `.spk` could not reconstruct a generic OS artifact from it. Retained WAL replay can still work while the typed client has its own codec. |
| Hub WS | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` receives `ClientFrame::Hello` and calls `db.hello`; it forwards DB welcome/follow-up frames. `schema` is ignored and only the first nonzero `pack_schema_hash` is pinned in the in-memory `schema_hashes` map. | The only current schema pin disappears on hub restart and cannot be used for cold open. |
| Native Rust and wasm Rust clients | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`, in both `ArtifactActor::on_hub_frame` and `WasmActor::on_binary`, explicitly treats `Bootstrap::Snapshot`, `SnapshotChunk`, and `SnapshotDone` as accepted but ignored because no DB snapshot decoder exists. | Snapshot bootstrap is not applied. |
| Browser production client | `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` says the same in `handleHubFrame`: welcome updates resume/frontier; snapshot frames are ignored. Its local snapshot APIs encode canonical artifact pack+SPR, a different format. | Browser has reconnect/backoff/outbox mechanics but no cold snapshot recovery. |
| Local artifact persistence | `🏪️store/🔄️sync/🦀️.rs`’s `FolderEventLogStorage` persists an indivisible `(pack, spr)` pair; pack+SPR are documented as authoritative. `BackboneMessage::Snapshot` and `ArtifactEvent::SnapshotReplaced` refer to this artifact-level representation, not DB `.spk`. | This is the reusable client-side canonical replacement seam. |
| MCP workspace | `🌉️mcp/🏠️workspace/🦀️.rs`: a hub workspace’s `workspace_artifact_ids()` returns only `open_probes`; `read_artifact_bytes()` returns `None` for an unopened hub artifact. `ensure_probe_artifact()` creates only `os.agent.probe/v1`. | Hub MCP cannot enumerate or read a cold real artifact. |
| Inference | `🌉️mcp/💡️inference/🦀️.rs` correctly reads declared inference metadata, but `inference_get` always returns retryable `channel.not-wired`; no artifact-infer command exists in the workspace channel. The reusable execution router exists in `🔌️plugin/🖥️host/🦀️.rs` and is assembled by `🏃️run/🦀️.rs`, not by MCP. | Discovery is honest; execution is intentionally unimplemented. |

### Reachability Matters

`🛢️db/📄️artifact/🦀️.rs` has a real internal `ArtifactEngine::snapshot_now`, but the public `ArtifactHandle::snapshot_now` in `🛢️db/⚙️engine/🦀️.rs` returns `DbError::Unimplemented`. The hub neither invokes compaction/snapshot publication nor advances a retention floor. Consequently the broken snapshot *consumer* path is presently dormant in the ordinary hub product; it becomes a correctness fault as soon as compaction/retention is enabled or a snapshot is injected.

The current hub also discards `broadcast::RecvError::Lagged(_)` in its document fan-out loop (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`). A live, lagged connection therefore has the same resynchronization requirement as a reconnect, but no forced rebootstrap today.

## Exact Byte Semantics

```text
DB internal recovery path (current wire):
DocumentState / path-map roots
  -> .spk = KIND_CHUNK pages + KIND_SNAPSHOT descriptor
  -> DbIoPages hash misleadingly named `pack_hash`
  -> Bootstrap::Snapshot + SnapshotChunk* + SnapshotDone
  -> ignored by Rust native, Rust wasm, and TS browser stores

OS artifact path (local folder/current in-memory store):
typed ArtifactStore state and history
  -> canonical `.pack` + `.spr` pair
  -> ArtifactEvent::SnapshotReplaced / folder event
  -> typed app/plugin codec can restore and expose the artifact
```

`.spk` is a pack-*file* in the DB storage sense, but it is neither an OS `.pack` artifact nor an SPR history. No existing parser converts it into the latter, and such a converter would be architecturally wrong: it would expose DB storage internals to every OS client while still being unable to restore foreign application-schema operations.

## Directory Discovery: What Is Reusable And What Is Missing

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` already provides the correct reusable seams:

- `OperationContext` carries cancellation and deadline; `DirectoryTransport` is transport-injectable; `DirectoryClient::spaces`, `space`, and `me` carry the configured bearer token.
- `DirectoryClient::space(ctx, space_id)` requests authenticated `GET /directory/spaces/{id}` and returns `SpaceDetail { space, members, documents, invites }`.
- Native `NativeDirectoryTransport<TokioHostRuntime>` already uses the OS HTTP/compute pools and implements deadline/cancellation. It is available behind the existing kernel `ureq` feature; MCP presently enables only `sync` in `🌉️mcp/📦️packages/🦀️rust/Cargo.toml`. Enabling that existing feature is not a new runtime dependency.

This reveals document IDs for an authorized space, but `DocumentView` in `📇️directory/🧬️schema/🦀️.rs` contains only `id`, `head_seq`, `commit_seq`, and `epoch`. It has no artifact schema, owner plugin, pack-schema hash, bootstrap version/hash, or canonical snapshot locator. The hub derives it from its DB catalog (`documents_for_space`), which makes the list an incidental projection of opened DB handles rather than a durable document registry.

Therefore: an authenticated `DirectoryClient` can discover a document **ID**, but cannot discover which codec/plugin/schema may open it. `ClientFrame::Hello.schema` is not a safe substitute: `handle_ws` presently destructures it away, and client-provided claims must never become a mutable authorization bypass.

## Design Gaps Versus Shared Build Blocker

### Proven design gaps (independent of compilation)

1. Public `Bootstrap::Snapshot` has the wrong ownership and bytes for artifact restoration.
2. All three actual store consumers drop the bootstrap frames.
3. DB materialization is path-map-specific; ordinary OS operation schemas do not produce a canonical artifact state there.
4. The hub does not publish/retire DB snapshots and its public snapshot handle is unimplemented.
5. Schema identity is only a volatile hub map, while directory documents lack descriptor metadata.
6. Hub MCP can neither enumerate nor read unopened remote artifacts, and cannot infer.
7. Lagged fan-out does not force a resume/rebootstrap.
8. MCP constructs `AgentPrincipal` from CLI `--principal/--scopes` even for a hub workspace (`🌉️mcp/🦀️.rs`); this is not an authority derived from the hub bearer session. It must not be used to grant document access.

### Separate shared-build blocker

The shared DB compilation break and concurrent Cargo work prevent a meaningful end-to-end runtime confirmation now. This audit did not run a broad build and does not attribute any finding to that failure. Fixing compilation is necessary to run the probes below, but it does not resolve any of the eight source-proven design gaps.

## Dependency-Ordered Implementation Packets

Each packet is deliberately file-bounded. It should land with its tests and runtime probe before a later packet depends on it. The terms “publish”, “announce”, and “rebuild projection” below are event-sourced operations; do not add a CRUD document table/API.

### P0 — Durable, Authorized Document Descriptor

**Goal:** make a space’s document identity and open contract a durable, queryable projection before any snapshot transfer.

**Owned files/boundaries:**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs` and its TypeScript schema twin: replace the bookkeeping-only document projection with a schema-first `DocumentDescriptor` carrying document ID, artifact kind/schema, owning plugin/package identity/version/hash, immutable pack-schema hash, bootstrap-format version, and authoritative frontier/snapshot hash. Keep head/commit/epoch as sync fields, not the sole identity.
- Directory domain/event/read-model files under `📇️directory/**`: add a `document-announced` domain event and projection. An explicit, authorized schema migration event may exist later; do not silently change an existing document’s codec.
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`: authenticate and authorize first, then announce/validate the descriptor during document creation/open. Persist it through the directory port rather than `schema_hashes`; delete the volatile map once the durable invariant replaces it. Reject a conflicting descriptor before accepting mutations.
- `📇️directory/🔌️client/🦀️.rs`: decode and expose the descriptor from `SpaceDetail`.

**Rules:** derive caller identity and role from the hub session, retain per-space isolation, bind descriptor schema/hash to the first authorized document-creation event, and refuse zero/unknown identity for documents intended to be cold-opened. Do not trust a reconnecting client to redefine it. Membership changes/revocation must be checked at every protected descriptor/snapshot read.

**Tests and probe:**

1. Add a language-neutral JSON descriptor fixture (valid descriptor, conflicting schema hash, cross-space same document string, revoked reader). Rust and TypeScript must both parse the same fixture and preserve canonical bytes/fields.
2. Validate the published descriptor JSON with the already-declared third-party `ajv` dependency in `🌉️mcp/📦️packages/🟦️typescript`; Rust must compare its own emitted JSON to the same fixture. This supplies the required independent-schema oracle without a new dependency.
3. Hub integration: author A announces `space-a/doc`; reader B lists it; non-member C gets no descriptor or bytes; a conflicting later Hello is rejected; hub restart still returns the original descriptor.
4. Runtime probe through the registered launch configuration: create two spaces with same `document_id`, connect two authenticated users, log only `[DEBUG]` descriptor ID/schema/hash/frontier, restart hub, and verify no cross-space descriptor leaks.

**Exit criterion:** Directory client can return authorized document IDs *and* a durable, validated schema/open descriptor after hub restart.

### P1 — Artifact Bootstrap Protocol, Separate From DB `.spk`

**Goal:** replace ambiguous `Bootstrap::Snapshot` as the client contract with an artifact-owned transfer unit.

**Owned files/boundaries:**

- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` and `🧰️framework/🔨️modules/📡️replication/🟦️.ts`: introduce a new wire-versioned `ArtifactBootstrap` variant. Do not overload `pack_hash` or reinterpret existing snapshot bytes.
- Shared fixtures beneath `🧰️framework/🔨️modules/📡️replication/**/🧪️fixtures`: add canonical binary vectors for inline and chunked artifact bootstrap.
- Keep DB `BootstrapPlan` and `.spk` types internal to `🛢️db/**`; remove their use from the public OS-client welcome path once P2 supplies artifact bootstrap.

**Required artifact-bootstrap fields:** document descriptor hash; artifact schema/kind and pack-schema hash; protocol/format version; baseline frontier; immutable pack hash; immutable SPR hash; total lengths; chunk count; ordered chunk index; a single aggregate content hash; and the required tail frontier. The pair is valid only when both pack and SPR are present and all bounds/hashes/sequence numbers match. Chunks need explicit total-byte limits, an assembler budget, cancellation token, progress (`received/total`), deadline, and no partial commit.

**Tests and probe:**

1. Rust and TypeScript encode/decode the same inline and multi-chunk vectors byte-for-byte, including unknown-version, wrong descriptor, duplicate/out-of-order/missing chunk, oversize, bad pack hash, bad SPR hash, and bad aggregate hash failures.
2. A language-neutral fixture declares expected canonical hashes and lengths; the TS descriptor envelope is additionally AJV-validated. Keep the current Rust/TS wire fixture mechanism; do not duplicate codecs.
3. Instrument a loopback transfer to cancel at chunk N. Assert progress is monotonic, memory returns to the bounded assembler budget, no `SnapshotReplaced` event is emitted, and reconnect starts a fresh transfer.

**Exit criterion:** There is one unambiguous public artifact transfer whose bytes a store can atomically replace, without importing DB snapshot parsing.

### P2 — Canonical Artifact Snapshot Authority And Retention

**Goal:** give the hub an authoritative source for the canonical pair at every retained-floor boundary.

**Owned files/boundaries:**

- New schema-first hub artifact-snapshot domain port/projection under `🌎️hub/**` (or the appropriate OS artifact service module), with append-only `artifact-bootstrap-published` / retention events and a read port keyed by `(space, document, descriptor hash, frontier)`.
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`: invoke the authority after admission, authorize read/write access from the directory role, select artifact bootstrap versus tail, and turn `broadcast::Lagged` into explicit resynchronization rather than dropping it.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` and `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`: define the producer acknowledgement needed before the hub advances a floor; do not let a client declare arbitrary opaque data authoritative.
- `🛢️db/📄️artifact/🦀️.rs`, `🛢️db/⚙️engine/🦀️.rs`, and `🛢️db/🔄️sync/🦀️.rs`: only if DB compaction is still needed, finish `ArtifactHandle::snapshot_now` and keep `.spk` use server-internal. It is a separate recovery implementation, not a substitute for the artifact authority.

**Critical design choice:** The hub currently cannot derive an OS canonical pair from opaque foreign-schema operations. It must either (a) call an authorized plugin artifact authority that owns the codec and validates the resulting pair, or (b) accept a signed/verifiable canonical publication with a server-validated revision chain. Do not publish a client-supplied pack merely because its hash is well formed; that creates a forged baseline. The chosen authority must atomically couple baseline frontier, descriptor hash, and retained floor.

**Tests and probe:**

1. Two writers produce concurrent accepted operations, an authority produces one canonical baseline, hub retires earlier tail, and fresh native/browser clients restore exactly the same typed value and history position before applying the later tail.
2. Cross-space and cross-schema swaps with identical bytes/hashes fail because descriptor/frontier binding fails.
3. Force a live broadcast lag and verify the session is told to rebootstrap rather than silently continuing.
4. Hub restart preserves baseline, retention floor, and authorization; a revoked token cannot obtain either existing snapshot or a newly scheduled one.

**Exit criterion:** The hub never advertises a frontier beyond what a reconnecting authorized client can reconstruct from a validated artifact baseline plus tail.

### P3 — Native, Wasm, And Browser Atomic Restore

**Goal:** make every current store consumer apply P1 bytes through the existing artifact snapshot replacement seam.

**Owned files/boundaries:**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`: replace current ignored frame arms in `ArtifactActor::on_hub_frame` and `WasmActor::on_binary` with a bounded bootstrap assembler. Decode canonical pair through the registered document codec, stage it separately, then atomically install it through the existing `ArtifactEvent::SnapshotReplaced`/local persistence behavior.
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`: implement the identical state machine: welcome starts the expected transfer, chunk ordering/integrity gates assembly, done validates/decodes/stages/replaces, only then update frontier/live status.
- Shared test fixtures belong with replication/store fixtures, not in one client implementation.

**Reconnect/cancellation rules:** retain unsent local operations in the existing outbox; never acknowledge a new server frontier before replacement succeeds; on cancellation, socket close, descriptor mismatch, decode error, or missing chunk, discard the staged pair and reconnect from the last committed local frontier. Surface progress and a cancellable sync job to UI/MCP rather than freezing. Reapply only the server tail after baseline install, then reconcile local pending operations using existing conflict semantics.

**Tests and probe:**

1. Native, wasm, and TS each consume the exact P1 vector and produce equal pack bytes, SPR bytes, semantic typed state, and frontier.
2. A two-client reconnect test forces one full baseline and the other tail-only; both end equal after pending local edit replay, while neither sees the other space’s payload.
3. Cancel and malformed-frame tests prove no partial state/event/frontier is committed; resume token retries work after short outage.
4. Browser worker test uses a real WebSocket loopback, disconnects between chunks, asserts bounded retry and an observable progress/cancel status.

**Exit criterion:** Snapshot frames are no longer ignored anywhere, and update of storage, semantic state, and frontier is atomic.

### P4 — Authenticated Hub Workspace Index And Raw Artifact Read

**Goal:** let MCP open a cold hub workspace without synthesizing a probe document.

**Owned files/boundaries:**

- `📇️directory/🔌️client/🦀️.rs`: expose the descriptor through `SpaceDetail` and add an authenticated binary artifact-bootstrap/read method over existing `DirectoryTransport::http`; preserve `OperationContext` cancellation/deadline and response-size limits. Reuse `NativeDirectoryTransport`/HTTP pool; enable the existing kernel `ureq` feature in `🌉️mcp/📦️packages/🦀️rust/Cargo.toml` rather than adding a dependency.
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`: add an authorized read endpoint/port for a descriptor-bound canonical artifact bootstrap. It must check the bearer session and current membership itself; directory discovery alone is not a capability grant.
- `🌉️mcp/🏠️workspace/🦀️.rs`: add a `HubWorkspaceIndex`/artifact-source seam. For `WorkspaceOrigin::Hub`, enumerate `DirectoryClient::space`, read the descriptor and canonical pair lazily, cache only with token/descriptor/frontier binding, and make `read_artifact_bytes`, resource listing, `/schema`, and `/validation` use it. Keep folder behavior unchanged.
- `🌉️mcp/🦀️.rs`: resolve the MCP effective principal from `DirectoryClient::me` and membership at hub binding time; treat CLI scopes as local process policy only, never as remote artifact authorization. Revalidate on reads/jobs and invalidate cached bytes on 401/403, descriptor revision, role change, or reconnect.

**Tests and probe:**

1. Fake `DirectoryTransport` tests cover bearer propagation, cancellation/deadline, non-JSON snapshot response, body limit, member/non-member/revoked outcomes, and cache invalidation.
2. Real hub integration: a fresh MCP process lists an authorized cold document, reads its pack/SPR and descriptor, lists its resource URI, then obtains no list/read after membership revocation. Another space with the same artifact ID remains absent.
3. Exercise the MCP TypeScript end-to-end client against the binary and validate resource JSON output using its existing AJV dependency.

**Exit criterion:** `semio://workspace`, `semio://workspace/artifacts`, and `semio://artifact/{id}` accurately expose authorized cold hub artifacts, without opening a `os.agent.probe/v1` substitute.

### P5 — Inference Over A Cold, Authorized Artifact

**Goal:** connect MCP to the existing plugin inference execution model only after P4 can identify and retrieve the artifact safely.

**Owned files/boundaries:**

- `🌉️mcp/💡️inference/🦀️.rs`: replace `execution_not_wired_error` with an explicit cancellable inference job port. Its input must contain descriptor identity, expected baseline frontier/revision, artifact bytes or an authorized artifact reference, requested inference schema, and caller authorization context.
- `🌉️mcp/🏠️workspace/🦀️.rs` and MCP channel files: add an `artifact-infer` channel command/result/progress/cancel path; reject a stale expected revision when committing/returning result.
- `🔌️plugin/🖥️host/🦀️.rs` and `🏃️run/🦀️.rs`: reuse `ArtifactInferenceRouter` registration, contributor ownership checks, dependency DAG, job stepping, cancellation, and revision/generation validation. Do not recreate a router in MCP or hardwire a map implementation.

**Tests and probe:**

1. An authorized cold document whose descriptor matches a registered plugin runs its declared map/inference and returns result with source frontier/generation. A missing descriptor/plugin/service remains `PLUGIN_UNAVAILABLE`/`NOT_FOUND`, never fabricated data.
2. Cancel a long job, inspect progress, mutate the document before completion, and verify stale output is rejected rather than attached to a newer revision.
3. Cross-space, wrong-plugin, contributor-owner mismatch, and revoked-token cases are rejected before plugin execution. Verify a second user cannot observe job state or result.
4. Use the existing plugin host router tests as the third-party/runtime boundary; add a language-neutral input/output fixture consumed by MCP and the plugin guest where the artifact format permits it.

**Exit criterion:** map/AI inference is a real, cancellable, revision-bound operation on a cold hub artifact, with authorization derived from the hub session.

## Non-Negotiable End-To-End Acceptance Matrix

| Scenario | Required proof |
| --- | --- |
| Cold join after compaction | New native, browser, wasm, and MCP clients discover descriptor, obtain canonical artifact baseline, atomically restore it, then apply tail to the exact same typed state/frontier. |
| Short interruption | Existing local state stays interactive; progress/cancel is visible; reconnect neither freezes nor advances frontier until validated restore/tail completion. |
| Two spaces / same document ID | Descriptor, baseline, tail, presence, MCP resources, and inference are completely isolated by `(space, document)`. |
| Authorization | Read/write/admin role and revocation are enforced both on WS and REST/raw read; CLI-requested MCP scopes never bypass hub membership. |
| Integrity | Descriptor, schema hash, pair hashes, aggregate hash, ordering, limits, and frontier are checked before atomic replacement. |
| Server restart | Directory descriptor, artifact baseline, and retention projection survive; volatile schema maps are irrelevant. |
| Lagged client | Hub forces resume/rebootstrap rather than dropping broadcast messages. |
| AI | Inference resolves only the descriptor-selected plugin, emits progress, honors cancellation, and rejects stale/cross-space/revoked work. |

## Reusable Interfaces, No New Runtime Dependency Required

- `DirectoryTransport`, `DirectoryClient`, `OperationContext`, and `NativeDirectoryTransport` already supply authenticated HTTP, transport injection, cancellation, deadlines, and pooled native I/O.
- `FolderEventLogStorage`, `ArtifactStore::snapshot_pack`, `BackboneMessage::Snapshot`, and `ArtifactEvent::SnapshotReplaced` already establish the canonical `(pack, spr)` persistence/atomic-replacement vocabulary.
- Existing replication Rust/TS codecs and shared fixtures provide the correct place for a language-neutral binary bootstrap contract.
- The plugin host’s `ArtifactInferenceRouter` and run process already own contributor routing, dependency order, job progress/cancellation, and revision/generation checks.
- MCP’s `ajv` dependency can act as an independent JSON-schema oracle for descriptor/resource contracts; it is already declared and need not be added.

No client should take a runtime dependency on DB `.spk` parsing. No new network/HTTP library is required for hub workspace discovery/read; the missing work is correct ports, authenticated projections, canonical artifact authority, and protocol/state-machine implementation.
