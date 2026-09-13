# Working-Memory Admission Owner Audit

## Root Follow-Up

The initial inspected set omitted the generic value-resident and UI-resident authorities. See [working-memory-resident-reuse-review.md](working-memory-resident-reuse-review.md) before choosing a new shared primitive. The generic resident module already provides substantial composition admission and exact native allocation machinery, although its current wiring does not govern arbitrary process-wide FEM/Pack backing. The original audit below remains scoped to its listed inspected mechanisms.

## Decision

**No existing shared framework authority may govern general retained working-memory admission.** The inspected mechanisms either measure memory, impose an instance-local ceiling, limit queue or task slots, or own a deliberately narrow resource class. None carries an authoritative, process-shared reservation through allocation, owner transfer, cancellation, and physical cleanup for FEM model/mesh/solver backing and Pack source/catalog/value/inflater backing.

Do not repurpose the job payload-page ledger, `Fem3dBackingCredit`, trace heap witness, actor `memory_bytes`, Wasmtime store limiter, or Store retirement slots as that authority. They have incompatible units or lifecycle semantics. A domain-neutral execution allocation authority is required at the shared runtime boundary, with concrete owner adapters for each physical backing owner. This is a boundary decision, not a proposal for a replacement API name or shape.

## Scope and method

Read-only audit on 2026-09-13. I read the two prerequisite reviews first:

- [fem-process-backing-owner-boundary-review.md](fem-process-backing-owner-boundary-review.md)
- [fem-mounted-model-physical-owner-next-slice.md](fem-mounted-model-physical-owner-next-slice.md)

I then inspected the shared trace, job, actor, async worker, OS Store, plugin host/reactor, `PagedList`, Pack value owner, and their focused tests with `rg` and source reads. No Cargo or broad test command was run. The conclusion covers the inspected shared execution paths; it does not claim a repository-wide inventory of unrelated product-specific pools.

## Existing mechanisms and their boundary

