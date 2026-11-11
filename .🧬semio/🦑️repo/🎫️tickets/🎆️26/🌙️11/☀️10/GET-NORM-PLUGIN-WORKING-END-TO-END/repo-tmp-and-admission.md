# Repo temp-dir leaks and capacity admission

## Root causes
- Temp leaks: ~1000 test files and a few tools used `os.tmpdir()` (RAM tmpfs on this host). Test runs started with a bare `bun test` (no repo runner) never got the sandbox TMPDIR, and any killed or timed-out process skipped its `afterAll` cleanup. The slice pool spooled its result (up to hundreds of MB) into `/tmp/semio-slice-*` and leaked it if spawn threw.
- Swap admission: reason `swap` fired when swap use was above 25 percent and the swap-out rate was unknown or above 0 (any trickle). Cold zram pages therefore blocked runs. pswpin was never read.
- Grants: the door took the full execution grant before the run asked the graph service for its plan, so runs held 4-6 GB while waiting on graph derivation and starved the graph worker's admission (priority inversion).

## Fixes
- Scratch home: new `repo scratch` module (`🧰️framework/🔨️modules/🏃️process/🧹️scratch/`, protocol `🔣️.json`): `.cache/scratch/<pid>-<prefix>-<random>`, `withScratchDirectoryV1` (removed on completion, error, cancel), `isolateProcessScratchV1` (private TMPDIR/TMP/TEMP per process, removed on exit or signal). A bun test preload (`🧪️testing/🟨️bun/🧹️scratch/📜️script.ts`) is registered in `bunfig.toml`, so every test process and its children use disk scratch. The slice pool now uses it.
- Cache ledger: new area `scratch` (native tree, 512 MiB, pid custody, 1 h unused age, 10 min quiet) so `repo cache gc` sweeps dead-owner entries and never live ones. The ledger now allows pid unit patterns on trees. Native root budget 256 -> 768 MiB; machine default total, weights, README and build-guide updated to match.
- Swap: Linux counter is now pswpout+pswpin (macOS Swapouts+Swapins), so the sample's swap rate is swap traffic. New policy field `swap.activeMegabytesPerSecond` (1): `swap` needs occupancy above the ceiling AND traffic at that rate; unknown rate no longer blocks. Schema, policy, fixtures, SQLite and Python oracles updated; hand cases added.
- Grant after planning: for runs planned by the host service the door takes a small quiet grant; `admitPlannedBuildCapacityV1` (called in `RepoScript.run` right before execution) grows it to the full request. `maximumQuiet` 4 -> 16 so many planning runs can wait at once.

## Tests (managed Bun, direct; logs in 🗑️generated/)
host-capacity (framework + library, 17 files): all pass. New `🛰️planned-grant` (3). Ledger suites (5 files) and cache-gc units pass. Scratch module (11) and cache-gc sweep (2, Python oracle) pass. Resident cancel (31) passes with PATH set.

## Not done
- The test sandbox (`/tmp/st`) still lives in the OS temp dir; it has its own reclaim and tight laws.
- Other runtime `tmpdir()` users (jco install, devcontainer lifecycle, fresh-component) are covered only by the preload in test runs.
- No Python oracle for planned-grant; the PSI warn threshold (10 percent) is unchanged.
- README admission text still says "swapping out below 64 MB/s" without the new active-traffic rule.
