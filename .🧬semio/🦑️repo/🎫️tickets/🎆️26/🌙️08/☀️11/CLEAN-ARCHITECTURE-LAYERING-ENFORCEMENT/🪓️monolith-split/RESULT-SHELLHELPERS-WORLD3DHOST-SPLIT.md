# Result: ShellHelpers and World3dHost module-level split

Executed 2026-10-15 by the Sonnet executor. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.
Seam map used: `SEAM-MAP-SHELLHOST-TSX.md` (sections 5 and 6, plus the cycle analysis). ShellHost was NOT touched.

## 1. Outcome at a glance

| File | Before | After | Children |
|---|---|---|---|
| `E/🛠️ShellHelpers/🟦️.tsx` | 6,708 lines | 390 lines (pure re-export barrel) | 10 new |
| `E/🌐️World3dHost/🟦️.tsx` | 8,551 lines | 2,845 lines (imports, barrel lists, `World3dHost` component, 5 pinned units, see 4) | 9 new |

Public API is unchanged. Checked with the TypeScript checker (`getExportsOfModule`, name plus type/value kind), before vs after:
ShellHelpers 349 exports identical, World3dHost 181 exports identical. Every original top-level declaration appears verbatim exactly once across the new files (468 ShellHelpers units: 462 declarations plus 6 `export ... from` lines kept in the parent; 378 World3dHost units, 378 of 378 exactly once).

## 2. Files created

ShellHelpers children (`E/🛠️ShellHelpers/<dir>/🟦️.ts(x)`, UTF-8 path bytes in brackets):

| Child | File | Lines | Holds |
|---|---|---|---|
| `🏷️labels` | `🟦️.tsx` | 392 | shell label family, tab icons, frozen texts, command ids, manifest/app label resolution (leaf) |
| `🔩️plumbing` | `🟦️.ts` | 999 | history projection, reserved ids, examples, leftover views, presence, `useUIHistory`, downloads, file requests, effect dispatch |
| `🎞️media-import` | `🟦️.ts` | 582 | chunked import, `requestMediaFrames` tiers 1 and 2 |
| `🌳️tree-windows` | `🟦️.tsx` | 487 | utility registry, UI intents, tree-window scheduler, cached tree-panel configs |
| `🛣️route-and-chrome` | `🟦️.tsx` | 987 | routes, breadcrumbs, layout seeds, engagement, plugin install bands, panel-tab nodes, refusal notices |
| `🧾️transfer-checkin-tutorial` | `🟦️.ts` | 572 | document transfer, open-with, check-in/sync status, tutorial UI bridge (max path 171 bytes) |
| `🛑️extension-io` | `🟦️.ts` | 521 | cancellation maps, latest/directional/coalescing dispatchers, Puzzle3d brush mesh upload |
| `🪟️window-panes` | `🟦️.tsx` | 1,099 | window measures, selection utility options, window action pane, staged-arg controls |
| `🎛️command-registry` | `🟦️.tsx` | 698 | OS command registry, command category tree/tabs, tool registry tree/tabs |
| `🔄️ui-refresh` | `🟦️.ts` | 496 | JSON identity helpers, UI refresh cache, dirty-scope algebra, coalescer, request builders |

World3dHost children (`E/🌐️World3dHost/<dir>/🟦️.ts(x)`):

| Child | File | Lines |
|---|---|---|
| `🔭️scene-schema` | `🟦️.ts` | 696 |
| `🎨️mesh-render` | `🟦️.tsx` | 1,223 |
| `🧩️glb-surface` | `🟦️.tsx` | 329 |
| `🔗️instance-selection` | `🟦️.ts` | 818 |
| `🧮️gumball-math` | `🟦️.ts` | 531 |
| `🧱️instance-layers` | `🟦️.tsx` | 1,534 |
| `🎯️marquee-and-bridges` | `🟦️.tsx` | 766 |
| `🗂️catalogue-drop` | `🟦️.tsx` | 270 |
| `🪟️window-view-store` | `🟦️.tsx` | 178 |

