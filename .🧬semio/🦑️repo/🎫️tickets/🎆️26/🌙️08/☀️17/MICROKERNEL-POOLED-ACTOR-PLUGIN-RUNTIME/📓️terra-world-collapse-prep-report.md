# 🗺️ terra-world-collapse-prep — design only, NOTHING applied to the live tree

Executor: `terra` (Sonnet 5 High), packet `world-collapse-prep` inside the `wide-verification-fleet`
wave. **No file outside this ticket folder was edited.** Every claim below was re-verified by reading
`🔌️plugin/🧬️schema/📜️component.wit` fresh off disk today (2026-08-19), not inherited from
`📋️master-u.md` or `terra-s7-component-wit-diff.md` — both of which I *did* read and cross-check, and
one of which (§0 below) I found materially disagrees with the live file.

## 0. ⚠️ A correction to the plan of record — validate-your-assumptions catch

`📋️master-u.md:19` and this packet's own brief both assert **"all 37 funcs are already `async func`"**.
Reading the live schema line-by-line, **this is false today**. Of the 37 funcs in the package
(`pure`×3, `reactor`×1, `jobs`×3, `checkpoint`×2, `describe`×1, `host-async`×26, `runner`×1), exactly
**12 are still plain `func`**:

| interface | plain `func` today |
|---|---|
| `pure` | `log`, `now-ms`, `trace-span` (:824-826) |
| `reactor` | `poll` (:863) |
| `jobs` | `start-job`, `step-job`, `cancel-job` (:992-994) |
| `checkpoint` | `checkpoint`, `restore` (:1006-1007) |
| `describe` | `describe` (:1019) |
| `host-async` | `emit`, `emit-patch` (:951-952) — deliberately, the two fire-and-forget doors |

The 25 that ARE `async func` today are `host-async`'s other 24 imports + `runner::run`. So the
diff below is not a rename-only exercise — **7 of `world actor`'s own future exports (poll, 3× jobs,
2× checkpoint, describe) must gain the literal `async` keyword**, which is the actual mechanical
content of "collapse," not a side effect of it.

This also resolves an ambiguity in this packet's own brief: it says
`emit_carries_the_whole_effect_variant` "asserts emit is NOT async, which the schema now contradicts."
**As read today, nothing contradicts it** — `emit`/`emit-patch` are still plain `func`, and nothing in
the B1 design (`📋️master-u.md:40-47`) proposes changing that; they stay the deliberate one-way doors.
I could not find a real basis for "the schema now contradicts" it and I am not asserting one — I
flag it as either stale phrasing carried over from an earlier design iteration, or shorthand for "the
FILE-LEVEL doc comment framing (`emit is the exception in a mostly-sync-vs-async split between two
worlds`) goes stale," which is true and is exactly what my re-spec in §3 handles. The re-spec I propose
does **not** make emit/emit-patch async.

## 1. Exact diff to `component.wit`

All line numbers are against the file as read today; **re-check them before applying** — six sibling
packets are mid-flight elsewhere in the schema's own file today per `important.md`'s live-peer table
(though none of that table's rows touch this file — the peer's contended files are `world.wit`/
`🦀️component.rs`/`🟦️component.ts`, a different WIT package under `🎠️kernel`, not this one).

### 1a. `interface pure`'s doc comment (:818-822) — becomes false, must drop "the ONLY interface"

```diff
-/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §1): the ONLY interface `world
-/// actor` ever imports. Everything else the guest needs from the host — reads, writes, network,
-/// dialogs, jobs — is an `effect` returned from `poll`, answered by an `event` on a later `poll`
-/// call. Keeping the import surface to these three side-effect-free/idempotent calls is what lets
-/// a pooled multi-instance actor stay `Send`-free and reentrancy-safe.
+/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2 → B1 world-collapse). One of `world actor`'s two
+/// imports (the other is `host-async`) — these three side-effect-free/idempotent calls are the only
+/// ones a guest may call SYNCHRONOUSLY; every stateful host operation goes through `host-async`'s
+/// awaitable imports instead (design-abi.md §1's original poll/effect/event round trip is now the
+/// pattern `reactor::poll` uses INTERNALLY when it awaits a `host-async` import mid-turn, not a
+/// separate calling convention).
```

The three `pure` funcs themselves (`log`/`now-ms`/`trace-span`, :824-826) are **unchanged** — still
plain `func`, deliberately (no I/O, no suspension point, E1-style: nothing to await).

### 1b. `interface reactor` (:829-864) — `poll` gains `async`, doc comment's "avoids stackful-async" claim goes false

```diff
 /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §1). The one turn-loop entry
 /// point every actor exports — replaces `exchange`, the poll-backbone + refresh-ui heartbeat, and
 /// every per-verb surface (`handle-action`, `handle-command`, `update-window`, `refresh-ui`,
 /// `context-menu`, `apply-mutations[-text]`, `read/load-app-document-{text,pack}`,
 /// `attach/detach-backbone`, `consume/produce-media`). Deliberately not dependent on stackful-
 /// async component-model features (still incomplete in wasmtime) — the reactor/effect shape gets
 /// the same async ergonomics without needing them. Rust SSOT for `budget`/`turn-status`/
 /// `turn-result`: `semio_framework::kernel::{Budget, TurnResult}` (packet A3).
