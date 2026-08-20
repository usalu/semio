# 📓️ terra-world-collapse — report

Packet `world-collapse` (ATOMIC, sole live packet). **Landed in full.** The plugin ABI is now ONE
async world. Every acceptance gate ran in the foreground this turn; exit codes and counts below are
pasted from the real runs, not inferred.

---

## 0. Verdict

| what | state |
|---|---|
| `🧬️schema/📜️component.wit` collapsed to one world | ✅ done |
| `interface runner` + `world actor-async` deleted | ✅ done (`grep` proof §5) |
| all 7 `world actor` exports are `async func` | ✅ done |
| `world actor` imports `pure` + `host-async` | ✅ done |
| guest SDK (`🔌️plugin/**`) compiles + exports the async ABI | ✅ done, incl. a REAL `.wasm` (§6) |
| host (`🖥️host/**`) links + calls the async world | ✅ done |
| schema-parity test re-spec'd | ✅ done, 7/7 pass |
| every baseline regression gate | ✅ unregressed (§5) |
| **residue** | 2 items, both named and bounded (§7) — neither is a broken build |

No half-collapsed state. Nothing in the repo still references a world that does not exist except
`⏳️runtime.rs`, which is **not mounted** and now carries a loud `⛔️ SUPERSEDED` header saying exactly
what a future packet must do to it.

---

## 1. Schema (`🔌️plugin/🧬️schema/📜️component.wit`)

Applied the prep report's §1 diff, re-verified line-by-line against the file as it was today (line
numbers had drifted ~+9 from the prep report; the CONTENT matched exactly). The SUPERSEDED
`job-budget`/`job-step` hoist into `interface types` was **not** applied, as instructed.

