# Audit — os s frontend (OS shell boot, plugin/artifact registration, build state, tests, known-broken)

Read-only audit. Scope: `🧰️framework/🛍️products/💻️os`, `🧰️framework/📦️packages`, `🧰️framework/🔨️modules/🖱️ui`, `✏️s/🔨️modules`, `✏️s/🧪️tests`, root `📜️script.ts`, `.claude/launch.json`, `nx.json`, `📋️project.json`, taxonomy/registry files. No files edited, no builds triggered beyond the read-only introspection commands below. Raw command captures are under `🗑️generated/os-*.txt` in this ticket folder (other files in that folder, e.g. `hub-*`, `plugins-*`, `mcp-*`, `collab-*`, belong to the sibling audits `audit-hub-backend.md` / `audit-plugins-artifacts.md` / `audit-ai-mcp.md` / `audit-collaboration.md` and were left untouched).

Session note: at the time of this audit, concurrent agents held active `cargo`/`rustc` processes (`ps aux` showed `rustc --crate-name semio_s_plugin_procedural` at 92.8% CPU compiling wasm32-wasip2, and a `semio-s-artifact-remodel-remodeling` test binary running). Per the task's own instruction ("if a build lock is held by another process, report that and skip"), **no `cargo check` was run** against the os host crate — see §3.

## 0. tl;dr on architecture

The ticket's framing ("s operating-system shell that boots all plugins and artifacts") is **not literally how the repo is wired**. `dev s` boots exactly one plugin — **`space`** (`pluginId: "space"`, `✏️s/🔌️plugins/🪐️space`) — as the OS **hub** shell (`host.landingAppId: "home"`, `host.hostAppId: "studio"`, see §1). Every other plugin (puzzle3d, cad, process3d, fem, …) is its own separate "playground variant" with its own dev port and its own `dev <plugin>` / `serve-<plugin>-react-dev` Nx target (55+ separate `.claude/launch.json` entries). Whether the `space` hub can dynamically load an arbitrary registered plugin's wasm at runtime when a user opens an artifact of that kind (i.e. true "boots all plugins" behavior) could not be confirmed from the TS/Rust surfaces searched — see the P0 item in §6.

## 1. How the os frontend boots

**Entry command chain** (root `📜️script.ts`):
- `bun ./📜️script.ts dev s` (and `bun nx run workspace:dev -- s`) → `DevScript.run()` at `📜️script.ts:402`, branch at `📜️script.ts:412-415`:
  ```
  if (segments[0] === "s") {
    runFrameworkOsPlaygroundDev("s", segments.slice(1));
    return;
  }
  ```
- Falls through to the same call with no segments at `📜️script.ts:441` (`runFrameworkOsPlaygroundDev("s")`) when `dev` is invoked bare.
- `runFrameworkOsPlaygroundDev` (`📜️script.ts:266-273`) shells out to `bun nx run @semio-tech/framework-os-dev:dev -- <plugin> <rest>`, with env from `frameworkOsPlaygroundDevEnv(...)`. The doc comment directly above it (`📜️script.ts:255-265`) is the clearest description of the actual boot chain:
  > "A bare `dev <variant>` under `SEMIO_RENDERER=react` runs the variant's whole Nx activation chain — every selected plugin's `component-<profile>`/`materialize-<profile>`, the browser support bundle, the guest fonts, the engine `wasm` producers, the generated playground session, then `prepare` and `activate` — before Vite serves the receipt (os-dev `DevScript`)."
  > "`served` opts out of that chain and serves whatever `dist/<profile>/🔌️plugin-modules/` … already hold… It also forces react, because `frameworkOsPlaygroundDevEnv` defaults `SEMIO_RENDERER` to `wgpu`."
