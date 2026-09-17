# 📓️ Wave 5F — puzzle 🖐️5d boot chain, serve supervisor, Playwright battery

Slice 5F (boot + proof infrastructure). No Nx/cargo build, no serve started, stopped or recycled.
Everything below is either an edit on disk or a command actually run, with its verdict.

## 0. Boot commands for the coordinator

```bash
cd /Users/ueli/Documents/semio
# 1 — activate (builds component-dev + materialize-dev + prepare, then stages the react runtime)
bun nx run @semio-tech/framework-os-dev:activate-puzzle5d-react-dev
# 2 — serve (already dependsOn activate, so step 1 is optional but makes the build failure legible)
bun nx run @semio-tech/framework-os-dev:serve-puzzle5d-react-dev
#    …or keep it alive across the inevitable vite disappearance:
nohup bash '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/🔁️serve-supervisor.sh' puzzle5d 6014 >/dev/null 2>&1 & disown
# 3 — URL
open 'http://127.0.0.1:6014/?plugin=puzzle5d'
# 4 — chrome discovery first (ids are NOT yet stable, see §5)
bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/🔍️browser-probe-5d.ts' --explore --port=6014
# 5 — the battery
bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/🔍️browser-probe-5d.ts' --battery --reload-between-groups --port=6014
```

All five are registered in `.claude/launch.json` as `puzzle5d-react` / `puzzle5d-react-supervised` /
`puzzle5d-react-attach` / `puzzle5d-battery-explore` / `puzzle5d-battery` (AGENTS.md:50-51).

**Target existence proven from source, not run.** `playgroundPreparationTargets`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1013-1026`) emits, per playground row × profile,
`serve-<variant>-react-<profile>` AND `dev-<variant>-react-<profile>` (both `continuous`, both
`dependsOn: activate-<variant>-react-<profile>`), plus `activate-…` and `prepare-…`. It branches on nothing
but the `[[package.metadata.semio.playground]]` rows, and `🟨️.mjs:951` (`playgroundSessionTargets`) emits
`session-<variant>` the same way. The `puzzle5d` row exists at
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` with `ports = { react = 6014, wgpu = 6114 }`.

## 1. Launch entries

### 1a. The seed — already complete for 5d, nothing to add

The `.vscode` seed is `.vscode/🧩️launch.seed.jsonc`, rendered into `.vscode/launch.json` by
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`. It **already carries the full 5d
family**, authored in the same order/grouping/naming as 2d and 3d:

| seed line | entry |
|---|---|
| `.vscode/🧩️launch.seed.jsonc:905` | `@generated:puzzle5d:react` (react dev + release, ports from the Cargo row) |
| `:906` | `@generated:puzzle5d:wgpu` |
| `:908-916` | `🛠️dev🧩️puzzle👯️5d🧊️wgpu🖥️native` |
| `:919-937` | `…5d🎛️concrete🌲️forest⚛️react` (PUZZLE_5D_PLAY_PORT 6014) |
| `:939-957`, `:959-967` | concrete-forest wgpu wasm + native |
| `:970-990`, `:992-…` | `…5d🎛️capsule🌙️dream⚛️react` / `…🧊️wgpu🌐️wasm` (ports 6015 / 6115) |
| `:2544-2566` | the `puzzle5d` devLauncher template (`order: 250`, `wgpuOrder: 250.1`) |

The generated `.vscode/launch.json:1704-1723` contains a live `🛠️dev🧩️puzzle🖐️5d⚛️react` on 6014. **No seed
edit was needed and none was made.** E4 §4a already said this; it is confirmed.

### 1b. `.claude/launch.json` — the real gap, now closed

`.claude/launch.json` is hand-maintained (no generator anywhere in the tree — a repo-wide grep for writers
returns only ticket prose describing hand edits, `🗑️generated/5F/` evidence). Before this slice it had zero
`puzzle5d` entries. Added, following the puzzle3d rows' order/grouping/naming:

| entry | form | port |
|---|---|---|
| `puzzle5d-react` | `bun nx run @semio-tech/framework-os-dev:serve-puzzle5d-react-dev`, `SEMIO_RENDERER=react`, `S_OS_PORT=6014` | 6014 |
| `puzzle5d-wgpu` | `bun nx run @semio-tech/framework-os-dev:dev -- puzzle5d`, `SEMIO_RENDERER=wgpu` | 6114 |
| `puzzle5d-native` | `bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d` (target exists, wgpu `📋️project.json:419`) | — |
| `puzzle5d-react-attach` | attach-only `url: http://localhost:6014` | 6014 |
| `puzzle5d-react-supervised` / `puzzle3d-react-supervised` / `puzzle2d-react-supervised` | `bash 🔁️serve-supervisor.sh <app> <port>` | 6014 / 6013 / 6012 |
| `puzzle5d-battery` / `puzzle5d-battery-explore` | `bun 🔍️browser-probe-5d.ts …` | — |

