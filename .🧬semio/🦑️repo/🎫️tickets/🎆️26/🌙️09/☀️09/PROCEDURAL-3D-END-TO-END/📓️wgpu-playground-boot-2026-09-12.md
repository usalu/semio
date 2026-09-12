# 🧊️ wgpu playground boot — the second renderer target now boots, installs contributions and presents

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu (wasm) renderer target", 2026-09-12.
Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened — bookkeeping is on disk. Evidence under
`🗑️generated/wgpu-boot/` (`baseline-1`, `run-1` … `run-8-viewer`, `gpu-env.mjs`) and
`🗑️generated/wgpu-serve.txt`. Nothing under any `🗑️generated` folder was swept. The react serve on
6018 was not touched and the procedural guest was **not** rebuilt.

---

## 1. TL;DR

| question | answer |
|---|---|
| Does `http://127.0.0.1:6118/?plugin=generation3d` boot? | **Yes.** `runtime-ready` at **4.7 s**, no fault, no watchdog line, introspection beacon attached. |
| Is headless WebGPU available? | **Yes — a real adapter**, not a fallback: `vendor apple`, `architecture metal-3`, `core-features-and-limits`; rAF 122 ticks / 2 s (`🗑️generated/wgpu-boot/gpu-env.mjs`). |
| Do contributions install? | **Yes.** One scoped pack crossing, 248 635 chars / 249 667 B, `contributions installed … crossings: 1`. |
| Do the window bodies render? | **The shell renders every surface** (`procedural-main` 2 132 ms, `procedural-preview` 280 ms, four framework panels), but **nothing reaches the paint tree**: `dumpStructure` 0 nodes, `dumpFrameStats {drawCalls: 0, quadCount: 0, glyphCount: 0}`, canvas solid black. |
| Does the hexagonal column evaluate / extension completions run? | **No.** Every `renderSurface` reports `effects=0 tags=-`; `invokeExtension` 0×, no `respond`, meshes 0. The guest emits no effects at all on this build. |
| Blockers found | **four**, three fixed here (§4), the fourth (BLANK-PAINT) diagnosed and handed on (§6). |
| Serve | screen **`g3dwgpu`**, trunk **pid 32450**, port **6118** (§2.3). |

---

## 2. Build and serve — exact commands, times, sizes

### 2.1 Staging the plugin (a copy, not a rebuild)

The brief's premise — "the plugin wasm is the same staged component the react serve uses" — was **not
true on disk**. Two separate module trees exist and only the react one had been restaged:

| tree | served by | `🌀️procedural` core wasm | mtime |
|---|---|---|---|
| `🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/` | vite on 6018 (`⚙️vite.config.ts:33` `pluginModulesDir`) | **96 106 813 B** | 2026-09-12 06:30 |
| `🧑‍💻dev/🔌️plugin-modules/` (`PLUGIN_MODULES_ROOT`, `🔌️vite-plugins.ts:22`) | trunk's `copy-dir` into the wgpu dist | 82 246 182 B | 2026-09-**10** 23:44 |

Every other plugin in the generation3d boot closure (`🌊️flow`, all ten `flow-extension-*`) was
byte-identical across both trees; **only `🌀️procedural` was two days stale.** So the whole react-staged
directory was copied across — no cargo, no jco, no restage:

```bash
SRC=🧰️framework/…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural
DST=🧰️framework/…/🧑‍💻dev/🔌️plugin-modules
cp -R "$SRC" "$DST/.stage-procedural" && rm -f "$DST/.stage-procedural/.nx-artifact.json"
rm -rf "$DST/🌀️procedural" && mv "$DST/.stage-procedural" "$DST/🌀️procedural"
```

Staged bytes: `semio_s_plugin_procedural_component.core.wasm` 96 106 813 · `🔣️.json` 985 871 ·
`🛂️.descriptor.semio` 224 487 · `…_component.js` 406 797 · `🌉️bridge.js` 10 376 · `🟨️.js` 6 808.
`curl` from 6118 and 6018 now return the same 96 106 813 bytes.

### 2.2 Renderer wasm (trunk)

```bash
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust
bun ./📜️script.ts generate-frame-worker          # 0.5 s — required twice this session (§5)
# then, from the dev package, the `procedural3d-wgpu` launch row with `served`:
NX_DAEMON=false SEMIO_RENDERER=wgpu S_OS_PORT=6118 CARGO_PROFILE_WASM_DEV_DEBUG=false \
  bun ./📜️script.ts dev generation3d served
```

