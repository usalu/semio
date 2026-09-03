# Terra Audit — Hub-Authoritative Document Open Plan

**Scope.** Read-only source audit on 2026-09-03. This is the smallest security and authority boundary shared by React and native after the session-foundation work. No build, test, or runtime launch was run; findings below are source-backed, not execution claims.

## Outcome and sharp finding

There is no authoritative open plan today. A client may announce a structurally-valid `DocumentDescriptor`, choose the local plugin/app/schema/surface to mount, then send client-controlled `schema`, `pack_schema_hash`, and `actor` in the document WebSocket `Hello`. The hub compares the first two values only with that stored descriptor, but promotes the client `actor` into the durable command/presence session. A valid session is therefore not enough to establish the document's executable identity or the actor that writes it.

The minimal safe seam is a server-derived, short-lived `DocumentOpenPlanV1`: an authenticated request names only a structural document scope and an optional *preference* for a surface. The hub authorizes its session, loads the immutable descriptor, resolves that descriptor through the verified trusted catalog, selects an allowed declared app/surface, derives the actor, and binds all of that to an opaque one-time plan receipt. The first WebSocket frame proves the current session plus receipt; it contains neither actor, schema, plugin, package, hash, nor writable-role input.

This can be implemented in parallel with loader/P2-C in its schema, directory, session, and shell-client layers. It cannot become a usable general open path until the trusted catalog is configured and able to resolve descriptor-owned app/surface metadata; current hub startup also has no configured identity verifier/local bootstrap and fails its mode validation.

## Current path, with evidence

