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
`dev-<variant>-react-<profile>` target is the one wired to the whole chain, and nothing routes to it.

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
  receipt at all.
- `SKIP_ENGINE_BUILD` is left exactly as it is (`buildEngineWasm`, line 1050 — wgpu `dev` and
  `BuildScript`). The react path no longer calls `buildEngineWasm`; engine freshness there is Nx's
  (`{surface,editor,flow-core,puzzle-plugin}:wasm` in `prepare`'s `dependsOn`), so the flag has no
  react meaning to honour any more and inventing one would fork the freshness authority.

### 3.2 root `📜️script.ts`

- `runFrameworkOsPlaygroundDev`: stops filtering `served` out of the argument list and stops writing
  the dead `SKIP_PLUGIN_BUILD=1`/react-meaningless `SKIP_ENGINE_BUILD=1` pair; `served` is forwarded
  as a segment and only `SEMIO_RENDERER=react` is forced. Docstring rewritten to describe the
  activate→serve contract instead of the extinct skip-the-plugin-build one.

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
| 4 | `bun nx run @semio-tech/plugin-registry:session-puzzle3d` | see §6 |
| 5 | `bun ./📜️script.ts prepare puzzle3d react dev` (before the component exists) | see §6 |
| 6 | materialize dry run from the Sep-7 component | see §6 |
| 7 | `bun ./📜️script.ts activate puzzle3d react dev` + receipt | see §6 |
| 8 | `ServeScript` on 6013 + `curl /` | see §6 |
| 9 | registry `generate` + `check` | see §6 |

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
- Then the whole rest of the boot is:

```bash
SEMIO_RENDERER=react bun ./📜️script.ts dev 3d
# equivalently, straight at the Nx graph:
bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev
```

  and the browser URL is `http://127.0.0.1:6013/?plugin=puzzle3d` (the bare root expands the full
  `s` studio host — `browser-boot/🟦️.ts:32`, findings §34.1).

## 6. Run log

Filled in by the implementation pass below.

## 7. Files changed

See §9.

## 8. Open questions

Filled in below.

## 9. Change list

Filled in below.
