# Backbone Session-Bound Browser Actor Activation

## Current seam

The lower pieces are present, but no production call currently activates a reserved child:

- The verified private lease is published only after the socket-grant exchange at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1177).  It retains the actor identity, descriptor bytes, component bytes and a single private `DocumentBrowserActorGrant` ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:608).
- An exact `Session.actor` sets `hubActorReady` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2032), but does not start the actor.
- `reserveDocumentBrowserActorChild` only reserves a `DocumentBrowserActorReservation` at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1012).  Its existing neutral corpus expressly asserts zero body requests and zero loads.
- The close path is sound as the terminal base: socket close drops the lease at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1621), and lease drop aborts the reservation before wiping retained bytes ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:656; [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:979)).

The smallest activation remains module-private.  `handleHubFrame` must capture the *current WebSocket object* when it accepts the exact Session, then hand that object to one `DocumentBrowserActorReservation.activate_after_session(socket, binding, intent)` promise.  `connectHubOnce` already has the actual socket at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1573), while the current frame handler only receives an optional presence object; make the socket explicit rather than inferring it from an actor id.

## Retained-owner contract

`DocumentBrowserActorReservation` should retain exactly one activation promise and no public child/byte accessor.  Before issuing the body request, and after every await (response headers, body EOF, SHA-256, `child.load`, and `child.invoke`), its private assertion must require all of:

1. `state.browserActorReservation === this`, the private lease object is still `state.executionTargetLease`, and `documentBrowserActorLease(state) === this.lease`.
2. The captured socket is `state.socket`; the state is not closed/aborted; `hubActorReady` is true; and `state.actor === grant.actorId`.  This is the fence absent from a scope-only check when a reconnect replaces a socket.
3. The current runtime key, hub scope/origin/schema/installed selection still match, the grant object is identical, and `now < reserveBeforeMs` and `now < retireAtMs`.

The fixed `browser-actor` route already reconstructs a document-scoped path and accepts only a bounded `DocumentOpenIntentV1` ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:685)).  Reuse it with the original private intent and `DocumentExecutionTargetBrowserActorV1.byteLength`; do not accept a URL, path, digest, package, receipt, generation, or caller actor selection.

Use the existing reader with the actor's exact expected length and `DOCUMENT_BROWSER_ACTOR_MAX_BYTES` ([browser-actor/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:2)).  It already rechecks ownership at every chunk, cancels without awaiting an adversarial underlying cancellation, wipes chunks, and releases the reader ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:724)).  SHA-256 must equal the retained lease actor identity before the one `ArrayBuffer` crosses `child.load`; the child then rehashes the transferred bytes itself ([child/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:73)).

The staged descriptor cannot be returned by the lease: its `#descriptor` is deliberately class-private.  Add a lease-private `verify_browser_actor_describe(result)` operation that (a) enforces the describe result capacity before fetching the actor, (b) accepts only one exclusive `Uint8Array` result, and (c) calls `verifyBrowserActorDescribeV1(result, this.#descriptor, { decode: decodePackValue, encode: encodePackValue })`.  The verifier already requires canonical bytes and compares only after blanking the three emitter hashes ([describe/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts:21)).  It is incorrect to raw-compare the guest response with `descriptor.semio`.

The result must be wiped by the reservation in a `finally`.  On any body/hash/load/describe failure, it must close itself and then drop the exact lease/close the exact socket.  Closing the reservation alone clears its state slot but leaves the already-admitted grant reusable; that would permit a retry under the same exchanged authority.  No renderer/plugin route, actor result, host effect, mutation, or public status may be published before the final owner assertion and descriptor comparison.  The public status may remain `renderer-unavailable`: successful private `describe` is not a renderer claim.

## Minimal neutral corpus

Extend the existing OS fixture family with `semio.os.document-browser-actor-session-activation/v1`, reusing the authenticated document-open and child reservation fakes rather than adding an alternate actor transport.  The existing mismatched-Session test at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:5103) remains the wrong-actor row.  Add these six activation rows:

| Case | Required observation |
| --- | --- |
| `before-session` | Lease/grant exists, but no exact Session means zero actor body requests, child reservations, loads, describes, routes and capacity. |
| `valid-session` | One exact Session yields one fixed actor request, exact length/SHA, one transfer/load/invoke `describe.describe`, normalized descriptor acceptance, then cleanup gives `{actors:0, bytes:0}`. |
| `socket-close-during-body` | A parked actor stream is cancelled and wiped by socket close; no child load/describe/status publication survives, slot is null and capacity is zero. |
| `socket-replaced-during-load` | A's body has transferred and its `loaded` reply is parked.  Closing A and admitting Session B terminates A exactly once; a late A `loaded`/result cannot verify, route, clear B, or emit B state.  B alone owns its request and child. |
| `descriptor-mismatch` | Exact body and successful child load followed by noncanonical/wrong-normalized `describe` closes the exact child, lease and socket; no active/public renderer state, outbox, mutation, or host effect. |
| `same-document-other-space` | A close/replacement has no effect on B with the same document id under a different runtime key; B's reservation and descriptor comparison complete independently. |

Each row must retain assertions for: exact socket object; exact reservation/lease/grant identity; body/source/result wiping or transfer detachment; one child termination; no leaked reader; no actor bytes or lease/receipt posted; no duplicate route or result; and zero child capacity after every terminal.  A source fake alone is not the real-GIS acceptance: retain the separately registered fresh-GIS-to-Chromium `describe` gate for the actual guest.

No product edits or builds were performed for this audit.
