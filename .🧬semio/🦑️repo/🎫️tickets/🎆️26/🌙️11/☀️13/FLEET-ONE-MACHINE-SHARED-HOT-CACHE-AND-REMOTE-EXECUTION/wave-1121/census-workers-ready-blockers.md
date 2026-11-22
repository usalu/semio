# Census: Open Blockers Keeping Workers From Ready (read-only audit, 2026-11-21 22:52 UTC)

Scope: ticket `WORKERS-READY-WITH-LOW-RESOURCES` (☀️14). Sources: the ticket's newest files (last written 21:12 UTC), `ROUND.md` (round 31, last changed 2026-11-15), `PROTOCOL.md`, `agent-preamble.md`, and the live store receipts `/Users/Shared/semio-store/v1/origins/<origin>/ready.json` (read at 22:51 UTC). The store receipts are newer and more authoritative than the ticket's `workers/<origin>/state.json` files (see blocker 7).

## Current state

- Store `ready.json`: 0 of 13 Ready. Ten macOS workers (g1-m01, m02, m03, m06, m07, m08, m10, m11, m12, m13) and Linux g1-m04 are in running stages. Windows g1-m05 is waiting on dependencies. No `ready.json` exists for g1-m09.
- Ten `ready.json` receipts are fresh (0 to 5 s old at 22:51 UTC). m05 is 31 s old. Those are the active workers. Their current supervisors started 21:09 to 21:22 UTC.
- g1-m04 (last write about 23:53 UTC on Nov 20, about 23 h old) and g1-m06 (last write about 18:27 UTC on Nov 20, about 28 h old) are stale.
- Stage counts: 12 workers are on stage 5 of 5 (`workspace`), 1 on stage 4 of 5 (m04, `tools`), and 1 on stage 3 of 5 (m05, `dependencies`). Native, repo-mcp and dependencies are not the open front on the macOS workers.
- Coordinator (g1-server) is publishing: `attest.json`, `pool.json` and `pool-published-*` were written at 22:51 to 22:52 UTC.
- Ticket narrative: last file is `verification-low-resource-pulse-2105.md`, written 21:12 UTC. No human or agent has added ticket evidence for about 1 h 40 min.
- Lock files: none. I searched `wave-1121/`, the ☀️13 FLEET folder (depth 4) and the ☀️14 ticket folder (depth 2) for `landing.lock` and `*.lock`. No `landing.lock` exists in any of them.

## Ranked open blockers

### 1. Whole-graph generation never completes; complete-generation adoption is 0 of 13 (all 13 workers)
- Symptom: native fleet status shows no complete generation. The whole-graph request (14,336 MiB, PID 24613) is queued behind a swap gate. The verifier reports 13 of 13 destination adoption receipts missing and short-circuits before producer or signature checks (`verification-low-resource-pulse-2105.md`, `verification-low-resource-pulse-2031.md`).
- Affected: all 13 workers, because each must adopt a complete generation.
- Root cause as understood: the coordinator's whole-graph rebuild asks for 14,336 MiB on a 32 GiB host. The gate refuses while swap is in use (19,510 MiB swap at 20:09, 30,954 MiB at 20:56). Lab admission also needs a fixed 60,129,542,144-byte disk floor. Host free space fell from 44.6 GB (20:26 UTC) to 24.4 GB (20:56 UTC) and was 37.4 GB at 21:12 UTC. Swap allocations (8.6 GB, then 3.2 GB more) and Rust image growth explain most of the drop. The production reclaim predicate was false on all eight managed backings. Even the total theoretical slack (29.2 GB) would leave the deficit (35.7 GB) unfunded.
- Source files: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/audit-coordinator-physical-pressure-2056.md`; `…/WORKERS-READY-WITH-LOW-RESOURCES/audit-cold-lab-disk-1954.md`; `…/WORKERS-READY-WITH-LOW-RESOURCES/audit-build-disk-native-reclaim-pressure.md`; `…/WORKERS-READY-WITH-LOW-RESOURCES/audit-m11-scoped-plan-capacity.md`; build module `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.
- Tried: capacity gate and swap refusal left as is; reclaim ingress bound to the measured device (`audit-build-disk-native-reclaim-pressure.md`); the 60 GB floor kept fixed by design; the coordinator's graph service restarted and reloaded (round 28 to 31).
- Design issue: yes. A 14 GB whole-graph memory request and a fixed 60 GB lab floor cannot be met on this fleet's hardware (32 GiB coordinator, 24 to 44 GB free disk) without reclaim that the managed predicates do not allow. Adoption of a whole generation makes every worker's Ready depend on this one.

