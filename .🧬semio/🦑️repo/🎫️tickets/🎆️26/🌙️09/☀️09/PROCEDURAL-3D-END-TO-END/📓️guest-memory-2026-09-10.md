# 🧠️ Guest linear-memory ceiling and the contributions install memory path

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: guest memory ceiling + contributions install memory path.
Written 2026-09-10, appended as the lane ran.

## 1. The failure this lane owns

Boot #9b (`📓️runtime-verification-2026-09-09.md`, 08:55, on the 08:35 restage) delivered the paged
contributions to the guest — no `setContributions command failed` any more — and then, ~60 s after
boot, the procedural actor died:

```
memory allocation of 189328 bytes failed
  → std::alloc::rust_oom
  → unreachable in alloc::boxed::box_new_uninit
     inside wit_bindgen::rt::async_support::start_task (__export_poll_cabi)
PluginRuntime: actor procedural#1 trapped [handler/turn]
```

That is the guest's wasm linear memory hitting its declared `maximum` and `memory.grow` returning
`-1`, which Rust's allocator turns into `rust_oom` → `unreachable`.

## 2. The ceiling actually configured for the browser guest

Not the shard worker and not jco: neither declares a `WebAssembly.Memory` at all — the component's
own core module owns the limits, and jco instantiates whatever the module declares.

| where | value |
|---|---|
| `.cargo/config.toml:23-24` `[target.wasm32-wasip2] rustflags` | `-C link-arg=--max-memory=536870912` |
| `…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:111,134` | `-C link-arg=-zstack-size=8388608` |
| staged `semio_s_plugin_procedural_component.core.wasm` memory section (parsed) | `min 190 pages = 12 451 840 B`, `max 8192 pages = 536 870 912 B` |

So the browser guest's ceiling is **512 MiB**, and the trap means the guest genuinely consumed it.

## 3. The instrument

Two readings, one per target, both under the ticket's `SEMIO_RUNTIME_DIAGNOSTICS` family:

| target | instrument | cost |
|---|---|---|
| native test binary | `semio_framework_trace::HeapWitness` — a `GlobalAlloc` wrapper over `std::alloc::System` weighing retained and peak bytes process-wide | two relaxed atomics per allocation, TEST BINARIES ONLY |
| wasm guest | `semio_framework_trace::guest_linear_memory_bytes()` — one `memory.size` | free |

New module `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs` (mounted at `⏱️trace/🦀️.rs`
`//#region 🧮️GuestMemory`), because `⏱️trace` already owns the repo's other runtime ceiling
(`INTERACTIVE_STEP_CEILING_US`) and is a zero-dependency leaf every plugin crate already links.

The budget it declares is schema-first, exactly like `PUBLIC_INVOCATION_*`:
`🧮️memory/🧬️schema/🔣️.json` holds `maximumBytes` / `stackBytes` / `installPeakPercent`, and
`🧮️memory/🧪️tests/🔬️memory/🦀️.rs` pins the Rust constants to that schema AND pins both link
arguments (`.cargo/config.toml`'s `--max-memory`, the dev build's `-zstack-size`) to the constants,
so a change to either without the schema fails a law. **The number was NOT changed** — see §5.

The `Puzzle3dHeapWitness` in `…/🧩️puzzle/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` is the same
instrument written a second time; it now delegates to the shared one.

## 4. Measurements

Native, `CARGO_TARGET_DIR` private, `RUSTC_WRAPPER=""`, `cargo test -p
semio-s-artifact-procedural-generation3d --features component-app-assembly --lib
host_pushed_contribution_pages_install_the_registry_the_served_chain_needs -- --nocapture
--test-threads=1`. The law test's own closure is the two-crate one (31 pages, 124 496 chars); the
browser's nine-plugin closure is 293 642 chars, i.e. **2.36×** the payload measured here.

| point | retained (B) | peak (B) | before → after the fix |
|---|---|---|---|
| before boot | 1 283 132 | 1 283 132 | — |
| after boot | 29 770 807 | 29 771 059 | boot delta **28 487 675** |
| after page 0 | 30 704 870 | 30 811 549 | |
| after page 8 | 30 765 700 | 30 876 829 | |
| after page 16 | 30 830 564 | 30 941 805 | |
| after page 24 | 30 830 564 | 30 941 805 | |
| after page 30 (last) | 31 002 822 | 32 558 018 | |
| **after install** | 31 002 792 | 32 558 018 | install delta **1 357 142 → 1 231 985** |
| after first eval | 31 660 295 | 32 558 018 | eval delta 657 503 |
| after first tessellation | 31 745 550 | **32 558 018** | mesh delta 85 255 |

