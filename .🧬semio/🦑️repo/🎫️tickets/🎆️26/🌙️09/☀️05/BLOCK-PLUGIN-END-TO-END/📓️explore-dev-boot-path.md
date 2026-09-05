# Explore: `bun run dev:block:2d`/`3d`/`5d` boot path

Scope: read-only static exploration. No cargo/dev server run. Timestamps below are `ls -la` mtimes read at
exploration time (repo date 2026-09-05); other sessions are actively mutating the tree (git status shows
1513 changed paths repo-wide at time of writing), so treat file-presence facts as a snapshot, not a
guarantee.

## 1. Exact chain, env vars, engines, ports, default renderer

**Command chain** (all citations are file:line):

1. `package.json:88-90` — `dev:block:2d`/`:3d`/`:5d` → `bun ./📜️script.ts dev block 2d` (etc).
2. Root `📜️script.ts:474-514` `DevScript.run(["block","2d"])` → `resolvePlaygroundDevApp` (`📜️script.ts:177-181`)
   → `resolveFrameworkOsPlaygroundPlugin` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2740-2750`)
   matches alias `"block 2d"` against the generated catalog and returns `{ app: "block2d", rest: [] }`.
3. `runFrameworkOsPlaygroundDev("block2d", [])` (`📜️script.ts:194-202`) spawns
   `bun nx run @semio-tech/framework-os-dev:dev -- block2d` with env from `frameworkOsPlaygroundDevEnv`
   (`🟦️.ts:2753-2763`): `SEMIO_PLUGIN=block2d`, **`SEMIO_RENDERER = env.SEMIO_RENDERER ?? "wgpu"`**,
   `S_OS_PORT = env.S_OS_PORT || <catalog default port for that renderer>`.
   **This is the important default-renderer finding**: a bare `bun run dev:block:2d` with no
   `SEMIO_RENDERER` set in the shell boots **wgpu** (native `trunk serve`), not the react shell. The
   `served` rest-segment (or an explicit `SEMIO_RENDERER=react`) is what selects react
   (`📜️script.ts:183-201`'s own doc comment says this explicitly: "a bare `dev s` builds all 59 crates
   and then hands off to `trunk serve`").
4. nx `project.json` for `@semio-tech/framework-os-dev` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:12-23`)
   runs `bun ./📜️script.ts dev` (inner script, same cwd) with `forwardAllArgs: true`, so segment
   `block2d` reaches the INNER `DevScript` in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1827-1976`.
5. Inner `DevScript.run`: `variantSegment = "block2d"`, `renderer = process.env.SEMIO_RENDERER ?? "react"`
   (`:1851-1854`) — but by this point `SEMIO_RENDERER` was already set to `"wgpu"` by step 3's env, so this
   inner default of `"react"` never actually applies for the plain `bun run dev:block:2d` path.
   - **react path** (`SEMIO_RENDERER=react` explicitly set, e.g. via the `served` segment or manually):
     registry regenerate → engine wasm build → `runViteBunxDev` (`:1953-1966`) serves Vite at
     `http://127.0.0.1:6024/` (react port for `block2d`, see below), plugin crates stream in afterward
     (`buildPluginsStreaming`, `:1967-1973`).
   - **wgpu path** (default): builds `trunk` native dev server at `http://127.0.0.1:6124/…` (wgpu port for
     `block2d`) via `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts serve`
     (`:1933-1947`); `buildEngineWasm` is called but is a no-op for this renderer (see below).

