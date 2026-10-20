# Native Cache Recovery Route Review

## Current State Supersedes The Earlier Failure

Root's native cache-status1 completed actual0. Its log ends repository:build complete and reports the selected Nx artifact cache absent/empty (0 entries, 0 bytes). It did not clear or reset that cache. Root subsequently started one documented exec dry-run retry, dry2, handle63416, log generated/2026-10-20-native-process-inventory-third-dry2.log; it is active at this checkpoint. No actual observer has run. Do not poll/cancel that Root-owned handle from this task or launch another lane.

Fresh filesystem-only observations found the current leases publication settled: image.json is314bytes, regular/single-link, SHA2564a5dde4d9404f89a46eb59dd93173d8994584e80719e370e50f69ab614f56c90, unchanged across two reads. It includes bootSession1E4A0CBE-3710-481F-9614-F8F1FCBD61D0 and7ancestry rows. All7inode values match the current ordinary, nonsymlink ancestor directories. Backing fixed.dmg is regular/single-link with device16777229, inode422334229, size66060288 and allocated66060288bytes; these match the current receipt identity and remain within maximumPhysicalBytes67108864. Mounted filesystem size66019328bytes is within receipt filesystemBytes66060288. The publication intent is absent. Root independently confirmed no intent or swap. The fresh inventory-third receipt is still absent.

The successful native cache status is the authoritative current admission result; this review did not execute a boot-session command or infer the active mounted root's underlying mount-directory inode. No current stale-leases admission blocker is established. The missing-field dry1 snapshot and earlier speculative retry hold below are historical, superseded where they describe current state. Continue the Root-owned dry2. Do not clear/reset, modify receipts, alter peers' cleanup Source, or create migration code. No historical receipt rewrite is attributed to Root: Root reports no such modification or cleanup was issued by it.

## Earlier Read-only Routing Findings

No existing native command was established that can safely inspect and retire only the stale leases publication .cache/b/l before ordinary build admission while preserving other agents' cache work. Do not try cache clear, reset, compiler retirement or another exec invocation as speculative repair. No command, cleanup, source edit or private builder action was performed by this review.

Root's already successful native prototype preflight proves host prerequisites only. The matching native --help and preflight paths bypass leases admission but neither inspects nor retires .cache/b/l. There is no additional safe cleanup argv to recommend from the inspected routing.

## Actual Failure

Root's immutable generated/2026-10-20-native-process-inventory-third-dry1.log fails before the observer: Exact canonical persistent Native publication required at .cache/b/l. The schema reports missing /bootSession and /ancestry. Filesystem and maximum physical sizes match (66,060,288 and 67,108,864 bytes); backing device identity differs in that logged tuple. Do not fabricate missing receipt fields or lineage from current host state. The dry-run did not execute the observation adapter or produce its inventory-third receipt.

## Routing Evidence

The native frontend routes cache and reset into the build owner, just like exec. In 📚️library/🏗️build/📜️script.ts, runOwnedBuildInvocationV1 handles help and preflight before opening leases. It also permits only the exact closed selectors returned by readNativeRepoRecoveryPlanV1 to bypass that outer leases opener. Every other route calls openRepositoryCachePartitionV1(root,leases) before runOwnedBuildInLeasesV1 reaches its cache or reset branch.

Consequently:

- repo --tool=nx cache status: opens leases first; then dependencies; its eventual purpose is selected backend task-cache measurement, not physical publication inspection.
- repo --tool=nx cache clear: same admission; eventually measures, clears the backend task cache, then measures again. It cannot reach stale-publication repair before the failure and can discard other agents' cached results if it later succeeds.
- repo --tool=nx reset: same admission; eventual Nx path invalidates the build graph and invokes Nx reset. Bazel reset similarly owns Bazel state. Neither is a physical leases-image retirement command.
- repo run repo:cache-retire-inactive-preview and repo run repo:cache-retire-inactive: exceptional early routing exists, but the owner is inactive Cargo/compiler storage, not .cache/b/l. InactiveCargoRetirementScript opens the rust partition, which recursively opens leases. Preview does not avoid that admission and is not a safe stale-leases inspector.
- repo run repo-mcp:build and repo run repo-mcp:install: exceptional native frontend recovery exists, but its owner is frontend publication. It does not select or retire the old leases publication; using it would start unrelated build/install work.

The exceptional selector is fully closed by 🏗️build/🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json Protocol: repo-mcp build/install and repo cache-retire-inactive-preview/cache-retire-inactive, with a fixed options list. No generic partition inspection or .cache/b/l retirement selector is present. Appending arbitrary arguments does not create an exception; unmatched forms fall back to ordinary routing.

The opening implementation 📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts opens the leases partition first. Its opportunistic idle-lease sweeping happens only after a valid physical image is opened and rechecked; it cannot correct a refused publication receipt. Every non-leases partition opens leases as its parent dependency.

## Concurrent Source Change

The actual failing log shows inspectPersistentReceiver at line131. A fresh read during this review found current Source at line132 calls readNativeBootSessionV1 and invokes rebindPersistentReceiptV1 when the authored boot session differs. rebindPersistentReceiptV1 at lines134–140 passes prior.bootSession ?? null and prior.ancestry ?? null into decideNativeReceiptBootV1, and the independent native-audit reviewer identified its old-receipt rewrite path. This means another ordinary invocation may now mutate receipt metadata rather than reproduce the original refusal. No retry should be described as read-only merely because its intended operation is cache status or exec dry-run.

This review did not edit, execute or validate that new path. Root's no legacy-receipt patch/fabricated-lineage requirement applies. An owner needs to resolve that Source behavior and supply a genuine, narrowly scoped retirement interface under exact inactive/exclusive custody before any stale publication cleanup is attempted. Raw hdiutil, arbitrary deletion, receipt rewriting, broad cache reset and active-process interruption are outside this review.

## Source References

All paths below are relative to 🧰️framework/🛍️products/🦑️repo/🔨️modules unless explicitly starting with framework:

- 📚️library/🏗️build/📜️script.ts — runOwnedBuildInvocationV1 early routing; cache/reset branches.
- 📚️library/🏗️build/🚀️bootstrap/⌨️cli/🟦️.ts and 🧬️schema/🔣️.json — closed recovery selector.
- 📚️library/🏗️build/⚙️operations/🟦️.ts — task-cache status/clear contract.
- 📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️script.ts — inactive compiler-store maintenance and rust opening.
- 📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts — leases-first parent opening and post-admission idle-lease sweep.
- framework path 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts — persistent publication inspection and concurrent rebind path.

Course22 remains unclassified by this review. Missing handle22832, old log position and missing terminal receipt do not prove its state. Builder/process reconciliation remains pending the legitimate native observation route becoming admissible.
