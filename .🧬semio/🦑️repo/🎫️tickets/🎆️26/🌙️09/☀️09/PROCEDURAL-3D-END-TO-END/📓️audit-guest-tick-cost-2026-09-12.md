# Guest `flowEvalTick` cost audit — where the 12–18 s per hop actually goes, 2026-09-12

Read-only audit, ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`. Repo/semio MCP both refused to connect
(`repo`: `invalid initialize params`; `semio`: `CONNECTION_CLOSED`); bookkeeping is on this file only,
no ticket state touched. No source was edited, no build was started, no dev server was (re)started.
`http://127.0.0.1:6018/` answered `200` before any probe ran.

**Headline, ahead of the detail.** Every fix the six prerequisite reports describe (tick-arming latch,
contributions-push starvation, node-graph surface retention, the per-turn `cabi_realloc` leak, the
106–249 kB manifest re-parse cache) is **present in the working tree and confirmed LIVE** by a fresh
150 s probe run for this audit (`🗑️generated/tick-cost-1/`, §1). With all of that landed, the example
still converges in **~130 s**, and the residual cost is **not** node-graph paint, **not** the
contributions pack, and **not** JSON re-serialization — it is one **synchronous, non-yielding Rust call
inside a single reactor turn**, most likely `FlowEvalSession::tick`/`evaluate_tick`'s own dag walk or
the `flow-extension-brep`/`flow-extension-math` component's `evaluate`/`tessellate` handler, running as
**unoptimized (`opt-level = 0`) wasm** in the browser's baseline tier. §3 shows the wasm-dev profile
never overrides `opt-level` for any of these crates, unlike two sibling plugins that hit the identical
symptom and were exempted for exactly this reason. §4 ranks the fix.

---

## 0. Method

New probe `🐍️tick-cost-probe.mjs` (written for this audit, kept in the ticket folder), built on
`🐍️console-dump-probe.mjs`'s harness: it arms `localStorage.SEMIO_RUNTIME_DIAGNOSTICS=1` via
`context.addInitScript` before the page's own scripts run (the switch `📓️example-switch-runtime-2026-09-12.md`
§1.1 names), and timestamps every console line with a monotonic `performance.now()`-class clock instead
of `Date.now()`'s 1 ms bucket. Run:

```
cd <ticket> && SEMIO_PROBE_SECONDS=150 SEMIO_PROBE_OUT=tick-cost-1 bun 🐍️tick-cost-probe.mjs
```

Result: 208 console lines, 201 `[DEBUG]` lines, **converged** — `window:procedural-main` all 7
sections `ok`, `window:procedural-preview` `phase: idle`, `facesDone 8/8`, `unitsDone 44/44`,
`ratio 1.0`. Raw output kept at `🗑️generated/tick-cost-1/{console.txt,debug-deltas.jsonl,hosts.json,final.png}`.

