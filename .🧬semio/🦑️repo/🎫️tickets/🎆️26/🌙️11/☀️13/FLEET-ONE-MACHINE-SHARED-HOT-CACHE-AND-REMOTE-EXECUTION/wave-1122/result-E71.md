# E71 worker-re-storage (2026-11-22 15:41 to 16:06)

## Verdict
Landed: per-volume disk gate, owner-liveness retirement of dead workers' action roots, a gc that finally collects the sidecar workers' home, idle-gc wiring, lease-registry prune that drains the whole backlog. Live after the replica rollout (16:05): m08 and m11 left `disk-full` (now only `no-headroom` memory). m01 still says `disk-full` (0.4 GiB free, now "one action needs 1.0 GiB"), m12 not yet restarted (still old text, 3.5 GiB free). m13 `sidecar-restarting` is not a fault (see 3).

## 1. What fills `d/m/remote-execution` (read from code, the fleet store and this host)
- The worker home `.cache/b/d/m/remote-execution` is inside the dependencies image `d` (36 GiB). Workers' ready logs show `d/m: 362 of 36863 MB free` (m01), 364/3474 (m08), 1557 (m11), 1783 (m12): the image is full of dependency generations (installs fluctuate 0.4 -> 8.8 -> 0.4 GiB on m01 within one stage), NOT of remote-execution content. On the coordinator the RE home is 212 MB (blobs only, bounded 256 MiB, pruned every 30 s by the worker).
- The `disk-full` verdict was wrong in kind: `worker.disk.reserveGigabytes` 3 GiB ("for the machine: swap, replica, owner builds") was measured on the free space of the image, which holds nothing but dependencies. One action needed 3.5 GiB of an image with 1.5 GiB free, so the worker refused although the host volume had 30-50 GiB free.
- Real RE leaks existed too: a worker removed only its OWN `work/<name>` at start (name = host-pid), so every sidecar restart (many during this wave: each landing restarts agents) left the in-flight actions' private roots of the old pid behind; the only collector removed them after 24 h by mtime. And `repo cache gc` never collected the sidecar home at all: both ledger areas (`remote-execution`, `remote-execution-worker`) ran `collectRemoteExecutionForLedgerV1`, which resolved only the policy directory in the checkout (`.🧬semio/🦑️repo/⚡️cache/remote-execution`, absent) and answered `absent`.
- E16's CAS cap does not apply: the RE area is not a CAS and is bounded by `budget.basisPoints 1000` of the dependencies partition, which only reports.

## 2. Fix (schema first, tests with oracles, landed under locks)
- Policy + schema `worker.disk`: `reserveGigabytes` 0.5 (work volume), new `hostReserveGigabytes` 3 (checkout volume), `perActionGigabytes` 0.5; new `worker.ownerlessWorkSeconds` 86400. Gate `remoteExecutionDiskSlotsV1` takes the minimum over both volumes and reports the limiting one (`volume`); the agent measures the host volume at `options.root` (`hostFreeBytes` injectable).
- `claimRemoteExecutionWorkV1` writes `work/<name>/.owner` {pid}; `retireRemoteExecutionWorkV1` removes a sibling whose owner pid is gone at once (event-driven: at worker start and after every finished action), one without a readable record after `ownerlessWorkSeconds`; never the caller's own, never a live pid (EPERM counts alive).
- `collectRemoteExecutionForLedgerV1(workspace,dryRun,signal,area)`: the cache-gc delegate passes the area id; `remote-execution-worker` resolves `cacheAreaHomeV1` (the home the agents write), a detached image answers `absent`.
- `repo cache gc --idle` (the hourly background pass of every agent/cache sidecar from E16 round 2) now runs the worker-home collection and the lease-registry prune before the image compaction.
- Decision NOT taken: moving the area out of the dependencies image to the host volume. That image exists so the replica's file watcher (and fseventsd, 9.6 GB RSS on the coordinator) never sees action churn; a host home would put it back. The ledger declaration, the `needs` table and install.json stay unchanged (a move would also need a retirement of the leftover inside an image, which the ledger does not support: `within` is only for control areas).

## 3. m13 `sidecar-restarting`
Not a crash. It is the `ready` role's decision `restartPending` (readyDecideV1): after any change of its import closure the sidecar waits `quietMilliseconds` 300 s (max deferral 1800 s, each further change resets the quiet time) before restarting and meanwhile starts no stage. During this wave every landing re-arms it, so m13 sat in "sidecar-restarting" at attempt 39 although its stage attempts were ordinary dependency failures. No restart loop in the store (no `service-failure-*` for m13, no restart lines in its ready logs). At 15:57-16:05 m13 shows `idle`/`busy` and its own stage failure (`dependencies` exit 1, rustup `wasm32-wasip2` conflict / bun-cache rename, E56's lane). Nothing changed.

## 4. Lease registry prune (E60 item)
`pruneLeaseRegistryIfDueV1` is async: after claiming the stamp it runs bounded slices (maximumFolders 256, budget 20 ms) and yields between them until the last folder, so the whole dead backlog goes in one pass; an aborted pass stamps `more` and resumes after 20 s. `schedulePrune` also arms an unref'd 20 s interval (cheap no-op through the stamp until the 10 min interval is due), so a long-lived process prunes without lease activity; `repo cache gc --idle` calls it too. Live: `prune.json` `more:false`, doctor `leases.registry-stale` is now info (197 live folders).

## Proof
- Tests (`repo test-file`): lease prune 7/0 (new: whole backlog in one pass with slice size 1, aborted pass resumes), order 78/0, status 2/0, wait 15/0; remote-execution disk 7/0 (new two-volume rule + 400-case Python oracle; the live agent test serves/refuses with injected free bytes), volume 2/0, worker retirement 4/0 (new, Python oracle of the survivors), worker 6/0, main 17/0, command 7/0, offload cluster 10/0; cache-gc main 19/0, verify 4/0, sweep 4/0.
- Live: `repo cache gc --idle` printed "Remote execution worker home: removed 0 blobs and 0 input roots of 0 dead workers"; fleet status 15:57 old text on m01/m08/m11/m12, 16:05 m08 and m11 no longer disk-full, m01 shows the new text ("one action needs 1.0 GiB").

## NOT verified
- m01: its image is genuinely full (0.4 GiB); it serves once the dependency collector frees about 0.6 GiB. m12 had not restarted its sidecar yet. I could not see any worker's disk (nothing is published); the leak of dead roots is derived from code, not measured on a worker.
- Typecheck not run; Windows/Linux paths (host reserve uses the same free-space call) not run.
- The retire-at-worker-start and after-action hooks are covered by the unit contract and the cluster suite passing, not by a test that kills a worker mid-action.

## Others must know
- Work folders now carry `.owner`; anything that lists `work/*` as action roots must skip it.
- Findings: W/findings-E71.md (4096-entry dependency generation store; keep-alive of fixed 3 GiB assumptions elsewhere).
- Landings: T/landings.md "Wave 1122 E71" parts 1 to 3. Scripts: T/🧰️scripts/E71/. Locks all released.
