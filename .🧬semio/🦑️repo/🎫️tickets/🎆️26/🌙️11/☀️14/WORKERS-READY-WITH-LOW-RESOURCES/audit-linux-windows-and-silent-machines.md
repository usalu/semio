# 🔍️ Audit: g1-m04, g1-m05, g1-m06, g1-m09 Not Ready, Plus Silence Check (read-only)

Snapshot: 2026-11-17 21:01 CET (UTC+1). Read-only: no source file, no build, no git write, no sub-agent. Code paths are relative to the repo root `/Users/ueli/Documents/semio/`.

Sources: the evidence store `/Users/Shared/semio-store/v1/origins/<machine>/` (`ready.json`, `ready-timings.json`, `pool.json`, `attest.json`, `ready-log-<stage>[-failed|-stalled|-containment].txt`, `service-failure*.txt`), the ticket `WORKERS-READY-WITH-LOW-RESOURCES/workers/<machine>/` notes (`round-31.md`, `round-31.tools-*.md`, `round-31.tools-capacity-error.md`, `BLOCKER.md`, `round-29/30.tries.txt`), `result-R99.md`, and code found with `rg`.

Limits that matter for the answers:

- The store keeps only the last failure tail per stage. `ready-log-<stage>-failed.txt` is overwritten on every failure, and `ready-timings.json` keeps about 40 rows with the outcome but no message. Error text for earlier tries is therefore quoted from the ticket notes where they exist. Where I only infer it, the text says "inferred".
- The `ready-timings.json` times are local CET. The `*ms` fields in the JSON are epoch milliseconds, converted to CET.
- Housekeeping: a scratch listing was briefly written into this ticket folder by mistake and removed again. Search scratch files went to the session scratchpad.

## 1. Overview

| Origin | Platform (host) | Newest store write (CET) | Age | `ready.json` state | Stage, tries | Last sign / last error | Verdict |
|---|---|---|---|---|---|---|---|
| g1-m04 | Linux, Steam Deck (`ueli-workstation-lin-1`) | 2026-11-16 23:07 | 22 h | running, stage `tools`, attempts 38, reason `adopted` | `tools`: 39 finished runs (37 `stage-failed`, 1 `stage-timeout`, 1 `stalled`) plus 1 adopted run | `[repository:command] still running after 6741 s`, no progress lines after that. Last explicit error: `Internal error: unable to open database file` (try 37, 14:55 CET 11-16) | silent since 23:07, needs a human |
| g1-m05 | Windows (`DESKTOP-6NSLEUO`) | 2026-11-17 16:16 | 4 h 45 min | running, stage `tools`, attempts 12 (adopted) | `tools`: 36 finished runs (9 `stage-failed`, 3 `stage-timeout`, 24 `stalled`) plus 1 running since 14:31 CET 11-17 | `[repository:command] still running after 6264 s`, tail is heartbeat only, last phase `source.served`. Last explicit error (14:11 CET 11-17): `Internal error: Immutable input physical work capacity exceeded` | alive but stuck, needs a human to look |
| g1-m06 | macOS, Mac mini M4 16 GB (`...mac-mini-m4-16gb-256gb-04`) | 2026-11-16 10:41 | 34 h 20 min | running, stage `workspace` (5 of 5), attempts 2 | `native`: 9 `stage-failed` then ok 02:47; `repo-mcp`, `dependencies`, `tools` ok; `workspace`: 2 `stage-failed` (exit 75), 12 `stalled`, 1 running | `Reading the project graph slice (1079 s)`. Last error: exit 75 "could not be read on this machine yet" (inference grew to 9006 MB, floor 6553 MB) | silent since 10:41 Nov 16, needs a human |
| g1-m09 | unknown | 2026-11-14 02:13 (only `fleet-auth.json`) | 91 h | no `ready.json` | never started, never attested | none | not enrolled, needs a human |

Context: g1-m01, m02, m03, m07, m08, m10 and g1-server are alive (newest file 21:01 CET). Their `ready.json` shows stage `workspace` (5 of 5) with "Reading the project graph slice" or a "previous-failed" back-off. None of the seven is `ready: true`. This is the same picture as the BLOCKER and R99 notes.

## 2. Q1: g1-m04 (Linux, Steam Deck, `/home/deck`)