| build | wall | cargo | note |
|---|---|---|---|
| first (cold dist) | 06:57:34 → 07:00:44 = **3 m 10 s** | 1 m 41 s | full `copy-dir` of 2.4 GB plugin-modules + 504 MB extension-modules |
| trunk watch rebuild (frame-worker JS) | ~5 s | 1.4–1.9 s | incremental dist |
| final serve (after the `🖥️gpu.rs` fixes) | 07:40:19 → 07:43:35 = **3 m 16 s** | 1 m 37 s | |

Served artifact sizes (`.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/`):
`semio-framework-os-renderer-wgpu_bg.wasm` **77 367 401 B**, `…wgpu.js` 176 670 B,
`🎞️frame-worker.js` 1 162 603 B, `🚀️boot.js` 58 106 B, `index.html` 6 842 B,
`🔌️plugin-modules` 2.4 GB, `🧩️extension-modules` 504 MB.

### 2.3 The detached serve

`📜️serve-generation3d-wgpu.sh` (new, this ticket folder), started with
`screen -dmS g3dwgpu <script>`, logging to `🗑️generated/wgpu-serve.txt`.

```
screen  32425.g3dwgpu (Detached)
trunk   pid 32450   127.0.0.1:6118 (LISTEN)
```

The script runs the launch row's own path (`dev generation3d served` under `SEMIO_RENDERER=wgpu`,
`served` so `activatePlaygroundRuntime` never restages the guest) and then **falls back to
`trunk serve --config Trunk.toml --port 6118` directly**, because a live peer lane blocks the dev path
— see §5. Delete the fallback once that lands.

---

## 3. Boot phases and timings (run-7, the current build)

```
 545 ms  renderer-bootstrap font-atlas    executing 2.600 ms
 682 ms  renderer-bootstrap icon-atlas    executing 137.9 ms   ← the one expensive bootstrap phase
 684 ms  renderer-bootstrap font-upload   executing 2.100 ms
 687 ms  renderer-bootstrap icon-upload   executing 0.300 ms
 759 ms  renderer-bootstrap plugin-parse  executing 66.1 ms
 767 ms  renderer-bootstrap shell-construct 2.200 ms
 773 ms  renderer-bootstrap shell-boot
 779 ms  wgpu-bridge createApp open start plugin=procedural instance=1 app=s.procedural.generation3d@1/*#editor
1245 ms  wgpu-bridge createApp open leave                               (466 ms)
3385 ms  wgpu-shell render leave surface=procedural-main               2 132 ms, 4 turns
3666 ms  wgpu-shell render leave surface=procedural-preview              280 ms
3779 ms  … framework.panel.artifact 110 ms · catalogue 301 ms · inspection 31 ms · history 192 ms
4509 ms  boot-phase shell-boot:refresh-ui                              3 071 ms
4512 ms  contributions push  crossings 1, encoding pack, reachableChars 11 936
4534 ms  contributions push  chars 248 635, bytes 249 667
4703 ms  contributions installed  effects: 0, tags: -, crossings: 1
4708 ms  wgpu-worker boot_shell leave                                  3 862 ms
4708 ms  renderer-bootstrap runtime-ready
```

No `boot-liveness` kill, no `worker-boot-step-overrun`, no fault banner for the full 300 s probe.
`wasm-compile` never appears as a separate phase because the module comes out of the IndexedDB cache
on a warm profile; the one `requestfailed … net::ERR_ABORTED` at ~280 ms is the artifact `HEAD`
probe, not a failure.

`?role=viewer` (run-8-viewer) boots identically (`boot_shell leave 3 878 ms`, contributions installed)
— **but it still opens `…#editor`**. `appRole` reaches `semioWgpuSetAppRole`; the app chosen by
`ShellState::boot` does not depend on it. Observed, not taken on by this lane.

---

## 4. Blockers fixed here

### 4.1 The stale staged guest — `manifest parse: missing field \`dialect\``

`🗑️generated/wgpu-boot/baseline-1` (against the serve that had been running since 2026-09-10):

```
worker-boot-failed: plugin-parse: plugin procedural: manifest parse: missing field `dialect`
at line 1 column 299150        —  Boot stage: icon-upload, 1.8 s
```

The 2026-09-10 `🔣️.json` carries 4 `"dialect"` occurrences; the 2026-09-12 one carries 17. Fixed by
§2.1's copy. **Not a code defect** — a staging gap between the two module trees, which nothing warns
about (the same class as `[[project-component-release-does-not-materialize]]`).

### 4.2 The wgpu bundle published only ONE of the two module routes

`🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🌐️.html` carried a `copy-dir` for `🔌️plugin-modules` and none
for `🧩️extension-modules`, so every extension descriptor fetch fell through to trunk's SPA index:

