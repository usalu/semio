# g1-m01 Recovery Record for the 08:41 Crash Loop (2026-11-13)

Result: **daemon running again on the bundle code, synchronisation on, this machine's fleet line is `ok`.** `fleet --wait 600` exited 1 because other origins are diverged, lagging, stale or off.

- originId: g1-m01
- storeRoot: /Volumes/semio-store (NFS, mounted)
- worktree: /Users/Shared/semio
- account: ueli, macOS

## What was done

1. 11:24:06 — read `.cache/f/machine.json`. `.cache/f/daemon.lock` did not exist (no daemon running).
2. `find /Volumes/semio-store/v1/bootstrap/code/229b2300d546 -type f | wc -l` → **38** (0 symlinks).
3. Compared the bundle with the worktree before copying. 33 files identical, 5 differed:
   - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🌿️git/🟦️.ts`
   - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🎣️tiers/🟦️.ts`
   - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🩺️attest/🟦️.ts`
   - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🩺️attest/🧬️schema/🔣️.json`
   - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`
4. 11:24:18 — `rsync -a /Volumes/semio-store/v1/bootstrap/code/229b2300d546/ /Users/Shared/semio/` → exit 0. All 38 files then matched the bundle.
5. Waited 90 s. The service was not started, stopped, unloaded or re-provisioned by hand; launchd started the daemon. No git command that changes the repository, no pkill or killall.
6. 11:25:58 — `daemon.lock`: pid 41152. `status` → status exit=0.
7. 11:26:20–11:36:20 — `fleet --wait 600` → fleet exit=1. pid still 41152 afterwards (no restart during the wait).

The `log show` step was not needed: the daemon was running at the first status.

## status output (11:25:58)

```
Replica g1-m01 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 41152)
274799 files tracked; queued: 0 incoming, ? outgoing; 8 overwritten files kept in .cache/f/lost
Journal: 667.13 MiB in 473 segments of 7 origins.
Newest snapshot by g1-m02: 60952s old, 87.65 MiB.
Reclaimable now: 66 journal segments (94.40 MiB).
Git state: shared (stage and commits replicate); stage tree ea21979e7393e0c66dcd4858b8066a20d5e5bf7a.
Background tiers, last pass (cost, duty cycle of one core): hot -, directory sweep 0 ms 0%, file sweep -, tree walk -, git view -.
Latency over the last 0 publishes and 0 applies: publish p50 - ms, p95 - ms; apply p50 - ms, p95 - ms.
g1-m01: 0 events (0 B) behind, last event 3s ago
g1-m02: 0 events (0 B) behind, last event 77s ago
g1-m03: 0 events (0 B) behind, last event 42726s ago
g1-m04: 0 events (0 B) behind, last event 26293s ago
g1-m05: 0 events (0 B) behind, last event 54081s ago
g1-m06: 0 events (0 B) behind, last event 42328s ago
g1-server: 0 events (0 B) behind, last event 88s ago
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 9s 0/0 on 7757dd83d95b 16/0/14 -/-|-/- ok
  g1-m02 macos 1s 0/0 on 7757dd83d95b 69/16/14 401589/401589|401/18396 diverged
  g1-m03 darwin 42309s 4/0 on 1c30590cd115 52/0/122 188/341|244/850 stale (attestation too old)
  g1-m04 linux 9576s 3/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 2705s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 darwin 42306s 4/0 on 1c30590cd115 37/0/83 1906/6816|359/5296 stale (attestation too old)
  g1-server macos 3s 0/0 on 7757dd83d95b 0/0/4 3273/3301|1585/7717 diverged
  g1-server differs in view.digest: ecac0e11455f instead of 4f8e5feac38f
  g1-m02 differs in guards.modeMismatch: 1 instead of 0
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

## fleet --wait 600 output (11:26:20–11:36:20)

"Waiting for agreement" progress lines omitted.

```
Fleet: origin platform age lag (origins behind/queued) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 3s 0/0 on 7757dd83d95b 84/16/28 -/-|1051/4121 ok
  g1-m02 macos 4s 0/0 on 7757dd83d95b 69/16/14 244/1717|918/1971 diverged
  g1-m03 macos 9s 1/0 on 7757dd83d95b 119/16/37 158/740|600/6377 lagging (not settled)
  g1-m04 linux 10182s 5/0 on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 3311s 6/536 off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 darwin 42912s 6/0 on 1c30590cd115 37/0/83 1906/6816|359/5296 stale (attestation too old)
  g1-server macos 9s 1/0 on 7757dd83d95b 119/16/39 3266/3293|647/2266 lagging (not settled)
  g1-m02 differs in view.digest: 4f8e5feac38f instead of 1d1a2272cd49
The fleet is not in agreement (7 origins; fresh means within 60 s).
```

## This machine's line

```
  g1-m01 macos 3s 0/0 on 7757dd83d95b 84/16/28 -/-|1051/4121 ok
```
