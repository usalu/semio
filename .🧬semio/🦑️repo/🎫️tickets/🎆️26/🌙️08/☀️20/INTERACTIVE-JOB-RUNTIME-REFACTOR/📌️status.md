# Refactor Status Dashboard

## 2026-08-22 Live Gate Checkpoint

This checkpoint supersedes the older progress table below without deleting its diagnostic history.

| Phase | Current gate state |
| --- | --- |
| 0 — Observability and freeze | Met; deny-mode audits and dependency ratchet are installed. |
| 1 — One-pool runtime | Implementation gate met; the process-wide pool, finite timer turns, admission ledger, and interactive compute job boundary are in place. |
| 1.5 — De-async repair | Open; Animate native library and `wasm32-wasip2` compilation are clean, with canonical descriptor/native runtime/strict-warning/release/`wasm32-unknown-unknown` gates still running. The framework strict-warning cohort and 164 tests pass; the exact plugin gate now reaches the distinct OS-plugin cohort. |
| 2 — Job and progress protocol | Met; bounded torture-job, cancellation, checkpoint, replay, and worker-count gates are green. |
| 3 — UI-thread isolation | Open; the static deny audit is green, but the mounted native renderer still builds product/domain frame work from `redraw`. |
| 4 — Puzzle 3D | Open; the exact unchanged 8 ms fill-build watchdog passes with retained cross-worker sessions. The full `j4` suite then exposed cross-document serialization in the first process-wide mutex design; per-app sharded sessions plus persistence/equality and two-document isolation regressions are now implemented and awaiting a stable shared-plugin rerun. |
| 5 — Frame transaction and renderer | Core transaction/layout/prepared-packet jobs are green; final mounted renderer isolation remains coupled to Phase 3. |
| 6 — FEM | Native job graph, numerical, timing, release, WASI-P2 compilation, and the official composite descriptor assembly are green. |
| 7 — WFC, Puzzle 2D, Energy | Implemented and evidenced by the three packet reports; final integration is retained in the master gate run. |
| 8 — Every tool | Production coverage/classification is complete; the exact plugin all-target warning-denial gate remains open upstream. |
| 9 — Runtime dependencies | Open at 63 Rust identities. Owned replacements include triangulation, planar booleans, codecs/compression pieces, schema/error pieces, shader-contract validation, and a bounded WASM interpreter; the owned WASM production-host/default/deletion packet is active, but the retained runtime boundary is not empty. |
| 10 — UI and tooling dependencies | Open at 122 JavaScript identities plus the native UI/render dependency cohort. The ownership-aware parity gate now covers root fallback ownership, nested technology targets, recognized config, re-exports, and template-safe lexical scanning. |

The live freeze gate is **185** third-party identities, down 53 from the 238 baseline, and rejects
additions. This is progress evidence only: zero is still the Phase 9/10 exit condition.

## ⚠️ ACTIVE COLLISION — a second large refactor is running in this tree

A peer Claude session is executing ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION` **concurrently**, with its own parallel "lanes", and it is rewriting the plugin tree wholesale. Identified from the content of its own edits (its diffs reference that ticket by name and say "this lane's agent owns `📦️glue.rs`" and "the other artifact lanes running concurrently"), not inferred from file timestamps.

Scale at the time of writing: **2,664 uncommitted files**, concentrated in `🗄️stdio` (758), `📕️norm` (345), `📸️remodel` (205), `🗒️note` (199), `🎥️shooting` (187), `📏️layout` (151), `📐️cad` (121), `🧱️block` (105).

**Consequences for this refactor, and the decisions taken:**
1. **De-async codemod application in plugin territory is STOPPED.** Packet R15 halted on its own initiative when `🗄️stdio` jumped 747→758 modified files mid-verification, none of it matching its journal's runIds. Fixing files that a competing migration is concurrently rewriting is wasted work at best and a merge hazard at worst.
2. **Error counts in plugin crates are not currently meaningful.** Five crates previously verified "cleared and holding" (`mesh-engine`, `math`, `graph`, `os-kernel`, `os-kernel-db`) reappeared with fresh *test-target* errors that are **not** attributable to this session's runIds. Treat as the peer's transient state; re-verify once their migration settles rather than chasing it.
3. **Framework-level work continues safely** — `⏳️async`, `⏱️trace`, `🧵️job`, `🎭️actor`, `🖱️ui/🖥️host` and `📺️renderer` are essentially disjoint from their plugin work, and that is where Phases 3 and 5 live.

This is the hazard the repo's own rules anticipate: never fight a peer session, never use git to revert its work, and poll rather than chase.

Baseline commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`. Host: 10 logical cores ⇒ `cpu_worker_count = 9`.

