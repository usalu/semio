# H87: remaining non-ready workers (m03 m04 m05 m10)

Store evidence only (`S/origins/<o>`), no SSH, no landing (no code defect found that is not already landed by H71/H80/H82). Times CET 11/27.

## Timeline
- 14:11 baseline. All four `running`, none ready; supervisors restart every few min (landings), each time adopting the stage child.
  - m04 tools attempt 3: past the old blockers (lease/inodes: 629k inodes free, 75 GB; no `Bounded relative file read` anywhere). Now in the dependency payload capture (`Store dependency payload profile`): 11:00 -> 14:11 rows 21.8k -> 40.7k, work 97k -> 166k at 990 s, 255k at 1361 s (14:24). Native calls are only ~6 % of wall (fstat/read/openat ~0.7 s per 11 s profile); the machine (Steam-Deck class, 15.8 GB, swap 6.4 GB used) runs 3 cargo + 3 rustc local units from agents, so the JS stage is CPU-starved. Slow, advancing, not stuck.
  - m05 (Windows): promoted past tools to `workspace` (stage 5/5, 14:07 adoption), steps `installed-paths:resume`, then `dependencies:workspace-edges`, `source.served`; 6084 s of repo work. Last ready-wedge 12:10 (H65 watchdog), no repeat since.
  - m03: workspace attempt 5 on current code; last failure 12:52 was `Module not found .../sealed/<id>/.../buildscript` (F9 of H79, a retired sealed dir), covered by H82 (sealed snapshot leases, 13:00). Now `Reading the project graph slice`, waiting for capacity (3.4 GB request, 2 running, floor 2048).
  - m10: workspace attempt 35; stalls at 12:55/13:35 were the supervisor stopping a silent graph read; the read now prints progress (`taken 545 of 616`, `reading the owners 545 of 3600`, `scanning the owners`) so the stall watchdog sees movement. The slice worker holds 3.8 GB on a 16 GB mini with 9.9 GB swap (pressure warn); the read restarted at 14:22 (78 s) after a graph hand-over.
- 14:24 m03 `repository:command` 240 s, m04 payload 1361 s, m05 source.served, m10 read attempt 1 again.

## Blockers named
1. m04: CPU starvation by concurrent agent cargo (not a code defect; priority H57 applies, payload capture is long). Possible follow-up: reduce per-entry `advance()` overhead in `readJavascriptDependencySelectedPayloadV1` (dependencies/storage), not measurable without a profile.
2. m03/m10: 16 GB Macs: graph slice read needs 3.4-3.8 GB under a 2048 MB floor with agents and swap; reads queue for capacity and restart on graph hand-over.
3. m05: slow boot under Defender (user action `C:\semio` exclusion open).

## Unverified
No worker reached Ready during this lane's observation window written here; see the final timeline lines below if appended.

## Round 2 (14:35-15:56 CET)

### 1. Payload capture overhead (`readJavascriptDependencySelectedPayloadV1`), measured
Harness: `T/🧰️scripts/H87/bench.test.ts` (+ `mk-fixture.ts`), run with `repo test-file`, `SEMIO_TEST_FILE_OFFLOAD=never`, fixture outside the checkout, before = live code, after = a copy-on-write overlay with the staged patch. (A first set of runs hung 10 min: my harness replaced `process.stderr.write`, the capture waits for the write callback; harness bug, not product.)
- Cause: per entry the descriptor layer and the loop called `checkJavascriptDependencyReadContextV1` about 40 times (198k checks for 5k entries, 9.9 M for 250k; invocation lineage, refinement chain and lease check each time), the final sort built two Buffers per comparison, every role name built a TextDecoder, and the postorder removal re-listed the whole directory twice per removed entry (quadratic in directory size).
- Fix (patch `stage-pay.patch`, 3 files): cheap gate (full check every 16th call, handed to the relative layer too), one UTF-8 key per row for the sort, ASCII fast path and one decoder for role names (`🏷️roles`), and in `retireRepositoryDependencyRelativePayloadV1` the roster re-list only every 256 removals and at directory end (the directory stamp check stays per removal), ASCII-exact `compare`.
- Result, same fixture, identical payload sha (`6372bcd795ae6194`):

| entries | before | after | checks before / after |
|---|---|---|---|
| 5 002 (x2) | 4.0 s / 8.5 s (load) | 2.2 s / 3.9 s | 198 239 / 13 670 |
| 250 002 | 373.1 s (1492 us/entry) | 223.8 s (895 us/entry) | 9 901 093 / 681 908 |

-40 % on this Mac under load 45. Not done: directory reopen by path per directory (depth x openat+fstat; in the real `.bun` layout with few files per directory this is the next cost), and skipping the per-file hash when the content store knows the digest (needs a custody proof; not attempted).

