# 🔁 Fleet Replica Sync Health on g1-m05 — Report

Host: Windows workstation, fleet origin `g1-m05`, worktree `C:\semio`, store `S:\` = NFS `\\192.168.178.70\Users\Shared\semio-store`.
Times are local (UTC+1) unless marked Z.

## Verdict

**Still broken at close.** It was down for about 15 h (no publish since 08:53, no inbound since about 12:15). Two replica defects are fixed: below-normal priority and the stuck `attest.json`. It still can't recover while the host thrashes on memory from 3 × 10–11 GB `link.exe` (see "State at close").

## Findings at start (2026-11-22 23:29–23:35)

| Check | Observed | Expected |
|---|---|---|
| Scheduled task `semio-fleet-replica` | `Running`, `LastTaskResult 0x800710E0`, Priority **7** (below normal), action `wscript quiet.vbs` | running |
| `.cache\f\boot\beat.json` | **missing** (no daemon alive between restarts) | seconds old |
| `.cache\f\health.json` | written by pid 70756 at 18:25:24Z, `"store":"stalled"`, failures `loop-blocked-restart ×52`, `loop-blocked ×11`, `attest ×3` | seconds old |
| `S:\v1\origins\g1-m05\head.json` | segment 3, last publish **2026-11-22 08:53:31** | moving |
| `S:\v1\origins\g1-m05\attest.json` | last write **05:06:36**, every rewrite `EPERM` (209 errors in `run.log`) | ~seconds |
| `.cache\f\state.wal` / `state.ckp` | last persisted **12:15:43** / 08:23:55 | recent |
| Other origins' heads | g1-m01/02/03/06/07/08/10/11/12/13/server written within seconds (23:34); g1-m04 since 15:10; g1-m09 since 2026-11-14 | — |
| Inbound | `GET-STAFFAGE-…/plants-b.md` (g1-m03, hlc 23:06Z) missing locally; `…/🎚️budget/🟦️.ts` local sha `02e3322b…` ≠ fleet base `722c0a05…` | present |

`run.log` (`%LOCALAPPDATA%\semio\fleet\entry\run.log`) shows a restart loop: `The daemon was ended (the watchdog of the daemon fell silent)` → exit 137 → sealed snapshots tried → `No replica code starts, trying again in 300 s`. `loop-blocked-restart.json` count 54. Also 8× `ENOSPC` while C: was full, 10× `Store call refused: an earlier store call did not return within its deadline`, 4× `schtasks /Create failed for semio-fleet-replica` (self-update of the task definition, needs elevation).

## Root causes

1. **CPU starvation + below-normal priority → watchdog kill loop.** The host was at 100 % CPU on 16 cores (46 `rustc`, 22 `cargo` of the FEM ticket, Defender), 3 000 pages/s paging, 6 GB free RAM. The installed task runs the replica at Task Scheduler priority 7 (BelowNormal; verified `Get-Process … PriorityClass`). The daemon's event loop (lag up to 26 s) and even its beat thread were starved; the boot stage kills a daemon whose beat is silent for 30 s, so no daemon completed a round, published or persisted state. The source already declares the `replica` service class `interactive` (task priority 3, `🛰️sidecar/🪜️priority/🧬️schema/🔣️.json`) and a pending definition `service-definition\semio-fleet-replica.xml` (Priority 3, `conhost.exe --headless`) exists since 16:30, but it cannot be registered from a non-elevated process, and nothing applied the class at runtime.
2. **`attest.json` stuck on the NFS mount.** Renaming any temp file onto `S:\v1\origins\g1-m05\attest.json` failed with `EPERM` while renames onto `head.json`, onto a fresh file in the same folder and onto a byte copy of `attest.json` in `S:\v1\tmp` all succeeded and `open(r+)` of the file worked — a stale file node of the Windows NFS client. Moving it aside cleared it (the daemon wrote a fresh one at 00:03:14 and again at 00:06:21; rename-over verified ok after).
3. NFS latency from this host is high: 200–430 ms per stat/read, ~1 s per origin `readdir` (origin folders hold up to ~690 entries, many stale `*.tmp`), ping 17–19 ms. This amplifies 1.

## Fixes (worktree code, picked up by the boot stage's next worktree attempt)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🪜️priority/🟦️.ts`: new `applyServiceUnitPriorityV1(service)`; role and service share one `applyServicePriorityClassV1` core.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts`: `applyReplicaPriorityV1` — the boot stage and the daemon child of `run` put themselves into the declared `replica` class (dynamic import, never throws, logs `replica scheduling: …`). Independent of how the service manager started it.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/📒️journal/🟦️.ts`: `replaceFile` (own-origin files such as `attest.json`, `head.json`) moves a target that keeps refusing replacement (`EPERM`/`EACCES`) into `tmp`, renames again and removes the displaced copy.
- Tests: `…/🛰️sidecar/🧪️tests/🪜️priority/🟦️.ts` (+ service apply with injected ports; + real Windows child process at BelowNormal checked with third-party `Get-Process` → `AboveNormal`): 11 pass, 1 skip, 1 fail = pre-existing python oracle (`jsonschema` not installed in bare `python3`). `…/📒️journal/🧪️tests/🟦️.ts` (+ read-only origin file replaced, nothing left in tmp; confirmed Windows returns `EPERM` for this without the fix): 130 pass, 2 fail = pre-existing timing assertions under 100 % CPU (`multi-segment append`, `big document streams` 5 s timeout), unrelated files.

