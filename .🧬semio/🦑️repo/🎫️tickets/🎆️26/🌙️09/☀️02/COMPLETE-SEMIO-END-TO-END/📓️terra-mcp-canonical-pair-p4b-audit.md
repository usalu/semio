# MCP P4-B Canonical Pair Read Audit

## Decision

P4-B must be one authenticated, exact-`DocumentScope`, **active-checkpoint snapshot** read.  It is not a promotion of the generic blob CAS, a generic hash URL, a client-selected checkpoint, or an MCP raw-byte resource.  The server selects the current durable descriptor and active verified checkpoint; it verifies the two immutable blobs; and it emits a bounded `(pack, spr)` transfer whose header binds all of those identities.

The first deterministic blocker is **high**: P2-A1 admits a 64 MiB pair, while the current `DbImmutableArtifactBlobStore` rejects either durable blob over 496 KiB.  P4-B must wait for P2-D's reference-safe chunk-manifest CAS.  Shipping a route over today's storage would silently narrow the advertised authority contract and would still leave unreferenced CAS retention unresolved.

This was a read-only audit on the current shared tree.  No build or test was run.

## Current boundary census

| Boundary | Current source evidence | P4-B consequence | Severity |
| --- | --- | --- | --- |
| Descriptor identity | `DocumentDescriptor` carries structural scope, artifact kind/schema, plugin/package/version/component hash, pack schema and bootstrap data; its v1 SHA-256 is canonical field encoding, not JSON (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:296-399`). | Use this exact digest; never infer an owner from an MCP artifact ID or plugin label. | High |
| Durable checkpoint projections | The public checkpoint deliberately excludes `storage_key`; the private record retains it.  The directory checks descriptor digest, exact checkpoint identity, active-parent progression, and atomic public/private append (`🌎️hub/📇️directory/🦀️.rs:866-959,1300-1332,1383-1397`; SQLite projections `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:86-124,202-209,636-651`). | The HTTP handler may read the private record only after exact-scope authorization.  A locator, manifest ID, chunk ID, or backend error never crosses the response boundary. | High |
| Rebootstrap verifier | `VerifiedRebootstrapSource::verified` checks descriptor, active public checkpoint, private checkpoint equality, digest and checkpoint ID; `load` checks pack/SPR SHA-256, lengths, concatenated aggregate and 64 MiB/4 KiB/16,384-chunk limits (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:187-280`). | Extract this selection/verification into a reusable server-internal reader.  It is currently only used to form socket rebootstrap control (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:832-842`), not an MCP route. | High |
| Generic blob REST | `/spaces/{space}/blobs/{hash}` accepts a client hash and reads/writes a space-wide payload; it buffers complete pages into an HTTP body (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:704-775,2092`).  Its authorization borrows document authorization with a hash substituted as a document ID (`:449-480`). | Never reuse it.  It has no descriptor/checkpoint/aggregate/frontier association, exposes hash probing/correlation within a space, and cannot atomically return a pair. | High |
| Storage capacity and retention | The current immutable adapter maps public SHA-256 to private BLAKE3 payload locators and caps one blob at `496 * 1024` (`🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs:20-23,172-276`).  The P2-D packet specifies scoped 256 KiB chunks, private manifests, reservations/references and a generation-fenced sweeper (`📓️terra-artifact-chunk-cas-retention-audit.md:5-31,82-109`). | P4-B's only storage port must be the P2-D manifest reader.  It must never expose or delete generic payload objects. | **High — first blocker** |
| Hub route and session policy | The router has descriptor status, share and document WS routes but no canonical pair route (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2047-2105`).  Current `resolve_auth` accepts session, exact-document share, and public fallback (`:452-480`). | Pair authorization needs the active authenticated-carrier migration; it must not be added to the raw bearer/query-token paths that migration removes. | High |
| MCP P4-A | `HubRemoteBinding` authenticates `/auth/sessions/me`, requires current membership and indexes by full `DocumentScope` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:109-185,266-315`). | This is the correct metadata precondition, but it contains no checkpoint or byte transport. | Medium |
| MCP resources | Hub resources list only structural descriptor URIs; `read_artifact_bytes`, raw, schema and validation intentionally return retryable `PluginUnavailable` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1278-1304,1558-1666,1716-1730`). | Keep this honest until a verified pair is mounted through the trusted openable catalog.  Do not base64 a 64 MiB pair into MCP. | Medium |
| Client transport | `DirectoryTransport` returns only status plus an already materialized `Vec<u8>` and `DirectoryClient` stores a raw bearer (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:30-113,275-346`). | Add a separate bounded streaming pair transport/capability-carrier seam; do not stretch the directory JSON client into an all-body binary reader. | High |
| Catalog generation | The trusted bundle loader creates an immutable package/codec snapshot but has no public generation field (`🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:212-245,246-352`), and `linked_native_codec_bindings()` is currently empty (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:179-190`).  The native-openable-catalog audit specifies restart-only generation semantics (`📓️terra-trusted-native-codec-openable-catalog-audit.md:41-105`). | A pair can be integrity-verified before a codec exists, but it cannot truthfully activate MCP schema/validation/opening until a verified catalog generation is available. | High |

The directory's public event stream already carries `ArtifactCheckpointPublished` and retention events (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:152-176`).  Thus it is an invalidation signal, not a blob-read API.

