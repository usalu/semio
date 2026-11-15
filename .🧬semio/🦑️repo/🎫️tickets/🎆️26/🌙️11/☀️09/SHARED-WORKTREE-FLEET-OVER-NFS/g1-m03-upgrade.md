# g1-m03 Replica Upgrade (2026-11-13)

- origin: g1-m03
- account: ueli
- OS: macOS 27.2.1 (worktree `/Users/Shared/semio`, store `/Volumes/semio-store`)
- service: launchd `system/tech.semio.fleet.replica` (LaunchDaemon, running as `ueli`)

## Step 1: new module code and verify

- `🌿️git` and `🩺️attest` exist beside `🔁️replica`: yes.
- sha256 `🔁️replica/🟦️.ts`: `a01f4e42df35f54137ec06cbe81137c84b36b7fc68c67db06510e77834b770f1` (unchanged between 00:16:45Z and 00:26:51Z).
- sha256 `🌿️git/🟦️.ts`: `c06aa5b73276a4fc3d067354a29281e719104749106289fd535e6b8de1a9b3b9`.
- `repo ./📜️script.ts verify --lang en` (old daemon still running): **exit 1**.

First run, 00:16Z:

```
Drift on 106 paths (272314 shared paths checked).
  missing locally: .git/HEAD
  missing locally: .git/objects/01/28a62dc9f66592d8b5d9a7b06da456e58d16b4
  missing locally: .git/objects/06/3041dc6115332ff9d7b2dcf043522f5783ff27
  ... (all listed paths under .git/objects)
  ... and 56 more
```

Second run with `--json`, 00:26Z: exit 1, `{"version":1,"ok":false,"checked":272314,"total":102,"drift":102,"truncated":false,"refusedTotal":0,"refused":0}`. Drift grouped by location, every entry `missing-local`:

| Count | Location |
| --- | --- |
| 1 | `.git/HEAD` |
| 95 | `.git/objects/**` |
| 1 | `.git/packed-refs` |
| 5 | `.git/refs/**` |

No drift outside `.git/`. `/Users/Shared/semio/.git` does not exist on this worker at all, so the old daemon never materialised shared git state here. This differs from the signature described in `1113-coordinator-answer-upgrade.md` (index, logs, config, hooks present but differing); here the whole directory is absent.

## Step 2: running replica

- `.cache/f/daemon.lock`: `{"version":1,"pid":13176,"startedMs":1794451558355,"originId":"g1-m03"}`
- old pid: **13176** (`bun ./📜️script.ts run`, user `ueli`, ppid 1, started 2026-11-12 03:45:58 local).

## Step 3: stop the old replica

The dev confirmed in chat to continue despite the pre-restart verify exit 1 (drift confined to `.git/`).

- 00:28:18Z `kill -TERM 13176` as `ueli`, once, exit 0. No `pkill`, no `killall`; relay, cache and compaction untouched.
- Log: `{"version":1,"event":"stopped"}`, `Replica stopped; state saved.`

## Step 4: new pid

- launchd restarted the service within 1 s: `.cache/f/daemon.lock` = `{"version":1,"pid":52181,"startedMs":1794529698654,"originId":"g1-m03"}`.
- new pid: **52181**.
- About 50 s later the new code restarted itself once because the module sources changed on disk (fleet code digest `1ab5cb760b01` to `a6bd6778a398`): log `{"version":1,"event":"restarting"}`, `The code on disk changed; state saved, the service manager starts the new code.` Lock afterwards: `{"version":1,"pid":52237,"startedMs":1794529749953,"originId":"g1-m03"}`. Current pid: **52237**. This confirms the Self-restart section works under launchd on this machine.

## Step 5: status and verify

`repo ./📜️script.ts status --lang en` at 00:28:25Z (pid 52181): **exit 0**

```
Replica g1-m03 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 52181)
272315 files tracked; queued: 0 incoming, ? outgoing; 89 overwritten files kept in .cache/f/lost
Git state: not shared on this machine, because the worktree has no usable root .git.
g1-m01: 0 events (0 B) behind, last event 947s ago
g1-m02: 0 events (0 B) behind, last event 349s ago
g1-m03: 0 events (0 B) behind, last event 79s ago
g1-server: 0 events (0 B) behind, last event 166s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 2s 0/0 on 1ab5cb760b01 ok
  g1-m02 darwin 0s 0/0 on 1ab5cb760b01 ok
  g1-m03 - - - - - missing (no attestation)
  g1-m04 linux 2s 0/0 on 1ab5cb760b01 ok
  g1-server darwin 2s 0/0 on 1ab5cb760b01 ok
The fleet is not in agreement (5 origins; fresh means within 60 s).
```

