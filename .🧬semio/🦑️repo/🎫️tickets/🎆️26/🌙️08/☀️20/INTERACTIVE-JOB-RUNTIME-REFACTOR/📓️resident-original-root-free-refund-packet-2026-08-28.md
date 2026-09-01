# Original-Root Resident Free → Refund Packet

## Status and Boundary

Ticket-only schema/test preparation, 2026-08-28. No production/API/include/dependency edit, native command, or reference execution belongs to this packet. The seven Rust tests reference proposed private types and fields; they have not been compiled. The strict-Ajv/Immer controller is authored but unexecuted. There is no new passing result.

The immutable resident baseline remains Retained R8: **17 native PASS, 0 skipped, .038s**, and R9: **two Wasm compile checks PASS, .32s/.39s**, with all 64 selected tuples stable. Passing per-test stdout was not captured. Full evidence: `📓️resident-private-consumer-r8-native-r9-wasm-green-2026-08-28.md`. This baseline does not implement the phases below.

The separate Opening7 source remains unchanged. Root's new independent hardened Opening reference passed 7 cases/39 Immer transitions/2 hostile checks; that is source-model evidence, not native/API/Layout/cleanup execution. Historical R3 and its two earlier routing failures remain unchanged.

## Actual Current Release Paths

Canonical authority: `🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs`. Current `close_step` lines 192–300 calls `state.release(partition, charge)` before destruction/deallocation in all five cases below.

| Original structural source | Allocation state before closing | Descendants/aliases required empty first |
| --- | --- | --- |
| `LedgerState.retiring` / admission's `ErasedRecord` | Exact `RecordNode<S>` allocation, initialized or raw | Exact `Option<S>` empty, record aliases zero |
| `LedgerState.pending` | Reservation only, or raw `AdmissionNode` | Original `ErasedConsumer` detached in its own admitted step |
| `LedgerState.head` | Initialized `AdmissionNode` | Record transferred to release ownership; admission aliases zero; consumer reference detached; next link transferred |
| `LedgerState.pending_consumer` | Reservation only, raw or initialized `ConsumerNode<C>` | Empty initialized source; no published aliases or admissions |
| `LedgerState.consumers` | Initialized `ConsumerNode<C>` | Writes revoked, exact C empty, aliases and admissions zero, next link transferred |

`record_release`, `release_consumer`, and `AdmissionPage::release` currently combine `drop_in_place` with `dealloc`. `allocated_bytes` changes after that combined call but reserved bytes/slots/owners change before it. A later caller cannot observe a retained charge between the actual free and refund. That is the specific missing behavior, not a claim that the original seventeen tests failed.

Null-allocation failure and failure to acquire the preallocation consumer reference are different: no backing was allocated. Their checked reservation rollback must not generate destruction/free events or a fabricated freed-allocation witness. Existing consumer/admission failure can also leave a reservation structurally pending for cancellation. Preserve those original owners and existing R8 failure laws.

## Chosen Private Representation

One inline original-root slot **replaces** `LedgerState.retiring: Option<ErasedRecord>`; there is no heap ticket, vector, detached public receipt, new registry, Arc, Box, or second budget. Proposed declarations, not implemented:

```rust
enum ResidentReleaseOrigin {
    Record, PendingAdmission, Admission, PendingConsumer, Consumer,
}

struct ResidentReleaseAllocation {
    pointer: NonNull<u8>,
    layout: Layout,
}

enum ResidentReleaseStage {
    Destroy {
        allocation: ResidentReleaseAllocation,
        destroy_empty: unsafe fn(NonNull<u8>),
    },
    Free { allocation: ResidentReleaseAllocation },
    Refund { released_layout: Option<Layout> },
    Clear { released_layout: Option<Layout> },
}

struct ResidentRelease {
    origin: ResidentReleaseOrigin,
    partition: ResidentPartition,
    charge: ResidentResources,
    stage: ResidentReleaseStage,
}
```

The allocation descriptor is not Clone or Copy: only the original root's private stage is callable authority. No public constructor, cloneable release token, pointer-equality refund API, or external `free(pointer)` operation is introduced. After `System::dealloc` returns, the stage contains **no pointer or destructor function**. The original partition, full charge, and diagnostic Layout remain in that same root slot. `released_layout: None` is the reservation-only path, not a synthetic freed pointer.

