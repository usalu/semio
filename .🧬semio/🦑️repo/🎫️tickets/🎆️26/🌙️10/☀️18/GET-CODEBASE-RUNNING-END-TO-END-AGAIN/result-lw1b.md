# Result lw1b: product-law tests moved out of plugins (WP1 remainder)

Executor `lw1b`. Foreground, no cargo, no sub-agents, no modifying git command, no `package.json` / `bun.lock` edit, nothing under the frozen storage / revision paths. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `T` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, `C` = this ticket folder.

## Outcome

The five open plugin tests of `result-lw1.md` section 5 are gone from the plugins. Four moved into the repository product that owns the law, one was a duplicate of a case its owner already runs and was retired with its vitest project.

| plugin test (was) | owner and new place | result |
|---|---|---|
| `📐️cad/…/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement` (applied `toolJob*Exact` laws and `WORKSPACE_ROOT` to OS store sources, ran as a `twins` self-test inside the cad Rust package script) | repo product tool-job laws: `L/🧪️tests/🔬️cad-presence-retirement/🟦️.ts`, now a `bun:test` case (`cadPresenceRetirementChecks()` = 83 checks, fixture still read by path from the cad plugin because the cad Rust tests consume the same JSON) | 1 pass |
| `📕️norm/📇️registry/🧬️contract/🧪️tests/🧱️definition-ownership` (applied the Rust source direction engine to the norm contract) | repo product dependency direction: `L/🧪️tests/🧱️norm-definition-ownership/🟦️.ts`, fixture `L/🧪️tests/🧫️fixtures/🧱️norm-definition-ownership/🔣️.json` (gained `plugin`, `contract`, `project` fields so no path depends on the plugin location) | 3 pass |
| `📕️norm/🧪️tests/🔮️oracle-source-ownership` (repo-test platform law about the norm oracle adapters; `🟦️.ts`, `🐍️.py`, fixture) | repo test platform: `T/🧪️tests/🔮️oracle-source-ownership/{🟦️.ts,🐍️.py}`, fixture `T/🧪️tests/🧫️fixtures/🔮️oracle-source-ownership/🔣️.json`; imports of `@semio-tech/repo-test/*` became relative imports inside the module; the root comes from the SDK `findWorkspaceRoot` | 6 pass |
| `🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧩️index` (only vitest case of `@semio-tech/animate-js`) | none needed: `🎤️presentation/🧪️tests/🧭️slide-glob-assembly/🟦️.ts` is the same case, same assertions (verified by diff: only the import and the describe title differ); it passes (`bun test`, 1 pass). The animate vitest project is retired | n/a |

## Changes

Inputs re-pointed (not just files):
* `T/📋️project.json`: new target `test-oracle-source-ownership` (cached, inputs: `normOracleSources`, norm `*.py` / `*.feature`, oracle manifests, taxonomy, EN 1991 fixtures and assets). `T/📜️script.ts`: `test oracle-source-ownership`.
* `L/📦️packages/🟦️typescript/📋️project.json` and `…/📜️script.ts`: targets / commands `test-norm-definition-ownership` and `test-cad-presence-retirement` (cached, inputs name the test, fixtures, the laws, the norm contract sources resp. the OS store, plugin, channel and value-read sources the test reads).
* Norm: `📦️packages/🟦️typescript/📋️project.json` lost `test-oracle-source` and the `normOracleSources` named input (the repo-test project already owns the same list; the moved test now asserts the plugin no longer declares them); its `📜️script.ts` lost `OracleSourceScript`. Norm contract Rust package: `test-definition-ownership` target and `TestDefinitionOwnershipScript` removed.
* Cad Rust package `📜️script.ts`: `twins` option and its import removed (the `twins` option itself stays in the framework API).
* Animate: removed `🧪️tests/🎚️config/🟦️.ts`, `📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`, the `🎛️apps` test and the now empty directories.
* Launch SEED (`.vscode/🧩️launch.seed.jsonc`, never launch.json): the norm rows now run `@semio-tech/repo-test-domain:test-oracle-source-ownership` (`⚖️test-oracle-source-ownership🦑️repo🔨️modules🧪️test`) and `@semio-tech/repo-lib:test-norm-definition-ownership` (`🧪️test🦑️repo🧱️norm-definition-ownership`); new row `🧪️test🦑️repo🔬️cad-presence-retirement` (group `9_gates`, order 900.058025); `@semio-tech/animate-js` removed from the `projectTarget.test` input. `.vscode/launch.json` regenerated with `bun ./📜️script.ts generate launch`.
* Root `📜️script.ts`: no static import touched by these moves (it imports only `toolJobFixedOperationRegistrySelfTests`, which did not move). Verified `bun build --target=bun 📜️script.ts`: bundled 2,355 modules, no error.
* Ticket input kept: `C/lw1b-tools/{exports-needed.json,files.txt,tc.ts,wp1.ts,kinds.ts}`.

## Verification (exact commands, outcomes)