**Last sign of life.** `ready.json`, `pool.json`, `attest.json` and `ready-timings.json` were all written 2026-11-16 23:06 to 23:07 CET. The last tools line in the store is `[repository:command] still running after 6741 s` (header `2026-11-16T22:06:55.753Z tools`). Nothing newer exists for m04, not even in the replica cursors.

**Current try.** `ready.json` says `state: running`, `stage: tools` (4 of 5), `attempts: 38`, `reason: adopted` ("Stufe tools übernommen, sie lief schon"), `pid 487094`, `peakMegabytes 1866`, tail `[script] … native-artifact-input-read:34138`. The try was adopted, not started, so its start is the replica's `startedMs` (21:34 CET 11-16). The stage counter (6741 s at 23:06) and the adoption time do not line up exactly. Either way, no output has been written since 23:06.

**Every earlier try** (`ready-timings.json` rows, all `tools`, all `exit 1` except the last):

| Tries | Start (CET) | Duration | Outcome | Error text |
|---|---|---|---|---|
| 1 to 9 | 11-16 01:40 to 04:16 | 567 to 751 s, peak 1370 to 1602 MB | `stage-failed` | not preserved in the store. Inferred from the round-31 quotes from the 02:28 CET section on: `Internal error: Exact selected dependency fields required` |
| 10 to 35 | 11-16 04:47 to 13:09 | 1 to 21 s, no RSS | `stage-failed` | not preserved in the store. Inferred from round-31 section 03:48Z (04:48 CET, try 10): `Internal error: Authorizer source closure capacity exceeded: files=2049/2048, bytes=18543240/20971520` |
| 36 | 11-16 13:29 | 3209 s | `stage-failed` | not preserved (round-31 section 13:55Z, see below) |
| 37 | 11-16 14:42 | 750 s | `stage-failed` | round-31, last tools run (13:55 UTC): `Retained dependency generation eda9de17b6b269e988a1 cannot be read with the current field set (file bindings.json; missing ["singletons"], unexpected []; …); it was written by older code and is retired as obsolete` then `Internal error: unable to open database file` then `repository:command failed with exit code 1` |
| 38 (stalled) | 11-16 19:28 | 1576 s | `stalled` | `SIGTERM received…`, `Cancelled (SIGTERM); leases released.`, `Internal error: Original Script process terminated` (`ready-log-tools-stalled.txt`). The SIGTERM comes from our own sidecar, see R99 section 1 |
| 38 (timeout) | 11-16 19:54 | 3601 s | `stage-timeout` | `[repository:command] still running after 3590 s`, `SIGTERM received…`, `Cancelled (SIGTERM); leases released.`, `Internal error: Original Script process terminated` (`ready-log-tools-failed.txt`) |
| adopted (now) | since 21:34 CET 11-16 | still open | running | none after `still running after 6741 s` |

So: 39 finished `tools` runs, 1 open run. The last explicit error is "unable to open database file" (try 37). The last error in the store is the timeout text from 19:54 CET.

**Disk, quota, free space, packages.**

- Disk quota and space (from `workers/g1-m04/round-31.md`, sections 18:19Z to 18:28Z on 11-16): `/tmp` is a tmpfs with a per-user quota. `build-agent.log` shows `pool stopped: EDQUOT: unknown error, write` 20 times from 04:17Z. `/home` (ext4) reached 5 966 399 of 5 966 400 inodes used, so `pool stopped: ENOSPC: no space left on device` 63 times and `agent lease not renewed for 600 s, leaving` at 14:10Z. The replica daemon could not take its lock (`ENOSPC … daemon.lock.steal`). The owner then removed 100 orphaned `owner-pool-staging` folders and 55 `/tmp/semio-owner-pool-*` folders at 18:24Z, which freed 420 169 inodes. Nothing newer than that about free space is in the store, so the current state of the disk is unknown.
- Missing system package: none found. The native stage printed the pacman package list (`ready-log-native.txt` line 2) and finished with "Native (Unix) bootstrap complete".
- Stage `tools` has no copy of the disk error in the store. The `unable to open database file` message is SQLite's generic text. See section 5 for why the sidecar does not classify it as `store-full`.

**Hypothesis (short).** Two things stack up. The disk ran out in inodes and quota on 11-16 (round 31). The stage then failed on SQLite open errors, and the code classifies only the "No space left (EDQUOT|ENOSPC) in … : the disk" text as `store-full`. The raw SQLite text falls through to `stage-failed`. The "retired as obsolete" generation (10 of 11 fields) and the closure ceiling (2049/2048 files) are separate, older faults. After 19:54 the run shows nothing at all, and the store has no record after 23:07 CET 11-16. This is consistent with the Deck being asleep or off, or the replica daemon having stopped. Nothing in the store proves which one.

