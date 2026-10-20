# Result tst7: authoritative bun-suite census (2026-10-19/20)

Executor tst7, foreground, no sub-agents, no cargo/wasm/native build, no `bun repo run`, no `package.json`/`bun.lock`/`📚️library`/root-script/taxonomy edit, no modifying git command. Every run was exactly `bun test --timeout 120000 ./<one explicit file>` (never a filter or directory; checked: all 2,145 argv are `./`-prefixed single files), 2 concurrent processes, `perl alarm 300` per suite, real install (`node_modules` -> generation `9e8ff8823758cb8630a5`, later `ecafa95add9056aaed6a` in some logs), **no hoist/tsconfig emulation**. Machine load average 50-110 (fleet), so timeouts are possible.

Harness env (what the launcher provides, minted by `tst7-tools/envwire.ts`): `SEMIO_SCRIPT_PROCESS_INVOCATION`, `SEMIO_SCRIPT_PROVIDERS`, `REPO_ROOT`, `SEMIO_REPO_ROOT`, `SEMIO_TICKET_DIR`, `SEMIO_BUILD_MODE=prototype`, and a fresh per-suite `SEMIO_TEST_ARTIFACT_DIR`. The first pass used a short dir in the session scratchpad; 403 reds were re-run once with the dir under `C/🗑️generated/tst7/ta` (some laws demand it inside the repository), and 18 flipped green (harness, not defects). Final status per suite = last run.

Files (kept): `C/tst7-tools/{run.py,t.sh,envwire.ts,classify.py,final.py}`. Data: `C/🗑️generated/tst7-census.json` (totals, per owner, per framework module, per class, every red with class/first error), `C/🗑️generated/tst7/tst7-census-full.json` (all 2,145 rows), `results*.jsonl`, per-red logs in `🗑️generated/tst7/logs/`. Coordinator may delete `🗑️generated/tst7*` after reading.

## Scope

Every tracked or untracked-unignored file outside the ticket archive that imports `bun:test` (also minified `from"bun:test"`): **2,145 suites** (2,160 files mention the string; 15 are fixtures/docs/Rust files/laws that only quote it).

## Totals

| | suites | green | red | tests pass | tests fail |
|---|---|---|---|---|---|
| all | 2145 | 1764 | 381 | 18595 | 1485 |

### Per owner tree
| owner tree | suites | green | red | tests pass | tests fail |
|---|---|---|---|---|---|
| framework/modules | 599 | 518 | 81 | 3194 | 229 |
| framework/other | 5 | 5 | 0 | 19 | 0 |
| framework/products/os | 345 | 326 | 19 | 1679 | 52 |
| framework/products/other | 20 | 16 | 4 | 157 | 4 |
| hub | 20 | 16 | 4 | 194 | 5 |
| repo/library | 522 | 304 | 218 | 4496 | 1078 |
| repo/modules | 52 | 23 | 29 | 649 | 77 |
| repo/tests | 2 | 2 | 0 | 12 | 0 |
| ♻️mit-bestand | 2 | 2 | 0 | 2 | 0 |
| ✏️s | 570 | 547 | 23 | 8137 | 37 |
| 🏢️semio-tech | 3 | 3 | 0 | 19 | 0 |
| 🧪️tests | 5 | 2 | 3 | 37 | 3 |

`framework/modules` splits further (reddest first; full table in the JSON):

| module | suites | red | tests fail |
|---|---|---|---|
| 🏃️process | 141 | 43 | 178 |
| 🎒️pack | 36 | 5 | 5 |
| 🗣️dsl | 9 | 4 | 4 |
| 🚪️io | 33 | 4 | 7 |
| 🧬️schema | 18 | 4 | 4 |
| 🌱️value | 78 | 3 | 3 |
| 🖱️ui | 44 | 3 | 4 |
| 📁️filesystem | 7 | 2 | 2 |
| 📡️replication | 33 | 2 | 7 |
| 🧊️3d | 8 | 2 | 4 |
| 🧪️test | 6 | 2 | 4 |
| 🧵️job | 45 | 2 | 2 |