+///
+/// 🧬️ B1 world-collapse: `poll` itself is now `async func` — S7 (terra-probe-spikes, reproduced)
+/// found a plain sync `func` EXPORT is uncallable at all on any Store with
+/// `wasm_component_model_async(true)`, and `world actor` now imports `host-async`, which requires
+/// exactly such a Store. The "avoids stackful-async" sentence above is retired by this: `poll`
+/// depends on component-model-async now, same as every other export in this world. What it still
+/// avoids is a SECOND long-lived call shape (`runner::run`'s `stream<event>`) — `poll` stays one
+/// call in, one `turn-result` out, it just runs on an async-capable Store like everything else here.
 interface reactor {
   use types.{plugin-error};
   use effects.{effect};
   use events.{event};
   use ui.{ui-patch};
   ...
-  poll: func(events: list<event>, budget: budget) -> result<turn-result, plugin-error>;
+  poll: async func(events: list<event>, budget: budget) -> result<turn-result, plugin-error>;
 }
```

### 1c. `interface jobs` (:980-995) — all three funcs gain `async`

```diff
 interface jobs {
   use types.{plugin-error};
   ...
-  start-job: func(job: u64, kind: string, input: list<u8>) -> result<_, plugin-error>;
-  step-job: func(job: u64, budget: job-budget) -> result<job-step, plugin-error>;
-  cancel-job: func(job: u64);
+  start-job: async func(job: u64, kind: string, input: list<u8>) -> result<_, plugin-error>;
+  step-job: async func(job: u64, budget: job-budget) -> result<job-step, plugin-error>;
+  cancel-job: async func(job: u64);
 }
```

No field/type changes — `job-budget`/`job-step` stay defined right here, NOT hoisted into `types`.
(`terra-s7-component-wit-diff.md` proposed hoisting them into `interface types` so a sync `jobs` and
an async `jobs-async` could share the type via `use`. That hoist was solving a problem this design no
longer has — there is only one `jobs` interface now, so there is nothing to share the type WITH. Do
not apply that diff; it is superseded by `📋️master-u.md`'s later B1 decision.)

### 1d. `interface checkpoint` (:1003-1008) — both funcs gain `async`

```diff
 interface checkpoint {
   use types.{plugin-error};
-  checkpoint: func() -> result<list<u8>, plugin-error>;
-  restore: func(state: list<u8>) -> result<_, plugin-error>;
+  checkpoint: async func() -> result<list<u8>, plugin-error>;
+  restore: async func(state: list<u8>) -> result<_, plugin-error>;
 }
```

### 1e. `interface describe` (:1018-1020) — gains `async`

```diff
 interface describe {
-  describe: func() -> list<u8>;
+  describe: async func() -> list<u8>;
 }
```

### 1f. `interface host-async` (:866-953) — NO functional change, doc comment needs one edit

All 26 funcs, all types, unchanged verbatim. Only the header doc (:866-886) references "the async
counterpart" / implies a second world exists; retarget it at "the world's async import surface"
(one-line edit, not reproduced in full here — low risk, cosmetic).

### 1g. Delete `interface runner` in full (:955-966, including its 6-line doc comment and the blank
line after it)

```diff
-/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (async-worlds). `world actor-async`'s turn-loop entry
-/// point — the host writes `event`s into the stream and the guest consumes until the host closes
-/// it, replacing `world actor`'s `reactor::poll` call/response loop with one long-lived async call.
-/// UI patches and effects travel the OTHER direction through `host-async`'s `emit`/`emit-patch` (or
-/// a direct `host-async` async import) as the guest produces them, rather than being collected into
-/// a `turn-result` and returned — there is no `turn-result` in this world.
-interface runner {
-  use types.{plugin-error};
-  use events.{event};
-
-  run: async func(events: stream<event>) -> result<_, plugin-error>;
-}
-
```

### 1h. `world actor` (:1022-1035) — doc comment refreshed, gains `import host-async;`

```diff
 /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §1). One world for both roles —
 /// a plugin and an extension are the same wasm component contract now; `describe()`'s
 /// `PackageDescriptor.role` (packet E1) is what tells the host which one it loaded, not a
 /// different WIT world. Deleted from the old split-role design: `plugin-world`, `extension-world`,
 /// the `contributor`/`host` interfaces, and the `manifest`/`instantiate-app`/`exchange`/
 /// `clear-instance-guard`/`activate`/`deactivate`/`invoke` exports — see `types.wit`/`effects.wit`/
 /// `events.wit`/`jobs.wit`/`describe.wit` for where each one's job went.
