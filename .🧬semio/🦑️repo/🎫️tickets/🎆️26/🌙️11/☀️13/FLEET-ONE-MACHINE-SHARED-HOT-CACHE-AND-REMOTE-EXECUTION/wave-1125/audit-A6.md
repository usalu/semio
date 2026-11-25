# Wave 1125 Audit A6: consolidated red tests, open items, user actions

Read-only consolidation at 2026-11-25 ~18:00 CET. Inputs: `result-F1..F27` (no F24, F28, F29, F30 result files yet), `audit-A1..A5`, `coordination.md` (last entry 17:56), `brief.md`, `claims-tests.md`, and the Wave 1125 sections of `../landings.md`. No test was run, no code edited, no git command, no process touched.

Abbreviations: `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `B` = `L/🏗️build/`. Paths are shortened to the last segments that the result files record; the full path is `L/...` where the result gives it relative to `L`.

Lanes still active at 17:56 per coordination: F6 (graph seal watcher), F24 (dwg snapshot split, staged), F28 (budget/headroom/admission test drift), F29 (421 guard part 2 and Go launcher suite), F30 (whole-read throughput). F20 retired at 17:56. All other F-lanes are DONE or retired.

Last recorded fleet state: Ready 1/14 (g1-m12, coordination 17:33; 2/14 at 17:24, lost again at 17:33).

---

## 1. Test files still red or partially red

Counts are the last recorded `pass/fail` (or timeouts). "Reported by" is the lane that measured it last.

| # | Test file (relative to `B` unless noted) | Last counts | Reported by | Suspected cause | Owner |
|---|---|---|---|---|---|
| 1 | `L/…/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts` ("the nx partition provisions its own backing and charges the artifact envelope") | 21/2 | F6 landing, A5 (17:5x) | "Actual Native platform image required": peer's open mount-contract edit (F6 says so); not caused by F6 | F28 (budget test drift, coordination 17:39) |
| 2 | budget `🕸️graph/📏️headroom/🧪️tests/🟦️.ts` | 19/8 (F26); earlier 22/5 (F22) | F22, F26 | five custody/Authorizer failures unrelated to the partition change; `retiredSpill` and unbound peer helpers | F28 (headroom reds added 17:53) |
| 3 | `…/📏️headroom/🧾️closures/🧪️tests/🟦️.ts` | timeout at 590 s (F7) | F7 | load-bound (not re-run since); cause unknown | F28 (headroom reds) |
| 4 | `…/📏️headroom/🧪️tests/🟦️.ts` | timeout at 590 s (F7) | F7 | load-bound; cause unknown | F28 (headroom reds) |
| 5 | `…/📏️headroom/🧾️closures/♻️groups` | 2/11 (F26); earlier 7/6 (F7) | F7, F26 | "Closure reference directory changed … m/tmp"; drift after F26 landing | F28 |
| 6 | `…/📏️headroom/🧾️closures/📏️cardinality` and `📁️census` | red | F26 | "retiredSpill is not defined": extracted `directories` without the new binding (F26 harness drift) | F28 |
| 7 | `…/🔀️selected-state-recovery/🟦️.ts` | 0/2 | F22 | peer's unbound `refreshStaleImageBeforeOpenV1` | F28 (claimed 17:45) |
| 8 | `…/🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped/⚠️scoped-retirement-and-bridge` | 2/1 | F22 | peer's unbound `requireRepositoryPartitionFitsHostV1` | F28 (claimed 17:45) |
| 9 | `…/📁️inventory` (load) | 1/9 | F27 (earlier 10/0 in F19) | "Canonical workspace and declared cache partition required", from a peer's cache-partition policy change | UNOWNED |
| 10 | step-overdue (`🧪️tests/🪜️step-overdue`) | 18/4 | F25 | inventory step semantics ("seconds since progress") and a 16384-byte tailBytes fixture; F25 says pre-existing | UNOWNED |
| 11 | `…/🏪️store/🧪️tests/🟦️.ts` | 3/6 | F7 | spawned child prints only the bun banner (child filter/entry not run). The `📦️child` sub-suite is green (9/0), so the root file is the red one | UNOWNED |
| 12 | `…/🌐️vite/💾️storage/🧪️tests/🟦️.ts` | 8/1 | F7 | "Finite Original physical cleanup reserve required before effects" | UNOWNED |
| 13 | `…/📤️publication/⚓️anchor` | 0/1 | F7 | needs a case-sensitive volume (environment) | UNOWNED (environment) |
| 14 | `…/📤️publication/🏗️native-build/🟦️.ts` | 4/3 | F18 | source-extraction harness drift, `mac:absent` | UNOWNED |
| 15 | `…/📬️test-body/🟦️.ts` | 0/9 | F18 | "Live Original process issuance required": extracted-function harness drift | UNOWNED |
| 16 | `…/🧵️prepared-worker/🟦️.ts` (+ its `📜️script.ts`) | 1/2 (script 0/1) | F7, F18 | "Original required" cluster; 1 ms deadline rounding between receipt and wire | UNOWNED |
| 17 | `…/📡️session/🟦️.ts` | 4/1 | F7 | worker cancellation | UNOWNED |
| 18 | `…/🔏️admission/admitted-worker-harness` | 1/2 | F7 | not diagnosed | UNOWNED |
| 19 | `…/🔒️bounded/🟦️.ts` | 6/1 | F18 | value drift (16 GiB vs 48 GiB) | UNOWNED |
| 20 | `…/📐️phases/🟦️.ts` | 8/1 | F18 | stage order drift | UNOWNED |
| 21 | `…/📬️script` child programs (asset-publication, graph-purpose, native-dependencies purpose, nx purpose, cargo child, `♻️roles.ts`) | "false red" via `repo test-file` | F18 | child programs run only by `repo test-file`; discovery never runs them; `routeTestFilesV1` refusal proposed, not landed | UNOWNED |
| 22 | `🛠️service` (`🗺️topology/🧪️tests/🛠️service/🟦️.ts`) | 13/2 | F8, F10, F17, A3 | the 2 real-launchd macOS tests time out at 90 s under load 35-44 | UNOWNED |
| 23 | replica resilience test file (path not recorded in F8; the store publish/prune tests) | 25 pass, 1 fail | F8 | "cooperative yield" is a load-sensitive timing test | UNOWNED (low) |
| 24 | Go launcher suite `repo run @semio-tech/repo-launcher-go:test` | exit 1, package `launcher` FAIL | A5 (17:54) | `TestLauncherReadsItsStateBelowTheDeclaredDirectory` (zz_semio test line 87). The workspace package has no `ok` line, so its hidden-console test is unconfirmed | F29 (coordination 17:54) |
| 25 | Go launcher suite, earlier runs | not run / blocked | F17, F16 | resident graph service `BUILD_SOURCE_UNPARSEABLE`; F16 saw a 20-min hang | F29 |
| 26 | F7 red C sample (18 files re-measured: 4 green, 12 red, 2 timeouts) | see F7 list | F7 | see rows 11-21 | see rows |
| 27 | F7 not re-measured: 93 C files and all of B (17 red per E67) | not measured | F7 | runner is resumable: `python3 T/🧰️scripts/F7/run.py T/🧰️scripts/F7/red-C.txt c1.jsonl 520` (T = ticket folder) | UNOWNED |

Resolved since the last red report (verified by its lane, listed so nobody re-staffs them): `📏️capacity` 11/2 -> 13/0 and `📋️owner-cmd-policy` 5/6 -> 11/0 (F18); `📦️owner` 10/2 -> 11/0 (F18); `📥️release` and `📥️persistent` (F21); `🏋️unit-admission` 10/2 -> 12/0 (F13 r3); `🧪️tests/🟦️.ts` ready 58/0 (F1 r4); `♻️collection` 19/19 (F4 r2); `📥️opening/🚦️admission` 6/0 (F22); `🪪️observation/🧪️tests/🟦️.ts` 69/0 (F20 r2, A5's 62/7 not reproduced); `🛡️warden` 46-49/0 (F9, A3); `remote-cache/🖥️server/🧪️tests/🧬️instances` 2/0 (F29); `🕸️graph/…/🧠️admission-memo` 14/0 (F19).

---

## 2. Open, not verified, or not done items (deduplicated, sorted by impact on "all workers Ready")

Owner is the lane named in coordination.md, else UNOWNED. "Retired" means the lane wrote its result and stopped.

| Rank | Item | Evidence | Owner |
|---|---|---|---|
| 1 | Graph not sealed: `current.json` gen 260 is a quarantine pointer (closure-missing). Seal pending; then `repo run repo-mcp:build` and `repo-mcp:install`. Every worker's workspace stage and Nx hashing depend on it. Whole read at 287/5093 owners at 17:55, about 2 per minute (about 40 h) | F6 r1/r2, coordination 17:55 | F6 (seal watcher), F30 (throughput, launched 17:55) |
| 2 | Post-fix workspace behaviour not proven on any worker. F1 rounds 1-4, F22, F23, F27, F21 and F12 all landed but no worker has completed a workspace stage since 17:33; Ready 1/14 | F1 r3/r4, F22, F23, F27, coordination 17:33 | UNOWNED (F1, F22, F23, F27 retired) |
| 3 | Workspace stage Nx run-many exits 1 on 9 workers ("Owned Nx invocation failed (130)"; `repo-mcp:build:prototype`, `norm-din4108-rs:build`). Root cause not found. F21 only surfaces the failed-task tails; F2's native lease fix is a different class | A2 problem 1, F2, F21, F5 | UNOWNED |
| 4 | Native dependency without build project owner: `semio-s-artifact-norm-en1993-scene-schema-grips` blocks pool on m01 (3 failed) and g1-server (4 failed, 1 running). Pool claims 729 units per origin | A2 problem 2, pool.json lastError | UNOWNED |
| 5 | g1-m05 (Windows) dependencies: attempt 46; a changed binding costs 9-14 s; 1,300-1,500 bindings needed; each run replays from binding 1. F14 only made unchanged bindings cheap (0.05 ms). The per-changed-binding cost is unexplained (native counters under 1 s per lap) | F4 (b), F14, A2 problem 4 | UNOWNED (F14 retired); USER for profiling (section 3) |
| 6 | g1-m04 (Linux) dependencies never started. Inode gate: about 122k free vs derived cold need 428,796 (declared 300k). Stale unfinished generations were not retired by F4 r4 (cause of inode pressure not in cache collectors). F3's EACCES fix is unexercised. Free inodes fell slowly (123,043 at 17:05, 122,462 at 17:20) | F3, F4 r2-r4 | UNOWNED (code side; F3, F4 retired); USER for measurement (section 3) |
| 7 | Stage-failed refresh clears Ready on every worker at once (a deterministic environment-wide failure). Design question: keep Ready for that class too, or not | F5 r2 | UNOWNED (coordinator decision) |
| 8 | m13 `native-bootstrap.done` marker not written: waits on workspace only. F8 says no human run is needed. Depends on rows 2-3. ticket PROVISION-MAC-WORKER-G1-M13 still open | F8, A1 #10-11 | UNOWNED (follows rows 2-3) |
| 9 | m12 workspace refusal "Exact live repository partition cache owner required": F22 made the four causes typed but the m12 cause is not proven (failed or closed owner most likely). Verify on the next m12 failure log | F22, F12 | UNOWNED (F22 retired) |
| 10 | m02 stale-facts exit 75 (three identical causes): F23 targeted refresh and F27 tombstones landed; the persisted layer that kept the same row for three fresh workers was not reproduced live | F23, F27, coordination 17:24 | UNOWNED (F23, F27 retired) |
| 11 | g1-m09 never provisioned. `S/origins/g1-m09/` holds only `fleet-auth.json`. Nothing on the coordinator side can start it | A1 #9, F4 (c) | USER (section 3) |
| 12 | Coordinator host disk: 827 of 926 GB used; graph image 18 GiB, full earlier, at 8.2 GiB free after F6 r2. The in-place `hdiutil resize` and the elastic growth to 24/32 GiB were never exercised live (the image was never idle). Three test-sandbox images are mounted under `.cache/test-sandbox/rJTEOQy` | F6 r1/r2 | USER (free disk) + F6 (verify growth) |
| 13 | g1-server status: registered executor with concurrency 10 and headroom True at load 37 on 10 cores. F13 r2 added a CPU ceiling (1.5 per core) and 2 reserved slots in code; not verified live. Toolchain is hand-provisioned and not checked by the loop; `unit-cache-read-only` on g1-server (5053 units compiled without sharing) | A4, F13 r2, A2 problem 10 | UNOWNED (decision in section 3) |
| 14 | Doctor verdict `blocked` at 16:31 (graph image 196 MB free, load 40.7). Not re-run since F6 r2 | A2 doctor-A2.json | UNOWNED |
| 15 | Beyond-host queue: the 120 s queue timeout for cad/puzzle finals and the 60 min beyond-host wait are not measured live. A beyond-host unit blocks a compile slot for up to an hour while the coordinator is saturated | F13 r1-r3 | UNOWNED (decision) |
| 16 | Landings in build/graph/ready code are blocked until 19:00 (quiet window 18:15-19:00). Staged product work waiting: F24 dwg_snapshot_objects split (4,894 MB library; closure-wide carve), F28, F29 | coordination 17:51-17:55 | F24, F28, F29 |
| 17 | Slice read cost: 7 min before the slice (graph service boot 25-111 s, source capture walk about 11 ms per file, unlabelled pre-inventory window now labelled by F25), round restarts from replica churn, persisted verdict memo and path-journal re-verification not landed | F5, F12, F25, F20 task 1 | F30 (throughput); rest UNOWNED |
| 18 | Replica health: publish p95 up to 183 s, failing sync rounds 9-29, leftover `*.tmp` in `.cache/f/boot` and store origin. Every build-system landing restarts 14 supervisors, which blocks convergence | A1 #13, coordination 17:51 | UNOWNED |
| 19 | Cache spool and uploads failing on m01: `spool-full` 27,743, `upload-failed` 14,684. Token `SEMIO_REMOTE_CACHE_TOKEN` absent in the audit shell (verify on workers) | A2 problem 9, A2 remote cache | UNOWNED |
| 20 | Remote cache: Nx hit rate 0.2 % (no lane; F16 recommends coarser granularity). AC divergences still 107-194 per day for unit before the F11/F16 fixes; their live effect is unmeasured. Namespace `semio` writers on m06 and m07 still present at 17:26 (F16 asked a re-measure after 30 min) | F11, F16 | UNOWNED |
| 21 | F5 stall fix (`readingPatterns`) unmeasured: needs an hour of workspace outcomes after 16:40 | F5 r2 | UNOWNED |
| 22 | Process-level leftovers: idle cargo holder pid 96461 (5521 s at 0.3 % CPU, flagged by doctor). Not killed by any audit | A2 problem 6 | USER (section 3) |
| 23 | Staged index entries: 2,738 entries (`AD -inf` and `replica/sa` strays) await the archivist commit. The `-inf` creator (a command printing `[capacity]` output) is unidentified | F8 #3, A1 #12 | UNOWNED (archivist) |
| 24 | Landing records and claims with errors: F1 record-machine and enum hashes swapped; F4 `disk-recovery` `.py` hash does not match the landed one and the `.ts` changed after its landing; F4 r2 dated 17:10 (ahead of the audit time); claims-tests.md gives the wrong hidden-spawn test path | A3 table | UNOWNED (documentation) |
| 25 | Warden: bypass runs whose target dir and sources are outside the workspace are invisible; old ticket scripts still bypass admission; live warden restart on the new digest unchecked | F9 | UNOWNED |
| 26 | Hidden spawns outside `🧰️framework`: 64 calls (generators, hub bootstrap, teaching, tests). Not fleet processes | F17 | UNOWNED (low) |
| 27 | Go: workspace hidden-console test not run; Windows `hideChild` and `hideConsoleWindow` only vetted with `GOOS=windows go vet` | F17 | F29 |
| 28 | Puzzle final charge not measured; sourcing 5,519 MB and binary-io registry 199 MB not analysed; cad charge 8,036 MB still over the 4,096 MB ceiling (only dwg split is in progress) | F13, F15 | F24 (dwg only); rest UNOWNED |
| 29 | `closures/🟦️.ts` is 65,281 B against the 64 KiB pin (65,536). Any growth breaks the implementation pin | F26 | UNOWNED |
| 30 | Native image and library opening copies still have the 32-operation bound (F22 changed only the partition opening) | F22 | UNOWNED |
| 31 | Loaded processes keep the old graph closure code ("Stale loaded Graph closure implementation; restart required") until they restart. Any worker that has not restarted since F26 still refuses its own closure proof | F26 | UNOWNED (resolves on restart) |
| 32 | Wave-1122 leftovers outside this wave: E83 history fixture, E29 farm-child and late-refs owner isolation, layout target admits about 1,054 owners vs 74 declared, Nx orphan funding reservation, history/playground private gaps | A1 #17-20 | UNOWNED |
| 33 | Stale m05 manual replica restart (`findings-E6.md`) is no longer needed: m05 is in sync (A1). Closed, listed so nobody re-runs it | A1 closed items | none |

Notes on the ranking: rows 1-3 gate every worker; rows 4-6 each block one worker class; rows 7-10 decide whether a failure can still flip Ready; the rest matter for convergence, cost, and hygiene.

---

## 3. User actions requested (machine, exact command or check)

1. **g1-m04 (Linux, user `deck`), one privileged step, after its replica has applied the code.** The daemon logs `service definition differs ... service-bootstrap` once. Read first, then run (source: `result-E64.md`, "Needs user"):
   ```
   cd /home/deck/semio && repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/📜️script.ts" service-bootstrap --group g1 --user deck | less
   cd /home/deck/semio && repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/📜️script.ts" service-bootstrap --group g1 --user deck | sudo sh
   ```
   Idempotent; a few seconds of replica restart.

2. **g1-m04, inode measurement (read-only).** Needed to decide row 6:
   ```
   df -i /home/deck/semio/.cache
   sudo du --inodes -x -d3 /home/deck/semio 2>/dev/null | sort -n | tail -20
   ls /home/deck/semio/.cache/b/d/m/j
   ```
   If the inode table is small (chosen at mkfs), the clean fix is a dedicated volume for the store with a high inode count, for example `sudo mkfs.ext4 -N 40000000 /dev/<spare-partition>` mounted at `/home/deck/semio/.cache`. This destroys the partition's data. It is a user decision, not an agent action.

3. **g1-m04, only if the next dependency attempt still shows EACCES on a patch file:**
   ```
   chmod -R u+rwX /home/deck/semio/.cache/b/d/m/bun-cache
   ls -ld /home/deck/semio/.cache/b/d/m /home/deck/semio/.cache/b/d/m/bun-cache
   ```
   Owner must be `deck`.

4. **g1-m09 (a Mac, logged in at the screen), bring it into the fleet.** Run the paste-ready prompt in `prompt-g1-m09.md` (wave-1125). Decide first whether it is the original g1-m09 or a new `g1-m15`. Exit codes are in the prompt.

5. **g1-m05 (Windows), Defender.** Exclude `C:\semio` from Defender real-time scanning. Requested by F4 and F14.

6. **g1-m05, confirm the popup and hidden-task fixes (F10, F17).** Within about 10 minutes of code arrival:
   - `schtasks /Query /TN semio-fleet-store-mount /XML` (`<Command>` must be `conhost.exe`, `<Arguments>` must start with `--headless`), and the same for `semio-fleet-replica`, `semio-fleet-cache`, `semio-fleet-relay` if present.
   - Decode the replica `-EncodedCommand`: `[Text.Encoding]::Unicode.GetString([Convert]::FromBase64String('<value>'))`. It must read `do{& powershell.exe --% ...`.
   - Crash test: end the replica `powershell.exe` or `bun.exe` in Task Manager. It must return within about 10 s (new PID), with no window.
   - Watch 3 minutes of dependencies and readiness: no console flashes. In Task Manager (Details), no visible `cmd.exe` or `powershell.exe`.
   - `net use` (mount still maps) and `repo fleet status` (m05 still syncs).

7. **g1-m05, profile of a changed binding (row 5), only if the next attempt is still slow.** With the replica daemon stopped, run one `repo dependencies sync --all` under Windows Performance Recorder or Process Monitor on `C:\semio\.cache` and `C:\semio`. Report the top costs.

8. **Coordinator Mac, free disk (row 12).** Host volume 827 of 926 GB used. The graph image can grow to its 24-32 GiB cap only when host free space allows it. Free space by removing the test-sandbox images under `.cache/test-sandbox/rJTEOQy` (if they are not in use) or other host data. Decision by the user.

9. **Decision: g1-server (row 13).** Either keep it as an executor and document that its toolchain is provisioned by hand, or remove its worker registration and make it coordinator-only (A4).

10. **Decision: stage-failed refresh keeps Ready or not (row 7).** Coordinator or user decision (F5 r2).

11. **Decision: archivist commit of the 2,738 staged index entries (row 23).** Only the archivist machine, through the `commit` skill. Ask before running; never from a worker.

12. **Idle cargo holder pid 96461 (row 22).** Check and decide; the audit did not kill it:
    ```
    ps -o pid,etime,%cpu,command -p 96461
    ```

No other user action was requested by any lane. Items 1 to 6 are the only ones that unblock a worker directly.

---

## 4. Counts and top UNOWNED

Section 1: 27 rows (rows 1-25 are red or partially red test files and groups, row 26 is the F7 sample summary, row 27 is the unmeasured set). Section 2: 33 items. About 6 rows have a live owner (F6, F28, F29, F30, F24); the rest are UNOWNED, USER, or closed. Section 3: 12 user actions (items 1-6 block workers directly).

Top 5 UNOWNED by impact on "all workers Ready":
1. Post-fix workspace verification on workers (F1, F22, F23, F27, F21): no live lane.
2. Workspace Nx run-many exit 130 root cause on 9 workers.
3. Native dependency without build project owner (`semio-s-artifact-norm-en1993-scene-schema-grips`), blocking the pool on m01 and g1-server.
4. g1-m05 changed-binding cost (9-14 s each) after F14.
5. g1-m04 inode gate and EACCES code-side verification (F3, F4 retired).