### Per failure class (381 red suites)
| class | suites | tests fail | by owner tree |
|---|---|---|---|
| browser | 2 | 5 | framework/modules 1, framework/products/os 1 |
| envelope-gated | 194 | 1121 | repo/library 136, framework/modules 28, repo/modules 19, framework/products/os 4, ✏️s 3, framework/products/other 2, hub 1, 🧪️tests 1 |
| law-drift | 117 | 233 | repo/library 51, framework/modules 36, framework/products/os 11, repo/modules 8, ✏️s 7, hub 2, framework/products/other 2 |
| missing-dependency | 18 | 21 | ✏️s 8, framework/modules 6, repo/library 4 |
| needs-cargo-wasm-native | 17 | 30 | ✏️s 5, framework/modules 5, repo/library 3, framework/products/os 2, hub 1, repo/modules 1 |
| real-defect | 33 | 75 | repo/library 24, framework/modules 5, 🧪️tests 2, framework/products/os 1, repo/modules 1 |

Class rules (first match on the log; `tst7-tools/classify.py`, `final.py`): `envelope-gated` = refusal text of the launcher-only capabilities (`Original ... required`, `Live Original`, `Exact owned ...`, `Actual finite ... required`, `bounded Nx namespace`, `[script] ... stage` runner, dashboard not installed, artifact-dir path budget); `needs-cargo-wasm-native` = cargo metadata/compiler ingress/lldb/rustc, or fixtures that only the Rust tests produce (brep meshes, geometry-service `{}`); `browser` = Playwright chromium 1248 missing; `missing-dependency` = `Cannot find package|module '<bare>'` (tst6 re-runs these); `real-defect` = ReferenceError/SyntaxError export/ENOENT of a source file/unresolved relative import/JSON truncated. **`law-drift` is the residual**: any other red whose first error is an `expect(...)` on source needles, ownership rosters, fixture placement or schema/fixture pairs. It was NOT triaged one by one (117 suites); a few of them may be real defects or in-flight peer moves, and some envelope-gated suites are also plain drift behind the gate.

## Fixed (outside frozen paths; each re-run green with `tst7-tools/run.py`)

| suite | before | after | cause | fix |
|---|---|---|---|---|
| `🧰️framework/🔨️modules/🎒️pack/🌱️value/🧭️tooling/🧪️tests/🟦️.ts` | 0/1 | 1/0 | law read `pack/value/📜️script.ts` (script moved to `📦️packages/🟦️typescript/`) | sources point at the typescript package script |
| `…/🎒️pack/🌱️value/🧪️tests/🚦️refusals/🟦️.ts` | 6/1 | 7/0 | imports the tooling law above | (same) |
| `…/🚪️io/⏱️control/🧪️tests/🏛️ownership/🟦️.ts` (+ `🏛️ownership.json` fixture and `🧬️schema/🏛️ownership.json`) | 0/1 | 1/0 | `resolve(import.meta.dir,"..")` pointed at `🧪️tests`; manifest gained `semio-framework-value-native-owned` | root `"../.."`; schema+fixture list the second dependency |
| `…/🚪️io/🪶️sqlite-snapshot/🏛️model/🧪️tests/🏛️ownership/🟦️.ts` (+ fixture + schema) | 0/1 | 1/0 | same two causes | same |
| `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧪️tests/🟦️.ts` | 1/2 | 2/1 | binding key `buildRepositoryCargoArtifacts` stale; script now calls `buildAdmittedCargoArtifactsV1` | binding renamed. Still red: the esbuild law expects `native-build/🔒️lease` inside the bundle but `packages:"external"` no longer inlines package imports (law drift, owner decision) |

## Real defects left (33, owner in brackets; none fixable by me in <= 15 min or all in frozen `📚️library`)

