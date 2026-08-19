# Get `s` Working Again — Universal-Async, De-Dyn, Microkernel Runtime, All Plugins, End to End

## Context

`s` (the semio app: 33 plugins + 26 extension crates, 58 playground variants, react + wgpu-web + wgpu-native renderers) does not currently compile or run. The MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME ticket (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/`) replaced the legacy runtime (one worker/Engine per plugin, sync `exchange()` ABI) with a pooled-actor microkernel (`poll(events,budget)→TurnResult`, effects-only ABI, jobs, checkpoint, static descriptors, capability broker, shard pools, wasmtime 47.0.3 with proven component-model-async). Then the owner decreed **universal async** ("every single function must have async keyword and be implemented with async" + "use WIT 0.3 async"), two codemods ran (fleet: committed; framework: staged), and the tree stopped compiling on **E0038: `async fn` in traits is not dyn-compatible** — ~88 async methods sit behind first-party trait objects.

Owner decisions locked for this plan (2026-08-19):
1. **Drop dyn dispatch.** No boxed-future trait methods as the end state. Redesign all first-party dyn-dispatched registries/consumers to enum/static/generated dispatch so plain AFIT (`async fn` in trait) works everywhere. Language-fixed exceptions stay sync (external-trait impls, `const fn`, `extern "C"`, `fn main` — ~1.9%).
2. **Single coordinator.** No other coordinator session is live. The new Opus coordinator in the main chat takes over the whole program, absorbs all in-flight packet state from `📓️status.md` + the working tree, and is the only registrar.
3. **Legacy compose excluded**: ignore the root `compose/` tree entirely. (The new `semio.compose` cold-job path inside the framework is IN scope.)
4. **Sync external deps: literal reimplementation** where no async version exists; async-native replacement where one exists — always behind a first-party interface.

Workforce (user-mandated): plan = Fable 5 (this file) · coordination = main chat **Opus 5 High ("sol")** · execution = parallel **Sonnet 5 High ("terra")** · read-only exploration = parallel **Haiku 4.5 ("luna")**. Same live tree, no branches/worktrees, auto-commit bot runs. All binding rules in the ticket's `📌️important.md` remain in force (registrar files, lease-requests, scratchpad `CARGO_TARGET_DIR`, `--lib` AND `--all-targets`, coordinator owns acceptance builds, atomic packets never interrupted, baselines are named sets, etc.).

## Verified current state (measured 2026-08-19, not from stale logs)

- **Fleet asyncify is COMMITTED** (`09c3cf6df6`, 9,291 files): 56,680 `fn`→`async fn` under `✏️s/🔌️plugins`. The codemod was blind: it also asyncified external-trait impls (548 `Default`, 600 serde, 53 `From`, 31 `fmt`), 11,553 `#[test] async fn`, and dropped 19 `const` + 2 `extern` qualifiers.
- **Framework asyncify is STAGED, uncommitted** (388 files: 314 `🧰️framework`, 73 `✏️s`, the WIT): trait-aware (external impls skipped) but converted trait decls to AFIT — including `GuestRuntime`, `HostAsyncRuntime`, `SpaceMember`, `PluginApp` etc. that are consumed as `Arc<dyn>/Box<dyn>` — and double-futured the boxed prior art (`async fn … -> DbFuture/HostFuture/ComposeFuture`). 4,704 more `#[test] async fn`. **Keep it — the direction is now correct under decision 1; the dyn consumers are what changes.**
- **WIT**: all 37 funcs are now `async func` (staged). Both worlds (`actor`, `actor-async`) still exist though the sync/async distinction that justified two worlds is gone. S7 (categorical, reproduced): sync `func` exports are uncallable on an async-configured Store.
- **Dyn wall** (measured): `PluginApp` 49 async methods / 26 dyn uses, `SpaceMember` 25/16, `GuestRuntime` 9/15, `HostAsyncRuntime` 3/10, `Backbone`+`BackbonePort` 5/3, plus per-plugin registries (`.editor::<>()`, `.viewer::<>()`, codecs, composers) and fn-pointer tables (`AsyncComposeFn`, 163 `ComposerEntry` sites, surface factories `fn(&AppDefinition) -> Box<dyn PluginApp>`).
- **Native runtime**: shared-Engine one-Store-per-actor done; `⏳️imports.rs` (24 async host imports) mounted; **`⏳️runtime.rs` (the real async actor runtime, 36.8 KB) is unmounted, was never compiled, needs tokio, and was written against WIT interfaces that don't exist** (schema went a different way). Poll-world backend (`ShardLoop`/`poll_ready`/`ParallelRuntime`) still what wgpu-native wires. `🏃️run` bypasses the microkernel. Winit-thread `pollster::block_on` sites remain (glue.rs:1896–2580, Shell:3299, ProgramBridge:523) + direct `ureq::get` (glue.rs:1577).
- **Web**: react path IS wired to the new shard runtime end-to-end (ShellHost → PluginRuntime → ActivationRegistry + ShardClient → `_shard/🟨️shard-worker.js` → `createActorApi()`), but the 48 materialized bridges on disk are stale pre-rewrite output (`runSerialized`, no `createActorApi`) — needs a real fleet wasm rebuild. wgpu-web is fully OLD path (`PluginWorkerClient` at `🧊️wgpu/🟦️typescript/🟦️boot.ts:49` — one of two banned copies still live) and `🧊️wgpu/📦️index.ts:6` imports deleted symbols. `⚙️vite.config.ts:63` omits `_shard` for single-variant prod builds. TS `exchange` seam (`PluginWasmHandle.exchange`, `AppChannelHandle`) still exists.
- **Describe pipeline broken by design change**: `📇️describe/📦️glue.rs:122` builds a non-async Store while `describe` is now `async func`. Descriptors 26/33 emitted, 13 ratcheted; 7 missing (demonstrator, fem, playbook, trinity, stdio, puzzle, block) with classified causes and a proven migrate-to-`.declare_artifact` recipe (trinity done).
- **Extensions**: `ActorKind::Extension` + `.sxt` package format exist; **no host activation path exists at all**; no linked-extension gate. Scale fixture (50×50, 2,550 records, 7 profiles) exists; bench budget 3 ran native only (and shard execution parallelism wasn't truly parallel).
- **Census** (the user's headline metrics, re-measured): `block_on` 134 (flow 59, cad 45, stdio 15, process 13, animate 2) · `pending_effects` 3 · `register_job_kind` 0 · `AsyncTask` 0 · `DownloadMediaExport` payload builds 41 · fleet adoption of the new async mechanisms is literally zero.
- **Banned-symbol residue**: `PluginWorkerClient` (wgpu-web copy), TS `exchange` members, stale generated bridges. Everything else on the must-not-exist list is already tombstones only.
- Bench: 7 of 8 budgets green natively; budget 5 (interactive p95 ≤ 8 ms native under 40 cpu actors) genuinely fails (140.9 ms) — a design result the async runtime rework must address, not an instrument bug.
- ~16,257 `#[test] async fn` (fleet 11,553 + framework 4,704) don't compile — the single largest mechanical repair.

## Design B — one async world, runtime end to end

(Abbreviations: `PLUGIN/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.)

### B0. Three spikes, FIRST, in parallel
- **SP-W1 jco async-export spike** (biggest external risk). Facts: repo has jco 1.27.0 (`node_modules/@bytecodealliance/jco`); jco ≥1.20 passes upstream WASI-P3 tests and transpiles P3 components (async funcs/streams) to ES modules; a wit-bindgen-async guest uses the P3 **callback ABI** (event-loop driven, should NOT need JSPI); `--async-mode jspi` is the legacy lift for sync-ABI exports; JSPI is default-on Chrome, flagged Firefox. Spike fixture (e.g. `💻️os/🧫️fixtures/🔌️jcoprobe/`): minimal wasip2 component against the reduced world (async `reactor.poll`, `pure` + `host-async` incl. one `stream<u8>`), transpile with repo jco, drive from a Web Worker in bun + Chrome + Firefox (JSPI off). Success criteria: S1 poll callable returning promise of turn-result; S2 guest awaits an import resolved 50 ms later WITHOUT blocking the worker loop; S3 a detached guest task completes after poll resolves (if not: detached awaits complete in a later poll turn — design note); S4 stream<u8> readable chunk-by-chunk; S5 all of it with JSPI unavailable. Verdicts: GO-callback → plain jco P3 transpile everywhere. GO-jspi → Chrome-first + fallback F2 (hand-rolled callback-ABI driver in the bridge generator `🌐plugin-web-materialize.ts`). NO-GO → escalate; temporary fallback F3 = second sync-lifted build for web only, flagged temporary.
- **SP-N1 native async-runtime harness**: rewrite the harness subset of `⏳️runtime.rs` against the REAL schema in the async-probe fixture; re-prove (a) cancellation = drop Store inside the task, (b) epoch preemption ratio 1.00 across two async Stores (S1b/S1c re-run on the async engine), (c) poll while a spawned guest subtask holds a pending host import.
- **SP-G1 guest toolchain + brep-await probe**: build 🗒️note as wasip2 with `wit_bindgen::generate!({async: true})` against the collapsed world, validate + instantiate on an async store (gates the fleet rebuild). Run the never-executed probe from `📓️luna-brep-await-spec.md` (guest LocalExecutor awaiting a BrepKernel async fn inside `jobs.step-job`); GO unlocks the brep/block_on sweep.

### B1. Collapse to ONE async world
**Decision: one world named `actor`, TURN-SHAPED; `interface runner` and `world actor-async` die; `reactor.poll` is the sole turn entry.**
```wit
world actor { import pure; import host-async; export reactor; export jobs; export checkpoint; export describe; }
```
Rationale: everything scheduler-side is turn-shaped (DRR grants, Budget/TurnResult, FailurePolicy, ShardFrame wire, shard worker, MockGuestRuntime, benches); `runner::run(stream<event>)` needed GrantWindow/GrantedEventProducer/synthesize_turn_result purely to fake turns back; async poll returning a promise is the easy jco case and the wired react path survives as-is. Async ergonomics come from awaitable `host-async` imports inside poll/jobs (exports may suspend on imports); long awaits span turns via guest LocalExecutor + `next-wake`.
Edits in `PLUGIN/🧬️schema/📜️component.wit`: delete `interface runner` (:961) + `world actor-async` (:1044); add `import host-async` to `world actor` (:1029); rewrite stale doc comments (:1022,:1037,:1041); re-specify `emit`/`emit-patch` intent (fire-and-forget door for between-poll background tasks; `turn-result.effects` stays canonical for turn-atomic batches).
Schema-parity re-spec (`🖥️host/🧪️schema-parity/🦀️component.rs:236,:101`): replace the "emit is NOT async" assertion with (a) every func in the file is `async func` (S7 stated as intent), (b) emit's param is the whole effect variant, (c) exactly one world exists importing pure + host-async.
**Delete list** (census-enforced at exit): WIT `runner`/`actor-async`; native `ShardLoop`/`ShardExecutor` (`🖥️host/🧵️shard/`), `poll_ready` (:550), `ParallelRuntime` (`🎯️targets/🧊️wgpu/🎠️runtime.rs`), sync turn/job/checkpoint bodies of `WasmtimeRuntime` (its compile/cache/limiter plumbing is reused); web second `PluginWorkerClient` copy, TS `exchange`. **Survives**: MockGuestRuntime (turn-shaped, genuinely awaited), `build_shared_engine` + EpochTicker + BudgetLimiter + compiled cache, ShardClient/ShardFrame, the events/effects/jobs/checkpoint interfaces.

### B2. Native: rewrite + mount `⏳️runtime.rs` as the sole backend (`AsyncPluginRuntime`)
Keep from the draft: `build_async_engine` (wasm_component_model_async + concurrency), `AsyncEngineHandle`, `DeadlineCell`/`install_epoch_budget`, `AsyncActorTask` command-channel skeleton with the proven Store-inside-task-body rule, per-export `AccessorTask` pattern. Rewrite: bindgen against the collapsed `world actor` (real names — the draft's `jobs-async`/`checkpoint-async` predictions are dead); reuse `⏳️imports.rs` (`AsyncActorHostState` + 24 host-async impls) verbatim as the import half; delete GrantWindow/GrantedEventProducer/synthesize_turn_result; task body = command channel `{Poll, StartJob, StepJob, CancelJob, Checkpoint, Restore, Shutdown}` with oneshot replies, `run_concurrent` driving background guest tasks, DeadlineCell-armed epoch + `set_fuel` per command, Shutdown/channel-close ⇒ drop Store ⇒ hard cancel. `AsyncPluginRuntime` implements AFIT `GuestRuntime` and slots into the closed-set `enum Runtime { Wasmtime(compile-only, folded away at exit), Async, Mock }` from Design A.
**Tokio routing**: tokio stays owned by `semio-framework-os-services`; plugin-host gains tokio `["sync","rt"]` but never constructs a Runtime — it receives `HostAsyncRuntime(tokio::runtime::Handle)` injected at `AsyncPluginRuntime::new`; build-time bins (describe, benches, 🏃️run bin) own their own current_thread runtimes (sanctioned). Delete `poll_ready` + every caller (mcp's holders become real awaits).
Acceptance: `cargo test -p semio-framework-plugin-host` incl. migrated S1b/S1c preemption tests + new `cancellation_drops_store_mid_import`; grep `poll_ready|ShardLoop|ShardExecutor` in 🖥️host → 0.

### B3. Fleet rebuild + describe pipeline (parallel with B2)
- **Describe async fix** (`📇️describe/📦️glue.rs:119–137`): Config gains component-model-async + async_support; linker keeps `pure` and adds a **stub host-async** (all 24 impls error "describe must be pure"); `instantiate_async` + `call_describe(...).await` on an own current_thread runtime. Acceptance: describe bin emits both descriptor files for an async-world 🗒️note.
- **Fleet rebuild** (after SP-G1 + Design A green): 33 plugins + 26 extensions to wasip2 with async bindgen. Descriptor completion for the 7 missing (demonstrator, fem, playbook, trinity✅, stdio, puzzle, block) via the proven `.declare_artifact` migration recipe; ratchet `DESCRIPTOR_MIGRATED_PLUGINS` (`PLUGIN/🦀️component.rs:16569`) 13 → 33.

### B4. Web: bridges, shard worker, host-shim (after SP-W1 + B3)
`🌐plugin-web-materialize.ts`: `transpilePluginComponent` (:363) gains the SP-W1-determined flags + `--map semio:framework/host-async=./🟨️host-shim.js`; `pluginComponentBridgeSource` (:295) survives with promise-returning api; `🟨️host-shim.js` grows the host-async impl — each import posts `effect-request` to the shard worker's host bridge, returns a promise resolved by `effect-complete`, riding the existing ShardFrame Envelope wire; http-fetch/blob-read adapt ReadableStream → jco streams. Re-materialize all 48 bridges (kills stale `runSerialized` output). Add `"_shard"` to `⚙️vite.config.ts:62` `pluginModuleDirNames`.
Acceptance: dev boot loads the s hub; a plugin turn round-trips worker → wasm poll → turn-result; an http-fetch effect resolves through the shim.

### B5. Targets (four parallel packets, after B2+B4)
- **5a wgpu-native**: replace `ParallelRuntime` with `KernelAsyncRuntime` — services-owned tokio Runtime, one AsyncActorTask per actor, outcomes mpsc → `Kernel::complete` pumped on the kernel thread; real parallelism from multi-thread tokio + independent Stores.
- **5b winit unblocking**: kill `pollster::block_on` at `🧊️wgpu/📦️glue.rs:1896,1909,1919,1923,2580`, `Shell/🧊️component.rs:3299`, `ProgramBridge/🧊️component.rs:523`, the `ureq::get` at glue.rs:1577, and the 100 ms sleep at :2604 — command channel into tokio + `EventLoopProxy` wakeups; winit thread ends with ZERO block_on.
- **5c wgpu-web onto ShardClient**: delete the `PluginWorkerClient` copy in `🧊️wgpu/🟦️typescript/🟦️boot.ts` + retired ABI; port to ShardClient as PluginRuntime does; fix `🧊️wgpu/📦️index.ts:6` dead imports.
- **5d 🏃️run + 💻️os/🖥️host through the kernel**: `WasmtimeNodeHost` stops constructing WasmtimeRuntime directly and minting RuntimeActorIds (:1275,:1352,:1470); takes a kernel-owned activation facade (`kernel.activate(plugin_id) → ActorId`), turns via kernel dispatch; `💻️os/🖥️host` boots the identical stack so native smoke exercises the one real code path.

### B6. Sync-dep literal-reimplementation packets (parallel throughout)
- **6a async HTTP**: hyper + rustls **inside `semio-framework-os-services` only**, behind the existing `AsyncHttpTransport`/`HttpBody` seam; replaces ureq in the directory client and glue.rs:1577. (If zero-new-deps is insisted on later: in-services HTTP/1.1 over tokio-TCP+rustls behind the same seam — the seam makes them interchangeable.)
- **6b pack sleep-in-poll**: `🎒️pack/⏳️async/🦀️component.rs:69` (200 µs) and :505 (20 ms), sibling `🎒️pack/🌐️http/🦀️component.rs:161` — store the Waker, wake on state transition.
- **6c brep adoption** (after SP-G1 GO): sweep the 134 `block_on` sites (flow 59, cad 45, stdio 15, process 13, animate 2) to real `.await` inside jobs/tasks; pairs with `register_job_kind` adoption per CPU-heavy plugin and the 41 `DownloadMediaExport` payload builds → jobs/tasks.
- **6d sanctioned-block_on allow-list** (define once, census-enforced): (1) binary/main executor entries (os-services, describe, benches, 🏃️run bin), (2) dedicated-thread actor bridges where the thread IS the executor (db postgres/neo4j bridges — explicitly sanctioned), (3) StorageScheduler bounded-blocking via spawn_blocking. Winit thread and wasm host paths are NEVER sanctioned.

### B7. Extension activation end to end (after B3 + B2; web half after B4)
Native install→activate region in `💻️os/🖥️host`: `.sxt` verify/unpack (existing `🧩️extension/🦀️component.rs`) → extension store dir → descriptor registered. **Gating**: on plugin activation the kernel queries installed descriptors with `extends == plugin_id` and activates each as `ActorKind::Extension`, pinned to the parent's shard, capabilities scoped to the parent, deactivation cascades — data-driven off descriptors so the scale fixture's 2,500 synthetic extensions use the same code path. Web: extensions materialize like plugins; `ActivationRegistry` grows the same extends-gated activation. Bench: extend budget 3 with `budget_50x50_activate` on the async runtime (first genuinely parallel measurement — the single-ShardLoop caveat dies with ShardLoop).

### B8. exchange-seam removal + channel shape
**Decision: AppChannel request→batched-reply does NOT survive; becomes send + outcome stream.** `PluginWasmHandle.exchange` (`🎠️kernel/🟦️component.ts:107`) → `enqueue(events): void` + `outcomes: AsyncIterable<TurnOutcome>`; `AppChannelHandle` (`💻️os/🟦️component.ts:1863`) → `AppChannelPort { send(events); outcomes }` with request/reply correlation via `request` events + the `respond` effect (the schema's documented exception); bump `APP_CHANNEL_VERSION` 12 → 13 + its cross-language pin test (:2546); `PluginRuntime/🟦️component.tsx:669` follows. Census: no `\bexchange\b` in first-party TS.

## Design A — universal async + zero first-party dyn (compile repair)

Full mechanism detail (code shapes, per-family analysis, codemod specs) in the companion file `/Users/ueli/.claude/plans/get-s-working-again-quiet-raccoon-agent-ad70381f6b8e88ba0.md` — sol copies it into the ticket as `📓️design-dedyn.md` at kickoff. Essentials:

### Rulings (ratify into `📌️important.md` first — everything cites them)
- **R1 dyn scope**: zero `dyn T` for the 236 first-party traits. `dyn Future/Fn/Any/Error` (std/lang) stay permitted, but dyn-Future erasure is confined to (i) argument-position plumbing (`HostFuture<T>` as `spawn_scoped`'s arg) and (ii) fn-pointer thunk returns in erasure tables (`ComposeFuture`, new `IoFuture`). dyn Future is BANNED from trait-method return position (that is the double-future damage being removed).
- **R2 async-literal exception classes**: E1 external-trait impls · E2 `const fn` · E3 `extern`/`fn main`/proc-macro entries · **E4 (new)**: fn items stored in fn-pointer slots (`AsyncComposeFn`, `IoEntry.run/sniff`, `SurfaceDeclaration.factory`, `OnceLock<fn()>` installers, `RawWakerVTable`) — an `async fn`'s pointer type is unnameable, language-fixed; E4 fns are macro-generated or tagged `// 🚫️async: E4` · **E5 (new)**: sync↔async executor bridges (`block_on`, `LocalExecutor` internals, `resolve_ready`), ≤1 per crate, tagged.
- **R3 Send boundary**: guest side (SDK, fleet, kernel guest paths) futures are ?Send (single-threaded wasm, LocalExecutor). Host side gets Send STRUCTURALLY (concrete enums at every former dyn seam; compiler derives Send at spawn sites) — never by `+ Send` RPITIT bounds. The one erased spawn channel stays `spawn_scoped(.., fut: HostFuture<()>)`.

### Dispatch decisions per family (all dyn uses are framework-side; the fleet has ZERO first-party dyn — verified)
1. **GuestRuntime** → hand-written closed-set enum `GuestRuntimes { Wasmtime, /*later*/ Async(AsyncPluginRuntime), Mock, Recording }` in `🖥️host/🦀️component.rs`; trait kept as AFIT contract for the concrete impls; 15 `Arc<dyn GuestRuntime>` sites → `Arc<GuestRuntimes>`; double-future collapse (`async fn execute_turn → Result<TurnResult, TurnFault>` directly); `poll_ready` replaced by `block_on` at shard THREAD ROOTS until Design B deletes the shard loop entirely.
2. **HostAsyncRuntime** → generics `Arc<R: HostAsyncRuntime>` (enum layering-impossible: impls live above the trait's crate — TokioHostRuntime in 🛎️services, InlineRuntime in db); `sleep_until`/`cancel_scope` un-double-futured; new dependency-free `pub fn block_on<F: Future>` (~25-line thread-park executor, E5) in `⏳️async`.
3. **Db storage family** (one crate `semio-framework-os-kernel-db`) → `DbBackend<R>` enum (Memory/Fs/Sqlite/Postgres/Neo4j/Fault) + per-sub-trait facet-ref enums (`WalRef<'a,R>` …) replacing `-> &dyn WalStorage`; `DbFuture` deleted (233 lines unwrapped); `ArtifactEngine<R>` generic; `Database::open` selects the variant from the URI.
4. **Backbone/BackbonePort** → `Backbones`/`BackbonePorts` enums in 🏪️store (all impls in-file); this is the one family where the enum also `impl`s the trait (downstream blanket impls key off it); SDK `attach_backbone` takes `Backbones` by value.
5. **PluginApp/editors/viewers** (the big one; all 26 dyn uses inside the SDK) → SDK decl-macro **`plugin_apps!`** defined next to `trait PluginApp` generating a per-plugin enum + `From` impls + match-delegating `impl PluginApp`; `SurfaceDeclaration<A>` with `factory: fn(&AppDefinition) -> A` (bare fn pointer kept, E4); declaration tree/builder/guest runtime genericized over `A: PluginApp` (`Plugin<A>`, `AppInstance<A>`, `PluginProgram::App` assoc type); guest statics relocate from SDK `thread_local!` into the `plugin_exports!(crate::plugin, apps = NoteApps)` expansion (`GuestHost<A>`); `PluginAppMediaFuture` deleted (plain AFIT, ?Send automatic). Fleet edits: 33 roots add `plugin_apps!` + the `apps =` arg; ~15 subset files annotate return types. A forgotten variant is a compile error (missing `From`), so no runtime registration drift.
6. **SpaceMember cluster** (highest-risk) → `SpaceHost<M>`/coordinator generics + per-plugin `space_members!` enum with a `MemberFactory` replacing the GLOBAL `dyn ChildStoreFactory` registry; `NoMembers` uninhabited default so non-composing plugins change nothing; fallback = fleet-wide enum in the one aggregator crate that already links the whole fleet natively.
7. **Not actually dyn** (verified): the other SDK traits (serializers, editors, viewers as traits, ArtifactApp, WindowKit, …) and all six registries hold data + fn pointers — no redesign, only E4 re-sync + double-future unwrap.
8. **Fn-pointer tables stay fn pointers**: `compose_thunk!` macro wraps async hops in macro-generated sync E4 thunks returning `ComposeFuture` (`fn __thunk<'a>(s) -> ComposeFuture<'a> { Box::pin($f(s)) }`); hop fns keep their literal `async fn`. `IoEntry.run/sniff` become `IoFuture`-returning fn pointers built by SDK generic erasure fns. `resolve_ready`'s staged-broken raw-waker helpers → `std::task::Waker::noop()`.

### Repair codemods (S1–S6, specs in the companion file; all live in the ticket folder)
- **S1** `deasyncify-external-impls.py`: reverse of asyncify-universal (same 236-trait census) over `✏️s` only — ~1,232 external-impl fns (548 Default, 600 serde, 53 From, 31 fmt) lose the wrong `async`.
- **S2** `restore-qualifiers.py`: restore the 19 `const` + 2 `extern` from the asyncify-fleet commit diff, byte-equality-guarded.
- **S3** `#[async_test]`: new proc-macro crate `semio-framework-async-macros` at `⏳️async/✨️macros/📦️packages/🦀️rust` (precedent: draw-fsm-macros, schema-derive). Source keeps the literal `#[async_test] async fn`; the macro emits the `#[test]` wrapper + an INLINE self-contained thread-park block_on (no runtime dep fan-out, no tokio; dev-dependency only). Script rewrites all **16,427** sites (2,897 files) + inserts the dev-dep per Cargo.toml.
- **S4** `unwrap-double-futures.py` (framework only): ~300 sites — `async fn … -> DbFuture/HostFuture/ComposeFuture/PluginAppMediaFuture/impl Future` → direct-return `async fn`; delete each alias when its use count hits the R1-permitted residue.
- **S5** `insert-await.py`: the span-keyed fixpoint loop off `cargo check --message-format=json` (E0308 "consider awaiting" / E0599 on opaque type / E0277), edits applied per pass sorted by descending offset, guard set per (file,span); 5–15 iterations/crate; residues surfaced for hand-fixing. Precedent: db-trait-flip's 4 scripts.
- **S6** `compose-thunk-rewrite.py`: wrap the 163 `ComposerEntry`/`IoEntry` bare-path registrations in `compose_thunk!`/io thunks; idempotent.

### Gate ladder to a compiling tree (Design A ordering)
Spine: `semio-framework-async` → `semio-framework` (io/pack) → kernel (store) → kernel-db → **SDK (ATOMIC)** → plugin-host → services/mcp/renderer/run/os → framework tests → fleet offline codemods → 🗄️stdio → 32 plugins + 26 extensions in parallel batches → workspace sweep. Gate 1 = `cargo check -p semio-framework-plugin --lib` zero E0038. Quantified fleet residue after de-dyn: S1's 1,232 + S2's 21 + S3's 11,553 + S5 awaits + S6's 163 + 33 root/15 subset hand edits — the 56,680 `async fn` bodies already MATCH the AFIT traits and produce no new signature errors.

## Reconciliation of A and B

- A first, B second: A gets the whole tree compiling with the poll backend still in place (GuestRuntimes::Wasmtime, shard threads on `block_on`); B then collapses the world, mounts `AsyncPluginRuntime` into the same enum, and deletes ShardLoop/poll_ready/ParallelRuntime. `resolve_ready` survives only per E5 at genuinely-sync boundaries inside `plugin_exports!` (builder install), nowhere in the host.
- **`world-collapse` (B1) lands immediately after the SDK atomic packet (A5), before plugin-host (A6)** — it changes the SDK/host bindgen surface, so it must sit inside the same quiet window; it is a sol-registrar atomic packet (schema + schema-parity re-spec + export-macro glue).
- The jco spike (SP-W1) and native harness spike (SP-N1) run against COPIES of the reduced WIT (S7 precedent — never the live schema) so they can start on day one, before A5.
- SP-G1's brep probe runs standalone on day one; its note-as-async-guest half waits for A5+B1.
- No git commands ever: the staged framework asyncify is simply left for the auto-commit bot; work proceeds on the working tree (diff against HEAD, never bare `git diff`).

## Workforce program (for the Opus 5 coordinator "sol")

Roles and contracts exactly as `📓️design-workforce.md` + `📌️important.md` (packet contract, lease-requests, luna claim format). Additional standing rules, all already ratified in `📌️important.md`: executors write code + cheap checks only, acceptance is coordinator-run (both `--lib` AND `--all-targets`), `CARGO_TARGET_DIR=<scratchpad>/target-<slug>` (never the ticket folder — EPERM), explicit 600000 ms timeouts, ≤6 live executors / ≤3 concurrent cargo builds, atomic packets never interrupted, baselines are named sets, negatives re-verified with python over emoji paths, `run_in_background:false` for executor Agent calls (background children die with the parent turn).

### Kickoff (sol, first turn)
1. `ticket_reopen` with the EXPLICIT path `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`.
2. Append the takeover announcement to `📓️status.md`: single coordinator; the two prior coordinators' in-flight state is absorbed (io-async-signatures never reported — its symbols landed and are re-exported; declare it ABSORBED and its scope released); the papert fleet plan's undispached packets are superseded by this program.
3. Ratify into `📌️important.md`: the four owner decisions, R1–R3, E4/E5, the sanctioned-block_on allow-list (B6d), the slug registry below (audit for collisions first, ruling-5 style), and the supersession of the two-coordinator path contract.
4. Copy the two design files into the ticket (`📓️design-dedyn.md`, `📓️design-async-world.md`); `df -h`; verify liveness of any peer session via `git log --date=iso` + report mtimes before touching contested files.

### Wave DAG

```
U0 kickoff (sol)
U1  jco-spike ∥ async-harness-spike ∥ brep-probe ∥ macros-blockon ∥ luna: dyn-census, stale-docs-census
U2  vocab-repair → { io-thunks ∥ store-dedyn ∥ db-dedyn } → sdk-dedyn (ATOMIC) → world-collapse (sol, ATOMIC)
    → host-dedyn → os-ripple (∥ per crate) → framework-tests → fleet-codemods (offline, all at once)
    → asyncfleet-stdio → asyncfleet-a..f (≤6 ∥) → GATE C (workspace compiles, tests run, dyn census 0)
U3  async-plugin-runtime ∥ describe-async → fleet-wasm-descriptors → GATE R (33/33 descriptors, note turn on async runtime)
U4  web-bridges → { wgpu-native-async ∥ winit-unblock ∥ wgpu-web-shard ∥ run-through-kernel ∥ extension-activation }
    → exchange-removal → GATE W (dev boot turn round-trip, native smoke path unified)
U5  http-hyper ∥ pack-waker (start any time) · adopt-stdio → adopt-a..f (block_on→0, jobs/tasks, dl_export, pending_effects)
    → GATE F (census targets moved)
U6  parity-rebaseline ∥ bench-web-rows → full ladder → exit checklist
```

### Packet registry (owner · path_scope exclusive · deps · size)

| Packet | Owner | path_scope | Deps | Size |
|---|---|---|---|---|
| `jco-spike` | terra | `💻️os/🧫️fixtures/🔌️jcoprobe/**` (new; WIT copy) | — | M |
| `async-harness-spike` | terra | `💻️os/🧫️fixtures/🔌️asyncprobe/**` (WIT copy) | — | M |
| `brep-probe` | terra | new probe crate per `📓️luna-brep-await-spec.md` §5 | — | S |
| `macros-blockon` | terra | `⏳️async/✨️macros/**` (new crate), `⏳️async/🦀️component.rs` block_on region | lease: root Cargo.toml member | S |
| `luna dyn-census` | luna | read-only: verify the 6-family list is complete over all 236 traits repo-wide | — | S |
| `vocab-repair` | terra | `⏳️async/🦀️component.rs` (HostAsyncRuntime unwrap, Waker::noop, ManualRuntime) | macros-blockon | M |
| `io-thunks` | terra | `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, `🎒️pack` staged damage | vocab-repair | M |
| `store-dedyn` | terra | `🏪️store/🦀️component.rs` (Backbones/BackbonePorts, SpaceMember generics, space_members!, ChildStoreFactory deletion) | vocab-repair | XL |
| `db-dedyn` | terra | `🛢️db/**` (DbBackend, facet-refs, DbFuture unwrap, `Arc<R>` generics) | vocab-repair | L |
| `sdk-dedyn` | terra | **ATOMIC**: `🔌️plugin/🦀️component.rs`, `🏗️builder/🦀️component.rs` (plugin_apps!, generics, GuestHost, plugin_exports! rework, PluginAppMediaFuture deletion, S4 on SDK) | io-thunks, store-dedyn | XL |
| `world-collapse` | **sol** | **ATOMIC registrar**: `🧬️schema/📜️component.wit`, `🧪️schema-parity/🦀️component.rs`, export-macro glue | sdk-dedyn | M |
| `host-dedyn` | terra | `🖥️host/🦀️component.rs` + `🧵️shard/**` (GuestRuntimes enum, thread-root block_on, poll_ready demotion) | world-collapse | L |
| `os-ripple` | terra ×N | per-crate: services, kernel, mcp, renderer, 🏃️run, 💻️os host (Arc<GuestRuntimes>, `Arc<R>` ripple, S5 loops) | host-dedyn | L |
| `framework-tests` | terra | S3+S5 over 🧰️framework tests (4,704 sites) | os-ripple | M |
| `fleet-codemods` | terra | offline S1+S2+S3+S6 over `✏️s/🔌️plugins/**` (no compile) | sdk-dedyn | M |
| `asyncfleet-stdio` | terra | `✏️s/🔌️plugins/🗄️stdio/**` (root plugin_apps!, subsets, S5 loop) | fleet-codemods, GATE after host-dedyn | L |
| `asyncfleet-a..f` | terra ×6 | remaining 32 plugins + 26 extensions, batched as papert's partitions (small/cad/flow/imperative/heavy-a/heavy-b+long-tail) | asyncfleet-stdio | L each |
| `async-plugin-runtime` | terra | `🖥️host/⏳️runtime.rs` rewrite + mount, tokio handle injection, GuestRuntimes::Async | GATE C, async-harness-spike | XL |
| `describe-async` | terra | `📇️describe/**` (async store + stub host-async) | world-collapse | M |
| `fleet-wasm-descriptors` | terra | 7 missing descriptors via `.declare_artifact` recipe; ratchet 13→33; wasm rebuild driven by sol | describe-async, asyncfleet-* | L |
| `web-bridges` | terra | `🌐plugin-web-materialize.ts`, `🟨️host-shim`, vite `_shard` fix, re-materialize 48 bridges | jco-spike, fleet-wasm-descriptors | L |
| `wgpu-native-async` | terra | `🎯️targets/🧊️wgpu/🎠️runtime.rs` (KernelAsyncRuntime replaces ParallelRuntime) | async-plugin-runtime | L |
| `winit-unblock` | terra | `🧊️wgpu/📦️glue.rs` block_on/ureq/sleep sites, `ProgramBridge`, Shell lease | wgpu-native-async | M |
| `wgpu-web-shard` | terra | `🧊️wgpu/🟦️typescript/**`, `🧊️wgpu/📦️index.ts` (delete PluginWorkerClient, port to ShardClient) | web-bridges | M |
| `run-through-kernel` | terra | `🏃️run/**`, `💻️os/🖥️host/**` (kernel activation facade) | async-plugin-runtime | M |
| `extension-activation` | terra | `💻️os/🖥️host` install region, kernel + ActivationRegistry extends-gating, `budget_50x50_activate` | async-plugin-runtime; web half after web-bridges | L |
| `exchange-removal` | terra | kernel TS `PluginWasmHandle`, os TS `AppChannelHandle` v13, PluginRuntime consumer | web-bridges | M |
| `http-hyper` | terra | `🛎️services/**` (hyper+rustls behind AsyncHttpTransport), directory client, glue.rs:1577 site (lease) | — | M |
| `pack-waker` | terra | `🎒️pack/⏳️async`, `🎒️pack/🌐️http` waker redesign | — | S |
| `adopt-stdio`, `adopt-a..f` | terra ×≤6 | per-plugin: block_on→await (134), register_job_kind, AsyncTask, dl_export→jobs (41), pending_effects→0 (3) | brep-probe GO, GATE W | L each |
| `parity-rebaseline` | terra+luna | parity harness re-baseline react-new vs wgpu-new; 58/58 sweep sharded | GATE W | M |
| `bench-web-rows` | terra | dev `📜️script.ts` bench react/wgpu rows (currently `benchWebSkippedRow`), 8 budgets ×3 renderers | wgpu-web-shard | M |
| `census-zero` | luna+sol | scripted census with 6d allow-list; banned-symbol sweep | all | S |

### Gates
- **GATE C** (compile): `cargo check --workspace --all-targets` exit 0; `cargo test` ladder green per the A table; dyn-census of 236 first-party traits = 0; async-literal census ≥ 98%; the 5 known-by-name plugin suite failures tracked as named sets.
- **GATE R** (runtime): describe emits for an async 🗒️note; note executes a turn through `GuestRuntimes::Async`; cancellation-drops-Store + S1b/S1c preemption tests green on the async engine; 33/33 descriptors, ratchet 33.
- **GATE W** (wired): dev boot loads s, plugin turn round-trips worker→wasm poll→turn-result, http-fetch completes through the shim; native smoke 33/33 via 🏃️run THROUGH the kernel; extension install→activate round-trip; no `PluginWorkerClient`/`exchange`.
- **GATE F** (fleet adoption): census `block_on` 0 (minus 6d allow-list), `pending_effects` 0, `register_job_kind` > 0 per CPU-heavy plugin, dl_export payloads in jobs/tasks.
- **EXIT**: verification ladder 1–10 (Design B §Phase 9): schema gate · native runtime gate · fleet gate · native smoke 33/33 · web dev boot · parity re-baselined 58/58 react+wgpu (same architecture on both sides) · 8 bench budgets ×3 renderers incl. budget 5 (p95 ≤ 8 ms native — the current 140.9 ms failure must fall to the async runtime rework) + `budget_50x50_activate` · census zero · end-to-end s scenario (edit → checkpoint → kill shard worker → FailurePolicy restore → state intact, web AND native) · zero rust warnings native + wasip2 + wasm32-unknown-unknown · launch.json regenerated · `[DEBUG]` sweep · `📌️important.md` emptied last · `ticket_close` with explicit path + full file list.

### Risk register (owned)
1. **jco async exports (SP-W1)** — the one genuinely external risk; verdict gates the whole web path; fallbacks F2 (hand-rolled callback-ABI driver) and F3 (temporary sync-lifted second build) specified.
2. **SpaceMember genericization** — highest-risk de-dyn family; aggregator-crate fleet-enum fallback specified.
3. **S5 convergence on the two giant files** (SDK 20,733 lines) — budget manual passes; sol runs those loops itself.
4. **sdk-dedyn + world-collapse form one long quiet window** — nothing else may build against the SDK during it; schedule fleet-codemods (offline) inside that window to reclaim the time.
5. **Budget 5 (interactive p95) is a real design failure** (140.9 ms vs 8 ms) — wgpu-native-async must demonstrate the fix (dedicated interactive lanes on the multi-thread runtime, no shared single ShardLoop); if it doesn't fall out, a dedicated packet follows before exit.
6. **Live tree**: auto-commit bot + possible stray sessions — every packet re-reads files before editing, diffs against HEAD, uses `git log --date=iso` for provenance (commit-message dates are fake).

## Verification (how the plan itself is checked during execution)
After GATE C: full test ladder + dyn census. After GATE R: descriptor freshness ×33 + async-runtime harness tests. After GATE W: parity smoke note+cad on both renderers + native smoke + extension round-trip. After GATE F: census JSON snapshot diffed against the baseline in the ticket. Exit: the full checklist above, every command's output pasted into `📓️status.md`.
