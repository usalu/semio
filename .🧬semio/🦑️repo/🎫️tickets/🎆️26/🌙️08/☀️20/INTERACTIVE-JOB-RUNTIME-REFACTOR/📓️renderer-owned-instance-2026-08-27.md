# Activation-Owned Renderer Instance

## Current Implementation

`ui/contract/retained/instance/component.ts` now holds one exact captured `ShardActorActivationLease` object and the canonical three-field `ActorInstanceLifetime`. The native guest lifetime is required; it is not synthesized from the numeric instance ID. Constructor, lookup and patch operation admission validate the captured live authority. Retirement does not call `assertActive` and releases its captured activation reference before terminal completion.

Surface ownership is an intrusive per-instance list, not a process-global Map. Only one lookup is admitted at a time; one surface cell is compared per advance. Repeated lookup returns the same frozen read-only facade. Its private cell/mint cannot be rebound to another aggregate, including an independently constructed owner with the same tuple. Facades expose subscriptions and captured scene reads, not the underlying mutable `OwnedUiSurface`.

The native surface-name bound comes from `SurfaceId(UiText)` and `UI_TEXT_MAX_BYTES=512`. Lookup comparison charges 2112 bytes: at most two 512-code-unit UTF-16 strings (2048 memory bytes) plus fixed metadata. Admission checks at most 512 code units and bounded UTF-8 encoding. This is not a document-size cap or an 8ms platform timing certificate.

Close cancels the one pending lookup, releases one work-queue cell per step, then closes each surface's admitted wire stream before its paired surface roots. Issued React/scene reads still block the exact old owner. An old guest's close does not look up a replacement by actor name or numeric instance. The work queue is drained before surfaces, and managed scene readers remain serviced by the underlying surface's child-first retirement.

## Executed Evidence

Canonical command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedInstance'`.

- R1: missing-module collection RED, four failed suites, zero executed tests.
- R2: one PASS, 603 skipped, 604 discovered; 7.33 seconds. Strict Ajv validates the language-neutral fixture; Immer creates the replacement guest lifetime. Exact activation identity, guest 13→14, one-slot lookup admission, repeated-facade identity, foreign facade rejection, revoked operation admission, old-reader close blocking and independent replacement survival execute.
- R3: one PASS, one FAIL. A stale old patch facade could return the successor's cached input receipt because `??=` skipped the private current-patch check. The subsequent release assertion was not reached on RED.
- R4: two PASS, 603 skipped, 605 discovered; 5.20 seconds. Every receipt access now checks the exact current patch before reading a shared cell. Both stale read and stale release reject while the successor retains its receipt.
- R5: two PASS, one FAIL. The acknowledgement token export was not yet defined; this was an executed test-body RED.
- R6: three PASS, 603 skipped, 606 discovered; 4.43 seconds. Raw forged token roots and reflected construction reject without consulting public getters/matchers.
- R7: four PASS, 603 skipped, 607 discovered; 3.81 seconds. Native 512-byte Unicode names, one-cell pending lookup cancellation, and reflected facade/lookup mint rejection execute.
- R8: four PASS, one FAIL, 603 skipped, 608 discovered; 4.22 seconds. The new exact retirement-witness test reached the missing `takeRetirementWitness` method.
- R9: five PASS, 603 skipped, 608 discovered; 3.71 seconds. The private final witness transfers once, rejects same-tuple foreign owners/activation aliases, and remains unavailable while an exact reader is held. Root independently captured the same five-test checkpoint: five PASS, 603 skipped, 608 discovered; 6.34 seconds.
- R10: four PASS, two FAIL, 603 skipped, 609 discovered; 5.23 seconds. Both native fixture tests stopped at the newly enforced native turn-status boundary: the authored mock open response omitted `status`. This was not a production ACK failure and did not reach the new source signature. The fixture now includes the actual native idle status.
- R11: six PASS, 603 skipped, 609 discovered; 5.58 seconds. Real ShardClient lifecycle source capture, typed two-operation publication, notifications, minted token, post refusal/fault-status retry, private successful submission receipt and revoked-operation close execute.
- R12: six PASS, one FAIL, 603 skipped, 610 discovered; 11.20 seconds. A second independently constructed same-lifetime UI owner could admit a zero-operation source on its own surface. `matchesOwner` now checks the exact concrete host registered with the lifecycle owner before any UI mutation.
- R13: seven PASS, 603 skipped, 610 discovered; 6.95 seconds. The new negative owner-claim case passes. The native join also executes Accepted→Retired, rejects a different real host witness, holds the real witness across a failed final ACK send, retries that exact witness, reaches lifecycle Complete and permits disposal.
- R14: seven PASS, one FAIL, 603 skipped, 611 discovered; 6.99 seconds. The input-retirement test reached the not-yet-defined `OwnedUiPatchInputRetirement` export. The class and UI half are now authored, but native `releaseInput` is still being implemented by the producer peer; R13 does not certify this newer stage.
- R15: eight PASS, 604 skipped, 612 discovered; 15.45 seconds. The completed native `releaseInput` join validates normal and cancellation retirement; cancellation before decode retains the held input until its exact private token releases the source slot.
- R16: eight PASS, one FAIL, 605 skipped, 614 discovered; 8.98 seconds. The new accepted-input claim test reached the missing `OwnedUiPatchInputAcceptance` export. Its private class and post-successful-offer installation are now authored, while the producer's `acceptInput` method remains the next join.
- R17: eight PASS, one FAIL, 607 skipped, 616 discovered; 7.69 seconds. The accepted-input test expected `/whole|view/`, but the unchanged intrinsic boundary correctly reported `Native ownership requires an entire non-shared admitted buffer`. No production repair was needed for this assertion mismatch.
- R18: nine PASS, 607 skipped, 616 discovered; 18.64 seconds. The exact assertion now reaches all rejection/retirement assertions. Native reservation, private post-transfer acceptance and exact private retirement are joined; rejected partial-buffer admission keeps the allocation intact and cannot produce a UI-retirement token. Normal and cancelled accepted input, private ACK retry and final host-witness retry remain green.