### 2. m03 root cause found (stale build-script stand-in), fix staged
`ready-log-workspace-failed.txt` of m03 shows the SAME `Module not found .../sealed/7cb8abd8c9858a0b/.../unit-cache/🏗️buildscript/🟦️.ts` at 12:52, 14:24 and 15:50 CET (attempts 4, 6, 8; `serde_core` and `zmij` stand-ins `q90be4b/prototype/build/serde_core/{18f2c98debd24e9d,91a24c2b2d688bae}`). Cause: `shimSourceV1` bakes the sealed module path into the `sh` stand-in that Cargo keeps as `build_script_build`; Cargo treats the unit as fresh forever, so once that sealed snapshot is retired every run fails. H82's leases only protect snapshots from now on. Fix (`stage-fix.patch`): the stand-in runs the real build script directly when its module is gone (`[ -f module ] || exec real "$@"`), test `🧪️tests/🪦️retired-module` (passes in a standalone overlay run).

### 3. Landing blocked by the `repo land` gate (finding for H74)
`repo land check` of both stages fails only in declared tests that pass on the live tree and fail the same way for a whitespace-only change (control `stage-ctl`):
- storage-opening: `Exact Original workspace required` (control root vs overlay path), also with a no-op edit of the publication file.
- javascript-storage: test reads a ticket file under `process.cwd()` (`🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL`), absent in the overlay.
- cargo-unit-cache-processes: `sccache: error: path must be shorter than SUN_LEN` (overlay path `~/Library/Caches/semio/land/<id>/overlay/<lane>/.cache/scratch/...`).
typecheck and dangling-imports pass for both. Nothing landed. Until the gate runs tests with a short overlay path whose root equals the control root, no caching member can land through `repo land`.

### 4. Worker check 15:54
- m03: attempt 8 failed 15:50 on the stale stand-in (above); backoff.
- m04: left payload capture (14:35), `selected-postorder` (work 229k, 15:31), now `selected-ownership:frontier:1107776` (15:54); still adopted `tools` run, attempts 3.
- m05: workspace stage, `workspace-edges` 195k work, 4665 s of the same supervisor.
- m10: attempt 36 failed 15:49 with a Rust compile error (`native retirement requires its original ...`, norm-din4108-rs build; peer mid-edit), graph slice read had finished (`owners done 616 of 616`).

### Round 2b: landed 16:00 and the gap that remains (16:15)
- Landed 16:00:02 CET, manual (coordinator exception), locks `caching-caching` (3 payload files) and `caching-cargo` (stand-in fix + test), `T/🧰️scripts/H87/land.py`, landings.md entry written. Live: storage-opening 4/0, javascript-storage 47/0, retired-module 1/0, `repo typecheck` passed.
- Gap found after landing: the stand-in guard only protects stand-ins written from now on. m03's existing stand-ins (`q90be4b/prototype/build/serde_core/*/out/build_script_build`, also `zmij`) are old text with the sealed path baked in and Cargo keeps them fresh, so m03 keeps failing until they are rewritten or deleted.
- Staged, not landed: `stage-heal.patch` (`healStaleBuildScriptShimsV1` in `🏗️buildscript`, called once per build directory and module from `runUnitCacheMainV1` before the first rustc of a run; rewrites a stand-in whose module is gone via the idempotent `installBuildScriptShimV1`; second test in `🪦️retired-module`, 2/0 in a standalone overlay run; `repo land check`: typecheck, dangling, debug lines pass; the only red is the known sccache SUN_LEN overlay false red). Needs the 17:00 slot (manual with `land.py <stage> caching-cargo` while the gate false red stands) and another coordinator exception, or H89's gate fix. Ordering caveat: the sweep runs when the first rustc wrapper starts; if Cargo runs the stale build script before any rustc call, the heal comes one attempt late. A user-side alternative: delete `.cache/b/r/m/u/l/q90be4b/prototype/build/{serde_core,zmij}` on m03.
- Worker check 16:15: m03 attempt 8 running again (graph slice read, adopted); m04 `tools` adopted, now `Source-aliases` (past payload/postorder/ownership/plan phases); m05 and m10 not rechecked beyond 16:03 (m05 workspace 7662 s, m10 backoff after the 15:49 Rust failure).

### Round 2c (16:56): heal patch revoked
Coordinator revoked the 17:00 landing of `stage-heal.patch` (full build-system freeze until the seal; the 16:00 landings of H87 and H88 restarted the proof graph read). Nothing landed after 16:00. The patch stays staged: `T/🧰️scripts/H87/stage-heal.patch` (+ `land.py`), to land after the seal with a manual or `repo land` landing. m03 is therefore not healed by code; until then only deleting `.cache/b/r/m/u/l/q90be4b/prototype/build/{serde_core,zmij}` on m03 clears the stale stand-ins.
