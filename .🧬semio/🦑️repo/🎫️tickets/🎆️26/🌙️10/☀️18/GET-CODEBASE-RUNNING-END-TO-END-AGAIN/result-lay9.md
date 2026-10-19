# Result lay9: WP9 Renderer, cycles, deep imports and renderer-host routing

Executor `lay9`. No `package.json` / `bun.lock` edit, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`, no cargo, no git-modifying command, nothing under `✏️s/**`, nothing in `framework/modules/ui`. Paths below are under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` unless stated. Ticket tools: `C/lay9-tools/`.

## 1. Totals (violations whose source is inside `📺️renderer`, same cruiser config and rules as lay0/lay6)

Scoped run `lay6-tools/cruise.sh` (include-only `🧰️framework`, about 4 min), before = `base-r.json`, after = `c4.json`:

| rule | before | after |
|---|---:|---:|
| no-circular | 203 | **0** |
| no-core-path | 2 | 0 (a peer renamed the flow crate path meanwhile) |
| no-cross-package-relative | 1,447 | 1,218 |
| renderer-hosts-only-ui | 161 | 161 (spec, see 4) |
| total | 1,813 | 1,379 |

The ncpr count counts each edge once per package alias (`framework` and `framework-rs` share a source owner), so 1,218 is 812 unique `(importer, target)` edges. It also moved by +18 after my last run because peers split wgpu-reachable framework modules meanwhile.

## 2. Cycles: 203 to 0 (three strongly connected components of 60, 13 and 2 modules)

The 60-module component (`ShellHost` x 24 hooks, `Shell`, `ShellHelpers`, `Interpreter`, `World3dHost`, `ElementHosts`) had only ONE runtime-relevant cause plus type-only back edges; with the runtime edges alone it was 37 modules. Three real cuts removed it, no behavior change:
1. **`ShellFaultBoundary` to its own leaf** `🧱️elements/🐚️Shell/🧯️fault-boundary/🟦️.tsx` (new). Five hosts (`surface-bridge`, `display-views`, `tree-windows`, `panel-hosting`, `canvas-footer`, `ShellHost`) imported the boot module only for this class, so every Interpreter/ShellHelpers module sat below the composition root and the root sat below them. `🐚️Shell/🟦️.tsx` keeps `bootFrameworkOs` and no longer re-exports `SpacePanelState`/`SpawnedAppEntry`; those two types are imported from their owner `🛠️ShellHelpers/📌️panel` by 4 hosts and the react target (`lay9-tools/movesym.ts`, a TypeScript-AST "move symbols between modules" rewriter that updates all importers).
2. **`ServiceMountedViewV1`**: `🏛️ShellHost/🪪️host-bootstrap` imported it through the ShellHost root (`import("../🟦️.tsx")`); it is owned by `🏛️ShellHost/🎚️shell-model`, now imported from there. That one edge closed the whole `ShellCoreAfter*` hook chain.
3. **`UtilityTree` ↔ `🪛️utility-bar-options`**: the tree imported the shell-owned `SelectionUtilityOptions` row. `UtilityTree` now takes `renderSelectionOptions` (new exported `UtilityTreeSelection` type), `utilityBarNode` passes the row. Only `direction="up"` ever rendered it, and only `utilityBarNode` uses that direction.

Not touched: the 13-module `📃️UiDocumentStore/🧪️tests` component (the module registers its in-source tests dynamically, the policy rule `no-circular` explicitly treats `🧪️tests/` as leaves; no cruiser violation).

Consequence found by the suites and fixed in the tests: `Shell` used to load the composition root (host registration at module load) as a side effect of importing `ShellFaultBoundary`. Interpreter-level tests relied on that. `🗣️Interpreter/🧪️tests/🗺️element-host-registry` now imports the composition root `🏛️ShellHost` explicitly (its describe title is "importing a composition root registers the concrete hosts"); `🗣️Interpreter/🧪️tests/🪪️container-node-ids` registers only the `node-graph` host it mounts (`registerComponentSceneHostV1`, retired in `afterEach`). Importing `ElementHosts` there would have re-created a cycle through the in-source test edge (my own scan flagged it).

### New test (language-agnostic, third-party oracle)
`🧪️tests/🔁️acyclic-import-graph/🟦️.ts` + `🧫️fixtures/🔣️.json` (8 portable cases: value pair, type-only back edge, slot augmentation, dynamic edge, diamond, in-source test registry exempt, cycle through a test module still counts, explicit `🟦️.tsx` entries). Own SCC scan agrees with dependency-cruiser's `cruise` API (same `no-circular` rule shape as the policy, `tsPreCompilationDeps`) on every case, then asserts no cycle in the real renderer tree (757+ files). Registered in the react vitest config (`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, `engineSuite("🔁️acyclic-import-graph")`, node environment pragma).
- `bun test ./…/🔁️acyclic-import-graph/🟦️.ts`: 2 pass, 19 expects (also run through the react vitest config via `lay9-tools/runvt.py`: 2 pass).

## 3. Deep imports (no-cross-package-relative)

`lay9-tools/rfix.ts` (TypeScript checker; same method as lay6 `ncpr-fix.ts`, extended to every target and to the host rule) rewrites an import to a bare `@semio-tech/*` specifier only when an EXISTING export entry exposes every imported symbol with the identical alias-resolved declaration. Guards: no entry inside the importer's own package, no entry inside the renderer for a foreign package (it would re-create cycles), no `-rs` wasm packages that vitest stubs by alias, host files only via entries the host rule allows.
- 129 import specifiers in 79 files rewritten (`lay9-tools/rfix-changed.txt`): `framework-compiler-ts/syntax/module-tree` 36, `ui-react` 18, `framework-actor/shard-client` 18, `framework-actor/retained-turn/bootstrap/testing/fixtures` 14, `framework-kernel` 10, `schema-ts/oracles/validator` 7, value, canvas, os-dev, styling, assets and others the rest. No file lists the new package in a nearer `package.json`, none has one (the workspace root applies); `Bun.resolveSync` and vitest resolve all of them.
- **wgpu target excluded and restored.** `targets/🧊️wgpu` bundles through a schema-owned allow list (`📽️projection`: `profile.workspaceImports` binding per bare specifier). The first rewrite broke `🧩️package-integration` ("WGPU browser import is not schema-owned: @semio-tech/assets/mesh …"). I reverted all 45 wgpu rewrites and re-pointed 17 specifiers that peers' module splits had moved meanwhile (`lay9-tools/restale.ts`: resolves the symbol through the old bare entry to its current defining file; 6 files). A bare specifier in the wgpu closure needs the profile binding first.
- Remaining 812 unique edges cannot be fixed without new `exports` (manifest freeze). Spec: `C/lay9-tools/exports-needed.json` (lay6 format, plus `rules`): `byPackage` 29 packages, 347 new entries for 651 importer edges (largest: `framework` 60/195, `framework-os` 117/140 (the react target reaching into elements), `framework-renderer-wgpu` 31/79, `repo-lib` 41/66, `framework-actor` 12/35, `plugin-registry` 11/24). The later manifest batch must add the glue file plus manifest key, then re-run `rfix.ts apply`.

## 4. renderer-hosts-only-ui: 161 edges, 0 fixable without new exports

The rule lets a host (`🎯️targets/⚛️react`, `🧱️elements`) resolve only into the host root, `framework/modules/ui`, `framework/📦️packages` (the `@semio-tech/framework` glue) and react/vite. I searched every non-wildcard export of every workspace package that resolves into the allowed set for the identical symbol: none exists for the 161 edges (63 unique targets). Spec in `exports-needed.json` under `uiContract`: 63 new `@semio-tech/framework` entries (glue under `🧰️framework/📦️packages/🟦️typescript/`), 161 importer edges. Largest groups: os modules (47, mostly `infinite/world/r3f`, 17, a real host-to-product dependency that needs a ui port rather than an export), actor `shard-client` (33), actor packages (14), `wire-turn` (10), manifest `ui-contract-and-canvas-types` (6), repo process (7). 14 edges are renderer-internal: `🎬️media/{lifecycle,channel}` (4), wgpu `♿️accessibility-mirror` (3), the react `📜️script.ts` oracle (1), and 6 shared test-fixture JSON files under `🧪️tests/🧫️fixtures` that Rust tests also `include_str!`, so they were not moved (no cargo to prove it).

## 5. Verification (exact commands, outcome)

All from `/Users/ueli/Documents/semio`, foreground (the long vitest runs were detached from the shell and polled).
- Cruise before/after: `zsh C/lay6-tools/cruise.sh 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer C/🗑️generated/lay9-run/c4.json 🧰️framework` and `bun C/lay9-tools/agg.ts`: table in 1. Fast iteration scan `bun C/lay9-tools/quickscc.ts all` (own SCC scan, reproduced the cruiser's 60/13/2 before): now only the exempt 13.
- tsc: `NODE_OPTIONS=--max-old-space-size=8192 node node_modules/typescript/bin/tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json --pretty false`. Baseline run after the cycle edits: 35 errors, all environment (missing `@dnd-kit/*`, `react-resizable-panels`, `katex`, `jsdom`/`pngjs` types, `@webgpu/types`) or unrelated peers; identical set after the 129 rewrites. Last run: the same 35 plus errors only in files I do not own (`store/🪆️child` and the `flow-core` rename a peer is mid-way through); zero in files I touched.
- Renderer vitest (react config, `SEMIO_TEST_LEVEL=long`, runner `lay9-tools/runvt.py`): as shipped 64 of 223 files pass, 147 fail at import on the missing `@dnd-kit/*` (environment). With a throwaway alias config that stubs only the four missing packages (`lay9-tools/vt-stubs`, copied next to the real config for the run and deleted): **198 of 223 files, 3,135 of 3,186 tests pass**. All remaining failures are independent of my files: 16 Playwright browser binary missing, 2 `tailwindcss` missing, 1 katex math (my stub), 1 dnd-kit geometry (my stub), 2 `resolvePluginLoadOrder` (the `@semio-tech/framework` entry lacks it, test untouched), 1 Rust-source assertion `FrameLatencyStage::SnapshotPublish`, 2+2 wgpu bundle "not schema-owned" for `replication/…` and `actor/shard-client/ui-patch` (peers' splits, profile not yet updated), 1 label discriminators. There is no pre-change baseline run (no stash/worktree allowed); the argument is the failure classification above plus the two real regressions the suites did find and I fixed (wgpu allow list, host registration).
- wgpu vitest config (same stub config): 43 of 56 files pass; the 13 failing are Playwright, source-text assertions over peers' Rust/TS splits (`wgpu-u64-seam`, frame-latency), `tree-sitter-wasms` missing and the same `resolvePluginLoadOrder`.
- bun suites: `bun test` over the 14 renderer files importing `bun:test`: 132 pass, 7 fail (3 Playwright launch, `tree-sitter-wasms`, harness env `SEMIO_TEST_ARTIFACT_DIR` / script invocation).

## 6. File list

New: `🧱️elements/🐚️Shell/🧯️fault-boundary/🟦️.tsx`; `🧪️tests/🔁️acyclic-import-graph/🟦️.ts`; `🧪️tests/🔁️acyclic-import-graph/🧫️fixtures/🔣️.json`.
Edited (cycles): `🧱️elements/🐚️Shell/🟦️.tsx`; `🏛️ShellHost/{🪪️host-bootstrap,🧩️shell-core-state,🎮️focus-and-input,🛫️spawned-session,🖼️canvas-footer,📋️panel-hosting,🟦️,🔌️extension-runtime}`; `🗣️Interpreter/{🔌️surface-bridge,🔤️display-views}`; `🛠️ShellHelpers/🌳️tree-windows`; `🛠️ShellHelpers/🪟️window-panes/🪛️utility-bar-options`; `🎛️UtilityTree`; `🎯️targets/⚛️react/🗣️interpreter-and-shell`.
Edited (tests/config): `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (one `engineSuite` line); `🗣️Interpreter/🧪️tests/{🗺️element-host-registry,🪪️container-node-ids}`.
Edited (deep imports): the 79 files in `C/lay9-tools/rfix-changed.txt`; wgpu `🌐️server/{🟦️,🎚️config/🟦️}`, `🐚️plugin-bridge/{🧵️shard-and-actor-calls,🍽️ui-intake-and-route,🔌️plugin-module-loader}`, `🧩️plugin-modules` (re-pointed to current owners).
Ticket tools kept in `C/lay9-tools/` (scripts, `exports-needed.json`, `rfix-changed.txt`, `vt-stubs`). Generated output (`C/🗑️generated/lay9-run`, big JSON reports, `rfix-applied.json`, `rfix-final.json`) deleted. The temporary `lay9-validate.config.ts` copies next to both vitest configs were deleted.

## 7. Open

- Manifest batch: the 347 `byPackage` + 63 `uiContract` entries in `exports-needed.json`, then `bun C/lay9-tools/rfix.ts <fresh cruise json> apply <out>` (wgpu files additionally need `profile.workspaceImports` bindings first).
- `infinite/world/r3f` (17 host edges) needs a ui port, not an export.
- Peers' in-flight splits currently leave `tsc` (`store/🪆️child`, `flow-core`) and the wgpu browser allow list red; not caused by this work.
- `UiDocumentStore/🧪️tests` 13-module self-registering component is policy-exempt but still a real cycle if the exemption is ever tightened.
