# Puzzle 3D dev boot path (read-only audit, 2026-09-08/09)

Read-only exploration. No build/test/server was run; no ticket was opened/closed/reopened; no file
outside this ticket's own markdown was touched. All line numbers verified against the working tree at
commit `de617a7c17` (HEAD at time of writing).

## 0. Headline finding before the recipes

**As committed right now, `SEMIO_RENDERER=react dev 3d` (the `🛠️dev🧩️puzzle🏙️3d⚛️react` launch.json entry,
port 6013) cannot cold-boot puzzle3d — it throws before Vite even starts.** The `wgpu` variant (port
6113) is the one that actually builds and serves puzzle3d today; it is also the one 2026-09-05/07's
`📓️findings-2026-09-05.md` §33-§46 spent the whole session getting to 94% boot. Details in §3 below.
The `⚛️react` findings in §14-§17 of that same doc describe an **older** version of `DevScript`'s react
branch (one that still called `buildPlugins`/`buildEngineWasm` before serving) — that code path no
longer exists in `DevScript` as of this audit. Treat §14-§17 as historically accurate but not
reproducible against current HEAD without first fixing §3's gap.

---

## Minimal boot recipe (reuse whatever is already staged)

**Use the wgpu renderer — it is the one that is actually wired to build+serve today:**

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=/path/outside/repo/private-target \
CARGO_PROFILE_WASM_DEV_DEBUG=false \
SEMIO_RENDERER=wgpu \
S_OS_PORT=6113 \
bun ./📜️script.ts dev 3d
```

What this does, and why each piece matters (all read from current source, see §3-§4):
- Resolves `3d` → `puzzle3d` via `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:34-37`.
- `renderer==="wgpu"` branch of `DevScript.run` (`…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1432-1478`)
  runs `buildPlugins("puzzle3d")` (rebuilds only the crates the catalog resolves to — currently
  **puzzle + stdio**, not the full ~59-crate catalog; see §14 of the findings doc) then
  `buildEngineWasm("puzzle3d","wgpu")`, which is a **no-op for wgpu** (the function early-returns
  unless `renderer==="react"`, line 1050) — the wgpu path does not need `surface`/`editor`/`flow-core`.
- Then spawns `bun ./🎯️targets/🧊️wgpu/📜️script.ts serve` in the wgpu renderer package with
  `SEMIO_PLUGIN=puzzle3d SEMIO_RENDERER=wgpu S_OS_PORT=6113` (line 1464-1473).
- `CARGO_PROFILE_WASM_DEV_DEBUG=false` cuts the puzzle/stdio cargo RSS from ~8.6 GB to ~165 MB
  (measured, findings §14) — without it a loaded box (this one is at load ~60) can hit an OOM wall.
- `CARGO_TARGET_DIR` outside the repo avoids the "a peer deleted the shared `target/` mid-build"
  failure mode findings §14/§21 hit; **do not** put it inside the repo (§21: still not safe — deleted
  by an unrelated concurrent build).
- Puzzle's own `[profile.wasm-dev.package.semio-s-artifact-puzzle-3d] opt-level = 2` override is
  **already committed** (`Cargo.toml:488-489`) — no env var needed for the interactive-ceiling fix from
  findings §17.
- Currently staged (§4 below): puzzle's plugin-catalog wasm is **fresh** (10:56–20:08 today), but
  **stdio's is 3 weeks stale (Aug 18) against 5,741 newer `.rs` files** — `buildPlugins("puzzle3d")`
  will therefore recompile stdio from scratch on this "minimal" run; there is no way to skip that and
  still get a working boot, because stdio's staged descriptor is **entirely missing**
  (`🔌️plugin-modules/🗄️stdio/` has no `🔣️.json`/`🛂️.descriptor.semio`, only `.js`/`.wasm`/`.d.ts`) — a
  cold boot 404s on `plugin.descriptor-unavailable` without it (findings §14).
- After the server is listening, **open `http://127.0.0.1:6113/?plugin=puzzle3d` explicitly** — the
  bare root URL defaults to the full `s` studio host and expands all ~56 plugins
  (`browser-boot/🟦️.ts:32`, findings §34.1).
