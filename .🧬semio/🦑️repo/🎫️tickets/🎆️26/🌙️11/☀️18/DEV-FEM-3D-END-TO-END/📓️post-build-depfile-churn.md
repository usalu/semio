# Post-build dep-info churn discarded a green component build

## Evidence

Attempt log `🗑️generated/attempts/semio-hub-fem-component_component-1795064107587.txt`: the report line (712 units, 5495 s) is followed by `cargo:workspace-selection`, then five "Another process edited ...\semio_hub_fem_component.d ... attempt N of 5" announcements (layer `snapshot`), then "Another process kept editing ... (1 of 6)" and exit 75.

## Who reads the `.d`

- `withCargoWorkspaceSelectionV1` (`snapshot` layer) wraps the whole component continuation. Its body ends with `await scope.view.recheck()` for every retained compilation (`component-pass/🟦️.ts`, continuation after the report). That view's recheck is `CargoProvenanceSessionV1.recheckCompletedFiles` (`native-build/🟦️.ts`).
- `recheckCompletedFiles` verifies a claim whose output was relocated against its retained copy, and every other claim by re-hashing the live file. Only Cargo artifact outputs were relocated (`completedCargoOutputClaimsV1`). The uplifted dep-info `<artifact>.d` is in the roster as `retained` (owned by the target root) and was copied, but its claim stayed on the live path.

## Why two cargos touch one lane

`acquireCargoBuildLeaseV1` is a shared (cohort) lease by design: the lane (`p488470`) is one compiler store shared by every build of the profile; Cargo's own unit locks serialize writers. The exclusive part is the target recipe slot (`t-<digest>`), held by `acquireCargoTargetRecipeV1` only until the artifacts are captured (`produce`). The recheck runs later, outside that lease, so another agent's cargo of the same recipe rewrote `semio_hub_fem_component.d` in place in between. Making the lane exclusive per build would serialize every agent for the whole 90 min and is not needed once the build stops reading its own outputs live.

## Fix

`native-build/🧾️receipt/📦️retention/📋️inventory/🟦️.ts`: `completedCargoOutputClaimsV1` now also returns the dep-info rows of every unit that are `retained` (owned by the build or target root). `retainCompletedOutputs` therefore verifies the dep-info bytes against the copy taken under the recipe lease and relocates the claim, so the later `recheckCompletedFiles` no longer reads the live `.d`; a peer rewrite of the lane cannot discard a finished build. Side effect: dep-info is now reclaimable with its outputs when a pass reclaims disk (it was already in the reclaim list but skipped).

## Test

`native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts`: new test "relocates owned dep-info with its deliverable" (owned dep-info and artifact are claims, equal to an independently computed oracle; dep-info outside the funded roots and plain sources stay current). Run with `bun retention-run.ts ./<test file> `: 19 pass, 0 fail; size-law test 1 pass.

## Not done

No full component build was run; the live path (retainCompletedOutputs with a real `.d`) is covered by the existing session tests in the same file only.
