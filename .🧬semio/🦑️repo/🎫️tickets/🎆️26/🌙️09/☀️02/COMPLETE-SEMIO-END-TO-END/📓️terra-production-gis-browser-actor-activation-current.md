# Production GIS Browser-Actor Activation Frontier

## Verdict

The shortest coherent production slice is now available, but no production path calls it yet:
after the Hub has authenticated the WebSocket `Session`, the private reservation must acquire the
single selected `browser-actor` body, load it into its already-reserved dedicated Worker, and prove
the real GIS `describe.describe` export.  This is a byte-activation proof only.  It must not expose
an actor body URL, host bridge, renderer, Map mutation, Store journal, or plugin view state.

The existing `renderer-unavailable` result is therefore still accurate.  The current reservation
law expressly requires `body-fetch=0 actor-load=0` and uses a mocked Worker; the Chromium child law
uses synthetic actor modules.  Neither is a GIS activation acceptance.

## Reusable authority and ownership

| Need | Existing closed owner | Required use |
| --- | --- | --- |
| Current document/scope/selection | `DocumentExecutionTargetLease` and `documentBrowserActorLease` in [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:539) and [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:882) | Preserve the actual lease object, not its public clone.  Its full `sameLeaseFieldsV1` relation already binds the actor identity, descriptor/component source hashes, actor digest, interfaces, policy and selected length. |
| Session admission | `requestDocumentSocketAuthority` gets the plan receipt grant and calls `lease.admitBrowserActor` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:991) | Do not load on grant issue.  `handleHubFrame` accepts the exact server `Session.actor` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1832); activation belongs immediately after that exact comparison, while `hubActorReady` is true. |
| One document actor slot | `reserveDocumentBrowserActorChild` and `DocumentBrowserActorReservation` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:841) | Retain this exact reservation.  It already owns doc/lease abort listeners, the plan-expiry timer, monotonic generation, child capacity, and identity after reserve. |
| Server-selected actor body | Four-member fixed route is in [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:620); Hub re-authenticates, resolves the current selection, then revalidates subject/revision at [bin.rs](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2447) and [bin.rs](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2536). | Request only `browser-actor` with the internally reconstructed exact `DocumentOpenIntentV1` (scope, selected surface, retained client instance).  It must never accept a package, digest, path, generation, receipt, module URL, or request-provided actor row. |
| Bounded private body transfer | `readBoundedExecutionTargetBody` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:649) | Reuse it with `fields.browserActor.byteLength` and schema-owned `DOCUMENT_BROWSER_ACTOR_MAX_BYTES`, after generalizing the progress stage away from the current `"component" | "descriptor"` union at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:736). |
| Dedicated activation | `BrowserActorChild.load` and `invoke` at [child/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:73) | `load` first consumes the caller ArrayBuffer, hashes it, then transfers it once; the child rehashes and activates with only actor id/generation and its local host/WASI port. |
| Real descriptor oracle | actual plugin `describe` blank hashes and staged descriptor receipts, described in [terra-real-gis-jco-describe-child-chromium-proof.md](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-real-gis-jco-describe-child-chromium-proof.md) | Call `child.invoke(["describe", "describe"], [])`; canonical-decode the returned Pack and compare it to the staged descriptor after blanking all three `hashes.{wasmSha256,coreWasmSha256,descriptorSha256}` fields.  Do **not** compare returned bytes with `descriptor.semio` byte-for-byte. |

The typed actor contract is strict and sufficient for this slice:
`DocumentClosedBrowserActorV1` accepts only the exact policy/schema and source package hashes in
[browser-actor/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:13), and its lease projection adds the independently selected actor body length at [browser-actor/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:88).  The Hub `BrowserActor` route returns only the retained Arc bytes matching that selection at [bin.rs](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2562).

## Minimal private API shape

Keep all of this module-private in `backbone-worker.ts`.

`DocumentBrowserActorReservation` needs three methods, not a public child accessor:

