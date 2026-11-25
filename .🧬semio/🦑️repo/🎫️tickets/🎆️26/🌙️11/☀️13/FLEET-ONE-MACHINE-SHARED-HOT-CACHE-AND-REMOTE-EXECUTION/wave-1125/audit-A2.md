# Wave 1125 Audit A2: Live Fleet Census

Read-only census, taken 2026-11-25 ~16:31 local (origin files read at epoch 1795620718).
Raw inputs: origin files under `/Users/Shared/semio-store/v1/origins/<origin>/`, `repo fleet status --json` (persisted tool output), `repo remote-cache status`, `repo remote-execution status`, `repo doctor --json` saved to `doctor-A2.json` in this folder (exit 1, verdict `blocked`; ran to completion, under 5 min).

Summary: 1 of 14 reporting workers is Ready (g1-m07, refreshing). 13 are not Ready: 9 in `workspace` (5/5), 2 in `dependencies` (3/5), 1 stalled and waiting. g1-m09 has no status files and g1-server has no `ready.json`.

## Per-worker table

Columns: platform; Ready; state / stage / attempts; last failure class; memory available (of total) and swap used; volume free (proxy for disk free, from `memory.json` host `swapVolumeFreeMegabytes`); attest manifest / HEAD agreement; age of last `ready.json` update.

Attest HEAD for every worker is `8d17d8ba399`, which matches repo HEAD. Attest manifest is `452470193469` on every worker, so manifest agreement across workers is full. `ready.json` digest differs from attest manifest on every worker, so that comparison is inconclusive. Stage agreement with the fleet: 11 at `workspace`, 2 at `dependencies`.

| Worker | Platform | Ready | State / stage / attempts | Last failure class | Mem avail / swap MB | Vol free MB | Attest HEAD | Ready digest | Update age s |
|---|---|---|---|---|---|---|---|---|---|
| g1-m01 | macos | no | backoff / workspace 5/5 / 10 | stage-failed: Owned Nx run-many (norm-din4108, repo-mcp) exit 1 | 6422/16384, swap 2054 | 20483 | 8d17d8ba399 (ok) | 4debcdde5e94 | 35 |
| g1-m02 | macos | no | backoff / workspace 5/5 / 7 | stage-failed: same Nx failure | 5831/16384, swap 2503 | 16006 | ok | 4debcdde5e94 | 79 |
| g1-m03 | macos | no | running / workspace 5/5 / 1 | stalled (timings: last outcome stalled, SIGTERM history) | 6189/16384, swap 575 | 52184 | ok | 4debcdde5e94 | 5 |
| g1-m04 | linux | no | waiting / dependencies 3/5 / 22 | disk-short refusal; Fixed Store dependency installer failed | 6752/15848, swap 6092 | n/a | ok | 65147b8a8769 | 37 |
| g1-m05 | windows | no | running / dependencies 3/5 / 46 | adopted; still running after 15559 s, binding profile ~85-95 s per pass | 30999/65447, swap n/a | n/a | ok | 4806eb881458 | 6 |
| g1-m06 | macos | no | running / workspace 5/5 / 8 | previous-failed (retrying) | 5793/16384, swap 1846 | 59794 | ok | 4debcdde5e94 | 6 |
| g1-m07 | macos | YES | ready / workspace 5/5 / 0 | none (reason refreshing) | 4065/16384, swap 2363 | 18977 | ok | 4debcdde5e94 | 0 |
| g1-m08 | macos | no | running / workspace 5/5 / 22 | previous-failed; memory level guard, pressure warn | 3053/16384, swap 3265 | 17810 | ok | 4debcdde5e94 | 1 |
| g1-m09 | ? | no | no status | none: origin folder holds only fleet-auth.json (Nov 14) | n/a | n/a | n/a | n/a | n/a |
| g1-m10 | macos | no | backoff / workspace 5/5 / 36 | stage-failed: same Nx failure | 5562/16384, swap 2496 | 23398 | ok | 4debcdde5e94 | 21 |
| g1-m11 | macos | no | running / workspace 5/5 / 3 | previous-failed (retrying) | 6423/16384, swap 673 | 70957 | ok | 4debcdde5e94 | 0 |
| g1-m12 | macos | no | waiting / workspace 5/5 / 5 | disk-unfit refusal; Nx run-many exit 1 | 7486/16384, swap 1522 | 15096 | ok | 4debcdde5e94 | 54 |
| g1-m13 | macos | no | backoff / workspace 5/5 / 10 | stage-failed: same Nx failure | 6211/16384, swap 2491 | 53681 | ok | 4debcdde5e94 | 42 |
| g1-m14 | macos | no | backoff / workspace 5/5 / 10 | stage-failed: same Nx failure | 6458/16384, swap 1330 | 22133 | ok | 4debcdde5e94 | 74 |
| g1-server | macos | no (no ready.json) | pool only: pulling | pool lastError: native dependency without build project owner (see problems) | n/a | n/a | ok | n/a | pool 1 |

Pool state (`pool.json`): 13 origins in `pulling` or `idle`, each claiming 729 units. g1-server stored 793 with 4 failed and 1 running. m01 has 3 failed and m13 has 1, with lastError on m01 naming `semio-s-artifact-norm-en1993-scene-schema-grips`.

## Fleet-level