- Verify the served bundle is not stale: `strings <staged>.wasm | grep <a string only in your target
  commit>` — port-listening is not proof of a rebuilt binary (findings §44 "Port-listening is NOT proof
  of a rebuild" — trunk binds its port and serves the previous `dist` while still compiling).

If you only want to confirm the server *boots and serves bytes* (not that puzzle3d itself renders),
skip the cargo work entirely with the wgpu package's own reuse check:
`bun ./🎯️targets/🧊️wgpu/📜️script.ts serve` will itself detect an already-listening port at 6113 and
short-circuit (`isDevPortInUse`/`probeWgpuDevPort` in `DevScript.run`, lines 1440-1463) — i.e. if a
previous session already has 6113 up, re-running the recipe above is a no-op that just prints the URL.
As of this audit **nothing is listening on 6013 or 6113** (§5) — there is no already-warm server to
reuse right now.

### The react path — do not use it yet, see the fix it needs

`bun ./📜️script.ts dev 3d` / `SEMIO_RENDERER=react` (`🛠️dev🧩️puzzle🏙️3d⚛️react`, port 6013) currently
resolves to `DevScript.run`'s react branch, which is exactly:

```ts
// …🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1428-1431
if (renderer === "react") {
  await new ServeScript(this.root).run([plugin, renderer, semioBuildMode() === "ship" ? "release" : "dev", ...serverArgs]);
  return;
}
```

No `buildPlugins`, no `buildEngineWasm` call anywhere in this branch. `ServeScript.run`
(line 1410-1420) immediately does:

```ts
const receipt = readActivationReceipt(join(runtime, "activation"));   // :1415
```

`readActivationReceipt` (`…🧑‍💻dev/♻️activation/🟦️.ts:38-40`) is a bare `readFileSync` — it throws ENOENT
if the receipt file doesn't exist. `runtime` is `developmentRuntimeRoot(this.root, "puzzle3d", "dev")` =
`…🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/puzzle3d` (activation.ts:45-47). **That directory does
not exist on disk right now** — the only variant with a runtime/activation receipt staged is `note`
(`dist/runtime/dev/note/activation`), which looks like vitest test-fixture output for this package's own
test suite, not a real dev session. So `dev 3d react` throws immediately, before Vite starts.

The receipt is populated only by `ActivationScript` (line 1376), which first requires
`PreparationScript` (line 1320) to pass — and that in turn requires, per plugin, a component already
staged at `browserModuleRoot(profile)/<pluginDir>` = **`…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/<dir>`**
with a descriptor JSON, a bridge file, and a `.nx-artifact.json` (PreparationScript:1330-1334), plus a
generated `…🔌️plugin/📇️registry/dist/sessions/puzzle3d/🟦️session.ts`. **Neither exists for puzzle3d
today** — `dist/dev/🔌️plugin-modules/` only has `🧵️shard`, `🪞️vendor`, `🗒️note` (verified on disk), and
`dist/sessions/` has no `puzzle3d` entry either.

The component that *would* populate that profiled directory is `MaterializeScript`
(`…🔌️plugin/📦️packages/🟦️typescript/📜️script.ts:99-133`, usage `materialize <dev|release> --manifest
<Cargo.toml>`, plus its sibling `SupportScript` at line 81-96 for the shared vendor/shard support
files) — but **no plugin crate's `📋️project.json` registers a `materialize` target** (checked every
`project.json` under `✏️s/🔌️plugins`, zero matches for `"materialize"`, `component-dev`, or
`component-release`), and puzzle's own `📋️project.json` (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`)
only exposes `wasm`/`test*`/`fixtures-lint`/`describe` — no target that produces
`dist/component-<profile>/<crate>.wasm`, which `MaterializeScript` itself requires as input
(line 105-106: `throw new Error("Missing component or browser support prerequisite; run the
materialize target through Nx")`).

**In short: `support`/`materialize`/`prepare`/`activate`/`serve` (react's real, profiled pipeline,
introduced today per `git blame` — `ServeScript`/`ActivationScript`/`PreparationScript` all land in
commit `0a0bb7438` at 14:30 today) is code that exists but is not wired to any plugin's Nx targets yet.**
It is not a "run one more command" situation — the crate-side half of the pipeline (a `materialize`
target per plugin, feeding from a `component-dev`/`component-release` cargo output) has not been
authored for any plugin, puzzle included. Until that's built, react dev boot for puzzle3d (or any real
plugin) is not reachable through `dev`/`serve` as committed.