1. `assert_active_session()` verifies, after every await, its exact state slot, private lease object,
   grant object, runtime key/scope/origin selection via `documentBrowserActorLease`, live
   `hubActorReady`, `state.actor === grant.actorId`, `Date.now() < reserveBeforeMs`, and
   `Date.now() < retireAtMs`.
2. `activate_after_session()` reconstructs the fixed open intent from private state/binding, obtains
   the closed route response, uses the bounded reader, SHA-256 checks its owned bytes against the
   private actor lease field, transfers the one resulting `ArrayBuffer` to `child.load`, then makes
   the no-argument real `describe.describe` call.
3. `close()` remains the sole terminal path.  Any fetch/read/hash/load/describe rejection first
   zeroes an untransferred `Uint8Array`, then closes the reservation/child.  The existing listener
   removal and child-before-lease-wipe ordering at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:868) are the correct terminal base.

`handleHubFrame` is already asynchronous, so after setting `hubActorReady` at
[backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1848), it can invoke one
private `activateDocumentBrowserActorAfterSession(state)`.  It must not await it while holding any
transport lock; capture only `state`, then on completion re-run the reservation's assertion.  A
failure closes the socket/lease or remains `renderer-unavailable` with a private failure status; it
does not publish child results.  A Session mismatch already closes the socket, and socket close
already calls `dropDocumentExecutionTargetLease` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1431), which reaches the reservation.

This begins no Map effect bridge.  The child worker keeps document/network/storage effects denied at
[worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:20); its local
clock and bounded diagnostic output are adequate only because `describe` needs no admitted
document operation.  A later `reactor` bridge must wait for the typed per-document Store/DB owner.

## Current bounded reader review

The root implementation now addresses the previously reported pending-read ownership bug:

- it races every `reader.read()` against one abort/deadline rejection, keeps a `pending|claimed|abandoned`
  slot, and wipes a late abandoned chunk ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:674));
- it invokes `reader.cancel()` at most once without awaiting an untrusted underlying cancel promise,
  zeroes owned/chunk bytes, removes signal/timer, and releases the reader in `finally`
  ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:713));
- its source law builds a never-settling `cancel()` and checks rejection, unlocked stream, and source
  chunk wipe ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5060)).

This is coherent for the actor extension: Stream cancellation closes pending read requests before
`releaseLock`, so waiting for an adversarial underlying `cancel()` would be the wrong ownership
rule.  I did not run this test.  The only activation-relevant implementation delta is to permit the
actor progress discriminant or call the lower reader directly; do not copy a separate reader.

One bounded compatibility decision is still required for the real `describe` law: a selected raw
descriptor may be up to 4 MiB (`DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES`), whereas an
individual child result is capped at 1 MiB (`BROWSER_ACTOR_CHILD_LIMITS.outputBytes`).  The
activation must either reject a selected descriptor above the child result budget before it fetches
the actor, or raise the dedicated child result budget with corresponding aggregate admission.  Do
not silently truncate the `describe` Pack.  The fresh GIS output may be smaller, but that is not a
general source contract.

## First real acceptance gate

Add one registered sibling to the existing Chromium child containment target, rather than extending
the synthetic `BrowserActorChildWorkerContainmentCheckScript` at
[Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3943).  The new gate must first run the actual fresh
GIS materializer (`materializeTrustedStdioGisBundle` derives and stages the actor at
[Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5445)), start the selected SQLite Hub with that exact
generation, issue a real authenticated document open for `s.gis.gismap`, and run a real Chromium
page through Backbone.

Required observations:

1. The page obtains plan, manifest, component, descriptor and socket grant through the broker; Hub
   accepts a real `Session` before the one actor request.
2. The actor request is exactly the protected document `browser-actor` endpoint; the response length
   and SHA equal the installed private lease; no direct generation/staging file or module URL is used.