- A separate `multi` branch exists at `📜️script.ts:416-425`, forwarding to `@semio-tech/framework-os-dev:dev -- multi …` **without** setting `SEMIO_PLUGIN` (comment at `📜️script.ts:417-420` explains this is deliberate because `multi` is "not a registered playground variant"). This looks like the intended path for hosting more than one plugin at once, but its implementation was not found among the TS files under `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev` grepped for the literal `"multi"` — follow-up needed (§6, P0/P1).
- Any other `dev <app>` resolves through `resolvePlaygroundDevApp` (`📜️script.ts:248-251`, `428`) against the generated playground catalog (`loadFrameworkOsPlaygroundSelections`/`resolveFrameworkOsPlaygroundPlugin`, imported at `📜️script.ts:105,112`) — this is the per-plugin dev-serve path, distinct from the hub.

**Renderer targets**: two renderer backends exist side by side — `react` (Vite-served) and `wgpu` (native/browser wgpu renderer, `trunk serve`-style per the same doc comment). `SEMIO_RENDERER` env selects between them; `served` mode forces react.

**Vite / dev-server plumbing** lives under `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev`:
- `🏗️builder/🌐️vite/🟦️.ts` — the Vite config builder (referenced as a named input on almost every target in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:8`).
- `🔌️vite-plugins/🟦️.ts`, `⚙️engine/🧭️selection/🟦️.ts`, `⚙️engine/📤️publication/🟦️.ts` — plugin selection/publication into the dev bundle.
- `♻️activation/{🔐️lease,🩺️readiness,📥️installation,🔍️freshness,🧰️preparation,🏃️execution,🌐️serve}/🟦️.ts` — the "prepare→activate→serve" pipeline the doc comment above describes.
- `📇️registry/🔄️refresh` regenerates the plugin/playground catalogs (see §2) — invoked when the on-disk plugin set changes.
- The wasm-plugin loading path (host-side ABI) lives under `🧰️framework/🛍️products/💻️os/🖥️host` (activation: `🖥️host/🎠️activation/🦀️.rs`) and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/{🖥️host,🌐️browser-bundle,🌐host}`.

**`.claude/launch.json` — full-hub vs per-plugin dev serves:**
| config | maps to | scope |
|---|---|---|
| `s-react` (port 6070) | `bun nx run workspace:dev -- s` | **the hub** (`space` plugin), react renderer |
| `s-react-served` (port 6070) | same + `served` segment | hub, served from prebuilt dist (skips activation chain) |
| everything else (`cad-react`, `puzzle3d-react`/`-wgpu`, `puzzle5d-*`, `dag-react`, `procedural3d-*`, `gis2d-wgpu`, `process3d-react`, `fem2d-react`, `fem3d-react`, `energy-react`, plus the `*-attach` and `*-supervised` variants for forms/raster/shooting/remodel/layout/note/wfc-*) | per-plugin `dev <plugin>` / `serve-<plugin>-react-dev` Nx targets, each on its **own** port | single-plugin playgrounds, not the hub |

No `launch.json` entry starts "all plugins at once" — every entry is either the `space` hub alone, or exactly one other plugin's isolated playground.

## 2. Plugin/artifact registration coherence

Registry source of truth: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` (1476 lines, regenerated by `🔄️refresh/🟦️.ts`, validated by `🔎️discovery/🟦️.ts` + `🗿️taxonomy-validation/🟦️.ts` against `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — referenced as a named input at `nx.json:21`). It currently lists **60 entries** (`pluginId`, `packageId`, `cratePath`, `wasmOut`, `capabilities`, `activationEvents`, `hashes`…).

Cross-checked disk vs. registry:
- Disk top-level plugin dirs under `✏️s/🔌️plugins/` (35 dirs): `animate, architect, block, cad, dag, demonstrator, draw, energy, fem, flow, forms, gis, imperative, layout, lowpoly, mathematical, norm, note, playbook, procedural, process, puzzle, raster, reasoning, remodel, sequence, shooting, sourcing, space, stdio, trinity, vcs, wfc, writer, 🗟️artifacts`.
- All 34 of those (everything except `🗟️artifacts`) have a matching top-level `pluginId` in `🔌️plugins.json`.
- The 26 remaining registry entries are `*-extension-*` / `*-module-*` sub-plugins nested inside their parent's `🧩️extensions/` subtree (e.g. `cad-extension-aec-building` → `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust`, `flow-extension-brep` → `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust`, `sourcing-module-beams` → `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust`, etc.) — every one of these `cratePath`s exists on disk; verified by directory listing.