Source descriptors retain their original metadata until the complete detach work is admitted. For admission/consumer lists, move the original next link back into the root and clear any prepared pointer during that same fully admitted detach. Busy release slot means leave source, links, counters, and charge unchanged. No new release can overwrite a Refund/Clear descriptor. The root serializes its existing source priority; this does not add an alternate retirement queue.

`ErasedRecord.release` and `ConsumerPage.release` become private empty-shell destruction functions instead of combined destroy/free callbacks. Admission uses its concrete empty-node destruction function. The neutral free operation uses the original `Layout` once. Generic `S::Drop`/`C::Drop` is never a terminal witness: their slots must already be structurally empty, and the node's owned links/references must already be detached. These generic functions only destroy the now-empty typed node shell. A live or unknown destructor must never enter this phase.

## Exact Transition Rules

| Step | Physical action | Original charge | Allocation/pointer after return |
| --- | --- | --- | --- |
| Detach | Move an eligible original descriptor and its list links into the one root slot | Unchanged | Destroy if initialized; Free if raw; Refund if never allocated |
| Destroy | Invoke only the concrete empty-node `drop_in_place`; then replace stage | Unchanged | Original raw allocation retained in Free |
| Free | Call exact allocator deallocation; on return replace stage and decrement actual allocated bytes | **Unchanged on all three axes** | Pointerless Refund; original Layout retained |
| Refund | Checked subtraction from the descriptor's original partition, then replace stage | Removed once | Pointerless Clear, diagnostic Layout and original charge values retained |
| Clear | Clear the original inline descriptor | Already refunded, no second subtraction | Root slot empty |
| Final root | Existing final-root work check; require all roots, charges, pointers and release slot empty | Zero | Original root may report empty; its containing parent remains separate |

Each row is a separate `close_step` call with at most one item. No telemetry, callback, task switch, allocator observer capable of panicking, or fallible operation sits between successful allocator return and pointerless-stage installation. All arithmetic and next-counter checks occur before physical mutation. `GlobalAlloc::dealloc` may not unwind; tests must not inject a panic inside it. This is not a timing proof: even correctly charged allocator work may exceed a deadline; the real callback-tail/clock authority is separate and remains unmounted here.

Cancellation revokes forward access but does not skip Destroy/Free/Refund/Clear. Dropping/moving a borrowed facade is not completion and cannot refund the root. Actual original-root caller loss remains a parent-lifetime problem: tests keep the real root outside fallible callbacks and do not claim that arbitrary last-owner loss becomes safe.

## Fields, Layout, and Admission Inventory

No native byte numbers are guessed or copied from TS logical prices. Exact sizes and alignments must be measured from the eventual compiled types, including enum padding and cfg(test) differences.

| Location | Proposed field change | Actual Layout obligation |
| --- | --- | --- |
| `LedgerState` | Replace `retiring` with `release: Option<ResidentRelease>` | Measure whole `ResidentLedgerRoot` and both old/new inline field layouts; do not assume their difference equals a sum without padding |
| `PendingAdmission` | None | Existing `Option<AdmissionPage>`, original consumer reference, partition and charge remain charged by their original owner |
| `ConsumerPage` / `ErasedRecord` | Replace combined release function pointer with empty-shell destruction function pointer | Measure complete source descriptors; no assumption of ABI size equality |
| `ResidentRelease` | Origin, partition, charge, discriminated stage above | One inline allocation descriptor only; original root holds it before any source is detached |
| `ResidentNativeLayout` | Add `release_slot_bytes`, `pending_consumer_bytes` for diagnostics | Compare to independent `Layout::new::<Option<ResidentRelease>>()` and `Layout::new::<Option<ConsumerPage>>()` in tests |
| Actual Opening composition | No change in this packet | Must preadmit the **whole changed root layout** and its stable allocation handle before producer allocation; old measured root price cannot be reused |

The existing allocation-free root constructor has one explicit externally supplied root-backing exclusion, not free descendants. Increasing its inline size requires the actual Opening parent to fund that new full root. Native tests derive only their isolated data capacity from the three actual page layouts; they do not fund a live parent, invent a composition total, change the 32MiB UI domain, or turn a numeric work grant into resident authority. Test-root bytes are externally held fixture storage, explicitly not charged a second time to that test data partition.

