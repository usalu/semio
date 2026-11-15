# g1-m02 Replica Upgrade Record (2026-11-13)

Result: **stopped at step 1** — `verify` exited 1 (drift against the store). The running replica was not stopped; steps 3–6 were not run.

- origin: g1-m02 (from `.cache/f/machine.json`)
- account: ueli
- OS: macOS
- worktree: /Users/Shared/semio
- old pid (`.cache/f/daemon.lock`): 20228 (started Thu Nov 12 23:15:59 2026, `bun ./📜️script.ts run`, still running)
- new pid: none (no restart performed)
- service: `system/tech.semio.fleet.replica` (`/Library/LaunchDaemons/tech.semio.fleet.replica.plist`, username ueli, keepalive, state running, pid 20228)

## Step 1

- `🌿️git` and `🩺️attest` exist beside `🔁️replica`: yes
- sha256 of `🔁️replica/🟦️.ts` before verify (00:54:20 CET): `a01f4e42df35f54137ec06cbe81137c84b36b7fc68c67db06510e77834b770f1`
- sha256 of `🔁️replica/🟦️.ts` after verify (00:55:04 CET): `a01f4e42df35f54137ec06cbe81137c84b36b7fc68c67db06510e77834b770f1` (unchanged, equal to the hash g1-m01 recorded after its verify)
- `repo ./📜️script.ts verify --lang en`: **exit 1**

Verify output (stderr complete; stdout held only progress events, omitted):

```
Drift on 35 paths (272384 shared paths checked).
  state differs: .git/compose-metrics-cache.json
  state differs: .git/compose-micro-commit-active
  state differs: .git/compose-micro-commit-bullets
  state differs: .git/compose-micro-commit-digest
  state differs: .git/config
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/index.bin
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/metadata.json
  state differs: .git/cursor/crepe/10864fdee8eb78a6c81aa3a4f7c83c74abfeac51/postings.bin
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/index.bin
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/metadata.json
  state differs: .git/cursor/crepe/277881dca4ea640e85beffcc5527b3d7e3feae94/postings.bin
  state differs: .git/description
  state differs: .git/gkcommittemplate.txt
  state differs: .git/hooks/applypatch-msg.sample
  state differs: .git/hooks/commit-msg.sample
  state differs: .git/hooks/fsmonitor-watchman.sample
  state differs: .git/hooks/post-update.sample
  state differs: .git/hooks/pre-applypatch.sample
  state differs: .git/hooks/pre-commit.sample
  state differs: .git/hooks/pre-merge-commit.sample
  state differs: .git/hooks/pre-push.sample
  state differs: .git/hooks/pre-rebase.sample
  state differs: .git/hooks/pre-receive.sample
  state differs: .git/hooks/prepare-commit-msg.sample
  state differs: .git/hooks/push-to-checkout.sample
  state differs: .git/hooks/sendemail-validate.sample
  state differs: .git/hooks/update.sample
  state differs: .git/index
  state differs: .git/info/refs
  state differs: .git/logs/HEAD
  state differs: .git/logs/refs/heads/⛳wip
  state differs: .git/logs/refs/heads/🐙ueli/⛳wip
  state differs: .git/logs/refs/remotes/origin/HEAD
  state differs: .git/logs/refs/remotes/origin/🐙ueli/⛳wip
  state differs: .git/logs/refs/stash
```

## Observations

- All 35 drift paths are `state differs` under `.git/` (raw `.git/index`, `.git/logs`, `.git/config`, hook samples, tool caches). The contract's Rollout section says old daemons keep publishing these and new daemons drop them, so this drift is the expected signature of an old daemon and will not clear before the restart.
- Unlike g1-m01, there is no `content differs` on the module sources: the worktree copy of the new replica code matches the store.
- `🔁️replica/🟦️.ts` and `🌿️git/🟦️.ts` have mtime 2026-11-13 00:53:16, one minute before verify started; `🩺️attest/🟦️.ts` 00:25:54. The sources were still receiving edits shortly before the run.
- The running daemon (started 2026-11-12 23:15:59) predates those edits. `/var/log/semio-fleet-replica.log` contains no `restarting` event, so it has not self-restarted and is an old daemon in the sense of the Rollout section.

## Steps 2–6

- Step 2: pid 20228 read and noted, replica is running.
- Steps 3–6 (stop, wait for new pid, `status`, `fleet --wait 120`): not run, per the stop condition in step 1. status exit code: n/a. fleet exit code: n/a.

## Open point for the coordinator

Step 1 requires verify exit 0, but the remaining drift is exactly the `.git/` state that only goes away once old daemons are replaced. On this machine the stop condition and the Rollout section contradict each other. Either the step must accept drift confined to `.git/` realm paths, or verify must exclude them; then rerun from step 1.

---

# Continuation after the coordinator answer (2026-11-13, `1113-coordinator-answer-upgrade.md`)

Result: **replica restarted on the new code; `verify` shows no drift; `status` exit 0; `fleet --wait 120` exit 1** because g1-m01 and g1-m03 have no attestation yet. g1-m02 and g1-server are both `ok` on the same code digest. Git state is still `waiting for the base stage`.