`puzzle5d-react` uses `serve-…` rather than `dev-…` (the two targets are identical aliases,
`🟨️.mjs:1013`) to match every entry added since `process3d-react`. JSON re-parsed: **44 configurations, 0
duplicate names.**

⚠️ **Port collision to decide, NOT touched by me.** `.claude/launch.json` already had
`puzzle3d-react-release-e2e-attach` pointing at **6014** — which is puzzle5d's assigned react port. Its
origin is the 3d ticket's own note, `…/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-11-wave-B19-inspection-leftover.md:17`:
*":6014 is `workspace:dev -- 5d`"* — i.e. a 3d session opportunistically borrowed the 5d serve (one cdylib
serves all three variants, so `?plugin=puzzle3d` works on any port). The entry is now misleading. I left it
(not my slice's row); the coordinator should rename or delete it.

## 2. Taxonomy filename drift — renamed, discovery proven unchanged

Renamed by hand (plain `mv`, no git command):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🛂️manifest.jsondefault.manifest.json` → `…/🧊️3d/🛂️manifest.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🛂️manifest.jsondefault.manifest.json` → `…/🖐️5d/🛂️manifest.json`

Contents untouched (`id: puzzle3d-default` / `puzzle5d-default`). `◻️2d` was already canonical (renamed by
the 2d ticket's wave A2, `…/☀️06/PUZZLE-2D-END-TO-END/📓️wave-A2-report.md:101-121`, which explicitly left
3d/5d "out of region"). The two `🔱️trinity` files with the same corruption
(`🛂️manifest.jsonnakagin.manifest.json`, `🛂️manifest.jsonrewrite-lhs.manifest.json`) are a different
plugin and were left alone.

**Readers/references — repo-wide grep for `manifest.jsondefault` (excluding node_modules/dist):** exactly
two live hits outside ticket prose, neither needing an edit.

1. `✏️s/…/◻️2d/…/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:295` — the ONLY `include_str!` consumer of any
   puzzle artifact manifest, and it already reads `🛂️manifest.json` (2d's). **3d's and 5d's manifests have
   no Rust, TS or JSON consumer at all** — they are graph-discovery inputs only.
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔏️path-emoji-statutes/🔣️.json:257` —
   `{ "name": "🛂️manifest.jsondefault.manifest.json", "expected": true }` under `graphManifestNames`. This
   is a **predicate vector**, not a file reference: the test at
   `…/🧪️tests/🔏️path-emoji-statutes/🟦️.ts:571-587` extracts the accept-expression out of
   `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts` and cross-checks it against
   `picomatch("*manifest.json")`. Left alone, exactly as wave A2 left it.

**Correction to E4 §5.** E4 cites `🔍️discovery/🟦️.ts:5141` as the statute these files fail. That check
validates the **values of the `semanticManifestFilenameOverrides` map**, not files on disk, and the
artifact-root manifest is not a *semantic collection* manifest at all —
`semanticManifestFilenameForCollection("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d")` returns **`🔣️.json`**
(measured, `🗑️generated/5F/manifest-discovery-*.txt`), because `semanticManifestFileKindId` is `"json"`.
The real admission rule for these files is `name.endsWith("manifest.json")`
(`📥️admission/🟦️.ts:62`), which BOTH spellings satisfy. So the drift was a **naming-convention** defect
(a flattening sweep concatenated a `🛂️manifest/` directory name onto `default.manifest.json`), not a gate
that was silently returning zero. Discovery was never blind to them.

**Member count proven before and after** (read-only probe over `findManifestFiles` +
`readGraphManifestDocuments`, outputs in `🗑️generated/5F/manifest-discovery-before.txt` /
`-after.txt`):

| | before | after |
|---|---|---|
| admitted manifests | 12 | 12 |
| puzzle manifests | 3 | 3 |
| document ids | `writer-languages, flow-dag, wires, rewrite-lhs, nakagin, drawing-layers, puzzle2d-default, puzzle5d-default, puzzle3d-default` | identical |

**Gates run (foreground, verdicts):**

| command | verdict |
|---|---|
| `bun ./📜️script.ts check-generated` in `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust` | **7 pass / 0 fail, 98 expect(), "9 generated manifests are fresh"** — `🗑️generated/5F/graph-check-generated.txt` |
| `bun test ./🧪️tests/🔏️path-emoji-statutes/🟦️.ts -t graph` in `…/📚️library` | **2 pass / 0 fail / 54 expect()** |
| same file, full run | 30 pass / **7 fail** — all 7 are mutation-vector / glTF-fixture / TSV-payload / OS-semantic-stem rows being churned by sibling slices right now (5G, 2F). None mentions graph manifests; none moved with my rename. `🗑️generated/5F/path-emoji-statutes.txt` |

`🔣️taxonomy.json`'s `graph-catalog-manifests` member list (line 7770-7785) names the manifests by **id**
(`◻️puzzle2d-default`/`🧊️puzzle3d-default`/`🖐️puzzle5d-default`) and
`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json:13-15` keys its generated registry on `id` too —
so the rename is invisible to both. `semanticManifestFilenameOverrides`
(`🔣️taxonomy.json:28665-28668`) has no puzzle entry and needed none.

## 3. Boot preconditions verified from source

| # | precondition | status | evidence |
|---|---|---|---|
| 1 | Nx targets `activate-/serve-/dev-puzzle5d-react-dev`, `session-puzzle5d` | **OK** | `🟨️.mjs:951` + `:1013-1026`; purely row-driven, no per-plugin branch |
| 2 | playground row, port 6014/6114 | **OK** | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` (`variant = "puzzle5d"`, `app = "s.puzzle.puzzle5d@1/*#editor"`, `aliases = ["5d", "puzzle 5d"]`). 5d has no `engines` key — same as 3d; `engines` only feeds `prepare-*-wgpu-*`, never the react lane |
| 3 | one cdylib serves all three variants | **OK** | plugin `Cargo.toml` depends on `semio-s-artifact-puzzle-{2d,3d,5d}` with `features = ["component-app-assembly"]`; `componentTargets` keys on `metadata.component.package = "semio:puzzle"` |
| 4 | examples on disk for the 5d playground | **OK** | `…/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/{🌙️capsule-dream,🌲️concrete-forest,🏗️nakagin-capsule-tower}` — three dirs, matching the three ids |
| 5 | example id strings a probe must click | **OK** | `…/✏️editor/🦀️.rs:61-63`: `concrete-forest`, `nakagin-capsule-tower`, `capsule-dream`; `🎮️commands/🛍️set-active-example/🦀️.rs:19-24` also accepts the short aliases `concrete` / `nakagin` / `capsule` |
| 6 | dual-surface windows declared | **OK** | `…/🎭️modes/✏️edit/🪟️windows/◻️2d/🦀️.rs:19-21,34,184` (`WINDOW_KIND_ID "puzzle5d-2d"`, `SurfaceKind::Board2d`, `scene_surface`) and `…/🧊️3d/🦀️.rs:28-30,44,205` (`"puzzle5d-3d"`, `SurfaceKind::World3d`) |
| 7 | engagement input id per pane | **OK** | `…/🎭️modes/✏️edit/🦀️.rs:57` `format!("puzzle5d-engagement-{window}")` with `window = WINDOW_KIND_ID` → `puzzle5d-engagement-puzzle5d-2d` / `-puzzle5d-3d`; status `puzzle5d-status-<kind>` |
| 8 | Fill is a first-class Tool | **OK (landed by 5B during this slice)** | `…/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:20-51` — `TOOL_ID "fill"`, `RUN_JOB_KIND "s.puzzle.puzzle5d.fill.run"`, `REVALIDATE_JOB_KIND`, `run_definition()`. The battery drives `#tool.fill` accordingly |
| 9 | settings panel | **MISSING at the time of writing** | `…/✏️editor/📌️panels/` holds only `🔍️inspection`, `🗿️artifact`, `🛍️catalogue`. Owned by slice 5C; the battery's `§19-settings` lane names `puzzle5d.panel.settings` and will read red until 5C lands it |
| 10 | window-option ids | **partially landed** | `puzzle5d-play-{board-grid,board-select,world-grid,world-lod,world-grip-show,world-grip-direction,world-select}` already appear in 5d's editor source (slice 5E in flight). No `projection` id found yet — the battery's `world-projection-switches` lane will read red until it exists |
| 11 | session-registry closure for `puzzle5d` | **OK** | `🟨️.mjs:927-943` is row-driven; `🧑‍💻dev/🔗️boot-query/🟦️.ts` contains **zero** `puzzle` occurrences (fully variant-agnostic). Diffing the `puzzle3d` vs `puzzle5d` grep hit-sets over `🔌️plugin/📇️registry/**` and `🧑‍💻dev/**` (excluding dist) gives an **empty diff** — every file naming 3d also names 5d, including `🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/🔣️.json` (controllerId `s.puzzle.puzzle5d@1/*#editor`). The only asymmetric file is `🔌️plugin-modules/🎪️demonstrator/🔣️.json`, which is the demonstrator's own bundle manifest, irrelevant to a standalone 5d boot |
| 12 | plugin builder id == component metadata id | **OK** | `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:66,69` — `Plugin::<PuzzleApps>::builder("puzzle").package_id("semio:puzzle")` matches `Cargo.toml:12` `package = "semio:puzzle"`. `PuzzleApps` (`:22-29`) includes `Puzzle5dEditor`/`Puzzle5dViewer`; `.declare_artifact` ×3 (`:70-72`), `.editor_mutation_roster::<…Puzzle5dPlayApp>()`/`.viewer_mutation_roster` (`:77-78`), `.activation(OnArtifactKind{ semio_s_artifact_puzzle_5d::artifact_kind().id })` (`:85`). The builder-id-drift trap does not apply |
| 13 | `VITE_SEMIO_APP_ID` resolves non-empty | **OK** | `♻️activation/🌐️serve/🟦️.ts:54` sets it to `resolved.appId ?? ""` from `resolvePlaygroundFilter(variant)` (`🔌️plugin/🏗️build/📋️plan/🟦️.ts:119-121`), which finds the row by `variant` and returns its `app`. 5d's row declares `app`, so it pins to `s.puzzle.puzzle5d@1/*#editor` — same generic path as 3d |
| 14 | per-lane vite cache dir | **OK** | `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:73` — `repoCacheDirectory(repoRoot, "vite", "os-dev", \`${plugin}-${renderer}-${profile}\`)` with `plugin = process.env.SEMIO_PLUGIN` (`:26`). puzzle5d gets its own `puzzle5d-react-dev` dir, distinct from 3d's and 2d's, with no special-casing |
| 15 | R3F CJS include for the world pane | **OK** | `🌐️vite/🟦️.ts:204-209` calls `playgroundSceneHostOptimizeDeps({…})` **unconditionally**, not gated on `plugin === "puzzle3d"`. The helper (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:966-973`) hardcodes `PLAYGROUND_SCENE_HOST_CJS_INCLUDE = ["scheduler", "stats.js", "use-sync-external-store/shim/index.js", "use-sync-external-store/shim/with-selector.js"]` plus `"three"`. 5d's `World3d` pane gets the identical shim list |
| 16 | **`/mesh` + `/infinite-assets` routes mounted for the 5d serve** | **WAS BROKEN — FIXED in this slice** | see §3a |
| 17 | examples-discovery filter | **defective but harmless for 5d — logged, not fixed** | see §3b |

Items 9-10 are other slices' deliverables and are deliberately left to them.

### 3a. Fixed: the World3d pane had no mesh route on a puzzle5d serve

`generatePlaygroundRegistry` keeps an asset row only when `asset.app === undefined || asset.app === playground.app`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts:172`). Both
`[[package.metadata.semio.assets]]` rows in the puzzle plugin's `Cargo.toml` were scoped to
`app = "s.puzzle.puzzle3d@1/*#editor"` **only**, so a `puzzle5d` serve resolved `assets = []` and the vite
config's `resolvedPlaygroundAssets` (`🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:81`) would mount neither `/mesh` nor
`/infinite-assets`. 5d's own examples reach for `/mesh/*.glb` constantly — measured on disk:
`🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio` **180** hits,
`🌙️capsule-dream/…/🗣️.dsl.semio` **2880**, `🌲️concrete-forest/…` 1. Every one would have 404'd, i.e. a
mesh-less (visually empty) World3d pane on first boot.

**Fix:** `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` — the two rows duplicated with
`app = "s.puzzle.puzzle5d@1/*#editor"`, and the comment above them corrected to say why every variant that
reaches for `/mesh` must name itself. `[package.metadata]` is inert for compilation, so no crate rebuild is
implied.

**Proof (run, `🗑️generated/5F/playground-registry-after.txt`):** `generatePlaygroundRegistry(repoRoot)` now
returns

```
puzzle3d: ports={"react":6013,…} assets=["mesh-collection:/mesh","static-dir:/infinite-assets"]
puzzle5d: ports={"react":6014,…} assets=["mesh-collection:/mesh","static-dir:/infinite-assets"]
puzzle2d: ports={"react":6012,…} assets=[]
```

and the manifest still parses strictly (4 asset rows, `bun` TOML import). 2d was left with `assets=[]` — it
is a board-only variant that boots green on 6012 today, and widening it is not this slice's call.

### 3b. Logged, not fixed: `declaredExampleIdsForPlayground` filters on a field that does not exist

`…/🎮️playground/🔎️discovery/🟦️.ts:127` filters descriptor rows on `(row as { appId?: unknown }).appId === app`,
but the Rust `ExampleDefinition` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3772-3778`) has **no `appId`
field** — it serializes `dialect: {artifactKind, standard, subset}`. So `declared` is always empty for every
playground row and `discoverExamplesForPlayground` (`:143-146`) always falls through to the unfiltered
`registryExampleCatalog` union, which walks `pluginRoot/🗿️artifacts/*` — and 2d/3d/5d share one crate.

Measured consequence (same probe run): **all three variants get the identical 4-example union**
`["🌙️capsule-dream","🌲️concrete-forest","🎬️demo-session","🏗️nakagin-capsule-tower"]`. For **puzzle5d this is
harmless** — the three ids the battery needs are all present — but `capsule-dream` and the editor-only
`demo-session` leak into puzzle2d's and puzzle3d's pickers, where no such document exists. Minimal fix would
be at `:127`: match `row.dialect` against the dialect derived from `app` instead of a non-existent `appId`.
**Not applied here**: it is shared registry code that would change 2d's and 3d's pickers while slices 2A/2G
are driving the 2d battery, and the ticket brief's claim that wave A2 already fixed this generator is
therefore only half true (A2 fixed the `_pluginId`/`_variant` plumbing, not the field name it filters on).
The battery's `picker-lists-three` lane asserts "≥3 options AND all three names present", so the extra
`demo-session` row does not make it red.

## 4. Serve supervisor

`TICKET/🔁️serve-supervisor.sh` (chmod +x, `bash -n` clean). Ported from
`…/☀️06/PUZZLE-2D-END-TO-END/🔁️serve-supervisor.sh`, parameterised:

```
🔁️serve-supervisor.sh [variant] [port]      # default: puzzle5d 6014
🔁️serve-supervisor.sh puzzle3d 6013
🔁️serve-supervisor.sh puzzle2d 6012
```

- `ROOT` resolves 7 levels up from the script; verified to land on `/Users/ueli/Documents/semio`.
- Logs to `🗑️generated/serve-<port>-supervised.txt`, events to
  `🗑️generated/serve-<port>-supervised-events.txt` (`.txt`, not `.log` — a gitignored extension is dropped
  from a ticket's `files` array).
- `alive()` = `curl` 200 on `/`; two consecutive misses 10 s apart → `start()`, which kills the listener
  **by pid** (`lsof -t`, TERM then KILL) before relaunching `bun ./📜️script.ts serve <variant> react dev`
  in the os-dev TS package (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:26`
  registers `serve`).
- nohup+disown friendly; **not run by me.**

## 5. The battery — `🔍️browser-probe-5d.ts`

1 645 lines, **46 registered steps emitting 103 verdicts**, ordered `read` → `mutate` → `replace`, plus a
`guest-alive-<group>` checkpoint per group and a `battery-hard-faults` closer. Flags: `--battery`,
`--only=`, `--port=` (default 6014), `--plugin=`, `--explore`, `--reload-between-groups`, `--settle=`,
`--tail=`, `--boot-polls=`, `--board-surface=`, `--world-surface=`.

Outputs into `TICKET/🗑️generated/`: `probe5d-<stamp>.md`, `probe5d-<stamp>.ndjson`, one screenshot per
step, and the summary line
`battery PASS=n FAIL=n FAULTS=n HARD=n first-hard-fault-at=<s> guest-death-faults=n`.

### 5a. The one design decision worth knowing

**Panes are resolved by capability, never by a hardcoded id.** A window *instance* id is a layout-seed
product (`puzzle3d-main-perspective`, `2d-overview`), not the `WINDOW_KIND_ID` the guest declares, and 5d's
instance ids cannot be known before the first boot. So `waitForBoot` enumerates `[data-surface-id]` and
classifies: the element publishing `data-board-nodes` is the **board** pane, the one publishing
`data-instances-json` is the **world** pane, and each one's owning `[data-slot="window"]` id becomes the
scope for that pane's Actions/engagement/utility/option controls. `--board-surface=`/`--world-surface=`
override it. Booted therefore means *both* panes mounted and publishing — a single-pane boot is reported as
a boot failure, not a slow boot.

Everything scoped per window: `actionsOpen`, `setActions`, `runAction`, `engage`, `armUtility`,
`unfoldUtilities` all take a window id, because both panes mount at once and the framework authors the same
control ids into each (the defect that cost the 2d battery its `import` lane, `📓️E7` §3).

### 5b. Lane table

`hook` is the selector/verb the lane reaches for; it is echoed into the lane's own verdict detail, so a red
names the missing hook. Owner = the slice that must make it green.

| § / lane | asserts | hook(s) | owner |
|---|---|---|---|
| §1 `panes/two-windows` | ≥2 window instances, ≥2 canvases | `[data-slot="window"]` | 5F (boot) |
| §1 `board-vitals-published` | board surface present, `nodes ≥ 0`, `fixture-parsed=true` | `data-board-nodes`, `data-board-fixture-parsed` | 5E / framework |
| §1 `world-vitals-published` | world surface present, instances + meshes published | `data-instances-json`, `data-meshes-json` | 5E / framework |
| §5 `picker-lists-three` | picker offers concrete-forest + nakagin + capsule | `playground.navbar.fixture` | 5F/registry |
| §5 `concrete-forest-loads` / `nakagin-loads` | census grows on both panes, no recovery card | `setActiveExample` | 5A2 |
| §5 `capsule-dream-loads-or-refuses` | loads **or** is refused with a notice; **zero new hard faults** either way | `setActiveExample` | 5A2 |
| §5 `switch-returns-to-concrete` | switching back re-loads | `setActiveExample` | 5A2 |
| §17-19 `panels` ×5 | outliner / catalogue / inspector / history / tool-runs each open non-empty | `framework.panel.{artifact,catalogue,inspection,history,toolRun}` | 5C |
| §18 `kind-rows-non-empty` | catalogue rows exist (inferred kinds fallback) | `puzzle5d-play-kinds.{parts,grips,fasteners,ropes}` | 5C |
| §16 `inspector-shows-selected-part`, `inspector-not-document-summary` | per-entity fields, not the document summary | `puzzle5d-play-inspector.*`, not `.empty` | 5C |
| §19 `settings-panel-present`, `settings-stepper-changes-value` | app settings child tab + a working stepper | `puzzle5d.panel.settings`, `[data-slot=stepper-plus]` | 5C |
| §2 `board-wheel-zoom-persists` + `board-zoom-leaves-world-camera` | board camera changes, world camera does **not** | `data-board-camera-json` / `data-camera-json` | 5A1 (`setCamera2d`) |
| §2 `world-orbit-persists-camera` + `world-orbit-leaves-board-camera` | orbit persists, board camera untouched | `setCamera3d` → `data-camera-json` | 5A1 |
| §6 `board-click-selects` → `board-selection-highlights-world` | click in board, same id confirmed in world | `data-board-selection-json` → `data-guest-selection-json` | 5E (cross-pane law) |
| §6 `world-click-selects` → `world-selection-highlights-board` | reverse direction | `data-guest-selection-json` → `data-board-selection-json` | 5E |
| §7 `board-hover-publishes`, `hover-pairs-across-panes` | hover in one pane names the same entity in the other (a gap **neither** existing battery covers) | `data-board-hovered-id` → `data-interaction-json.hoverTarget` / `data-hover-paint-id` | 5E |
| §6 `marquee-selects-many` | rubber band selects ≥2 | `data-board-selection-json` | 5E |
| §6 `select-same-kind` | context row grows the selection | `selectSameKindSelection` | 5A2 |
| §15 `board-part-menu-opens`, `board-part-menu-vocabulary`, `world-menu-opens` | rows per selection kind: delete / duplicate / same-kind / hide-show / lock-unlock / zoom-or-focus | `[role="menu"]` rows vs 5d's own Migrated set | 5D (menu audit) |
| §8 `board-drag-moves-flat-pose` | board drag moves x/y only | `applyBoardEvents` → `data-board-positions-json` | 5A2 |
| §8 `gumball-translate-changes-pose` + `gumball-translate-updates-board` | 3d pose changes AND the flat pose follows | `translateSelection` | 5A2 |
| §8 `gumball-rotate-changes-pose` | rotate handle moves the pose | `rotateSelection` | 5A2 |
| §9 `brush-previews-candidate`, `tab-cycles-candidate`, `shift-tab-cycles-back`, `brush-click-places-part` | place from a grip, Tab/Shift+Tab cycle | `data-engagement-preview-json`, `cycleBrushCandidate`, `addBrushPart` | 5B |
| §13 `suggestions-popup-opens` | alt+right-click opens the grip-suggestion popup | `data-suggestion-menu-json`, `targetBrushSuggestions` | 5B |
| §10 `volume-brush-paints-volume` | a painted volume appears | `data-target-volumes-json`, `addTargetVolume` | 5G |
| §10 `fill-stays-inside-volume` | every instance fill places sits inside the union of published volumes; escapees are **named** | fill ∩ `data-target-volumes-json` | 5G + 5B |
| §12 `fill-tool-tab-present`, `fill-count-measure`, `fill-weight-groups` | tool activate toggle, count, part+grip weight groups (EN **or** DE) | `tool.fill`, `puzzle5d-fill-count`, `setObjectKindWeight`/`setVortexKindWeight` | 5B |
| §12 `fill-start-places-provisional-board` + `…-in-world` | provisional placements visible in **both** panes | board `nodes` + `data-instances-json` | 5B |
| §12 `fill-{pause,step,resume}-control` | ToolRun chrome present and clickable | ToolRun buttons | 5B |
| §12 `fill-raise-mid-run`, `fill-lower-mid-run` | target count re-plans mid-run | `setFillCount` | 5B |
| §12 `fill-finalize-keeps-placements` | finalize after completion keeps the parts | ToolRun `finalize` | 5B |
| §12 `fill-undoes-in-one-step` | the **whole** run undoes with one `action.undo` (a per-placement ledger kills the store at 64 edits) | `action.undo` | 5B |
| §12 `escape-aborts-run` | Escape ends a live run | `engagementAbort` | 5B |
| §11 `world-relocate-moves-part` | relocate utility commits | `worldRelocate` | 5A2 |
| §11 `fastener-create`, `fastener-retarget-row`, `fastener-delete` | edge count up / menu row present / edge count down | `createFastener`, `retargetFastener`, `deleteFastener` | 5A2 |
| §11 `proximity-connect` | engagement `connect` changes the fastener census | `proximityConnect` | 5A2 |
| §16 `inspector-patch-writes-back` | an inspector field edit survives | `patchPart`/`patchGrip`/`patchFastener` | 5A2 + 5C |
| §17 `outliner-hide-toggles`, `outliner-lock-toggles`, `outliner-show-restores` | hide/lock flip **and flip back** (3d's live `outliner-show-restores` FAIL) | `setSelectionFlag` | 5C |
| §18 `catalogue-click-adds-part` | a kind row adds one part | `addPartKind` | 5A2 + 5C |
| §18 `catalogue-drag-into-board`, `catalogue-drag-into-world` | drag-drop into **each** pane | `pushPuzzle5dFixtureDropPreview`, `WorldCatalogueDropPreviewStore` | 5D + host |
| §23 `add-part-dialog-opens`, `…-lists-live-kinds`, `…-adds` | dialog with **live** kinds, not one hardcoded "Part" | `action.addNode` | 5D |
| §22 `delete-selection`, `duplicate-selection`, `focus-selection-moves-camera` | key verbs; focus is measured after orbiting AWAY (framing a framed pane republishes a bit-identical pose) | `deleteSelection`, `duplicateSelection`, `focusSelection`/`zoomToSelection` | 5A2 |
| §21 `copy-paste-adds`, `cut-removes`, `paste-restores-cut` | clipboard round trip | `Puzzle5dClipboardJob` | 5D |
| §20 `history-panel-lists-ledger`, `undo-changes-document`, `redo-restores-document`, `checkpoint-available`, `board-still-parses-after-history` | full history lane incl. redo (a gap in the 2d battery) | `framework.panel.history`, `action.{undo,redo,checkpoint}` | 5D / framework |
| §24 `export-downloads-json`, `export-names-the-example` | a real download whose filename names the example (3d's live failure) | `action.exportFixture` | 5D |
| §24 `import-round-trip` | file chooser fires, census returns to the exported one, board still parses | `action.openImportFixture` | 5D |
| §14 `engagement-advertises-verbs` | the placeholder lists the parsed verbs | `puzzle5d-engagement-puzzle5d-2d` | 5B |
| §14 `engagement-fill-12`, `escape-aborts-engagement-run`, `engagement-move`, `engagement-rotate`, `engagement-scale`, `engagement-repeat-last` | the grammar, typed **whole** (a per-character echo race swallows keyed-in text) | engagement input + `engagementRepeatLast` | 5B |
| §4 `board-options-present`, `board-grid-toggle-accepts` | grid + LOD + selectable kinds on the board pane | `puzzle5d-play-board-{grid,lod,select}` | 5E |
| §4 `world-options-present`, `world-sun-toggle-publishes`, `world-projection-switches`, `world-grip-show-toggles` | sun/projection/grip-show/grip-direction/LOD/grid/select on the world pane | `puzzle5d-play-world-*`, `toggleSun` → `data-sun-json` | 5E |
| §25 `german-flips-labels`, `english-restored` | EN ↔ DE round trip | `framework.settings.language` | 5C / terminology |
| §0 `guest-alive`, `guest-alive-{read,mutate,replace}`, `battery-hard-faults` | no recovery card, no guest-death fault, **both** panes still publishing | `data-plugin-recovery` + both panes' vitals | all |

### 5c. Reconciliation with the sibling slices

**No `📓️wave-5?-report.md` existed in the ticket folder when this was written**, so every selector above
was derived from 5d's own Rust source where it already exists, and from the 3d/2d naming otherwise. The
ones that are *predictions*, to be reconciled against the owning slice's report:

| predicted selector | derived from | reconcile with |
|---|---|---|
| `tool.fill` + ToolRun `start/pause/step/resume/finalize/abort` button labels | 3d's `🛠️tools/🪣️fill` + framework ToolRun chrome; 5d's `TOOL_ID = "fill"` confirmed | 5B |
| `puzzle5d-fill-count` input | fault-code string `puzzle5d-fill-count` in 5d source | 5B |
| "Part Weights" / "Grip Weights" panel headings (EN **or** DE matched) | 3d's fill weights groups | 5B |
| `puzzle5d.panel.settings` child tab id | 2d's `puzzle2d.panel.settings` | 5C |
| `puzzle5d-play-kinds.parts.*` row ids | the `puzzle5d-play-kinds.{parts,grips,fasteners,ropes}` strings already in 5d source | 5C |
| `action.{undo,redo,checkpoint,exportFixture,openImportFixture,addNode}` | 2d's Actions-pane row ids | 5D |
| `puzzle5d-play-world-projection*` (**not found in source today**) | 3d's projection option | 5E |
| utility rail labels matched as `/move|translate|transform/`, `/rotate/`, `/relocate/`, `/volume/`, `/brush/` | 5d's seven `.utility(...)` registrations | 5B / 5E / 5G |
| world-pane gumball drag geometry (centre → +120 px) | 3d's `dragGumballMoveX` | 5A2 |
| `engagement connect` as the fastener verb | 2d slice 2D's planned `connect` | 5A2 / 5D |
| target-volume JSON shape (`{min,max}` **or** `{origin,size}`; both handled) | 3d's `data-target-volumes-json` | 5G |

⚠️ One trap already avoided and worth repeating to whoever writes further lanes: the example labels are
**"Concrete Forest", "Nakagin Capsule Tower", "Capsule Dream"** — a bare `/capsule/i` matches Nakagin, so
`capsule-dream` is matched on both words (`CAPSULE_DREAM = /capsule\s*[-_]?\s*dream/i`). A lane that picks
the wrong document measures the wrong thing and still reads green.

### 5d. Type-check and self-check (run, with verdicts)

| command | verdict |
|---|---|
| `bunx tsc --noEmit --skipLibCheck --strict --target esnext --module esnext --moduleResolution bundler --types node ./🔍️browser-probe-5d.ts` | **exit 0, zero diagnostics** (`🗑️generated/5F/probe-typecheck.txt`) |
| `bun build --no-bundle ./🔍️browser-probe-5d.ts` | **transpiled clean** |
| `bun ./🔍️browser-probe-5d.ts --explore --port=1 --boot-polls=1 --settle=0.1` | ran end to end against a dead port: dumped an empty inventory, wrote `.md` + `.ndjson`, printed `battery PASS=0 FAIL=1 FAULTS=0 HARD=0 …` and exited 0 — proves the whole file executes, not just compiles |

Three fixes the type-check forced, worth knowing because the 2d/3d probes carry the same shapes:
`page.mouse.click` takes **no** `modifiers` (the chord is held on `page.keyboard` around the press — two
sites); and `import.meta.dir` is Bun-only and does not type-check without `bun-types`, so the probe uses
`decodeURIComponent(new URL(".", import.meta.url).pathname)` instead. The 2d and 3d probes still use
`import.meta.dir` and would not pass a plain `tsc`.

## 6. Not verified / handed off

- **Nothing on :6014 was ever contacted.** No serve was started, stopped or recycled, no Nx target run, no
  cargo invoked. Every "target exists" claim is a source read of `🟨️.mjs`, not a `bun nx show project`.
- **The three classic black-boot causes are all OK in source** (§3 items 13-15: app-id pinning, per-lane
  vite cache dir, R3F CJS includes) — but "OK in source" is not "boots". If the 5d page comes up blank, the
  remaining live suspects are a **fresh variant cache dir racing the dep optimizer** on its very first boot
  (the cache dir is per-variant and has never been populated for `puzzle5d-react-dev`; the symptom is a
  pageerror naming `scheduler` or "no default export" — wipe
  `⚡️cache/vite/os-dev/puzzle5d-react-dev` and restart), and a **stale staged plugin module** (the boot
  banner will say so). `--explore` distinguishes them: zero windows = app id / session; a pageerror naming
  a module = the cache.
- **The mesh-route fix (§3a) is source-only and unproven at runtime** — `Cargo.toml` metadata is read by
  the Nx/registry TS layer, which I exercised, but no serve has mounted the route. If the World3d pane
  boots mesh-less, re-check `resolvedPlaygroundAssets` on the live serve first.
- **`§19-settings` and `§4 world-projection-switches` will be red until 5C and 5E land** their panel and
  option; that is expected, and the lanes name the missing hook.
- **Registries were not audited by me** — `PUBLICATION_CONTRACTS` lanes, `command_from_action`,
  `bounded_first_step_tool_proofs!` coverage and the `Migrated` reclassification of 5d's 35
  `BatchOnlyPendingRewrite` verbs are slices 5A1/5A2/5B's deliverables. The battery simply measures whether
  the click does anything.
- **A dead-port smoke artifact is in `🗑️generated/`**: `probe5d-2026-09-17T09-09-09.{md,ndjson,png}`,
  `booted=false PASS=0 FAIL=1`. It is the §5d self-check, not a real run; ignore or delete it.
- **`puzzle3d-react-release-e2e-attach` on port 6014** in `.claude/launch.json` is left as found and needs
  a decision (§1b).
- The `🔱️trinity` manifests with the same filename corruption were left alone (different plugin).

## 7. Files touched

| file | change |
|---|---|
| `.claude/launch.json` | +8 configurations (`puzzle5d-react`, `-wgpu`, `-native`, `-react-attach`, `puzzle5d/3d/2d-react-supervised`, `puzzle5d-battery`, `puzzle5d-battery-explore`); 44 total, 0 duplicate names |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🛂️manifest.json` | renamed from `🛂️manifest.jsondefault.manifest.json`, contents unchanged |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🛂️manifest.json` | renamed likewise |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` | +2 `[[package.metadata.semio.assets]]` rows for `s.puzzle.puzzle5d@1/*#editor` (`/mesh`, `/infinite-assets`), comment corrected — §3a. Metadata only; no compilation impact |
| `TICKET/🔁️serve-supervisor.sh` | new, parameterised by variant + port |
| `TICKET/🔍️browser-probe-5d.ts` | new, 46 steps / 103 verdicts |
| `TICKET/🗑️generated/5F/*.txt` | command transcripts cited above |

No `.vscode/🧩️launch.seed.jsonc` edit (already complete for 5d). No Rust, no schema, no framework TS edit —
the only non-ticket source change is the two `Cargo.toml` metadata rows in §3a.
