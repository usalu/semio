# result-xplat3-g (executor xplat3-g, wave 1026g): findings of audit-xplat2-1026g.md

Nothing of the live tree was edited, no git command, no ticket tool, no sub-agent, no official build or test run (single-file graph-free runs only, `store-g-tools/ft.sh` on an overlay). The batch is STAGED: `C/xplat3-g-tools/staged/files/` (19 files, every one a full copy of the live file at staging with the edit; `bases.tsv` = sha256 of the live file at staging). Tools: `python3 -I C/xplat3-g-tools/apply.py check|apply|revert [--root DIR]` (apply refuses a drifted file, keeps the originals in `C/xplat3-g-tools/backup/` for revert; checked on a scratch root: apply 19, revert 19), `overlay.py <dir>` builds a symlink overlay of the repository with the staged files for single-file runs. At 19:03 `check` says 19 ready, 0 drift.

## Findings and what happened

| finding | state | change / reason | law |
|---|---|---|---|
| F1 witness test writes to `/tmp` | FIXED (staged) | both paths now under the test's own `tmpdir` base, cleanup is the base removal | witness tests 6/0 |
| F2 Node native provider: no Windows test | PARTLY (staged) | pure win32 plan/compile/publish law (account name of 40 chars, both msvc targets, `.exe` compiler, no link arguments, 256-char limit refusal) runs everywhere; a real Windows law (`skipIf` not win32) builds the core, loads it under `node.exe` and checks all 7 primitives, and fails (not skips) on a CI Windows host without rustc. NOT run on Windows | addon tests 15/0, native tests 11 pass + 1 skipped (the Windows law) |
| F3 Windows suite rows advisory in CI | OPEN, owner decision | needs `platforms` tags on the POSIX-only suites first (ci-g report lists them); flipping `advisory: false` before that turns the workflow red | none |
| F4 renames over open files (observation/inputs) | FIXED (staged) | `renameOverBusyFileSyncV1` in the churn module (the `read` layer's declared pauses, last busy error thrown, other errors at once); used at the 4 sites (`👁️observation` x2, `📥️inputs` x2) | busy tests 8/0 (3 new: retry then land, exhaustion reports once, real rename) |
| F5 `Global\` mutex in the PowerShell installer | FIXED (staged), not run on Windows | `New-InstallMutex`: `Global\` first (cross-session exclusion kept), `Local\` when the account may not create it (UnauthorizedAccessException) | install-contract 7/0 (text law pins both names and the order) |
| F6 symlinks and `libc.so.6` in the runtime tests | FIXED (staged) | one symlink probe (`symlinks`), the symlink cases run only where links can be made, a CI host must be able to (law), the FFI probe picks `libSystem`/`ucrtbase`/`libc.so.6` per platform | runtime tests 32 pass / 2 fail: the same 2 fail in the live tree (31/2 before: "no production source names a runtime outside the Bun adapter", and its Node re-run), unrelated |
| F7 statfs failure counted as "healthy" | FIXED (staged) for the two named sites | `leaseStoreFreeRatioV1` answers `undefined` when unmeasurable; unknown counts as nothing free (pressure sweep), the sweep report carries `null` and says so in EN/DE (`pressureUnmeasured`, `? % free`), never a made-up percentage; dedupe `free()` answers `undefined`, the gain is `null` (JSON) and the summary says it is unknown (`dedupe.freeUnknown`, `dedupe.summary.unmeasured`). The other ~23 `statfsSync` sites were not touched (each needs its own decision) | sweep tests 10/0, admission 17/0, dedupe 9/0 (2 new) |
| F8 read oracle macOS-only in practice | ALREADY DONE in the live tree | report and CLI line carry `backend` and `strength` (`kernel-enforced` / `output-equivalence`); the landlock run in the devcontainer needs a Docker host | none |
| F9 `sh`/`ps` in the native parity test | PARTLY (staged) | the case-folding probe is `existsSync`; the `sh -c` corpus runner and `ps -o pgid=` stay POSIX-only behind `available` (they need the shell pid and the group id) | native tests 11/0 |
| 11 SIGTERM on Windows | ALREADY DONE in the live tree | `terminationSignalsV1` (SIGINT, SIGTERM, SIGBREAK, SIGHUP on win32, documented) | existing |
| 12 dedupe silent without clones | FIXED (staged) | `clones:false` in the outcome and in the JSON row, so zero replacements are explained (message existed); see F7 for the free-space half | dedupe tests |
| 13 POSIX hooks need `sh` on Windows | ALREADY DONE in the live tree | `resolveHookShellV1` / `reportHookShellV1` | existing |
| 14 devcontainer exchange unverified | OPEN | needs one live `repo` native-store check inside a Docker Desktop devcontainer | none |

## Files (19, all staged under `C/xplat3-g-tools/staged/files/`)
churn module (`renameOverBusyFileSyncV1`) + its busy test; observation and inputs `.mjs`; `.ps1` installer + install-contract test; witness test; runtime test; dedupe module + test, cache-gc command + ledger texts; leases facade, sweep module + test + schema texts, admission test; node native tests (2).

## Verification
Overlay single-file runs (official envelope, graph-free): busy 8/0, sweep 10/0, admission 17/0, witness 6/0, dedupe 9/0, install-contract 7/0, addon 15/0, native 11/0 (+1 skipped), runtime 32/2 (same 2 as the live baseline). No tsc run (graph-free; types are checked by bun's transpile only). Order: independent of the other staged batches except two shared files that other lanes may touch (the leases facade and the churn module): `apply.py check` names any drift.

## Needs one Windows run
F2 real-load law, F5 installer from a standard account, F4 rename retry against a real open handle.
