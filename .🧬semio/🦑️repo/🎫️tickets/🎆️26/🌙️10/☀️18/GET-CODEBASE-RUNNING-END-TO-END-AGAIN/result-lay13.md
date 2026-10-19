# Result lay13: dependency-cruiser re-measure, deep-import rewrite, residual spec

Executor `lay13`. No `package.json` / `bun.lock` edit, nothing under `🔧️tools/🏗️bazel/**` or the nx tool dir, no cargo, no modifying git command, no sub-agent. C = this ticket folder, tools in `C/lay13-tools/`.

## 1. Measurement method

A whole-repo cruise (`🧰️framework ✏️s 🌎️hub ♻️mit-bestand`) does not finish: 590 s timeout (CPU-starved at load 40-80), a detached 57 min run also did not. Even `♻️mit-bestand` alone timed out. So the measurement is per area (`lay13-tools/cruise-area.sh`, `area-config.cjs`): the policy of `🟨️.cjs` plus one extra `doNotFollow` clause `^(?!(<area>)(/|$))`, so edges leaving the area are recorded but their targets are not expanded. 12 areas (mit-bestand, hub, repo product, OS renderer, OS rest, framework modules, framework packages+small products, s plugins in 4 slices, s modules/dev/tests), merged and de-duplicated by (rule, from, to, cycle) with `lay13-tools/merge.ts`. Limit: a cycle that runs through two areas is not seen. Same method before and after, so the deltas are valid; the absolute totals are not comparable to the 5,469 of the 23:30 whole-repo figure.

## 2. Before / after (same method, error severity)

| rule | before | after |
|---|---:|---:|
| no-cross-package-relative | 2,696 | 1,688 |
| renderer-hosts-only-ui | 220 | 215 |
| plugins-framework-sdk-only | 106 | 97 |
| no-cross-technology | 61 | 37 |
| no-circular | 34 | 27 |
| framework-no-products | 28 | 28 |
| plugin-no-extension-or-artifact | 21 | 21 |
| ui-no-framework-packages | 6 | 6 |
| no-core-path | 4 | 4 |
| ui-foundation-closure | 2 | 2 |
| no-state-outside-os | 1 | 1 |
| **total** | **3,179** | **2,126** |

By owner after (rule :: source area, before to after): ncpr :: framework modules 743 to 635; ncpr :: OS renderer 806 to 363; ncpr :: repo product 443 to 322; ncpr :: course 120 to 120; ncpr :: s plugins 168 to 101; ncpr :: OS (non-renderer) 265 to 73; ncpr :: hub 55 to 3; ncpr :: framework packages 50 to 43; ncpr :: s dev 36 to 25; renderer-hosts-only-ui :: renderer 220 to 215; plugins-framework-sdk-only :: s plugins 106 to 97; no-cross-technology :: s plugins 48 to 28; framework-no-products :: framework packages 25 (unchanged, misplaced glue, section 5); no-circular :: framework modules 24 to 22 (34 total to 27).

## 3. Code changes

### 3.1 Deep imports re-pointed to existing public entries (1,017 imports in 407 files)

`lay13-tools/ncpr-all.ts` (TypeScript AST, no program): for each `no-cross-package-relative-*` edge it resolves the live exports of the target package and its source-owner siblings and accepts an entry only when every imported name has the identical deep origin (following `export {a as b} from`, `export *`, `export type`; a type-only re-export is rejected for a value origin). Specifier: bare `@semio-tech/...` outside the CLI closure; relative path to the glue file for closure files, `📜️script.ts`, `.mjs/.cjs`, config and bootstrap files. Applied 1,274 first (501 files), then reverted 257 (section 3.2). File list of what stays: `lay13-tools/ncpr-changed.txt`; exact edits: `lay13-tools/ncpr-plan-applied.json` (minus `reverted-regress.json` and `reverted-closure.json`). No compat re-export was added.

### 3.2 Reverted rewrites (regression guard, `revert-regress.ts`, `revert-closure.ts`)

The rewrite was measured, and rewrites that traded one violation for another were removed:
- 136 rewrites: 116 `renderer-hosts-only-ui` (renderer react host or elements importing the OS package glue, whose resolved path is not in the allow-list) and 20 `ui-no-framework-packages` (ui to `@semio-tech/framework` glue). Evaluated with the policy's own rule objects on the old and new target.
- 38 rewrites that created import cycles (SCC check on the cruise graphs: glue barrels of repo-lib `storage`/`artifacts` aggregate library files that import the nested bootstrap tools back).
- 84 rewrites inside the zero-touch CLI closure (coordinator message): every bare specifier and every repo-lib barrel glue import in a file reachable from `🏗️build/📜️script.ts` was set back to the original relative path. Scanner `bun C/scan-bare-imports.ts 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts @semio-tech/ typescript` reports files 419, bare {} (closure 457 to 419 files).
- Revert bug found and fixed on the way: the string revert replaced the first occurrence of the replacement text, which swapped two imports of `🧪️test/🟨️.mjs` and two of `🎛️dashboard/📦️installation/🟦️.ts` (the latter pre-existing identical text). Repaired (`repair-reverts.ts`, `hazard.ts`: only these two files were affected, both verified against `git show HEAD`).

### 3.3 Cycles and glue fixed at the root cause

