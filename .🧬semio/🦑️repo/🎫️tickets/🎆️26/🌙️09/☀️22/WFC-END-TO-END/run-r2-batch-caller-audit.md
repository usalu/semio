# Current Batch Caller Audit

Read-only source review, 2026-11-05. No build or admission queue was started. These are current API observations, not native compilation evidence.

## Canonical Entry

The public worker API is `WorkerJobSessionAdmission<J, BatchJobSession<J>>::new()`, then repeated `advance(&mut Option<J>, &mut Option<BatchJobParams>, &mut Option<BatchJobSession<J>>, &mut StepContext) -> Result<bool, OriginalTurnFailure>` on the same genuine original caller. Source: `🧰️framework/🔨️modules/🧵️job/👷️worker/🚪️session-admission/🦀️.rs:33–60`; public reexport in worker root line28.

Every advance validates `cx.original_source_parent()?.validate_borrowed_context(cx)` and the retained `OriginalSourceBinding` (line41). Birth has separate source, authority, slot, frame, node, handoff, admission-close and output turns. Pending admission remains owned and needs same-source paid close on cancellation/error; Drop asserts terminal emptiness (line74). It is not a synchronous constructor replacement.

For an existing plugin caller, the current higher-level production entry is `RefusedWorkerSource::pending(job,(operation,generation))`, then `admit_pending_batch(config,&mut output,cx)`. Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🚪️session/🦀️.rs:11–29`. This wrapper stages binding, an original cancellation alias and BatchJobParams as separate paid events; params use the supplied context's operation, generation and clock. It does not fabricate a replacement caller.

## Actual Production Consumers Beyond WFC

- Plugin preview driver: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🦀️.rs:870–895`. Pending admission at879; same caller step883; checkout884; exact receipt taken/checked885–886; outcome888; acknowledgement881 and resume882 on later turns; close858–865.
- Runtime live cleanup: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs:101–120`. It retains pending producer, borrows config from cx, admits at104, steps108, checks receipt116 before inspecting outcome117. Close cleanup has the parallel call at299.
- Services compute pool uses the same admission generic with MountedWorkerJobSession rather than Batch: `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧰️compute-pool/🦀️.rs:136–159`. Admission executes inside `caller.turn_funded(retained_recipient,...)`; partial state is retained in completion. This is a legitimate headless source/custody pattern.
- Services `ComputeOriginalCaller::admit(operation,generation,stock,deadline)` is a real bounded source-parent birth API, not a test fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧮️compute/🎟️original-caller/🦀️.rs:35–49`. It pays cancellation root, payload authority and original source parent from stock. Its turn65–74 borrows context from that parent, narrows to remaining stock, preserves clock binding and cumulative receipt. A plugin job already carrying a genuine cx should retain that source rather than birth this unrelated services caller.

## Avoid Obsolete Examples

Worker test helpers using OriginalJobFixtureCaller are cfg(test), not runtime entry points. Several source-string coverage tests still mention removed `BatchJobSession::try_admit_owned` or `try_new`; these are not reliable canonical API examples. The clipboard renderer consumer also retains older preparation/cancellation signatures (scene-intents-and-clipboard source283–285), so it is not the appropriate migration template.

Current Batch methods are `step_on_original_caller(cx)`, `checkout_outcome()`, `take_checked_out_retained_step_receipt()`, `checked_out_outcome() -> Result<Option<JobOutcomeView>,ValueError>`, `acknowledge_outcome(cx)`, `resume()`, `begin_close()`, and `close_step(cx)` in worker/🧰️mounted-and-batch-sessions source129 onward. No current constructor justifies resurrecting self-funded context or closing outside the original caller.


## Finite Headless Inference Bound Review

Root is migrating all five artifact HeadlessSite literals to explicit16M item/1GiB copy-capacity-release/depth256 lifetime stock,100M total fuel and60Mµs original duration. These are finite admission ceilings, not eager memory allocations or proof of all legal model sizes. Per-step grant/fuel must remain unchanged and merely narrow the original source.

Exact clock source is job/🕰️clock-identity-and-budget/🦀️.rs49–58. default_now_us reads trace::try_now_us; an unbound bare-Wasm host returns None and must refuse admission rather than synthesize time. logical_now_us advances one microsecond per clock READ in a persistent thread-local counter. It does not represent wall time or reset to zero per solve. Therefore derive the total absolute deadline with checked_add(duration) from the same initial supplied clock once; a literal absolute60M deadline would incorrectly fail later tests on the same thread, and per-turn recomputation would renew the lifetime bound. Constant clock0 test-driver contract is distinct from artifact solve_with_clock(logical_now_us) tests.

A60M logical duration permits60M clock reads, which is independent of100M business fuel and may exhaust first. A60s real duration is a genuine cap even if many short steps are making progress; report a bounded refusal, not a solver contradiction or universal support failure. Source stock counts cumulative allocated capacity, copies and releases, not peak resident memory; repeated payload/checkpoint publication can consume1GiB even with low live memory. No reviewed input size limits prove these ceilings cover every legal graph/bitmap/grid instance.

Actionable admission obligations sent to native owner: missing/backward/overflowing clock must fail cleanly; deadline/fuel/retained exhaustion must leave the driver loop instead of retrying zero-progress Pending forever; the same original source must retain sufficient teardown authority and physical release budget. A business limit that consumes all source fuel cannot then be repaired by resetting funding or granting infinite close authority. Native lifecycle tests should exercise a genuinely small lifetime limit alongside normal fixture success.

Existing public run_headless/solve_with_clock signature carries only job,operation,clock,site,finish; no external cancellation handle is exposed by that synchronous convenience route. An internally owned root supports job cancellation semantics and deadline cancellation, but it must not be described as an externally controllable UI cancellation API. Mounted fill tools use their separate public host/run ports and remain the interactive cancellation acceptance path.

Point-in-time read found old headless source during native owner's concurrent migration, so this section is a contract/bound audit, not approval of an unseen completed implementation. No queues or runtime edits made.
