# g1-m01 Sync Check (2026-11-17, 20:01Z to 20:15Z)

Asked for by the account on this machine ("make sure everything is syncing"). Read-only except for the four directory renames below.

## Replica `status` at 20:01Z (exit 0)

- Store `/Volumes/semio-store` reachable, synchronization on, daemon running (pid 61529), 307157 files tracked, 0 incoming queued, 0 events behind every origin.
- Code `4fbc052ae13d`, the same as g1-server. Git state `shared`, stage tree `4898e64694ba0742871ec526b3bae15b477af061`.
- Sidecar `build-agent` running (process 61790); `build-scheduler` and `build-cache` off on this machine.
- Verdict of g1-m01: `lagging (not settled …)`, 52 s old; the daemon had been started 89 s before.
- `launchctl`: `tech.semio.fleet.replica` running (pid 61318, `runs = 1`), `tech.semio.fleet.compact` loaded.

## Finding 1: the daemon crashed and was restarted at about 20:00Z

`~/Library/Logs/semio-fleet-replica.log`: `…/fleet/entry/boot.sh: line 33: 16656 Bus error: 10 "$@"`, then `Replicating … as origin g1-m01 of group g1`, `started`, `applied 24`, `applied 9`, then five `unstable` events on `.git/objects/…` (`a5/de33c8…`, `87/84d9ad…`, `74/d014d9…`, `2b/533d9c…`, `86/51b14d…`). `health.json` counts `loop-blocked-restart` 21 times, the last at about 19:36Z.

## Finding 2: case-only drift of four directories (repaired here)

`verify` at 20:03Z: `Drift on 13 paths (296401 shared paths checked)`: 10 `missing locally` under `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/` and 3 report files of workers that were being written.

Cause of the 10: the fleet state and the git index name the directories `🏛️bestest-600ff` and `🏛️bestest-900ff` (under `📚️examples` and `🖼️assets`); on this machine they were `🏛️bestest-600FF` and `🏛️bestest-900FF`, dated 9 November (the extraction of the bootstrap archive). The volume is case-insensitive APFS, so every read through the lower-case path worked and the daemon saw nothing to do; `verify` compares the listed names exactly. A case-only rename of a directory that happened in the fleet was never applied here.

Repair: one `rename(2)` per directory to the fleet's spelling (four calls, no intermediate name). After the first one the replica published nothing (`g1-m01: last event` kept ageing, 5381 s then 5413 s) and `git status --porcelain` of the folder stayed empty; then the other three. The second `verify` no longer lists any of the 10 paths. Whether other Mac workers carry the same four directories in upper case: not measured (their `verify` would show the same 10 lines).

## Finding 3: the daemon falls behind while the machine is loaded

- Second `verify` at 20:06Z: `Drift on 57 paths`: files of other origins `missing locally` or `content differs` (tickets written in the last minutes) and this folder's own `round-31.part-2.md` `content differs`, so neither applies nor publishes kept up.
- `status` at 20:08Z: g1-m01 `stale (attestation too old)`, 318 s, `6/0` origins behind, `Latency over the last 0 publishes and 9 applies: … apply p50 72003 ms, p95 142079 ms`, `g1-m01: last event 5568s ago`.
- `health.json` was last written at 20:03:58Z (age 638 s at 20:14Z); `state.wal` the same; `mirror-state.json` was written at 20:13:41Z, and a 3 s `sample` shows the process running (11.8 % CPU, 23.5 s CPU time in 14 minutes, `__rename` among the top frames, a git pack open). The daemon works, slowly.
- Load at the same time: `load averages: 51.27 49.31 39.23` (16 GB Mac mini), four `cargo build --locked --manifest-path …/🌎️hub/Cargo.toml -p semio-hub-…` processes running for about an hour (up to 395 % CPU), `ANECompilerService` at 229 %, a `git` process with 2.1 GB resident. The cargo builds have no living parent chain to a session of mine; they were not touched.

## State Of The Setup At 20:08Z

`ready.json`: stage `workspace`, attempts 53, state `backoff`. Attempt 52 had read the graph as far as `taken 4371 of 4455`; the newest `workspace.log` shows `Planning every target of the workspace.`, the first time a run on this machine is past the graph read.

