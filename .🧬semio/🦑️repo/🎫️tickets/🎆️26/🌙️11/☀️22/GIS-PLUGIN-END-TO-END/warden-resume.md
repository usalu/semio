# Warden resume deadlock: a swapped-out subject was never resumed

## Root cause
Pid 53028 (slice worker, 5777 MB, claim grant 6255 MB, so sheltered) was suspended at 14:13:17 (pressure critical, 2295 MB available). No resume or cancel alarm for it exists in `alarms.jsonl` afterwards; the warden's `warden.log` shows only the suspend lines.

The resume rule in `decideHostWardenV1` is not "room for the whole footprint" but "page-in fits the room": a suspended subject resumes only when `pageIn(subject) <= available - guard` (after the host is quiet, `available >= resume`, `calmTicks` calm ticks). `pageIn` is the subject's non-resident (swapped) megabytes, about 5769 MB for a fully swapped 5777 MB worker. At 14:27 the room was 3488 - 1638 = 1850 MB, and a swapped worker can never get 5769 MB of room on a 16 GB host. The overdue path (120 s) was also closed: a sheltered subject is never cancelled as overdue (only at severe danger), and the overdue resume had the same page-in condition. So: no resume, no cancel, suspended forever. The hypothesis is right except that the budget is the swapped part (page-in), not the full footprint.

## Fix (schema first, pure decision, loop)
- Schema: policy `suspendHardMaximumSeconds` 1800 and `resumeSettleSeconds` 30; `Previous.resumedSeconds`; reason `limit` (Action, Alarm, Cancel) with bilingual text ("stayed suspended beyond the hard maximum without the host recovering; cancelled so its client can retry" / German).
- Recovered host (quiet or calm, available at or above the resume line, `calmTicks` ticks): resumes one subject per tick, then waits `resumeSettleSeconds` (loop tracks the last resume). Before a subject is overdue (120 s, 600 s for a final link) its page-in must still fit the room (cheap resumes preferred); once overdue it resumes whatever its page-in cost. An unsheltered cancellable subject that does not fit is still cancelled as before (its owner booked nothing).
- Order: stuck first (suspended at least 1800 s), then largest grant (megabytes without grant), then longest suspended, then dearest. Largest first because it holds the most work and needs the longest to page in, so it goes while the host has the most room; the small ones page in cheaply afterwards. Oldest-first would let a long small one block the big one.
- Maximum suspension: at 1800 s a subject is resumed first on a recovered host; otherwise cancelled with reason `limit` (cancellable), or resumed (not cancellable). Nothing stays suspended indefinitely.

## Tests
- Oracle `🧪️tests/🐍️.py` mirrors the law; 10 new rows in `🧪️tests/🧫️fixtures/🔣️.json` (14:13 to 14:27 scenario, below the resume line, largest grant first, settle window 10 s and 30 s, no grants, limit cancel at 1800 s, 1799 s not yet, non-cancellable resumed, stuck first). One expectation changed (calm hysteresis resumes the largest, was the dearest). Swap-churn `resume-room` rows and sqlite oracle updated (overdue non-cancellable resumes on a recovered host).
- `bun test` over the warden tests: 10 pass; `🧪️tests/🟦️.ts`: 58 pass, 2 MOUNTED real-process tests fail (also fail on the unmodified code, they depend on real child processes and timing here).

## Live
The warden restarts itself when its source digest changes; pid 6650 is gone, pid 52203 runs digest 75068e1eb1a78d39, which equals the digest of the current sources (mode enforce). At 14:35:46 it suspended 53028 again; the worker was at 111 s suspended, then cancelled as overdue at 14:37:47 (it had grown to 6258 MB, 3 MB above its 6255 MB grant, so it was no longer sheltered; host in warn): the bounded cancel path, so the client can retry.
