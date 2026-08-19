# 📌️ Binding rules — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME

**Empty this file before `ticket_close`.**

---

# 🌅️ U-PROGRAM RULINGS (2026-08-19) — these SUPERSEDE anything below that contradicts them

Plan of record: `📋️master-u.md`. Designs: `📓️design-dedyn.md` + §"Design B" of `📋️master-u.md`.

## Owner decisions (not negotiable, not re-litigable by any packet)

- **O1 — DROP DYN DISPATCH.** Every first-party dyn-dispatched seam becomes enum / static / generated
  dispatch so plain AFIT (`async fn` in trait) works. Every first-party fn keeps the **literal `async`
  keyword**. The boxed-future-trait-method route (`DbFuture`-shaped) is **REJECTED** as an end state —
  the existing instances of it are damage to be removed, not precedent to follow.
- **O2 — ONE COORDINATOR.** The W5/W6 coordinator and the papert fleet coordinator are stood down.
  The two-coordinator path contract (fleet owns `✏️s/🔌️plugins/**`, everything else registrar-only) is
  **withdrawn**; ownership is now purely the U-program packet registry.
- **O3 — the root `compose/` tree is OUT OF SCOPE ENTIRELY.** Never edit it, never gate on it. The
  framework's own `semio.compose` cold job kind IS in scope. (Do not confuse the two.)
- **O4 — external sync deps**: literal reimplementation where no async version exists; async-native
  replacement where one does; always behind a first-party interface.

## R1 — what "zero dyn" means

