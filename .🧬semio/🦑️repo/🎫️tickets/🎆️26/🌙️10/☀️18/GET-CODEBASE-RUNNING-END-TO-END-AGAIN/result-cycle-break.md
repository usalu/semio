# Result cycle-break (2026-10-19 evening to 2026-10-20 05:40)

Executor `cycle-break`, ticket C = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.

State of the work:

- **Applied before the freeze (code only, no manifest/library/taxonomy)**: the "direct parts" d1, d1b, d3, d5, d5b, d6, d7 (92 files touched) and the first typed asset modules.
- **Staged, not applied**: everything that touches a `package.json`, `📚️library/`, `🔣️taxonomy.json`, a fixture under `📚️library/`, or depends on one of those. It is one merged delta, `C/cycle-break-delta.json` (471 manifest ops, 16 new files, 1144 edits in 535 files, 243 file ops), with the applier `C/cycle-break-tools/apply-delta.ts`.
- **TOTAL WRITE PAUSE honoured**: since the pause message nothing outside C was written. The guard pass of the applier against the live tree (read-only) is clean.

## 1. Decisions

| item | decision |
| --- | --- |
| package cycles | A shared contract sits below both sides; each side points at the lower package. 15 new module-root packages (module dir = package dir, exports point straight at source, no glue) and 9 existing packages re-owned (23 carve specs in `carves.json`, `puzzle-5d-ts` comes from `w1`). The declared production graph (`dependencies`, peers, optionals) ends with a **single** cyclic pair (`print` and `print-viz-inference`). |
| `ui-no-framework-packages` | The ui may depend on framework **foundation** only, and the foundation is a closed set below the ui. No glue form is admitted. The foundation is admitted by path (`ownerPaths` of the `ui-foundation` role), because the alternative (`semio.dependencyRole: "ui-foundation"` in the package manifest) makes the whole module a foundation: for `manifest` that is 8 closure violations (manifest to schema-ts, effect, io-schema; value-ts to hash, async, schema-ts). Path-level admission has 0. |
| `?raw` / `?url` | Typed asset modules inside the owning package (`export const xText`, `/// <reference types="vite/client" />`), published as `…/testing/source-text/…` or `…/examples/…` keys. Works unchanged in bun and vite. |
| wgpu and plugin-registry | The production direction is wgpu to registry. The registry reaches wgpu only from its composition suite (tests), which is a devDependency and no longer enters the cycle graph. The exports-stage cycle tooling now checks the production graph only. |

### 1.1 Why the foundation is module-root packages

`@semio-tech/framework` is one mega package whose keys point at glue files in `🧰️framework/📦️packages/🟦️typescript/<module>`. Every glue path is a different owner than the module source, and the cruiser policy only admits the module paths. A module-root package makes the resolved path equal to the source path, so no policy form has to be invented and the glue layer disappears for the carved modules (the delta deletes 236 files, almost all of them glue).

### 1.2 Policy encoding (staged in the taxonomy part of the delta)

`dependencyDirections.roles` in `🔣️taxonomy.json`, edited structurally (`json` edit mode, sorted lists stay sorted):

- `ui-foundation.ownerPaths`:
  - removed: 3 stale glue forms (`📦️packages/🟦️typescript/🫧️transient`, `…/🕹️interaction`, `…/🛂️manifest/🤖️generated`);
  - retargeted: `🛂️manifest/🗺️window-layout` to `🎓️onboarding/🗺️window-layout` (moved by d5b);
  - added: `🛂️manifest/📋️context-menu` (moved by d5b), `🔏️hash` (value-ts depends on it).
- `ui-test-support.ownerPaths`: added `🏃️process/…/🏗️build/🎟️capacity` and `…/🎛️execution`. The ui tests `python-package-commands` and `rust-package-commands` import them, a drift that predates the carve.
- The `wgpu-frame-worker` browser profile: 7 `workspaceImports` keys re-keyed to the carved packages (new `manifestPath`), `sourceModulePaths` and `inputPatterns` follow the moved sources (duplicates merged).

File-level evaluation with the live rule definitions (`C/cycle-break-tools/ui-policy.ts`, current tree vs. delta plus mirrored taxonomy):

| | `ui-no-framework-packages` | `ui-foundation-closure` |
| --- | --- | --- |
| now (after the direct parts, glue resolved) | 10 edges / 4 targets | 3 edges / 2 targets |
| after the delta | **0** | **0** |

