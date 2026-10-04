# 📓️ Audit 2 — registrations and read-only verifiers after the second round

Ticket `2026/10/02/QUIZ-PETS`, read-only audit run 2026-10-03 between 18:37 and 19:30 (host time), Windows, bun 1.4.2. Method of `📓️audit-registrations.md` (round one), re-run against the tree as it stands; F1 was still editing core files while I worked, so the pets schema twins and the stage-trace vectors may move after this report.

Abbreviations: `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `Q` = `🧰️framework/🛍️products/❓️quiz`, `QR` = `Q/🎯️targets/⚛️react`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TAX` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, `TK` = this ticket folder, `SCR` = `TK/🗑️generated/audit2-registrations` (every command output and scratch script of this audit; 12 MB, goes with `🗑️generated`).

What I wrote: this file and `SCR`. No git command that modifies anything, no cargo, no vitest, no e2e, no generator, no server, no `bun install`, no `write-baseline`, no `schema generate`. Read-only commands run: `verify taxonomy report` (three scopes), `discover`, `verify dependencies` (plain, `literal-external`, `list`, `parity js --format json`), `schema generate --check`, `schema check`, `schema verify`, `check-generated` of the plugin registry, `nx show project(s)`. In-process pure calls: `caseContractBreaches`, `validateAllContracts`, `testLayoutBreaches`, `renderCatalogFiles`, `generateLaunchJson`, `inventorySchemaScopes`, `renderSchemaCatalog` (diffed in memory; nothing written).

## 0. Findings