`status` again at 00:31:08Z (pid 52237): **exit 0**

```
Replica g1-m03 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 52237)
272363 files tracked; queued: 0 incoming, ? outgoing; 89 overwritten files kept in .cache/f/lost
Git state: not shared on this machine, because the worktree has no usable root .git.
g1-m01: 0 events (0 B) behind, last event 1108s ago
g1-m02: 0 events (0 B) behind, last event 510s ago
g1-m03: 0 events (0 B) behind, last event 240s ago
g1-server: 0 events (0 B) behind, last event 4s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 2s 0/0 on a6bd6778a398 ok
  g1-m02 darwin 3s 0/0 on a6bd6778a398 ok
  g1-m03 darwin 4s 0/0 on a6bd6778a398 diverged
  g1-m04 linux 4s 0/0 on a6bd6778a398 ok
  g1-server darwin 3s 0/0 on a6bd6778a398 ok
  g1-m03 differs in manifest: 5b5d0403cc58 instead of 6aca1b1f7466
  g1-m03 differs in head.commit: - instead of 5ececaa78b92
  g1-m03 differs in indexTree: - instead of 6bc6e1d07900
The fleet is not in agreement (5 origins; fresh means within 60 s).
```

**Requirement not met:** status must print `Git state: shared`; it prints `Git state: not shared on this machine, because the worktree has no usable root .git.`

`repo ./📜️script.ts verify --lang en` after the restart, 00:30:41Z: **exit 1**

```
Drift on 104 paths (272363 shared paths checked).
  missing locally: .git/HEAD
  missing locally: .git/objects/01/28a62dc9f66592d8b5d9a7b06da456e58d16b4
  ... (49 listed paths under .git/objects)
  ... and 54 more
```

## Step 6: fleet

`repo ./📜️script.ts fleet --wait 120 --lang en`, 00:28:26Z to 00:30:27Z: **exit 1**. The command is known. It printed `Waiting for agreement: 1 origins pending, <n> s left.` once per second (2 origins pending for the last 23 s, when g1-server went stale), then:

```
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 0s 0/0 on a6bd6778a398 ok
  g1-m02 darwin 6s 0/0 on a6bd6778a398 ok
  g1-m03 darwin 1s 0/0 on a6bd6778a398 diverged
  g1-m04 linux 1s 0/0 on a6bd6778a398 ok
  g1-server darwin 84s 0/0 on a6bd6778a398 stale (attestation too old)
  g1-m03 differs in manifest: 6cff440d55d6 instead of 5d685b11ede8
  g1-m03 differs in head.commit: - instead of 5ececaa78b92
  g1-m03 differs in indexTree: - instead of 6bc6e1d07900
The fleet is not in agreement (5 origins; fresh means within 60 s).
```

## Result

| Item | Value |
| --- | --- |
| old pid | 13176 |
| new pid | 52181, then 52237 after one self-restart |
| sha256 `🔁️replica/🟦️.ts` | `a01f4e42df35f54137ec06cbe81137c84b36b7fc68c67db06510e77834b770f1` (same before and after) |
| verify before restart | exit 1, 102 to 106 paths, all `missing locally` under `.git/` |
| verify after restart | exit 1, 104 paths, same shape |
| status | exit 0, `Git state: not shared` |
| fleet --wait 120 | exit 1, g1-m03 `diverged` |

The replica on g1-m03 runs the new code (code digest `a6bd6778a398`, equal to every other origin, lag 0/0). The upgrade is **not complete**: g1-m03 has no root `.git` in `/Users/Shared/semio`, so the git realm is skipped, `head.commit` and `indexTree` are empty in its attestation and the allow-listed `.git` files (`HEAD`, `packed-refs`, `refs/**`, `objects/**`) are not materialised.

## Open fault for the coordinator