## Phase progress

| Phase | State | Ticket |
| --- | --- | --- |
| 0 — Observability & dependency freeze | **closed, gate met** | `…/PHASE-0-INTERACTIVITY-OBSERVABILITY-AND-DEPENDENCY-FREEZE` |
| 1 — One-pool worker runtime | **in progress** | `…/PHASE-1-ONE-POOL-WORKER-RUNTIME` |
| 1.5 — De-async repair (NEW, unplanned) | **in progress** | `…/PHASE-1.5-DE-ASYNC-REPAIR-SWEEP` |
| 2 — Job & progress protocol | **core landed, gate met** | `…/PHASE-2-RESUMABLE-JOB-AND-PROGRESS-PROTOCOL` |
| 3 — UI thread isolation | **in progress** | `…/PHASE-3-UI-THREAD-ISOLATION` |
| 4–10 | pending | — |

## Phase 3 progress

Landed (packets P3a, P3b):
- `RenderSnapshot`/`RenderSnapshotSink` — immutable frame publication. The first `Arc`+`AtomicPtr` design had a **real use-after-free** (SIGTRAP under concurrent load) caught by a stress test; it is now a safe `Mutex<Arc<T>>`. Do not "optimise" it back without a proof and that stress test.
- Enqueue-only `EventQueue`/`CoalesceSlot`, plus the `UiThreadToken` / `WorkerContext` capability split (9 tests).
- `OsHost::redraw` split into `build_and_publish_snapshot` / `present_snapshot`; UI and kernel threads registered with the trace crate; a `UiEvent`-stage watchdog added.
- **19 UI-thread-reachable `pollster::block_on(ParallelRuntime::…)` sites removed** — blocking bridges fell **142 → 124**. 15 in `kernel_runtime::KernelThreadState` collapsed into one legitimate entry-point bridge; 4 in `poll_pending_assets` were a genuine synchronous-`ureq`-HTTP-on-the-UI-thread bug, now on the non-blocking `spawn_app_task` path.
- `FrameBuildJob` — a real `semio_framework_job::InteractiveJob` running the World3D wheel-zoom-deadline scan on the worker pool, polled non-blockingly (`try_recv`, never `recv`). Results are re-validated against live state before use, making staleness safe **by construction**.

Two blockers to full frame-building offload, both now precisely diagnosed:
1. `AppRuntime` is `!Send` because of exactly one field — `self_weak: Weak<RefCell<AppRuntime>>`, self-referential. Every other field was checked individually; `window: Arc<Window>` is structurally `Send` but must not be called off the main thread on macOS/Web/iOS.
2. **The subtler one:** `Shell/🧊️component.rs` reads and writes ~15 `thread_local!` statics during chrome rendering. Moving that to a worker would **not fail to compile** — it would silently read and write empty per-thread state. A latent correctness landmine, not a type error.

Still on the UI thread, and named as Phase 5's actual work: chrome layout, text shaping and tessellation (`render_chrome` takes `&mut GpuContext` and touches those thread-locals), and GPU submission (fused encode+submit+present in one `ui_wgpu` method).

Verification caveat: `semio-framework-os-renderer-wgpu` **cannot currently be compiled** because the peer session's migration leaves `os-infinite` (821 errors) and `s-plugin-stdio` (4,824 on wasm32) broken upstream — zero mentions of our files in either error output. Both packets verified instead via standalone path-dependency crates that compile the code under test for real (7/7 tests each, debug+release, including a stall-resistance test proving the UI thread presents at cadence under a 300 ms builder stall). That technique is honest but is not a substitute for compiling the real crate once the tree settles.

## Phase 1 progress

