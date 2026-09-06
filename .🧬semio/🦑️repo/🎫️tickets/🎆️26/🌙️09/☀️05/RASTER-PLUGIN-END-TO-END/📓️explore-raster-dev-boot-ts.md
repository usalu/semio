# Explore: raster plugin dev-boot path and TypeScript side

Scope: read-only static exploration (no cargo/bun build/dev server run). Repo date at exploration time:
2026-09-06 (session started 2026-09-05 evening). `git status --porcelain --untracked-files=no` = **137**
changed paths repo-wide at time of writing (much calmer than the sibling BLOCK ticket's 1513 at its
start) — other sessions are still concurrently mutating the tree, treat file-presence/hash facts below as
a snapshot. This report reuses the boot-mechanics facts already nailed down by
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md` (same
machinery, different crate) and only re-derives what differs for raster; all raster-specific facts below
were independently re-verified against the current tree, not assumed from that report.

## 1. Exact chain, env vars, ports, engines, plugin load closure

**Command chain** (file:line):

1. `package.json:104` — `"dev:raster": "bun ./📜️script.ts dev raster"`.
2. Root `📜️script.ts:476-514` `DevScript.run(["raster"])` → no `"storybook"`/`"s"`/`"multi"`/`"mcp"` match
   → `resolvePlaygroundDevApp(["raster"])` (`📜️script.ts:177-181`) →
   `resolveFrameworkOsPlaygroundPlugin` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2477-2487`)
   matches catalog row `variant === "raster"` (raster declares no `aliases`, so only the literal token
   works) and returns `{ app: "raster", rest: [] }`.
3. `runFrameworkOsPlaygroundDev("raster", [])` (`📜️script.ts:196-203`) spawns
   `bun nx run @semio-tech/framework-os-dev:dev -- raster` with env from `frameworkOsPlaygroundDevEnv`
   (`🟦️.ts:2489-2500`): `SEMIO_PLUGIN=raster`, **`SEMIO_RENDERER = env.SEMIO_RENDERER ?? "wgpu"`**,
   `S_OS_PORT = env.S_OS_PORT || <catalog default port for that renderer>`.
   **Default renderer is wgpu**, exactly like block/procedural — a bare `bun run dev:raster` with no
   `SEMIO_RENDERER` in the shell boots the native `trunk serve` wgpu path, not the react `ShellHost`. The
   `served` rest-segment or an explicit `SEMIO_RENDERER=react` is what selects react.
4. `@semio-tech/framework-os-dev`'s `project.json` `dev` target
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:12-23`) runs
   `bun ./📜️script.ts dev` (inner script, same cwd), `forwardAllArgs: true`, so segment `raster` reaches
   the inner `DevScript.run` in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1214-1352`.
5. Inner `DevScript`: `variantSegment = "raster"`, `renderer = process.env.SEMIO_RENDERER ?? "react"`
   (`:1240-1241`) — but by this point `SEMIO_RENDERER` is already `"wgpu"` from step 3 unless the caller
   exported it, so this inner default of `"react"` is moot for the plain `dev:raster` invocation.
   - **wgpu path** (default, `:1288-1319`): probes `S_OS_PORT`/default port for a live trunk server, else
     spawns `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts serve`
     with `SEMIO_PLUGIN=raster`, `SEMIO_RENDERER=wgpu`, `S_OS_PORT=<port>` (`:1305-1313`); logs
     `[dev] wgpu trunk serving at http://127.0.0.1:<port>/?plugin=raster` on success.
   - **react path** (`SEMIO_RENDERER=react` set): `ensurePluginRegistry`/`buildPlugins` +
     `buildEngineWasm("raster","react")` run, then `runViteBunxDev` serves Vite at the react port with
     `SEMIO_PLUGIN=raster`/`VITE_SEMIO_PLUGIN=raster` etc. (`:1345-1352`); plugin crates stream in after
     Vite is already listening when a build lease is held (`streamPluginBuilds`, `:1247`).

