# Current Runtime Latency Evidence

Checkpoint7 is a debug renderer with diagnostics enabled on a shared busy host. The initial journey recorded large frame-step overruns and the corrected journey observed 3–7second registry publication delays. These are acceptance concerns, not an attribution to GPU batching.

## Diagnostic Traffic

Console file `paired-checkpoint-7/wgpu/console.txt` has 33996 lines. Normalized leading categories:

- 2706: `[actor] [DEBUG] turn phase enter: retained # B, delta #`
- 2706: `[actor] [DEBUG] turn phase lifecycle: retained # B, delta #`
- 2706: `[actor] [DEBUG] turn phase ingress: retained # B, delta #`
- 2706: `[actor] [DEBUG] turn phase continuation: retained # B, delta #`
- 2706: `[actor] [DEBUG] turn phase presence-clock: retained # B, delta #`
- 2706: `[actor] [DEBUG] turn phase render: retained # B, delta #`
- 2706: `[actor] [DEBUG] guest linear memory turn=# bytes=# delta=# percent=# events=# eventsBytes=# elapsed_`
- 2704: `[actor] [DEBUG] turn phase publish: retained # B, delta #`
- 2084: `[DEBUG] frame build admitted generation=Generation(#)`
- 2078: `[actor] [DEBUG] reactor more-work streak=# seen=# sources=["process_pool", "close_cleanup", "reconci`
- 1046: `[DEBUG] wgpu-shell engine surfaces syncs=# drains-with-graph=# drain=# live=[] old-rule-would-evict=`
- 990: `[DEBUG] os_host frame gate blocked=false pending=false phase=None retirement=false retained-fault=No`

## Follow-up Boundary

Measure current canonical activation with diagnostics on and off through an outcome-based input-to-publication probe. Separate guest update, document projection/reconcile, CPU paint, submission and presentation costs. The renderer currently submits each scalar prepared scene immediately after writing its uniform, so the hypothesized last-uniform overwrite is not proven by buffer slot reuse alone. Preserve scheduling and event-order semantics when addressing measured bottlenecks.

The existing ticket journey now accepts `SEMIO_PROBE_DIAGNOSTICS=0`, records the selected mode, and marks parity unmeasured with diagnostics disabled. Errors remain failures and all controls/surfaces/screenshots remain recorded. Timing records separate operation duration from first observed registry publication and semantic geometry/surface change after the operation. These are polling measurements, not GPU timestamps or isolated CPU execution costs. Current runtime rerun is pending canonical activation8.
