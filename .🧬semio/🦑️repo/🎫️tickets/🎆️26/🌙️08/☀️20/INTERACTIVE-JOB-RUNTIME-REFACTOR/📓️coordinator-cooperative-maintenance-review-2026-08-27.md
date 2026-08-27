# Cooperative Maintenance Independent Review

## Source and Actual Native Evidence

The coordinator read the live plugin maintenance branch, the cooperative scheduler, fixed diagnostic snapshot, and all3new tests. The old queued/running branch reported work without pumping; Maintenance accrues1credit per scan against8unit cost, so the initial single pump may leave its closure queued with deficit1. The corrected source gives the retained queued owner one pump on each revisit. It never loops to drain the pool or invents a clock sample.

Actual retained native output reviewed:

- Host-binding RED R2:0passed/1failed,50skipped,1second test not run due fail-fast,.018s. This is a source-string binding assertion, not mounted plugin execution.
- Cooperative GREEN R4:3passed/50skipped,.190s. It mounts the real wasm_pool implementation under cfg(test) in the native async crate. DEBUG records Maintenance8turns,Background4,UserVisible2,Timer3,Interactive1, each exactly one execution. Held snapshot-lock testing preserves the queued closure. The host-binding test also rejects5mutants, but remains source-only.

The fixed6-lane snapshot uses try_lock and retains every closure. The existing scheduler pump's own mutex behavior was not changed or certified by the snapshot test. The plugin's wasm-only helper is still uncompiled in this checkpoint.

## Fresh Component Boundary

The peer's current fresh GIS run remains RED:5initial surfaces publish; accepted setActiveExample operation1 and Ephemeral are followed by idle ingress/more-work through4096turns. This symptom is consistent with the source liveness gap, but exact runtime attribution and a successful command/publication/close require a fresh component rebuild/probe.

The helper emits bounded [DEBUG] samples at powers of2 through4096calls, at most13samples. They expose actual clock presence, instance/generation, maintenance status before/after, entered-turn count, fixed pump-phase bits and6-lane queue/deficit/selection counters. Stderr formatting is diagnostic work, not a timing certificate; no8ms claim is made from an instrumented run. Missing actual clock marks Fault rather than a fabricated tick.

The demonstrator peer owns shared component/module outputs and its current Flow integration. This fleet wrote none. No cap was raised, no source or evidence was removed, and no WGPU file was changed for this packet.

