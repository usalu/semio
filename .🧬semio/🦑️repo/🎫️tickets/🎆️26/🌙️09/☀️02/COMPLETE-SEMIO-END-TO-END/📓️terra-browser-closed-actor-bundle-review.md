# Closed browser actor bundle: lifecycle and retained-poll audit

## Decision

`closedBrowserActorBundle` is a useful build-time closure foundation: it embeds the admitted core bytes, builds a single ESM, rejects residual imports/network loaders, and creates a host per `activate` call.  It is **not yet safe to call a completed actor lifecycle**, because two error/cancellation paths can retain a guest component or running invocation indefinitely.  It is also not yet an end-to-end proof of the canonical `host-async` result/stream path.

No source edit or build was performed for this audit.

## P0: close rejection skips guest release and active drain

In the generated actor wrapper, `close` sets `closed` and assigns:

```ts
closing = host.close().then(async () => {
  await Promise.allSettled([...active]);
  component = null;
});
```

at [`browser-bundle/📜️script.ts:29-36`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:29).  `createBrowserHostActivation.close` can reject after it has already rejected pending effects, for example when `port.cancelEffect` returns an invalid state or a stream's `cancel` rejects ([`host/🟦️.ts:26-45`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:26)).  A rejected `host.close()` skips the `.then` body: `active` is not awaited and `component` remains strongly retained by the activation closure.

Fix this as one terminal owner transition: capture the host error, **always** await `Promise.allSettled([...active])` and clear `component` in `finally`, then rethrow the captured host error.  It keeps close's failure observable without retaining the actor.  The law must start a canonical async guest invocation that has a pending host request, make `cancelEffect` fail, call `close`, assert close rejects, assert the invocation settles, and assert all later `invoke` calls reject `closed`.  A synchronous counter invocation cannot exercise this branch.

## P0: abort closes only the host, not the actor owner

The bundle supplies `control.signal` to `createBrowserHostActivation` ([`script.ts:21-24`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:21)), whose abort handler executes only `void close()` for the host ([`host/🟦️.ts:46,125-126`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:46)).  The outer actor's `closed`, `closing`, `active`, and `component` variables are untouched.  Subsequent `invoke` happens to deny because it separately tests `control.signal?.aborted`, but the component can remain retained forever if the shell correctly treats the abort as terminal and never calls `actor.close()` itself.

Install one actor-level, once-only abort listener after `close` is defined.  It must call the actor's own `close`, and `close` must remove that listener.  The host's listener can remain; both calls are idempotent and share the host close promise.  Add an actor—not host-only—law: activate a real asynchronous import guest, begin an invocation, abort without calling `close` manually, then assert the pending guest call settles, post-abort invocation is rejected, and the close promise is terminal/idempotent.  The existing host-unit abort law at [`script.ts:430-436`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:430) does not prove actor component retirement.

## Exact host-async qualification gap

The counter fixture used by `testClosedBrowserActorBundle` imports only `pure` and performs synchronous `next` calls ([`actor-factory/🔣️.json:5-10`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🧊️actor-factory/🔣️.json:5)).  It proves ESM closure and per-actor pure state, but not active-invocation tracking or host result/stream translation.

There is a stronger real guest fixture: `actor-import` imports `semio:framework/host-async@1.0.0`, awaits `link-resolve`, and consumes `blob-read`'s `stream<u8>` ([`world.wit:7-21`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit:7); [`library/🦀️.rs:11-24`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs:11)).  Its current runner uses raw JCO/`instantiateFreshComponent` and directly supplies a test host object; it never calls `closedBrowserActorBundle(...).activate(...)` ([`actor-import/📜️script.ts:57-107`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts:57)).

The next native/browser boundary law should compile that exact guest once, build the closed actor ESM, activate two actors with distinct `BrowserHostPort`s, route exact request IDs back through `resolveEffect`, and prove all of:

1. accepted and error-tagged `linkResolve` bytes round-trip per activation;
2. an accepted `ReadableStream<Uint8Array>` is consumed as the guest's canonical byte stream, and a close/abort cancels it exactly once;
3. a reply with the sibling's request ID is refused;
4. close while `linkRoundtrip` is pending drains that invocation and does not affect the sibling.

This is also the right test to use the **actual `generated.imports` from JCO** as the sole import manifest.  The present static binding table admits exactly `semio:framework/pure@1.0.0` and `semio:framework/host-async@1.0.0` ([`script.ts:13-18`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:13)).  If the active JCO probe emits a versionless key, update this single canonical table and the fixture expectation to that observed key; do not add a permissive alias map.  The source currently provides no evidence that a versionless key is admitted.

## WASI fence and production non-claim

The current closure logic is fail-closed for an unsupported interface: a manifest containing a WASI import is rejected by the static table before an actor module is built, and omission of that import is rejected by the component factory's exact manifest comparison ([`script.ts:13-17`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:13); [lines 160-164](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:160)).  Add the above real guest's actual `generated.imports` as a deliberate negative test when it includes `wasi:*`; assert rejection before `Bun.build`/activation.  This qualifies the fence without inventing a browser WASI shim.

Finally, `BrowserHostIdentity` includes only `actorId` and `activationGeneration` ([`host/🟦️.ts:1-8`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:1)); it has no verified plan receipt, execution-target digest, document runtime key, or capability policy.  The wide `hostAsync` surface is therefore a foundation API, not authorization evidence.  Do not mount it into Shell/plugin routing or claim private-plan/Map execution until the caller supplies a verified runtime scope and the worker enforces it for every effect.

## Corrected journal conclusion

The Store durable-group journal's synchronous `advance(grant)` does not require a blocking executor.  It can be a retained polling contract when the actual WAL submit is owned by a dedicated per-document actor and `advance` merely starts/checks an owned reply operation.  `DbIoTaskOperation` demonstrates the retained-handle/waker/result-lease pattern ([`storage/🦀️.rs:4154`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4154))—not an excuse to block a worker closure.

For Map, the sink commit should own `NotSubmitted | Awaiting(reply) | Committed | Failed`.  On its first Store `advance`, it transfers the bounded request-bound envelope to the private Map WAL actor and returns `Pending`.  That actor alone owns `ArtifactWal`, creates/closes `WalRecordBatch`, obtains its backend facet, and awaits `ArtifactWal::submit(Fsync)`.  Later Store advances inspect the reply and return `Committed` only with its exact receipt.  A fire-and-forget Hub task borrowing `&mut ArtifactWal` and `WalRef<'_>` is not valid: `DbBackend::wal` produces the borrowed latter ([`storage/🦀️.rs:4853`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4853)).

Thus the smallest bridge preserves the generic synchronous Store host and introduces an owned, bounded Hub Map WAL actor/mailbox.  An `async` journal trait is optional later refactoring, not a precondition; neither design may use `WorkerPool`'s `FnOnce` jobs to `block_on` the submit.
