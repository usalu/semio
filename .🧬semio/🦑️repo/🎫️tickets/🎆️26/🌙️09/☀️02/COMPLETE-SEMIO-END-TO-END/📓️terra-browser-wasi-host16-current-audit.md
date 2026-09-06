# Browser WASI Runtime and Host Stream Audit

## Scope and status

Read-only review of the newly landed first-party Preview2 foundation at [🌐️wasi/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts) and the current host16 stream changes at [🌐️host/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts).

The WASI foundation correctly has a fresh admission symbol, classes, resource set, timers, waiters, and import objects for each createBrowserWasiActivation call. It has no module-global resource identity. It rejects a foreign Pollable by class identity and retires timers/waiters on close. This is source inspection only; no runtime or JCO compilation was launched here.

## P0: an admitted output stream cannot always report a sink failure as its declared WASI error

OutputStream.write first admits a write and then, if port.write throws, creates a WasiIoError resource to throw the required stream-error shape; see [lines 128–138](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:128). Resource construction is itself capped at 256 at [line 46](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:46).

Therefore: admit one stdout handle, fill the other 255 ordinary resource slots with pollables, call checkWrite on stdout, then make its binary sink throw. Creating the error resource fails with a plain resource-capacity exception, replacing the expected thrown stream-error tag. The JCO catch-handler cannot lower that to the declared WASI error resource. The current neutral fixture now contains exactly this adversarial trace at [lines 53–64](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌐️wasi-activation/📜️script.ts:53), so it should remain RED until the implementation changes.

The clean small ownership transition is to retire the now-failed OutputStream's active resource slot before admitting the one error resource, while retaining a terminal shell whose future operations deterministically throw closed and whose dispose is inert. That keeps the total externally live resource budget at 256 without reserving capacity that changes normal admission. On a sink exception, consume the output slot in that one-way transition, hand its slot to a disposable WasiIoError, and throw the exact tagged last-operation-failed value. Verify subsequent output operations are closed and no credit leaks after close.

## P0: the foundation is not yet an admissible blocking JCO runtime

The new Pollable.block and poll return promises at [lines 90 and 99–114](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:90). The retained current JCO actor-import lowering invokes Pollable.block synchronously, as documented in [actor-import.js](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-tQ9aw0/jco/actor-import.js:7177). The closed actor builder still rejects every WASI import at [browser-bundle script line 16](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:16), and its JCO transpile calls have not yet passed a JSPI async policy.

The TypeScript fixture awaits poll directly, but that does not prove generated JCO can suspend. Do not mount this runtime for a component that may block until the browser bundle transformation uses installed JCO asyncMode jspi and the exact async selector set recorded in the earlier WASI packet. The generated closed artifact must bind that mode, JCO version, and selectors in its verified digest. No Promise-under-sync-wrapper workaround is safe.

## P1: binary sink ownership must be an explicit synchronous-copy contract

write slices guest bytes then zeroes that copy in finally at [lines 132–138](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:132). This is correct only if BrowserWasiPort.write synchronously consumes or copies its Uint8Array. The current TypeScript type says only void, and the neutral test expands bytes immediately, so it cannot detect a retaining mounted port receiving all zeros after return.

Document that contract on BrowserWasiPort.write and add a law whose sink intentionally retains the argument; it must observe a copy supplied by the sink, while the activation's temporary input is zeroed. An asynchronous port must copy before queuing. This protects output correctness without retaining guest linear-memory bytes or adding a global buffer.

## P1: host16 explicit generator return while blocked needs a consumer-termination state

The host stream wrapper starts retirement and reader.cancel before invoking the underlying generator return or throw at [lines 88–96](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:88). A generator parked inside reader.read wakes after cancel, then its post-read checkStream at [line 78](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:78) sees retired and faults. Thus caller-initiated return can reject with stream retired instead of completing its own termination. The same ordering risks masking a supplied throw reason.

Keep external close behaviour—parked next must reject—but add distinct consumer-return and consumer-throw states. While a consumer is terminating, cancellation may unblock reader.read without checkStream converting that consumer termination to stream retired; the underlying return/throw owns its specified completion. A reader.cancel failure must be surfaced either by that terminating call or retained as an activation-close failure; it cannot silently disappear when retire's finally removes the stream from the active set. The current cleanup is otherwise sound: retire's finally releases the reader, zeroes copied bytes, decrements the buffered budget, removes the stream, and close aggregates cancellation errors that remain registered.

Add these host laws:

1. ReadableStream next is parked; generator.return succeeds, cancel runs once, resource capacity/bytes recover.
2. The same setup with generator.throw preserves the caller's thrown value if cancel succeeds.
3. A reader.cancel rejection is surfaced by the terminating operation (or a retained activation error) while reader lock, stream membership, copied bytes, and capacity still retire exactly once.
4. External activation close during parked next still rejects that next rather than being misclassified as voluntary return.

The current neutral host fixture covers external close of a parked next and unused stream close, but not these voluntary parked return/throw or cancel-rejection cases; see [test lines 457–522](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:457).

## Confirmed good seams

- Exact 0.2.0 WASI keys are frozen in one local list at [lines 7–11](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:7); no @0.2.9 substitution is made.
- Environment, arguments, cwd, and terminal are denied at [lines 182–197](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:182). stdin is closed rather than ambient.
- The resource and poll bounds are enforced before admission at [lines 41–58 and 99–110](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:41); clock values are exact u64 and nondecreasing at [lines 21–30](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:21).
- Timer callbacks check closed/resource membership before rearming, and close rejects waiters, clears timers through forced resource retirement, and is idempotent at [lines 72–97 and 170–179](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:72).

## Correction to the earlier interface sketch

The JCO result-catch handler convention is that imported WASI stream methods return unwrapped successful values and throw the stream-error tag; they do not return an outer BrowserWasiResult envelope. The current runtime follows that convention for closed stdin and port failure. The earlier planning packet's envelope type is superseded by this concrete ABI correction.
