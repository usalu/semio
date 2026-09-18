# Build / verify / describe / register / boot pipeline for a plugin — evidence for the WFC extraction

Read-only exploration. Primary evidence: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/` (`📓️status.md` read in full, its
`📜️*.sh` scripts, `📓️explore-dev-boot-ts-storybook.md`, `📓️w11-integration.md`, `📓️w7a-synthetic-e2e.md`, all five `🐍️*.mjs` probes),
cross-checked against the live repo (`📜️script.ts` at root, per-plugin `📋️project.json`, `.vscode/launch.json`, `.claude/launch.json`,
`package.json`, `.cargo/config.toml`), plus `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/` — the direct ancestor
ticket for the plugin WFC is being extracted FROM (`📜️lib-suite-run.sh` / `📜️lib-deadlock-probe.sh`, `--features component-app-assembly`,
`RUST_MIN_STACK=33554432`, against artifact crate `semio-s-artifact-procedural-generation3d`).

No plugin directory exists yet at `✏️s/🔌️plugins/🌊️wfc` — everything below is the general pipeline plus the remodel/procedural
precedent to clone.

---

## 0. Shared build dir (read this before any cargo command)

`.cargo/config.toml`:
```
target-dir = ".🧬semio/🦑️repo/⚡️cache/cargo/target"
build-dir  = ".🧬semio/🦑️repo/⚡️cache/cargo/build"
```
One build-dir under `⚡️cache` for the whole repo, fine-grain-locked, shared with every peer session/agent. Never point
`CARGO_TARGET_DIR` at a private path for anything but a throwaway warm-up lane — the real deliverables and locks live here.
Expect SIGKILL (exit 137) under peer memory pressure; every ticket script below retries on 137 with a sleep, never on other
exit codes (a real compile error must NOT be retried).

---

## 1. Native `cargo check` / `cargo test` per crate

### 1a. Plugin crate check (fast, no features)
```
cd /Users/ueli/Documents/semio
cargo check -p semio-s-plugin-remodel --lib --tests --message-format=short
```
Verbatim from `📜️check-remodel-native.sh`. Confirmed green in the ticket log (2026-09-16): "passes clean (6m57s under load ~100)".
For wfc: `cargo check -p semio-s-plugin-wfc --lib --tests --message-format=short`.

### 1b. Artifact crate check — needs `component-app-assembly`
```
cargo check -p semio-s-artifact-remodel-remodeling --lib --tests -j 4 --message-format=short
```
(`📜️check-remodel-artifact-tests.sh`). Remodel's artifact crate check does NOT pass `--features component-app-assembly` in this
particular ticket's script, but the sibling PROCEDURAL-3D ticket's equivalent lane (the crate WFC is extracted from) always does:
```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-run --message-format=json
```
(`📜️lib-suite-run.sh:15`, `📜️lib-deadlock-probe.sh:14`). **Use `--features component-app-assembly` for every wfc artifact crate**
(`semio-s-artifact-wfc-bitmap`, `-grid2d`, `-graph2d`, `-grid3d`, `-graph3d` or whatever the final split is) — this is the
feature gate that turns on the app-assembly test/editor code paths; omitting it is why some crates type-check but their tests
never actually exercise the editor.

### 1c. Native unit tests, with SIGKILL retry
```
cargo test -p semio-s-artifact-remodel-remodeling --lib -j 4 -- --test-threads=4
cargo test -p semio-s-plugin-remodel --lib -j 4
```
(`📜️test-remodel-native.sh`) — looped 6× with `sleep 90` between attempts, breaking only when exit code ≠ 137.

Focused/filtered variant (generic, parameterized):
```
📜️test-remodel-filtered.sh <log-name> [--ignored] <test filters…>
# → cargo test -p semio-s-artifact-remodel-remodeling --lib -j 2 -- --test-threads=4 --nocapture "$@"
```
10 retry attempts, `sleep 120` on 137. Clone this shape for wfc as `📜️test-wfc-filtered.sh`.

### 1d. Stack-overflow guard: `RUST_MIN_STACK`
Not present in the remodel ticket's own scripts, but the direct ancestor (`PROCEDURAL-3D-END-TO-END`) sets it on every
`--lib` test run of the artifact crate WFC is being cut from:
```
export RUST_MIN_STACK=33554432   # 32 MiB
```
(`📜️lib-suite-run.sh:13`, `📜️lib-deadlock-probe.sh:12`, both before `cargo test -p semio-s-artifact-procedural-generation3d`).
Deep recursive engine code (procedural generation, WFC propagation) can blow the default 8 MiB test-thread stack — export this
before any `cargo test --lib` on a wfc artifact crate that inherits deep recursion from procedural.

### 1e. Build-then-stream-run pattern for a stalling suite
`📜️lib-suite-run.sh` / `📜️lib-deadlock-probe.sh` (procedural ticket) show the pattern for a suite prone to serial-lock
deadlock: `cargo test … --no-run --message-format=json` to get a JSON stream, extract the compiled test binary's path with a
small inline `python3` (`reason == "compiler-artifact" and target.kind == ["lib"]`), then run that binary directly so stdout can
be watched for a stall (`sample <pid> 3 -file …` + `kill -9` after N seconds of silence). Worth keeping in reserve for wfc's own
propagation/backtracking loops if a suite ever hangs instead of failing.

### 1f. ASCII `[[test]]` names for emoji test paths
Procedural's Cargo.toml (the crate wfc is extracted from) declares separate top-level test targets for anything that must run
outside the `--lib` target, with an ASCII `name` (Cargo requires ASCII/identifier-safe target names) pointing at the real
emoji path:
```toml
[[test]]
name = "boot_deadline"
path = "../../🧪️tests/🔬️boot-deadline/🦀️.rs"