| Boundary | Current implementation | Authority consequence |
|---|---|---|
| Space/document creation | `DirectoryCommand` admits client-supplied `announce-document` with the complete descriptor ([directory TS:133-145](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts), [directory Rust:1097-1183](../../../../../../../../../../../../🌎️hub/📇️directory/🦀️.rs)). | Descriptor fields are merely shape-validated; clients select its owner/package/schema. |
| Descriptor projection | Descriptor has owner `(pluginId, packageId, version, packageHash)`, scope, artifact schema, pack hash and bootstrap fields ([TS:191-222](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts), [Rust:293-399](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs)). It has a strict SHA-256 leaf encoding/digest, not JSON hashing ([TS:274-337](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts)). Projection validation checks nonempty fields/lowercase hashes and bootstrap shape only ([hub directory:761-793](../../../../../../../../../../../../🌎️hub/📇️directory/🦀️.rs)). | The digest is a reusable immutable identity, but neither catalog validation nor executable-app selection occurs. |
| REST selection | `POST /directory/commands` authorizes `AnnounceDocument` for an author and executes it with a server-formatted event actor ([hub bin:1313-1378](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). Space detail returns documents to public/member readers ([hub bin:1381-1430](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). No open-plan route is registered ([hub bin:1982-2010](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). | REST's event actor does not repair a malicious descriptor; catalogue-wide metadata can be exposed before opening is authorized. |
| Relay / document socket | The first `Hello` destructures client `schema`, `pack_schema_hash`, `actor`, `token`, and `frontier`; it compares schema/hash to the stored descriptor then retains client `actor` in color, principal, DB hello, server session, and sync session ([hub bin:928-1085](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). Subsequent commands use the client-frame actor ([hub bin:856-925](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). | **Critical actor spoofing and client selected executable identity.** Exact comparison is insufficient if the descriptor was client-announced. |
| Session/share rules | Session resolution checks active/revocation/membership; share capability is document-scoped and maps to spectator ([hub bin:399-457](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). Only an administrator creates/revokes shares ([hub bin:585-611](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). | The role is server-derived, but the socket still accepts client actor and query/Hello token transport. A share must never become a space enumeration, blob-read, or editor grant. |
| Auth transport/startup | Directory streams use `?token=` ([hub bin:1514-1608](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)); TS `mintSession` still posts obsolete `/auth/sessions` and `stream` also uses query token ([OS TS:3981-4094](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts)). Main selects `None` verifier and local bootstrap then validates mode; default non-loopback bind rejects production/dev setup ([hub bin:468-524](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs), [2116-2124](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). | Plan work must consume the secure-session replacement, not keep query credentials. Current default hub cannot be a valid end-to-end oracle. |
| Trusted catalog | A trusted authority is optional and only configured from bundle/profile environment; native codec bindings are currently empty and the result is stored but unused ([hub bin:150-167](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs), [2143-2147](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)). | No production component yet resolves descriptor owner to verified manifest app/surface choices. Do not invent this mapping in React/native. |
| React selection | `openDocument` resolves a local relay target and constructs hub/folder bindings with client `documentId`, `schema`, actor, and surface ([ShellHost:3337-3397](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx)); the URI path can open a hard-coded index/schema ([ShellHost:3259-3285](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx)). The worker puts client schema/hash/actor/token into `Hello` ([backbone worker:550-648](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts)). | Local navigation is a presentation preference only; it cannot authorize an opening or choose executable identity. |
| React P2-C | The worker has 15 s bootstrap/mutation bounds and abort/progress messages ([backbone worker:109-113](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts), [752-858](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts)); it reacts to rebootstrap by replacing the session ([ShellHost:1365-1484](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx)). | It verifies a pair but not an authority-issued descriptor/surface plan. |
| Native selection | Native constructs a local `shell_actor`, client persistence binding and surface ([Shell native:342-370](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs)); the relay accepts plugin/app/schema/document/space from the caller and defaults editor ([Shell native:417-533](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs)), then forwards them to `open_document` ([Shell native:3554-3584](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs), [Shell native:3907-3944](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs)). | Native has the same trust inversion. A native relay target is intent, not authority. |
| Native attachment | The WGPU `ProgramBridge` backbone attachment deliberately returns “retired … no effect backbone replacement” ([ProgramBridge:274-283](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs), [520-524](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs)). | Native cannot use a sound plan to attach a live backbone yet; this is a downstream native blocker, not a reason to loosen hub authority. |

The cited native path still describes per-surface presence in its relay area. The required invariant is instead a **document-wide** roster keyed by structural document scope and server-derived actor. `surface` is peer telemetry only: never a presence filter or authorization selector.

## Required schema-first contract

Add the cross-language schema beside the directory document schema, with strict bounded decoders in Rust and TypeScript. This is a greenfield replacement, not a compatibility layer.

```text
DocumentOpenIntentV1 {
  version: 1,
  scope: { spaceId, documentId },
  requestedSurfaceId?: CanonicalSurfaceId,   // preference only
  clientInstanceId: OpaqueId[1..128]         // diagnostics/rate limit only
}

DocumentOpenPlanV1 {
  version: 1,
  receipt: Opaque256BitId,
  expiresAtUnixMs: u64,                      // fixed <= 30 seconds
  scope: DocumentScope,
  descriptor: DocumentDescriptor,
  descriptorDigestV1: LowerHexSha256,
  package: { pluginId, packageId, version, packageHash },
  artifact: { kind, schema, packSchemaHash },
  surface: { surfaceId, appId, pluginId, mode: read | write },
  grant: { read: true, write: boolean },
  revalidation: { sessionGeneration, membershipGeneration, directoryRevision }
}

OpenSocketV1 { receipt: Opaque256BitId, credential: SessionOrShareCapability, resumeFrontier?: Frontier }
OpenGrantedV1 { actor: ServerDerivedActorId, plan: DocumentOpenPlanV1-without-receipt }
```

The client never supplies actor, role, plugin/package identity, artifact schema/hash, document descriptor, or a private checkpoint locator. A WebSocket path may carry structural scope for routing but must equal the receipt scope; remove `surface` and token query parameters. The credential goes in the first encrypted WebSocket control frame, is never logged/reflected, and is checked before the receipt is disclosed as usable. `receipt` is random, opaque, short-lived, single-use for an initial mount, and held only in a bounded in-memory registry keyed by a digest of it. Its server-side record binds session ID, session/membership revocation generations, document scope, descriptor digest, exact trusted package identity, selected surface, grant, expiry, and an optional resume nonce. A hub restart invalidates receipts and requires a fresh plan—safe and simple without another runtime dependency.

The server determines a stable actor from the authenticated subject and authenticated session (for example `user:{subject}#session:{session-id}`); it must overwrite/reject any actor field in command envelopes. An anonymous public plan, if public documents remain supported, gets a per-plan server-generated ephemeral actor and `write=false`; it cannot enumerate a space. A share plan is exact-document only, always `write=false`, and has no authority to list unrelated documents, read private blobs, or select an editor surface.

### Server decisions and data flow

1. Make descriptor publication server-only. Replace public `announce-document` with `CreateDocumentIntentV1 { spaceId, requestedArtifactKind, requestedSurfaceId? }`; client fields are selectors, not identities. For existing documents, do not accept a descriptor in an external command.
2. The directory command handler first derives principal/session/membership role and requires `author` to create. It asks a new bounded `OpenableDocumentCatalog` query implemented by the verified trusted catalog to resolve allowed artifact kind → canonical package/manifest app/surface/schema/pack hash. It allocates the document ID, materializes/validates initial checkpoint through the artifact authority, computes the strict descriptor digest, then emits server-derived `DocumentAnnounced`. Failure/cancellation leaves neither a directory event nor a visible document.
3. `POST /spaces/:space/documents/:document/open-plan` accepts only `DocumentOpenIntentV1` under the secure session capability. It revalidates session, membership/share scope and descriptor activity; resolves the **stored** descriptor against the trusted catalog; intersects its declared surfaces with the server-derived grant; selects requested surface only when it is in that intersection, otherwise selects the catalog default or returns a bounded typed error. It issues the receipt and plan. It must not leak which forbidden surfaces exist.
4. The document socket replaces `Hello` with `OpenSocketV1`. It resolves the current credential then registry record, checks expiry, all generation values, scope/digest/package/surface/grant, and derives its actor. Only then does it create color/presence/DB session. The hub writes the selected surface as non-authoritative telemetry. It gates every mutation on `plan.grant.write`, server actor, schema/hash from plan, and current session/membership state.
5. Both clients verify that a locally loaded renderer/package exactly matches the plan package/hash and selected app/surface before mount; mismatch is a fail-closed localized “required component unavailable” state, never a fallback to a client-picked implementation. The public `(pack,spr)` bootstrap is then validated against the plan descriptor/artifact fields before codec decode.

`DocumentDescriptor` itself already has enough immutable fields to start this; add `descriptorDigestV1` to the plan rather than changing the descriptor. Add a server catalog output type—do not make arbitrary package manifests or generated client registry rows an authority source. The current `VerifiedTrustedCatalog` needs a small query that exposes only verified descriptor-owned opening choices; current optional `_artifact_authority` does not offer or call such a query.

## Reconnect, P2-C and cancellation

The hub already closes lagging document sockets for rebootstrap ([hub bin:1139-1144](../../../../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs)); the shared directory stream type has `RebootstrapRequired` ([directory Rust:528-562](../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs)). Apply these rules:

- A reconnect obtains a new plan unless a still-live receipt passes all checks. Rebootstrap **always** fetches/revalidates a fresh plan before downloading a public pair or reconnecting the socket.
- Descriptor digest, package/schema/pack hash, granted surface/mode, session active state, membership generation, share active state, document/space deletion, and admin kick/revoke invalidate a receipt. A frontier/checkpoint change alone does not alter immutable descriptor identity, but all received P2-C controls must match the fresh plan scope and descriptor artifact fields.
- On invalidation, immediately close the socket, abort the current transfer, clear remote writable state, retain at most the existing bounded local pending work without replay, and re-admit it only after a fresh write plan. Do not turn a stale plan into a spectator plan silently.
- Plan generation/catalog resolution has a fixed 10 s deadline, a propagated cancellation signal, 8 KiB request/64 KiB response caps, one outstanding plan per session+scope, and a small bounded registry (for example 1,024 entries with expiry). Use typed `denied`, `stale`, `catalog-unavailable`, `component-unavailable`, `cancelled`, and `deadline-exceeded` errors; do not return storage/database error text.
- Preserve the worker's current bounded progress/abort flow, but label its transfer as authorized only after plan validation. Native needs the same deadline/cancel wiring rather than an unbounded directory refresh.

## Severity and first deterministic blockers

| Severity | Blocker / exact root cause | Required ordering |
|---|---|---|
| **Critical** | Client socket `Hello` actor flows into durable session/commands; client descriptor is accepted without trusted catalog resolution. | 1: schema plus server-derived plan/actor and remove external descriptor announcement. |
| **High** | The configured trusted catalog/authority is optional, no linked native codec bindings exist, and the authority is stored but unused. Thus no server can select a verified app/surface for a descriptor. | 2: bounded `OpenableDocumentCatalog` projection and hub wiring. This has a hard dependency on the catalog/loader completion packet, not on P2-C mechanics. |
| **High** | Current default hub startup provides neither an identity verifier nor a loopback bootstrap transport and rejects its selected mode; TS clients retain retired `/auth/sessions` and query credential flow. | 3: land secure-session foundation, protected local bootstrap, and first-frame auth before any end-to-end open test. |
| **High** | React/native currently turn local relay/URI/plugin data into schema/surface/actor binding; client control survives even if plan REST exists. | 4: refit both clients to plan-only opening. React can proceed immediately after contract; native transport adapter follows. |
| **High (native only)** | ProgramBridge deliberately has no effective backbone attachment. | 5: repair attachment before asserting native collaborative opening; do not bypass hub-plan check. |
| **Medium** | Space detail/document listing permits public detail and stream still uses query tokens; this exposes more topology/credentials than the intended plan. | Make listing visibility a server policy; route stream through session first-frame auth in the session packet. |
| **Medium** | P2-C verifies public artifact identity but no plan descriptor digest/grant. | Bind plan to bootstrap/rebootstrap controls and revoke/refresh on invalidation. |
| **Low** | Relay comments and UI imply a surface-scoped presence model. | Correct semantics while wiring document-wide roster; surface stays telemetry. |

The first deterministic failure chain for a local E2E attempt is currently: default hub launch selects a mode with `None` verifier/bootstrap → mode validation rejects startup → no authenticated open endpoint exists. If startup is externally configured, the next deterministic authority defect is `Hello` accepting caller actor after only client/stored schema/hash comparison.

## Ordered implementation packet

1. **Schema and fixtures (no loader dependency).** Add `DocumentOpenIntentV1`, `DocumentOpenPlanV1`, receipt/open socket control frames, typed error codes/limits, strict TS/Rust codecs, and language-neutral JSON fixtures under the directory schema fixture tree. Remove `actor`, schema/hash, token, and surface from `Hello`; make the old external `announce-document` schema unreachable.
2. **Session-bound plan registry and relay (secure-session dependency).** In hub session/directory/`bin.rs`, add authenticated open-plan route and bounded in-memory receipt registry; derive actor, check membership/share/revocation every mount, bind command writer to plan, replace query token/old `Hello`. Emit audit events: `DocumentOpenPlanned`, `DocumentOpenDenied`, `DocumentSocketMounted`, `DocumentPlanInvalidated` with opaque receipt digest/reason—not credentials or private locators.
3. **Verified catalog opening projection (catalog/loader dependency).** Extend verified trusted catalog, not generated registry or client manifest, with a fixed bounded lookup of stored `DocumentOwner` plus artifact kind/schema to canonical package and declared app/surface modes. Wire current hub `_artifact_authority` into creation/open-plan. Fail closed when the catalog, codec, descriptor, or allowed surface is absent. Integrate the distinct 64 MiB versus 496 KiB durable-store packet before creation claims large pair support.
4. **Server-derived document creation.** Replace REST directory descriptor submission with server command intent; catalog resolves identity and authority stages initial public checkpoint; only then persist server-built immutable descriptor. Ensure error/cancel produces no event/pair reference leak.
5. **React adapter.** Make ShellHost URI/cards/relay emit only structural open intent. Fetch a plan with secure session, ensure local package/app/surface exact match, pass receipt/credential control to worker, and map plan/rebootstrap states to accessible UI. Retire `mintSession`, token query and client `shellActorId` from hub binding.
6. **Native adapter after ProgramBridge repair.** Make `OpenArtifactRelayTarget` an intent only; replace local schema/default editor/actor with plan data; have the store attachment validate plan identity then use `OpenSocketV1`. Fix ProgramBridge attachment first. Keep native roster document-wide.
7. **P2-C, revoke, launch.** Tie receipt invalidation to checkpoint/rebootstrap and directory revocation/delete/kick streams; register protected dev bootstrap / production verifier configuration in launch profiles without a network-exposed mint endpoint.

## Acceptance and independent oracle packet

1. **Neutral fixtures.** One valid plan plus invalid cases: forged actor, foreign package, mismatching descriptor digest, unwanted editor surface, cross-space document, expired/reused receipt, revoked session/share, deleted scope, and stale membership generation. Decode the same fixtures in Rust and TypeScript; raw JSON alone must not be the descriptor identity oracle.
2. **Independent identity oracle.** Compare descriptor digest fixtures to a small independent SHA-256 implementation/reference (for example Node Web Crypto or system `sha256sum`) over the documented strict leaf byte stream; assert the Rust and TS decoders produce the identical lower-hex digest.
3. **Real-socket integration.** With an explicit loopback-only dev identity bootstrap and a configured trusted catalog: create two users in one space; plan/open two document-wide presences; attempt a second socket that submits user A's actor/plugin/schema and prove server records user B's derived actor and refuses mismatch. Use an external WebSocket client library already confined to tests for frame-level verification, in addition to in-process tests.
4. **Authorization oracle.** Admin creates a document share; independent spectator client may request exactly that read-only plan and see document-wide roster, but cannot list a foreign document, select a write-only/editor surface, mutate, fetch private CAS locators, or retain access after revoke/kick. Repeat across two spaces with identical IDs where applicable.
5. **Recovery oracle.** Force lag/rebootstrap, revoke between plan issuance and mount, expire/reuse receipt, restart hub, delete document, and corrupt `(pack,spr)`. Assert fresh plan/revalidation, bounded abort/progress, no writable stale replay, no credential/ref leak, and localized accessible error.
6. **UI oracle.** Browser accessibility test and native semantic-tree test assert status is announced with `role=status`/live region, progress has a readable label/value, cancel is keyboard reachable, and translation catalogs contain both English and German keys without a fallback/default-language assumption.

Focused commands to run **after** implementation and only when concurrent jobs permit (none were run for this audit):

```sh
bun nx run os-hub:test-quick -- open-plan
bun nx run os-hub-ts:test-quick -- open-plan
bun nx run @semio-tech/framework-os:test-quick -- open-plan
bun nx run @semio-tech/framework-renderer-react:test-quick -- open-plan
bun nx run @semio-tech/framework-renderer-wgpu:test-native -- open-plan
```

For the real socket oracle, use the existing hub launch target only after its secure verifier/bootstrap configuration is present; do not treat its current default launch as a passing prerequisite. Add the real process invocation to the existing `📜️script.ts`/`launch.json` machinery rather than a new ad hoc script.

## Privacy and authority invariants

- The hub, not a plugin, URI, relay, worker, or native bridge, derives subject, actor, scope, role, descriptor, package, schema, and allowed surface.
- Immutable descriptor digest and trusted package hash bind executable identity; public bootstrap pair integrity does not authorize it.
- A session/share is reevaluated at plan creation, mount, write, rebootstrap, and revocation. Receipts are scoped, short-lived, opaque, and never cross a space/document.
- Presence is document-wide for admitted actors. Surface remains non-authoritative peer telemetry and does not filter presence or grant capability.
- Denials, audit events, UI messages, and fixtures reveal no bearer, raw storage locator, or forbidden catalog topology.