## 3. Q2: g1-m05 (Windows, `DESKTOP-6NSLEUO`)

**Last sign of life.** `ready.json` and `ready-log-tools.txt` were written 2026-11-17 16:16 CET. `ready.json`: `state: running`, `stage: tools`, `attempts: 12`, `reason: adopted`, `tail: [repository:command] still running after 6264 s`. The last tools lines are heartbeats, with `[repo] still working after 6265 s; last phase started: source.served`. There is no counter line in the last 60 lines. `pool.json` in the store is stale (written 2026-11-16 16:42 CET), the newest pool file is a `.tmp` from 16:16.

**Current try.** Started 13:31:24Z (14:31 CET) per round-31. It shows no progress in the store, and the last heartbeat is 16:16 CET. Its worker `bun.exe` was busy (round-31 measurement: 5.2 s CPU in 10 s at 19:15Z UTC). So the worker is not idle, it is quiet.

**Every earlier try** (`ready-timings.json`, CET):

| Tries | Start | Duration | Outcome | Error text |
|---|---|---|---|---|
| 1 to 7 | 11-16 14:35 to 15:10 | 10 to 19 s | `stage-failed` | `Internal error: Exact safe dependency physical identity required` (quoted in `workers/g1-m05/round-31.tools-stage.md`; not in the store) |
| 8 to 18 | 11-16 15:30 to 18:24 | 620 to 3345 s | `stalled` (8 and 9 are `exit 1`/`exit none`) | `Process 70836 (… show projects) holds the dependency store since 08:09:50Z; waiting for it` (`ready-log-tools-stalled.txt`, 10:03Z) |
| 19 | 11-16 19:16 | 4555 s | `stage-timeout` | not preserved |
| 20 to 21 | 11-16 20:52, 21:30 | 635 s, 4567 s | `stalled`, `stage-timeout` | not preserved |
| 22 | 11-16 23:31 | 14 424 s | `stage-timeout` (hard cap) | not preserved |
| 23 to 29 | 11-17 03:52 to 05:14 | 617 to 1523 s | `stalled` | not preserved |
| 30 | 11-17 05:25 | 13 355 s | `stage-failed` | not preserved. Round-31 says the tail was replaced and the same text is an assumption |
| 31 to 35 | 11-17 09:28 to 10:53 | 617 to 1684 s | `stalled` | `Process 70836 …` / `Process 53380 (… show projects)` holds the store (waiting) |
| 36 | 11-17 11:03 | 11 273 s | `stage-failed`, ended 14:11 CET | the three-line tail below |

The last explicit failure (try 36, ended 14:11 CET 11-17, `ready-log-tools-failed.txt`):

```
[DATA] Dependency sync incomplete {"completedBindingsChanged":0,…}
Internal error: Immutable input physical work capacity exceeded
repository:command failed with exit code 1
```

Counter at the failure: `native-artifact-input-stage:2584` after 11 259 s (`round-31.tools-capacity-error.md`).

**Quota, free space, packages.** No `ENOSPC`, `EDQUOT`, quota, "no space" or missing-package text exists in the m05 store or in its ticket notes (ripgrep over the worker folder found none). `ready-log-tools-containment.txt` at 14:44 CET shows the lease registry: 197 entries, 144 not ours, 13 not alive, 30 windows. Stale leases are present, but no disk problem is named. The `dependencies` stage finished on 11-16 13:31 CET (3810 s, exit 0).

**Hypothesis.** The 11:03 try is the first one that ends in the immutable-input capacity error. The R99 read plan (landed 11-17 00:00 CET) adds steps to the protected-input read, and each step increments `work.count`. The cap is reached after about 2 584 steps, roughly 3 hours in, and the stage is refused with exit 1. That is a fail-closed counter ceiling, not a disk problem. Separately, the current run prints heartbeats only, and the stall rule in the sidecar only fires when there is no output and no processor time for 600 s. A busy Windows worker never meets that condition, so the run is not stopped. This matches what the store shows, but I did not verify it on the machine.

## 4. Q3: g1-m06 (macOS, Mac mini M4 16 GB)