| § | change |
|---|---|
| 1a | `interface pure`'s "the ONLY interface `world actor` ever imports" doc rewritten — it is now one of two, and the reason the three funcs stay sync is restated (no I/O, no suspension point) |
| 1b | `reactor::poll` → `async func`; the "deliberately not dependent on stackful-async" paragraph RETIRED in place with the S7 reasoning, and what `poll` still avoids (a second long-lived call shape) stated explicitly |
| 1c | `jobs::{start-job, step-job, cancel-job}` → `async func`. `job-budget`/`job-step` stay defined in `jobs`, NOT hoisted |
| 1d | `checkpoint::{checkpoint, restore}` → `async func` |
| 1e | `describe::describe` → `async func` |
| 1f | `interface host-async` — zero functional change; header doc retargeted from "async counterpart" to "`world actor`'s async import surface". `emit`/`emit-patch` remain plain `func` |
| 1g | `interface runner` DELETED in full (doc + body + trailing blank) |
| 1h | `world actor` gains `import host-async;`; doc gains the B1 paragraph (one world, seven async exports, S7's no-staged-path finding) |
| 1i | `world actor-async` DELETED in full |
| 1j | the three other stale "pure is the only import" doc claims fixed: `interface types` (:4), `interface documents` (:160), `interface ui` (:771) |

Final inventory: **12 interfaces, 1 world**. 7 async exports, 24 async imports, 5 deliberate sync
funcs (`pure`×3, `emit`, `emit-patch`).

---

## 2. Guest (SDK)

### 2a. `🔌️plugin/🦀️component.rs` — `pub mod component`

`wit-bindgen` **0.57.1** with plain `features = ["macros"]` mirrors WIT `async func` as `async fn` on
the generated `Guest` trait, needs no `async: true` option, and `export!` handles it. This was not
guessed: the ticket's own already-proven fixture `🧫️fixtures/🔌️asyncprobe/👽️guest-turn` does exactly
this against a reduced copy of this very world, on the same pin.

All seven `Guest` methods became `async fn`. The interesting part is what came OUT:

- **Every `resolve_ready` E5 bridge in this module is GONE** (6 sites). `sdk-wasm-guest` installed
  them because the WIT-fixed `Guest` trait was sync and the SDK's `jobs`/`checkpoint`/`describe`
  bodies were genuinely `async fn`. The trait is async now, so they are plain `.await`s. This is the
  collapse's actual payoff on the guest side: a job body that awaits a `host-async` import no longer
  has to resolve on its first poll.
- **`ensure_plugin_initialized()` stays a synchronous call** at all seven entry points, and
  genuinely runs. `sdk-wasm-guest` R9-reverted it to a plain `fn` (its body is a
  `std::sync::Once::call_once` around a sync `fn()` installer — zero suspension). It is called for
  effect, not awaited-and-dropped; the forced-rebuild census (§5) confirms zero dropped futures in
  the crate, so the installer that was dead for the whole program before today is live and stays live.
- `crate::reactor::poll` is sync (R9, `sdk-wasm-guest`) and is called without `.await`; that is
  correct and unchanged — a sync callee inside an `async fn` export is fine.

Most of `sdk-wasm-guest`'s ~89 R9 reversions therefore stand untouched, exactly as the brief predicted.

### 2b. `🔌️plugin/🌐host/🦀️component.rs` — `mod direct`

`world: "actor-async"` → `world: "actor"`. I did **not** delete `mod direct` (prep §2a.2 recommends
it): deleting it means rewriting this file's ~40 `HostBackend::Direct` arms, which is a design change
to a file the prep report itself assigns to `sdk-dedyn`/`host-dedyn`, not a bindgen-mount fix. The doc
now says plainly that the second `generate!` is redundant post-collapse and what folding it away costs.

**Unmasked and fixed — a target that had never been green.** `--target wasm32-wasip2 --all-features`
had 18 errors sitting behind the `component-guest-async` + `wasm32/p2` double gate that **no gate in
this program has ever compiled** (R14, exactly). They were pre-existing universal-async residue
(missing `.await`), not collapse damage — but they were previously masked by the bindgen step. Fixed:

- `pack` R9-reverted to sync with an E5 `resolve_ready` bridge — byte-for-byte the decision the
  sibling `⚛️reactor/🦀️component.rs` already made for its own identical helper, and required because
  `pack` is consumed from sync `Option::map` closures (R10 residue shape 1, `.await` illegal there).
- `kernel_effect_to_direct_wit` / `kernel_endpoint_to_direct_wit` /
  `kernel_outcome_to_direct_wit_respond` / `kernel_placement_to_direct_wit` R9-reverted to sync (pure
  variant translation, zero I/O), matching their `⚛️reactor` counterparts.
- 3 `BodyReader::direct` sites and 1 `next_direct_job_id` site given real `.await`s (one needed a
  `Result::map` → `match` hoist, R10 shape 1).
- `direct_unavailable_fault` left `async` — every call site already awaits it.

⚠️ **A justification the collapse invalidated, flagged not swept.** `⚛️reactor/🦀️component.rs`'s ~24
`resolve_ready` bridges are documented as safe "because `world actor` imports no `host-async`, so
this call never has anything real to suspend on". That premise is now **false**. The bridges are
still SOUND — the real reason is that their callees (`store::pack_rt::{encode,decode}_wire_value`,
`protocol::{decode,encode}_app_frame`) are pure in-memory codecs with no suspension point — but the
written reason is stale. I corrected the wording where I touched it (`🌐host`) and did not mass-edit
the sibling; whoever next opens `⚛️reactor/🦀️component.rs` should restate them on the callee-purity
argument.

### 2c. `🧫️fixtures/🔌️scale/🦀️component.rs`

The one OTHER first-party `wit_bindgen::generate!` against this schema (the scale bench fixture,
outside `🔌️plugin/**` but a direct ABI consumer — leaving it red would have been a false green). Its
seven `Guest` methods went `async fn`. It calls no `host-async` import at all, deliberately: the
bench measures the runtime, so the guest must not add work of its own.

---

## 3. Host

### 3a. One `bindgen!` for the crate

`⏳️imports.rs` no longer runs its own `bindgen!`. `pub(crate) use super::actor_bindings as
host_async_bindings;` — the crate's single invocation (`🦀️component.rs`'s `mod actor_bindings`, now
`pub(crate)`) generates everything both files need, because the collapsed world carries `pure`,
`host-async`, the turn-loop exports and all the type-only interfaces together. The 24 `host-async`
implementations for `AsyncActorHostState` are **unchanged, verbatim** and now compile against the
live world instead of a dead one. `⏳️imports.rs`'s own comment about "`bindgen!` does not dedupe
types across two separate invocations" is updated: there is only one invocation now, so the two
files' `wit_message_endpoint_to_kernel` copies are a genuine merge candidate.

