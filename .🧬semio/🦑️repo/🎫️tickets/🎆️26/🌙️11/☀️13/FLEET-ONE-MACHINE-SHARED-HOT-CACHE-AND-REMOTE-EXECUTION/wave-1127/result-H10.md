# H10 Result: stalled stages and version drift

## 1. Stall detector (what it measures)
Code: `.../sidecar/🚦️ready/🟦️.ts` (stage runner) with `🌲️process-tree/🟦️.ts`. A stage is stopped with SIGTERM ("stalled", not counted as failure) when `stallStopMilliseconds` (600 s, scaled by machine speed factor) pass without "movement": a new normalised log line, a growing `taken N of M` counter in the log, CPU >= 500 ms in the stage's OWN process tree, a new child, a warden hold or a grant wait. Elapsed-time spellings (`(2073 ms)`, `(N s)`) are normalised away.

## 2. Evidence (13 origins, ready-log-*-stalled.txt, ready-timings.json)
- 11 of 13 workspace stalls end on the same last lines: `Starting the repo graph service` / `Loading the project graph of 440cffa5563d` / `Planning the selected targets and what they need.` then SIGTERM after 606-690 s (m10: 20 runs in a row at ~610 s, many 1000-2200 s when the stall clock was reset by other moves).
- The planning work runs in the resident graph service's slice worker (a bun process OUTSIDE the stage tree, 80-170 MB stage vs 2.3 GB RSS worker). Its CPU is never credited, and its own progress (`taken 559 of 616`, ledger done/running, uptime 614 s, in `slice-memory.json` and the owned slice report) is not mirrored into the stage log tail, so the stage looks silent.
- m10 sample (`ready-log-workspace-sample.txt`): worker main thread busy 100% for 188 s, one hot native frame, heap 1.77 GB / 15.5M objects, `ownerFactsMemo` 1.38 GB on a machine with swap 8-9 GB: slow GC/memory-bound planning, not a deadlock. The tail of m04/m05/m08 `tools` stalls and m05 `dependencies` are similar (counters / steps repeating after normalisation; m08 tools shows `Checking declared package providers: N edges` climbing).
- m02/m08/m13: 67/162/90 `[capacity] waiting` lines (admission wait of 6 min+ behind `graph-plan` 4.5-4.9 GB) precede the same silent planning.
- Verdict: FALSE stalls (slow but progressing, memory-bound on 16 GB machines), plus a repeated-restart loop that throws away the progress of the planner each ~10 min ("no lost work on retry" is violated for the in-memory plan; sealed owner results do persist, hence the counters creep).
- The units ledger (`compiled 787`) in those rows is machine-wide counters, not stage evidence; not used.

## 3. Fix landed
- New pure `readySliceMovedSignatureV1(report, startedMs)` in `🌲️process-tree/🟦️.ts`: signature of the fresh owned slice report (phase name/done/total, owners ledger done/running/total, owners finished/taken); `null` for a report of an earlier attempt, a finished read or a malformed one. CPU/uptime excluded on purpose: a spinning loop that decides nothing is still a stall.
- `🟦️.ts` stage runner (mirror tick, next to the existing grant-wait/slice-silent check): a changed signature calls `moved()` (restarts the stall clock). The kill rules `sliceSilentMilliseconds` (900 s without a report write) and hard caps are unchanged, so true wedges (stopped loop, same counts for 10 min) still end as stalls with the real reason.
- Test: `🌲️process-tree/🧪️tests/🚶️slice-moved/🟦️.ts` (independent CPython model); `bun test ./🟦️.ts` passes (5 expects). Syntax of both edited files verified with `bun build --no-bundle`.

## 4. Version drift
- `guards.version` in `attest.json` is the installed GIT version of the host (`gitVersionV1`, resolved from `SEMIO_GIT`/PATH/OS locations), not a code or tool-pin version. Code revision is identical everywhere (head 9c8a98ec2349, code 5af6df83766e, guards.pins 4c7bc38fc560 on all). m04 Linux distro git 2.55.0, m05 Git for Windows 2.53.0, macOS 2.54.0. Git is not in the pinned tool table (`🔩️native/🥾️bootstrap` pins go, node, rustup, uv, zig, dotnet only), so nothing can derive it from the revision; the fleet warning `git-version` correctly compares against the fleet majority.
- Not changed: pinning git needs a provisioned portable git per platform (MinGit zip + tool row); attest tests (`🩺️attest/🧪️tests` line 901) encode the majority rule. Recommended follow-up: add `git` to `tool-setup` table + bootstrap pins and make the guard compare to that pin.
- m05 node 24.14.1 vs pin 24.15.0 (package.json engines): the pinned Windows node lives in `%LOCALAPPDATA%\semio\node\v24.15.0\node.exe` (bootstrap `🔵️.ps1`); m05 only has `C:\Program Files\nodejs` 24.14.1, so verify reports `differs`. The tools stage there never reaches provisioning (stage process "gone, no exit file", EPERM esbuild.exe, partition capacity): a Windows lane issue, left to the lane that owns m05.

## 5. Unverified
No `repo typecheck`, dangling-imports or full suite run (host load 60+, load gate); no live re-run of a stalled stage; landing lock not taken (single atomic edit per file, re-read first). Workspace-specific failures of m01/m03/m08/m14 (H1) not touched; `result-H1.md` exists but was not read.