| # | Severity | Where | What | Fix |
|---|---|---|---|---|
| 1 | should-fix (workspace owner / coordinator) | `🎓️teaching/bun.lock` (HEAD, commit 🚩️665; 0 occurrences of `pets`), `QR/📦️packages/🟦️typescript/package.json`, `S/📦️packages/🟦️typescript/package.json` | The repository now has two bun workspaces: the root (`🧰️framework/**`) and `🎓️teaching/package.json` (`**` and `../🧰️framework/**`, the one that holds the site). The teaching lock has **no workspace entry for `@semio-tech/pets` or `@semio-tech/pets-react`**, its `@semio-tech/quiz-react` snapshot (`bun.lock:157-183`) lacks `pets`, `pets-react`, `@types/aria-query`, `aria-query`, `colord`, `i18next`, and its `@teaching/architecture-quiz` snapshot (`bun.lock:547-562`) lacks `@semio-tech/pets`. A frozen install in `🎓️teaching` must therefore be refused. The repo's own `verify dependencies parity js` cannot see it: it reports `lock-workspaces=1 lock-mismatches=0` (root lock only). The root lock is clean for all four pets/quiz packages (`SCR/lockcheck.ts`: `pets`, `pets-react`, `quiz`, `quiz-react` in sync, including the hand-added `@testing-library/user-event` line of the pets-react snapshot at `bun.lock:326` and its resolution at `bun.lock:1709`). | The coordinator runs `bun install` once in `🎓️teaching` at close (agents must not); extend the parity verifier to read `🎓️teaching/bun.lock` as well. Not caused by a pets agent, but the pets packages are the entries missing. |
| 2 | should-fix (pets) | `P/🔮️oracles/🔣️.json:173` (`oracles[9]` `pets-react-user-event`) | `engine.family` is `"@testing-library/user-event"`; the registry schema wants `^[a-z0-9]+(?:-[a-z0-9]+)*$`. This is the **only** contract breach that names a pets path (`validateAllContracts` over the 23 cases plus the repository gates: 1665 breaches repo-wide, 1 pets; `testLayoutBreaches`: 4592 repo-wide, 0 pets; the 23 case contracts: 0). `contract --owner pets` is red for this alone on top of the repo backlog. D2 saw it and left it ("not D2's"). | `"family": "testing-library-user-event"`. |
| 3 | should-fix (after F1 is done with the schema) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`, `📓️schema-catalog.md` | `schema generate --check` exit 1, `schema verify` exit 1 (JSON and Markdown), `schema check` has `schema-catalog-stale=1`. Exactly three scopes differ: `framework.product.pets` (the three twins were rewritten at 17:29, the catalog was written at 12:51 / 13:16: **four exports exist in the schema that the catalog lacks, `Foothold`, `Pitch`, `PuffFrame`, `Trip`** (121 against 117), plus the file hashes of `🔣️.json`, `🟦️.ts`, `🦀️.rs`), `framework.product.quiz` (hashes of `🔣️.json`, `🦀️.rs` only), `print`. `schema check` names no pets path. | After the last schema edit: `bun ./📜️script.ts schema generate && bun ./📜️script.ts schema docs`, then `schema verify`. |
| 4 | should-fix (coordinator, at close) | `🔒️dependencies.json` (baseline of commit 40a2736) | `verify dependencies` exit 1 with 68 new entries (was 73); **six come from pets packages**: `d3-ease@3.0.1`, `polygon-clipping@0.15.7`, `aria-query@5.3.0` (all `test-oracle`), `@testing-library/user-event@^14.6.3` (`test-oracle`; **new in round two**, shared with `quiz-react`), `@types/aria-query@5.0.4`, `@types/d3-ease@3.0.2` (`repository-tooling`). The other 62 are `temp/brepjs`, `temp/brepkit`, print, quiz, proctor, ui. Classification of every pets test library is complete and consistent with the registry (§4.2). No new Rust crate, no new Python distribution. | `bun ./📜️script.ts verify dependencies write-baseline` once for the whole repo at close. |
| 5 | should-fix (coordinator, at close) | 25 untracked files: 24 under `P` (new Rust twins of the second round: five module twins `🔨️modules/{✨️effects,🚧️clearance,🧗️climbing,🪄️mischief,🪢️swing}/🦀️.rs`, seven `🧪️tests/🔬️unit/🦀️.rs`, eleven `🧪️tests/<case>/🦀️.rs`, and `🎥️projection/🧪️tests/🔬️unit/🟦️.ts`) and `QR/🔨️modules/🐾️pets/🎪️stage/🟦️.tsx` | The tracked glue `P/📦️packages/🦀️rust/🦀️.rs:24-46` mounts the untracked modules with `#[path]`, and the quiz glue lazy-loads the untracked chunk, so a commit of tracked files only would not compile. The rest of the tree is staged (`A`, `AM`, `MM`; many entries have newer working-tree edits than the index). | `git add` the whole tree when the owner commits (the owner commits everything; do not commit selectively). |
| 6 | should-fix (docs) | `P/README.md` Layout table (lines 14-44) | Six of the 25 core modules have no row: `🚧️clearance`, `🪢️swing`, `👆️gesture`, `💗️feeling`, `✨️effects`, `🪄️mischief` (the last two appear in the prose of other sections only). The eight React modules have no row at all and `🪞️lifting` is never named (only "the lifted copy" at line 200). The `🧪️tests/<case>/` row does not list the 24 Protocol v2 cases or the 8 React suites; the site README and `AP/README.md` have case/suite tables, the product README has none. | Add the six rows and one row per React module; add a table of the cases with capability, oracle and generator. |
| 7 | nit | `TAX:14165` (`members-of-tests`) | `🎮️pet-play` is registered (A2 registered 16 test names) but exists nowhere: no directory under `P`, `PR`, `Q`, `QR`, `S`, `AP`, no reference in any file but the taxonomy. The "Play with the pets" group is tested inside `Q/🧪️tests/🐾️pet-companions`. Round one's dead names (`🖌️pet-depiction`, `🐾️pet-companions` as fixtures) are gone. | Delete the name (greenfield: no dead registrations). |
| 8 | nit | `.venv/Scripts/python.exe` vs `P/🔮️oracles/🔣️.json`, `pyproject.toml:22,25`, `uv.lock` | Unchanged since round one: registry, `pyproject.toml` and `uv.lock` say numpy 2.5.0 / scipy 1.18.0; the interpreter that generated and replays all vectors has numpy 2.4.3 / scipy 1.17.1 (Python 3.14.4, jsonschema 4.26.0 matches). The oracle evidence therefore comes from older engines than the registry names. | `uv sync`, rerun the generators once; they should answer "unchanged". |
| 9 | nit | `verify dependencies parity js` (scanner) and `Q/🧪️tests/🐾️pet-companions/🟦️.tsx:27-29` | 14 undeclared-import lines name pets paths, all the same class as round one's finding 12 (suites scored against the wrong manifest): `P/🧪️tests/{📡️surface-survey,🤏️pet-handling,🫥️decorative-layer}` import `@testing-library/react`, `@testing-library/user-event`, `aria-query`, `react` against `P/📦️packages/🟦️typescript/package.json` (declared in the react package), the quiz suite imports four packages against the quiz core manifest, and `S/🧪️tests/🐕️pet-walk:35` imports `@playwright/test` against the site manifest (as every site e2e spec). Separately, the quiz suite uses `@testing-library/user-event` and `lodash/uniq` and the quiz oracle registry has no pets capability for either (lodash entries there are for `throttle` and the crowd). | Scanner fix (map `🧪️tests/*/🟦️.tsx` to the target package). Optionally register the quiz suite's capability or keep both as helpers like `@testing-library/react`. |
| 10 | nit | `P/📦️packages/🦀️rust/Cargo.toml` `description`, `PR/📦️packages/🟦️typescript/package.json` `description`, root `Cargo.toml:159`, `README.md:665-666` | Unchanged since round one: the crate description stops at "behaviour … and the stage" (the `🦀️.rs` docstring was extended, the manifest was not); the pets-react description names depiction, survey, pacer and layer only (grasp, gear, effects, lifting are missing); `semio-framework-pets` is a `[workspace.dependencies]` entry no manifest consumes (the generated test hosts address the crate by path); the root README port table lists neither `6069` nor `6074` nor `6061`. | One sentence each; keep or drop the workspace entry. |
| 11 | nit | `TK/.🧬semio/…/QUIZ-PETS/🗑️generated/wp-r/gate-dev-reduced-steps/report.json`, `TK/__pycache__/generate_behavior_vectors.cpython-314.pyc`; ignored dev caches `PR/📦️packages/🟦️typescript/node_modules/.vite/stories-{6071,6072,6237,6253,6254}` (19 MB package `node_modules`); repo root `test-results/.last-run.json` (13:31), `activation.log` (10-02 15:08) | A tool wrote a report to a path relative to the ticket folder (a whole `.🧬semio/…` tree nested inside it), the generator left a bytecode file outside `🗑️generated`, and abandoned stories ports left caches. All are tool output (delete at close); the dev caches and the two root files are git-ignored. | Delete with `🗑️generated`; `rm -r` the nested `.🧬semio` and `__pycache__` in `TK`. |
| 12 | info | `bun ./📜️script.ts check-generated` (plugin registry) | exit 1 (`Generated registry output is stale: 🧰️framework.json, 🏗️framework/🟦️.ts, .vscode/launch.json`). **None is pets**: the in-memory diff (`SCR/launch_diff.txt`) is `@semio-tech/print-rs` (two lines of `.vscode/launch.json`) and `print.viz-inference` replacing `print.viz-kernel` (catalog files). The pets rows are byte-equal (23 lines containing `pets` in expected and actual launch.json). Round one's port finding is fixed (see §3). | The print ticket regenerates (`plugin-registry:generate`). |
| 13 | info | `verify taxonomy report --scope "🎓️teaching"` | `clean=false errors=17`, **all at the root of `🎓️teaching`** (new workspace files: `.config/nextest.toml`, `Cargo.toml`, `Cargo.lock`, `bun.lock`, `package.json`, the `🔒️.lock` collision). None names a pets path or `AP`. Round one was clean because those files did not exist. | The teaching-workspace ticket registers them. |
| 14 | info | `TK/🗑️generated/f1/stack-dev/proctor-48228.exe` | A proctor started from this file is **running** (`proctor-48228.exe`, pid 8556, beside `bun` pid 48228; a second `proctor-5616-…`, pid 59932, is of unknown origin). Windows will not delete a running exe, so the folder cannot be removed at close until the stack is stopped. | F1 stops its stack before the cleanup. |

