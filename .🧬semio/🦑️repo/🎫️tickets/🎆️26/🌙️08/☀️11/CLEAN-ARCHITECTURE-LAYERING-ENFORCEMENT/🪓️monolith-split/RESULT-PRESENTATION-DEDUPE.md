# Result: presentation react engine dedupe (B deleted) and split of C

Executed 2026-10-15 by the Sonnet executor. Notation:
- `B` = `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/` (deleted).
- `C` = `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/` (package `@semio-tech/presentation-react`, `exports["."] = "./🟦️.tsx"`, `exports["./🎨️.css"]`; unchanged).
- `M` = `T/🔨️modules/` where `T` = `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/`. Sections 1 to 6 describe the first split inside `C`; section 8 supersedes the placement: the implementation (barrel plus all modules) now lives in `T`, and `C` keeps only a thin entry. All file paths below that say `C/🔨️modules/…` or `C/🟦️.tsx` as implementation now read `T/…`.
Tooling and logs are in `🪓️monolith-split/🗑️generated/pd-split/` and `🗑️generated/pd-*.log` (kept).

## 1. Outcome

| Item | Before | After |
|---|---|---|
| B | 5,559-line `🟦️.tsx`, css, 2 module copies, 3 in-source test files | deleted (7 files, whole `📺️renderer` directory) |
| C `🟦️.tsx` | 5,550 lines | 364 lines (barrel: header, bare css imports, child re-exports, `@semio-tech/presentation` re-exports, in-source test hook) |
| C children | 0 | 14 new modules under `M`, largest 1,167 lines |
| animate vitest | 4 failed suites, 0 tests (config was already broken) | 1 file, 1 test passed |

Public API of C is unchanged: 219 exports before, 219 after, identical name, kind and type string (TypeScript checker, section 6).

## 2. Hunk decisions (B vs C, whitespace-normalised: 32 hunks in `🟦️.tsx`, plus the module and css files)

Method: `diff -w`. Every hunk was classified as stale in B (C is newer or equal), test-only in B, or a newer fix only in B. Result: no hunk is a newer fix present only in B, so nothing was ported into C. Evidence that C is the newer copy: B has 24 TypeScript errors in `🟦️.tsx` and 62 in its in-source test file (baseline `🗑️generated/pd-tsc-baseline.log`), C has 0.

