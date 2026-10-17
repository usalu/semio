# Result CENSUS4-PKG-50 (prefix `c4p50`): framework/modules TypeScript roots split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `M` = `🧰️framework/🔨️modules`. Package 50 of `F/MONOLITH-CENSUS-4.md` section 4 (5 files, 7,275 lines). Tools in `F/c4p50-tools/`, raw outputs in `F/🗑️generated/c4p50-*`, pre-edit snapshots in `F/🗑️generated/c4p50-snap/` (`SHA.txt`).

## 0. Verdict

**DONE.** All five files are split and installed; nothing is half applied. Every root is at most 712 lines and every new child at most 529 lines (limit about 1,000); 42 new files, longest new path 101 bytes. Behaviour is unchanged where it can be run: full `tsc` over the 133-root closure reports 0 diagnostics before and after, every baseline test keeps its status (tool-run bun 32 tests, machine vitest 3, os bun oracles 10, renderer subset 88, os vitest 609 baseline tests), and the descriptor-digest test (whose schema reader I re-pointed) passes 8 of 8.

| # | Target (under `M/`) | Before | Root after | Children (largest) |
|---|---|---:|---:|---|
| 1 | `⏯️tool-run/🟦️.ts` | 1,276 | 65 | 8 (`🔤️codec` 529) |
| 2 | `📇️directory/🧬️schema/🟦️.ts` | 1,321 | 28 | 11 (`📋️admin-page` 185) |
| 3 | `🔄️machine/🟦️.ts` | 1,156 | 293 | 7 (`🧠️kernel` 302) |
| 4 | `🛰️backbone/🟦️.ts` | 1,631 | 139 | 10 (`🧱️worker-wire` 249) |
| 5 | `🧾️app-channel/🟦️.ts` | 1,891 | 712 | 6 (`🎛️command-codec` 373) |

## 1. Found state

- Fresh slice, started 2026-10-17 15:20. No earlier `c4p50` tool folder, snapshot or log. The five targets were unmodified (no `git diff` against HEAD). Re-measured sizes equal the census: `M/⏯️tool-run/🟦️.ts` 1,276, `M/📇️directory/🧬️schema/🟦️.ts` 1,321, `M/🔄️machine/🟦️.ts` 1,156, `M/🛰️backbone/🟦️.ts` 1,631, `M/🧾️app-channel/🟦️.ts` 1,891.
- Machine load 36 to 43, about 20 peer agents (c4p46 renderer TS, tt4 type errors and others). None of my five files nor their module trees lie in c4p46's renderer files.
- Importers: module-internal importers (children of the five modules, the module package wrappers), the SDK wrappers under `🧰️framework/📦️packages/🟦️typescript/<module>/`, os and plugin modules that import `app-channel` and `directory/schema` by relative path, the os and renderer tests that import `@semio-tech/framework/backbone` etc. through the wrappers, source-text readers (two hub checks read the schema file with `readFileSync`), and path lists (nx inputs, `🔣️taxonomy.json` `sourceModulePaths`, two fixtures).

- Resume 17:14: the cut left no half-applied batch. `⏯️tool-run` is fully applied (root 65 lines, 8 children, lossless, bun tests identical, quick tsc 0 diagnostics); the other four files still match their snapshot SHA (`c4p50-snap/SHA.txt`). No background process of mine is alive.

## 2. Baselines (recorded before the first edit)

