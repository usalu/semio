# Result E70 (tests-real-modules)

## 1. Ratchet: extracted function text -> real-module loader
Ceiling 293 files / 818 uses -> **257 files / 770 uses** (36 files left the list, 48 uses removed). The law test `process/🧪️testing/🧩️extracted-function/🧪️tests` passes (1/0) after every batch. Ceilings were lowered with `T/🧰️scripts/E70/ceiling-update.py` (reloads the live JSON, recounts the named files, changes only those keys). Full file list with after-hashes: `table-E70.md`. Claims: `claims-tests.md` (E70 lines).

Method: the test calls `loadProductFunctionsV1` / `loadProductFunctionsSyncV1` with the real module and only the ports it replaced before; module-level helpers are now the product's own (a moved helper fails at load). No export was needed: the loader reads unexported top-level functions, classes and consts from the source file. No port declaration or product export was added, so no schema-first step was required.

| file (short) | before | after |
|---|---|---|
| inference/owner-discovery source-matches | n/a | 3/0 |
| observation analysis-pair | file red | 3/1 (the 1 is `observationsHold` after the discovery run, line 99: expected true received false; not caused by the loader, not analysed) |
| native directory usage | 14/0 | 14/0 |
| caching reset recovery | 4/2 | 5/0 |
| caching native-build observation | 5/1 | 6/0 |
| caching tools activation | 4/0 | 4/0 |
| native retirement-file | n/a | 1/0 |
| fixed paths / birth / main, publication fixed | n/a | 1/0, 1/0, 7/0, 3/0 |
| process budget timers | 7/1 | 8/0 (also fixed a stale expectation: the timeout message is now `process.execution.timed-out {...}`) |
| recipe spawn | n/a | 1/0 |
| read-plan, read-plan journal, initial | n/a, 0/2, n/a | 9/0, 1/0, 2/0 |
| storage rebudget, conditional retirement | n/a | 4/0, 7/0 |
| contract bootstrap | 1/3 | 3/0 |
| platform child cleanup, directory exchange | n/a | 2/0, 32/0 |
| owned-execution capture counters | n/a | 7/0 |
| source admission memo | 3/1 | 3/1 (the failing test, "source reads ... contextualize genuine read failures", is not mine) |
| lease descriptors, cargo order | n/a | 2/0, 1/0 |
| inputs storage ledger-busy, custody, retirement-window | n/a | 1/0, 1/0, 4/0 |
| inputs publication, boundary | n/a | 1/0, 2/0 |
| dependencies platform, diagnostics, compiler settlement | n/a | 1/0, 2/0, 1/0 |
| nx-plugin probe | n/a | 4/0 |
| retention negative receiver | 0/4 | 1/3 (environment: "Exact private retention workspace required"; the migrated function was proven to load in a standalone probe) |
| cargo execution phase (helper, no tests) | - | parent `execution/🧪️tests` 68/22, all 22 in the Original/native-caller cluster (E57); the function was proven to load, the parent could not prove it |

Two fixes the migration exposed (not planned):
- The loader threw `testing.product-function-required` / `testing.product-import-required`, keys that were not in the process catalogue (`📖️texts/🔣️.json`): every typechecked suite (e.g. native-child-cleanup) failed `repo test-file` with TS2345. Both keys added (en/de). The texts law is red on `capacity.empty-process-table is unused`, a peer's key, not these.
- Process rows grant no `artifacts` capability; files that declare `capabilities:{artifacts:true}` fail with `process.testing.capabilities-differ`. For the two files I migrated (journal, contract bootstrap) the declaration is now `capabilities:{}` and they pass. Still red for that reason, not touched: `native-build/💾️storage/🔒️mapping`, `📥️borrower`, `🧹️cleanup/📥️receiver/📎️existing`. A suite row with `artifacts:true` (like `resource-lease-writer`) is the other fix.

Not migrated because they need product work, not a swap (still on the ceiling): tests that extract a closure inside a function (fixed receipt `checkPhysical`/`recheckPhysical`, contract mount `withExclusiveMaintenance` arrows, the platform failure guard expression), tests that mutate or substitute the extracted text on purpose (inputs consumers, native-dependency-closures oracle from git HEAD), functions with a dynamic `import("...")` (graph writer lifetime/admission) and functions whose ceiling is more than one use. Each needs the closure lifted to a named module-level function (declared through its member port) first.

## 2. Real Bazel test
`B/🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts` **passes here: 1/0, 24 expects, 82 s**, run as `SEMIO_TEST_BAZEL_BINARY=/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel repo test-file <file>` (Bazel 8.4.2, suite build-bazel-snapshots). `/tmp` was not denied in this run: `mkdtemp("/tmp/pathlaw-")` and the hermetic root `/tmp/<12 hex>/r` (the test asserts that shape) worked. So the premise (hermetic base denied) did not reproduce, and I did **not** add the `.test-scratch` base: it would thread a base override through `generateBazelWorkspaceV1` -> `bazelHermeticManifestV1` (manifest `scratch` is `BAZEL_PATH_PLAN_V1.hermeticRoot`, read by the Bun runner) and relax the test's `/tmp` assertion; left as a design item if a host really denies `/tmp`.

## 3. Corpus row `missing-parent`
The row (`📥️install.json` accepted true, children 2) is read only by the dependencies storage main test, `🪜️ancestry` and `🔗️pair`; `producer/*` and `reading/` do not read it.
- `🏭️producer/*` 14 test files: **all 2/0** (🔗️missing, 🔗️subset, 📦️complete, 📋️ready, 💥️publication, 🛠️installer, 🚧️collision, 📴️offline, 🧊️diagnostic, ♻️recovery, 🏺️retained, 🔁️reuse, ❔️diagnostic; the parent `🏭️producer/🟦️.ts` is a helper). No regression from the row.
- `👓️reading/🟦️.ts` was 49/36: every failure was `"hold" is not a declared SelectedPlanProtocol role`, i.e. the dependency generation hold directory (E58) was not in the closed role list. Fixed by declaring `hold` as a partition role in `🧬️schema/📥️install.json` (en/de text). Now **67/18**; the remaining 18 cascade from one vector, `neutral dependency completion actual all-scope barrier preserves partial failures`, mode `preserving-payload-metadata-drift`: the run refuses at `dependencies:ready-write` before binding 1 (expected `bindings:1`, received 0) and then the Original owner stays closed (`OriginalAdmissionClosed ... first failure: preserving-payload-metadata-drift`), failing every later test in the file. The sibling vector `preserving-capsule-drift` passes. Not caused by `missing-parent` (no parent creation involved); cause not found (needs a debug run of that vector; the test is itself a text-extraction offender).

## NOT verified
- analysis-pair 3/1 and source-admission memo 3/1 keep one red test each (unrelated to the loader as far as the traces show, but I did not run the original files to prove the same failure existed before).
- Load/memory: load average stayed 29-55 during the run (gate 40) but every run was a single file; memory free 77-86 %.
- Before-hashes of the 39 edited files are not recorded (in-place edits).
- Bazel `.test-scratch` base not implemented (see 2); the `/tmp` question is open for hosts that deny it.