[[test]]
name = "close_ladder"
path = "../../🧪️tests/🚪️close-ladder/🦀️.rs"

[[test]]
name = "idle_turns"
path = "../../🧪️tests/😴️idle-turns/🦀️.rs"
```
(`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:87-105`). If any wfc test file needs its own allocator/global state
(procedural's `idle_turns` installs its own `HeapWitness`) or must run independent of the lib test target, give it a `[[test]]`
block the same way — the `path` may contain emoji, only `name` must be ASCII.

---

## 2. wasm32-wasip2 check/build

### 2a. Plain check
```
cargo check -p semio-s-plugin-wfc --target wasm32-wasip2
```
Same shape used by W11's central check 3 for remodel when the native host crate was blocked by a peer refactor
(`📓️status.md` 2026-09-06 22:50: "the coordinator started central check 3 with `--target wasm32-wasip2`" — remodel only reaches
the framework host off-wasm, so a wasm-target check type-checks the plugin crate independent of host churn).

### 2b. Swap-thrash guard for the wasm dev build
```
export CARGO_PROFILE_WASM_DEV_DEBUG=false
```
Set in `📜️activate-remodel-react.sh:6` alongside `NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=6063`. Memory
`project-wasm-dev-profile-debug-off-and-swap-thrash`: this single env var took a debug rustc invocation from 8.6 GB to 165 MB —
**always export it** before any `bun nx run …:activate-<plugin>-react-dev` or wasm dev build. Effect measured directly on
remodel: core wasm build shrank to 417 KB with the flag vs. a peer plugin's 1.1 MB without it (`📓️status.md` 2026-09-16 01:02).

### 2c. The real os-dev "build a plugin for the browser" pipeline: `activate-<plugin>-react-dev`
There is no bare `bun ./📜️script.ts plugin <x>` — the actual chain the remodel ticket used and proved live is the generated nx
target on the `@semio-tech/framework-os-dev` project:
```
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=6063
bun nx run @semio-tech/framework-os-dev:activate-remodel-react-dev
```
(`📜️activate-remodel-react.sh`, ports/env verbatim). This is what restages the guest wasm (`component-dev`) and materializes the
react dev module (`component-release`/`materialize`) under the dev package's `dist`. It is retried up to 6× with `sleep 90`
whenever the log contains `SIGKILL`/`signal` (host swap exhaustion kills the wasm cargo mid-build); an nx run that already
completed is cached, so a retry only redoes the lost work. Observed timings on remodel: restage #1 11m19s
(`component-dev` 9m34s alone); restage #2 (after `CARGO_PROFILE_WASM_DEV_DEBUG=false` had already been applied once) only 2m29s
because just `semio-framework-plugin` + the two remodel crates needed to recompile for wasm.

For wfc, once the plugin crate + its artifact crates exist, this becomes:
```
bun nx run @semio-tech/framework-os-dev:activate-wfc-react-dev
```
(nx target name follows `activate-<pluginId>-react-dev`; the target is generated per plugin from the plugin's playground
registration in Cargo.toml metadata, not hand-written — confirm it exists after the crate builds and is registered, §3).

### 2d. Dev-mode boot (native trunk/wgpu OR react, driven by env)
```
bun ./📜️script.ts dev remodel                 # == package.json "dev:remodel"
# renderer defaults to wgpu; force react:
SEMIO_RENDERER=react bun ./📜️script.ts dev remodel
```
(`package.json:171` `"dev:remodel": "bun nx run workspace:dev -- remodel"`; confirmed identically live). Root `DevScript.run`
resolves the plugin via `resolvePlaygroundDevApp`/`resolveFrameworkOsPlaygroundPlugin` against the generated playground catalog
(`variant === "remodel"`), then spawns `bun nx run @semio-tech/framework-os-dev:dev -- remodel` with
`SEMIO_PLUGIN=remodel`, `SEMIO_RENDERER = env.SEMIO_RENDERER ?? "wgpu"` — **the default renderer is wgpu, not react**; a bare
`bun run dev:remodel` without `SEMIO_RENDERER=react` boots native `trunk serve` wgpu instead of the browser `ShellHost`.
`SEMIO_PLUGIN_ONLY` narrows which crates get cargo-built but never narrows the browser registry load closure.
For wfc, once registered: `"dev:wfc": "bun nx run workspace:dev -- wfc"` (package.json script analogous to `dev:remodel`; add it,
or just call `bun nx run workspace:dev -- wfc` directly — the script.json entries are conveniences, not required).

Note the source plugin `procedural` uses a subset-qualified variant instead (`bun nx run workspace:dev -- procedural 3d`,
`package.json:172`) because it registers multiple playground variants (`2d`/`3d`). If wfc ships as one plugin with several
artifact kinds (bitmap/grid2d/graph2d/grid3d/graph3d) sharing ONE playground variant, `dev -- wfc` (remodel's flat shape) is
the model; if each artifact kind needs its own dev boot variant, `dev -- wfc <variant>` (procedural's shape) is the model —
decide from how the plugin registers its playground row (Cargo.toml `[[package.metadata.semio.playground]]`, one row per
variant), not by guessing.

### 2e. `component-dev` / `component-release` / `materialize` outputs
Live under the dev package's own `dist`:
```
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/
```
(confirmed by directory listing during this exploration — `dist/build-<plugin>-react-release/…`, `dist/asset/…` subtrees exist
per-plugin, e.g. `dist/build-energy-react-release/…`, `dist/build-fem3d-react-release/…`). The `activate-<plugin>-react-dev` nx
target is what drives `component-dev` (wasm build) → `describeBuiltPlugin`/`stagePluginDescriptor` (self-heals the owner-root
descriptor from source, see §3) → materialize (browser-servable module under this `dist`). `component-release` is the
equivalent for a production/release build; not exercised in the remodel ticket (only `-react-dev` was used).

### 2f. Serve the restaged plugin
```
cd /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6063 SEMIO_PLUGIN=remodel
bun ./📜️script.ts serve remodel react dev
```
(`📜️serve-remodel-react.sh`, exact env). Run detached (`exec > "$LOG" 2>&1` + backgrounding, or a `screen` session as the
ticket log records: `screen remodel-serve`/`screen remodel-activate`) — per memory `feedback-preview-start-servers-vanish-use-nohup`,
prefer `nohup … & disown` with a log file and poll with `curl` over a bare background shell, since Bash-tool background
processes and dev-preview servers both die when their parent turn/agent ends. **Never start these from a subagent** — the
serve/screen must live in the coordinating session (`feedback-start-servers-from-main-session`).

Browser URL: `http://127.0.0.1:6063/?plugin=remodel` (query-param selects the plugin at a fixed dev-server port; wfc will get
its own port pair, see §3).