**One more piece of drift worth flagging while you're in this code:** `SKIP_PLUGIN_BUILD` — the flag
`collabStartUserDevServer` (line 2636) and `startParityDevServer` (line 4132) both set when spawning
`bun …script.ts dev` — is **read nowhere in `DevScript`, `ServeScript`, or anywhere else in this file**
(grep confirms 3 write sites, 0 read sites). Root `📜️script.ts`'s `runFrameworkOsPlaygroundDev`
docstring (lines 244-254) still describes a `SKIP_PLUGIN_BUILD=1`+`SKIP_ENGINE_BUILD=1` "served" contract
that skips the plugin build — that contract predates today's `ServeScript` rewrite and no longer holds:
`SKIP_ENGINE_BUILD` is still real (read at `buildEngineWasm`, line 1050) but only matters for the wgpu
`DevScript` branch and `BuildScript` (react `dev` never calls `buildEngineWasm` at all now);
`SKIP_PLUGIN_BUILD` is dead code everywhere it's set.

---

## Full rebuild recipe (cold cache, nothing staged)

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=/path/outside/repo/private-target \
CARGO_PROFILE_WASM_DEV_DEBUG=false \
SEMIO_RENDERER=wgpu \
S_OS_PORT=6113 \
bun ./📜️script.ts dev 3d
```

Same command as the minimal recipe — there is no separate "full" incantation for wgpu, because
`buildPlugins`/`buildEngineWasm` are always invoked; the only difference cold vs warm is how much cargo
work they find to do. Expected durations, all measured in findings-2026-09-05.md under the same
`opt-level=2`/`CARGO_PROFILE_WASM_DEV_DEBUG=false` conditions:
- puzzle-only incremental rebuild after the `opt-level=2` Cargo.toml edit: **55.4 MB output**, described
  as a normal incremental build (§17) — no wall-clock figure given, but RSS-bounded (~165 MB) so it's not
  swap-bound on this box.
- Guest wasm rebuild after the lifecycle-ceiling fix, **wasm-release profile: 1h04m** (§39) — this is
  the profile the `SEMIO_PLUGIN_PROFILE=wasm-release` env knob (findings §14 table) forces because the
  plain `dev`-profile stdio component exceeds the component parser's 1,000,000-function ceiling.
- Full `s` catalog rebuild (59 crates) from `SEMIO_PLUGIN_ONLY` misuse: **3h01m and produced 0 plugins**
  (§36) — **do not do this**; `/?plugin=puzzle3d` already narrows the boot plan to 2 crates
  (puzzle + stdio) without any manual scoping flag.
- If `stdio` needs a real rebuild (it does right now — see §4): budget for it separately. It is the
  single largest plugin (376 MB unstripped core wasm as of the Aug 18 build) and findings §45 documents
  it as the "recurring outer blocker" — churn from a concurrent session broke it three separate times in
  one week. **Check `find ✏️s/🔌️plugins/🗄️stdio -name '*.rs' -newer <staged wasm>` before you start** —
  if it's mid-edit by another session (mtimes within the last few minutes), poll instead of racing it.

Add, only if reproducing the *react* pipeline once it's fixed:
```bash
SKIP_ENGINE_BUILD=1   # only if surface+editor+flow-core wasm are ALL already fresh — see §3
```
`SKIP_ENGINE_BUILD` is an all-or-nothing gate on 3 unrelated crates written to 3 different paths
(`buildEngineWasm`, `…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1042-1061`) — it is not per-crate
freshness-checked, so verify all three yourself first (findings §14 "the correct safety rule").

---

## 1. Full command resolution chain

| step | file:line | what it does |
|---|---|---|
| `.vscode/launch.json` `🛠️dev🧩️puzzle🏙️3d⚛️react` | `.vscode/launch.json:1967-1985` | `bun nx run workspace:dev -- 3d`, env `PUZZLE_3D_PLAY_PORT=6013 SEMIO_RENDERER=react` |
| `.vscode/launch.json` `🛠️dev🧩️puzzle🏙️3d🧊️wgpu🌐️wasm` | `.vscode/launch.json:1987-2005` | same command, env `PUZZLE_3D_PLAY_PORT=6113 SEMIO_RENDERER=wgpu` |
| `.vscode/launch.json` `…🎛️concrete🌲️forest⚛️react` / `…wgpu` | `.vscode/launch.json:2019-2077` | `bun nx run workspace:dev -- 3d fixture concrete`, same ports/env pattern |
| root `📋️project.json` target `dev` | `📋️project.json` (`"dev"` block, ~line 46) | `bun ./📜️script.ts dev`, `forwardAllArgs: true` |
| root `📜️script.ts` router | `📜️script.ts:17658` | `.register("dev", DevScript)` |
| root `DevScript.run` | `📜️script.ts:488-524` | dispatches `segments[0]==="3d"` through `resolvePlaygroundDevApp` (line 513-517) → `runFrameworkOsPlaygroundDev("puzzle3d", rest)` |
| alias resolution | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2603-2613` | `resolveFrameworkOsPlaygroundPlugin` matches `"3d"` against the Cargo-declared `aliases` |
| catalog source | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:33-37` | `variant="puzzle3d"`, `aliases=["3d","puzzle 3d"]`, `ports={react:6013,wgpu:6113}` |
| spawn | `📜️script.ts:255-261` (`runFrameworkOsPlaygroundDev`) | `bun nx run @semio-tech/framework-os-dev:dev -- puzzle3d [fixture concrete]`, env via `frameworkOsPlaygroundDevEnv` |
| os-dev `📋️project.json` target `dev` | `…🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` (`"dev"` block) | `bun ./📜️script.ts dev`, cwd `…🧑‍💻dev/📦️packages/🟦️typescript` |
| os-dev `📜️script.ts` router | `…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5420` | `.register("dev", DevScript)` — **this is the real `DevScript`, distinct from the root one above** |
| os-dev `DevScript.run` | `…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1422-1479` | branches on `renderer` — see §3 |

`fixture concrete` (the `…🎛️concrete🌲️forest…` launch.json variants): `resolvePlaygroundDevApp(["3d","fixture","concrete"])`
matches the 1-segment alias `"3d"` first (longest-alias-first search, root `📜️script.ts:245-254`), so
`rest=["fixture","concrete"]` passes straight through as `serverArgs`/extra argv into
`os-dev DevScript.run`'s `variantSegment`-stripped tail (line 1424-1425) — for react this lands as extra
Vite CLI args via `runViteBunxDev`'s `serverArgs`, for wgpu it is currently **unused** (the wgpu branch
never reads `serverArgs`). Practically: for `SEMIO_RENDERER=wgpu`, `fixture concrete` on the command line
has no observable effect today — the fixture selection for wgpu must come from elsewhere (not traced
further; out of scope for this boot-path audit).

**`PUZZLE_3D_PLAY_PORT` is dead.** Grepped the entire repo (`.ts` sources, excluding `node_modules`/
`target`): zero read sites. The actual port comes from `frameworkOsPlaygroundDefaultPort` reading the
Cargo-declared `ports.react=6013`/`ports.wgpu=6113` (…🟦️.ts:2588-2592), consumed as `S_OS_PORT`
(`frameworkOsPlaygroundDevEnv`, …🟦️.ts:2616-2624). It happens to equal the same numbers, so nobody
noticed. Same dead pattern confirmed on the sibling `procedural3d-react` entry
(`PROCEDURAL_3D_PLAY_PORT` — also zero read sites) — this is a repo-wide copy-paste convention, not a
puzzle-specific bug, and it's harmless (just misleading) as long as it stays in sync with Cargo.toml.

---

## 2. `dev 3d`'s actual step sequence, by renderer

### `renderer=react` (`…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1422-1431`)

```ts
if (renderer === "react") {
  await new ServeScript(this.root).run([plugin, renderer, semioBuildMode() === "ship" ? "release" : "dev", ...serverArgs]);
  return;
}
```

That's the entire branch. `ServeScript` (line 1410-1420) reads an activation receipt and, if present,
calls `runViteBunxDev` (…🦑️repo/…/🟦️.ts:2777-2828) — a plain `bunx vite --config ⚙️vite.config.ts --host
… --port … --strictPort`. **No plugin build, no engine wasm build, no registry refresh happens in this
branch at all** — see §0/§3 above for why that's currently fatal for a cold boot.

### `renderer=wgpu` (`…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1432-1479`)

1. `ensureAppleDeveloperDir()` — macOS-only codesign prerequisite, no-op elsewhere.
2. `await buildPlugins("puzzle3d")` (line 1434 → function at line 682-692):
   - `preparePluginBuildTargets` (line 636-659): `ensureWasmTarget()` (rustup target add), then
     `ensurePluginRegistry("puzzle3d")` (line 615-621) — **this is the "plugin registry refresh" step**:
     runs `bun …🔌️plugin/📇️registry/📜️script.ts generate`, then
     `syncBuiltPluginDescriptors`/`writePlaygroundSession("puzzle3d", …)`.
   - `resolvePluginBuildTargets` (line 624-641) narrows to the catalog's real dependency closure for
     `puzzle3d` — **`SEMIO_PLUGIN_ONLY=<id>` here is a trap** (line 628, and findings §14): it forces a
     single crate and silently drops any other crate the boot needs (e.g. `stdio`), which then 404s at
     runtime with `plugin.descriptor-unavailable`. Leave it unset; the catalog resolver already narrows
     correctly (`program build scope: puzzle, stdio`, not the full 59).
   - `buildPluginCatalog` (line 575-606) runs `cargo build` per target serially, then materializes
     (transpile + descriptor + bridge) up to `materializeConcurrencyLimit()` in parallel, writing to
     `pluginOutRoot` = `PLUGIN_MODULES_ROOT` (line 93) =
     `…🧑‍💻dev/🔌️plugin-modules/` (unprofiled — **not** the `dist/<profile>/` directory react's
     `PreparationScript` reads; these are two separate trees, see §3).
3. `await buildEngineWasm("puzzle3d","wgpu")` (line 1435 → line 1042-1061): **no-op** — line 1050 early-
   returns for any renderer other than react.
4. Resolves the port (`frameworkOsPlaygroundDefaultPort`, default 6113 for wgpu) and, if something is
   already listening there, either reuses it (`probeWgpuDevPort` finds the real entry path) or restarts
   a stale `trunk` occupant (`stopTrunkDevPort` + `awaitTcpReady`, lines 1440-1463).
5. Spawns `bun 🎯️targets/🧊️wgpu/📜️script.ts serve` (the wgpu renderer package's own script, `WGPU_SCRIPT_PATH`/
   `WGPU_PACKAGE_ROOT`) with `SEMIO_PLUGIN=puzzle3d SEMIO_RENDERER=wgpu S_OS_PORT=6113`
   (lines 1464-1473) under `daemonBudgetOpts()` (§6). This is the process that actually runs
   `trunk serve` — findings §31's zero-budget kill bug lived exactly here, now fixed (§6).

Neither branch does a "storybook" step — storybook is a fully separate `dev storybook …` command
(root `📜️script.ts` `DevScript.runStorybook`, line 528-547), unrelated to `dev 3d`.

---

## 3. React vs wgpu: which frontend, which port, which entry, which one actually rendered

| | react (6013) | wgpu (6113) |
|---|---|---|
| shell | React OS shell (`🏛️ShellHost` per the task's naming; served by Vite from `…🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts`) | `🌉️ProgramBridge` (`…🔌️plugin/…/🎯️targets/🧊️wgpu/…`), served by `trunk` |
| build tool | `bunx vite` (`ServeScript`, line 1418) | `trunk serve` via the wgpu package's own script (line 1464) |
| boot URL | needs `?plugin=puzzle3d` to avoid the `s` studio-host default (findings §34.1) | same `?plugin=puzzle3d` requirement applies — `browser-boot/🟦️.ts:32` is renderer-agnostic |
| asset server | n/a (Vite serves everything) | separate fixed port **6141** (`SEMIO_ASSET_SERVER_PORT`, …🎮️playground/🟦️.ts:2588) for the `mesh-collection`/`static-dir` routes puzzle3d's Cargo.toml declares (lines 51-61) — trunk forwards `/mesh`, `/infinite-fixture` there |
| what actually rendered per findings-2026-09-05.md | §14-§17 (2026-09-05/07): **rendered successfully** — "both windows render a scene", Nakagin Capsule Tower example, working panels — but against an **older DevScript** that still built plugins/engine before serving; not reproducible against current HEAD without §0's fix | §33-§46 (2026-09-05 continued): boot progressed from a hard crash to **94%** — retained-document rendering was implemented, but the session ended blocked on a **peer** compile break (`semio-framework-os-flow`, from the concurrent `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` migration), not on anything puzzle3d-owned (§46) |

So: **as of the findings doc, react is the one that was seen actually drawing pixels**, but that was
against code that no longer exists (§0). **wgpu is the one whose current code path is live and
build-capable today**, but its last documented run stalled one layer below puzzle3d (a peer crate).
Neither is a proven, reproducible "just run this" path right now — pick wgpu for the fewest surprises
(it's self-contained: no missing Nx targets), and budget time to re-verify the peer crates (`stdio`,
`flow`) compile clean before trusting a render.

---

## 4. Where artifacts land, and current on-disk staleness (checked 2026-09-08 ~23:50 / 2026-09-09 early)

### wgpu / catalog path (`🧑‍💻dev/🔌️plugin-modules/`, unprofiled — what `buildPlugins` writes)

```
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/
  semio_s_plugin_puzzle_component.core.wasm   39,744,966 B   2026-09-08 10:56
  semio_s_plugin_puzzle_component.js             443,348 B   2026-09-08 10:56
  semio_s_plugin_puzzle_component.d.ts             3,457 B   2026-09-08 20:08   (regenerated later)
  🌉️bridge.js                                      9,443 B   2026-09-08 10:56
  🔣️.json  (descriptor, human-readable)         5,388,499 B   2026-09-08 10:56
  🛂️.descriptor.semio  (descriptor, packed)      4,296,040 B   2026-09-08 10:56
  🟨️.js  (host shim)                                6,808 B   2026-09-08 10:56
  interfaces/                                                 2026-08-30 01:08
