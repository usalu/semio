# Result: Asset and JavaScript Edges (slice ASSET-JS-EDGES, prefix `aje`)

Status: DONE (2026-10-16 20:56 to 21:55). Tools: `aje-tools/` (this folder; `apply.ts` is the declarative batch, the rest are census, check and verification helpers; the psc3 tools were reused for the workspace list, locks, tsc overlay, test runner and cruiser diff). Logs and raw outputs: `🗑️generated/aje-*` (kept). Folder `F` = this folder.

## 1. Verdict

- All 31 edges of `RESULT-PACKAGE-SPECIFIER-CODEMOD-3.md` section 8.2 are gone. In the exact cruiser run, the 31 (importer, target) pairs matched 31 of 31 before and 0 of 31 after. The 62 error lines of those pairs (55 `no-cross-package-relative-*`, 4 `no-cross-technology-✏️s-to-🧰️framework`, 1 `no-cross-technology-🌎️hub-to-🧰️framework`, 2 `plugins-framework-sdk-only`) became 2 lines: the two puzzle story edges of `plugins-framework-sdk-only` now point at the new framework-os facades instead of the image files (same violation, same count).
- `not-to-unlisted`: 405 lines before and after, set difference 0. `no-circular`: 568 lines before and after, set difference 0. `plugins-framework-sdk-only`: 627 and 627. Whole run: 8,858 error lines before, 8,813 after.
- **No file had to move.** The census (section 3) shows that every one of the 25 distinct target files is used by its own owner package and by at least one other package. By the coordinator ruling they stay with the owner and get explicit `exports` entries, and consumers import them through the package specifier. No copies, no symlinks, no wildcard.
- **The ruling needed one correction in mechanism.** An `exports` target must lie inside the package directory, and every owner here is a "glue" package (`📦️packages/🟦️typescript` holds only `package.json` plus facades; the real files sit in the region above). So an export cannot point at the asset itself. The owner exports a one-line TypeScript facade next to its other facades, and the facade re-exports the asset with the query suffix (`?raw`, `?url`, `?inline`) or, for JavaScript, imports the `.js`/`.mjs`. This is the established pattern (JSON fixtures are exported the same way). The query lives with the owner; importers write no suffix. Proven with a real Vite resolve and load (section 5).
- Skipped areas: none of the 31 edges has an importer or target in `📚️library`, `🔌️plugin/**` or renderer TS. Nothing was skipped for that reason. Related edges I saw and left alone are in section 7.
- Not run: the launcher (`bun repo run …`), `workspaces-check`, `verify taxonomy`, `verify layering`, `bun install`, Storybook itself, and the flow-core script targets that import the four JavaScript hosts (they need the launcher environment). Everything else in the task's verification list was run.

## 2. The 31 edges