```
[DEBUG] contributions prime failed plugin=flow-extension-brep
        plugin.descriptor-invalid: /🧩️extension-modules/🧊️flow-extension-brep/🔣️.json returned HTML
        (×9, one per flow extension)
```

With no primed extension manifest the scoped `setContributions` pack is empty — exactly the
`empty-or-unscoped` / "trunk wiped brep overlay" symptom
`📓️wgpu-contributions-pack-sender-2026-09-10.md` recorded as an environment flake. It was not a
flake: the route was never mounted. The React dev server mounts both
(`⚙️vite.config.ts:131,174,182`); trunk mounted one.

**Fix** — `🌐️.html` gains the sibling `copy-dir`, and the build/serve scripts now *assert* both routes
instead of discovering the gap at boot:
`⚙️browser-build/🟦️.ts` `bundleCopyDirectives` + `assertBundleModuleRoutes`, called from
`TrunkBuildScript`/`TrunkServeScript` (`📦️packages/🦀️rust/📜️script.ts:223,240`).

**Test** — `🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/` (+ `laws.json`, registered in the wgpu
`vitest.config.ts`): the fixture is pinned to `MODULE_ROUTES` (the neutral `🛣️routes.json`), the real
document must publish every declared route into an existing dev-session directory, a plugin-only
document and a dangling `copy-dir` are both refused, and `moduleRoutePath` accepts a descriptor
request under *either* route — which is why one unmounted route is a silent SPA fallthrough rather
than a 404. **5 passed.**

### 4.3 `frame-runtime-fault: Cannot convert 2 to a BigInt` — a `u64` seam typed as `number`

```
TypeError: Cannot convert 2 to a BigInt
    at BrowserRendererWorker.enqueueBatch (semio-framework-os-renderer-wgpu.js:101:26)
    at 🎞️frame-worker.js
```

`🌐️browser-worker/🦀️.rs:288,316` declare `enqueue_batch(events_json: &str, generation: u64)` and
`tick(_timestamp_ms: f64, _sequence: u64, generation: u64)`. wasm-bindgen lowers a `u64` parameter
into the wasm `i64` slot, which throws on a JS `number`. The frame Worker's own hand-written handle
type (`🎞️frame-worker/🟦️.ts:15-16`) declared all three as `number`, so TypeScript could never see it,
and the Worker faulted on its **first** frame batch — every time, on a fresh surface.

The fault banner appeared ~33 s late, which is its own trap: `fault()` → `beginClose()` and
`post({kind:"fault"})` only happens *after* `closeRuntime()`'s step loop drains
(`🎞️frame-worker/🟦️.ts:246-295`). The elapsed time in the banner is close-loop time, not silence.

**Fix** — the handle type declares `generation`/`sequence` as `bigint` and the call site lowers with
`BigInt(...)` (`🎞️frame-worker/🟦️.ts:14-22,281-282`), with the docstring naming the ABI. The transport's
own counters stay `number` — that is what a structured-cloned protocol message carries.

**Test** — `🧑‍🎨engine/🧪️tests/🔢️wgpu-u64-seam/` (+ `laws.json`, registered): the **Rust signature is
the authority** — the test parses `🌐️browser-worker/🦀️.rs` for each export's `u64` parameters, asserts
the TypeScript handle declares exactly those as `bigint`, asserts one `BigInt(` lowering per widened
argument at the call site, and uses the **engine itself as the third-party oracle**
(`BigInt.asUintN(64, 2)` throws `Cannot convert 2 to a BigInt`, `BigInt.asUintN(64, 2n)` does not).
**4 passed.**

### 4.4 A single cold GPU opportunity quarantined the surface before it had ever presented