Zero `dyn T` where `T` is one of the ~236 first-party traits. `dyn Future`, `dyn Fn/FnMut/FnOnce`,
`dyn Any`, `dyn Error` (std/lang) remain PERMITTED, but **dyn-Future erasure is confined to**
(i) argument-position plumbing (`HostFuture<T>` as `spawn_scoped`'s argument) and (ii) the return type
of fn-pointer thunks in erasure tables (`ComposeFuture`, `IoFuture`).
**`dyn Future` is BANNED from trait-method return position** — that is exactly the double-future damage
being removed. A trait method returning `Pin<Box<dyn Future>>` is a bug from now on.

## R2 — async-literal exception classes (the ONLY legal reasons a first-party fn is not `async`)

- **E1** impls of externally-declared traits (serde, `Display`/`Debug`, `From`/`TryFrom`, `Default`,
  `Drop`, `Iterator`, `Future::poll`). Signature fixed outside this repo.
- **E2** `const fn`. **E3** `extern "abi" fn`, `fn main`, proc-macro entry points.
- **E4 (NEW)** fn items whose VALUE is stored in a **fn-pointer-typed slot** — `AsyncComposeFn`,
  `IoEntry.run/sniff`, `SurfaceDeclaration.{factory,app_schema,mutation_roster}`, `OnceLock<fn()>`
  installers, `RawWakerVTable` members. An `async fn` item's pointer type is unnameable, so this is
  language-fixed, same class as E3. **Discipline: E4 fns are either macro-generated (invisible in
  source) or tagged `// 🚫️async: E4 fn-pointer slot`.**
- **E5 (NEW)** sync↔async bridge entry points: `block_on`, `LocalExecutor` internals, `resolve_ready`,
  hand-rolled `Future::poll` impls. **At most one per crate**, tagged `// 🚫️async: E5 executor bridge`.

Anything outside E1–E5 that is not `async fn` is a defect. Untagged E4/E5 is a defect.

## R3 — the Send boundary

- **Guest side** (`semio-framework-plugin`, the store's guest paths, all 63 fleet crates): futures are
  **?Send**. Single-threaded wasm, `LocalExecutor`, thread_local state. Never add `+ Send`.
- **Host side**: Send-ness is obtained **STRUCTURALLY** — every former dyn seam becomes a concrete enum,
  so at each spawn site the future's concrete type is known and the compiler derives `Send` itself.
  **Never `+ Send` RPITIT, never return-type-notation, never `trait-variant`.** If a generic host path
  needs to spawn a trait-method future, the fix is *route it through the enum*, never *add a bound*.
- The one erased spawn channel that survives is
  `HostAsyncRuntime::spawn_scoped(&self, scope, ctx, fut: HostFuture<()>)` — callers build the box at
  concrete types (argument-position, R1-legal).

## R4 — sanctioned `block_on` allow-list (census-enforced; everything else must reach 0)

1. Binary / `main` executor entry points: `semio-framework-os-services`, the describe bin, benches,
   `🏃️run/📦️bin.rs`.
2. **Dedicated-thread actor bridges where the thread IS the executor** — the db `postgres`/`neo4j`
   bridge threads are explicitly sanctioned under this clause.
3. `StorageScheduler`'s bounded-blocking storage ops (deliberate: bounded + lane-prioritised +
   quota-accounted, which `tokio::fs`'s unbounded `spawn_blocking` pool is not).
4. Shard/actor **thread roots** for as long as a thread-loop backend exists (removed when the async
   runtime becomes the sole backend).

**NEVER sanctioned**: the winit thread, any wasm host path, any per-call site inside a turn.

**Clause 5 (added after `pack-waker` correctly asked): a `#[test] fn` body is a sanctioned executor
entry point.** A test harness is a `main`-equivalent — it is the thread root, and something has to be the
bridge. So `block_on` inside `#[cfg(test)]` is allowed and is NOT counted against the census target.
Preferred form is still `#[async_test]` (which keeps the literal `async fn` and generates the bridge for
you); a hand-written `block_on` in a test is acceptable where the test needs to control the executor
itself. Tag either way. The census must therefore report **production** `block_on` separately from
**test** `block_on` — a single blended total would be both a false alarm and a false all-clear.

## R8 — `#[async_trait]` must go (it is a boxed-future trait method by another name)

The external `async_trait` macro desugars precisely to `Pin<Box<dyn Future>>` in trait-method return
position, which **R1 bans** and which **O1 rejects** as an end state. Measured surface — small and fully
enumerated, so there is no excuse for it to survive:

| location | sites |
|---|---:|
| `🧰️framework/🔨️modules/🎒️pack/🌐️http` | 5 |
| `🧰️framework/🔨️modules/🎒️pack/⏳️async` | 3 |
| `🌎️hub/📇️directory/` (`🦀️component.rs`, `🐘️postgres`, `🪶️sqlite`, `🌐️neo4j`) | 4 |
| **total** | **12 attribute sites in 6 files**, 5 `Cargo.toml` declarations |

Replace with plain AFIT (`async fn` in trait) plus enum dispatch at the consumer, exactly as O1 requires
everywhere else; then drop the `async-trait` dependency from those 5 manifests. Assigned: the `🎒️pack`
half to the follow-up that re-accepts `pack-waker`; the `🌎️hub/📇️directory` half to `os-ripple`.

## R5 — packet slugs are U-program slugs

`jco-spike` `async-harness-spike` `brep-probe` `macros-blockon` `dyn-census` · `vocab-repair`
`io-thunks` `store-dedyn` `db-dedyn` `sdk-dedyn` `world-collapse` `host-dedyn` `os-ripple`
`framework-tests` `fleet-codemods` `asyncfleet-stdio` `asyncfleet-a`…`asyncfleet-f` ·
`async-plugin-runtime` `describe-async` `fleet-wasm-descriptors` · `web-bridges` `wgpu-native-async`
`winit-unblock` `wgpu-web-shard` `run-through-kernel` `extension-activation` `exchange-removal` ·
`http-hyper` `pack-waker` `adopt-stdio` `adopt-a`…`adopt-f` · `parity-rebaseline` `bench-web-rows`
`census-zero`. Reports are `📓️terra-<slug>-report.md`, audits `📓️luna-<topic>-audit.md`.

## R11 — OPEN extension points de-dyn via GENERICS + ASSOCIATED TYPES, never an enum, never a box

`kernel-ripple` escalated the first genuine architectural blocker of the de-dyn program: four traits in
`🧰️framework/🔨️modules/🚪️io` are **open host-extension points with no closed implementor set**, so
`dyn_enum_close!` cannot apply and **R1 bans the boxed-future alternative**. Ruling, after reading every
one of their 17 use sites:

**They are not one problem, they are two.**

**(a) Parameters and borrowed references — trivially generic.** `&mut dyn PayloadSource`,
`&dyn RandomAccessPayload`, `&'a mut dyn PayloadSink` (`:387, :435, :479, :504, :545, :624, :2156`) become
`<S: PayloadSource>(source: &mut S)` etc. No design question; just do it.

**(b) The real one: a trait method that RETURNS a runtime-chosen implementation.**
```rust
async fn resolve_decode(&self, request: &ResourceRequest) -> CodecResult<Box<dyn PayloadSource>>;
async fn resolve_encode(&self, request: &ResourceRequest) -> CodecResult<Box<dyn PayloadSink>>;
```
A resolver decides *at runtime* whether to hand back a file, a memory slice, a stream. An enum in `🚪️io`
cannot enumerate what third-party resolvers will return, so the closed-set mechanism genuinely does not fit.

**Resolution — associated types push the choice to the implementor:**
```rust
pub trait ResourceResolver {
    type Source: PayloadSource;
    type Sink: PayloadSink;
    async fn resolve_decode(&self, request: &ResourceRequest) -> CodecResult<Self::Source>;
    async fn resolve_encode(&self, request: &ResourceRequest) -> CodecResult<Self::Sink>;
}
```
and every holder of `Arc<dyn ResourceResolver>` (`:370, :418`) takes a generic parameter instead.

**Why this is the right shape, not a dodge:** the openness is real but it lives at the *implementor*, not
the *call site*. A resolver that genuinely needs runtime variance declares its own enum over the source
kinds **it** supports — and may generate it with `dyn_enum_close!`. So the erasure happens where the set is
actually closed, which is the whole principle behind **O1**. Nothing is boxed, nothing is `dyn`, and no
caller loses expressiveness.

**Consequence to accept honestly:** this monomorphises the codec paths and the type parameter threads
through their holders. If it threads through more than ~10 public types, **stop and report** — that is a
coordinator call, exactly as it was for `SpaceMember`.

**Generalises to every remaining open family**: open set ⇒ generics (+ associated types where a method
returns an implementation); closed set ⇒ `dyn_enum_close!`; exactly one impl ⇒ delete the trait object and
use the concrete type. **Never** reintroduce a boxed trait object to avoid the work.

## R10 — 🚫 NEVER build a NAME-KEYED `.await` inserter. Use the span-keyed shared tool.

**This already happened and cost a packet most of its budget.** `math-dedyn` hit the point where
`insert-await.py` reached fixpoint with residue, built a bulk tool that appended `.await` to any call
matching a locally-declared `async fn` **name**, and it **corrupted ~250 of its own 1,479 edits**.

The reason is not carelessness, it is arithmetic: first-party async fns are named `len`, `new`, `get`,
`fill`, `count`, `is_empty`, `clear`, `push`, `contains`, `split`, `as_str` — and those names also belong
to `Vec`, `HashMap`, `str`, `u64` and every other std type in scope. A name-keyed pass **cannot tell them
apart**, so it awaits sync std methods and silently produces nonsense that compiles in some places.

This is the *same* defect the ticket already recorded as rule 27 (span-keyed edits are safe; name-keyed
edits hit production code that merely shares an identifier). **It was rediscovered the expensive way
because the rule was buried in a report instead of in the rules.** Hence R10, stated as a prohibition:

- ✅ Use `insert-await.py`. It is **span-keyed** — it applies only the byte span rustc itself points at,
  only when the diagnostic yields exactly ONE candidate, and it refuses ambiguity rather than guessing.
- ⛔ Do **not** write a name/regex-based awaiter, however tempting, and however "obviously safe" the name
  list looks. There is no safe name list; the collisions are with std.
- **When the shared tool reaches fixpoint, the residue is HAND work, not tool work.** That residue is
  where the genuinely interesting cases live (below), and each needs a decision recorded in the report.

### The residue shapes no tool can fix — recognise them, fix them by hand
1. **`.await` inside a sync closure.** `sort_by`, `dedup_by`, `map`, `filter` take **sync** closures;
   `.await` is illegal there (E0728). Fix by either hoisting the await out of the closure, precomputing
   the keys before the sort, or — if the awaited fn is a pure accessor — R9.
2. **Awaiting one future repeatedly inside a loop/closure.** A future is consumed by a single `.await`;
   awaiting it n times is a *bug the async conversion exposed*, not a conversion artifact. Hoist it.
3. **Self- or mutually-recursive async fns** — need `Box::pin` to break the infinite future size.
4. **Futures stored in structs**, and `map`/`and_then` chains over futures.

If you build a recovery tool for a bad bulk edit, make it **diagnostic-driven** (delete exactly the byte
span rustc flags), never name-driven — and **save it into the ticket folder** so the next packet inherits
it rather than rebuilding it.

## R9 — E1 is TRANSITIVE: a pure computation whose consumers cannot be async stays sync

The blind codemod made pure in-memory helpers `async`. Where those helpers are consumed by code that
**can never** be async — impls of externally-declared traits (serde `Serializer`/`Deserializer`,
`Display`, `Debug`), fn-pointer slots (E4), or encoders that are themselves E1/E4 — the helper cannot be
async either. `async` there buys nothing (no suspension point exists) and costs a compile error with no
alternative fix. **E1 therefore propagates one hop backwards along the call graph.**

**Decision procedure — per function, with evidence, never as a blanket sweep:**
1. Does the fn perform any I/O? Check for `std::fs`, `tokio`, `reqwest`, `ureq`, `File::`, `TcpStream`,
   `spawn`, `sleep`, `SystemTime`. If yes → it stays `async`; fix the consumer instead.
2. If it is pure AND at least one consumer is E1/E3/E4 → make it sync and **tag it**:
   `// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9`
3. If it is pure and every consumer *can* become async → **make the consumer async instead.** That is the
   direction the decree wants; R9 is a fallback, not a shortcut for avoiding await-insertion work.

Worked precedents (both verified I/O-free before conversion, both went green immediately):
`🧰️framework/🔨️modules/🌱️value/**` (11 + 8 fns; consumers were hand-rolled serde impls) and
`🧰️framework/🔨️modules/⚠️diagnostic/**` (39 + 2 fns). Their `.await`s were removed along with the
keyword — **an orphaned `.await` after de-asyncifying is E0728 and must be removed in the same edit.**

⚠️ Do NOT use R9 to de-asyncify something merely because awaiting it is inconvenient. The test is
"no suspension point exists AND a consumer is language-barred from being async", and both halves must be
shown in the report.

## R7 — `async_fn_in_trait` is ALLOWED, crate-wide, with a written reason (do NOT "fix" it)

Measured on the first crate to go green (`semio-framework-async`): `--lib` and `--all-targets` and
`cargo test` all exit 0, with **6 warnings, all of them**:

> `warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified`

Under universal async this fires on **every public trait with an async method** — i.e. ~93 trait families,
potentially hundreds of warnings, against an exit bar that demands zero.

**The lint's concern is real but it is already answered by R3.** It warns that callers cannot assume the
returned future is `Send`. Our architecture answers that *structurally*: every former `dyn` seam becomes a
concrete enum, so at each spawn site the future's concrete type is known and the compiler derives `Send`
itself. Guest-side futures are deliberately `?Send`.

**Therefore:**
- ✅ Add `#![allow(async_fn_in_trait)]` at crate root, with a one-line comment pointing at R3 and R7.
- ⛔ **NEVER silence it by writing `-> impl Future<Output = T> + Send` on the trait method.** rustc
  suggests exactly this in the warning text, and it is the WRONG fix here: it re-imposes `Send` on guest
  traits whose futures cannot be `Send` (single-threaded wasm, `LocalExecutor`, thread_local state), and
  it contradicts R3 in the letter. Do not take the compiler's suggestion.
- ⛔ Never resolve it by making the trait method sync.

Every other warning class still counts toward the zero-warning exit bar.

## R6 — ATOMIC packets in this program (rule 25 applies: redirect BEFORE start or let them FINISH)

`sdk-dedyn` · `world-collapse` · each `asyncfleet-*` crate sweep · `fleet-codemods`.
`sdk-dedyn` + `world-collapse` form ONE long quiet window in which nothing else may build against the
SDK. Offline work (`fleet-codemods`) is deliberately scheduled inside that window.

---

## Hard prohibitions (every agent)

1. **No git-modifying commands.** No `commit`, `stash`, `checkout`, `reset`, `worktree`, `add`. Other sessions are live in this tree and an auto-commit bot runs. `git status` is NOT a churn detector — use `git log --oneline -3 -- <path>` and file hashes.
2. **No `ticket_close` / `ticket_reopen` by anyone but sol.** A subagent closing this ticket closes the whole umbrella.
3. **Never edit outside your packet's `path_scope`.** A region name inside a shared file is not ownership. Need a shared-file change → emit a `lease-request` block and stop.
4. **Never run bare workspace cargo.** Always `CARGO_TARGET_DIR=<ticket>/🎯️target` and `-p <crate>`. A slow build is not a hung build.
5. **Scratch files are `.txt`/`.md`/`.json` inside the ticket folder.** Never `.log` (repo-wide gitignored — `ticket_close` silently drops them).
6. **Do not touch `.cargo/config.toml` or add per-crate `RUSTFLAGS`** — the uniform 512 MiB wasm limit is deliberate; per-crate flags churn cargo fingerprints across the whole fleet.
7. **Never claim a test passed without pasting its output and exit code.**
8. Temporary logs carry the `[DEBUG] ` prefix and are removed before a packet reports done.

## Registrar-only files (sol edits these; everyone else sends a `lease-request`)

`/📜️script.ts`, `/Cargo.toml`, `/Cargo.lock`, `/📋️project.json`, `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json`, `🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `🔌️plugin/📦️packages/🟦️typescript/📇️registry/**`, all `🤖️generated/**`, `Shell/🧊️component.rs` (shared with live hover/selection tickets), `ShellHost/🟦️component.tsx`.

## Replace, never wrap — these must not exist at exit

`exchange` (WIT + all callers) · `PluginWorkerClient` (BOTH copies: `🎠️kernel/🟦️component.ts`, `🧊️wgpu/🟦️typescript/🟦️boot.ts`) · `LeasePool`/`PluginModuleLease` in the kernel (the generic `createLeasePool` relocates to `📦️packages/🟦️typescript/🟦️glue.ts` for its 3 non-plugin users) · `WasmPluginRuntime` · `ExtensionRuntime` · **both** `ProgramSupervisorState` definitions · `PLUGIN_FUEL_BUDGET` · `PLUGIN_WORKER_UNRESPONSIVE_MS` · `INSTANCE_GUARD`/`clear-instance-guard` · `host_port` · `component::host_*` · `install_io_fallback_dispatcher` · `set_host_backbone_channel` (process-global) · `runSerialized` retry/reload loop · `loadPluginModuleUncached`.

## Naming hazards

- `kernel::ActorId` **already exists** (re-export of `protocol_core::ActorId`, the presence/collab actor, `🎠️kernel/🦀️component.rs` L40). The runtime actor id is re-exported as **`RuntimeActorId`**. Never shadow.
- `🎭️actor` crate must stay pure: no `wasm_bindgen`, `web_sys`, `winit`, `tokio`, `std::thread` in the crate core — transports are injected. This is what keeps mobile open.

## Sequencing constraints

- The ABI flip is **big-bang**: A2/A3/B1 land and the fleet rebuilds before W3 fans out. The SDK crate is frozen during W3.
- `🗄️stdio` migrates alone and first in W3 (every plugin depends on it). `🎪️demonstrator` migrates last (bundles panes from cad/process/puzzle/procedural/gis/sourcing).
- Linked extension mode is feature-gated to avoid the `semio-framework-os-flow` ↔ extension crate cycle.
- Descriptor `extends` gates extension-actor activation on parent activity — a linked extension must not also run as an actor.

## ⚠️ Live peer ticket contending for our core files (2026-08-17 21:05)

`26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` slice **W1-D** is running RIGHT NOW (its `📓️w1-d-report.md` was written 21:02) and holds large **uncommitted** work in files this ticket must rewrite:

| file | peer's uncommitted delta | our packet |
|---|---|---|
| `🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit` | +37 (guest `list-io-entries`/`io-run`/`io-sniff`; host `io-routes`/`io-run`/`io-identify`) | `A2` |
| `🔌️plugin/🖥️host/🦀️component.rs` | +459 (`IoRouter` route resolution) | `B1` |
| `🔌️plugin/🦀️component.rs` | +1259 / −96 (io mechanism in the guest SDK) | `A2` |
| `🎠️kernel/🟦️component.ts` | +341 (`//#region 🔖️IoRouter`, `IoEntryGraph`, `ioRun`) | `A3` |

Rules that follow:

- **User decision 21:10: proceed now, absorbing the current working tree.** The hold on `A2-abi-sdk` / `B1-host-native` is lifted. The peer's uncommitted work **is the baseline** — treat the working tree, never `HEAD`, as the state to build on. Any agent in those files re-reads from disk immediately before every edit, edits surgically by region, and must be able to show the peer's io mechanism still present (as absorbed job kinds / effects) at the end.
- When A2/B1 do run, they **absorb** rather than delete the io mechanism: guest `io-run`/`io-sniff` become the cold job kinds `semio.io-run`/`semio.io-sniff`; host `io-routes`/`io-identify`/`io-run` become the `RegistryQuery` and `IoCompose`/`IoRun` effect variants with completions. The route-resolution algorithm, the ≤3-hop cycle-free rule, the ranking order (highest minimum fidelity → fewest hops → lexicographic) and the self-owned-hop reentrancy guard are all preserved semantics — they map onto host-side routing after a turn, which is exactly where the new design already puts cross-plugin routing.
- Any agent editing a file in that table makes **surgical region-scoped edits only**, never a full-file rewrite, and re-reads from disk immediately before each edit.

## Environment

- Disk was at 100% on 2026-08-17; freed by removing the `🎯️target` dirs of the two CLOSED tickets `☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` and `☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` (user-approved). sol checks `df -h` at every wave start and asks the user before deleting anything further.
- Ports: bench/parity use the 7300+ pool via `findFreeParityPortPair`, never the catalog ports 6012–6205.
- ≤6 concurrent building agents (cargo lock + disk).
- **One ticket target dir serializes our own builds.** Observed 2026-08-17: two of this ticket's own commands sat on `Blocking waiting for file lock on build directory` against our shared `🎯️target`, while a third peer ticket saturated the global `~/.cargo` package-cache lock — 54 cargo processes, a wasm check that compiles in seconds taking 23 minutes. Consequences, binding from W2 on:
  - **Only ONE packet at a time may hold a cargo build.** Parallel *editing* is free; parallel *building* is not. Stagger acceptance runs, or give each concurrently-building packet its own `🎯️target-<packet>` dir (prior tickets did exactly this — e.g. `🎯️target-w3-cad`, `🎯️target-verify`) and accept the disk cost.
  - Prefer one `-p <crate> --all-targets` check per packet over a suite of narrower ones; never `--workspace` from an executor.
- **Executors must run cargo in the FOREGROUND, in a single turn.** All four W1 executors independently stalled in wake/idle loops on backgrounded builds that cannot survive a subagent turn boundary (~1.4M tokens spent collecting nothing). Acceptance runs belong to the coordinator session; executors report what they actually observed.
- **After any atomic rename, the coordinator re-greps the tree.** A3's 132-file sweep missed a live `type HostEffect` import in the React renderer entry — an executor's own file count is not proof of completeness.

## W5+ additions (async-first rewrite) — measured this session, binding on every packet

9. **An API that exists is not an API that is implemented.** wasmtime 34.0.2 exposes the whole `component-model-async` surface — feature flag, `Config` knobs, types — and its engine is ~35 bare `todo!()` bodies with `StreamReader<T>` carrying zero trait impls. It compiles, links, and panics. Before adopting any new external capability, read its source or execute it; version availability is not feature availability. (The certified working version is **wasmtime 47.0.3**.)
10. **`cmd | tail -N; echo $?` reports tail's exit code, not the command's.** Read the pass/fail summary line, or run the command without a pipe. A confidently-pasted wrong exit code is worse than none — and rule 7 demands exit codes.
11. **Baselines are named sets, never counts.** Failure counts are meaningless across a suite that has grown (the plugin SDK suite went 230→247 tests, so "4 failures" became uncomparable and cost a packet real effort proving innocence). Record *which* tests fail, and settle attribution by running suspects **in isolation** — deterministic-alone means pre-existing; passes-alone-fails-in-suite means shared global state.
12. **After fixing one variant of a serde-shape defect, sweep every sibling.** `#[serde(tag = "kind")]` cannot serialize a newtype variant whose payload is not a map (string/int/`Vec<u8>` all fail at RUNTIME, compiling clean). The `JobStep` fix recorded this instruction in W4 and nobody executed it; six more instances were sitting in `🎭️actor` (`Payload::Event`/`Cancel`, `Origin::Actor`, `TurnStatus::Faulted`, `FailureSignal::Trap`, `Backpressure::Dropped`). They are **latent, not live** — that crate has no `serde_json` dependency and the wire uses the hand-rolled `pack_encode` — but the generated TS mirror renders them as impossible `object & string` intersections, so the mirror cannot type those variants.
13. **A vitest config with explicit filename arrays silently ignores new files.** `🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts` lists names in `include`/`coverage.include`/`includeSource` rather than globbing, so a new test file **does not run while the suite still reports green**. Add the filename, then re-run with `--reporter=verbose` and confirm your tests appear **by name**. Several packages also double-count in-source suites (91 unique → 182 reported); divide before comparing to a baseline.
14. **Never name the hidden library in the interface that hides it.** `semio-framework-async` briefly had a `tokio_workers` field and a `ThreadRole::TokioWorker` — in a serialized, ts-rs-mirrored type, so the leak would have reached the TypeScript wire. Now `io_workers` / `IoWorker`. Doc-comment prose naming today's concrete choice is fine; identifiers are not.
15. **W5+ packet ids are descriptive slugs, not letter-numbers.** `A1`, `H2`, `P1`, `M1`, `R1`, `G1` and `W0` all collide with ids/waves/gates this ticket already used; one packet nearly overwrote a finalized `📓️terra-R1-report.md`. Use `spike`, `async-iface`, `params`, `wasmtime-upgrade`, `services`, `shard-grants`, `kernel-loop`, `effects-async`, `shell-unpark`, `directory-and-run`, `lifecycle`, `sdk-async`, `async-worlds`, `packaging`, `e2e-proof`, `web-*`. Reports are `📓️terra-<slug>-report.md`.
16. **Reconnect backoff must reset after SUSTAINED health**, never on socket-open alone (open-only resetting defeats the backoff against an accept-then-drop server, which is the failure it exists for). Required by the "support short connection-shortages without freezing" rule: a monotonically growing counter makes a healthy session wait ~`maxMs/2` after a momentary blip.
17. **Do not put two packets in one file.** `shard-grants` was held out of `🖥️host/🦀️component.rs` while `wasmtime-upgrade` rewrote it, and the TurnResult bridge was relocated to `🧵️shard/` so the collision cannot recur. This ticket has already absorbed four half-landed peer changes with the same signature — *the artifact moved, its registration did not*.

18. **`include` + `includeSource` naming the same file makes vitest collect it TWICE.** Every TS baseline recorded in this ticket before 2026-08-18 ~20:30 was inflated 2×. Fixed in the four packages this ticket touches (`🧰️framework`, `💻️os`, `🧑️‍💻️dev`, `🎭️actor`) by setting `include: []` and keeping `includeSource`. **In-source suites belong in `includeSource` only.** Other packages still carry the bug (`mcp`, `shell`, 4 cad extensions, `animate` — see `📓️terra-web-kernel-package-report.md`). Corollary: a file absent from `includeSource` does not run at all while the suite still reports green, so adding a file means editing that list.

### Current verified baselines — **RE-MEASURED after the double-count fix** (measure again before trusting)

| target | baseline |
|---|---|
| `semio-framework-actor` (test) | **60 passed / 0 failed** |
| `semio-framework-plugin-host --lib` | **86 passed / 0 failed / 1 ignored** |
| `semio-framework-plugin --lib` | 242-ish passed, **5 known failures** — 4 fail in isolation (pre-existing), `a_child_survives_…channel_frames` passes alone (global-state interference). Compare NAMED SETS, not counts |
| `semio-framework-async` (test) | **16 passed / 0 failed** |
| `semio-framework-os-services` (test) | **26 passed / 0 failed** |
| `🧰️framework/📦️packages/🟦️typescript` | **87 passed** (was reported 174 pre-fix) |
| `🎭️actor/📦️packages/🟦️typescript` | **29 passed** (was reported 58 pre-fix) |
| `🎠️kernel/📦️packages/🟦️typescript` | **29 passed** — NEW package; these tests were in no gate at all before |
| `💻️os/📦️packages/🟦️typescript` | **184 passed / 2 failed** (was reported 370/2 pre-fix). The 2 failures are **two DISTINCT pre-existing** Rust-fixture/wasm tests, not one doubled: `🟦️component.ts` → `matches the Rust plan_workflow … decoded via wasm`, and `🟦️backbone-worker.ts` → `decodes the Rust-generated binary wire fixtures byte-identically`. I previously mis-recorded these as a single doubled failure because I grepped for only one of the two names — a narrow grep is not a census |
| `🧑️‍💻️dev/📦️packages/🟦️typescript` | **17 passed** (was reported 34 pre-fix) |
| repo-wide `tsc --noEmit` | **19 pre-existing errors** in trinity / stdio schemas / vscode extension — routed to a separate task, not this ticket's. Exit code observed as both 1 and 2 by different runs; report what you see |

## W4 additions — measured 2026-08-18, binding on every packet

These are not advice. Each cost a packet real time this session, and several were discovered twice
because the first discovery stayed buried in a report nobody else read.

1. **`--features component-guest` is NOT a plugin-crate feature.** Plugin crates declare no
   `[features]` section at all; `component-guest` is a *dependency* feature each enables on
   `semio-framework-plugin`. Passing it to `cargo -p <plugin>` fails with "does not contain this
   feature". Found by D0, re-found by Z1 after it blocked an entire target, and present in sol's own
   `verify rust-warnings` verb until Z1 hit it.
2. **Descriptors live at the plugin OWNER ROOT**, sibling of `🛂️manifest.json` — never under
   `🤖️generated/`, which is globally gitignored and therefore cannot hold a committed artifact.
3. **A descriptor is only ratcheted after its `descriptor_is_fresh` test passes.** Emitting is safe;
   ratcheting a plugin whose declarations may still move turns the tree red for every session.
   Unratcheted descriptors still feed the generated catalog, so a stale one is a silent
   data-correctness bug — that is the trade, and it is deliberate.
4. **`[DEBUG] ` means DELETE ME.** It has been repurposed for permanent operator diagnostics
   (312+ repo-wide); a blind sweep would strip the bench's entire error reporting. Re-prefix
   permanent diagnostics; only genuinely temporary lines carry the marker.
5. **Fuel exhaustion and pooling caps surface as a bare "error while executing"** with no mention of
   fuel or of which pool. Measure, never estimate: `🗒️note`'s `describe()` alone burns ~92M fuel in
   an unoptimized wasip2 build, and wasmtime meters component instances, core instances, memories,
   tables and GC heaps from FIVE separate pools that each default to 1000.
6. **Native builds never compile `#[cfg(target_arch = "wasm32")]` code.** A signature change can
   leave the wasm bindings broken behind a green native build AND a green test suite. `verify gate`
   now compiles the actor kernel's wasm bindings for exactly this reason.
7. **A test that passes against `MockGuestRuntime` is not a test of the runtime.** Every one of the
   ten defects found this session was covered by a green `cargo check` plus mock-backed tests.
8. **Cross-packet findings must be lifted HERE or into a coordinator message the moment they are
   read.** A finding left in a packet report does not reach a sibling packet. Item 1 above is the
   proof: correct, written down, and still cost a second packet a fully blocked target.
9. **Executors: run cargo in the FOREGROUND, in one turn.** Background watchers do not survive a
   subagent turn boundary. Six packets have now lost budget to this; briefs alone do not prevent it.
10. **Prune `🎯️target-*/**/incremental/` and stale `.wasm` between plugins.** One packet reached
    84 GB before doing so; after pruning it held ~12 GB for the rest of its run.
19. **Pass an explicit long `timeout` to every build command — the Bash tool auto-backgrounds at ~120 s by default.** This, not carelessness, is the mechanism behind the wake/idle trap: three packets in one wave "chose" to background builds because the harness detached them at the default, then idled across a turn boundary where the result can never arrive. Executors: set `timeout` to the maximum (600000 ms) on every cargo command, and if a build still exceeds it, report it unrun rather than detaching. Coordinator: your `run_in_background` tasks DO survive turn boundaries and notify you — subagents' do not. That asymmetry is why acceptance runs belong to the coordinator.
20. **A `use`d WIT type is an ALIAS, not the same `TypeId`.** `wit-parser` materialises `use effects.{x}` as a fresh `TypeDef` whose kind is `TypeDefKind::Type(original)`, so genuinely-shared types compare UNEQUAL by raw id. Any schema test comparing types must resolve alias chains to their root first (`canonical_type` in `🖥️host/🧪️schema-parity/`). This produced a false "the async world copied the payload records" report — the schema was correct.
21. **A negative result from a query that cannot report its own failure is not evidence of absence.** Four times in one wave a too-narrow or silently-failing query produced a confident wrong picture: a grep that turned two distinct test failures into "one doubled"; a `find -newermt` returning nothing while `ls` showed a one-minute-old file; a `grep wit_bindgen::generate!` that missed the scale fixture's private generator and cost the bench a run; and a file-existence gate that counted `#[path = "."]` directory anchors as missing files and so could never pass. Three would have produced a wrong conclusion about ANOTHER session's work. **Where a negative would change a judgement, reproduce it with a differently-implemented tool** — shell globbing/`find`/`grep` over emoji paths have all silently under-reported; python over explicit absolute paths has not.
22. **Acceptance must run the command the CONSUMER runs, feature flags included.** `cargo build -p semio-framework-os-scale-fixture --target wasm32-wasip2` succeeded while the bench's own `--features component-guest` build failed on W0 fallout — I verified an artifact built from a code path that excluded the defect, then reported the bench unblocked. A build without the feature gating the code under test is not evidence about that code.
23. **Executors must not run acceptance builds at all when the machine is loaded — the COORDINATOR owns every build.** Five packets in one wave ended a turn idling on a detached build. The mechanism is structural, not a lapse: the Bash tool auto-backgrounds at ~120 s, a subagent's detached job cannot report across its turn boundary, and above ~20 concurrent cargo processes even the 600 s maximum timeout will not finish a wgpu build — so "run it in the foreground" stops being available and detaching looks like the only option. A coordinator's `run_in_background` task DOES survive and notify. Therefore: briefs should ask executors to **write code and reasoning**, run only cheap checks, and mark acceptance **UNRUN**; the coordinator runs the real gates and pastes the numbers. This costs nothing — the coordinator was re-running every packet's acceptance anyway, because an executor's own figure has never been accepted as evidence on this ticket.

### ⏱️ LATEST coordinator-verified baselines — supersedes the table above (2026-08-19, W5)

| target | verified |
|---|---|
| `semio-framework-actor` | **70 passed / 0 failed** (60 → 69 shard-grants → 70 interactive-isolation) |
| `semio-framework-plugin-host --lib -- --skip schema_parity` | **113 passed / 0 failed / 1 ignored** (was 115; `race_deadline` + its 2 tests were DELETED, not lost — the deadline race moved down into `StorageTicket::await_result`, and equivalent coverage now lives in `semio-framework-os-services`) |
| `semio-framework-plugin-host --lib schema_parity` | **4 passed / 0 failed** (the 3 that failed were the TEST comparing raw `TypeId`s across `use` aliases — see rule 20) |
| `semio-framework-async` | **16 / 0** · `semio-framework-os-services` **26 / 0** |
| `semio-framework-plugin --lib` | **263 passed / 5 known failures BY NAME**, and now DETERMINISTIC across repeated runs (the documented 5-vs-6 wobble is gone — see W6-A acceptance). The 5: `identities_and_locales…`, `plural_definition…`, `registry_rejects_duplicate…`, `merge_channel_commands…` (all 4 fail in isolation), plus `a_child_survives_…channel_frames` (passes alone) |
| `semio-framework-os-renderer-wgpu --lib` | **exit 0** (`--all-targets` still fails on another session's `Dock` test-module break — not ours) |
| `🧰️framework/📦️packages/🟦️typescript` **87** · `🎭️actor/…/🟦️typescript` **40** · `🎠️kernel/…/🟦️typescript` **29** · `💻️os/…/🟦️typescript` **206 / 1** · `🧑️‍💻️dev/…/🟦️typescript` **17** · react-renderer **325 / 336** (11 = exact subset of the 15-name baseline) |
| native bench, `--shards 4` | **7 of 8**; only budget 5 fails, and it is an **instrument** defect under correction — see the W5 consolidation entry |

The single remaining `💻️os` failure (`matches the Rust plan_workflow … decoded via wasm`) is **not** ambient: `pkg/semio_framework_os.js` cannot build because `RUSTFLAGS` replaces `.cargo/config.toml`'s wasm32 `getrandom_backend` cfg. Routed out-of-band. Do not re-label it "pre-existing" — that word cost this ticket two days of carrying a fixable bug.
24. **Cargo target dirs must live in the session scratchpad, NOT in the ticket folder.** As of 2026-08-19 a build with `CARGO_TARGET_DIR=<ticket>/🎯️target-*` fails with `couldn't read …/out/private.rs: Operation not permitted (os error 1)` — rustc gets EPERM on build-script output under the repo's `.🧬semio/` tree even though the file is readable from the shell (`com.apple.provenance` xattr present). Reproduced in both a fresh and a warm ticket target dir; the identical build in `/private/tmp/claude-501/…/scratchpad/target-<slug>` finishes clean. Use the scratchpad. Bonus: the ticket folder had accumulated ~20 target dirs (one at 5.1 GB) which no longer belong there at all.
25. **An atomic packet may be redirected BEFORE it starts, or allowed to FINISH — never interrupted.** A scope change does not make a half-applied atomic refactor safe. Cost of learning this on 2026-08-19: `semio-framework-os-kernel-db` left RED with 84 errors (9 db files + hub bin half-converted to async `DbFuture` traits) when the `db-trait-flip` packet was stopped mid-flight.
26. **Neither `--lib` nor `--all-targets` is a sufficient gate alone — run BOTH.** Hit from opposite directions the same day: `--lib` hid a `cfg(test)` trait impl (7 errors); `--all-targets` hid a missing *production* `tokio` `macros` feature by unifying it out of dev-dependencies. Confirmed again immediately: a green `--lib` wgpu check while `--all-targets` still had a real error.