| # | Importer (line) | Target | New import |
| --- | --- | --- | --- |
| 1-2 | puzzle 3d story `🎭️3d-world/🧪️.story.tsx` (18, 19) | `♾️infinite/🖼️assets/…grundriss/🖼️.jpg`, `….png` (`@semio-tech/framework-os`) | `@semio-tech/framework-os/modules/infinite/assets/abbau-aufbau-masterarbeit-grundriss` and `…/rathaus-ahlen-grundriss` |
| 3-4 | `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination` (44, 45) | fem 2d and 3d `🖼️assets/🎬️demo/🗣️.dsl.semio` | `@semio-tech/fem-2d/standards/1/subsets/any/assets/demo`, `@semio-tech/fem-3d/…` |
| 5-10 | `✏️s/🧑‍💻dev/🧱️block/📖️stories/🧭️coordination` (39-44) | block 2d left and right, 3d left and Nakagin, 5d left and Nakagin `🗣️.dsl.semio` | `@semio-tech/block-{2d,3d,5d}/examples/<example>/assets/<asset>` |
| 11 | same file (46) | `🖼️assets/🌱️metabolism/…/🧊️capsule_J.glb` | `@semio-tech/assets/metabolism/representation/capsules/j/capsule-j` |
| 12-15 | flow core `📜️script.ts` (97, 111, 112, 121) | `🕸️wasm/🧪️tests/{🧬️schema-oracle,🖥️host,🎬️draw-list,⏱️consumed-browser-clock}/🟨️.js` | `@semio-tech/framework-os/modules/flow/wasm/testing/{schema-oracle,host,draw-list,consumed-browser-clock}` (dynamic imports) |
| 16 | ifc sqlite snapshot test (16) | `📇️registry/…/🚦️sqlite-cohort/🔣️.json` | `@semio-tech/stdio-registry-ts/contract/part21/testing/fixtures/sqlite-cohort` |
| 17 | gltf fixture corpus test (1) | `📇️registry/🧬️schema/🔣️.json` | `@semio-tech/stdio-registry-ts/schema` |
| 18 | puzzle browser contribution test (7) | `🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs` | `declaredBrowserSessionEnginesV1` from `@semio-tech/framework-os-dev/engine/selection` (existing key, extended) |
| 19 | hub stdio oracle composition test (10) | `🧪️test/🕸️dependencies/🟨️.mjs` | `@semio-tech/repo-test/dependencies` |
| 20-26 | 7 quiz tests (`🌗️contrast-states`, `🏠️home-grid`, `💭️crowd-client`, `📐️adaptive-layout`, `📡️presence-client` line 53, `📱️home-text-and-viewport`, `🚶️journey-and-tabs`) | `🎯️targets/⚛️react/🎨️.css?raw` | `@semio-tech/quiz-react/stylesheet` |
| 27-30 | quiz `🕷️radar-geometry` (32-35) | four `🚀️anta/…/🔤️outline.ttf?inline` | `@semio-tech/assets/fonts/anta/{latin,latin-ext,math,symbols}/regular/outline` |
| 31 | quiz `📡️presence-client` (52) | `👆️cursor/🖱️default/☀️light.svg?raw` | `@semio-tech/assets/cursor/default/light` |