+///
+/// 🧬️ B1 world-collapse (📋️master-u.md §B1): as of this packet ONE world for both the poll-loop AND
+/// the async-import surface — `interface runner` and `world actor-async` are DELETED, not merely
+/// unreferenced. Every export below is now `async func` (S7's categorical finding: a sync export is
+/// uncallable on the async-configured Store this world's `host-async` import now requires). This
+/// world is no longer a "sync/poll compatibility backend" for anything — it is the only backend.
 world actor {
   import pure;
+  import host-async;
   export reactor;
   export jobs;
   export checkpoint;
   export describe;
 }
```

### 1i. Delete `world actor-async` in full (:1037-1051, doc comment + body)

```diff
-/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (async-worlds). WASI 0.3 async counterpart to `world
-/// actor` above — every record/variant is shared verbatim between the two worlds, only the
-/// turn-loop shape and the completable-effect calling convention differ (`reactor::poll`'s
-/// call/response loop vs. `runner::run`'s long-lived `stream<event>` plus `host-async`'s awaitable
-/// imports). `world actor` itself is UNCHANGED by this world's existence — it stays the
-/// compatibility backend for web/jco and the fallback everywhere a wasmtime async host is not
-/// available.
-world actor-async {
-  import pure;
-  import host-async;
-  export runner;
-  export jobs;
-  export checkpoint;
-  export describe;
-}
```

This is the doc line the packet brief specifically calls out ("the one claiming `actor` is the
sync/poll compatibility backend") — confirmed, it is line 1042 above, and it dies with the whole block.

### 1j. Cosmetic-only, lower priority: three more doc comments say "pure is the only import"

`interface capabilities` (:125), `interface documents` — no, only `capabilities` and `ui` actually say
it — `interface ui` (:770-771, "`pure` is the only import, so a `resource` here would have no
host-callable methods"). Both explain why `capability-token`/`surface` are plain records instead of
WIT `resource`s. **The underlying reasoning still holds** — `capabilities`/`ui` are still never
themselves imported or exported by the world, only referenced for their types — only the phrase "pure
is the only import" needs to become "pure and host-async are the only imports, and neither IS
capabilities/ui." Not required for correctness, only for the doc comments to stay true; low risk,
can be folded into the same edit pass or deferred.

## 2. Every consumer that must change

### 2a. Guest SDK — TWO `wit_bindgen::generate!` sites, confirmed both, both in the guest SDK

1. `🔌️plugin/🦀️component.rs:18-21` — `generate!({ world: "actor", path: "../../🧬️schema" })`, gated
   `component-guest`/`component-extension-guest`. This is the EXPORT side (`ComponentGuest` implements
   `ReactorGuest`/`JobsGuest`/`CheckpointGuest`/`DescribeGuest`, `export!(ComponentGuest)` at :78).
   **Every trait method wit-bindgen generates for these `Guest` traits becomes `async fn`** once the
   schema changes land (wit_bindgen mirrors `async func` as an `async fn` on the generated `Guest`
   trait). `impl ReactorGuest for ComponentGuest { fn poll(...) }` (:33) and the 6 other impl methods
   at :40-76 all need the `fn` → `async fn` edit, and their bodies (which currently call sync
   `crate::reactor::poll`/`jobs::start_job` etc.) need to `.await` those calls once THEY become async
   too (already `pub async fn` bodies in most cases per the SDK's own universal-async conversion —
   this is `sdk-dedyn`'s scope, not this packet's, flagged here only so the coupling is visible).
2. `🌐host/🦀️component.rs:60-63` (inside `pub mod direct`, gated `component-guest-async`) —
   `generate!({ world: "actor-async", path: "../../🧬️schema" })`. **`world: "actor-async"` no longer
   exists — this call fails to resolve the schema at all** once §1i lands. This whole `mod direct`
   (:47-≈300, everything gated on `component-guest-async`, built specifically because `world actor`
   used to import ONLY `pure` and could not reach `host-async`) becomes **redundant**, not merely
   broken: `world: "actor"`'s own single `generate!` in `🔌️plugin/🦀️component.rs` now ALSO generates
   bindings for `host-async` (since `world actor` now imports it), so there is no longer a reason for
   a second, separately-gated `generate!` call at all. The header doc's own framing (:1-23, "the
   Direct arm ... only constructible with `component-guest-async` on ... a landing pad, not a live
   path") already flags this as unfinished scaffolding — the collapse is the natural point to delete
   `mod direct` entirely and fold its intent (host-async calls bypassing the `RequestRegistry`/`poll`
   round trip) into how the SDK's OWN async fns now call `semio::framework::host_async::*` directly,
   generated once, from the one `generate!` call. This is a real design simplification the collapse
   unlocks, not just cleanup — flagging it for whichever packet (`sdk-dedyn` or `host-dedyn`,
   per `📋️master-u.md`'s packet table) owns `🌐host/🦀️component.rs`.

### 2b. Host — TWO `wasmtime::component::bindgen!` sites, both need attention, one is the highest-risk item in this whole packet

1. `🖥️host/🦀️component.rs:799-810` (`mod actor_bindings`, inside `WasmtimeRuntime`) —
   `bindgen!({ world: "actor", path: "../../../🧬️schema", additional_derives: [Clone] })`.
   Three separate consequences:
   - The world now needs `wasm_component_model_async(true)` on its `Engine`/`Config` — currently
     `build_shared_engine`/`SharedEngineConfig` builds a **sync-only** engine (I did not find
     `wasm_component_model_async` set anywhere in `SharedWasmtimeEngine`'s region, :1-364 of this
     file). `execute_turn` (:930-≈960) calls `state.bindings.semio_framework_reactor().call_poll(&mut
     state.store, ...)` — the plain, non-`_async` accessor. Once `poll` is `async func`, this exact
     call becomes the S7 failure mode ("store configuration requires that *_async functions are used
     instead"), because it is BOTH a sync-shaped call AND (once host-async lands) a Store that must be
     async-configured for `host-async`'s own imports to link at all.
   - **`additional_derives: [Clone]` cannot survive the merge.** `⏳️imports.rs:37-42`'s own doc
     comment already states why its SEPARATE `host_async_bindings` module does NOT request `Clone`:
     `host-async` carries `stream<u8>` (`blob-read`, `http-fetch`'s `http-response.body`), which lowers
     to `StreamReader<u8>` — "a one-shot resource handle that is deliberately not `Clone`," and
     `additional_derives` applies to every generated type in the `generate!`/`bindgen!` call, not
     selectively. Once `world actor` imports `host-async`, THE SAME merged bindgen call carries both
     the poll-world's plain records (currently `Clone`-derived, likely relied on somewhere in
     `🖥️host/🦀️component.rs`'s own turn/effect conversion code) and the new stream-carrying types.
     Whoever applies this diff must either (a) drop `additional_derives: [Clone]` and hunt down every
     `.clone()` call site on a `wit_*` type this file's `execute_turn`/`kernel_event_to_wit`/
     `kernel_effect_to_wit` functions currently rely on, replacing each with an explicit field-by-field
     rebuild, or (b) confirm (I did not have budget to trace every call site) that nothing actually
     needs it and the derive was defensive. **This is the single most concrete "this will not just
     recompile, it needs real engineering" finding in this report** — flag prominently for whichever
     packet applies B2 (`async-plugin-runtime` per the packet table, or `host-dedyn` if it lands first).
   - Practically, this means `WasmtimeRuntime`'s ENTIRE `execute_turn`/`instantiate` pair (the real,
     non-mock guest runtime, :866-≈960) needs to become the async-Store, `Accessor`/`run_concurrent`-
     shaped design `⏳️runtime.rs` already prototyped (§2c below) — not a small signature tweak. This
     is not explicitly named as a "consumer" in this packet's brief, but it is the largest one I found;
     the brief's four named consumers (SDK ×2, `imports.rs`, `runtime.rs`, describe bin, jco) do not
     include it, and I think that is an omission worth surfacing to the coordinator rather than
     silently leaving out of this report.
2. `⏳️imports.rs:45-48` — `bindgen!({ world: "actor-async", ... })`. Same "world no longer exists"
   failure as §2a's guest-side `mod direct`. Per `📋️master-u.md`'s B2 plan, this file's `AsyncActorHostState`
   + its 24 `HostWithStore` impls (:491-810) are reused **verbatim** as the `host-async` import half —
   only the `world:` string and the fact that it now shares ONE `bindgen!` call with `reactor`/`jobs`/
   `checkpoint`/`describe` (rather than being its own separate module) changes. Whether this becomes a
   THIRD bindgen module merged into `actor_bindings`, or whether `actor_bindings` itself is deleted and
   this file's `host_async_bindings` module is renamed and extended to cover the turn-loop exports too,
   is an open call for whoever applies this — I recommend the latter (one bindgen call, one module,
   matching wasmtime's own "generate everything a world needs from one macro invocation" idiom) but
   did not verify wasmtime accepts `additional_derives` differently per-type, which would change the
   calculus in §2b.1.

### 2c. `⏳️runtime.rs` — unmounted, was written against interfaces that were never created

Confirmed directly (I reread the file, not just its own report): `🖥️host/⏳️runtime.rs:421` calls
`instance.semio_framework_runner().call_run(accessor, events)` — the `runner` interface, which dies in
§1g. Its own report (`terra-async-runtime-report.md` §"checkpoint / jobs — schema history mid-packet")
already documents that its `AsyncActorCommand::{Checkpoint, Restore, StartJob, StepJob, CancelJob}`
dispatch was written against PREDICTED bindings for `jobs-async`/`checkpoint-async` — interfaces from
`terra-s7-component-wit-diff.md`'s proposal, which §1c/§1d of THIS report supersede (no `-async`
suffix; the existing `jobs`/`checkpoint` interface names go async in place). So `runtime.rs` needs,
at minimum: (a) delete the `call_run`/`GrantWindow`/`GrantedEventProducer`/`synthesize_turn_result`
machinery built around `runner::run`'s `stream<event>` shape (nothing in the collapsed world has a
long-lived stream entry point — `poll` stays call-in/result-out), (b) replace it with a command-channel
loop that calls `reactor().call_poll_async(...)`-shaped bindings once per `Poll` command instead, (c)
fix every `instance.semio_framework_jobs_async()`/`semio_framework_checkpoint_async()` accessor name
to the real generated names for `jobs`/`checkpoint` (no `_async` suffix — wit-bindgen names accessors
after the WIT interface name, not its async-ness). This is a near-total rewrite of the file's control
flow, not a rename — `📋️master-u.md §B2` already says as much ("Rewrite: bindgen against the collapsed
`world actor`... the draft's `jobs-async`/`checkpoint-async` predictions are dead").

### 2d. `📇️describe/📦️packages/🦀️rust/📦️glue.rs` — confirmed cannot describe an async component

Read in full (:119-137). `describe_component` builds `wasmtime::Config::new()` with only
`consume_fuel(true)` — no `wasm_component_model_async`, no `async_support` — then calls
`bindings.semio_framework_describe().call_describe(&mut store)`, the plain sync accessor. Once
`describe::describe` is `async func` (§1e) AND `world actor` links `host-async` (§1h), this Config
cannot even LINK the component (host-async's `Host`/`HostWithStore` impls are never satisfied — this
crate has none), let alone call `describe()`. `📋️master-u.md §B3` already specs the fix: Config gains
`wasm_component_model_async`, the linker keeps `pure` and adds a **stub** `host-async` impl (all 24
methods return an error — "describe must be pure," since a real plugin's `describe()` legitimately
should never await a host effect), and the call becomes `instantiate_async` + `call_describe(...)
.await` on the crate's own `current_thread` runtime (already an R4-sanctioned `block_on`/tokio site —
this is a build-time-only bin, category 1 of the allow-list). I did not verify the exact wasmtime API
names for the async instantiate/call path beyond what `⏳️runtime.rs`'s own proven harness already
demonstrates (`AsyncEngineHandle`, `Accessor`-based calls) — the packet applying this should reuse that
harness's already-proven primitives rather than re-deriving them.

### 2e. jco bridge generator — GOOD NEWS: already built assuming the target shape

`📦️packages/🟦️typescript/🌐plugin-web-materialize.ts` (`transpilePluginComponent`, :431-449, and its
async twin :524-535) **already** passes `--map semio:framework/host-async=./🟨️host-shim.js` alongside
`--map semio:framework/pure=./🟨️host-shim.js`, and its own doc comment (:435-437) states plainly:
`world actor`'s import surface is now `pure` ... PLUS `host-async`" — describing the POST-collapse
shape, today, before the schema itself has changed. Likewise `pluginComponentBridgeSource` (:359-379)
destructures `{ reactor, jobs, checkpoint, describe }` and wraps every one of them in an explicit
`async (...) => ...`, with its own doc comment (:350-352) noting "every WIT function in the target
world is `async func`." This was landed by an EARLIER packet (`terra-web-bridges`, per its doc
attribution) written **against the target design**, not the live schema. Two implications:
- **This file needs no further change** for the collapse itself — it is already correct for the
  post-collapse shape, and (per its own doc, :83-85 and :432-438) already confirmed via a real
  transpile-diff spike that jco's JS glue calls `WebAssembly.Suspending`/`promising` unconditionally
  regardless of `--async-mode`, for a component whose every WIT function is already `async func` —
  meaning jco's OWN behavior is not sensitive to whether `poll`/`jobs`/`checkpoint`/`describe` were
  sync or async before this packet; it treats every export as async-capable either way. This
  substantially de-risks the web side of the collapse relative to the native/host side (§2b).
- The inverse risk: because this file currently maps `host-async` unconditionally, and TODAY's real
  wasm builds (world `actor`, `pure`-only) don't actually export/import `host-async` at all, the
  `--map` flag is presently inert (jco silently ignores a `--map` for an interface absent from the
  artifact). **Nobody has verified this file against a REAL component built from the collapsed
  schema** — only against the pre-collapse `world actor` (sync, `pure`-only) and, per its own honest
  gaps section (:554-561), against the standalone `jcoprobe` spike fixture, never a real `s` plugin.
  That first real transpile is exactly where `📋️master-u.md`'s B4/`web-bridges` packet's own risk
  register item lives (jco async exports, "the one genuinely external risk").

### 2f. Downstream ripple not named in this packet's brief, worth lifting now

`⚛️reactor/💼️jobs/🦀️component.rs`'s own doc comment (:22-32, "Host-await restriction (deliberate,
v1)") explains that `JobCtx::host()` — letting a job's async body `.await` a `host-async` import — is
gated `#[cfg(feature = "component-guest-async")]` and **must never be ungated for `world actor`**,
because the poll-world's `run_job_to_completion` relay (`🖥️host/🦀️component.rs`) loops
`start_job`→`step_job`* WITHOUT ever re-entering `poll` in between, so a job that awaited a host effect
there would park on a future that can only resolve inside a `poll` call that never comes again — an
infinite `Running` spin. Once `world actor` is the ONLY world and its jobs execute inside the same
async-`Store`/`Accessor` machinery `⏳️runtime.rs`/§2c describe (command-channel `StepJob`, not a
poll-call relay), **this rationale for the gate may no longer hold** — but I did not verify this
either way (whether the collapsed `run_job_to_completion`-equivalent re-pumps in a way that lets a
parked host-await resolve). This is exactly the kind of decision R9/R10-adjacent rules want made WITH
evidence, not assumed; flagging it as a real open design question for whichever packet (`sdk-dedyn` or
`async-plugin-runtime`) ends up touching `JobCtx::host()`'s gate, not resolving it here.