Logs remain in the master ticket as `renderer-owned-instance-rN`.

## Executed ACK Capability Join

`OwnedUiPatchAcknowledgement` is defined at the same source module. Its constructor requires the private module mint. Static `matches(token, sourceAuthority)` checks private fields, not a supplied callback or public method. Read-only `owner` identifies the original aggregate; frozen `value` contains exact lifetime, surface, revision and canonical hash. There is no public mint or boolean discharge method.

The private issuer is connected only to `OwnedNativeUiPatchAuthority`, captured from the exact returned lifecycle turn. `OwnedUiInstance.beginPatch(source, facade)` validates its private brand, activation/lifetime and the exact native-bound host owner. `OwnedUiInstancePatch.offer(ordinal)` reads only that source's original operation; callers cannot substitute a different raw operation. Its one input slot rejects wrong ordinals and busy admission before accessing another input. `peekAcknowledgement()` mints one stable token only after the wire stream yields completed paired publication, mandatory notification and retirement evidence. `acceptAcknowledgement(receipt)` accepts only `OwnedNativeUiPatchSubmissionReceipt.matches(receipt, exactSource, exactToken)` and cannot be discharged with a boolean or structural record.

An initial unmounted ordinary `activation.turn` submit draft was removed after root clarified that new-operation authority is revoked during close. The executed dedicated `submitUiAcknowledgement` path stays on the original lifecycle owner after operation revocation. Post refusal and native fault status retain the same source/token; only the private successful receipt releases the outbox. This is mock-worker runtime integration, not a fresh WASM/browser proof.

Host-local per-operation input-release receipts are not invented WIT per-operation network ACKs. The current WIT only acknowledges an entire patch. The parent input owner must retire its processed record before releasing the local receipt; the actual source/page handoff remains part of the pending live transport join.

## Final Host Retirement Witness

`OwnedUiInstanceRetirement` is runtime privately minted only at the aggregate's terminal transition, after its lookup, wire outboxes, streams, read owners and surface cells are empty. `takeRetirementWitness()` transfers that token once. `OwnedUiInstanceRetirement.matches(witness, owner, activation, lifetime)` checks all four exact/private bindings; a structural `isRetired()` callback is not evidence. The final witness owns the small captured activation alias until the lifecycle owner consumes the final native ACK; it owns no surviving UI document payload. `OwnedUiInstance.matches` permits the peer to bind a concrete instance before retirement without trusting a public boolean callback.

## Input Retirement Handoff

The authored `OwnedUiPatchInputRetirement` has a private runtime mint and `matches(token, sourceAuthority, ordinal, originalOperation)` checks exact private identities without reading a public matcher or ordinal getter. Normal issuance follows the wire stream's completed decode/input-close/apply receipt. Cancellation issuance requires the entire bound wire to be terminal while retaining its held input identity; close then blocks until the native owner releases that slot. `releaseInputReceipt(exactToken)` invokes the native source's `releaseInput(token)` itself and clears the UI outbox only on true. It accepts neither a raw ordinal record nor a caller's claimed success flag. Refusal after patch beginClose occurs before another source operation is accessed.

`OwnedUiPatchInputAcceptance` has a separate private brand. It is minted only after successful `wire.offer`, then synchronously installed with `source.acceptInput`. Merely reading/reserving a native operation cannot count as accepted UI ownership. The UI holds its accepted original/input before installing the claim, so a refused claim installation does not lose the acquired payload. The producer requires this exact claim before accepting the later retirement token. A rejected buffer subview never mints the acceptance claim or retirement token.

Three fixture-only strict-type errors reported by the coordinator were corrected: replacement guest IDs now derive from the neutral fixture with `BigInt`, not an immutable literal-13n type, and the surface collection has an explicit schema-owned `OwnedUiInstanceSurface[]` type. No production authority type was widened or cast away. Coordinator strict rerun remains separate from R18.

The native producer must separately retire an operation that was read but whose UI constructor admission failed, plus every unoffered operation and the original containers. Such inputs cannot acquire a completed UI-retirement token. Captured original-turn identity also does not by itself freeze original mutable payload aliases; exact raw-source ownership remains part of that producer packet.

## Remaining Required Work

The native source still owns the original whole returned-turn/patch/operations; bounded input release and final raw-turn retirement remain peer-owned, explicit prerequisites. The aggregate closes its UI roots, not that separate native source owner. No native WIT paging or whole-output-copy certificate follows from the tested UI stream. Connect the scoped adapter in both React and WGPU only after that source handoff is coherent. Captured lifetime must initialize the aggregate before open-turn UI publication reaches it. The fourteen nested-JSON host projections and TextEditor generic-pack input also remain required before live UiNodeView cutover. No whole-object compatibility conversion or silent empty fallback has been added.

Targeted `git diff --check` passed for the aggregate, its two fixtures and the renderer test module after R13. No Rust builds, destructive commands, ticket cleanup or git mutations were run in this packet.
