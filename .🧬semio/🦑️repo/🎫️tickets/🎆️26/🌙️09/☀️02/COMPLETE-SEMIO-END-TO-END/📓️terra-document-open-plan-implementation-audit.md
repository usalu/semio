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

## Current D0 source and gate re-read — no activation drift

The D0 packet is now present and remains deliberately non-activating. Rust and TypeScript expose
strict intent, plan, exchange, and redacted error forms; both reject unknown fields, noncanonical
43-character receipt tails, control characters, unsafe integers, ambiguous session/share
generation, mismatched checkpoint/frontier, bad hash text, and invalid grant/surface pairing
(`🧬️schema/🦀️.rs:601-760`, `🧬️schema/🟦️.ts:409-532`). The fixture contains one descriptor,
nonempty catalog row, canonical SHA-256 receipt digest, and sixteen hostile vectors
(`🧫️fixtures/📇️directory/📄️document-open-plan-v1.json:1-154`).

The process-local ledger is bounded to 1,024 records and 64 issued records per binding. It stores
only a domain-separated SHA-256 receipt digest, zeroizes the boxed secret on drop, uses one mutex
to make exchange single-winner, checks an exact current private authority, and invalidates/reaps
replacement, expiry, and revocation indexes (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:754-1088`).
Its selected law includes concurrent exchange, stale/expired/replaced receipts, restart-scoped
absence, capacity, and session/share administrative invalidation (`:5659-5806`).

The fail-closed catalog boundary is honest at D0: there is no public open-plan or
receipt-to-SocketGrant route in the live router, which only mounts existing socket-grant routes
(`📦️bin.rs:4489-4519`). Nor is there a catalog provider/authority-construction route that can
turn an empty catalog into a plan. `catalog-unavailable` is contractual vocabulary, not a falsely
claimed live behavior yet; D1/D2 must bind a nonempty verified catalog generation before adding
an issuer. This is intentionally still not an open/mount authority claim.

`os-hub:open-plan-check` is uncacheable and launch-registered
(`📋️project.json:103-109`, `.vscode/launch.json:4422-4429`). It runs the self-contained Node
descriptor/catalog/receipt/negative oracle before Cargo, exact-lists then exact-runs one kernel
schema law and two hub-ledger/revocation laws (`📜️script.ts:1886-2129`). The corrected schema
selector is the physical package `semio-framework-os-kernel`
(`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:1-5`). No current runtime terminal is
claimed here: I did not run this Cargo gate while the shared target is occupied.

## Cargo-101 attribution and narrow-law boundary

The recorded `open-plan-check` terminal is an honest preflight failure, not a zero-test pass:
the independent Node oracle and its separately labelled production-codec parity finished, then
the first `cargo test --all-features -p semio-framework-os-kernel --lib … --list` returned status
101 before any Rust law could be enumerated. Its first reported error was the unrelated missing
`issues_scoped_to_new_solids` symbol in the B-rep plugin source. This attribution is consistent
with the kernel's test dependency fan-in: its `Cargo.toml:126-160` declares the plugin fixture
crates as direct dev-dependencies, so the kernel lib-test harness cannot be treated as isolated
from that graph merely because the selected schema law is D0-owned.

There is no truthful one-command narrow substitute for all three gate laws. The kernel schema
law must remain red/runtime-unproven until its actual lib-test harness compiles. The two hub
binary laws have no feature guard in their definitions (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5712-5809,
5811+`) and their test state uses the hub's default SQLite feature. After target ownership is
available, two separately labelled default-feature exact commands may therefore provide useful
*subset* evidence for those same production test functions:

```text
cargo test --manifest-path Cargo.toml --bin os-hub <resolved-ledger-FQN> -- --exact --test-threads=1
cargo test --manifest-path Cargo.toml --bin os-hub <resolved-revocation-FQN> -- --exact --test-threads=1
```

They must retain the gate's exact-one `--list` resolution first and must not replace or clear the
separately red `--all-features` schema/graph terminal. I did not run either command in this audit.

## D1 receipt-to-socket-grant source re-read

