# FEM Process Backing Owner Boundary Review

## Observed Scope

The accepted paged-model slice owns actual node/element/support backing locally and accepts an explicit maximum allocation grant at each engine admission call. Its three actual backing totals and bounded release are tested. That does not establish process-wide memory admission.

FEM3D `Fem3dBackingCredit` is an ordinary per-instance struct with admitted/live item and byte counters. Its constants are named `FEM3D_PROCESS_BACKING_*`, but the inspected implementation does not aggregate instances globally. It currently covers solver pages and two visual-order arrays. Numerical model, mesh/assembly/solver working memory, identity strings and the outer numerical child are not all charged to it. The current model caller supplies the engine's4096-byte per-backing ceiling; it does not consume a general process reservation.

The shared job module has a real atomic process total for `JobPayloadOperationLedger`. Its policy is specifically bound to five payload stream kinds and fixed16KiB payload pages. Reusing a checkpoint/preview/fault stream to represent arbitrary model or inflater backing would place ordinary working memory at the wrong semantic level. The inspected job/plugin files do not expose a general working-allocation ledger.

## Required Owner

Further read-only inspection found the shared trace memory module. It owns guest linear-memory/contiguous-request ceilings and a test-only process-wide `HeapWitness`; neither is a production allocation reservation ledger. Its comments explicitly distinguish measurement from admission. A focused search of the Store root and existing native aggregate-backing law found no matching generic ledger declaration, but this remains a scoped inventory rather than proof about every framework module.

Shared process and operation working-memory admission belongs in the domain-neutral execution framework. Concrete FEM owners should declare logical maxima and ask that shared authority for one physical allocation opportunity. The backing owner must retain the admission reservation through cancellation, rejection and owner transfer; physical close releases the actual accounting only after the backing is freed. Logical item removal cannot return physical credit.

The contract must distinguish requested demand, reserved ceiling and actual allocator capacity. Allocator rejection and an unexpected actual capacity beyond the reserved ceiling must retain a first fault and the real allocation until close; silently trusting `try_reserve_exact` or clamping reported capacity is insufficient. Multiple operation generations must not share a release identity. Moving model ownership into construction and assembly should move its physical credit with it.

A read-only cross-framework inventory is still needed before choosing the shared API. This report does not claim no such authority exists elsewhere, select numerical quotas, or implement a new general ledger. It records why another FEM-local counter or a JobPayloadStream alias would not complete the required abstraction.

## Evidence

Source inspection: shared job `JobPayloadOperationLedger`, FEM3D `Fem3dBackingCredit`, paged list query/reserve/release API, and the new engine `MountedAnalysisModel` admission and exact backing report. The accepted model native run is `fem-assembly-physical-native-green-1.log`; the full child post-paging integration is separately running.