Longest new path: 171 UTF-8 bytes (limit 240). All children are depth 1. The existing children (`⏪️time-travel`, `📌️panel`, `📏️modelling`, ...) were not touched.
Each child opens with a JSDoc whose first token is the child's folder emoji and which links back to the parent (`{@link ../🟦️.tsx | ShellHelpers}`). No comments were added inside definitions. Relative specifiers were re-computed with `path.relative` (explicit extensions kept, emoji segments intact). Neither monolith contains `?worker&url`, `new URL(..., import.meta.url)` or dynamic `import()` (grep), so no URL specifier needed adjusting; every new specifier resolves (tsc reports no TS2307 for the new files).

Edited outside the two files: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (registration, see 3).
Changes to parent barrel semantics: the three `export ... from` lines in ShellHelpers (segmented download, panel carriage, PluginRuntime isolated-job drive) stay in the parent verbatim; the three local re-export statements of imported bindings (`export { IMPORT_CHUNK_BYTES, ... }`, `export type { ImportChunk }`, `export { pluginLoadRemainingMs }`) were rewritten as `export { ... } from "<original specifier>"` (same specifier, same names).

Module DAGs (verified by an import-graph SCC check over parent plus children: 0 SCCs of size greater than 1, 0 children importing their parent):

- ShellHelpers: `labels` and `extension-io`, `ui-refresh` are leaves; `plumbing` -> `labels`; `media-import` -> `plumbing`; `tree-windows` -> `labels`; `window-panes` -> `extension-io`, `labels`, `plumbing`; `route-and-chrome` -> `plumbing`, `labels`, `tree-windows`, `window-panes`; `transfer-checkin-tutorial` -> `labels`, `route-and-chrome`; `command-registry` -> `labels`, `window-panes`, `plumbing`.
- World3dHost: `scene-schema` is the leaf; `mesh-render` -> schema; `instance-selection` -> schema; `gumball-math` -> schema, mesh; `glb-surface` -> mesh, schema, instances; `marquee-and-bridges` -> schema, mesh; `instance-layers` -> gumball, schema, mesh, instances, glb, marquee; `catalogue-drop` -> mesh, gumball, marquee, schema, glb; `window-view-store` -> marquee, schema.

Deviations from the seam map (all forced by the real unit-level dependency graph, computed with the TypeScript checker):

- ShellHelpers got 10 children instead of the map's 8, because the map's `registries` module was 1,311 lines and `effects-and-io` 1,558 lines. Split: `plumbing` + `media-import`, and `window-panes` + `command-registry`. `tree-windows` took `BrowserActorPanelHostV1`; `route-and-chrome` took `spawnedWindowChromeForKind`; `FRAMEWORK_CHECKIN_CONTROLLER_ID` lives in `labels`. These moves remove the map's `route <-> tree` and `labels <-> transfer` cycles; the map's claim that moving 4860-4924 into the tree module avoids a cycle was wrong (`windowMeasuresChrome` reads `windowMeasuresOverlay`), so all window-measure code sits in `window-panes`.
- World3dHost: nine children as in the map, with these reassignments: `World3dMarkerInteractionFields` -> schema (types read it), surface helpers, vertex-mark constants and `meshBoundsCorners` -> mesh, `clientPointOverHost` -> instances, `CatalogueDropPreviewInvalidate` -> catalogue, `WorldGumballTransformPreview` -> gumball. The module-eval destructuring `const { useFrame, useLoader, useThree } = sceneHostPort.fiber;` is owned by `mesh-render` and imported by the others.
- World3dHost parent keeps five units besides the component so that two source-text gates keep working (see 4): `WorldToolRunTrace`, `world3dHoverActionArgs`, `world3dSelectionActionArgs`, `world3dSelectionTargetsActionArgs` (each is used only by the component). Parent is therefore 2,845 lines (map estimate about 2,400): about 280 lines of imports plus about 200 lines of barrel export lists are the difference.
- Module-level mutable state each lives in exactly one module (celebrate maps in `mesh-render`, leftover overlay and `WORLD_INSTANCE_RECORD_TEXT` in `instance-selection`, gumball transform preview store in `gumball-math`, catalogue/selection preview stores in `catalogue-drop`, pending projection map in `window-view-store`, `let pendingPanelUiNodeSingleton` in `tree-windows`, cancellation maps and brush mesh registry in `extension-io`).
- The module-level `if (typeof window ...)` right-down listener install (World3dHost line 6141) now runs when `instance-selection` evaluates instead of when the parent evaluates; it only registers window listeners, so there is no order dependency.

