# P7b Independent WFC Exit Audit

## 2026-08-22 implementation response

The independent verdict below is preserved as the historical finding it was. The current source
now addresses its production-integration blockers and is ready for a new independent audit; this
response does not replace that audit and does not claim the deferred Cargo matrix.

- `procedural::plugin()` freezes a metadata-only `s.assembly.solve` route while registering the
  exact `semio.infer` ActionBus factory. Plugin `describe()` now reads the installed plugin's frozen
  route roster, so the host can discover the route without a duplicate synchronous inference
  facade.
- The cold handler dispatches the canonical payload and optional restart checkpoint through the
  factory-owned `s.assembly.inference.request.v1` wire decoder.
- Guest and host scheduling use persistent `WorkerJobSession` state: exactly one bounded
  `InteractiveJob::step` is admitted per shared WorkerPool caller turn. The 50,000,000-fuel / 200-ms
  host relay and host self-requeue path are removed.
- The guest bridge owns a 1-MiB latest preview slot, two-item / 2-MiB lossless checkpoint+commit
  FIFO, and 32-item / 64-KiB diagnostic ring. Saturation and oversize source regressions preserve
  queued lossless items while allowing preview coalescing.
- `ArtifactInferenceRouter` owns active live revision/generation authority, exposes the model-actor
  update handoff, and invokes `validate_commit` against that live pair immediately before returning
  the terminal result. The stale regression changes both values.
- Mounted source regressions cover production registration/collision, `semio.infer` cancellation,
  checkpoint restart after dropping the original job, and exact terminal mutation; actual
  WorkerPool source coverage drives 1, 2, 4, and host-default worker configurations.

Allowed validation on the repaired tree: `rustfmt --check` exited 0 for all nine touched Rust
leaves; `bun ./📜️script.ts verify interactivity tool-jobs --format json` exited 0 with 775/775
bounded commands, zero batch-only rows, one production factory/registration/dispatch, and zero
failures; scoped `git diff --check` exited 0; and the changed bridge/builder/Assembly leaves had zero
debug-output hits. No Cargo command was run. Focused debug/release, native strict/release, and both Wasm
target gates remain explicitly deferred to the serialized Cargo owner.

Date: 2026-08-22  
Scope: Phase 7a Assembly/WFC production path and current-tree evidence only.  
Verdict: **REJECT**

## Decision

The solver source has materially improved: its parent and WFC stages are persistent, the fixed-work uniform sampler is present, checkpoint restore is cursor-driven, and the exact ActionBus key has an idempotent registration implementation. These source properties do not clear the Phase 7 gate.

Two production integration defects remain, and all mandatory current-tree runtime evidence is explicitly absent. No Cargo command was run for this audit, per the serialized/disk-constrained lane instruction.

## Blocking Findings

### P0 — The production `semio.infer` path cannot reach `s.assembly.solve`

`procedural::plugin()` registers `AssemblyInferenceJobFactory` in the process-global `ActionBus`, but neither a caller nor an inference-service registration binds `s.assembly.solve` to the production `semio.infer` cold-job route.