```
Puzzle is **fresh** — no `.rs` source newer than the `.core.wasm` (not independently re-verified for
every file this run, but the `.d.ts` regeneration at 20:08 plus a Sep 8 hot-swap marker
`{"pluginId":"puzzle","rebuiltAt":1788857813040}` (`♻️hot-swap.json`) both point at a same-day build).

```
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🗄️stdio/
  semio_s_plugin_stdio_component.core.wasm   376,957,060 B   2026-08-18 11:14   ← STALE
  semio_s_plugin_stdio_component.js              327,714 B   2026-09-04 17:20
  semio_s_plugin_stdio_component.d.ts               3,042 B   2026-09-08 20:07
  🌉️bridge.js                                       9,414 B   2026-09-01 22:58
  🟨️.js                                              6,808 B   2026-09-01 17:03
  interfaces/                                                 2026-08-18 11:09
  (no 🔣️.json, no 🛂️.descriptor.semio — descriptor is MISSING)
```
`find ✏️s/🔌️plugins/🗄️stdio -name '*.rs' -newer <that .core.wasm>` returns **5,741 files** — the staged
component is 3+ weeks behind source. Combined with the missing descriptor, this is exactly the
`plugin.descriptor-unavailable: /🔌️plugin-modules/🗄️stdio/🔣️.json (HTTP 404)` failure mode findings §14
diagnosed under `SEMIO_PLUGIN_ONLY=puzzle` misuse — except right now it would fire on a **correctly
scoped** `puzzle,stdio` build too, because stdio genuinely needs rebuilding, not just re-including.

**Staleness check recipe** (per findings §44's "port-listening is not proof of a rebuild", and §14's
mtime rule): `find <crate dir> -name '*.rs' -newer <staged .core.wasm>` — zero hits and a matching
`strings <wasm> | grep <a string unique to your target commit>` is the only trustworthy freshness
signal; mtime-newer alone still isn't proof the content differs.

### react / profiled path (`dist/<profile>/🔌️plugin-modules/` — what `PreparationScript` needs, nothing currently writes for puzzle)

```
…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/
  🧵️shard/
  🪞️vendor/
  🗒️note/     ← only variant with any staged component; looks like a test fixture, not a real dev run