## Schema-first P4-B contract

Add a language-neutral `CanonicalCheckpointPairReadV1` schema and fixture before implementation, proposed under `🌎️hub/🗿️artifact-authority/🧬️schema/`.  The single query is:

```text
GET /spaces/{spaceId}/documents/{documentId}/active-checkpoint/pair
Accept: application/vnd.semio.canonical-checkpoint-pair.v1
If-None-Match: <optional prior canonical ETag>
```

There are **no query parameters** and no checkpoint ID, descriptor digest, blob hash, storage locator, chunk locator, app, plugin, actor or frontier selector.  `Range` is rejected.  The endpoint derives principal, session generation, roles and scope from the authenticated carrier introduced by the socket/client-auth migration; callers do not supply an actor.  It must not use the current directory WebSocket query credential (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1621-1637`) nor reintroduce token values into MCP configuration, URLs, logs or cache keys.

The successful binary body has a fixed, length-prefixed `CanonicalCheckpointPairReadV1` header followed by bounded data records.  Use repository-owned fixed-endian/length-prefixed encoding, not JSON/CBOR canonicalization.  Its header is exactly:

```text
formatVersion = 1
scope = (spaceId, documentId)
descriptorDigestV1[32]
activeCheckpointId[32]
catalogGenerationId[32]
baselineFrontier = (documentId, headEditOrdinal, headEditId, lastCommitSeq, chainHash)
requiredTailFrontier = baselineFrontier
pack = (sha256[32], byteLength u64)
spr  = (sha256[32], byteLength u64)
aggregateSha256[32] = SHA-256(packBytes || sprBytes)
```

`catalogGenerationId` is the future verified native-openable-catalog generation digest, not a package name, native symbol, bundle path or client preference.  A process whose catalog is not ready returns a bounded unavailable result; it never substitutes an empty/zero generation.

The body records are `(part: pack|spr, ordinal u32, byteOffset u64, byteLength u32, bytes)`, in pack then SPR order, contiguous from offset zero.  Each record is at most the existing 4 KiB bootstrap chunk size; the pair is at most 64 MiB and at most 16,384 data records, matching the established P2-C protocol bounds (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:226-280`).  The header is capped at 16 KiB and each identity retains the already-enforced bounded UTF-8 rules.  There is no private storage field anywhere in the schema, fixture or success/error body.

