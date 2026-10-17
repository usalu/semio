# Result CENSUS4-PKG-46 (prefix `c4p46`): os/renderer TypeScript element hosts split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`. Package 46 of `F/MONOLITH-CENSUS-4.md` section 4. Tools in `F/c4p46-tools/` (copied from `c4p27-tools`), raw outputs in `F/🗑️generated/c4p46-*`, pre-edit snapshots in `F/🗑️generated/c4p46-snap/` (`SHA.txt`, `MAP.txt`).

## 0. Verdict

**DONE.** All six files are split, installed and verified; nothing is half applied (resumed 17:33 after the usage-limit cut: region balance, name check and file inventory re-run, no half-applied batch found). Roots are 55 to 344 lines, the 45 new files 30 to 349 lines (limit about 1,000), longest new path 208 UTF-8 bytes.

| # | Target (under `E/`) | Before | Root after | Children (largest) | Mode |
|---|---|---:|---:|---|---|
| f1 | `🖋️InkCanvasHost/🟦️.tsx` | 1,782 | 149 | 10 (`👆️ink-pointer` 323) | 6 module children by region, `InkCanvasHost` (794 lines) cut into 4 hooks over one typed `core` bag |
| f2 | `🖥️Board2dHost/🟦️.tsx` | 1,814 | 102 | 9 (`🔁️board-lifecycle-and-sync` 295) | 4 module children by region, `Board2dHost` (1,153 lines) cut into 5 hooks |
| f3 | `🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx` | 1,588 | 344 | 8 sibling test folders (`🧷️refresh-and-mutation-pages` 280) | 7 sibling vitest files plus one shared support module, registered in the react vitest config and `.vscode` |
| f4 | `🛠️ShellHelpers/🔩️plumbing/🟦️.ts` | 1,005 | 204 | 6 (`📥️downloads-and-file-requests` 244) | module children |
| f5 | `🛠️ShellHelpers/🪟️window-panes/🟦️.tsx` | 1,106 | 245 | 5 (`📝️staged-args` 336) | module children |
| f6 | `🧭️TiledMapHost/🟦️.tsx` | 1,296 | 55 | 7 (`🖼️map-renderer` 349) | 4 module children (incl. the 337-line `MapRenderer` class), `TiledMapHost` (571 lines) cut into 3 hooks |

Sum: 8,591 lines before; 904 lines in the six roots plus 7,790 lines in the 45 new files (the hook scaffolding, `core` destructures and docstrings are added).

## 1. Found state

- Fresh slice, started 2026-10-17 15:15. No earlier `c4p46` tool folder, snapshot or log existed.
- None of the six targets lies under a `🧊️wgpu` target folder, so nothing is skipped on that ground.
- Re-measured (snapshot `f1..f6`, taken before any edit; the census counted 8,587 lines, the live tree has 8,591 because peers touched f3 and f4 in the meantime):

| # | Target (under `E/`) | Lines | Largest unit |
|---|---|---:|---|
| f1 | `🖋️InkCanvasHost/🟦️.tsx` | 1,782 | `InkCanvasHost` 794 lines |
| f2 | `🖥️Board2dHost/🟦️.tsx` | 1,814 | `Board2dHost` 1,153 lines |
| f3 | `🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx` (vitest) | 1,588 | 13 regions, 25 `describe` blocks |
| f4 | `🛠️ShellHelpers/🔩️plumbing/🟦️.ts` | 1,005 | 78 small units |
| f5 | `🛠️ShellHelpers/🪟️window-panes/🟦️.tsx` | 1,106 | `buildActionCategoryTree` 98 lines |
| f6 | `🧭️TiledMapHost/🟦️.tsx` | 1,296 | `TiledMapHost` 571 lines, `MapRenderer` 337 lines |

- f4 and f5 are children of the earlier `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md`; `E/🛠️ShellHelpers/🟦️.tsx` (394 lines) is the pre-existing explicit re-export barrel of those children. Its `export { … } from "./🔩️plumbing/🟦️.ts"` and `"./🪟️window-panes/🟦️.tsx"` lists are re-pointed to the new owners (the barrel is not mine and not unbarrelled here; it only follows the moved names).
- TS-TYPE-ERRORS-4 (`tt4`) edits files repo-wide: every file is re-read right before it is edited, and the splitters refuse to write when the parent changed during their run.

