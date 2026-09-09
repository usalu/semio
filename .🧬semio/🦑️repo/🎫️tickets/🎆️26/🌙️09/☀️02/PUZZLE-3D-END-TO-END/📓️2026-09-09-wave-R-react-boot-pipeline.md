# Wave R — puzzle3d react dev boot pipeline (design + implementation)

Ticket: `2026/09/02/PUZZLE-3D-END-TO-END` (owned by the coordinator; this wave neither opened nor
closed it). Companion audit: `📓️2026-09-08-dev-boot-path.md`.

## 0. Correction to the audit's headline gap

`📓️2026-09-08-dev-boot-path.md` §0/§3 concluded that the react pipeline is unreachable because *"no
plugin crate's `📋️project.json` registers a `materialize` target … zero matches for `"materialize"`,
`component-dev`, or `component-release`"*. That grep is accurate but the conclusion is not: those
targets are **not authored in `📋️project.json` at all** — they are **inferred by the repo's own Nx
plugin**, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`, which `nx.json` registers with
`include: ["**/📋️project.json", "**/Cargo.toml", …]`:

| inferring function (`🟨️.mjs`) | targets it synthesises |
|---|---|
| `componentTargets` (line 622) | `component-dev`, `component-release`, `materialize-dev`, `materialize-release` on **every** Cargo package with `package.metadata.component.package` + `semio.role ∈ {plugin, extension}` |
| `playgroundSessionTargets` (line 653) | `session-<variant>` on `@semio-tech/plugin-registry`, one per Cargo-declared playground variant |
| `playgroundPreparationTargets` (line 675) | `prepare|activate|serve|dev-<variant>-react-<profile>` and `build-<variant>-react-release` on `@semio-tech/framework-os-dev` |

Verified by running the graph, not by reading the plugin:

```
$ bun nx show project @semio-tech/puzzle-plugin --json
  … "component-dev"  → bun "…⚡️caching/🦀️cargo/📜️script.ts" native component dev --manifest "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml"
       outputs: ["{projectRoot}/dist/component-dev"], cache: true, parallelism: false
  … "materialize-dev" → bun "…🔌️plugin/📦️packages/🟦️typescript/📜️script.ts" materialize dev --manifest "…/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml"
       dependsOn: ["component-dev", "@semio-tech/framework-plugin-web:support-dev"], cache: true
       outputs: ["{workspaceRoot}/🧰️framework/…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle"]

$ bun nx show project @semio-tech/framework-os-dev --json
  prepare-puzzle3d-react-dev   dependsOn: [@semio-tech/plugin-registry:session-puzzle3d,
                                          @semio-tech/framework-plugin-web:support-dev,
                                          semio-framework-os-infinite:fonts,
                                          @semio-tech/framework-surface-rs:wasm,
                                          @semio-tech/framework-editor-rs:wasm,
                                          semio-framework-os-flow-core:wasm,
                                          @semio-tech/puzzle-plugin:wasm,
                                          @semio-tech/puzzle-plugin:materialize-dev]
  activate-puzzle3d-react-dev  dependsOn: [prepare-puzzle3d-react-dev]
  dev-puzzle3d-react-dev       dependsOn: [activate-puzzle3d-react-dev], continuous, command
                               `bun ./📜️script.ts serve puzzle3d react dev`
```

The peer's contract test already pins every one of these shapes:
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:531-625`
against `…/⚡️caching/🧫️fixtures/nx-contract/🔣️.json` (`componentProfiles`, `materialization`,
`playgroundSessions`, `playgroundPreparation`, `fontAssets`). The producers, the profiled
`dist/<profile>/…` outputs, the `.nx-artifact.json` staging convention (`stageArtifacts`) and the Nx
cacheability the wave-R brief asked for **already exist and are already tested**. `note` has been
carried through them end to end (`✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_note.wasm`,
49.8 MB, 2026-09-08 11:14, with its `.nx-artifact.json`).

So the wave-R gap is **not** the pipeline. It is exactly one thing: **the entry point everybody
actually uses does not enter the pipeline.**

## 1. The real gap

```
.vscode/launch.json  🛠️dev🧩️puzzle🏙️3d⚛️react
  → bun nx run workspace:dev -- 3d                                (env SEMIO_RENDERER=react)
  → root 📜️script.ts DevScript → runFrameworkOsPlaygroundDev("puzzle3d")
  → bun nx run @semio-tech/framework-os-dev:dev -- puzzle3d        ← the PLAIN `dev` target
  → os-dev 📜️script.ts DevScript.run, renderer==="react"
  → new ServeScript(...).run([...])                               ← reads the activation receipt
  → readActivationReceipt(dist/runtime/dev/puzzle3d/activation)   ← ENOENT, throws
```

