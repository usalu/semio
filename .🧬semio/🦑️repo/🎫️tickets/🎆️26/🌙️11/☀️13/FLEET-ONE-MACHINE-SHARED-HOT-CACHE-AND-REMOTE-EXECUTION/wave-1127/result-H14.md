# H14 result: fleet cache server reliability

## Findings (evidence: `.cache/f/sidecars/build-cache.log` + `.log.1`, `remote-cache status`, counters.json)
- The "36 restarts" are NOT crashes. `ledger.restarts` 37 (since 11-25) = orderly hand-overs: 35 in the current log over 21 h (every ~30 min). Cause: the cache role's code closure is wide (cas, host-capacity, native-build ...), every landing in it restarts the sidecar after 5 min quiet / 30 min max / 30 min minimum interval. Each hand-over is zero-downtime by design (successor binds with reusePort, old drains 20 s) but costs 22-37 s startup scan with 2-6 s event-loop stalls and ~1 GB RSS churn; no crash, no OOM, no warden kill found (no "ended with status"/SIGKILL in the 21 h log).
- Warden: `🛡️warden/🧬️schema` "Never" table already excludes every `🛰️sidecar/` role (cache server included), so it is never a subject.
- Real failed starts were found only on 11-25 19:43-22:18Z (`.log.1`: 568x "Another remote cache server uses ..." and "Listen on the fleet interface, not on a wildcard address"): a second launcher generation crash-looped against the lock while the first generation kept serving. Noise, not an outage; it ended by itself.
- The multi-hour "down" seen by workers (04-08Z 11-26) was the capacity episode: server alive (pid 17090 06:40-08:40Z) but volume 96 % full (36.7 GiB free < 74 GiB hard reserve), 1.13 M puts answered 507, plus guard waits (3 x 30 s) longer than the client's 20 s blob deadline = "no answer" -> client breaker `down`. Already fixed by F79 (single 15 s admission budget), F80 (+86 GiB freed), F82 (volume relief before the CAS reserve, floor rows); the log shows the relief running at < 2x reserve ("needs 148.22 GiB").
- Why 507 is legitimate when it happens: the 52 GiB in the CAS store are hardlinked by two live unit-cache holders, so evicting held objects frees no bytes; only the volume's other writers can. I did not add held-object eviction (it would destroy the build cache without freeing space).
- Live now: free 93-94 GiB (reserve 74): close again; session since 23:18Z: cas puts 3441, rejected 27 (0.8 %).

## Changes (landed, tested)
1. Cache role restart policy (`🛰️sidecar/🧬️schema/🔣️.json`): quiet 15 min, max deferral 3 h, minimum interval 3 h (was 5 min / 30 min / 30 min): at most ~8 hand-overs per day instead of ~48, still only for changes inside its own closure. Test `the cache server ... at most every three hours` + the in-flight test widened to 400 ticks: restart tests 24 pass.
2. Server refusal log (`🖥️server/🚨️refusals/🟦️.ts`, wired in `🖥️server/🟦️.ts`, `🛰️sidecar/🗄️cache/🟦️.ts`): every 507 and 503 busy is counted and logged in EN/DE, first at once then one summed line per 60 s with the capacity detail (holders, cap, reserve), flushed on stop. Previously the log carried no 507 at all (counter `cas.rejected` mixed 400/413/507/503). Tests 3 pass; server suite 10 pass; typecheck 0 errors; dangling imports 0.

## Before / after
Before: ~48 hand-overs/day, 0 server-log lines for 507. After: NOT measured live (the sidecar picks the new code up at its next hand-over, minimum interval of the old policy 30 min; the running pid 41367 started 00:18 with the old code). Read afterwards: `grep -c "cache handover: successor (pid" .cache/f/sidecars/build-cache.log` per hour and `grep "507" ...` lines from the new refusal log.

## Unverified / open
- No live hand-over timing or 507 rate after the change; uptime proof pending the next hand-over.
- Client breaker (downFaults 2 / 15 s window) untuned, not touched (uploader H3 area).
- Free space is again near the reserve (93 GiB): if other writers keep filling the volume, 507 returns; needs a human/disk guard for non-store data (F80 candidates: Docker.raw 70 GiB).
