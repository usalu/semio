# Terra Audit — Document Open Plan Implementation Packet

Date: 2026-09-03  
Scope: read-only reinspection of the current tree, the plan of record, acceptance matrix, authoritative-open-plan audit, authenticated socket-grant audit, and trusted native-codec/openable-catalog audit. No production or test source was changed. No build, test, or runtime probe was run for this audit.

## Decision

Implement `DocumentOpenPlanV1` **after SocketGrant S1/S2** as the document-policy layer above the already-authenticated WebSocket carrier. The hub alone resolves the stored immutable descriptor through an immutable verified `OpenableDocumentCatalog`, derives the actor and grant from the authenticated binding, selects an allowed declared surface, and returns a 30-second opaque plan receipt. The receipt is exchanged once for a plan-bound `SocketGrantV1`; the WebSocket consumes that socket grant before upgrade and never receives a bearer, actor, descriptor, package, schema/hash, app, or surface from the client.

This resolves the incompatible older shape (`OpenSocketV1 { receipt, credential, ... }`) without widening the new socket-grant carrier: the **socket grant is the only WebSocket credential**. `DocumentOpenPlanV1.receipt` travels only in the protected HTTP body of the document socket-grant issuance command; the ledger records its digest, then the successful socket-grant exchange consumes the receipt. `HelloVNext` remains credential- and identity-free.

There is no current implementation of either `DocumentOpenPlanV1`, `OpenableDocumentCatalog`, or `SocketGrantV1`. The current document socket still trusts `ClientFrame::Hello { schema, pack_schema_hash, actor, token, ... }`, records that actor in durable sync state, and accepts the same caller-controlled mutation actors. The current catalog binding function returns `Vec::new()`, while the resulting optional artifact authority is retained as `_artifact_authority` and is not consulted by document creation or opening. Therefore a plan endpoint must fail closed until its verified catalog dependency is actually published; it must not replay the current client-selected opening path.

## Reinspected authority facts

| Boundary | Current source-backed fact | Required replacement |
| --- | --- | --- |
| Durable descriptor | `DocumentDescriptor` already persists scope, owner `(pluginId, packageId, version, packageHash)`, artifact kind/schema, pack hash, and bootstrap identity; its strict cross-language digest is available. `DirectoryCommand::AnnounceDocument` still accepts the whole descriptor from the caller. | Retain the descriptor/digest as immutable input, but make document creation server-built from an intent and the verified catalog. Existing-document opening never accepts a descriptor. |
| Socket identity | The document handler parses a client actor/token/schema/hash, puts actor into colour/presence, `Principal`, sync-session record, and command flow. The wire encodes the same actor and token. | SocketGrant S2 supplies a server-owned `SocketSubjectV1`; the plan-bound grant supplies the selected open identity. Replace the Hello wire shape and stamp/reject envelope actor at ingress. |
| Catalog | `linked_native_codec_bindings()` is empty; the trusted loader can be absent, and its verified result has no openable-app/surface query. | A selected profile must atomically publish an immutable, descriptor-exact `OpenableDocumentCatalog`; absence, ambiguity, or an incomplete provider is `catalog-unavailable`, never a local fallback. |
| React | `ShellHost` and the backbone worker assemble bindings from local route/plugin/session data. The worker sends the configured actor, schema/hash, and token in Hello. It already has bounded bootstrap progress/cancellation and handles `RebootstrapRequired`. | Route/card/URI data becomes an intent preference only. The first plugin mount waits for exact plan/package/surface validation; rebootstrap gets a fresh plan and does not replay writable work under the stale one. |
| Native | The WGPU relay accepts caller document/schema/app/surface values, creates a local shell actor, and includes the session token in `PersistenceBinding::Hub`. `ProgramBridge::attach_backbone` deliberately returns a retired/no-effect error. | The native relay becomes a structural intent. It may fetch/validate a plan after the carrier exists, but it cannot claim an open or attach a guest until the plan-keyed effect/event mount replacement is repaired. |

The existing document-wide roster remains the required semantic. A selected surface is server-derived, written as peer telemetry by the hub, and is never a roster partition, a route selector, or a permission source.

## Schema-first contract

Put all public forms beside the existing directory descriptor contract, with strict Rust and TypeScript codecs and one neutral JSON fixture family. All identifiers are canonical UTF-8 values of at most 256 bytes; `clientInstanceId` is 1–128 bytes; all finite lists are duplicate-free and canonicalized before comparison. Hashes are exact lowercase nonzero SHA-256 text unless stated otherwise. Unknown fields and version values fail decoding.