**Last sign of life.** `ready.json`, `pool.json` and `ready-timings.json` were written 2026-11-16 10:41 CET. The last log line is `Reading the project graph slice (1079 s).` (`ready-log-workspace.txt`). `ready.json`: `state: running`, `stage: workspace` (5 of 5), `attempts: 2`, `peakMegabytes 7539`. Silent for 34 h 20 min.

**Last stage.** `workspace`. Native, repo-mcp, dependencies and tools all passed by 02:52 CET 11-16, so the last stage is workspace.

**Every try** (`ready-timings.json`):

- `native`: 9 `stage-failed` (exit 1, 81 to 447 s) from 11-15 22:17 to 11-16 00:01 CET, then ok at 02:47. Error text: `ready-log-native-failed.txt` (11-15 23:03 CET): `awk: can't open file /Users/ueli/Documents/semio/package.json` and `Invalid workspace package: /Users/ueli/Documents/semio/package.json`. This is a missing workspace `package.json` on the first runs, and the stage passed later.
- `repo-mcp` ok 02:47, `dependencies` ok 02:47 (299 s), `tools` ok 02:52 (630 s).
- `workspace` (14 finished runs): 03:03 `stage-failed` exit 75 (8314 s, 7539 MB peak); 05:22 `stalled`; 05:44 `stage-failed` exit 75 (3359 s); then 12 `stalled` runs from 06:41 to 09:37 CET (607 to 2769 s, 5 to 8 GB peak). The 09:37 run was stopped at 10:23 CET (`ready-log-workspace-stalled.txt`).

**Last error (exit 75, 05:44 try, ended 06:40 CET):** `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 9006 MB, more than the 6553 MB this machine gives a plan, and only 2746 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could.` (`ready-log-workspace-failed.txt`). The last stall reason is `Stage workspace showed nothing new for 10 minutes and was stopped; started again at once.`

**Disk, quota, packages.** Nothing about disk or quota. Memory is the limiting factor (see hypothesis).

**Hypothesis.** The workspace plan on a 16 GB Mac grows past the 6553 MB plan floor, so the graph read is refused or stalls at "verification 3195 of 3195" (`ready-log-workspace-failed.txt`). The stall rule (10 minutes without new output) then restarts it. The "cut smaller" path reduces the wave but does not bring the plan below the floor in time. Why the machine went silent at 10:41 CET is not in the store.

## 5. Q4: g1-m09

- Only `/Users/Shared/semio-store/v1/origins/g1-m09/fleet-auth.json` exists (2026-11-14 02:13 CET, 562 bytes, key set for the origin, `sequence 1`). No `ready.json`, `pool.json`, `attest.json`, `head.json`, logs or segments.
- No origin's `attest.json` lists `g1-m09` among its cursors (checked for m04, m05, m06 and g1-server; they list the other ten origins). The ticket notes say "never attested" and "no `attest.json`" (`result-R49.md`, `result-R62.md`, `result-R63.md`, `audit-cross-network.md`).
- There is no `workers/g1-m09` folder in the ticket.
- Verdict: enrolled once, never joined the replica. The evidence does not show whether the machine was ever set up.

## 6. Q5: Code behind each tools-stage and workspace message

Paths are relative to `/Users/ueli/Documents/semio/`. Message keys are in the `🧬️schema/🔣️.json` or `📖️texts/🔣️.json` files next to the code.