- `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:139-143` only calls `register_assembly_inference_factory(ActionBus::production())`.
- `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:197-205` expressly says Assembly is unmounted.
- A whole-procedural source scan finds no `inference_service`, `ArtifactInferenceService::new`, or `register_artifact_inference_service`; it finds `s.assembly.solve` only at `…/🧬️schema/💡️inferences/🦀️component.rs:17`.
- The actual production cold-job handler calls the unrelated artifact-inference registry: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:27-32`. It has no ActionBus lookup or `s.assembly.solve` dispatch.
- The post-turn host still drives that cold job by a run-to-completion loop with a 50,000,000-fuel, 200-ms grant: `…/🖥️host/🦀️component.rs:2508-2512,2539-2556`.

The factory's local ActionBus test is not a mounted production action/effect → shared worker → preview/checkpoint → freshness-validated commit path. The public `semio.infer` route must resolve the exact Assembly factory (or the factory must be removed in favor of the actual registered contract), and the production scheduler must drive one `InteractiveJob::step` at a time.

### P0 — No production commit consumer validates the Assembly candidate's base revision and generation

The job checks the *StepContext* identity at `…/🧬️schema/💡️inferences/🦀️component.rs:347-355`, but it returns its `CommitCandidate` directly at `:416-419`. The sole Assembly use of `validate_commit` is an isolated assertion in a unit test at `:867-869`; no production Assembly caller invokes it. The generic framework helper exists at `🧰️framework/🔨️modules/🧵️job/🦀️component.rs:131-139`, but is not wired here.

Bind the result to the authoritative model actor and validate the candidate against live revision/generation immediately before the commit is admitted. Add an integration test that changes both values after the final WFC step and proves the candidate is discarded, not committed.

### P1 — Required latest-source runtime matrix is not executed

`📓️p7a-wfc-job.md:5-6,64-65,203-206` records source-only status and says the final focused, native, release, browser-Wasm, and WASI runs are pending after the source repair. Consequently there is no current-tree proof of:

- focused debug **and release** WFC/Assembly tests, including restore, cancellation, allocation pressure, fixed-work RNG, preview cadence, and ActionBus production routing;
- actual shared `WorkerPool` replay at 1/2/4/default worker counts (the WFC source test module at `…/🧵️job/🦀️component.rs:1571-1906` contains no `WorkerPool` use);
- procedural native dev, `-D warnings`, and release builds after the final edits;
- `wasm32-unknown-unknown` and `wasm32-wasip2` builds after the direct dependency/source changes.

These are explicit Phase 7 exit gates, not optional confidence checks. The historic results cannot be carried forward to the altered tree.

### P1 — Progress is emitted as raw JSON bytes without the required channel contract

The live parent and WFC/restore `step()` implementations serialize JSON directly into `StepOutcome::PreviewReady`:

- Assembly: `…/🧬️schema/💡️inferences/🦀️component.rs:167-174`.
- WFC: `…/🧩️wfc-engine/🧵️job/🦀️component.rs:1056-1065`.
- Restore: `…/🧩️wfc-engine/🧵️job/🦀️component.rs:1480-1486`.

No route connects those outcome bytes to a declared latest-wins/coalesced preview channel or a lossless bounded checkpoint/commit channel. The framework's `ChannelPolicy` types alone do not prove such wiring. In particular, the unreachable ActionBus dispatch returns an erased job and has no publication channel (`🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:271-280`).

Introduce a typed progress bridge with explicit item/byte limits and policies: coalesced/latest-wins for previews, lossless bounded for checkpoints and commits. Test saturation: previews may coalesce, but checkpoints and commits must neither drop nor bypass freshness validation.

## Source Evidence That Is Not a Blocker by Itself

- Exact key/schema/classification are correctly defined at `…/🧬️schema/💡️inferences/🦀️component.rs:16-18,440-472`; ActionBus `register_once` preserves a same-factory registration and rejects a different identity at `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:227-256`.
- The Assembly parent owns cursorized weights/modules/rules/model/slots/edges/topology/fixed/restore/solve/map/encode stages and transfers the request fields into the job at `…/🧬️schema/💡️inferences/🦀️component.rs:67-151,181-296,347-436`. The only parent snapshot clone observed is in the documented headless adapter at `:475-491`, not factory creation.
- WFC has persistent stages, fixed-work multiply-high uniform sampling, per-arc cursor access, and cursorized checkpoint/commit materializers at `…/🧩️wfc-engine/🧵️job/🦀️component.rs:23-32,100-105,797-884,941-1042,1496-1567`.
- Restore begins from empty decode/cache state and reconstructs it one bounded cursor unit at a time (`…/🧩️wfc-engine/🧵️job/🦀️component.rs:1112-1183,1272-1435,1459-1493`).
- The 1-MiB checkpoint/commit admission formulas use checked/saturating size checks and `try_reserve_exact` (`:283-315`); nevertheless their watchdog proof must be rerun in debug and release under allocation pressure. A maximum reserve occurs synchronously in `begin_checkpoint` from `step()` (`:935-938,1513-1517,1550-1552`), so static admission is not latency evidence.
- Production mounts the intended WFC subset while test-only leaves are cfg-gated at `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs:1177-1296`. The observed procedural allowance is macro-local at `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:9-17`, rather than a crate-wide allowance.

## Commands Run (Read-only)

```text
sed -n '1,260p' /Users/ueli/.codex/attachments/2225dd4d-c3b6-4564-b4b1-f552928e8ff3/pasted-text.txt
sed -n '1,360p' <P7>/📓️p7a-wfc-job.md
sed -n '1,360p' <P7>/📓️p7-coordinator-interim-audit.md
rg --files --hidden …; rg -n … <procedural> <framework>
sed/nl reads of Assembly inference, WFC job/restore, model, topology, production plugin root, ActionBus, cold-job inference route, and PluginInstanceHandle
rustfmt --edition 2021 --check <procedural root/glue/Assembly/WFC/model/topology>  # exit 0
```

No Cargo/Bun test/build command was run, no target/cache/ticket-status/production source was modified, and no historical test result was treated as current-tree evidence.

## Required Repairs and Runtime Gates Before Re-audit

1. Wire the exact `semio.infer` / `s.assembly.solve` request through the production route to the existing factory and shared WorkerPool one bounded step at a time; remove the batch bridge from any interactive path.
2. Connect preview/checkpoint/commit outcomes to declared bounded channels and the model actor; enforce live base-revision/generation validation at final commit.
3. Add a mounted end-to-end test covering idempotent registration, collision non-replacement, first/cadenced previews across every parent/WFC/restore stage, channel saturation, cancel, checkpoint restart after state loss, and stale commit rejection.
4. With sufficient disk headroom, run the serialized focused debug+release, real shared-WorkerPool 1/2/4/default replay, native dev/strict/release, `wasm32-unknown-unknown`, and `wasm32-wasip2` matrix against one immutable tree. Record command, exit status, timing maxima and p99, and actual runtime route in this ticket before requesting another exit audit.
