# Result: CAD renderer rules-of-hooks fix and module split

Executed 2026-10-15 by the Sonnet executor. `R` = `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer`.
Seam map used: `SEAM-MAP-TSX-TIER2.md` section 2.A. Method as in `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md` (the same TypeScript-API unit analyzer, copied to `🗑️generated/cadr-split/tool/`).

## 1. Outcome at a glance

| Item | Before | After |
|---|---|---|
| `R/🟦️.tsx` | 6,859 lines | 424 lines (imports, barrel, in-source test block) |
| New children | 0 | 10 (see 3) |
| Largest child | n/a | `🎛️repl` 1,860 lines |
| Public API | 183 exports | 183 exports, identical (name plus type/value kind) |
| Rules-of-hooks violations | 3 components, 5 hook calls | 0 |
| New test suite | none | `R/🧪️tests/🪝️hookorder/🟦️.tsx`, 4 tests |

## 2. Step 1: rules-of-hooks bug fix (test first)

### 2.1 Confirmation

Reading plus the third-party oracle `eslint-plugin-react-hooks` 7.1.1 (`react-hooks/rules-of-hooks`). The repo has no ESLint config and the plugin is not in the root `node_modules`; the only copy is `.🧬semio/🌐hub/s15-rs1-overlay/node_modules/eslint-plugin-react-hooks`. I ran root `eslint` 10.8.0 with a scratch flat config (`🗑️generated/cadr-split/eslint.config.mjs`, `@typescript-eslint/parser`). Caveat: the rule only recognises `React.useX` member calls, not `reactHostPort.useX`, so I linted a copy with `reactHostPort.use` rewritten to `React.use` (`cadr-split/lint/orig.tsx`). Baseline (`cadr-eslint-baseline.log`) reported exactly these 5 hook calls and nothing else in the 6.9k-line file (so `InteractionRepl`'s 136 hooks are clean):

| Component | Early return | Hooks called after it |
|---|---|---|
| `PreviewItem` (L1549) | `if (!p) return null;` (L1546) | `useMemo` for `transform` |
| `SpatialPickTargetNode` (L2655, L2659) | `if (target.kind === "vertex") return ...` (L2646) | 2 x `useMemo` (`wireSegments`, `nurbsPoles`) |
| `TessellatedCommitMesh` (L3325, L3330) | `if (!showFaces && !showEdges) return null;` (L3324) | `useMemo` (`faceInfoById`), `useCallback` (`resolveFace`) |

### 2.2 What React actually does (found while writing the tests)

React 19.2.8 only throws "Rendered fewer hooks than expected" when at least one hook ran before the early return. So:
- `TessellatedCommitMesh` (3 hooks before the return) crashes at runtime when `showFaces` and `showEdges` both flip to false after a first render.
- `PreviewItem` and `SpatialPickTargetNode` have every hook after the return. React does not throw (zero hooks is treated as a mount) but hook state is silently dropped and re-created. They are still violations of the rule, and the hook count differs between renders.

### 2.3 Tests written first (red), then fix (green)

New file `R/🧪️tests/🪝️hookorder/🟦️.tsx` (`registerTests2`), wired from the in-source block of the parent next to `registerTests1`. The block now builds one `testDependencies: RendererTestDependencies` object and passes it to both suites. `RendererTestDependencies` gained `PreviewItem`, `SpatialPickTargetNode`, `TessellatedCommitMesh` (the first two were not exported; they are passed through the test dependency object only, so the exported API did not change) and `reactHostPort`. Tests use `@testing-library/react` (already in the root `node_modules`) under the file's `jsdom` pragma. Harness: an error boundary records render errors, `console.error` is captured, and `vi.spyOn` counts every call on the `reactHostPort` hook functions (`useMemo`, `useCallback`, `useEffect`, `useRef`, `useState`, ...) for each frame. A test passes only if no hook-order fault is reported and every frame calls the same number of hooks.

| Test | Red (before fix) | Green (after fix) |
|---|---|---|
| harness sees a component whose hook count depends on its props (positive control; its conditional hook is invoked through an index access so the lint rule does not flag the test) | pass | pass |
| `PreviewItem` same hooks with/without `params` | FAIL (`expected 2 to be 1`, i.e. 0 vs 1 hook) | pass |
| `SpatialPickTargetNode` same hooks for vertex and edge targets | FAIL (0 vs 2) | pass |
| `TessellatedCommitMesh` same hooks when faces and edges are hidden | FAIL (React error "Rendered fewer hooks than expected ... accidental early return" in `<TessellatedCommitMesh>`) | pass |

Logs: red `🗑️generated/cadr-vitest-hooks-red.log` (4 failed = these 3 plus the pre-existing wasm failure, 69 passed), green `cadr-vitest-hooks-green.log` (1 failed, 72 passed).

### 2.4 Fixes (behaviour preserving)

- `PreviewItem`: `previewKind` is read with `p?.previewKind`, the `useMemo` moved above the guard and returns the identity transform when `p` is undefined (the identity is never used because the component returns `null` right after).
- `SpatialPickTargetNode`: both `useMemo` calls moved above the vertex return; `wireKind` is `null` for vertex targets (as before, a vertex never reached the hooks), so the memo bodies still early-return `[]` for them.
- `TessellatedCommitMesh`: `faceInfoById` and `resolveFace` moved above `if (!showFaces && !showEdges) return null;`. They depend only on `data`.

Re-run of the lint oracle on the fixed code (`cadr-eslint-step1.log`, empty) and on all 13 files of the final layout including the tests: exit 0 (`🗑️generated/cadr-split/lint2/`).

## 3. Step 2: split

Method: units from the TypeScript parser, references from the checker; assignment by unit index per the map; the generator copies each unit verbatim, adds `export` where a sibling needs a previously private unit, re-computes relative specifiers and imports only what each module uses. Unit assignment, child DAG check (`cadr-split/check.mjs`) and generator are in `🗑️generated/cadr-split/` (temporary helpers, not repo tooling).

| Child (`R/<dir>/🟦️.ts(x)`) | Lines | Holds |
|---|---|---|
| `🔌️ports` (`.ts`) | 54 | host-port bindings (`useThree`, drei `Line`/`Text`, `THREE` with the `DEFAULT_UP` import-time side effect), kernel record type aliases, `R3FPreviewKernel`, the `scenePreviewKernelRef` singleton |
| `🎨️style` (`.ts`) | 148 | spatial scene colour palette and cache, CAD field class constants |
| `🧲️geometry-targets` | 1,805 | mesh transfer hooks and bounds, archived footprints, layout math, pick kinds/toggles, geometry bucket queries, pick-target creation, typology style and hover-key helpers, typology pattern material, committed mesh layers |
| `🖼️display` | 437 | display primitives, `PreviewItem`, display-kind registry, `InteractionDisplay` |
| `🔖️interaction` | 363 | transform gumball, spatial view slots/theme, ground pick plane, constrained pointer bridge, drag surfaces, R3F adapter |
| `🎯️pick-layer` | 620 | pick scoring and selection-gesture math, render-set resolution, `SpatialPickTargetNode`, `SpatialPickGeometryLayer`, hit targets |
| `🪩️canvas` | 666 | host-state hooks, world coordinate system, auto fit, `InteractionCanvas`, `InteractionSpatialView` |
| `🧾️repl-model` | 835 | REPL suggestion model, selection pruning/merging, history presentation, host value/callback/prop types, default chrome state, engagement and search builders |
| `📋️repl-panes` | 310 | `SelectionAttributesPane`, `ModelStatsPane`, `SelectionPropertiesPane` |
| `🎛️repl` | 1,860 | `InteractionRepl` and `InteractionReplViewport` |
| parent `R/🟦️.tsx` | 424 | pragmas, header, imports, `export ... from` barrel (every originally exported name), `export type { SpatialComputeMode }`, in-source test block under a corrected `// #region 🧪️Tests` (the mojibake label is gone) |

Every child opens with an emoji JSDoc linking back to the parent; no comment was added inside a definition; the two `#region` markers that sat inside `PreviewItem` were removed with their blank lines. Module DAG: `ports` and `style`->`ports` are leaves; `geometry-targets`->{ports,style}; `display`, `interaction`, `pick-layer`->{ports,style,geometry-targets}; `canvas`->those plus display, interaction, pick-layer; `repl-model`->{geometry-targets,display,interaction,pick-layer,canvas}; `repl-panes`->style; `repl`->{ports,style,geometry-targets,pick-layer,canvas,repl-model,repl-panes}; parent->all. Checked with an SCC pass over parent plus children: 0 cycles, 0 children importing the parent.

Deviations from the map:
- `🧾️repl-types` renamed `🧾️repl-model` (it holds functions and one component as well as types).
- Extra child `📋️repl-panes` (the map's `🎛️repl` would have been 2,144 lines with imports; the cap is about 2,000).
- `InteractionRepl` was NOT decomposed into custom hooks. At 1,860 lines it fits the cap, and decomposing a 1,674-line component with 136 hooks risks the exact hook-order problem this task is about. Its internal split remains a possible follow-up.
- Unit placement forced by the real graph: `createSolidTypologyStyleResolver` -> `pick-layer`; `cadFieldClass`, `ENUM_FIELD_NONE_VALUE` -> `style`; the kernel record type aliases -> `ports`. Previously private units used across children (for example `PreviewItem`, `SpatialPickTargetNode`, `scenePreview`, `readVec3`) are now exported from their child, but are not re-exported by the parent.
- Imports that were already unused in the monolith (`WorldOrbitProjectionSwitch`, `applyTransformation`, `InteractionRegistry`, ...) were not carried into the children.
- `useFrame` (unused in the original) stays in `ports` unexported.

Module-level state has exactly one owner: `scenePreviewKernelRef` and the `DEFAULT_UP` side effect in `ports`; `spatialSceneColorCache` in `style`; `typologyPatternMaterialCache`, `cadMeshGeometryPool` in `geometry-targets`; the `spatialDisplayItemRenderers` ephemeral map (same registry id string) in `display`. The side effect still runs before any use because every consumer imports `ports` (directly or through `style`).

### 3.1 Registration and tooling edits outside the renderer

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: 10 names appended to `semanticDirectoryMemberKinds."members-of-members-of-engine".memberNames` (textual insert after `🧩️assembly`, nothing else touched; bytes checked equal to the directory names, each emoji followed by U+FE0F, all RGI, no emoji repeated among the 10 siblings or with `🧪️tests`/`🟦️`; `📋️` is also used as folder `📋️forms`). The file also contains uncommitted edits of peers (`members-of-registers` and the ShellHelpers/World3dHost names); I did not touch them.
- `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🎚️config/🟦️.ts`: vitest `coverage.include` now also lists `.../📺️renderer/*/🟦️.{ts,tsx}`; without it the moved code would drop out of the coverage report (`includeSource` still lists only the parent, the only file with `import.meta.vitest`).
- No source-text reader of the renderer file exists (checked every hit of the path in the repo; the only code consumer is `📖️stories/🎭️renderer/🧪️.story.tsx` with a namespace import, which still resolves).

## 4. Verification (baseline vs after; all foreground, logs in `🗑️generated/cadr-*.log`)

Environment fact: the cad vitest project cannot run the renderer suite as shipped, because the workspace links `@semio-tech/cad-js`, `@semio-tech/cad-js-module-*` and `brepjs*` are missing from the root `node_modules` (baseline, config unchanged: 10 of 13 files fail with "Cannot find package '@semio-tech/cad-js-module-spatial-shape'"; the renderer file reports 0 tests). To get real numbers I wrote `🗑️generated/cadr-split/vitest.overlay.config.ts`, which merges the real config and adds aliases for those packages to their source files (activation proven by the renderer suite collecting 69 tests). Runner script: `cadr-split/vitest.sh` (same pattern as `tsx-vitest.sh`).

| Check | Baseline | After |
|---|---|---|
| Scoped tsc (`tsc --noEmit -p cadr-split/tsconfig.json`: renderer parent, both test files, the story; follows imports into the children) | 15 errors (`cadr-tsc-baseline.log`) | identical sorted error list after step 1 and after the split (`cadr-tsc-step1.log`, `cadr-tsc-final.log`); 0 errors in any renderer file |
| `noUnusedLocals` diagnostics inside `R` (script `cadr-split/unused.mjs`, original text injected for the "before") | 23 | 10, a strict subset (the 13 dropped ones are unused imports not carried over); no new unused name |
| Renderer in-source suite via overlay config | 69 tests: 68 passed, 1 failed | 73 tests: 72 passed, 1 failed. The 4 added tests are the new hook-order suite; the sorted pass/fail list differs only by those 4 added lines |
| The 1 failing test (both runs) | `concrete forest fixture keeps committed face visibility toggles`: `Cannot find module 'brepjs-opencascade/src/brepjs_single.wasm'` (environment, the wasm package path does not resolve) | same |
| Full cad vitest config, no overlay (the shipped configuration) | 10 files failed, 3 passed; 12 tests, 2 failed, 10 passed | 10 files failed, 3 passed; 12 tests, 2 failed, 10 passed; the FAIL lines are identical (sorted diff empty) |
| Public API (TypeScript checker `getExportsOfModule`, name plus type/value kind) | 183 exports (pristine original) | 183 exports; the diff against the pristine original (`exports-orig.tsv`) and against the step-1 text is empty |
| Losslessness (`cadr-split/coverage.mjs`) | 343 top-level units + 1 export declaration | each of the 342 declarations/statements appears exactly once across the 11 files, none missing, none duplicated (region marker comments excluded from the comparison) |
| Rules of hooks (eslint plugin, `reactHostPort.use` rewritten to `React.use`) | 5 errors in 3 components | 0 errors in all 11 source files and both test files |
| Child import graph | n/a | 0 SCCs of size > 1, 0 children importing the parent |
| Path length | existing longest path 217 bytes (a test file) | unchanged maximum 217 bytes, all new paths shorter |
| `verify taxonomy report --scope R` (`cadr-taxonomy-after.log`) | not run before the split | `errors=158`, all `projection-old-token-stale` in other files of the scope (peer rename in `🗂️catalog/*.rs` and the spatial-kernel geometry file); 0 findings on any renderer path, so the 10 children are admitted |
| dependency-cruiser (`depcruise --config 🧹️lint/🕸️dependency-boundaries/🟨️.cjs --focus R`, `cadr-depcruise-after.log`) | not run before the split | 96 errors in total over the focused slice (30 modules). Categories on renderer files are the same as the monolith's own: `plugins-framework-sdk-only` and `no-cross-technology-✏️s-to-🧰️framework` (imports of `@semio-tech/ui-react`, `infinite-world-r3f`, `ui-styling`, `framework` resolve to framework source), `no-cross-package-relative-semio-s-workspace` (relative imports of `📔️registry` and the spatial kernel), and `no-cross-package-relative-*` for the test kernel import in the parent. These edges are now attributed to the child that owns the import instead of to one file, so the per-file list is longer but each edge existed before. The only `no-circular` entries involving `R` are parent <-> test files (`🧪️repluserfacingsuggestiondetail` pre-existing, `🪝️hookorder` new, same type-only import of `RendererTestDependencies`); none involves a child. Other `no-circular` entries in the slice are `🎬️actions` <-> `🧬️typology` <-> `🗿️artifact`, not mine |

## 5. Not done / limits

- I ran no baseline for the taxonomy and dependency-cruiser gates before editing (the 158 taxonomy errors and the 96 cruiser errors are therefore not shown as before-numbers; the argument above is by edge identity and path filter).
- The browser, real R3F `Canvas` rendering and the story/Storybook runner were not run. The hook tests render the components with `react-dom` in jsdom (R3F intrinsics become unknown DOM tags, so only hook behaviour is covered, not drawing). Module-evaluation order in a browser build is proven only by vitest's SSR transform.
- `InteractionRepl` (1,674 lines, 136 hooks) was not decomposed; `geometry-targets` (1,805) and `repl` (1,860) remain the largest children.
- `eslint-plugin-react-hooks` is not a repo dependency, so the static oracle is a one-off scratch run, not a gate. The permanent protection is the runtime hook-count suite.
- Generated distribution manifests (such as the `🧾️manifest.json` files used by the ShellHelpers split) were not checked for entries naming the renderer file.
- Scratch copies (`cadr-split/scratch`, `lint`, `lint2`, `step1-renderer.tsx`, `orig-renderer.tsx`) and logs stay in `🪓️monolith-split/🗑️generated/`.