```text
DocumentOpenIntentV1 {
  schema: "semio.hub.document-open-intent/v1",
  version: 1,
  scope: { spaceId, documentId },
  requestedSurfaceId?: CanonicalSurfaceId,  // display/mount preference only
  clientInstanceId: OpaqueId
}

DocumentOpenPlanV1 {                       // HTTP response, never persisted in DirectoryEvent
  schema: "semio.hub.document-open-plan/v1",
  version: 1,
  receipt: "open.v1.<base64url-256-bit-secret>",
  expiresAtUnixMs: u64,                    // issue-to-expiry <= 30 seconds
  scope: DocumentScope,
  descriptorDigestV1: LowerHexSha256,
  catalog: { generationId: LowerHexSha256 },
  package: {
    pluginId, packageId, version,
    componentSha256: LowerHexSha256,
    componentBlake3: LowerHexBlake3,
    descriptorByteSha256: LowerHexSha256
  },
  artifact: { kind, schema, packSchemaHash: LowerHexSha256 },
  surface: {
    surfaceId, appId, windowKindId,
    role: viewer | editor,
    rendererTarget: react | wgpu | wasm
  },
  grant: { read: true, write: boolean, observe: true },
  checkpoint?: { checkpointId, descriptorDigestV1, baselineFrontier, aggregateSha256 },
  revalidation: {
    directoryRevision: u64,
    membershipGeneration: u64,
    sessionGeneration?: u64,
    shareGeneration?: u64
  }
}

DocumentPlanSocketGrantIntentV1 {
  schema: "semio.hub.document-plan-socket-grant-intent/v1",
  version: 1,
  planReceipt: "open.v1.<base64url-256-bit-secret>"
}

HelloVNext {
  wireVersion: u32,
  protocolVersion: u32,
  resumeToken?: OpaqueId,
  resumeFrontier?: Frontier
}
```

The endpoint pathname and `DocumentOpenIntentV1.scope` must be equal; neither is an authority claim. `requestedSurfaceId` is rejected when it is not an eligible catalog target; the server may select the catalog default only from the already policy-filtered target set. The response deliberately omits actor, subject/session/share identifiers, raw descriptor/component bytes, factory symbols, storage keys, and catalog rows that were not selected.

`DocumentOpenPlanV1` echoes the descriptor's owner identity only after the catalog proves the owner and artifact row exactly match; `componentSha256` must equal the descriptor's stored `owner.packageHash` under the new explicit package/receipt rule. It does not infer a package from a plugin ID, a renderer manifest, a WASM filename, or a client route. `descriptorByteSha256` and `componentBlake3` remain distinct from the descriptor digest and package SHA-256.

### Private plan record and one-use exchange

The plan registry is process-local, bounded, and deliberately disappears on hub restart. Store only `SHA-256("semio/hub/document-open-plan-receipt/v1\\0" || receipt-secret)`, never the receipt. Its private record additionally carries:

```text
receiptDigest, issuedAtMs, expiresAtMs, state: issued | consumed | invalidated,
scope, full DocumentDescriptor, descriptorDigestV1,
exact catalog generation/package/artifact/surface/grant/checkpoint projection,
binding:
  session { id, authorizationGeneration, userId, membershipGeneration }
  | share { selector, generation, scope },
serverActorId,
directoryRevision, clientInstanceIdDigest, socketGrantSelector?
```

The actor is derived from the verified session binding, for example a domain-separated digest of `(session id, authorization generation)`. A share receives a per-plan server-generated opaque actor and `write = false`; it is neither reusable nor an account identity. Neither form is caller selected. A public anonymous opening is out of this packet: it must return `denied` until a separately specified anonymous principal policy exists.

After SocketGrant S2, `POST /spaces/{spaceId}/documents/{documentId}/socket-grants` accepts `Authorization: Bearer <session-or-exact-share>` and `DocumentPlanSocketGrantIntentV1`. In one mutex/transactional ledger transition it:

1. authenticates the bearer and exact scope again;
2. locates an unexpired `issued` plan by receipt digest and verifies all private bindings;
3. creates the existing `SocketGrantV1` only for `document.v1` and this exact scope, carrying the plan-record identity; and
4. marks the plan `consumed` and indexes it by the socket-grant selector.

The returned shape stays the socket-grant response (`protocol`, opaque grant, expiry); it does not echo the plan receipt or plan internals. The socket upgrade atomically consumes the socket grant before `on_upgrade`, receives the plan record through the server-owned subject, revalidates current directory/session/share state, then derives actor, package/schema/hash/surface/grant before it creates the DB session or any presence. A consumed/expired/revoked receipt cannot issue a second socket grant; a hub restart invalidates both ledgers and requires a new plan. The grant expiry must not exceed the plan expiry.