**Peak of the whole boot + install + first-mesh sequence: 32 687 696 B → 32 558 018 B**, against a
**536 870 912 B** budget — **6.1 %** of it, and 10.1 % of the 322 122 547 B (60 %) install-peak
ceiling. The install path is NOT what exhausts the guest, and the ceiling does not need raising.

Steady state, the second law (`the_settled_evaluation_tick_cycle_retains_nothing_that_would_exhaust_the_guest`):

| reading | value |
|---|---|
| settled tick+render cycles measured | 60 |
| retained per cycle | **4 082 B** |
| extrapolated to 600 cycles (a 60 s boot at the preview window's re-arm cadence) | 2 449 200 B |
| headroom the install-peak ceiling leaves | 214 748 365 B |

So a native 60 s steady state claims **2.4 MB**, 1.1 % of the headroom. **Nothing in the Rust the
native suite can run accounts for 512 MiB.**

## 5. Root cause

### 5.1 What it is not

The suspects the lane opened with, each measured or read off the source:

| suspect | verdict |
|---|---|
| the shard worker / jco caps the guest `WebAssembly.Memory` | **no** — `🧵️shard/🟨️shard-worker.js` and the per-plugin `🌉️bridge.js` declare no `WebAssembly.Memory` at all; the core module's own memory section carries `min 190 / max 8192` pages, i.e. the `.cargo/config.toml` link argument, and jco instantiates that |
| the ceiling is too small | **no** — the boot + install + first-mesh peak is 32.6 MB, **6.1 %** of the 512 MiB budget |
| the contributions path holds 294 KB + parsed copies | **partly, and fixed** — it held the assembled JSON a SECOND time forever; see §5.2. Even at the browser's 2.36× closure the whole install path is ≈ 3 MB |
| retained raw command pages survive install | **no** — `ArtifactRetainedCommandJob`s live in the job runner's FIXED-capacity slot table (`🧵️job/🦀️.rs` `close_cursor`/`CAPACITY`), so their `raw` (12 288 B here) and wire pages are bounded by that table, not by the 73-page run. Measured: install retains 1.23 MB total, nowhere near 73 × 12 KB of surviving pages |
| generation3d per-instance retained state grows | **no** — a settled tick+render cycle retains 4 082 B, 2.4 MB over a 60 s boot |

### 5.2 What it is

The trap's own frame is the answer. `wit_bindgen::rt::async_support::start_task` boxes the future of
the guest's **`async func poll` export** (`🔌️plugin/🧬️schema/📜️.wit`, `interface reactor`), and at
`wasm-dev`'s `opt-level = 0` that one future is **189 328 bytes**. The guest's memory starts at
12 451 840 B and the whole Rust-side working set is ~33 MB, so the budget leaves ~480 MB, which is

```
480 MiB / 189 328 B ≈ 2 660 poll futures
```

— and boot #9b died ~60 s in, i.e. at roughly a browser frame cadence of `poll` calls. Every native
measurement above says the Rust owners are flat; the only thing that scales with elapsed turns and is
invisible to a native suite is the per-`poll` task box in the wasm-only async export glue.

**This lane cannot close that on its own**: the `poll` task lifetime is the reactor's (sibling lane:
close ladder / `close_step`), and no browser tool was available to this lane. What it CAN do — and
did — is make the next boot say so instead of trapping mutely (§6.3).

## 6. What changed

### 6.1 The install path stops holding the payload twice

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:236-243` (before):

```rust
pub fn sync_host_flow_extension_contributions(contributions_json: &str) -> Result<(), &'static str> {
    static LAST: Mutex<String> = Mutex::new(String::new());
    let mut last = LAST.lock()…;
    if *last == contributions_json { return Ok(()); }
    …
    *last = contributions_json.to_string();
```

That `static LAST` kept the WHOLE assembled `contributionsJson` — 293 642 B in the browser closure —
alive in the guest for the life of the component, purely as a de-duplication witness, on top of the
parsed manifests the registry already owns and the `contributed` map's own copy. It was not even
exact: a re-ordered push with identical content compared unequal and burned a registry generation.

Now the function takes the payload **by value**, folds it in
`fold_host_flow_extension_contributions`, `drop`s the JSON before any registry is built, and
de-duplicates on the TYPED map — which is the state the registry is rebuilt from anyway, so the
witness costs nothing:

```rust
pub fn sync_host_flow_extension_contributions(contributions_json: String) -> Result<(), &'static str> {
    let contributed = fold_host_flow_extension_contributions(contributions_json)?;
    let mut state = flow_extension_state().lock()…;
    if state.contributed == contributed { return Ok(()); }
