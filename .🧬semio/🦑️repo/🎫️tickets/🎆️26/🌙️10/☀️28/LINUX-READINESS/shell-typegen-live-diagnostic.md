# Shell Typegen Live Diagnostic

Read-only observation at 2026-10-29T18:19:35.966514+00:00. Root73042 remains the sole producer; no cancellation, restart, compiler/test/Native launch, lease query or mutation was performed. The log is live and the following records describe observed phases rather than a terminal result.

## Actual Log Sequence

The initial capacity queue preceded graph admission. The log then verified848 input closures, with 1024 structured verification progress events and 1 decreasing progress counters. The observed final-pass tail is841→843→845→847→848. Exactly one earlier848→0 reset occurs, separated by a complete271982-fact verification pass; this is a finite two-pass sequence rather than an endless reset. The Source loader explicitly starts as a shared reader and retries stale/missing generation under builder custody (loadGenerationGraphV1); exact per-pass lock ownership is not separately established by this audit. A repeated display of build:graph-plan stage-running is not a fresh graph invocation by itself.

Graph Source freshness found155 stale facts:117analysis,20file,9readdir,8lstat,1digest. It explicitly attributed changes to the catalog façade relocation, print representation additions, browser provisioning and SDK observation additions, build policy and one dependency generation lstat. It refreshed527 projects in generation64. This was a recorded Source-invalidated graph refresh, not an inferred loop.

The graph child reported generation64 published after571seconds,562235 facts for4085 projects,46.8MB expanded of512MB limit and8.9MB packed. It then reported848 derived results asked again after155 stale facts:1unchanged/847changed. A later record says569 projects were planned by a child process that has exited. These are settled graph phases. The log subsequently advanced through nx-authorizers capture, build:implementation-source and nx:received-graph.

During refresh, parent reported exceeding its1792MB graph memory budget and serial inference; later samples in the log reached4224MB before completion. This explains a documented serialized resource-pressure phase, but does not independently identify retained object attribution or justify changing memory guards.

## Why Phase Lines Repeat

Actual Source `🏗️build/🧭️invocation/🕸️graph/🟦️.ts:loadBuildGraphInChildV1` launches one adopted captured graph child and passes an onProgress callback that repeatedly advances the parent build:graph-plan phase. Plan memo can separately settle edited facts through build:graph-derive. Repeated stage-running rows therefore cannot be counted as restarts. The verified result and published generation corroborate finite completed work in this run.

`process/🔒️leases/🟦️.ts:directoryAdmission` acquires the schema-owned resource-directory admission gate before opening/retiring directory resources. Observed shared waits alone provide neither exact lease directory nor holder identity; the resource basename is reused across directories. Owned sweeps later reported settled claims retired; no read-only audit itself invoked a sweep. No self-blocking acquisition cycle has been established from these records.

## Bounded Own Process Observation

PID1365406 was Bun, sleeping, PPID1365399,startTicks19498232,CPUticks2634,RSS19442pages. Its positively observed child1397199 was Bun, sleeping,PPID1365406,startTicks19607059,CPUticks22228,RSS94193pages. The child had no children listed at that instant. The bounded /proc/task children traversal is positive-only and is not used to prove that no live descendants exist; no global process scan or cmdline/env dump was performed. No CPU delta was sampled, so these lifetime CPU counters do not measure current utilization.

Next supported action is to let the existing run settle and inspect its actual next phase/terminal result. Existing nonpruning resource-holder APIs require an exact directory/resource plus genuine owned context; this log basename alone is insufficient to issue an authoritative holder query. No guard change, process stop or duplicate retry is warranted by the observed successful graph convergence.