Everything else this audit looked at is consistent (§1 to §6). No blocker belongs to pets.

## 1. Taxonomy

| Command | Result |
|---|---|
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` (409 inventory entries) |
| same, `--scope "🧰️framework/🛍️products/❓️quiz"` | `clean=true errors=0 warnings=0` (352 entries) |
| same, `--scope "🎓️teaching"` | `clean=false errors=17 warnings=0`, all at the teaching root (finding 13); no pets path, no `AP` path |

**The 47 second-round names** (`SCR/names.ts`, parsed from `TAX`, all `members-of-*` lists): 20 module names, 16 test names, 11 fixture names (A2 report §1.1). All 47 are registered, each in the list of its kind, with the right U+FE0F (code points compared by the script). 46 have a directory:

| Kind | Names | Directory |
|---|---|---|
| modules (20) | `📝️draft 🚧️clearance 📏️spacing 🗓️schedule 👀️attention 🚶️locomotion 💞️sociability 🎯️choice 👥️population 🕰️clock 🎥️projection 🪢️swing 🧗️climbing 👆️gesture 💗️feeling ✨️effects 🪄️mischief` under `P/🔨️modules`; `🤏️grasp 🪞️lifting 🧰️gear` (and `✨️effects`, shared name) under `PR/🔨️modules` | all 20 exist |
| tests (16) | eleven Protocol v2 cases under `P/🧪️tests`; `🤏️pet-handling 🪞️fixture-lifting 🎆️effect-painting 🧰️gear-depiction` (React suites) under `P/🧪️tests` | 15 exist; **`🎮️pet-play` has none** (finding 7) |
| fixtures (11) | the eleven case names under `P/🧫️fixtures` | all 11 exist, one `🔣️.json` each |

Directory without a name: none. Every directory of `P` and `AP` either carries a name in some `members-of-*` list or is one of the structural directories resolved by their own kind (`🎯️targets`, `📦️packages`, `🔨️modules`, `🧪️tests`, `🧫️fixtures`, `🧬️schema`, `🔮️oracles`, `🎚️config`, `🏗️builder`, `📖️stories`); the verifier agrees. `members-of-teaching-pets` has 20 names equal to the 20 species directories of `AP`; `members-of-products` has `🐾️pets`. The three names that also sit in other lists (`📏️spacing`, `🗓️schedule`, `🎥️projection`: options/configuration/engine of other owners) and the eleven names that are in both `members-of-tests` and `members-of-fixtures` are by design.

## 2. Protocol v2

### 2.1 `discover`

`cd TEST && bun ./📜️script.ts discover` (read-only listing): 554 cases repo-wide. **23 pets cases, every one `[rust,typescript,python]`; none lacks a Rust adapter** (round one: 12 cases, the same three languages; the 11 new cases brought their Rust adapters, though those eleven files are untracked, finding 5). `nx show projects`: 23 projects `test-framework-products-pets-0f63c0-<case>` plus `@semio-tech/pets`, `pets-react`, `pets-rs`.

### 2.2 Pure contract checks

- `validateAllContracts` over the 23 pets cases plus the repository gates, `caseContractBreaches` alone, `testLayoutBreaches`: see finding 2 (1 breach, `SCR/contract_pure.txt`). Nothing written (these functions do not write; `contract` itself writes a cache and was not run).
- `SCR/grammar.ts` on every pets feature, adapter and vector file: **no problem** (the single line it printed, a `py adapter lacks scenario(s)` for `🚧️clearance-proof`, is a false positive: that adapter registers its ten references from a `REFERENCES` table in a loop, `🐍️.py:512-552`, and the keys equal the scenarios).

| Case | Capability = oracle (profile) | Scenarios | Adapter keys ts / rs / py | Vectors generated by |
|---|---|---|---|---|
| `⚗️chemistry-rules` | scipy (float) | 8 | 8/8/8 | `generate_feeling_vectors.py` |
| `✨️particle-motion` | numpy (float) | 6 | 6/6/6 | `generate_effects_vectors.py` |
| `🎞️animation-sampling` | scipy (float) | 6 | 6/6/6 | `generate_animation_vectors.py` |
| `🎣️grapple-reach` | scipy (float) | 8 | 8/8/8 | `generate_climbing_vectors.py` |
| `🎪️stage-trace` | `@no-oracle-pets-stage-trace` (ordered-json) | 3 | 3/3/3 | `generate_behavior_vectors.py` |
| `🎲️counter-randomness` | numpy (ordered-json) | 5 | 5/5/5 | `generate_kinematics_vectors.py` |
| `🏞️terrain-walking` | numpy (float) | 4 | 4/4/4 | `generate_terrain_vectors.py` |
| `👀️gaze-tracking` | numpy (float) | 3 | 3/3/3 | `generate_kinematics_vectors.py` |
| `👆️gesture-recognition` | scipy (float) | 9 | 9/9/9 | `generate_gesture_vectors.py` |
| `💗️feeling-dynamics` | numpy (float) | 10 | 10/10/10 | `generate_feeling_vectors.py` |
| `📐️turn-trigonometry` | numpy (float) | 9 | 9/9/9 | `generate_kinematics_vectors.py` |
| `🚧️clearance-proof` | scipy (float) | 11 | 11/11/11 (py by loop) | `generate_clearance_vectors.py` |
| `🤝️bond-dynamics` | numpy (float) | 6 | 6/6/6 | `generate_behavior_vectors.py` |
| `🦘️hop-ballistics` | scipy (float) | 7 | 7/7/7 | `generate_terrain_vectors.py` |
| `🦴️rig-solving` | numpy (float) | 6 | 6/6/6 | `generate_kinematics_vectors.py` |
| `🧗️wall-climbing` | scipy (float) | 13 | 13/13/13 | `generate_climbing_vectors.py` |
| `🧠️behavior-choice` | scipy (float) | 8 | 8/8/8 | `generate_behavior_vectors.py` |
| `🧬️schema-conformance` | jsonschema (ordered-json) | 3 | 3/3/3 | `generate_schema_vectors.py` |
| `🪀️spring-settling` | numpy (float) | 5 | 5/5/5 | `generate_animation_vectors.py` |
| `🪂️parachute-descent` | scipy (float) | 9 | 9/9/9 | `generate_swing_vectors.py` |
| `🪄️mischief-choice` | numpy (float) | 8 | 8/8/8 | `generate_effects_vectors.py` |
| `🪜️ladder-geometry` | numpy (float) | 5 | 5/5/5 | `generate_climbing_vectors.py` |
| `🪢️swing-dynamics` | scipy (float) | 13 | 13/13/13 | `generate_swing_vectors.py` |

Checked for all 23: exactly one `@capability-`, one `@oracle-` or `@no-oracle-`, one `@comparison-` and no other header tag; every scenario exactly one `@id-`, `@level-`, `@mode-` and no other tag, ids unique; modes used: `differential`, `property`, `conformance`, `error`; handler keys equal the scenario ids in all three languages with no extra key; every `shared://` URI (feature and adapters) resolves under `P/🧫️fixtures/<case>/🔣️.json`; every Rust adapter gates its subject behind `#[cfg(feature = "sut")]` and imports `semio_repo_test_host`; every `@oracle-` resolves in `P/🔮️oracles/🔣️.json`, the oracle declares the feature's capability and allows its comparison profile (`pets-float-v1` is the only local profile; `ordered-json-v1` comes from the global one); `@no-oracle-pets-stage-trace` resolves and declares `pets-stage-trace`, and its `substitutes` now include `independent-implementations` (round one's finding 4 is fixed). The 24 fixture files (23 cases + `📡️surface-survey`) all carry `$comment` (round one's `_comment` nit is fixed) with "never edited by hand" and name a generator that exists in `TK` (11 distinct generators); each feature header names a generator too. Fixture sizes: 7.7 MB in total, largest `🎪️stage-trace` 1.5 MB, `🧬️schema-conformance` 1.3 MB. The generators have no read-only mode, so "second run unchanged" is taken from the reports, not re-proved.