Tooling used (kept for traceability, all inside `🗑️generated/tsx-split/`): `analyze.mjs` (TypeScript-API unit/reference analysis), `split.mjs` plus `write.mjs` (emit, hash-guarded: aborts if the source changed during analysis, and the `rewrite` mode refuses to overwrite a file that differs from the previous render), configs `cfg-shellhelpers.mjs`, `cfg-world3d.mjs`, `exports.mjs`, `coverage.mjs`, `graph-check.mjs`, copies of the originals `orig-ShellHelpers.tsx` and `orig-World3dHost.tsx`. These are temporary helper scripts in the ticket folder, not repo tooling.

## 3. Taxonomy decision and evidence

Validator: `bun ./📜️script.ts verify taxonomy report --scope <path>` (`runTaxonomy` in the root `📜️script.ts`, delegating to `verifyTaxonomy`).
Finding: it IS enforced. Baseline run on scope `E/🛠️ShellHelpers` (log `tsx-taxonomy-baseline-shellhelpers.log`, 1:55 min): `clean=false errors=20`, and 17 of the 20 are `directory-kind-unresolved ... Directory has no registered semantic kind`, including the existing unlisted children `⏯️tool-run-panel`, `🎚️measure-controls`, `📌️panel`, `🧩️contributions`. So existing unlisted children are pre-existing failures, not an exempt precedent.
Rule that makes a child admissible: its folder name must be a member of a registered directory kind. `semanticDirectoryMemberKinds."members-of-members-of-elements"` has `ownerKindIds: ["members-of-elements"]` and a `memberNames` list (before: `🧭️gumball`, `🗂️local-catalog`, `📎️local-folders`); it applies under every element folder.
Action: appended the 19 new names (exact strings, each emoji followed by U+FE0F, all RGI, no duplicate emoji among the siblings of one parent, none is `📁️ 📂️ 📄️`, none equals the sibling file emoji `🟦️`) to that `memberNames` array in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (textual edit, 1 hunk, +11/-1 then +9 lines, nothing else reformatted):
`🏷️labels 🔩️plumbing 🎞️media-import 🌳️tree-windows 🛣️route-and-chrome 🧾️transfer-checkin-tutorial 🛑️extension-io 🪟️window-panes 🎛️command-registry 🔄️ui-refresh 🔭️scene-schema 🎨️mesh-render 🧩️glb-surface 🔗️instance-selection 🧮️gumball-math 🧱️instance-layers 🎯️marquee-and-bridges 🗂️catalogue-drop 🪟️window-view-store`.
Result after the split:
- scope `E/🛠️ShellHelpers` (`tsx-taxonomy-after-shellhelpers.log`): `errors=20`, the same 20 findings as the baseline (one reference-edit-required position shifted from 173:37 to 169:37 because a peer edited `🔬️engine-contract`), none on a new child.
- scope `E/🌐️World3dHost` (`tsx-taxonomy-after-world3dhost.log`): `errors=6`, all `directory-kind-unresolved` on pre-existing unregistered dirs (`📏️modelling`, `🧪️tests/📏️modelling`, `🧪️tests/🔀️projection-pane`, `🧪️tests/🤏️multi-touch`, `🧪️tests/🧫️fixtures/🔀️projection-pane`, `🧬️schema/🧭️gesture-targets`); none on a new child. I ran no World3dHost-scope baseline before registering, so I cannot show that number "before"; the six paths are all dirs I did not create.
I did not register the pre-existing unlisted children (not my files).