The new private `exchange_to_socket_grant` path retains the plan-ledger mutex through exact
receipt-digest lookup, expiry/state/authority validation, socket-ledger issue, and marking the
plan consumed (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1027-1063,1071-1109`). There is no reverse
nested lock in the invalidation callers; they finish socket-ledger invalidation before taking the
plan ledger. A full socket ledger therefore returns `DeadlineExceeded` before the plan record is
consumed, so the same receipt can retry after capacity is released. The focused D1 law races eight
exchanges (one success/seven `AlreadyConsumed`), proves the exact actor/audience/subject/expiry
pending grant, fills capacity, checks the plan remains `Issued`, then retries successfully
(`:5821-5894`). Expiry is capped at the minimum of issue+socket TTL, plan expiry, and binding
expiry (`:1079-1091`).

Both capability types redact debugging: the plan capability is boxed and zeroed on drop
(`:758-769`), and the established socket-capability debug implementation shows only selector and
`[REDACTED]` secret (`🌎️hub/📇️directory/🦀️.rs:583-615`). The two ledgers retain only
domain-separated secret digests, never either raw secret. The response necessarily contains the
new one-use socket grant for its caller; it does not echo the plan receipt. No log/audit format was
added by this private helper.

D1 remains source-first and non-activating: the router still has no `open-plan` route
(`📦️bin.rs:4553-4588`), readiness still reports `open_plan: false` (`:1527-1560`), and no catalog
authority or `DocumentPlanSocketGrantIntentV1` route handler is present. The Node pre-Cargo oracle
now rejects any open-plan route, any public exchange intent, or a production count other than one
private helper (`📜️script.ts:2056-2064`). The primary all-feature gate now exact-selects all four
current laws, including D1 (`:2106-2137`).

The separately added `open-plan-server-check` is an honestly labelled default-feature hub subset:
it exact-lists and runs the two pre-existing hub laws plus D1 exchange, then explicitly says the
kernel all-feature schema qualification remains separate (`📜️script.ts:2140-2165`; uncacheable
project target `📋️project.json:111-117`). It is not a substitute for the prior all-feature-graph
red. Its launch-seed entry is currently ahead of generated `.vscode/launch.json`
(`.vscode/🧩️launch.seed.jsonc:3085-3093`; generated file has no matching entry): generated launch
freshness remains a release-blocking registration gap. No Cargo command was run by this audit.

### D1 redaction/zeroization hold

`document_open_plan_base64url_decode` zeroes its stack buffer for late tail/length failures, but
the invalid-alphabet branch uses `document_open_plan_base64_value(byte).ok_or(...)?` after prior
bytes may already have been decoded (`📦️bin.rs:803-829`). That early return does not fill
`decoded`. Because D1's exchange parses the externally supplied plan receipt, this is a real
partial-secret/capability candidate wipe gap despite the successful-path boxed drop and Debug
redaction. Replace the early `?` with an explicit fill-before-error (or a wiping candidate type)
and add an observer law for an invalid character after valid decoded prefix. This finding holds
secret-hygiene acceptance; no source route is activated by it.

### Wipe-hold source repair — gate selection still required

The preceding early-return wipe finding is source-closed. The decoder now wraps its candidate in
`DocumentOpenPlanDecodedSecretV1` before parsing; its `Drop` wipes every error exit, while the
successful transfer to the boxed live capability clears the stack copy (`📦️bin.rs:831-900`). The
new direct observer law supplies an invalid final character after 31 decoded bytes and observes
`nonzero_before: 31` plus a zeroed post-drop candidate (`:5963-5971`). This is a real secret-wipe
repair, not a diagnostic-only assertion.

It remains runtime-pending because neither current gate exact-selects that new law: the main
all-feature list includes schema, ledger, revocation, and exchange only
(`📜️script.ts:2127-2136`), and the default-feature subset likewise lists the three hub laws only
(`:2157-2164`). The pre-existing ledger law has no wipe observer. Add the new exact law before
claiming a registered D1 wipe terminal.

## Final D1 cutover reread — prior selection and generated-launch holds superseded

The preceding wipe-selection hold is **superseded in current source**. Both
`OpenPlanCheckScript` and the explicitly labelled default-feature
`OpenPlanServerCheckScript` exact-list, require exactly one fully qualified
match, and exact-run the same four hub laws: ledger, revocation, receipt
exchange, and `document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes`
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2244-2276,2279-2305`). The latter law
still observes a 31-byte partially decoded hostile receipt candidate followed
by an all-zero post-drop buffer (`📦️bin.rs:5971-5983`). This is a non-vacuous
source registration; no Cargo result is claimed by this audit.

The generated launch file now contains the same two registered uncached NX
commands and ordering as the seed: `open-plan-check` at `411.108` and
`open-plan-server-check` at `411.109`
(`.vscode/🧩️launch.seed.jsonc:3085-3105`, `.vscode/launch.json:4433-4453`; the
project target is `🌎️hub/📦️packages/🦀️rust/📋️project.json:119-125`). Thus the
old seed/generated divergence is source-closed. Freshness runtime evidence
must come from a terminal generation/check command, not this byte comparison.

`SocketGrantRecordV1` now retains the private document-plan authority as an
`Arc` and passes it atomically from private exchange issue into the stored
record (`📦️bin.rs:548-651,1139-1177`). The ledger compares that authority on
consume, live registration, and every live-authority check
(`:675-743`), while the D1 law reads the pending record and proves the exact
authority equals the plan authority (`:5928-5939`). This closes the former
identity propagation gap without exposing the authority in the public receipt.

The only opposite lock direction is private exchange's plan-ledger mutex then
socket-ledger issue. Revocation callers first complete socket-ledger
invalidation and only then take the plan ledger (`:1789-1802,3504-3526,4068-4097`);
neither invalidator retains its own ledger mutex after it returns. Therefore
the current code does not form a nested reverse plan/socket mutex cycle.
The binding admission guard remains deliberately held around the durable
revocation and sequential invalidations, preserving revocation-versus-dial
ordering. This conclusion is source-qualified; runtime still requires a fresh
registered terminal.

## D1 activation drift — RED