**Finding 0, load-bearing for everything below**: arming `SEMIO_RUNTIME_DIAGNOSTICS` from the browser
**does not reach the guest**. `runtimeDiagnosticsEnabled()` (`🏛️ShellHost/🟦️.tsx:1640-1655`) is a
TS-only reader (`import.meta.env`, then `localStorage`) — nothing in the tree calls the guest's
`set_runtime_diagnostics` or threads a value through `wasi:cli/environment` (grepped; no hits outside
doc comments). The guest resolves `RUNTIME_DIAGNOSTICS_ENV` itself
(`🧰️framework/🔨️modules/⏱️trace/🦀️.rs:191-198`, gated on `cfg(any(not(wasm32), target_env = "p2"))` —
`wasm32-wasip2` qualifies for the env-reading branch, but nothing ever sets that env var for the
component instance the browser boots) — so it is **always OFF in the browser**, independent of the
localStorage flag. Every Rust-side `[DEBUG]` line this ticket's prior lanes added (`maintenance
stage=…`, `reactor more-work streak=…`, `guest linear memory turn=…`) is consequently invisible in
every probe run in this ticket, including this one. Confirmed empty: zero matches for `maintenance
stage=`, `more-work streak=`, or `guest linear memory turn=` in `console.txt`. **This is the audit's
own rank-2 recommendation (§4).**

---

## 1. Per-turn breakdown of one `flowEvalTick`

### 1.1 How to count turns from the console alone

`🔌️PluginRuntime/🟦️.tsx:2291-2328` (`runQueuedTurn`) is what `[DEBUG] command ingress lane` /
`[DEBUG] command ingress settled` bracket. Inside it, after the first page is submitted, a
**bare `for` loop drives one `submitTurn` (one guest `poll()` call, i.e. one reactor turn) per
iteration, unconditionally logging every 32nd one**:

```ts
for (let continuation = 0; terminal !== "command-complete" && continuation < 1_024; continuation += 1) {
  …
  const continued = await submitTurn(actorId, acknowledgements, { activation });
  …
  if (continuation % 32 === 31) console.warn(`[DEBUG] command ingress continuation ${continuation + 1} …`);
}
console.warn(`[DEBUG] command ingress settled status=${terminal ?? "missing"} …`);
```
(`🔌️PluginRuntime/🟦️.tsx:2320-2327`, exact lines as read this session)

This log is **not** gated by `runtimeDiagnosticsEnabled()` — it is unconditional. **It never appears
anywhere in the 150 s capture.** Since the modulo is 32, its absence is direct proof that every
`flowEvalTick` command in this run completed in **fewer than 32 reactor turns — almost certainly
exactly ONE**, because the loop body only runs at all once the first page's own `submitTurn` returned a
non-terminal status, and every observed gap below is a single silent interval with no intermediate
line of any kind (§1.2). One `command ingress lane` → one silent gap → one `command ingress settled`,
with nothing between, is the signature of **zero-to-one intervening `submitTurn` calls**, not many
cheap ones.

### 1.2 The five `flowEvalTick` hops actually captured, timestamped

From `🗑️generated/tick-cost-1/console.txt` (all times ms since navigation):

| dispatch (`performInvocation`/`command ingress lane`) | settle (`command ingress settled`) | **silent gap** | extension answered | extension→answer gap | capability |
|---|---|---|---|---|---|
| 5 894.87 (×2, both preview windows' install re-arms) | 24 247.73 / 24 355.57 | **18.35 s / 18.46 s** | 24 758.70 | 511 ms after settle | `flow-extension-math` evaluate |
| 40 345.83 | 40 391.85 (+40 404.00) | **46 ms / 58 ms** | 41 053.73 | 662 ms | `flow-extension-brep` evaluate |
| 53 614.63 | 65 171.03 | **11.56 s** | 65 436.81 | 266 ms | `flow-extension-brep` evaluate |
| 77 852.67 | 88 825.02 | **10.97 s** | 89 173.04 | 348 ms | `flow-extension-brep` tessellate |
| 106 546.71 | 110 053.79 | **3.51 s** | 110 244.77 | 191 ms | `flow-extension-brep` tessellate (final, converges) |

(A sixth, cheap tessellate hop at 102 737.10/102 737.36 settles inside the SAME silent window as the
one above it and is not separately dispatched.) These five/six hops match
`📓️tick-arming-latch-2026-09-12.md` §4's own derivation exactly: 1 math evaluate + 2 brep evaluate +
3 brep tessellate = 6 extension round trips for one preview window's convergence, and **every one
answers in `turns: 1`** (confirmed in `console.txt`: `turns: 1` on all six `extension request
answered` lines) — i.e. the extension side is not spinning either.

**What happens in the silent gap, verified by absence, not inference:**

- `🗑️generated/tick-cost-1/console.txt` lines 77852–88825 (the 10.97 s gap): **zero lines** between
  `command ingress lane` and `command ingress settled` — no `refreshUi`, no `flowpaint`, no `dag draw`,
  no `node-graph host mount`, no `maintenance stage`, no `more-work streak`. The node-graph canvas is
  **not** repainting during this window (its own `flowpaint`/`dag draw` lines land AFTER the settle, at
  70 908/95 741, i.e. once per completed refresh, not during the wait) — `📓️node-graph-surface-retention-2026-09-12.md`'s
  fix holds: this run shows exactly **one** `flow surface context created` (13 494 ms, boot) and **zero**
  `node-graph host mount` after it, so the ~6 s canvas-recreate cost `📓️dispatch-timer-throttle-2026-09-12.md`
  §3 measured is gone (paint-only lines here cost ≤44 ms: 13 494.05→13 538.04).
- The gap is bounded on both sides by a SINGLE `command ingress lane`/`settled` pair with no
  continuation log, so per §1.1 it is **one reactor turn** — one `poll()` export call — taking
  3.5–18.4 seconds of wall time by itself.

### 1.3 The mechanism that lets one turn run for 18 seconds

`REACTOR_EXECUTOR.run_until_deadline(64, 256*1024, now + 8ms)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1169`) is the reactor's own
8 ms/turn budget. Its implementation
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs:170-200`) checks the
deadline **once per loop iteration, between two different tasks' `step()` calls** — never inside one:

```rust
while remaining_units != 0 && std::time::Instant::now() < deadline {
    …
    let step = if closing { task.close_step(budget) } else { task.step(budget) };   // ← not preemptible
    …
}
```

`generation3d`'s `flowEvalTick` command handler is a **plain synchronous `fn`**, not `async`
(`✏️s/…/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:37` `pub fn evaluate(…)`, calling
`preview_eval::evaluate_tick` at line 62 — no `.await` anywhere in the chain), so once the executor
calls into it as one task's `step()`, nothing can interrupt it until it returns. `FlowEvalSession::tick`
(`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs:2860-2865`) itself budgets by **node count**, not wall time:
`host.evaluate_step(FLOW_EVAL_TICK_STEP_BUDGET)` with `FLOW_EVAL_TICK_STEP_BUDGET = 512`
(`…/🖥️host/🦀️.rs:2504`) — 512 nodes is far more than this 7-node graph needs, so that budget is not
what caps the call; nothing time-based does.

The interactive-step ceiling itself is explicitly **advisory, not preemptive**:
`interactive_step_contract_violated` (`⏱️trace/🦀️.rs:99-103`) only classifies elapsed time AFTER the
fact, and the comment at `🔌️plugin/🦀️.rs:29546` (quoted in `📓️runtime-hotpath-audit-2026-09-10.md` §2)
says outright it measures "wall time including descheduling" and is "recorded, not fatal." There is no
code path in this reactor that can stop a synchronous Rust function once the executor has called it.

**Attribution, honestly bounded.** The candidates for the actual 3.5–18 s of CPU inside that one
un-preemptible step are, in order of what the evidence rules in or out:

1. **`FlowEvalSession::tick`/`evaluate_step`'s own dag walk over the real fixture** — ruled UNLIKELY as
   the sole cause: `hex_column_boot_stays_inside_the_interactive_turn_budget`
   (`📓️hotpath-optimization-2026-09-10.md` §7) measures this exact function, on a comparable
   hexagonal/rectangular column fixture, at **6.9–7.4 ms** natively (dev profile, i.e. also
   `opt-level = 0`); `📓️tick-arming-latch-2026-09-12.md` §3 measures it again post-fix at **2.3–2.6 ms**.
   Both are the SAME code path this report's turns exercise.
2. **The `flow-extension-brep`/`flow-extension-math` component's own `evaluate`/`tessellate` handler**
   — plausible but not confirmed from this probe alone: `serve()`
   (`✏️s/…/generation3d/🧪️tests/🔬️brep-extension/🦀️.rs:31-46`) documents that the native in-process test
   harness runs "the same kernel, not a stub" (`semio_framework_os_flow::brep_geometry::tessellate_step_envelope_json`,
   `evaluate_invoke_json`), and that SAME 6-round-trip chain converges natively in **0.6 s total**
   (`📓️node-graph-surface-retention-2026-09-12.md` §4, `📓️tick-arming-latch-2026-09-12.md` §4)
   — ≈100 ms/call. But every `extension request answered` line in this probe fires only 191–662 ms
   AFTER the outer `flowEvalTick`'s own `command ingress settled`, i.e. AFTER the multi-second silent
   gap already ended — so on the evidence actually captured here, **the extension's own compute is not
   what the silent gap is timing**; it is fast once it starts.
3. **Something else inside the SAME reactor turn, competing for the same `run_until_deadline` task
   rotation** — the cooperative-maintenance job (stage 22, the byte-proportional
   `window_transient_store.maintenance_step` retirement `📓️runtime-hotpath-audit-2026-09-10.md` §2
   already named as the ONE stage that does real work here) or a concurrently-active spawned job
   (`[DEBUG] spawn-job routed kind=framework.reserved.tool job=129` fires at 5 895 ms in this same
   capture) are both plain synchronous `step()` calls in the identical un-preemptible position.

