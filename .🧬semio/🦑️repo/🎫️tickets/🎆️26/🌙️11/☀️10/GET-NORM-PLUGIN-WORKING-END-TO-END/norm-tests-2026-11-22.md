# Norm Plugin Test Targets, 2026-11-22

## Outcome

No Norm target went green. The first target never reached any Norm code: the resident repo service failed while reading the project graph slice (file inventory of the tree), before planning `@semio-tech/norm-plugin:check`. Every later target was not run, because each attempt took 30 to 78 minutes and the coordinator asked for a wrap-up.

| Target | Outcome |
| --- | --- |
| `@semio-tech/norm-plugin:check` | FAIL (infrastructure, 4 attempts, see below) |
| `@semio-tech/norm-plugin:test` | not run |
| `@semio-tech/norm-js:test` | not run |
| `@semio-tech/norm-js:test-wire-twins` | not run |
| `@semio-tech/s-norm-composition-tests:verify-norm-document-contract` | not run |
| `@semio-tech/s-norm-composition-tests:verify-norm-results-window-ownership` | not run |
| mechanics project test (`show_project` not reached) | not run |

## Attempts of `@semio-tech/norm-plugin:check`

All with language en.

1. 29m33s, exit 1. Error (reference d039b24e): `Error: Inventory record content differs from its name`, at `readInventoryRecordLeanV1` (`build/📥️inputs/🪶️lean-inventory/🟨️.mjs:22`), called from `build/👁️observation/📥️reuse/🟨️.mjs`.
2. 1h18m, exit 1. The slice worker exited 3 times with code 75: `Private Nx engine import missing: ../../🌐️remote-execution/🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts from build/🚢️fleet-status/📥️readers/🟦️.ts`. Cause: the directory `🚦️ready/🩺️workers` did not exist at that moment. Another agent was refactoring the framework build module, and the directory was back about 90 seconds after I looked.
3. 31m, exit 1. Error (reference 21f2bb2d): `Error: Closed inference observation block rows required`, at `readObservationBlockV1` (`build/👁️observation/🟨️.mjs:1354`). That file was modified at 01:48 during my run, so it is another concurrent edit of the framework.
4. 1h02m, exit 1. Same error as attempt 1 (reference 669a9ccd): `Inventory record content differs from its name`, thrown from `inventoryNamesSettledV1` / `missingInventoryNamesV1` in `build/👁️observation/📥️reuse/🟨️.mjs`. The service log is `/var/folders/t7/ckfnmc712gsdkt0tc8042br00000gn/T/semio-repo/501/host-c4b8c0ed734e.log`.

## Root cause

Not a Norm defect. The attempts hit three different failures in the shared repo build framework: a corrupt or inconsistent lean-inventory record, a briefly missing worker directory, and a changing observation module. The graph-slice phase also ran for 30 to 60 minutes (progress counters past 900 of 600 expected seconds) on a tree of about 214k source observations, on a memory-constrained machine with a parallel native build.

The "Inventory record content differs from its name" error occurred twice, in attempts 1 and 4, hours apart. It looks like a persistent damaged inventory record rather than a race. I did not repair it because the inventory is shared state, and the right fix belongs to the owner of the framework inventory module.

## Fixes and changed files

None. No file was changed by me. The only file I wrote is this report.

## Recommended next steps

1. Have the owner of `build/📥️inputs/🪶️lean-inventory` and `build/👁️observation/📥️reuse` identify and repair the inventory record that fails the name check (reference 669a9ccd in the service log).
2. Once the framework sources are stable, rerun the targets one at a time in the order above.