**Ports** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json`
raster row (also declared at source in
`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:16-19`, `[[package.metadata.semio.playground]]`):

| variant | react port | wgpu port |
|---|---|---|
| raster | 6060 | 6160 |

`RASTER_PLAY_PORT` (the var `.vscode/launch.json` sets for the two `dev:raster` launch entries, see §7
below) **does not exist anywhere in the live codebase** — a repo-wide grep over `*.ts`/`*.rs`/`*.json`
finds it only as a `{PORT}`-templated placeholder inside a stale scratch file from an unrelated older
ticket (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/LAUNCH-JSON-GENERATOR-FROM-PLAYGROUND-REGISTRY/scratch/extracted-dev-launchers.json:625,634`).
It is dead/vestigial, exactly the same pattern the BLOCK ticket found for `BLOCK_2D_PLAY_PORT` — the only
port override that actually does anything is the generic **`S_OS_PORT`** (`🟦️.ts:2493,2498`; inner script
`:1240,1268,1305-1313,1345`). Setting `RASTER_PLAY_PORT` in `.vscode/launch.json` has zero effect on which
port the server actually binds; only `serverReadyAction`'s hard-coded regex pattern (which does contain the
right literal port numbers, 6060/6160) makes VS Code open the right URL despite the dead env var.

**Env vars** (identical mechanics to the BLOCK ticket's findings, re-verified against the current script
line numbers — line numbers can drift a little session to session since the tree is live, but the logic
is unchanged):

- `SEMIO_PLUGIN_ONLY` — narrows `resolvePluginBuildTargets` (dev-package `📜️script.ts:517-524`) to
  crates whose `pluginId === only`; throws if it matches zero crates. Does **not** change which registry
  entries the browser session tries to load — that's decided by `filterPlugin`/`resolveCatalogFilterPluginId`
  independent of this var (see §3 below for raster's own closure).
- `SKIP_PLUGIN_BUILD` — meaningful with `SEMIO_RENDERER=react` (root `📜️script.ts:183-201`'s `served`
  path, or set directly): skips `buildPlugins`/`buildPluginsStreaming` entirely and serves whatever
  `🔌️plugin-modules/` already holds.
- `SKIP_ENGINE_BUILD` — read once inside `buildEngineWasm`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:941`):
  `if (renderer !== "react" || process.env.SKIP_ENGINE_BUILD === "1") return;` — only relevant for the
  react renderer; for wgpu the function is already a no-op.
- `CARGO_TARGET_DIR` — read at dev-package `📜️script.ts:356` (`resolve(repoRoot, process.env.CARGO_TARGET_DIR)`,
  else `<repoRoot>/target`) to locate the plugin's freshly built `.wasm`; not otherwise threaded into the
  actual `cargo` child's env by this script's plugin-build path — export it yourself for isolation from
  peers' shared `target/`.
- `SEMIO_BUILD_BUDGET_MS` — overrides the default 20-minute (`1_200_000`ms) budget every
  `cargo`/wasm-pack sub-step runs under (`buildBudgetMs()`,
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`); the long-lived
  dev server itself runs under `daemonBudgetOpts()` (24h), only the build sub-steps are budget-bound.
- `CARGO_PROFILE_WASM_DEV_DEBUG` — not read literally anywhere in the TS scripts; it is the standard Cargo
  env-var override for `[profile.wasm-dev].debug`. Root `Cargo.toml:251-253` currently reads:
  ```
  [profile.dev]
  debug = false
  incremental = true
  ...
  [profile.wasm-dev]
  inherits = "dev"
  codegen-units = 1
  ```
  i.e. `debug` is **already `false`** for `wasm-dev` (inherited from `[profile.dev]`), and this is
  committed (no uncommitted diff to `Cargo.toml`; last touch `b0dfa0f09b`, 2026-09-05 20:55). Per project
  memory (`project-wasm-dev-profile-debug-off-and-swap-thrash`), a stale `debug=true` wasm-dev profile
  previously drove a plugin's wasm rustc to 8.6 GB RSS and stalled the host under swap; that specific
  regression is **not currently present** in this tree's root `Cargo.toml`, but exporting
  `CARGO_PROFILE_WASM_DEV_DEBUG=false` explicitly before any raster wasm build is still cheap insurance
  against a future/local override.
- `RASTER_PLAY_PORT` — dead, see ports table above; use `S_OS_PORT`.