### 3b. `additional_derives: [Clone]` — the prep report's #1 ranked risk, resolved

**Removed, and it cost nothing.** The prep report was right that it cannot survive the merge
(`host-async` carries `stream<u8>` → `StreamReader<u8>`, deliberately not `Clone`;
`additional_derives` is blanket). It was wrong that this would break "dozens of call sites": I ran
its own prescribed check FIRST — grep `.clone()` near every `wit_*` value in
`🖥️host/🦀️component.rs` — and every single hit is on the KERNEL-side `String`/`Vec<u8>` being moved
INTO a generated constructor, never on a generated value. The derive was defensive. Zero call sites
changed; the build confirms it.

### 3c. `WasmtimeRuntime` — async Store, async instantiate, async calls

Every one of these is the shape proven in this ticket's own wasmtime-47.0.3 harness
(`🧫️fixtures/🔌️asyncprobe/🖥️host-turn/🦀️main.rs`), reused rather than re-derived:

| site | before | after |
|---|---|---|
| `build_shared_engine` | `wasm_component_model(true)` only | `+ wasm_component_model_async(true)`. `concurrency_support` left at its `true` default — wasmtime **rejects** a Config that enables component-model-async while disabling it |
| linker | `pure::add_to_linker` + `wasi::p2::add_to_linker_sync` | `Actor::add_to_linker` (whole world: `pure` AND `host-async` in one call) + `wasi::p2::add_to_linker_async` |
| `instantiate` | `Actor::instantiate` | `Actor::instantiate_async(..).await` |
| `execute_turn` | `call_poll(&mut store, &events, budget)` | `store.run_concurrent(async \|accessor\| bindings…call_poll(accessor, events, budget).await).await` — owned args, moved into the concurrent task |
| `start_job` / `step_job` / `cancel_job` / `checkpoint` / `restore` | direct `&mut Store` calls | same `run_concurrent` + `Accessor` shape; error nesting grew one layer (`run_concurrent` trap, call trap, `plugin-error`) |

`Store::run_concurrent` is the ONLY shape wasmtime 47 offers for an async-lifted export, and it
requires `T: Send + 'static` — `ActorHostState` satisfies it structurally (R3: no bound was added
anywhere).

**Five empty `Host` impls + one `HostSurface` were required and are empty by construction, not by
omission.** `Actor::add_to_linker` demands a `Host` impl for every interface `wit-parser` surfaces as
an import — including `types`/`capabilities`/`effects`/`events`/`ui`, which are there only because an
exported signature references their types and which declare no functions at all. That is precisely
the `functional_import_names` vs `type_only_import_names` distinction the schema-parity test asserts.
`ui`'s empty marker `resource surface` additionally forces a `HostSurface::drop` that can never be
called (no host function ever hands the guest a `Surface` handle).

### 3d. `host-async` under the poll-backed runtime — a boundary, not a stub

`emit`/`emit-patch` are **fully functional**: they push onto per-Store sinks that `execute_turn`
drains and merges into the same `turn-result` as the effects `poll` returned (emitted-first, since
they happened earlier in the turn by construction). No `block_on` bridge on a wasm host path —
`emit` is sync in the WIT, so it pushes the raw WIT variant and `execute_turn` does the async
`wit_effect_to_kernel` conversion where it can actually await. `emit_patch_sink` is drained and
visibly discarded, because this runtime's `ui_patches` is unconditionally empty pending the WIT
`patch-op` ↔ kernel `PatchOp` encoding agreement; draining rather than accumulating keeps a
long-lived actor from growing an unbounded sink.

