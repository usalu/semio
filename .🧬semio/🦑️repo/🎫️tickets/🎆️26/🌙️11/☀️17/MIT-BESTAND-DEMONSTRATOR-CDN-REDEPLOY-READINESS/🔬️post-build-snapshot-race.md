# Post-build snapshot race (browser-partD, exit 75)

## Symptom
`components describe --facet browser` finished ("5 component(s) staged ... 0 failure(s)", report written), then "Another process edited .../p9c42c7/wasm32-wasip2/release/build/semio-framework-value-paged-clone/<hash>/out/lib...rlib while it was read", 5 snapshot attempts, a whole-command restart, exit 75.

## Cause
- The step is the completed-compilation view recheck: `CargoProvenanceSessionV1.completedSnapshot(...).recheck` -> `recheckCompletedFiles()` in `native-build/🟦️.ts`. It runs after `consume`, once per task (`withNativeBuildCompilationTaskV1`, `withCapturedCargoCompilationTaskV1`) and once more in the `completed()` epilogue.
- `recheckCompletedFiles` re-hashed every claim of the shared custody (10347 files, 4.4 GiB) except outputs relocated into private copies. The out-dir rlib is a dependency input of other units, not a relocated output, so it was re-read live, about two hours after the compile that consumed it.
- The sibling gis describe shares unit-cache lane p9c42c7 and legitimately re-emits that unit (rustc writes in place; the restore path already uses temp + rename). Different bytes => `concurrentEditV1("bytes", ..., "snapshot")` -> 5 retries -> restart (exit 75), although every unit had succeeded and `session.close()` had already verified all inputs once ("Final recheck: passed").

## Fix
- `CurrentPhysicalFilesV1.subset(paths)` (`📁️current/🟦️.ts`): custody over chosen claims sharing the memo.
- `recheckCompletedFiles` distinguishes the sealed state (`state==="closed"`): at close everything is verified once (unchanged); afterwards only (a) retained copies of relocated outputs and (b) original output claims (`originalClaims`) that consumers still read from the target are verified live, plus directory structures. Sealed inputs are never re-read, which also removes the 4.4 GiB re-hash per task.
- Unit-cache publication needed no change: `restoreUnitV1`/`publishUnitV1` already write aside and rename, objects are content-addressed (immutable). Only rustc's own in-place output of a recompiled unit remains, which is now irrelevant to a sealed build.

## Test
`📁️current/🧪️tests/🧊️sealed-subset/🟦️.ts`: real files claimed through `CurrentPhysicalOwnerV1`, digest cross-checked with Python `hashlib`; a peer rewrite after sealing fails the full recheck, passes the sealed subset, and still fails when the rewritten file is a live output claim; plus a structure check that the session uses the sealed branch. Run via `repo tools exec bun -- test`: 2 pass. Memo suite unaffected (5 pass).

## Rosters (follow-up)
- `CurrentPhysicalOwnerV1.recheckStructure(keep?)` and the shared custody `structures(keep?)` take a filter over directory and absent paths.
- After sealing, `recheckCompletedFiles` keeps only directories that contain an original output claim (`originalClaims`); rosters and absent-path checks of sealed inputs (e.g. shared-lane build-script `out/` dirs) are skipped. At close everything is still checked once.
- Test extended: a peer adding a file to a sealed input directory passes the filtered recheck, to a live output directory fails ("roster changed"); the unfiltered recheck fails for both. Result: 4 pass.