`DevScript`'s react branch jumps to the **last** stage of a six-stage chain. The inferred
`dev-<variant>-react-<profile>` target is the one wired to the whole chain.

### 1.1 …except when the invocation passes through `bun nx`, which already remaps it

Second correction to the audit. `bun nx` is **not** the Nx CLI — `package.json`'s `nx` script is
`bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx`, and that
dispatcher's `resolveNxInvocation` (line 213-233) already rewrites the playground `dev` invocation:

```ts
// …⚡️caching/🚀️bootstrap/📜️script.ts:229-232
const served = app.rest.includes("served"), env = frameworkOsPlaygroundDevEnv(catalog, app.plugin, served ? { SEMIO_RENDERER: "react" } : {});
if (env.SEMIO_RENDERER === "react") {
  const profile = process.env.SEMIO_BUILD_MODE === "ship" ? "release" : "dev", command = served ? "serve" : "dev";
  return { args: ["run", `@semio-tech/framework-os-dev:${command}-${app.plugin}-react-${profile}`, … ], … };
}
return { args: ["run", "@semio-tech/framework-os-dev:dev", ...options, "--", app.plugin, ...app.rest], env };  // wgpu
```

Observed live (`bun nx run @semio-tech/framework-os-dev:dev -- puzzle3d served`):

```
 NX   Running target serve-puzzle3d-react-dev for project @semio-tech/framework-os-dev and 19 tasks it depends on:
✔  nx run @semio-tech/framework-schema:generate
✔  nx run @semio-tech/ui-rs:generate
✔  nx run @semio-tech/ui-styling-tokens:generate
✔  nx run workspace:deps-wasm-opt
✔  nx run @semio-tech/framework-plugin-web:support-dev
   … (and it started @semio-tech/puzzle-plugin:component-dev — see §6.9)
```

So the *Nx-mediated* react entry point was already wired to the chain, and the plain `dev` target is
reached only by a **direct** `bun ./📜️script.ts dev …` invocation that bypasses `bun nx` — which is
exactly what `collabStartUserDevServer`, `startParityDevServer` and the hub's browser-host harness do.
The wave-R fix therefore has two halves: make the direct invocation enter the chain too (§3.1), and
make `served` mean the same thing on both routes.

One consequence worth recording: through the bootstrap, `served` maps to `serve-<variant>-react-<profile>`,
whose `dependsOn` is `activate-…` — so it **still builds the whole chain** and only suppresses the
`watch`. "Serve exactly what is staged" is reachable only via the direct script command
(`bun ./📜️script.ts dev <variant> served`, or `serve <variant> react <profile>`). Left as-is here
because the Nx target shapes are pinned by the peer's contract test; see §8.

## 2. Target chain (unchanged — this is the peer's shape, restated for the record)

```
@semio-tech/puzzle-plugin:component-dev          cargo rustc --target wasm32-wasip2 --profile wasm-dev
    ↓ dist/component-dev/semio_s_plugin_puzzle.wasm (+ .nx-artifact.json)
@semio-tech/puzzle-plugin:materialize-dev        jco transpile + descriptor probe + bridge
  ⊕ @semio-tech/framework-plugin-web:support-dev preview2 shim vendor + shard worker
    ↓ …🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/{🔣️.json,🛂️.descriptor.semio,🌉️bridge.js,…}
@semio-tech/plugin-registry:session-puzzle3d     dist/sessions/puzzle3d/🟦️session.ts
semio-framework-os-infinite:fonts                dist/fonts/🔤️guestslim-typst-fonts.bin
{surface,editor,flow-core,puzzle-plugin}:wasm    engine 🕸️bindings / pkg
    ↓
@semio-tech/framework-os-dev:prepare-puzzle3d-react-dev    (verifies every input above)
    ↓
@semio-tech/framework-os-dev:activate-puzzle3d-react-dev   dist/runtime/dev/puzzle3d/activation/🔣️receipt.json
    ↓
@semio-tech/framework-os-dev:serve|dev-puzzle3d-react-dev  bunx vite, S_OS_PORT=6013
```

