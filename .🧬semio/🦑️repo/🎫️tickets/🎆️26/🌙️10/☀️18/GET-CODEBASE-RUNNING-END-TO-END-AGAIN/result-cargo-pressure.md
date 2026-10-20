# Result cargo-pressure (executor cargo-pressure, 2026-10-20)

No edit under `📚️library/`, no package.json, no root script, no taxonomy. Everything is staged in `C/cargo-pressure-tools/` (`apply.sh`, `apply.py`, `staged/`, `bench.ts`, `mint.ts`). `C/🗑️generated/cargo-pressure-*` was deleted (mirrors, logs, three leaked test DMGs detached).

## 1. "Original compiler pressure owner required"

### Root cause (reproduced on a real image, before and after)
- `recoverCompactCompilerHeadroomV1` is only entered when physical headroom on the Rust image is below the compiler charge (`maximumWorkspaceBytes`, 4 GiB): `withCompilerInitialHeadroomV1` calls `recover()` only when `available < requiredBytes`. It is therefore a free-space event, not a package-count event. Large check runs fill the image, which is why only the 36-package run met it and smaller groups did not.
- HEAD checked `original.control.signal!==control.signal`. `control` is the `reservationControl` of `withRepositoryCompilerStoreOwnedV1`, which descends from `withRequestedIssuedResourceControlV1(control,buildPreparationBudgetMs(),…)` in `withRepositoryCompilerStoreV1`. A budgeted request mints its own signal, so the two signals can never be equal; the first recovery attempt always threw.
- The callee must keep receiving the reservation control (`requireNativeStorageReservationV1` demands `originalReservations.get(storage)===control.signal`), so the correct invariant is invocation identity, not signal identity.
- The live working tree already contains that fix (`requireIssuedResourceControlInvocationV1(control,original)` replaces the signal comparison; file mtime Oct 19 22:30, together with a mocked birth-law test). value-fix-6.log (22:30:10) is the last run that saw the old text; value-fix-8 (22:40) no longer hits it. The birth-law test mocks `requireCompilerWriterCustodyV1`/`requireNativeStorageReservationV1`, so the real chain was untested. I covered it.

### Staged regression test (`staged/pressure.test.ts` + `pressure.json`, target `…/📏️capacity/🧪️tests/♻️pressure/`)
Real 32 MiB image outside the diagnostics mount, real `acquireNativeStorageV1`, control chain `scriptCompilerAdmissionControlV1(560 s) -> withRequested(500 s) -> withRequested(0)` (the production shape), two rounds of `withCompilerWorkspaceWriteV1` each forced into recovery by a 12 MiB inactive unit.
- Before (mirror with `git show HEAD:` recovery file): write 0 throws `Original compiler pressure owner required` after 19 s (identical to the production message).
- After (live file): passes in 46 s (3 of 4 runs). One run stalled to the 500 s preparation deadline (`Original process admission closed`) with no error text; I attribute it to peer lease contention on the shared diagnostics partition (the recovery opens it) but did not prove it.
- Side observation: after a failed write in this test the cleanup (`retire`, `image.close`) hangs until the control deadline and leaks the mounted DMG. Not investigated.

## 2. Why source admission needs > 590 s

### Measurements (this machine, load 55 to 220, `cargo-pressure-tools/bench.ts`)
| operation | median | note |
|---|---|---|
| `storage.recheck()` | 1340 ms (max 3485) at load ~150; 415-540 ms at load ~55 | a durable-journal transaction behind `port.withOperation` (cross-process exclusive lease), see `heldReservationV1` |
| `readPhysicalCargoSourceV1` (8 KB / 67 KB) | 98-217 ms / 149-335 ms | 6 rechecks of ~16 ancestors, plus a byte-replay |
| `rustc --print cfg` | 290-430 ms | one spawn per unit |
| `control.publish + yieldContinuation` | ~0 ms | not a factor |

### Where the time goes (value-fix-7/8 logs)
- Run 8 (6 requested packages) admitted 192 workspace units (150 distinct packages, `--tests` pulls dev-dependencies) and 2,059 reads in about 27 minutes before the alarm: 0.78 s per read. Run 7 reached 305 reads in 590 s (1.9 s per read, unit 27 of 43).
- `actualCargoSourceAdmissionPortV1` called `storage.recheck()` before every read AND after it (+1 per cfg): about 4,300 durable-journal transactions per run, each queued behind the 13 peer `bun repo` processes on the same lease. That is the dominant cost, not the reads.
- Per-package re-walks are NOT the problem: only 21 of 173 units repeat a package and those hold 303 of 2,059 reads (15%). The real redundancy is across invocations (the whole 150-package closure is re-admitted on every `bun repo cargo`) and the per-read rechecks.