## Evidence after the fix

- 00:01:40 boot stage starts the **worktree** code: `replica scheduling: class interactive, nice -5, applied`; daemon pid 54484 `PriorityClass AboveNormal` (its parents 70296/4720 stay BelowNormal — they only supervise).
- Beat age 00:02:46–00:05:41 always < 1.3 s; lag after start 15 s → < 200 ms; a 70 s block in `follow` at 00:12 was survived (beat kept going); `health.json` written again at 00:12:06 (first time since 18:25Z).
- No reference to `C:\t` in replica source or `run.log`; the replica does not touch `C:\t`.
- Note: `head.json` mtimes 23:43:37 and 00:06:43 come from my identical-content rename probes, not from publication (content still segment 3).

## State at close (2026-11-23 00:31) — STILL BROKEN, cause outside the replica

- 00:17:46 daemon 54484 (AboveNormal, worktree code) killed: beat silent > 30 s during `follow` (longest loop block 190 s). 00:22 daemon 60336 killed the same way; at 00:31 the boot stage is again in its retry back-off.
- Why: the host is **memory-thrashing**, not only CPU-bound. Three `link.exe` of the FEM build held 10–11 GB each (26–36 GB together during 00:22–00:29), available RAM 1.0–4.7 GB, 20 858 pages/s. A paged-out process (even its 1 s beat thread) stalls for tens of seconds; priority cannot help with that.
- Effect: `head.json` still segment 3 (published 2026-11-22 08:53:31), `state.wal` still 12:15:43, `health.json` last real write 00:12:06 (later mtimes are the watchdog's loop-blocked counter). The probe file written into this ticket folder at 00:09 was not published; `plants-b.md` of g1-m03 did not arrive. The replica never persists during this catch-up, so every restart starts the same backlog again.
- Expected recovery: as soon as the linkers finish (or run with less parallelism), the next start runs at AboveNormal with the fixes above and should finish `follow`, persist and publish within minutes. Nothing needs restarting by hand; do not kill processes by image name.

Trace evidence (from the deleted `🗑️generated` traces): 00:02:46–00:05:41 beat age ≤ 1 228 ms, lag 37–196 ms; 00:12:00 lag 52 682 ms, 00:14:31 lag 122 342 ms (`follow`); 00:22:24 pid 60336 lag 46 228 ms, then no daemon; `link` GB 33/34/35/36/36/34/33/32/34/36/36/26/27/28/28/29 at ~30 s steps 00:22:24–00:29:47.

## Session 2 (2026-11-23 01:00–) — catch-up without losing local work

### Safety net first
- `C:\b\semio-local-20261123\` holds a sha256-manifested copy of all 1 321 files that differed locally from the fleet before any inbound file was applied; `C:\b\semio-local-20261123-lost\` the `.cache\f\lost` state at that time. Neither lives in the worktree, so the replica never touches them.

### What makes local work "lost" here
The replica resolves a conflict per path by last writer: a local file that differs from the replica's known version loses to an inbound event unless it was written after that event and within 30 s of it (`conflictWinnerV1`). While a machine is 129 k events behind, every local edit made since the stall is "an edit on a view without the newer fleet content", so the fleet version is materialised and the local bytes go to `.cache\f\lost\<hlc>\<path>` (`overwritten` + `stale-base` events). Nothing is deleted, but the work is no longer in the tree. A merge written back while later events for the same path are still pending loses again, so merging only sticks after the backlog for that path is drained.

### Reconciling lost copies (`reconcile-lost.mjs`)
For every lost copy since the stall: identical to current → nothing to do; its sha in the fleet history → already published, nothing lost; superseded by a later lost copy of the same path → skip; otherwise unique local work → three-way merge (`git merge-file --diff3` on temp copies in `C:\b\reconcile`, never on the repo) of the lost bytes onto the current fleet bytes with the nearest fleet version as ancestor, written by compare-and-swap (the target must still hold the bytes the merge was computed against). Conflicts are resolved by hand.

### Bringing modules ahead of the ordered catch-up (`prefetch-latest.mjs`)
Peers (FEM session) were blocked because half-applied module sets broke `repo` graph reads (`🕸️graph/🧾️registry-reads` missing, `planFleetJobsV1` missing, `🚀️offload/🧰️command` and `🗃️unit-cache/🏗️buildscript/📦️run-wire` missing). By the replica's own rules it is safe to place the latest fleet bytes ahead of the ordered apply: `materialiseWork` treats identical content as `same` → `applied`, nothing goes to `lost`. The script takes the newest event per path across all origins, verifies blob sha256 and size, and writes only when the local file is missing, or holds an unmodified earlier fleet version (sha in the fleet history, or older than the last publish 2026-11-22 08:53). Local edits are left untouched; temp files are staged outside the worktree (`C:\b\prefetch`) so the watcher never sees them. Applied to `🕸️graph/🧾️registry-reads/**`, `🏃️process/🏗️build/🎟️capacity/🧮️units/**`, `🏃️process/📦️artifacts/🏗️native-build/💾️storage/**`, `🚀️offload/**` and finally all of `🧰️framework/🛍️products/🦑️repo/**` (32 written, 135 refreshed, 1 915 identical, 10 local edits untouched, 48 fleet deletions with no local file).

### Own fixes overwritten by the catch-up, merged again
- `🛰️sidecar/🪜️priority/🟦️.ts` and its test, `🤝️shared-worktree/📒️journal/🟦️.ts`: the newer g1-server versions arrived and moved my edits to `lost`; the daemon then logged `replica scheduling: not applied (module.applyServiceUnitPriorityV1 is not a function)`. Merged onto the fleet versions (test: import line conflict resolved by hand, `shellPriorityLinesV1` from the fleet + `applyServiceUnitPriorityV1`). Tests: priority 11 pass / 1 skip / 1 fail (python `jsonschema` missing, pre-existing); journal 131 pass / 1 fail (`multi-segment append`, pre-existing, unrelated).
- `🕸️graph/📥️load/♻️pass-reuse/🟦️.ts`: first merged (`complete?`/`trustSurvivors`), then reverted to the fleet design (`distrust()` on reuse, `evictReferencing(moved)` at the boundary), because the local `complete` flow depended on a `load.ts` path the fleet had replaced.

### Wrong merge base — found and corrected (`verify-merges.mjs`)
The first reconcile used the *nearest fleet version* of a path as merge ancestor. That is wrong: when the nearest version equals the current one, the three-way merge just puts the local file back and **reverts later fleet work**. The true base of a local edit is what this machine saw when the edit was made. The replica froze at 2026-11-22 11:15:43Z, so the base is the latest fleet `put` with hlc ≤ min(mtime, 11:15:43Z), or the git commit before that time. `verify-merges.mjs` recomputed every merge against that base (`VIEW_FROZEN_AT`):
- 28 merges and conflicts had already used the right base (`ancestor-ok`).
- 13 files had fleet work reverted. Each was restored to the exact fleet bytes by compare-and-swap, then re-merged against the frozen-view base where local work remained:
  - `🎨️visuals/🐍️.py`: the fleet version plus the local `reference_chip`, which `solar-radiation` uses.
  - `📦️original/🦀️.rs`: `release_bytes:VcsArtifactApp::<A,M>::heap_bytes()`.
  - `🔏️authorizers/🟦️.ts` and `🧬️schema/🔣️.json`: the local superset (`SourceMembershipRetry`, the `walkNxAuthorizerSourcesV1` attempt, `probe`).
  - `🥾️prerequisite/🧪️tests/🟦️.ts`: the union of the local and fleet tests; 14 pass.
- The remaining re-conflicts were decided file by file. Where the fleet had re-implemented the change, or the local edit relied on APIs the fleet had removed, the fleet version stays.

### Local work intentionally left only in `.cache\f\lost` (superseded)
- FEM: the 3D results `source_label` caption, the `encode_result_source_parts` test, and the `Fem2dSnapshotRetiring` / `Fem3dSnapshotRetiring` tests.
- Trinity/Jack:
  - `jack_self_funded_grant` / `JACK_SELF_FUNDED_BODY_BYTES` (the fleet uses `semio_plugin::plugin_self_funded_grant`);
  - the document-retirement tests;
  - `[DEBUG]` lines.
- Puzzle testkit API tests: fill, gesture-kinds, host-admission.
- Storage `privatizeJavascriptDependencyPatchedCacheV1`: it imports the nonexistent `patchedDependencyKeysV1` and would drop `holdDependencyGenerationsV1` and two other fleet helpers. The local helper `🗄️cas/🧲️adopt/🔒️private/🟦️.ts` is still in the tree.
- `📬️finalize`: the `[DEBUG] hr3` line and the old `commit_failure` approach.
- `🏁️replay`: the local version would remove the fleet's `source_effected`/`effect_turn` split.
- The taken-memo `verified` variant.
- `🧳️frame` `success_outcome_extent`: the fleet has it in `✅️success`.
- Full copies stay in `C:\b\semio-local-20261123\`. The slots in `C:\b\reconcile\<slot>\` hold local / ancestor / base / current / merged / remerged.

### Outcome (2026-11-23 05:50)
- Catch-up drained: 0 origins behind, 10 incoming. `state.wal` and `health.json` are fresh; the daemon (pid 47000) runs at AboveNormal.
- `published-check.mjs`: for all 31 merged or restored files, the fleet's latest event matches the local bytes (`published`).
- One new `lost` entry after 05:00 (`🧳️frame/🦀️.rs`, local edit at 05:18 vs. g1-m13 at 05:27). This is an ordinary concurrent edit by another agent; the later fleet edit won, by design.

### Remaining host condition
- 03:00–04:30 NFS from this host: stat ≈ 1 s, single reads up to 15 s, origin `readdir` > 100 s; the daemon's 10 s store deadline flips the store guard between `stalled` and `reachable` and costs rounds, but the backlog keeps falling (129 k → 77.7 k 03:10 → 28 k 04:06 → 11.9 k 04:30).
- The replica reloads whenever its own module changes on disk (inbound code updates) — expected during catch-up, stops when the module is current.

## Follow-ups (not done here)

- If lost-copy reconciliation ever becomes a replica feature, it must use the frozen-view base (the last fleet event applied before the edit), never the nearest fleet version.

- Re-register `semio-fleet-replica` from an **elevated** shell with the pending definition (`conhost.exe --headless`, Priority 3); the runtime fix makes this non-urgent.
- `follow` still blocks the event loop synchronously for up to 70 s on a slow store; a starvation-tolerant watchdog and chunked follow would remove the remaining kill risk.
- Stale `*.tmp` debris in `S:\v1\origins\*` (e.g. ~70 `pool.json.*.tmp` from 2026-11-16/17 in g1-m05) and `.cache\f\boot\beat.json.*.tmp` slows NFS listings.

## Session 3 (2026-11-23 12:58–13:25) — full verification

- **State found:** the daemon had been killed by its watchdog and restarted about 60 times between 10:13 and 12:45 ("the watchdog of the daemon fell silent", exit 137). In that window this origin published nothing; inbound events were still applied. Every start re-hashed the git packs (`pack-2678b….pack` 6.3 GB, `pack-d5546….pack` 926 MB: their mtimes change, the content does not), and several kills landed during that hash or during `store-unreachable: stalled` phases.
- **Since 12:45** one daemon (pid 60500) has run without a kill; lag 70–160 ms.
- **Proof of sync:** `repo …/🔁️replica/📜️script.ts verify --json` (re-hashes the whole shared tree and compares it with the local state and the folded store) checked 439,538 paths: 439,517 identical, 21 drifted. All 21 had been modified during the 20-minute verify run (mtimes 12:01–12:20Z); `published-check.mjs` right after shows all 21 equal to the fleet's latest event.
- **Local work set aside since 05:50:** 2 files in `.cache\f\lost`.
  - `🏪️store/⚙️engine/🧳️bundle/🦀️.rs`: merged by the new apply merge (event `merged`); the file has moved on since.
  - `📦️budget/🧪️tests/🟦️.ts`: its one extra local line is present in the current file.
- **Not fixed:** the kill loop can come back. Git refreshes the mtime of a pack when it rewrites an object the pack already holds, so the replica re-hashes gigabytes after every start. Follow-up: skip the content hash of a `.pack` whose size and `.idx` are unchanged, or keep local-only packs out of the publish walk before hashing.