---

## 3. Descriptor regeneration, registry check, taxonomy check, dependency audit, launch.json entry

### 3a. `describe` — regenerates the owner-root `🔣️.json` + `🛂️.descriptor.semio`
```
bun nx run @semio-tech/remodel-plugin:describe
```
Confirmed wired in `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📋️project.json`:
```json
"describe": {
  "executor": "nx:run-commands",
  "options": { "cwd": "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust", "command": "bun ./📜️script.ts describe", "forwardAllArgs": true },
  "outputs": [
    "{workspaceRoot}/✏️s/🔌️plugins/📸️remodel/🛂️.descriptor.semio",
    "{workspaceRoot}/✏️s/🔌️plugins/📸️remodel/🔣️.json"
  ]
}
```
**Why this matters, proven live on remodel**: the committed owner-root `🔣️.json`/`🛂️.descriptor.semio` are gitignored-generated
snapshots' SOURCE (they ARE tracked in git, unlike the downstream `🤖️generated/🔌️plugins.json` projection which IS gitignored) —
but they go stale the moment source Rust changes (artifact kind id, dialect id, example registry) without a `describe` re-run.
Remodel's committed descriptor was 3 days stale (rename commit vs. a later source touch), carrying `3d.remodel` instead of
`3d.remodeling` and `manifest.apps[].examples: []` — which **directly blocked the react example picker** (ShellHost reads
`activePluginManifest?.examples` straight from the descriptor, `🏛️ShellHost/🟦️.tsx:6418`) until `describe` (or an un-narrowed
dev boot, which self-heals the same way via `materializePlugin`/`describeBuiltPlugin`/`stagePluginDescriptor`) regenerated it.
**For wfc: run `describe` after every source change to artifact kind id / dialect id / example registry, and always once before
trusting the example picker or registry check.**

