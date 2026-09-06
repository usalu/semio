# Native Lease Provenance And Backbone Child Owner

Status: read-only current-source audit on 2026-09-06. No native or browser command was run here. Root reported `native identity` receipt `59496/TUOA0T` green with 17 laws; that is not independently re-executed in this audit.

## Native retained-lease correction

The self-matching defect is fixed at the right authority boundary. `DocumentSocketAuthorityV1` now has crate-private `admitted_lease` ([directory client](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:416)). `matches_lease_fields` first requires complete equality between the retained admission and the caller candidate, then re-projects the received authority at the **retained** component, descriptor, and actor lengths ([same file](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:465)). A candidate can no longer choose its own lengths to make a changed plan appear equal.

### Provenance through the actual native path

1. `ArtifactHost::set_document_execution_target_lease` accepts one full lease only before the Hub document is opened, keyed by exact `{spaceId,documentId}`; opening removes the one-shot entry and injects it into that actor ([Store sync](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1129), [open handoff](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1149)).
2. `ArtifactActor::start_connect_hub` copies that exact actor-held lease into `DocumentSocketExpectationV1` ([sync](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1977)).
3. `DirectoryClient::admit_document_socket` validates a fresh protected plan against that expectation before the receipt exchange and retains a clone in the returned in-memory authority ([client](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1135), [retention](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1168)).
4. After the real WebSocket opens, `finish_connect_hub` checks hub origin, scope, schema, codec hash, surface, expiry, and the complete retained lease relation **before** it stores the socket ([sync](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2017)). A mismatch closes the stream and schedules reconnect.

The only source literals of `DocumentSocketAuthorityV1` are the production client and the Store sync test source; the latter must set the new crate-private field explicitly ([client](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1168), [sync fixture source](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:5137)). There is no serialized or public constructor migration.

There is one API-coherence consequence: `HubSocketGrantSource` and `ArtifactHost::set_hub_socket_grant_source` are public ([trait](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:492), [setter](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1125)), while an external trait implementation cannot construct the returned authority with its crate-private `admitted_lease`. Do not reopen that field. Either make the source an internal/sealed integration seam, or add a public **constructor** that takes `DocumentSocketExpectationV1` plus receipt-free authority fields and itself copies the expectation lease. The latter is the smallest coherent public contract: a source never controls the retained admission, and the Store test fake can exercise omission/mismatch only through an explicit hostile constructor mode inside the crate.

### Precise no-lease behavior

`authority.matches_lease_fields(expected)` is fail-closed: `admitted_lease: None` returns false; the native directory unit already asserts that fact ([client law](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:2358)). Consequently, where the Store actor holds a lease, a custom/malformed `HubSocketGrantSource` that omits it is rejected after connection setup and never becomes `semio_hub`.

There is intentionally a separate **sync-only no-lease** branch: both initial plan matching and post-WebSocket matching use `Option::is_some_and`, so a native actor without a locally installed execution target may authenticate and replicate a Hub document ([client plan branch](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1145), [sync branch](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2023)). That does not currently fetch or activate a browser actor. It must remain a documented non-renderer/sync capability, rather than being mislabeled as a full target admission.

Do **not** globally reject a `closed-browser-actor` plan merely because a headless/native synchronizer has no browser lease: that would conflate browser code execution with authenticated document replication. The safe invariant is narrower: any browser body fetch, child reservation, child invocation, or UI actor-ready state requires the private live lease. The upcoming Backbone seam should enforce that invariant.

### Missing focused native law

Add one Store sync law using its existing in-crate `HubSocketGrantSource` fake:

`native_sync_rejects_socket_authority_without_the_locally_admitted_full_lease`

Give the actor a valid local lease, return a syntactically valid authority with `admitted_lease: None`, and drive `finish_connect_hub`. Assert the stream was closed, `semio_hub` and `socket_authority` remain `None`, no `Welcome`/outbox flush runs, and reconnect is scheduled. A sibling row should alter only `component.byte_length` in the returned authority while the retained lease is unchanged. This is the call-site proof missing from the direct relation corpus.

## Private Backbone owner before any actor body route

The existing browser state has the right initial authority owner but no child slot: `ArtifactState.executionTargetLease` is private and keyed by `runtimeKey = documentRuntimeKeyV1(scope)` ([Backbone](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:222)); its lease owns verified component and descriptor bytes and is wiped on close/reconnect ([lease](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:537), [cleanup](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:772)). A verified non-React target is currently only retained and reported as `renderer-unavailable` after receipt exchange ([request path](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:805), [status](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:865)).

