# Closed Async Actor Shutdown and JSPI Admission Audit

## Scope and evidence

Read-only audit of the failed `actor-import-9Uq185` closed-runtime receipt and the current browser-bundle source. No build was started.

The failure is reproducible from the retained receipt as an unhandled `Error: browser host: closed` from `actor-import.closed.js:27`, during the explicit `pending-host-close-check` fixture path. It is not evidence that the caller forgot to observe the invocation: the fixture creates `assert.rejects(...)` before calling `close()` at [actor-import fixture script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts:303).

## P0: JCO leaves a second rejected promise orphaned

The JCO output owns two promise branches for a manual async import. Its internal `_lowerImportBackwardsCompat` rejects the guest-visible `manualAsyncResult`, then rethrows from an `async` `queueMicrotask` callback whose returned promise has no owner:

- [generated JCO artifact](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:3198) creates the detached callback.
- It rejects the caller-visible branch at [line 3214](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:3214) and throws again at [line 3216](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:3216).
- The generic `_lowerImport` has the same unowned `queueMicrotask(async ...)` + `throw` structure at [lines 3032–3046](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:3032).

The direct cause is the host’s plain Error retirement at [browser host line 32](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:32). JCO deliberately extracts an *own* `payload` property as a declared WIT `result` error, but rethrows any ordinary `Error` at [generated line 6685](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:6685). The raw host request’s `result.catch(() => {})` at [host line 127](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:127) only observes that raw promise; it cannot own JCO’s later child promise.

### Required correction

The planned first-party `Error` with an own `payload: Uint8Array` is the correct narrow boundary correction. Its bytes must be the existing host/WIT fault JSON convention, specifically the same `Fault { origin: "os", code: "capability-revoked", severity: "error", message: "browser host: closed", scope: {}, retryable: false }` bytes produced by native `dsl::encode_fault_bytes`; the native convention is documented at [plugin host](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs:38) and constructed at [native async imports](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:250).

Do **not** suppress JCO’s unhandled rejection globally or patch generated JCO catch blocks. A payload-own Error follows JCO’s declared `result-catch-handler` route, so the guest receives its normal WIT `err` value instead of a JavaScript trap.

Apply that rule to every error which an asynchronous `hostAsync` import can produce, not solely the close loop: the already-closed request check, request/stream capacity rejection, dispatch exception, malformed tagged response, and response projection exception at [host lines 100–125](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:100) and [HTTP projection](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:136). Any one left as a plain Error can re-enter the same installed-JCO orphan path.

This changes the actor fixture’s expected shape: `link-roundtrip` is declared `result<list<u8>, list<u8>>`, so after JCO consumes the payload the public guest export rejects with the `Uint8Array` fault bytes, not an `Error` matching `/browser host: closed/`. Replace that assertion with a byte/decoded-Fault predicate. The existing browser-host neutral law already establishes exactly this decoded payload expectation at [browser-bundle script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:466) and its schema fixture.

Required exact runtime laws:

1. Start `linkRoundtrip` with replies disabled, attach a rejection handler, close, and assert one cancellation, terminal `closed`, no process `unhandledRejection`, and the exact decoded `capability-revoked` payload.
2. After close, invoke the same async import and assert the same payload path (not a plain Error/JCO trap).
3. Trigger a capacity and a `dispatch` exception through an actual async import under the JCO fixture; assert they are declared payload failures with no unhandled branch.

## P0: JSPI AST guard accepts a decoy suspension wrapper

[`validateWasiSuspension`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:15) records `fnName` and `manuallyAsync`, then marks `_trampolineN` suspended if **any** `new WebAssembly.Suspending` subtree contains `{ importFn: _trampolineN }` ([lines 23–29](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:23)). It does not prove that the resulting `trampolineN` binding is the one supplied to a real `instantiateCore` import object. A malicious or incompatible generated source can append a decoy Suspended wrapper while binding the physical core to the fallback lowerer; the guard accepts it.

The retained JCO output shows the relationship the guard must prove, rather than merely discover:

- `_trampoline18` has `fnName = "wasi:io/poll@0.2.9#block"` and `manuallyAsync = true`.
- Its conditional `trampoline18` wrapper is at [artifact line 9128](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:9128).
- The actual core imports that exact binding as `[method]pollable.block: trampoline18` at [line 10368](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:10368). The transformed poll binding is likewise passed as `trampoline30` at [line 10404](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-9Uq185/actor-import.closed.js:10404).

Replace the three independent sets with a per-binding AST record:

1. collect each `_trampolineN` `fnName` and literal `manuallyAsync` assignment;
2. collect only the `let trampolineN = _trampolineN.manuallyAsync ? new WebAssembly.Suspending(_lower...{ importFn: _trampolineN }) : ...` relationship;
3. collect identifiers used as values in the object literals passed to `instantiateCore` (including nested interface objects);
4. for every emitted blocking selector, require `manuallyAsync === true`, a matched wrapper relationship, and use of that *wrapped* identifier in an `instantiateCore` import. Reject a decoy, a fallback binding, an unlisted manual-async selector, duplicate selector-to-wrapper mappings, and a missing emitted selector.

The current regex does recognize the three selectors emitted by the retained component (`poll`, `block`, `blockingFlush`), but should be an explicit policy mapping from the JCO `asyncImports` manifest to the emitted versioned `fnName`; it must not silently rely on a partial camel-case pattern. Add hostile AST vectors for a detached decoy wrapper, `manuallyAsync = false`, and a wrapper not passed to `instantiateCore`.

## Qualification status

`actor-import-9Uq185` is RED. This audit validates the fault route and admission flaw statically from its retained generated JavaScript; it does not claim a corrected browser runtime has executed.
