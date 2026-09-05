# WGPU-wasm tier readiness for Wave 3 (`🛠️dev🖥️s🧊️wgpu🌐️wasm`, port 6066) — 2026-09-05 19:00-ish

Read-only, current-source pass (file:line citations below). Several files in this area are under
active concurrent edit (`git status` shows `M`/`A` on the wgpu `📜️script.ts`/`📋️project.json`,
`🎞️frame-worker.js`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `🧊️renderer/🦀️.rs`, `PluginRuntime`, `ShellHost` —
a `sol`-lane retained-Home-ACK/presence packet per `📓️explore-wgpu-and-native-shells.md`'s caveat) —
re-read before editing.

## 1. `renderer === "wgpu"` pipeline (current source)

**Dev package (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`, `DevScript.run`):**
- Plugin-crate build is **not** skipped for wgpu the way `buildEngineWasm` is. Branch at
  `:1250-1290`: `streamPluginBuilds = renderer === "react" && SKIP_PLUGIN_BUILD !== "1"` — always
  `false` for wgpu. So unless `SKIP_PLUGIN_BUILD=1` is set, wgpu's `dev` falls into the blocking
  `else` at `:1287-1290`: `await buildPlugins(filterPlugin); await buildEngineWasm(plugin, renderer)`.
  `buildPlugins("s")` (`:573-583`, host filter expands to **all 59 catalog crates**, `wasm32-wasip2`,
  `PLUGIN_WASM_TARGET` at `:83`) is all-or-nothing — `assertPluginCatalogComplete` (`:497-499`) throws
  if any crate fails, unlike React's streaming boot which tolerates per-crate failure.
- `buildEngineWasm` (`:932-940`) is a **hard no-op for wgpu**: `:940` `if (renderer !== "react" || …)
  return;`. The wgpu engine's own wasm (the renderer crate `semio-framework-os-renderer-wgpu`,
  `wasm32-unknown-unknown`) is built entirely by the delegated wgpu-package `serve`/`wasm` command
  below, not by this dev-package function.
- wgpu branch (`:1291-1334`): resolves the play URL, probes the port, and if not already serving,
  shells to `bun <wgpu-package>/📜️script.ts serve` (`:1319-1329`) with
  `SEMIO_PLUGIN/SEMIO_RENDERER/S_OS_PORT` forwarded, `cwd` = the wgpu package dir. **Bug found**:
  `BuildScript.run` (ship build, `:1372-1376`) resolves the wgpu script at a *different, stale* path —
  `…/🧑‍🎨engine/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` (missing the `🎯️targets/` segment). Verified this
  path does not exist on disk (`ls` → "No such file or directory") while the DevScript path
  (`:1319`, with `🎯️targets/`) does. **Only `SEMIO_BUILD_MODE=ship`'s `bun ./📜️script.ts build --
  <wgpu-variant>` is affected — `dev`/`serve` (Wave 3's actual path) use the correct path.** Flag for
  Opus: `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1373` needs the same `🎯️targets/🧊️wgpu/` segment as
  `:1319`, or `BuildScript`'s wgpu ship path is dead on arrival.

**WGPU package (`…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`):**
- `TrunkServeScript.run` (`:257-273`, registered as `serve`/`dev`, both `dependsOn: ["generate-frame-
  worker"]` in `📋️project.json`): `ensureTrunk()` (installs `cargo install trunk` if missing, `:127-
  132`), `ensureWasmTarget()` (`rustup target add wasm32-unknown-unknown`, `:117-122`),
  `buildBootScript` (in-memory `Bun.build` → writes `🟦️typescript/🚀️boot.js`, `:213-216`),
  `checkFrameWorker` (`:235-239`, **throws if the committed `🎞️frame-worker.js` differs from a fresh
  in-memory render** — confirmed currently RED, see §4), `ensureAssetServer("s")` (no-op: the `s`
  catalog row declares `assets: []`, `🎮️playgrounds.ts:72`), then `trunk serve --config Trunk.toml
  --port <port>` as a real subprocess (`:270-272`) — this is the actual `cargo build` +
  `wasm-bindgen` step, budgeted only by `buildBudgetMs()`/daemon defaults, not the dev-package's own
  budget.
- `Trunk.toml:1-4`: target `🌐️.html`, `dist` → `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu`
  (relative path resolves correctly). `[watch]` (`:6-7`) covers the renderer/engine/browser-boot/
  frame-worker/interactive-job-port/frame-transport dirs plus `🟦️typescript`, `🌐️.html`, and
  `🧑‍💻dev/🔌️plugin-modules` (a plugin rebuild also restarts trunk's dev server).
- `🌐️.html`'s `data-trunk rel="rust"` builds the crate with `wasm-bindgen` (`web` target,
  `wasm-opt=z`) and copies `🔌️plugin-modules`/`🖼️assets`; the real entry script is `🚀️boot.js`, not the
  wasm module directly (confirms `📓️explore-wgpu-and-native-shells.md`'s finding, unchanged).
- `NativeBuildScript`/`NativeRunScript` (native tier, `:301-355`) are unaffected by anything above;
  restated only for completeness — `attach_backbone` native stub is out of this wave's scope.

**SKIP flags**: `SKIP_PLUGIN_BUILD=1` is honoured (routes to `ensurePluginRegistry` +
no-op `buildEngineWasm`, skipping the blocking `buildPlugins`) — this is exactly what
`startParityDevServer` sets for the `parity` harness's own wgpu boot (`:4008-4013` region, see §3).
`SKIP_ENGINE_BUILD` has **no effect on the wgpu path** — it only gates the already-no-op
`buildEngineWasm` branch, so it is a dead knob for this renderer. **`SKIP_WGPU_BUILD`** exists but is
only read by `BuildScript` (`:1372`, the ship path with the broken script reference above) — the
`dev`/`serve` path has no skip-the-trunk-build knob at all; `serve` always attempts a real build.

## 2. Same plugin cache / shard worker as React — CONFIRMED YES, same JSPI requirement

- `pluginOutRoot = PLUGIN_MODULES_ROOT` (`🔌️vite-plugins.ts:20`) = `…/🧑‍💻dev/🔌️plugin-modules` — the
  wgpu package imports the **same constant** (`📜️script.ts:34`, `pluginOutRoot`) and passes it as
  `SEMIO_PLUGIN_MODULES` to the native runner and reads it for asset sync; the browser tier fetches
  plugin bundles from the same `/🔌️plugin-modules/` mount both Vite (react) and Trunk's static-file
  copy (wgpu, `🌐️.html`'s `data-trunk rel="copy-dir"`) serve.
- `publishShardWorker()` (`🟦️.ts:75-79`, called by both `buildPlugin`/`buildPluginCatalog` regardless
  of renderer) writes the **one** shard worker to `🔌️plugin-modules/🧵️shard/🟨️shard-worker.js` —
  identical artifact for both shells.
- wgpu's plugin bridge (`…/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts`) does **not**
  reimplement worker management: `getShardClient()` (`:74-83`) calls `createPooledActorRuntime` from
  `🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts`, which constructs `ShardClient` from
  `🎭️actor/📮️shard-client/🟦️.ts` — the exact same module `PluginRuntime.tsx` (React) uses. `ShardClient`'s
  default `createWorker` (`📮️shard-client/🟦️.ts:52`) opens `new Worker(SHARD_WORKER_URL, {type:
  "module"})` where `SHARD_WORKER_URL = "/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js"`
  (`📮️shard-client/🟦️.ts:22`) — the corrected emoji path baseline-runtime.md's 15:30 entry recorded as
  the fatal-boot fix for React applies identically here; wgpu was never on the stale ASCII path
  because it shares the literal constant, not a re-typed string.
- **JSPI requirement is identical and unavoidable**: the shared `shardWorkerSource()` unconditionally
  constructs `WebAssembly.Suspending`/`WebAssembly.promising` at worker module top level
  (`📮️shard-client/🟦️.ts:519-520`, `SHARD_JSPI_FAULT_CODE`/`SHARD_JSPI_FAULT_TEXT` at `:524-530`) — a
  browser without JSPI fails every shard, wgpu included. `ParityVerifyScript`'s own Playwright launch
  args (`🧑‍💻dev/…/📜️script.ts` `verifyParityVariant`, `chromium.launch({ args: ["--use-angle=swiftshader",
  "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] })`) do **not** pass a JSPI flag; per
  baseline-runtime.md this is currently moot (headless Chromium 151 exposes JSPI by default on this
  machine) but is worth a `--enable-features=WebAssemblyJavaScriptPromiseIntegration` addition if a
  different Chromium build is ever pinned.
- Plugin runtime shape difference (unchanged from `📓️explore-wgpu-and-native-shells.md`, re-verified
  by direct read): wgpu's bridge is a **narrower** `AppChannelClient` consumer (only
  `manifest/createApp/destroyApp/handleAction/handleCommand/render/contextMenu`, `🐚️plugin-bridge.ts`
  header comment + `WgpuPluginHandle` interface) and uses a **plain per-actor promise chain**
  (`submitTurn`, `:100-107`) instead of React's coalescing `TurnScheduler` — a deliberate, documented
  simplification, not a bug.

## 3. On-disk freshness and the `parity verify s` prerequisite chain

**Freshness (mtimes, this machine, 2026-09-05 ~19:00):**
| Artifact | State |
|---|---|
| Trunk dist (`.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu`) | **Empty** — directory only, mtime 2026-09-03 02:13. No wasm-bindgen output, no `.js`/`_bg.wasm` on disk. No completed `trunk build`/`serve` since at least that date, consistent with `📓️explore-wgpu-and-native-shells.md` §1's "no successful wgpu-wasm boot on record." |
| Newest `.rs` under `…/🎯️targets/🧊️wgpu/` (engine crate) | `🧊️renderer/🦀️.rs` 2026-09-05 05:07:53 |
| `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | 2026-09-04 01:54:32 (unchanged this session) |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (shared wasm+native shell, retained-Home-ACK site) | **2026-09-05 17:13:21** — actively edited today, matches the git-modified state and the `sol`-lane in progress per the caveat above |
| `🎞️frame-worker.js` (checked-in browser worker bundle) | `git status`: `M` (modified in worktree, not matching a fresh render — see §4) |

Every `.rs` file under the wgpu target tree is newer than the empty dist cache, i.e. **the wasm
artifact on disk (there is none) predates every current source file** — any claim of a working wgpu
boot from before today is stale by construction.

**`parity verify s` exact prerequisite chain** (`ParityVerifyScript` → `verifyParityVariant`,
`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:4254-4273` calling into `:4013-4062`; router entry
`:5163-5171`, Nx `parity` target = `bun ./📜️script.ts parity`,
`🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:125-135`):
1. `prebuildParityPlugin("s")` (`:3944-3979`) spawns `bun 📜️script.ts plugin s` — this is
   `PluginBuildScript` → `buildPlugins("s")`, i.e. **all 59 crates must build clean to
   `wasm32-wasip2`**, all-or-nothing (`assertPluginCatalogComplete`). Budget:
   `PARITY_DEV_SERVER_BOOT_BUDGET_MS` (default 900 000 ms / 15 min, `:3936`, overridable via
   `PARITY_BOOT_BUDGET_MS`). **This is the same all-59-crates bar as Wave 2's `catalog-rebuild`** —
   `parity verify s` cannot get past step 1 until that lane's rebuild is complete and green.
2. `startParityDevServer("react", "s", ports.react)` (`:3981-4008`) — spawns `bun 📜️script.ts dev`
   with `SKIP_PLUGIN_BUILD=1 SEMIO_RENDERER=react` on a free port from the 49-shard pool starting at
   7300 (`findFreeParityPortPair`, `:3928-3934`) — **not** port 6070, so it doesn't collide with a
   manually-run `dev s served`. Same boot budget. This must reach the same "shard 0 heartbeat / router
   / stale-core-ABI" state React's own port-6070 boot is currently fighting (per `📓️baseline-
   runtime.md`'s 18:35 log) — `parity verify s` inherits every React-tier boot blocker, not just wgpu's.
3. `startParityDevServer("wgpu", "s", ports.wgpu)` (same function, `renderer="wgpu"`) — this recurses
   into the DevScript wgpu branch from §1 with `SKIP_PLUGIN_BUILD=1` (so it correctly skips step 1's
   `buildPlugins` a second time) and shells to `TrunkServeScript`. **This is where `checkFrameWorker`'s
   current RED (§4) aborts the whole `parity verify s` run before `trunk serve` is even invoked** —
   confirmed by reading `TrunkServeScript.run` (`:262`, unconditional `await checkFrameWorker(this.root)`
   before the `trunk serve` call at `:270-272`).
4. `triageParityBoot` on both pages waits for `#semio-wgpu-canvas` (`document.querySelector`,
   `PARITY_BOOT_TIMEOUT_MS`) then `window.wasmBindings.dumpStructure` (the wgpu Introspection region) —
   needs a real, successful wasm boot, not just a listening port.
5. `compareParityStructural`/`compareParityRegion`/`runParityProbe` then diff the two dumps; results
   write to `parity-report-v2.{json,md}` under `PARITY_OUT_DIR` (defaults to a different ticket's
   folder, `26/07/11/WGPU-RENDERER-FULL-PARITY` — `parityOutDir()`, `:4041-4045` — an Opus run should
   set `PARITY_OUT_DIR` into this ticket's `🗑️generated/` instead of the stale default).

## 4. Cheap gates — registered in `📋️project.json`, run in the foreground (no cargo, no builds, read-only where noted)

| Target | Command | Nature | Result (this run) |
|---|---|---|---|
| `check-frame-worker` | `bun ./📜️script.ts check-frame-worker` | pure TS, read-only (in-memory `Bun.build` compare, no write) | **RED** — `error: 🎞️frame-worker.js is stale; run the generate-frame-worker target` (`📜️script.ts:238`), consistent with `git status` showing that file `M` |
| `directory-retained-home-bootstrap-source-check` | `bun ./📜️script.ts directory-retained-home-bootstrap-source-check` | pure TS, reads fixture JSON + Ajv schema + text-scans `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` for markers, no cargo | **GREEN** — `checks=39 clean` |
| `normalized-presence-rows-source-check` | `bun ./📜️script.ts normalized-presence-rows-source-check` | pure TS, text-scan of the same shell file | **GREEN** — `checks=6 clean` |
| `lint` | `bun ./📜️script.ts lint` | pure TS, regex-scans `🧊️renderer/🦀️.rs` for raw color constructors | **RED** — 3 violations: `🦀️.rs:15240-15242` (`Rgba::from_srgb8` for `JobProgressKind::CommitValidated/Cancelled|Fault/_`) outside the `ui_wgpu::Theme` module |
| `native-environment-check` | `bun ./📜️script.ts native-environment-check` | pure TS, spawns a poisoned Node child + text-scans the native entrypoint `.rs`, no cargo | **GREEN** — "poisoned ordinary runner sanitized and binary fail-closed guard precedes credential/plugin activation" |
| `preview-generated` | `bun ./📜️script.ts preview-generated` | pure TS, in-memory render, prints JSON to stdout only (no file write) | **GREEN** (exit 0, valid `wgpu-frame-worker` contract JSON) |
| `test-browser-worker` | `bun ./📜️script.ts test-browser-worker` | vitest, pure TS (browser-frame-transport + interactive-job-port protocol tests) | **GREEN** — 2 files, 32/32 tests passed, 2.21 s |
| `test-preview-generated` | `bun ./📜️script.ts test-preview-generated` | vitest, pure TS (`🧩️package-integration.ts`) | **RED (infra)** — killed by the 15 000 ms test budget: `[budget] … exceeded 15000ms — killed`; not a logic failure, needs re-leveling (quick/long) or a faster fixture, same class of issue `test-quick` has elsewhere in this ticket |

**Deliberately not run** (would write tracked production files, out of this explorer's read-only
scope): `generate-frame-worker` (writes `🎞️frame-worker.js`) and `check-browser-worker` (calls
`buildBootScript`, writes `🚀️boot.js`) — both are one-line, no-cargo fixes an implementer should run
directly rather than have this pass mutate source. **Not run** (require cargo, explicitly forbidden
for this pass): `test`, `test-native`, `directory-retained-home-bootstrap-native-check`,
`normalized-presence-rows-native-check`, `wasm`/`build`, `serve`/`dev`, `native`/`native-build`.

Cross-reference: `stdio-check-census.md`'s 18:21 update confirms the repo-wide `semio-framework-os-
kernel` `E0432 DirectorySpaceDetailV1` blocker (which was blocking **every** Rust check, including
this crate's dependency graph) was repaired ~04:50 today and `semio-s-plugin-stdio` now compiles
clean on both native and `wasm32-wasip2` in `target-s-e2e`. **Not independently re-verified for
`wasm32-unknown-unknown`** (the wgpu engine's own target) in this pass — that is a distinct target
from the two the census checked and should be spot-checked (e.g. `cargo check -p semio-framework-os-
renderer-wgpu --target wasm32-unknown-unknown` in a private `CARGO_TARGET_DIR`) before assuming the
whole ~40+ crate dependency graph compiles for Trunk.

## 5. Dependency-ordered checklist for the Opus implementer

1. **Fix the stale `🎞️frame-worker.js`** (blocks every wgpu `dev`/`serve`/`parity verify` invocation
   outright, per §3 step 3). Run `bun nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker`
   (writes the file) then `check-frame-worker` to confirm green. Cost: seconds, no cargo.
   Owner: `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` (`generateFrameWorker`/`checkFrameWorker`,
   `:217-239`), artifact `…/🟦️typescript/🎞️frame-worker.js`.
2. **(Parallel, non-blocking but should not ship red)** Fix the 3 `lint` violations at `🧊️renderer/
   🦀️.rs:15240-15242` — route the three `Rgba::from_srgb8` literals through `ui_wgpu::Theme` instead
   of constructing colors inline. Cost: minutes. Owner: same file, `JobProgressKind` match arm.
3. **Fix `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1373`'s stale wgpu-script path** (missing
   `🎯️targets/`) in `BuildScript` so `SEMIO_RENDERER=wgpu bun ./📜️script.ts build` doesn't immediately
   throw "module not found" — low priority for Wave 3 (dev/serve path is unaffected) but cheap and
   currently dead code.
4. **Confirm `wasm32-unknown-unknown` compiles for the wgpu engine crate** now that the kernel
   `E0432` blocker is fixed (§4 cross-reference) — a plain `cargo check -p semio-framework-os-
   renderer-wgpu --target wasm32-unknown-unknown` in a private `CARGO_TARGET_DIR` (never the shared
   `target/`) before attempting a full Trunk build; this is the cheapest way to catch a compile break
   without paying for `wasm-bindgen`/`wasm-opt`.
5. **Wave 2's catalog rebuild must land** (all 59 plugin crates → `wasm32-wasip2`, fresh owner
   descriptors) — `parity verify s`'s own `prebuildParityPlugin` step is all-or-nothing on this same
   set (§3 step 1); this item is shared with and gated by the React tier's Wave 2 lane, not new work
   for this wave. Do not duplicate; coordinate with lane F.
6. **First real wgpu boot attempt, standalone** (isolate wgpu-specific failures from React-tier
   ones before running the combined harness): `SEMIO_PLUGIN=s SEMIO_RENDERER=wgpu S_OS_PORT=6066 bun
   nx run @semio-tech/framework-os-dev:dev` (the registered `🛠️dev🖥️s🧊️wgpu🌐️wasm` launch entry,
   `.vscode/launch.json:2488-2500`) in the foreground with a generous budget — expect the first
   attempt to pay for a full `cargo build`+`wasm-bindgen`+`wasm-opt=z` of the ~40+ crate dependency
   graph (no completed build on record since 2026-09-03, dist cache empty). Watch for the same shard/
   router/stale-core-ABI classes of fault the React tier hit (§2 — identical `ShardClient`/JSPI
   machinery), plus wgpu-specific ones (`#semio-wgpu-canvas` mount, `window.wasmBindings.
   dumpStructure` presence).
7. **`bun nx run @semio-tech/framework-renderer-wgpu:directory-retained-home-bootstrap-native-check`
   and `:normalized-presence-rows-native-check`** (cargo-backed, currently only source-checked green
   in this pass, §4) — cheap relative to a full Trunk build (single-crate `lib` target laws via
   `runExactCargoLaws`), should run once step 4's target check is confirmed clean, well before the
   full `serve`.
8. **`bun nx run @semio-tech/framework-os-dev:parity -- verify s`** with `PARITY_OUT_DIR` redirected
   into this ticket's `🗑️generated/` (default points at a different ticket, §3 step 5) and a raised
   `PARITY_BOOT_BUDGET_MS` for a cold multi-crate build. Expect this to re-exercise every item above
   in one shot (it independently boots both react and wgpu); treat any `STALE-BRIDGE` boot status per
   `isParityStaleBridge` (`:4390` region) as "bridge needs regeneration," not a structural failure.

**Estimated cost**: steps 1-3 are seconds-to-minutes, no cargo. Step 4 is one `cargo check` on one
crate/target (likely 5-20 min cold, given the ~40+ crate graph). Step 5 is the large shared cost
(Wave 2, already budgeted at 4h in the plan). Step 6 is the first full Trunk build — budget it like a
cold build (no prior receipt to estimate from; the wgpu wasm target has never finished a build this
week). Steps 7-8 are incremental once 6 succeeds.
