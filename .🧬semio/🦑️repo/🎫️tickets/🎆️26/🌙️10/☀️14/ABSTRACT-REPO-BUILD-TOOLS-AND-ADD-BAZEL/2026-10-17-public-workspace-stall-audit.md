# Public Workspace Admission Stall Audit

Read-only Source and saved sample inspection. Actual parent reports PID78062 remains running; no terminal refusal or pending-publication failure was observed by this audit. No process, store, Native or filesystem census operation was executed.

## Evidence Limits

Saved 🗑️generated/coordination-17/public-workspace-sample.txt contains766 main-thread samples in unsymbolized Bun/JIT frames, footprint45.3MB and peak57.2MB. It supports active main-thread work during the sampled interval, but cannot identify a JavaScript function or distinguish module evaluation from function execution. The unchanged public log alone cannot establish which await was entered.

## Earliest Path And Instrumentation

Exact owner `build/📜️script.ts:85–88` first authenticates phase/caller, dynamically imports caller, then dynamically imports cache opening and awaits openRepositoryCachePartitionV1(root,"leases",control). Only afterward does it open diagnostics/dependencies stores and enter build selection/bootstrap/graph/Nx. The first existing lease DEBUG appears only after initial image open, recheck, exclusive maintenance and both retirement passes. Absence of that message cannot isolate initial open from prior imports.

For a fresh diagnostic using unchanged same Original, add temporary [DEBUG] wall/performance timestamps immediately before/after: caller import; readOwnedBuildInvocationV1; cache-opening dynamic import; first leases open. These four brackets are the highest-value first split. Keep actual API calls, ordering, controls and errors unchanged; avoid per-call logging in generic controls.

Inside cache opening `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:38–45`, bracket prepareRepositoryCacheParentsV1, repositoryCacheBudgetV1, initial openNativePlatformStorageV1, image.recheck and image.withExclusiveMaintenance; inside maintenance bracket retireResourceLeasesV1 and retireResourceQueueClaimsV1 separately. Initial lease configuration includes maximumLeaseDatabaseBytes16384 and wait:false maintenance; no read-only evidence establishes lock-wait causality.

If cache-opening import is the stalled segment, inspect module evaluation/compile attribution before modifying Native logic: its static closure loads platform router, all three host implementations and cleanup/storage owners. CompileJsonSchemaSubset at validator/🟦️.ts:137 snapshots complete schema/root/documents then prepares refs/properties/patterns; validateJsonSchemaSubset:220 recompiles for every call. Instrument only bounded aggregate compile/snapshot counters/timing and root schema identity, not every recursive node/log. Snapshot already has identity memoization and prepare has per-root/schema memoization; no demonstrable infinite recursion was found from Source inspection. Repeated whole-document compilation is a plausible CPU amplifier, not a proven PID78062 bottleneck.

If initial platform open is the stalled segment, macOS path is platform/🟦️.ts:25 → 🍎️image/🟦️.ts:30 → 🧹️cleanup/🎛️command/🟦️.ts:138 openPersistentNativeImageV1 → openPersistentNativeImageBodyV1. Bracket lifetime entry, envelope validation, initial guard acquisition, receipt/mapping observation and verification as separate existing boundaries. Control refinement `routing/📥️invocation/🏃️process/🟦️.ts:82` walks retained parent controls using two finite lineage loops; aggregate depth/count can reveal unexpected multiplication without changing checks.

The Source ancestry loops in prepareRepositoryCacheParentsV1 and persistentParents explicitly terminate at dirname(path)===path; no obvious nonterminating pure loop appears there. Do not optimize by deleting validation/controls or infer pending refusal from historical metadata diagnostic.

## Coverage Follow-Up

Parent reports reverse65/reverse129 added and permanent pure test1/319 passed. This audit did not rerun it. Those cases address the material paired-pop alignment coverage gap previously reported.
