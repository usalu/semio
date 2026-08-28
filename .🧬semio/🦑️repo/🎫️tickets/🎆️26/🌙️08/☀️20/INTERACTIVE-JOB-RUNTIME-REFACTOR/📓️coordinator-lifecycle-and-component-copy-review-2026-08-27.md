# Lifecycle and Component Copy Integration Review — 2026-08-27

## Verified Checkpoint

The coordinator read the retained actual native output, not only agent summaries:

- Canonical actor lifecycle codec R2: **3 passed, 97 skipped, 0.065 seconds, exit 0**. Shared independent LEB128 vectors, invalid authority before writing, and exact accepted identity are exercised. This is standalone codec proof, not guest reactor/WIT or app close completion.
- Canonical component copy R34: **2 passed, 130 skipped, 0.182 seconds, exit 0**, following missing-API RED. All eighteen component variants match native serde. Select32 cancellation exercises eight partial-copy frontiers and close grants 1/64/4096. The pre-Rust strict Ajv/Node Buffer oracle reports 25 checks across paging/bindings/copy. This does not establish active runtime adoption, retained equality, full resident accounting, Process fit, or callback timing.

Exact output remains in `📓️actor-lifecycle-green-r2-native-2026-08-27.md` and `📓️component-copy-runtime-checkpoints-2026-08-27.md`. No new independent native compile was started by the coordinator; the existing fleet compiler slot remains serialized.

## Component Copy Source Review

The entire new `ui/🧬️contract/📋️copy/🦀️component.rs` and its tests were inspected. Copying separates allocation from initialized/copied bytes, hides incomplete candidates, shares the typed field catalog with retirement, and retains source and partial candidate for bounded close. The canonical fixture supplies **32768-byte allocation and work grants**. `UiFixedBytes` requires a complete 32768-byte zero initialization before copying semantic bytes, and root candidate construction requires `size_of::<Component>()` work. Correction after direct constant verification: the existing active runtime `SURFACE_RECONCILE_PAGE_BYTES` is **32 * 1024**, not 4096; the earlier 4096 statement was an incorrect assumption. The new component-copy work regression deliberately tests smaller 4096-byte work slices separately from the existing physical admission ceiling. The executor's actual R37 RED measured inline Component at 3096 bytes (fits 4096) and isolated the whole byte initialization stall. The original two component-copy tests do not prove this smaller-work forward progress.

The executor owns a focused integration regression for actual runtime grants before adoption. No work grant or runtime quota increase is authorized as a substitute for paging. A live `ManuallyDrop` cursor must stay in a structurally retained owner during faults/unwind; suppressing Drop during unwind alone is not recovery.

## Native and Host Lifecycle Join

### Later Executed Foundation Gates

The coordinator subsequently read the exact R39 component/compare output: **5 passed, 130 skipped, 0.223 seconds, exit 0**. The 4096-byte Surface work regression now passes after the actual R37 RED. A private retained byte buffer reserves physical capacity separately, initializes/copies bounded slices, and moves into the boxed representation only after exact length/capacity equality. No ceiling was changed. Retained comparison covers all eighteen component variants plus seven value cases, grants 1/64/4096, arena contention and seven cancellation frontiers. Active runtime adoption and resident accounting remain separate work. Evidence: `📓️component-copy-compare-green-r39-native-2026-08-27.md`.

Native actor R4 is now **5 passed, 97 skipped, 0.050 seconds, exit 0**, including the two outer TurnResult tests after three expected compile errors and the previous three lifecycle tests. The existing outer Usage record is three fixed-width little-endian u64 values (24 bytes), not three varint zero bytes; the newly authored provisional vectors were corrected before GREEN. Empty/max-receipt TurnResult vectors are 30/74 bytes. Evidence: `📓️actor-lifecycle-all-green-r4-native-2026-08-27.md`.

Root independently executed OwnedInstance R1: **5 passed, 603 skipped, 608 total, 6.34 seconds, exit 0**, with matching before/after scoped source hashes. This precedes the later source/ACK integration tests and certifies only the five tested host aggregate laws. Full actual output: `📓️coordinator-owned-instance-r1-2026-08-27.md`.

The agreed canonical lifecycle uses the existing reactor event/poll path. A pre-open host owner retains the exact activation/worker. Native construction captures the exact app cell and supplies a fresh checked guest-lifetime serial before any host handle is exposed. Captured, Accepted and Retired receipts each require an exact ACK; one fixed optional receipt per turn remains retained across backpressure and faults. Final Retired ACK requires the actual host UI/ingress/publication join.

Actor outer-frame vectors in `actor/🚪️lifetime/🧪️fixture.json` cover None, Captured, Retired, and maximum-u64 Retired. The field follows existing command ingress, as length-prefixed canonical receipt bytes (zero for None; maximum 44); existing next-wake/status/usage follow. Open/Close/ACK wires are invalid in the receipt field. The outer frame is covered by native R4 above; Kernel/WIT converters are now source-mounted but the guest lifecycle aggregate/descendant reducer has not passed a Plugin integration gate.

UI owns privately minted `OwnedUiPatchAcknowledgement`; native transport owns privately minted `OwnedNativeUiPatchAuthority` and `OwnedNativeUiPatchSubmissionReceipt`. Submission must match both exact private identities, preserve both on refusal, and consume the UI outbox only after the actual captured ACK turn. A separate privately minted final host-retirement witness replaces the provisional structural `isRetired()` callback. Host-local input page release receipts are not additional WIT ACK messages.

## New Read-Only Findings Routed to Owners

The coordinator read the current ShardClient lifecycle implementation and sent two exact source findings to the demonstrator owner:

1. `beginInstanceLifecycleClose` still calls actor-name keyed `abortOutstandingEffects`, whose helper deletes the current actor-name ledger and synchronously invokes every controller's abort listeners. An old captured open owner after worker loss/rebuild must not cancel a replacement activation's effects. Cancellation must retain exact activation ownership and avoid synchronous listener fan-out during close admission.
2. The current ACK-result handler clears an acknowledged receipt when any returned object omits `lifecycleReceipt`; it does not first validate canonical turn status. Tests must distinguish malformed, refused and clock-fault responses from an authoritative ACK outcome. Missing output is not a final-retirement witness. Dag owns the matching native outbox/fault semantics; no second ABI is being introduced.

These are source findings, not new live guest failure claims. The current UI aggregate and ACK pairing are under active TDD and not yet mounted into PluginRuntime.

## Plugin Test Prerequisites and Preservation

The Interaction direct-leaf attempt R4 reached Rust and failed compilation with 89 diagnostics before tests. Mutation owns the fixture-derive and concrete test-mutation metadata joins; our fleet owns the missing typed-command fixture dependency at Plugin component line 17684. The prior source-verifier checkpoint-fixture repair was a different fixture and is not a valid replacement.

The remaining include references the absent active-ticket `🧪️shared-typed-command-full-operation-v1.json`. Its consumers require forty language-neutral output, grant, freshness, admission, publication, lane, raw-page, fault and close laws. No exact surviving production copy was found in the scoped search. Dag will use an exact authoritative copy only if one exists; otherwise a newly authored permanent domain fixture/schema will be recorded as new evidence, not reconstruction of lost history. The compile errors and missing-file fact remain recorded.

No cleanup, deletion, source restoration, modifying Git command, output publication, WGPU preimage repin, or browser-policy workaround was performed. Full all-app native/Wasm/browser, cancellation/replay, accessibility/platform and strict under-8-ms runtime gates remain open.