All from `/Users/ueli/Documents/semio`, foreground. `SEMIO_TEST_ARTIFACT_DIR` pointed into `C/🗑️generated` (deleted).
* `bun test --timeout 900000 ./L/🧪️tests/🧱️norm-definition-ownership/🟦️.ts ./L/🧪️tests/🔬️cad-presence-retirement/🟦️.ts ./T/🧪️tests/🔮️oracle-source-ownership/🟦️.ts`: 10 pass, 0 fail, 147 expects, 130 s (final run after the last import edit). Earlier single runs agree: norm 3 pass (0.3 s), cad 1 pass (6.7 s), oracle 6 pass (161 s).
* Presentation owner: `bun test ./🧰️framework/🛍️products/🎤️presentation/🧪️tests/🧭️slide-glob-assembly/🟦️.ts`: 1 pass.
* `bun test ./L/🧪️tests/🚀️launch-catalog/🟦️.ts`: 30 pass (launch catalog law after the seed edit); `bun ./📜️script.ts generate launch --check`: "launch.json is the expansion of the seed".
* Route resolution `bun C/seed-tools/verify-routes.ts .vscode/launch.json`: 13,402 routes, 11,288 ok + 769 ok-noTarget + 1,337 ticket-scoped, the same 8 "by construction" leftovers as `result-seed.md`; the three new or retargeted rows resolve.
* Strict tsc (`bun C/lw1b-tools/tc.ts C/lw1b-tools/files.txt`): the 3 moved tests plus the 5 touched scripts, 0 diagnostics (the tool was checked to report a seeded error).
* The cad test was instrumented once with a `[DEBUG]` log to read the check count (83) and restored; no `[DEBUG]` string is left in the three files.
* Not done: running the new targets through `bun repo run …`. `bun repo run @semio-tech/repo-lib:test-norm-definition-ownership` only loops on `repository:command running` (build graph construction under load 35+), so it was killed after about 90 s. `bun ./📜️script.ts test …` in the package needs the parent envelope of the CLI. The wiring (project.json target, router branch, seed row) is syntax-, schema- and route-checked, and the test files themselves run through `bun test`.

## Dependency-cruiser, WP1 scope (plugins to `framework` outside `💻️os`)

Same config and command as `result-lw1.md` (`C/lw1-tools/cruise.sh`, plugin sources only, framework not followed), filtered by `C/lw1b-tools/wp1.ts`.

| | edges |
|---|---:|
| `result-lw1.md` after (start of this task) | 32 (13 files) |
| full cruise of this task (started before the old `norm/…/oracle-source-ownership` directory was deleted, so it still contains it) | 13 |
| of which the stale old oracle test (deleted since; now under `T`) | 10 |
| **left in WP1** | **3**: `📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🎚️config/🟦️.ts` (2 ncpr, 1 sdk-only) toward `ui/targets/react/build-tooling`; the animate config that shared this finding is gone |

The cad presence test (12 edges), the norm definition test (3), both animate files (4) and the old oracle test (10) no longer exist in any plugin. Scoped re-cruise of the three moved tests (`C/lw2-tools/cruise-files.sh`, 23 modules): 2 violations, both `no-cross-package-relative` of the oracle test into `L/🔍️discovery/🔐️taxonomy-authority` and `…/🗂️file-kind-resolution` (repo-internal, belongs to WP10); the other two moved tests: 0. The two entries are recorded in `C/lw1b-tools/exports-needed.json` (`@semio-tech/repo-lib` has no `exports` for them; manifest frozen).

## Still open

1. `@semio-tech/animate-js` is retired in everything but its manifest: `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json` (and its `bun.lock` entries, and the workspace list of `♻️mit-bestand/package.json`) are frozen and still exist; its `scripts.test` forwards to `repo run @semio-tech/animate-js:test`, a target that no longer exists. When the freeze lifts: delete the package directory and the three lock / workspace entries. Until then do not run `nx run-many -t test` blindly (Nx may infer `test` from the script and recurse).
2. `./ui/targets/react/build-tooling` for `@semio-tech/framework` (`C/lw1-tools/exports-needed.json`) now has exactly one importer left (aec-building-structure config); the 3 remaining WP1 edges vanish with that one manifest entry.
3. `@semio-tech/repo-lib` exports for the two discovery modules (above) for the oracle test; or move the two helpers behind the repo-test module's own surface in WP10.
4. The new repo-lib targets have no `package.json` forwarding scripts (freeze), like the other freshly added ones.
5. The `SEMIO_TEST_ARTIFACT_DIR` of the re-pointed norm definition launch row still names a swept ticket folder (carried over unchanged; the runner derives its own artifact directory).
6. Finding for the owner of the test platform: `loadOracleRegistry` walks the whole repository once per process and took 257 s at load 35; the oracle test was already red on its 60 s timeout before the move. It now warms the registry in a `beforeAll` with a 900 s budget so the per-case timeouts measure the cases, not the walk.