- old pid: 20228
- new pid: 29375 (lock `startedMs` 1794528137210, launchd `runs = 2`, `last exit code = 0`)
- attested code digest of the new daemon: `1001ce4e940a` (same as g1-server)

## Source stability (60 s)

Polled every 4 s from 01:01:04 to 01:02:08 CET, unchanged; same values again at 01:08 after all checks:

- `🔁️replica/🟦️.ts`: `a01f4e42df35f54137ec06cbe81137c84b36b7fc68c67db06510e77834b770f1`
- `🌿️git/🟦️.ts`: `196abadcd2a573372ce542bacf180a7fce91e9c1700b8f864b90546abf847108`

## Steps 2–4

- 01:02:14 pid 20228 read from `.cache/f/daemon.lock`, confirmed as `bun ./📜️script.ts run` owned by ueli, `kill -TERM 20228` sent once as ueli.
- 01:02:17 (3 s later) `.cache/f/daemon.lock` names pid 29375; pid 20228 gone. launchd (`system/tech.semio.fleet.replica`, KeepAlive) restarted it. The service log shows `stopped`, `Replica stopped; state saved.`, then `started`.

## First pass, directly after the restart

`status` (01:02:23): **exit 0**

```
Replica g1-m02 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 29375)
272370 files tracked; queued: 0 incoming, ? outgoing; 38 overwritten files kept in .cache/f/lost
Git state: waiting for the base stage of the group server; stage and commit state is not shared on this machine yet.
g1-m01: 1 events (296 B) behind, last event 7s ago
g1-m02: 0 events (0 B) behind, last event 21s ago
g1-m03: 0 events (0 B) behind, last event 30112s ago
g1-server: 0 events (0 B) behind, last event 97s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 - - - - - missing (no attestation)
  g1-m02 - - - - - missing (no attestation)
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 2s 0/0 on 1001ce4e940a ok
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

`verify` (01:02:25): **exit 1** — the same 35 `state differs` paths under `.git/` as before the restart, no other path (272335 shared paths checked). The daemon was 8 s old; its first scan logged `{"event":"scan","count":272335,"total":35}`.

`fleet --wait 120` (01:03:18–01:05:18): **exit 1**. Final table (the `Waiting for agreement: 2 origins pending, N s left.` lines are omitted):

```
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 - - - - - missing (no attestation)
  g1-m02 darwin 1s 1/0 on 1001ce4e940a lagging (not settled)
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 2s 1/0 on 1001ce4e940a lagging (not settled)
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

## Second pass, three minutes after the restart

`verify` (01:05:36): **exit 0**

```
No drift: 272336 shared paths agree with the state and the store.
```

`fleet --wait 120` (01:06:19–01:08:20): **exit 1**. Final table:

```
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 - - - - - missing (no attestation)
  g1-m02 darwin 1s 0/0 on 1001ce4e940a ok
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 1s 0/0 on 1001ce4e940a ok
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

`status` (01:08:20): **exit 0**

```
Replica g1-m02 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 29375)
272336 files tracked; queued: 0 incoming, ? outgoing; 38 overwritten files kept in .cache/f/lost
Git state: waiting for the base stage of the group server; stage and commit state is not shared on this machine yet.
g1-m01: 0 events (0 B) behind, last event 3s ago
g1-m02: 0 events (0 B) behind, last event 377s ago
g1-m03: 0 events (0 B) behind, last event 30468s ago
g1-server: 0 events (0 B) behind, last event 211s ago
Fleet: origin platform age lag (origins behind/queued) sync code verdict
  g1-m01 - - - - - missing (no attestation)
  g1-m02 darwin 3s 0/0 on 1001ce4e940a ok
  g1-m03 - - - - - missing (no attestation)
  g1-server darwin 2s 0/0 on 1001ce4e940a ok
The fleet is not in agreement (4 origins; fresh means within 60 s).
```

## Observations

- The `.git/` drift does not clear at the instant of the restart: `verify` 8 s after start still listed all 35 paths, `verify` three minutes later listed none. A verify directly after the restart is too early.
- `fleet` fails only on g1-m01 and g1-m03 (`missing (no attestation)`), which still run old daemons; g1-m03's last event is about 8.5 h old. During the first wait g1-m02 and g1-server were `lagging (not settled)` with 1 origin behind each; in the second pass both are `ok` with lag 0/0. The fleet cannot agree before those two origins are upgraded.
- `status` does not print `Git state: shared` yet; six minutes after the restart it still prints `waiting for the base stage of the group server`. Before the restart, the old daemon logged the server's base stage as dropped: `{"event":"refused","path":".cache/f/index-base","reason":"ignored","hlc":"1794527144997-000000-g1-server"}`. The new daemon's log has no line about the base stage. Whether the new daemon picks up that already-consumed event or needs the server to publish the base stage again is open and is for the coordinator to check.
- The new daemon logs `refused … reason "ignored"` for raw `.git/index` and `.git/logs/…` events from g1-m01, as the Rollout section predicts for an old origin.
- No `restarting` event in the log: this restart was the manual one; self-restart was not exercised.