## 2. Target package DAG

Declared production graph after the delta (`verify-delta.ts --dag`, ranks = longest path to a leaf; the one cyclic pair is condensed):

| rank | packages (carved or re-owned ones named) |
| --- | --- |
| 0 | 172 packages: `framework-hash`, `framework-trace`, `framework-transient`, `framework-filesystem`, `framework-interaction`, `framework-onboarding`, `framework-effect`, `plugin-registry-publication`, `ui-viewport-ts`, `repo-test`, … |
| 1 | 30 packages: `framework-manifest` (to effect, onboarding), `ui-styling`, `repo-server-library`, `plugin-registry`, `nx-bootstrap-tools`, … |
| 2 | 9: `framework-action-argument-resolution` (to manifest), `repo-coordinator` (to server-library) |
| 3 | 5: `ui-contract` (to manifest, value-ts) |
| 4 | 2: `ui` (to action-argument-resolution, interaction, manifest, transient, schema-validator, ui-contract) |
| 5 | 5: `ui-react` (to ui, manifest, onboarding, interaction, transient), `ui-locale-rs`, `repo-lib` |
| 6 to 11 | os, plugin, renderers, os-dev, `repo-product` (rank 8) and the products |

Cycles that were closed (each: contract moved down, importers re-pointed):

| cycle | contract (lower package) | how |
| --- | --- | --- |
| ui-react and framework | `ui` owns appearance and i18n (d1); `ui-contract` (retained scene, wire, resident), `framework-manifest`, `framework-onboarding`, `framework-interaction`, `framework-transient` below it | d1, d5, d5b, d7 plus the foundation carve |
| kernel to framework | `framework-effect` (spawned-job, video-render-program) | d5 plus carve |
| actor and kernel | the shard client no longer re-exports the kernel return content | d6 (+ fix6 for the test kit) |
| manifest and ui | action-arg facets live in `ui-contract` (d7); window-layout in onboarding, context-menu in manifest (d5b) | d5b, d7 |
| repo-coordinator and framework | `repo-server-library` (authentication, event, persistence, source-index, ports) | d3 plus carve |
| repo-lib to plugin-registry | `plugin-registry-publication` (leaf) | carve |
| os to os-dev | broker client moved to os-dev | w2d |
| repo-lib and repo-product | dashboard installer moved into the library bootstrap | w2e |
| repo-product (new) | `repo-product` owns the dashboard coverage oracle (blocked export 2 of 3) | carve |

### 2.1 What is still cyclic (honest ledger)

1. **Declared production graph: one pair.** `print` and `print-viz-inference`: 38 edges inference to print (schema snapshot/chart types), 59 edges print to inference (31 are `print` tests, the rest host/io adapters). Proposed fix: extract the chart snapshot schema into a `print-chart-contract` leaf and move the six host/io inference adapters into the inference package.
2. **Source-level ring (non-test import edges, not declared).** `sim.ts` finds one component of 14 packages: repo-lib, nx-bootstrap-tools, repo-product, plugin, plugin-registry, plugin-extension-store, os, os-dev, os-mcp, os-node-graph, renderer-react, renderer-wgpu, print, print-viz-inference. Feedback edges against the layering in `layers.json`: 71 files.

| edges (files) | meaning | proposed fix |
| --- | --- | --- |
| repo-lib to plugin 15 edges, os 8, os-mcp 3, print 1 | product-aware commands inside the build library: `commands/verify`, `release`, `os`, `dev`, `setup`, `laws/tool-job-coverage`, cargo/rust-evidence import product test helpers | move those commands into a `repo-product-commands` package above both; the root script registers them (needs the root `📜️script.ts`, frozen) |
| nx-bootstrap-tools and repo-lib (82 and 31) | accepted closure knot: the CLI closure must stay relative, so the package split cannot be expressed in imports | split recipe from code inside the bootstrap tools |
| plugin to registry 13 files, os to plugin 15, os to node-graph 3; plugin, registry, renderer-wgpu and os to os-dev 8 | os-dev composes everything | os-dev is the composition root: replace the upward imports of os-dev by injected ports |
| registry composition suite to wgpu / renderer-react / os-dev | tests inside the registry package | relocate to a `plugin-registry-composition-tests` package above all three (devDependencies only today) |

These need the upper products to agree on the composition-root rule and the command split; none of them is a missing shared contract. They are the next wave, not blocked by this delta.

## 3. Changes