Registry capabilities with no feature: `pets-react-depiction`, `pets-react-gear-depiction`, `pets-react-effect-painting`, `pets-react-fixture-lifting` (on `pets-react-gl-matrix`), `pets-react-decorative-layer`, `pets-react-surface-survey`, `pets-react-fixture-lifting` (on `pets-react-aria-query`), `pets-react-pet-handling` (on `pets-react-user-event`): all belong to React suites without a feature (the quiz precedent). Every numpy/scipy/jsonschema capability is used by a feature.

### 2.3 Libraries imported by pets tests (`SCR/libs.ts`, `libs.txt`)

| Library | Imported by | Registry entry | Declared in | Installed |
|---|---|---|---|---|
| numpy 2.5.0, scipy 1.18.0, jsonschema 4.26.0 (python, 22 + 9 + 1 adapters) | `P/🧪️tests/*/🐍️.py` | `pets-numpy`, `pets-scipy`, `pets-jsonschema`, host packages | `pyproject.toml`, `uv.lock` | `.venv`: **numpy 2.4.3, scipy 1.17.1** (finding 8), jsonschema 4.26.0 |
| ajv 8.20.0 | `P/🔨️modules/✅️validation` unit suite, `S/🧪️tests/🐾️pet-cast` | `pets-ajv-structure` | P package (exact) | 8.20.0 |
| gl-matrix 3.4.3 | 8 unit suites, `PR` | `pets-gl-matrix`, `pets-react-gl-matrix` | P and PR packages (exact) | 3.4.3 |
| d3-ease 3.0.1, polygon-clipping 0.15.7 | `🎞️animation`, `🏞️terrain` unit suites | `pets-d3-ease`, `pets-polygon-clipping` | P package (exact) | 3.0.1, 0.15.7 |
| aria-query 5.3.0 | `📡️surface-survey`, `🫥️decorative-layer` | `pets-react-aria-query` | PR package (exact) | 5.3.0 |
| @testing-library/user-event 14.6.3 | `🤏️pet-handling`, `Q/🧪️tests/🐾️pet-companions` | `pets-react-user-event` (finding 2) | PR and quiz-react packages (`^14.6.3`) | 14.6.3 |
| vitest, jsdom, @testing-library/react, react, @vitejs/plugin-react, @playwright/test | runners and environment | none (not oracles by nature) | PR / site packages | vitest 4.1.10, jsdom 24.1.3, RTL 16.3.2, react 19.2.8, plugin-react 5.2.0 |
| serde, serde_json (Rust unit suites) | 14 `🦀️.rs` unit suites, `🧬️schema` | none (production dependency, and `float_roundtrip` dev-dependency) | crate manifest | workspace |