## Later Checks (20:20Z to 20:32Z)

- 20:19Z: the daemon's log shows `store-unreachable … reason: stalled` then `store-reachable` (twice by 20:30Z); `health.json` is fresh again. From this machine the store lists in 0.01 s and `192.168.178.70` answers ping in 0.7 ms, so the stall is local.
- 20:21Z: verdict of g1-m01 `lagging (not settled …)`, `7/0` origins behind, still `0 publishes and 9 applies` since the daemon's start, `g1-m01: last event 6424s ago`.
- Third `verify` (20:30Z): `Drift on 2155 paths (296402 shared paths checked)`: `.git/index` `content differs` and `.git/objects/…` `missing locally` (a commit of the group server arriving), on top of the unapplied ticket files. The four renamed directories are not in the list.
- Load `57.60 44.60 40.82`. The four `cargo build --locked --manifest-path …/🌎️hub/Cargo.toml -p semio-hub-{fem,energy,demonstrator}-component` processes (pids 14615, 15286, 17869, 17870; started 19:08Z to 19:10Z) have parent 1, no child process, 320 to 656 KiB resident each and 24 to 224 minutes of CPU time: they compile nothing and spin. Their environment names another agent session of this machine (`CLAUDE_CODE_HOST_SESSION_ID=local_c8359bff-…`, `SEMIO_BUILD_MODE=release`). Not started by this session and therefore not stopped by it (protocol: kill only what you started); the account was told.
- The daemon runs at nice 0 with 50 s of CPU time in 30 minutes.

## Correction And Outcome (20:33Z to 20:35Z)

**Finding 2 was a wrong diagnosis; the renames were undone.** The fleet state and the git index hold BOTH spellings: `git ls-files` lists 3 paths under each of `🏛️bestest-600FF` and `🏛️bestest-900FF` and 5 under each of `🏛️bestest-600ff` and `🏛️bestest-900ff`. On a case-insensitive volume only one directory can exist, so `verify` always reports 10 paths `missing locally`: with `FF` on disk the 10 `ff` paths (first `verify`), with `ff` on disk the 10 `FF` paths (fourth `verify`, 20:32Z). This is a case collision in the shared tree, not drift of this machine; the fleet counts it as `guards.caseMismatch`. After my rename `status` showed `g1-m01 differs in guards.caseMismatch: 0 instead of 10` for about three minutes. At 20:33Z the four directories were renamed back to `…FF` (again one `rename(2)` each), the spelling this machine had since 9 November; `git status --porcelain` of the folder is empty. At 20:34Z `status` lists `g1-m03` and `g1-server` as `differs in guards.caseMismatch: 10 instead of 0`, so the origins do not agree on this guard among themselves. For the coordinator: the tree needs one spelling (the collision makes `verify` exit 1 on every case-insensitive worker).

**The stalled daemon.** At 20:22Z the four spinning `cargo build` processes (parent 1, no children, under 1 MiB resident, their agent session `CLAUDE_PID=9455` no longer exists) were sent SIGTERM by this session at the request of the account to get the machine syncing; they ended at once. This departs from the rule "kill only processes you started" and is reported here for that reason. Within seconds the daemon went from 0 to about 290 % CPU and caught up:

- `status` 20:25Z: `g1-m01: 0 events behind, last event 7s ago`, `0/0` origins behind; load 14 to 20 from 20:28Z (57 before).
- `status` 20:32Z: `Latency over the last 17 publishes and 100 applies: publish p50 265 ms …; apply p50 1207 ms, p95 5102 ms`; the log shows `publish 1299 of 1299`, then 87, 166, 1, 4. Git state `shared`, stage tree `a0d3c8dbcbb0656ed141aab1b444c9b2efdec829`.
- Fourth `verify` 20:32Z: `Drift on 12 paths (298376 shared paths checked)` (2155 before): the 10 collision paths and 2 files being written by others.
- `status` 20:34Z: verdict of g1-m01 `lagging (not settled …)`, 3 s old, `publish p50 210 ms`, `apply p50 11820 ms`, last event 20 s ago.
