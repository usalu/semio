# Descriptor Retirement Admission Audit

This is a read-only Source audit during the bounded-node-file implementation. No Native image, compiler, cache-pressure workload, OS inventory or deletion command ran. Existing physical/default SDK/review holds remain.

## Observed Production Paths

- `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` constructs the genuine compiler Store, retains a private WeakMap owner and Native reservation, and enters `withCompilerWorkspaceLifetimeV1`. That lifetime is accepted allocation custody, not a global compiler writer/read lock. Its recovery callback is merely retained for a later writer.
- `📏️capacity/🟦️.ts` enters the actual writer only in `withCompilerWorkspaceWriteV1`; `withCompilerInitialHeadroomV1` may call the retained recovery callback while that writer is held. This corrects the tempting but false assumption that Store construction already completed headroom maintenance.
- `📏️capacity/♻️recovery/🟦️.ts` currently recovers build/target data for prototype/release through `retireInactiveCargoStoreV1`. `🧹️pruning/🏪️budget/⚙️execution/🟦️.ts` admits exactly buildDirectory plus targetDirectory, scans Cargo build/incremental units, and uses real writer, physical locks and process/descriptor inventory. It does not enumerate or retire normalized descriptor nodes in captureDirectory/units. Its physical diagnostics are not permitted under the current Source-only hold.
- The actual selected backend holds a per-mode descriptor namespace SH reader across preparation, publication and the backend consumer. The producer and unit CLI also retain the same reader, with accepted consumer drainage before release. Node file writers use a separate exact-file EX lease.
- `withRepositoryCompilerArtifactCacheV1` already authenticates the exact genuine Store, Original, carried control, reservation and accepted workspace borrow before opening the actual bounded Native artifact cache. That is an existing integration seam; it is not permission to replace a missing child/parent Kernel with a fake volume or a parent alias.

## Lock-Order Consequence

Adding descriptor cleanup to the current writer recovery callback is unsafe: the callback can run beneath the caller's already held namespace SH. Attempting namespace EX there would wait for its own reader. Releasing namespace SH between publication and backend consumption would instead expose a deletion gap. Both approaches are rejected. No such implementation was added.

A standalone descriptor maintenance operation must enter before its own descriptor reader, authenticate the real Native/Store allocation, and retain namespace EX through its accepted work. If a compiler writer is also required, its order must be namespace EX then writer; writer then namespace can deadlock against an existing namespace reader waiting for the writer. Maintenance must not start traversing another mode's namespace while already holding the first mode/writer. All expensive work needs bounded iteration, progress/cancellation and the existing Original's remaining budget. A caught failed Original is not a retry mechanism. Controlled pressure must be a declared DATA outcome if admission needs to unwind and retry in the same healthy Original.

## Next Implementation Decision

Prefer integrating descriptors as exact immutable entries of the existing bounded artifact infrastructure over creating another uncharged directory cache or an independent ad-hoc LRU journal. First verify the actual Core volume policy, child/parent Kernel custody, path/locator contract and held-reader semantics. Root compiler descriptors currently have Rust-image file paths; moving to another managed partition requires a coherent schema/producer/reader/CLI/Nx locator change, not an alias, symlink, legacy decoder or forged authority. Existing genuine compiler-cache entry APIs may provide the same-parent route; their capacity and retirement behavior still require actual qualification.

If an interim same-directory retire path is necessary, its schema must explicitly cover the metadata admission bound, canonical node layout, all still-referenced task roots, file/inode recensus, unknown entries, interruption, accepted drainage, and free-space confirmation before any physical claim. A full scan with an arbitrary early limit cannot be advertised as recovery when historical metadata can exceed that scan frontier. Ages alone do not establish inactivity or authority.

## Scope of Current Progress

The existing-node publication check now uses the bounded exact Source reader, which streams a finite hash without retaining the node body, rather than allocating from a pathname after an earlier lstat. Direct CLI predecessor lookup now indexes once. These improve the actual production path but do not implement the retirement/admission protocol above or qualify the fixed physical repository/cache envelope. The broad goal stays active.

## Follow-Up Interface Inspection

Direct inspection of Core `📦️artifacts/💾️cache/💾️volume/🟦️.ts` changes the reuse decision: `NativeArtifactCacheVolumeV1` is an opaque compiler-state workspace, claimed once through `withNativeArtifactCacheWorkspaceV1`; it exposes directory/incremental/workspace facts under genuine child-image and parent-allocation custody. It is not an immutable blob put/get interface. Opening one of these producer/state images for every small descriptor would repeat large physical envelopes and is not an acceptable descriptor-cache implementation. The existing compiler cache seam remains useful custody infrastructure, not a drop-in descriptor pool.

The repository immutable artifact publication layer instead authenticates an actual `artifact-publication` reservation and opaque destination, then stages finite files with accepted cleanup and physical publication guards. It is a more appropriate publication primitive, but it does not by itself provide the descriptor reachability catalog, eviction policy, bounded catalog recovery or active-reader pinning. A descriptor cache must compose the real publication/inventory/admission primitives and qualified Native storage, rather than alias the compiler-workspace grant or assume that an API named artifact cache already supplies immutable entry retention. This finding narrows the next implementation seam based on actual code.