Python test imports beyond those three: none (`import a` / `import the` hits are words in docstrings). No third-party crate in the Rust adapters. Version drift in JS: none.

## 3. Workspace and launch

| Registration | State |
|---|---|
| root `package.json` workspaces | now the glob `🧰️framework/**` (+ exclusions) instead of the explicit lists of round one; covers the three pets packages. Not a pets change. |
| root `package.json` scripts | `dev:pets:stories`, `typecheck:pets` (new), `typecheck:pets:react`, `test:pets`, `test:pets:react`, `test:pets:rs` (lines 113-118); all six call `bun nx run …` and every target exists (`nx show project --json`: `@semio-tech/pets` test, test-quick, test-long, test-exhaustive, **typecheck**; `pets-react` the same plus dev; `pets-rs` build, check, test, test-quick, test-long, test-exhaustive). |
| root `Cargo.toml` / `Cargo.lock` | member `Cargo.toml:62`, workspace dependency `:159` (no consumer, finding 10); `Cargo.lock` has `semio-framework-pets` with `serde`, `serde_json`, equal to the crate manifest. The working-tree diff of both files is the print crate only. |
| `bun.lock` (root) | the diff against HEAD is print plus one pets line (`@testing-library/user-event` in the pets-react snapshot); manifest and snapshot equal for `pets`, `pets-react`, `quiz`, `quiz-react` (every section, every range); `lock-mismatches=0`. The teaching lock: finding 1. |
| `📋️project.json` | pets: `namedInputs` cover `🔨️modules/**/🟦️.ts`, `🧪️tests/**/🟦️.ts`, fixtures, schema; pets-react: modules, stories, builder, tests, fixtures; pets-rs: `**/*.rs`, schema, fixtures. `quiz-react` now lists the pets globs (round one's finding 8 is fixed); the site lists `AP/**/*` and `P/**/*`. |
| vitest / tsconfig | the unit-suite globs (`🔨️modules/*/🧪️tests/🔬️unit/🟦️.ts`, `🧬️schema/…`) and the typecheck includes (`🔨️modules/**/*.ts`, `🧪️tests/*/🟦️.ts`; react: `🧪️tests/*/🟦️.tsx` of the product) pick up every new module and suite without an edit. Aliases for `pets`/`pets-react` present in the site Vite and vitest configs, the quiz-react vitest and tsconfig `paths`. Playwright project `pets` runs `🐕️pet-walk`. |
| `.vscode/🧩️launch.seed.jsonc` | seven rows, lines 3403-3409, `presentation.group 3_dev`, orders 213.69, 213.7, 213.71, 213.715, 213.72, 213.73, 213.74, **all unique** among 2071 generated configurations (no duplicate name): `🧪️test🐾️pets🟦️`, `…⚛️react`, `…🦀️` → `…:test`; `🛠️dev🐾️pets🟦️🪁️typecheck` → `@semio-tech/pets:typecheck` (new), `🛠️dev🐾️pets⚛️react🪁️typecheck`; `🛠️dev🐾️pets📖️stories` (`PETS_STORIES_PORT=6069`), `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories` (6074, `PETS_MENAGERIE`) → `pets-react:dev`. Every command targets an existing nx target. |
| `.vscode/launch.json` | the pets rows are identical to the in-memory regeneration (finding 12 for the print drift). |
| `.claude/launch.json` | 72 entries, no duplicate name; `pets-stories` 6069 and `architecture-pets-stories` 6074 (lines 655-669), each port used once there. Search of the repository for `6069` and `6074`: besides the launch files, the script/builder defaults (`PR/📦️packages/🟦️typescript/📜️script.ts:24,27`, `PR/🏗️builder/🌐️vite/🟦️.ts:87`), the pets READMEs and a literal in a workspace-contract test (`S_OS_PORT: "6074"`, `…/🧪️tests/🔬️workspace-contract/🟦️.ts:2216`, a fixture of the `s` os harness, never started). Round one's collision on 6071 (the `s` wgpu harness) is fixed by the move to 6069. The 17 ports that repeat in the file (e.g. 6061, 6013, 6070) are attach/supervised/served aliases of one app. |
| Docker, CI | `S/🚀️deploy/Dockerfile.dockerignore` admits `**/Cargo.toml` and `**/*.rs` and the proctor's lock `🎓️teaching/Cargo.lock`; neither that lock nor `🎓️teaching/Cargo.toml` mentions pets (no teaching crate depends on the pets crate), so nothing is owed there. `.github/workflows/architecture-quiz.yml` installs with `bun install --frozen-lockfile` at the repository root, which no longer contains the site (finding 1, not a pets matter). |

## 4. Schema catalog and dependencies

### 4.1 Schema catalog

`schema generate --check` exit 1; `schema check` 9348 findings, one is `schema-catalog-stale`, none names pets; `schema verify` exit 1 (JSON and `.md` stale; 13 Rust-entry findings of other scopes, none pets). In-memory diff (`SCR/schema_diff.ts`): 3734 scopes on both sides, 3 differ (`framework.product.pets`, `framework.product.quiz`, `print`). Pets: the exports `Foothold`, `Pitch`, `PuffFrame`, `Trip` are missing from the catalog (121 expected, 117 recorded) and the three file hashes differ, because the twin files changed after the catalog was written; quiz and print differ by hashes only. Finding 3.

### 4.2 `🔒️dependencies.json` (`verify dependencies list all`, 314 third-party rows)

| Library | In the baseline | Current kind |
|---|---|---|
| ajv 8.20.0, gl-matrix 3.4.3 | yes | test-oracle |
| numpy, scipy (python), jsonschema (python) | yes | test-oracle (numpy was `test-runner` in the baseline) |
| @testing-library/react, vitest, @types/react*, @vitejs/plugin-react, vite, @playwright/test | yes | repository-tooling |
| jsdom ^24.1.3 | yes | test-oracle (was repository-tooling; reclassified by the quiz oracle, not new) |
| serde, serde_json | yes | production-build, production-runtime, test-runner |
| d3-ease, polygon-clipping, aria-query | **no** | test-oracle |
| @testing-library/user-event | **no** | test-oracle |
| @types/d3-ease, @types/aria-query | **no** | repository-tooling |

Nothing is unclassified. `literal-external` shows 7 oracle conflicts, one of them `rust:serde_json`, declared by 86 manifests including the pets and quiz crates (the optional `sut` dependency plus the dev-dependency are two uses of one crate); repo-wide, not pets-specific.

## 5. Docs registries

| Document | State |
|---|---|
| `P/README.md` | 19 of 25 core modules have a Layout row; six do not, React modules have no rows, no case index (finding 6). The domain-model sections cover states, tricks, purr, emitters, gear, chemistry, the hand, mischief and the React props (`play`, `mischief`, `controls`, `walls`, `props`, `ref`, `tempo`); `🧰️gear`, `✨️effects` and `🤏️grasp` are named in the React section. `README` run commands include `typecheck:pets`. |
| `AP/README.md` | names all 20 species; sections for states, tricks and gear, chemistry, adding a state/trick/reaction, tests (pet-cast, pet-walk). |
| `Q/README.md` ("Pets", lines 684-755) | preference `pets`, switch, `petsPlay`, `petsMischief`, topics (`data-pet-prop`), "Play with the pets", tempo seam, stage classes: complete. |
| `S/README.md` | pets section, ports 6074, the two suites, bundle chunks: complete. |
| Port table of the root `README.md` | neither 6069 nor 6074 (finding 10). |

## 6. Leftovers

- Product trees (`P`, `AP`, the quiz module and the three quiz/site pet suites, 455 entries): no scratch file, log, image, shell script, `.mjs`, `.py` other than the 23 adapters, no `__pycache__`, no empty directory; no `[DEBUG]`, `TODO`, `FIXME`, `console.log`, `debugger` in `P` or the quiz glue. Path budget: longest file 125 UTF-8 bytes / 93 units, longest directory 110 / 78, nothing over the 240 / 213 / 202 limits.
- Untracked files outside tickets, `P` and print: none (181 untracked files in the repository, all in tickets, `P`, `QR` glue or print).
- Ticket folder: finding 11 (nested `.🧬semio` tree, `__pycache__`); harness directories `quiz_pets_preview`, `c1a_harness`, `c1b_harness`, `wp_g_harness`, `wp_q_import_cost` hold inputs (HTML, vite and vitest configs, probes) and stay under the "inputs are kept" rule; 306 entries in the folder root.
- `TK/🗑️generated` to be deleted at close (finding 14 for the running proctor): **367 MB, 3323 files** at 19:25 (round one's auditor found 12 MB of screenshots there).

| Subfolder | MB | What it is |
|---|---|---|
| `f1` | 65.3 | F1's stack (`stack-dev` with `proctor-48228.exe` 13.7 MB, a Vite deps cache with `typescript.js` and its 14 MB map) |
| `wp-m` | 56.3 | a cargo target directory (`build/release/…`: `.exe`, `.pdb`, `.rmeta` files) |
| `c3`, `c3b` | 48.3, 47.2 | preview caches (`typescript.js` 10 MB + map 14 MB each) and budget builds |
| `c2` | 20.7 | gallery screenshots |
| `h3`, `h4`, `h2`, `h1` | 18.7, 18.1, 10.2, 9.0 | art sheets |
| `a2` | 14.4 | A2 logs (a 6.4 MB taxonomy-teaching dump) |
| `audit2-registrations` | 11.3 | this audit (a 6.4 MB taxonomy-teaching dump, a 4.2 MB `schema check` dump, a 0.8 MB parity JSON) |
| `a9`, `a1`, `d1`, `d2`, `b5`, `b4`, `c4`, `d3`, `a8`, `a3` | 7.4, 6.1, 5.9, 5.8, 4.7, 3.2, 3.0, 2.9, 2.7, 1.4 | logs, traces, dumps |
| `wp-e`, `b1`, `c1a`, `c1b`, `b3`, `p1` | 1.0, 0.7, 0.3, 0.1, 0.1, 0.0 | small |

## 7. Round-one findings: status

| Round 1 | Status |
|---|---|
| 1 `shared-presence` line 75 | the assertion is gone (`S/🧪️tests/👥️shared-presence/🟦️.ts` now has no count of `[data-crowd-item]` under the item); not re-run (no servers) |
| 2 schema `.md` stale | **again stale** (finding 3, new cause: the twins changed at 17:29) |
| 3 port 6071 collision | fixed (6069) |
| 4 stage-trace no-oracle text | fixed (`independent-implementations` in `substitutes`, present tense) |
| 5 dependency baseline | open, now six pets entries (finding 4) |
| 6 two dead fixture names | fixed; one new dead test name (finding 7) |
| 7 `.venv` drift | open (finding 8) |
| 8 quiz-react `namedInputs` | fixed |
| 9 façade docstring / crate description | docstring fixed, manifest descriptions open (finding 10) |
| 10 workspace dependency without consumer | open (finding 10) |
| 11 `_comment` key | fixed |
| 12 parity scanner mapping | open (finding 9) |
| 13 semantic census | not a gate, not re-run |

## 8. Not run, and limits

cargo, vitest (any suite), Playwright, deploy-check, parity, the generators, `contract` (writes a cache; I called its pure functions), `verify docstrings`/`debug-tags` (acceptance records), `schema generate`/`docs`, `bun install` (including `--dry-run`: I could not prove it writes nothing and the ticket rule is "never run `bun install`", so the lock consistency of §3 and finding 1 is reasoned from the files by `SCR/lockcheck.ts`), any server. I did not re-prove that the generators reproduce their fixtures ("unchanged on a second run"); Rust adapter content was checked structurally, not executed. Time-dependent facts (the running proctor, folder sizes, which files are untracked) are as of 19:25.