**Engines and raster's own declared engine.** raster's Cargo.toml declares one extra engine:
```
[[package.metadata.semio.playground]]
variant = "raster"
ports = { react = 6060, wgpu = 6160 }
engines = ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"]
```
(`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:16-19`; mirrored in the generated
`🎠️playgrounds.json`'s raster row). `buildEngineWasm` (dev-package `📜️script.ts:933-953`) **always**
builds three universal engines first — `framework_surface`/node-graph
(`🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts`, `:946-947`), `framework_editor`
(`:948-949`), `flow-core` (`:950-951`) — and this graph script path is **byte-identical** to raster's own
declared `engines` entry. `buildEngineWasm`'s catalog-row loop (`:952-954`) even explicitly skips
re-building `flow-core` variants to avoid double work, and the `Set(...)` dedup means raster's declared
engine collapses into the already-mandatory `framework_surface` build. **Net effect: raster needs no
engine build beyond the universal three, and its Cargo.toml `engines` declaration is redundant with (not
additive to) `framework_surface`.** This is confirmed end-to-end by the React `Paint2dHost`'s
`createRasterSession()` (§4 below), which loads exactly `@semio-tech/framework-surface-rs` and constructs
`module.RasterSession()` — the same wasm build `buildEngineWasm` always produces first.

At exploration time the `framework_surface` bindings on disk
(`🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface_bg.wasm`, mtime
2026-09-05 13:37-13:38) are **very fresh** — a `SKIP_ENGINE_BUILD=1` react boot is a safe shortcut right
now (re-check mtimes before relying on this later; other sessions rebuild these).

**Plugin load closure (dependsOn).** raster's generated registry entry
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json:1113-1136`):
```json
{
  "pluginId": "raster", "packageId": "semio:raster",
  "cratePath": "✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust",
  "capabilities": ["documents.write"], "contributes": [], "consumes": [],
  "dependsOn": ["stdio"],
  "activationEvents": ["on-artifact-kind:2d.raster"], "executionMode": "isolated"
}
```
matches raster's Cargo dependency on `semio-s-plugin-stdio`
(`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:24`). No `consumes`/topic-based additions declared.
`stdio` itself has no further `semio-s-plugin-*` path dependency, so the closure is exactly **`{raster,
stdio}`** — same two-crate shape the BLOCK ticket found for block. Same warning applies:
`SEMIO_PLUGIN_ONLY=raster` only narrows which crates get **cargo-built**
(`resolvePluginBuildTargets`, `:517-524`), not which registry entries the browser tries to **load**
(decided earlier by `filterPlugin`); if `stdio` lacks a valid on-disk build+descriptor, a narrowed build
risks cascading both `raster` and `stdio` to `plugin.descriptor-unavailable` on first boot (same mechanism
as the BLOCK ticket's §3). Right now `git status --porcelain` on `✏️s/🔌️plugins/🗄️stdio/` is **clean** (no
in-flight edits, unlike the BLOCK ticket's snapshot), so this is lower-risk today than it was for block.

## 2. Owner-root descriptor: raster HAS one, and it is currently STALE relative to source

Unlike `block`/`stdio` (which the BLOCK ticket found never had a committed owner-root `🔣️.json`), raster
**does** have both:
- `✏️s/🔌️plugins/🖨️raster/🔣️.json` (committed; last touched at commit `21fbcd3538` 2026-09-02 12:19:02)
- `✏️s/🔌️plugins/🖨️raster/🛂️.descriptor.semio` (binary format, 134 lines/~variable bytes)

The manifest's two apps and four windowKinds (`registryExampleCatalog`/descriptor read directly):
```
app s.raster.raster@1/*#editor windowKinds: raster-composite (paint-2d), raster-navigator (paint-2d)
app s.raster.raster@1/*#viewer windowKinds: framework.window.image (canvas-2d), raster-view-navigator (canvas-2d)
```

**Verified hash mismatch — the currently served plugin-module wasm is stale:**
- `✏️s/🔌️plugins/🖨️raster/🦀️.rs` (plugin root) was last modified at commit `21fbcd3538` (2026-09-02), but
  `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml` was touched more recently at `b0dfa0f09b`
  (2026-09-05 20:55:43) and the whole raster directory's newest touch is 2026-09-05 22:02:04 — i.e. raster
  source changed as recently as ~2h before this exploration.
- The dev-served build at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🖨️raster/semio_s_plugin_raster_component.core.wasm`
  has mtime **2026-08-17 18:40** (its `.js` wrapper was refreshed 2026-09-04 17:16, its
  `🛂️.descriptor.semio` refreshed 2026-09-05 23:37, its `🔣️.json` refreshed 2026-09-04 11:17 — three
  different staleness generations layered on the same served directory).
- `sha256(served .core.wasm) = 5f6620dcfa04014a990c3b41d4f2dc62c24c6689e9b1dd71d0497b095e9ac55e`, but the
  registry's own recorded hash for raster
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json:1129-1131`
  `coreWasmSha256`) is `9040c81c6daee99c3d31b9eac685c68ea24d551ac7f33f31cad68fe75487e4e6` — **these do not
  match.** The served artifact is a stale leftover, not what the current registry generation believes is
  live.

**Actionable conclusion**: booting `dev raster` right now with `SKIP_PLUGIN_BUILD=1`/a follower-lease
serve-only path would serve genuinely stale raster behavior (whatever the crate looked like on 2026-08-17)
against a registry/descriptor that has moved on. The first boot of a work session on raster **must** be a
full, un-narrowed build (no `SKIP_PLUGIN_BUILD`, no cold `SEMIO_PLUGIN_ONLY=raster`) so
`materializePlugin`/`describeBuiltPlugin`/`stagePluginDescriptor` (dev-package `📜️script.ts`, same
functions the BLOCK ticket cited) regenerate a matching wasm+descriptor trio. There is also a dedicated fix
path independent of the dev server: `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📜️script.ts:16-19`'s
`DescribeScript` (`bun nx run @semio-tech/raster-plugin:describe`, wired via
`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📋️project.json:40-47`) rebuilds the `wasm32-wasip2` component
and re-emits `🛂️.descriptor.semio`+`🔣️.json` at the owner root directly — this is the same command
`registry:check`'s descriptor-gate warning tells a developer to run.

## 3. Registry example catalog vs. descriptor manifest — reconciled, not a bug

The generated `🎠️playgrounds.json` raster row lists `"examples": ["🎬️demo", "🎬️demo-session"]`, while the
descriptor manifest's `examples` field reads `[]` (empty) and the subset-root examples directory
(`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`) contains only
one child, `🎬️demo/`. This looked at first like the same "orphan demo-session example" defect the BLOCK
ticket found for block2d/block3d — **it is not**. `registryExampleCatalog`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:8799-8825`) scans example
directories at **three** levels: the artifact root, each subset root, AND each subset's per-surface-role
directory (`for (const role of taxonomy.surfaceRoles) { … subsets/<subset>/<surfaceDirNames[role]>/…examples… }`,
`:8819-8822`). Raster's second example, `🎬️demo-session`, lives at the **editor-surface** level:
`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/`
(confirmed on disk: `🦀️.rs`, `🟦️.ts`, `🧪️tests/🦀️.rs`+`🟦️.ts`, `🖼️assets/🎮️.cmd.semio`) — a location this
scan function legitimately walks, so both examples are real, on-disk, and correctly discovered. The
descriptor manifest's `examples: []` is simply a different (and likely stale, dated 2026-09-02) snapshot
field that predates whichever commit added `demo-session`; it is not consulted by
`registryExampleCatalog`, only by whatever reads the descriptor directly (raster's own §7 note below on
"ShellHost reads `PluginManifest.examples`, not the registry" — the BLOCK ticket's coordinator fixed this
exact reconciliation gap in the discovery function itself on 2026-09-05 13:30, see its `📓️status.md`).
Given that fix already landed upstream in `registryExampleCatalog`, whether the **descriptor's own**
`examples: []` field is separately stale/consulted by the shell's example switcher is worth re-verifying
once raster is rebuilt (§2's stale-wasm finding means the descriptor should be regenerated anyway).

Raster's TS test target actually exercises both example dirs directly — see §4.

## 4. TS package `📦️packages/🟦️typescript/` — package.json is a near-verbatim CAD copy; runtime code is real

**`package.json` is copy-pasted from `cad-js` and was never edited beyond the name/repository fields.**
Side-by-side (`✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/package.json` vs.
`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json`):

- `name`: `@semio-tech/raster-js` (correctly renamed) vs `@semio-tech/cad-js`.
- `description`: **verbatim CAD text**, unedited — `"📐️ CAD plugin TS: spatial factory runtime/model
  graph (core), R3F renderer, brepjs kernel, construct query language, XState machine adapter, and the
  runtime composition root — folded from the 6 former cad-js-* packages."` — none of this is true for
  raster (no spatial factory, no R3F, no brepjs, no XState, no "6 former packages").
- `scripts`: `"test": "bun nx run @semio-tech/cad-js:test"`, `"generate": "bun nx run @semio-tech/cad-js:generate"`,
  `"fixture": "bun nx run @semio-tech/cad-js:fixture"` — **all three literally invoke CAD's nx project**,
  not raster's own `@semio-tech/raster-js`. A bare `bun run test`/`generate`/`fixture` inside this package
  directory would build/test **cad**, not raster.
- `dependencies`: includes `@semio-tech/cad-js-module-spatial-shape`, `@semio-tech/cad-js-module-aec-building`,
  `@semio-tech/cad-js-module-aec-building-energy`, `@semio-tech/cad-js-module-aec-building-structure` —
  four CAD-specific extension packages raster has no relationship to at all.
- `repository.directory` is the one field that WAS correctly updated to
  `"✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript"`.

**This is dead metadata, not a live break**, because nx does not read `package.json`'s `scripts` field for
`bun nx run <project>:<target>` — it reads `📋️project.json`'s `targets`, and raster's own
`📋️project.json` (`✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📋️project.json:1-14`) correctly wires
exactly one target, `test`, to `bun ./📜️script.ts test` in raster's own directory. So `bun nx run
@semio-tech/raster-js:test` genuinely works; the wrong `package.json` only misleads a human reading it
directly, or a raw `bun run <script>` invoked without going through nx. Recommend fixing description/
scripts/dependencies regardless, since it is confusing and would mislead anyone auditing raster's real
dependency surface.

**`🟦️.ts` (12 lines) is a real, correct facade** — re-exports raster's own schema/snapshot/diff/mutations/
io modules under `raster_*` names from `../../🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/…`; all
11 referenced paths were checked and exist on disk (no dangling import).

**`📜️script.ts` (12 lines) is real** — its `TestScript` (`run()`) invokes Node's/Bun's built-in test
runner directly against two concrete test files:
```
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts
```
Both exist and were read: `demo-session`'s test asserts its `🎮️.cmd.semio` asset is non-trivially long
(>8 bytes); this is thin but genuinely wired, not a path bug. **What `bun nx run @semio-tech/raster-js:test`
would execute**: exactly these two test files via `node --test`/bun test through
`this.repoRoot`-resolved paths (script.ts:8-9), plus the `[DEBUG] raster ts ok` console line on success.

**`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/`**'s own project.json/script.ts are separate from the TS
package: `📋️project.json` wires `test`/`test-quick`/`test-long`/`test-exhaustive`/`describe` targets
(all correctly `cwd`'d to raster's own rust package dir), and `📜️script.ts`'s `TestScript` runs
`runCargoTestBudgeted(["semio-s-plugin-raster"], this.repoRoot)` — a real cargo test invocation, and its
`DescribeScript` is the descriptor-regeneration path noted in §2.

**`✏️s/🔌️plugins/🖨️raster/🧪️oracle/🔣️.json`** declares `oracles: []`/`noOracleDecisions: []` (deferred to
the per-subset manifest, `🪆️subsets/🔣️.json:1-11`, which records `subsetPolicy: "single"` with a rationale:
raster's `s.raster.raster@1` is a self-contained print-raster document with no alternate conformance
profile to split against — `✳️any` is deliberately the only subset). It DOES declare one
`oracleHostPackages` entry, `semio-s-plugin-stdio-test-oracle` at
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` (verified this path exists) — raster links stdio's
native-format host package for whichever cases exercise import/export through stdio's codecs, not a raster-
owned oracle.

**Is `package.json` a verbatim copy?** Yes, of `cad-js`'s, confirmed byte-for-byte on `description`/
`scripts`/`dependencies` — only `name` and `repository.directory` differ. It is dead-but-misleading
metadata; the actual nx target machinery bypasses it.

## 5. React renderer side — real, wired components for both of raster's surface kinds

Both of raster's declared `surfaceKind`s resolve to real, already-implemented React hosts via the
Interpreter's dispatch table
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`):
- `resolveComponentSceneHost` switch, `:289` `case "canvas-2d": return Canvas2dHost;` and `:299`
  `case "paint-2d": return Paint2dHost;`.
- `SURFACE_KIND_SCENE_FIELD` map, `:338` `"canvas-2d": "canvas2d"`, `:343` `"paint-2d": "paint2d"`.

So raster's editor windowKinds (`raster-composite`, `raster-navigator`, both `paint-2d`) mount
`Paint2dHost`, and its viewer windowKinds (`framework.window.image`, `raster-view-navigator`, both
`canvas-2d`) mount `Canvas2dHost` — no missing/undefined host, no stub.

**`Paint2dHost`** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx`,
688 lines) is a substantial, real implementation: brush/eraser utilities, marquee selection, pick
interaction, camera pan/zoom reusing `Canvas2dHost`'s math, context-menu plumbing via `Interpreter`. Its
header docstring (`:3-5`) explicitly names it "raster-2d `ComponentSceneHost`: drives a `paint2d`-shaped
scene through the raster wasm session". It imports `createRasterSession` from
`../🪪️WasmSessionLoader/🟦️.tsx` (`:26`) and calls it inside `attachCanvas` (`:636`).

**`createRasterSession`**
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:209-211`):
```ts
export async function createRasterSession(): Promise<RasterWasmSession> {
  return createSurfaceSession((module) => new module.RasterSession());
}
```
`createSurfaceSession` loads `@semio-tech/framework-surface-rs` (`:18,33`, `loadSurfaceSessionModule`) —
**exactly** the same crate/wasm bundle raster's own `engines` Cargo declaration names (§1) and that
`buildEngineWasm` always builds first as the universal `framework_surface`/node-graph engine. So the
React-side raster editing surface is powered by the shared `framework-surface-rs` wasm module's
`RasterSession` class, not a raster-specific wasm build — confirming §1's "redundant declaration" finding
end-to-end.

**wgpu renderer**: raster's `paint-2d` surfaces are also genuinely rendered natively, not merely stubbed —
`SurfaceKind::Paint2d` has real render/interaction code in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
(`render_paint_2d`, `:1914`; `Paint2dDocSyncJson`/`Paint2dLayerJson` decode, `:2026-2065`; pick/marquee at
`:1448,1466`; camera-settle dispatch at `:422,804,914-999`). The **one** documented gap: navigator-fit
camera math is **reimplemented independently** in the wgpu target rather than sharing code with the
browser session — see the doc comment at `:2117-2119`: `paint2d_navigator_fit_viewport` is "a port of
`RasterHost::navigator_fit_camera_json`... that crate is a sibling used by the React `Paint2dHost`'s WASM
raster session and is not wired into this wgpu renderer's dependency graph, so the fit math is
reimplemented here". This is a narrow, deliberate code-duplication note, not a missing feature — the wgpu
native/browser targets both draw raster's composite+navigator windows.

**wgpu native target script** (`.vscode/launch.json`'s `"🛠️dev🖼️raster🧊️wgpu🖥️native"` entry runs `bun
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts
native raster`): `NativeRunScript.run` (`:326-355`) builds the standalone native binary via
`NativeBuildScript` (full release/dev native build, not the trunk browser-wasm target), starts the asset
server if raster declares assets (it declares `assets: []`, so this is a no-op for raster), resolves the
catalog's app args for `raster`, and runs `semio-wgpu-native --plugin raster <app-args>` — this is a THIRD
rendering mode distinct from both `dev raster` (browser, wgpu-via-trunk or react-via-vite): a genuine
native OS window, useful for `--smoke` (headless widget-tree dump) verification without a real display.

## 6. Storybook

- `.storybook/stories/framework/os/plugins.stories.tsx:51` — `export const Raster: Story = { args: {
  plugin: "raster" } };` — one of the generic `PLUGIN_BUILD_TARGETS`-matrix stories (every registered
  plugin crate gets a literal export, `toPascalCase(pluginId)`-named, per the header's documented Vite-CSF
  indexer constraint at `:8`) — this boots the whole `FrameworkOsShell` for `raster` inside Storybook via
  `OsBootHost`.
- `.storybook/stories/framework/hosts/Paint2dHost.stories.tsx` — a dedicated, raster-shaped fixture: two
  stories (`COMPOSITE_SCENE`/`NAVIGATOR_SCENE`, `:16-30`) construct a `Paint2dScene` with
  `documentSyncJson: {"schema":"raster.document","id":"raster","layers":[]}` and mount the real
  `Paint2dHost` component (imported from `@semio-tech/framework-renderer-react`, not a mock) — this is
  component-level (not full-shell) coverage of exactly the surface raster uses.
- Neither of these lives under a scope literally named `"raster"` — `.storybook/scopes.ts` has no raster-
  specific hand-curated entry; both stories are reached under the `framework` scope (`plugins.stories.tsx`
  is `framework/os/…`, `Paint2dHost.stories.tsx` is `framework/hosts/…`). This differs from the BLOCK
  ticket's finding of "storybook scope `block` resolves but has zero stories" — raster has no analogous
  dedicated scope at all (there's nothing under `.storybook/stories/raster/`), but it IS covered by both
  the whole-shell matrix and a real component fixture.

**End-to-end Playwright specs** (`.storybook/*.spec.ts`) that exercise raster without naming it literally:
- `.storybook/os-plugins.spec.ts:4,10,48` — iterates `PLUGIN_BUILD_TARGETS` (same generated catalog),
  navigates `iframe.html?id=<plugins-story-id>` per plugin, and asserts a deterministic boot outcome
  (`semioOsReady`/`semioOsError` dataset beacon) — raster is one of the iterated targets automatically.
- `.storybook/framework-hosts-wasm.spec.ts:66-77` — `test("Paint2dHost composite view: boots the real
  RasterSession WASM engine", …)` and a navigator-view test — asserts `.semio-paint-2d-canvas-surface` is
  visible and zero unexpected console/page errors. **This is real, currently-checked-in end-to-end
  coverage that the exact `RasterSession` wasm class boots and paints in a browser**, independent of
  today's stale-wasm finding in §2 (this spec targets the shared `framework-surface-rs` engine build, not
  raster's own plugin crate wasm).
- `.storybook/s-end-to-end.spec.ts` covers only the `s`/`space` host plugin specifically (asserted via
  `PLUGIN_HOST_CONFIGS`, `:11-17`) — it does not reference `block`/`raster`/any other plugin id literally;
  general per-plugin coverage for those lives in `os-plugins.spec.ts` above, matching the header comment's
  own framing ("`os-plugins.spec.ts` proves every plugin reaches a deterministic boot outcome... this spec
  makes the stronger claim `s` alone has to meet").

## 7. Sibling BLOCK ticket learnings that transfer identically to raster

From `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md` +
`📓️status.md` — every one of these mechanisms is the SAME code path for raster, only the ids/paths differ:

1. **Default renderer is wgpu, not react**, for the bare `bun run dev:raster` — must export
   `SEMIO_RENDERER=react` (or use `served`) to reach `ShellHost`.
2. **`SEMIO_PLUGIN_ONLY=<id>` never isolates the browser-side load closure** — it only narrows which
   crates get cargo-built; the registry's `filterPlugin`-derived closure (`{raster, stdio}` per §1) is
   fixed independent of it. Cold use risks cascading both to `plugin.descriptor-unavailable`.
3. **Registry `generate` is a hard gate** before anything else boots
   (`ensurePluginRegistry`/`bun 📇️registry/📜️script.ts generate`, throws on non-zero exit) — reads
   `🔣️taxonomy.json`; any concurrent taxonomy edit that breaks its invariants blocks raster's boot exactly
   as it would block block's, independent of anything raster-specific.
4. **Shared `target/debug/.cargo-lock` contention** and the 20-minute default `SEMIO_BUILD_BUDGET_MS` — the
   BLOCK ticket's whole `📓️status.md` log (04:16 through 19:35) is a record of exactly this class of stall
   (stdio recompiles under swap, sccache serialization, orphaned lock holders) hitting `block`'s shared
   dependency on `stdio` — raster shares the identical `stdio` dependency edge, so any stdio build/compile
   state block's session left behind (or a peer currently churning stdio) applies to raster's boot too.
   At this exploration's snapshot, `stdio`'s own directory shows **zero** uncommitted changes
   (`git status --porcelain` empty) — calmer than the BLOCK ticket's mid-session snapshot, but re-check
   before relying on this.
5. **`CARGO_TARGET_DIR` isolation pattern** (`export CARGO_TARGET_DIR=target-raster`, mirroring the BLOCK
   ticket's `target-block` / the PROCEDURAL ticket's `target-gen3d`) — same rationale: avoid contending
   with other sessions' shared `target/debug` cargo lock.
6. **Poll a real HTTP port, not a log file**, for readiness — `isDevPortInUse`/`probeWgpuDevPort`
   (`🟦️.ts`) or a direct `curl`/`fetch` against `http://127.0.0.1:<port>/` is the legitimate signal, per
   both the BLOCK and PROCEDURAL tickets' converged practice (background dev servers' own logs can vanish
   if `🗑️generated/` gets swept mid-run by another agent — see project memory
   `feedback-subagents-sweep-ticket-generated-folder`).
7. **`registryExampleCatalog` gap fix already landed** (BLOCK ticket, 2026-09-05 13:30): the discovery
   function now also scans `🪆️subsets/<s>/📚️examples/` at the editor/viewer surface level, not just the
   subset root — this is exactly the mechanism that makes raster's `demo-session` example (nested under
   `✏️editor/📚️examples/`) show up correctly in the generated catalog (§3 above); if this fix had NOT
   landed, raster's `demo-session` example would be invisible/orphaned the same way block's originally was.
   The BLOCK ticket also notes "the shell's example switcher reads `PluginManifest.examples` (ShellHost),
   not the registry" — worth re-checking once raster's stale descriptor (§2) is regenerated, since its
   current `examples: []` field, if actually read by the switcher rather than the registry, would mean NO
   example shows up in raster's own example picker despite both being discoverable by the registry scan.
8. **A committed owner-root descriptor is not itself sufficient** — raster HAS one (unlike block), but
   §2 shows it/its wasm sibling are stale relative to source; a fresh, un-narrowed build (or the dedicated
   `describe` nx target) is still required before trusting it.

## 8. Concrete boot recipes

**(a) Build raster's wasm with a private target dir** (isolating from peers' shared `target/`):
```
export CARGO_TARGET_DIR=target-raster
export CARGO_PROFILE_WASM_DEV_DEBUG=false   # cheap insurance; already false in committed Cargo.toml today
export SEMIO_BUILD_BUDGET_MS=3600000        # 1h, in case of shared-lock contention
cd /Users/ueli/Documents/semio
cargo build -p semio-s-plugin-raster --target wasm32-wasip2 --profile wasm-dev --features component-guest
```
(Mirrors the pattern `dev`'s own plugin-build step uses internally, and matches the `--profile`/
`--target` shape `pluginCargoArgs` builds — see dev-package `📜️script.ts` `pluginWasmProfile`/
`PLUGIN_WASM_TARGET` region.) Then regenerate the descriptor pair directly (fixes §2's hash mismatch
without booting the whole dev server):
```
bun nx run @semio-tech/raster-plugin:describe
```

**(b) Boot the react playground**:
```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=target-raster
export SEMIO_RENDERER=react
export SEMIO_BUILD_BUDGET_MS=3600000
# SKIP_ENGINE_BUILD=1 only if framework_surface/framework_editor/flow-core bindings are still fresh —
# re-check mtimes under 🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/ etc. first.
bun ./📜️script.ts dev raster
```
First boot should NOT set `SKIP_PLUGIN_BUILD`/cold `SEMIO_PLUGIN_ONLY=raster` (§1/§2/§7.2) — let it do a
full, un-narrowed build of `{raster, stdio}` at least once. Poll `http://127.0.0.1:6060/` for readiness.

**(c) Boot the wgpu playground** (this IS the bare default, no env needed):
```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=target-raster
export SEMIO_BUILD_BUDGET_MS=3600000
bun ./📜️script.ts dev raster
```
Poll `http://127.0.0.1:6160/?plugin=raster`; or, for the standalone native OS window (a third mode, not a
browser):
```
bun 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native raster
# add --smoke for a headless boot that dumps the widget tree as JSON instead of opening a window
```

**(d) Confirm windows actually render**:
- Structural DOM checks (react): `.semio-paint-2d-canvas-surface` (Paint2dHost's composite/navigator
  surface, class name confirmed at `🖌️Paint2dHost/🟦️.tsx:576`) should be visible for the editor's two
  windowKinds; the viewer's `canvas-2d` windowKinds mount `Canvas2dHost` (no raster-specific class name
  confirmed here — check `Canvas2dHost/🟦️.tsx` directly if auditing the viewer surface).
- Console: expect a clean `read_console_messages`/Playwright `page.on("console")` — the existing
  `.storybook/framework-hosts-wasm.spec.ts:67-72` spec already proves the shared `RasterSession` wasm boots
  cleanly in the generic Paint2dHost fixture; a raster-plugin-specific boot should be held to the same bar
  (zero unexpected `console.error`, filtering only benign asset-404s per `significantConsoleErrors`).
- Readiness beacon (`ShellHost`'s `data-semio-os-ready`/`data-semio-os-error` on `<html>`, the same
  mechanism `s-end-to-end.spec.ts`/`os-plugins.spec.ts` read) is the authoritative "did raster boot"
  signal for a full `dev raster` react session, not just the component-level Paint2dHost fixture.
- `[DEBUG]`-prefixed console logs are the project convention for temporary verification logging (per
  CLAUDE.md) — add/remove them around any new raster-specific boot-path code under test, don't leave them
  in permanently.

## Files referenced (absolute paths)

- `/Users/ueli/Documents/semio/package.json`
- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/.vscode/launch.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🖨️raster/` (served build)
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/` (universal engine build)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🛂️.descriptor.semio`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📋️project.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📜️script.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/package.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📋️project.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📜️script.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/🟦️.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🧪️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json` (comparison source)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/` (raster's oracle host dep)
- `/Users/ueli/Documents/semio/.storybook/stories/framework/os/plugins.stories.tsx`
- `/Users/ueli/Documents/semio/.storybook/stories/framework/hosts/Paint2dHost.stories.tsx`
- `/Users/ueli/Documents/semio/.storybook/os-plugins.spec.ts`
- `/Users/ueli/Documents/semio/.storybook/framework-hosts-wasm.spec.ts`
- `/Users/ueli/Documents/semio/.storybook/s-end-to-end.spec.ts`
- `/Users/ueli/Documents/semio/.storybook/scopes.ts`
- `/Users/ueli/Documents/semio/Cargo.toml`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md` (sibling reference)
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️status.md` (sibling reference)