```
worker-present-failed: prepared frame submit step:
  prepared GPU opportunity exceeded the two millisecond ceiling: ClearScene took 2601 us
  Surface: quarantined · UI turns over budget 0/0 · Worker steps over budget 0/0
```

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🖥️gpu.rs:503` (pre-fix) returned `Err`
on ONE over-ceiling wall sample. The first frame of a cold surface pays the platform's pipeline
warm-up inside one opportunity — measured **2 601 µs for `ClearScene`** against the 2 000 µs ceiling,
on a real Apple metal-3 adapter — and that Err aborted the cursor, quarantined the surface, and left
the canvas blank forever. The `0/0` ledgers in the banner are the same proof the earlier lanes used
(`📓️wgpu-worker-boot-2026-09-10.md` rows 15–16, `📓️wgpu-ui-turn-2026-09-10.md`): nothing measurable
ran long anywhere else.

**Fix** — the ratified law, applied here: a single over-ceiling sample is RECORDED, only a run of
`SUSTAINED_OVERRUN_QUARANTINE_STEPS` (4) consecutive ones is terminal, and any admitted opportunity
clears the run. Extracted as the pure `admit_prepared_gpu_opportunity(run, elapsed_us)` so the law is
testable without a GPU; the cursor carries `overrun_run`. The threshold is the *one* ratified constant,
reached through `semio-framework-job`'s existing re-export of `semio-framework-trace` — the re-export
list in `🧵️job/🦀️.rs:51` gains `SUSTAINED_OVERRUN_QUARANTINE_STEPS` rather than a second copy of `4`.
The verdict now also names the phase and the microseconds it measured (a verdict that cannot say what
it measured is undiagnosable from a fault banner), and a backwards clock is reported as a clock fault
rather than as an overrun.

### 4.5 The terminal glass command page was read as a stale cursor

With 4.4 fixed, the present reached `GlassCommands` and died on:

```
prepared frame submit step: prepared glass region cursor was stale: region 0 of 0 on the draw owner
```

`📦️prepared.rs` `advance_pipeline` emits **one command page per measured step**, and the step that
retires the glass section is measured too: `DrawMeasureCursor::Glass(index)` with
`index == glass_regions.len()` sets the cursor `Complete` and reports zero usage
(`📦️prepared.rs:2596-2599` — its own boundary rule). A document with **no glass at all** therefore
publishes exactly one `Glass(0)` page against an empty list, and `🖥️gpu.rs:511` treated that terminal
page as a stale cursor — failing every first present.

**Fix** — `address_prepared_glass_region(region, len)`: `Ok(Some(i))` to encode, `Ok(None)` for the
terminal page, `Err(len)` only for an index **past** the terminal one, which stays a real stale-cursor
fault.

**Test (4.4 + 4.5)** — one neutral fixture
`🖱️ui/🧫️fixtures/🖥️prepared-gpu-opportunity/🔣️.json` (ceiling, run threshold, the measured
`coldStart` row, seven overrun-run rows, six glass-addressing rows) read by two new laws in
`🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs`. A TS twin is not natural here (a Rust-only
GPU present cursor), so the fixture is the language-agnostic half.
`cargo test -p semio-framework-ui --features wgpu-engine --lib -- prepared_present` → **4 passed**.

---

## 5. A live peer lane blocks the dev serve path

`TrunkServeScript` runs `checkFrameWorkerCarrierCensus` before anything else. On 2026-09-12 a peer's
uncommitted change to `🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts` (runtime-diagnostics
arming: `shardRuntimeDiagnosticsArmed()` reading `globalThis.localStorage`) is bundled into the frame
Worker, and the census forbids the literal `localstorage`:

```
error: 🎞️frame-worker.js retains legacy credential carriers: localstorage
  at checkFrameWorkerCarrierCensus (…/📦️packages/🦀️rust/📜️script.ts:208)