### 2. Workspace graph slice is too slow and too heavy on 16 GB Macs (10 macOS workers)
- Symptom: `ready-log-workspace.txt` repeats `Reading the project graph slice (N s): taken X of 600`. Examples at 22:51 UTC: m01 stuck at `taken (pass 2) 249 of 600` for more than 16,000 s (counter moved 249 to 250 over the tail); m02 113 of 600 at 468 s; m03 297 at 6,113 s; m06 327 of 596 at 3,211 s; m08 503 of 600 at 10,762 s; m10 518 at 10,629 s; m11 240 at 9,853 s; m12 358 at 6,985 s; m13 463 at 7,274 s. m07 is waiting for the load of a leader checkout (194,607 source observations, 2,209 owner checks). Peak resident size: 5.1 to 10.7 GB per worker (`peakMegabytes` in `ready.json`).
- Affected: m01, m02, m03, m06, m07, m08, m10, m11, m12, m13 (16 GB hardware where stated in hostnames: m01, m02, m03, m07, m11, m12).
- Root cause as understood: the plan-stage inference grows past the per-machine plan budget. Round 30 added a step-down (6 steps, 4 projects at a time on 16 GB), which lets the read finish but slowly. Each `attempts` restart (m12 at 26, m11 at 19, m06 at 16) re-enters the slice from its earlier point. Workers also contend for the same load: the leader and follower checkouts block each other (m07 wait line).
- Source files: store logs `/Users/Shared/semio-store/v1/origins/<origin>/ready-log-workspace.txt` and `ready.json`; `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/ROUND.md` (round 30 text); `…/WORKERS-READY-WITH-LOW-RESOURCES/audit-m11-scoped-plan-capacity.md`.
- Tried: round 30 step-down; scoped plan capacity reservation (the grant is 4,188 MiB against a 2,048 MiB minimum; at 985 MiB usable the minimum is short by 1,063 MiB); checkpoint cadence change (`audit-source-checkpoint-cadence-2030.md`).
- Design issue: yes, probably. A whole-graph slice sized for a 32 GiB coordinator is run on 16 GB workers, and the slice does not finish within a practical time. Whether the stall on m01 is a real livelock or a very slow pass is not established by the data here.

### 3. Windows m05: dependencies blocked by disk floor, EPERM rename, and bun.exe sharing
- Symptom: `ready.json` state `waiting`, stage `dependencies`, refusal `disk-collecting`: needs 6,144 MB free on `C:\semio\.cache`, has 4,937 MB (22:51 UTC). Reclaim finished with `image-slack freed 0 B` and `leaked-images freed 0 B`. The dependency installer failed with status 1 and repeated `bun:dependency-storage remains active on …\bun.exe (shared)` lines. Earlier counts: 162,150 native EPERM rename events (21:05 UTC) and 23,193 target errors (20:31 UTC). The 13 attempts are all dependency retries.
- Affected: m05 only.
- Root cause as understood: (a) the 6,144 MB admission floor sits above what m05's reclaim can free (5.0 to 5.2 GB free across samples); (b) Windows staged-file rename returns EPERM. The cause is not established. The audits say macOS data cannot prove Windows behaviour, and the replica also retries the rename with no path or handle attribution; (c) a bun.exe image stays open in shared mode during dependency storage, which may be the same kind of sharing conflict.
- Source files: `/Users/Shared/semio-store/v1/origins/g1-m05/ready.json`, `…/g1-m05/ready-log-dependencies-failed.txt`; `…/WORKERS-READY-WITH-LOW-RESOURCES/audit-m05-rename-eperm-2045.md`; `…/WORKERS-READY-WITH-LOW-RESOURCES/Staged rename target metadata.md`; `…/WORKERS-READY-WITH-LOW-RESOURCES/m05 disk recovery heartbeat.md`; replica module `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`.
- Tried: census budget of 5 s on the Ready census (`Ready disk consumer census budget.md`); recovery heartbeat (`m05 disk recovery heartbeat.md`); per-collector order; operation counters for unlink and rename (`Replica target sharing operation counts.md`). No physical Windows test has been run.
- Design issue: partly. The 6,144 MB floor is a design constant that the reclaim path cannot reach on this disk. The EPERM and bun.exe sharing are bugs or environment issues that need an actual Windows reproduction.