**Discrepancy found:** `✏️s/🔌️plugins/🗟️artifacts` (with subtree `◻️2d/🏅️standards/🔖️1/🪆️subsets`) is **present on disk but has no `pluginId: "artifacts"` entry** in `🔌️plugins.json` (`grep -c '"pluginId": "artifacts"'` → 0 hits), and it has **no `package.json`/`📋️project.json`** at all. This is consistent with it being a *standards/shape namespace* for the `◻️2d` artifact kind rather than an activatable wasm plugin, so it is likely correctly excluded — but it's worth a second look from whoever owns `🗟️artifacts` to confirm it isn't a plugin that never got wired into the registry generator.

No entries were found on the "registered but missing from disk" side — every `cratePath` in `🔌️plugins.json` resolved to a real directory.

**Coherence risk not fully resolved by this audit:** the registry's `activationEvents` field (e.g. `animate`: `["on-artifact-kind:animate.present"]`, `space`: `["on-artifact-kind:space.shome", "on-artifact-kind:space.sspace"]`) is consumed by TS in registry/catalog-verification and test code (`🔨️modules/🔌️plugin/📇️registry/{✅️catalog-verification,🔎️discovery,📽️projection}/🟦️.ts`, `📇️registry/🤖️generated/🧩️plugins/🟦️.ts`), but a search for the literal string `activationEvents`/`on-artifact-kind` in the Rust host (`🖥️host/🎠️activation/🦀️.rs` and siblings) returned **no hits**. That means this audit could not confirm whether the hub actually uses these declarations to lazy-load a plugin's wasm at runtime when an artifact of that kind is opened, vs. the field being catalog/documentation metadata only consumed by build-time tooling. Given the ticket's stated goal ("a working os s frontend with all plugins and artifacts"), this is the single most important open question — see §6 P0.

## 3. Compile/build state

