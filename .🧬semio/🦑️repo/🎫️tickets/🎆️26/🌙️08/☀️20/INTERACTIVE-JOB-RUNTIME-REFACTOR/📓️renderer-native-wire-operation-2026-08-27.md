# Retained Native Wire Operation Intake

## Scope

The new `ui/contract/retained/operations/wire` cursor accepts one explicit native tag, exact node ID and owned field payload. Eight packed profiles use the existing retained typed decoder; the child field uses its native u64 buffer; two scalar tags require no payload. It does not decode a whole operation array. The typed payload remains owned until `OwnedUiOperation` captures its exact normalized owners. Decoder and redundant payload retirement finish before the operation can transfer to the surface patch.

The test drives all eleven tags through an actual `OwnedUiSurface` patch. Source view identity and the absence of ACK are checked after every operation. Final publication includes paired prepared-scene state, and the resulting hash/ACK is compared with Node Buffer plus Immer/JSON serialization. This is a composed class test, not mounted React/WGPU transport credit.

## Executed Tests

Canonical command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedWireOperation'`.

- R1: missing-module collection RED, four failed suites, zero executed tests.
- R2: one PASS, 597 skipped, 598 discovered; 10.77 seconds. All eleven tags and publication/ACK law executed.
- R3: one PASS, two FAIL, 597 skipped, 600 discovered. The hostile/cancellation test reached its final phase-name assertion (incorrectly expected the outer capture phase rather than the decoder's reported `typed-ready`); the forged operation constructor was actually accepted. No claim that the retirement-constructor assertion executed on this RED: the earlier assertion stopped that test.
- R4: three PASS, 597 skipped, 600 discovered; 13.26 seconds. Ten hostile cases, maximum 128 child slots, every admitted decode/capture/retirement prefix, zero-grant close, actual operation mint rejection and getter non-access all executed. `OwnedUiOperation` and its retirement now require the module-private mint before reading supplied state.
- R5: native child representation RED: one PASS, two FAIL, 597 skipped. The previous array-only input rejected the real BigUint64Array field; the accessor-array rejection expectation also exposed that permissive path.
- R6: one PASS, two FAIL; ordinary native BigUint64Array buffers crossed a test realm and were incorrectly rejected by `instanceof ArrayBuffer`.
- R7: one PASS, three FAIL. In addition to R6, the new shadowed-view law proved that reading public `input.buffer` could transfer an unrelated buffer while leaving the real view attached. The exact assertion observed the real 42-byte view still attached.
- R8: four PASS, 597 skipped, 601 discovered; 8.21 seconds. Intrinsic typed-array and ArrayBuffer getters now bind the actual source allocation; no public view metadata is read, and unrelated buffers remain attached. This preserves SharedArrayBuffer/subview rejection and native maximum child admission.
- R9: four PASS, one FAIL; the new streaming patch owner did not exist yet. This was an executed test-body RED, not a missing-module collection failure.
- R10: five PASS, 597 skipped, 602 discovered; 6.67 seconds. One operation slot, exact ordinals, non-consuming offer refusal, page receipt versus patch ACK, prepublication cancellation and late committed cancellation all executed.
- R11: five PASS, one FAIL, 597 skipped, 603 discovered; 7.29 seconds. A real `node:vm` Uint8Array failed at the outer public operation `instanceof` check. The subsequent BigUint64Array case was not reached on RED. R12 source routes unknown values to the same intrinsic admission instead; no realm-specific early gates remain.
- R12: six PASS, 597 skipped, 603 discovered; 7.50 seconds. Both direct public-entry VM fields execute and transfer correctly. Targeted `git diff --check` exits zero. Nx prints its historical flaky-task heuristic after alternating deliberate RED/GREEN runs; the actual R12 process exits zero, and no retry was requested.

Root independently executed the six-test wire group: six PASS, 597 skipped, 603 discovered, 9.06 seconds. This is independent class/DOM-suite integration coverage, not a mounted artifact/browser claim.

Every output is retained in the same ticket as `renderer-owned-wire-operation-rN` text. Later results are appended after actual execution.

## Native Representation Evidence

Current `plugin/schema/component.wit` declares `node-id = u64`, `patch-set-children.children = list<node-id>` and packed fields as `list<u8>`. The existing real jco output in `FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/jco-jspi-transpile-2/interfaces/semio-framework-ui.d.ts` declares `NodeId = bigint`, `children: BigUint64Array`, and `Pack` as Uint8Array through its types module. This retained artifact has an older numeric SurfaceRef while current WIT uses a string surface, so it is evidence for this list representation, not a current full-ABI runtime certificate.

R6 source removes the permissive array assumption: the child cursor requires a whole non-shared BigUint64Array buffer and detaches aliases. Its maximum source buffer is 128×8=1024 bytes; the normalized fixed-slot output is at most 1024 scalar bytes. One scalar is checked per advance; partial source/output cleanup claims 2112 bytes. The constructor's fixed-size reservation/transfer is an explicit admission boundary, not a generic large-array allocation certificate. Node IDs above MAX_SAFE_INTEGER are rejected, never narrowed to u32 or rounded. Unknown tags, extra scalar payloads, unsafe target IDs and wrong payload kinds fail before byte transfer.

## Remaining Live Joins

`PluginRuntime.applyRetainedWindowPatches` still calls the synchronous whole-array decoder, then old apply/hash/publication. The generic actor `wire-turn.ts` used by WGPU still contains older recursive replace/insert operation interpretations; it must not remain a parallel live escape after cutover. The current shard result callback resolves an entire `unknown` result, and WIT lifting/worker structured cloning remains an explicitly unaccounted transport allocation/copy boundary. A retained per-operation input owner is not proof that those copies are paged.

The next owner must keep one admitted wire operation, exact patch source/revision and page ordinal, and defer native ACK until the paired surface publication and mandatory notification/retirement obligations complete. Rejected offers must not consume buffers, and cancellation must retain admitted input through close. Actual actor activation plus u32 instance plus surface routing and aggregate teardown still need mounting in both consumers.

`OwnedUiWirePatchCursor` now implements that one-slot stream over the exact existing surface candidate. `offer(ordinal, nativeVariant)` reads only the fixed native fields using own data descriptors, rejecting inherited fields/accessors. A completed operation has a separate ordinal receipt, which must be taken before the next offer. This receipt is not a native publication ACK. `finishInput` requires all declared operations and receipts; only the underlying paired surface publication can produce a patch ACK. Late cancellation continues committed obligations and blocks close until the ACK is taken. The stream is not yet mounted into the existing whole-result shard callback.

The prepared host boundary also remains open: fourteen supported host schemas contain nested JSON and TextEditor still rebuilds generic pack synchronously. No compatibility materialization or empty fallback has been added.

## Activation-Owned Aggregate API Checkpoint

Root approved the next aggregate. It must capture the actual `ShardActorActivationLease` object and canonical `ActorInstanceLifetime` with activationGeneration, instanceId and native-issued guestLifetime. Same-activation ID reuse (guest 13 to 14) must not reuse an aggregate. The canonical TS lifecycle producer is being replaced by Demonstrator; the UI implementation will import that schema type once its three-field checkpoint lands, not duplicate it locally.

Proposed concrete host participant: `OwnedUiInstance(activation, lifetime, limits, profile)`; at most one `beginSurfaceLookup(surfaceId)` cursor; stable frozen read-only surface facades; `beginPatch(exactFacade, base, revision, count)` returning an owned native stream; explicit `beginClose`, `closeStep(1,4096)` and `terminalIsEmpty`. Receipt access remains available during close. The aggregate owns the wire stream before the surface and waits for issued React/scene roots; it never looks up a replacement by actor name or numeric ID. Surface lookup compares one cell per advance. The 512-byte surface-name admission is grounded in `document.rs` SurfaceId(UiText) and `action.rs` UI_TEXT_MAX_BYTES, not a new document limit. A new language-neutral fixture records these laws before implementation.

The first aggregate source now imports the canonical three-field lifetime. Lookup and patch admission compare the exact captured activation object plus all native lifetime fields. The frozen surface facade has no mutation-owner accessor; private cell/mint checks reject foreign facades. Close intentionally does not call the revoked operation authority.

Root identified the crucial outgoing ACK distinction: ordinary `captureActorActivation.turn` is revoked for new operations and must not drive the old guest's close-time UI ACK. That initial unmounted draft method has been removed. The outbox retains patch ACKs with no discharge API until Demonstrator's dedicated lifecycle-authorized submit owner lands. There is no callback/boolean-success fallback and no claim that an aggregate with published ACK obligations can currently reach terminal.

Native `UiNodeId(pub u64)` and `UiRevision(pub u64)` admit values above JS safe integers; the current renderer rejects them exactly. The safe-53-bit fixture is not full native u64 parity. Root owns the later consistent schema-domain decision or wide-identity implementation; no rounding or silent u32 narrowing was introduced.

## Native Known-Field Retirement Constructor

`OwnedUiWireOperationCursor.fromNative(value: unknown)` now routes the same fixed own-data-descriptor parser used by live `OwnedUiWirePatchCursor.offer`. A caller can immediately call `beginClose` and advance `closeStep(1,4096)` to retire a known native field without first materializing its semantic operation. Constructor failure leaves an unaccepted original with its caller. This closes only the selected known field, not arbitrary wrapper properties or unknown nested payloads.

Canonical `OwnedWireOperation` R13 executed six PASS and one FAIL, 608 skipped, 615 discovered, 9.52 seconds: the new test reached missing `fromNative`. R14 executed seven PASS, 608 skipped, 615 discovered, 16.82 seconds. It covers packed node, native BigUint64 children and scalar cancellation; an invalid target leaves its buffer attached, an accessor is rejected without invocation, and an 8192-byte unknown wrapper payload remains intact under a Node Buffer oracle. This deliberate preserved-extra law forbids treating cursor terminal state as an arbitrary raw-record erasure certificate.

The newer exact native-source/ACK/host-witness implementation and its input-token stage are recorded in `renderer-owned-instance-2026-08-27.md`; the earlier proposed raw aggregate signatures above are historical, not the current production API.