Checked work is a sum, not a `max` that hides two moved owners:

- Detach: actual source slot + full destination release slot + any next-link/prepared-pointer fields written. Every source kind has its own concrete sum. Pending-consumer reservation-only detach is `Layout<Option<ConsumerPage>> + Layout<Option<ResidentRelease>>`.
- Destroy: actual typed node Layout + full release slot written for the stage change.
- Free: exact allocation Layout + full release slot + the `u64 allocated_bytes` field written.
- Refund: full release slot + the actual `ResidentResources` partition counter written.
- Clear: full release slot.
- Final root: whole actual root Layout; no parent-shell or heap backing release is inferred.

The seven tests compute these phase values independently from actual Rust types and `Layout`, not from an API's `required_bytes()` returning its own oracle. They capture oversize/refusal before cleanup and assert afterward; the grant remains 4096. An actual type/layout requiring more than 4096 is an explicit blocked fit/design result, not permission to raise a grant or claim progress. Fixed cleanup bounds are derived from the three original page owners, their five declared phases, reference detach/revocation steps, root check, and measured layout units. Bound exhaustion is failure/incomplete ownership, not liveness evidence.

## Poison and Concurrent Access

Ordinary access keeps its one-attempt Acquire-CAS/Release discipline and sticky poison. Two callers cannot free or refund the same root stage; busy returns zero work. A close-only internal gate is proposed **only** for a pointerless Refund/Clear residue and the subsequent empty-root terminal check. It never clears the poison bit, admits forward work, invokes a destructor, walks remaining live pages, or promotes a public mutable Option to a parent capability.

The poison law first establishes an actual deallocation return with a pointerless original-root charge. Only then it acquires the actual gate and panics with a concrete unit payload; the allocator has already returned and is outside the unwind. After the guard is gone, exclusive test access reads the original root for observation, without resetting poison or mutating its receipt. The three separately granted refund/clear/final checks must finish that isolated pointerless root while forward access remains refused. `terminal_is_empty` must permit a read-only structural empty check under sticky poison; it cannot treat poison itself as emptiness.

**Poison with an allocation/live C/live S/unknown panic payload is not solved or exercised as successful cleanup here.** It must retain the original owner for the actual Opening typed quarantine/fault handoff. No leaked test root, cleared fault, generic panic-payload destructor, or forged empty witness is proposed to manufacture a passing test. The close-only pointerless exception requires root review before production mount; it is not currently implemented.

## Actual Native Test Packet and Instrumentation Boundary

Files: `🧪️resident-release/{🔣️.json,🧬️schema/🔣️.json,📜️script.ts,🦀️.rs}`. The schema is authored before the seven tests. Nine explicit cancellation frontiers cover consumer reservation/raw/initialized/published, admission reservation/raw/initialized, and record raw/initialized. Both the native frontier law and staged third-party model consume those same rows.

| Native law | Intended observation |
| --- | --- |
| `record_keeps_charge_after_actual_free` | Actual empty record destruction, actual System free, unchanged charge between calls, lost/moved facade leaves receipt, later refund and clear |
| `cancellation_covers_allocated_and_reserved_frontiers` | Nine original public construction frontiers; exact aligned Layout totals and counts; reservation-only has zero destroy/free |
| `short_grants_preserve_every_original_phase` | Zero items and actual work-minus-one leave phase/counters/events unchanged; exact work advances only one phase |
| `aliases_block_destruction_and_live_payload_drop` | Actual live record and original alias refuse close; same original payload transfers to external typed slot, alias retirement then permits node close; payload drops only after root cleanup |
| `concurrent_close_frees_and_refunds_once` | Actual other-thread call while original gate held is busy; two real close threads have one aggregate deallocation |
| `poison_after_free_keeps_pointerless_charge` | Actual guard unwind after free; charge remains, no pointer dereference, no second free, sticky forward refusal, narrowly permitted receipt cleanup |
| `metadata_is_inline_and_measured_before_detach` | Root construction makes no allocation; direct Layout matches diagnostic; exact source+destination short transfer refuses |

