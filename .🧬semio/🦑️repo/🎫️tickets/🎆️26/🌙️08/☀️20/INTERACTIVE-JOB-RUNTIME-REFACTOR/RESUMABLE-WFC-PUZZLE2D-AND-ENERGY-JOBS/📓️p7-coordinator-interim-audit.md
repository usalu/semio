# P7a Coordinator Interim Audit

## 2026-08-22 source-repair reconciliation

**SOURCE REPAIR COMPLETE; INDEPENDENT/RUNTIME VERDICT PENDING.** The rejection below records the
pre-repair tree and is retained as the audit input. No Cargo command was authorized in the repair
lane, so this update does not convert the ticket to PASS.

| Finding | Reconciled source evidence |
| --- | --- |
| 1. Synchronous Assembly compile/no factory | `AssemblyInferenceJob` owns persistent Weights/Modules/Rules/Model/Slots/Edges/Topology/Fixed/Restore/Solve/MapCommit/EncodeCommit stages. Specialized cursor builders avoid both opaque final compiler calls. `plugin()` idempotently registers the exact `semio.infer` / `s.assembly.solve` factory on the production bus. Typed snapshots move into the factory job without clone/serde before step one. Headless `InferredField` is an explicit batch adapter over the same parent. |
| 2. Preview gaps | First preview is due after unit one; subsequent previews are due every 16 units or 16 ms. Initialize, entropy/choose, propagate, contradiction, and backtrack are all eligible. Parent compile/map/encode and restore publish bounded progress too. |
| 3. Uniform rejection loop | `JobRng::range` is one multiply-high mapping from one RNG word. Source test asserts identical state advancement to one direct draw. |
| 4. Monolithic restore | `WfcRestore` incrementally decodes header/domain words/trail/decisions/observations, verifies trailing bytes, and rebuilds counts/sums/revisions/heap one pattern per fuel unit. Interactive restart uses it. The batch constructor delegates to this job only. |
| 5. Allocation boundary | Checkpoint and commit envelopes have checked 1 MiB maxima. Maximum and one-byte-over source tests exist, including allocation pressure and cancellation. Other new compiler storage grows incrementally. Runtime debug/release watchdog evidence is pending, not asserted. |
| 6. Warning masking | WFC module-wide `dead_code`, procedural crate-wide `unused_*`, and crate-wide `async_fn_in_trait` were removed. Non-production WFC leaves are `cfg(test)`; only exact item/macro-local allowances remain. |

Additional source coverage now requires maximum factory preview cadence, registered-key lookup and
collision preservation, stale commit identity, map-loss restart, cancellation in every restore
phase and materializer, specialized compiler parity, and fixed-work RNG. Static scans are clean for
private executors/threads/pools and debug output. Rustfmt check and scoped diff check exit 0.

### Required next audit

Independent Terra must inspect the immutable source and the serialized Cargo owner must run the new
tests plus native dev/strict/release and both Wasm targets. Until those gates execute, the original
runtime rejection is neither affirmed nor cleared.

## Verdict

**HISTORICAL PRE-REPAIR VERDICT: REJECT — the per-unit solver core was substantially bounded, but
the Phase 7 WFC gate was not met.**

This is an interim coordinator review while the requested independent Terra slot is unavailable. It does not replace the Terra exit audit. No Cargo command was run because the shared filesystem has insufficient headroom.

## Blocking Findings

### 1. The production Assembly operation still compiles and runs synchronously outside the job

`💡️inferences/🦀️component.rs:17-54` walks and clones all modules, weights, rules, slots, edges, and fixed assignments, then performs whole model and CSR topology builds before `WfcJob` exists. `:59-70` immediately drives the job to completion through the batch adapter. The three `InferredField` boundaries also serialize whole snapshots and invoke this synchronous path (`:111-118`, `:145-149`).

The current product may not mount Assembly, but Phase 7 requires the WFC operation itself to be the resumable, preview-producing path. A future `semio.infer` note is not an implemented action/effect/job route. The compilation stages and public inference dispatch must become persistent worker-owned state, with the synchronous inference retained only as an explicit headless adapter over that same complete job.

### 2. Preview publication is not continuous during expensive stages