**Remote cache** (`repo remote-cache status`): endpoint `192.168.178.70:6094`, instance `main`, upload effective true. Token env `SEMIO_REMOTE_CACHE_TOKEN` is not present in this shell. Probes are reachable and return HTTP 404.
- Server entries 64749, 136 MB, cap 2.58 GB, validation complete, not read-only.
- AC: hits 155240, misses 170039, hit rate about 47.7%, conflicts 17, stale 2202.
- CAS: hits 1545991, misses 438321, hit rate about 77.9%, rejected 43627.
- nx: hits 1, misses 427, hit rate about 0.2%.
- Divergences: 957 (namespaces `semio` and `main`, kind `ac`); kept g1-m02 vs refused g1-m07 in the recent examples.
- Fleet denials: replay 33, clock 4.

**Remote execution** (`repo remote-execution status`): endpoint `192.168.178.70:8980`, scheduler `:8981`, gRPC probe reachable in 14 ms.
- 14 workers registered, 0 draining, 1 running, 0 queued.
- Offloaded in: 1374 total.
- Counters: completed 1326, failed 5, cancelled 13, refused 30, retried 32, offloadRejected 1, queue wait p90 1 ms.
- Workers with headroom false: g1-m03 and g1-m08.
- Workers with 0 offloaded in: g1-m04, g1-m05, g1-m07, g1-m03.

**Fleet status** (`repo fleet status --json`): alerts 7, warnings 9.
- Source readers `storage` and `workers` timed out at 5 s.
- Memory 13927 MB available of 32768 MB, swap 1083 MB, load 31.05 on 10 cores.
- Cache: CAS used 96.7 GB, 98.99% of cap.

**Doctor** (`doctor-A2.json`): verdict `blocked`, counts error 2, warn 7, info 1, repairs none.
- Errors: `storage.image-below-floor` on graph partition (196 MB free vs 1 GB floor, on this host); `host.load-high` (load 40.7 on 10 cores).
- Warnings: `dependencies.inputs-newer` for root and 4 scopes (bun.lock newer than generation); `processes.idle-holder` pid 96461 holding cargo for 5521 s at 0.3% CPU; `doctor.probe-timeout` on caches (8 s); `worker-warden-intervened` on g1-m07 and others; `worker-swap-high` on g1-m04 (6071 MB); `worker-memory-tight` on g1-m08; `unit-cache-read-only` on g1-server (5053 units compiled without sharing).

## Top 10 problems by impact on "all workers Ready"

1. **Workspace Nx build fails on 9 workers.** `repo-mcp:build:prototype` and `@semio-tech/norm-din4108-rs` run-many exits 1, with `Internal error: Owned Nx invocation failed (130)`. Evidence: `/Users/Shared/semio-store/v1/origins/g1-m01/ready-log-workspace-failed.txt`, `.../g1-m12/ready-log-workspace-failed.txt`, `ready.json` `tail` in each origin.
2. **Native dependency has no build project owner.** `semio-s-artifact-norm-en1993-scene-schema-grips` is blocked, and pool `lastError` reports it. Evidence: `.../g1-m01/pool.json` (`lastError`), `.../g1-server/pool.json`.
3. **g1-m04 dependencies fails and is swapping.** Fixed Store dependency installer failed (reference 8ef44f48), swap 6092 MB, `worker-swap-high`, warden intervened 169 times. Evidence: `.../g1-m04/ready-log-dependencies-failed.txt`, `.../g1-m04/ready.json`.
4. **g1-m05 (Windows) stuck in dependencies.** 46 attempts, `adopted`, still running after 15559 s. Store binding profile takes 85-95 s per pass, dominated by `GetFileType` on about 16k calls. Evidence: `.../g1-m05/ready.json`, `.../g1-m05/ready-log-dependencies.txt`.
5. **Disk gates ambiguous.** g1-m12 refused `disk-unfit` and g1-m04 refused `disk-short`, but volume free is 15 GB and 129 GB respectively. The `.cache/b/g/m` image partition is at 2–8 GB free on macOS workers. Evidence: `.../g1-m12/ready-log-workspace-failed.txt` disk facts, `.../g1-m04/ready-log-dependencies-failed.txt`. Doctor shows `storage.image-below-floor` on this host at 196 MB free.
6. **Host load and cargo idle holder.** Doctor `host.load-high` (load 40.7 on 10 cores). Pid 96461 holds cargo for 5521 s at 0.3% CPU. Evidence: `doctor-A2.json` findings, fleet `runs`. Not killed (read-only audit).
7. **Memory pressure and warden churn.** g1-m08 at guard level with 3053 MB available and 3265 MB swap, warden suspended 406 times. g1-m04 suspended 169. Evidence: `.../g1-m08/memory.json`, fleet findings `worker-warden-intervened`.
8. **Remote cache divergences and low AC/nx hit rate.** 957 AC divergences; nx hit rate about 0.2%; token absent. Evidence: `repo remote-cache status` (`server.divergences`, `server.counters`).
9. **Cache spool and upload failures on macOS workers.** `cacheSkipReasons` shows `spool-full` 27743 and `upload-failed` 14684 on g1-m01. Evidence: fleet `machines[].cacheSkipReasons`.
10. **Status readers and gaps.** Fleet `storage` and `workers` readers time out at 5 s; g1-m09 has no status; g1-server has no `ready.json`; doctor caches probe times out at 8 s. Evidence: fleet `sources`, `doctor-A2.json` `doctor.probe-timeout`, `/Users/Shared/semio-store/v1/origins/g1-m09/`.

## Notes
- Read-only: no code edits, no git writes, no processes killed, no tickets touched. Only this file and `doctor-A2.json`/`.err` were written.
- Raw `repo doctor` error text was in the `title`/`detail` fields; the `.err` file is empty.