## 2. Baselines (recorded before the first edit)

| Check | Command | Before |
|---|---|---|
| Type-check, whole renderer | `nice -n 10 node node_modules/typescript/bin/tsc --noEmit -p F/🗑️generated/c4p46-tsconfig.json` (extends `💻️os/tsconfig.json`, include `📺️renderer/**/*.ts(x)`, so every importer is in the program), normalised by `c4p46-tools/tscnorm.py` | 24 diagnostics (`c4p46-tsc-before.txt/.tsv`), none in my six files or their importers; they sit in unrelated tests and in `🎯️targets/🧊️wgpu` (peer-held) |
| Unit tests, whole renderer-react package | `F/c4p46-tools/vitest.sh <label>` (vitest `run`, the package config `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, `SEMIO_TEST_LEVEL=long`, JSON reporter; set extraction `c4p46-tools/vtsum.py`) | `c4p46-vt-base-full`: 215 files, 194 passed / 21 failed; 3,206 tests, 3,169 passed / 37 failed. In my closure: the f3 file has 1 failing test (`📡️ a history change replaying before adoption is announced…`), `🧪️tests/🎚️window-measure-controls` 2 failing (reads window-panes), the rest of the Board2d, TiledMap, ShellHelpers tests pass |

## 2b. Results after the split

| Check | Command | Result |
|---|---|---|
| Type-check, whole renderer | same command as the baseline, run after the splits (`c4p46-tsc-after1.txt`) and again at the very end (`c4p46-tsc-final.txt`) | 24 diagnostics both times; `cmp` against `c4p46-tsc-before.txt` says byte-identical |
| Check is live | same config plus `noUnusedLocals` (`c4p46-tsc-unused-after.txt`, 825 lines repo-wide) | the only hits in my 51 files are 3 declarations already unused in the originals: `INK_MARQUEE_THRESHOLD_PX`, `boardEntityFlag`, `windowInstanceId` (each occurs once in the snapshots). The first run also exposed unused `*CoreN` type imports in the generated roots, removed by `tidy.py` |
| Unit tests, whole renderer-react package | `F/c4p46-tools/vitest.sh final` (long level) | 222 files (215 before plus the 7 new f3 suites), 201 passed / 21 failed; 3,206 tests, 3,169 passed / 37 failed. The per-test status list equals the baseline (`diff` empty) once the 8 time-travel file paths are folded into one; the single failing f3 test (`📡️ a history change replaying before adoption…`, an anchor in a ShellHost tree that moved earlier) now lives in `🧪️tests/📡️reprojection-status` and still fails exactly as before |
| Intermediate run | `c4p46-vt-after1` | 3 new failures, all caused by my split and fixed (5.1) |

## 3. What was done, per file

Tools: `c4p46-tools/gsplit.cjs` (module splitter from c4p27: picked top-level statements move to children, imports rebuilt per file, `export` added to moved private names, regions re-opened, importers re-pointed), `hooks.cjs` (hook decomposition; extended this run so names declared inside the parent tail are not reported as unresolved), `tsplit2.cjs` (new: splits one vitest file into sibling test files plus one support module, resolving references with the type checker, region markers re-opened and closed per range), `tidy.py`, `postfix.py`, `fmtexports.py`, `moveend.py` (hand-fix helpers), `lossless.cjs`, `hookcheck.cjs`, `regioncheck.py`, `namecheck.py`, `cycles.cjs`, `emojicheck.py`. Specs: `c4p46-tools/specs/`.

- **f1 InkCanvasHost.** Module children `🧾️ink-model` (154), `📐️ink-geometry` (250), `🌳️ink-document-ops` (194), `🧱️ink-block-views` (223), `🪟️ink-overlays` (137), `🧲️ink-host-state` (30); hooks `🧰️ink-session-state` (118), `🎬️ink-gestures` (128), `👆️ink-pointer` (323), `✂️ink-edit-and-clipboard` (181). The root keeps the imports, `InkCanvasHost` (four hook calls, the destructure, the early `if (!scene || !doc)` return and the JSX). Importers re-pointed by the splitter: the react target, `👥️scoped-presence`, `🖋️ink-canvas-clipboard`, and `Board2dHost` (`parseSelectionIds`).
- **f2 Board2dHost.** Module children `🔬️board-parsing` (279), `📡️board-events` (123), `📍️drop-and-camera` (103), `🔄️session-sync` (131); hooks `🧰️board-session-state` (146), `💡️board-suggestions-and-events` (280), `🔁️board-lifecycle-and-sync` (295), `👆️board-pointer-and-keyboard` (249), `🖱️board-context-menu-and-drop` (211). The root keeps `Board2dHost` (hook calls, `if (sessionError) throw`, the empty-scene return, JSX). Importers re-pointed: the react target, `canvas-presence`, and the five Board2dHost tests.
- **f3 time-travel component test.** `🧰️history-test-support` (177, the shared corpus readers, oracles and builders, exports added only to what a sibling uses) and sibling suites `🕰️history-body` (118), `📝️draft-editor` (249), `📌️frozen-and-restored` (64), `🛰️shell-reveal` (166), `🧷️refresh-and-mutation-pages` (280), `🧺️list-notices-and-document-load` (151), `📡️reprojection-status` (115); `🧩️component` keeps the band corpus, chords, replay progress and bundle texts (344). All files sit at the same depth as the original, so every relative fixture path and import specifier is unchanged. 17 `describe` and 62 `it` before and after.
- **f4 plumbing.** `🆔️action-ids` (143), `🔀️example-switching` (75), `🎮️leftover-views` (132), `👥️presence-identity` (128), `📥️downloads-and-file-requests` (244), `🚀️effect-dispatch` (75); the root keeps the panel chrome constants, the history projection and `useUIHistory`. A mis-placed JSDoc (reserved action ids) was moved to `FRAMEWORK_RESERVED_ACTION_IDS`.
- **f5 window-panes.** `🎚️window-measures` (235), `🪛️utility-bar-options` (116), `📝️staged-args` (336), `⌨️keyboard-chords` (100), `📋️clipboard-activation` (35); the root keeps the action categories, the category tree and `WindowActionPane`.
- **f4/f5 barrel.** `E/🛠️ShellHelpers/🟦️.tsx` is the pre-existing explicit re-export barrel of the earlier ShellHelpers split. Its `plumbing` and `window-panes` lists now point at the new owners (13 export lists, reflowed in the existing style); `🎞️media-import`, `🪟️window-panes` and the two window-panes importers (`🎛️command-registry`, `🛣️route-and-chrome`) import from the owners directly. I did not unbarrel ShellHelpers (hundreds of importers, not my files).
- **f6 TiledMapHost.** `📐️map-parsing` (187), `🎨️map-theme` (59), `🗃️tile-cache` (110), `🖼️map-renderer` (349); hooks `🧰️map-state` (134), `🎞️map-render-effects` (217), `👆️map-pointer` (234). The root keeps `TiledMapHost` (55 lines). Importers re-pointed: the react target and the component test.

## 4. Source-text readers and registrations

- `engine/🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts` read the Board2d, Ink and TiledMap roots through `?raw` and searched them for `onPointerCancel`; now `readModuleTreeSource` of the root (follows the hook imports).
- `TiledMapHost/🧪️tests/🧩️component/🟦️.ts` read the root through `?raw` to parse `onPointerCancel`; now `readModuleTreeSource(resolve(dirname(fileURLToPath(import.meta.url)), "../../🟦️.tsx"))` (the jsdom `URL` is not accepted by `fileURLToPath`, hence the path form).
- No other reader of the six targets exists in TS, JSON or scripts (`git grep` over `🧰️framework` and `✏️s`). Doc comments in the peer-held `🎯️targets/🧊️wgpu` Rust trees and `EngineCanvas` still cite line numbers such as `Board2dHost/🟦️.tsx:190`; they were stale before and are left alone.
- Vitest config `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`: seven `elementSuite("🛠️ShellHelpers/⏪️time-travel", …)` lines added after `🧩️component`.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: seven entries `🧪️test📜️history{🕰️body,📝️draft-editor,📌️frozen-restored,🛰️shell-reveal,🧷️mutation-pages,🧺️list-notices,📡️reprojection-status}⚛️react` inserted after `🧪️test📜️history📈️replay-steps⚛️band` (group `9_gates`, orders 900.05811551 to 900.05811557, same command shape and `--run` path). `launch.json` parses as JSON, the seed as JSONC.
- Taxonomy: `bun F/tf8-tools/taxsim.ts 🔣️taxonomy.json <45 new folders>` (list `c4p46-newdirs.txt`): `validateTaxonomy` 0 problems and every folder resolves (`crate-module-children`, `members-of-tests`), so no `🔣️taxonomy.json` edit was needed.

## 5. Verification actually run

| Check | Method | Result |
|---|---|---|
| Lossless | `lossless.cjs` per file (non-blank line multiset, imports removed, leading `export` ignored) | only lost lines: the rewritten root docstrings and each hooked function signature (`function X({…}: Props)` becomes `function X(props: Props)` plus the hook scaffolding); added lines are the child docstrings, region repeats and scaffolding. f3: 62 `it` and 17 `describe` before and after |
| Hook body equivalence | `hookcheck.cjs` for f1 (statements 0 to 67, 25,070 characters), f2 (0 to 123, 43,340), f6 (0 to 52, 19,445) | hook bodies concatenated equal the original body, `equal: true` |
| Region markers | `regioncheck.py` over the 45 new files and the 6 roots | 0 unbalanced; every marker kept (4 stray `#endregion` lines moved into the previous hook by hand: Board2dHost `BoardEventFlush` and `Keyboard`) |
| Import cycles | `cycles.cjs` over the 51 files | 0 strongly connected components of size above 1 |
| Folder names, docstrings, paths | `namecheck.py` | every folder is emoji + U+FE0F + kebab, every docstring starts with the folder emoji, siblings unique, 0 problems, longest path 208 bytes; every emoji is already a taxonomy prefix (`emojicheck.py`) |
| Rules of hooks | by construction | all hooks are called unconditionally, in the original order, before the early returns, which stay in the root |
| Debug logs | `grep '[DEBUG]'` over the new files | none |

### 5.1 Failures seen on the way and fixed (all caused by my split)

`c4p46-vt-after1` had 3 new failures: `scene pointer cancellation contract records the current React producer boundary exactly` and the two `MapRenderer … pointer is cancelled` tests. Both tests read a host root as `?raw` text and looked for `onPointerCancel`, which now lives in the hook children. Fixed with `readModuleTreeSource` (section 4); the two files then pass (`c4p46-vt-fix2`: 3 files, 31 tests) and the final full run has the baseline failure set.

## 6. Hand-offs and limits

1. `ShellHelpers/🟦️.tsx` is still a re-export barrel (394 lines, from the earlier split); unbarrelling it needs its own slice.
2. The hooks use one type assertion each (`{} as XCoreN`), the same trade-off as the NodeGraph, ShellHost and World3dHost decompositions.
3. Pre-existing and not touched: the 21 failing files / 37 failing tests of the baseline, the 24 tsc diagnostics, 3 unused locals in the moved code.
4. Not run: ESLint, a Storybook build, a live browser mount of the split hosts (the mounted-React suites `Board2dHost/🧪️tests/*`, `TiledMapHost/🧪️tests/*`, the ink clipboard and presence tests and the time-travel suites exercise them and pass as in the baseline).
5. Stale line numbers in Rust doc comments of the peer-held wgpu trees (section 4).

## 7. Files touched

Created (45): `c4p46-tools/new-files.txt`. Rewritten roots: the six in the table. Importers and readers edited: `🛠️ShellHelpers/🟦️.tsx`, `🎞️media-import`, `🎛️command-registry`, `🛣️route-and-chrome`, the react target `⚛️react/🟦️.tsx`, `👥️scoped-presence`, `🖋️ink-canvas-clipboard`, `👕️canvas-presence`, five Board2dHost tests, the TiledMapHost component test, `🛑️scene-pointer-cancellation`, the react vitest config, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`. Tools stay in `F/c4p46-tools/`; `F/🗑️generated/c4p46-*` holds snapshots, tsc and vitest outputs, to be removed when the ticket closes.
