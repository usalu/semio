# g1-m02 Recovery From the 08:41 Crash Loop (2026-11-13)

Result: **daemon running again, synchronisation on, moved itself to code `7757dd83d95b`** (the digest g1-server runs). `fleet --wait 600` exit 1 as expected; this machine's line at the end of the wait:

```
g1-m02 macos 4s 1/0 on 7757dd83d95b 69/16/14 257/1717|925/3654 lagging (not settled)
```

- origin: g1-m02, group g1, store `/Volumes/semio-store` (from `.cache/f/machine.json`)
- account: ueli, macOS, worktree `/Users/Shared/semio`
- No git command, no `pkill`/`killall`, no service stop, unload or re-provisioning. The daemon was never started or stopped by hand.

## Before (20:21:14 CET)

- Store mounted: `192.168.178.70:/Users/Shared/semio-store on /Volumes/semio-store (nfs)`.
- `/Volumes/semio-store/v1/bootstrap/code/229b2300d546/`: 38 regular files, no symlinks, all under `🧰️framework/` (replica module, runtime main and language, schema validator and closure, build `🔣️policy.json`).
- `system/tech.semio.fleet.replica`: `state = spawn scheduled`, `runs = 19962`, `last exit code = 1`; no `.cache/f/daemon.lock`.
- Last error in `/var/log/semio-fleet-replica.log`, repeating:

```
The code of the worktree does not start (Export named 'gitGuardsV1' not found in module '/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🌿️git/🟦️.ts'.)
No replica code starts: neither the worktree nor a sealed snapshot
```

## Copy (20:21:27)

`rsync -a "/Volumes/semio-store/v1/bootstrap/code/229b2300d546/" /Users/Shared/semio/` — exit 0. A checksum dry run afterwards (`rsync -a -n -i -c`) listed 0 differing files.

## Status after 90 s (20:23:23) — exit 0

launchd had started the daemon by itself: pid 72741, `runs = 19966`.

```
Replica g1-m02 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 72741)
274796 files tracked; queued: 0 incoming, ? outgoing; 42 overwritten files kept in .cache/f/lost
Journal: 667.09 MiB in 473 segments of 7 origins.
Newest snapshot by g1-m02: 60798s old, 87.65 MiB.
Reclaimable now: 66 journal segments (94.40 MiB).
Git state: shared (stage and commits replicate); stage tree -.
Background tiers, last pass (cost, duty cycle of one core): hot 2 ms 0.6%, directory sweep 431 ms 10%, file sweep 4461 ms 3.9%, tree walk 26966 ms 1.8%, git view 85314 ms 5%.
Latency over the last 3 publishes and 100 applies: publish p50 20 ms, p95 172 ms; apply p50 201 ms, p95 572 ms.
g1-m01: 0 events (0 B) behind, last event 45215s ago
g1-m02: 0 events (0 B) behind, last event 13s ago
g1-m03: 0 events (0 B) behind, last event 42572s ago
g1-m04: 27 events (5.76 KiB) behind, last event 26139s ago
g1-m05: 0 events (0 B) behind, last event 53927s ago
g1-m06: 0 events (0 B) behind, last event 42174s ago
g1-server: 777 events (210.76 KiB) behind, last event 112s ago
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 darwin 42153s 3/0 on 1c30590cd115 55/0/128 -/-|133/449 stale (attestation too old)
  g1-m02 darwin 42151s 3/0 on 1c30590cd115 54/0/124 20/172|201/572 stale (attestation too old)
  g1-m03 darwin 42153s 3/0 on 1c30590cd115 52/0/122 188/341|244/850 stale (attestation too old)
  g1-m04 linux 9421s 2/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 2550s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 darwin 42151s 3/0 on 1c30590cd115 37/0/83 1906/6816|359/5296 stale (attestation too old)
  g1-server macos 3s 0/0 on 7757dd83d95b 0/0/4 3274/3301|4721/4802 ok
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

## Fleet, first run (20:23:46–20:24:12) — exit 1, aborted

`fleet --wait 600` ended after 26 s (16 `Waiting for agreement` lines) with the single error line:

```
ESTALE: unknown error, read
```

It coincided with the daemon's self-restart: the service log shows `{"event":"restarting"}`, and `.cache/f/daemon.lock` then names pid 73509 (started 20:23:45, `runs = 19967`). The service log also holds one failed start between the two runs of the daemon, on the mixed code during catch-up:

```
TypeError: attester.running is not a function. (In 'attester.running()', 'attester.running' is undefined)
      at <anonymous> (/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🟦️.ts:2264:150)