The 24 awaitable imports return a typed `host-async.poll-backed` fault. **This is the correct
semantics for this runtime, not a gap**: a `host-async` import must resolve DURING the guest's call,
while `WasmtimeRuntime` answers host operations by returning the effect in the `turn-result` and
delivering `event.completed` on a LATER `poll`. There is no point in the turn at which such a future
could complete — parking would deadlock the turn, trapping would kill the actor. The runtime that
serves them for real is the one built on `⏳️imports.rs`'s `AsyncActorHostState` (24 real
implementations dispatching onto `AsyncServices`), which `async-plugin-runtime` mounts.

I deliberately did **not** switch `WasmtimeRuntime` to `AsyncActorHostState`: that needs an
`Arc<AsyncServices<TokioHostRuntime>>` and a router handler that `WasmtimeRuntime::new(cfg)` has no
way to construct, and per prep §4.6 the `⏳️imports.rs`/`⏳️runtime.rs` mounting is explicitly
`async-plugin-runtime`'s packet (rule 25).

---

## 4. Schema-parity test (`🖥️host/🧪️schema-parity/🦀️component.rs`)

Per prep §3, scoped as §3 specifies rather than as the brief's literal wording asked (the literal
"every func is `async func`" reading contradicts the same brief's emit/emit-patch exception):

