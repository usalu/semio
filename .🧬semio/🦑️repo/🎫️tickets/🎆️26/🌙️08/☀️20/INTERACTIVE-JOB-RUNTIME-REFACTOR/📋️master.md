# Interactivity-First Refactor — One Worker Scheduler, Resumable Jobs, 8 ms Ceiling, Zero External Dependencies

## Context

At commit `95b8688ee2` Semio has the right primitives but no hard interactivity guarantee. Verified in-code at these anchors:

- **Role-split threads**: `ThreadPlan { kernel, shards, io_workers, compute, epoch_ticker }` and debug-only `ThreadBudget` in `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/🦀️component.rs` (~lines 275–365); `TokioHostRuntime` builds a tokio pool in `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/🦀️component.rs` (~238–326); one OS thread per shard in `…/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs` plus per-shard forwarder threads that poll with `block_on` in `…/💻️os/🖥️host/🎠️activation.rs` (~87–126).
- **Opaque blocking**: `run_blocking(FnOnce)` and `block_on` in the async crate (~378–500); call sites in services `ComputeScheduler` (~605), db storage, host activation (~1956–1985), CLI entry `…/🏃️run/📦️bin.rs` (~254).
- **Run-to-completion UI**: `UiRuntime::transact` in `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs` (~246–261) drains deltas → routes intents → effect fixpoint → presents dirty surfaces → reconciles trees, all synchronously on the UI thread (count limits `PROJECTION_DRAIN_LIMIT=256`, `EFFECT_STORM_BUDGET=64`, but no elapsed-time yield).
- **Synchronous host callbacks**: `WindowDelegate { handle_event, handle_metrics, redraw }` in `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs` (~252–274); winit `NativeHost` (~347–533), browser `CanvasHost` RAF → `redraw` (~617–690); `WinitApp`/`OsHost` in `…/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs`.
- **Full-result simulations**: Puzzle 3D fill (`FillBuilder`, 12 ms soft cap `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS`, indivisible `solid_overlap_volume`) under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/⏳️precompute/`; monolithic WFC `compile_and_solve` (10.9k LOC) under `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/…/💡️inferences/`; FEM engine under `✏️s/🔨️modules/🏗️fem/⚙️engine/` (algebra full of non-suspending `async fn`); Energy `Engine::run` full-run loop in `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs` (~19–100).
- **Actor budgets are contracts, not enforcement**: `Budget`/`Lane`/DRR `Scheduler`/`SceneStore` in `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/🦀️component.rs` (lane defaults 4/16/50/200 ms).
- **External deps**: Rust — serde, serde_json, thiserror, tokio, ts-rs, wasm-bindgen/web-sys, wit-bindgen, wasmtime, wgpu, vello, swash, parley, fontdb, winit, arboard-equivalent clipboard, flate2/libz-sys/miniz_oxide/zip, image, gltf, bytemuck, rusqlite/sqlx/neo4rs, axum, reqwest, rayon, getrandom, spade (FEM), nalgebra+parry3d (procedural), uuid, jsonschema, prost, notify, objc2, naga, criterion. JS — React, Nx, esbuild, Vite plugins, Vitest, Playwright, Storybook, ESLint, chevrotain, MDX/remark/rehype, binaryen, dependency-cruiser.

**Governing rule:** *No interactive operation is a function call that runs until the operation is finished. Every interactive operation is a persistent state machine whose individual step is bounded, cancellable, observable, and preview-producing.* 8 ms is the hard ceiling for any single UI callback or worker step (normal steps 0.5–2 ms); a logical operation may last seconds but must continually yield and publish intermediate state. Goal = maximum interactivity, not command throughput. Endgame = zero external dependencies (runtime AND build/test/tooling), everything programmed bottom-up. `./compose` is out of scope.

## Handoff & fleet model

This plan was authored by the main chat on **Fable 5 High**. Execution protocol:

1. **User switches the main chat model to Opus 5 High** (model selector) — the main chat becomes the **coordinator**: owns tickets, wave dispatch, merge arbitration, gate verification. Coordinator never implements; it reads, dispatches, verifies.
2. **Executors: Sonnet 5 High** — `Agent(subagent_type: "general-purpose", model: "sonnet")`, one per work packet. Dispatch each wave as one message with many parallel Agent calls, `run_in_background: false` (background children die when the coordinator's turn ends — confirmed repo hazard). Max useful parallelism per wave is bounded by file-disjointness, not agent count: packets below are sliced to be file-disjoint so every packet in a wave can run simultaneously.
3. **Scouts: Haiku 4.5** — `Agent(subagent_type: "Explore", model: "haiku")` for all read-only work: inventory sweeps, call-site enumeration, gate audits, post-wave verification scans. Fan these out maximally (they are cheap and file-disjoint by construction).
4. **Hard rules briefed into every subagent prompt**: no git-modifying commands, no worktrees (`isolation` never set), never call `ticket_close`/`ticket_reopen` (coordinator only, always with explicit path), scratch files inside the ticket folder only (`.txt` not `.log`), `[DEBUG]` prefix on temp logs, don't inline `#[path]`-split taxonomy files (Single-File-Repo scope notes), validate with real builds/tests before claiming success, wasm targets must be built after framework-wide API changes (`#[cfg(target_arch="wasm32")]` code doesn't compile natively).
5. **Tickets**: one **master ticket** opened via `ticket_open` under goal `🎯r2602🎯runningsketchpad`, title "Interactive Job Runtime Refactor" — holds this plan, the dependency/tool inventories, and the gate dashboard. One **per-phase ticket** opened when its wave starts (e.g. "One-Pool Worker Runtime", "Resumable Frame Transaction", …), closed with explicit path + ≥1 ASCII file path in `files`, `📌️important.md` cleared last.
6. **Concurrent devs**: before every wave, coordinator runs `git log --date=iso` against wave start to detect peer churn (auto-commit messages have fake dates); repo-wide cargo breakage may be another session's refactor — check shared files before assuming fault.

