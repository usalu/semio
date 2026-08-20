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

### The residue is one shape, and it is the real remaining problem

~1,180 diagnostics in `os-infinite` and 67 in `framework-plugin` share a single shape: the Future-typed expression is a **bare `let`-bound local variable**, assigned from a call earlier and used later — not a fresh call expression. The tool deliberately refuses these (no `(` follows the identifier), because guessing is what produced the one corruption. Fixing them needs **def-use / data-flow tracing back to the assignment site**, which is a genuinely harder tool than the span-local one built here. That is the top work-list item.

Also residue: mutually-recursive `async fn` pairs (E0733) in `semio-s-imperative` where cycle members await non-cycle callees of unverified suspension status — needs either extending the cycle or a human `Box::pin` decision.

### Measurement caveat

Workspace-wide error totals are **non-deterministic run to run** (observed 1058 → 569 → 957 → 1145 with no edits in between) because failures cascade through reachability: fixing one crate unmasks others. Per-crate `cargo check -p <crate>` is the only reliable measure, and progress must be tracked per crate, never by workspace total.