| Message (as seen) | Machine | Code | Hypothesis |
|---|---|---|---|
| `Immutable input physical work capacity exceeded` | m05 (11:03 try) | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🟦️.ts:149` (and `:171`, group variant): `if(++work.count>limits.maximumWork)throw Error(…)` | Step counter ceiling. The R99 read plan adds steps, and the cap is reached after about 2 584 steps. New since R99 landed. The cap counts steps, not progress. |
| `[DATA] Dependency sync incomplete` (in m05 log) | m05 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:792`, which prints `[DATA] Store dependency sync incomplete` | Text differs by the word "Store". Either m05 runs a different build or the mirror trimmed the text. Check the deployed digest per machine. |
| `Exact selected dependency fields required` | m04 (tries 1 to 9 and later, inferred) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:194` (`captureJavascriptDependencySelected…`) | Field set of the old generation (10 of 11 fields) differs from the new code's expectation. The retirement path should remove it, but the stage ends with an Internal error instead. |
| `Retained dependency generation … retired as obsolete` | m04 (try 37) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🔣️.json:86` (text); retirement under `…/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/` | Same cause as above. The text says "retired", yet the run still fails. |
| `Internal error: unable to open database file` | m04 (try 37) | SQLite's own message. `🧰️framework/🔨️modules/⚙️runtime/🗃️sqlite/🟦️.ts:8` to `:20` rewrites it only when a probe finds EDQUOT or ENOSPC. The sidecar classifier `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts:80` to `:83` uses `storeFullPattern` (`No space left \((EDQUOT\|ENOSPC)\) in … : the disk`, `🧬️schema/🔣️.json:47`). | Disk full (round 31) surfaces as the generic SQLite text, which the classifier does not match. The try ends as `stage-failed` (`…/🚦️ready/🟦️.ts:288`), not as `store-full`. |
| `Authorizer source closure capacity exceeded: files=2049/2048, bytes=…/20971520` | m04 (tries 10 to 35, inferred) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json:883`; check at `…/🔏️authorizers/🟦️.ts:126` to `:128` | Fixed ceiling, one file over. Raise it or derive it from measured closure size. |
| `Exact safe dependency physical identity required` | m05 (tries 1 to 7) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/📸️observation/🟦️.ts` (exact line not located) | Windows file identity cannot be proven, so the observation refuses. Pre-R99. |
| `[repository:command] still running after N s` | m04, m05, m06 | `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧬️schema/📣️heartbeat.json:12` | Heartbeat only. It is not progress by itself (`🌲️process-tree/🧪️tests/🔣️.json:40`, "heartbeats alone are no progress"). |
| `Cancelled (SIGTERM); leases released.` / `Internal error: Original Script process terminated` | m04 (19:54 timeout and stalled tries) | `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧬️schema/🔣️.json:47` (text); the kill is sent by the sidecar: stall tracker and progress key at `…/🛰️sidecar/🚦️ready/🟦️.ts:395`, outcome and failure codes at `:263` to `:288` | Our own stall or timeout kill, as R99 section 1 says. Not a crash. |
| `Stage workspace showed nothing new for 10 minutes…` | m06 | `…/🛰️sidecar/🚦️ready/🟦️.ts:274`, text `🧬️schema/🔣️.json:95` | Stall rule restarts the stage. |
| `The project graph could not be read on this machine yet: … grew to 9006 MB, more than the 6553 MB …` | m06 (exit 75) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧬️schema/🔣️.json:2037` to `:2040` (`host-graph-plan-too-heavy`, `host-graph-memory-refused`, `host-graph-owner-too-heavy`) | Plan floor (6553 MB) on 16 GB. The plan does not shrink enough. |
| `graph: waiting for memory headroom …` | m06, others | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧬️schema/🔣️.json:60` | Admission wait. Not a failure. |
| `[capacity] waiting Ns for run: not enough free memory` | m04 (repo-mcp stage) | `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json:1396` | Admission wait behind graph-plan and workspace:setup. Not a failure. |

**R99 comparison.** R99 (`result-R99.md`) covers "stage tools never ends" on m04 and m05: the SIGTERM from our sidecar, the 3600 s cap, and the holder wait. Its fix landed 2026-11-16 23:00Z (00:00 CET 11-17). Its section 6 says the m04 run keeps old code and the m05 new-code run was not yet observed.

- Same as before: the `Exact selected dependency fields required` / retirement pair (m04, pre-R99), `Authorizer source closure capacity exceeded` (m04, 03:48Z, pre-R99), `Exact safe dependency physical identity required` (m05, 14:35 CET 11-16, pre-R99), the holder waits on `show projects` (m05, pre-R99).
- New since R99: `Immutable input physical work capacity exceeded` on m05 (first try under R99 ending in it: 11:03 CET 11-17). The `unable to open database file` on m04 is also not in R99, but it comes from the disk-full episode of 11-16, which R99 does not cover.
- Still open from R99: the stall rule with a busy worker that prints heartbeats only (m05 now, m04 earlier).

## 7. Needs a human at the machine