Landed:
- `WorkerPool` in `🧰️framework/🔨️modules/⏳️async/🦀️component.rs` — work-stealing, DRR-fair, admission-controlled, native threads natively and a cooperative pump on wasm. `ThreadPlan`, `thread_plan()`, `ThreadBudget`, `ThreadRole` **deleted**. `PermitLedger` is release-checked and never wraps (closes the Phase 0 defect). `TimerWheel` replaces the epoch-ticker thread. `run_blocking` removed from `HostAsyncRuntime`; `block_on` behind an `entrypoint` feature. `ChannelPolicy` bounds items **and** bytes.
- Services (`🛎️services`) re-hosted: builds no `tokio::runtime::Runtime`; tokio features narrowed to `sync`+`macros`.
- Actor/shards: per-shard `ShardExecutor` OS threads and `semio-os-host-kernel-shard-forward-*` forwarder threads **deleted**; shard turns are pool submissions with affinity as a scheduling constraint. Actor DRR/mailbox/budget/failure-ladder semantics preserved (76/76 actor tests pass).
- Ad-hoc threads: db storage blocking bridge, db engine per-submit thread, store-sync per-document thread + embedded runtime, pack HTTP per-request fetch thread, hub `HubDbRuntime` — all eliminated or bounded.
- Epoch ticker and process-transport heartbeat moved to `Lane::Timer`. Two blocking-pipe I/O threads remain in process-transport, bounded per child process and registered as `ThreadRole::IoBoundary` so the census can see them.
- Renderer re-hosted onto one injected `WorkerPool`; `ParallelRuntime` no longer owns threads; frame callback wrapped in the trace watchdog.

Verified by the coordinator (not merely reported): the five core Phase 1 crates build clean; production shard and forwarder threads are gone; `ThreadPlan`/`ThreadBudget` survive only in doc prose.

Open items before the Phase 1 gate can close:
- ~~`ManualRuntime::drive()` polls with a no-op waker and cannot observe completions from the real cross-thread pool.~~ **Closed 2026-08-21:** task-specific retained wakers plus an epoch/condition-variable wake path now observe real cross-thread completion; plugin-host is 141/0/1 in both debug and release.
- ~~`WorkerPool::shutdown()` deadlock flagged by P1f.~~ **Closed 2026-08-21:** shutdown drains the timer wheel before joining; focused in-flight-timer shutdown coverage passes in debug and release.
- Low-priority admission's check-then-increment race is closed with an atomic RAII reservation held for the whole job. Async is 43/43 and services is 30/30 in both debug and release; async clippy and both wasm targets are clean. Services wasm remains blocked by 15 existing actor-glue errors.
- `ComputePool::run_blocking` retains an opaque `FnOnce` signature (now a pool submission rather than a thread). The governing rule wants opaque blocking closures gone from interactive paths — carry into Phase 3 with the capability tokens.
- P1h removed four of the six residual production sites: pack retry sleep now uses the shared `TimerWheel`; store-sync uses bounded `WorkerPool` actor turns; DB artifact authority and the feature-gated DB actor use injected bounded pool turns. Their owned production census is zero, and every `ArtifactHost::new` callsite now injects a shared pool. Debug/release pack and OS-kernel checks pass natively; OS-kernel sync passes debug/release on `wasm32-unknown-unknown`. The literal "UI thread + pool workers only" gate is still **not met** because Shell identity bootstrap and Shell directory streaming remain, alongside the separately classified renderer-kernel and registered process-I/O boundaries. Exact architecture, commands, current de-async blockers, and census: `PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1h-residual-threads.md`.
- P1i removed the two Shell residuals: identity bootstrap is a deadline/cancellation-aware retained-waker `Lane::Io` future, and directory delivery is a Send-capable bounded state machine driven by finite pool turns and `TimerWheel` wakeups. Shell identity/directory production contains zero spawn, blocking receive, or local executor sites; native OS-kernel and browser-wasm kernel library checks pass. Full renderer/test execution remains masked by unrelated Phase 1.5 syntax/stale-await failures. The strongest repository-wide literal gate still sees separately owned renderer-kernel and procedural-WFC CPU threads plus classified process/CLI I/O boundaries. Exact evidence: `PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1i-shell-threads.md`.