- `/Users/Shared/semio/.git` has never existed on this worker. `git` itself is present (`/usr/bin/git`, 2.54.0).
- Observed: the old daemon logged `{"event":"refused","path":".git/objects/...","reason":"ignored"}` for the shared `.git` events of g1-m02 and g1-server up to the moment it stopped; the new daemon reports 0 incoming queued and every origin 0 events behind, yet the files are still missing.
- Interpretation (worker's, not verified in the code): the old daemon consumed those events past its cursor, so the new code does not see them again, and without a root `.git` it has nothing to anchor the realm to. This is the worktree-file analogue of the dropped base register described in `1113-rollout.md`.
- Nothing was done to work around it: no `git init`, no copy of `.git` from the store, no cursor or state edit, no re-join. The worker rules forbid git commands that change `.git`, and the prompt has no step for a worker without `.git`. A decision is needed on how such a worker obtains its root `.git` (for example a re-join, or a replica operation that re-materialises the allow-listed `.git` paths from the manifest).

## Repair attempt (coordinator decision, 2026-11-13 00:50Z)

Preconditions, checked at 00:50:18Z:

- `🔁️replica/⌨️command/🟦️.ts` contains `repair` (`repair:{valued:["--worktree","--lang"],plain:[]}`), sources written 01:48:11 local.
- sha256 `🔁️replica/🟦️.ts`: `3f5d760eabfda9857933715a9a2addbd71ff776fd5f4bee13b1195bb64e7f7e9`; `⌨️command/🟦️.ts`: `a6cf8f380cb186dcac5e56a8231d547856b612dcf4e6fd622540c3f10d9d5511`.
- The replica restarted itself on that code: `restarting` event in the log, `.cache/f/daemon.lock` = `{"version":1,"pid":53680,"startedMs":1794530899907,"originId":"g1-m03"}` (started 01:48:19 local, after the sources).
- `/Users/Shared/semio/.git` now exists but holds only `objects/` (33 loose objects that arrived as new events since the new code runs). No `HEAD`, `refs`, `packed-refs`.

`repo ./📜️script.ts repair --lang en` at 00:50:27Z: **exit 0**

```
The replica of /Users/Shared/semio re-creates every shared file that is missing on this machine on its next pass. A local deletion that was not published yet is undone.
```

Replica log afterwards (pid 53680):

```
{"version":1,"event":"scan","count":272349,"total":0}
{"version":1,"event":"repaired","count":0}
{"version":1,"event":"scan","count":272349,"total":0}
```

**The repair queued nothing: `repaired` count 0 instead of about 100.** The request file `.cache/f/repair` was consumed. Waited 5 minutes (until 00:55:28Z): `/Users/Shared/semio/.git/HEAD` did not appear.

`status` at 00:55:28Z, exit 0 (first lines):

```
Replica g1-m03 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 53680)
272349 files tracked; queued: 13 incoming, ? outgoing; 89 overwritten files kept in .cache/f/lost
Git state: not shared on this machine, because the worktree has no usable root .git.
```

`verify --json` at 00:55:38Z: **exit 1**, `{"version":1,"ok":false,"checked":272349,"total":102,"drift":102,"truncated":false,"refusedTotal":0,"refused":0}`, all `missing-local`:

| Count | Location |
| --- | --- |
| 1 | `.git/HEAD` |
| 95 | `.git/objects/**` |
| 1 | `.git/packed-refs` |
| 5 | `.git/refs/**` (`heads/⛳wip`, `heads/🐙ueli/⛳wip`, `remotes/origin/HEAD`, `remotes/origin/🐙ueli/⛳wip`, `stash`) |

`fleet --wait 300` was not run: its preconditions (`.git/HEAD` present, `Git state: shared`, verify exit 0) are not met. Stopped here as instructed; no `git init`, no manual copy of `.git`, no state edit.

### Why the repair found nothing (worker's reading of `🔁️replica/🟦️.ts`, not proven by a test)

- `repairV1` walks `replica.manifest.ids()`, the manifest the running daemon holds locally, and queues an entry only when it is in that manifest, shared, not a delete, not queued and absent on disk.
- `.cache/f/state.json` on this machine has 272960 file entries, of which only 33 are under `.git/` (exactly the 33 objects on disk) and none is `.git/HEAD`, `.git/packed-refs` or `.git/refs/**`. The queue has 13 entries, none under `.git/`.
- So the 102 missing paths are not in the local manifest at all: the old daemon refused those events as `ignored` and advanced its cursor without folding them. `verify` sees them because it folds the manifest from the store; `repair` does not because it only looks at the local one.
- Consequence: `repair` has to compare against the store fold (the same fold `verify` uses), or the cursors of this origin have to be rewound so the dropped events are read again. Either is a code or state change outside this worker's instructions.

Current state of g1-m03: replica running on the new code (pid 53680), worktree files in sync apart from the 102 `.git` paths, git realm skipped.

## Second repair (fixed `repairV1`, 2026-11-13 01:05Z)

Preconditions, checked at 01:05:25Z:

- `repairV1` in `🔁️replica/🟦️.ts` contains `foldStoreV1(settings.storeRoot,signal)).manifest.entries()` (1 occurrence). File written 02:01:22 local, sha256 `94220d2b669d6bba7773e465efa95d64b6d441d6fea20d86efb1e4b7fdb9ce7e`.
- `.cache/f/daemon.lock` = `{"version":1,"pid":54735,"startedMs":1794531690563,"originId":"g1-m03"}`, process started 02:01:30 local, 8 s after the file (self-restart, `restarting` event in the log).

`repo ./📜️script.ts repair --lang en` at 01:05:34Z: **exit 0**

```
The replica of /Users/Shared/semio re-creates every shared file that is missing on this machine on its next pass. A local deletion that was not published yet is undone.
```

Replica log: `{"version":1,"event":"repaired","count":244}`.

- `/Users/Shared/semio/.git/HEAD` existed by 01:06:36Z (about one minute after the request). `.git` now holds `HEAD`, `config`, `index`, `objects`, `packed-refs`, `refs`.
- Incoming queue 0 at 01:15:34Z. The pid stayed 54735 throughout.
- Status never showed `waiting for the base stage` when checked.

`repo ./📜️script.ts status --lang en` at 01:15:34Z: **exit 0**

```
Replica g1-m03 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 54735)
272672 files tracked; queued: 0 incoming, ? outgoing; 89 overwritten files kept in .cache/f/lost
Git state: shared (stage and commits replicate); stage tree a96407003a9b32b13494442076db770567c0d14e.
g1-m01: 0 events (0 B) behind, last event 17s ago
g1-m02: 0 events (0 B) behind, last event 21s ago
g1-m03: 0 events (0 B) behind, last event 21s ago
g1-m04: 0 events (0 B) behind, last event 21s ago
g1-server: 0 events (0 B) behind, last event 22s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 2s 0/0 on 04c8c5e990bb ok
  g1-m02 darwin 1s 0/0 on 04c8c5e990bb ok
  g1-m03 darwin 2s 0/0 on 04c8c5e990bb ok
  g1-m04 linux 2s 0/0 on 04c8c5e990bb ok
  g1-server darwin 2s 0/0 on 04c8c5e990bb ok
All 5 origins with sync on are fresh (within 60 s), settled and identical.
```

`repo ./📜️script.ts verify --lang en`, finished 01:16:07Z: **exit 0**

```
No drift: 272606 shared paths agree with the state and the store.
```

`repo ./📜️script.ts fleet --wait 300 --lang en` at 01:16:13Z: **exit 0**, agreement at once (no waiting lines)

```
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 darwin 0s 0/0 on 04c8c5e990bb ok
  g1-m02 darwin 1s 0/0 on 04c8c5e990bb ok
  g1-m03 darwin 4s 0/0 on 04c8c5e990bb ok
  g1-m04 linux 3s 0/0 on 04c8c5e990bb ok
  g1-server darwin 4s 0/0 on 04c8c5e990bb ok
All 5 origins with sync on are fresh (within 60 s), settled and identical.
```

## Final result

| Item | Value |
| --- | --- |
| old pid | 13176 |
| current pid | 54735 (after the manual stop and several self-restarts on new code) |
| repair | exit 0, `repaired` count 244 |
| status | exit 0, `Git state: shared` |
| verify | exit 0, no drift on 272606 paths |
| fleet --wait 300 | exit 0, all 5 origins `ok` on code `04c8c5e990bb` |

The upgrade of g1-m03 is complete. No `git init`, no manual copy of `.git`, no state or cursor edit was made.
