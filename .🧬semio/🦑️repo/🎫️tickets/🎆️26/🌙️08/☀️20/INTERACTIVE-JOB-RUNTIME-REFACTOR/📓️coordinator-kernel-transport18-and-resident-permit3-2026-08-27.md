# Coordinator Native Transport and Resident Permit Review — 2026-08-27

## Actual native evidence reviewed

The coordinator read the executor's full raw common-Kernel R8 output: **18 passed, 0 failed, 235 skipped, 0.334 s, exit 0**. It includes the original 12 ownership/transport laws, all three previously observed held-lock transport failures, and the external-handback, poison and exact-byte laws. The controlled mutex-poison panic is caught by the test, whose actual result is PASS. This is the common Kernel target, not the unrelated OS-kernel cfg(test) target, Plugin integration, Wasm or complete guest close.

Raw: `🧪️member-kernel-turn-transport-green-r8-native-2026-08-27.txt`.
Executor report: `📓️kernel-turn-transport-green-r8-native-2026-08-27.md`.

The coordinator also read the complete neutral UiResidentPermit implementation and the R57 report/raw output. Its actual isolated native result remains **3 passed, 147 skipped, 0.400 s**, with the source oracle reporting 63 checks. Nine reservations are not nine rendered surfaces.

Raw: `🧪️member-ui-resident-permit-green-r57-native-2026-08-27.txt`.
Executor report: `📓️resident-permit-green-r57-native-2026-08-27.md`.

## Source review

Transport producer and checked-out-lease Drop publish into a preowned fixed handback cell without acquiring the transport arena. The tuple has a compile-time <=4096-byte size assertion. Normal progress/claim/close uses try_lock, reports contention without completion, and treats poison as a fault retaining ownership. Exact session/epoch/slot and external-owner state gate admission and handback; a transport slot cannot become vacant while its external handback remains outstanding.

This does not close the separate raw ownership gaps: UiTurnPatchTransportLease::take_owner makes the transport slot vacant when it transfers the UiTurnPatches root, and UiTurnPatches::into_iter still exposes a raw patch after removing it from that root. Canonical return paging and its native descendant owner must join these lifetimes rather than treating a transfer as final retirement. Native host WIT conversion still has the separately recorded whole-output discard seam. No final guest receipt or live UI publication claim is inferred.

UiResidentPermit maintains one fixed 64-slot ledger, unchanged 32 MiB aggregate/8 MiB per-reservation and item limits. Root/output owner bits preserve paired credit; deferred Drop only sets its affine bit. A slot remains occupied until exact release under the ledger mutex, and pending return bits prevent admission reuse. Successful explicit close clears the permit's private key while the ledger lock is held, disarming Drop. try_shrink is restricted to the unsplit sole root owner. These source properties agree with the isolated held-lock, reuse, paired-owner and cross-worker tests; they do not establish real reconciliation adoption.

## Deliberate unresolved integration failure

The executor's actual runtime resident-join R32 is **0 passed, 1 failed, 99 skipped, 0.056 s**, not a compile error:

```text
[DEBUG] runtime-resident-join expected-bytes=65536 observed-bytes=0 expected-slots=1 observed-slots=0
the runtime must not retain an independent second aggregate ledger
```

The old runtime ledger is being replaced, not retained alongside a second live quota. Original runtime R30 (allocation before resident admission) and R31 (whole 32 KiB compare/copy without allocation debit) remain required red acceptance gates. Real nine-surface coexistence, old/candidate/output overlap, cancellation, retained readers, final-root-associated credit and unchanged Process limits still require actual integration tests.

## Source census at review

These are read-time hashes, not retroactively asserted pre/post hashes of the executor's earlier runs.

```text
57617af2b6cef3ae37b3e00cc1863f0df61027c87a196cf5b2f81e813f3b84ee  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️component.rs
de04f4a06a0828fd4b93581af29bc1aa0b68f4e31e501161f7506ec33fa48599  🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs
```

No native compiler, cleanup, deletion, relocation, output publication or production edit was performed by the coordinator.

## Runtime Shared-Ledger Follow-up R33–R34

The executor has now replaced the old duplicate runtime ledger with UiResidentPermit. The coordinator read both complete raw outputs: R33 **2 passed, 99 skipped, 0.022 s** and R34 **3 passed, 98 skipped, 0.051 s**, both exit 0. The shared reservation observed exactly 65,536 bytes and one slot; held resident-ledger return preserved credit and bounded maintenance resumed. Patch handoff also preserved exact slots, rejected ACK ownership and three controlled unwind frontiers.

This supersedes R32's separate-ledger failure only. Canonical root/final-reader binding, original R30/R31 and complete rendered-surface admission remain open. The new source review additionally records that close_surface_reconcile_handback_one still blocks on the separate handback mutex, recovers its poison and discards a drain_one error. A genuine held-handback/poison regression has been assigned; held resident-ledger proof is not evidence for that other mutex.

Evidence: `📓️runtime-resident-join-green-r33-r34-native-2026-08-27.md`, `🧪️member-runtime-resident-join-green-r33-native-2026-08-27.txt`, `🧪️member-runtime-resident-patch-r34-native-2026-08-27.txt`. These are executor-native runs with coordinator output/source review, not a coordinator native rerun.

