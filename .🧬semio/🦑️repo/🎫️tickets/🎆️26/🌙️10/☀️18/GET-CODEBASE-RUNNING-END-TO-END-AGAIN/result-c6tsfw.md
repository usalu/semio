# Result c6tsfw: monolith census 6, TypeScript slice (P60, P62, P63, P64, P65, P70, over-10k test)

Nothing was built through `bun repo`, `nx`, `cargo` or `bazel`. No modifying git command ran. `node_modules` is a dangling symlink, so no vitest or bun test ran. Tools and scratch live in `/Users/ueli/.cache/c6fw2/` (outside the repo): `split.mjs` (AST splitter), `importers.mjs`, `run.mjs`, `check.mjs`, `bodysplit.mjs`, `i18nsplit.mjs`, `specs/*.json`.

## Verdict per file

| File | Before | After (leaf, largest child) | Verdict |
| --- | --- | --- | --- |
| P60 `ŌĆ”/­¤ÄŁ’ĖÅactor/­¤ō«’ĖÅshard-client/­¤¤”’Ėı.ts` | 2,618 | leaf 1,655, 7 children (largest 275) | PARTIAL: `class ShardClient` is 1,593 lines with ES `#private` fields and a static block, which cannot be split losslessly |
| P62 `theme/­¤¤”’Ėı.ts` | 1,084 | leaf 214, 7 children (largest 267) | split |
| P62 `Tree/­¤¤”’Ėı.tsx` | 4,620 | leaf 666, 19 children (largest 561) | split |
| P62 `Canvas/­¤¤”’Ėı.tsx` | 2,135 | leaf 867, 7 children (largest 422) | split. `Mode` (647 lines) stays in the leaf |
| P62 `Scene/­¤¤”’Ėı.tsx` | 2,158 | leaf 510, 8 children (largest 533) | split |
| P63 `styling/builder/vite/­¤¤”’Ėı.ts` | 1,579 | leaf 253, 10 children (largest 219) | split |
| P63 `styling/tests/suite/­¤¤”’Ėı.ts` | 1,279 | leaf 8, 10 children (largest 354) | split (side-effect imports, 81 titles equal) |
| P63 `ui/scene/­¤¤”’Ėı.ts` | 1,387 | leaf 25, 7 children (largest 250) | split |
| P63 `Diagram/layout/­¤¤”’Ėı.ts` | 1,501 | leaf 289, 5 children (largest 531) | split |
| P63 `Diagram/­¤¤”’Ėı.tsx` | 2,053 | leaf 93, 7 children (largest 647) | split |
| P63 `Diagram/tests/component/­¤¤”’Ėı.tsx` | 1,588 | leaf 5, 5 children (largest 479) | split (58 titles equal, vi.mock moved with its `vi.hoisted` into `harness`) |
| P64 `react/i18n/­¤¤”’Ėı.ts` | 2,354 | leaf 252, 5 children (largest 575) | split. Assembled `uiChromeTranslationBundles` is byte-identical (JSON incl. key order) to the original |
| P64 `react/­¤¤”’Ėı.tsx` | 6,570 | leaf 345, 29 children (largest 411) | split (barrel mode, see decisions) |
| P65 `ContextMenu/­¤¤”’Ėı.tsx` | 1,154 | leaf 464, 4 children (largest 252) | split |
| P65 `LayeredOverview/­¤¤”’Ėı.tsx` | 1,211 | leaf 479, 5 children (largest 216) | split |
| P70 `store/worker/­¤¤”’Ėı.ts` | 7,083 | leaf 449, 27 children | PARTIAL: `DocumentBrowserActorReservation` is a 1,133-line class (child `actor-reservation`, 1,162 lines with imports). All other children are at most 742 |
| P70 `retirement/­¤¤”’Ėı.ts` (test) | 1,399 | leaf 3, 3 children (largest 525) | split (63 titles equal) |
| over-10k `owned-locale-detector-retirement/­¤¤”’Ėı.tsx` | 11,949 | leaf 72, 27 children + harness (largest 577) | split. 566 `it`/`test` before and after, 0 multiset differences; 4 wrapper `describe`s are replicated (one per child) |

## Verification (static only)

