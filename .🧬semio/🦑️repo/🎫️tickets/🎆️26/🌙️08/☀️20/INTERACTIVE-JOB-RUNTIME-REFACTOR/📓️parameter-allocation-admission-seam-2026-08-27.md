# Parameter Allocation Admission Seam

## Existing Authorities Inspected

- `framework/modules/job/🦀️component.rs`: `JobPayloadOperationLedger` is private and admits fixed 16-KiB payload pages, up to 256 pages / 4 MiB per operation, with a 64-MiB process total. Its five streams are checkpoint, preview, commit state, commit output, and fault. It tracks the actual retained page owners and releases their exact credits. It is not a typed-vector allocation ledger.
- `StepContext` owns that private ledger but exposes no arbitrary byte-allocation reservation capability. Its ordinary budget contains fuel and a deadline, not a memory grant.
- `ToolExecutionContract` bounds raw wire, decoded item count, per-step work, output bytes, and time. It has no candidate-memory admission authority.
- `ArtifactOwnedToolJobRequest` supplies an exact operation, captured roots, typed command, raw input, and app-instance owner capability. It does not carry an allocation lease. `ArtifactInstanceOperationOwnerHandle::with_mut` can address one concrete instance-owned ledger, but no such allocation ledger is currently implemented.
- `ArtifactStoreOneItemPreparationRequest` supplies the immutable base read, moved mutation, metadata, and private live authority. It has no app-instance allocation capability shared with the producer.
- Store preflight limits are exactly 65,536 work items and 1,048,576 retained bytes. Canonical edit sealing has a separate 16-MiB encoded-envelope limit. These are existing admission bounds, not evidence that a whole arbitrary source root fits in one allocation.
- Host Wasmtime limits configure linear memory. They do not establish a native allocator bound for the same typed cursor and do not certify allocator latency.

## Reuse Decision

Do not borrow payload-page credits as a proxy for candidate vectors. Those pages have their own owners and release lifecycle; using their counters for unrelated allocations would lose accounting and could release credits while candidate memory remains live. The existing checked atomic reservation/rollback pattern can inform an implementation, but its page ownership model cannot be reused unchanged.

The copier's existing `FlowCopyAllocationBudget` correctly checks single and cumulative requested reservations and performs a dedicated uninitialized reserve phase. It neither reserves process memory nor establishes an eight-millisecond allocation bound. Its fixture timing logs are not maximum-envelope certification.

## Minimal Coherent Interface Proposal

If contiguous allocation admission remains necessary, introduce one runtime-owned retained-allocation authority with explicit configured single-allocation and live-byte limits. It is a resource budget, not a semantic widget-ID/document cap. Bind its child authority to exact operation and generation. Pass the same authority into the producer request and the Store preparation publication context; independently constructed local counters cannot establish a shared limit.

The authority must own or inseparably wrap the admitted allocation, not merely hand an application a freely releasable numeric credit. A checked allocation object should reserve the exact layout, perform the actual allocation, retain the token beside the backing, and release credits only when that backing is transferred to an equally-accounted persistent owner or retired. Failed allocation must roll back only its own reservation. Cancellation, publication transfer, retry, and final-owner retirement must keep the allocation and token inseparable.

The copier needs a reservation operation at its existing dedicated reserve phase, not a callback that performs an unbounded clone. A completed candidate must transfer both backing and accounting ownership through Store publication; dropping a job cannot return credits for a published candidate still owned by the document. This means the current bare `String` / `Vec` destination ABI does not by itself preserve global accounting lifetime. A lease attached only to the short-lived copier would be incorrect.

## Admission Is Not Latency

Even correct shared accounting does not make an arbitrarily large contiguous allocation a bounded step. Full-domain interactivity requires bounded-page persistent variable collections, or a declared maximum contiguous allocation with an executed isolated release-mode maximum-size and maximum-total allocation gate under the unchanged eight-millisecond watchdog. No such maximum-allocation evidence exists for this packet. A larger logical byte grant or accumulated credit is not a substitute.

The current parameter producer can avoid copying unrelated generation content by sharing `GenerationPlayRoot`; selected widgets can share ordered map/Dictionary roots. It must still account for selected ID/label storage and the Store candidate's variable vectors. The remaining arbitrary contiguous-root boundary should remain explicit until the chosen backing representation and executed latency gate justify it.

## Adjacent Exact Ownership Requirement

The generic retained-command shell currently retires command, snapshot, config, history, interaction state, hover, and context through whole-owner removal. Dedicated parameter jobs cannot inherit that as an implicit bounded close proof. Snapshot retirement has an app hook; exact command retirement now has the latest-wins disposer hook; the other captured roots still need their concrete runtime/Store return or typed retirement paths. No generic wrapper is credited by this research.

## Status

Read-only inspection and interface proposal only. No new framework allocation API was added, no grant or watchdog was raised, and no full-domain allocation timing claim was made. Source-backed primitive and parameter payload work remains available independently of this unresolved admission/lifetime seam.
