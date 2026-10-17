# Result CENSUS4-PKG-21 (prefix `c4p21`): plugin-family TS and Python files split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Package 21 of `F/MONOLITH-CENSUS-4.md` section 4 (8 files, 11,074 lines per the census; in fact 2 TS plus 6 Python). Tools in `F/c4p21-tools/`, raw outputs in `F/🗑️generated/c4p21-*`, pre-edit snapshots in `F/🗑️generated/c4p21-snap/`.

## 0. Verdict

**All eight files are split, installed and verified. Nothing is half applied.** Every root is now at most 598 lines (the largest root is energy `💨️airflow-and-controls`), every new child at most 778 lines (limit about 1,000), the longest new path is 239 UTF-8 bytes. Behaviour is unchanged where it can be run: the Python oracles give row-for-row identical results (norm 15 adapters 1,235 scenarios, remodeling 275, energy 1,189), the TS closure reports exactly the same diagnostics as before, the cad suite title multiset is identical, and the moved TS data is deep-equal.

| # | Target (under `✏️s/🔌️plugins/`) | Before | Root after | Children (largest) | Mode |
|---|---|---:|---:|---|---|
| 1 | `📐️cad/…/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts` | 2,331 | 22 | 6 (`🗂️registries` 778) | `registerXTests` per describe group, root orchestrates |
| 2 | `📕️norm/🔮️oracles/🏃️execution/🐍️.py` | 1,332 | 315 | 3 (`📄️document` 579) | python children loaded by `_load` |
| 3 | `📸️remodel/📖️stories/🧭️coordination/🟦️.ts` | 1,184 | 79 | 5 (`🪟️window-scenes` 241) + 2 JSON fixtures | module imports, importers re-pointed |
| 4 | `📸️remodel/…/🧪️tests/📸️mutate-remodeling-1/🐍️.py` | 1,261 | 108 | 4 (`⚙️forward-appliers` 378) | python children |
| 5 | `🔋️energy/…/🏛️mutate-energy-model-1/💨️airflow-and-controls/🐍️.py` | 1,188 | 598 | 2 (`🌡️thermostats-and-humidistats` 386) | python children |
| 6 | same, `📆️schedules-and-layers/🐍️.py` | 1,215 | 299 | 2 (`⏱️schedules` 680) | python children |
| 7 | same, `🔌️power-and-plant/🐍️.py` | 1,300 | 587 | 2 (`❄️refrigeration-water-and-faults` 429) | python children |
| 8 | same, `🔧️zone-systems/🐍️.py` | 1,268 | 565 | 2 (`🌬️air-loops-and-setpoints`, `🏭️plant-loops-and-outdoor-air` 387 each) | python children |

## 1. Found state