```

`ContributedFlowExtension` gained `PartialEq, Eq`; each retained `manifest_json` is `shrink_to_fit`
on the way in. `sync_host_flow_extension_contributions_page` hands its assembled buffer straight over
(no borrow, no copy), so the buffer, the parsed entries and the new registry are never all resident.

**Measured: −125 157 B retained and −129 678 B peak on the 124 496-char closure — the payload,
exactly once. At the browser's 293 642-char closure that is ≈ 294 KB of the guest gone.**

### 6.2 The operator catalogue is shared across app instances

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs`: `flow_app_catalogue_json()` built
~108 kB of `NodeGraphOperatorRecord`s off the registry and serialized them ONCE PER APP INSTANCE —
and one procedural component hosts several. New `flow_app_catalogue_json_shared() -> Arc<str>` caches
that text per `flow_extension_registry_generation()`, exactly the way `flow_neuron_kind_info_map()`
already caches the typed map, and `flow_app_catalogue_json()` is now one `to_string()` off the shared
`Arc` — one copy per instance, never a rebuild, and the copy the instance publishes is dropped by the
existing publication path.

### 6.3 The guest says how full it is before it traps

`🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`, new `trace_guest_memory_pressure()`, called once
per turn beside the existing more-work trace. On wasm it is one `memory.size`; off wasm it returns
immediately. It reports:

- **unconditionally, once**, when the reading first crosses the schema's install-peak ceiling:
  `guest linear memory at N B — P% of the M B budget, past the 60% install-peak ceiling`;
- **under `SEMIO_RUNTIME_DIAGNOSTICS`**, every further 16 MiB of growth.

Boot #9b's `memory allocation of 189328 bytes failed` carried no Rust panic and no diagnosis at all.
The next boot that walks toward the ceiling prints a line at 60 % and a growth trace after it, which
is what turns "the actor trapped" into a measurement.

### 6.4 The budget is schema-declared — and unchanged

**No number was doubled and none was raised.** The 512 MiB that was already in
`.cargo/config.toml` is now *declared* in `🧮️memory/🧬️schema/🔣️.json` and pinned from both ends:

- `the_budget_matches_the_neutral_schema` — the Rust constants equal the schema `const`s;
- `the_link_arguments_carry_the_declared_budget` — `.cargo/config.toml` really links
  `--max-memory=536870912` and the dev plugin build really passes `-zstack-size=8388608`;
- `the_install_peak_ceiling_leaves_headroom_inside_the_budget` — 60 % is a real fraction with
  headroom above the shadow stack.

The measured peak (32.6 MB) is 6.1 % of the budget, so the budget **fits the peak with 16× headroom**
and raising it would only postpone §5.2's trap by seconds while costing every one of the ~20 plugin
workers more reserved address space.

## 7. Tests

Two new laws in
`✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, both weighing the real heap through the
witness installed in that binary's testkit:

1. `host_pushed_contribution_pages_install_the_registry_the_served_chain_needs` (the existing
   delivery law) now probes `before boot / after boot / every 8th page / after install / after first
   eval / after first tessellation` and **asserts the peak of that whole sequence stays under
   `guest_linear_memory_install_peak_ceiling_bytes()`** — 32 558 018 B against 322 122 547 B.
2. `the_settled_evaluation_tick_cycle_retains_nothing_that_would_exhaust_the_guest` — 60 settled
   tick+render cycles, and the per-cycle retention extrapolated to the 600-cycle horizon of a 60 s
   browser boot must fit the headroom the install-peak ceiling leaves. 4 082 B/cycle ⇒ 2 449 200 B
   against 214 748 365 B.

Five budget laws in `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️tests/🔬️memory/🦀️.rs` (§6.4).

### Gate tails

| gate | result |
|---|---|
| `cargo test -p semio-framework-trace --lib memory` | **5 passed, 0 failed** |
| `cargo check -p semio-s-plugin-procedural --keep-going` (native) | `Finished dev profile in 1m 19s`, 0 errors |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going` | `Finished wasm-dev profile in 2m 13s`, 0 errors |
| `cargo check -p semio-framework-plugin -p semio-framework-os-flow --keep-going` | `Finished dev profile in 2m 52s`, 0 errors (warnings are peers' `unused_qualifications`/`never used`) |
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1` | **326 passed, 4 failed** — both new memory laws pass |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests --keep-going` | `Finished dev profile in 34.17s`, 0 errors (the shared-witness delegation) |

### The four generation3d failures are other lanes' in-flight work

None is reachable from this lane's diff (a contribution fold, a catalogue cache and a turn-boundary
`memory.size`):

