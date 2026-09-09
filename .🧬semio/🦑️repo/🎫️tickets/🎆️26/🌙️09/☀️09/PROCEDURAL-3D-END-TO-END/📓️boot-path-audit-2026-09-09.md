# Procedural 3D boot-path audit (read-only, 2026-09-09)

Read-only. No file outside this ticket was touched, no server/cargo build was started, no git command
was run. All file:line evidence read against the working tree as of this audit (HEAD `9b605a4550`
region, other sessions still committing under this ticket's neighbours — ignored per instructions).

Artifact: `s.procedural.generation3d@1`. Plugin crate: `semio-s-plugin-procedural`
(`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`). Playground **variant** id: `generation3d`
(not `procedural3d` — every Nx target and runtime directory below is named `generation3d`). Aliases:
`["procedural 3d"]`. Ports: react `6018`, wgpu `6118`
(`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:20-25`).

Prior art read in full and reconciled against current HEAD: puzzle3d ticket's
`📓️2026-09-08-dev-boot-path.md`, `📓️2026-09-09-wave-R-react-boot-pipeline.md`,
`📓️2026-09-09-runtime-verification.md`, tail of `📓️status.md`. **The wave-R fix (react boot goes
through the Nx `activate-<variant>-react-<profile>` chain, `served` skips it) is committed and is
fully variant-agnostic** — confirmed by reading current `DevScript`/`ServeScript`/
`activatePlaygroundRuntime` source (below), which take a bare `variant`/`plugin` string with no
puzzle-specific branch. Procedural3d gets the same fix automatically; nothing puzzle-specific needs
porting.

## Headline

- **React and wgpu are both code-reachable today**, exactly as they are for puzzle3d post-wave-R. Vite
  will run `activatePlaygroundRuntime("generation3d","dev")` (an Nx invocation) before serving unless
  the `served` segment is passed; wgpu runs the older `buildPlugins`/`buildEngineWasm(no-op)` path.
- **Nothing is currently staged for the react path**: `dist/runtime/dev/` only has `note` and
  `puzzle3d` — **no `generation3d`** — and the profiled `dist/dev/🔌️plugin-modules/` tree has no
  `generation3d`/`procedural` entry either. React boot for procedural3d has never been run to
  completion on this box; it starts cold.
- **The wgpu-path unprofiled component is stale**: 513 `.rs` files under
  `✏️s/🔌️plugins/🌀️procedural` are newer than the staged `.core.wasm` (2026-09-07 08:45).
- **The build closure for procedural3d is bigger than puzzle3d's** — 11 crates, not 2 — because
  `consumes = ["forms.questionKind", "flow.extension"]` pulls in every crate that `contributes` those
  topics (`forms` + the 9 `flow-extension-*` crates under `✏️s/🔌️plugins/🌊️flow/🧩️extensions/`). None of
  them depend on `stdio`, so procedural3d **does not** hit puzzle3d's stdio-404 trap — but `forms`'
  staged component is 162 `.rs` files stale (Aug 17 build) and, unlike stdio, **does have** a
  descriptor, so it won't 404 — it will just serve 3+-week-old forms behavior if not rebuilt.
- No `opt-level = 2` override exists for `semio-s-artifact-procedural-generation3d` in root
  `Cargo.toml` (there is one for `puzzle-3d` and `lowpoly-lowpoly`, lines 483/489) — if procedural3d
  hits the same "interactive-ceiling" maintenance-step-budget class of fault puzzle3d did, there is no
  committed per-crate mitigation to fall back on; that would need the same kind of fix wave (W-R2 in
  the puzzle3d ticket) applied to procedural3d, not assumed already covered.
- A **valid seed exists**: repo-root `target-gen3d-opt/` (1.9 GB, mtime 2026-09-08 21:18) has resolved
  fingerprints and compiled `.rlib`/`.rmeta` for the procedural/generation2d/generation3d/assembly
  crates and dozens of their framework dependencies under `wasm32-wasip2/wasm-dev/` — but **no final
  component `.wasm`** was found in it (deps only). It is a genuine partial-build seed, not a substitute
  for a real build. No process currently holds it (nothing running against that path right now).

## 1. `.vscode/launch.json` entries

| entry | file:line | command | env | port |
|---|---|---|---|---|
| `🛠️dev🔧️procedural🏙️3d⚛️react` | `.vscode/launch.json:2016-2033` | `bun nx run workspace:dev -- procedural 3d` | `PROCEDURAL_3D_PLAY_PORT=6018` (dead, see below), `SEMIO_RENDERER=react` | 6018 |
| `🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm` | `.vscode/launch.json:2035-2052` | same command | `PROCEDURAL_3D_PLAY_PORT=6118` (dead), `SEMIO_RENDERER=wgpu` | 6118 |
| `🛠️dev🔧️procedural🏙️3d🧊️wgpu🖥️native` | `.vscode/launch.json:2054-2063` | `bun nx run @semio-tech/framework-renderer-wgpu:native -- generation3d` | none | n/a (native window, no port) |
| `🛠️dev🔧️procedural🏙️3d🧩️hexagonal🧱️column⚛️react` | `.vscode/launch.json:2169-2187` | `bun nx run workspace:dev -- procedural 3d fixture hexagonal-column` | `PROCEDURAL_3D_PLAY_PORT=6018`, `SEMIO_RENDERER=react` | 6018 |
| `…hexagonal🧱️column🧊️wgpu🌐️wasm` | `.vscode/launch.json:2189-2206` | same, `SEMIO_RENDERER=wgpu` | `PROCEDURAL_3D_PLAY_PORT=6118` | 6118 |
| `…hexagonal🧱️column🧊️wgpu🖥️native` | `.vscode/launch.json:2208-2215` | `bun nx run @semio-tech/framework-renderer-wgpu:native -- generation3d` | none | n/a |
| `🛠️dev🏚️mitbestand⚙️generator🧊️wgpu🌐️wasm` | `.vscode/launch.json:25309-25327` | `bun nx run workspace:dev -- generator` | `S_OS_PORT=6127, SEMIO_PLUGIN=demonstrator, SEMIO_RENDERER=wgpu, SEMIO_APP=s.procedural.generation3d@1/*#editor` | 6127 |
| `🛠️dev🏚️mitbestand⚙️generator🧊️wgpu🖥️native` | `.vscode/launch.json:25329-25340` | `bun nx run @semio-tech/framework-renderer-wgpu:native -- generator` | `SEMIO_PLUGIN=demonstrator, SEMIO_APP=s.procedural.generation3d@1/*#editor` | n/a |

Plus the full generated ten-row Nx-target set (same shape as puzzle3d's, described in §2):
`.vscode/launch.json:25098-25200` — `🎮️generate🧩️generation3d session` (25098-25107),
`🎮️prepare🧩️generation3d⚛️react dev|release` (25109-25129), `🎮️build🧩️generation3d⚛️react release`
(25131-25140), `🎮️activate🧩️generation3d⚛️react dev|release` (25142-25162),
`🎮️serve🧩️generation3d⚛️react dev|release` (25164-25195), `🎮️dev🧩️generation3d⚛️react dev|release`
(25175-25206).

**Which are wired to a code path that exists today, and which aren't:**

- The `⚛️react` and `🧊️wgpu🌐️wasm` rows (2016-2052, 2169-2206): **wired**. They resolve through
  `resolvePlaygroundDevApp` → `runFrameworkOsPlaygroundDev` → the same generic `DevScript`/`ServeScript`
  code puzzle3d uses (§2).
- The `🧊️wgpu🖥️native` rows: **wired**, `@semio-tech/framework-renderer-wgpu:native` is a real target
  (not traced further — out of scope of the react/wgpu playground boot this audit covers; no port to
  verify against since it opens a native window).
- The `🏚️mitbestand⚙️generator` rows (25309-25340): **wired but a different boot**. `SEMIO_PLUGIN=demonstrator`
  boots the **demonstrator** plugin crate (`✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`),
  whose own playground row is `variant = "generator"` with `app = "s.procedural.generation3d@1/*#editor"`
  (Cargo.toml:39-40) and `depends-on = ["cad","gis","procedural","process","puzzle","sourcing"]`
  (Cargo.toml:31) — it statically embeds procedural
  (`procedural = { path = "…/🌀️procedural/📦️packages/🦀️rust", package = "semio-s-plugin-procedural",
  default-features = false }`, Cargo.toml:142, `default-features=false` turns off procedural's own
  `plugin-entry` feature exactly per its own docstring at
  `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:44-49`, avoiding the duplicate `#[no_mangle]`
  link error). `SEMIO_APP` then selects which embedded app the demonstrator's shell opens on boot.
  **This is not a substitute for booting procedural3d standalone** — it builds/serves the whole
  six-pane demonstrator bundle (cad+gis+procedural+process+puzzle+sourcing in one wasm component), a
  much heavier and differently-scoped boot. Use it only if the task specifically needs the
  demonstrator's embedding of generation3d; the recipes in §6 use the standalone procedural3d rows.
- **`PROCEDURAL_3D_PLAY_PORT` is dead**, same class of bug the puzzle3d audit found for
  `PUZZLE_3D_PLAY_PORT`: zero read sites repo-wide (grepped `.ts`, excluding `node_modules`/`target`).
  The actual port comes from `frameworkOsPlaygroundDefaultPort` reading Cargo-declared
  `ports.react=6018`/`ports.wgpu=6118`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2598-2602`), consumed
  as `S_OS_PORT`. Harmless because the numbers happen to match.

`.claude/launch.json:36-48` `procedural3d-react` has a **real defect**, the same class the puzzle3d
wave-R doc flagged for `puzzle2d-react`: it sets `PROCEDURAL_3D_PLAY_PORT=6019` (already dead, and
wrong — Cargo declares 6018) and declares `"port": 6019`, but **sets no `SEMIO_RENDERER`**.
`frameworkOsPlaygroundDevEnv` defaults `SEMIO_RENDERER` to `wgpu`
(`…📦️packages/🟦️typescript/🟦️.ts:2618-2620`), so this entry actually binds **wgpu on port 6118**, not
react on 6019. `preview_start`/VS Code's `serverReadyAction` port-match will simply never fire. Add
`"env": {"SEMIO_RENDERER": "react"}` (or drop the dead `PROCEDURAL_3D_PLAY_PORT` and use port 6018) if
this entry is meant to launch react. **No puzzle3d-style `.claude/launch.json` rows exist for wgpu or
for the direct Nx target** (`serve/dev-generation3d-react-dev`) — only the one broken react-labeled
entry. See §6 for corrected entries a later agent could add (not written, per read-only scope).

## 2. Command resolution chain (`bun nx run workspace:dev -- procedural 3d`)

| step | file:line | what it does |
|---|---|---|
| root `📋️project.json` target `dev` | `📋️project.json:131-136` | `nx:run-commands`, `bun ./📜️script.ts dev`, `forwardAllArgs: true` → args become `dev procedural 3d` |
| root `📜️script.ts` router | `📜️script.ts:17899` | `.register("dev", DevScript)` |
| root `DevScript.run` | `📜️script.ts:487-524` | segments `["procedural","3d"]` don't match `storybook`/`s`/`multi`/`mcp`; falls to `resolvePlaygroundDevApp(segments)` (line 513-517) |
| alias resolution | `📜️script.ts:236-239` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2605-2614` | `resolveFrameworkOsPlaygroundPlugin` tries longest alias first: `"procedural 3d"` (2 segments) matches the Cargo-declared `aliases=["procedural 3d"]` exactly → returns `{plugin:"generation3d", rest:[]}` |
| catalog source | `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:20-25` | `variant="generation3d"`, `app="s.procedural.generation3d@1/*#editor"`, `aliases=["procedural 3d"]`, `ports={react:6018,wgpu:6118}` |
| spawn | `📜️script.ts:255-261` (`runFrameworkOsPlaygroundDev`) | `bun nx run @semio-tech/framework-os-dev:dev -- generation3d`, env via `frameworkOsPlaygroundDevEnv` (`served` here is false, so no react-force) |
| os-dev `📋️project.json` target `dev` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | `bun ./📜️script.ts dev`, cwd os-dev package |
| os-dev `📜️script.ts` router | `…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5420`-ish (register call, unchanged from puzzle3d audit) | `.register("dev", DevScript)` — the **os-dev** `DevScript`, distinct from root's |
| os-dev `DevScript.run` | `…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1439-1479` | branches on `renderer` — see below |

### `renderer=react` path (current, post-wave-R)

```ts
// …🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1439-1449
class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const served = segments.includes("served");
    …
    const plugin = variantSegment ?? process.env.SEMIO_PLUGIN ?? …;
    const renderer = … process.env.SEMIO_RENDERER ?? "react";
    if (renderer === "react") {
      const profile = semioBuildMode() === "ship" ? "release" : "dev";
      if (!served) await activatePlaygroundRuntime(plugin, profile);
      await new ServeScript(this.root).run([plugin, renderer, profile, ...serverArgs]);
      return;
    }
```

- `activatePlaygroundRuntime("generation3d","dev")` (defined `…📜️script.ts:1428-1436`) runs
  `bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`. Its Nx `dependsOn` chain
  (inferred by `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`, `componentTargets` line 705,
  `playgroundSessionTargets` line 736, `playgroundPreparationTargets` line 758 — **variant-agnostic**,
  same functions puzzle3d's wave-R doc verified by running `bun nx show project`) is, by the same shape
  documented there:
  `@semio-tech/plugin-registry:session-generation3d` + `@semio-tech/framework-plugin-web:support-dev` +
  `semio-framework-os-infinite:fonts` + `{surface,editor,flow-core}:wasm` +
  **every crate in the closure** (`@semio-tech/procedural-plugin:wasm` +
  `@semio-tech/procedural-plugin:materialize-dev`, plus the 9 flow-extension crates' and forms'
  `component-dev`/`materialize-dev` if the closure includes them as separate playground-owned
  materializations — not independently re-verified by running `bun nx show project` here, per the
  no-build constraint; inferred from the same generic `🟨️.mjs` logic already confirmed generic for
  puzzle3d). **This was not run** (would trigger a real cargo build); treat the exact task list as
  "the same shape as puzzle3d's, scaled to procedural3d's larger closure" until a later agent runs
  `bun nx show project @semio-tech/framework-os-dev --json` to confirm.
- `ServeScript.run(["generation3d","react","dev"])` (`…📜️script.ts:1413-1421`) reads the activation
  receipt at `developmentRuntimeRoot(root,"generation3d","dev")` =
  `…🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/generation3d/activation/🔣️receipt.json` — **does not
  exist yet** (§4 below) — then `runViteBunxDev` with `S_OS_PORT` defaulting to 6018.
- `served` segment (`bun ./📜️script.ts dev procedural 3d served` from the **os-dev** package, or via
  the root's `runFrameworkOsPlaygroundDev` which forwards `served` as a plain segment,
  `📜️script.ts:255-261`) skips `activatePlaygroundRuntime` and serves whatever is already staged —
  fails today because nothing is staged (§4).

### `renderer=wgpu` path (unchanged shape from puzzle3d's pre-wave-R wgpu branch)

1. `ensureAppleDeveloperDir()` — macOS no-op prerequisite.
2. `await buildPlugins("generation3d")` (`…📜️script.ts:1451`-ish → function at `685-703`):
   - `preparePluginBuildTargets("generation3d")` (`656-684`): `ensureWasmTarget()`, then
     `ensurePluginRegistry("generation3d")` (`615-621`) — regenerates the registry, then
     `resolveCatalogFilterPluginId("generation3d")` (`161-163`) maps variant → crate pluginId
     `"procedural"` via `resolvePlaygroundFilter` (`155-158`).
   - `filterProjectedPluginRegistry` (`🔌️plugin/📇️registry/📜️script.ts:579-584`) →
     `resolveRegistryPluginIdsForFilter` (`622-626`) → `runtimeComponentClosure`
     (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs:6-29`): starts
     from `["procedural"]`, walks `dependsOn` (none declared for procedural) and, **for each topic in
     `consumes`, pulls in every crate that `contributes` it**. Procedural declares
     `consumes = ["forms.questionKind", "flow.extension"]`
     (`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:16`). Verified contributors:
     `forms` (`consumes=["forms.questionKind"]` itself, `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/Cargo.toml:16`
     — no `contributes` line found there; the actual `forms.questionKind` contributor was not
     independently traced beyond `forms`) and the nine crates under
     `✏️s/🔌️plugins/🌊️flow/🧩️extensions/{🏗️bim,📃️list,📐️brep,📖️dictionary,📝️text,🔤️primitive,🖍️draw,🧠️logic,🧮️math}/📦️packages/🦀️rust/Cargo.toml`,
     each declaring `contributes = ["flow.extension"]` (line 17 in each, docstrings literally say
     "…to flow-play and procedural3d-play"). **Net closure: 11 crates** (`procedural` + `forms` + 9
     `flow-extension-*`) — bigger than puzzle3d's `puzzle,stdio`. **None declare a dependency on
     `stdio`** (grepped every Cargo.toml in the closure) — the stdio-404 trap that blocked puzzle3d's
     wgpu boot does **not** apply here.
   - `buildPluginCatalog` (`578-608`) `cargo build`s each target serially, then materializes to
     `pluginOutRoot` = `…🧑‍💻dev/🔌️plugin-modules/` (unprofiled tree, same as puzzle3d's wgpu path).
3. `await buildEngineWasm("generation3d","wgpu")` — **no-op** (early return for non-react, same line
   ~1050 area as puzzle3d's finding).
4. Port resolution / stale-trunk handling identical logic to puzzle3d's wgpu branch
   (`…📜️script.ts:1440-1478`-ish, unchanged by wave-R which touched only the react branch).
5. Spawns `bun 🎯️targets/🧊️wgpu/📜️script.ts serve` with `SEMIO_PLUGIN=generation3d SEMIO_RENDERER=wgpu
   S_OS_PORT=6118`.

### Env vars honoured

| var | honoured? | where |
|---|---|---|
| `SEMIO_RENDERER` | yes | `DevScript.run`, decides react vs wgpu branch |
| `S_OS_PORT` | yes | `ServeScript`/wgpu port resolution, defaults from Cargo `ports.{react,wgpu}` |
| `SEMIO_APP` | yes, but only meaningful for a **host-filtered** boot (the demonstrator's `generator` variant, §1) — for a direct `procedural 3d` boot the app id is already pinned by the catalog row and `SEMIO_APP`/`VITE_SEMIO_APP_ID` would only matter if overriding which embedded app a host shell opens |
| `SKIP_PLUGIN_BUILD` | **dead** — zero read sites repo-wide (same finding as puzzle3d's audit; only write sites remain, and per wave-R those writes were removed from `collabStartUserDevServer`/`startParityDevServer`) |
| `SEMIO_PLUGIN_ONLY` | honoured but a **trap** for a multi-crate closure like generation3d's: `resolvePluginBuildTargets` (`…📜️script.ts:628-641`) forces a single crate and silently drops the other 10, which then 404 on `plugin.descriptor-unavailable` for whichever missing crate the shell needs. Leave unset. |
| `CARGO_TARGET_DIR` | honoured straight through to `cargo` (inherited env, same as puzzle3d) |
| `CARGO_PROFILE_WASM_DEV_DEBUG` | **redundant, not dead** — root `Cargo.toml:465-466` already sets `[profile.dev] debug = false` and `[profile.wasm-dev] inherits = "dev"` (`:474-475`), so the 8.6 GB→165 MB RSS fix is committed. Passing the env anyway is harmless. |

## 3. Staged artifacts and staleness

### wgpu / unprofiled path (`…🧑‍💻dev/🔌️plugin-modules/`)

```
🌀️procedural/
  semio_s_plugin_procedural_component.core.wasm   64,507,898 B   2026-09-07 08:45
  semio_s_plugin_procedural_component.js             443,352 B   2026-09-07 08:45
  semio_s_plugin_procedural_component.d.ts              3,457 B   2026-09-08 20:07
  🌉️bridge.js                                          9,447 B   2026-09-07 08:45
  🔣️.json                                           1,014,353 B   2026-09-08 14:08
  🛂️.descriptor.semio                                 229,409 B   2026-09-08 14:08
  🟨️.js                                                 6,808 B   2026-09-07 08:45
```
`find ✏️s/🔌️plugins/🌀️procedural -name '*.rs' -newer <that .core.wasm>` → **513 files** newer than the
staged component. Descriptor is present (regenerated 2026-09-08 14:08, a day after the wasm — so the
descriptor itself may already be desynced from the wasm bytes it describes) but the underlying wasm is
stale against source. Not a 404 risk like stdio was for puzzle3d, but **will serve stale procedural
behaviour** if trusted without a rebuild.

The 9 flow-extension crates + forms (the rest of the wgpu closure):

| plugin dir | wasm mtime | size |
|---|---|---|
| `🏘️flow-extension-bim` | 2026-09-08 17:34 | 5.2 MB |
| `📃️flow-extension-list` | 2026-09-06 09:15 | 14.4 MB |
| `🧊️flow-extension-brep` | 2026-09-06 09:15 | 21.2 MB |
| `📚️flow-extension-dictionary` | 2026-09-06 09:15 | 14.4 MB |
| `📝️flow-extension-text` | 2026-09-06 09:15 | 14.3 MB |
| `🔤️flow-extension-primitive` | 2026-09-06 09:15 | 14.4 MB |
| `🎨️flow-extension-draw` | 2026-09-06 09:15 | 15.7 MB |
| `🔀️flow-extension-logic` | 2026-09-06 09:15 | 14.3 MB |
| `🧮️flow-extension-math` | 2026-09-06 09:15 | 14.5 MB |
| `📋️forms` | **2026-08-17 21:20** | 24.7 MB |

`forms` is **162 `.rs` files stale** (`find ✏️s/🔌️plugins/📋️forms -name '*.rs' -newer <forms wasm>`) —
3+ weeks behind, same order of staleness class as puzzle3d's stdio blocker, but it **does have** a
descriptor (`🔣️.json` 2026-09-04, `🛂️.descriptor.semio` 2026-09-08) so it will **not** 404 — it will
just run old forms code. The 9 flow-extension crates are all fresh (Sep 6-8), no `.rs`-newer check run
against every one individually (time-boxed), but their mtimes cluster with the rest of a recent
same-week build.

### react / profiled path (`…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/`)

**No `generation3d`/`procedural` entry.** Listing matches the puzzle3d pre-wave-R audit exactly:
only `🧵️shard`, `🪞️vendor`, `🗒️note` present. Confirms react has never been materialized for
procedural3d on this box.

### `…📇️registry/dist/sessions/`

`generation3d/` **does exist** (staged, presumably from a `session-generation3d` Nx run or the
`ensurePluginRegistry` step of a wgpu boot attempt — not dated/inspected further).

### `dist/runtime/<profile>/<variant>/activation/`

`…🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/` contains only `note` and `puzzle3d` —
**no `generation3d`**. React boot for procedural3d starts fully cold; there is no stale receipt to
trip over (unlike a scenario where a stale receipt with mismatched `variant`/`profile` throws — that
just isn't reachable here because the directory is absent).

### Engine wasm (`buildEngineWasm`'s three crates)

Not re-checked in this audit (unchanged since the puzzle3d audit read them fresh as of 2026-09-08
20:51/21:00/20:25 — likely still fresh a day later on an actively-worked framework, but not verified
here; re-check mtimes before trusting `SKIP_ENGINE_BUILD` if that ever becomes relevant to the react
path, which per the puzzle3d wave-R doc it no longer is for `dev`/`activate` — Nx's `wasm` targets own
freshness there now).

## 4. Example fixtures and selection mechanism

Eight examples under
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`:

```
🍄️hexagonal-mushroom-column   🍩️sphere-cut-with-torus   🐚️box-shell-preview
📐️box-fillet-preview          📦️rectangle-extrude-volume 🧲️sphere-box-fuse
🧹️face-sweep-extrude          🪢️rectangle-wire-preview
```

Each has `🖼️assets/`, `🟦️.ts`, `🦀️.rs`, `🧪️tests/`.

**Selection mechanism** (traced, not the launch.json `fixture <name>` segment — see below):
- `resolveBootExampleId(activeExampleId, exampleOptions, defaultsExampleId)`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:289-297`): keeps a
  still-valid active/default id, else falls back to `exampleOptions[0]?.id`. The engine-contract test
  (`…🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:7449`) pins `resolveBootExampleId("", options)` →
  `"hexagonal-mushroom-column"` for this exact scenario — first-registered example wins with nothing
  else set, matching the folder listing's first alphabetical/declared entry.
- `defaultsExampleId` comes from `import.meta.env.VITE_SEMIO_DEFAULT_EXAMPLE`
  (`…🧑‍💻dev/🟦️.ts:41`) — a boot-time **default**, still overridable in-app.
- A separate, stronger lock exists: `import.meta.env.VITE_SEMIO_LOCKED_EXAMPLE`
  (`…🧑‍💻dev/🟦️.ts:32`) — a boot-time-only lock with **no `?query=` override** per its own docstring
  (`…🧑‍💻dev/🟦️.ts:29-30`).
- `activeExampleId` itself (the "still-valid active" branch) is durable per-brand shell state
  (`initialShellState`/localStorage under the `semio.os.`/`ui.chrome.` prefixes cleared by
  `clearDurableShellStorage`, `…🐚️Shell/🟦️.tsx` — not a URL hash/query mechanism).
- **The launch.json `procedural 3d fixture hexagonal-column` segment is dead for both renderers**,
  same finding the puzzle3d audit made for its `fixture concrete` rows: grepped
  `…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` and root `📜️script.ts` for any `"fixture"` literal —
  **zero matches**. `resolvePlaygroundDevApp` strips the alias `"procedural 3d"` (longest match) and
  passes `["fixture","hexagonal-column"]` through as `rest`/`serverArgs`: for react these become extra
  Vite CLI args (harmless, Vite ignores unknown positional args or errors — not tested here); for wgpu
  they are simply never read. **To actually pin an example, use `VITE_SEMIO_DEFAULT_EXAMPLE=hexagonal-mushroom-column`
  (soft) or `VITE_SEMIO_LOCKED_EXAMPLE=hexagonal-mushroom-column` (hard, no in-app override) as an env
  var on the react boot**, not a command-line segment. No equivalent env-var pass-through was traced
  for the wgpu/native renderer in this audit (out of scope — wgpu's example selection likely lives in
  the Rust host/native window, not this TS layer).
- Note the folder is `🍄️hexagonal-mushroom-column`, not `hexagonal-column` — the launch.json fixture
  segment's value doesn't even match a real example id (further evidence it's vestigial/copy-pasted,
  never resolved against the example catalog).

## 5. Cargo build hygiene and target-dir inventory

Repo memory recipe (unchanged, reconfirmed applicable): private lane-qualified `CARGO_TARGET_DIR`
outside the repo (or at least not the shared root `target/`), `RUSTC_WRAPPER=""` to bypass sccache
serialization, `CARGO_PROFILE_WASM_DEV_DEBUG=false` (redundant here per §2 but harmless), nohup+disown
with a scratchpad log if backgrounding, poll readiness with `curl`, never `timeout` (no such command on
macOS).

**Existing target dirs found, with sizes/mtimes (2026-09-09, this audit's read pass):**

| path | size | mtime | relevance |
|---|---|---|---|
| `./target` (repo root, shared) | 67 GB | 2026-09-09 09:12 | the shared lock-contended dir every session should avoid pointing `CARGO_TARGET_DIR` at |
| `./target-gen3d-opt` (repo root) | 1.9 GB | 2026-09-08 21:18 | **candidate seed** — see below |
| `/Users/ueli/.semio-targets/p3d` | 11 GB | 2026-09-08 21:14 | puzzle3d's private target (wave-R's `CARGO_TARGET_DIR=/Users/ueli/.semio-targets/waveR` sibling); has a valid stale puzzle component per the wave-R doc, not procedural-relevant |
| `…/9f5f6952…/scratchpad/target-buildaudit` (this session's own scratchpad) | 36 MB | 2026-09-09 14:15 | **native `debug` profile**, not `wasm32-wasip2` — not a wasm seed; likely another concurrent task's tiny `cargo check` in this same session lineage, pre-existing before this audit started (this audit ran no cargo) |
| `…/9e1e818a…/scratchpad/target-p3d-wasm` | 32 GB | 2026-09-09 04:32 | peer session, puzzle-labeled, has only `semio-framework-os-infinite` build-script output for procedural2d icon assets, no procedural component — not a useful seed |
| `…/9e1e818a…/scratchpad/target-p3d` | 38 GB | 2026-09-09 03:29 | peer session, puzzle-labeled — not inspected further (out of scope, not procedural-relevant) |
| `…/cd1780e5…/scratchpad/target-w6`, `target-w3` (and siblings `w4,w5,w10,w2e-registry,w10d`) | 1.8-2.3 GB each | 2026-09-08 19:44-20:47 | peer session's wave-lettered targets, not inspected (unrelated ticket by naming) |

**`./target-gen3d-opt` is the one worth reusing.** It has resolved `.fingerprint` entries (2026-09-08
22:22) for exactly `semio-s-plugin-procedural`, `semio-s-artifact-procedural-assembly`,
`semio-s-artifact-procedural-generation3d`, and `semio-s-artifact-procedural-generation2d` under
`wasm32-wasip2/wasm-dev/`, plus ~130 compiled `.rlib`/`.rmeta` files for shared framework dependencies
(`semio_framework`, `semio_framework_3d`, `semio_framework_actor`, `semio_framework_geometry`, serde/
wit-bindgen toolchain crates, etc.) — real, resolved dependency-graph state for this exact build. **No
final component `.wasm` was found** in `wasm32-wasip2/wasm-dev/deps/` or at the `wasm-dev` top level —
so this is mid-build state, not a finished artifact to copy in directly (unlike wave-R's copy-the-wasm
trick for puzzle). No process currently has it open (`ps` found nothing referencing the path). **If a
later agent's build points `CARGO_TARGET_DIR` here (same path, same profile env), cargo's own
incremental/fingerprint reuse should pick up the resolved deps and only need to compile
procedural/generation2d/generation3d/assembly themselves** — worth trying before a from-scratch build,
but verify the profile flags used to produce it match (not established here; the fingerprint files
don't reveal the exact env without a real build attempting to reuse them).

## 6. Recipe

**Open URL after boot**: `http://127.0.0.1:<port>/?plugin=generation3d` — the bare root defaults to the
full `s` studio host and expands the entire plugin catalog, same as puzzle3d's `?plugin=puzzle3d`
finding (`browser-boot` query resolution is renderer- and variant-agnostic;
`resolvePlaygroundBoot(PLUGIN_CATALOG, import.meta.env.VITE_SEMIO_PLUGIN || PLAYGROUND_SESSION.variant, …)`
in `…🧑‍💻dev/🟦️.ts:12-14` reads `VITE_SEMIO_PLUGIN`, which `ServeScript`/the wgpu spawn both set to the
variant string).

### React (port 6018) — cold, will build the 11-crate closure through Nx

```bash
cd /Users/ueli/Documents/semio
bun nx run @semio-tech/framework-os-dev:dev-generation3d-react-dev
```
This is the single command that pulls `component-dev`/`materialize-dev` for all 11 crates,
`support-dev`, `fonts`, the three engine `wasm` targets, `session-generation3d`, `prepare`, `activate`,
then serves — Nx's own cache decides what's already fresh. **Do not** invoke via
`SEMIO_RENDERER=react bun ./📜️script.ts dev procedural 3d` directly if reproducing puzzle3d's §8.1
open question: that root-script path was still unexplained-flaky through `bun nx run …` from a spawned
child as of the puzzle3d ticket's last note (async-ESM Nx plugin loading failure only when invoked via
`runCmd`'s `spawnSync`, not from an interactive shell) — untested here for procedural3d; use the direct
Nx target above (or the `.vscode/launch.json` row `🎮️dev🧩️generation3d⚛️react dev`) to sidestep it.

If a previous attempt already staged the profiled tree and you only want to serve it without rebuilding:
```bash
cd /Users/ueli/Documents/semio
SEMIO_RENDERER=react S_OS_PORT=6018 bun ./📜️script.ts dev procedural 3d served
```
(will currently fail — nothing is staged yet, §3 — included for completeness once a build has run once).

### wgpu (port 6118) — builds the same 11-crate closure via `buildPlugins`, then `trunk serve`

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/<your-session-uuid>/scratchpad/target-gen3d \
RUSTC_WRAPPER="" \
SEMIO_RENDERER=wgpu \
S_OS_PORT=6118 \
bun ./📜️script.ts dev procedural 3d
```
- Leave `SEMIO_PLUGIN_ONLY` unset (§2 trap — would drop 10 of the 11 needed crates).
- `CARGO_PROFILE_WASM_DEV_DEBUG=false` is redundant (already committed) but harmless to add.
- Consider seeding from `./target-gen3d-opt` (§5) by rsyncing it into your private
  `CARGO_TARGET_DIR` before the build, same trick as the "Seed Private Target From Peer Check Target"
  memory — not verified here to actually save time (no final wasm was in it, only resolved deps).

### What "booted" looks like

Carried over verbatim from the puzzle3d runtime-verification doc (same shell/framework, only the
plugin differs):
- Console: `[DEBUG]` prefixed lines from the page's own hooks; a real boot briefly shows
  `[DEBUG] local interaction observation failed …` / `[DEBUG] action failed …` for the first ~10-30s
  while the reactor's cooperative-maintenance pump catches up (this was a real bug in puzzle3d, fixed
  across several rebuilds in that ticket's runtime-verification doc — if procedural3d shows the same
  `more-work streak=4096` pattern, it is very likely the **same framework-level issue**, not
  procedural-specific, and the fix already landed framework-side per that doc's rebuilds #4-#8; expect
  it not to reproduce, but don't assume — verify).
- DOM: `document.getElementById("root")` populated, window title reflecting
  `semio · procedural · 3d` (or similar — exact string not independently re-derived for this plugin;
  the pattern is `semio · <shellLabel> · <variant>` per puzzle3d's observed `semio · puzzle · 3d`).
- Two or more windows/panes rendering a 3D scene matching the boot example
  (`🍄️hexagonal-mushroom-column` per §4's default resolution), catalogue/utility panels populated.
- **Port-listening is not proof of a rebuild** (findings §44 in the puzzle3d ticket, verbatim
  applicable): confirm via `strings <staged .wasm> | grep <a string unique to your target commit>`, not
  just a successful `curl`.

### Known pitfalls carried over verbatim from the puzzle3d docs (apply here too)

- `SEMIO_PLUGIN_ONLY=<id>` silently drops every other crate the boot needs → runtime
  `plugin.descriptor-unavailable` 404. Leave unset (worse here: 11-crate closure, not 2).
- A killed `nx run …:dev`/`…:serve-<variant>-react-<profile>` orphans its `cargo`/`rustc` children
  (ppid 1) holding the shared `target/` lock — kill by pid after checking ancestry, not by name.
- `nx run …:dev -- <variant> served` is **not** a dry command through the bootstrap remap — it still
  builds the whole chain via `serve-<variant>-react-<profile>`'s `dependsOn`; only the direct
  `bun ./📜️script.ts dev <variant> served` (bypassing `bun nx`) is genuinely build-free.
- A fully loaded box (many concurrent sessions' cargo work) can turn a `cargo check`/`build` into a
  many-minutes-at-1%-CPU lock-wait; check `ps` for a live `rustc` child under the `cargo` pid before
  concluding it's hung.
- Do not run a full `s`-scoped/59-catalog rebuild by mistake — always scope through the
  `procedural 3d`/`generation3d` variant filter, never a bare `dev s` or `SEMIO_PLUGIN_ONLY` misuse.
- No `timeout` command on macOS — any recipe embedding one silently no-ops the whole line.

## Files referenced (all read-only)

- `/Users/ueli/Documents/semio/.vscode/launch.json:2016-2215,25098-25200,25309-25360`
- `/Users/ueli/Documents/semio/.claude/launch.json:1-78`
- `/Users/ueli/Documents/semio/📋️project.json:131-136`
- `/Users/ueli/Documents/semio/📜️script.ts:236-261,487-524,17899`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2598-2628`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:615-705,736,758`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs:1-29`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:150-172,575-703,1042-1061,1323-1479`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts:1-46`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:560-655`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:285-297`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:7447-7453`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:1-89`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/Cargo.toml:1-20`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/{🏗️bim,📃️list,📐️brep,📖️dictionary,📝️text,🔤️primitive,🖍️draw,🧠️logic,🧮️math}/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:1-150`
- `/Users/ueli/Documents/semio/Cargo.toml:280-284,453,465-489`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/` (directory listing)
- Puzzle3d ticket, read in full for reconciliation: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-08-dev-boot-path.md`, `📓️2026-09-09-wave-R-react-boot-pipeline.md`, `📓️2026-09-09-runtime-verification.md`, tail of `📓️status.md`