`session-puzzle3d` resolves the component closure to **`puzzle` only** — verified by running
`buildPlaygroundSession("puzzle3d")`, which returns `{variant: puzzle3d, registryPluginId: puzzle,
plugins: [puzzle]}`. **`stdio` is NOT in the react boot plan for puzzle3d** (it is only in the wgpu
branch's `filterProjectedPluginRegistry` scope), so the audit §4 "stdio is 3 weeks stale and has no
descriptor" blocker does **not** apply to the react path. Nothing needs to be done about stdio for
this wave.

## 3. What this wave changes

### 3.1 `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`

- New `activatePlaygroundRuntime(variant, profile)` — runs
  `bun nx run @semio-tech/framework-os-dev:activate-<variant>-react-<profile>` at the workspace root
  under `buildBudgetMs()`. One Nx invocation, so the **declared** dependsOn graph stays the single
  source of truth (no second copy of the closure in TypeScript) and Nx's own cache supplies the
  "reuse whatever is already fresh" behaviour the brief asked for.
- `DevScript.run` react branch: `activatePlaygroundRuntime(...)` **then** `ServeScript`. A `served`
  segment (already the launch.json spelling, e.g. `.claude/launch.json` `s-react-served`) skips the
  activation step and serves what is already staged — this is the honest replacement for the dead
  `SKIP_PLUGIN_BUILD=1` contract, and it is a command segment so it stays reachable from
  `launch.json`, which carries no `env` field.
- `served` is parsed out of `segments` before the variant/vite-arg split so it can never leak into
  `bunx vite`'s argv.
- Dead `SKIP_PLUGIN_BUILD: "1"` writes removed from `collabStartUserDevServer` (line ~2636) and
  `startParityDevServer` (line ~4132) plus their docstrings. Grep for a read site:
  `process.env.SKIP_PLUGIN_BUILD` → **zero** repo-wide (only writes). Both call sites keep spawning
  `dev`, which now means "activate + serve" — which is what they need, since both of them prebuild
  into the **unprofiled** `🧑‍💻dev/🔌️plugin-modules/` tree and would otherwise have no activation
  receipt at all. The third write site, the hub's `🌎️hub/📦️packages/🦀️rust/📜️script.ts:11472` browser-host
  harness, spawns `dev` with `SEMIO_TEST_BROWSER_*` roots it stages itself, so it becomes
  `dev served` — byte-identical behaviour to what the dead flag was meant to express.
  `♻️mit-bestand/…/🧫️pipeline.json` keeps its `SKIP_PLUGIN_BUILD` entry: that list is
  `forbiddenOrchestration`, i.e. a negative assertion that the demonstrator never sets it.
- `SKIP_ENGINE_BUILD` is left exactly as it is (`buildEngineWasm`, line 1050 — wgpu `dev` and
  `BuildScript`). The react path no longer calls `buildEngineWasm`; engine freshness there is Nx's
  (`{surface,editor,flow-core,puzzle-plugin}:wasm` in `prepare`'s `dependsOn`), so the flag has no
  react meaning to honour any more and inventing one would fork the freshness authority.

### 3.2 root `📜️script.ts`

- `runFrameworkOsPlaygroundDev`: stops filtering `served` out of the argument list and stops writing
  the dead `SKIP_PLUGIN_BUILD=1`/react-meaningless `SKIP_ENGINE_BUILD=1` pair; `served` is forwarded
  as a segment and only `SEMIO_RENDERER=react` is forced. Docstring rewritten to describe the
  activate→serve contract instead of the extinct skip-the-plugin-build one.

### 3.2b `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts`

- `devToolingEnv` no longer forces `NX_ISOLATE_PLUGINS ??= "false"`. Since **today 16:26**
  (`6152f9ca6a`, `git log -L 16,16`) the workspace's Nx inference plugin is an **async** ES module — it
  top-level `await import()`s the runtime-component closure under a `?revision=` query — and Nx's
  non-isolated path loads plugins with `require()`, which rejects an async module outright:

  ```
   NX   Failed to load 1 Nx plugin(s):
    - ./🧰️framework/…/📚️library/🟨️.mjs: require() async module … is unsupported. use "await import()" instead.
        at getPluginsSeparated (nx/dist/src/project-graph/plugins/get-plugins.js:200)
        at runPreTasksExecution (nx/dist/src/project-graph/plugins/tasks-execution-hooks.js:13)
  ```

  Forcing isolation off therefore guarantees that failure for any `nx run` that has to load plugins in
  its own process. An explicit caller override still wins (`devToolingEnv({NX_ISOLATE_PLUGINS:"true"})`
  is asserted by the workspace contract and still passes). This did **not** by itself fix the failure
  seen from the root script — see §8.1.

### 3.3 `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`

- The `served` env assertions (lines ~2115-2118) exercised `frameworkOsPlaygroundDevEnv`'s override
  passthrough with two now-removed variables; re-pointed at live ones so the test still proves
  passthrough without documenting dead flags.

### 3.4 `.claude/launch.json`

- `puzzle3d-react` (port 6013, `SEMIO_RENDERER=react`) and `puzzle3d-wgpu` (port 6113,
  `SEMIO_RENDERER=wgpu`), both `bun ./📜️script.ts dev 3d`, in the style of `procedural3d-react`.
  The renderer env is **not** optional here: `frameworkOsPlaygroundDevEnv` defaults
  `SEMIO_RENDERER` to `wgpu`, so an entry without it would bind the wgpu port and ignore the
  configured one (the pre-existing `puzzle2d-react` entry has exactly that defect).

### 3.5 `.vscode/🧩️launch.seed.jsonc` / `.vscode/launch.json`

- **No seed change needed.** This wave adds no new executable command: `🖥️launch.ts:224-247`
  already synthesises `📦️build🧩️<plugin>⚙️{component,materialize}-{dev,release}`,
  `🎮️generate🧩️<variant> session`, `🎮️{prepare,activate,serve,dev}🧩️<variant>⚛️react <profile>`
  from the registry, and the committed `.vscode/launch.json` already carries all ten puzzle3d rows
  (`🎮️generate🧩️puzzle3d session`, `🎮️prepare|activate|serve|dev🧩️puzzle3d⚛️react dev|release`,
  `🎮️build🧩️puzzle3d⚛️react release`). Regeneration is still run below as a gate.

## 4. Verified by running

| # | command | result |
|---|---|---|
| 1 | `bun nx show project @semio-tech/puzzle-plugin --json` | `component-dev/-release`, `materialize-dev/-release` present, exact commands/outputs/cache flags as in §0 |
| 2 | `bun nx show project @semio-tech/framework-os-dev --json` | all ten `*-puzzle3d-react-*` targets present with the dependsOn shown in §0 |
| 3 | `bun -e 'buildPlaygroundSession("puzzle3d")'` | `{variant:"puzzle3d", registryPluginId:"puzzle", plugins:["puzzle"]}` — stdio not required |
| 4-13 | the pipeline itself, the two `DevScript` branches, the tests, the gates | see the run log in §6 |

Facts established while auditing, worth recording:

- **`CARGO_PROFILE_WASM_DEV_DEBUG=false` is redundant now.** `Cargo.toml:464-466` has
  `[profile.dev] debug = false` and `[profile.wasm-dev] inherits = "dev"` (line 473-475), so the
  8.6 GB→165 MB rustc RSS fix from `📓️findings-2026-09-05.md` §14 is committed, not env-dependent.
  Passing the env var anyway is harmless.
- `[profile.wasm-dev.package.semio-s-artifact-puzzle-3d] opt-level = 2` is committed
  (`Cargo.toml:488-489`) — the interactive-ceiling fix needs no env either.
- `native component dev|release --manifest` (`…⚡️caching/🦀️cargo/📜️script.ts:217-231`) is the
  existing component-build helper, and it validates the WASI component header
  (`[0,97,115,109,13,0,1,0]`) and single-file output. No new cargo wrapper was written, per the brief.
- A **valid, stale** puzzle wasip2 component exists in a peer's private target dir:
  `/Users/ueli/.semio-targets/p3d/wasm32-wasip2/wasm-dev/semio_s_plugin_puzzle.wasm`
  (55,226,140 B, 2026-09-07 22:36, component header verified) and a `wasm-release` sibling
  (20,080,251 B, 2026-09-07 23:43). Used only for the §6 dry run; **not** a substitute for the
  coordinator's build.

## 5. What still needs the coordinator's cargo build

One target, one command (everything else in the chain is cached TypeScript or already fresh):

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=/Users/ueli/.semio-targets/waveR \
  bun nx run @semio-tech/puzzle-plugin:component-dev
```

- `CARGO_TARGET_DIR` is honoured straight through (`{env:"CARGO_TARGET_DIR"}` is a declared input of
  the inferred `component-dev` target, and `buildCargoArtifacts` inherits `process.env`); keep it
  **outside** the repo and **lane-private** per the shared-target-wipe incidents.
- No `CARGO_PROFILE_WASM_DEV_DEBUG` / `opt-level` env needed — both are committed (§4).
- Do **not** set `SEMIO_PLUGIN_PROFILE` / `SEMIO_PLUGIN_ONLY`: neither is read by this target, and
  the second is the `plugin.descriptor-unavailable` trap from findings §14.
- `component-dev` is `parallelism: false` and takes the **shared** repo `target/` lock unless
  `CARGO_TARGET_DIR` says otherwise, so run it alone; §6.9 shows what a killed run leaves behind.
- Then the whole rest of the boot is one command:

```bash
bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev
```

  which pulls `materialize-dev`, `support-dev`, `fonts`, the four engine `wasm` targets,
  `session-puzzle3d`, `prepare` and `activate` through `dependsOn` and then serves. Prefer this over
  `SEMIO_RENDERER=react bun ./📜️script.ts dev 3d` until §8.1 is closed. The browser URL is
  `http://127.0.0.1:6013/?plugin=puzzle3d` — the bare root expands the full `s` studio host
  (`browser-boot/🟦️.ts:32`, findings §34.1).
- Before trusting what renders, delete or overwrite the Sep-7-derived caches listed at the end of §7,
  or confirm the served descriptor's `wasmSha256` differs from `9d681a1ada4e…`.

## 6. Run log

Every command below was run in the foreground from `/Users/ueli/Documents/semio` on 2026-09-09
between 00:25 and 01:20, on a box carrying five other sessions' cargo work (load ~40, swap full).
Server logs and captured output live in this session's scratchpad
(`…/9e1e818a…/scratchpad/waveR/`), not in the ticket's `🗑️generated`.

**6.1 `bun nx run @semio-tech/plugin-registry:session-puzzle3d`** — success, 1m02s, 0/3 cache hits.

```
.vscode/launch.json regenerated -> /Users/ueli/Documents/semio/.vscode/launch.json
> bun ./📜️script.ts session puzzle3d
Playground session puzzle3d: 1 plugins staged
 NX   Successfully ran target session-puzzle3d for project @semio-tech/plugin-registry and 2 tasks it depends on
```

Produced `…🔌️plugin/📇️registry/dist/sessions/puzzle3d/{🟦️session.ts (996 B), .nx-artifact.json}`.

**6.2 `bun ./📜️script.ts prepare puzzle3d react dev` (before any component existed)** — failed, and
the failure names exactly one missing input:

```
ENOENT: no such file or directory, open
 '…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/🔣️.json'
   at run (…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1331:37)
```

So session, browser support and fonts were all already satisfied; only `materialize-dev` was missing.
(Nit for a later wave: line 1331 reads the descriptor with a bare `readFileSync`, so a missing
component surfaces as a raw ENOENT instead of `PreparationScript`'s own
`Incomplete prepared component <id>` message one line below.)

**6.3 materialize dry run.** `/Users/ueli/.semio-targets/p3d/wasm32-wasip2/wasm-dev/semio_s_plugin_puzzle.wasm`
(55,226,140 B, 2026-09-07 22:36, header `[0,97,115,109,13,0,1,0]`) copied to
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_puzzle.wasm`, then:

```
$ bun ./📜️script.ts materialize dev --manifest "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml"
Materializing puzzle dev: transpile
Materializing puzzle dev: descriptor
Materialized puzzle dev: browser bridge and descriptor staged
```

**The descriptor emitter did not exhaust fuel** (findings §18's concern did not reproduce): the
60 s-budget `node --experimental-wasm-jspi` probe returned a valid descriptor for a 55 MB component
on a fully loaded box. Output at
`…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/`:

| file | bytes |
|---|---|
| `semio_s_plugin_puzzle_component.core.wasm` | 55,175,507 |
| `semio_s_plugin_puzzle_component.js` | 443,348 |
| `semio_s_plugin_puzzle_component.d.ts` | 3,415 |
| `🌉️bridge.js` | 9,443 |
| `🔣️.json` | 5,388,441 |
| `🛂️.descriptor.semio` | 4,295,998 |
| `🟨️.js` | 6,808 |
| `interfaces/` | 28 `.d.ts` |
| `.nx-artifact.json` | owner `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:browser:dev`, 35 files |

Descriptor identity: `manifest.pluginId = "puzzle"`, `hashes.wasmSha256 = 9d681a1ada4e…`,
`coreWasmSha256 = 742bd12a295c…`, `descriptorSha256 = 683eae0bb28f…`.

**6.4 `prepare` then `activate`** — both succeed:

```
Prepared puzzle3d react dev: 1 components, session, browser support and 17 fonts
Activated puzzle3d react dev: 1 completed components (changed)
```

Receipt written at `…🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/puzzle3d/activation/🔣️receipt.json`:

```json
{"schema":"semio.dev.activation/v1","variant":"puzzle3d","profile":"dev","plugins":[{"pluginId":"puzzle","artifactSha256":"b4fcfec8be3455928cebe0d03f398e552adfae159e2653061657bcb2d3b10445","rebuiltAt":1788906618659}]}
```

**6.5 `bun ./📜️script.ts serve puzzle3d react dev`** — Vite up on the configured port, serving the
**profiled** module tree:

```
VITE v7.3.6  ready in 1524 ms
➜  Local:   http://127.0.0.1:6013/
$ curl "http://127.0.0.1:6013/?plugin=puzzle3d"                       → HTTP 200, 2,626 B  (semio · os shell)
$ curl "http://127.0.0.1:6013/🔌️plugin-modules/🧩️puzzle/🔣️.json"      → HTTP 200, 5,388,441 B
$ curl "http://127.0.0.1:6013/🔌️plugin-modules/🧩️puzzle/🌉️bridge.js"  → HTTP 200, 9,443 B
```

The descriptor byte count (5,388,441) matches the newly materialized **profiled** file, not the
unprofiled `🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/🔣️.json` (5,388,499) — proof that the react server reads
`dist/<profile>/🔌️plugin-modules/`, as §2 assumes. Server stopped afterwards by pid; port free.

One pre-existing defect surfaced in Vite's startup, in another wave's file — it only disables
dependency pre-bundling, so the server still starts:

```
(!) Failed to run dependency scan. Skipping dependency pre-bundling.
  ✘ [ERROR] Syntax error "d"
    ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts:337:5:
      337 │     2d: row["2d"] === undefined ? undefined : puzzlePuzzle5dArtif…
```

`2d:` is not a valid identifier — it needs quoting (`"2d":`). Not touched by this wave.

**6.6 the new `DevScript` react branch, chain path** —
`SEMIO_RENDERER=react bun ./📜️script.ts dev puzzle3d` in the os-dev package:

```
[dev] activating puzzle3d react dev via @semio-tech/framework-os-dev:activate-puzzle3d-react-dev
```

Stopped deliberately 12 s in, before Nx could reach `component-dev` (this wave must not run cargo);
`ps` confirmed no cargo/rustc child had been created.

**6.7 the new `DevScript` react branch, `served` path** —
`SEMIO_RENDERER=react S_OS_PORT=6013 bun ./📜️script.ts dev puzzle3d served`: **no** `[dev] activating`
line, `VITE … ready in 1829 ms`, `➜  Local: http://127.0.0.1:6013/`, `curl /?plugin=puzzle3d` → HTTP 200,
`🌉️bridge.js` → HTTP 200. So `served` skips the chain, and the `served` segment does not leak into
Vite's argv. Stopped by pid.

**6.8 activation contract, language-neutral cases** — `♻️activation/🧫️cases.json` replayed against
`nextActivationReceipt`/`parseActivationReceipt`, validated by **ajv** against
`♻️activation/🧬️schema/🔣️.json` and cross-checked against a **lodash**-built oracle (the same two
third-party oracles the peer's `⚡️cache-contracts` test uses):

```
activation cases: 4 ajv=1 lodash-oracle=1 all pass
live puzzle3d receipt parses: puzzle3d dev puzzle
```

**6.9 accident to declare: two `component-dev` cargo builds were started and killed.** While probing
why the root-script invocation fails (§8.1) I ran
`bun nx run @semio-tech/framework-os-dev:dev -- puzzle3d served` twice from a shell; the bootstrap
remapped both to `serve-puzzle3d-react-dev`, whose graph includes
`@semio-tech/puzzle-plugin:component-dev` — so two `cargo rustc … --target wasm32-wasip2 --profile
wasm-dev` builds started against the **shared** repo `target/`. Killing the wrappers orphaned them
(ppid 1) exactly as `feedback-killing-wrapper-orphans-cargo` predicts. Both were then killed by pid
(25924/25928 and 28027/28031, plus their orphaned `node` nx parents 18978/18449). Neither had
compiled anything: 1.53 s of CPU time, state `SN`, no `rustc` child — they were queued on the
`target/` lock held by a peer's `native component release` build for `note`. Verified afterwards that
the only remaining cargo processes belong to live peer sessions, and that nothing listens on 6013.
**Lesson for the coordinator: `nx run …:dev -- <variant>` and `…:serve-<variant>-react-<profile>` are
NOT dry commands — they pull the cargo component build through `dependsOn`.**

**6.10 workspace-contract tests (`bun test` — the runner this file uses, not vitest)**

```
$ bun test "../../🧪️tests/🔬️workspace-contract/🟦️.ts" -t "frameworkOsPlaygroundDevEnv"
  2 pass, 0 fail, 586 filtered out, 12 expect() calls        [5.95s]
$ bun test "../../🧪️tests/🔬️workspace-contract/🟦️.ts" -t "isolated plugin"
  1 pass, 0 fail, 587 filtered out                          [5.91s]
$ bun test "../../🧪️tests/🔬️workspace-contract/🟦️.ts" -t "Nx Unicode project transport"
  1 pass, 1 fail
  (fail) builds the describe graph without a lossy duplicate repo-test root [37236ms]
         ^ this test timed out after 20000ms
```

The failing case spawns `bun nx show projects --with-target describe` with an explicit
`NX_ISOLATE_PLUGINS: "true"` (untouched by this wave) and asserts exit 0 within a 20 s budget; the
graph took 37 s on this box. Load-induced, not a behaviour change — the same probe run by hand
returns exit 0.

**6.11 script load check** — both edited routers import cleanly under bun
(`os-dev script loads, exports: 12`, `root-ok`).

**6.12 launch generation gates**

```
$ bun ./📜️script.ts check-generated       (@semio-tech/plugin-registry)
plugin registry generated catalog and launch bytes are fresh.
```

`.vscode/launch.json` was regenerated by 6.1's `generate` dependency. Its working-tree diff
(+32/-2) is entirely **peers'** seed work that had not been regenerated yet
(`🧪️test🔎️jack-query-oracle`, `📥️deps wasm optimizer`, two `framework-actor:test` → `test-long`
gate rewrites); no puzzle3d row changed, because all ten were already present. `.claude/launch.json`
re-parses as JSON with 17 configurations including `puzzle3d-react` and `puzzle3d-wgpu`.

**6.13 `bun ./📜️script.ts check` (@semio-tech/plugin-registry)** — **exit 1, pre-existing and
unrelated**: it stops at its first gate with 19,364 lines of
`plugin taxonomy tree violations (area(s) "✏️s/🔌️plugins" is "clean")` across `✒️writer`,
`➗️mathematical`, `🌀️procedural`, `🏗️fem`, `📕️norm`, `🪵️sourcing`, … (`… is not reachable from Cargo
manifest`, `missing 🧬️schema/`, `missing 📜️.wit`). It never reaches the launch-freshness gate, which
`check-generated` covers instead (6.12). No puzzle3d/launch/component finding appears anywhere in
that output.

## 7. Files changed

| file | change |
|---|---|
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | new `activatePlaygroundRuntime`; `DevScript` react branch runs the activation chain then serves; `served` segment parsed and honoured; two dead `SKIP_PLUGIN_BUILD` writes + docstrings removed |
| `/Users/ueli/Documents/semio/📜️script.ts` | `runFrameworkOsPlaygroundDev` forwards `served` as a segment, no longer writes the dead/meaningless skip pair; docstring rewritten |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts` | `devToolingEnv` no longer forces `NX_ISOLATE_PLUGINS=false` (async-ESM Nx plugin); docstring records why |
| `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts` | browser-host harness spawns `dev served` instead of `dev` + dead `SKIP_PLUGIN_BUILD` |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` | `served` env passthrough assertions re-pointed at live variables |
| `/Users/ueli/Documents/semio/.claude/launch.json` | `puzzle3d-react` (6013, react) and `puzzle3d-wgpu` (6113, wgpu) |
| `/Users/ueli/Documents/semio/.vscode/launch.json` | regenerated (registry `generate`); content is peers' pending seed work |
| `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-09-wave-R-react-boot-pipeline.md` | this note |

No `📋️project.json` was touched: every target in the chain is inferred (§0). No new command was
added, so `.vscode/🧩️launch.seed.jsonc` needed no edit (its working-tree modification is a peer's).

**Build artifacts left on disk, with provenance** (all are dev-profile caches the coordinator's real
build overwrites; recorded so nobody mistakes them for fresh):

- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_puzzle.wasm` — **copied by
  hand** from a 2026-09-07 22:36 peer build, no `.nx-artifact.json`, so Nx owns nothing here and
  `component-dev` will replace it on its next (input-hash miss) run.
- `…🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/` and
  `…🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/dev/puzzle3d/` — derived from those Sep-7 bytes by the
  script commands (not through Nx), so no cache entry claims them either. Anything served from them
  is **Sep-7 puzzle code**; the digests in 6.3/6.4 are how to tell.
- `…📇️registry/dist/sessions/puzzle3d/` — genuinely fresh (Nx-produced, 6.1).

## 8. Open questions

**8.1 `bun ./📜️script.ts dev 3d` still fails at Nx plugin loading, and I could not isolate why.**
Four runs (two before and two after the `devToolingEnv` change, one of them with the sandbox
disabled) all died the same way:

```
 NX   Failed to load 1 Nx plugin(s):
  - ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs: require() async module … is unsupported.
error: bun nx run @semio-tech/framework-os-dev:dev -- puzzle3d served exited with status 1
      at runFrameworkOsPlaygroundDev (📜️script.ts:257)
```

while the identical command from a shell — same cwd, and same env reconstructed from
`frameworkOsPlaygroundDevEnv` — reaches `Running target serve-puzzle3d-react-dev … and 19 tasks`
(§1.1). Ruled out by experiment: the env itself (`nx run` of `session-puzzle3d` spawned from a bun
child **with** `frameworkOsPlaygroundDevEnv`'s env succeeds, `plugin-load-failure: no`), `bun nx`
resolution (the bootstrap script's `$ bun ./…🚀️bootstrap/📜️script.ts nx …` echo does appear from a
spawned bun child), and the sandbox. What is left is something about `runCmd`'s `spawnSync`
(`🏃️process/🟦️.ts:111`) — stdio/detach/budget — versus a plain shell spawn. The `NX_ISOLATE_PLUGINS`
removal is kept regardless: forcing isolation off is unambiguously wrong against an async-ESM plugin.
**Until this is closed, boot puzzle3d through the Nx target rather than the root alias:**
`bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev` (or the launch.json row
`🎮️dev🧩️puzzle3d⚛️react dev`), which is the same chain. A cheaper alternative worth trying first is
making `runFrameworkOsPlaygroundDev` invoke `⚡️caching/🚀️bootstrap/📜️script.ts nx run …` explicitly
instead of relying on `bun nx` name resolution.

**8.2 `served` no longer means "do not build" on the Nx route.** Through the bootstrap it maps to
`serve-<variant>-react-<profile>`, which `dependsOn` activation, so it builds everything and only
skips the `watch` (that is how 6.9 happened). Only the direct script command is truly serve-only. If
the intent is "serve what is staged", the bootstrap should route `served` to the plain `dev` target
with the `served` segment (i.e. `{args: ["run", "@semio-tech/framework-os-dev:dev", "--", plugin,
"served"]}`) — but `serve-*`'s `dependsOn` is pinned by
`⚡️cache-contracts/🟦️.ts:616-621`, so that is a contract decision for whoever owns that test, not a
drive-by fix.

**8.3 `collabPrebuildPlugins` is now redundant work.** Collab and parity prebuild the **unprofiled**
`🧑‍💻dev/🔌️plugin-modules/` tree with `buildPlugins`, then (after this wave) spawn `dev`, which builds
the **profiled** tree again through `materialize-<profile>`. Both trees come from the same crates, so
this is two full component passes per collab run. The clean end state is for those harnesses to drop
`buildPlugins` and rely on the Nx chain; out of scope here because it changes what the collab e2e
gate builds.

**8.4 `PreparationScript`'s missing-component diagnostic.** See 6.2 — a bare `readFileSync` on the
descriptor pre-empts the intended `Incomplete prepared component <id>` error.

**8.5 puzzle5d schema has an unquoted `2d:` key** (6.5) which breaks Vite's dependency scan for every
react playground, not just puzzle3d.

## 9. Change list

Superseded by §7 (files) and §5 (the one command still outstanding).