| Check | Command | Before |
|---|---|---|
| tsc, closure of all five module trees + SDK wrappers + known importers | `F/c4p50-tools/tsc.sh before` (`tsc --noEmit -p` a config extending `💻️os/tsconfig.json` with 91 root files from `mkroots.py`; the program holds 2,918 files, 1,118 of them under `🔨️modules`) | **0 diagnostics**, exit 0 (`c4p50-tsc-before.txt`); liveness is re-proved after the splits with `noUnusedLocals` |
| machine unit (in-source) | `F/c4p50-tools/vitest.sh machine-before 🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts` (vitest `run`, level long, JSON reporter) | 1 file, 3 tests, 3 passed |
| tool-run bun tests | `F/c4p50-tools/buntest.sh toolrun-before M/⏯️tool-run/🧪️tests/🧪️conformance/🟦️.ts M/⏯️tool-run/🏛️ownership/🧪️tests/🟦️.ts M/⏯️tool-run/🎞️tick/🛫️encode/🧪️tests/🟦️.ts` (the three files of the package's `test-source` route, junit reporter) | 32 tests, 30 passed, 2 failed: `schema oracle original pending tick and writer custody matches independent fields` (ajv: no draft 2020-12 meta schema) and `General ToolRun TypeScript imports remain inside the neutral Framework owner` (the `🦀️rust/📜️script.ts` of the module imports the repo library) |
| os vitest (whole `@semio-tech/framework-os` config) | `vitest.sh os-before 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts` | 23 files (20 passed, 3 failed), 609 tests: 499 passed, 110 failed (107 in `🎭️actor-transport`, 1 in `⏱️directory-client-http` (fixture glob finds 0 files), 2 in `🧾️app-channel-codec` (`ENOENT` on two fixtures)). Per-test status list: `c4p50-vt-os-before.set` |
| os bun channel oracles | `buntest.sh os-oracles-before` on the five `os/🧪️tests/🧪️{document-archive-load-host,attached-document-replacement,document-port-control-turn,folder-archive-persistence,folder-read-back}` files | 10 tests across 5 files: 7 passed, 3 unhandled errors between tests |
| renderer react subset that imports the five modules | `vitest.sh rend-before 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts <name filters>` | 9 files, 88 tests, all passed |

## 3. Progress

- `⏯️tool-run/🟦️.ts` 1,276 -> 65 (Limits, Identity, `ToolRunCodecError`). Children: `🚦️lifecycle` 88, `📊️progress` 96, `🗄️trace-store` 208, `🎞️tick` 19, `🔤️codec` 529, `🧾️json` 144, `🪧️panel` 26, `🏷️actions-labels` 132. 12 importers re-pointed by `gsplit.cjs` (existing `📊️progress/*`, `📽️tick`, `🕹️actions`, the module package entry, the conformance test via per-name imports, three os/plugin tests). Lossless (`c4p50-lossless-f1.log`): only the root docstring changed. Bun tests `toolrun-after1`: per-test set identical to the baseline (30 passed, same 2 failures).
- `📇️directory/🧬️schema/🟦️.ts` 1,321 -> 28 (vocabulary and actor types; the root docstring). The six `export … from` barrel statements (27 names from the browser-actor, check-in and scope modules) are removed (`unbarrel.cjs`) and the importers (SDK entry, directory root, `🏘️spaces`, `🚪️io/📝️text`, directory client, backbone, browser-bundle view-context) import each name from its owner. Eleven children: `📜️event` 178, `📄️event-page` 105, `🎫️command` 132, `🛠️admin` 109, `👀️views` 115, `📋️admin-page` 185, `📂️document-open` 117, `🎯️execution-target` 168, `🧊️artifact-checkpoints` 84, `🪢️checkpoint-pair` 56, `🌊️stream` 36. A first apply used line numbers taken before `unbarrel` had collapsed blank runs, so the partition was off by a few statements; I rolled back only my own files (children deleted, root restored from the snapshot, `unbarrel` re-run, 6 importers re-pointed by the new `repoint.cjs` from the final owner map) and re-split with ranges re-read from the post-unbarrel outline. Final lossless (`c4p50-lossless-f2.log`): only the barrel statements are lost.
- `🔄️machine/🟦️.ts` 1,156 -> 293 (ids, configuration, schema, tables, commands, snapshot). Children: `🔎️inspect` 37, `🧠️kernel` 302, `🌐️host` 157, `💾️persist` 79, `🎬️runtime` 128, `🔁️step` 88, `🧭️testing` 108 (holds the in-source test block). The vitest config's `includeSource`/`coverage.include` now name `🧭️testing`, the extracted suite's `dependencies` type picks `init` from the kernel and `runConformance` from `🧭️testing`. The package entry `export *` is replaced by explicit per-owner lists of exactly the old exported names (`exportsmap.cjs`, `genwrapper.py`). Lossless: nothing lost (`c4p50-lossless-f3.log`).
- `🛰️backbone/🟦️.ts` 1,631 -> 139 (sync controller id, endpoint paths, backbone URIs, document pack bytes, framework sync utilities, blob path). Ten children: `🧭️opening-relay` 97, `🌐️envelope-io` 126, `🔀️message` 147, `🪟️desktop-window` 46, `🔑️persistence-binding` 71, `🎭️artifact-actor` 170, `🧱️worker-wire` 249, `🔬️worker-parsers` 219, `📨️worker-messages` 245, `🗺️workflow-planner` 117. The original file closed a `🔖️EnvelopeCodec` region it never opened; I added the matching start marker before `BackboneWorkerWireMessage` first. Lossless: nothing lost.
- `🧾️app-channel/🟦️.ts` 1,891 -> 712 (the `AppChannelClient` class and its progress type). Six children: `🏷️codec-primitives` 56, `🎛️command-codec` 373, `🎞️frame-codec` 369, `📋️report-decoders` 76, `🔢️request-sequence` 107, `🗃️archive-load-host` 172. Lossless: nothing lost.
- Path lists and readers: the os and wgpu `📋️project.json` input lists (globs `*/🟦️.ts` for the five modules), the `⚙️config-graph.json` deny list (the 16 new backbone and app-channel children), the `🖥️os-source-topology` fixture consumer, the channel-version consumer registry (`🛂️manifest/🔖️channel-version/📇️consumers.json`: the literal 23 now lives in `📂️document-open`), `🔣️taxonomy.json` (`wgpu-frame-worker` `sourceModulePaths` 209 -> 240 with the schema root removed because it is type-only now, and `inputPatterns` 225 -> 257, both byte-sorted; observed set from `c4p50-tools/wgpu-observed.ts` run against the live tree), two hub checks and the descriptor-digest test now use `readModuleTreeSource` for the schema text.
- Taxonomy: `taxsim.ts` over the 42 new folders: `validateTaxonomy` 0 problems, 42 resolved, 0 unresolved; every emoji prefix already occurs as a folder prefix in the taxonomy (`emojicheck.py`); every new file starts with a docstring beginning with its folder emoji; longest new path 101 bytes; no `[DEBUG]` text.

## 4. Results after the splits

| Check | Command | Result |
|---|---|---|
| tsc, same 91 roots plus the 42 new files (133 roots) | `F/c4p50-tools/tsc.sh after1` (log `c4p50-tsc-after1.txt`) | **0 diagnostics**, exit 0: identical to the baseline |
| tsc liveness | `tsc --noEmit -p c4p50-unused.json` (`noUnusedLocals`, new files plus the five roots) | 40 diagnostics (so the check reports); in my files only `editedArtifactFrontier` in `🎫️command` (a private helper that was already unused in the original) and the pre-existing unused `actor` property of `AppChannelClient`; no generated import is unused |
| tool-run bun tests | `buntest.sh toolrun-after2` | per-test set identical to `toolrun-before` (32 tests, 30 passed, same 2 failures) |
| machine vitest | `vitest.sh machine-after …` | 3 of 3 passed; same three titles, now reported from `🧭️testing/🟦️.ts` (the in-source host) |
| os bun channel oracles | `buntest.sh os-oracles-after` | per-test set identical (7 passed, same 3 unhandled errors) |
| renderer react subset | `vitest.sh rend-after …` | 9 files, 88 tests passed; `.set` identical to `rend-before` |
| os vitest, whole config | `vitest.sh os-after …` (log `c4p50-vt-os-after.log`) | all 609 baseline tests have the identical status (0 changed, 0 missing). The config grew meanwhile (a peer added include entries: 35 files, 722 tests, 114 failed); the 113 extra tests are new files, and their only failures are missing fixtures or modules (`🏷️schema-vocabulary` ENOENT on a hub schema, `🔁️execution-target-retry` cannot find module `retry`, `🗂️surface-opens-kind` ENOENT), none related to this split |
| descriptor-digest bun test | `buntest.sh digest-after` | 8 of 8 passed (it now reads the schema through `readModuleTreeSource`) |
| Lossless | `lossless.cjs` per file (`c4p50-lossless-f1..f5.log`) | no code line lost in tool-run, machine, backbone, app-channel; schema loses only the six removed `export … from` barrel statements; added lines are the child docstrings; every region marker is kept, repeated and balanced in each child that holds moved statements (the backbone `🔖️EnvelopeCodec` end marker had no start in the original; I added it) |
| Import cycles | gsplit cycle detection per spec (children only import siblings or the root; the root imports no child except `🧾️app-channel`, whose root is the client) | no cycle reported |
| Taxonomy | `taxsim.ts` over `c4p50-tools/new-dirs.txt` | 0 problems, 42 resolved to `crate-module-children` |
| Launch registration | no new test entry point was added (the machine suite stays under its existing route; its in-source host moved inside the same package config) | `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` unchanged |

## 5. Hand-offs and limits

1. Not run: the two hub authority checks (`🌎️hub/🧪️tests/🔑️directory-authority-checks`, `🛂️space-administration-checks`) drive the cargo gate; I only re-pointed their schema read to `readModuleTreeSource`. Their TS assertions already looked for functions (`parseDirectorySpaceAdministrationPageV1`) that an earlier extraction moved out of this file, so they were red before this slice.
2. The orphan JSDoc stubs (docs of functions that an earlier extraction removed) in the schema file travelled with the next statement into `📄️event-page`, `🎫️command` and `🛠️admin`; I left them (lossless), they should be deleted by an owner who knows the intent.
3. `🧾️app-channel/🟦️.ts` stays at 712 lines because it holds the single `AppChannelClient` class (688 lines); splitting a class body would need a hook-style decomposition, left out.
4. `wgpu-frame-worker` `sourceModulePaths` still drifts from Bun's observed set in unrelated paths (pre-existing, see `RESULT-OS-TS-R3F.md`); I only added the 32 children this slice created and removed the now type-only schema root.
5. Tool defects, for later users of `c4p50-tools/gsplit.cjs`: `unbarrel.cjs` collapses blank-line runs, so line ranges for a following `gsplit` must be re-read from a fresh `outline.cjs` run (name selectors are safer); a shorthand property of a name that is both a type and a value (`{ EventId, … }`) does not count as a reference (I added the machine test-block imports by hand).

## 6. Files touched

Created (42): `F/c4p50-tools/new-dirs.txt` lists every child folder. Rewritten roots: the five in the table. Importers and entries re-pointed: the SDK entries `🧰️framework/📦️packages/🟦️typescript/{⏯️tool-run,🔄️machine,🛰️backbone,🧾️app-channel,📇️directory/🧬️schema}`, the module package entries of tool-run and machine, existing tool-run children (`📊️progress/*`, `📽️tick`, `🕹️actions` and its test), the conformance test, `📇️directory/{🟦️,🏘️spaces,🚪️io/📝️text}`, `📞️directory-client`, `🌐️browser-bundle/{🎯️action-handoff/*,🪟️view-context}`, os plugin tests (`📖️read-only-run`, `🛑️terminal-quiet`, `🪟️host-opening-context`, the browser-actor action-handoff test, `🏗️materialization`), the renderer `⌨️window-scope` test, `🔄️machine/🧪️tests/{🧪️semio-tech-machine,🎚️config}`, two hub checks, the descriptor-digest test, both `📋️project.json` input lists, three fixtures, the channel-version consumer registry and `🔣️taxonomy.json`. Tools stay in `F/c4p50-tools/`; `F/🗑️generated/c4p50-*` holds snapshots, tsc, vitest and bun outputs.
