# Phase 8 Post-Wave Source Audit

## Scope And Method

Read-only audit of the four completed source packets: Remodel reconstruction commands, Draw `canvasPointerDown`, Forms `setTryValue`, Flow `duplicateWidget`, and the permanent tool-job ledger. No Cargo command was run because P4 owns the Cargo lane. The static verifier was executed at 2026-08-22 and passed with `774` production rows, `774` bounded rows, `0` batch-only rows, and zero reported failures. `rustfmt --check` was also executed; it found formatting diffs in Flow, Forms, and Remodel, but did not validate Rust type correctness.

## Gate Result

The ledger result is a useful inventory result only. It cannot be accepted as the Phase 8 completion gate: the production paths below contain one compile blocker and two genuine worker/runtime correctness blockers, while the verifier currently only checks declarations and framework shape.

## Findings

### P0 — Flow Duplicate Widget Loses Its Graph In The Worker

`duplicateWidget` schedules the worker continuation, then calls `flow_working_scene(doc.snapshot)` at [duplicate-widget component.rs:134-140](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs:134). That helper reads the `FLOW_SCRATCH` thread-local cache at [flow artifact component.rs:190-199](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs:190), returning an empty scene when that thread has not seeded it. The Phase 8 job bus deliberately executes the command on a worker, so that cache is not a portable source of the document graph. An empty worker scene makes the Source phase at [duplicate-widget component.rs:141-151](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs:141) return a no-op rather than commit a duplicate.

Required repair: carry an owned, worker-portable graph snapshot in the job session (or make `FlowSnapshot` itself supply the graph), and add an executable worker-path regression that proves a real duplicate mutation is emitted. A cache-dependent helper is not an admissible worker input.

### P0 — Draw Packet Does Not Typecheck Because Its New Continuation Uses Futures As Values

The new trace job declares `TracePointerJob::advance` as `async` at [canvas-pointer-down component.rs:561](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:561), yet treats its returned future as `bool` at [line 786](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:786). The same packet has nested non-awaited future calls at [588](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:588), [601](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:601), [611](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:611), and [671](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:671). Its outer calls likewise return futures where `Emit` is required at [777](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:777), [857](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:857), [863](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:863), and [866](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs:866).

Required repair: this is predominantly stale, artificial asyncness—not asynchronous I/O. Make the trace-only pure helpers and the new `advance` chain synchronous (including `consider_trace_candidate`, `trace_world_bounds`, and its pure transform helper); retain `async` only at the framework handler boundary or where a real awaited dependency remains. The existing broader Draw gesture helpers also display pre-existing artificial async signatures; repair every call reachable from `canvasPointerDown` coherently rather than adding a scatter of awaits. Then run the exact Draw Cargo target before accepting its source-added tests.

### P1 — Remodel `runStage` And `retryStage` Discard Their Only Input

Both commands accept `stage`, but their handlers name the payload `_payload` and unconditionally restart from ingestion: [retry-stage component.rs:13-20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️retry-stage/🦀️component.rs:13) and [run-stage component.rs:13-20](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-stage/🦀️component.rs:13). This is relabeling three distinct production actions onto one start, not preserving the requested stage semantics advertised by the action argument catalogue.

Required repair: model the requested/retry stage in the resumable session and its payload/checkpoint, advance only that stage or the proper dependency prefix, and add tests showing distinct stage inputs produce distinct engine entry behavior.

### P1 — Remodel Terminal Commit Is Not Bounded

The job's nonterminal engine work has a cap of one unit, but `EngineStatus::Done` enters `finish_reconstruction` at [run-reconstruction component.rs:179-180](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs:179). That one dispatch reads complete sparse output, iterates every camera pose, materializes full point output, takes mesh/quality/geo products, encodes raster assets, and creates all mutations at [201-255](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs:201). This violates the 8 ms worker step ceiling precisely at the terminal commit.

Required repair: make terminal materialization a continuation state with fixed chunks and cancellation/freshness checks. Publish only capped previews during progress; issue the final atomic artifact commit only after all bounded preparation is complete.

### P1 — Forms Re-serializes Unbounded Config State On Every 64-Element Step

The numeric vector growth itself is capped at 64 in [set-try-value component.rs:106-113](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs:106). However each continuation inserts then serializes the complete `values` map and extracts the complete vector again at [151-153](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs:151); the initial action also parses the complete try-values JSON at [186-202](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs:186). Consequently the claimed unit cap does not cap real per-step work for a large form or existing vector.

Required repair: make the canonical try-values representation structured and update only the changed vector prefix/cursor, or explicitly chunk parse/serialization under the same job model. Add a timing test with a large unrelated config map and existing vector.

## Packet Status

- Flow has a well-formed 64-row search/checkpoint/cancellation shape, hidden non-palette continuation registration, and source tests, but is blocked by its non-portable worker input.
- Draw has a useful 32-work-unit state machine and generation preview shape, but is compile-blocked. Its continuation is intentionally the internal, non-palette `canvasPointerDown` action rather than a separate hidden action id.
- Forms has a real generation/checkpoint and an internal non-palette continuation registration, but does not yet bound whole-config work.
- Remodel has generation/session/coalesce/freshness mechanics and an internal continuation registration, but stage semantics and terminal preparation are not yet admissible.

## Required Revalidation After Repairs

1. Cargo check and focused tests for Draw, Flow, Forms, and Remodel under the P4 Cargo lease.
2. Production worker-path tests: real Flow duplicate commit, Draw trace completion, Forms large-config timing, and Remodel stage-specific + terminal timing/cancellation.
3. Rerun `bun ./📜️script.ts verify interactivity tool-jobs --format json`; retain zero batch-only rows only after the executable gates pass.