**Ports** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json:97-151`
(also declared at source in `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml:24-38`):

| variant | app id | react port | wgpu port |
|---|---|---|---|
| block2d | `s.block.block2d@1/*#editor` | 6024 | 6124 |
| block3d | `s.block.block3d@1/*#editor` | 6025 | 6125 |
| block5d | `s.block.block5d@1/*#editor` | 6026 | 6126 |

`BLOCK_2D_PLAY_PORT` **does not exist anywhere in the repo** (checked with a repo-wide grep over
`*.ts`/`*.rs`/`*.toml`) — the only port override is the generic `S_OS_PORT` env var
(`🟦️.ts:2756,2760`; inner script `:1868,1907,1940,1954`).

**Env vars**:

- `SEMIO_PLUGIN_ONLY` — only affects which plugin **crates get cargo-built**
  (`resolvePluginBuildTargets`, dev-package `📜️script.ts:1130-1146`); it narrows the already
  registry-filtered target list further to `pluginId === only`. It does **not** change which registry
  entries the browser session tries to *load* (see §3). Throws if it matches zero crates (`:1136-1138`).
- `SKIP_PLUGIN_BUILD` — only meaningful combined with `SEMIO_RENDERER=react`+the `served` rest-segment
  (root `📜️script.ts:183-201`); skips the whole `buildPluginsStreaming`/`buildPlugins` step and serves
  whatever is already in `🔌️plugin-modules/`.
- `SKIP_ENGINE_BUILD` — read once, inside `buildEngineWasm` (dev-package `📜️script.ts:1554`):
  `if (renderer !== "react" || process.env.SKIP_ENGINE_BUILD === "1") return;` — i.e. it is **only ever
  relevant for the react renderer**; for wgpu the function is already a no-op regardless of this var.
- `SEMIO_BUILD_BUDGET_MS` — overrides `BUILD_BUDGET_MS` (default **1,200,000 ms = 20 min**,
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1241-1246`), the hard
  ceiling `runCmdStatus`/`runCmd` apply to every `cargo`/wasm-pack invocation in the build chain
  (registry generate, plugin cargo build, engine wasm build). A build queued behind another session's
  cargo target-dir lock for >20 min is SIGKILL'd with an ETIMEDOUT-style failure (this is exactly what
  killed boot attempt #1 in the sibling procedural ticket). The dev-SERVER process itself
  (`runFrameworkOsPlaygroundDev`'s outer spawn, and the inner `runViteBunxDev`) runs under
  `daemonBudgetOpts()`/no budget, i.e. 24h, so only the build sub-steps are budget-bound, not the
  long-lived Vite/trunk server.
- `CARGO_TARGET_DIR` — read at dev-package `📜️script.ts:969` (`resolve(repoRoot, process.env.CARGO_TARGET_DIR)`
  when set, else `<repoRoot>/target`) for locating the freshly-built plugin `.wasm` artifact after
  `cargo rustc`. Not otherwise threaded into the `cargo` invocation's env by this script's plugin-build
  path (`buildPluginCargo`, `:963-974`, spawns `cargo` with the current `process.env` unmodified) — so
  setting `CARGO_TARGET_DIR` in the shell before invoking `bun run dev:block:2d` does redirect cargo's
  own output, and this script picks the artifact up from the same override, but it is not something the
  script sets FOR you to get isolation; you must export it yourself (as the sibling procedural ticket did
  with `target-gen3d`).
- `BLOCK_2D_PLAY_PORT` — not a real variable (see above); use `S_OS_PORT`.

**Engines (react-only) and their current on-disk state** (checked at exploration time, all "pkg"-style
output directories are actually named `🕸️bindings` for two of the three, and bare package dir for the
third — not literally `pkg/`):

| engine | build script | output dir | present? | mtime |
|---|---|---|---|---|
| `framework_surface` (node-graph) | `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts` | `.../🕸️bindings/` | yes | 2026-09-04 21:25 |
| `framework_editor` | `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts` | `<crate>/` (wasm-pack `--out-dir .`) | yes | 2026-09-04 21:25 |
| `flow-core` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/` | yes | 2026-09-04 21:55 |

All three currently exist and are only a few hours stale, so `SKIP_ENGINE_BUILD=1` on a react boot is
currently a valid shortcut (unlike the state the sibling procedural ticket found on 09-03, where flow-core's
`pkg` was missing). This is volatile — other sessions rebuild/clean these — re-check `ls -la` before relying
on it. `buildEngineWasm` (dev-package `📜️script.ts:1546-1570`) builds these three UNCONDITIONALLY for every
react session regardless of which variant/app is active, plus whatever the catalog row's `engines` array
declares (`block2d`/`block3d`/`block5d` all declare `engines: []` in `🤖️generated/🎠️playgrounds.json`, so
block needs no extra engine beyond the universal three).

**Default renderer**: **wgpu**, not react, for the bare `bun run dev:block:2d` invocation — see step 3
above. The task description's premise that this path lands on the react `ShellHost` is only true once
`SEMIO_RENDERER=react` is exported or the `served` alias is used.

## 2. Plugin-module descriptor discovery and the missing `🔣️.json`

**Discovery mechanism** (react renderer): `loadPluginModule`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1061-1062`)
calls `fetchDescriptorManifest(pluginId, moduleUrl, signal)`
(`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:108-132`), which does an HTTP `GET` for
`<dirname(moduleUrl)>/🔣️.json` (`:110-111`) BEFORE any actor runtime starts. On non-2xx, HTML content-type,
invalid JSON, missing `manifest.pluginId`/`apps`, or an id mismatch, it throws a `SemioFaultError` with code
`plugin.descriptor-unavailable` / `plugin.descriptor-invalid` / `plugin.descriptor-identity-mismatch`
(`:118,119,124,128,129`). `loadPluginModuleResilient`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1403-1415`)
catches this, logs `[DEBUG] program load failed`, and returns `null` for that plugin — it does not abort the
whole boot by itself, but see the cascade below.

The served `🔣️.json` is staged per-crate by `stagePluginDescriptor`
(dev-package `📜️script.ts:820-833`): it copies `<ownerRoot>/🔣️.json` into
`🔌️plugin-modules/<pluginId>/🔣️.json`, where `ownerRoot = join(root, target.cratePath, "..", "..")`
(block's `cratePath` is `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust`, so `ownerRoot` =
`✏️s/🔌️plugins/🧱️block`). If the owner-root file doesn't exist, it **removes** any stale copy from the
output dir and returns `false` (`:823-826`) — leaving the served module dir with no descriptor at all.
`ownerRoot/🔣️.json` is written by `describeBuiltPlugin` (dev-package `📜️script.ts:948-960`) as part of
`materializePlugin` (`:981-1005`), which runs once per crate right after that crate's own `cargo`
artifact is transpiled (`:994-996`, and note `:996` **throws** `Missing fresh descriptor for <id>` if
`stagePluginDescriptor` returns false right after a build — so a genuinely fresh, successful build of a
crate always leaves it with a valid descriptor).

**Current on-disk state** (checked, not assumed):

- `✏️s/🔌️plugins/🧱️block/🔣️.json` — **absent**, and `git log` shows it was **never committed**.
- `✏️s/🔌️plugins/🗄️stdio/🔣️.json` — **absent**, also never committed.
- 29 other plugins DO have a committed owner-root `🔣️.json` (e.g. `💠️lowpoly`, `🌀️procedural`, `📋️forms`,
  …; full list gathered via `find ✏️s/🔌️plugins -maxdepth 1 -iname 🔣️.json`); `block` and `stdio` are not
  in that list.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🧱️block/` and `.../🗄️stdio/` both currently
  contain a stale `.core.wasm`/`.js`/`.d.ts` from a prior build (block: 2026-08-18/09-04; stdio:
  2026-08-18/09-04) but **no `🔣️.json`** in either output dir right now.

Net effect: booting `dev block 2d` in react mode **right now, without a fresh full build of both `block`
and `stdio`**, will 404 on `GET /🔌️plugin-modules/block/🔣️.json` (and the same for stdio) — this is exactly
the failure class the sibling procedural ticket hit for `stdio` ("stdio has no committed `🔣️.json`"). It is
not gitignored (`git check-ignore` confirms no match) — it's simply never been generated+committed for
`block`/`stdio` at all, in this tree's history. A normal full build (no `SEMIO_PLUGIN_ONLY` narrowing) is
self-healing: `materializePlugin` runs `describeBuiltPlugin`+`stagePluginDescriptor` per crate and hard-fails
loudly (`:996`) rather than silently shipping a stale/missing descriptor, so a green build of both crates
leaves both descriptors correct on disk (uncommitted, but present for subsequent dev-session reuse until
someone cleans `🔌️plugin-modules/`).

Does block have a committed `🔣️.json`? **No.** What breaks at boot if it's still missing: `loadPluginModule`
for `block` (and independently for `stdio`) throws `plugin.descriptor-unavailable`; per §3's dependency
cascade, this can also take down loading of anything the react shell tries to load that (transitively)
depends on the failed plugin.

## 3. Plugins loaded alongside block; does `SEMIO_PLUGIN_ONLY=block` isolate the boot?

`ensurePluginRegistry(filterPlugin)` (dev-package `📜️script.ts:1121-1128`) calls
`generatePluginRegistry(repoRoot, { filterPlaygroundPlugin: "block" })` (via `resolveCatalogFilterPluginId`,
`:132-134`, which resolves the `"block2d"` variant to crate pluginId `"block"`). `generatePluginRegistry`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:624-635`) with a
`filterPlaygroundPlugin` narrows to `resolveRegistryPluginIdsForFilter("block")`'s id set
(`:579-606`): the target plugin itself, plus every plugin whose `contributes` intersects the target's
`consumes` (topic-based), plus the FULL transitive `dependsOn` closure.

- `block`'s Cargo metadata (`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`) declares no
  `[package.metadata.semio] consumes = […]`, so no topic-based additions.
- `block`'s only plugin-crate Cargo dependency is `semio-s-plugin-stdio`
  (`Cargo.toml:44` `semio-s-plugin-stdio = { path = "../../../🗄️stdio/📦️packages/🦀️rust", … }`), which
  `parseCargoPluginDependencyIds` turns into `dependsOn: ["stdio"]`
  (registry `📜️script.ts:283-285`).
- `stdio`'s own Cargo.toml has **no** `semio-s-plugin-*` path dependency, so the closure stops there.

**Conclusion**: for `dev block 2d`, exactly **two** crates are in the session's registry: `block` and
`stdio` — no `flow-extension-*`/sourcing/cad-extension plugins get pulled in (those appear for `procedural`
because procedural directly depends on `stdio` too but ALSO consumes topics/extends other plugins; block
does neither beyond the `stdio` edge).

**`SEMIO_PLUGIN_ONLY=block` does NOT isolate the runtime boot** — it only narrows
`resolvePluginBuildTargets` (`:1130-1146`), i.e. which crates get **cargo-built** this run. The *registry*
entries served to the browser are still `{block, stdio}` (decided earlier, by `filterPlugin`, independent of
`SEMIO_PLUGIN_ONLY`). If `stdio` is excluded from the build by `SEMIO_PLUGIN_ONLY=block` and does not
already have a valid build+`🔣️.json` on disk (true right now, per §2), then:
1. `stdio`'s `loadPluginModule` fails (`plugin.descriptor-unavailable`).
2. `loadPluginModulesInDependencyOrder`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1968-2010`)
   treats any entry whose declared dependency already failed as **also failed**
   (`:1986-1992`, `blockedDependency` check) and pushes a `loadFailures` row for it without even attempting
   the load — so **`block` itself would be skipped too**, since it depends on `stdio`.

So `SEMIO_PLUGIN_ONLY=block` is only safe to use for FAST ITERATION after at least one full,
un-narrowed build of `{block, stdio}` has already produced valid descriptors for both on disk; used cold
(as in "first ever boot in this tree"), it risks cascading both plugins to fail to load, mirroring the
sibling ticket's `stdio` failure exactly but for a different reason (there it was "stdio wasn't the crate
being rebuilt"; here it would be "stdio was explicitly excluded from the crate build").

## 4. App id resolution: `s.block.block2d@1/*#editor`

The canonical id format is `surface_app_id(dialect, role) = format!("{}#{}", dialect.to_coordinate(), role.as_str())`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3402-3405`), where `ArtifactDialect::to_coordinate`
= `format!("{}@{}/{}", artifact_kind, standard, subset)`
(`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:81-86`).

`block` declares exactly the dialect that reproduces the catalog's app id:
`BLOCK2D_DIALECT: Dialect = Dialect { artifact_kind: "s.block.block2d", standard: StandardId("1"),
subset: SubsetId::ANY }` (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🦀️.rs:78`; `SubsetId::ANY = SubsetId("*")`
per `🚪️io/🧬️schema/🦀️.rs:41`) → coordinate `"s.block.block2d@1/*"`, and role `editor` → the exact string
`"s.block.block2d@1/*#editor"` from the catalog row (`Cargo.toml:25` / `🎠️playgrounds.json:101`).

Registration: the crate's top-level app enum `BlockApps` has a
`Block2dEditor(VcsArtifactApp<EditorApp<Block2dPlayApp>>)` variant
(`✏️s/🔌️plugins/🧱️block/🦀️.rs:16`), and the plugin root wires it via `.declare_artifact(block2d::artifact())`
(`🦀️.rs:209`) + `.editor_mutation_roster::<Block2dPlayApp>()` (`🦀️.rs:212`) +
`.activation(ActivationEvent::OnArtifactKind { kind: block2d::artifact_kind().id })` (`🦀️.rs:218`), where
`artifact_kind().id = "2d.block"` and `artifact()` declares kind `ArtifactKindId::parse("s.block.block2d")`
(`◻️2d/🦀️.rs:82-97,169-173`). So **yes**, the crate registers apps with exactly the catalog's ids — the same
pattern repeats for `block3d`/`block5d` (`🦀️.rs:18,20` variants, `🦀️.rs:210-211,214,216` wiring), matching
`Cargo.toml:31,37`'s `s.block.block3d@1/*#editor` / `s.block.block5d@1/*#editor`.

Caveat: `git status` shows uncommitted, in-progress edits (+304/-13 lines) to
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (plus smaller edits
to the sibling `👁️viewer/🦀️.rs` and `🧬️schema/🦀️.rs`) at exploration time — this is the block2d editor
module itself, so a concurrent session is actively changing exactly the code this section describes; the
citations above are to the current on-disk (uncommitted) content, not to `HEAD`.

## 5. Recipe for booting block2d fast, concretely

Adapting the procedural ticket's converged recipe (`.../PROCEDURAL-3D-END-TO-END/📓️status.md`,
"Self-inflicted stall found and cleared" / boot attempts 6-11) to block:

1. **First boot of the session must be a full, un-narrowed build** (no `SEMIO_PLUGIN_ONLY`) so `block` AND
   `stdio` both get fresh `🔣️.json` descriptors (see §2/§3) — do not reach for `SEMIO_PLUGIN_ONLY=block` on
   a cold tree.
2. Isolate the cargo target dir to avoid contending with other sessions' shared `target/debug/.cargo-lock`:
   `export CARGO_TARGET_DIR=target-block` (own value, matching the procedural ticket's `target-gen3d`
   pattern) — remember this is a plain env var this script reads but does not set for you (§1).
3. Pick renderer explicitly — react, to reach the `ShellHost` code path the ticket cares about:
   `export SEMIO_RENDERER=react`.
4. Since the three universal engines (`framework_surface`, `framework_editor`, `flow-core`) are currently
   fresh on disk (§1 table), `SKIP_ENGINE_BUILD=1` is a valid shortcut right now — re-check mtimes before
   trusting this on a later run; if any is stale/missing, drop the skip for that first boot.
5. Raise the build budget so a shared-lock queue doesn't SIGKILL the boot at the 20-minute default:
   `export SEMIO_BUILD_BUDGET_MS=3600000` (1h, matching the procedural ticket's own escalation).
6. Run: `CARGO_TARGET_DIR=target-block SEMIO_RENDERER=react SEMIO_BUILD_BUDGET_MS=3600000
   SKIP_ENGINE_BUILD=1 bun ./📜️script.ts dev block 2d` from the repo root (root `📜️script.ts`, not the
   dev-package one — the root script is the one wired to `package.json`'s `dev:block:2d`).
7. Registry `generate` runs first (`ensurePluginRegistry`, dev-package `📜️script.ts:1121-1128`, invoking
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate`) — this is a hard gate
   (§6 below); if it fails on a taxonomy/contract validity error, that is an unrelated concurrent-session
   break, not a block-specific problem, and must clear before Vite/trunk ever starts.
8. Vite comes up first (react streams `block`/`stdio` crate builds in afterward); poll readiness at
   `http://127.0.0.1:6024/` (the react port for `block2d`, from §1's table / `🎠️playgrounds.json:105-108`) —
   the same "poll a real HTTP port, not a log file" approach the procedural ticket converged on after
   `🗑️generated/` got swept mid-run.
9. Once `{block, stdio}` are confirmed built with valid descriptors (first successful boot), later
   iterations MAY add `SEMIO_PLUGIN_ONLY=block` to skip re-building `stdio`'s crate — but only as long as
   `🔌️plugin-modules/🗄️stdio/🔣️.json` still exists on disk (i.e. nobody cleaned plugin-modules since).

## 6. What could block boot before block's own code even runs

- **Registry `generate` gate**: `ensurePluginRegistry` runs `bun 📇️registry/📜️script.ts generate` before
  anything else and throws if it exits non-zero (`📜️script.ts:1123`). This script reads
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` for path-emoji/contract validation
  (the same file whose live edits caused the procedural ticket's boot attempts 6/7 to die on
  `pathEmojiPolicy.reservedSubtreeDirectoryNames must be a unique array` and a missing
  `root-pytest-config`/`root-eslint-config` contract). **That file is currently mid-edit**: `git status`
  shows it `M` (staged) with an mtime of 2026-09-05 03:52 — very recent relative to this exploration. It
  parses as valid JSON right now, but its structural validity against the registry generator's specific
  invariants was not (and per this task's read-only/no-generate constraint, could not be) checked without
  invoking the generator, which would write into `🤖️generated/`.
- **`stdio` is mid-refactor**: `git status --porcelain` on `✏️s/🔌️plugins/🗄️stdio/` shows a large set of
  staged renames/adds/deletes/modifies (registry fixtures, `🦀️.rs`, `Cargo.toml`, BREP-related test files)
  — the same in-flight work the sibling procedural ticket tracked as semio-ac's
  `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`. Since `block` depends on `stdio` directly, any compile
  break in `stdio` blocks `block`'s own crate build too (`cargo tree -i` relationship, not verified here
  since running cargo was out of scope) — this was the exact gating mechanism that stalled the procedural
  ticket for hours.
- **`block` itself has uncommitted WIP**: 3 modified files under
  `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/` (`✏️editor/🦀️.rs` +304/-13,
  `👁️viewer/🦀️.rs` +6, `🧬️schema/🦀️.rs` +7) from a concurrent session — not verified to compile (no cargo
  run per instructions); if broken, `block`'s own crate build fails independent of anything else in this
  report.
- **Repo-wide churn**: `git status --porcelain --untracked-files=no` currently reports **1513** changed
  paths — consistent with multiple concurrent sessions; the sibling ticket's "repo-wide path-corruption
  event" (an external Codex session rewriting emoji paths across ~110+ manifests) is a documented example of
  the kind of surprise this volume of concurrent activity can produce. No such corruption was observed in
  the specific files this report cites, but it was not exhaustively re-checked repo-wide.
- **Shared `target/debug/.cargo-lock` contention**: not something to fix, but budget for it — see §5's
  `CARGO_TARGET_DIR` isolation and `SEMIO_BUILD_BUDGET_MS` raise, both directly modeled on how the
  procedural ticket survived it.