## 3. Schema-parity test re-spec (`🖥️host/🧪️schema-parity/🦀️component.rs`)

Full file read (330 lines, 4 tests). What survives unchanged, what breaks outright, and the proposed
replacement, test by test:

- **`every_req_bearing_effect_has_a_matching_host_async_import`** (:126-199) — **unaffected**. It only
  inspects `interface effects` and `interface host-async`, neither of which changes shape. Keep as-is.
- **`spawn_job_has_a_matching_host_async_import_despite_carrying_no_req`** (:206-229) — **unaffected**,
  same reason. Keep as-is.
- **`emit_carries_the_whole_effect_variant`** (:236-252) — **unaffected in its actual assertions**
  (`emit.kind == Freestanding`, i.e. NOT async, still true per §0). Only its FILE-LEVEL framing changes
  (the header doc at :1-11 describes "contract-parity between `world actor`'s poll-based effect variant
  and `world actor-async`'s awaitable host-async imports" — two worlds — which becomes false prose,
  not a false assertion). Fold the file header rewrite into this packet's doc-comment pass; no
  assertion-level change needed here.
- **`both_worlds_share_the_same_export_surface_and_actor_is_untouched`** (:268-329) — **breaks outright
  and must be replaced**, not patched: `fixture.world("actor-async")` (:271) panics
  (`"world actor-async must exist in 🧬️schema/📜️component.wit"`) the instant the world is deleted, so
  this test cannot even reach an assertion. Delete it. Replace with:

  1. **`exactly_one_world_exists`** — `resolve.packages[package].worlds.len() == 1` and its name is
     `"actor"`. This is the direct, load-bearing replacement for what the deleted test's name promised
     ("actor is untouched") — except now the claim is stronger: `actor` doesn't just survive, it's the
     only world, which is the entire point of the collapse. A future packet that accidentally
     resurrects a second world (or a stray probe/spike world left in the live schema by mistake — this
     ticket has that failure mode on record, `terra-s7-component-wit-diff.md` proposed exactly such an
     addition before B1 superseded it) fails this test immediately.
  2. **`world_actor_exports_and_imports_exactly`** — reuses `export_names`/`functional_import_names`
     from the deleted test (both are pure helper closures, keep them) and asserts
     `export_names(actor) == {"reactor","jobs","checkpoint","describe"}` (unchanged) and
     `functional_import_names(actor) == {"pure","host-async"}` (the actual functional change — was
     `{"pure"}` alone).
  3. **`every_export_of_world_actor_is_async_func`** — NEW, the direct test of §0/§1's mechanical
     content. Walks `actor`'s four exported interfaces (`reactor`, `jobs`, `checkpoint`, `describe`)
     and asserts every one of their functions has `FunctionKind::AsyncFreestanding`. Scoped
     DELIBERATELY to exports-of-`world-actor` only, not "every func in the package" — `pure`'s 3 funcs
     and `host-async`'s `emit`/`emit-patch` stay plain `func` by design (§0/§1a/§1f), and a
     package-wide "every func is async" assertion would be actively wrong, failing on those 5 forever.
     This is the assertion this packet's brief asked for ("every func is async func") — I am
     proposing it scoped correctly rather than literally, because the literal reading contradicts
     the very emit/emit-patch exception the SAME brief asks to preserve two sentences later.
  4. **`every_fallible_host_async_import_returns_a_result`** — NEW, the assertion this packet's brief
     asked for from the real finding in §0's verification: for every function in `host-async` EXCEPT
     the two documented exceptions (`emit`, `emit-patch` — assert their absence from `results` instead,
     i.e. they return nothing), assert `function.results` is exactly one type and that type's
     `TypeDefKind` (through `canonical_type`, reusing the existing alias-resolution helper) is
     `TypeDefKind::Result(_)`. I independently verified this holds for all 24 today (every one of
     `storage-read/write/delete`, `blob-load/write/read`, `http-fetch`, `document-read/write`,
     `link-resolve`, `registry-query`, `io-compose/run`, `cache-derive/read`, `invoke-extension`,
     `open-window/dialog`, `dispatch-action`, `spawn-plugin-instance`, `request-file-open/
     media-frames/capability`, `spawn-job` returns `result<T, pack>` for some `T`) — this test
     encodes that finding permanently rather than leaving it as a one-time manual check. The
     motivating bug this guards against, per the brief: a future `host-async` import declared with a
     bare return type would let a real host-side fault silently decode as that type's Rust default
     (e.g. an empty `Vec<u8>` instead of a propagated error) instead of failing loud — this test makes
     that a compile-time-adjacent (test-time) failure instead of a runtime data-correctness bug.

