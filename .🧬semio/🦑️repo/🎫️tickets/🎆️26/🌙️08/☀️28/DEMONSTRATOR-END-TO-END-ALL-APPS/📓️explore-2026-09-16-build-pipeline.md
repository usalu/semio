# Explore — Demonstrator build pipeline (2026-09-16)

Read-only research. A live build was running concurrently while this was written
(`bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev`, pid 68618, started 10:08,
apparently the "Fable coordinator" session per `📓️status-2026-09-16.md`) — some facts below are
snapshots that will already be stale by the time this is read; timestamps/process states are as
observed at ~10:09-10:20 on 2026-09-16.

## 1. Where `prepare-<variant>-react-<profile>` / `activate-<variant>-react-<profile>` come from

Not declared in any `📋️project.json`. They are Nx-inferred by the workspace plugin registered in
`nx.json:34-39` (`"plugin": "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"`), whose
`createNodesV2` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1189`) walks every
`📋️project.json`/`Cargo.toml`, and — only for the project literally named
`@semio-tech/framework-os-dev` — merges in `playgroundPreparationTargets(...)`
(`🟨️.mjs:1052`, function body `🟨️.mjs:903-970`).

`playgroundPreparationTargets` scans every `Cargo.toml` whose
`package.metadata.component.package` + `package.metadata.semio.role ∈ {plugin, extension}` is set
(`🟨️.mjs:906-916`), collects each crate's `[[package.metadata.semio.playground]]` rows
(confirmed on disk: `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:33-79` declares 7
playground rows — `demonstrator`, `generator`, `koordinator`, `aggregator`, `aussuchen`,
`bearbeiten`, `verfolgen`, all under pluginId `demonstrator`), then for every
`(playground, profile ∈ {dev,release})` pair emits:

- `prepare-<variant>-native-<profile>` / `run|smoke-<variant>-native-<profile>` (wgpu native path — irrelevant to the demonstrator, `🟨️.mjs:936-948`)
- `serve|dev-<variant>-react-<profile>` — `dependsOn: [activate-<variant>-react-<profile>]`, runs `bun ./📜️script.ts serve <variant> react <profile>` (`🟨️.mjs:949`)
- **`activate-<variant>-react-<profile>`** (`🟨️.mjs:950-956`): `cache:true, parallelism:false, outputs:["{projectRoot}/dist/runtime/react/<profile>/<variant>"]`, `dependsOn:["prepare-<variant>-react-<profile>"]`, `options.command: "bun ./📜️script.ts activate <variant> react <profile>"` (cwd = `🧑‍💻dev/📦️packages/🟦️typescript`, i.e. `framework-os-dev`'s project root)
- **`prepare-<variant>-react-<profile>`** (`🟨️.mjs:958-964`): `outputs:[]`, `dependsOn: ["@semio-tech/plugin-registry:session-<variant>", "@semio-tech/framework-plugin-web:support-<profile>", "semio-framework-os-infinite:fonts", ...engines, ...materialize-<profile> for every component id in the runtime closure]`, `options.command: "bun ./📜️script.ts prepare <variant> react <profile>"`
- (wgpu equivalents, `🟨️.mjs:965-982`, irrelevant here)
- `build-<variant>-react-release` (release distribution artifact, `🟨️.mjs:984-993`)

The **runtime closure** for each variant is computed at `🟨️.mjs:919`:
`runtimeComponentClosure([...components].map(...dependsOn: extends+depends-on...), [playground.pluginId])`
— i.e. the plugin crate's own declared `extends`/`depends-on` metadata, walked transitively.

**Cargo/wasm build, down at the bottom of the chain:**
- `component-<profile>` (`componentTargets`, `🟨️.mjs:851-859`): per plugin/extension crate, `cache:true, parallelism:false, outputs:["{projectRoot}/dist/component-<profile>"]`, runs `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts native component <profile> --manifest <Cargo.toml>` — **this is the actual `cargo`/wasm-component build**.
- `materialize-<profile>` (`🟨️.mjs:860-869`): `dependsOn:["component-<profile>", "@semio-tech/framework-plugin-web:support-<profile>", ...(release: "workspace:deps-wasm-opt")]`, outputs `🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/🔌️plugin-modules/<moduleDirectory>`, runs `bun 🔌️plugin/📦️packages/🟦️typescript/📜️script.ts materialize <profile> --manifest <Cargo.toml>`. `moduleDirectory` is resolved from the **hand-authored** (not generated) catalog `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json` (`🟨️.mjs:855-857`); for `pluginId: "demonstrator"` that catalog line 13 gives `directoryName: "🎪️demonstrator"`.
- **Registry refresh**: `session-<variant>` (`playgroundSessionTargets`, `🟨️.mjs:885-901`, in project `@semio-tech/plugin-registry`) — `dependsOn:["generate"]`, outputs `{projectRoot}/dist/sessions/<variant>`, runs `bun ./📜️script.ts session <variant>`. `generate` itself is declared literally in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json:15-29` (`command: "bun ./📜️script.ts generate"`, `dependsOn:["repo:generator-inputs"]`, **no explicit `outputs` key** — relies on Nx's inferred/default output tracking) and rewrites everything under `🔌️plugin/📇️registry/🤖️generated/` including `🎮️playgrounds/🟦️.ts` (`PLAYGROUND_BUILD_TARGETS`, generator = `GenerateScript` in `📇️registry/📽️projection/🟦️.ts:339-354`).

### What `prepare`/`activate` in `🧑‍💻dev/♻️activation/` do (from a parallel deep-read, `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:24-27`)

- **`prepare <variant> react <profile>`** → `🧰️preparation/🟦️.ts:70-88` — **pure verification, writes nothing**: imports the generated session (`registry/dist/sessions/<variant>/🎮️playground-session/🟦️.ts`), checks `session.variant`/`registryPluginId`; for every plugin in the session's closure, checks the staged module's `🔣️.json` manifest, `🌉️bridge.js` and `.nx-artifact.json` exist under `pluginModulesRoot(profile)`; checks browser-support files (preview2 vendor `.nx-artifact.json`, shard worker) and the font asset. Throws on anything missing.
- **`activate <variant> react <profile>`** → `🏃️execution/🟦️.ts:77-111` — **writes state**: re-loads the session, computes `runtime = developmentRuntimeRoot(root, variant, profile, "react")` = `dist/runtime/react/<profile>/<variant>` (`♻️activation/🟦️.ts:65-69`), hashes support files + each plugin's staged module (`activationFilesDigest`, `📥️installation/🟦️.ts:39-47`) into a per-plugin `artifactSha256`, computes `nextActivationReceipt(...)` (`♻️activation/🟦️.ts:50-57` — keeps `rebuiltAt` unchanged for unchanged plugins, i.e. warm reuse), installs every **extension**-role plugin into `runtime/extensions/<moduleDirectoryName>/` via `publishActivatedExtension` (`📥️installation/🟦️.ts:50-74`, atomic via a temp dir + `stageArtifacts`), then **atomically writes the activation receipt** `runtime/activation/🔣️receipt.json` via `publishActivationReceipt` (`♻️activation/🟦️.ts:92-106`, temp-file + rename, short-circuits if byte-identical).
- Receipt schema (`♻️activation/🟦️.ts:38-47`, `ACTIVATION_RECEIPT_FILE="🔣️receipt.json"`): `{schema:"semio.dev.activation/v1", variant, profile, plugins:[{pluginId, artifactSha256 (sha256 hex), rebuiltAt (epoch ms)}]}`.
- `♻️activation/🟦️.ts:71-79` doc comment: this staging root is deliberately singular — cites a past 2-day drift incident (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`) where a second staging path diverged from `materialize-*`'s real output.
- `🔍️freshness/🟦️.ts` (read-only reporting, never blocks) and `🔐️lease/🟦️.ts` (PID-based build lock, file `target/semio-dev-leases/plugin-build-<variant>.json`, self-heals a stale lock by checking `isPidAlive(existing.pid)`) exist but are **not** in the `prepare`/`activate` call path for the Nx-driven flow above — they belong to an older/alternate direct-dev-CLI orchestration path (see §4 re `pipeline.json`'s `forbiddenOrchestration`). No lease directory currently exists on disk (`target/semio-dev-leases` absent) — no stale locks.
- `🌐️browser-host/🟦️.ts` is unrelated to this pipeline — it's schema/validation for a hardcoded-`variant:"s"` native test-browser fixture, not the demonstrator's Vite serve.

## 2. What the demonstrator needs on disk, and which target produces it

| Artifact | Path pattern | Produced by |
|---|---|---|
| Cargo/wasm component | `<crate>/dist/component-<profile>/` | `component-<profile>` (per plugin/extension crate) |
| Staged browser module + `.nx-artifact.json` (`files` incl. `🌉️bridge.js`, `🔣️.json`) | `🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/🔌️plugin-modules/<moduleDirectoryName>/` | `materialize-<profile>` |
| Preview2 vendor + shard worker support | same root, `🪞️vendor/`, `🧵️shard/`, both with `.nx-artifact.json` | `@semio-tech/framework-plugin-web:support-<profile>` |
| Fonts | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts/{.nx-artifact.json,🔤️guestslim-typst-fonts.bin}` | `semio-framework-os-infinite:fonts` (depends on `semio-framework-os-font-assets:build`) |
| Registry generated catalog (`PLAYGROUND_BUILD_TARGETS`, `🎠️playgrounds.json`, etc.) | `🔌️plugin/📇️registry/🤖️generated/**` | `@semio-tech/plugin-registry:generate` |
| Playground session dist (`PLAYGROUND_SESSION`) | `🔌️plugin/📇️registry/dist/sessions/<variant>/🎮️playground-session/🟦️.ts` | `@semio-tech/plugin-registry:session-<variant>` (×7: aggregator, aussuchen, bearbeiten, generation3d, generator, koordinator, verfolgen) |
| Engine wasm (gis/verfolgen only) | `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/dist/**` | that project's own `wasm` target (named in `PLAYGROUND_BUILD_TARGETS[...].engines`) |
| Activation receipt | `🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/<profile>/<variant>/activation/🔣️receipt.json` | `activate-<variant>-react-<profile>` (×7, but the demonstrator only directly requires the **`generator`** one — see below) |
| Extensions installed into runtime | `.../runtime/react/<profile>/<variant>/extensions/<moduleDirectoryName>/` | same `activate-*` target, via `publishActivatedExtension` |
| Demonstrator's own prepare/activate verification (no output) | n/a | `@semio-tech/mit-bestand-demonstrator:prepare-dev` / `:activate-dev` |
| Served app | `http://127.0.0.1:6029/` | `@semio-tech/mit-bestand-demonstrator:dev` → `:serve` (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts` `ServeScript`, logs `Demonstrator ready: <url>`) |

**Important resolved subtlety**: the demonstrator's `📌️important` gate (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/♻️activation/🟦️.ts:5-9`, `readDemonstratorActivation`) only ever reads **one** activation receipt — `developmentRuntimeRoot(..., "generator", "dev", "react")` — and requires its `plugins` id-set to equal `demonstratorRuntimeComponentIds()` (`📦️assets/🟦️.ts:9-12`), which is the **union closure** across every pane's plugin (`demonstrator` [koordinator/aggregator/aussuchen/bearbeiten/generator/verfolgen all share this one pluginId] ∪ `procedural` [generation3d] ∪ both their transitive deps). So the other 6 `activate-<variant>-react-dev` Nx targets still have to run (each is a real dependency edge from `mit-bestand-demonstrator:prepare-dev`, and `prepare-<variant>-react-dev` gates on `materialize-dev` for the whole per-variant closure), but the demonstrator's runtime script itself only ever *reads* the `generator` one. Observed session sizes tonight: `generation3d`: 11 plugins staged; `generator`/`aussuchen`/`bearbeiten`/`aggregator`/`koordinator`/`verfolgen`: 21 plugins staged each (from the live run's log, see §5).

## 3. Current state vs. git (snapshot ~10:09-10:20, 2026-09-16)

**A live build is in progress right now** (pid 68618: `bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev`, started 10:08; log `.🧬semio/…/🗑️generated/activate-dev-2026-09-16-run2.txt`). At the moment this was captured, it had progressed through: registry `generate` (cache hit), all 7 `session-*` targets (cache hit), font-assets build + `infinite:fonts` (fresh, wrote 8,757,072 bytes), and was mid-way through `@semio-tech/cad-plugin:component-dev` (a `cargo` build, ~30s elapsed and counting) — one of several component builds needed before `materialize-dev` for pluginId `demonstrator` can run.

| Artifact | State at snapshot | Note |
|---|---|---|
| `🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🎪️demonstrator/` | **absent** | not yet materialized; `dist/dev/🔌️plugin-modules/` dir mtime 10:06:58 |
| `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/dist/` | only `cargo-artifacts-LMcG0a` scratch dir, **no `component-dev/`** | contrast: `✏️s/🔌️plugins/📐️cad/…/dist/` already has `component-dev/` |
| `🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/{generator,koordinator,aggregator,aussuchen,bearbeiten,verfolgen}/` | **absent**; only `cad, fem2d, fem3d, generation3d, process3d, puzzle3d, sourcing` present | `generation3d` already activated separately (own receipt has 11 plugins, none named `demonstrator`) |
| `🔌️plugin/📇️registry/dist/sessions/{aggregator,aussuchen,bearbeiten,generation3d,generator,koordinator,verfolgen}/` | **all present** | fresh per tonight's cache-hit log |
| `♾️infinite/…/dist/fonts/{.nx-artifact.json,🔤️guestslim-typst-fonts.bin}` | present, just rebuilt this run | |
| nx daemon / workspace-data | active, being written continuously (`.nx/workspace-data/d/daemon.log` last line at the time of the check) | |
| `git log -1 --date=iso -- ✏️s/🔌️plugins/🎪️demonstrator` | `6f33e313da 2026-09-15 23:17:05 +0200` | crate itself last touched yesterday evening; nothing to compare it to in the current build-output tree since that tree doesn't exist yet |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/` | git-ignored (`.gitignore:634` `**/📦️packages/*/dist/`) | staleness can only be judged by mtime vs. source mtime, not git — this is exactly what `🔍️freshness/🟦️.ts`'s `collectStagedModuleFacts` does at runtime |
| `http://127.0.0.1:6029/` | connection refused (curl exit 7 / code 000) | expected — server not started yet, build still running |

**Conclusion for Q3**: as of this snapshot, everything upstream of the `demonstrator` pluginId's own `component-dev`/`materialize-dev` is fresh/cached; the missing pieces are precisely the `demonstrator` crate's own component build (in progress, queued behind other component builds in the closure) → its `materialize-dev` → the 6 `activate-<variant>-react-dev` receipts that depend on it → the demonstrator's own `prepare-dev`/`activate-dev` → `serve`.

## 4. Env vars

- **`SEMIO_BUILD_BUDGET_MS` / `SEMIO_CMD_BUDGET_MS`** — live and broadly used (e.g. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts:15-23`, dozens of call sites) — general cargo/command timeout budgets, still effective for any step in this pipeline.
- **`RUSTFLAGS` / `CARGO_TERM_QUIET`** — standard cargo env, effective on every `component-<profile>` cargo invocation.
- **`CARGO_PROFILE_WASM_DEV_DEBUG`** — not referenced anywhere in current `.ts`/`.mjs`/`.toml` sources (grepped repo-wide excluding `node_modules`/`dist`); the debug-off trick from `project-wasm-dev-profile-debug-off-and-swap-thrash` memory isn't wired into this pipeline's own code, but as a genuine Cargo env var it would still apply if set (Cargo reads it directly, no repo code needs to reference it).
- **`SEMIO_PLUGIN_ONLY`** — read at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts:91-94`, but that file belongs to a **different, non-Nx orchestration path** (`buildPlugins`-style, the one the demonstrator's `pipeline.json` explicitly forbids — see below); not part of the `component-<profile>`/`materialize-<profile>`/`activate-*` chain used by `bun nx run @semio-tech/mit-bestand-demonstrator:dev`.
- **`SKIP_PLUGIN_BUILD` / `SKIP_ENGINE_BUILD` — DEAD for this pipeline.** Repo-wide grep for `process.env.SKIP_PLUGIN_BUILD` / `process.env.SKIP_ENGINE_BUILD` (excluding old ticket scratch scripts) finds **zero read sites** in current `.ts`/`.mjs`. They are only ever *set* (not read) by `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/🦀️.rs:296-300` and `⌨️cli/🦀️.rs:312-316` (native CLI/dashboard tools building an env for spawning some other legacy dev entrypoint), and by old per-ticket verify scripts. Direct confirmation: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️pipeline.json:22-27` declares
  ```
  "forbiddenCommandImports": ["🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"],
  "forbiddenOrchestration": ["buildPlugins", "buildEngineWasm", "ensurePluginRegistry", "SKIP_PLUGIN_BUILD", "spawnDaemon"]
  ```
  i.e. the demonstrator's own runtime script is *contractually forbidden* from doing the kind of self-orchestrated build that `SKIP_PLUGIN_BUILD`/`SKIP_ENGINE_BUILD` used to gate; it now only ever verifies/consumes outputs the Nx graph already produced. **`.claude/launch.json`'s `mit-bestand-demonstrator-fast` (SKIP_PLUGIN_BUILD=1, line ~189) and `mit-bestand-demonstrator-noengine` (SKIP_PLUGIN_BUILD=1 + SKIP_ENGINE_BUILD=1, line ~231) are stale relative to the current Nx-inferred pipeline** — those two env vars will be silently ignored; only `SEMIO_BUILD_BUDGET_MS`/`SEMIO_CMD_BUDGET_MS`/`RUSTFLAGS`/`CARGO_TERM_QUIET` in those entries still do anything.
- `.claude/launch.json` `mit-bestand-demonstrator` (no env overrides, plain `bun nx run …:dev`, port 6029) is the one config that matches the current pipeline as-is.

## 5. Minimal command sequence

Given the current state (§3), the plain, no-flag entry is already correct — the stale env vars in `-fast`/`-noengine` don't hurt (they're just ignored), they just don't help either. Straightforward sequence:

```bash
cd /Users/ueli/Documents/semio

# (a) produce every missing artifact — this alone pulls in every upstream target
#     (font-assets, registry generate/sessions, cad/procedural/… component+materialize
#      builds, demonstrator component+materialize, 7x activate-*-react-dev) via the Nx graph:
bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev

# (b) same command also satisfies "activate" for the demonstrator itself (activate-dev
#     depends on prepare-dev + activate-generator-react-dev, and IS the activation step)

# (c) serve on :6029 (separate continuous target; activate-dev's outputs are cached so this is fast)
bun nx run @semio-tech/mit-bestand-demonstrator:dev &

# readiness check
until [ "$(curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:6029/)" = "200" ]; do sleep 2; done
```

Timing hints from tonight's live run: registry `generate`+`session-*` = cache hits, near-instant;
font-assets rebuild ≈ a few seconds once Xcode-license blocker is worked around; each plugin
`component-<profile>` cargo build observed running ≥30s and still going (cad-plugin) — with
`SEMIO_BUILD_BUDGET_MS` default 3,600,000ms (1h) per budgeted step and no override active. Expect
the full chain (cad + demonstrator + whatever else sits in the shared closure, serialized by
`parallelism:false` on `component-*`/`activate-*` but limited by `nx.json`'s `"parallel": 3`) to
take low-single-digit minutes on a mostly-warm cache, more if cold or the machine is loaded (load
~10 observed, per a peer's own status note).

**Live blocker encountered by the concurrent run** (from `📓️status-2026-09-16.md`, 10:40 entry):
Xcode was silently upgraded to 27.0 with an unaccepted license, which makes every native `cc`
link fail (`exit status 69`, and breaks `python3` too). Real fix needs `sudo xcodebuild -license
accept` from the user. Per-process workaround with no system change:
`DEVELOPER_DIR=/Library/Developer/CommandLineTools bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev`
— confirmed working in the live run's log (font-assets build succeeded on run2 after this override).

## 6. Nx cache / cargo build-dir / daemon contention

- **Nx cache**: `nx.json:60` → `.🧬semio/🦑️repo/⚡️cache/nx` (`maxCacheSize: "16GB"`).
- **Cargo build/target dirs**: `.cargo/config.toml:10-11` → `target-dir = ".🧬semio/🦑️repo/⚡️cache/cargo/target"`, `build-dir = ".🧬semio/🦑️repo/⚡️cache/cargo/build"` — one shared build-dir for the whole workspace/every agent/dev, with `[unstable] fine-grain-locking = true` (`.cargo/config.toml:16`) so concurrent `cargo` invocations lock per compilation unit, not per directory: they **share** already-built units and only block on units another invocation is actively compiling. This matches the standing memory note and was directly observed: the peer's `nx watch … activate-sourcing-react-dev` (pid 62593, started 09:57, still running) and the live `activate-dev` run (pid 68618) are compiling different crates concurrently without apparent lock contention in the log (cad-plugin's cargo build proceeding normally).
- **Nx daemon**: `useDaemonProcess: true` (`nx.json:6`) — a single shared daemon process was found running: pid 62582 (`node … nx/dist/src/daemon/server/start.js`, socket `/tmp/.nx/501/sockets/d1a8bc728cf19feffd0f/d.sock`, `.nx/workspace-data/d/server-process.json`), up 10+ minutes, actively logging `HASH_TASKS` and file-watch events (`daemon.log`). **Every** `nx run`/`nx watch` invocation in this workspace (mine if I ran one, the peer's watch at pid 62593, and the live coordinator's `activate-dev` at pid 68618) talks to this **same** daemon for project-graph/task-hash coordination — it is a single process, so graph computation/hashing requests are serialized through it, but that's typically sub-second per request (seen: `Handled json message HASH_TASKS. Handling time: 409`ms for a large graph). Actual task **execution** (the cargo/bun child processes) run independently outside the daemon, so real build work parallelizes; the only shared contention point is (1) the daemon's single-threaded coordination and (2) the fine-grain-locked cargo build-dir described above — neither looked like an actual bottleneck at this snapshot. No stale lock files were found (`target/semio-dev-leases` doesn't exist).

## Key file citations

- Nx target inference: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:903-993` (`playgroundPreparationTargets`), `:851-883` (`componentTargets`), `:885-901` (`playgroundSessionTargets`), `:1052` (wiring into `framework-os-dev`'s project.json)
- Demonstrator project: `♻️mit-bestand/🧺️demonstrator/📋️project.json`
- Demonstrator runtime script: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts` (`PreparationScript` L15-30, `ActivationScript` L33-38, `ServeScript` L41-50)
- Demonstrator runtime layout/closure: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🟦️.ts`, `♻️activation/🟦️.ts`, `📦️assets/🟦️.ts`
- Pane catalog: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json`
- Pipeline contract (forbidden legacy orchestration): `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️pipeline.json`
- Generic dev activation: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`, `/🧰️preparation/🟦️.ts`, `/🏃️execution/🟦️.ts`, `/📥️installation/🟦️.ts`, `/🔍️freshness/🟦️.ts`, `/🔐️lease/🟦️.ts`
- Generated registry targets: `PLAYGROUND_BUILD_TARGETS` in `🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts`; generator in `📇️registry/📽️projection/🟦️.ts:339-354`; `generate`/`session-*` targets in `📇️registry/📋️project.json`
- Deployment catalog (hand-authored): `🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`
- Env-var/orchestration: `.claude/launch.json` (`mit-bestand-demonstrator*` entries), `🔌️plugin/🏗️build/📋️plan/🟦️.ts:91-94` (`SEMIO_PLUGIN_ONLY`), rust env-setters `🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/🦀️.rs:296-300`, `⌨️cli/🦀️.rs:312-316`
- Cache/build dirs: `nx.json:60`, `.cargo/config.toml:7-19`