```

followed by `overwritten` events for `🩺️attest/🟦️.ts`, `🌿️git/🟦️.ts` and `🩺️attest/🧬️schema/🔣️.json` (events of g1-server; the replaced copies are kept under `.cache/f/lost`), then `started` and `{"event":"sync","stage":"entry","result":"boot program 2 installed"}`. No further restart since 20:23:45.

## Fleet, second run (20:24:30–20:34:30) — exit 1 after the full 600 s

Same command, unchanged. Final table (393 `Waiting for agreement` lines omitted):

```
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 3s 1/0 on 7757dd83d95b 84/16/28 -/-|1048/4563 lagging (not settled)
  g1-m02 macos 4s 1/0 on 7757dd83d95b 69/16/14 257/1717|925/3654 lagging (not settled)
  g1-m03 macos 3s 0/0 on 7757dd83d95b 119/16/37 158/740|561/28973 diverged
  g1-m04 linux 10073s 5/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 3202s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 darwin 42802s 6/0 on 1c30590cd115 37/0/83 1906/6816|359/5296 stale (attestation too old)
  g1-server macos 3s 0/0 on 7757dd83d95b 119/16/39 3267/3294|645/1772 ok
  g1-m03 differs in view.digest: e3c0c8d89586 instead of 785d1d7b929b
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

## Status after the fleet run (20:34:30) — exit 0

```
Replica g1-m02 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 73509)
274823 files tracked; queued: 0 incoming, ? outgoing; 42 overwritten files kept in .cache/f/lost
Recurring failures of kind unstable: 28 times, last 85 s ago.
Journal: 667.17 MiB in 473 segments of 7 origins.
Newest snapshot by g1-m02: 61464s old, 87.65 MiB.
Reclaimable now: 66 journal segments (94.40 MiB).
Git state: shared (stage and commits replicate); stage tree ea21979e7393e0c66dcd4858b8066a20d5e5bf7a.
Background tiers, last pass (cost, duty cycle of one core): hot 7 ms 2.3%, directory sweep 551 ms 10%, file sweep 2877 ms 3.8%, tree walk 41394 ms 2.3%, git view 72315 ms 5%.
Latency over the last 44 publishes and 98 applies: publish p50 257 ms, p95 1717 ms; apply p50 925 ms, p95 3654 ms.
g1-m01: 0 events (0 B) behind, last event 461s ago
g1-m02: 0 events (0 B) behind, last event 45s ago
g1-m03: 2 events (558 B) behind, last event 0s ago
g1-m04: 0 events (0 B) behind, last event 26805s ago
g1-m05: 0 events (0 B) behind, last event 54593s ago
g1-m06: 0 events (0 B) behind, last event 12s ago
g1-server: 0 events (0 B) behind, last event 28s ago
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 1s 0/0 on 7757dd83d95b 84/16/28 -/-|1048/4563 ok
  g1-m02 macos 0s 0/0 on 7757dd83d95b 69/16/14 257/1717|1126/2534 diverged
  g1-m03 macos 0s 0/0 on 7757dd83d95b 119/16/37 158/740|561/28973 diverged
  g1-m04 linux 10080s 5/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 3209s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 darwin 42810s 6/0 on 1c30590cd115 37/0/83 1906/6816|359/5296 stale (attestation too old)
  g1-server macos 10s 1/0 on 7757dd83d95b 119/16/39 3267/3294|645/1772 lagging (not settled)
  g1-m02 differs in view.digest: 4f8e5feac38f instead of 1d1a2272cd49
  g1-m03 differs in view.digest: e3c0c8d89586 instead of 1d1a2272cd49
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

## Observations

- g1-m02 attests fresh (4 s) on the server's code digest `7757dd83d95b` but did not settle within 600 s: it stays one origin behind (`1/0`). The status above names the origin and the amount.
- `status` reports `Recurring failures of kind unstable: 28 times`; the log names `.git/objects/pack/pack-303ed9a6176a84d383384e4bcabf914f136f9b53.pack` each time. That path was already reported as unstable on 2026-11-13 01:02.
- Other origins, as seen from here: g1-m03 `diverged` (view.digest), g1-m04 and g1-m06 stale, g1-m05 sync off, g1-m01 lagging like this machine.
- The first `fleet` run failing with a bare `ESTALE` shows the command does not survive a store file being replaced under it.