`requiredTailFrontier` deliberately equals the checkpoint baseline in P4-B.  The pair is an immutable checkpoint snapshot, not a claim that it represents the current live document.  The current hub has no atomic checkpoint-barrier-to-tail cursor response: document status samples an opened DB handle (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:643-660`) and P2-C currently sends a control/close, not a tail transfer.  A later authenticated open-plan/socket lane must issue an exact post-baseline cursor and replay its tail before it may call the view “current.”  It must reject a changed descriptor/checkpoint/generation rather than combine an old pair with a new tail.

The canonical ETag is the lowercase SHA-256 of this explicit input:

```text
"semio.mcp.canonical-checkpoint-pair.etag.v1\\0" ||
field(scope.spaceId) || field(scope.documentId) ||
descriptorDigestV1 || activeCheckpointId || catalogGenerationId ||
field(baselineFrontierEncodingV1) || field(requiredTailFrontierEncodingV1) ||
pack.sha256 || u64be(pack.byteLength) || spr.sha256 || u64be(spr.byteLength) || aggregateSha256
```

`304 Not Modified` is allowed only after authenticating, reauthorizing, rereading the exact descriptor and active public/private checkpoint, checking the catalog generation, and comparing this ETag.  It is never an authorization shortcut.  Replies use `Cache-Control: private, no-store` and `Vary: Authorization`; the MCP process may maintain only a verified in-memory cache keyed by hub origin, **session authorization generation (never capability text)**, full scope, descriptor digest, checkpoint ID, catalog generation and ETag.

## Server selection, privacy and failure rules

1. Authenticate the protected carrier and check deadline/cancellation before lookup.  A member session needs current read authority in the exact space.  A share capability is acceptable only for the exact document URL and yields `read/checkpoint-pair` capability only; it cannot enumerate a space, open another document, write, select a codec, mutate or infer.  Public-space fallback is denied.  An administrator has no implicit data-plane bypass: it must be a current member or possess a separate, audited export capability.  A revoked/expired session, removed membership or revoked share is rechecked before metadata, before transfer and on each bounded output turn.
2. Construct `DocumentScope` from route parameters.  Read descriptor, active public checkpoint and corresponding private checkpoint; require exact public projection equality, descriptor digest, checkpoint identity encoding and scope.  Read the current catalog generation and require `resolve_existing(descriptor)` in the future provider.  Any absence/change/ambiguity is a generic bounded unavailable/not-found outcome; it never falls back to a prior checkpoint or generic blob.
3. Under the P2-D reader, validate both private manifest locators, every scoped chunk and raw pack/SPR SHA-256/length, then the aggregate.  To avoid calling corrupt bytes a successful pair without keeping 64 MiB in memory, perform an incremental preflight verification pass and a second incremental emit pass; rehash while emitting.  Progress is `authorize → metadata → verify-pack → verify-spr → stream-pack → stream-spr → ready`; cancellation/deadline checks occur before every 256 KiB storage chunk and every 4 KiB output record.  The existing P2-C 15 s deadline is the initial bound (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:11-59`); one transfer per authenticated MCP principal is admitted at a time.
4. Before the first record, a semantic failure is a bounded 401/404/409/503 class with no locator, chunk/object hash, backend, filesystem or peer diagnostic.  After streaming begins, terminate the framed response with a fixed `cancelled`, `unavailable`, `integrity`, `stale-generation` or `deadline` terminal code and close it.  The client discards all partial bytes.  A corrupt/missing referenced object never falls back to a client snapshot; it is an integrity/unavailable incident for repair/re-materialization.
5. P2-D storage chunks are 256 KiB, scoped by space, and only same-space deduplicate; P4-B's 4 KiB records are transport chunks, not CAS identities.  Consequently two documents may share physical scoped chunks without gaining cross-document access, and equal document IDs in different spaces have distinct authorization and cache identities.