For wfc: `bun nx run @semio-tech/wfc-plugin:describe` (nx project name follows `@semio-tech/<pluginId>-plugin`, per remodel's
`@semio-tech/remodel-plugin` and procedural's presumable `@semio-tech/procedural-plugin`).

### 3b. Registry check (the DoD-3 gate — "registry check accepts remodel")
```
bun nx run @semio-tech/plugin-registry:check
```
Wired at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json` (`"check"` target →
`bun ./📜️script.ts check`, cwd the registry module). This is the literal command other tickets use as their registry gate
(confirmed via repo-wide grep across multiple ticket `.md`/plan files: `STDIO-ARTIFACTS-AND-IO`, `CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`,
`ARTIFACT-SYSTEM-OVERHAUL…`, `.cursor/plans/*`). Companion regeneration:
```
bun nx run @semio-tech/plugin-registry:generate
```
(regenerates the gitignored `🤖️generated/🔌️plugins.json` / `🤖️generated/🎠️playgrounds.json` projections FROM the owner-root
descriptors — run `describe` first, then `generate`, then `check`).

### 3c. Taxonomy check
```
bun ./📜️script.ts verify taxonomy report --scope <path>     # report only
bun ./📜️script.ts verify taxonomy enforce --scope <path>    # throws on any error finding
bun ./📜️script.ts verify taxonomy implementation report      # separate, repo-wide filesystem walk (slow, has --scope-less progress logging)
bun ./📜️script.ts verify taxonomy implementation enforce
```
(`VerifyScript.runTaxonomy`, `📜️script.ts:8014-8046`, confirmed live). Scope to the wfc plugin root once it exists, e.g.
`--scope "✏️s/🔌️plugins/🌊️wfc"`.

### 3d. Dependency audit gate ("Dependency Truth Gate", memory `project-dependency-truth-gate`)
```
bun ./📜️script.ts verify dependencies literal-external
```
(`VerifyScript.runDependencyFreeze` → `runDependencyVerification`, `📜️script.ts:6951-6955,8445-8449`). Also has
`summary`/`list`/`self-test` sub-modes; `literal-external` is the authoritative "red until zero" mode — run this whenever wfc's
Cargo.toml gains/loses a dependency (e.g. after splitting procedural's WFC code into its own crate, procedural's Cargo deps that
are actually only used by the WFC code must move with it).

### 3e. Full policy rule aggregator (broader than `verify`)
```
bun ./📜️script.ts policy
```
Memory-worthy nuance (from `ARTIFACT-SYSTEM-OVERHAUL…/STATUS.md:64`, independently corroborating what this exploration found):
`verify` (the `VerifyScript` class above) is a DIFFERENT, narrower pipeline (dependency-cruiser + a few nx targets + 2 narrow
policy rules) from `bun ./📜️script.ts policy`, which is the actual full policy-rule aggregator (all ~25 rules, including
plugin-dependency-direction / dialect-literal-path / mutation-vocabulary / trait-impl checks) reached via a separate
early-dispatch path (`dispatchPolicyArgv`) checked BEFORE the `verify`/`os`/`semio` router. **Do not conflate `verify`'s output
with `policy`'s — run both** when auditing a new plugin's policy compliance; `verify`'s `@semio-tech/plugin-registry:check` nx
target happens to reuse some of the same taxonomy-scanning library code, which can make a `verify`-only pass look more complete
than it is.

### 3f. Registered launch.json shape to clone

**`.vscode/launch.json`** (three entries per plugin, generated from `.vscode/🧩️launch.seed.jsonc` — edit the seed, not the
generated file, for a permanent addition):
```json
{
  "name": "🛠️dev📸️remodel📸️remodeling⚛️react",
  "type": "node-terminal", "request": "launch",
  "command": "bun nx run workspace:dev -- remodel",
  "cwd": "${workspaceFolder}",
  "env": { "REMODEL_PLAY_PORT": "6063", "SEMIO_RENDERER": "react" },
  "presentation": { "group": "3_dev", "order": 389 },
  "serverReadyAction": {
    "action": "openExternally",
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6063)",
    "uriFormat": "%s"
  }
},
{
  "name": "🛠️dev📸️remodel📸️remodeling🧊️wgpu🌐️wasm",
  "command": "bun nx run workspace:dev -- remodel",
  "env": { "REMODEL_PLAY_PORT": "6163", "SEMIO_RENDERER": "wgpu" },
  "serverReadyAction": { "pattern": "(...:6163)", … }
},
{
  "name": "🛠️dev📸️remodel📸️remodeling🧊️wgpu🖥️native",
  "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- remodel"
}
```
**Important, confirmed by live grep of `.vscode/launch.json`**: the `<PLUGIN>_PLAY_PORT` env var (`REMODEL_PLAY_PORT` here) is
**dead** — it is not read anywhere in the scripts; only `serverReadyAction`'s own hard-coded port regex does real work. The
actual port is whatever the plugin's Cargo.toml playground metadata declares (baked into the generated
`🎠️playgrounds.json`/`🔌️plugins.json`) and is read by `S_OS_PORT`/the dev server itself. Keep the `<PLUGIN>_PLAY_PORT` var for
cosmetic parity with every other plugin's entry, but do not rely on it — get the real port from Cargo.toml (§3g) and hard-code
it into the `serverReadyAction` pattern, exactly as every existing entry does.

**`.claude/launch.json`** (attach-only entries, used by the Claude_Browser preview tooling and the `mcp__ccd_*` session
tooling — NOT the thing that starts the server):
```json
{
  "name": "remodel-react-attach",
  "url": "http://localhost:6063",
  "port": 6063
}
```
This is a bare `{name, url, port}` triple with no `runtimeExecutable` — it attaches to an ALREADY-RUNNING server (started via
§2f's `serve` invocation, e.g. in a `screen` session) rather than launching one itself. Clone this shape for wfc once its port
is known: `{"name": "wfc-react-attach", "url": "http://localhost:<port>", "port": <port>}`.

### 3g. Port source of truth
```
✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml:17-19   →  react port 6063, wgpu port 6163 (source)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:1097-1110  (generated projection, gitignored)
```
`[[package.metadata.semio.playground]]` rows in the plugin crate's own Cargo.toml are authoritative; the generated
`🎠️playgrounds.json` is a downstream snapshot regenerated by `plugin-registry:generate`. When adding wfc, pick free ports
(remodel: 6063/6163; check neighboring tickets — energy 6106, forms 6058, raster 6060, shooting 6019, layout 6079, note 6080 —
for what is already taken) and declare them in wfc's own Cargo.toml metadata block, then regenerate.

### 3h. How the plugin gets discovered at all (load closure)
Generated registry entry (downstream of the owner-root descriptor, not independent source):
```json
{ "pluginId": "remodel", "packageId": "semio:remodel",
  "cratePath": "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust",
  "capabilities": ["documents.write","ui.dialog"], "contributes": [], "consumes": [],
  "dependsOn": ["stdio"], "activationEvents": ["on-artifact-kind:3d.remodel"],
  "executionMode": "isolated" }
```
`dependsOn` mirrors the plugin crate's own `[dependencies]` on other `semio-s-plugin-*` crates (remodel only depends on
`semio-s-plugin-stdio`). Load closure = transitive plugin-crate dependency closure; `SEMIO_PLUGIN_ONLY=<id>` narrows cargo
builds but not this closure, so a broken/undescribed dependency plugin cascades `plugin.descriptor-unavailable` faults into
every plugin that depends on it. If wfc depends on `stdio` (near-certain, every artifact-bearing plugin does) make sure stdio's
own descriptor is fresh before debugging a wfc boot fault that might actually be upstream.

---

## 4. Browser verification: playwright/mjs probe pattern

All five remodel probes live directly in the ticket folder (not a shared location — copy the pattern, don't import cross-ticket)
and share the same skeleton: `import { chromium } from "playwright"`, launch **headless** with
```js
chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] })
```
**`--use-angle=metal` is load-bearing** (memory `feedback-headless-chromium-swiftshader-webgl`): without it headless Chromium
falls back to the SwiftShader software rasterizer, which is both slow and gives you no signal about real WebGL/WebGPU
performance or correctness — always pass it when probing a 3D/canvas-rendering window (wfc's grid2d/grid3d/graph2d/graph3d
windows almost certainly render to canvas).

Every probe: captures `console`/`pageerror` events into a `lines[]` array with a monotonic `t0`-relative timestamp; polls a
`state()`/`snap()` page-evaluate function that reads `[data-surface-id]` elements for `data-status-json` (phase/fault),
`data-meshes-json` (mesh count), canvas count, and the shell readiness beacon `document.documentElement.getAttribute("data-semio-os-ready")`;
writes `console.txt` + a JSON report + PNG screenshots per step into `🗑️generated/<probe-name>/`. Run with:
```
cd .🧬semio/🦑️repo/🎫️tickets/<wfc-ticket-folder>
SEMIO_PROBE_URL=http://127.0.0.1:<port>/?plugin=wfc SEMIO_PROBE_OUT=wfc-boot bun 🐍️wfc-console-dump-probe.mjs
```
(env vars `SEMIO_PROBE_URL`/`SEMIO_PROBE_OUT`/`SEMIO_PROBE_SECONDS`/`SEMIO_PROBE_GUEST_DIAGNOSTICS` are all read via
`process.env` at the top of each script with sane defaults — clone remodel's files and only the URL/plugin-id/window-id
literals need to change).

Five probe roles, each a template to clone:

1. **Console-dump/boot probe** (`🐍️remodel-console-dump-probe.mjs`, 38 lines) — loads the URL, polls for
   `data-semio-os-ready` + at least one `[data-surface-id]` host, screenshots, dumps console + a `state.json` snapshot (host
   phase/fault/mesh-count/text-length per surface). This is the minimal "does it boot and render non-empty" proof for DoD-4.
2. **Interact probe** (`🐍️remodel-interact-probe.mjs`, 199 lines) — scripted multi-step journey: open a panel by
   `[id="<panel-id>"]` toggle, open the Actions pane via an `[id$=".engagement.toggle"]` engagement toggle, click
   `[id="action.<actionId>"]` then `[id$=".action.<actionId>.execute"]`, assert a `history patch applied … create-<x>` console
   line and a panel-text change, undo via `Meta+z`/`Control+z` and assert the state reverts, flip a window-config measure
   toggle (`[id="<window>/<measure-id>"]`) and assert `setLayerVisibility`-class console lines + a `data-published-value`
   flip, switch the navbar example combobox (`[role="combobox"]` → `[role="option"]` filtered by label text) and assert new
   document content, switch editor mode. Every step's helper `note()` records `faultLines()` — a regex over
   `trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey`
   — a **zero-length fault array is the pass condition** for each step, not just "no exception thrown".
3. **Viewer probe** (`🐍️remodel-viewer-probe.mjs`, 35 lines) — boots the editor, clicks
   `[id="playground.navbar.roles.viewer"]`, asserts the viewer's own window host renders with ≥1 canvas and no faults.
4. **Panel-refresh probe** (`🐍️remodel-panel-refresh-probe.mjs`, 85 lines) — the specific "did the mutation land in the
   document, or did the panel just not repaint" diagnostic: perform a mutation, read the panel summary row immediately, close
   + reopen the panel, switch to a different panel and back — three independent re-reads to distinguish a UI staleness bug from
   a document bug. Directly responsible for catching pitfall #6 (§6).
5. **Reconstruction/long-running-job probe** (`🐍️remodel-reconstruction-probe.mjs`, 134 lines) — arms a Tool-category job
   (`[id="framework.category.tool"]` → auto-arms its first row, **do not click the row again or it disarms**), starts with the
   framework chord `Meta+Enter`/`Control+Enter` on the Tool-run panel tab, then polls up to `runSeconds` (default 240) for a
   terminal body-text match (`Faulted|Finalized|Completed|Failed`), recording every distinct `[id^="framework.toolRun"]` row
   snapshot along the way. Template for any wfc job that runs over many ticks (e.g. a long WFC solve/backtrack run).

Guest diagnostics (verbose per-panel-refresh logging) are opt-in via `localStorage`, set through `page.addInitScript`:
```js
if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1")
  await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
```

---

## 5. Oracles: python/TS, and storybook

### 5a. TS example/oracle tests
```
bun nx run @semio-tech/remodel-js:test
```
(`📦️packages/🟦️typescript/📋️project.json`, wired to `TestScript` → `runVitest(root, rest, "🧪️tests/🟦️.ts")`; the vitest
CONFIG file `include`s `["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts"]`). For wfc: `bun nx run @semio-tech/wfc-js:test`
(nx project name `@semio-tech/<pluginId>-js`, package at `📦️packages/🟦️typescript/`). W3's landed state on remodel (383/383,
later 1353/1353 after fixture expansion) shows the intended shape: a byte-exact TS snapshot codec twin of the Rust one +
cross-language fixture oracle, not just "file is non-empty" smoke tests (remodel started at that thin state and the ticket
explicitly calls it out as a gap to close — do not let wfc ship with only-stub TS tests).

### 5b. Python oracle scripts (read-only, stdlib-only, no `jsonschema`/`pytest` dependency)
All invoked the same way, `cd` to the ticket folder first:
```
python3 🐍️schema-validate.py       # validates every committed fixture against the regenerated normative JSON Schema leaves
python3 🐍️fixture-audit.py         # does every asset://.../local://.../shared:// URI in the feature file resolve on disk
python3 🐍️mount-check.py           # resolves every #[path="..."] attribute, reports dangling mounts
python3 🐍️python-oracle-dryrun.py  # runs every registered oracle handler offline (no cargo/bun/host) against committed bytes
python3 🐍️grammar-check.py         # third-party independent oracle for the text-grammar leaves (mirrors the Rust lexer/parser)
python3 🐍️canonicalize-remodel-fixture-floats.py   # rewrites fixture JSON so float fields match the schema's canonical form (see pitfall §6 item "float canonical form")
```
Each script is self-contained with an absolute `ROOT = Path("/Users/ueli/Documents/semio")` (or a relative walk-up) and a
docstring `Usage:` line — read the header before running, they are cheap, fast, and safe (no writes except the canonicalize
script, which is explicitly a rewrite tool). Clone all six for wfc, substituting the artifact/plugin path.

### 5c. Storybook gate
Every plugin gets one FREE generic matrix row automatically (no dedicated file needed):
```
.storybook/stories/framework/os/plugins.stories.tsx   — one `export const <Plugin>: Story = { args: { plugin: "<id>" } }`
```
This boots `FrameworkOsShell` for the plugin inside Storybook and is exercised by `.storybook/os-plugins.spec.ts` (iterates the
generated `PLUGIN_BUILD_TARGETS` from `🧩️plugins.ts`) for "reaches a deterministic boot outcome, zero unexpected `console.error`"
— automatic once the plugin is registered, no extra file needed. A DEDICATED scope (`.storybook/scopes.ts` entry +
`.storybook/stories/<plugin>/` directory with real DSL-fixture-backed component stories) is optional, higher-value work that
remodel itself never got (`w5-storybook.md` landed 26 stories for remodel eventually, in this same ticket, after DoD work was
mostly done) — not required to close a WFC DoD, but worth doing if time allows, following block's `w5` pattern as the fullest
precedent. Run storybook tests with `bun run test:storybook` (referenced in `.cursor/plans/example_shape_refactor_adb7b675.plan.md`
as part of the repo-wide gate list; confirm the exact package.json script name at commit time).

---

## 6. Known pitfalls (remodel's numbered fault list) — numbered fix ledger for wfc agents

Faults are listed in the order the remodel ticket hit them; each links straight to a fix pattern to apply proactively for wfc.

1. **Empty `VITE_SEMIO_APP_ID` → playground block has no `app`.** Fix: add `app = "s.<plugin>.<artifact>@1/*#editor"` to the
   plugin crate's `Cargo.toml` playground block. (Same root cause the remodel ticket calls out as recurring across
   forms/shooting/remodel — check this FIRST on any new plugin's first boot.)

2. **React duplicate key `framework.panel.artifact`** when a plugin owns the Artifact tab as a BRANCH (multiple children,
   e.g. Reconstruction + Run) rather than a single leaf. `ShellHost.workbenchLeftTabs` used a leaves-only helper
   (`flattenPanelTabNodeLeaves`) to decide `hasPluginArtifactTab`, so a branch-shaped Artifact tab was invisible to that check
   and the shell mounted its OWN Artifact tab under the same key beside it. Fixed framework-side
   (`flattenPanelTabNodes`, every node, in `🏛️ShellHost/🟦️.tsx`) — already landed, but if wfc's editor also gives the Artifact
   tab multiple children, verify this is still fixed on your working tree (`grep flattenPanelTabNodes 🏛️ShellHost/🟦️.tsx`).

3. **Spurious document edit at boot** (`canUndo=true` on a document that IS the boot example, because
   `replace_document_operations` unconditionally deleted/recreated every collection + rewrote every parameter group instead of
   diffing against declared state). Fix pattern: implement `set-active-example` as a **declared-state diff** — entries
   deleted/created only when missing or changed, parameter groups only when different — with a unit test asserting
   `upserts=0, canUndo=false` on re-selecting the boot example.

4. **Panel refresh refused: `plugin-ui.intake-rejected:typed-normalize:Unknown UI field: window`.** Happens when a peer/host
   ticket adds a new field to the retained UI wire contract (e.g. `TreeSectionProps.window`) faster than the host's typed
   normalizer (`🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts`) learns to accept it. This is a **framework/host-timing** bug,
   not necessarily a wfc bug — if you hit it, check whether a peer ticket just extended the UI contract and the host mirror
   needs the same field added (`🛂️manifest/🟦️.ts` re-export + `ui-contract-rs:generate` regeneration), same as remodel's fix.

5. **Viewer boot toast "This app does not declare that action."** The shell's automatic boot `setActiveExample` call fires
   even in a read-only Viewer surface whose only command is `Noop`. Two-sided fix: the viewer's `initial_snapshot` should parse
   the subset's committed default example itself (editor and viewer share one scene) rather than falling back to a hardcoded
   default scene; AND `ShellHost`'s boot-example effect should skip its automatic announcement when
   `undeclaredActionDiagnostic` says no window kind of THIS app declares `setActiveExample` (already landed framework-side —
   verify present, don't re-fix).

6. **Phantom document edit: a create-verb journals a history patch but the panel keeps showing zero, and a follow-up verb on
   the just-created entity produces NO edit at all.** Two layered causes to check for any bounded/one-item-store plugin:
   (a) a manifest Actions-form DEFAULT value (e.g. `cameraId: "cam-0"`) that doesn't actually exist in the boot document, so
   the mutation's `diff` answers a FATAL `mutation.invariant`; (b) the one-item store preparation applying the fatal outcome's
   EMPTY diff anyway and minting a no-op edit that undo/ledger then wrongly treat as real. Fix: (i) the SDK's generic
   `bounded_config_store_one_item_preparation_factory::<Snapshot, Mutation>(…)` now refuses a fatal outcome outright — **use
   the SDK factory, do not hand-write a bespoke `*StorePreparation::advance`** (remodel deleted a 158-line duplicate in favor
   of it — precedent: shooting/dag/trinity already used the factory); (ii) any command that references an id by a form default
   must validate that id exists and refuse with a clear domain error (`<domain>.<entity>.unknown-<ref>`) rather than silently
   defaulting to an empty/uncalibrated placeholder.

7. **Frames/2D window "Empty canvas" despite a document with real frame data.** A window-config's cursor (e.g. "which stream
   /frame is active") defaulting to unset/empty must FALL BACK to the document's first real entry, not render nothing. Same
   defensive pattern applies to any wfc window whose content depends on a selectable cursor into the document (which cell/tile
   is being inspected, which generation step is shown).

8. **Window payload shape must match the exact host contract, not a plausible-looking JSON shape.** `JsonLayersCanvasSession`
   (`Canvas2dHost/🟦️.tsx`) branches on `layer.kind` (not `type`) and EVERY layer needs explicit `x/y/width/height` or
   `layerBounds()` returns null and the layer silently falls through to a label-only render ("draws nothing recognizable").
   `points:[[x,y],…]` belongs only to a `polyline` layer, never to a marker/circle. **Read the actual host component's prop
   contract before emitting window JSON for a new window kind — do not infer the shape from the mutation's own field names.**
   Same class of bug: `report_table_json` must set `sortable: true` per column or `TableHost`'s sort control is dead even
   though it renders.

9. **`describe`/registry staleness silently blocks DoD, not just cosmetically.** A 3-day-stale committed descriptor blocked
   both the registry check's id agreement AND the react example picker (`manifest.apps[].examples: []` read verbatim by
   `ShellHost`). **Run `describe` after any change to artifact-kind id, dialect id, or the example registry — before trusting a
   playground boot, not only before a commit.**

10. **The bare `app()`/`new_app` test harness (registry-less) is unusable for dispatch tests** — panics in the tool-proof
    catalog join (`interactive-job.catalog-authority … migrated={}`). Memory `project-registryless-testkit-new-app-unusable`.
    Fix pattern (remodel's, mirrored from shooting): a self-closing `<Plugin>App` newtype wrapping ONE registry-backed `app()`
    constructor; `dispatch` settles and returns `Dispatched { result, lanes }`; tests read `edited_document()` /
    `edited_only_window_config()` off `lanes`, not `result.mutations` directly. Build wfc's test harness on this shape from the
    start rather than discovering the panic later.

11. **Non-canonical float literals in fixture JSON fail `committed … is not canonical` / diff-mismatch tests** (`2` vs `2.0`
    for an f32/f64 field, key ORDER inconsistent with the struct's declared field order). Fix tool pattern: a canonicalizer
    script that derives the float-typed keys from the NORMATIVE JSON SCHEMA (including nullable `["number","null"]` unions) and
    rewrites fixtures in place, plus explicit struct-field-order-matching key reordering for any mutation with a fixed shape.
    Related deeper framework bug (F1, confirmed, NOT yet fixed upstream as of this ticket): `ToValue for f32` widens through
    f64 before `pack::json` prints it, so `0.42f32` prints as `0.41999998688697815` on the wire — fixtures must be authored
    f32-quantized to match, this is a wire-format fact, not a bug to "fix" locally in wfc's own fixtures.

12. **Unclassified/un-migrated verbs abort the descriptor probe** — the ticket brief's phrasing "unclassified verbs abort the
    descriptor probe" maps to remodel's own DoD-4 precondition: EVERY editor verb must be `Migrated` (bounded-tool retained-tool
    roster, exact publication contracts) before a react boot's descriptor probe passes; remodel already had this
    (`REMODELING_RETAINED_TOOL_IDS`, all 35 verbs Migrated) at ticket-open, unlike sibling plugins (shooting) which discovered
    un-migrated verbs live. **Audit wfc's full command roster for `Migrated` status before the first playground boot attempt** —
    an un-migrated/BatchOnly verb reaching the interactive dispatch path is a hard boot-blocking fault, not a soft warning.

13. **`command_from_action` must accept the DSL value shape, not raw `serde_json::Value`.** A stale/async trait shape
    (found and fixed in remodel's editor/viewer explore pass) had `command_from_action(serde_json::Value)` instead of
    `dsl::DslValue` — this silently breaks the args bridge between the retained UI action row and the dispatched command.
    Verify wfc's `args_bridge::command_from_action` (or equivalent) is typed against `DslValue` from the start.

14. **`BatchOnly` verbs are hard-dead in the interactive app** (memory `project-interactive-job-classification-gates-dispatch`:
    `BatchOnlyPendingRewrite` never reaches runtime dispatch). If any wfc command is classified `BatchOnly` it will never be
    clickable/executable from the Actions pane in the playground — classify every user-facing verb `Migrated`, reserve
    `BatchOnly` only for genuinely non-interactive batch-only operations.

15. **`#0` duplicate sibling keys** — a React key-collision class of bug (`DuplicateSiblingKey`, explicitly matched by the
    interact probe's `faultLines()` regex) generally caused by two host-rendered nodes computing the same id/key when a branch
    node isn't distinguished from a leaf (see item 2) or when a list of dynamically-generated rows (e.g. wfc's per-cell/per-tile
    rows) doesn't include a genuinely unique discriminator. Watch the probe's fault lines for this pattern on any panel that
    lists a variable-length collection.

16. **Missing `[[test]]` declarations for emoji-path test files silently exclude them from `cargo test`.** See §1f — Cargo
    resolves emoji paths fine via an explicit `[[test]]` block, but forgetting the block means the file is simply never
    compiled or run, with no error (it's just not a discovered target). Audit every emoji-named top-level test file under a new
    wfc crate against its Cargo.toml `[[test]]` list.

17. **UiText / owned UI text is capacity-bounded (~512 bytes per admission unit)** — memory
    `project-one-ui-admission-fault-kills-every-later-refresh`: a single oversized `UiText` write in ANY panel can fail
    `refreshUi` admission for the WHOLE surface, not just that panel, silently killing every later panel refresh. Keep any
    wfc panel/report text field that echoes user data (long grid descriptions, long tile/rule labels) truncated defensively;
    do not assume large text fields are safe just because they compile.

18. **Orphaned `#[cfg(test)]` before a region-comment silently cfg-gates a PUBLIC function out of non-test builds.** A codemod
    (de-asyncing) left `#[cfg(test)]` immediately above `//#region …` immediately above `pub fn build_engine_params` — Rust
    attributes bind to the next ITEM, a comment is not an item, so the gate silently landed on the real function, producing a
    cryptic downstream `E0432: no <fn> in <module>` at every call site. **After any bulk codemod (de-async, rename, region
    reshuffle), grep every crate touched for `#\[cfg\(test\)\]` immediately followed by a comment line, not a `mod tests`/`fn`
    — that pattern is always a bug.**

19. **Host/framework churn can block a compile gate through no fault of the plugin** (W11's `TurnResult::cold_pair_ingress`
    saga: a peer's in-progress kernel migration broke `semio-framework-plugin-host`, which sits upstream of every plugin crate
    in the dependency graph, so `cargo check -p semio-s-plugin-remodel` never even got compiled). Diagnostic habit: when a
    plugin-crate check fails with errors in a FRAMEWORK file the ticket doesn't own, run
    `cargo check -p semio-s-plugin-<wfc> --target wasm32-wasip2` instead (remodel only reaches the native host off-wasm,
    so a wasm-target check type-checks the plugin independent of host-side native churn) and flag the peer breakage rather
    than trying to fix it in-ticket.

20. **`native_composer_entries()` / `.composers()` is a dead IO channel on the new declaration-tree shape** — `SubsetDeclaration`
    has no `composers` field; the replacement channel is a typed `IoEntry` slice returned by the subset's own `io()` function
    (`SchemaDeclaration{..., io: io::io()}`). If wfc's IO layer is copy-pasted from an older plugin still using
    `.composers(native_composer_entries())`, migrate it to the `io()` / `IoEntry` shape from the start rather than discovering
    the dead channel later.

---

## 7. Quick-reference command block for a fresh wfc crate (fill in the blanks once the crate exists)

```bash
cd /Users/ueli/Documents/semio

# 1. native check/test (plugin crate)
cargo check -p semio-s-plugin-wfc --lib --tests --message-format=short

# 1b. native check/test (each artifact crate — component-app-assembly REQUIRED)
export RUST_MIN_STACK=33554432
cargo check -p semio-s-artifact-wfc-<kind> --features component-app-assembly --lib --tests -j 4 --message-format=short
cargo test  -p semio-s-artifact-wfc-<kind> --features component-app-assembly --lib -j 4 -- --test-threads=4

# 2. wasm target check (independent of native-host churn)
cargo check -p semio-s-plugin-wfc --target wasm32-wasip2

# 2c. restage wasm + materialize react dev module
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=<port>
bun nx run @semio-tech/framework-os-dev:activate-wfc-react-dev

# 2f. serve (in a detached screen/nohup session from the MAIN/coordinating session, never a subagent)
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=<port> SEMIO_PLUGIN=wfc
bun ./📜️script.ts serve wfc react dev

# 3a. regenerate descriptor after ANY source id/example-registry change
bun nx run @semio-tech/wfc-plugin:describe

# 3b. registry check (DoD gate)
bun nx run @semio-tech/plugin-registry:generate
bun nx run @semio-tech/plugin-registry:check

# 3c. taxonomy check, scoped
bun ./📜️script.ts verify taxonomy report --scope "✏️s/🔌️plugins/🌊️wfc"
bun ./📜️script.ts verify taxonomy enforce --scope "✏️s/🔌️plugins/🌊️wfc"

# 3d. dependency truth gate
bun ./📜️script.ts verify dependencies literal-external

# 3e. full policy aggregator (broader than `verify`)
bun ./📜️script.ts policy

# 4. browser boot proof (headless, --use-angle=metal is load-bearing)
cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN
SEMIO_PROBE_URL="http://127.0.0.1:<port>/?plugin=wfc" SEMIO_PROBE_OUT=wfc-boot bun 🐍️wfc-console-dump-probe.mjs

# 5a. TS/vitest oracle
bun nx run @semio-tech/wfc-js:test

# 5b. python oracles (from the ticket folder, after cloning them)
python3 🐍️schema-validate.py
python3 🐍️fixture-audit.py
python3 🐍️mount-check.py
python3 🐍️python-oracle-dryrun.py
```

---

## Files referenced (all under `/Users/ueli/Documents/semio/`)

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/📓️status.md` (full 137-line read)
- Same folder: `📜️activate-remodel-react.sh`, `📜️serve-remodel-react.sh`, `📜️check-remodel-native.sh`,
  `📜️check-remodel-artifact-tests.sh`, `📜️test-remodel-native.sh`, `📜️test-remodel-add-stream.sh`, `📜️test-remodel-filtered.sh`,
  `📓️explore-dev-boot-ts-storybook.md`, `📓️w11-integration.md`, `📓️w7a-synthetic-e2e.md`,
  `🐍️remodel-console-dump-probe.mjs`, `🐍️remodel-interact-probe.mjs`, `🐍️remodel-viewer-probe.mjs`,
  `🐍️remodel-panel-refresh-probe.mjs`, `🐍️remodel-reconstruction-probe.mjs`,
  `🐍️schema-validate.py`, `🐍️fixture-audit.py`, `🐍️mount-check.py`, `🐍️python-oracle-dryrun.py`, `🐍️grammar-check.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📜️lib-suite-run.sh`, `📜️lib-deadlock-probe.sh` (direct
  ancestor of wfc's source crate; `RUST_MIN_STACK`, `component-app-assembly` precedent)
- `.cargo/config.toml` (shared build-dir)
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📋️project.json` (`describe`/`test`/`regenerate-example` nx targets)
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` (`[[test]]` ASCII-name pattern, lines ~87-105)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json` (`generate`/`check` targets)
- `.vscode/launch.json` (remodel dev entries, `🛠️dev📸️remodel…` block ~line 3251-3300), `.vscode/🧩️launch.seed.jsonc` (generator source)
- `.claude/launch.json` (`remodel-react-attach`, line ~426)
- `📜️script.ts` (root) — `DevScript`, `VerifyScript.runTaxonomy`/`runDependencyFreeze`, policy dispatch
- `package.json` (`dev:remodel`, `dev:procedural:3d` scripts)
