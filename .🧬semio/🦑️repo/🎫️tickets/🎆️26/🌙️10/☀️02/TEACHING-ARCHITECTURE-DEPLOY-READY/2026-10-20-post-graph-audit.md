# Post-Graph Preparation Source Audit

The build:graph complete marker is emitted before the graph function awaits portableInputs and validates its final readBuildGraphV1 result. It does not prove that loadBuildInvocationGraphV1 has returned or that selected Rust preparation started. portableInputs first captures inference inventory references, walks the object graph with a WeakSet and invocation checks every 1,024 visited objects, then resolves compact Source/Cargo closure tokens before returning the portable graph. Only after loadBuildInvocationGraphV1 actually returns does the build script enter withSelectedRustBuildGraphV1. That wrapper validates and clones the captured graph, selects actual tasks and their prerequisites, identifies explicit Cargo build intents, and returns unchanged when none are selected. Otherwise it opens issued Rust unit code and compiler Store lifetimes, collects a bounded descriptor working set, grafts selected projects/dependencies, verifies Source in two exact read batches, rechecks Store storage and only then enters the consumer which pins the graph and runs the chosen backend.

The selected Rust graft loops yield continuations and check resource lease control at the authored operationsPerYield boundaries. Selection, final publication and consumer transitions check issued lease/invocation custody. Exact Source loops use withExactSourceReadBatchV1 and observedExactSourceFileV1 rather than an unfenced general filesystem read. These graph validation, prerequisite selection, descriptor and Source verification stages can consume work after the earlier build:graph complete event and before backend output.

This is a bounded static Source review of build/📜️script.ts and build/graph/rust/🟦️.ts. It does not locate the current CPU call site, establish a stall, prove cancellation latency across every transitive helper, or claim compiler/test collection. The parent independently observed advancing CPU ticks; no additional process observation or diagnostic was created by this audit.

A bounded read-only private comparison confirms the reviewed graph reader/portableInputs Source is byte-for-byte equal to current host at SHA-256 `149a62e84e8c3c0bd154af9174f0e46d17551bc7f34b0c619250cb2750d3e61c` (52110 bytes). The marker-before-portableInputs ordering is therefore verified against actual private bytes, not merely assumed from shared host Source.

The actual private exact-read policy matches host SHA-256 `3bb515be6be4314e43fd8026c9f8ccdcf97a33b2153a62970d00475203d7f54e`. Its metadata ceiling 268,435,456 bytes divided by maximum closure 134,217,728 bytes gives cohort width two (within maximumInventories). Therefore the offset += width loop advances under this actual policy; no zero-width-loop concern is supported. This does not identify the live CPU location.

The actual private Bun 1.4.2 help command completed successfully. Its output is retained as `generated/2026-10-20-private-bun-help.log`. Supported profiling switches shown by help are:

- `--cpu-prof                      Start CPU profiler and write profile to disk on exit`
- `--cpu-prof-name=<val>           Specify the name of the CPU profile file`
- `--cpu-prof-dir=<val>            Specify the directory where the CPU profile will be saved`
- `--cpu-prof-md                   Output CPU profile in markdown format (grep-friendly, designed for LLM analysis)`
- `--cpu-prof-interval=<val>       Specify the sampling interval in microseconds for CPU profiling (default: 1000)`
- `--heap-prof                     Write a heap profile to disk on exit (.heapprofile)`
- `--heap-prof-name=<val>          Specify the name of the heap profile file`
- `--heap-prof-dir=<val>           Specify the directory where the heap profile will be saved`
- `--heap-prof-md                  Generate markdown heap profile on exit (for CLI analysis)`
- `--heap-prof-interval=<val>      Specify the average sampling interval in bytes for heap profiling (default: 524288)`

This was help only. No profiler was started or attached, and the live managed process was not signalled or modified.