### 3.1 Applied (direct parts, before the freeze)

Full file lists are in `C/cycle-break-tools/delta-parts/d*.json` (backups under `C/🗑️generated/cycle-break-x/applied-d*`, to be deleted with the generated folder).

| part | change |
| --- | --- |
| d1 (28 edits, 20 files, 7 moves) | react-target appearance and i18n moved into `ui`; ui-react re-exports |
| d1b (4 edits) | ui self-imports |
| d3 (2 edits, 1 move) | `coordinator/🔌️ports` moved to `server/📚️library/🔌️ports` |
| d5 and d5b (50 and 39 edits, 8 moves) | contract extraction: spawned-job and video-render-program into `kernel/⚡️effect`; final layering: window-layout into onboarding, context-menu into manifest, `ui-contract-and-canvas-types` stays in manifest. d5 first put them into ui-contract and broke the relative imports of the gitignored `🤖️generated` types; d5b reverted that |
| d6 (6 edits) | the shard client stops re-exporting the kernel return content; consumers import it from the kernel |
| d7 (5 edits, 1 new) | `🖱️ui/🧬️contract/🎛️action-arg-facets` |
| typed assets (new files) | 7 puzzle example DSL modules (2d, 3d, 5d), emblem url, ui-styling palette and chrome text, wgpu host-io and renderer-boot, node-graph wasm graph surface |

### 3.2 Staged (`C/cycle-break-delta.json`, schema `cycle-break/delta@1`)

| part | content |
| --- | --- |
| `carves` | 23 carve specs (14 new manifests, 9 existing packages re-owned), 431 manifest ops, 1071 edits, 232 file deletions |
| `w1` | typed asset exports and rewrites of the remaining `?raw`/`?url` imports (15 original); the ui i18n source text moves from the react target into `ui` (`@semio-tech/ui/i18n/source-text`); the ui test fixture loaders move to `ui-contract/testing/source-text/retained-fixtures` (new typed module) |
| `w2a` to `w2e` | publication leaf, server library, repo-product, broker client, dashboard installer |
| `extras` | 20 stale declaration removals, restricted to edges inside a declared cyclic component |
| `refs` | 17 edits: taxonomy (JSON ops, section 1.2), retention schema, resolved-boundary fixture (glue-form row dropped), workspace-exports fixture, repo-source-ownership fixture, manifestless-source-closure fixture (4 aliases), wgpu `📋️project.json` inputs (9) |
| `fix6` | the settlement test kit still destructured `OwnedKernelReturnContent` from the shard client (found by tsc on d6; see section 5) |

New packages (15): `plugin-registry-publication`, `repo-server-library`, `repo-product`, `framework-hash`, `framework-trace`, `framework-interaction`, `framework-transient`, `framework-filesystem`, `framework-onboarding`, `framework-manifest`, `framework-action-argument-resolution`, `framework-effect`, `ui-contract`, `ui`, `puzzle-5d-ts`. Workspace entries added in the 5 product roots and the root (84 ops).

Tool changes outside the delta: `exports-stage-tools/{deps,cycles,validate}.ts` check the production graph only (dev edges are not cycles).

### 3.3 Reconciliation with the exports-stage batch

The batch (`exports-stage-tools/batch.json`) is applied live: 405 of its adds are present, 20 removals done. The delta moves 145 of those keys to the carved packages and deletes their 145 glue files (`C/cycle-break-supersedes.json`: entry id, old key, new key). No unapplied batch add collides with a delta removal. After the delta: lock, sync, then re-run `exports-stage-tools/stage.ts` for the residue.

Blocked export entries of the batch:

| entry | status |
| --- | --- |
| `framework ./products/repo/modules/dashboard/testing/coverage/oracle` | solved: `@semio-tech/repo-product` owns it |
| `repo-lib ./build/script` | still waits for tax-stage; re-stage after it |
| `energy-js ./extensions/epw/testing/unit` | open: the glue itself violates `plugin-no-extension-or-artifact-energy`; the two importers belong in the extension, not the core package |

## 4. Apply procedure (after "PAUSE LIFTED" and after the sync has exited)