`wfc-engine/🧵️job/🦀️component.rs:1177-1233` returns `PreviewReady` only in `CommitSlot` (`:1186-1197`). Domain initialization, candidate weighing/selection, initial and later propagation, contradiction handling, and long backtracking can consume arbitrarily many bounded steps without publishing the accumulated active slot, propagation wave, changed domains, or backtrack path.

The hard per-step watchdog therefore passes while the plan's first-substantive-preview `<50 ms` and active-preview cadence requirements can fail by seconds. Preview checkpoints must be emitted on a bounded step/time cadence from all long stages without changing solver ordering.

### 3. Uniform sampling contains an unbounded loop inside one step

`wfc-engine/🧵️job/🦀️component.rs:125-135` uses rejection sampling in a `loop`. It is probabilistically fast but has no deterministic iteration bound, fuel check, or resumable cursor. `begin_choice` calls it inside a single `ChooseCandidate` unit. Replace it with a fixed-work deterministic mapping or persist rejection attempts across steps.

### 4. Checkpoint restore is a run-to-completion constructor

`wfc-engine/🧵️job/🦀️component.rs:438-567` decodes every domain word, trail entry, decision, and observation, then rescans every domain/pattern and rebuilds all caches and the entropy heap before returning. That is substantive restart work and is not resumable or cancellable. Calling it a batch constructor leaves checkpoint recovery outside the interactive contract.

Restore must be a persistent job stage (or a bounded parent job) so process-loss recovery observes fuel, cancellation, progress, and the 8 ms ceiling.

### 5. Large one-shot allocations remain in interactive stages

`CheckpointBuild::new` reserves the calculated full checkpoint capacity in one call (`wfc-engine/🧵️job/🦀️component.rs:318-327`), and `CommitBuild::new` reserves the full assignment envelope (`:337-340`). These constructors execute from `step` at `:1192-1206` and are not covered by an admission maximum that proves the allocation remains below 8 ms. Incremental serialization does not make its initial full-capacity allocation incremental.

Use bounded pages/chunks with incremental growth, or add an explicit maximum envelope plus debug/release maximum-admission timing evidence that includes allocation pressure.

### 6. Warning denial is masked too broadly

`📦️packages/🦀️rust/📦️glue.rs:1185-1186` applies `#[allow(dead_code)]` to the entire private WFC engine module, while `🦀️component.rs:2` allows `unused_doc_comments` and `unused_qualifications` for the entire procedural crate. This prevents `-D warnings` from demonstrating hygiene in the affected scope. Mount only the production subset, gate test-only leaves, or narrow every allowance to the actual item/macro boundary with a local justification.

## Positive Evidence Confirmed Statically

- `WfcJob::step` checks cancellation and operation/generation freshness before work and after each consumed fuel unit.
- Domain construction, entropy heap inspection, choice removal, raw topology arc access, compatibility-word union, restriction removal, trail restoration, checkpoint encoding, and commit encoding have persistent cursors.
- Graph/grid topology exposes stable `out_arc_bound`/`out_arc_at` access instead of allocating neighbor vectors.
- Preview payload inspection is capped, terminal assignment encoding is cursor-based, and Assembly consumes the returned commit output instead of rescanning domains.
- The interactive production path contains no private thread, pool, executor, or scheduler.
- The packet report honestly records the pending final browser-Wasm rerun, unstarted WASI build, and final-source native/release reruns.

## Required Exit Matrix

After the blockers are repaired, the immutable-tree gate still requires:

1. independent Terra source audit;
2. focused debug and release tests, including maximum-admission preview cadence, restart-as-job, fixed-work uniform sampling, cancellation, and 1/2/4/default real `WorkerPool` replay;
3. production native development, strict-warning, and release builds;
4. production `wasm32-unknown-unknown` and `wasm32-wasip2` builds;
5. mounted public inference/action/effect-to-job-to-preview/checkpoint/commit freshness coverage rather than only direct/private job driving.

## Read-Only Commands

```text
sed -n '1,420p' <P7>/📓️p7a-wfc-job.md
rg -n "struct WfcJob|MaterializeCheckpoint|MaterializeCommit|run_to_completion|out_arc_bound|out_arc_at" <procedural>
sed -n '1,1265p' <wfc-engine>/🧵️job/🦀️component.rs
sed -n '1,260p' <assembly>/💡️inferences/🦀️component.rs
rg -n "allow\\((dead_code|unused_doc_comments|unused_qualifications)" <procedural>
```
