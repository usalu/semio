# Lease Store Preparation Static Audit

Source-only audit while actual captures remain live. No SQLite database, queue lock, Store/factory, native image, initializer, process sample, cache cleanup or signal was opened/executed by this reviewer. Stage labels do not establish lock ownership or native work. Live sources were not edited.

## Defining Order

Build runOwnedBuildInvocationV1 at73 opens the leases image under original repository:lease-store control and keeps it open through diagnostics/dependencies Store callbacks and selected build. Each non-leases open recursively opens another leases image at opening36, then its requested image, closing requested image before leases in finally.

Inference nodes/dependencies route into withRepositoryBuildGraphFactStoreV1. Its storage module81 opens a `withRepositoryCacheStoreV1(...,"leases",invocation,...)` callback, derives original cache:leases-store control at store34, acquires an exclusive `repository-graph-facts:<workspace>` queued lease, then opens graph image/Store. That writer scope closes registered handles and images before releasing the exclusive graph lease. Concurrent independent inference invocations intentionally serialize on this workspace graph-facts resource.

Store callback reentrancy at33 applies only when its AsyncLocalStorage already has the same partition and exact invocation, and live callback is validated. The initial bare leases image in build73 does not register that Store scope. Thus nested opens can hold independent shared references to the same leases image. This is a concrete static nesting path, not proof of a live deadlock.

The leases partition opener21 uses separate controlDirectory and bounded direct resource-lease database, avoiding queued image custody inside its own queue namespace. It takes image shared custody first. For an existing/mounted valid image that shared reference remains until close. Missing/unmounted image opening releases shared before exclusive initialization/attach and reacquires shared afterward.

For missing/zero queue catalog, opening23 asks exclusive image maintenance and then reopens. In a waiting maintenance implementation, a nested opener releasing only its own shared reference could wait for another shared reference held by its own outer invocation. Whether this branch occurred, which loaded API was used, and whether the outer reference actually remains live require actual evidence unavailable from stage labels. No catalog size/current queue was read here.

## Canonical Queue Custody

resourceQueueCatalogPathV1 derives one canonical catalog per namespace from the retention resource. Retirement221 holds catalog exclusive, examines a bounded sorted claims batch, and attempts each claim exclusive with timeout0. A busy live claim is skipped; it is not inferred dead from PID or ticket age. Inode checks bracket ticket removal, release and claim unlink. Metadata headroom custody is retained and rechecked before catalog release. Queued acquisition first runs bounded retirement, then holds catalog shared only while acquiring its kernel-held claim and publishing its ticket; it releases catalog before waiting/holding the actual resource. Shared cohorts admit only prior explicit shared-cohort tickets. These phases must not be replaced by raw queue deletion or direct SQLite manipulation.

## Current Conditional API Defect

Current opening23 still calls `image.withExclusiveMaintenance(async()=>...)` with one argument. Current platform contract25 requires `(request,body)`; Mac implementation58 validates the first argument with readNativeImageMaintenanceRequestV1 before acquisition. Therefore current bytes reject that callback as an invalid closed request if the missing/zero catalog branch is entered. This is a definite conditional Source mismatch, but it cannot explain the live wait without loaded-source/branch evidence. It warrants a future test-first owning correction after actual termination; no hot patch was made.

## Qualified Next Evidence

Possible live explanations include waiting for intentional workspace graph-facts exclusive custody, image shared/exclusive transition, catalog acquisition, or Store reservation recovery. The cache:leases-store prefix alone cannot distinguish them. The exact awaited resource/mode and existing original progress path are needed; PID/inode/stat/process samples are diagnostic observations, not authority or permission to retire a claim. Self-nesting is a static risk only until the catalog/transition branch and same original outer hold are demonstrated. No writer attribution is made.

Current observed hashes:

- Cache opening `1b4f6e65de88ac511ac00408bdcb725d8420b02a6cf3150d599accb9e1b2ec50`.
- Mac image `4f747a6186dea7418d1a93a14c3377164262f4846140aa65f1cfa2a29bf212ec`.
- General leases `9c30f9d5bb1ff1c0914dee648d0de3a259c68a173d709304dd424d89417202e5`.