This is the only receipt consumption path. Do not add a plan receipt to the URL, subprotocol, `HelloVNext`, close reason, telemetry, durable connection record, error, or broadcast frame. The WebSocket offers only `semio.socket.v1, socket.v1.<selector>.<secret>` and the server answers only `semio.socket.v1`, as SocketGrant S2 specifies. `HelloVNext` contains no actor, token, descriptor, plugin/package, schema/hash, app, role, or surface. The hub itself stamps plan artifact identity into bootstrap/rebootstrap validation and plan surface into presence telemetry.

## Commands, CQRS, and projection boundary

### Existing document

`POST /spaces/{spaceId}/documents/{documentId}/open-plan` is a protected command endpoint, not an enumerable catalog read. It accepts only `DocumentOpenIntentV1`. The handler resolves active session membership or exact share; loads the stored descriptor; obtains one exact row through `OpenableDocumentCatalog::resolve_existing`; intersects server-derived document policy with declared operations; selects the target; captures the public active checkpoint only when available; and records the ephemeral plan.

No new Directory event is emitted for planning or mounting: a plan/receipt is an ephemeral capability, and persisting it would undermine restart invalidation. Add redacted append-only security audit records through the directory's existing audit persistence seam: `document-open-planned`, `document-open-denied`, `document-open-plan-consumed`, and `document-open-plan-invalidated`. They contain outcome/reason, correlation id, server peer class, subject/session reference where available, scope digest, and receipt digest prefix only—never a capability, actor, app, catalog topology, checkpoint locator, or raw package byte.

The document projection remains read-side data: immutable `DocumentAnnounced`, descriptor lookup, membership/share state, and active public checkpoint/retention projection. It must expose a monotonic `directoryRevision`, `membershipGeneration`, and durable share generation. Current session generation already exists; membership/share generations need explicit backend-parity fields rather than deriving them from cached role data or an email lookup.

### Create document

Replace `DirectoryCommand::AnnounceDocument { descriptor }` with one server command, for example:

```text
CreateDocumentIntentV1 {
  schema: "semio.hub.create-document-intent/v1",
  version: 1,
  spaceId,
  requestedArtifactKind,
  requestedSurfaceId?,
  clientRequestId
}
```

The author role, new document id, package/codec/app/surface, descriptor fields, descriptor digest, and first checkpoint are server-derived. The catalog's `list_creatable` selects only policy-allowed rows. It materializes and validates the initial `(pack, spr)` pair under the selected native codec before emitting the existing durable `DocumentAnnounced`; checkpoint publication follows the existing append-only checkpoint path. Cancellation, deadline, catalogue failure, codec failure, or duplicate request must leave no `DocumentAnnounced`, checkpoint/public projection, or externally visible partial descriptor. An existing document never takes this command and never receives a replacement descriptor.

`DocumentAnnounced` must be produced with a server-derived `DirectoryActor` tied to the authenticated session rather than a client field. Do not preserve `announce-document` as a compatibility command, hidden route, test-only HTTP shape, or client fallback. Fixture helpers may construct server-owned descriptors only behind an explicit trusted fixture catalog.

## Issuance, mounting, revocation, and rebootstrap laws

Plan issuance and socket mounting are bounded control operations: 10-second end-to-end plan deadline, 2-second socket-grant exchange deadline, propagated cancellation token, one outstanding plan per binding+scope, at most 64 plans per binding and 1,024 process-wide entries, 8 KiB intent and 64 KiB response limits, and bounded fixed reason codes. Issuance has no user-facing percentage progress because it is small; catalog startup and bootstrap transfer retain their established staged progress. Cancellation before publication creates neither plan nor audit success; cancellation after receipt publication invalidates the entry and returns `cancelled`.

The only externally stable errors are `denied`, `not-found`, `catalog-unavailable`, `component-unavailable`, `stale`, `expired`, `already-consumed`, `cancelled`, and `deadline-exceeded`. Map backend/catalog/codec failure details to these redacted codes. A forbidden requested surface must not reveal other declared surfaces.

Revalidate at plan issue, plan-to-socket-grant exchange, socket upgrade, before every privileged inbound frame, and on the existing one-second live-binding tick. These events invalidate unconsumed plans and close the matching live socket terminally:

- session revoke/expiry or changed authorization generation;
- membership removal/role generation change, space/document deletion, or admin kick where the policy makes the socket non-admissible;
- exact share revoke/expiry/generation change;
- descriptor digest mismatch, selected catalog generation stale/restarted, or a selected target no longer present;
- release of a checkpoint the plan selected for bootstrap.