### Staged fix (anchored patches, `apply.sh`)
1. `🎟️admission/🟦️.ts`: new optional port method `recheck?(control, phase)`; `admitCargoSourceBoundariesV1` calls it once per unit (after the cfg progress event) and once after the last unit. `actualCargoSourceAdmissionPortV1` no longer rechecks in `cfg`/`read`; `recheck` goes through the new exported `storageRecheckGateV1(interval, now, recheck)`: first call and `"complete"` always recheck, otherwise at most one per interval (`storageRecheck.intervalMilliseconds = 15000`, new constant in `🧬️schema/🔣️.json`). The admitted Source is read outside the store, so unit-boundary custody is enough; a failed recheck still aborts the run.
2. Same file: `rustc --print cfg` memoised per exact argument vector inside the port (failed promises are evicted).
3. `📖️read/🟦️.ts`: the two loop-entry `recheck()` calls that directly follow another recheck are skipped on the first iteration (6 -> 4 identity checks per file; later chunks keep per-chunk checks; mutation test still catches the change after the first continuation).
4. `🧪️tests/🥒️.feature`: two scenarios. New tests `🧪️tests/🔁️recheck/` (+ JSON corpus): event order of recheck/cfg/read per unit vs a SQLite recursive-CTE oracle; gate vs an independent SQLite throttle oracle; failed recheck is retried.

### Effect (measured, bench with real storage, 12-read unit, load ~55)
Old port: 12.3 s per unit (1 cfg + 12 reads); new port with 2 rechecks (worst case, no throttling effect): 3.0 s (4.1x); the second identical cfg 1195 ms -> 0 ms. Extrapolation for the 192-unit run: about 39 min -> about 10 min before the gate and cfg memo help further. NOT measured: a full 36-package admission (needs a cargo window).

### Proposed, not staged (post-freeze design work)
- Persistent admission memo: key = unit identity + cfg vector + toolchain; value = `inputs`, environment references and every probed path with its `lstat` stamp (dev, ino, size, mtimeNs, ctimeNs, nlink; ENOENT probes included); a hit validates stamps only and replays `readRustSourceClosureV1` from the memo. Removes the repeated 150-package walk across invocations. Needs a storage location under the compiler store and the gate-limit semantics reviewed.
- Admit units concurrently (4 to 8) once rechecks are gone; cross-unit read cache would save only about 15%.

## 3. Verification
- `python3 apply.py check`: all anchors found exactly once; `apply.sh apply --dry-run`: refuses while `bun repo dependencies sync --all` runs (pid 38018); `--ignore-sync --dry-run` lists the 4 patches and 4 new files.
- Mirror overlay (symlink tree + patched real files, `apply.py mirror DIR --copy <tests dirs>`):
  - `bun test …/🎟️admission/🧪️tests/🔁️recheck/🟦️.ts`: 3 pass.
  - `bun test …/🎟️admission/🧪️tests/🟦️.ts`: 4 pass, 1 fail (pre-existing, identical on the real tree: `progress vectors … oracle`, "files 9/8", the fixture graph unit boundary is 8 files).
  - `bun test …/🎟️admission/📖️read/🧪️tests/🟦️.ts`: 2 pass.
  - `tsc --noEmit --strict` over patched admission + recheck test: 0 errors; `bun build --no-bundle` OK.
  - `bun test …/📏️capacity/🧪️tests/♻️pressure/🟦️.ts`: before = fail with the production error, after = pass (see section 1).
- The existing `📏️capacity/🧪️tests/🟦️.ts` recovery law fails on the live tree with `Nested persistent image requires actual parent scope` (its image directory is inside the diagnostics mount); pre-existing and unrelated. Fix: create that image outside `diagnostics.root`, as `♻️pressure` does.
- Not run: full 36-package `bun repo cargo`, `crate-boundaries`/taxonomy laws for the new test directories `🔁️recheck` and `♻️pressure` (names not checked against `🔣️taxonomy.json` fileKinds), `bun repo --help` after apply.

## 4. Apply (after the freeze lifts)
`sh C/cargo-pressure-tools/apply.sh apply` (refuses when a target changed since staging, `--rebase` re-anchors; refuses while a sync runs), then run the three admission/read test files, `♻️pressure` once, and the taxonomy law. If `apply.sh` reports a changed base, rerun `python3 apply.py bases` only after reading the diff.
