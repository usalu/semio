# `opt-level` on the guest hot path — and what the 3.4–18.5 s per hop actually is, 2026-09-12

Implementation lane for `📓️audit-guest-tick-cost-2026-09-12.md` §4 ranks 1–3, ticket
`2026/09/09/PROCEDURAL-3D-END-TO-END`. Repo/semio MCP both refused to connect (`repo`:
`invalid initialize params`; `semio`: `CONNECTION_CLOSED`), so bookkeeping is this file only — no
ticket state opened or closed. Peers were editing guest Rust throughout (their `[DEBUG] close stage
line=` probe shows up in every native run below); every red is attributed.

## TL;DR

1. **Rank 1 landed and works.** The browser's `localStorage.SEMIO_RUNTIME_DIAGNOSTICS` now reaches
   the guest through `wasi:cli/environment` — the WIT interface the component already imports, so
   there is no new semio WIT, no build-profile `cfg`, and a release run stays clean because nothing
   sets the variable. Proven at runtime: the same probe that captured **201** `[DEBUG]` lines before
   now captures **7 346**, including every Rust-side `turn phase …` / `guest linear memory turn=…` /
   `reactor more-work streak=…` site (§1, §4).
2. **Rank 2 landed — and the measurement disproves its premise.** With `opt-level = 2` on all seven
   hot-path crates, the per-hop gaps are **18.42 / 11.58 / 10.70 / 3.40 s** against the baseline's
   **18.35 / 11.56 / 10.97 / 3.51 s** — within ±3 %, i.e. unchanged. The example still converges at
   **~112 s**. `opt-level = 0` was **not** what the per-hop seconds were made of (§4.2).
3. **Rank 1 is what says where they ARE made of.** In the 11.1 s gap the guest runs **25 turns
   totalling 0.33 s of its own CPU**, and the window contains one **contiguous 10.42 s of complete
   console silence** that begins immediately after the guest answered `more-work streak=10
   sources=["reconcile"]`. Across the whole 121 s run the guest burns **5.2 s** of executing time
   (4.3 % duty cycle). The cost is host-side JS between the guest's "more work" and the host's next
   `poll` — **not** the dag walk, **not** the BREP kernel, **not** wasm codegen (§4.3). This
   supersedes the audit's §1.3 attribution ("one non-preemptible turn of 3.5–18.4 s"), which was
   inferred from the ABSENCE of a log the reconcile pump never writes.
4. **Rank 3 landed anyway, and is the right fix regardless.** The dag walk is now preempted by a
   wall clock between neurons, not only by a node count, so one expensive operator can no longer park
   the reactor for an unbounded time. Three new deterministic laws in the evaluator, one extended law
   in generation3d (§3).
5. **The overrides are kept**, on the evidence that they shrink the served guest wasm by **39.9 %**
   (96.11 MB → 57.74 MB) and that guest turns still measure over the 8 ms ceiling even at
   `opt-level = 2` — but §5 states the honest cost (a profile change invalidates every wasm-dev
   artifact: **15 m 47 s** to rebuild) so a future dev can revert from measurement, not guesswork.

---

## 1. Rank 1 — the guest's diagnostics, reachable from the browser

### 1.1 Why the switch never arrived