3. Parent transfer detaches the source, child rehash/load succeeds, and real
   `describe.describe([])` returns canonical bounded Pack bytes that satisfy the normalised staged
   descriptor oracle.
4. Closing the document while the actor body read or invocation is pending terminates the child,
   rejects the operation, removes its runtime-key reservation, leaves no output/MutationEnvelope/
   outbox/host-effect, and returns `browserActorChildCapacity()` to `{ actors: 0, bytes: 0 }`.
5. A same-document different-space page, a stale/revoked session, substituted body digest/length,
   and noncanonical/wrong-normalised `describe` output all deny before a ready status.

This proof is intentionally not a renderer, persistence, undo, MCP, inference, or collaborator
acceptance.  It is the smallest non-fixture production evidence required before exposing any actor
activation result to Shell.

No product edits or builds were performed for this audit.

## 2026-09-06 Addendum: Open-owner finalizer and a no-Hub-build real GIS gate

### Rechecked current ownership finalizer

The stale-owner cleanup observation is resolved in the current source.  In
`requestDocumentSocketAuthority`, [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:959)
keeps `published` false until the final `assertExecutionTargetRead(grantControl)` succeeds.  Its
`finally` then clears `state.executionTargetLease` only if that field is still the exact local
lease, and drops the local lease at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1011).
Thus a synchronous status observer which re-enters and changes the owner between status emission
and the final assertion cannot retain the stale lease in state.  The parent reported the revised
ownership gate `17784` GREEN; this audit did not execute it.

An extra observer-mutation row would document this precise regression, but it is not a remaining
authority bypass.  I found no additional concrete race, byte-retention, or route-widening defect
in `captureDocumentOpenOwner`, `readBoundedExecutionTargetBody`,
`installDocumentExecutionTargetLease`, or `requestDocumentSocketAuthority`.

### First genuine GIS proof without waiting for the Hub/Stdio native binary

The final Hub/SQLite/Chromium acceptance above remains required.  It should be preceded by a
separate registered **materializer-to-Chromium** gate so a long `os-hub`/Stdio build cannot conceal
whether the real GIS component can be closed and described.

The new gate belongs beside the existing Hub script Chromium harness, but must not invoke
`runExactCargoLaws`, start a Hub server, or call `materializeTrustedStdioGisBundle`.  It should call
the already public fresh owner directly:

```ts
produceFreshComponentV1(repoRoot, {
  pluginId: "gis",
  cargoPackage: "semio-s-plugin-gis",
  componentPackageId: "semio:gis",
  outputName: "semio_s_plugin_gis.wasm",
  componentProfile: "wasm-release",
  rootCdylib: true,
}, isolatedTarget, isolatedStage, control, async lease => {
  const actor = await buildClosedBrowserActorArtifactV1(lease.componentBytes(), ...);
  return lease.consume(/* stage receipt plus actor only */);
})
```

The exact fresh request is enforced by
[plugin describe script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:417).
It builds the GIS guest into an isolated target, captures the component and extracted core before
transpilation/emission, checks the required `checkpoint`, `describe`, `jobs`, and `reactor`
exports at [the same script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:449),
and requires the derivation callback before its private raw owner retires.  The callback must
require the derived actor's component digest to equal the fresh receipt; it must not accept any
peer-built component path or a pre-existing target file.

Feed only those derived verified actor bytes to `reserveBrowserActorChild`, `load`, and
`invoke(["describe", "describe"], [])` in the existing real Chromium static-child harness.  The
oracle is the staged descriptor with exactly
`hashes.wasmSha256`, `hashes.coreWasmSha256`, and `hashes.descriptorSha256` blanked, then canonical
Pack-compared to the returned value.  The component guest intentionally reports those fields blank;
comparing raw descriptor bytes is false-negative.  Require source buffer detachment, child rehash,
close, and final capacity `{ actors: 0, bytes: 0 }`; deny host document/network/storage effects and
require no output/outbox mutation.

