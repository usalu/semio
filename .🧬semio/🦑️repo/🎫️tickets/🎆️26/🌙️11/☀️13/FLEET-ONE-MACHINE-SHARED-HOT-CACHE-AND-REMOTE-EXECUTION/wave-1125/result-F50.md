# Result F50: signed-fleet cluster test, 0 assignments (23:20)

## Cause: test environment, not a product regression
- Both agents registered and reported, but their worker state was `refusing`, code `disk-full`: "0.1 GiB free on d/m/remote-execution, 0 of 10 slots usable, one action needs 1.0 GiB". F48's per-agent test partition was a 96 MB APFS image. The worker's disk gate (correct product behaviour) left no slot, `headroom:false`, the scheduler kept the unit QUEUED (`/v1/fleet/idle`: workers 2, compatible 2, headroom 0, candidates 0) until the 120 s queue timeout.
- NOT the CPU gate: the test injects a host view (load 0.5 on 8 cores) into every agent, so real host load (21-38 here) is never read. NOT the resolver or identity: submit, register, claim and signed gRPC all worked. Run 2's "both agents report headroom" timeout was the same disk refusal.
- Fix (test only): 8 GB sparse APFS image per agent; failure of the headroom wait prints the last load table. Then offload worked (10 s) and two stale expectations surfaced: the input manifest of the slot is a second action-cache entry carrying the slot (`ac.puts` 2 + manifests, `slotted` 1 + manifests; same law as the unit-cache fleet tests). Test now 1/0.

## Live (`repo remote-execution status`, 23:03)
- Scheduler on g1-server started 21:52 (counters reset then): queued 0, executing 0, completed 1, refused 0, offloadRejected 0, queue wait p90 1 ms. 33 workers registered (all seen <6 s ago), 16 with headroom, 17 refusing. offloadedIn since 21:52: 1 (g1-m12), refused 0. A2 at 16:48 had 1,374 in / 30 refused over the whole run, so the counters are not comparable: since the 21:52 restart only one action has reached the scheduler.
- Refusing (headroom false): g1-m02 x3, m03 x2, m08 x3, m13 x2, m14 x4, g1-server x3, i.e. the loaded Macs (the CPU gate, load per core >= 1.5; coordinator load was 22-38 on 10 cores). With headroom: m01, m05 (win), m06, m07, m10, m11, m12, m15. Also dead workers still registered (several per origin).
- Requester side (`.cache/f/offload-counters`, this machine): last `out` write 20:05; reasons since: `local-slot` (latest 22:52), `final-wait` 22:51, `infrastructure` 22:10 (fellBack), `no-endpoint` 21:56 (around the resolver landing), `unmeasured`/`not-portable` 21:56-21:57. So offload requests are few because builds are quiet (workspace failures) and mostly stay local by `local-slot`; no evidence of a fleet-wide regression, but also no fresh end-to-end offload proof on live hardware after 21:52.

## Follow-ups (not done)
- Several registered workers per origin (restarts leave old ids); expiry is by lastSeen only.
- `no-endpoint` at 21:56 deserves a check that the resolver never returns none while the facts exist (not reproduced).
Landing: T/landings.md "Wave 1125 F50".