A newly published checkpoint/frontier alone does not rewrite the immutable descriptor/package/surface identity and need not kill an already-live plan-bound session. However, on `RebootstrapRequired`, the client must close/abort its transfer, discard committed remote writable state and resume token, obtain a **fresh plan**, exchange it for a fresh socket grant, and only then validate the public `(pack, spr)` pair and reconnect. P2-C controls must match the fresh plan scope, descriptor digest, artifact schema/hash, and (when supplied) selected checkpoint. Pending local work remains bounded and deduplicated, but cannot replay or turn into a spectator action until a fresh `write = true` plan reaches live state.

## React and native first-frame impact

React is the first consumer after server policy is available:

1. `ShellHost` converts a card/URI/relay selection into `{ scope, requestedSurfaceId?, clientInstanceId }`; plugin/app/schema/actor in the relay are ignored and eventually removed.
2. It fetches a plan through the protected local relay/production verifier, verifies a locally loaded package against all returned package and selected surface fields, and then creates the worker binding from the plan—not from local app order or URI defaults. A missing or mismatched local component is a localized `component-unavailable` view, never a fallback renderer.
3. The worker exchanges the plan receipt for a document socket grant for every dial, offers the grant as the second subprotocol, and sends credential-free `HelloVNext`. It treats `Welcome` as authorized only after the plan identities match. Its present P2-C abort/progress handling becomes plan refresh before pair transfer/reconnect.
4. Before a plan is usable, first frame renders an accessible status; bootstrap progress has a named value; Cancel aborts the request/transfer; and EN/DE translation entries exist with no implicit default-language fallback.

Native must use the exact same plan and socket-grant contracts, but its executable mount stays blocked. `OpenArtifactRelayTarget` becomes only the structural preference; plan verification selects the local package/app/surface; a window-owned, plan/scope/generation-keyed event/effect mount supervisor attaches the guest only after both plan and component verification. The current `ProgramBridge::attach_backbone` is explicitly retired and returns an error, so native may show a bounded component/mount-unavailable status but cannot claim collaboration, first-frame app mount, or parity until its separate repair lands. Its roster is document-wide; surface is supplied only from the server plan telemetry. The native/MCP identity and sync bindings must stop carrying session capability in `PersistenceBinding::Hub`/Hello.

## Dependency-bounded landing order

| Order | Can land now / prerequisite | Deliverable | Hard boundary |
| --- | --- | --- | --- |
| D0 | No runtime prerequisite | Rust/TS schemas, `HelloVNext` schema, strict codecs, error vocabulary, neutral fixtures/oracle, and a plan registry type whose issuer returns `catalog-unavailable` when no verified catalog is published. | No usable plan endpoint, document creation, or client mount may fall back to the old descriptor/Hello path. |
| D1 | SocketGrant S1/S2: digest-only ledger, binding revalidation port, protected issuance, atomic upgrade consume, server actor stamping | Plan-receipt-to-document-socket-grant exchange and plan identity carried privately by `SocketSubjectV1`; retire actor/token/schema/hash/surface authority from socket ingress. | SocketGrant S3 still must migrate every caller and delete old token/actor carriers; until then no secure end-to-end open claim. |
| D2 | Catalog H1–H5: explicit package/open-target descriptors, generated static provider, complete selected fixture profile, immutable catalog publication | Existing-document plan issuance and socket mount for the minimal verified cohort. | Empty/partial catalog, package mismatch, missing codec, or ambiguous surface is fail-closed; no generated registry row or local manifest is an authority substitute. |
| D3 | D2 plus verified initial-pair materialization/retention within actual storage limits | Server-only `CreateDocumentIntentV1`, `DocumentAnnounced`, and first checkpoint transaction. | Do not claim large-pair creation until the chunk-CAS retention/release packet covers it; the historical 496 KiB single-blob limit is not a 64 MiB authority proof. |
| D4 | D1+D2 | React plan-first opening, fresh plan on rebootstrap/reconnect, accessible EN/DE states. | No shell URI/plugin/app/schema/actor fallback and no stale-write replay. |
| D5 | D1+D2+D4 plus native ProgramBridge effect/event mount repair | Native plan consumption and genuine first-frame mount. | Native stays a fail-closed unavailable state until attachment is real; do not bypass it with a legacy direct backbone. |

## Exact source ownership