1. **g1-m04 (Steam Deck).** Check that the Deck is on and awake and that the replica daemon runs (silent since 23:07 CET 11-16). Run `df -i /home` and `df /tmp` to see whether the inode and quota problem came back. The old Trash worktree (about 1.02 million inodes) is the owner's decision. Check `~/.cache/f/owner-pool-staging` for new orphans. Then decide whether to stop the adopted run.
2. **g1-m05 (Windows).** Check that the machine is awake and that the sidecar and wrapper processes are alive (round-31 PIDs 15372, 81548, 66712 are named there; their current state is unknown). The run started 14:31 CET has no progress since. The decision to stop it is the owner's.
3. **g1-m06 (Mac mini).** Check that the machine is on and the replica runs (silent since 10:41 CET 11-16). Memory: 16 GB, the plan floor is 6553 MB.
4. **g1-m09.** Install and enrol the replica, or confirm it is out of the group. Nothing has ever run there.

## 8. Fixable in code

1. **Protected-input step cap (m05).** `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🟦️.ts:149` and `:171`. Count progress (files or bytes), not every step, or raise the cap. This is the new m05 failure.
2. **Stall rule with a busy, quiet worker.** The sidecar's stall condition needs no output and no CPU for 600 s. A busy `bun` child with heartbeats never meets it (m05 now). Count heartbeats without a counter move as no progress, as the process-tree test already expects. Code: `…/🛰️sidecar/🚦️ready/🟦️.ts:395`, `🌲️process-tree/🟦️.ts:87`.
3. **Classify raw SQLite open failures as `store-full`.** `…/🚦️ready/🟦️.ts:80` to `:83` and `storeFullPattern` (`🧬️schema/🔣️.json:47`) match only the EDQUOT/ENOSPC text. Make the SQLite open path (`🔨️modules/⚙️runtime/🗃️sqlite/🟦️.ts:8` to `:20`) carry the errno, so the classifier can see it.
4. **Authorizer closure ceiling.** `…/🔏️authorizers/🧬️schema/🔣️.json:883` and the check at `…/🔏️authorizers/🟦️.ts:126` to `:128`. The m04 closure is one file over (2049 of 2048). Raise the policy or derive it from the measured closure.
5. **Retired-generation path.** `…/📦️dependencies/💾️storage/🟦️.ts:194` and the retirement under `…/⚙️setup/♻️retirement/`. A generation with a different field set should be retired, not end the stage with "Internal error".
6. **Per-try logs.** `…/🚦️ready/🟦️.ts:288` (`ports.mirror?.(`${row.name}-failed`, next.tail)`) overwrites the failure file on every failure, so tries 1 to 36 on m04 are lost. Keep one file per try, or append.
7. **Plan admission on 16 GB.** The `cut smaller` path does not bring the m06 plan under the 6553 MB floor (`🟦️.ts` under `…/🏡️resident/`). Either shrink waves further or derive the floor from measured free memory with the 2048 MB reserve.
8. **Build skew.** The m05 log says "Dependency sync incomplete", the code at `…/📦️dependencies/💾️storage/🟦️.ts:792` says "Store dependency sync incomplete". Compare the deployed digest per machine (`ready.json` digests: m04 `304d5d98…`, m05 `ce0ca918…`, m06 `70ffaf34…`) before concluding.

## 9. Age of the newest file per origin (Q6)

| Origin | Newest file (CET) | Age | Alive? |
|---|---|---|---|
| g1-m01 | `ready.json` 2026-11-17 21:01 | 0 min | yes |
| g1-m02 | `attest.json` 2026-11-17 21:01 | 0 min | yes |
| g1-m03 | `pool.json` 2026-11-17 21:01 | 0 min | yes |
| g1-m04 | `pool.json` 2026-11-16 23:07 | 22 h | no |
| g1-m05 | `pool.json.56492.tmp` 2026-11-17 16:16 | 4 h 45 min | no (last write 16:16) |
| g1-m06 | `ready.json` 2026-11-16 10:41 | 34 h 20 min | no |
| g1-m07 | `attest.json` 2026-11-17 21:01 | 0 min | yes |
| g1-m08 | `attest.json` 2026-11-17 21:01 | 0 min | yes |
| g1-m09 | `fleet-auth.json` 2026-11-14 02:13 | 91 h | never joined |
| g1-m10 | `pool.json.11598.tmp` 2026-11-17 21:01 | 0 min | yes |
| g1-server | `pool.json` 2026-11-17 21:01 | 0 min | yes |

Notes: `pool.json` on m05 is itself stale (2026-11-16 16:42), so m05's puller is not running. The replica cursors of m04, m05 and m06 are the same as the others' (10 origins), so their last cursors are their last replica contact.