### 4. Linux g1-m04 and macOS g1-m06 are stale; g1-m09 is absent
- Symptom: m04 `ready.json` last written about 23:53 UTC on Nov 20, stage `tools`, `[repository:command] still running after 6,671 s`, tail repeats `native-artifact-input-read` locks. m06 last written about 18:27 UTC on Nov 20, stage `workspace`, round-31 report says load above 80 and memory pressure for over an hour. m09 has only `fleet-auth.json` (Nov 14) and no `ready.json`.
- Affected: m04, m06, m09.
- Root cause as understood: no live writer for m04 and m06, so their receipts cannot be trusted; m04's `tools` stage hung on a native artifact read. m09 has not been provisioned or has never attested.
- Source files: `/Users/Shared/semio-store/v1/origins/g1-m04/ready.json`, `…/g1-m04/ready-log-tools.txt`; `…/origins/g1-m06/ready.json`; `…/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m06/round-31.md`.
- Design issue: no. Operational: each needs a manual check. Do not restart anything from this audit.

### 5. g1-m02 workspace disk full (ENOSPC) and g1-m12 SIGTERM plus lease contention
- Symptom, m02: `ready-log-workspace-failed.txt` at 22:23 UTC: `Unable to initialize workspace data while attempting to create lock file: No space left on device (os error 28)`; attempts 8, state `previous-failed`. Older (19:41 UTC) `service-failure.txt`: ENOENT on lstat of `…/🦀️rust/Cargo.toml` under the process3d path, raised from owner discovery.
- Symptom, m12: `ready-log-workspace-stalled.txt` at 20:54 UTC: `SIGTERM received`, run cancelled. Containment at 20:55 UTC: 31 lease registry entries, 19 `not-ours`. Attempts 26.
- Affected: m02 (disk), m12 (lease contention and external signal).
- Root cause as understood: m02's Nx workspace volume filled up; why the free space was consumed is not established by these logs. The Cargo.toml ENOENT is an `lstat` probe with no missing-file tolerance (owner discovery `catalogNativeManifestCandidatesV1` in build module, `observation` probe), as the stack trace shows. m12's log shows an external SIGTERM; its sender is not identified. The shared lease registry holds other owners' entries (PIDs 79243, 26983, 48458, 35413).
- Source files: `/Users/Shared/semio-store/v1/origins/g1-m02/ready-log-workspace-failed.txt`, `…/g1-m02/service-failure.txt`, `…/g1-m12/ready-log-workspace-stalled.txt`, `…/g1-m12/ready-log-workspace-containment.txt`; build module `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/` (owner-discovery and observation files, `…/👁️observation/🟨️.mjs`, `…/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`).
- Tried: none recorded for ENOSPC; ENOENT not seen again in the logs I read.
- Design issue: the ENOENT is a code bug. ENOSPC is a disk-floor gap on a 16 GB machine. The m12 kill is operational.

### 6. g1-m13 native stage retry rollback (historical, resolved for now)
- `audit-m13-native-rustup-retry.md` records two native failures at 20:08 and 20:09 UTC (rustup rollback, `Directory not empty`). The next run succeeded at 20:11 UTC. m13's current `ready.json` is at workspace. This is no longer blocking, but the Rust toolchain writer race is not fully explained.
- Design issue: partly (no lock covers user-home writers outside the native lock).

### 7. Reporting hazard: stale worker `state.json` says ready
- `workers/<origin>/state.json` in the ticket shows `"ready":true` for g1-m01, m02, m03, m08 and m10, each dated Nov 18 (round 31). These contradict the live store receipts, which show `ready:false` for all 13. g1-m03 also records `native_rc:130`. These files should not be read as Ready evidence.
- Source files: `…/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m01/state.json`, `…/workers/g1-m02/state.json`, `…/workers/g1-m03/state.json`, `…/workers/g1-m08/state.json`, `…/workers/g1-m10/state.json`.

## Is anyone working on it now?

- Yes, automatically. Ten macOS workers, m05 and the coordinator are writing receipts every few seconds. Their supervisors restarted at 21:09 to 21:22 UTC and run without a session.
- No human or agent has written ticket evidence since 21:12 UTC (about 1 h 40 min at 22:52 UTC). ROUND.md still says round 31 with the rule not to run setup commands by hand.
- No `landing.lock` exists in this ticket, the ☀️13 FLEET folder, or `wave-1121/`.

## Verification of this audit

Read-only. I changed no source, process, lease, image, cache, store file, lock, or ticket state. I did not open or close tickets and did not delete anything in any 🗑️generated folder. Times are UTC (the local shell is UTC+1). Store receipts were read at 22:51 UTC. Ticket facts are as of their own timestamps.