| test | state |
|---|---|
| `every_req_bearing_effect_has_a_matching_host_async_import` | KEPT verbatim — unaffected |
| `spawn_job_has_a_matching_host_async_import_despite_carrying_no_req` | KEPT verbatim — unaffected |
| `emit_carries_the_whole_effect_variant` | KEPT verbatim (its assertions were always right; only the file header's two-world framing was false) |
| `both_worlds_share_the_same_export_surface_and_actor_is_untouched` | **DELETED** — `fixture.world("actor-async")` panics before reaching an assertion |
| `exactly_one_world_exists` | NEW — `packages[pkg].worlds == {"actor"}` AND no `interface runner`. Catches a resurrected world or a stray spike world |
| `world_actor_exports_and_imports_exactly` | NEW — exports unchanged `{reactor,jobs,checkpoint,describe}`; functional imports `{pure, host-async}` (was `{pure}`), with the type-only-import sanity assertion kept |
| `every_export_of_world_actor_is_async_func` | NEW — walks `world actor`'s 4 exported interfaces, asserts `FunctionKind::AsyncFreestanding` on all 7 functions, asserts the exact named set, and **positively asserts `pure`'s 3 funcs are still `Freestanding`** so a future blanket sweep cannot take them along |
| `every_fallible_host_async_import_returns_a_result` | NEW — all 24 return `result<_, _>` (through `canonical_type`, rule 20); `emit`/`emit-patch` asserted to return NOTHING and to be `Freestanding`. Guards the silent-default-decode bug: a bare return type would let a host fault decode as an empty `Vec<u8>` |

The file header was rewritten from "parity between two worlds" to the two invariants it now proves.

---

## 5. Acceptance — every command run in the foreground, this turn, target named

Target dirs: `…/scratchpad`-adjacent session dirs `target-host` (native), `target-wasm` (wasip2),
`target-wu` (wasm32-unknown-unknown), all under
`/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/`.

### Baselines that must not regress

| check | required | measured | ✓ |
|---|---|---|---|
| `cargo test -p semio-framework-os-kernel --lib` | 779 / 0 | EXIT 0 — **779 passed; 0 failed; 0 ignored** | ✅ |
| `cargo test -p semio-framework-os-kernel-db --lib` | 424 / 0 | EXIT 0 — **424 passed; 0 failed; 0 ignored** | ✅ |
| `cargo test -p semio-framework-plugin-host --lib` | 122 / 0 / 1 ignored | EXIT 0 — **125 passed; 0 failed; 1 ignored** | ✅ (+3 = the parity re-spec's net new tests: 1 deleted, 4 added) |
| `cargo check -p semio-framework-plugin --lib` (target `x86_64/aarch64-apple-darwin`, forced rebuild) | EXIT 0, 0 dropped futures | **EXIT 0, 0** | ✅ |
| `cargo check -p semio-framework-plugin --lib --all-features` (native) | EXIT 0 | **EXIT 0**, 0 dropped futures | ✅ |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | EXIT 0 | **EXIT 0**, 0 dropped futures | ✅ |
| `… --features component-extension-guest` | EXIT 0 | **EXIT 0**, 0 dropped futures | ✅ |
| `cargo check -p semio-framework-plugin-host --lib` | EXIT 0 | **EXIT 0** (1 warning, the pre-existing `SharedThreadTransport` visibility one), 0 dropped futures | ✅ |
| `cargo check -p semio-framework-os-kernel --lib --target wasm32-unknown-unknown` | EXIT 0 | **EXIT 0** | ✅ |

### New for this packet

| check | result |
|---|---|
| `cargo test -p semio-framework-plugin-host --lib schema_parity` | **7 passed / 0 failed**, all seven listed BY NAME (4 new/rewritten, 3 kept) |
| `grep` — `interface runner` / `world actor-async` gone from the WIT | ✅ only two DOC-COMMENT mentions survive, both explaining the deletion. `grep '^world \|^interface '` → **12 interfaces, 1 world (`actor`)** |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --all-features` | **EXIT 0** — a target that has never been green in this program (18 errors before, §2b) |
| `cargo check -p semio-framework-plugin-describe --all-targets` + `cargo test` | **EXIT 0**, **5 passed / 0 failed** — also never green before (§7 note) |
| `cargo check -p semio-framework-os-scale-fixture --lib --target wasm32-wasip2 --features component-guest` | **EXIT 0**, 0 dropped futures |

### Dropped-future census (R12 amended + R13 + R17)

Run the turn each crate went green, on **forced rebuilds** (`cargo clean -p …` removed 624 files /
1.1 GiB in `target-host`; each log independently confirmed by `grep "Checking <crate> v"` that the
crate really recompiled). Grepped `unused implementer of` **alone**, per R12-amended.

**Instrument verified against a known positive before trusting the zeros** (R12's own lesson): a
two-line probe (`async fn f(); pub fn g() { f(); }`) compiled with plain `rustc` produced
`warning: unused implementer of \`Future\` that must be used`, matched by the grep, count 1. Note it
rendered the SHORT form — direct confirmation that R12's original long-form-only pattern would have
returned a false zero here.

| crate / target | dropped futures |
|---|---|
| `semio-framework-plugin --lib` (native) | **0** |
| `semio-framework-plugin --lib --all-features` (native) | **0** |
| `semio-framework-plugin --lib` wasip2 `component-guest` | **0** |
| `semio-framework-plugin --lib` wasip2 `component-extension-guest` | **0** |
| `semio-framework-plugin --lib` wasip2 `--all-features` | **0** |
| `semio-framework-plugin-host --lib` | **0** |
| `semio-framework-plugin-describe --all-targets` | **0** |
| `semio-framework-os-scale-fixture --lib` wasip2 | **0** |

No `insert-await.py` run — R20 respected. Every `.await` in this packet was placed by hand against a
rustc span I read myself, and no whole-file rewriting script was used (the four `python3` passes were
literal-string substitutions with `assert count == 1` on every pattern plus a before/after line-count
print, all reported: +0/+0/+0/+2/+7/+167/+6 lines, every delta accounted for).

---

## 6. The strongest evidence: a real component, inspected

`cargo check` proves nothing about the ABI. So a real artifact was built and read back:

    cargo build -p semio-framework-os-scale-fixture --lib --target wasm32-wasip2 --features component-guest
    → EXIT 0, semio_framework_os_scale_fixture.wasm (829,336 bytes)

`jco wit` on that component:

    world root {
      import semio:framework/pure@1.0.0;
      import semio:framework/{types,capabilities,effects,ui,events}@1.0.0;
      import wasi:io/poll@0.2.9; …
      export semio:framework/reactor@1.0.0;
      export semio:framework/jobs@1.0.0;
      export semio:framework/checkpoint@1.0.0;
      export semio:framework/describe@1.0.0;
    }

`jco print` (WAT), the decisive lines — all seven exports carry the component-model **async** ABI:

    (export "[async-lift]semio:framework/reactor@1.0.0#poll" …)
    (export "[async-lift]semio:framework/jobs@1.0.0#start-job" …)      (+ step-job, cancel-job)
    (export "[async-lift]semio:framework/checkpoint@1.0.0#checkpoint" …) (+ restore)
    (export "[async-lift]semio:framework/describe@1.0.0#describe" …)
    (export "[callback][async-lift]…" × 7)
    (import "$root" "[waitable-set-poll]" …) (import "$root" "[task-cancel]" …)

**Two findings worth lifting from this:**

1. **The async lift is real and mechanical.** Seven `[async-lift]` exports plus their seven
   `[callback]` companions plus the `waitable-set` / `task-cancel` builtins. This is what S7's
   "a sync export is uncallable on an async Store" is about, now on the artifact side.
2. **A guest that does not CALL `host-async` does not IMPORT it.** The world declares the import;
   `wasm-ld`/`wit-component` strip what the core module never references, so this fixture's component
   imports `pure` and the type-only interfaces and nothing else. Consequence: the host's `host-async`
   linker definitions are simply unused for such a component (wasmtime permits extra definitions),
   so **the collapse does not force every existing component to be rebuilt against a wider import
   surface**. That de-risks the fleet rebuild materially, and it means a plugin only pays for
   `host-async` when it actually uses it.

The web/jco half needed **no change**, confirmed by reading it rather than trusting the prep report:
`📦️packages/🟦️typescript/🌐plugin-web-materialize.ts` already passes
`--map semio:framework/host-async=./🟨️host-shim.js` alongside `pure`, already wraps every export in
`async (...) =>`, and already ships a `host-async` shim region. It was written against the target
design by `terra-web-bridges`. GO-jspi's finding that jco emits `WebAssembly.promising` regardless of
`--async-mode` is why no flag change is needed either.

---

## 7. Residue — precisely two items, neither a broken build

**(a) `🖥️host/⏳️runtime.rs` — unmounted, and now definitively stale.** It is not in `📦️glue.rs`, so
it does not compile and never did; nothing regressed. But it targets `world actor-async` and is built
entirely around `interface runner`'s `run: async func(events: stream<event>)`, both now deleted. I
added a `⛔️ SUPERSEDED — DO NOT MOUNT THIS FILE AS-IS` header spelling out the three concrete things
a future packet must do: (i) `ActorAsync` → `Actor`; (ii) the whole `call_run` / `GrantWindow` /
`GrantedEventProducer` / `synthesize_turn_result` machinery — the majority of the file — has no entry
point left and must become a command-channel loop calling `reactor().call_poll(accessor, …)` once per
`Poll`; (iii) the predicted `semio_framework_jobs_async()` / `semio_framework_checkpoint_async()`
accessors never existed — `jobs`/`checkpoint` went async IN PLACE, so the real accessors are the
unsuffixed ones. **Budget it as a rewrite, not a rename.** `🦀️component.rs`'s `WasmtimeRuntime` now
demonstrates the working call shape for all seven collapsed exports, so the rewrite has a worked
reference it did not have before.

**(b) `⚛️reactor/🦀️component.rs`'s ~24 `resolve_ready` justifications are stale prose.** Detailed in
§2b. Sound bridges, wrong stated reason ("`world actor` imports no `host-async`"), not swept.

**Open design question raised, deliberately not resolved (recorded in the file itself).**
`⚛️reactor/💼️jobs`'s `JobCtx::host()` gate is justified by "`world actor-async`'s `runner::run` has
no such gap" — a justification that just died with that world. The gate is still CORRECT today: the
gap is a property of `run_job_to_completion`'s `start_job → step_job*` relay loop never re-entering
`poll`, not of the WIT. But a job stepped through the collapsed world's `async` `step-job` CAN now
suspend, so whether the gate may be relaxed depends on whether the host's relay re-pumps in a way
that lets a parked host-await resolve. That needs **measuring, not assuming** — it is invisible to
the compiler either way, so it will not announce itself. I wrote the question into the module's own
doc so it cannot get lost in a report.

---

## 8. Two things that were fixed as fallout, and were never green before this packet

Both were **already red before I started** (universal-async codemod residue behind gates nothing
compiled) and both are now green. Reporting them so nobody re-opens a packet for them.

**`semio-framework-plugin-describe`** — `cargo check --all-targets` **EXIT 0**, `cargo test` **5
passed / 0 failed**. 14 of its 15 errors were pre-existing (`impl pure::Host` methods declared
`async fn` against a bindgen-fixed sync trait; missing `.await` on `sha256_hex`/`hex_encode`/
`pack_rt::{encode,decode}_wire_value`/`run_describe`/`describe_component` and in 5 tests; `fn main`
calling an `async fn run`). The collapse added the 15th and required the rest of `master-u §B3`:
`wasm_component_model_async(true)`, `Actor::add_to_linker`, `add_to_linker_async`,
`instantiate_async`, `run_concurrent` for `call_describe`, and a **`host-async` surface that refuses
every import with a `describe.impure` fault** — because `describe()` must be pure, and a component
whose `describe()` tries to read storage or open a window must fail loudly at describe time rather
than emit a descriptor built from a half-satisfied environment. `📦️main.rs` gained the one sanctioned
`block_on` (R4 clause 1, binary entry point, tagged E3).

**`semio-framework-plugin --lib --target wasm32-wasip2 --all-features`** — **EXIT 0**, see §2b.

---

## 9. Files changed

Schema
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`

Guest
- `…/🔌️plugin/🦀️component.rs`
- `…/🔌️plugin/🌐host/🦀️component.rs`
- `…/🔌️plugin/⚛️reactor/💼️jobs/🦀️component.rs` (doc: the open question)
- `…/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` (comment)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/🦀️component.rs`

Host
- `…/🔌️plugin/🖥️host/🦀️component.rs`
- `…/🔌️plugin/🖥️host/⏳️imports.rs`
- `…/🔌️plugin/🖥️host/⏳️runtime.rs` (SUPERSEDED header only)
- `…/🔌️plugin/🖥️host/🧪️schema-parity/🦀️component.rs`
- `…/🔌️plugin/🖥️host/📦️packages/🦀️rust/📦️glue.rs` (doc)
- `…/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` (`component-model-async`)

Describe
- `…/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`
- `…/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️main.rs`
- `…/🔌️plugin/📇️describe/📦️packages/🦀️rust/Cargo.toml`

Excluded areas were not touched: `🗣️dsl/**`, `💡️inference/**`. No `🛎️services/**` or `⏳️async/**`
change was needed — the async Store wiring is entirely inside `🔌️plugin/🖥️host`. No git-modifying
command was run. No `ticket_close`/`ticket_reopen`.

**One dependency note for the registrar:** `component-model-async` is now declared EXPLICITLY on
`wasmtime` in two crate manifests (`plugin-host`, `plugin-describe`) rather than left to feature
unification through `wasmtime-wasi`. `Config::wasm_component_model_async`, `Store::run_concurrent`,
`Accessor` and `StreamReader` are all `#[cfg(feature = "component-model-async")]`-gated; relying on
another crate to turn them on is exactly the kind of invisible coupling that breaks the first time
the dependency graph shifts.