| Candidate | Existing authority | Why it cannot admit ordinary retained working memory |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` `PagedList` | A concrete owner can query its next page allocation, reserve with a supplied grant, record actual `Vec` capacity, and physically release an empty page. | This is the correct **per-instance physical-owner** primitive, but it has no shared process ledger, operation identity, transfer protocol, or aggregate admission. Its `allocated_bytes` is local to one list. |
| FEM `Fem3dBackingCredit` and paged FEM owners | `FEM3D_PROCESS_BACKING_*` limits selected FEM3D owner pages per instance; current records distinguish admitted/live bytes and close locally. | The name is misleading: it is not an aggregate over FEM instances and does not cover all mounted-model, mesh, solver, outer numerical, or string backing. A caller-provided 4096 ceiling is a local ceiling, not a process reservation. |
| Pack source/catalog/value owners | Current Pack owners measure and retain their local PagedList or vector capacity; source/catalog/value reviews establish physical-close rules. | Mounted-session totals aggregate local owners only. They do not arbitrate simultaneous sessions or cross-domain FEM/Pack allocations. The inflater document is a design slice, not an existing authority. |
| `🧰️framework/🔨️modules/🧵️job/🦀️.rs` job payload ledger | `JOB_PAYLOAD_PROCESS_OWNED_BYTES: AtomicUsize` is a real process total for exactly five `JobPayloadStream` kinds and fixed 16 KiB pages. A rejected page has a retained writer and exact close path. | Its unit is a typed payload page and its private ledger is coupled to stream/page lifecycle. It cannot account for arbitrary `Vec`/PagedList capacity, allocator overgrant, owner transfer, or long-lived model and Pack backing. Sharing its counter would lie about the physical resource. |
| `FixedOperationRegistry` in job | Exact `OperationId` plus `Generation` keys, bounded cancellation and terminal close. Tests cover stale-generation/ABA protection and rejected-owner return. | Admission receives an already-constructed owner, so it is after the allocation opportunity. Its byte count is the admission snapshot; it does not reconcile changing actual capacities. `take` decreases retained bytes before the owner has physically closed. It is a lifecycle registry, not physical credit. |
| `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs` | Guest linear-memory/request ceilings and `HeapWitness` retained/peak atomics. | `HeapWitness` is explicitly test-only instrumentation around the global allocator. It measures allocations after they happen and associates none with a logical owner. Guest limits are Wasm/request limits, not native retained backing admission. |
| `🧰️framework/🔨️modules/🎭️actor/🦀️.rs` | `Budget.memory_bytes` and `Usage.memory_bytes`; metrics record the supplied usage. | These are budget/measurement fields. The inspected actor paths do not reserve physical bytes before allocation, maintain a shared atomic balance, or release capacity at terminal physical close. |
| `🧰️framework/🔨️modules/⏳️async/🦀️.rs` worker `PermitLedger` and pool | RAII permits bound worker concurrency; saturated submissions return their closure. | The ledger counts permits, not bytes or backing capacity. A worker job may retain memory after the permit or task turn ends. |
| Plugin host `BudgetLimiter` | Per-Wasmtime-`Store` `ResourceLimiter` rejects guest linear-memory growth above a configured maximum and records a high-water mark. | It governs Wasm guest linear memory only. It has no host-native FEM/Pack backing accounting, process aggregate, transferable owner credit, or physical-close release. |
| Plugin reactor executor | Local task-slot admission, operation/generation/cancellation identity, bounded close, and rejected task cleanup. | `maximum_bytes` is a step-work/close budget, not a retained allocation credit. Tasks are boxed before admission and the executor has no process byte ledger. |
| OS Store displaced-owner retirement | Store-local reservation slots have generations and exact consume/release rules; retirement delegates bounded close until terminal empty. | This protects local Store slot ownership and stale reservations. It reserves owner **slots**, not actual backing bytes, and offers no process-wide ledger for arbitrary owner classes. |

## Required semantics at the proper boundary

The shared authority must own the aggregate process balance. A FEM or Pack object remains the only authority that knows how to allocate and free its backing. The two authorities must meet at each actual allocation opportunity and at physical release.

1. A prospective physical allocation asks the shared authority before it calls the allocator. The request must identify the exact owner and, while operation-owned, the precise `(OperationId, Generation)`. A retained root with no live operation needs an equally stable root identity; a reused operation ID alone is not sufficient.
2. The authority grants a reservation against one process aggregate. The configured maximum is a **reserved ceiling**; it is not evidence that memory exists or that allocation succeeded.
3. The owner allocates, measures its actual backing capacity, and reconciles that actual capacity while retaining responsibility for every outcome. Requested bytes, reserved bytes, and actual capacity must remain separately observable. `Vec::try_reserve_exact` can overgrant.
4. A failed allocation or an overgrant cannot silently drop either the grant or the backing. The owner retains the accountable reservation/backing state, records its first failure, and exposes only its bounded close path until physical release is complete. The payload writer's rejected-page pattern demonstrates this requirement for pages, but not a general implementation.
5. Moving an owner between process, operation generation, Store, worker, or plugin execution context moves the same reservation; it must not create a fresh local counter or let the previous context release it. A stale generation may request cancellation/close of its own record but must never release a newer generation's credit. `FixedOperationKey` and reactor authority-generation tests demonstrate the identity rule.
6. Logical removal does not release aggregate capacity. For example, a `PagedList` pop can remove an item while its page remains allocated. The shared balance changes only after the concrete owner has physically freed the backing and reached the relevant terminal condition.
7. Close is idempotent and bounded. It returns physical capacity only after the concrete close step proves that backing was released. Rejection, cancellation, shutdown, and cleanup all use the same retained-owner close route.

This keeps payload stream quotas separate: job payload pages retain their existing page-stream ledger, while general retained backing uses a unit that represents actual native allocation capacity. It also keeps guest linear-memory caps separate from host allocation admission.

## Evidence from existing tests

The existing focused tests establish useful local laws, but not process-wide working-memory admission:

- `🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/🔬️list/🦀️.rs` checks rejected reservation leaves a list unchanged, logical removal retains page backing, and physical page release reports exact allocation bytes.
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️fixed-operation-registry/🦀️.rs` checks admission rejection returns the original owner and stale-generation/ABA close preserves exact authority.
- `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️tests/🔬️memory/🦀️.rs` checks that heap witness counters are readable/resettable, confirming instrumentation rather than admission.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs` checks Wasm guest-linear-memory high-water behavior, confirming the limiter's guest scope.

The implementation slice that introduces the shared authority needs language-agnostic acceptance cases for: aggregate exhaustion across two independent FEM/Pack owners; requested/reserved/actual divergence including allocator overgrant; logical removal with no credit return; physical-close credit return; transfer preserving one reservation; same-operation-ID stale-generation rejection; and rejected/cancelled retained-owner cleanup with no over-release. The existing owner tests can remain the concrete physical-allocation oracle, but cannot validate aggregate admission on their own.

## Decisive recommendation

Keep current FEM and Pack local backing ledgers as physical-owner adapters and keep job payload quotas, trace measurement, actor metrics, guest limits, and task/Store slots in their present scopes. Introduce the missing shared execution allocation authority before expanding ordinary retained backing beyond its current per-instance controls. Its contract must be settled against the requirements above before selecting any API; no inspected existing API has the correct unit, identity, transfer, and physical-release semantics to reuse unchanged.