## Fixed architecture decisions (agents do not re-litigate)

1. **One process-wide worker pool.** `cpu_worker_count = max(1, N−1)` native-interactive (UI thread reserved); `N` headless. Kernel, shards, I/O, compute, timers become logical **lanes** on one substrate; no nested/subsystem pools; ≥1 slot effectively reserved for interactive+preview lanes when ≥2 workers. Release-mode **checked permit ledger** (over-allocation errors; never wraps).
2. **Capability split.** `UiThreadToken` (zero-size, unforgeable) vs `WorkerContext { budget, cancellation, operation, generation }`. UI thread: event read+timestamp, fixed-size enqueue, coalesce replaceables, atomic snapshot swap, cursor/IME/accessibility directives, bounded pre-prepared present submission. Everything else demands `WorkerContext`.
3. **Resumable job protocol.** `InteractiveJob::step(&mut StepContext) -> StepOutcome{Yield, PreviewReady, CheckpointReady, Complete(CommitCandidate), Cancelled, Fault}`; `StepContext { deadline, fuel, cancel, operation, generation, previews }`. Synchronous + explicitly resumable. `async fn` only where genuine suspension exists (the FEM algebra's decorative `async` gets stripped). `run_blocking` removed from interactive APIs; `block_on` only at process/test entry points; batch adapters drive the same jobs to completion.
4. **Timing.** UI event ≤1 ms, present ≤2 ms, interactive step 0.5–1 ms, sim step 1–2 ms, background 2–4 ms; **<8 ms hard ceiling everywhere**. Lane budgets (16/50/200 ms) become grants of many bounded steps. Watchdog on every UI callback and worker step: debug/CI fails the latency test; production quarantines the violator after return, keeps last valid snapshot.
5. **FrameTransaction.** `transact()` becomes a persistent worker-owned state machine — DrainProjectionDeltas → RouteIntents → FlushEffects → PresentSurface → ReconcileTree → BuildRenderPackets → PublishSnapshot — every stage cursor-resumable with fuel/time/item/node/byte limits, deterministic ordering and atomic publication preserved, new input prioritized over stale reconciliation, obsolete work abandoned on newer revision/generation.
6. **Three state classes.** Authoritative document snapshot (model actor only; immutable, revisioned, undoable) / operation state (worker-owned transient) / preview overlay (revisioned, replaceable, never persisted/undoable). Operations carry `operation_id, base_document_revision, input_generation, preview_sequence, deterministic_seed`; input change ⇒ increment generation, cancel old, discard stale previews; commit only on matching base revision+generation, else explicit rebase or discard.
7. **Progress as data.** One progress stream vocabulary (Started, StageChanged, CandidateTested, PreviewPatch, Diagnostic, Checkpoint, CommitCandidate, Completed, Cancelled, Failed) with record fields per the spec. Channel policies explicit: pointer/hover/resize latest-wins; preview geometry coalesced by (operation, entity, stage); commits+checkpoints lossless bounded; diagnostics bounded ring; telemetry lossy; large geometry byte-credit. UI never receives direct simulation callbacks.
8. **Determinism under parallelism.** Stable entity/partition ordering, worker-local accumulation, deterministic merges, seeded checkpointed RNG, sequencer for first-acceptable decisions, revision/generation validation at publication, replay logs. Same snapshot+inputs+seed ⇒ identical final result at any worker count.
9. **Plugin host: owned WASM interpreter** (user decision). Keep the WASM plugin ABI and sandbox; replace wasmtime + wit-bindgen with a repo-owned WASM core-spec interpreter — fuel-metered and checkpointable at instruction granularity, which *is* the resumable-job enforcement mechanism for guest code (a guest that can't cooperate is preempted by fuel exhaustion at the interpreter level). Baseline JIT later, behind the same interface. Browser keeps the platform's WASM engine (platform boundary).
10. **Storage: owned event-log engine.** rusqlite/sqlx/neo4rs replaced by a repo-owned append-only event log + snapshot + index store, matching the mandated CQRS/event-sourcing (no CRUDs, no CRDTs). Persisted-local / persisted-shared / ephemeral-local / ephemeral-shared distinguished at the store API.
11. **Zero-dependency boundary.** No third-party source/package deps in runtime, build, test, codegen, docs, release tooling. Compiler, linker, OS/browser ABI, graphics driver, browser WASM engine = platform boundaries. No verbatim vendoring. Every replacement: dual-run differential tests → parity/perf gates → new default → old deleted. Dependency freeze starts at Phase 0. No flag-day purge before runtime+UI contracts are stable.

## Phases

Waves are dependency-ordered; **within** a wave all packets are file-disjoint and run as one parallel Sonnet fleet. Every phase ends with a Haiku audit fleet verifying the exit gate before its ticket closes. Phase tickets: `ticket_open` at wave start; all listed exit gates go in the ticket's status file.

### Phase 0 — Observability & freeze (fleet: 1 Sonnet framework packet + 4 Haiku inventory scouts)
New `🧰️framework/🔨️modules/⏱️trace` module: UI-callback timers, worker-step timers, thread-ownership assertions (`debug_assert_ui_thread`/`assert_worker`), active-worker + permit counters, queue item/byte counters, operation/generation tracing, preview-latency and cancellation-latency tracing. Static forbidden-call audit (a `📜️script.ts verify`-wired lint pass) for UI-reachable modules: `block_on`, `run_blocking`, filesystem/network/clipboard waits. Haiku scouts produce, into the master ticket: (a) full dependency inventory (crate → users → purpose → replacement owner) as machine-readable md/json, (b) interactive-command inventory across all plugins (`.editor_mutation_roster` sites, dispatch/action-bus registrations in `🧰️framework/🔨️modules/🔀️dispatch/`, `🎯️action-bus/`), (c) thread/pool-creation census (`std::thread::spawn`, rayon, tokio builders), (d) non-suspending `async fn` census. Dependency freeze: CI check rejecting new externals.
**Gate:** every input/frame/operation/preview/commit followable by ID through a trace; inventories complete.

### Phase 1 — One-pool runtime (fleet: 4 Sonnet packets)
- **P1a (async crate)**: replace `ThreadPlan` role counts + debug `ThreadBudget` with global `WorkerPool` (work-stealing deques, logical lanes, deadlines, checked permit ledger), timer wheel on the pool (kills the epoch-ticker thread), keep scopes/cancellation/`ChannelPolicy` (extend capacities to items+bytes). `run_blocking` deleted from the host trait; `block_on` gated behind an `entrypoint` feature.
- **P1b (services)**: `TokioHostRuntime` reimplemented on `WorkerPool` (tokio itself removed later in Phase 9, but the pool becomes the only thread owner now); `ComputeScheduler::run_blocking` call sites → job submissions; db storage I/O → I/O lane.
- **P1c (actor kernel)**: `Kernel`/`Scheduler` (DRR, both levels), `ShardTable` re-hosted: shard executor threads and forwarder threads deleted; actor turns become pool tasks on lanes; `block_on` polling loop in `🎠️activation.rs` removed; mailboxes/budgets/failure ladder kept.
- **P1d (admission)**: worker admission control — background lanes cannot occupy all workers while a UI is live; interactive-reserve slot.
**Gate:** thread census shows exactly UI thread + pool workers; no subsystem can create a CPU pool; permit over-allocation errors in release.

### Phase 2 — Job & progress protocol (fleet: 3 Sonnet packets)
- **P2a**: new `🧰️framework/🔨️modules/🧵️job` module: `InteractiveJob`, `StepContext`, `StepOutcome`, `OperationId`/generation/`Checkpoint`/`CommitCandidate`, structured child jobs (parent scope-owns), watchdog integration, batch-drive adapter.
- **P2b**: progress stream types + channel-policy matrix + preview overlay store (extends actor `SceneStore` with generation-tagged preview snapshots distinct from committed scenes); commit validation (base revision + generation).
- **P2c**: actor bridge — generalize `Payload::JobStep`/`Suspend`/`Resume`/`TurnStatus::CheckpointReady` onto the job protocol (one job step per turn); deterministic-replay log format + harness; synthetic torture job (long-running, previewing, cancellable) as the protocol's conformance test.
**Gate:** synthetic job stays responsive, previews continuously, cancels <8 ms p99, replays deterministically at 1..N workers.

### Phase 3 — UI thread isolation (fleet: 4 Sonnet packets)
- **P3a (host)**: `WindowDelegate` → enqueue-only host contract: `handle_event`/`handle_metrics` become fixed-capacity enqueue + coalesce (pointer/wheel/resize/hover latest-wins); `redraw` = "atomically acquire newest prepared `RenderSnapshot`, submit, apply cursor/IME/accessibility directives". Winit `NativeHost` + browser `CanvasHost` adapted; browser clipboard stays async-fed as `DispatchEvent::Paste`; native clipboard moves to I/O lane.
- **P3b (input worker)**: event interpretation, `PointerRegistry`, hit-testing against last committed `DispatchTree` index, tool-state transitions → input actor on the pool; input generation IDs prevent late hit results acting on stale pointer state; speculative drag overlay path.
- **P3c (tokens)**: introduce `UiThreadToken`/`WorkerContext` and thread the requirement through UI-reachable APIs; forbidden-call audit turns from warn to deny.
- **P3d (renderer seam)**: `OsHost`/`WinitApp` split so frame *building* leaves the UI thread; UI thread only presents prepared packets; `RenderSnapshot` type (scene revision, preview generation, draw packets, clip state, hit index, directives, damage, upload refs).
**Gate:** instrumentation proves zero product/plugin/domain code on the UI thread; UI callback p99 ≤2 ms in stress scenes.

### Phase 4 — Puzzle 3D vertical slice (fleet: 3 Sonnet packets, file-disjoint within the puzzle plugin)
- **P4a (fill job)**: `FillBuilder`/`FillStep` → nested resumable states (SelectTarget, EnumerateCompatibleCandidate, ConstructCandidateTransform, QueryBroadPhase, TestCollisionPair, RunNarrowPhaseSampleBatch, Accept/Reject, PublishPlanPrefix) with persistent cursors (target, candidate, broad-phase, pair, sample), RNG state, accepted sequence, rejection reasons. 12 ms cap → 1–2 ms steps. Deterministic candidate-index sequencer (later-finishing candidate can't beat an earlier one still under evaluation).
- **P4b (collision)**: `solid_overlap_volume` → sample-batched state machine; incremental spatial hash/BVH over placed bodies (no full scans).
- **P4c (preview)**: publish translucent ghost of current candidate, target slot, broad-phase set under test, colliding IDs, contact/overlap samples, rejection reason, accepted prefix + search count; coalesced at display cadence; renderer shows latest tested candidate even if rejected ms later.
**Gate:** adversarial fill scenes never exceed 8 ms/step; live search visible; fill result byte-identical across worker counts and to replay.

### Phase 5 — Resumable UI transaction & renderer (fleet: 5 Sonnet packets)
- **P5a**: `UiRuntime::transact` → persistent `FrameTransaction::step()` with the seven cursor-resumable stages; keep revision guards (`is_stale_intent`), effect ordering, `EFFECT_STORM_BUDGET` semantics as per-stage limits; commit only at consistent boundaries; abandon superseded work.
- **P5b**: reconciliation (`SurfaceReconciler`) + presentation node-cursor resumable.
- **P5c**: layout + text shaping (parley/swash paths for now) chunked into jobs.
- **P5d**: tessellation + draw batching + GPU upload-packet preparation as jobs (wgpu target).
- **P5e**: multi-window/surface scheduling on lanes; resize-storm and effect-storm stress tests.
**Gate:** input responsive during effect storms, large tree changes, resize storms, multi-window invalidation; no transaction stage step ≥8 ms.

### Phase 6 — FEM (fleet: 6 Sonnet packets over `✏️s/🔨️modules/🏗️fem/⚙️engine/` + plugin artifacts)
- **P6a**: strip decorative `async` from algebra (`MatD`/`VecD` et al.); staged job graph skeleton (ValidateReferences → … → Finalize).
- **P6b (meshing)**: replace render-time full triangulation/tetrahedralization previews with incremental mesh jobs — coarse valid mesh first, batch-published refinement by edge-length/quality queues, deterministic element IDs, chunked boundary/adjacency, region-scoped cancel/restart. (Also: replace `spade` with an owned Bowyer-Watson — differential-tested; may land in Phase 9 if riskier.)
- **P6c (assembly)**: per-element chunked assembly into worker-local triplet buffers, deterministic merge, published "assembled" element marks.
- **P6d (iterative solve)**: `PcgJob` with persistent x/r/z/p/Ap, iteration, residual, preconditioner state; publish displacement field, residual vectors+norm, reactions, approximate contours per bounded iteration batch; coarse-mesh loose-tolerance first preview.
- **P6e (direct + eigen)**: LDLT split by column/block/supernode into job steps; subspace iteration job publishing per-iteration eigenvalue estimates, mode shapes, per-mode residuals, converged counts.
- **P6f (visual language)**: unmeshed/coarse/refined region distinction, assembling elements, load/support glyphs, residual and displacement live fields, converged-vs-unconverged marking, validated-final marking. 2D first, then 3D.
**Gate:** edits cancel stale solves immediately; coarse preview <50 ms; final results meet reference numerical tolerances; no step ≥8 ms.

### Phase 7 — WFC, Puzzle 2D, Energy (fleet: 3 Sonnet packets, disjoint plugins)
- **P7a (WFC)**: `compile_and_solve` → resumable solver job (InitializeDomains, FindMinimumEntropySlot, ChooseCandidate, PropagateCompatibilityEdge, DetectContradiction, BacktrackTrailEntry, CommitSlot, Complete) with per-slot domain bitsets, entropy heap w/ generations, propagation queue, trail, seeded RNG, checkpoint stack; bounded compatibility-edges per step; publish active slot, candidates, tested tile, propagation wave, changed domains, contradiction/backtrack path, incomplete grid as preview overlay; authoritative grid only on accepted generation; sequenced collapse decisions (parallel only for deterministic domain scans/regions). Reuse existing wfc-engine submodules (domain/motif/trail/constraint) — restructure, don't rewrite the math.
- **P7b (Puzzle 2D)**: same fill/job pattern as Phase 4 against the 2D board (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/`).
- **P7c (Energy)**: `Engine::run` → `EnergyJob` stages (Validate, ResolveWeather, PrecomputeZone/Surface/Fenestration, Initialize, WarmupDay/Iteration, RunZoneTimestep, RunSystemSubstep, AggregateMeters, PostprocessBlock, Finalize); publish zone temps, heating/cooling demand, surface heat transfer, HVAC/fan loads, facility totals, warmup convergence, running time-series per bounded block; four labeled quality tiers (steady-state estimate → design-day → coarse-timestep → final); parallel zone/component work with deterministic reductions, chronological state; checkpoints after warmup iterations and timestep blocks; `Engine::run` kept as batch adapter driving the job. No solver-specific schedulers anywhere.
**Gate:** all three display internal search/simulation progress, cancellable at every stage, deterministic under worker-count variation.

### Phase 8 — Every remaining tool (fleet: ~8 Sonnet packets sliced by plugin: cad, draw, layout, block, process, sourcing, vcs, animate + framework commands)
Tool registry: tools supply an `InteractiveJob` factory instead of event callbacks (`Tool input → Operation spec → Job factory → Progress/preview → Commit`), wired through dispatch/action-bus. Small tools complete in their first step but use the same path. Imports/exports, serialization, compression, selection expansion, snapping, boolean geometry, routing, animation baking, search, diffing, package ops — same contract. Classify every command from the Phase 0 inventory: `migrated | batch-only pending rewrite | forbidden from UI | deleted`; release build rejects unclassified.
**Gate:** inventory 100% classified; zero unmigrated interactive callbacks reachable from UI.

### Phase 9 — Runtime dependency removal (fleet: staged Sonnet packets, each dual-run differential-tested)
Order (dependents first where independent, one packet each): thiserror → plain enums/Display; serde/serde_json/ts-rs → internal schema description + existing `🎒️pack` codec extended (`encode_record_body`/`decode_record_body` for wire values) + owned streaming JSON + schema-to-language generation with versioning tests; tokio → the Phase 1 pool + owned platform I/O reactor (kqueue/epoll/IOCP/browser); flate2/libz-sys/miniz_oxide/zip → owned streaming DEFLATE/ZIP (only formats actually consumed); getrandom/uuid → owned seeded RNG + OS entropy shim; image/gltf → owned decoders for formats actually used; rayon → pool lanes; jsonschema/prost/notify/reqwest/axum → owned equivalents scoped to actual consumers; rusqlite/sqlx/neo4rs → owned event-log store (decision 10); **wasmtime + wit-bindgen → owned WASM interpreter** (decision 9; biggest packet — instruction-fuel metering replaces wasmtime fuel, checkpoint/resume at interpreter level, differential-tested against wasmtime on the full plugin suite before wasmtime is deleted); nalgebra/parry3d/spade → owned math/collision/triangulation (differential-tested against Puzzle/FEM suites).
**Gate:** cargo dependency audit shows only owned crates + platform boundaries; parity/perf gates green for every replacement.

### Phase 10 — UI & tooling dependency removal (fleet: staged Sonnet packets)
winit → direct Win32/Cocoa/Wayland+X11/browser hosts behind the existing host contract; wgpu/naga/vello/swash/parley/fontdb → direct D3D12/Metal/Vulkan/WebGPU bindings + owned shader pipeline + owned text shaping/layout (differential against Phase 5 render fixtures); wasm-bindgen/web-sys → generated low-level imports/exports + small owned JS boundary; clipboard → native APIs on I/O lane. React removed product by product once the worker-built UI covers each app (dev harness `🧩️multi.tsx`, `FrameworkOsShell`, backbone worker stay until parity); Storybook → internal scene fixture browser; Playwright → owned WebDriver/CDP client + native accessibility automation; Nx/Bun-orchestration/esbuild/Vitest/ESLint/chevrotain/MDX chain → repo-owned build graph, cache, test runner, bundler, lint/format, docs pipeline, package/release tool (extending `📜️script.ts` as mandated). launch.json entries updated to the owned commands, following existing order/grouping/naming.
**Gate:** package-lock/Cargo/Python/Go/CMake audits report zero third-party deps under the declared boundary; CI rejects new ones.

## Verification (continuous, wired into `📜️script.ts` verify/test targets and CI)

- **Timing**: no UI callback or worker step ≥8 ms (watchdog-enforced tests); UI callback p99 ≤2 ms; interactive ack within one frame; first substantive sim preview ≤50 ms; active previews ≥ every 33 ms under load.
- **Cancellation/freshness**: superseded operation observes cancel ≤8 ms p99; no stale-generation preview after newer accepted; stale commits rejected; window/document close drains all descendant jobs.
- **Threading**: no domain/plugin code on UI thread (ownership assertions); no sync FS/network/clipboard/db reachable from UI; global core limit respected; no nested pools; `block_on` only at approved entry points (static audit).
- **Boundedness**: every mailbox/stream item+byte limited; pointer/resize storms coalesce; user commands never silently dropped; preview overload drops previews, never commits.
- **Correctness**: replay tests reproduce all commits; identical results across worker counts/scheduling; FEM/Energy meet reference tolerances; previews visibly distinct from validated finals; Puzzle placement sequence deterministic.
- **Resilience**: fault injection — worker panic, stuck job, cancel races, stale generation, memory pressure, queue saturation, storms, device loss; failed simulation never freezes UI; last valid snapshots survive faults.
- **Dependencies**: dependency audit empty at the boundary; CI rejects new externals from Phase 0 onward.
- Run levels: wire into existing `fundamental/quick/long/exhaustive` nextest/vitest tiers; wasm targets built in CI for every framework API change.

## First actions on approval (coordinator, Opus 5 High)

1. `ticket_open` master ticket under `🎯r2602🎯runningsketchpad`; copy this plan + explorer maps into the ticket folder as md files (all research lives in ticket files, not chat).
2. Open Phase 0 ticket; dispatch the Phase 0 fleet (1 Sonnet + 4 Haiku scouts, parallel, foreground).
3. Verify Phase 0 gate with a Haiku audit; close Phase 0 ticket (explicit path, ASCII file in `files`, `📌️important.md` cleared last); proceed wave by wave.

---

# Plan revision, after executing Phases 0, 1 and 1.5

The original plan assumed a compiling repository. It was wrong, and the correction is the single most important thing learned so far. Live status is tracked in `📌️status.md`; this section records what changed in the *plan itself*.

## Revision 1 — Phase 1.5 inserted as a prerequisite

The repo did not build at the baseline commit. `cargo check --workspace --all-targets --keep-going` revealed roughly **20,000 in-scope errors**, essentially all of one class: `async fn` called without `.await`, the mechanical fallout of `AGENTS.md:44` ("You SHOULD implement everything async when it makes sense") applied to ~53,000 functions of which Phase 0 measured 88.28% as never suspending.

Phases 3 and 5 rewrite the UI runtime and the frame transaction. They cannot land on crates that do not compile. So de-asyncing moved from a Phase 6/7 clean-up to **Phase 1.5, a hard prerequisite**.

## Revision 2 — measurement methodology is part of the plan

Three traps cost real time and must be treated as standing rules:

1. **Always use `--keep-going`.** A crate that fails to compile stops `rustc` from reaching its dependents, so plain `cargo check --workspace` reports a small fraction of reality. The damage was discovered in layers — the derive macro hid `framework-machine` (408), which hid `framework-hash` (29), which hid `os-infinite` (2,155) and `s-plugin-stdio` (16,725).
2. **Never judge progress by a workspace total.** Totals are non-deterministic run to run (1058 → 569 → 957 → 1145 with no edits between) precisely because of that masking. Measure per-crate with `cargo check -p <crate> --all-targets`.
3. **Check platform-gated crates on their real target.** "452 errors" in the wasm32-only webgpu backend and "332" in the Windows-only d3d12 backend were pure artifacts of checking them on macOS; zero mentioned `Future` or `await`. Before attributing an error to the async class, confirm the diagnostic actually names `Future`/`await`.

Related structural fix: `compile_error!` platform gates make a clean workspace build impossible forever, since `cargo check --workspace` visits every member regardless of dependency edges. All four render backends now use cfg-gated empty libs, with "wrong platform is an error" preserved at the consumer's target-gated dependency edge.

## Revision 3 — tooling over headcount

Hand-repairing ~20,000 errors crate by crate does not scale, and more parallel agents would not have fixed that — the work is sequential by dependency order and each fix changes what the next measurement means. The answer was a **compiler-driven codemod** (`🔧️r13-deasync-codemod.ts`): consume `cargo check --message-format=json`, apply span-keyed edits ranked by rustc's own machine-applicable suggestions, iterate to a fixpoint under a monotonic guard that reverts and halts on any regression, journalling every edit so the tool can undo itself (there is no `git stash` here).

It took `s-plugin-stdio` from 16,725 errors to 5 in one wave of 8,327 edits, and cleared `os-kernel` (145), `geometry` (8), `mesh-engine` (11) and `math` (9) to zero.

**The generalisable lesson for the later phases:** mass mechanical change in this repo should be compiler-driven and span-keyed, never name-keyed. R12 measured the difference concretely — of ~86 grep-matched call sites only ~14 were genuine edits, and two *unrelated* same-named `hash_bytes` functions existed that a textual replace would have silently corrupted. The one corruption the codemod did produce came from the single place it guessed (a `let-else` pattern sub-span mistaken for a call), and was closed with a denylist plus a regression self-test.

## Revision 4 — carried-forward work items

- **Def-use residue.** ~1,250 remaining diagnostics share one shape: the Future-typed expression is a bare `let`-bound local, not a fresh call. Resolving them needs data-flow tracing to the assignment site — a harder tool than the span-local one. This is the top remaining work item.
- **Recursive async cycles** (E0733) in `semio-s-imperative` need either cycle extension or a human `Box::pin` decision.
- **Trait-shape lockstep.** De-asyncing a trait method forces matching changes in macro-generated impls; `framework-machine` ↔ `machine-derive` already required this. Any future trait de-async must be planned as a coordinated multi-site change.
- **Phase 1 leftovers**: `ManualRuntime::drive()` cannot observe completions from the real cross-thread pool (no-op waker); `WorkerPool::shutdown()` deadlock; `ComputePool::run_blocking` still takes an opaque `FnOnce` and should lose it when Phase 3 introduces the capability tokens.

## Revision 5 — Phase 2 input

`semio-framework-machine` already has much of the job protocol's shape: a `PersistedSnapshot`/`step()` round-trip, a `Command` effect/kernel split, and an `Inspector` event stream. But its `run_to_completion` drains to quiescence bounded by a *count*, not a deadline, with no mid-macrostep yield/resume, no preview channel and no fault channel. Phase 2 should make that microstep loop budget-aware and resumable — its pending-trigger queue is already a reified value — rather than adopting it wholesale as `InteractiveJob`.