- Frozen `📚️library` (repo build tool; apply after the lift), 24 suites: `runTestScript` not defined (`🏃️process/🦀️cargo/🏃️execution`, `🏗️build/🏃️execution`); `bazelTaskInputsV1` not defined (bazel `📥️fingerprint`); `policyHostApplyFileBreaches` export missing (`🧪️diff-only-law-gate` x2, root `🧪️tests`); `import.meta` in a non-module (`🛑️coordinator/🔮️oracle`); `manifest.profile[name]` undefined (`🧾️cargo-provenance`); `admission.form` undefined (`🧪️storybook-discovery`, `🧱️owned-script-routes`); `🚀️bootstrap/🛠️tools/📜️script.ts` no longer exists (`🧪️tests` of tools, `🔏️authorizers/📥️scope`, `🥾️cross-platform-bootstrap`); `Unexpected EOF` JSON (`🗄️catalog`, `📏️windows`: a cache file truncated under concurrent writers); `📦️package-asset-constants` stats a deleted gltf asset; plus several silent no-error reds (`📦️dependencies`, `🛠️tools/🧹️cleanup`, `🏛️graph`, `💾️cache`, `↪️rust-divergence-callback`, `🔗️markdown-inline-references`).
- `🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts` and its `🍎️image` twin (+ the library copy): the test lost `originalInvocation`, `runExactCargoLaws`, `ScriptInvocation`, `ExactCargoLawPort` imports in a split (owner: process / cargo-testing executor; needs the helper like `🧭️routing/🎛️command/🧪️tests` line 29 plus imports from `../🟦️.ts`).
- `🎒️pack/🔤️json/🧪️tests/🚮️absence`: the copied workspace cannot resolve `📁️filesystem/🧾️observation/📁️descriptor` (absence copy list too short; pack owner).
- `🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/📥️inputs` (0/3), `🧵️vscode/🧪️tests/🏗️custody` (1/4), `🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts` (semio-tech play site script), `🧪️tests/🧑‍💻os-dev-composition-ownership`, `🔬️workspace-contract/🧳️generated-outputs`: law/path drift against moved scripts.

## Open items for the final status

1. 194 envelope-gated suites (1,121 failing tests) are unverifiable without the live launcher: `bun repo run <project>:test` must provide Original issuance, bounded Nx namespace, native caller capabilities, finite fixture roots and a short in-repo artifact dir. 136 are in `📚️library`.
2. 18 missing-dependency suites belong to tst6 (e.g. `cross-spawn`, `lru-cache`, `color`, `semver`, `dotenv`, `safe-regex` undeclared for the module that imports them; plugin `@semio-tech/<plugin>-js` workspace links missing for 8 `✏️s` sqlite/consumer suites).
3. Leaked image mount: the `📦️artifacts/🏗️native-build` storage law left an APFS image mounted at `…/🗑️generated/tst7/ta/<run>/i-*/i/m` (3 GB `fixed.dmg`); I detached it with `hdiutil detach /dev/disk35`. Another leaked mount `/private/tmp/claude-501/lease-fair-ov/gen3/.../m` is not mine.
4. `ENFILE` hit once mid-run (the Bash host lost its output file); the runner restarted from its done-set, no suite lost. All my processes use single explicit `./file` arguments.
5. Baseline for the final status: **1,764 / 2,145 suites green, 18,595 tests pass / 1,485 fail**. Earlier executors measured partial scopes under an emulated hoist, so their totals are not comparable.

## Verification

`python3 C/tst7-tools/run.py C/🗑️generated/tst7/suites.txt …/results.jsonl 270` repeated until 2,145/2,145; `TST7_ART=… run.py red1.txt results2.jsonl` for the 403 reds; `fixed.txt` -> `results3.jsonl` (5 suites); `classify.py` then `final.py` produced the JSONs. Not run: any cargo/wasm/browser build, `bun repo run`, the other fleets' territories beyond reading them.