- `check.mjs` per split: lossless top-level declaration multiset (modulo `export` and relative-path depth), test-title multiset, relative import and named-export resolution over all children and all importers, value-import cycle scan, and a TypeScript 5.9.3 program check. Structural diagnostics (TS2304, 2305, 2724, 2459, 2552, relative TS2307, ŌĆ”) are 0 for every split. The remaining diagnostics are the known environment noise (missing `react`, `node:*`, `vitest` types).
- Statement-level call graph of the worker is a DAG; the 28 groups are acyclic. Name gate (`names.mjs`): all new folder names are RGI emoji + U+FE0F with unique first graphemes among siblings (0 issues).
- `reach.mjs` confirms `readModuleTreeSource(<leaf>)` reaches every child of the worker and of the owned-locale test.

## Decisions and deviations

- Importers repointed (no compatibility re-exports in leaves). Package barrels that `export *` a split leaf got explicit named re-export lines to the new children (theme, ui/scene, ŌĆ”). `framework-source-topology` `maximumPackageLines` 3 -> 12 for the styling theme entry.
- The React target barrel (`react/­¤¤”’Ėı.tsx`) is the public package entry, so it uses barrel mode: explicit `export {ŌĆ”} from` per implementation child plus five `export *` sub-barrels (`­¤ōż’Ėıadapter-exports`, `­¤¬×’Ėıchrome-exports`, `­¤ö¦’Ėıpanel-exports`, `­¤ōÜ’Ėıcomponent-exports`, `­¤Ś┐’Ėıframework-exports`). Importers of that file are unchanged.
- Test seam: the worker's `documentWorkerTestDependenciesV1` assigned module `let`s across modules, which ES imports cannot do. The owning child exports `setX(value)` (9 setters, e.g. `setDirectoryClient`) and the seam calls them.
- `Diagram` test harness exports `setDiagramFrameStack` for the one cross-module assignment.
- `retirement` test: `new URL(relative, import.meta.url)` in a loop now uses `../${relative}`; all other `import.meta`/`new URL`/`vi.mock`/`import()` relative literals were rewritten by the tool.
- Orphan comments of removed imports in the `react` leaf and region markers across earlier splits may be unbalanced (cosmetic).

## Text readers re-pointed

Tree (4), Canvas (1), worker (`readFileSync` -> `readModuleTreeSource` in 12 files: hub tests, os package script, directory-checks, canonical-bootstrap-folder-mirror, renderer react script, gismap script, hub-session-capability, gis document-open, hub-socket-close; the execution-target-lease check now reads the 6 children of the old region), React (`checked-values`, Rust `browser-prefs-persistence` key list), vite (device unit test, vendor-transport, artifacts test, dev builder dynamic imports split into 6 opaque imports), ui/scene (3 tests), i18n (merge-policy law), plugin-architecture config-adapter prefix, owned-locale test reader (`window-lifecycle-template-drag`). Nx inputs and vitest coverage globs extended for shard-client, ui/scene, theme. `browser-graph.json` denyModules got the 10 vite children.

## Remaining (exact list)

1. `shard-client/­¤¤”’Ėı.ts` (1,655): split `class ShardClient` needs a design decision (ES `#private` fields cannot be shared across modules). Report only.
2. `worker/­¤¦Ę’Ėıactor-reservation/­¤¤”’Ėı.ts` (1,162): `class DocumentBrowserActorReservation` (TS `private`, no `#`). A layered base/derived split with `private` -> `protected` plus abstract declarations is feasible but not lossless; not attempted.
3. `taxonomy.json` `generatorContracts.wgpu-frame-worker`: `inputPatterns` and `packageGeneration.browserProfile.sourceModulePaths` (byte-ordered, exact) list leaf paths of split modules (shard-client, ui/scene, Diagram/layout, ŌĆ”). Children reachable from those leaves must be inserted in byte order, or the wgpu package generation will not see them. Not done.
4. Not run: any vitest/bun test, the browser-graph module-count limit (`maxModules` 1600), `watch-policy.json` fixture for the worker children.
5. Stale fixture, not mine: `generator-ownership` `browserTransport.imports` expects the worker to import `service/io`, which the original did not either.
6. Cycles on the type level only (`import type` of the leaf from children) exist in shard-client; no value cycles anywhere.

Coordinator compile list: nothing for Rust. TS tests worth running once `node_modules` exists: ui `styling` (theme-resolve, suite), `Diagram` component test, `owned-locale-detector-retirement`, store `worker` in-source tests, `retirement`, `checked-values`.
