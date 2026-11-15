# Fleet Workers Absent With "Bootstrap Pending"

## Cause

- The R13 change (ticket WORKERS-READY-WITH-LOW-RESOURCES) made a worker lend only after the marker `.cache/f/native-bootstrap.done` existed (policy `worker.readyMarker`, why code `bootstrap-pending`).
- Chicken-and-egg: the native bootstrap (`.cache/f/native-bootstrap.log`) runs `repo` commands that read the project graph ("Reading the project graph slice ... door.script"), and the graph needs fleet workers. No marker, so no worker, so no graph, so no marker. On this host the bootstrap log ends in SIGINT after 947 s and the marker never appeared; the same held fleet-wide.
- The empty detail: the readiness sentence for `bootstrap-pending` carried the marker path only on some paths and the `ready()` fallback reported a different code, so the status line named no step.

## State found

Another agent already removed the marker gate before this check (files changed 05:59): no `readyMarker` in `policy.json`, no `bootstrap-pending` in the why texts, the schema enum or the sidecar. Readiness now means the toolchain behind the offer exists (`no-toolchain`, with detail "the compiler of the offer is gone or has no target"). The build-agent log shows `load report failed: Scheduler answered 400` (old agents reporting a code the new schema no longer knew), then `agent code changed, restarting`, `joining ... offering rustc for 3 target(s)`, `worker registered`.

## Verification

`repo fleet status` at 06:03: 10 machines reporting; g1-m07 worker refusing for "no headroom (queue-order)", i.e. registered and lending when idle; m01, m02, m03, m08, m10 and g1-server idle; m04 and m06 refusing for memory or queue order; only g1-m05 (Windows) absent, reason `no-toolchain` named with detail (Rust toolchain not provisioned there, a separate matter).

## Change by this check

- Stale docstring of `runRemoteExecutionAgentV1` still said `bootstrap-pending`; now says `no-toolchain`.

## Open

- g1-m05: Rust toolchain missing on Windows, memory pressure critical.
- Graph line still says "no builder; 0 computer(s)": no graph build is running at the moment (generation 222, 7 dirty owners).
