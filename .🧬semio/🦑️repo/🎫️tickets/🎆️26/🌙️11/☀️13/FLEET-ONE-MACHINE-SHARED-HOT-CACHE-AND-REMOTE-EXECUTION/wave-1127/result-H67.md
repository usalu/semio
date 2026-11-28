# H67: stage-failed runs since 08:00 CET on the 16 GB workers (measured 10:30-10:55 CET)

## Sources and their limit
- `S/origins/<o>/ready-timings.json` keeps every run (outcome, exit, signal, peak MB, seconds). `ready-log-workspace-failed.txt` keeps ONLY the newest failure per origin (overwritten each time), so the cause of an older run is only visible when it was the last one. Per-run causes below are therefore: evidenced (log of that run, marked E), inferred from the run's own record plus warden rings (I), or unknown (U).
- 24 non-ok runs ended since 08:00: 20 `stage-failed` and 4 `stalled` (m10 3, m07 1; stalls are the code-change/holder stops, not analysed here). Failed runs: m02 8, m07 6, m06 1, m08 1 (a 3.7 h run), m10 1, m12 2 (1 workspace, 1 dependencies), m15 1 = 20.
- Important: the timing outcome `stage-failed` is NOT the supervisor failure code. A run that the supervisor classifies `source-transient`, `memory-cancelled` or `interrupted` keeps a Ready worker Ready (`refreshKeepingFailureCodes`) but the timing still says `stage-failed`, so the A6 count "20 stage-failed" overstates how often workers lost Ready.

## Class table
| # | Class | Runs | Evidence | Flips Ready before the fix? |
|---|---|---|---|---|
| 1 | Stopped from outside by SIGTERM, sender unknown (exit null/143, or exit 1 with "SIGTERM reached this run ... the sender is unknown" in the tail) | 11: m02 08:22, 08:25, 09:19 (E); m07 08:00, 08:59, 09:02, 09:05, 09:11, 09:15 (E for 09:15); m06 10:00 (exit 143, no log left; its newer 10:24-10:34 run ended the same way, E); m15 10:01 (E) | `signal SIGTERM` + `endEvidence exit-receipt`; failed logs: "the stage process was ended by signal SIGTERM", "Cancelled (SIGTERM); leases released", Nx daemon `RepoError: missing caching.daemon.service-required` thrown 12 times on its shutdown (a symptom of the kill, not the cause); the m07, m06 and m15 logs show the stage was started by one supervisor and ADOPTED by another 1-8 minutes before the SIGTERM (m02 09:19 was not adopted) | yes when the worker refreshes: code `stage-failed` is real evidence, not in the keeping list |
| 2 | Graph slice worker ~3.3-4.2 GB on a 16 GB host, exit 1, tail not kept | 5: m02 07:57, 08:11, 08:15, 08:36, 09:10 | peaks 3460/4117/4196/4051 MB; warden ring m02 08:50:42 `cancel overdue slice-worker 4505 MB` matches the 08:36-08:50 run; the others have no ring entry left (rings keep 2 alarms) | I: memory/warden or "Slice worker exited" (already source-transient); not provable per run |
| 3 | Graph read hand-over churn: service superseded by newer code 3 times without progress, the client gives up (`maximumStalledHandovers` 3) | 2: m10 08:11-08:43 (E), m08 04:52-08:37 (E, 13519 s incl. >2 h waiting on `semio.host.capacity/v1`) | log ends with "The repo graph service ends (superseded) before this graph read finished ..." after three "handed over to current code; asking again" | yes: text matched no pattern, code `stage-failed` |
| 4 | Peer half-edit in the graph service | 1: m12 08:20 (E) | `Error: Closure reference proof leaf changed` (47 s) | no: already `source-transient` |
| 5 | Dependencies image deleted under the stage (coordinator outage 09:47, H63) | 1: m12 09:53 dependencies (E) | `ENOENT lstat .../♻️mit-bestand/node_modules` | no: already `source-transient` (ENOENT under the checkout root) |
| 6 | Nx deadline / capacity starvation | 0 as the end cause; m08's run waited 7800 s on host capacity before it ended | no `124` or soft-cap lines in any readable log | - |
| 7 | Real compile errors | 0 | none of the 7 readable logs; the earlier E0425 cases (H1) are already classified | - |