New exports (24 keys, 24 facades, each one file `<mirror dir>/🟦️.ts` in the owner's package dir, first line a docstring with its own emoji, `/// <reference types="vite/client" />` where a query suffix is used):
- `fem-2d`, `fem-3d`: 1 key each. `block-2d` 2, `block-3d` 2, `block-5d` 2. The block keys drop `standards/1/subsets/any/` (the facade path with it was 266 to 267 bytes, over the 240-byte limit; the mirror rule of psc2 for long paths). Longest new path now under 240 bytes (checked for all 24).
- `framework-os`: 2 image urls and 4 flow wasm test hosts (the hosts sit next to the existing `retained-receipt`/`retained-grant`/`open-ownership` testing facades).
- `assets`: capsule J mesh url, 4 inlined fonts, 1 cursor svg.
- `quiz-react`: `./stylesheet`. `repo-test`: `./dependencies`. `framework-os-dev`: no new key; `./engine/selection` also exports `declaredBrowserSessionEnginesV1`, and a new `🟨️.d.mts` types the `.mjs`.
- New package `@semio-tech/stdio-registry-ts` at `✏️s/🔌️plugins/🗄️stdio/📇️registry/📦️packages/🟦️typescript` with 2 keys (`./schema`, `./contract/part21/testing/fixtures/sqlite-cohort`). The registry had no TypeScript package. Its region has no other TypeScript or JavaScript importer from outside (AST scan: 12 hits, all inside the region plus the two edges), so the new region creates no further edge.

## 3. Why nothing moved: consumer census

Tool: `aje-tools/census.ts` (`git grep -F` of the target's distinctive path tail, grouped by owning package, longest `sourceOwner` prefix). Raw output: `🗑️generated/aje-census.txt`. "Owner" counts the file's own package (Rust `include_str!`/`include_bytes!`, own tests and fixtures, project inputs).

| Target group | Packages that reference the file (besides the importer of the edge) |
| --- | --- |
| fem 2d/3d demo DSL | owner (Rust `include_str!` in `📚️examples/🎬️demo` and `🚪️io` snapshot, owner tests, 5 hits each) |
| block concrete forest left/right, Nakagin | owner (6 to 12 hits per file in Rust and TS tests), and for the left forest also `stdio` `🧿️semio` (python tests, 8 hits) |
| abbau-aufbau jpg | stdio `jpg` (11), `binary` (6), `tiff` (2), `infinite` own story, wgpu renderer notes, puzzle 3d |
| rathaus-ahlen png | stdio `png` (3), `tiff` (2), `bmp` (2), `infinite` own story, puzzle 3d |
| capsule J glb | assets catalog and mesh tests (owner), mesh-engine tests, wgpu renderer tests |
| Anta fonts | `◻️2d` font catalog (`new URL`), wgpu renderer, framework text fixtures, process image specimens |
| cursor svg | `ui-styling` css (2), wgpu cursor Rust + fixture (2) |
| quiz stylesheet | owner `🟦️.tsx`, project inputs, 12 product-level tests and READMEs, pets and presentation reference their own css |
| flow wasm JS hosts | owner region (`🌊️flow/🧪️tests/📋️declarations` declares all of them, the ownership fixture lists them, canvas fixtures name `draw-list`), flow core build script and project inputs |
| dev selection `.mjs` | owner (its own contribution test, 3 hits), library (6 hits: inference `.mjs`, caching tests, fixtures), puzzle test |
| repo-test dependencies `.mjs` | owner (11 hits), library (4 hits), hub stdio test |
| registry schema JSON | 8 other packages (mp4, json, gltf, hub, `✏️s` dev and service tests, wgpu project, library, os derive index) |
| sqlite-cohort JSON | owner registry tests (3), ifc (7), step (3) |

So no file has exactly one consuming package. The rule "shared stays with the owner and gets an explicit export" applies to all of them.

## 4. Other edits

- Importers: 17 files (table above), plus the vitest config of `quiz-react` (`🧪️tests/🎚️config/🟦️.ts`): the alias `{ find: "@semio-tech/quiz-react" }` became `{ find: /^@semio-tech\/quiz-react$/u }`. A string alias in Vite also swallows subpaths, so the old form rewrote `@semio-tech/quiz-react/stylesheet` to `…/🟦️.tsx/stylesheet`. Proven in `aje-tools/alias-check.mjs` (old alias: wrong id; exact alias: the facade).
- Manifests (`workspace:*`): `s-dev-block-ts` (devDependencies block-2d, block-3d, block-5d, assets), `puzzle-3d-ts` (framework-os), `flow-core-build` (framework-os), `stdio-ifc` and `stdio-gltf` (stdio-registry-ts), `hub-stdio-test-oracle` (repo-test), `quiz` (assets). Not added: `quiz-react` to `quiz` (the existing tests already import it unlisted; listing it would make a workspace cycle, since quiz-react depends on quiz). Exports added to the 11 owner manifests.
- `flow core 📋️project.json`: the four facade paths added next to the four JavaScript host inputs (both input lists).
- `workspaces` of `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech` (one entry each for the new package; byte-sorted). Locks: `lockgen.sh` for the six scopes (`bun install --lockfile-only --offline`, checked-in lock restored at once), then `psc3-tools/lock.ts --write`: root 2 and teaching 2 changed workspace entries; `✏️s` 6 changed, 1 workspace and 1 package row added; hub 7/1/1; mit-bestand 6/1/1; semio-tech 6/1/1. Backups: `🗑️generated/aje-lock-backup/`. The same two unrelated third-party notes as in psc3 (`is-promise`, `readable-stream`).
- Links (untracked, `bun install` makes them): `node_modules/@semio-tech/stdio-registry-ts`, `node_modules/@semio-tech/block-5d` (the latter was missing, which the Vite resolve check caught).
- Nothing else references the files as moved, because nothing moved: no `include_bytes!`, fixture reader, `publicDir`, asset glob or taxonomy entry changed. The 24 new facade folders resolve in the taxonomy (`frs-tools/taxcheck.ts`: checked 24, unresolved 0).
- Not touched: `AGENTS.md`, `.vscode` (no new command), the rule generator, any ticket tool, any git state. No `[DEBUG]` log.

## 5. Verification (real results)

Cruiser, exact command (`aje-tools/cruise.sh`, alarm 7200 s): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err`.

| Run | Window | Exit | Error lines | Modules / dependencies | Raw |
| --- | --- | ---: | ---: | --- | --- |
| before | 20:57 to 21:06, before my first edit | 154 | 8,858 | 18,469 / 51,789 | `aje-before-err.txt` |
| after 1 | 21:31 to 21:41, all edits except the `🟨️.d.mts` | 107 | 8,811 | 18,512 / 51,917 | `aje-run1/aje-after-err.txt` |
| after (final) | 21:43 to 21:54, final tree | 109 | 8,813 | 18,518 / 51,938 | `aje-after-err.txt` |

(The exit code is the error count modulo 256.)

| Rule (all / outside tests) | before | after (final) |
| --- | ---: | ---: |
| `no-cross-package-relative-*` lines | 4,928 / 2,899 | 4,888 / 2,875 |
| `no-cross-package-relative-*` distinct edges | 4,006 / 2,433 | 3,995 / 2,428 |
| `not-to-unlisted` | 405 / 0 | 405 / 0 |
| `no-circular` | 568 / 487 | 568 / 487 |
| `ui-no-framework-packages` | 297 / 232 | 297 / 232 |
| `framework-no-products` | 176 / 117 | 176 / 117 |
| `plugins-framework-sdk-only` | 627 / 558 | 627 / 558 |
| `renderer-hosts-only-ui` | 371 / 198 | 371 / 198 |
| `no-core-path` | 904 / 556 | 905 / 556 |
| `no-cross-technology-hub-to-framework` / `s-to-framework` | 1 / 4 for my pairs | 0 / 0 |
| total error lines | 8,858 | 8,813 |

Attribution (`errdiff.ts`, ASCII keys): family lines removed 67, added 27; `no-core-path` removed 2, added 3; `plugins-framework-sdk-only` removed 2, added 2; hub-to-framework removed 1; s-to-framework removed 5; `no-circular` and `not-to-unlisted` sets identical. The 27 added family lines: 27 of 27 have importers in new library files of peers (`📚️library/🏗️build/🛠️cmake/**` and `…/📥️inputs/📋️data/**`, rule `@semio-tech/process`); the 3 added `no-core-path` lines are peer files in the library and in process. None of the added lines has an importer or target in a file of mine except the 2 `plugins-framework-sdk-only` lines that replace the 2 old ones. The 12 further family removals and the one extra `s-to-framework` removal are in files I did not touch (peers edited during the runs); I did not attribute them in detail.

| Check | Result |
| --- | --- |
| `tsc --noEmit` over the 42 touched or created TypeScript files plus their closure (13,548 files). Before = the tree before my edits through the compiler host overlay of the batch snapshot (`overlay-tsc.mjs`); after = `tsc -p` on the final tree | 2,073 diagnostics before, 1,903 after (`tscdiff.ts`: 174 removed, 4 added; the 4 added are in peer library test files, the removals are peer schema fixes made between the two runs). In my 42 files: 12 diagnostics before, 11 after; the 11 are identical (stdio-ifc missing package link, puzzle story wasm `pkg`, the hub test's generic call signatures); the one removed is `TS7016` (no types for the dev `.mjs`), fixed by the new `🟨️.d.mts`. After the first run it had moved from the puzzle test into the dev facade; the `.d.mts` removes it everywhere. Files: `aje-tsc-before.txt`, `aje-tsc-after.txt`. |
| Bun test per file, 5 files (ifc sqlite snapshot, gltf fixture corpus, hub stdio composition, dev contribution, flow declarations) | identical: 642 pass and 6 fail before, 642 and 6 after, same failing tests (`testdiff.ts`: 5 same, 0 worse, 0 better) |
| Puzzle browser contribution function (`testPuzzleBrowserContributionV1`) | passes before and after |
| Vitest, quiz-react config, the 8 touched quiz suites | before: 8 suites fail at load, 0 tests; after: the same 8 suites fail with the same errors. Cause (not mine): the config's string aliases `@semio-tech/framework` and others swallow subpaths such as `@semio-tech/framework/transient` and `/manifest`. So this route cannot execute the new imports. They are proven through the equivalent Vite resolution below. |
| Real Vite plugin container, all 24 new export keys (`aje-tools/vite-check.mjs`) | 24 of 24 resolve to their facade; 21 loaded through Vite's SSR loader (the 3 side-effect hosts only resolve here, see the next row); every `?raw` value equals the file text byte for byte, every `?inline` value is a data URL, every `?url` value is a url; 0 failures. Log: `aje-vite-check.log` |
| Bun load test (`aje-tools/load-check.ts`, fresh process, repository root) | `repo-test/dependencies` (`packagesForOwner`), `framework-os-dev/engine/selection` (both functions), both `stdio-registry-ts` keys, `consumed-browser-clock` (`testFlowBrowserClock`): all load |
| JavaScript hosts through the facades, from the flow core script directory, against the direct import of the `.js` before | `consumed-browser-clock`: loads before and after. `host`, `draw-list`, `schema-oracle`: the module graph loads, then the same `ENOENT` on a stale fixture path before and after (`🕸️wasm/🧫️fixtures/…`, `🖼️canvas/🧫️fixtures/…`; the fixtures moved under `🧪️tests/🧫️fixtures` in the taxonomy sweep). Identical outcome (`aje-js-before-full.txt`, `aje-js-after.txt`). |
| sha256 (`aje-tools/sha.ts`) | the 19 distinct asset and data files of the batch are byte-identical to their `HEAD` blobs (nothing moved or changed); the four inlined font data URLs decode to the exact file bytes. `aje-sha.log` |
| `aje-tools/alias-check.mjs` | old string alias maps the stylesheet specifier to `…/🟦️.tsx/stylesheet`; exact-match alias maps it to the facade |

## 6. Commands run

| # | Command | Result |
| --- | --- | --- |
| 1 | `aje-tools/cruise.sh before full` | exit 154, 8,858 lines |
| 2 | `dump-policy.cjs`, `census.ts`, `importers.ts`, `refs.sh`, `taxcheck.ts` (read-only) | section 3; registry importers 12 hits; taxonomy 24 of 24 |
| 3 | baselines: `runtests.ts` (5 files), vitest 8 suites, puzzle function, JS host direct imports | section 5 |
| 4 | `apply.ts --write --snapshot` | 24 facades, 25 created files incl. the new manifest, 35 edited files, 32 importer edits, 17 manifests; 0 problems; every anchor unique, files re-read before the write |
| 5 | `workspaces.ts --write`, two `ln -s` links | 4 scope lists; 2 links |
| 6 | `vite-check.mjs`, `load-check.ts`, `sha.ts`, `alias-check.mjs` | section 5 |
| 7 | `lockgen.sh` six scopes, `lock.ts --write` six scopes | section 4 |
| 8 | `tsc.sh after`, `overlay-tsc.mjs` before, twice after (before and after the `.d.mts`) | section 5 |
| 9 | `runtests.ts` after, vitest after | section 5 |
| 10 | `cruise.sh after full` twice | section 5 |
| 11 | not run: `bun repo run …` (launcher), `workspaces-check`, `verify taxonomy`, `verify layering`, `bun install`, Storybook, the flow core script targets | see section 1 |

## 7. Foreign findings and hand-offs (not fixed)

1. **psc3 section 8.2 was wrong for CSS.** `@semio-tech/quiz-react/🎨️.css` is a one-line `@import "../../🎨️.css";` stub; with `?raw` it returns that line, not the stylesheet. The same holds for `ui-styling` (`🎨️palette.css`, `🖌️ui.css`): the three UI-list quiz test edges that psc3 proposed to point at those keys (`🌗️contrast-states` lines 20, 21 and `💭️crowd-client`) would read the stub too. They need the same kind of raw-text facade, and the vitest alias fix. They belong to the UI peer's list and I left them.
2. **Storybook aliases swallow subpaths.** `.storybook/main.ts` maps `@semio-tech/assets` and `@semio-tech/ui-styling` to the package directory by string alias, which substitutes by prefix: `@semio-tech/assets/mesh` becomes `<package dir>/mesh`, which does not exist (the facade is `🥽️mesh/🟦️.ts`). This already affects the story's existing `@semio-tech/assets/mesh` imports and now also the new glb import. The package-dir aliases should become exact-match regexes, as I did for quiz-react. The 24 keys resolve in a plain Vite container (section 5) but I did not run Storybook.
3. **The quiz-react vitest route is red at HEAD** (8 of 8 touched suites; the first failing import is `@semio-tech/framework/transient` or `/manifest`): the config's string aliases for `@semio-tech/framework`, `@semio-tech/ui-react`, `@semio-tech/pets` need the same exact-match treatment.
4. **Stale fixture paths in the flow wasm JavaScript hosts** (`host`, `schema-oracle`, `draw-list`): they read `../../🧫️fixtures/…` and `…/🖼️canvas/🧫️fixtures/…` which moved under `🧪️tests/🧫️fixtures` in the taxonomy sweep. All three fail with `ENOENT` at runtime, before and after my change.
5. **`plugins-framework-sdk-only` for the puzzle story:** a plugin story imports the infinite floor-plan images from `@semio-tech/framework-os` (before: by relative path). The violation is unchanged; the clean fix is a framework SDK export for story media, or the story owning its two reference images. Not decided here.
6. **Misplaced tests:** the quiz tests that exercise `quiz-react` (stylesheet, DOM, contrast) live in `❓️quiz/🧪️tests` and are run by the quiz-react config. They belong in `🎯️targets/⚛️react/🧪️tests`, where the stylesheet would be an intra-package file and no export would be needed. Moving all of them is a separate slice.
7. **Remaining relative edges of the same family, not in the 31:** the dev and repo-test `.mjs` files import `📚️library/🏗️build/👁️observation/🟨️.mjs` by relative path (parent, library); the quiz test `📡️presence-client` also imports `ui-styling` css and `ui-react` internals (UI list); `import.meta.glob`/`readFileSync` readers of the quiz css and pets css are not cruiser edges and were left as they are.
8. Project inputs for the quiz-react test target do not list the assets files that `radar-geometry` and `presence-client` read (they did not before either); the Nx hasher cannot see them.

## 8. Files changed

- Created: 24 facades (listed by key in `aje-tools/apply.ts` rows; one `🟦️.ts` each in the package dirs of `fem-2d`, `fem-3d`, `block-2d/3d/5d`, `framework-os`, `assets`, `quiz-react`, `repo-test`, `stdio-registry-ts`); `✏️s/🔌️plugins/🗄️stdio/📇️registry/📦️packages/🟦️typescript/package.json`; `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.d.mts`.
- Edited: the 17 importers and the quiz-react vitest config of section 4; `engine/selection` facade of `framework-os-dev`; flow core `📋️project.json`, `package.json` and `📜️script.ts`; 17 `package.json` files (11 exports, 7 dependency lists, with overlap); 4 scope `package.json` workspace lists; 6 `bun.lock`.
- Snapshot of every pre-edit file: `🗑️generated/aje-snap/batch.json` (path to text or null); pre-edit scope lists: `🗑️generated/aje-snap/ws/`.
- Untracked environment: two `node_modules/@semio-tech/*` links.
