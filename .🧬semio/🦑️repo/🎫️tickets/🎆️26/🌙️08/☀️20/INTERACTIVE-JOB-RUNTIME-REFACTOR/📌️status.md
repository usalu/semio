# Refactor Status Dashboard

Baseline commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`. Host: 10 logical cores ⇒ `cpu_worker_count = 9`.

## Phase progress

| Phase | State | Ticket |
| --- | --- | --- |
| 0 — Observability & dependency freeze | **closed, gate met** | `…/PHASE-0-INTERACTIVITY-OBSERVABILITY-AND-DEPENDENCY-FREEZE` |
| 1 — One-pool worker runtime | **in progress** | `…/PHASE-1-ONE-POOL-WORKER-RUNTIME` |
| 1.5 — De-async repair (NEW, unplanned) | **in progress** | `…/PHASE-1.5-DE-ASYNC-REPAIR-SWEEP` |
| 2 — Job & progress protocol | pending | — |
| 3–10 | pending | — |

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
- `ManualRuntime::drive()` polls with a no-op waker and cannot observe completions from the real cross-thread pool — 2 debug-only test failures in the plugin host, passing in release. Spans the async and services crates.
- `WorkerPool::shutdown()` deadlock flagged by P1f, unfixed.
- `ComputePool::run_blocking` retains an opaque `FnOnce` signature (now a pool submission rather than a thread). The governing rule wants opaque blocking closures gone from interactive paths — carry into Phase 3 with the capability tokens.
- Thread census re-run to prove "UI thread + pool workers only".

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

### Measurement caveat

Workspace-wide error totals are **non-deterministic run to run** (observed 1058 → 569 → 957 → 1145 with no edits in between) because failures cascade through reachability: fixing one crate unmasks others. Per-crate `cargo check -p <crate>` is the only reliable measure, and progress must be tracked per crate, never by workspace total.