1. Cross-language directory/open-plan schemas and strict codecs: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`, and new neutral fixture(s) under `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️` beside `document-descriptor.json`/`artifact-authority.json`.
2. Durable descriptor creation, generations, directory decision/projection, and SQLite parity first: `🌎️hub/📇️directory/🦀️.rs`, `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`, then matching PostgreSQL/Neo4j backend modules. The `HubDirectory` port must expose id-only active session/share/membership generation checks; it must not accept a raw bearer for live revalidation.
3. Socket plan exchange, HTTP routes, `HubState` bounded ledgers, server actor stamping, command gate, P2-C refresh, and test loopback server: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`.
4. Credential-free handshake and actor-free mutation boundary: `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` and `🧰️framework/🔨️modules/📡️replication/🟦️.ts`.
5. Verified catalog/provider query and static factory proof: `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`, descriptor/manifest owner modules, and the generated composition output owned by that script. Do not hand-edit generated catalog output.
6. React intent, plan UI, socket-grant exchange, and refresh: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx`, `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`, and `🧰️framework/🛍️products/💻️os/🟦️.ts`.
7. Native intent/binding/plan mount and its distinct attachment prerequisite: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`, plus store-sync/MCP callers that currently encode Hub tokens.

## Neutral fixture/oracle and focused runtime laws

Use one language-neutral `document-open-plan-v1` JSON corpus, not a copied production catalog. It contains one verified viewer/editor artifact with a known descriptor/receipt/catalog/checkpoint binding and hostile vectors for: foreign scope, forged actor (rejected because it is absent from the public input), descriptor/package/component/hash mismatch, editor requested by a read-only share, mismatched renderer target, stale membership/session/share generation, revoked/deleted scope, expired plan, second exchange, second socket use, hub restart, cancelled/deadline plan resolution, released checkpoint, and rebootstrap refresh. Its Node built-in `crypto` oracle recomputes descriptor, catalog-generation, and receipt digests from documented byte encodings and compares redacted accepted/rejected outputs with Rust and TypeScript. A second implementation must not import the production codec helpers being checked.

After implementation, focused laws—not current evidence—are:

1. Rust schema/ledger law: equal neutral vectors decode identically; receipt digest has no plaintext; concurrent plan-to-grant exchanges yield exactly one grant; wrong scope/binding/generation and max-plus-one entries fail without leaks.
2. Real loopback socket law: issue a plan, exchange it for a socket grant, verify the selected subprotocol is only `semio.socket.v1`, send `HelloVNext`, and prove client actor/schema/package/surface cannot affect stored `Principal`, sync record, presence, or mutation actor. A second exchange/upgrade fails.
3. Catalog law: no plan for absent/ambiguous/incomplete target; exact descriptor/package/checkpoint/catalog generation succeeds; a share receives only viewer/read/observe and cannot enumerate foreign documents or storage locators.
4. Revocation/recovery law: revoke between issue and exchange, between exchange and upgrade, and while live; force lag/rebootstrap, selected-checkpoint release, catalog-process restart, and document delete. Assert terminal close, fresh-plan requirement, bounded abort, no stale writable replay, and no credential/locator in errors/audit/broadcasts.
5. React/native UI law: React first-frame status/progress/Cancel and EN/DE keys are accessible; worker reissues plan/socket grant on every dial. Native semantic-tree coverage remains explicitly blocked until the ProgramBridge mount supervisor exists, then asserts the same plan identity and document-wide roster.

## Blocker order

1. **Critical — SocketGrant S1/S2 and its server-derived actor boundary.** Current Hello and mutation actors remain caller-controlled; an open plan cannot repair an insecure carrier around it.
2. **Critical — complete immutable verified openable catalog for at least one selected cohort.** Current empty bindings and no query mean the hub cannot authoritatively choose a package/app/surface. Plan issuance must return `catalog-unavailable` rather than imitate local routing.
3. **High — replace public descriptor announcement with server-side creation.** A trusted existing-open plan does not repair a malicious descriptor that an author can still publish.
4. **High — all-client socket migration and deletion of old carrier fields.** React/native/MCP/admin token/actor query/frame routes remain a release blocker until removed together.
5. **High — React plan-first mount/rebootstrap; native ProgramBridge repair afterward.** React can enforce plan validation once D1/D2 exist. Native cannot truthfully mount until the retired attachment has an event/effect replacement.
6. **Medium — checkpoint retention/release and exact tail barrier.** This gates broad cold-open/large-pair claims, not the plan schema or small verified existing-document policy.

The current acceptance states therefore remain `PARTIAL` for descriptor/bootstrap/React and `BLOCKED` for collaboration/native/integrated release. This report supplies implementation order only; it establishes no build, test, socket, mount, or end-to-end runtime result.
