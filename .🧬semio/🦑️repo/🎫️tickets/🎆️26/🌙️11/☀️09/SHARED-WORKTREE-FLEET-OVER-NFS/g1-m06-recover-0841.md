# g1-m06 Recovery From the 08:41 Crash Loop

Machine `g1-m06` (group `g1`, macOS), store `/Volumes/semio-store`, 2026-11-13.

## Result

- The daemon runs again (pid 60088 at 20:34) with synchronisation on.
- It caught up and moved itself from code `1c30590cd115` to the fleet's current code `7757dd83d95b`.
- The fleet verdict for this machine is still `diverged`: its view digest is `3e7ffc06dbdf` instead of `1d1a2272cd49`, with 1630 modified, 64 deleted and 173 untracked paths in the view. `fleet --wait 600` exited 1.

## What Was Done

1. Read `.cache/f/machine.json`: `originId` `g1-m06`, `storeRoot` `/Volumes/semio-store`.
2. Store is mounted (NFS from `192.168.178.70:/Users/Shared/semio-store`); `v1/bootstrap/code/229b2300d546` holds 38 files.
3. Before copying, `status` failed with:
   `The code of the worktree does not start (Export named 'gitGuardsV1' not found in module '…/🤝️shared-worktree/🌿️git/🟦️.ts'.)` and `No replica code starts: neither the worktree nor a sealed snapshot`.
   The worktree `🌿️git/🟦️.ts` was 53270 bytes from 04:50 without `gitGuardsV1`; the store copy is 69909 bytes from 08:48 and has it. A dry run showed 5 of the 38 files differing in size (`🏗️build/🔣️policy.json`, `🌿️git/🟦️.ts`, `🎣️tiers/🟦️.ts`, `🩺️attest/🟦️.ts`, `🩺️attest/🧬️schema/🔣️.json`), the rest in mtime only.
4. 20:32:37 `rsync -a "/Volumes/semio-store/v1/bootstrap/code/229b2300d546/" /Users/Shared/semio/` exited 0; a checksum dry run afterwards reported 0 differing files.
5. Waited 90 seconds without touching the service, then ran `status` (20:34:18, exit 0).
6. Ran `fleet --wait 600 --lang en` (finished 20:44:35, exit 1).

No git command, no `pkill`/`killall`, no service stop, unload or re-provision, and no other repair was used.

## Status Output (20:34:18)

```
Replica g1-m06 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 60088)
274821 files tracked; queued: 0 incoming, ? outgoing; 39 overwritten files kept in .cache/f/lost
Journal: 667.17 MiB in 473 segments of 7 origins.
Newest snapshot by g1-m02: 61453s old, 87.65 MiB.
Reclaimable now: 66 journal segments (94.40 MiB).
Git state: shared (stage and commits replicate); stage tree -.
Background tiers, last pass (cost, duty cycle of one core): hot 4 ms 1.4%, directory sweep 1062 ms 10%, file sweep 5978 ms 5%, tree walk 53572 ms 3%, git view 896953 ms 5%.
Latency over the last 33 publishes and 100 applies: publish p50 1906 ms, p95 6816 ms; apply p50 359 ms, p95 5296 ms.
g1-m01: 0 events (0 B) behind, last event 450s ago
g1-m02: 12 events (3.27 KiB) behind, last event 34s ago
g1-m03: 17 events (4.63 KiB) behind, last event 1s ago
g1-m04: 27 events (5.76 KiB) behind, last event 26794s ago
g1-m05: 0 events (0 B) behind, last event 54582s ago
g1-m06: 0 events (0 B) behind, last event 0s ago
g1-server: 791 events (215.81 KiB) behind, last event 17s ago
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 3s 0/0 on 7757dd83d95b 84/16/28 -/-|1048/4563 ok
  g1-m02 macos 3s 0/0 on 7757dd83d95b 69/16/14 257/1717|925/3654 diverged
  g1-m03 macos 1s 0/0 on 7757dd83d95b 119/16/37 161/740|561/28973 diverged
  g1-m04 linux 10066s 5/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 3195s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 darwin 42796s 6/0 on 1c30590cd115 37/0/83 1906/6816|359/5296 stale (attestation too old)
  g1-server macos 3s 0/0 on 7757dd83d95b 119/16/39 3267/3294|645/1772 diverged
  g1-m02 differs in view.digest: 4f8e5feac38f instead of 1d1a2272cd49
  g1-m03 differs in view.digest: e3c0c8d89586 instead of 1d1a2272cd49
  g1-server differs in view.digest: 785d1d7b929b instead of 1d1a2272cd49
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

## Fleet Output (20:34–20:44, exit 1)

The 526 progress lines `Waiting for agreement: N origins pending, T s left.` (N alternating between 5 and 6 for the whole 600 s) are omitted.

```
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 4s 0/0 on 7757dd83d95b 84/16/28 233/3735|780/3962 ok
  g1-m02 macos 2s 0/0 on 7757dd83d95b 69/16/14 289/3684|1657/1814 diverged
  g1-m03 macos 5s 0/0 on 7757dd83d95b 119/16/37 160/790|698/3471 diverged
  g1-m04 linux 10678s 5/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 3807s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 2s 0/0 on 7757dd83d95b 1630/64/173 -/-|12577/142210 diverged
  g1-server macos 6s 0/0 on 7757dd83d95b 119/16/39 3258/5842|140/2124 diverged
  warning: g1-m06 runs git 2.50.1, other machines run another version
  g1-m02 differs in view.digest: 4f8e5feac38f instead of 1d1a2272cd49
  g1-m03 differs in view.digest: e3c0c8d89586 instead of 1d1a2272cd49
  g1-m06 differs in view.digest: 3e7ffc06dbdf instead of 1d1a2272cd49
  g1-server differs in view.digest: 785d1d7b929b instead of 1d1a2272cd49
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

## Open Points

- `g1-m06` is fresh, in sync (0 origins behind, 0 queued) and on the newest code, but its git view differs far more than the other machines' (1630/64/173 against roughly 70–120/16/14–39). Apply latency p95 was 142210 ms during catch-up.
- `g1-m06` runs git 2.50.1, which differs from the other machines.
- The status after the restart reported 39 overwritten files kept in `.cache/f/lost`.
- Only `g1-m01` has the verdict `ok`; `g1-m02`, `g1-m03` and `g1-server` are also `diverged`, `g1-m04` is stale and `g1-m05` has sync off.