## CRITICAL FINDING — the repo did not build at baseline

A full `cargo check --workspace --all-targets` reports ~698 errors. Attribution was verified rather than assumed: `semio-framework-ui` (557), `semio-framework-2d` (28), `semio-framework-graph` (8), `semio-framework-machine-derive` (4) and `semio-s-plugin-draw-fsm` (5) are **unmodified in the working tree**, and there are **zero commits since baseline**, so their content *is* baseline content. This breakage pre-dates the refactor.

The error signatures are one bug class:
```
no method named `X` found for opaque type `impl Future<Output = …>`
cannot apply unary operator `!` to type `impl Future<Output = bool>`
the `?` operator can only be applied to values that implement `Try`
`impl Future<Output = Vec<…>>` is not an iterator
cannot subtract `impl Future<Output = i32>` from `impl Future<Output = i32>`
```
That is **`async fn` called without `.await`**, at scale — the mechanical fallout of the repo-wide async convention (`AGENTS.md:44`: "You SHOULD implement everything async when it makes sense"). Phase 0's census measured the same phenomenon from the other side: 88.28% of 53,338 `async fn` are effectively non-suspending.

Already fixed as an unblocker: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs` had an illegal `async fn main()` plus an `async fn out_dir()` called without `.await` — it had never compiled.

Note on scoping: `semio-framework-ui` alone builds clean (`cargo check -p semio-framework-ui --all-targets` → 0 errors). The 557 errors appear only in a full-workspace build, i.e. under cargo's cross-member **feature unification**, and the failing set includes `semio-compose-rs` which is explicitly out of scope. The true in-scope breakage must therefore be measured with compose excluded before sizing the repair.

### Consequence for the plan

De-asyncing is promoted from a Phase 6/7 clean-up to **Phase 1.5, a prerequisite**. Phases 3 and 5 rewrite the UI runtime and the frame transaction; they cannot land on crates that do not compile under the workspace feature set. The repair is driven by Phase 0's `🔧️async-census.json` (one record per function, with body extents) and must use **compiler-verified, span-keyed edits — never name-keyed**, which silently hit unrelated production code.

The refactor narrows `AGENTS.md:44` to *genuine suspension only*. `AGENTS.md` must not be edited, so the narrowing is recorded here.

## Phase 1.5 wave 1 results

| Crate | Before | After | Note |
| --- | ---: | ---: | --- |
| `semio-framework-ui` | 557 | **0** | 400 `async fn` → `fn`, **zero** `.await` added. Repro: `cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib` |
| `semio-framework-2d` | 28 | **0** | marching-squares, Douglas–Peucker, planar booleans de-asynced |
| `semio-framework-graph` | 8 | **0** | `DslField::to_value`/`from_value` were async against a sync trait; 4 stray `.await` on sync calls removed |
| `semio-framework-machine-derive` | 4 | **0** | proc-macro entry points were `async` — impossible, since a proc macro runs inside `rustc` with no executor. 4 entry points + 47 helpers de-asynced |
| `semio-s-plugin-draw-fsm` | 5 | **0** | 4 missing `.await` on `restore(...)`, 1 missing generic argument |
| `semio-framework-ui-backend-vulkan` | 1 | **0** | not a defect — the crate's intentional non-Linux `compile_error!` gate |
| `semio-hub` | 70 | 18 | 70/70 attributed to pre-existing breakage (0 from the Phase 1 API change, 0 from P1d's edit); remaining 18 are a cross-boundary blocker |

Fixing the derive macro **cascaded**: it had been aborting the build before `rustc` reached several crates, so `framework-ui`, `framework-2d` and `framework-graph` were partly cleared by it too.

### Blockers discovered by wave 1

1. **`semio-framework-machine` (the runtime crate, not `-derive`): 351 errors**, same bug class, previously invisible because the macro crate's failure aborted the build first.
2. **`db` futures are not `Send`**: a `std::sync::MutexGuard` taken from an `if let`/`if` scrutinee is held across an `.await` (edition-2021 temporary-scope rules) in `⚙️engine/🦀️component.rs:885-896,906-907` and `🔒️security/🦀️component.rs:620`. Axum's `on_upgrade` and six REST handlers require `Send` futures, so this blocks hub's last 18 errors. No in-boundary workaround exists.
3. **UI render backend churn**: `🖱️ui/🖼️render/🎯️targets/{webgpu,d3d12}` moved 476→0 and 0→332 without corresponding edits from the packet that observed it. Needs attribution.
4. `semio-framework-ui-backend-d3d12` has an incomplete platform gate (ungated `windows::` import).
5. `semio-framework-ui`'s unittest target has a pre-existing `Label: From<&str>` compile-time-gate violation from an unrelated ticket — not the async bug class.

## Phase 1.5 waves 2–3 results

| Crate | Before | After | Note |
| --- | ---: | ---: | --- |
| `semio-framework-machine` (lib) | 90 | **0** | 160 `async fn` → `fn`, 0 `.await` added, 3 stale `.await` removed |
| `semio-framework-machine` (lib test) | 318 | 3 | remaining 3 need the derive crate updated in lockstep |
| `semio-framework-os-kernel` (lib test) | 16 | **0** | 12 stale `.await` in `🏪️store/🔄️sync` test fixtures on callees already sync in production |
| `semio-framework-ui` (lib test) | 84 | **0** | not the async class at all: 39 `Label: From<&str>` gate violations + 45 `String`/`Label` mismatches, all in fixtures. The deliberate gate itself was left untouched |
| `semio-hub` | 18 | **0** | unblocked by the db `Send` fix |
| `semio-framework-os-kernel-db` | — | **0** | 478/478 tests green across sqlite/postgres/neo4j |
| `semio-framework-ui-backend-webgpu` | "452" | **0** | phantom — wasm32-only crate checked natively; 0 errors on `wasm32-unknown-unknown`. Gated properly |
| `semio-framework-ui-backend-d3d12` | "332" | **0** | phantom — all cascading `E0433` from one ungated `windows::` import |
| `semio-framework-ui-backend-vulkan` / `-metal` | 1 / — | **0** | `compile_error!` gates replaced with cfg-gated empty libs |

### Two measurement traps, now recorded as method

1. **Platform-gated crates must be measured on their own target.** "452" and "332" were pure artifacts of checking a wasm32-only and a Windows-only crate on macOS; zero of those errors mentioned `Future` or `await`. Always confirm an error actually names `Future`/`await` before attributing it to the async bug class.
2. **`compile_error!` platform gates make a clean workspace build impossible forever**, because `cargo check --workspace` visits every member directly regardless of dependency edges. All four render backends now use cfg-gated empty libs instead, with "wrong platform is an error" preserved one layer up at the consumer's target-gated dependency edge.

### Remaining after wave 3

- `semio-framework-machine` (lib test) — 3 errors. `StatechartEvent::event_id`/`event_name` and `Machine::definition` went sync, so the derive crate's `emit()` / `expand_statechart_event()` must drop `async` from 5 generated methods. Separately, `export_wasm_machine!` has pre-existing bugs (stale `.await`, `restore::<M>` missing one of two generics, a missing `wasm_bindgen_futures` dependency) newly visible from the first-ever wasm32 check of that crate. Blast radius is zero external crates.
- `semio-framework-hash` (lib test) — newly surfaced, unmeasured.

### Phase 2 input from R8

`semio-framework-machine`'s `PersistedSnapshot`/`step()` round-trip, its `Command` effect/kernel split, and its `Inspector` event stream are a genuinely strong foundation for `InteractiveJob`. But `run_to_completion` drains to full quiescence bounded by a *count*, not a deadline, with no mid-macrostep yield/resume, no preview channel and no fault channel. Recommendation carried into Phase 2: make the microstep loop budget-aware and resumable (its pending-trigger queue is already a reified value) rather than adopting this kernel as a ready-made universal `InteractiveJob`.

## The reachability-masking iceberg — and the decision it forces

Phase 1.5 was repaired crate by crate, and every fix **unmasked more damage**, because a crate that fails to compile prevents `rustc` from ever reaching its dependents. The sequence actually observed:

1. The proc-macro crate's illegal `async` entry points aborted the build → hid `semio-framework-machine` (408 errors).
2. Fixing `semio-framework-machine` → unmasked `semio-framework-hash` (29).
3. Fixing `semio-framework-hash` → unmasked `semio-framework-os-infinite` (927 lib + 1228 test), `semio-s-plugin-stdio` (**5,545**), `semio-framework-os-mcp` (22), `semio-s-imperative` (6).

None of these later crates reference the earlier ones' symbols; they were simply never reached. **`cargo check --workspace` without `--keep-going` stops early and cannot show the true picture** — that is the methodological error that made this look like a small cleanup. All future measurement must use `--keep-going`.

### Consequence: hand-repair does not scale, build the codemod

Repairing ~8,000+ errors by hand, crate by crate, is the wrong instrument. Phase 0 already built the analysis half (`🔧️async-census.ts`, `🔧️async-census.json`, 30/30 self-test). The repair half should be **compiler-driven**:

- Run `cargo check --workspace --all-targets --keep-going --message-format=json`.
- Consume structured diagnostics (each carries exact spans, and often a machine-applicable suggestion).
- Apply span-keyed edits for the known bug-class shapes: `async functions cannot be used for tests` → drop `async`; anything shaped `impl Future<…>` where a value was expected → either drop the stale `.await` or de-async the callee, preferring **de-async when the callee has no genuine suspension point**.
- Re-check and iterate to a fixpoint, requiring the error count to fall monotonically.

This is exactly the "compiler-verified, span-keyed, never name-keyed" discipline the phase has been enforcing manually, and R12's cascade gave the empirical shape to automate: of ~86 raw grep-matched call sites, only ~14 were real edits, because most call sites sit inside functions that retain other genuine awaits. A name-keyed replace would have corrupted the other ~72 — and R12 confirmed two *unrelated* same-named `hash_bytes` functions (a render fxhash wrapper and a db/state blake3 wrapper) that a blind textual replace would have silently broken.

## The codemod (R13) — built, and it works

Tool: `🔧️r13-deasync-codemod.ts` (68 KB, TypeScript/bun, temporary, in the Phase 1.5 ticket folder). Journal: `📝️r13-journal.jsonl` (3.6 MB, every edit reversible). Compiler-driven: consumes `cargo check --message-format=json`, applies **span-keyed** edits ranked by rustc's own machine-applicable suggestions, iterates to a fixpoint with a **monotonic guard** that reverts and stops if the error count rises.

| Crate | Before | After |
| --- | ---: | ---: |
| `semio-s-plugin-stdio` | 16,725 | 5 (wave 1: 8,327 edits) |
| `semio-framework-os-infinite` | 2,155 | 2,029 |
| `semio-framework-os-kernel` | 145 | **0** |
| `semio-framework-plugin` | 125 | 114 |
| `semio-s-imperative` | 12 | 4 |
| `semio-framework-mesh-engine` | 11 | **0** |
| `semio-framework-math` | 9 | **0** |
| `semio-framework-geometry` | 8 | **0** |

The guard earned its keep twice: it caught a 2,029→2,033 regression and a 5→13,272 jump, reverting both. The second was then *diagnosed rather than retried* — it was a real reachability cascade (fixing `mesh-engine` and `math` unmasked stdio's own calls into now-sync functions), confirmed by reproducing the exact figure deliberately.

### One real corruption, caught and closed

On the first large run, `findFutureExprSpan()` attributed rustc's "found future" label to a *pattern* sub-span in a `let-else` tuple destructure, and since `Some(x)` is lexically indistinguishable from a call, the tool inserted `.await` after a pattern: `let (Some(src_ep).await, …) = …`. Caught by running the real compiler afterwards (a parse error), contained to exactly one line (verified by targeted grep), hand-repaired, then root-caused with a `PATTERN_CONSTRUCTOR_DENYLIST` (`Some`/`None`/`Ok`/`Err`) and a regression self-test. This is precisely why the packet mandated running the compiler after every crate rather than trusting the tool's self-report.

### Second bug class: stacked `.await`s — found, fixed, and honestly unwound

On the larger runs the tool's `findFutureExprSpan()` fallback picked the **method-call** span (`.into_iter()`, ~9 bytes) instead of the Future-producing receiver, which sits in a *separate, unlabeled, non-primary* span whose `byte_end` exactly abuts the primary span's `byte_start` (confirmed byte-for-byte on `semio-framework-surface`: `[16072,16148)` then `[16148,16157)`). So `.await` landed as `X.into_iter().await` instead of `X.await.into_iter()` — which does not fix the type error, so each later run re-diagnosed it and stacked another `.await`. Accumulated **up to 6 deep, across 116 sites in 16 files**, all attributable to this tool's own repeated runs (the stacking pattern matches its journal, not the peer session).

Why the earlier spot-checks missed it: the first validation of the `is not an iterator` shape went through the **de-async** branch (the callee resolved and was de-asynced), so the await-**insertion** path for that shape was never exercised at small scale.

Fixes: (1) span selection now prefers a non-primary span byte-adjacent to the primary one — a structural signal, not a guess; (2) an unconditional `wouldStackAwait()` backstop refuses any insertion where `.await` is already adjacent, so even an undiscovered future span bug surfaces as residue rather than corruption; (3) reproducing self-tests for both. Then a journaled, revertible `strip-stacked-awaits` command removed all stacked awaits at all 116 sites back to zero — verified by `grep -rn "\.await\.await"` returning nothing and no parse-level errors remaining.

Crucially, this **raises** the reported error counts, because the stripped sites return to their original honestly-broken state. The tool refused to report a corrupted-but-lower number. That is the right call, and the numbers below are the honest ones.

### Final Phase 1.5 numbers (codemod wound down at a clean checkpoint)

**At zero, with tests passing:** `os-kernel` (779 tests), `os-kernel-db` (424), `geometry` (57), `graph` (174), `mesh-engine` (20), `math` (191). Confirmed still zero with no regressions: `framework-ui`, `framework-machine`, `framework-2d`, `hub`, `hash`.

**Remaining in scope:** `semio-s-plugin-stdio` 10,450 · `semio-framework-os-infinite` 1,189 (from 2,155) · `semio-framework-surface` 870 (newly unmasked) · `semio-framework-plugin` 146 · `semio-s-imperative` 4 (E0733 recursive-async, documented refusal). Out of scope: `os-mcp` 24 (unrelated missing-API breakage, zero async-class), `semio-compose-rs`.

**Def-use extension: explicitly refused, and correctly so.** Four real bugs surfaced this session in the *narrower* span-local design — one of which corrupted 116 sites across 16 files. A def-use pass has the same "plausible but silently wrong" failure mode and needs its own dedicated, self-tested packet rather than a same-session extension. A documented refusal beats a third corruption.

**Concurrency:** one directly observed interaction with the peer Claude session in `manifest/component.rs`, on non-colliding byte spans, handled safely by design. Dependency ratchet re-verified at 238. No file left mid-edit; every edit journalled.

### Honest state after the strip

| Crate | Errors (lib / lib test) |
| --- | --- |
| `semio-s-plugin-stdio` | 4,827 / 9,537 |
| `semio-framework-os-infinite` | 869 / 1,152 |
| `semio-framework-plugin` | — / 146 |
| `semio-framework-os-mcp` | 22 / 22 (unrelated: `E0425`/`E0405` against plugin-host, not the async class) |
| `semio-framework-os-kernel` | — / 9 |
| `semio-s-imperative` | 2 / 2 (recursive-async residue) |
| 3 × `semio-s-plugin-cad-*` | — / 4 each |
| `semio-compose-rs` | out of scope |

Cleared during the sweep and holding: `semio-framework-graph`, `semio-framework-os-kernel-db`, `semio-framework-geometry`, `semio-framework-mesh-engine`, `semio-framework-math`, `semio-framework-ui`, `semio-framework-machine`, `semio-framework-2d`, `semio-hub`, `semio-framework-hash`.

### The residue is one shape, and it is the real remaining problem

~1,180 diagnostics in `os-infinite` and 67 in `framework-plugin` share a single shape: the Future-typed expression is a **bare `let`-bound local variable**, assigned from a call earlier and used later — not a fresh call expression. The tool deliberately refuses these (no `(` follows the identifier), because guessing is what produced the one corruption. Fixing them needs **def-use / data-flow tracing back to the assignment site**, which is a genuinely harder tool than the span-local one built here. That is the top work-list item.

Also residue: mutually-recursive `async fn` pairs (E0733) in `semio-s-imperative` where cycle members await non-cycle callees of unverified suspension status — needs either extending the cycle or a human `Box::pin` decision.

## Phase 2 — the job protocol has landed and its gate is met

New crate `semio-framework-job` at `🧰️framework/🔨️modules/🧵️job/`, built on the `⏱️trace` module's structural precedent. Public shape:

- `InteractiveJob::step(&mut self, cx: &mut StepContext) -> StepOutcome` — **synchronous and explicitly resumable, deliberately not `async`**, with `StepOutcome::{Yield, PreviewReady, CheckpointReady, Complete, Cancelled, Fault}`. Two-bound budgeting: fuel counter plus absolute wall-clock deadline.
- `Operation`/`RevisionId`/`validate_commit` — operation id, base document revision, input generation, preview sequence, deterministic seed. A commit is accepted only when base revision and generation still match.
- `JobScope`/`ChildJobGuard` — structured child jobs built on the async crate's `CancelToken` parent-chain, no separate registry.
- Ten-event `ProgressEvent` vocabulary with `channel_policy_for`/`default_channel_kind_for` implementing the policy matrix (latest-wins, coalesced, lossless bounded, ring, lossy, byte-credit).
- `drive_step` — the single point where a `StepOutcome` becomes a `semio-framework-trace` `record_*`/`Watchdog` call, so instrumentation is not duplicated.
- `run_to_completion`/`run_on_worker` — the batch adapter, so CLI and headless paths drive the *same* job implementation as the interactive path and cannot diverge.
- `TortureJob` — the conformance job.

**Exit gate verified by the coordinator, not just reported** — `cargo test -p semio-framework-job` 16/16 in debug and release, including all six torture tests: never trips the 8 ms watchdog ceiling; previews continuously; deterministic for identical seed and inputs; observes cancellation within 8 ms at p99 (measured over 40 trials); checkpoint → restore → resume byte-equal to an uninterrupted run; and replays deterministically across **actual** `WorkerPool` worker counts 1/2/4 rather than assuming. Clean on `wasm32-unknown-unknown` and `wasm32-wasip2`. Dependency ratchet unchanged at 238.

A real bug was caught building it: the initial RNG seeding used `seed | 1`, which collapsed adjacent seeds (42 and 43) onto identical state — replaced with a splitmix64 mixing step. Exactly the class of defect the determinism requirement exists to catch.

## Interactivity audit, re-run after Phase 1 (was 180 findings at Phase 0)

Now **198 findings**: 142 non-allowlisted blocking bridges (was 121), 36 sync-filesystem, 8 thread-pool (was 10), 6 sync-clipboard, 6 sync-process.

**Good:** the audit's own staleness detector independently confirms Phase 1's deletion — the `🎠️activation.rs` `block_on` allowlist entry is reported as "no longer matches any finding". That is machine-verified proof the shard-forwarder poll loop is gone, not an agent's self-report.

**Bad, and partly self-inflicted:** blocking bridges rose 121 → 142. A known contributor is packet P1e, which wrapped **17 `ParallelRuntime` call sites in `pollster::block_on`** because the type's methods became async. That runs directly against the rule that `block_on` is confined to approved process and test entry points. Phase 3 must remove these rather than let them settle — flagged here so the increase is not mistaken for drift.

**Remaining literal thread creation outside the pool after P1h:** the pack HTTP retry site is gone. Production Semio-owned sites still include renderer Shell identity bootstrap and directory streaming plus repo CLI; asyncprobe fixture spawns are test fixtures. Separately, the renderer kernel thread and plugin process-transport reader threads remain registered platform/process boundaries. P1h's four owned source files contain zero production spawns; see `PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1h-residual-threads.md` for the exact census and classifications.

### Measurement caveat

Workspace-wide error totals are **non-deterministic run to run** (observed 1058 → 569 → 957 → 1145 with no edits in between) because failures cascade through reachability: fixing one crate unmasks others. Per-crate `cargo check -p <crate>` is the only reliable measure, and progress must be tracked per crate, never by workspace total.