- `🌱️value/📋️list/🔺️edit/🔑️named/🟦️.ts` and `📇️index/🟦️.ts`: the plan names the admitted key directory through a new structural interface `NamedEditAdmittedBase` (the class implements it), no import of the index.
- `🧑‍💻dev/🔌️vite-plugins/🚀️activation-vite-plugin/`: option types moved to the new leaf `🧬️options/🟦️.ts`; plugin and runtime both import it; the public glue `📦️packages/.../🔌️vite-plugins/🟦️.ts` exports the type from the leaf.
- `🧵️job/⚠️failure/🟦️.ts`: self-import type replaced by the already imported name.
- `📚️library/⚖️laws/🎯️tool-job-coverage/`: `toolJobFactoryModuleFile` moved from `🏭️factory` to `📋️inventory` (owner of `toolJobRustBlock`, which it uses).
- `🧰️framework/📦️packages/🟦️typescript/🛰️backbone/🔗️binding/🟦️.ts`: `ActorDocumentMessagePortV1` is a class but was exported with `export type`; now a value export (TS1362 in the renderer actor-backbone test).

## 4. Verification

- `tsc --noEmit` (12288 MB heap, foreground) after the rewrite: OS `tsconfig.json` 44 errors, framework package 67, repo 62, `✏️s` 95, root 1,912. `lay13-tools/errs-in-changed.ts` intersects the diagnostics with the rewritten files: the only regression of mine was TS1362 (fixed above); the rest are the known missing dependencies (`@dnd-kit`, `react-resizable-panels`, jsdom types, flow-core) and peers' in-flight edits (wire-turn and shard-client splits, `parseBinary64` arity in fem diff schemas, missing `library/🟦️.ts`). Not re-run after the glue and cycle edits of 3.3 and the closure revert.
- `bun build --target=bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` bundles 625 modules; `bun build --target=bun 📜️script.ts` ok; `bun repo --help` boots and prints usage.
- Re-measure: 12 area cruises after the last edit (section 2). New vs old violation sets diffed with `lay13-tools/diff.ts`: net new are peers' in-flight edits (store worker, renderer test fixtures), no new cycle from my edits remains (1 new cycle `store/👷️worker/📁️folder` to `🗂️config-lane` is the peers' split).
- Not run: any vitest/bun test suite of the 407 rewritten files (type-check and bundle only), Rust side, repo taxonomy laws.

## 5. Residual spec: `lay13-tools/exports-needed.json`

Structure: `byPackage` (package, target file, proposed subpath and glue, symbols, importer files, status), `summary`, `notExportable`, `misplacedGlue`, `policyConflicts`. 374 entries in 37 packages for 675 edges:
- 51 rows are already planned by `repoapi-tools/exports-final.json`, the manifest key exists and the glue exists (`C/result-glue.md`: all 1,705 targets exist), but the glue does not export the imported names. Fix is in the glue file (add the names), no manifest line.
- 323 rows are not in `exports-final.json` at all: new keys plus new glue. Largest: `@semio-tech/framework` 166 entries (310 edges, 153 unplanned), `@semio-tech/repo-lib` 49 (95 edges, all unplanned; per the coordinator, narrow keys such as `storage/paths`, not the barrel), `@semio-tech/framework-os` 38, `@semio-tech/value-ts` 19, `@semio-tech/framework-plugin` 16, `@semio-tech/process` 12, `@semio-tech/framework-actor` 11, `@semio-tech/s-fem-engine-ts` 10, `@semio-tech/course-react` 6 (72 edges), `@semio-tech/course` 1 (44 edges).
- No row is "planned but manifest key missing": the repoapi batch is in the live manifests.
- `notExportable`: 250 reverted rewrites (policy conflicts, cycles, closure), 24 frozen, 22 imports without an AST node (`vi.mock`-style strings), 15 `?query` imports, 4 unknown package.
- `misplacedGlue` (28 `framework-no-products`): glue files under `@semio-tech/framework` (`📦️packages/🟦️typescript/🛍️products/...`, `.../OS/<module>/🧪️testing/...`) re-export OS, repo and renderer files; they belong in `framework-os` / `repo-lib` / `framework-renderer-wgpu`.
- `policyConflicts`: `renderer-hosts-only-ui` allows only ui, framework packages and the renderer itself, while `no-cross-package-relative` makes react-to-elements imports go through the OS package glue, whose path is not allowed (116 reverted edits). Needs a ruling: allow the OS package glue directory in the allow-list, or put renderer-internal entries under `engine/(targets/react|elements)/`.

## 6. Open items

1. Apply the spec (manifest batch, glue names), then re-run `bun C/lay13-tools/ncpr-all.ts <edges.json> <plan.json> apply C/lay13-tools/reverted-regress.json C/lay13-tools/reverted-closure.json` (edges via `cruise-area.sh` + `edges.ts`); it finds the new entries and rewrites the rest. 19 further edits (new since the rewrite, mostly vitest configs and `📜️script.ts` importing repo-lib barrels) were not applied: they would add barrel imports.
2. Remaining cycles (27): renderer PluginRuntime turn-settle/typed-result/turn-dispatch (peers editing), OS dev bundles (generated), process native-build storage cluster, leases/metadata (runtime dynamic import plus custody check, needs an injected checker and four test call sites), ui contract resident/pages, ui elements Tree/Label/Popover/ActionGroup cluster, actor shard-client and kernel input, build/bazel operations (not touched: bazel adapter dir).
3. `no-core-path` 4: folders named `🔗️shared` under `🌱️value/🧬️retained-clone` and `🛬️decode/🧳️owned` (rename touches the Rust `#[path]` twins; not done).
4. `ui-foundation-closure` 2, `no-state-outside-os` 1: fixture and hash coupling, not done.
