# Result lw5b: lw5 leftovers (policy false positives, stdio-docx, glb ownership, hub-compositions lock error)

Executor `lw5b`. No `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths, no cargo, no git-modifying command (the glb move is a plain filesystem `mv`).

## 1. `no-state-outside-os` false positives: fixed in the policy

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, rule `no-state-outside-os`: `to` gained `dependencyTypesNot: ["aliased-workspace"]` (same shape as `crossTechnologyRules`). A deep relative import of `🏪️store`/`🖥️host` stays forbidden; the public `@semio-tech/framework-os/modules/store/worker` alias resolves to the same path and is now allowed.
- Test (language-agnostic corpus, checked against our direct regex evaluation AND dependency-cruiser's `validateDependency`): two new cases in `…/🏗️construction/🧪️tests/🚫️resolved-boundary-classification/🧫️fixtures/🔣️.json`: `state-deep-relative-store-import-is-forbidden` (types `["local"]`, forbidden) and `state-public-workspace-alias-store-import-is-allowed` (types `["local","aliased-workspace"]`, allowed).
- Evidence: `bun test …/🚫️resolved-boundary-classification/🟦️.ts` -> 4 pass, 0 fail, 145 expects (the whole-policy `assertRuleSetValid` test included). Scoped dependency-cruiser run (lay0 config) on the three flagged files (gis `worker`, gis `tests/dependencies`, `service-composition/worker`): 0 violations (lw5 reported 3).
- Note: the `.cjs` also carries unrelated concurrent peer edits (`vendoredPaths`, `vendoredFiles`); I touched only the one rule.

## 2. `@semio-tech/stdio-docx`: convention is the built `dist`, no manifest change

- 36 of the 38 stdio artifact packages export `.` as `{types: ./dist/🟦️.d.ts, import: ./dist/🟦️.js}`; only `stdio-inventory-ts` exports source. docx follows the majority and is identical in shape to xlsx/pptx/zip. Its root source `📜️docx/🟦️.ts` exists; `dist` is a gitignored build output (`:build` target, bundles `📜️docx/🟦️.ts`, emits declarations, stages and binds into the package).
- docx and inventory-ts are the only packages without `dist/🟦️.js`; every other one is materialised. So the defect is a missing build output, not a wrong export. Switching docx to a source entry would deviate from 36 siblings. Fix: run `bun repo run @semio-tech/stdio-docx:build` (I did not run it: blocked by section 4; any `bun repo run` that projects the dependency store hits the same lock drift).
- Recorded in `lw5b-tools/exports-needed.json` under `noChange`.

## 3. glb ownership: the framework owns it; mit-bestand does not

Decision (asset has exactly one owner, at the lowest common owner of its consumers, per the taxonomy's lowest-common-owner rule and the `capsule_J.glb` precedent in the same module):
- Consumers: the framework mesh delivery catalog (`source` + its Vite serve/build plugin and the playground stub test, which does `existsSync(resolve(repoRoot, source))`), the s/dev block story, and the s plugin examples (via the public `/mesh/…` url). mit-bestand (the top layer) has no consumer. A framework catalog and framework test depending on a file inside a product is a layer inversion.
- Moved (plain `mv`, names unchanged to avoid new emoji admission): `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/{◀️hexagonal-cut-concrete-forest-left,▶️hexagonal-cut-concrete-forest-right}.glb` -> `🧰️framework/🔨️modules/🖼️assets/🏚️abbau-aufbau/` (mirrors `🌱️metabolism`: source dir name equals delivery `output` dir; delivery `path` and public urls are unchanged). The `.stp`/`.3dm`/`.gh` authoring sources stay in mit-bestand.
- `🥽️mesh/📇️catalog.json`: the two `source` values now point at the framework location.
- New entry module (mirrors `capsule-j`): `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🏚️abbau-aufbau/🧊️hexagonal-cut-concrete-forest-left/🟦️.ts` exporting `hexagonalCutConcreteForestLeftGlbUrl`.
- `✏️s/🧑‍💻dev/🧱️block/📖️stories/🧭️coordination/🟦️.ts`: the `../../♻️mit-bestand/…glb?url` import (cross-technology) is now an import of that entry module (relative for now) and the `BLOCK_STORY_MESH_SOURCES` key is the new `source`. The needed `@semio-tech/assets` export (`./abbau-aufbau/hexagonal-cut-concrete-forest-left`) with the exact importer rewrite is in `lw5b-tools/exports-needed.json` for the manifest batch (net edge count unchanged: one mit-bestand edge replaced by one assets edge the batch rewrites).
- Doc comments naming the old glb location updated in `…/🔺️stl/…/🔺️mutate-stl-ascii/🥒️.feature` and `…/📐️step/…/🚪️io/📐️geometry/🧪️tests/🔬️unit/🦀️.rs` (one string each).
- Evidence: all 93 catalog entries resolve and their `source` exists (`resolveMeshAsset` + `existsSync` over `MESH_DELIVERY_CATALOG`: 0 missing); `bun test …/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts` 3 pass, 33 expects; TypeScript API check of the story and the new entry module (root `tsconfig.json` options): 0 diagnostics. The Rust mesh unit test uses a self-contained fixture (no `source` of these entries), not run (no cargo).

## 4. `bun repo run @semio-tech/hub-compositions:typescript`: root cause is `bun.lock` drift, not the new target

The error is thrown in `…/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` line 152 (`project()` over `bun.lock`): every locked `@workspace:<path>` package must map to an authored root `workspaces` member; otherwise `Locked workspace package lacks original admitted Source`.
- Comparing root `package.json` `workspaces` (120) with `bun.lock`: exactly two locked paths are not members, exactly two members are not locked. A rename `🌊️flow/🫀️core/…` -> `🌊️flow/🎁️browser-package/…` (banned-stem `core` rename) reached `package.json` but not `bun.lock`:
  - `…/🌊️flow/🫀️core/📦️packages/🦀️rust` -> `…/🌊️flow/🎁️browser-package/📦️packages/🦀️rust` (`@semio-tech/flow-core-build`)
  - `…/🌊️flow/🫀️core/🕸️bindings` -> `…/🌊️flow/🎁️browser-package/🕸️bindings` (`@semio-tech/flow-core`)
- All other 120 workspaces' dependency maps match `package.json` exactly (checked deps/dev/peer/optional). So the fix is four path substitutions in `bun.lock` (2 `workspaces` keys near lines 1444/1451, 2 `packages` ids near 6411/6414) or `bun install --lockfile-only`. Recorded in `lw5b-tools/dependencies-needed.json` for the deps owner (frozen for me).
- The new `typescript` target on `🌎️hub/🧩️compositions` is not the cause: `@semio-tech/hub-compositions` is a project, not a package, so it is not part of the lock. Because the check is global, expect every `bun repo run` that materialises dependencies to fail until the lock is synced, including the docx build in section 2.

## 5. Ticket tools (inputs, kept)
`lw5b-tools/exports-needed.json`, `lw5b-tools/dependencies-needed.json`. Generated output deleted.