| # | B line (C line) | Difference | Decision |
|---|---|---|---|
| 1 | 2 | C docstring names the package | keep C |
| 2 | 12 (13) | C imports the `Disposition` type it uses (B has no local binding: type error) | keep C (stale in B) |
| 3 | 65 | B imports `act` for its in-source tests only | drop (test-only) |
| 4 | 122 | `markdownHtmlCompiler` box id: B `s.plugins.animate.apps.presentation.renderer.react.component.tsx.*`, C `framework.products.presentation.targets.react.*` | keep C (correct owner namespace). The B ids occur nowhere else in the repo (rg) |
| 5 | 134-137 | B in-source `registerTests1` hook (GFM table test) | drop; the same assertions live in C `🧪️tests/📝️owned-markdown-compiler/🟦️.ts` ("renders GFM tables through the active compiler of the React target") |
| 6 | 201 (197) | B re-exports `RenderSlide, Slide` in a second list | drop: C already exports both in its large type list (lines 158/161), so no API difference |
| 7-10 | 623, 634, 638, 702 | `AutoAnimateMatcherHost`, `elementIsTargetGhostAnchor`, `elementIsSourceGhostAnchor`, `elementIsInteractiveFigureDisposition` exported in C only | keep C (consumed by `♻️mit-bestand/…/33.projektetage/🧪️tests`) |
| 11 | 940 | `emphasisClass(emphasis: ParticipantEmphasis \| undefined)` | keep C (type fix) |
| 12, 13, 28 (two lines), 31 | 981, 1019, 5397-5398, 5541 | box ids `pdfWorkerReady`, `pdfCanvasPort`, `mountedRoot`, `surfaceChromeCleanup`, `jsonTreeRenderer` | keep C (same as 4) |
| 14, 15 | 1131, 1133 | `querySelector<HTMLElement>` / `closest<HTMLElement>` | keep C (type fix, same runtime) |
| 16 | 1157 | `observer?.observe(observed)` vs `if (observed && observer) observer.observe(observed)` | keep C (narrowing; `observer` is non-null only when `observed` is, so behaviour is equal) |
| 17 | 2388 | `{ kind: "figure", id: "", src: … }` | keep C (the schema now requires `id`) |
| 18 | 2443 | `vars` typed `CSSProperties & Record<string, string \| number \| undefined>` | keep C (type fix) |
| 19, 20 | 2489, 2754 | `figureCoverScrollContentSize(w,h,a)` vs `figureCoverScrollContentSizeAtZoom(w,h,a,1)` | keep C; equal result: the wrapper returns `base` for `zoom <= 1` and `base` is the `AtZoom(…, 1)` call |
| 21 | 2800 | `const markdown: string` | keep C (type) |
| 22 | 3109 | `swallowClick(clickEvent: Event)` | keep C (type) |
| 23 | 3601 | `transformFrameStyle` exported | keep C (used by tests) |
| 24 | 4914 | `marqueeStyle` structural type | keep C (type) |
| 25 | 5190-5192 | B `if (import.meta.vitest) return;` at the top of `syncPresentSlideMedia` | drop: a test guard in production code; C's `🧰️vitest.setup.ts` stubs `HTMLMediaElement.prototype.play/pause`, and C's 140 tests pass without the guard |
| 26, 27 | 5210, 5291 | `configuredDuration` local for `getConfig().autoAnimateDuration` | keep C (type narrowing, same value) |
| 29 | 5422-5427 | B in-source `registerTests2` hook (deck tests) | drop; C owns `registerPresentationReactDeckTests` in `🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx` |
| 30 | 5437 | `jsonPreview` exported | keep C (used by tests) |
| 32 | 5556-5557 | B hook `registerTests3` (json) | drop; covered by C's deck tests (`jsonPreview`, `renderJsonTree` are passed to them) |

Module files: `🔨️modules/🔌️pdf-canvas-port/🟦️.ts` and `📝️markdown-html-compiler/🟦️.ts` differ only by B's trailing `// #region 🧪️Tests` hook (7 lines each): dropped. `🎨️.css` is byte-identical (`diff -w` empty).

B-only tests compared with C's (extracted `it/describe` names, B 144, C+markdown 135): the nine B-only names are 8 projektetage-deck tests that also exist verbatim in `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎞️react-deck/🟦️.tsx` (git grep), plus the GFM table test (hunk 5). Remaining differences in the shared tests are type annotations (`id: ""`, `querySelector<HTMLElement>`, `as unknown as Event`) and two values (`"slide-background present"` and added `name: "scroll-off"`/`emphasis: "active"` in C): C is newer. Therefore B's tests are stale duplicates and were deleted with B.

## 3. Deleted and moved files

Deleted (all of `B`, 7 files):
- `…/📺️renderer/⚛️react/🟦️.tsx`, `🎨️.css`
- `…/🔨️modules/🔌️pdf-canvas-port/🟦️.ts` and `🧪️tests/🧪️pdfcanvasresourceowner/🟦️.ts`
- `…/🔨️modules/📝️markdown-html-compiler/🟦️.ts` and `🧪️tests/🧪️owned-markdown-html-compiler/🟦️.ts`
- `…/🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx`

The empty parent `📺️renderer` directory was removed too. Copies of the originals: `🗑️generated/pd-split/orig-B.tsx`, `orig-C.tsx`.

Also deleted: `✏️s/🔌️plugins/🎞️animate/🧪️tests/🧰️support/🌐️browser-environment/🟦️.ts` (and its directory). It is an older copy of C's `🧰️vitest.setup.ts` (diff: lacks the play/pause stubs), and its only consumer was the broken `setupFiles` entry of the animate config.