This proves a real freshly materialized GIS guest under the closed dedicated-Worker runtime.  It is
not authenticated Hub routing, a renderer, a Map mutation, persistence, undo, MCP, inference, or
collaboration proof.  The later selected-Hub acceptance must retain all five route/session/revocation
observations listed above.

The gate must make the descriptor-result budget explicit: selected descriptor input allows 4 MiB,
while `BrowserActorChild` caps one invocation result at 1 MiB.  Pre-deny a staged descriptor above
the output limit, or deliberately raise that dedicated child limit with aggregate-capacity laws;
never truncate a real `describe` Pack.

## 2026-09-06 Addendum: Minimal Private Backbone Activation

The current source has all required lower-level pieces but intentionally has no activation call:
`requestDocumentSocketAuthority` installs the verified lease and emits
`renderer-unavailable` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:995),
while the exact authenticated `Session` branch finishes at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1836).
`reserveDocumentBrowserActorChild` currently reserves only; its own law correctly records
`body-fetch=0 actor-load=0`.

The smallest production slice belongs entirely in that private lease region:

1. After exact `Session.actor === pendingSocketActorId`, call one module-private
   `activateReservedDocumentBrowserActor(state)`. It must be single-flight through the already
   stored `DocumentBrowserActorReservation`; no child accessor, result, receipt, URL, or bytes may
   cross the worker boundary.
2. The reservation reconstructs the fixed `DocumentOpenIntentV1` from its captured state/binding,
   calls `browserExecutionTargetAssetRequest(..., "browser-actor", ...)` at
   [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:593),
   and uses `readBoundedExecutionTargetBody` with the exact private
   `fields.browserActor.byteLength`. Generalise only the existing progress discriminant at
   [directory schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1396)
   to include `browser-actor`; do not create a second reader or a generic asset URL API.
3. After every await, assert the same reservation object, live lease, identical grant object,
   runtime key/scope/selection, `hubActorReady`, `state.actor === grant.actorId`, and both grant
   deadlines. SHA-256 must equal `fields.browserActor.sha256` before transferring the sole
   `ArrayBuffer` to `child.load`; no array-buffer alias remains at the parent after transfer.
4. Keep the private staged descriptor buffer in `DocumentExecutionTargetLease`. Invoke only
   `child.invoke(["describe", "describe"], [])`, then call the existing canonical normaliser
   (`verifyBrowserActorDescribeV1`) against that exact retained descriptor. Do not expose the
   descriptor or compare raw final descriptor bytes.
5. In every fetch/read/hash/load/describe failure, zero any untransferred body/result and close
   the reservation. `close()` already aborts, terminates the child, clears the state slot, and lets
   the lease wipe descriptor/component bytes at
   [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:844).

There is one explicit preflight: the selected descriptor can be 4 MiB, whereas child invocation
output is 1 MiB including framing. Before fetching the actor, reject when the private descriptor
cannot fit `assertBrowserActorDescribeCapacityV1`; never truncate a `describe` result. This is a
closed-actor capability constraint, not a reason to widen output budget implicitly.

The first production-source test needs a closed-actor row whose mock broker returns the exact
existing manifest/component/descriptor/socket grant plus an actor body. It must prove: no actor
request before exact Session; only the fixed `browser-actor` route after it; body SHA/length and
normalised `describe` acceptance; close/replacement/expiry while body read or invocation is pending
causes child termination, one detached source wipe, state-slot removal, zero child capacity, and no
posted actor bytes, renderer-ready status, host effect, outbox, or mutation. A same-document,
other-space row must retain its independent reservation. The real materializer-to-Chromium gate
continues to provide the non-synthetic GIS guest proof; this source row must not substitute a fixture
actor for that acceptance.

Success still leaves `renderer-unavailable` truthful: this slice proves only authenticated private
body activation and `describe`, not a renderer, document command bridge, Store/WAL sink, Map edit,
undo, MCP, AI, or peer-visible effect.
