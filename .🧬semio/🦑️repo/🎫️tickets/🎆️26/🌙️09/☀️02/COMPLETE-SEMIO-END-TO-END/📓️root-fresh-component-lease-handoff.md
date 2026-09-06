# Fresh Component Lease Handoff

Root read the complete Terra opaque-handoff audit before implementation. This continues the existing fresh Stdio/GIS producer; there is no alternate materializer or raw-path receipt.

## Implemented Boundary

`produceFreshComponentV1` now requires a derivation callback and returns a frozen metadata receipt plus its derived result. The callback receives only a frozen `consume` capability, with no component/core/descriptor pathname or retained source array. Consumption happens once, copies the admitted component in bounded chunks, and wipes the copy after settlement. Mutating the copy cannot change the producer's staged source or verified digest. A retained capability expires when the outer callback settles; the producer drains an already-started consumer even when the callback forgot to await it. Ignored consumer failures reject the producer, and a callback failure preserves its error while still draining the outstanding operation.

The shared stage/handoff boundary removes only files it successfully created on failure and wipes component/descriptor owners on every exit. The outer producer additionally retires the extracted core and private work directory. Cancellation is checked before allocation, during copying, after the consumer, and before completion. This is a trusted build callback with cooperative cancellation, not a sandbox for an arbitrary never-settling callback; callers can deliberately copy bytes they are loaned. It does not expose the producer's retained mutable owner.

The only production caller, Hub bootstrap, now consciously consumes and hashes the loan and compares its SHA256 to the returned receipt. Browser actor construction/catalog staging is the next separate integration, not claimed here. The generic result does not certify actor/source identity by itself; the GIS actor caller must compare the real builder's independently calculated component SHA256.

## Test-First Evidence

- Registered source session60035 is the expected missing-stage/handoff-helper RED after adding the schema-first laws.
- Session14082 is GREEN for all11 fresh staging/handoff laws, evidence `🗑️generated/fresh-component-staging-OvSSxQ`, plus the existing two-package/28-codec/one-target bootstrap corpus. AJV, WebCrypto, Node hashes, first-party Pack verification and BLAKE3 known answers remain independent checks.
- Added laws cover one-shot/expired capabilities, copied mutation isolation, throwing callback cleanup, cancellation before and after consumption, awaiting a deliberately unawaited gated consumer, and retaining an ignored consumer failure. Source-file replacements from the prior capture law remain in place before the successful handoff, so the loan digest cannot be taken from reopened source paths.

No fresh Cargo/JCO/emitter materialization, browser actor catalog, Hub startup, or multi-user UI acceptance follows from this source boundary receipt. Strict one-time codec capture and final generation revalidation remain open.