This makes share read-only and multi-MCP isolation explicit.  Each MCP process receives its own local-bootstrap-delivered upstream carrier; it owns a separate binding/cache and cannot reuse another process's entry.  Revocation, membership removal, space deletion or carrier expiry cancels active reads, zeroizes that process's pair cache and places its binding in `Revoked`.  A stream disconnect, any descriptor/checkpoint/retention event for the selected space, a P2-C `RebootstrapRequired`, ETag mismatch or catalog-generation change enters `Refreshing`, discards mounted pair/tail/resume state, then requires a fresh P4-A descriptor refresh before another pair read.  Restart has the same result: no persisted raw pair cache, fresh carrier validation, then a new exact read.

## MCP binding and resource contract

Add a `CanonicalPairTransport`/`CanonicalPairClient` alongside, not inside, `DirectoryTransport` in `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/`.  Its public request accepts only an `OperationContext` and a full `DocumentScope`; the capability carrier stays private.  Its result is an in-memory `VerifiedCanonicalPair` whose constructor recomputes descriptor/checkpoint/pack/SPR/aggregate/ETag bindings before transfer to a codec.  It owns progress, cancellation and zeroization.  It must not return a `Vec<u8>` through the existing JSON transport.

Extend `HubRemoteBinding` with a per-scope pair state:

```text
Absent | Fetching(progress) | Ready(verified identity key) | Refreshing | Revoked
```

Only the actor/worker performs I/O.  Existing synchronous MCP `GatewayBackend`/resource calls read the published state and return retryable unavailable while `Fetching`/`Refreshing`; they never block for network or silently turn a failure into an empty catalogue.  The current binding already distinguishes `Unbound/Refreshing/Ready/Revoked` and increments a refresh generation (`🏠️workspace/🔗️remote/🦀️.rs:37-185,226-315`), which is the reuse seam.

P4-B does **not** add an agent-visible raw `(pack,spr)` resource.  A 64 MiB raw response is neither a useful MCP resource nor safe to base64.  Keep:

* `semio://workspace/scopes/{space}/{document}/descriptor` as the small P4-A descriptor projection.
* Hub `semio://artifact/{documentId}` raw, `/schema` and `/validation` unavailable; a bare document ID is not a portable authority key.  The current explicit unavailability is correct (`🏠️workspace/🦀️.rs:1716-1730`).
* After the trusted native `OpenableDocumentCatalog` and server open plan exist, expose schema/validation only at a structural-scope URI and only from the verified mounted pair.  Their result binds the descriptor digest, checkpoint ID, catalog generation and baseline/tail state.  Read-only shares can observe declared schema/validation only if that contract treats it as non-mutating; they never receive mutation/inference authority from a renderer or package declaration.

Thus P4-B is pair transport/mounting, not a claim of a current decoded document, inference execution or generic blob access.

## Bounded implementation packet

| Order | Sol-sized packet | Exact owned seams and proof |
| --- | --- | --- |
| 0 | **P2-D first** | Land the scoped 256 KiB manifest CAS, atomic reservation/reference/release ledger, retention ordering and sweeper in the artifact authority/directory backends, then replace `DbImmutableArtifactBlobStore` use in `VerifiedRebootstrapSource`.  Prove a 496 KiB+1 and a 64 MiB pair across SQLite first; PostgreSQL/Neo4j remain explicit runtime probes.  Do not add P4-B against the old adapter. |
| 1 | **Secure carrier completion** | Finish the authenticated socket/client migration and expose a private MCP upstream carrier from `LocalBootstrapTransport`/`McpCredentialEnvelopeDelivery` (`🌎️hub/📇️directory/🦀️.rs:620-679`).  Derive session/principal/generation server-side; remove query credentials/caller actors from directory/document flows before the pair route uses them. |
| 2 | **Pure v1 pair schema and reader** | Add the schema, canonical encoder/decoder, ETag input and neutral vectors in `🌎️hub/🗿️artifact-authority/`; factor P2-C's `verified` logic into `VerifiedCanonicalCheckpointReader`.  It returns only public header data plus a private P2-D reader plan, has fixed progress/cancel/deadline/error bounds, and is shared by P2-C/P4-B. |
| 3 | **Hub exact route** | In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, add exactly the route above and an authenticated-request handler.  Select active checkpoint only, use the reader, run the two-pass bounded verify/emit transfer and perform revocation/generation checks.  Do not modify `/blobs/{hash}`. |
| 4 | **MCP pair actor** | In `🏠️workspace/🔗️remote/🦀️.rs` add the private streaming client and pair state; in `🏠️workspace/🦀️.rs` mount verified pairs by structural scope and preserve descriptor-only/public resource behavior until the next packet.  Evict on every specified event/control/error; capability text never reaches a cache key, resource or diagnostic. |
| 5 | **Catalog/open-plan activation** | Land `NativeOpenableCatalogProviderV1` generation and the server-issued open-plan.  Gate schema/validation/codec mounting on exact descriptor ownership plus generation; bind the socket's post-baseline tail cursor before allowing a “current” document claim. |

