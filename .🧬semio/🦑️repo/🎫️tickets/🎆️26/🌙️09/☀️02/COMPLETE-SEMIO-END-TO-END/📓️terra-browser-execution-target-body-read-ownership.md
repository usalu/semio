# Execution-Target Body Stream Ownership Before Actor Acquisition

## Verdict

`readExecutionTargetBody` is not yet safe to reuse for browser-actor bytes.
It correctly bounds declared length and wipes its aggregate buffer on a thrown
read, but its cancellation and reader ownership end at response headers rather
than at EOF. This is a concrete P0 for the next actor-body handoff: a server
can return authenticated headers and then leave `reader.read()` pending; a
document close, lease retirement, or expired plan does not settle this body
operation today.

No source, browser, or native test was run for this audit.

## Current failure path

The shared `fetchWithTimeout` owns its timeout and supplied abort listener only
until `fetch()` resolves a `Response`; it removes both in `finally`
([`framework/🟦️.ts:347-364`](../../../../../../🧰️framework/📦️packages/🟦️typescript/🟦️.ts#L347-L364)).
That is correct for header acquisition, but it is not a body deadline.

`readExecutionTargetBody` subsequently gets a reader and awaits `reader.read()`
at [`backbone-worker.ts:678-707`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L678-L707):

* it tests `signal.aborted` only **before** the await, so an abort while that
  read is pending cannot reject the owner;
* it has no `finally { reader.releaseLock() }`, including on its successful
  `return`, so every accepted response remains locked;
* its error branch awaits `reader.cancel()` but also never releases the lock;
  an unresponsive cancel can make cleanup itself an unbounded await;
* an over-cap chunk is rejected before `value.fill(0)`, leaving the supplied
  chunk non-zero; and
* it does not recheck cancellation/current document ownership after EOF, after
  a body read, or after the asynchronous component/descriptor SHA calls.

The sibling manifest reader has the same lifecycle class: it has no abort or
deadline input and cancels without releasing its reader
([`readExecutionTargetManifestJson:641-674`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L641-L674)).
Fixing only component and descriptor bodies would leave the protected
precondition stream able to strand a document open.

`closeArtifactRuntime` does correctly set `closed`, abort `docAbort`, drops the
lease, and removes the runtime identity
([lines 2990-3007](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L2990-L3007)).
The gap is that the pending body reader does not consume that abort. The old
state can therefore retain an in-flight body after a same-key reopen even
though the state map now contains a different owner.

## Small coherent repair

Keep `fetchWithTimeout` unchanged. It should not pretend to own a response
stream after it returns. Add a private, local `ExecutionTargetReadControl` in
`backbone-worker.ts` and use it for manifest, component, descriptor, and the
future browser-actor body:

```ts
type ExecutionTargetReadControl = Readonly<{
  signal: AbortSignal;
  deadlineAtMs: number;
  assertCurrent(): void;
}>;
```

For each asset request, establish `deadlineAtMs` *before* calling the broker as
the earlier of the operation's remaining request budget and `plan.expiresAtUnixMs`.
Pass its signal to `browserExecutionTargetAssetRequest`, then pass the exact
same control to the stream reader. This makes the existing per-asset timeout
cover **headers plus body**, rather than granting a fresh unbounded lifetime
after headers.

`assertCurrent()` must fail closed on every turn when any of these changes:

* `signal.aborted`, plan expiration, or the per-asset deadline;
* `state.closed` or `artifacts.get(state.runtimeKey) !== state`;
* changed runtime key/bound Hub scope, document id, schema, or requested
  surface; or
* after a lease exists, `state.executionTargetLease !== capturedLease`,
  `!capturedLease.live`, or its complete `sameLeaseFieldsV1` projection no
  longer equals the captured fields.

Call it before dispatch, immediately after every awaited fetch/read/hash, after
EOF, and immediately before lease mint/actor-child transfer. The existing
`documentBrowserActorLease` already contains the correct state-map, scope, and
full-field comparison shape at
[`backbone-worker.ts:845-851`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L845-L851);
factor a read-specific private predicate from that rather than introducing a
second, partial comparison.

Inside the shared bounded stream helper, install a one-shot abort/deadline
listener before each `reader.read()`. It must issue `reader.cancel(reason)` at
most once and make the caller's await reject immediately; its cancellation
promise is a retained cleanup operation, not something the document-close
path waits on. Remove every listener in `finally`. Release the reader lock on
normal EOF, and on cancellation/error after the pending read/cancel settles;
attach a caught continuation for that deferred release so an uncooperative
source cannot create an unhandled rejection. A standards-compliant native
`Response` settles a pending read after cancellation, so the normal abort path
releases its lock. JavaScript cannot synchronously unlock a reader whose
nonconforming custom source refuses both read and cancel; the correct
fail-closed behavior is immediate operation rejection, private buffer wipe,
and a retained no-throw cleanup continuation—not publishing bytes or waiting
forever.

For every non-EOF chunk use a per-iteration `try/finally`:

1. require an actual `Uint8Array` and check `value.byteLength <= declared - received`;
2. copy it into the sole preallocated private aggregate; and
3. `value.fill(0)` in `finally`, including bad type/oversize/copy failure.

On any non-success path wipe the aggregate. On success release the reader
before returning the sole aggregate owner. Recheck control after the EOF result
and before return: an abort racing the last fulfilled read cannot mint a lease.

The reader's `body` capability should be explicit in a private local structural
type extending `FetchTimeoutResponse`; it is intentionally absent from the
shared header-only public interface. Do not widen the framework response API
just for this body consumer.

## Actor-body-specific binding

The current reservation intentionally fetches no body
([`DocumentBrowserActorReservation.reserve:820-829`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L820-L829)); it only reserves a child with
the browser-actor SHA and length named by the live lease. When actor acquisition
is added, it must retain the exact private lease, admitted grant, and original
parsed open intent inside the reservation/lease owner—not reconstruct selectors
from a public module URL or caller actor metadata.

The actor request may use the existing fixed `browser-actor` POST route and the
same plan-free intent shape (the route is already allowlisted at
[`browserExecutionTargetAssetRequest:620-628`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L620-L628)), but every await must prove:

* the captured reservation remains `state.browserActorReservation`;
* the exact private lease remains live and equals the state's current lease;
* the grant has not passed `reserveBeforeMs`/`retireAtMs`; and
* the full `browserActor` row (kind, schema, policy, SHA, length, source
  component/descriptor hashes, and sorted imports) equals the captured lease
  fields before transfer to the child.

The actor bytes must verify SHA/length before `postMessage(..., [buffer])`; on
every fault, abort, stale control, or transfer failure, zero the local buffer,
close the reservation, and never leave a `DocumentExecutionTargetLease` or
child with a partially read actor. No fetch URL or route response is authority
by itself.

## Minimum hostile law

Extend the existing neutral
[`document-execution-target-lease-v1`](../../../../../../🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json)
`hostile` matrix with `component-pending-read-abort` (stage `component`,
expected `unpublished`) and exercise it beside the existing
`executionTargetBodyResponse` harness at
[`backbone-worker.ts:4838-4840`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L4838-L4840).
The harness response should be a real `Response(new ReadableStream(...))`:

1. enqueue one observable first chunk, then deliberately leave the second
   `pull` pending;
2. wait until `reader.read()` is pending, call `state.docAbort.abort()`, and
   await the open operation's rejection without resolving the second pull;
3. assert exactly one stream `cancel`, unlocked response body after standard
   cancellation settles, zeroed first source chunk, no socket-grant request,
   no lease, no browser child, and no later status other than cancellation or
   integrity failure; and
4. attempt a late enqueue/read resolution and assert it cannot publish a lease
   or revive the old runtime after a same-key close/reopen.

Add one `component-oversize-chunk-wiped` row using the same helper: a body with
a valid declared header but a first chunk one byte larger than remaining must
reject, zero that supplied chunk, release/cancel, and publish neither lease nor
grant. These are small runtime extensions of the existing fixture rather than
a separate synthetic body protocol.

## Planned Shared Reader Algorithm Review

The proposed private `control(signal, absoluteDeadline, assertCurrent)` plus a
single known/unknown-length preallocated reader is the right scope. Keep it
private to the execution-target path; it is not an alternate global
`fetchWithTimeout` contract.

### Release after a pending cancellation is standards-safe

For a standards-compliant `ReadableStream` response, `reader.cancel(reason)`
transitions the stream to closed and settles pending read requests before its
*underlying-source cancellation promise* necessarily settles. `releaseLock()`
is therefore safe in `finally` immediately after issuing the one cancellation
request; it does not need to await a source whose `cancel()` promise never
resolves. Even if a read request is still pending at release, the stream reader
release operation rejects that pending read instead of retaining the lock. The
late read promise must have a rejection observer so this normal cleanup does
not become an unhandled rejection.

The seven-row pending-cancel fixture is valuable precisely because an
underlying `cancel()` that never settles must not make close/deadline await
forever. The reader lock should still be released and all local aggregate
buffers wiped; the unresolved source cancellation is a detached, caught
retirement operation. A custom source which refuses both cancellation and its
pending read is outside the `ReadableStream` progress guarantee. It cannot be
forcibly unlocked by JavaScript, so no caller may retain/publish a body while
waiting for it.

### One necessary ownership detail for the late-read handler

Do not attach a simple `read.then(value => value.fill(0))` beside the race: a
normal fulfilled read can run its observer before the `await Promise.race`
continuation claims the same chunk, corrupting an otherwise valid body.

Use a per-read slot with explicit states instead:

```ts
type PendingReadSlot = { state: "pending" | "claimed" | "abandoned" };
```

The late settlement observer wipes a non-EOF value only when the slot is
`abandoned`, and always observes rejection. When the read wins, first retain
the result in the current iteration, set `claimed`, and run `control.assertCurrent()`;
if that post-await check fails, wipe that claimed value before propagating the
failure. The per-chunk copy remains inside `try/finally { value.fill(0) }`.
When abort/deadline/error wins, first mark the slot `abandoned`, issue the
one-shot caught `reader.cancel`, and then release in outer `finally`.

This covers the important race where a read resolves in the same turn that the
document closes: it either never becomes claimed, or the post-await control
check rejects and wipes it. It must never be copied into a mintable aggregate.

### Required control ordering

1. Validate headers and construct exact aggregate capacity before acquiring the
   reader. Component, descriptor, and future actor bodies have authoritative
   byte lengths: require `Content-Length` to equal that length and preallocate
   exactly it. Do not accept unknown length for these identities.
2. Before scheduling every read, install one combined signal/deadline reject
   gate. It must handle an already-aborted signal and an already-expired
   absolute deadline synchronously, then attach both listeners before starting
   the read to avoid a check-to-listener race.
3. Race that gate with exactly one read. Immediately after the winning read,
   call `assertCurrent()`; after EOF, call it again before successful return.
   `assertCurrent()` includes document map identity, closed/abort state, plan
   expiry, and (where one exists) the exact current lease/reservation/grant.
4. On any error or abort, zero the aggregate and use the slot rule above for
   the source chunk. On success, release the lock before returning the only
   aggregate owner.

Only the manifest may use unknown length; its 8 KiB cap makes one max-sized
private allocation acceptable. It must copy the received prefix into an
exact-sized returned owner and wipe the max-sized backing before return. A
generic unknown 64 MiB response cannot return a subarray, because that would
retain the full 64 MiB backing behind a small accepted result.

The named new corpus cases—`pendingAbort`, `pendingDeadline`, `staleAfterRead`,
and `abortEOF`—cover these four ordering points. Add an assertion in each
failure case that the response body is unlocked after standard cancellation
settles, with no socket grant, lease, actor reservation, or status publication.
