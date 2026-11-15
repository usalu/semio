# g1-m03 Recovery From the 08:41 Crash Loop (2026-11-13)

- origin: g1-m03, account `ueli`, macOS, worktree `/Users/Shared/semio`, store `/Volumes/semio-store` (NFS `192.168.178.70:/Users/Shared/semio-store`, mounted).
- No git command, no `pkill`/`killall`, no service stop, unload or re-provision. The service was never started by hand.

## Before (19:25Z)

- `.cache/f/daemon.lock` absent. Replica log, repeating:

```
The code of the worktree does not start (Export named 'gitGuardsV1' not found in module '/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🌿️git/🟦️.ts'.)
No replica code starts: neither the worktree nor a sealed snapshot
```

## Steps

1. `machine.json`: `{"version":1,"originId":"g1-m03","groupId":"g1","storeRoot":"/Volumes/semio-store"}`.
2. `find /Volumes/semio-store/v1/bootstrap/code/229b2300d546 -type f | wc -l` = **38**.
3. 19:25:49Z `rsync -a /Volumes/semio-store/v1/bootstrap/code/229b2300d546/ /Users/Shared/semio/`: exit 0. A checksum dry run afterwards reported 0 differing files.
4. Waited 90 s. launchd started the daemon by itself: lock `{"version":1,"pid":49735,"startedMs":1794597954751,"originId":"g1-m03",...}` (about 5 s after the copy).
5. `status` at 19:27:31Z: **exit 0**, daemon running, synchronisation on.

```
Replica g1-m03 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 49735)
273961 files tracked; queued: 1822 incoming, ? outgoing; 92 overwritten files kept in .cache/f/lost
Journal: 667.14 MiB in 473 segments of 7 origins.
Newest snapshot by g1-m02: 61044s old, 87.65 MiB.
Reclaimable now: 66 journal segments (94.40 MiB).
Git state: shared (stage and commits replicate); stage tree f3fc6c8568636a95769723c9e6d357f53250a1c4.
Background tiers, last pass (cost, duty cycle of one core): hot 2 ms 0.6%, directory sweep 696 ms 10%, file sweep 3062 ms 3.1%, tree walk 37367 ms 1.8%, git view 105380 ms 5%.
Latency over the last 3 publishes and 77 applies: publish p50 188 ms, p95 341 ms; apply p50 244 ms, p95 850 ms.
g1-m01: 31 events (10.35 KiB) behind, last event 40s ago
g1-m02: 101 events (34.53 KiB) behind, last event 169s ago
g1-m03: 0 events (0 B) behind, last event 42819s ago
g1-m04: 296 events (61.39 KiB) behind, last event 26385s ago
g1-m05: 0 events (0 B) behind, last event 54173s ago
g1-m06: 0 events (0 B) behind, last event 42421s ago
g1-server: 1803 events (576.30 KiB) behind, last event 3s ago
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 2s 0/0 on 7757dd83d95b 84/16/28 -/-|2502/4503 ok
  g1-m02 macos 1s 0/0 on 7757dd83d95b 69/16/14 401589/401589|767/4472 diverged
  g1-m03 darwin 42407s 4/0 on 1c30590cd115 52/0/122 188/341|244/850 stale (attestation too old)
  g1-m04 linux 9675s 3/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 2804s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 darwin 42404s 4/0 on 1c30590cd115 37/0/83 1906/6816|359/5296 stale (attestation too old)
  g1-server macos 2s 0/0 on 7757dd83d95b 0/0/4 3272/3299|1586/7717 diverged
  g1-m02 differs in view.digest: 4f8e5feac38f instead of 1d1a2272cd49
  g1-server differs in view.digest: ecac0e11455f instead of 1d1a2272cd49
  g1-m02 differs in guards.modeMismatch: 1 instead of 0
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

6. The daemon caught up (`{"event":"repaired","count":1822,"reason":"state-takeover"}`), applied the newest code and restarted itself once: pid **50232**, started 20:28:15 local, unchanged since. `fleet --wait 600` from 19:28:01Z to 19:38:02Z: **exit 1** (expected, other workers down), 411 waiting lines, then:

```
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 4s 1/0 on 7757dd83d95b 84/16/28 3735/3735|927/4319 lagging (not settled)
  g1-m02 macos 1s 0/0 on 7757dd83d95b 69/16/14 224/1776|1127/2196 ok
  g1-m03 macos 2s 1/0 on 7757dd83d95b 119/16/37 158/740|619/6164 lagging (not settled)
  g1-m04 linux 10284s 5/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 3413s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 94s 3/0 on 7757dd83d95b 16/0/15 -/-|2580/3351 stale (attestation too old)
  g1-server macos 1s 0/0 on 7757dd83d95b 119/16/39 3264/4035|451/3129 diverged
  g1-server differs in view.digest: 785d1d7b929b instead of 4f8e5feac38f
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

`status` at 19:38:15Z: daemon running (pid 50232), synchronisation on, 274834 files tracked, queued 0 incoming; line for this machine: `g1-m03 macos 4s 1/0 on 7757dd83d95b 119/16/37 158/740|619/5457 lagging (not settled)`.

## Result

The daemon runs again, on the newest code `7757dd83d95b` (same digest as g1-m01, g1-m02, g1-server), attestation fresh, incoming queue 0. Verdict for g1-m03 is `lagging (not settled)` with lag `1/0`: one origin still counted behind, nothing queued.

## Observations (not acted on)

- On the first start with the bootstrap code the log shows one `TypeError: attester.running is not a function` at `🔁️replica/🟦️.ts:2264` (in the `sealCodeV1` continuation). It did not stop the daemon and has not recurred since the self-restart on `7757dd83d95b`.
- The replica keeps publishing a file `undefined.stale` in the worktree root (`/Users/Shared/semio/undefined.stale`, between 18 and 73 MB, rewritten continuously, also arriving from g1-m02: `{"event":"kept","path":"undefined.stale",...,"reason":"newer-than-remote"}` and `unstable`). The name suggests a path built from an undefined value. It costs a blob publish of up to 73 MB per round and may be what keeps origins `not settled`. Left untouched.