## 4. Required follow-ups outside my ownership (gates that read these sources as text)

1. BREAKS: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts` line about 1797 (`runAuthoredSurfaceOracle`, `reference: surface`) reads `🧱️elements/🌐️World3dHost/🟦️.tsx` and requires the two `FunctionDeclaration`s `surfaceTuple` and `worldSurfaceMaterial` ("Production material constructor declarations were not found" otherwise). They now live in `🧱️elements/🌐️World3dHost/🎨️mesh-render/🟦️.tsx` and are used by other mesh-render code, so they cannot stay in the parent. Fix: point that `readFile` at `🎨️mesh-render/🟦️.tsx`. I did not run this oracle (needs a browser GPU and cargo); the break is by inspection plus the fact that the parent no longer declares those names.
2. Already handled by pinning units in the parent: `world3dPointerGestureOracle` (react target `📜️script.ts`, run by `world3d-interaction-check`) greps the parent for the selection/hover args builder bodies, and `interactivityPuzzleFillTraceFailures` (root `📜️script.ts`) requires `<ToolRunTraceLayer` in the parent. With the first cut those moved and 4 oracle checks failed; I extracted the oracle function and ran it against the original file and the new parent. Original: 72 checks, 0 failed. New parent (final layout): 72 checks, 0 failed. (Caveat: the oracle's merge-vocabulary fixture `🧰️framework/🔨️modules/🕹️interaction/🧫️fixtures/🎯️merge-modes.json` does not exist in the repo today, so the full function throws ENOENT after the host checks both before and after my change; I stubbed that read in my extract.)
3. Pre-existing cycle warnings still stand and were not worsened: `ShellHelpers -> Shell -> ShellHost -> ShellHelpers`, `ShellHelpers/🏷️labels -> 🗣️Interpreter -> ShellHelpers`, World3dHost <-> ShellHost <-> ShellHelpers. The Phase 0 import redirects (map section 7) are unchanged and still pending; the new children import the same external modules the parents did.
4. `E/🏛️ShellHost/🟦️.tsx` was modified by a peer at 02:39 and currently has 2 type errors at line 9191/9192 (`ActiveSession | null` vs `undefined`). Not related to this work.

## 5. Verification, baseline vs after (all foreground, capped, logs in `🗑️generated/tsx-*.log`)

Typecheck: `node_modules/.bin/tsc --noEmit -p 🗑️generated/tsx-tsconfig.json` (a scoped tsconfig extending `💻️os/tsconfig.json` with `files` = the two parents; the program pulls in 2,475 files). Used instead of `bun repo run`, because `bun repo run @semio-tech/framework-renderer-react:test` failed in this session with `Selected compile source import missing: ... 🔌️plugin/🏛️ownership/🧱️owner/📦️packages/🦀️rust/📜️script.ts` (4 min, log `tsx-vitest-probe.log`), a repo-infrastructure fault that is not caused by my files.
- Baseline (`tsx-tsc-baseline.log`): 32 errors, 0 in ShellHelpers/World3dHost (27 in `📃️UiDocumentStore/🧪️tests/🧪️typedwire`, 3 in `🔌️PluginRuntime`, 2 in `🪪️WasmSessionLoader`).
- After ShellHelpers split (`tsx-tsc-after-sh1.log`): identical 32 errors (sorted diff empty).
- After World3dHost split (`tsx-tsc-after-w1.log`, run with `--noUnusedLocals`): the non-unused errors are identical to the baseline; the `TS6133/6192/6196` hits in my files are declarations that were already unused in the originals (`isStudioMode`, `tools`, `ActionArgStringSchema`, `WorldContextMenuItem`, `computing`, `vortexPointerArm`); no unused import in any child.
- Final layout (`tsx-tsc-after-w2.log`): same 32 errors plus 2 new ones in `🏛️ShellHost/🟦️.tsx(9191,32)` and `(9192,43)` from the peer edit at 02:39 (see 4.4). None in the split files.

Unit tests (vitest through the react target's config `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, level `long`, `SEMIO_VITEST_POLICY` built by hand in `tsx-vitest.sh`):
- Targeted set of 22 suites that import ShellHelpers/World3dHost, baseline before editing (`tsx-vitest-baseline.log`): 4 files failed, 16 passed; 7 tests failed, 983 passed (990). After both splits (`tsx-vitest-after-w1.log`): the same 4 failing files and the same 7 failing tests; 982 passed of 989. The single missing test is `Draw utility action and interruption fixtures conform to their closed contracts` in `🔬️engine-contract`, which a peer deleted at 02:14 (confirmed with `git diff HEAD`).
- Full config suite (184 files, 2,909 tests) after the split vs the ORIGINAL sources: I built `tsx-vitest-orig.config.ts`, a vitest config that extends the real one and aliases the two parents to verbatim copies of the original files (`tsx-split/orig-abs/*.tsx`, relative specifiers made absolute; alias activation confirmed by a `[DEBUG]` line). Originals: 30 files / 38 tests failed, 2,871 passed (`tsx-vitest-full-baseline-originals.log`). New layout (`tsx-vitest-full-after2.log`): 30 files / 38 tests failed, 2,871 passed. The failure sets are identical (sorted diff) except one `🖌️Paint2dHost/✍️editing ... 🖱️selection-focus` test whose name a peer changed ("Apply" vs "Mask from selection"). An earlier full run after the split showed 28/36 failed (two failures that flapped in the baseline: `⚙️settings-general-layout` and the Paint2dHost one). The 28 failing files are pre-existing (fixture path ENOTDIR/ENOENT, missing `retainedProgress` schema property, `@semio-tech/framework-os-node-graph-rs` unresolved, jsdom multi-combobox, etc.); in particular the ShellHelpers/World3dHost-adjacent failures `🧪️command-rejection`, `🧪️fault-notices`, `🧪️staged-arg-controls`, `⏪️time-travel/🧪️tests/🧩️component`, `World3dHost/🧪️tests/🧩️component` fail identically against the original files.
- Source-text suites that read `World3dHost/🟦️.tsx` as text in `🔬️engine-contract` (`?raw` import plus `readFileSync`) pass.