- Started 2026-10-17 06:04. No earlier `c4p21` tool folder, result file or generated log existed (nothing to recover).
- File 1 was `M` in the git index (a peer's alias-subpath import sweep, `RESULT-ALIAS-SUBPATH-SWEEP.md`) with the working tree equal to the index (mtime 05:55); I split that text as found and the splitter asserted the live file equalled my snapshot before writing. Files 5-8 date from the earlier energy carve (10-16 13:15), files 2 and 4 from 10-06, file 3 from 10-16 05:59. No file of the package changed under me during the work (every apply script asserts live == snapshot).
- Reused from package 14: `c4p14-tools/{pysplit,pychk,pyrun}.py` and `gsplit.cjs`, copied into `c4p21-tools/` with the small fixes listed in section 2.

## 2. What was done, per file

### File 2, norm execution engine (shared by the fifteen `📕️norm` adapters)
- `c4p21-tools/pysplit.py` (c4p14 tool; changes: a name may be redefined inside one file, `used_by_text` relies on `symtable` only because the `ast.Name` sweep produced false cross-file edges from comprehension variables, an `origin` snapshot so a working copy of the snapshot may carry synthetic region markers). Spec `spec-f2.json`. The working copy carries one synthetic `🔖️Refusal` region carved out of `🔖️Verbs` so the `Refused` class can sit in `📄️document` (the derived views raise it) and the dependency graph is a DAG.
- Root keeps the docstring (plus a layout paragraph), `Subset` and the bounds (the adapters read `Subset` and `build_adapter` from the module), `Laws`, `Handlers`, `Registration`. Children: `📄️document` (wire spelling, document model, entities, derived views, `Refused`; 579), `🔧️verbs` (the closed verb table, `apply_mutation`, `inverse_mutation`; 370), `📝️carrier` (147). The module is still imported as `🐍️` by the adapters (`import_module("🐍️")`) and loads its children with the `_load` helper from its own folder.
- Readers re-pointed (outside the package list; required by "re-point source-text readers"): `📕️norm/🧪️tests/🧫️fixtures/🔮️oracle-source-ownership/🔣️.json` (new key `destinationChildren`), `📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts` (`normSources` now includes the children; the "single `.py` under `🔮️oracles`" assertion now expects the engine plus its listed children one level below), `📕️norm/📦️packages/🟦️typescript/📋️project.json` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json` (`normOracleSources` lists the three children; the test compares both lists with `normSources`).

### File 4, remodeling mutation adapter
- `pysplit.py`, `spec-f4.json`. Root keeps the docstring (plus a layout paragraph) and the registration (108 lines). Children: `📖️vocabulary` (kind tables, slots, envelopes, fixture readers, addressing; 162), `⚙️forward-appliers` (378), `⛔️refusals-and-inverses` (refusal rules and the single-step inverse; 342), `🔮️oracle-scenarios` (oracle and scenario handlers; 363).

### Files 5-8, the four energy per-group adapters
- `c4p21-tools/esplit.py` (generalisation of the c4p14 `fabsplit.py`), specs `spec-f5..f8.json`: contiguous function groups; the first group stays in the parent, later groups move to children; `VECTOR_ROOTS` rows follow the longest matching kind, `VOCABULARY`/`EXTRA_INVERT` rows follow the function they name; the parent merges the children's tables after its own rows (`**c1.TABLE`), keeping the original key order.
- 5 `💨️airflow-and-controls`: parent keeps infiltration and mechanical ventilation, children `🌡️thermostats-and-humidistats`, `💯️ideal-loads`.
- 6 `📆️schedules-and-layers`: parent keeps space lists and thermal enclosures, children `⏱️schedules`, `🧊️glazing-and-materials` (the first draft used `🪟️`, which the sibling `🪟️openings` docstring already uses; changed before the final run).
- 7 `🔌️power-and-plant`: parent keeps load centers, PV and batteries, children `🚿️hot-water-and-solar-thermal`, `❄️refrigeration-water-and-faults`.
- 8 `🔧️zone-systems`: parent keeps zone equipment, daylight, sizing and room air, children `🌬️air-loops-and-setpoints`, `🏭️plant-loops-and-outdoor-air`.

### File 3, remodel stories coordination
- `c4p21-tools/gsplit.cjs` (c4p14 general TS splitter, `spec-f3.json`), explicit importer list. The parent keeps the header and the reducer; children `📐️scene-model` (types, config; 132), `🗂️documents`, `🗣️labels`, `🪟️window-scenes` (constants, formatting, per-window scenes; 241), `🧩️panel-documents` (176). The six `🎭️*/🧪️.story.tsx` importers were rewritten by the tool to import each child directly; no barrel. Parent header: layout lines added, a stale `./fixture.ts` reference corrected, the two `//#region 🧲️Header` markers the tool dropped were restored.
- Data moved to fixtures (`c4p21-tools/f3-fixtures.ts`): `🗂️documents/🔣️.json` (the populated scene, 370 lines) and `🗣️labels/🔣️.json` (`{en, de}`, 166 lines); the two TS children shrink to 20 and 100 lines and read them (`as unknown as RemodelScene` for the scene, plain typed assignment for the labels).

### File 1, cad core-interactions suite
- `c4p21-tools/cadsplit.py`: the 12 top-level `describe` blocks of the single `registerTests1` function are grouped in their original order and move to six children, each a `registerXTests(vitest, dependencies)` that re-declares only the imports, dependency bindings and fixture constants its own text uses (relative specifiers re-based one folder deeper): `🕹️transforms` (241), `🗂️registries` (778), `📦️box-length` (506, box, length entry and the `stateEngine` option), `📏️measure` (163), `🕰️history` (180, document history, measure history, session undo/redo, undo routing), `🎞️e2e` (546). The root `registerTests1` keeps its signature (the artifact's in-source block calls it unchanged), bootstraps the cad modules once and calls the six children in order. Kebab names are at most 10 characters because the parent path is already 210 bytes (limit 240).

## 3. Verification actually run

| Check | Method | Result |
|---|---|---|
| Python oracle, norm | `c4p21-tools/norm-run.sh` (`pyrun.py` with `--local-source` for the engine dir) over the 15 `mutate-*-1` cases, before, after the split and after all later edits; `pycmp.py` compares every scenario row (status, full output projection, diagnostics, temp paths normalised, timing ignored) | 1,235 scenarios, all passed before and after; **0 rows differ** (before vs after and before vs final) |
| Python oracle, remodeling | same runner on `📸️mutate-remodeling-1` | 275 scenarios: 187 passed, 86 failed, 2 errored before and after (pre-existing); **0 rows differ**, tracebacks compared by their final line (the file path and line numbers inside a traceback change by design) |
| Python oracle, energy | same runner on `🏛️mutate-energy-model-1` (loads all four split files), before, after, and after the `🧊️` rename | 1,189 scenarios: 1,159 passed, 2 failed, 28 errored before and after (same as the package-14 end state); **0 rows differ** |
| Lossless, Python | `c4p21-tools/pychk.py` (`c4p21-pychk.log`) | `PROBLEMS 0`. Compile and `symtable` undefined-global check of every file; top-level statement multiset of the snapshot minus the new tree: **lost 0** in all six; added only wiring (`_load`, imports regions, docstrings, `**cN.TABLE` rows, synthetic `Refusal` region). Stripped line multiset: lost 1 line in files 2 and 4 (the narrowed `from semio_repo_test import …` line), 0 in 5-8. Energy tables loaded old versus new: `VECTOR_ROOTS`, `VOCABULARY`, `EXTRA_INVERT` equal in content, order and function identity (88/44/44, 90/45/45, 100/50/50, 96/48/48 keys) |
| Lossless, cad TS | `c4p21-tools/titles.py` (`describe`/`it` titles and stripped line multiset of snapshot vs new tree) | 89 titles before, 89 after, **0 lost, 0 added**; the only lost lines are the 11 import lines and the one dependency destructure the children re-declare (rewritten, not removed); every test body line is retained |
| Lossless, stories TS | `c4p21-tools/tschk3.py`; `f3-check.ts` (`c4p21-f3-check.log`) | statement set retained; lost lines are the 460 lines of the two literals now in the JSON fixtures, the moved import line, and `function` to `export function` for `fixed`/`debugEnum`; region markers: none lost (after restoring the two `Header` markers). Deep-equal: populated scene, empty scene, en and de labels, including key order (7 of 7 OK) |
| `tsc` | `c4p21-tools/tsc-run.py` (`node tsc --noEmit` over root files; configs `c4p21-tsc-*.json`) | before (cad artifact, f3, six stories): 11 diagnostics, all TS2307 for missing `brepjs`/`cad-js-module-*` in files outside this slice. After f3 split + fixtures: the same 11, none in the stories or children. After f1 split: the same 11 (message set identical with positions stripped). Extra pass with `noUnusedLocals` over the cad children: only `archivedSelectionTargets` and `seed`, two unused locals that were already in the original body; two unused `Model` imports and one unused `InteractionRuntime` binding my first draft had were removed |
| `bun test` norm ownership | `bun test` of `📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts`, before and after | 3 pass / 3 fail both times, identical test names and messages (line numbers shifted by one). The 3 passes include the two tests that read the moved text. The 3 failures are environmental and pre-existing: the registry lookup timeout and a host workspace-marker error (`workspace root with paired build and package markers was not found`) |
| Path emoji statutes | `bun test` of `🔏️path-emoji-statutes` (37 tests) after | 27 pass / 10 fail, the same counts the package-14 run recorded; no failure message names any of the 26 new folders |
| Path length | longest new relative path | 239 bytes (limit 240, hard limit 256) |
| Debris | `find` for `__pycache__`, `.pyc`, `.orig`, `.bak` in the touched trees; `[DEBUG]` grep over all 28 new files and the 8 roots | none |

Not run (not claimed): a vitest run of the cad suite (no config in the tree includes the engine artifact and its closure imports unresolved `@semio-tech/cad-js-module-*` and `brepjs`; the baseline and after are therefore tsc plus the title multiset); a Storybook build of the six remodel stories (types checked with tsc only); the Rust twins (no Rust file was edited).

Launch files: no new test entry point was created (the children are loaded by their roots, the cases and tests that run them already have entries), so `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` are unchanged. `git grep` found no launch entry naming any moved path.

## 4. Files touched

Created (28): `📕️norm/🔮️oracles/🏃️execution/{📄️document,🔧️verbs,📝️carrier}/🐍️.py`; `📸️remodel/📖️stories/🧭️coordination/{📐️scene-model,🗂️documents,🗣️labels,🪟️window-scenes,🧩️panel-documents}/🟦️.ts` plus `🗂️documents/🔣️.json` and `🗣️labels/🔣️.json`; `📸️remodeling/…/📸️mutate-remodeling-1/{📖️vocabulary,⚙️forward-appliers,⛔️refusals-and-inverses,🔮️oracle-scenarios}/🐍️.py`; the eight energy children listed in section 2; the six cad children listed in section 2. Full list in `F/c4p21-tools/new-files.txt`.
Rewritten: the eight package files. Edited outside the package list (importers/readers only): the six `🎭️*/🧪️.story.tsx` files, the norm ownership fixture and test, and the two `project.json` files named in section 2.
Tools (kept): `F/c4p21-tools/` (`pysplit.py`, `esplit.py`, `cadsplit.py`, `gsplit.cjs`, specs, `pyrun.py`, `pycmp.py`, `pychk.py`, `titles.py`, `tschk3.py`, `f3-fixtures.ts`, `f3-check.ts`, `tsc-run.py`, `norm-run.sh`, file lists). Logs, snapshots and result sets in `F/🗑️generated/c4p21-*` are generated output to be removed when the ticket closes.

## 5. Hand-offs

1. Taxonomy registration (held `🔣️taxonomy.json`): the new folder names `📄️document`, `🔧️verbs`, `📝️carrier`, `📖️vocabulary`, `⚙️forward-appliers`, `⛔️refusals-and-inverses`, `🔮️oracle-scenarios`, `🌡️thermostats-and-humidistats`, `💯️ideal-loads`, `⏱️schedules`, `🧊️glazing-and-materials`, `🚿️hot-water-and-solar-thermal`, `❄️refrigeration-water-and-faults`, `🌬️air-loops-and-setpoints`, `🏭️plant-loops-and-outdoor-air`, `📐️scene-model`, `🗂️documents`, `🗣️labels`, `🪟️window-scenes`, `🧩️panel-documents`, `🕹️transforms`, `🗂️registries`, `📦️box-length`, `📏️measure`, `🕰️history`, `🎞️e2e`. Each emoji is an existing folder prefix of the repo and unique among its siblings; they follow the same unresolved class as their older siblings.
2. Wrong-direction import, outside this slice: the cad artifact `…/⚙️engine/🗿️artifact/🟦️.ts` (and `🎬️actions/🟦️.ts`) still dynamic-imports its suite from inside an `import.meta.vitest` block. Moving it to a plain test file with a config entry is the fix `RESULT-TS-AUDIT-FIXES.md` applied for the kernel; it also needs a vitest config that actually includes the artifact and resolves `@semio-tech/cad-js-module-*`.
3. Unused locals carried over from the original bodies: `archivedSelectionTargets` (`🎞️e2e`) and `seed` (`🗂️registries`).