Native mounting later requires a child test module inside the **existing** allocator-owning resident test module. There is no second global allocator. Proposed cfg-only observer hooks:

1. After the concrete empty-shell `drop_in_place` has returned, call `observe_destroy_returned()`.
2. In the existing `ObservedAllocator::dealloc`, call `System::dealloc` first, then `observe_system_dealloc_returned(layout)`. Preserve old pre-call diagnostics distinctly; they do not prove return.

Observers use preinitialized fixed thread-local Cells, saturating counters, an eight-entry Layout array, and an overflow flag. No allocation, assertion, formatting, channel, lock, root reentry, callback, or panic occurs inside allocator hooks. Observation is enabled only around an actual `close_step` by a scope guard; thread startup/teardown and fixture allocations cannot masquerade as release events. There is no stored freed pointer and no authority minted from an event count. The actual root state is inspected independently after calls return.

The live payload and all available consumer/cell/record handles remain outside the fallible observation body. Cleanup attempts the exact retained handoff before dropping aliases; original external source/recovered slots are then retired before intended assertions. These are structural test destinations only, not original-parent funding proof. Preparation errors and cleanup refusal/overspend remain explicit test failures. No strict Store owner is introduced by this packet; the separate Opening7 retains those original stores.

## Store FIFO Join and Remaining Work

Retained's selected `ResidentRecord<ArtifactStoreBackboneRetirement>` remains the **one** shell allocation, with a typed entry in the same Store FIFO. No extra Box, retirement queue, or external allocation protocol is added. A future exact private Store/field/reservation binding must remain associated with the original root through Clear; deallocation alone is not entry completion. This packet's private origin enum is not that future binding or an exposed lookup key. Existing public `ResidentRecord` aliases cannot simply stay live through free; the actual private registered parent/FIFO binding must replace their access authority at the correct handoff, not bypass the alias barrier.

Next review sequence: approve this private representation/poison scope and hooks; then mount genuine missing-API tests into the canonical existing module under executor coordination; capture actual RED; implement reviewed phases; run new seven plus unchanged seventeen; only then join the already staged actual Opening parent and Store typed field. No current source-ready or native-ready claim is made by the ticket packet.

The authored reference can later run through the existing explicit one-project Nx exec route with an absolute ticket controller path. It is not run now, and no launch/production target is added for this temporary ticket model. Existing package metadata stays Taxonomy-owned. Prior source/native evidence is not overwritten.

## Selected Source Receipt

Read-only SHA-256 and Rust roster inspection completed after staging. These are source observations, not compiler/schema/test results. The staged Rust roster contains exactly seven `resident_release_` tests.

| New ticket file | SHA-256 |
| --- | --- |
| `🧪️resident-release/🦀️.rs` | `fc755c9bb83b5ddf915b270427595b607dc66c6c4e3f7eb418d5e2084d92e73a` |
| `🧪️resident-release/🔣️.json` | `076d975be11ba15eb2762c2314632a4bb99857fdb397046c069daca7810675c0` |
| `🧪️resident-release/🧬️schema/🔣️.json` | `2e9c7d9efb7a54fb416ec0890fb0b9a38cf365619d3afa6d16f2b53aa9177184` |
| `🧪️resident-release/📜️script.ts` | `dbde46a1cb14c51089de82d1f9c757af456c5edc208858034856e6af0b538e0b` |

Unchanged protected inputs observed in the same final read-only check:

- Resident authority: `508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f`.
- Resident existing17 tests: `ebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e`.
- Opening7 Rust: `01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1`.
- Opening fixture: `18a7d4d13790f59897fae10816672f543e971d229ee0a85e356188b8f7ebe729`.
- Opening schema: `089088860a36e347466d2be8269be9e340dbeeeecc86d0a8cb0a4afa1992111f`.
- Opening controller: `5b8b17927bbd4fc3c551a206d87477b200379aa6e695ca151fe6c55b349a3499`.

Owned changes in this packet are only those four new ticket files and this report. No Plugin/Opening/Kernel/Store/UI/runtime/resident production file was changed. No process was launched beyond read-only file/hash inspection; no compiler lease is held by this lane.
