# Closed Actor Stream Lifecycle Frontier

## Scope and current verdict

This is a source-only audit. I did not run the fixture. Root reports the newly isolated, registered `pending-host-close-check` as GREEN (`actor-import-yKCT5V`): raw plus closed component paths, JSPI eight, fourteen actor laws, and all six host-close laws. In particular, the real zero-high-water-mark `ReadableStream` parked in `blobCollect` rejects its invocation after close, receives exactly one cancellation, and releases its reader lock. That closes the previously suspected pending-close gap; this packet does not describe it as failing.

The remaining frontier is late stream failure and non-cooperative stream retirement. Neither is covered by the qualified pending-close vector.

## Verified promise ownership

The real probe guest consumes the imported `stream<u8>` with `stream.next().await` in [`🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs:19). The generated JCO transfer's `next` promise is thus awaited by the guest export. On the host side, the underlying promise is `reader.read()` at [`🌐️host/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:84), inside the returned async generator.

The exported invocation is retained before it can run: `invoke` adds its promise to `active` and deletes it only after settlement at [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:195). `close` awaits `host.close`, `wasi.close`, and every retained invocation at [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:177). Therefore the qualified cooperative close chain is:

```text
guest stream.next().await
  <- JCO transfer next promise
  <- host reader.read()
  <- invoke operation retained in active
  <- actor close's allSettled(active)
```

It is an owned rejection, not an orphan. The fixture performs the same end-to-end route at [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts:361), and requires rejected invocation, one cancellation, unlocked stream, and zero active invocations via its schema at [`🧬️.schema.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/🧬️.schema.json:5).

## P0: a late stream error does not have the required JCO fault carrier

The host normalizes the initial effect promise at [`🌐️host/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:133): raw failures become `HostFault`, whose own `payload` is the bytes JCO's generated result catch-handler recognizes. `reader.read()` has no equivalent boundary. A body that later executes `controller.error(new Error("late stream fault"))` therefore crosses a JCO async stream transfer as an arbitrary `Error`, not a declared packed result fault. The earlier generated-source inspection established that its error recovery reads an own `.payload` and otherwise rethrows `Error`; a bare late error can become a guest trap/unhandled branch rather than the invocation's declared `result<_, pack>` error.

The host is the correct repair seam; do not alter emitted JCO source. Add one private normalizer beside `HostFault` that preserves `Uint8Array` and `HostFault`, but maps every other thrown value to a bounded `HostFault("browser.host.stream-failed", message)`. Apply it around the `await reader.read()` plus host-side chunk validation in `stream`. That preserves the result convention used by the already-qualified initial effect errors.

There is a second precise precedence bug. The generator's `finally` awaits `retire(!complete)` at [`🌐️host/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:93). If `reader.read()` rejects and the consequent `reader.cancel()` rejects, the cancellation error replaces the primary read failure. Keep the canonical primary stream failure as the guest invocation result; retain the cancel failure so explicit `host.close()` can report it in its existing retirement `AggregateError` ([`🌐️host/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:46)).

### First law

Extend the real JCO `pending-host-close-check` fixture—not only the direct host test—with a `ReadableStream` that first receives its `pull`, then errors with a plain `Error`. Its `blobCollect` invocation must reject once with the canonical `browser.host.stream-failed` packed error, never resolve a partial vector or EOF, release the reader lock, and leave `activeInvocations: 0`. Execute Node with `--unhandled-rejections=strict`; this proves ownership instead of installing an unhandled-rejection suppressor. A second vector whose `cancel` fails must prove the primary read error remains the invocation error while close reports retirement failure and the lock is still released.

## P1: guest drop is not yet proven through JCO

The direct host test verifies an unused browser iterator is cancelled during host close ([`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:636)), but that is not a component guest dropping its `stream<u8>` resource. Add a second genuine guest export, e.g. `blob_drop(hash) -> result<(), pack>`, which awaits `blob_read`, drops the returned stream without calling `next`, and returns. Its JCO law must observe cancellation exactly once and no locked reader after the export settles. This catches a resource-transfer finalizer omission that a JavaScript iterator test cannot see.

## P0 for production containment: close cannot bound a non-cooperative stream

`retire(true)` directly awaits `reader.cancel()` at [`🌐️host/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:65). If that promise, or a pending `read()`, never settles, `host.close()` never settles, followed by `actor.close()` never settling. A `Promise.race` timeout in Backbone would only return while leaving the guest, its resources, and an active promise in the credential-owning worker; it is not containment.

The smallest honest production seam is a first-party per-activation actor worker. Build its runner into the already closed ESM (static source, no dynamic import or caller URL), construct the `Worker` only after verified bundle/core bytes and the exact target lease have been admitted by [`🧵️backbone-worker.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:220) onward, and communicate over one private `MessagePort`.

The Backbone worker remains the credential/lease and Hub-effect owner. It retains `{documentRuntimeKey, activationGeneration, target digest}` and bridges only bounded tagged effect requests and replies to the actor worker. The actor worker owns exactly one `activate` result, operation table (maximum 32 as the bundle already admits), and a terminal phase. Every invocation completion is caught inside the actor worker and posts one `{ invocationId, ok | canonicalFault }` frame; no detached promise crosses into Backbone.

On close, Backbone rejects new invocations, sends a scoped close frame, and starts one owned deadline. A terminal acknowledgement releases the port, terminates the worker, and discards the exact activation only after every invocation completion has been emitted. If no acknowledgement arrives, Backbone completes all still-open outer invocations with canonical `browser.actor.terminated`, closes the port, calls `Worker.terminate()`, and revokes the worker's object URL. This is the only indicated way to retire a non-cooperative `read`/`cancel` without pretending that it completed. The bridge must additionally cap copied invocation arguments, responses, outstanding effects, and per-activation core/linear-memory admission; the present 64 KiB stream-chunk cap is insufficient as a process memory bound.

## Required production tests

1. Real JCO late stream `Error`, strict unhandled-rejection mode: canonical declared fault, no partial success, zero active operations, unlocked reader.
2. Real JCO guest stream-drop: one cancel and unlocked reader after successful guest return.
3. Actor-worker close with a `cancel()` promise that never settles: parent deadline returns only `browser.actor.terminated`, worker is terminated, later stale effect/reply or terminal message cannot affect a reused `{runtimeKey, generation}`.
4. Same actor worker with a cooperative parked stream: preserve the already-qualified outcome—one cancellation, actual guest invocation rejection, orderly terminal acknowledgement—so forced termination does not replace correct cancellation.
5. Two same-document different-space activations: stale frames and late failures from A cannot settle, cancel, or expose any state of B. Reuse the exact scoped runtime-key discipline already in Backbone ([`🧵️backbone-worker.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:68)).

## Qualification boundary

The closed JCO bundle and the registered fixture now prove cooperative host-close stream ownership. They do not yet mount a production actor worker, prove late stream-error result lowering, prove guest resource drop, or provide a non-cooperative cancellation deadline. The trusted catalog/worker path remains fail-closed, so no production renderer or GIS execution claim follows from this fixture.
