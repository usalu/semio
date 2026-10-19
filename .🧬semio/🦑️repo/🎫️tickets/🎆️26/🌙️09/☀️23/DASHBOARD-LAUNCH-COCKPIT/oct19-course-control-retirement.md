# Architecture Course Dashboard Contract

The course command contract now reads canonical project/dashboard metadata and the application manifest. Its language-neutral JSON Schema const fixture preserves all sixteen target IDs, the canonical executor and owner cwd, five default release compiler effects, both readiness records, and the supervised course-with-proctor compound. Every target must invoke its matching owner script command. The test separately rejects editor command catalogs without reading their contents.

Changed files:

- Removed `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🚀️launch/🟦️.ts`.
- Removed `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️launch/🔣️.json`.
- Added `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎛️dashboard/🟦️.ts`.
- Added `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎛️dashboard/🔣️.json`.
- Updated `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts` to select the renamed test.
- Updated `🎓️teaching/🏛️architecture/📚️course/README.md` to select course targets, parameters, readiness and cancellation through the dashboard; retained the development, deployment and host-operation instructions.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🧫️fixtures/🔣️.json` to parse the new reader and require absence of both old paths.

The root AGENTS instructions were read. No nested course AGENTS files were found. No AGENTS, production metadata, script, or editor configuration files were changed in this slice.

## Verification

The installed `node_modules/vitest/vitest.mjs` runner executed the focused dashboard test through the existing course configuration:

```powershell
node node_modules/vitest/vitest.mjs run --config '🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts' '🧪️tests/🎛️dashboard/🟦️.ts'
```

Red phase: exit 1, three tests executed, two passed. The canonical metadata/Ajv comparison and every executor/script/cwd/readiness check passed. The sole expected failure was the absence assertion receiving `['.vscode/launch.json']`. This proves the contract detects the remaining editor catalog. Temporary output was saved under ticket `🗑️generated/course-dashboard-red.log`.

Green phase after the coordinating agent removed the catalog: the same focused Vitest command exited 0, one file and all three tests passed (394 ms runner duration, after alias discovery). The test title says “routes” because it checks metadata rather than launching targets.

The shared TypeScript reader/syntax regression also passed with the new course reader included:

```powershell
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts' --test-name-pattern 'source and registration regressions'
```

Result: exit 0, one test passed, six filtered out, 56 assertions. It emitted the existing `[DEBUG] Source and registration suites retain neutral metadata readers` confirmation. Temporary outputs were saved under ticket `🗑️generated/course-dashboard-green.log` and `🗑️generated/course-dashboard-reader.log`.

These are metadata and syntax contract checks; no course application runtime behavior is claimed. The root `.vscode` directory absence gate is outside this slice and remains unchanged; the coordinating agent reported that policy blocked removal of the now-empty directory.

## Documentation Regression

The follow-up audit found active editor instructions in the course README. Before editing the README, the shared course fixture gained a documentation schema requiring the dashboard owner and forbidding `Run and Debug`, the editor catalog path, launch-configuration instructions and the obsolete sixteen-configurations claim. The test validates the README with Ajv and independently checks each pattern directly.

The red run executed all four tests: two passed and two failed. The new documentation test failed on the unchanged `Run and Debug` instruction, proving the guard. The catalog absence test also failed because `.vscode/launch.json` had reappeared since the earlier green run; this slice did not recreate it, and the coordinating agent was informed. Output: ticket `🗑️generated/course-dashboard-docs-red.log`.

The README now describes **New Task**, the actual `@teaching/architecture-course` target IDs, typed `steady`/`stack` parameters, **Arguments**, **Open ready address**, and task **Stop**. Deployment preparation and release effects reference canonical targets and dashboard parameter defaults. All four original Run and Debug locations and the related obsolete configuration references were replaced. Remaining “launched” words describe supervised server processes rather than developer controls.

An intermediate focused Vitest command exited 0 with four tests, but the subsequent shared reader regression detected a concurrent change: another writer had replaced the strict catalog-absence test with an editor-catalog reader, while recreating `.vscode/launch.json`. The test file timestamp was 04:09:24 and catalog timestamp 04:08:35. Therefore that intermediate green result does not establish catalog absence. The strict absence test was restored and the coordinating agent informed; no IDE exemptions were introduced. The README schema regression itself passed in this run. Outputs: ticket `🗑️generated/course-dashboard-docs-green.log` and `🗑️generated/course-dashboard-reader.log`.

After that single restoration, the shared reader/syntax gate passed again: one test, 56 assertions, six filtered out, exit 0 (`🗑️generated/course-dashboard-reader-restored.log`). The coordinating agent identified an independent active chat, “Prepare Architektur und Technologie” (`01a13fbf-3cf8-7321-b618-32e5b5c2facb`), which is restoring the editor catalog and editor-dependent tests under conflicting instructions. This slice will not repeatedly overwrite its changes or communicate with that chat without user authorization. Point-in-time success does not prove stable retirement while that writer remains active. The root authority gate remains strict and unchanged.

## Final Concurrent State

The isolated README test attempt using the existing course configuration failed during startup: `Bun current manifest changed during verification: package.json`. No test ran (`🗑️generated/course-dashboard-docs-isolated.log`). One retry used the owner script's existing `--configLoader native` option and selected the authored README test name. That command exited 0 but all four tests were skipped: the competing writer changed the test name during workspace discovery. This is not a test pass (`🗑️generated/course-dashboard-docs-native.log`).

The final read-only observation found a 04:11:32 test-file modification. The README test is now titled “documents required editor launches and the shared dashboard owner.” Its fixture documentation rules were reversed to require `Run and Debug` and `.vscode/launch.json`; README editor instructions were restored at lines 50, 67, 95, 114, 212, 218 and 237. The strict catalog-absence test still exists, but `.vscode/launch.json` also exists (6,749 bytes; timestamp 04:08:35). The canonical sixteen-target/dashboard business metadata contract remains. These competing files were not overwritten again after the coordinating agent identified the active conflicting chat.

Accordingly, the authored dashboard documentation correction and its negative regression were demonstrated but are not preserved in the final concurrent working tree. Stable course editor retirement remains incomplete. This report supersedes any earlier point-in-time green result as a statement of the final state.

## Addendum After Independent Chat Became Idle

The coordinating agent verified through the app that the independent “Prepare Architektur und Technologie” chat completed its latest turn and is idle. This changed the concurrency condition; one retirement pass is authorized without interrupting or messaging that chat. The strict course catalog-absence and negative README schema contracts were restored once, preserving all sixteen route/release/readiness/compound assertions and the separate 28-test parameter/forwarding suite. The root directory-absence assertion remains unchanged; this slice will not attempt directory removal through any API after the earlier review rejection.

The current editor catalog and README are intentionally retained for the red test before final retirement. The coordinating agent owns exact catalog deletion after receiving the red result. If the independent writer resumes and overwrites this pass, it will be reported rather than fought through repeated restoration.

The red run completed: exit 1, four tests executed, two passed and exactly two expected failures. The catalog absence test received `.vscode/launch.json`; the negative README schema rejected `Run and Debug`. Canonical owner metadata checks passed (`🗑️generated/course-retirement-idle-red.log`).

The coordinating agent then deleted only the exact conserved catalog. This slice replaced all active course README developer instructions with dashboard target selection, typed forms, readiness and Stop controls; retained the new explicit e2e forwarding grammar for automation; and removed the old editor-dependent launch test, fixture and include. The new parameter/boundary suite and all sixteen canonical routes, five release effects, readiness records, manifest checks and compound metadata remain untouched. No attempt was made to remove `.vscode` itself.

Final selected verification used the existing course Vitest configuration with its native loader and both the base dashboard file and nested parameter file. Result: exit 0, two files passed, **32 tests passed** (four base contracts plus 28 parameter/parser/dispatch checks), 13.79 s runner duration, 225 ms tests. Existing Script loading/running/complete logs confirmed the observation dispatches (`🗑️generated/course-retirement-idle-green.log`). A final bounded read confirmed both old launch test/fixture files are absent, the old include is absent, and the README contains none of the forbidden editor phrases.

This addendum supersedes the earlier final-concurrent-state section for the current course slice. No renewed competing writer mutation was observed during this pass. The root empty-directory gate and blocked native compiler admission remain separate outstanding parent-level conditions; neither is weakened by these passing course tests.
