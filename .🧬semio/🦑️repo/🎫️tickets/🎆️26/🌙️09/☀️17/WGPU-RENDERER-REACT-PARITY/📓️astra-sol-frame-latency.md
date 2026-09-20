# Astra Sol Frame Latency Diagnostics

Source-stable implementation report, revised 2026-09-20 after checkpoint10. This packet changes diagnostic attribution and retention only. It does not change the scheduler, rAF/presenter admission, four-millisecond browser shares, queue ceilings, frame generation, or render behavior. Native compilation, wasm activation, and diagnostic-on/off browser measurement remain owned by the root agent.

## Checkpoint10 finding

The first scalar ring was bounded in memory but not useful for attribution. In the sealed `🗑️generated/astra-runtime/paired-checkpoint-10/steps.json`:

- `settings-open` had 709,701 admitted observations and 709,445 overwritten observations. Of its newest 256 rows, 254 were `presenterRetirement`; all retained rows summed to 3,999 µs.
- `settings-app` had 1,397,654 observations and `settings-general` had 1,798,993.
- the final `chord-fullscreen-exit` snapshot had 2,435,759 observations and 2,435,503 overwritten observations. Its newest rows were dominated by `transactionRouteIntents`.
- generation 37 in a presenter row identified a renderer frame, while generation 8 in `workerTick`/`workerReplyEncode` identified a browser input batch. The original bare `generation` field did not state that distinction.

A 256-row scalar ring therefore described the last repeated inner step, rather than the operation that occupied a frame. Its `dropped` counter was also mostly scalar overwrite pressure, not loss of phase attribution.

## Schema-first replacement

The version-2 neutral contract is defined by:

- `engine/🧬️schema/⏱️frame-latency/🔣️.json`
- `engine/🧫️fixtures/⏱️frame-latency/🔣️.json`
- `engine/🧪️tests/⏱️frame-latency/🟦️.ts`
- `engine/🧪️tests/⏱️frame-latency-diagnostics/🦀️.rs`

Every observation now carries an explicit authority pair:

- `rendererFrame` with the renderer frame generation for dispatch, transaction, Shell refresh, retained exchange, presenter, queue-submit, and snapshot publication stages;
- `browserInputBatch` with the browser transport generation for wire apply, worker tick, and reply encoding.

An equal numeric generation in those domains is deliberately two identities.

`dumpFrameStats.frameLatency` now publishes:

- `summaryCapacity`: 256 fixed recent phase slots;
- `totalObservations`: admitted scalar observations;
- `summaryEvictions`: unique phase summaries evicted from the recent window;
- `refused`: observations refused by the existing nonblocking diagnostic lock;
- `stageTotals`: lifetime cumulative count, total duration, maximum duration, and work count for each populated `(authority domain, stage)`;
- `recentPhases`: oldest-created retained summaries keyed by `(authority domain, generation, stage)`, with first/last sequence, observation count, first-start/last-finish envelope, total duration, maximum duration, and total work.

The backing cumulative matrix is exactly `2 × 27` fixed aggregates. The recent window is `[Option<FrameLatencyPhaseSummary>; 256]`. A fixed domain/stage index makes the normal repeated-stage update constant-time; the bounded scan is used only when an older live generation for the same stage interleaves. Eviction invalidates the corresponding index before the slot is reused. There is no map, string, side queue, or per-observation allocation.

This makes 300 `presenterRetirement` observations for one renderer frame consume one recent slot, while their full count, total, maximum, work, and wall envelope remain visible. If that phase is later evicted, its contribution remains in `stageTotals`. `summaryEvictions` now describes lost recent phase detail rather than repeated scalar churn.

## Disabled and enabled costs

`FrameLatencyTimer::start` still checks `semio_framework_trace::runtime_diagnostics_enabled()` before reading the monotonic clock or initializing the registry. With diagnostics disabled, the timer is a stack value with no timestamp and Drop returns before touching the registry. No heap allocation is introduced on that path.

With diagnostics enabled, each completed scope reads the clock and attempts the existing nonblocking mutex once. It updates fixed arrays with saturating integer arithmetic. A contended observation increments the atomic `refused` counter and does not wait. Only the explicit introspection snapshot allocates its two result vectors.

## Validation

The focused existing Bun+Nx Vitest command passed one file and all four tests. The laws:

1. validate the version-2 neutral fixture with Ajv;
2. compare an independent TypeScript aggregator with a 604-observation scalar-flood fixture;
3. prove numeric generation collisions remain distinct across authority domains;
4. pin explicit authority selection at the WGPU timing boundaries and publication through `dumpFrameStats`.

The mounted Rust laws mirror the neutral aggregation, verify fixed summary eviction with non-evicted cumulative totals, verify authority-domain separation, verify a late interleaved observation reuses its exact phase summary, exercise all transaction/presenter stage mappings, and inject a disabled clock to prove it is not read. Standalone `rustfmt --edition 2021 --check` parsed the production module and its mounted laws successfully. Both JSON inputs parse, and the focused TypeScript result was 4/4 passing.

No Cargo, native renderer, wasm build, activation, or browser journey was launched by this agent. The replacement contract has not yet been observed in an activated runtime.

## Supported granularity and remaining blind spots

The diagnostics can now retain whole renderer-frame and browser-input phase summaries through scalar floods and can compare long-run cumulative stage count/total/max values. `firstStartedUs` to `lastFinishedUs` is a monotonic phase envelope; `totalDurationUs` is the saturating sum of measured scopes. The envelope can include gaps between resumed steps, while the total can double-count nested timing scopes. Neither is presented as GPU execution time.

The seam still starts at Rust wire application. It does not timestamp DOM enqueue, page rAF queueing, Worker message delivery before Rust, Worker reply delivery to the page, GPU completion, first pixel, semantic-registry polling, or OS descheduling. Those require the root-owned paired browser journey and page-side timestamps. The next diagnostic-on/off run should inspect `stageTotals` deltas and `recentPhases` by authority domain, rather than comparing a single composite settle interval.
