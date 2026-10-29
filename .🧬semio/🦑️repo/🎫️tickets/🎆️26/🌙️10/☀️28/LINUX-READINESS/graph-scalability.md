# Cold Graph Scalability Observation

Read-only live lease/proc snapshot: graph peer160792 advanced from coordinator71/3761 at69seconds to177/3761 at142seconds. Authoritative lease activity said progressing with about6minutes remaining, zero deadlock cycles and zero recent order violations. Process state R; user12561/system3061 clock ticks; RSS2024128kB, HWM2143044kB,15threads. Machine admission had1450MB spare above1981MB floor, four granted and two queued operations. These facts support active computation under memory pressure, not a hang. No process stopped, signaled, restarted or modified.

## Actual Inference Owner

Graph inference project-nodes/🟨️.mjs225 creates jobs for3761 authored/generated native owners (more than static1489project manifests). Per-project loop251 runs through a memory governor, publishes running owner state and advances inference completion. Farm248 can obtain funded worker capacity; it reuses captured setup/closure identities. When host is saturated, additional workers queue; high elapsed time alone cannot identify deadlock.

Per project233 computes projectAssemblyAuthorityV1, then236 observeProjectInferenceV1 validates/reuses the record or computes body. In late closure mode239 resolves shared closure references. Owner-discovery projectAssemblyAuthorityV1 118 uses nativeDependencyRoots with passed nativeCache/nativeClosures, so repeated dependency closure construction is explicitly cached. Shared setup156 and script-input cache158 also avoid intentional whole-workspace rebuild per project.

## Potential Hot Paths, Not Proven Defects

- Large native dependency closures are mapped/JSON-serialized and hashed per project authority118 even when dependency roots are cached. Profile aggregate closure size and authorityMilliseconds234 before changing semantics.
- Cold project observe236 and resolveClosureRefs239 can read/verify many source observations. Shared script closure recipe cache736 and shared command walk877 exist specifically to prevent repeated full command traversal. No source evidence from this review proves the older Bun whole-frontier replay occurs once per project here.
- readBuildSourceInputMatchesV1 in owner-discovery recursively expands fixed pattern prefixes before matching, retaining walked lists per call. Calls with broad workspace patterns may be costly; verify selected-source-input progress and exact call frequency. It already excludes node_modules/cache/generated/ticket trees.
- Initial authored project construction168 uses Promise.all over configs, while later inference uses a memory governor. Catalog/preparation expansion in that phase could transiently allocate; no measured attribution established.

Use a later authoritative activity sample and bounded profiling counters to distinguish throughput, cold cache, worker admission and source-validation work. Preserve source custody and peer leases. Do not infer a deadlock from a waiting graph reader when the writer is advancing. No source edits by audit.

## Native Authority Memoization Safety Review

projectAssemblyAuthorityV1(owner-discovery118–121) always obtains validated nativeDependencyRoots, maps every reached owner through projectsByRoot, serializes [ownerPairs,componentRoots] and hashes it. nativeDependencyRoots(native-manifests169+) caches sorted root arrays through memoizedObservedComputationV1; shared transitive dependency members are also memoized. Its returned arrays are mutable, not Object.freeze. projectsByRoot is a mutable Map built once at project-nodes221; componentRoots is an ordinary sorted array built222. No deep immutable content identity is exposed for these values.

Observation memoizedObservedComputation201–206 validates cached source observations with memoRowsHoldV1 and retains all consumed facts in the receiving project. Large row-array verdicts are memoized per inference validation round194–198. Shared physical checks are also round-scoped186–191. Any digest optimization must still call this observed interface before reuse; caching the digest must not bypass source observation delivery or changed-source detection.

A potentially safe bounded optimization is invocation-local digest memoization only after obtaining the currently validated closure array, with immutable copies or an explicit sealed roster digest for owner-map and component-root projections. Cache may be keyed by validated closure identity plus those roster identities; release it with emojiProjectJsonNodes invocation. Do not use a global WeakMap solely by mutable array identity. A simpler safe improvement may precompute the component-roster serialization/digest once per invocation, but changing concatenation/hash composition requires a declared record-key version change and neutral tests.

Likely savings are uncertain: each nativeRoot generally appears as one project, so authority hashes may be one per distinct closure anyway. Source inspection establishes repeated linear serialization, not a confirmed dominating hot path. Measure authorityMilliseconds234 and cumulative serialized bytes/unique closure identities before implementing. No live-peer profiling signals, source mutations or process interventions performed.