Module-graph smoke test: `bun -e "await import('<parent>')"` fails for both parents with `Missing 'default' export in module ...🌎️hub-source/👷️service-worker/🟦️.ts?worker&url` (a Vite-only `?worker&url` specifier in a transitive dependency, not in my files), so it is skipped. The vitest runs, which load both parents and all children through Vite's SSR transform, are the graph evidence.

Taxonomy: see 3. Path length: max 171 bytes among new files.

## 6. Not done / limits

- Not run: the browser/GPU oracles (`world3d-scene-shading ... surface`), `verify interactivity`, any Rust gate, the repo-wide `verify taxonomy` (only the two element scopes), `bun repo run ...` (broken infrastructure, see 5), the real browser/Vite dev server (so no manual check of module evaluation order in a browser; vitest/Vite SSR is the only runtime evidence of the new eval order).
- The `🧾️manifest.json` under `🧑‍💻dev/📤️distribution/` lists the two parents with a byte count and sha256 from before the split; it is a generated artifact and was not regenerated.
- `🪟️window-panes` (1,099 lines), `🔩️plumbing` (999), `🛣️route-and-chrome` (987), `🧱️instance-layers` (1,534) and `🎨️mesh-render` (1,223) are still large; further splits are possible (instance layers: `WorldInstanceNode` alone is 460 lines).
- `Interpreter`, `World3dHost` imports from ShellHelpers/ShellHost and ShellHelpers' import of `registerPendingWorldProjection` still go through the parents (map Phase 0 not performed, per task scope).
- `🗑️generated` folders were left in place.