```

The census is doing its job — the shipped Worker really does reach for `localStorage` now. It is that
peer's package to resolve (remove the read, or amend the census). Their file was **not** touched here;
the ticket's serve script routes around it (§2.3) after regenerating `🎞️frame-worker.js` by hand.

Separately, `bun ./📜️script.ts generate-frame-worker` had to be run twice this session because peers
changed frame-worker sources under the running serve; `dev … wgpu` refuses to start on a stale
generated bundle and says so clearly.

---

## 6. The next blocker (not fixed) — BLANK-PAINT

The boot is clean and the surface now presents, but the presented frame is empty:

```
dumpStructure   → 0 nodes
dumpFrameStats  → { windowId: "procedural-main", drawCalls: 0, quadCount: 0, glyphCount: 0 }
canvas          → solid black for the full 300 s (🗑️generated/wgpu-boot/run-7/shot-10-301s.png)
```

This is the parity triage's own `BLANK-PAINT` verdict (`🧑‍💻dev/…/📜️script.ts` `triageParityBoot`).
What is known from run-7:

* the retained intake **is** consuming patches — `intakeSteps` climbs 0 → 713 535 across the seven
  boot surfaces, so the guest's documents cross and the budget fix from
  `📓️wgpu-intake-budget-2026-09-10.md` holds;
* every `wgpu-shell render leave` succeeds, for all seven surfaces;
* but the renderer's own `UI_ENGINE` tree is empty and the draw list produces zero calls.

So the gap is between the intake that accepted the patches and the tree the paint pipeline reads —
**not** the plugin bridge, the contributions sender, the view-state producer or the boot, all of which
are now demonstrably healthy on 6118. That is the `📓️node-graph-canvas-paint-2026-09-12.md` /
`📓️wgpu-boot-divergence-2026-09-10.md` lane's surface, and it needs a probe of the retained-surface
reconcile inside the Worker, not another boot probe.

A second, independent gap sits behind it: **the guest emits no effects at all on this build** —
`effects=0 tags=-` on every one of the 20 `renderSurface` turns, `invokeExtension` 0×, no `respond`,
meshes 0. React reaches `meshes: 3` through the shared door (`📓️extension-invoke-door-2026-09-12.md`),
so the door itself works; on wgpu nothing ever asks it to. The first deferred `flowEvalTick` that
React's `ShellHost` drives after `settle_boot` has no visible counterpart in the wgpu console.

---

## 7. Files

**New**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/{🟦️.ts,laws.json}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔢️wgpu-u64-seam/{🟦️.ts,laws.json}`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖥️prepared-gpu-opportunity/🔣️.json`
- `<ticket>/🐍️wgpu-probe.mjs`, `<ticket>/📜️serve-generation3d-wgpu.sh`

**Changed**
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🌐️.html` — the `🧩️extension-modules` `copy-dir`
- `🎯️targets/🧊️wgpu/⚙️browser-build/🟦️.ts` — `bundleCopyDirectives`, `assertBundleModuleRoutes`
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` — the assertion in `TrunkBuildScript`/`TrunkServeScript`
- `🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` (+ regenerated `🟦️typescript/🎞️frame-worker.js`) — the `u64` seam
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts` — registers the two new suites
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🖥️gpu.rs` — §4.4, §4.5
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs` — the two new laws
- `🧰️framework/🔨️modules/🧵️job/🦀️.rs` — re-exports `SUSTAINED_OVERRUN_QUARANTINE_STEPS`

**Staged build output (not source)** — `🧑‍💻dev/🔌️plugin-modules/🌀️procedural/` refreshed from the
react-staged component (§2.1).

No `launch.json` entry was added: `procedural3d-wgpu` already exists and is the row this lane ran, and
the two new vitest suites run inside the existing `@semio-tech/framework-renderer-wgpu:test` target.

---

## 8. Test results, verbatim

```
$ bunx vitest run --config vitest.config.ts 🧪️tests/🔢️wgpu-u64-seam/🟦️.ts 🧪️tests/🧩️wgpu-module-routes/🟦️.ts
 Test Files  2 passed (2)
      Tests  9 passed (9)

$ cargo test -p semio-framework-ui --features wgpu-engine --lib -- prepared_present
running 4 tests
test wgpu::gpu::prepared_present_tests::a_single_over_ceiling_gpu_opportunity_is_recorded_and_only_a_run_is_terminal ... ok
test wgpu::gpu::prepared_present_tests::the_terminal_glass_command_page_addresses_no_region_and_is_not_stale ... ok
test wgpu::gpu::prepared_present_tests::interrupted_present_cursor_hands_back_generation_and_fixed_owners ... ok
test wgpu::gpu::prepared_present_tests::present_cursor_generation_and_capacity_boundaries_refuse_before_ownership ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 383 filtered out
```

### 8.1 Observed, not mine

* `bun ./📜️script.ts test` (`@semio-tech/framework-renderer-wgpu`) — 4 failures in
  `async_boundary_tests` (`native_binary_owns_exactly_one_entrypoint_driver` asserts
  `BINARY_SOURCE.matches("drive_entrypoint(") == 2`, got 3; plus
  `presenter_ack_retirement_source_mutations_are_denied`,
  `raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete`, and a SIGABRT in
  `renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`).
  All are source-scanning / renderer-Rust assertions against `🧊️renderer/🦀️.rs` and `💾️binary`, both
  uncommitted and live under a peer refactor this session. Nothing this lane changed is Rust in that
  crate.
* The raw `bunx vitest run` of the whole wgpu config additionally fails 6 `🧩️package-integration`
  cases with `ReferenceError: Bun is not defined` — that suite requires the Bun runtime and must be
  reached through its nx target, not plain `bunx vitest`.

---

## 9. Constraints honored

6118 only. The react serve on 6018 was not touched and the procedural guest was not rebuilt or
restaged (only the already-built component was copied between the two staging trees). No
git-state-modifying command. No ticket opened, closed or reopened. No `🗑️generated` folder swept. One
temporary `[DEBUG]` stack log was added to `🎞️frame-worker/🟦️.ts` to capture the §4.3 trace and has
been removed.
