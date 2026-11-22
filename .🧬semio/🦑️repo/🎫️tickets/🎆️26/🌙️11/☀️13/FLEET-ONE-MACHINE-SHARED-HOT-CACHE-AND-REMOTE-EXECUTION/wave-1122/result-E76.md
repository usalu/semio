# Result E76 (scheduler-health)

## 1. Why g1-m05 (and m04, m01-wlan) are "lost" so often
Scheduler log `.cache/f/sidecars/build-scheduler.log`: 4061 joins, 514 lost; Windows 143 lost, linux 38, m04 106, m01-wlan 96, so not Windows specific.
Cause found in code: `register()` created a fresh random id on EVERY registration, and the worker agent calls `register()` from three independent places (heartbeat answered `known:false`, poll answered `known:false`, report got 404) besides the first start. After any scheduler restart or loss, those loops race: the log shows 2-3 `worker-joined` in the same millisecond for one process. The agent heartbeats only the latest id; every other id is an orphan that nobody heartbeats and the 30 s lease reports as `worker-lost` (log pairs: join at 14:43:29/31, two lost at 14:44:00/01). Second contributor: the sweep timer runs in the scheduler process on this loaded Mac (load 35-45); after a stall the timer fires before queued heartbeats are read, so live workers look stale (loss 80 s after join at 12:41:37 with a 30 s lease = stalled sweep).
Fixes (scheduler side, tolerates old agents such as m05's):
- `register()` is idempotent per name+origin: same worker keeps its id, counter `workersReused`, no join line.
- `tick()`: a sweep beat that came back later than 5x its interval judges no worker (counter `sweepStalls`); next beat judges on fresh marks.
Agent side (reaches machines on their next restart): register is single-flight (`registering` promise), heartbeats do not pile up (`beating` guard) and a lost heartbeat is logged once ("heartbeat lost: ...") instead of being swallowed.

## 2. Cancel churn
Per-cancel log line removed (event stage `cancelled` deleted). Event-sourced counters in the Counter schema (schema-first, `execution/🧬️schema/🌐️remote-execution.json`): `cancelledQueued`, `cancelledExecuting`, `workersReused`, `sweepStalls`, `queueWaitSamples`, `queueWaitP50/P90/MaxMilliseconds` (last 512 assignments); exposed in `/status`, `/v1/fleet/load`; the sidecar writes ONE `scheduler summary:` line per minute only when it changed.
Computed-twice finding: a requester that gave up kept its queued action alive for the 5 s linger and the scheduler could still place it on a worker, which compiled it while the requester compiled locally. Now an unwatched queued action (linger running) is not placed; if it started anyway it counts as `cancelledExecuting` (the measure of waste).

## 3. Queue timeout sizing (8000 ms, `timeouts.queueMilliseconds`) NOT changed
Measured: live sampler (`T/🧰️scripts/E76/queue-wait-sampler.ts`, 150 s, 32 actions): all started within the 1 s poll, p50/p90/max 0 s, none left queued. Log analysis of 4951 cancels: 43 % within 120 s of a worker join/lost/left (churn, no worker polls, more timeout would not help), the rest in saturation bursts (max 63 cancels per 10 s). Requester log (this Mac, 9 days): 159 queue-timeouts vs 181 offload runs; offloaded unit median 32 s. A queued action that is cancelled never started, so cancelling costs the requester only the waiting; a longer timeout would only add waiting in the storms. Without a case of a measured wait of 8-15 s I did not widen it; the new p50/p90/max counters give the exact figure after the next scheduler restart (read `scheduler summary:`), then size.

## Proof
scheduler tests 14/0 (3 new: idempotent register, stall-safe beat, abandoned action not placed + waits measured), other remote-execution files green (see landings.md "Wave 1122 E76").

## NOT verified
Live effect (scheduler on this Mac restarts with the new code only when its supervisor restarts it; m05 runs old agent code until the user restarts its replica, scheduler-side fix covers it); whether the Windows stall is the scheduler loop or the machine (no m05 worker log on this Mac); real queue-wait p90 under storm; the stall threshold 5x is untuned.