Counts by class (20 stage-failed): 1 = 11, 2 = 5, 3 = 2, 4 = 1, 5 = 1. Evidenced by the run's own log: 7 of 20 (m02 09:19, m07 09:15, m15, m10, m08, m12 x2); the other 13 are inferred from the run record (signal, exit, peak, seconds) and the warden rings.
Classes that keep Ready already: 4, 5, most of 2 (if warden attribution or "Slice worker exited with code null" matched). Classes that flip a refreshing Ready worker to not-ready: 1 and 3 (13 of 20), plus any class-2 run whose tail had no recognised pattern.

## Root fixes (all in `🚦️ready`, no graph code; LANDED 11:00 slot, typecheck 0 errors, 19 tests pass in external-stop/source-transient/refresh/warden-cancel; main `🧪️tests/🟦️.ts` result: see end)
1. `readyExternallyStoppedV1` (new, schema keys `externalStopSignals`, `externalStopExits`, `externalStopPatterns`): a stage that ended by SIGTERM/SIGHUP/SIGINT (signal or exit 129/130/143) or whose evidence says a stop signal reached the run is class `interrupted` (was `stage-failed`). `interrupted` is in `refreshKeepingFailureCodes`, so a refreshing worker stays Ready; its retry waits the shortest back-off (attempt 1 = 30 s) because a foreign stop says nothing about this machine. The EN/DE refusal text now says it was a stop from outside.
2. Timing outcome follows the class: `readyTerminalOutcomeV1` maps an external stop and a warden cancel to `interrupted` (was `stage-failed`), so `ready-timings.json`, the audits and the disk-growth statistics (which use only ok/stage-failed runs) stop counting foreign stops as failures.
3. Hand-over churn: the graph-read hand-over refusal (EN and DE text) joins `sourceTransientPatterns`. The stage is retried a debounce after the next change of the replicated tree (landing bursts end), the worker stays Ready while it refreshes. The `source-transient` texts EN/DE name the graph-service case.
4. Tests: new `🧪️tests/🛑️external-stop` (10 classification cases against an independent Python implementation, hand-over text both languages, refresh keeps Ready for `interrupted` and not for `stage-failed`).

## Not fixed, staged or open
- Who sends the SIGTERM is still unknown. All readable cases show an adopted stage (a supervisor restart minutes earlier) or a landing burst; the sender line says "unknown" because Nx only sees the signal. Evidence gap: the failure log is overwritten per attempt, so only the newest of each origin is readable. Suggested (graph-free, not done): keep `ready-log-<stage>-failed.txt` as an append-only ring of the last 8 failures per origin with the code line, so the next audit can classify every run.
- Graph hand-over limit (`maximumStalledHandovers` 3) lies in the sealed `B/🕸️graph` custody policy: staged proposal for after the seal window: count a hand-over as progress when the replaced service had served the read more than N seconds, or raise the limit during landing slots.
- Class 2 is the 16 GB capacity problem (slice worker 4-4.5 GB); owned by H46/H66, no change here.
- m08 run 04:52-08:37 held `semio.host.capacity/v1` shared for > 2 h: not analysed further.

## Verification
- Landed via `T/🧰️scripts/H67/apply.py` (atomic writes). Scratch-copy run before landing: 51 pass, the main ready test file could not run in the scratch tree (workspace root marker), so it ran on the real tree after landing.
- Unverified live: no worker has run the new code yet; the SIGTERM sender remains unknown. Stale `landing.lock.remote-execution` (H62, 10:00) left for the coordinator.