The current production router now mounts
`POST /spaces/{space_id}/documents/{id}/socket-grants` on
`issue_document_plan_socket_grant` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:4785-4790`).
That handler parses the receipt intent, authenticates and revalidates the
session/share subject, takes the binding admission gate, compares the durable
descriptor, and invokes the plan-to-socket exchange (`:1892-1960`). The
handler is therefore not a test-only helper.

The readiness shape has separately gained the granular
`open_plan_exchange: true` while retaining `open_plan: false`
(`:1629-1659`). That honestly distinguishes the exchange endpoint from a
complete open-plan feature. The independently executed pre-Cargo oracle used
by *both* registered D1 gates, however, still rejects any production
`DocumentPlanSocketGrantIntentV1` occurrence (`📜️script.ts:2197-2202`). It
will now throw before law selection, even though its path-string check misses
the newly mounted `socket-grants` spelling. No gate terminal can be credited
until the activation-positive/no-catalog contract and oracle are made
coherent.

There is also no non-test production caller of `DocumentOpenPlanLedgerV1`'s
issuance method in the hub binary: the route is an exchange consumer only.
Consequently, the new public surface is currently neither readiness-advertised
nor supplied by a production plan issuer. This is a fail-closed/inert path in
the present tree, but it is still an externally mounted contract and cannot be
described as the prior source-first, non-activating D1 scope.

### Selector drift — independently blocking

The current exchange test has been renamed to
`document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant`
(`📦️bin.rs:6079`) to match the now-mounted route. Both D1 gates still
exact-select the obsolete suffix ending in `_without_route`
(`📜️script.ts:2269,2300`). Their exact-one preflight will therefore select
zero even if the preceding oracle is made activation-positive. This is a
separate gate-registration RED and must be repaired together with the oracle;
no D1 terminal is claimed here.

## Current D1 exchange-only correction — source-qualified

The immediately preceding activation/selector REDs are superseded in the
current tree. The independent Node oracle now rejects only literal
`open-plan` routes, requires exactly one mounted
`/spaces/{space_id}/documents/{id}/socket-grants` route, and positively
requires the production intent, authenticated authority lookup, exchange
handler, and truthful `open_plan: false, open_plan_exchange: true` readiness
declaration (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2194-2210`). This correctly
describes the bounded exchange-only surface while retaining the absence of a
catalog-backed plan issuer.

Both registered gates now exact-preflight and run six laws rather than the
obsolete four: schema, ledger, revocation, receipt-to-grant exchange, mounted
authenticated route, and late-invalid wipe
(`📜️script.ts:2272-2307,2310-2337`). Each selection requires exactly one FQN
before execution. The former `_without_route` suffix no longer appears.

Source review finds no new D1 ownership regression. The plan-ledger mutex
holds through socket-grant issuance, so concurrent exchanges have one winner;
the grant expiry is the minimum of socket TTL, plan expiry, and binding expiry
(`📦️bin.rs:1126-1208`). A full socket ledger leaves the plan Issued for retry,
which the direct law checks (`:6152-6161`). The private plan `Arc` remains part
of SocketGrant record equality for consume, live registration, and liveness
(`:677-745`). Revocation sequentially returns from socket invalidation before
taking the plan ledger, avoiding a reverse nested mutex cycle. These are
source-only conclusions; no new Cargo terminal is asserted here.

## Final D1 exchange-only boundary — bounded ACCEPT

The preceding statement that both registered gates select six laws is now
superseded. Current `OpenPlanCheckScript` has **six** exact selectors: the
kernel schema law under `--all-features`, plus five hub laws (ledger,
revocation, atomic receipt exchange, hostile mounted route, and late-invalid
wipe). Current `OpenPlanServerCheckScript` has **five** exact selectors: only
those five hub laws under the hub's default feature set. Both list, require one
fully-qualified `: test` result per suffix, print it, and invoke its full name
with `--exact`; the server subset expressly says it cannot qualify the kernel
all-feature graph (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2290-2355`). Thus the
five-law server run is non-vacuous and is not presented as a substitute for
the separately red D0 kernel preflight.

The source keeps the production surface correctly bounded: it mounts exactly
one authenticated `POST /spaces/{space_id}/documents/{id}/socket-grants`,
rejects malformed/query/bounded-body requests, authenticates and revalidates
the caller under its binding gate, checks the current descriptor, and invokes
the atomic exchange (`📦️bin.rs:1893-1966,4790-4793`). Readiness reports
`openPlan=false` and `openPlanExchange=true` (`:1626-1660`); no issuance route
or verified catalog provider is implied.

Coordinator-recorded final session `79424` exited `0`: independent oracle and
production-parser parity completed with 16 general and 5 exchange hostile
negatives, then all five exact default-feature hub laws selected and passed
(`📓️sol-document-open-plan-foundation.md:89-111`). Session `63149` separately
verified generated launch/catalog freshness. I did not execute either command.
This accepts only the D1 exchange-only server boundary. It does not clear the
kernel/all-feature D0 preflight, create a public plan issuer, or establish
browser/native/WGPU/MCP acquisition or transport.