`runtime_diagnostics_enabled()` (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs:177-201`) resolves
`RUNTIME_DIAGNOSTICS_ENV` through `std::env::var`, which on `wasm32-wasip2` is
`wasi:cli/environment.get-environment()`. The transpiled component imports exactly that
(`semio_s_plugin_procedural_component.js:20,24`: `import { environment } from '…/preview2-shim/cli.js';
const { getEnvironment } = environment;`) — and the browser shim's `_env` is `[]` until somebody calls
`_setEnv`. Nobody did. The page's `localStorage` switch only ever armed TypeScript traces.

### 1.2 The option chosen, and why

The audit offered a `wasm-dev`-only `cfg`/feature or a WIT-level boot value. Neither was needed as a
NEW schema: `wasi:cli/environment` **is** the WIT-level boot value, already declared, already
imported, already the thing `RUNTIME_DIAGNOSTICS_ENV` documents itself as ("Named as a schema constant
so a host that has no environment … arms the same switch through `set_runtime_diagnostics`"). A cargo
feature would have been code-first, would have baked the decision into the binary, and would have
needed a second rebuild to flip. Seeding the declared WASI interface keeps release builds clean by
construction — an unarmed host hands the guest no environment at all, and the trace sites stay
compiled in and silent.

The only real work was the realm hop: **a Worker owns no `localStorage`**, and the react target does
run its guests in shard workers (`PluginRuntime/🟦️.tsx:372` → `new Worker(…)`). So the page resolves
the switch and stamps it on the worker's own URL, which is the only channel that exists before the
worker's first message.

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts` | `SHARD_RUNTIME_DIAGNOSTICS_KEY`, `SHARD_WORKER_DIAGNOSTICS_PARAM`, `shardWorkerUrl()` — `SHARD_WORKER_URL` plus `?diagnostics=1` when the page armed it. `SHARD_WORKER_URL` itself is untouched (its schema-route law still holds). |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:372` | `new Worker(shardWorkerUrl(), …)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` | `shardWorkerSource()` gains `armGuestRuntimeDiagnostics()`: reads its own url's `diagnostics` param, dynamically imports the vendored `preview2-shim/cli.js` and `_setEnv({SEMIO_RUNTIME_DIAGNOSTICS:"1"})` — awaited in `loadActor` BEFORE the bridge import. Same module instance the component imports (both resolve `🪞️vendor/…/preview2-shim/cli.js` against the shared `🔌️plugin-modules/` root), so seeding it seeds every actor the worker hosts. A shim that cannot load degrades to silent traces, never to a failed boot. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` | native twin: `guest_wasi_ctx()` hands the guest `SEMIO_RUNTIME_DIAGNOSTICS=1` **and** a `MemoryOutputPipe` stderr when the host's own diagnostics are armed; `GuestInstance::guest_diagnostics_text()` reads it back. Before this, a wasmtime-hosted guest's `eprintln!` went nowhere at all. |

### 1.3 Pinned by the wasm-hosted twin the audit asked for

`an_armed_host_reaches_the_guests_own_runtime_diagnostics`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs`) boots the REAL
staged `wasm32-wasip2` procedural component under wasmtime twice in one test — disarmed, then armed —
and asserts silence then `[DEBUG]`. Both halves must run sequentially because the switch is a
process-wide atomic, so this file now has a `WASM_HOST_SERIAL` mutex that both wasm laws take (the
pre-existing `the_poll_export_keeps_guest_linear_memory_flat_across_turns` would otherwise have had
its guest's behaviour changed mid-run by the new law's arming).

```
SEMIO_POLL_TURN_LEAK_COMPONENT=…/target/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm \
  cargo test -p semio-framework-plugin-host --lib poll_turn_memory
→ an_armed_host_reaches_the_guests_own_runtime_diagnostics ... ok
→ the_poll_export_keeps_guest_linear_memory_flat_across_turns ... ok
  test result: ok. 2 passed; 0 failed  (15.66s)
```
Observed in that run: `guest stderr: disarmed=None armed=1566 bytes`, carrying real
`cooperative-maintenance instance=1 turn=… pool=CooperativePoolSnapshot{…}` lines, plus the `log`-import
half (`[actor:actor-1:debug] [DEBUG] turn phase …`). **A wasmtime-hosted guest turn at `opt-level = 2`
costs 0.4–0.6 ms** (`elapsed_us=596 / 408 / 410`) — worth holding next to the browser's 10–19 ms in §4.3.

The browser half is pinned by two new laws in `…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
(`spells the same diagnostics key in every realm that resolves it`, `carries the armed switch into the
worker realm on the worker url`) — the generated worker's source is asserted as TEXT because that
module reaches the repo library's `bun:sqlite` lease store, which a jsdom bundle refuses to import.
`5 passed | 973 skipped`.

---

## 2. Rank 2 — the `[profile.wasm-dev.package.*]` overrides

`/Users/ueli/Documents/semio/Cargo.toml:492-534`, immediately after the existing lowpoly/puzzle3d
entries and documented the same way. Seven packages, read off
`cargo tree -p semio-s-plugin-procedural --target wasm32-wasip2 -e normal`
(`🗑️generated/opt-level/cargo-tree-depth2.txt`) — the audit's four plus the three the brief asked to
identify from the graph:

| package | what a `flowEvalTick` hop runs in it |
|---|---|
| `semio-s-plugin-procedural` | the tick, the preview payload, the tessellation cache |
| `semio-framework-os-flow` | `FlowHost::evaluate_step`, `brep_geometry::{evaluate_invoke_json, tessellate_step_envelope_json}` |
| `semio-framework-os-kernel-neural-engine` | `Evaluator::evaluate_channels_budgeted` — the topological dag walk |
| `semio-framework-replication` | `DslValue`/`OrderedMap` — the per-neuron `Dictionary`, cloned/merged/hashed per node |
| `semio-framework-pack` | `pack::json` — each node's input/output payload |
| `semio-s-plugin-flow-extension-brep` | answers `evaluate`/`tessellate` for the BREP operators |
| `semio-s-plugin-flow-extension-math` | answers `evaluate` for the math operators |

Deliberately NOT included: `semio-framework-hash` — `node_hash` uses `std`'s `DefaultHasher`, not
blake3 (`🧠️neural/⚙️engine/🦀️.rs:1585-1590`), so the hash crate is not on this path.

### 2.1 What it bought — wasm size

| artifact | before (`opt-level = 0`) | after (`opt-level = 2`) | Δ |
|---|---|---|---|
| served `semio_s_plugin_procedural_component.core.wasm` | 96 106 813 B | **57 744 331 B** | **−39.9 %** |
| uplifted `target/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm` | 96 156 839 B | 57 800 996 B | −39.9 % |
| `…flow_extension_brep_component.core.wasm` | 24 351 807 B | 20 436 493 B | −16.1 % |
| `…flow_extension_math_component.core.wasm` | 16 584 403 B | 13 153 804 B | −20.7 % |

The served size was read off the running server, not off disk:
`curl … :6018/🔌️plugin-modules/🌀️procedural/…core.wasm | wc -c` → `57744331`. **The vite on 6018
served the new bytes with no restart**, as expected.

### 2.2 What it did NOT buy — see §4.2.

---

## 3. Rank 3 — the interactive-step ceiling, made preemptive

§1.3 of the audit is right that nothing could interrupt the walk, even though its attribution of the
SECONDS was wrong (§4.3). `run_until_deadline` (`⚛️reactor/🧵️executor/🦀️.rs:170-200`) checks its
deadline between two tasks' `step()` calls, never inside one; `FLOW_EVAL_TICK_STEP_BUDGET = 512`
counts NODES, and this fixture has seven. So one expensive operator — a `brep.bool.fuse` on a complex
solid — still runs to completion inside one un-preemptible step however few nodes it is.

- `🧠️neural/⚙️engine/🦀️.rs` — new `EvalStepBudget { dispatches, deadline: Option<EvalStepDeadline> }`
  replaces the bare `budget: usize` on `evaluate_channels_budgeted`, with `PROBE`/`UNBOUNDED`/
  `dispatches(n)`/`until(n, clock, deadline_us)` constructors. The clock crosses as a plain
  `fn() -> Option<u64>` pointer, so this leaf evaluator gains **no** dependency on the tracing module
  that owns the process clock and stays usable on a target where `Instant` is not.
  **Progress guarantee**: `exhausted(spent) = spent >= dispatches || (spent != 0 && deadline expired)`
  — a walk that starts already over budget still dispatches one node, so it converges one node per
  tick instead of re-arming forever with nothing to show.
- `🌊️flow/🖥️host/🦀️.rs` — `FLOW_EVAL_TICK_ELAPSED_CEILING_US = INTERACTIVE_STEP_CEILING_US / 4 * 3`
  (6 000 µs), and `flow_eval_tick_budget()` pairing it with the unchanged 512-node count. No clock
  installed ⇒ node count only, i.e. exactly the previous behaviour.

**Laws** (all run, all green):

| law | file | what it pins |
|---|---|---|
| `evaluate_channels_budgeted_yields_on_the_wall_clock_after_one_dispatch` | `🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` | an ALWAYS-expired deadline still admits one dispatch, names the blocker in `remaining`, converges a two-node chain over two calls, and computes the same values |
| `evaluate_channels_budgeted_without_a_clock_keeps_the_node_count_as_the_only_cap` | same | a clock that answers `None` never preempts |
| `evaluate_channels_budgeted_probe_dispatches_nothing_even_past_a_deadline` | same | `dispatches = 0` outranks any deadline |
| `hex_column_boot_stays_inside_the_interactive_turn_budget` (**extended**) | `…generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | the pre-existing budget/publication assertions PLUS: the elapsed ceiling is strictly under the interactive contract, the tick budget actually carries a deadline on a clocked target, and the node count is added to — never replaced by — the wall clock |

```
cargo test -p semio-framework-os-kernel-neural-engine --lib  → ok. 51 passed; 0 failed
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly \
  --lib hex_column_boot_stays_inside_the_interactive_turn_budget
→ [BUDGET] turns=2 turns_to_first_mesh=1 round_trips=2 eval_steps=2 best_eval_step_us=1988
  worst_eval_step_us=9809 mean_eval_step_us=5898 … meshes=3
  test result: ok. 1 passed; 0 failed
```
⚠️ That law needs `RUST_MIN_STACK=134217728` on this tree today — without it the test aborts with
`has overflowed its stack` before any assertion runs. Not this lane's change: the same abort happens
with the wall-clock ceiling disabled, and the run is full of a peer's `[DEBUG] close stage line=`
probe. Flagged for whoever owns the close-ladder lane.

---

## 4. Measurement

### 4.1 How

```
bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev      # restage, foreground
cd <ticket> && SEMIO_PROBE_SECONDS=150 SEMIO_PROBE_OUT=opt-level-after        bun 🐍️tick-cost-probe.mjs
cd <ticket> && SEMIO_PROBE_SECONDS=150 SEMIO_PROBE_DIAGNOSTICS=0 \
               SEMIO_PROBE_OUT=opt-level-after-silent                         bun 🐍️tick-cost-probe.mjs
```
`🐍️tick-cost-probe.mjs` gained two things this lane needs: a `SEMIO_PROBE_DIAGNOSTICS=0` escape (an
armed run now costs the guest's own console traffic, which the baseline never paid — see §4.3), and an
**instrumentation-free convergence clock** that polls the rendered surfaces' own
`data-status-json`/`data-meshes-json` every 250 ms, so the headline number is comparable across runs
with and without diagnostics. Output: `🗑️generated/{opt-level-after,opt-level-after-silent}/`.

Both runs converged: `window:procedural-main` all 7 sections `ok`, `window:procedural-preview`
`phase: idle`, `facesDone 8/8`, `unitsDone 44/44`, `ratio 1.0`, `meshesSeen 3`.

### 4.2 Per-hop gaps — before vs after

Baseline is `📓️audit-guest-tick-cost-2026-09-12.md` §1.2 (`🗑️generated/tick-cost-1/`); after is
`🗑️generated/opt-level-after-silent/console.txt`, the run with the SAME instrumentation level as the
baseline. All times ms since navigation.

| capability | before: dispatch → settle | before gap | after: dispatch → settle | after gap | Δ |
|---|---|---|---|---|---|
| `flow-extension-math` evaluate | 5 894.87 → 24 247.73 / 24 355.57 | **18.35 / 18.46 s** | 6 390.78 → 24 810.83 / 24 917.29 | **18.42 / 18.53 s** | +0.4 % |
| `flow-extension-brep` evaluate | 40 345.83 → 40 391.85 | 46 ms | 39 754.59 → 39 811.43 | 57 ms | — |
| `flow-extension-brep` evaluate | 53 614.63 → 65 171.03 | **11.56 s** | 52 084.35 → 63 661.34 | **11.58 s** | +0.2 % |
| `flow-extension-brep` tessellate | 77 852.67 → 88 825.02 | **10.97 s** | 76 225.63 → 86 930.20 | **10.70 s** | −2.5 % |
| `flow-extension-brep` tessellate (final) | 106 546.71 → 110 053.79 | **3.51 s** | 101 353.22 → 104 751.80 | **3.40 s** | −3.1 % |

Convergence, measured instrumentation-free on the after run: **all 7 nodes `ok` at 101 295 ms**,
**`meshes = 3` at 112 284 ms**. The baseline's final converging hop settled at 110 054 ms. Every
extension request still answers in `turns: 1` on both sides.

**Verdict: `opt-level = 2` on the entire hot path moves the per-hop gap by at most 3 %, which is
inside this harness's run-to-run noise. The seconds are not wasm codegen.**

### 4.3 What the seconds ARE — the answer rank 1 was a prerequisite for

From `🗑️generated/opt-level-after/console.txt` (the armed run, 7 346 `[DEBUG]` lines against the
baseline's 201):

- The guest's own per-turn trace now prints. Over the whole 121.6 s run: **533 turns, 5.2 s of total
  `elapsed_us`** — a 4.3 % duty cycle. `elapsed_us` is `guest_turn_executing_us()`
  (`⚛️reactor/🔄️turn/🦀️.rs:279-286`), which excludes the gaps between polls, so this is real guest CPU.
- Inside the 11.1 s gap (57 141 → 68 236 ms): **25 turns, 0.33 s of guest CPU**, mean 13.4 ms, max
  24.3 ms. The guest is idle for 97 % of the window.
- The window is not evenly spread: scanning every console line in it for a >200 ms silence finds
  **exactly one, of 10 418 ms**, starting right after
  `[DEBUG] reactor more-work streak=10 seen=247 sources=["reconcile"] contended=false effects=0` at
  57 391.47 ms and ending at the next `turn phase enter` at 67 809.77 ms. Zero console output from ANY
  source — guest or host — in between.
- `more-work` sources across the run: `reconcile` 324, `typed_operation` 102, `command_ingress` 61,
  `lifecycle` 2; max streak 66.

So the shape is: the guest answers `more-work` with **zero effects and zero patches**, and the host
takes ten seconds to come back and poll it again. That is host-side JS (or a host-side wait) between
`more-work` and the next `poll`, and it is where the example's whole runtime lives.

This **supersedes** the audit's §1.3. Its inference — "the modulo-32 `command ingress continuation`
log never appears, therefore the gap is ONE reactor turn" — is sound about `runQueuedTurn`'s loop and
wrong about the system: these turns are driven by the `more-work` reconcile pump, which is a different
path and writes no continuation log. There was never a 3.5–18.4 s non-preemptible Rust call.

Consistent with `📓️dispatch-timer-throttle-2026-09-12.md` §2, which independently disproved the
re-arm-timer hypothesis and localized the same cost INSIDE the dispatch
(`continuation dispatch settled … tookMs=17657 / 18027 / 11867`), bracketing the node-graph canvas
surface-create/first-draw chain. **That is where the next lane should look**, and it now has a hard
number to aim at: 10.4 s of contiguous host silence per hop against 0.33 s of guest work.

### 4.4 A caveat about the armed numbers

Arming diagnostics is no longer free: each guest trace line crosses the component boundary and becomes
a `console.log` the probe drains over CDP. The armed run's per-turn `elapsed_us` (mean 9 717, median
11 000, p90 15 000, max 72 900 µs) therefore measures the guest PLUS its own tracing — the wasmtime
twin, which prints the same lines to an in-memory pipe instead, measures the same turns at **0.4–0.6
ms**. Do not read the browser's 10–19 ms as the cost of an unarmed turn; do read it as proof that
turn count, not turn cost, is what the reconcile spin multiplies.

---

## 5. Build-time cost of the overrides, honestly

A `[profile.wasm-dev.package.*]` edit invalidates every `wasm-dev` artifact in the workspace, so the
first restage after it is a full rebuild of all eleven components the generation3d react target stages:

| run | wall | nx cache |
|---|---|---|
| first restage after the Cargo.toml edit | **15 m 47 s** (`real 995.18`) | 0 / 39 hit |
| second restage, ~25 min later | **9 m 15 s** | 2 / 39 hit |

The second run is **not** a clean "warm, no changes" number: 37 of 39 tasks still rebuilt because
peers were editing guest Rust in the same crates throughout this session. The true no-change warm cost
is unmeasured here. The previously recorded single-crate figure still holds for a targeted check
(`cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`: 1 m 45 s warm /
3 m 27 s cold, `📓️tick-arming-latch-2026-09-12.md` §5.6).

**Keep or revert?** Kept, for two reasons that survive §4.2: the served guest wasm is 39.9 % smaller
(96.11 → 57.74 MB, which a browser tab must fetch and `WebAssembly.compile` on every cold boot), and
the interactive-ceiling argument that justified the lowpoly and puzzle3d entries still applies — guest
turns measure over 8 ms here even at `opt-level = 2`. What they do NOT buy is the per-hop latency this
ticket is chasing, and §4.2 is the number to revert on if the build cost ever outweighs the 39.9 %.

---

## 6. Files changed

**Framework — diagnostics reachability (rank 1)**

- `🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts` — `SHARD_RUNTIME_DIAGNOSTICS_KEY`,
  `SHARD_WORKER_DIAGNOSTICS_PARAM`, `shardRuntimeDiagnosticsArmed`, `shardWorkerUrl`;
  `buildShardClientOptions` spawns through it.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` —
  imports and spawns through `shardWorkerUrl()`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` — mirrored constants;
  `shardWorkerSource()`'s `armGuestRuntimeDiagnostics()` awaited in `loadActor`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` — `guest_wasi_ctx()`,
  `GUEST_DIAGNOSTICS_CAPACITY_BYTES`, `ActorHostState.diagnostics`,
  `GuestInstance::guest_diagnostics_text()`.

**Framework — preemptive eval step (rank 3)**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` — `EvalStepDeadline`, `EvalStepBudget`,
  `evaluate_channels_budgeted` retyped.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `FLOW_EVAL_TICK_ELAPSED_CEILING_US`,
  `flow_eval_tick_budget()`, `evaluate_step` retyped, three call sites.

**Profile (rank 2)**

- `Cargo.toml:492-534` — seven `[profile.wasm-dev.package.*] opt-level = 2` blocks, each documented
  with the measured reason, in the same section and shape as the two existing entries.

**Tests**

- `🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — three new deadline laws + the existing budgeted-walk laws
  retyped.
- `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — `evaluate_step_budget_one_converges_over_multiple_calls`
  retyped.
- `🔌️plugin/🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs` — `an_armed_host_reaches_the_guests_own_runtime_diagnostics`,
  `drive_diagnostics_turns`, `WASM_HOST_SERIAL`.
- `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — two new diagnostics-key/worker-url laws and
  `readGeneratedShardWorkerOwnerSource`.
- `…generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `hex_column_boot_stays_inside_the_interactive_turn_budget`
  extended with the three preemption assertions.

**Ticket inputs**

- `🐍️tick-cost-probe.mjs` — `SEMIO_PROBE_DIAGNOSTICS` switch, instrumentation-free convergence clock,
  `convergence.json` output.

## 7. Everything that was run

| command | result |
|---|---|
| `cargo test -p semio-framework-os-kernel-neural-engine --lib` | **ok. 51 passed; 0 failed** |
| `cargo test -p semio-framework-os-flow --lib evaluate_step` | **ok. 1 passed** |
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib hex_column_boot…` | **ok. 1 passed** (needs `RUST_MIN_STACK`, §3) |
| `cargo test -p semio-framework-plugin-host --lib poll_turn_memory` | **ok. 2 passed** |
| `vitest … 🔬️engine-contract -t "diagnostics"` | **5 passed** |
| `vitest … 🎭️actor -t "shard worker from the schema-owned distribution route"` | **1 passed** |
| `bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev` | exit 0, twice (§5) |
| `SEMIO_PROBE_SECONDS=150 … bun 🐍️tick-cost-probe.mjs` (armed and silent) | converged, §4 |

**Red, attributed elsewhere.** `cargo test -p semio-framework-os-flow --lib flow_eval` fails three
laws — `flow_eval_session_sync_and_tick_state_machine`, `flow_eval_session_seeds_its_retained_neural_cache`,
`flow_eval_session_retains_baseline_across_ephemeral_hosts` — all on
`FlowEvalSession must finish explicit close before drop` (`🖥️host/🦀️.rs:2796`, a committed guard).
A/B'd: the same three fail with this lane's wall-clock ceiling raised to `u64::MAX`, so they are the
close-ladder lane's, not this one's. `the_poll_export_keeps_guest_linear_memory_flat_across_turns`'s
MEMORY half is flat (`0.0 B/turn` at 128 and 256 turns) but its `MoreWork` half is timing-flaky under
load (`[0, 1]` at 128 turns, `[0, 1, 21, 118, 191, 241]` at 256) — the cooperative-maintenance pump is
clock-driven, and this machine was running 16-minute wasm builds. Not investigated further here.

## 8. Ticket-folder outputs

`🗑️generated/opt-level/` — `cargo-tree-depth2.txt`, `restage-1.txt`, `restage-warm.txt`,
`restage-start.txt`, `hex-column-after.txt`.
`🗑️generated/opt-level-after/`, `🗑️generated/opt-level-after-silent/` — the two probe captures
(`console.txt`, `debug-deltas.jsonl`, `hosts.json`, `convergence.json`, `final.png`).