Given #1's own native numbers stay in single-digit milliseconds even on this fixture, and #2's answer
timestamps land after the gap rather than inside it, **the strongest read of the evidence actually
collected is that the un-preemptible cost sits in generation3d's own reactor-turn machinery around the
tick** (maintenance/retirement or a fixture/eval-session rebuild not covered by the existing native
benchmark's exact fixture), but this audit cannot name the single function with certainty **because
Finding 0 (§0) removes the one instrument that would have shown it directly**. Fixing §4 rank 1 (below)
is a prerequisite to closing this attribution, not an alternative to it.

---

## 2. What scales with what

| cost | scales with | evidence |
|---|---|---|
| Contribution-table JSON re-parse (was 5.2 ms/call × every tick) | **fixed, now O(1) per registry generation** | `installed_flow_extensions_shared()` caches an `Arc<Vec<FlowExtensionInfo>>` keyed by nothing but the registry mutex, invalidated only by `FlowRegistryReplacement::publish` (`🌊️flow/📔️registry/🦀️.rs:472-499`, doc comment names the exact 106 kB/249 kB before-numbers). This probe's own `contributions push` line shows `chars: 248635` pushed **exactly once** (2045 ms), not once per tick. |
| Window-transient publication (the old "re-serialize the whole eval session every tick") | **fixed, now gated on content change** | `flow_eval_publication_for` (`🌊️flow/🖥️host/🦀️.rs`, `📓️hotpath-optimization-2026-09-10.md` §3): `Retained` when unchanged, publishes nothing. `refreshUi sections`'s `hashes` field in this probe changes generation number only when a section actually re-serialized (e.g. `procedural-preview` hash bumps 1→2→…→10 across the run, `procedural-main` only 8 times total in 130 s). |
| 64 KiB window-transient retirement paging (stage 22) | **scales with the published eval/mesh JSON's byte size**, but only fires on an actual publish now | `SURFACE_RECONCILE_PAGE_BYTES = 65536` (`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs`, multiple sites). The final tessellate answer here is 2 265 B — under one page; earlier "0-to-108 kB" eval-JSON scenarios from `runtime-hotpath-audit` would need 2 pages. |
| Node-graph canvas repaint (`flowpaint`/`dag draw`) | **fixed at O(1) per refresh** (was O(remount) = full surface recreate) | `📓️node-graph-surface-retention-2026-09-12.md`; this probe shows exactly 1 `flow surface context created`/`node-graph host mount` in the whole 130 s run, all later repaints costing ≤44 ms. |
| Per-turn `cabi_realloc` of the `poll` indirect-params area (was 4 360 B/turn leaked forever) | **fixed — flattened below `MAX_FLAT_PARAMS`, no parameter area at all** | `📓️guest-memory-retention-2026-09-10.md` §3: `poll(events, budget)` is 7 flats; no host↔worker postMessage of a whole document is in play for the react target at all — `submitPluginTurn`/`enqueuePluginTurn` (`🔌️PluginRuntime/🟦️.tsx:1300`) run the guest **in-tab**, not across a Worker boundary (`📓️dispatch-timer-throttle-2026-09-12.md` §3 confirms "no network/worker hop for the react target"). The wgpu target (`🎯️targets/🧊️wgpu`) is the one that actually pays a whole-page postMessage/OffscreenCanvas cost, and it prices that separately (`📓️wgpu-intake-budget-2026-09-10.md`'s node-scaled `RETAINED_UI_INTAKE_STEPS_PER_NODE = 8192`/node ceiling) — not applicable to the 6018 react target this ticket's browser probes all run against. |
| Contributions publisher itself (the 248 kB closure push) | **fixed at exactly one crossing per install**, independent of refresh cadence | `createContributionsPublisher` (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`, `📓️contributions-push-starvation-2026-09-12.md` §2.1): keyed `(pluginId, instanceId)`, concurrent callers join one run. This probe: `contributions push … crossings: 1` exactly once. |
| **The ~3.5–18.4 s per-hop residual (§1)** | **not shown to scale with any of the above** — the number of preview windows (both re-arm once each, no compounding observed after install), the contribution pack size (already O(1)), or the fixture's node count (7 nodes, well under the 512-node tick budget) | §1.3 — the residual is CPU time inside one non-preemptible reactor `step()`, priced by `opt-level`, not by any document/window/pack dimension this audit could vary in a read-only pass. |

---

## 3. The wasm-dev profile — `opt-level` confirmed at 0 for every crate on this path

`/Users/ueli/Documents/semio/Cargo.toml:465-490`:

```toml
[profile.dev]
debug = false
incremental = true
# (no opt-level key here → Cargo's own default, 0)

[profile.wasm-dev]
inherits = "dev"
codegen-units = 1
# (no opt-level key here either → still 0)

[profile.wasm-dev.package.semio-s-artifact-lowpoly-lowpoly]
opt-level = 2   # comment: "at opt-level = 0 a lowpoly render turn tessellates its seeded mesh
                #  in 10-11.5 ms" against an 8 ms budget — trapped `plugin.internal.interactive-ceiling`

[profile.wasm-dev.package.semio-s-artifact-puzzle-3d]
opt-level = 2   # comment: "at opt-level = 0 a puzzle3d turn overran the ceiling at 9100us
                #  against the 8000us budget"
```

**`generation3d`/`procedural` has no such override, and neither does the flow-extension crate it
depends on for the actual geometry kernel.** Grepped exhaustively (`grep -n "profile.wasm-dev.package"
Cargo.toml`): only the two entries above exist in the whole workspace. The relevant crates that would
need one:

| crate | package name (`Cargo.toml`) | what it does on this path |
|---|---|---|
| the plugin itself | `semio-s-plugin-procedural` | hosts `flowEvalTick`, `FlowEvalSession::tick`, the preview payload/tessellation cache |
| the shared flow kernel | `semio-framework-os-flow` (path `🧰️framework/…/🌊️flow/📦️packages/🦀️rust`) | `FlowHost::evaluate`, `brep_geometry::tessellate_step_envelope_json`, `evaluate_invoke_json` — the actual numeric geometry code, shared by every flow-domain plugin, not generation3d-specific |
| the answering extensions | `semio-s-plugin-flow-extension-brep`, `semio-s-plugin-flow-extension-math` | the wasm components that answer `capability: evaluate`/`tessellate` — real BREP/math work, separately compiled, also uncovered |

**Is opt-level 0 plausibly the whole story?** The two existing overrides are direct, measured precedent
for the SAME symptom class at a SMALLER scale: "seeded mesh" tessellation (lowpoly) and a generic
puzzle3d turn both already overshoot the 8 ms budget by ~10–14 % at opt-level 0 (9.1–11.5 ms vs 8 ms).
generation3d's BREP domain (`brep.solid.extrude`, `brep.bool.fuse`, `brep.solid.fillet`,
`brep.sweep.extrude`, `brep.solid.shell` — all present in this probe's `contributions scoped from
published examples` line) does substantially more numeric work per call than lowpoly's "seed a mesh."
An order-of-magnitude-plus escalation from the documented ~10 ms floor to the observed 3.5–18 s is
large, but not implausible for real solid-modeling operators at `opt-level = 0` compounded by a wasm
engine's baseline (non-optimizing) execution tier for a one-shot synchronous call that never gets
tiered up — exactly the gap the audit could not close without §4 rank 1's fix.

**Build-time cost of fixing it.** Already measured in this ticket for the exact package this needs it
most:

```
CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural \
  --target wasm32-wasip2 --profile wasm-dev --keep-going   → 0 errors, Finished in 1m 45s
```
(`📓️tick-arming-latch-2026-09-12.md` §5.6, warm target dir; `📓️hotpath-optimization-2026-09-10.md` §9
records a cold run of the same check at 3m27s.) That is a one-time, per-crate, incremental cost — not a
whole-profile rebuild — and it is the SAME shape of change already twice-precedented in this exact file.

---

## 4. Ranked fixes

### Rank 1 — wire the browser's diagnostics switch through to the guest (prerequisite, cheap)

**Why first:** every other fix in this list is currently unverifiable in the browser (§0). Without it,
nobody can confirm which function the 3.5–18 s actually sits in, or measure rank 2/3's effect live.

- **File:** `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:173` (`set_runtime_diagnostics`) is already `pub`.
  Add a WIT export (mirroring the shape of `stage-command-page` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`) that a guest boot path calls once
  from an env/config value the HOST can actually set — or, simpler and zero-schema-risk: read
  `SEMIO_RUNTIME_DIAGNOSTICS` from a build-time constant baked into the `wasm-dev` profile specifically
  (dev/debugging builds should not need a runtime toggle for a debugging aid that already does not ship
  in `wasm-release`), so `runtime_diagnostics_from_environment()`'s `p2` branch
  (`⏱️trace/🦀️.rs:196-198`) resolves `true` unconditionally under `cfg(feature = "…")` or a
  `wasm-dev`-only cfg, with no host wiring needed at all.
- **Test to pin it:** extend `hex_column_boot_stays_inside_the_interactive_turn_budget`
  (`✏️s/…/generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, region `📈️HotPathBudget`,
  `📓️hotpath-optimization-2026-09-10.md` §7) with a wasm-hosted twin that boots the compiled
  `wasm32-wasip2` component under `wasmtime` (the harness `📓️guest-memory-retention-2026-09-10.md` §5
  already uses, `🔌️plugin/🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs`) with `SEMIO_RUNTIME_DIAGNOSTICS=1`
  set on the wasmtime `WasiCtx`, and assert the SAME `[BUDGET]`/ledger lines print — this closes the
  "native says 2.6 ms, wasm says nothing" gap for good, independent of any browser probe.

### Rank 2 — `opt-level` override for the flow/BREP hot path in `[profile.wasm-dev]`

**File:** `/Users/ueli/Documents/semio/Cargo.toml`, immediately after the existing
`[profile.wasm-dev.package.semio-s-artifact-puzzle-3d]` block (line 490), add:

```toml
[profile.wasm-dev.package.semio-s-plugin-procedural]
opt-level = 2

[profile.wasm-dev.package.semio-framework-os-flow]
opt-level = 2

[profile.wasm-dev.package.semio-s-plugin-flow-extension-brep]
opt-level = 2

[profile.wasm-dev.package.semio-s-plugin-flow-extension-math]
opt-level = 2
```

Same lever, same file, same profile section as the two existing entries — no new architecture, no
compatibility layer, nothing to deprecate. `opt-level = 2` (not `1`) matches the two precedents rather
than introducing a third tuning value; if rank-1's instrumentation later shows `1` is enough for one of
these four crates, narrow it then, from measurement, not guesswork.

**Test to pin it:** run `hex_column_boot_stays_inside_the_interactive_turn_budget`'s wasm-hosted twin
(rank 1) before and after this change and assert `best_us`/`worst_us` drop by at least the same
order-of-magnitude the lowpoly/puzzle3d comments record (10–14 % over budget → comfortably under);
separately, re-run this audit's own `🐍️tick-cost-probe.mjs` against a restaged `:6018` and confirm no
`command ingress settled` gap exceeds ~100 ms once the extension answers are optimized (target stated
in the brief: full example under 5 s).

**Build-time cost:** ~1m45s warm / ~3m27s cold per `cargo check -p semio-s-plugin-procedural --target
wasm32-wasip2 --profile wasm-dev` (already measured, §3); the three additional crates share most of
their dependency graph with `semio-s-plugin-procedural` already, so the incremental delta from adding
them is expected to be small relative to that baseline, not four independent 1–3 minute costs.

### Rank 3 — make the interactive-step ceiling actually preemptive for long single-step work

**Why, and only after rank 1/2:** §1.3 shows the reactor's 8 ms budget is advisory
(`interactive_step_contract_violated`, `⏱️trace/🦀️.rs:99-103`), not enforced — `run_until_deadline`
(`⚛️reactor/🧵️executor/🦀️.rs:170`) cannot interrupt a single `task.step()`. Even after rank 2 shrinks
the *average* cost, a pathological document (more nodes, a slower operator) can still park the main
thread for an unbounded time in one non-yielding call, and nothing observes it.

- **File:** `✏️s/…/generation3d/…/🧵️preview-eval/🦀️.rs:481` (`evaluate_tick`, the `session.tick(&mut
  host)` call) and `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs:2860` (`FlowEvalSession::tick` →
  `host.evaluate_step(FLOW_EVAL_TICK_STEP_BUDGET)`). `FLOW_EVAL_TICK_STEP_BUDGET = 512`
  (`🖥️host/🦀️.rs:2504`) already exists as a NODE-count chunking mechanism for local evaluation; the
  precedent for chunking the EXTENSION side already exists too (`preview_tessellate_invocations`'s own
  `budget`/`chunk` fields, `…/🧵️preview-eval/🦀️.rs:454-455`, `PREVIEW_TESSELLATE_STEP_BUDGET`). The
  gap is that neither budget is TIME-based, so a single expensive node (one `brep.bool.fuse` on a
  complex solid, say) still runs to completion inside one un-preemptible step regardless of the node
  count budget.
- **Change:** lower `FLOW_EVAL_TICK_STEP_BUDGET` from a pure node count to a node count **capped by an
  elapsed-time check between nodes** (the same `Instant::now() < deadline` pattern
  `run_until_deadline` already uses, threaded one level deeper), so a slow node yields the reactor turn
  after itself rather than after the whole 512-node walk. This does not fix a slow BREP kernel call
  itself (that is rank 2's job) — it stops one slow call from blocking unrelated UI/paint/other-window
  work for the full duration, which is the difference between "this window is slow" and "the tab is
  unresponsive."
- **Test to pin it:** a new fixture-driven law beside `hex_column_boot_stays_inside_the_interactive_turn_budget`
  that stubs one operator kind with an artificially slow handler (a `std::thread::sleep`-free, budget-
  countable delay, mirroring how `📓️guest-memory-retention-2026-09-10.md`'s `poll-turn-memory` harness
  drives synthetic turns) and asserts the reactor turn returns control within one budget quantum even
  when that operator is present, rather than asserting the whole tick's wall time.

### Verified already-fixed (no further action; confirmed live by this audit's own probe, §0/§1/§2)

- Tick-arming latch (`📓️tick-arming-latch-2026-09-12.md`) — confirmed: exactly 2 `flowEvalTick` re-arms
  on install, 1:1 invocation:settle ratio, no duplicate ticks.
- Contributions-push starvation (`📓️contributions-push-starvation-2026-09-12.md`) — confirmed: exactly
  1 `contributions push`/`contributions publish` pair, `crossings: 1`.
- Node-graph surface retention (`📓️node-graph-surface-retention-2026-09-12.md`) — confirmed: 1
  `node-graph host mount`, 1 `flow surface context created`, 0 remounts in 130 s.
- Per-turn `cabi_realloc`/linear-memory leak (`📓️guest-memory-retention-2026-09-10.md`) — not directly
  re-measured here (would need the memory-growth probe, out of this audit's scope), but the WIT schema
  change it describes (`poll(events, budget)`, no `command-ingress-page` field) is present in the
  working tree.
- Manifest re-parse cache (`installed_flow_extensions_shared`, part of the tick-arming-latch lane) —
  confirmed: `contributions scoped pack {"chars":248635,…}` appears once, not once per tick.

---

## 5. Files referenced (all read-only; none modified)

- `/Users/ueli/Documents/semio/Cargo.toml:465-490` — `[profile.dev]`/`[profile.wasm-dev]` and the two
  existing per-package `opt-level` overrides
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏱️trace/🦀️.rs:158-198` — the diagnostics switch and
  its wasm32/p2 resolver branches
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1640-1655`
  — `runtimeDiagnosticsEnabled()` (TS-only, never reaches the guest); `:4634` — `refreshUi sections` log
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:2291-2328`
  — `runQueuedTurn`'s command-ingress continuation loop and its unconditional modulo-32 log
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1169`
  — the reactor's 8 ms `run_until_deadline` call site
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs:170-200`
  — `run_until_deadline`'s implementation (deadline checked between task steps, not within one)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:37-71`
  — the synchronous `evaluate`/`handle` command entry point
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs:454-533`
  — `evaluate_tick`, `preview_tessellate_invocations`'s budget/chunk fields
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2504,2843-2865`
  — `FLOW_EVAL_TICK_STEP_BUDGET`, `FlowEvalSession::sync`/`tick`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:472-556`
  — `installed_flow_extensions_shared`, `flow_extension_invocation_address`, the registry-generation cache
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs:1-46`
  — the native in-process extension runner, documented as the real kernel, not a stub
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` —
  `SURFACE_RECONCILE_PAGE_BYTES` (64 KiB paging)

## 6. Ticket-folder inputs added by this audit

- `🐍️tick-cost-probe.mjs` — kept (a reusable probe script, per the ticket's own convention of keeping
  input scripts)
- `🗑️generated/tick-cost-1/` — this audit's raw probe capture (console dump, per-line deltas, DOM
  snapshot, screenshot); deletable with the ticket per the standing convention for generated output
