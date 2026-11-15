# Audit: the fleet as it is now (2026-11-14, 09:05 to 09:12 CET)

Read-only audit on g1-server. Nothing in the worktree or the store was written. No git write, no kill, no sub-agent, no build or graph command. Secrets: none printed (only key names and public metadata).

Commands used, all read-only: `ls`/`cat`/`stat`/`du`/`df`, `python3 -I` (JSON reads), `lsof -nP -iTCP`, `ps`, `curl -s -m 5` (health and unauthenticated probes only), `repo fleet status --json` (source read first: `🚢️fleet-status/⌨️command` says "takes no lease, loads no graph and writes nothing"; its readers contain no write call). The first run at 09:06 failed with `Cannot find module '../../🌡️host-capacity/🟦️.ts'`; a second run at about 09:10 succeeded (exit 0), and the module exists on disk. Output: scratchpad `fleet-status.json`. `git ls-files` and `git log` were used once, read-only, to locate a module.

Sources: `design.md`, `result-L/N/O/T.md` (first 30 lines), the store `/Users/Shared/semio-store/v1`, `/Users/Shared/semio/.cache/f/`.

## 1. Which machines are alive (store, `v1/origins/*/attest.json`, sampled 09:08:11)

Attest is the replica heartbeat (`wallMs`). The head value is the mtime of the zero-byte `v1/heads/<origin>_...` marker.

| Origin | Platform | Attest age (s) | Heads marker age (s) | Notes |
|---|---|---|---|---|
| g1-server | macos | 1 | 59 | pending code waits (see 6.1) |
| g1-m01 | macos | 1 | 5484 | |
| g1-m02 | macos | 0 | 17784 | |
| g1-m03 | macos | 0 | 10024 | |
| g1-m04 | linux | 1 | 9513 | |
| g1-m05 | windows | 234 (108 at 09:05) | 10968 | stale, EBADF (6.4) |
| g1-m06 | macos | 1 | 17620 | |
| g1-m07 | macos | 98 (350 at 09:05) | 4351 | intermittent (6.5) |
| g1-m08 | none | none | none | only `fleet-auth.json`, 04:58 |
| g1-m09 | none | none | none | only `fleet-auth.json`, 02:13 |

Seven of ten origins heartbeat within 2 minutes. m05 and m07 are alive but not healthy.

## 2. Fleet facts and endpoints

`v1/fleet/cache.json`: generation 5, published 06:36:17 (about 2 h 30 min old), publisher g1-server (`.cache/f/cache-server`, port 6094), group g1, instance `main`, access `read-write`, protocols `bazel-http`, `nx-self-hosted`, auth scheme `semio-fleet-v1`, endpoint `http://192.168.178.70:6094`, execution endpoint `http://192.168.178.70:8980`. Secrets: none in the file.

| Check | Result |
|---|---|
| Cache 6094 `/healthz` via 192.168.178.70 | 200 |
| Cache 6094 `/healthz` via 127.0.0.1 | refused (bound to the LAN address only) |
| Cache 6094 `/status` unsigned | 401 (route: `🌐️remote-cache/🖥️server/🟦️.ts` line 108) |
| Scheduler 8980 listener | bun pid 81794 listening on 192.168.178.70:8980 |
| Scheduler 8980 over HTTP/1.1 | non-HTTP reply (curl: "Received HTTP/0.9") |
| Scheduler 8980 over HTTP/2 (h2c) | 200 with `grpc-status 13 "gRPC request required"`, so 8980 is the gRPC port |
| Scheduler HTTP routes | on 8981 (bun pid 81794): `/healthz` 200, `/v1/fleet/load` 401 unsigned |

The design text says "scheduler on 8980" for HTTP routes; in the code the HTTP routes are on 8981. Documentation mismatch, not a fault.

## 3. `GET /v1/fleet/load` (via `repo fleet status --json`)

Scheduler: `http://192.168.178.70:8981`. Memory: 32 GiB total, 19.1 GiB available, pressure normal, swap used 2.2 GiB, load 15.5. Queued 0, executing 0, offloaded in/out 0/0, refused 0, declined 0, retried 0. Claims active 0, waiting 0.