```
No `puzzle3d` (or `puzzle2d`/`puzzle5d`) directory. `…🔌️plugin/📇️registry/dist/sessions/` likewise has no
`puzzle3d` entry (checked the full listing — `note`, `generation3d`, `bearbeiten`, `verfolgen`,
`aussuchen`, `aggregator`, `s`, `generator`, `koordinator`, … no puzzle variant). See §0/§3.

### `dist/runtime/<profile>/<variant>/activation/🔣️receipt.json` (what `ServeScript` reads)

```
…🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/note/activation/   ← only "note" exists
```
No `puzzle3d` runtime/activation directory at all.

### Engine wasm (`buildEngineWasm`'s three crates — react-only, and only via `dev` for the wgpu branch's
own no-op check or the `build` command; not touched by the wgpu `dev` path in practice)

```
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface_bg.wasm   29,856,694 B   2026-09-08 20:51
🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/framework_editor_bg.wasm             27,461,325 B   2026-09-08 21:00
…🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm                     42,413,039 B   2026-09-08 20:25
```
All three exist and are same-evening fresh — `SKIP_ENGINE_BUILD=1` would be safe *if* you also confirm
no `.rs` under each crate is newer (not exhaustively re-checked here beyond the mtimes above).

### `@semio-tech/puzzle-wasm` — the "BoardSession" wasm-pack bridge (task's item 1 hint)

```
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle_bg.wasm   59,578,672 B   2026-09-08 12:11
```
Built by `bun ./📜️script.ts wasm` (nx target `wasm` on `@semio-tech/puzzle-plugin`,
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`) via `runWasmPackWebBuild`
(…🦑️repo/…/🟦️.ts:2916, `shipProfile:"wasm-release", noDefaultFeatures:true`). **This is NOT part of the
`dev 3d` boot chain for either renderer** — nothing in `DevScript`/`ServeScript`/the wgpu serve script
or `⚙️vite.config.ts` references it or depends on it. It's consumed by the puzzle **2d** editor's own
`🌉️wasm/` bridge and tests (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/🌉️wasm/`), not by puzzle3d.
No `PUZZLE_BOARD_SKIP_WASM_BUILD` env var exists anywhere in the repo (grepped, zero hits) — the actual
skip mechanism for this build is Nx's own target cache (`"cache": true`, `outputs:
["{projectRoot}/pkg"]`, keyed on `**/*.rs` under the puzzle plugin tree).

---

## 5. Is a puzzle3d dev server running right now?

```
$ lsof -nP -iTCP:6013 -sTCP:LISTEN     → (nothing)
$ lsof -nP -iTCP:6113 -sTCP:LISTEN     → (nothing)
$ ps -axo pid,ppid,etime,command | grep -E "script.ts dev|vite|trunk|storybook" | grep -v grep
                                        → (nothing)
```
No react or wgpu puzzle3d server, no stray vite/trunk/storybook process. Nothing was killed or touched.

---

## 6. Budgets

`spawnDaemon`'s "zero-budget kills every daemon on the next tick" bug from findings §31 **is fixed and
still fixed** — verified current source:

```ts
// …🦑️repo/…/📦️packages/🟦️typescript/🟦️.ts:2193-2211 (spawnDaemon)
const budgetMs = daemonBudgetMs();
const timer = budgetMs > 0 ? setTimeout(() => { killed = true; if (child.pid) killBudgetTree(child.pid); }, budgetMs) : undefined;
```
`0` now correctly means "no timer at all" (not `setTimeout(fn, 0)`).

**The "20-minute cargo budget" the task/older memory notes describe no longer exists as a default.**
Current defaults (`…🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts:12-40`), all overridable via env, all
default to **0 = unbounded**:

| constant | value | env override | used by |
|---|---|---|---|
| `BUILD_BUDGET_MS` | 0 | `SEMIO_BUILD_BUDGET_MS` | `cargo`-class commands (`buildBudgetMs()`) — `buildEngineWasm`, `buildPlugins`'s cargo step, `📜️script.ts setup deps cargo`, etc. |
| `CMD_BUDGET_MS` | 0 | `SEMIO_CMD_BUDGET_MS` | everything else run via `runCmd`/`runCmdStatus` without an explicit `budgetMs` |
| `ORCHESTRATOR_BUDGET_MS` | 0 | `SEMIO_ORCHESTRATOR_BUDGET_MS` | `orchestratorBudgetOpts()` — nx/script orchestrator wrapper spawns |
| `DAEMON_BUDGET_MS` | 0 | `SEMIO_DAEMON_BUDGET_MS` | `daemonBudgetOpts()` — every `dev`/`serve` daemon spawn, including both `DevScript` branches |

`0` reaches Node's `spawnSync({ timeout: 0 })`, which Node treats as "no timeout" (confirmed at
`runCmdInternal`, `🏃️process/🟦️.ts:87-96`) — so both the sync (`runCmd`) and async-daemon
(`spawnDaemon`) budget paths are genuinely unbounded by default today, not silently capped. The only
*hard-coded, non-zero* budgets left in this file are the **test-level** ones
(`runTestBudgeted`/`runCargoTestBudgeted`, unrelated to `dev`):
`TEST_LEVEL_BUDGET_MS = { fundamental: 15_000, quick: 30_000, long: 300_000, exhaustive: 900_000 }`
(…🟦️.ts:1078-1082, override `SEMIO_TEST_BUDGET_MS`) — 15 minutes at the top `exhaustive` level, not 20,
and it only applies to `bun ./📜️script.ts test …`, never to `dev`/`serve`/`build`.

If the "20-minute cargo budget" experience is still real on some other machine, it is coming from an
explicit `SEMIO_BUILD_BUDGET_MS`/`SEMIO_DAEMON_BUDGET_MS` set in that shell's environment or a
`.envrc`/CI config, not from this repo's compiled-in defaults as of `de617a7c17`.

---

## 7. `.claude/launch.json` — no puzzle3d entry; proposed addition (not written)

Checked `.claude/launch.json` in full — no `puzzle3d`/`puzzle-3d` entry exists. The closest sibling,
`procedural3d-react`, reads:

```json
{
  "name": "procedural3d-react",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "procedural", "3d"],
  "port": 6019,
  "env": { "PROCEDURAL_3D_PLAY_PORT": "6019" }
}
```
(same dead-env-var pattern as `PUZZLE_3D_PLAY_PORT`, confirmed harmless in §1). Mirroring it — using the
root `📜️script.ts` directly rather than `bun nx run workspace:dev` (fewer Nx-daemon-graph moving parts
for a `preview_start` launch) — the entries to add, once the react path is fixed (§0) or if you want the
wgpu path today:

```jsonc
{
  "name": "puzzle3d-react",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "3d"],
  "port": 6013,
  "env": { "SEMIO_RENDERER": "react", "PUZZLE_3D_PLAY_PORT": "6013" }
},
{
  "name": "puzzle3d-wgpu",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["./📜️script.ts", "dev", "3d"],
  "port": 6113,
  "env": { "SEMIO_RENDERER": "wgpu", "PUZZLE_3D_PLAY_PORT": "6113" }
}
```
`preview_start`'s own `url` field isn't needed for either — both bind to `http://localhost:<port>`
directly; navigate to `/?plugin=puzzle3d` after the pane opens (§3/§0) rather than the bare root.

---

## Files referenced (all read-only)

- `/Users/ueli/Documents/semio/.vscode/launch.json:1967-2077`
- `/Users/ueli/Documents/semio/📋️project.json`
- `/Users/ueli/Documents/semio/📜️script.ts:230-524,17658` (root `DevScript`, playground alias resolution)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts:1-70` (`loadFrameworkOsPlaygroundSelections`)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2588-2828,2193-2211` (playground ports, `runViteBunxDev`, `spawnDaemon`)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts:1-96` (all budget constants)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:93,575-760,1042-1479,2620-2645,4118-4140,5405-5518` (os-dev `DevScript`/`ServeScript`/`PreparationScript`/`ActivationScript`, `buildPlugins`, `buildEngineWasm`, router)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts:1-60`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts:1-133` (`browserModuleRoot`, `SupportScript`, `MaterializeScript`)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:1-65,482-489`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts:1-17` (`WasmScript`/BoardSession)
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️findings-2026-09-05.md` (§14-§46 read in full for this audit)
- `/Users/ueli/Documents/semio/.claude/launch.json` (read in full, no puzzle3d entry)