None of the four new/kept tests need the `Function`/`FunctionKind`/`TypeDefKind` imports to change —
`wit_parser` already exposes everything needed (`FunctionKind::AsyncFreestanding` is already used at
:160/:223, `TypeDefKind::Result` is a plain enum variant on the same `TypeDefKind` the file already
matches on at :53/:70/:113). No new dependency.

## 4. Ordered application plan, risks called out

This packet does not apply anything — the plan below is for whichever packet (`world-collapse` per
`📋️master-u.md`'s packet table, owner **sol**, ATOMIC registrar) does.

1. **Confirm `sdk-dedyn` (the packet this one depends on, per the packet table) has actually landed**
   before touching the schema — `world-collapse` is itself gated on it, and per rule 25 an atomic
   packet redirected mid-flight is exactly this ticket's worst-recorded failure mode (84 errors, once).
2. **Apply §1a-1j to `component.wit` in one edit** (it's one file, ~40 changed lines across 9 spots —
   doing it as N separate edits risks an intermediate state where the schema parses but is
   self-contradictory, e.g. `poll` async but `world actor` not yet importing `host-async`, which
   would make `poll`'s own internal await-a-host-import calls unlinkable).
3. **Apply the schema-parity re-spec (§3) in the SAME edit turn**, so `cargo test -p
   semio-framework-plugin-host --lib schema_parity` is the first acceptance signal — it needs no wasm
   build, no bindgen regeneration, just `wit-parser` reading the new file, so it is the cheapest
   possible check that the WIT diff itself is self-consistent before touching any Rust bindgen site.
4. **Delete `🌐host/🦀️component.rs`'s `mod direct` (§2a.2)** — this can happen in the same packet
   since it is pure deletion (no new code to write), and removes a `generate!({world: "actor-async"})`
   call that would otherwise hard-fail to resolve the instant the schema changes, breaking every build
   with `component-guest-async` on even before anyone gets to fix it properly.
5. **Fix the two `bindgen!`/`generate!` sites still targeting `world: "actor"` by NAME** (§2a.1,
   §2b.1) — these don't need a `world:` string change (the name survives), but DO need the
   `additional_derives: [Clone]` question (§2b.1) resolved BEFORE the next step, since it changes
   which follow-on call sites break.
6. **`⏳️imports.rs`'s bindgen (§2b.2) and `⏳️runtime.rs`'s rewrite (§2c) are the largest remaining
   work** and are explicitly a SEPARATE packet in the table (`async-plugin-runtime`) — do not fold
   them into the atomic `world-collapse` packet itself; `world-collapse`'s own job is the schema +
   parity-test + the two now-broken `generate!`/`bindgen!` `world:` targets, matching its `path_scope`
   in the packet table exactly (`🧬️schema/📜️component.wit`, `🧪️schema-parity/🦀️component.rs`,
   "export-macro glue" — which I read as §2a/§2b's `generate!`/`bindgen!` calls themselves, not the
   hundreds of lines of runtime logic behind them).
7. **`describe-async` (§2d) and `host-dedyn`'s `WasmtimeRuntime::execute_turn` rewrite (§2b.1's third
   bullet) both depend on `world-collapse` landing** but are independent of each other and of
   `async-plugin-runtime` — can run in parallel once the schema is in.
8. **Web (§2e) needs no code change but DOES need a real verification run** once a plugin is actually
   rebuilt against the collapsed schema — this is `web-bridges`' job per the table, already sequenced
   after `fleet-wasm-descriptors`.

### Risks, ranked

1. **§2b.1's `additional_derives: [Clone]` loss** — highest-confidence concrete breakage I found that
   is NOT already named in this packet's brief. Could silently compile-fail dozens of call sites in
   `🖥️host/🦀️component.rs`'s turn/effect conversion code the moment the two bindgen calls merge, or
   could turn out to affect nothing if those conversions never actually clone a `wit_*` type — I did
   not have budget to trace every call site and did not want to guess either way in this report.
   Whoever applies §2b.1 should grep `🖥️host/🦀️component.rs` for `.clone()` on any `wit_*`/
   `wit_effects::`/`wit_events::`/`wit_ui::`-prefixed value BEFORE merging the bindgen calls, not after.
2. **§2c (`⏳️runtime.rs`) is a near-total rewrite, not a patch** — its own report already says as
   much for the `jobs-async`/`checkpoint-async` naming; this report adds that its ENTIRE `call_run`/
   stream-producer/grant-window machinery (the majority of the file, per its own region list) is now
   built around an entry point (`runner::run`) that no longer exists in any form, not renamed. Budget
   this as a rewrite, not a find-and-replace, when sizing `async-plugin-runtime`.
3. **§2f (`JobCtx::host()`'s gate)** — an open design question, not a known-broken site; could be a
   one-line `#[cfg]` relaxation or could require real interleaving work in the jobs executor. Flagging
   so it doesn't get silently forgotten (it will not show up as a compile error — the gate currently
   compiles fine either way; it is a latent behavior question, exactly the kind of thing R9's "must
   validate assumptions" rule exists for).
4. **§2e's untested assumption** — jco's own `--async-mode` insensitivity (§2e) was proven against
   the standalone `jcoprobe` fixture and the PRE-collapse `world actor`, never against a real plugin
   built from the collapsed schema. Low risk given the evidence so far, but "the one genuinely external
   risk" per `📋️master-u.md`'s own risk register — worth a real transpile of one real (small) plugin
   as the FIRST acceptance step of `web-bridges`, before re-materializing all 48 bridges.
5. **Schema-parity re-spec correctness (§3)** — self-checking: if I mis-specified
   `every_export_of_world_actor_is_async_func`'s scope, the test itself will fail loudly against the
   real (already-changed) schema the moment it's applied, rather than silently passing on a wrong
   premise — this is why I scoped it to exports-of-`world-actor` with the reasoning spelled out (§3.3)
   rather than the literal "every func" reading, which would fail immediately and for the wrong reason.

## Files read (none written outside this ticket folder)

`🔌️plugin/🧬️schema/📜️component.wit` (full, 1052 lines) · `🔌️plugin/🦀️component.rs` (:1-120) ·
`🔌️plugin/🌐host/🦀️component.rs` (:1-100) · `🔌️plugin/🖥️host/🦀️component.rs` (:355-960) ·
`🔌️plugin/🖥️host/⏳️imports.rs` (:1-60) · `🔌️plugin/🖥️host/🧪️schema-parity/🦀️component.rs` (full,
330 lines) · `🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs` (full, 264 lines) ·
`🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts` (:1-100, :340-600) ·
`⚛️reactor/💼️jobs/🦀️component.rs` (:1-40) · `⚛️reactor/🦀️component.rs` (:295-320, :515-535) ·
`important.md`, `📋️master-u.md`, `terra-s7-component-wit-diff.md`, `terra-async-runtime-report.md`
(this ticket folder) · `📓️status.md` (:5150-5275).

## Files written

Only this file: `TICKET_DIR/📓️terra-world-collapse-prep-report.md`. No schema, no Rust, no TS file
was edited. No lease requested — everything I needed to read was readable; nothing needed writing
outside this report.
