# Graph Admission Blocker

## Current Classification

The exclusive graph compute slot is held by a live unrelated build, not a dead claim. Authoritative facts supplied by the read-only process audit at 18:45:20 UTC identify PID 211237, parent 211110, running the framework-os-mcp-rs build with parallelism one. Its main JS thread consumed 200 CPU ticks in two seconds and increased read/system-call counters. It has no OS child workers and no own resource wait entry. Repository leases identify slot zero task 120, claim 49620f499d3a2883fbe02d40, agent codex-fad27566, with the thesis process 229702/task143 waiting behind it. No lease cycles or violations were reported. See process.md for the authoritative samples.

This supports active CPU-bound inference, rather than a stale PID or a current host-capacity wait. A progress count of zero counts completed projects and does not establish a stopped worker. No evidence from this review justifies interfering with that unrelated owner.

## Source Findings

- The claims registry in graph/generations/claims acquires the exclusive compute slot and claim before invoking the computation; both are released in finally. A running claim is considered live solely through its held process lease. Dead claims are recovered by lease probing, not wall-clock age.
- The graph prefetch callback first books the rebuild capacity phase and then runs project inference. A claim holder could therefore wait for capacity while holding its graph slot, which is a cross-controller dependency not necessarily reported as a resource-lease cycle. Current process facts do not show that condition for PID211237: it uses a full CPU core and has no wait entry.
- Project-node inference starts its inference phase before dispatch. Its completed counter advances after an entire project finishes. The first expensive project can keep the counter at zero. Running-owner observations are separately emitted.
- The worker farm engages after at least24 cold project misses and25percent misses; earlier projects run in the parent. A disabled or inapplicable farm also leaves work in the parent. No child process is therefore not by itself a fault.
- Farm initialization has a180second timeout and worker jobs have a30minute timeout. These timers apply to child-worker requests, not inline inference in the parent. An hour-long parent-owned inference is not automatically recovered by the worker timeout.
- Extra farm capacity is elastic and does not block the parent; a paid own-grant worker is started once the farm opens. On shutdown, pending elastic capacity is cancelled before awaiting an own-only pool. No concrete farm-close deadlock was found.

## Safe Supported Path

Keep exactly one owned thesis build and observe the live owner's running-project/CPU progress through repository leases and fleet/doctor surfaces. Once its claim ends, the existing queued thesis invocation can use the warmed project records and proceed through the normal graph and compiler authority path. This is the only currently evidenced route to the compiler without changing infrastructure semantics.

If host memory contention warrants active relief, cancellation of our confirmed thesis command through its normal SIGTERM lifecycle is safe and releases only our leases/reservation; relaunch it once the shared slot is free. Cancellation alone does not resolve a healthy unrelated owner's CPU-bound first project, so it should be chosen for measured memory relief, not as a timeout retry. No own process was cancelled by this review.

The public graph slot override SEMIO_GRAPH_COMPUTERS exists, and the farm worker override SEMIO_GRAPH_WORKERS exists, but increasing slots overrides learned rebuild sizing and is not justified on the currently memory-constrained host. Disabling our farm cannot affect an unrelated existing holder. Neither option is a recommended recovery here. Deleting claims, forging invocation authority, bypassing the capacity governor, or cancelling the unrelated holder would violate ownership and was not attempted.

## Reviewed Sources

- graph/🕰️generations/🧩️claims/🟨️.mjs: admission, liveness, prefetch/finally, slotCountV1.
- graph/🟦️.ts: rebuildSlotsV1 and prefetchStaleGenerationV1.
- graph/📥️inference/📦️project-nodes/🟨️.mjs: project completion progress and farm setup.
- graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs: paid/elastic pool lifecycle.
- graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs and schema: engagement thresholds and worker timeouts.
- host-capacity/🟦️.ts: elastic farm capacity semantics.

All source paths are relative to the repository Build library directory. This was a read-only source/process investigation; the only file written is this audit.