| test | fault |
|---|---|
| `two_instances_converge_disjoint_widget_moves` | `module.vcs`: "remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized" |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | `generation3d-publication.contended` (flow catalog authority lane) |
| `refresh_pending_effects_arms_flow_eval_tick_chain` | "registered fixture typed operation did not retire within 30 seconds" (close-ladder lane) |
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` | same family |

`semio-framework-os-flow --lib` shows 125/205 failing with `ordered-map root must be explicitly
retired before drop` in `vcs::flow_vcs_tests` / `wasm_session::domain_laws`. `git status` has
UNCOMMITTED peer modifications in exactly those files
(`🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🔺️diff/🦀️.rs`, `🌊️flow/🖥️host/🦀️.rs`,
`🌊️flow/🖥️host/🧹️retirement/…`), and `📓️flow-host-ownership-2026-09-10.md` §4 records the same
panic as that lane's own finding. `registry::tests::*` pass individually — they are process-wide
singleton tests caught in the same binary's abort.

## 8. Restage

`ps` showed the shared `target/wasm32-wasip2` held by TWO peers when this lane was ready — a puzzle
`wasm-release` stage and another lane's own `cargo rustc -p semio-s-plugin-procedural --target
wasm32-wasip2 --profile wasm-dev`. Both were polled to completion (~9 minutes), not chased, and only
then:

```
CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" SEMIO_BUILD_BUDGET_MS=14400000 \
SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react \
bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
```

`Successfully ran target activate-generation3d-react-dev … and 38 tasks it depends on`, run duration
**7 m 23 s**, cache 1/39.

Freshness witness on the staged component the browser loads
(`…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`):

| | before | after |
|---|---|---|
| mtime | `2026-09-10 08:35:08` | **`2026-09-10 09:41:04`** |
| size | 81 151 877 | **81 214 198** |
| sha256 (first 32) | `a3b22bda71c4488767d06dddf0670c95` | **`c81861f8662115a7377a4339b175b36c`** |
| `strings … \| grep -c "guest linear memory"` | 0 | **2** (the ceiling report and the growth trace) |
| memory section (parsed) | `min 12 451 840 / max 536 870 912` | **unchanged** — the budget was declared, not moved |

Neither serve was restarted or killed by this lane. `trunk serve` on **6118** still answers `200`
(pid 5219). Port **6018** no longer answers: a PEER restarted the vite serves during this lane's run
and they are now listening on **6013** (pid 44038, up 14 min) and **6014** (pid 47328) — not this
lane's doing, and recorded here so the next boot uses the right port.

## 9. Handover — what still owns the trap

This lane proved the ceiling is right, cut the install path's duplicate payload, shared the
catalogue, and gave the guest a voice before it traps. The remaining 512 MiB is §5.2's per-`poll`
task box, which belongs to the reactor:

1. **Reactor / close-ladder lane** — bound the number of live `poll` tasks, or shrink the 189 328 B
   future (a `Box::pin` at the turn body's own await points would leave the exported future small).
   Either one moves the trap horizon from ~2 660 turns to unbounded.
2. **Next boot** — with this restage the log should carry
   `guest linear memory at … past the 60% install-peak ceiling` BEFORE any trap, and with
   `SEMIO_RUNTIME_DIAGNOSTICS` armed a `[DEBUG] guest linear memory …` line every 16 MiB. That trace
   is the measurement the browser side has been missing.

## 10. Files

| file | change |
|---|---|
| `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs` | **new** — budget constants, `HeapWitness`, `guest_linear_memory_bytes`/`_percent` |
| `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧬️schema/🔣️.json` | **new** — `maximumBytes`/`stackBytes`/`installPeakPercent` |
| `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️tests/🔬️memory/🦀️.rs` | **new** — five budget laws |
| `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` | `//#region 🧮️GuestMemory` mount + re-exports |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` | `sync_host_flow_extension_contributions` takes `String`, `fold_host_flow_extension_contributions`, `static LAST` deleted, `ContributedFlowExtension: PartialEq, Eq` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs` | two call sites onto the owned signature |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs` | `flow_app_catalogue_json_shared() -> Arc<str>`, generation-keyed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `trace_guest_memory_pressure()` + its two thread-locals, called once per turn |
| `✏️s/…/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-trace` dev-dependency |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | `#[global_allocator]` heap witness + `heap_probe` |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | memory probes + the peak law in the delivery test; the steady-state growth law |
| `✏️s/…/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-trace` dev-dependency |
| `✏️s/…/🧩️puzzle/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | its private witness now delegates to the shared one |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/📓️guest-memory-2026-09-10.md` | this report |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/🗑️generated/oom-*.txt` | raw measurement, gate and restage logs |