Nothing was moved: all of B's tests were duplicates of tests that C or the projektetage deck already own (section 2).

## 4. Consumers re-pointed

| File | Change |
|---|---|
| `✏️s/🔌️plugins/🎞️animate/🧪️tests/🎚️config/🟦️.ts` | rewritten: dropped the B `coverage`, `includeSource`, the `react()` plugin, the `ui-react`, `presentation-react` (pointed at a non-existent `🟦️.ts`), `framework` and `mit-bestand` spec aliases and the broken `setupFiles` (`../🧰️support/…` resolved to a non-existent path); `environment: "node"`; project name `@semio-tech/animate-js` (it was `@semio-tech/presentation-react`, a name clash with C's project); runs `🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts`. The React deck renderer is tested by its owner C |
| `✏️s/🔌️plugins/🎞️animate/📖️stories/🎭️presentation-deck/🧪️.story.tsx` | imports already used `@semio-tech/presentation-react` and `@semio-tech/presentation-react/🎨️.css` (= C); only the stale header comment (old B path, `globals.css`) was corrected |
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json` | added `"@semio-tech/presentation-react": "workspace:*"` (same style as the existing `@semio-tech/presentation`); removed `pdfjs-dist` and `reveal.js`, which only B used |
| `✏️s/bun.lock` | same dependency edit in the `animate-js` workspace entry; `bun install --frozen-lockfile --dry-run` in `✏️s` accepts it |
| `🔒️dependencies.json` | removed the animate `package.json` user line from `pdfjs-dist` and `reveal.js` (both stay `productionReachable`, C still uses them) |
| `🌎️hub/🧩️compositions/🎞️animate/📦️packages/🦀️rust/Cargo.toml` | storybook `sourceRoots` pointed at B; now `["../../../✏️s/🔌️plugins/🎞️animate"]` (same shape as the fem and block compositions) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎨️.css` | tailwind `@source` pointed at B (a framework to `✏️s` path); now `../../../🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react` (framework to framework). This removes one framework to `✏️s` reference |
| `🧰️framework/🛍️products/🎤️presentation/README.md` | layout row for the react target updated |
| `.vscode/launch.json`, `🧩️launch.seed.jsonc` | no change needed: they reference only project names (`@semio-tech/animate-js`, `@semio-tech/presentation-react`), and no project was added, removed or renamed |

Specifier used for C: `@semio-tech/presentation-react` (root `node_modules/@semio-tech/presentation-react` is a workspace symlink to C). The `rg -F '✏️editor/📺️renderer/⚛️react'` search over `✏️s 🧰️framework .vscode` (and over the whole repo for `📺️renderer/⚛️react`) finds only historical ticket files and an old `.cursor` plan afterwards.

Known stale gate, not changed: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` holds `projectionSha256` for the animate and presentation-react configs. The gate that reads it cannot run at baseline (`ENOENT` for `📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`; the fixture now lives under `📚️library/🧪️tests/🧫️fixtures/`), and a re-computation with the gate's own projection code does not reproduce the stored hash even for the unchanged `presentation` owner and for the HEAD version of the animate config. The hash for animate was therefore not edited (it would be unverifiable). Whoever repairs that gate must recompute it; the animate config now has `name: "@semio-tech/animate-js"`, which is what the fixture's `expectedName` already says.

## 5. Split layout of C

`C/🟦️.tsx` (364 lines) keeps the header, the two bare css imports (`reveal.js/dist/reveal.css`, `./🎨️.css`), the existing `export … from "./🔨️modules/🔌️pdf-canvas-port/🟦️.ts"` list, the `@semio-tech/presentation` re-exports, one `export { … } from` list per child (so every specifier of consumers stays), and the `if (import.meta.vitest)` hook (dynamic import path unchanged, since the parent did not move). The method is the one from `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md` (TypeScript-API unit analysis, hash-guarded emit, relative specifiers recomputed). Two tool changes: bare side-effect imports stay in the parent only, and the child docstring link names the barrel by symbol (`{@link presentation-react | …}`) instead of by relative path, which the taxonomy reference adapter rejects (section 6).

Children under `M`, each `🟦️.ts(x)` opens with a JSDoc whose first token is the folder emoji and that links to the parent:

| Child | File | Lines | Holds |
|---|---|---:|---|
| `🎚️deck-chrome` | `.ts` | 99 | markdown compiler box, mount options, slide-url sync, default surface chrome, reveal chrome variable sync |
| `🔀️reveal-morph` | `.ts` | 223 | resolved reveal arrangements, morph companions and target frames, centering |
| `✨️auto-animate` | `.ts` | 495 | settled state, many-to-one morph classes, ghost anchors, matcher, style-sheet patches, hidden preflight, ghost frame sync |
| `🏷️morph-context` | `.ts` | 52 | morph class names, slide epoch/aspect/enlarge/crop contexts, css size parser |
| `📑️pdf-canvas-view` | `.ts` | 213 | pdf.js loader, default port and box, pdf page and cover math, container size hook |
| `📐️figure-frames` | `.ts` | 244 | figure crop and mosaic geometry, asset url, morph frame and crop css variables |
| `🖼️figure-scroll` | `.tsx` | 618 | scroll and wheel-zoom math, zoom context, scroll viewport |
| `🔍️figure-zoom` | `.tsx` | 261 | zoomed image, video, pdf page, crop scroll content |
| `🎭️morph-views` | `.tsx` | 893 | text, authors, affiliations, bullet, figure, video, iframe, markdown, json, pdf morph views, disposition frame |
| `🧮️interaction-model` | `.ts` | 773 | transforms, placements, marquee and resize math, section measurement, flow offsets, interaction state context |
| `🫳️interaction` | `.tsx` | 1,167 | interaction hook, disposition registry, interactive dispositions and row bands, background interaction, reset, layer |
| `🧭️arrangement-section` | `.tsx` | 216 | arrangement section and surface, animate scene embed, interaction provider and bridge |
| `🎞️deck` | `.tsx` | 329 | `PresentationDeck`, `mountPresentation`, `unmountPresentation` |
| `🌳️json-tree` | `.tsx` | 129 | json preview, default tree, replaceable renderer |

Deviation from the seam map (section 2.C): the map proposed eight children with a 2,032-line `🧰️presentation-core` grab-bag. The real unit dependency graph allows fourteen cohesive children, none above 1,167 lines. Cycle fixes needed on top of the region grouping: `syncManyToOneGhostMorphFramesFromDom` goes to `auto-animate` while its frame helpers sit in `figure-frames`; the interaction state type, context and hook (`PresentationInteractionState`, `PresentationInteractionContext`, `usePresentationInteractionState`) sit in `interaction-model`, so `morph-views` does not depend on `interaction`; mount functions live with the deck.

Module DAG (import-graph SCC check over parent and children, `pd-split/graph.mjs`): 17 files, 0 SCCs, 0 children importing the parent, 0 unresolved relative specifiers. Edges: `deck` -> `deck-chrome`, `auto-animate`, `morph-context`, `arrangement-section`; `arrangement-section` -> `reveal-morph`, `interaction-model`, `interaction`; `interaction` -> `reveal-morph`, `auto-animate`, `morph-context`, `figure-frames`, `morph-views`, `interaction-model`; `morph-views` -> `deck-chrome`, `reveal-morph`, `morph-context`, `pdf-canvas-view`, `figure-frames`, `figure-scroll`, `figure-zoom`, `interaction-model`, `json-tree`; `figure-zoom` -> `pdf-canvas-view`, `figure-frames`, `figure-scroll`; `pdf-canvas-view` -> `morph-context`, `figure-scroll`, `interaction-model`; `auto-animate` -> `figure-frames`, `interaction-model`; `interaction-model` -> `reveal-morph`, `morph-context`; `figure-scroll` -> `figure-frames`; `deck-chrome`, `reveal-morph`, `morph-context`, `figure-frames`, `json-tree` are leaves.

Module-level state each lives in one module: `markdownHtmlCompiler` (`deck-chrome`), `pdfWorkerReady`, `pdfCanvasPort` (`pdf-canvas-view`), `jsonTreeRenderer` (`json-tree`), `mountedRoot`, `surfaceChromeCleanup` (`deck`), every React context in `morph-context`, `figure-scroll` (`FigureZoomContext`) or `interaction-model`/`interaction` (their own). Pre-existing hazards are untouched (conditional hook in `FigureCropMorphView` at original line 2540, `new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url)` now in `pdf-canvas-view`: bare specifier, resolution does not depend on the directory depth, but I did not build with Vite to confirm).

Longest new path: 169 UTF-8 bytes (limit 240).

Taxonomy: the 14 names were appended to `semanticDirectoryMemberKinds."members-of-modules".memberNames` (owner kind `modules`, same list that already holds `🔌️pdf-canvas-port` and `📝️markdown-html-compiler`) in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`; 198 names, no duplicates; each name is emoji + U+FE0F + kebab; no emoji repeats among the 16 siblings in `M`.