| Origin | Platform | Pressure | Load / cores | Running / queued | Headroom | Age (s) | Worker | Why | Cache hit / miss |
|---|---|---|---|---|---|---|---|---|---|
| g1-server | macos-arm64 | normal | 16.33 / 10 | 0 / 0 | yes | 4 | absent | disk-full | 1558 / 4678 |
| g1-m01 | macos-arm64 | normal | 5.95 / 12 | 0 / 0 | yes | 1 | absent | disk-full | 0 / 0 |
| g1-m02 | macos-arm64 | normal | 6.80 / 10 | 2 / 1 | no | 3 | refusing | no-headroom (memory) | 5 / 143 |
| g1-m03 | macos-arm64 | normal | 12.16 / 10 | 0 / 0 | yes | 4 | idle | | 0 / 0 |
| g1-m06 | macos-arm64 | normal | 2.59 / 10 | 1 / 1 | yes | 2 | idle | | 0 / 0 |

Not in the load table: g1-m04 (linux), g1-m05 (windows), g1-m07 (macos). No worker is registered for them. Five of ten origins report load; the scheduler sees only machines whose agent is joined.

Cache counters from `/status` (cumulative, since process start): hits 41298, misses 15703, puts 7455. Per-origin counters are not in the report.

## 4. Cache server statistics

- Process: bun pid 81793, started 2026-11-14 06:36:16, listening on 192.168.178.70:6094.
- Data dir: `/Users/Shared/semio/.cache/f/cache-server`. Size: 579 MiB by `du -sk` (592696 KiB); `du -sg` reports 1 GB (rounded).
  - `v1/` 57 MiB: `main` 12.5 MiB, `semio` 15.9 MiB, `claims.jsonl` 0.9 MB. Files: 3194 under `main`, 3882 under `semio` (ac and nx only).
  - `mirror/` 511 MiB (rustup, npm, go mirror).
  - `fleet/` 12 MB: `provenance.jsonl` 12.2 MB, `divergences.json`, `pins.json`.
- CAS blobs are not under the cache's `v1/`. The group store has `/Users/Shared/semio-store/v1/blobs`: 239,573 files, about 34 GiB. `fleet/install/blobs`: 8,067 files, about 4.9 GiB. Whether the cache serves CAS from there is not verified.
- Provenance (88,737 rows: 75,483 cas, 13,243 ac, 11 nx, spanning about 8.5 h). Per-origin totals: g1-m07 32,908; g1-server 29,319; g1-m01 9,229; g1-m03 6,678; g1-m04 5,567; g1-m06 4,712; g1-m02 324; g1-m05 0.
  - Last 3 h: g1-m01 7,309 (6,286 cas, 1,023 ac); g1-m04 4,438 (3,161 cas, 1,277 ac); g1-server 1,769 (754 cas, 1,015 ac); g1-m07 558 (331 cas, 227 ac); g1-m02, g1-m03, g1-m06, g1-m05: 0.
- Claims (`v1/claims.jsonl`, 4652 rows): 2699 `set`, 1953 `drop`. Rows per local hour: 06 h 380, 07 h 1578, 08 h 2680, 09 h 14. Claim traffic stopped at about 08:30.
- Divergences (`fleet/divergences.json`, 4 rows): ac `6e9dedb6…` kept g1-m02 vs refused g1-m07, 5.9 h ago; ac `7240a80f…` between g1-m07, g1-m06, g1-m01 (3 rows), 1.9 to 5.8 h ago. g1-m07 is in 3 of 4 rows.
- Slot supersession: no on-disk record. The count exists only in the signed `/status` counters, which I could not read. Not measured.
- Build counters (`.cache/f/build-counters.json`, g1-server): unit localHit 992, remoteHit 566, miss 4678, stored 2341, skipped 928. Offload out 21, refused 539, fellBack 289.

## 5. Sidecars per machine

Sidecar state is written by the replica to `.cache/f/health.json` (`sidecars` array, `🧷️sidecars/🟦️.ts` `report`) on each machine. Only g1-server's copy is readable from here.