## Neutral and independent-oracle tests

Create one neutral JSON fixture/schema containing accepted pair headers/bytes and expected outcome/cache transition for:

* initial cold member read; 4 KiB boundary, 496 KiB+1, and 64 MiB pair; Node built-in `crypto.createHash("sha256")` independently recomputes pack, SPR, aggregate and ETag;
* same `documentId` in two spaces; non-active/foreign checkpoint selector attempts; descriptor rotation, active-checkpoint rotation and catalog-generation mismatch between P4-A refresh and pair read;
* revoked/expired session, removed member, revoked/expired exact share, public-space request, admin-not-member request, two isolated MCP carriers and attempted cache reuse;
* cancellation during preflight/emission, deadline, truncated/reordered/oversize records, wrong raw/chunk/manifest hash, unavailable referenced object, restart, directory stream reconnect and `RebootstrapRequired`.

Rust laws should use a deterministic P2-D memory/SQLite CAS and recording authenticated carrier to prove selection is active/exact, private locators never serialize, and every partial body is rejected.  An Axum loopback oracle must revoke membership/share between metadata and emit, then assert cancellation/no cache return and cross-space non-disclosure.

TypeScript should validate the neutral JSON with AJV, decode the independent binary fixture, recompute hashes/ETag with Node `crypto`, and drive two separate MCP stdio processes against the real protected hub.  It must compare their resource lists and assert that raw pair bytes, carrier text, locators and generic blob URLs never appear.  This is deliberately independent of Rust's serializer and authority implementation.

## Focused follow-up commands

Run only after the preceding packet lands and concurrent Cargo work is clear:

```sh
bun nx run os-hub:test-quick -- canonical_checkpoint_pair
bun nx run @semio-tech/framework-os-mcp-rs:test-quick -- canonical_checkpoint_pair
bun nx run @semio-tech/framework-os-mcp:test-quick -- canonical-pair
HUB_E2E=1 bun nx run os-hub-ts:test-quick -- canonical-pair
```

The first three existing targets are defined in `🌎️hub/📦️packages/🦀️rust/📋️project.json`, `🌉️mcp/📦️packages/🦀️rust/📋️project.json` and `🌉️mcp/📦️packages/🟦️typescript/📋️project.json`; the final existing hub E2E script deliberately builds a real hub only when `HUB_E2E=1` (`🌎️hub/📦️packages/🟦️typescript/📜️script.ts:1-35`).  PostgreSQL/Neo4j require their external Docker/runtime prerequisites and are not a zero-touch green claim.

## Exit criteria

P4-B is complete only when an independently authenticated MCP process can cold-read one server-selected active checkpoint pair for an exact structural document, recompute every advertised identity, safely mount it, and discard it on revoke/rebootstrap/generation change; a share is exact-document read-only; equal document IDs across spaces and two MCP processes cannot cross-read/cache; no public request can probe private manifests/chunks/blobs; and the pair's advertised 64 MiB limit actually works on the P2-D substrate.  Until then MCP may expose P4-A descriptor metadata only.