- `bun nx show projects` succeeded (second attempt; the first attempt's redirected output silently lost the payload — see `🗑️generated/os-nx-show-projects.txt` vs the working capture `🗑️generated/os-nx-show-projects2.txt`). **969 total Nx projects** in the graph. Filtering for os-scoped projects (17 hits): `@semio-tech/framework-os`, `@semio-tech/framework-os-config`, `@semio-tech/framework-os-dev`, `@semio-tech/framework-os-host-rs`, `@semio-tech/framework-os-kernel`, `@semio-tech/framework-os-mcp`, `@semio-tech/framework-os-mcp-rs`, `@semio-tech/framework-os-scale-fixture`, `@semio-tech/framework-os-shell`, `@semio-tech/framework-os-shell-rs`, `semio-framework-os-flow`, `semio-framework-os-flow-core`, `semio-framework-os-font-assets`, `semio-framework-os-infinite`, `semio-framework-os-kernel-db`, `semio-framework-os-renderer-wgpu`, `semio-framework-os-run`.
- **No scoped TypeScript config for the os product** was found — `find … -iname "tsconfig*.json"` under `🧰️framework/🛍️products/💻️os` returned only `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/tsconfig.json`. The only workspace-wide config is root `tsconfig.json` (`include: ["**/*.ts", "**/*.tsx", …]`, `strict: true`), which type-checks **all 969 Nx projects** at once. Running `bun tsc --noEmit` against that root config would not be a "cheap" os-scoped check — it's a full-monorepo type-check with no os-specific include filter, and doing so while other agents hold active cargo/rustc builds risked exceeding the 10-minute cap and contending for shared resources. **Skipped**, and flagged as a gap in tooling (§6 P2): there is no `bun ./📜️script.ts <verb>` or Nx target that type-checks just `🧰️framework/🛍️products/💻️os` in isolation.
- **Single os host crate identified**: `semio-framework-os` at `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml:1-2` (`package.metadata.semio.id = "os-host"` at line ~11, `crate-type = ["cdylib", "rlib"]`).
- **`cargo check -p semio-framework-os` was NOT run.** `ps aux` at audit time showed live, CPU-hot cargo/rustc activity from other agents:
  ```
  rustc --crate-name semio_s_plugin_procedural … --target wasm32-wasip2 …   (92.8% CPU)
  …/cargo/build/debug/build/semio-s-artifact-remodel-remodeling/…/out/semio_s_artifact_remodel_remodeling-… --test-threads=4 …   (94.8% CPU)
  ```
  Per the repo's own fine-grain-locking build-dir convention (shared `⚡️cache/cargo/build`) and the task's explicit instruction to skip when a build lock is held, no additional cargo invocation was made. This should be re-run once the shared build dir is quiet.

## 4. Tests

No test suite was executed (time budget + live concurrent cargo activity made even a "quick smoke" risky to run safely alongside other agents' builds). Inventory only, from `bun nx show project @semio-tech/framework-os-dev --json` (`🗑️generated/os-nx-show-project-framework-os-dev.json`, 8.4 MB) and directory listing:

- **Unit/self-test suites** (TS "self test" functions wired into root `📜️script.ts`'s own import list, e.g. `toolJobOwnerFactoryResolutionSelfTests`, `toolJobFactoryProofJoinSelfTests`, `interactivityStoreSyncSelfTests`, `interactivityMountedFrameTransactionSelfTests`, `interactivityShardExecutorSelfTests`, `interactivityMcpHttpTransportSelfTests`, etc. — see `📜️script.ts:3-59`) cover most of the os module surface (plugin retained-commands, store canonical-edit, db io, renderer engine, mcp transport…).
- **`@semio-tech/framework-os-dev` Nx targets relevant to os testing** (names only, from the target list): `test`, `test-quick`, `test-long`, `test-exhaustive`, `catalog-smoke`, plus dozens of `activate-<plugin>-{react,wgpu}-{dev,release}` / `dev-<plugin>-{react,wgpu}-{dev,release}` targets (one pair per playground variant).
- **Dedicated test module dirs** under `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/`: `✅️verification`, `🤝️collaboration` (directly relevant to this ticket's "collaboration" goal — `🟦️.ts` single file, not yet inspected in depth by this audit), `🧹️capability-policy`, `🧹️layering-policy`, `🧹️host-handle-policy`, `🧹️export-path-policy`, `📇️canonical-bootstrap-folder-mirror`, `🧪️ticket-owned-browser-host-staging`, `🧪️source-contract`, `🎬️studio`, `🔬️catalog-smoke`, `🧪️multi-shell-harness`, `🔌️staging-root`, `🏃️execution`, `⚖️parity` (with sub-dirs `🏗️structure`, `🖼️pixels`, `🔬️probe`, `🌐️server-pool`, `📊️report`, `🏃️execution` — this is the wgpu/react pixel+behavior parity harness referenced heavily in the WGPU-RENDERER-REACT-PARITY ticket, §5).
- **Renderer engine contract tests**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (65 debug/unreachable hits, see §5) and `🔬️plugin-runtime/🟦️.tsx` (35 hits).
- **Storybook**: `launch.json` has `storybook-framework-os` (port 6011) and `storybook-framework-hosts` (port 6010) targets, plus `storybook-static`; stories live in `🧰️framework/🛍️products/💻️os/📖️stories/{🎭️plugins,🎭️shell,🎭️wgpu,🧭️coordination}`.
- **Host-side Rust unit tests**: `🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/{🔬️codec-abi-unit,🔬️host-unit,🔬️instance-unit,🔬️media-export-raster-wasip2,🔬️registry-unit,🔬️workflow-standalone,🔬️workflow-unit}`.

## 5. Known-broken / unfinished

### TODO/FIXME/unimplemented!/todo!/[DEBUG]/unreachable! grep

Ran across `🧰️framework/🛍️products/💻️os`, `🧰️framework/📦️packages`, `🧰️framework/🔨️modules/🖱️ui`, `✏️s/🔨️modules`, `✏️s/🧪️tests` (full capture: `🗑️generated/os-todo-fixme-grep.txt`, 2082 raw hits). Breakdown:

| marker | count |
|---|---|
| `TODO` | 3 |
| `FIXME` | 0 |
| `unimplemented!(` | 2 |
| `todo!(` | 1 |
| `[DEBUG]` | 1212 |
| `unreachable!(` | 864 |

The literal `TODO`/`FIXME`/`unimplemented!`/`todo!` count is tiny (6 total, listed in full):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:33` — a documented extension seam, explicitly *not* a TODO (comment says so).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/dist/asset/🎯️concepts/🟦️component.ts:53` — `TODO(follow-up): should be plugin-declared metadata`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-soft-skip/🦀️.rs:4` — matches the string `"TODO"` inside a fixture-classifier, not an actual TODO.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts:16` — `fn new() -> Self { todo!() }` (test stub).
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️element-unit/🦀️.rs:73,94` — two `unimplemented!("not exercised by this test")` in intentionally-partial test doubles.

`[DEBUG]` (1212 hits) is dominated by intentional fault/diagnostic logging (e.g. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:415,428` — `debug_runtime_line(format_args!("[DEBUG] retained command {} faulted…"`), not stray `console.log` debugging — though one ticket (`DEMONSTRATOR-REMOVE-DEBUG-CONSOLE`, ☀️17) exists specifically to strip some of these out of a demonstrator surface, so the convention isn't universally accepted as "keep."

`unreachable!(` (864) is **mostly noise from vendored/generated code**: excluding `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🗑️generated/cargo-plugin-host/debug/build/cranelift-codegen-…/out/isle_*.rs` (a vendored Cranelift codegen build artifact, 267+161+64+60+53+22 = 627 hits by itself) leaves 1454 non-vendored actionable-marker hits (`🗑️generated/os-todo-fixme-grep-nonvendored.txt`; further filtered to the 5 real markers in `🗑️generated/os-actionable-todo-fixme.txt`, 1218 lines).

**Hotspots (non-vendored, by file, `[DEBUG]`+`unreachable!` combined):**
| count | file |
|---|---|
| 100 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` |
| 65 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` |
| 41 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` |
| 41 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` |
| 35 | `…📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` |
| 29 | `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` |
| 29 | `…🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` |
| 27 | `…📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` |
| 26 | `…🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` |
| 26 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔬️unit/🦀️.rs` |
| 25 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` |

Reading these as "hotspots of fragility" rather than "known bugs": the **wgpu Shell/renderer files and their contract/parity tests dominate** — consistent with §5's ticket findings below that wgpu↔react parity is the single largest area of open, tracked breakage in the os frontend right now.

### Recent ticket status (2026-09-1[5-8], os/react/wgpu-relevant)

- **`WGPU-RENDERER-REACT-PARITY`** (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📓️status.md`, currently mid-edit per git status `MM`) — the largest, most active os-frontend effort. Latest waves (W12a-d) landed: pointer-hit-owner/capture model for chrome-vs-pane routing, a `BrushMeshRegistrar` twin, pane-overlay-before-panels ordering. **Open at last write:** no Actions/Search pane body painted on the wgpu renderer yet; W12c chord/camera parity run at only **17/37 match, 0 faults**; cap-close closes two windows on wgpu instead of one; empty-id dock window; ingress latency ~1s behind after 8 pointer moves. Wave 13 (in flight) covers dock close/reopen journals, Actions/Search pane bodies, world journals, and is fanning the same fix out to generation3d/gis2d/flow/note/puzzle2d/draw/layout/forms.
- **`DEV-PROCESS-REACT-E2E`** (☀️15) — reports a **green** end-to-end boot (activate→serve→browser, process3d) after fixing 4 root causes (wrong crate reference in `PluginApp::tool_run_trace_delta`, `UiFixedList::try_push` over-reserving capacity, `child_id` mismatch on 44 assets, primitives posed by corner instead of centre + 7 brep boolean law fixes). **Pre-existing red left as-is**: 32 brep kernel laws (incl. a timeout in `offset_sphere_matches_closed_form`), ~20 ui-contract retained-copy/assembly/retirement laws + 2 ui-runtime canonical-document laws, 37 process3d laws, and **renderer typecheck blocked** by "a peer's unstaged `req` in `FaultScope`" at `🔌️PluginRuntime/🟦️.tsx:3205`.
- **`INPUT-CAUSALITY-LEDGER`** (☀️16) — fixed a UX bug (gesture-originated dispatch failures were wrongly toasted as user-facing notices; fixed via a new `InputOriginV1 = "gesture"` stamp in `renderComponentSceneHost`/`🗣️Interpreter/🟦️.tsx`). In doing so it **surfaced** two pre-existing, still-open process3d faults handed off to the process3d owners: every `setCamera` sync is refused `dispatch-failed` ("validation failed: batched item candidate failed its exact fixed fold contract"), and `worldSelect` is `undeclared-action` on the `process-workpiece` window kind.
- **`ARTIFACT-TREE-VIRTUALISED-STREAMING`** (☀️16) — host-side windowing (`TREE_WINDOW_BODY_NODE_BUDGET = 111`, path-based window identity) landed and passing (Tree 31/31, Interpreter 112/112, ShellHelpers 21/21; F2/A6b/R2 waves also landed with 20-47 laws each). **Open:** still needs a browser re-probe wave (step "e" per the F1 report) once guests are restaged; also flags **known peer breakage**: the fem3d demo swap to "concrete-forest" (commit `0b460ed19f`) broke 42 fem3d window-related tests.
- **`PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D`** (☀️17) — largely green by the last entry: 2d suite progressed 603/268 → 849/29 pass/fail, 5d suite at 548/19, TS typecheck fleet-wide down to 0 diagnostics after 4 fixes. One flagged pre-existing gap: `engine-contract` law file "cannot load (pre-existing missing export `worldSceneContentBoundsKey`)" — same renderer-engine-contract file that tops the grep hotspot table above.

## 6. Prioritized gaps — "working os s frontend with all plugins and artifacts"

**P0 — blocks the stated boot goal**
1. **Confirm/implement runtime multi-plugin activation inside the hub.** The `space` hub plugin's `activationEvents` (`on-artifact-kind:*`) are declared in the registry (`🔌️plugins.json`) and consumed by build/catalog TS (`🔌️plugin/📇️registry/{✅️catalog-verification,🔎️discovery,📽️projection}/🟦️.ts`), but no Rust host consumer of that field was found under `🖥️host/🎠️activation/🦀️.rs`. If the hub can only ever serve the single plugin baked into its Nx `dev`/`activate` chain (as the doc comment at `📜️script.ts:255-265` implies — "every **selected** plugin's component/materialize…"), then "s with all plugins" does not exist as a runnable target today; it would need either (a) the `multi` mode (`📜️script.ts:416-425`) fully implemented and wired to select every registered plugin, or (b) genuine runtime lazy-loading of a plugin's wasm bundle when the hub opens an artifact of its kind. Files to touch: `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts`.
2. **wgpu↔react parity is the dominant blocker for a usable wgpu-backed hub.** Per `WGPU-RENDERER-REACT-PARITY/📓️status.md`, chord/camera parity is at 17/37 with wgpu still missing Actions/Search pane bodies and mishandling dock close/reopen. Files: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `…🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`, plus the wave-13 items already dispatched in that ticket.
3. **Cargo/tsc verification for the os host and dev packages is currently blind.** No scoped os tsconfig exists (root `tsconfig.json` is whole-monorepo), and `cargo check -p semio-framework-os` was never run in this audit due to concurrent build-lock contention — meaning this audit cannot assert the os host crate presently compiles clean. Needs a follow-up run once the shared cargo build dir is quiet: `cargo check -p semio-framework-os --message-format short`.

**P1 — feature/behavior missing but boot isn't blocked**
4. `engine-contract` renderer law file fails to load due to a missing export `worldSceneContentBoundsKey` (flagged independently by `PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D`) — file: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (also the #2 grep hotspot, 65 debug/unreachable hits).
5. process3d camera/selection dispatch is broken at the protocol level (`setCamera` always `dispatch-failed`, `worldSelect` `undeclared-action` on `process-workpiece`) — currently masked as silent `[DEBUG]`/console.error, now visible via the causality ledger but unfixed. Owner: process3d team per `INPUT-CAUSALITY-LEDGER/📓️status.md`.
6. fem3d: 42 window/tree tests broken by the concrete-forest demo swap (commit `0b460ed19f`) — needs reconciliation with the `ARTIFACT-TREE-VIRTUALISED-STREAMING` windowing work.
7. `renderer typecheck` blocked by an in-flight peer edit (`req` field in `FaultScope`, `🔌️PluginRuntime/🟦️.tsx:3205`) per `DEV-PROCESS-REACT-E2E` — re-check once that peer's change lands.
8. 32 brep kernel laws + ~20 ui-contract laws + 37 process3d laws remain red (pre-existing, not introduced by recent work, per `DEV-PROCESS-REACT-E2E`) — these gate a fully-green artifact pipeline even though the boot itself is green.

**P2 — polish / tooling debt**
9. No os-scoped `tsc`/typecheck Nx target or tsconfig exists, forcing any type-check to run against all 969 Nx projects — add a `🧰️framework/🛍️products/💻️os/tsconfig.json` (or an Nx `typecheck` target scoped to the os `sourceRoot`s) so this and future audits can cheaply verify the os surface alone.
10. `[DEBUG]` console logging left in shipped code paths in at least one place flagged by its own ticket (`DEMONSTRATOR-REMOVE-DEBUG-CONSOLE`, ☀️17) — sweep for stray (non-fault-reporting) `[DEBUG]` lines vs. the legitimate `debug_runtime_line`/fault-logging convention.
11. `✏️s/🔌️plugins/🗟️artifacts` has no `package.json`/`📋️project.json` and no registry entry — confirm with its owner whether this is intentional (standards-only namespace) or a plugin that fell out of the registry generator.
12. `bun nx show projects` piped straight to a file lost its payload on the first attempt (silent truncation to the echoed command line only) — worth a quick look at whether the fine-grain-locking bootstrap wrapper (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts`) buffers/forks output in a way that's unsafe to redirect directly; use `| tee` as a workaround (as done in this audit) until then.

## Evidence index (raw captures)

- `🗑️generated/os-nx-show-projects2.txt` — full `bun nx show projects` output (969 projects).
- `🗑️generated/os-nx-show-project-framework-os-dev.json` — full target list for `@semio-tech/framework-os-dev` (8.4 MB).
- `🗑️generated/os-todo-fixme-grep.txt` — raw grep of TODO/FIXME/unimplemented!/todo!/[DEBUG]/unreachable! across the audited dirs (2082 lines).
- `🗑️generated/os-todo-fixme-grep-nonvendored.txt` — same, with vendored cranelift-codegen isle_*.rs excluded (1454 lines).
- `🗑️generated/os-actionable-todo-fixme.txt` — filtered to the 5 real markers, vendored code excluded (1218 lines).
- `🗑️generated/os-todo-fixme-hotspots.txt` — per-file hit counts, all markers, descending.