1. `cd /Users/ueli/Documents/semio && bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/cycle-break-tools/apply-delta.ts` (guard pass: sha256 of every moved or deleted file, exact anchors, round-trip of every manifest and JSON file; refuses on any drift).
2. The same command with `--write` (one pass, backups under `C/🗑️generated/cycle-break-x/applied-delta-*`).
3. `bun repo dependencies lock` then sync (new workspace members must be linked). The coordinator runs this window.
4. `bun .../exports-stage-tools/stage.ts` and apply what is new (superseded entries are in `cycle-break-supersedes.json`).
5. Verify: `bun build --target=bun 📜️script.ts`, tsc with `cycle-break-tools/tsconfig.d2.json` and `…d6.json`, the per-area cruise (course, print, mit-bestand, hub, ui, framework), `bun test ./…/apply-json-ops.test.ts`, `bun test` of the resolved-boundary, workspace-exports, repo-source-ownership and manifestless-source-closure law files (explicit `./` paths, at most 2 at a time).

If tax-stage lands first, the taxonomy and fixture edits still apply: they are structural (key, path or row based), not line based. Register the set in `staged-merge-tools/sets.json` with `apply-delta.ts` as its entry point (dry-run default).

## 5. Verification

| command (run from the repo root) | outcome |
| --- | --- |
| `bun C/cycle-break-tools/merge.ts` | 471 manifest ops, 16 new files, 1144 edits, 243 file ops, 0 blockers; 12 notes (CLI-closure files keep their relative imports) |
| `bun C/cycle-break-tools/verify-delta.ts --dag` | scratch mirror of 825 files; 1036 specifiers resolve to existing targets, 2805 imported names exist at the target, 0 dead glue importers, 0 problems; declared production components: 1 (`print`, `print-viz-inference`) |
| `bun C/cycle-break-tools/apply-delta.ts` (guard only, live tree) | clean, 846 files |
| `bun C/cycle-break-tools/ui-policy.ts --post --mirror` | 0 and 0 violations (was 10 and 3) |
| `bun C/cycle-break-tools/raw-cross.ts` | 22 cross-package `?raw`/`?url` edges now; 0 not covered by a staged edit |
| `bun test ./…/cycle-break-tools/apply-json-ops.test.ts` | 7 pass (fixture corpus `fixtures/json-ops.json`, formatting cross-checked against Python's `json` module) |
| `bun build --target=bun 📜️script.ts` | bundles, 2641 modules (CLI closure intact after the direct parts) |
| `tsc -p cycle-break-tools/tsconfig.d2.json` | clean |
| `tsc -p cycle-break-tools/tsconfig.d6.json` | 2 error classes, both understood: (a) `?raw` JSON loaders in `ui/🧪️tests/🧫️fixtures` (the tsconfig has no `vite/client` types; staged `w1` moves them behind a typed module); (b) `OwnedKernelReturnContent` in the settlement test kit, a consumer d6 missed, staged as `fix6` |
| `bun C/cycle-break-tools/sim.ts carves.json` | 14-package source ring remains (section 2.1) |

Not run: the dependency-cruiser per-area cruise (it reads the live manifests and the new packages are not linked; `ui-policy.ts` and the exports-stage evaluator are the stand-ins, the latter calibrated against the cruise earlier), tsc or tests over the staged edits (the staged edits touch frozen files; the mirror holds only touched files), vite bundles, cargo, any product activation.

## 6. Open items

1. Print: chart contract extraction (section 2.1, item 1).
2. Repo-lib product-aware commands, os-dev composition root, registry composition suite relocation, nx-bootstrap recipe/code split (section 2.1, item 2).
3. `energy-js ./extensions/epw/testing/unit` and `repo-lib ./build/script` (section 3.3).
4. A `semio.dependencyRole` convention for new module-root packages is deliberately not used for foundation modules (section 1); the taxonomy `ownerPaths` list is the single place that grows with the foundation.

## 7. Kept inputs

`C/cycle-break-tools/` (planners `plan-*.ts`, `carve.ts`, `apply-plan.ts`, `apply-delta.ts`, `verify-delta.ts`, `merge.ts`, evaluators `ui-policy.ts`, `ui-boundary.ts`, `raw-cross.ts`, `sim.ts`, `layers.ts`, reference mapping `refs-map.ts`, `ref-sites.ts`, `orphans.ts`, `supersedes.ts`, `delta-parts/*.json`, `carves.json`, `layers.json`, `tsconfig.d2.json`, `tsconfig.d6.json`, the json-ops test and fixture), `C/cycle-break-delta.json`, `C/cycle-break-supersedes.json`. The scratch mirror and generated output (`C/🗑️generated/cycle-break-x`, the scan caches in `C/🗑️generated/exports-stage`) were deleted.