| Origin | build-cache | build-scheduler | build-agent | Evidence |
|---|---|---|---|---|
| g1-server | running, pid 57490, restarts 0 | running, pid 57489, restarts 0 | running, restarts 7, last exit "exit 1" | `health.json`, beat lag 253 ms, daemon health 1.3 s old |
| g1-m01 | not visible | not visible | worker absent, disk-full | fleet load table |
| g1-m02 | not visible | not visible | refusing, no-headroom | fleet load table |
| g1-m03 | not visible | not visible | idle | fleet load table |
| g1-m04 | not visible | not visible | not registered | not in table |
| g1-m05 | not visible | not visible | not registered | not in table |
| g1-m06 | not visible | not visible | idle | fleet load table |
| g1-m07 | not visible | not visible | not registered | not in table |

Scheduler log (`sidecars/build-scheduler.log`): three mac minis and g1-server's own agent joined at 05:50; all left between 08:05:37 and 08:06:39. The hostnames are `ueli-workstation-mac-mini-m4-16gb-256gb-0N`, and the mapping to origins is not verified. build-agent on g1-server crashed from 08:05 with `Cannot find module '../../🌡️host-capacity/🎛️control/🟦️.ts'`. The module exists now, and the agent rejoined at 08:07 without a worker (disk floor).

## 6. Broken or suspicious

1. **Replica spins on "blocked" on every origin.** g1-server `blocked` 109,083 (09:05:58) to 109,923 (09:08:11), about 6 per second. Other origins rise too (m01 71,307 to 72,220; m06 90,441 to 91,580). All pending code `2ad0bb52…`: "waiting: no origin runs this code yet and it was not all written on this machine". Evidence: `origins/*/attest.json` `failures`, `pending`; `health.json`.
2. **Workers refused by the disk floor on g1-server and g1-m01.** Agent message: "3.5 GiB free, floor 20 GiB" (g1-server), "5.9 GiB free" (m01). `df -g` shows 60 GiB available on `/System/Volumes/Data` (839 of 926 GiB used). The floor is measured on another path or budget. Not resolved.
3. **Old cache job still running on 6090.** bun pid 1062 (`--eval` script, started 2026-11-13 17:47, 15 h 19 min at 09:07, cwd `/Users/Shared/semio`). It listens on `*:6090` (all interfaces, reachable from the LAN). `/healthz` 200, `/status` 401. `fleet status` lists it as a `remote-cache serve` run in state "silent". I did not kill it (no pkill rule). Recommend the owner stops it.
4. **g1-m05 (windows) is stale.** Attest age 108 s at 09:05 and 234 s at 09:08. Failures: `attest` 442, `round` 365, `blocked` 3192. Pending: "EBADF: bad file descriptor, read". Heads marker 10968 s old. Not in the load table, no worker.
5. **g1-m07 is intermittently stale and leaves temp files.** Attest 350 s at 09:05, 98 s at 09:08. A 0-byte `origins/g1-m07/fleet-auth.json.22120-a90da73871a7.tmp` appeared at 09:05. Failures: `unstable` 16, `blocked` 19,393. It is the top provenance writer (32,908 rows) and in 3 of 4 divergences. Not in the load table.
6. **g1-m08 and g1-m09 have no replica heartbeat.** Only `fleet-auth.json` (04:58 and 02:13). No attest, no heads marker.
7. **AppleDouble metadata in the shared store.** 3,410 `._*` files under `/Users/Shared/semio-store/v1` (e.g. `origins/*/._attest.json`, `heads/._*`). These come from non-HFS copies and can confuse readers that list the store.
8. **Fleet is idle since about 08:30.** Queued 0, executing 0, offloads in/out 0/0 now. Offload refusals (539) far exceed offloads out (21) in the build counters.
9. **`repo fleet status` failed once at 09:06** on a missing module path. It worked at about 09:10 with the same code. The cause is not determined; the readers should not depend on a transient landing.
10. **Documentation mismatch.** The scheduler's gRPC port is 8980. The HTTP routes (`/healthz`, `/v1/fleet/load`) are on 8981. The design lists them under 8980.