## 6. Verification, baseline vs after

All commands foreground (or detached with nohup where noted); logs in `🗑️generated/`.

**Typecheck** `node_modules/.bin/tsc --noEmit -p 🗑️generated/pd-tsconfig.json` (extends `💻️os/tsconfig.json`; files: B `🟦️.tsx`, C `🟦️.tsx`, C's deck test, the animate story, the two projektetage files; the program follows the children).
- Baseline (`pd-tsc-baseline.log`): 233 errors: B 24, B's test 62, C 0, framework deck test 0, story 0, projektetage 147 (unresolved `@semio-tech/mit-bestand-praesentation-projektetage`, implicit any; pre-existing).
- After (`pd-tsc-after1.log`, tsconfig without B): 147 errors, the sorted error list of the projektetage files is identical to the baseline (diff empty); C, all 14 children, the deck test and the story: 0 errors. With `--noUnusedLocals` (`pd-tsc-after1-unused.log`): no unused import in any child; the only hit in the new files is `morphLineTargets` (`reveal-morph`), an unused local that already existed in C.

**Exports** (`pd-split/exports.mjs`, checker `getExportsOfModule`, original text compiled in memory next to C): 219 vs 219, 0 differences in name, value/type kind and type string; the in-memory original has 0 diagnostics.

**Losslessness** (`pd-split/coverage.mjs`): 267 non-export units of the original each occur exactly once across parent and children (modulo an added `export`).

**Unit tests**
- C (`vitest run --config 🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, jsdom): baseline 3 files, 140 tests passed (`pd-vitest-c-baseline.log`); after 3 files, 140 passed (`pd-vitest-c-after.log`). Includes the in-source deck tests that import the parent.
- animate (`vitest run --config ✏️s/🔌️plugins/🎞️animate/🧪️tests/🎚️config/🟦️.ts`): baseline 4 failed suites, no tests (`Cannot find module …/📦️packages/🧰️support/…`; the B in-source tests never ran); after 1 file, 1 test passed (`pd-vitest-animate-after.log`).
- projektetage (the other consumer of C): its vitest config is broken at baseline (node environment `window is not defined`; with an override the setup file is not found; plain node also fails to load the config), so no comparable count exists; typechecking (above) covers it. Logs `pd-vitest-mb*-baseline.log`.

**Dependency-cruiser** (`bun x dependency-cruiser 🧰️framework/🛍️products/🎤️presentation ✏️s/🔌️plugins/🎞️animate --config …/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err`, `pd-depcruise-after.log`): 832 modules cruised (transitive framework closure included), 1,022 violations repo-wide in that closure. Violations whose subject is the presentation product or its new modules: one `no-circular`, `C/🟦️.tsx` <-> `🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx` (the in-source test hook; it existed before the split). None involves a new child module. Animate: `plugins-framework-sdk-only` hits on the deck story (`→ presentation` and `→ presentation-react`, pre-existing since the story already imported them) and on pre-existing script/io/schema files. No new cross-plugin edge and no new framework to `✏️s` edge was added; the dev `🎨️.css` framework to `✏️s` source reference to B was removed. I did not produce a depcruise run of the old layout (the originals no longer exist in the tree), so this is an after-only result backed by the graph check above.

**Taxonomy** (`bun ./📜️script.ts verify taxonomy report --scope <C>`; about 9 minutes under load; logs `pd-taxonomy-after-c.log` (first run) and `pd-taxonomy-after-c2.log` (final)):
- First run after the split: `errors=140`. 14 of them were `reference-syntax-unsupported` on my new children, because the child docstring link `{@link ../../🟦️.tsx | …}` is a path-bearing token that the reference adapter rejects; one more was a `reference-edit-required` on the animate story, caused by a literal path I had written into its header comment. Fixed: the child links are now `{@link presentation-react | the barrel}` and the story header names no path.
- Final run: `clean=false errors=125`. There is no `directory-kind-unresolved` anywhere (the 14 registrations work), no finding on the animate plugin, and no `reference-*` finding on any new child. The 125 are: 71 `reference-edit-required` and 38 `reference-syntax-unsupported` (none in the presentation product's new files; the 38 are `.rs` files of other plugins that the scope walk pulls in), 2 `normalization-move-required`, 1 `semantic-stem-unresolved`, 1 `package-implementation-destination-unresolved` (`🧰️vitest.setup.ts`), 1 `fixed-source-disposition-unresolved` (`📜️script.ts`), 1 `generator-preview-invalid` and 10 `collision-*`.
- The 10 `collision-*` findings (byte, case-fold, nfc, same-kind, vs16-fold, once for the `🟦️.ts` group and once for the `🟦️.tsx` group) come from the planner's flat hoist of everything under `📦️packages/🟦️typescript/🎯️targets/⚛️react/` into `🎤️presentation/🟦️.ts(x)`. The `.ts` group already contained the two pre-existing modules (`🔌️pdf-canvas-port`, `📝️markdown-html-compiler`) before my change, so the mechanism is pre-existing; my children widen that group and add a `.tsx` group (parent plus six `.tsx` children). The baseline scope run was killed by my own 500 s alarm before it finished (`pd-taxonomy-baseline-c.log`, no verdict), so I cannot give a baseline total. This planner behaviour affects any module split under a `📦️packages/…/🎯️targets` implementation and needs a decision from its owner.

**Dependency truth gate** (`bun ./📜️script.ts verify dependencies literal-external`, `pd-deps-gate.log`): fails with `target=0, current=2543, oracle-conflicts=3` (rust `image`, `quick-xml`, `serde_json` declared by several manifests, including toolchain checkouts under `.cache`). Nothing in the output concerns `pdfjs-dist`, `reveal.js` or the animate manifest, so my package.json and `🔒️dependencies.json` edits introduced no visible finding; I have no baseline run of the gate to compare totals.


## 7. Not done / limits

- No browser, Vite build or Storybook run: the deck story is only typechecked. `storybook-discovery` (`bun test`, `pd-storybook-discovery.log`) fails 4 of 5 tests for causes outside this change (a missing `ui` source root and a missing OS story file; I did not run it before my edits, so that is inferred from the causes) before it reaches the animate scope; the animate `sourceRoots` entry now exists on disk and has the same shape as fem/block.
- `verify taxonomy` was run on the C scope only.
- `InteractiveDisposition` (530 lines) and `usePresentationInteraction` (250 lines) remain single units inside the 1,167-line `🫳️interaction`; further decomposition needs hook-level work.
- Rust-side animate crates (`semio-hub-animate`, artifact crates) were not built; only the `Cargo.toml` metadata value changed.
- Temporary helper scripts remain in `🗑️generated/pd-split/` (`analyze.mjs`, `split.mjs`, `write.mjs`, `cfg-c.mjs`, `exports.mjs`, `coverage.mjs`, `graph.mjs`, `groups.mjs`, `explore.mjs`, `sha.ts`, `scratch/`); no `[DEBUG]` logs were added to any source file.

## 8. Follow-up: collision-free placement (coordinator request)

### 8.1 What the collision rule checks (read from `🧹️normalization/🟦️.ts`)

- `collisionGroups` (line about 5780) buckets every inventory entry by its planned destination `entry.normalizedPath` under five comparisons (byte, same-kind, nfc, case-fold, vs16-fold). Two entries with the same destination are a `collision-*` error.
- `normalizedPath` is `canonicalFile(...).path`, except for package implementations (`classifyPackageRole` returns `implementation`): then `packageImplementationDestination` replaces it with `<package owner>/<semantic dir>/<kindOnly>` for files with a stem that matches a directory kind, and with `<package owner>/🟦️.ts` (or `.tsx`) for stemless leaves. Every implementation leaf is named `🟦️.ts(x)` (a different basename is itself a finding), so any package that holds more than one implementation leaf of the same extension has them all hoisted onto one path. The owner is the directory above `📦️packages`, here `🎤️presentation`.
- `classifyPackageRole` marks a file under `📦️packages/<language>/…` as implementation unless it is glue by the TypeScript glue grammar (declaration, registration, bootstrap, thin delegation, at most 32 delegation statements) or a fixed entry contract. A thin `export *` entry is glue and is not moved.
- Consequence: implementation code must not live under `📦️packages`. That is the layout the other products use: `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` and its children (`🎠️runtime`, `🖌️render`, …) hold the implementation, and `📦️packages/🟦️typescript/🟦️.tsx` is `export * from "../../🟦️.tsx"`. The pre-existing `.ts` collision was exactly this: `🔌️pdf-canvas-port` and `📝️markdown-html-compiler` (two implementation leaves of one package) and, after my first split, the 14 children and the barrel.

### 8.2 Change

- Moved the barrel and all 16 modules (my 14 plus the two pre-existing ones) from `C/` to `T/`: `T/🟦️.tsx` (364 lines) and `T/🔨️modules/<name>/🟦️.ts(x)`. The 14 names were already registered in `members-of-modules` (owner kind `modules`), which also covers `T/🔨️modules`; no new registration was needed.
- `C/🟦️.tsx` is now a 3-line thin entry: `import "./🎨️.css"; export * from "../../../../🎯️targets/⚛️react/🟦️.tsx";`. `C/package.json` is unchanged (`exports["."] = "./🟦️.tsx"`, `exports["./🎨️.css"]`), so the specifier `@semio-tech/presentation-react` and every alias (mit-bestand vite builder and test config, the animate story, root `node_modules` symlink) keep working. The stylesheet stays in the package (a package export must be inside the package) and is loaded by the entry; the barrel in `T` keeps `import "reveal.js/dist/reveal.css"`.
- Updated the paths that moved with the code: the in-source test import in `T/🟦️.tsx` (`../../🧪️tests/…`), the three module tests and the deck test under `🎤️presentation/🧪️tests/` (now import from `../../🎯️targets/⚛️react/…`; the deck test reads the stylesheet from the package through one `stylesheetRelativePath` constant instead of a sibling file), the vitest config (`coverage.include` and `includeSource` now name `../../../../🎯️targets/⚛️react/🟦️.tsx`), `C/📋️project.json` (added `{workspaceRoot}/…/🎯️targets/⚛️react/**/*` as an Nx input so edits to `T` invalidate the test target), the dev `🎨️.css` `@source` (now `T`), and the README layout table.
- Not moved: `🎨️.css`, `package.json`, `📋️project.json`, `📜️script.ts`, `🧰️vitest.setup.ts` (all package files; the planner still wants the css and setup file at other places, which it already wanted before).

### 8.3 Results

| Check | Result |
|---|---|
| `verify taxonomy report --scope 🧰️framework/🛍️products/🎤️presentation` (whole product, wider than before; `pd-taxonomy-after-c3.log`) | `errors=109`. `collision-*`: 10, none in `T` or in the package: 5 for `🎤️presentation/🟦️.ts` vs `📦️packages/🟦️typescript/🟦️.ts` (the product root glue file and the core model package, see 8.4) and 5 for `🧪️tests/🧫️fixtures/📝️markdown-html-compilation/📝️.md`. The `.ts` group of the two modules and the `.tsx` group of the children are gone: 0 collisions involve the react target. No finding at all (of any kind) names a file under `T/`, no `directory-kind-unresolved` on `T/🔨️modules/*`, no `reference-syntax-unsupported` in the product. `warnings=0`, so the thin entry is classified as glue (otherwise `package-implementation-file` would warn) |
| `tsc --noEmit -p pd-tsconfig-after.json` (T barrel, package entry, deck test, story, projektetage files) | 147 errors, sorted list identical to the baseline minus B (`pd-tsc-after3.log`); 0 errors in `T`, the 14 children, the package entry, the deck test and the story |
| `vitest run` on the C config | 3 files, 140 tests passed (`pd-vitest-c-after3.log`); an intermediate run failed one suite until the deck test's stylesheet path was fixed, as described above |
| animate vitest | 1 file, 1 test passed |
| export identity (`pd-split/exports.mjs`, original text compiled in memory next to `T/🟦️.tsx`, and again against the package entry `C/🟦️.tsx`) | 219 vs 219, 0 differences in name, kind and type string, both for the barrel and for the thin entry |
| losslessness (`pd-split/coverage.mjs`) | 266 of 267 units exactly once; the one deviation is the in-source test unit, whose dynamic import path I changed on purpose (`../../../../🧪️tests` to `../../🧪️tests`) |
| module graph (`pd-split/graph.mjs`) | 17 files, 0 cycles, 0 child imports the parent, 0 unresolved relative specifiers |
| dependency-cruiser (same command as before, `pd-depcruise-after3.log`) | 1,023 violations vs 1,022 before this move: the same rule counts except `not-to-unlisted` +1 (see 8.4) |

### 8.4 What stays open (documented, not fixed)

- `🎤️presentation/🟦️.ts` (the product root, 166 bytes, `export * from "./📦️packages/🟦️typescript/🟦️.ts"`) collides with `📦️packages/🟦️typescript/🟦️.ts`, the 69 KB core model package: the core model is an implementation leaf inside a package and is planned onto the product root. This pair does not involve any file I own or created, existed before this task, and the fix is the same relocation (core implementation to the product root or to a stemmed module), which would change the `@semio-tech/presentation` package entry, `node_modules` symlinks and four lockfiles. I left it for its owner.
- `🧪️tests/🧫️fixtures/📝️markdown-html-compilation/📝️.md` (5 collisions) and 31 `semantic-stem-unresolved` on the fixture `.md` files, the eight `directory-kind-unresolved` on test directories and `taxonomy/target-inside-package-boundary` for `📦️packages/🟦️typescript/🎯️targets` are pre-existing and unrelated to the children. The last one says the react package is nested the wrong way round (the target should own its package, as in `🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript`); moving the package would rewrite the workspace paths in six lockfiles, `node_modules` symlinks, `📋️build-owners.json` and the project schema path, so I did not do it.
- Side effect of moving the code out of the package: depcruise `not-to-unlisted` now flags `T/🔨️modules/📑️pdf-canvas-view/🟦️.ts` for its `import("pdfjs-dist")`, because the nearest manifest above `T` is the repo root, which does not list `pdfjs-dist`; before the move the file sat below the package manifest that lists it. This is the same class as the 54 existing `not-to-unlisted` findings (for example `🖱️ui/🧱️elements/🔌️Ports` importing `three`). It disappears when the package moves under the target (previous item) or when the manifest that lists the dependency becomes an ancestor of `T`.
- The baseline taxonomy run on the old layout never finished (my alarm), so "no new findings" is argued per file, not by totals: every finding that names a file I created or moved is gone, and the findings that remain name files that already carried them.