The static child is suitable as a contained executor foundation, but its exported reservation is deliberately not catalog authority: it accepts caller-shaped `{actorId, activationGeneration, bundleSha256, bundleByteLength}` ([child](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:16)). It must therefore be invoked only by Backbone’s private retained lease owner, never from Shell/plugin state or a route parameter.

### Smallest ownership seam

Add a private `browserActorChild` slot to `ArtifactState`, with a private lifecycle record conceptually:

```ts
type BrowserActorReservation =
  | { phase: "reserving"; lease: DocumentExecutionTargetLease; generation: bigint }
  | { phase: "reserved"; lease: DocumentExecutionTargetLease; generation: bigint; child: BrowserActorChild };
```

Keep a module-private monotonic `Map<string, bigint>` by `runtimeKey`; increment before every reservation and do not reset it on a close/reopen. `runtimeKey` includes space and document, so same document ids in different spaces cannot share a child owner. The child’s existing nonce plus this never-reused generation fences late port messages; no actor identifier, policy, receipt, origin, or broker crosses its `MessagePort`.

After all of the following succeed:

1. parsed plan and installed target pass `documentOpenPlanAuthority`;
2. component/descriptor are verified and a live `DocumentExecutionTargetLease` exists;
3. the plan-receipt → socket-grant exchange has succeeded and expiry is rechecked; and
4. `lease.fields().browserActor.kind === "closed-browser-actor"`,

reserve the child **before** introducing any actor-body request. Its tuple must be derived only from the lease: `bundleSha256 = browserActor.sha256`, `bundleByteLength = browserActor.byteLength`, and a private generation. `actorId` is a non-authoritative local label; use the exact runtime-key owner plus generation (or a private digest thereof), not a caller-selected actor/UI id. Reserve only after the grant succeeds: reserving earlier gives an ungranted open-plan request an avoidable fixed-child capacity DoS.

Before storing the awaited reservation, recheck all four identity anchors:

```ts
!state.closed
&& artifacts.get(state.runtimeKey) === state
&& state.executionTargetLease === lease
&& lease.live
&& state.browserActorReservation?.generation === generation
```

Otherwise close the returned child and treat the open as cancelled/stale. That is the necessary one-runtime-key single-flight fence; it also prevents a reconnect or close from publishing an old reservation after its await.

Extend the current one cleanup helper so every lease invalidation first closes the exact child, then drops/wipes the lease. The call sites are document close ([Backbone](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2892)), socket invalidation ([same](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1303)), and the next plan attempt ([same](../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:830)). The child already responds to the document abort signal and releases fixed capacity in `close`; the explicit parent cleanup is still required to eliminate the owner slot before a new generation is admitted.

No actor body should be fetched or loaded in this slice. The later private body fetch must require the `reserved` record and recheck the same lease/generation before and after every await, then transfer its exact verified `ArrayBuffer` only to that child’s `load`. Do not surface `BrowserActorChild`, `DocumentExecutionTargetLease`, bundle bytes, or a module URL through `BackboneWorkerResponse`, Shell state, or plugin `viewState`.

### Minimum executable laws

1. A valid closed actor plan/grant creates exactly one reservation for its exact `{runtimeKey,generation,sha256,length}`, but performs no actor-body request or load.
2. A `none` actor creates no child; a closed actor with no live private lease creates no child and no body request.
3. An expired/denied socket grant after a valid plan creates no child and restores child capacity to zero.
4. Close or abort while `reserveBrowserActorChild` awaits closes its result on arrival; capacity returns to zero and no stale state is published.
5. Reconnect after invalidation uses a strictly greater generation; a late old generation/nonce cannot affect the new document state.
6. Same document id in two spaces gets two distinct runtime keys and independent reservations; closing one does not close or publish into the other.
7. Reservation capacity/factory failure produces only local stale/unavailable status, no actor-body route request, no socket actor-ready state, and no retained lease/child.
8. Existing child malformed/duplicate/wrong-generation message tests are exercised through this state owner and prove no Backbone status/event/outbox mutation follows.

## Nonclaim

The reported Chromium child fixture proves fixed-worker containment behavior only. It is not mounted to Backbone today; this proposal deliberately stops at a private lease-bound child reservation. It does not fetch actor bytes, load/activate a component, expose a renderer, or make an execution-target route/worker/catalog acceptance claim.
