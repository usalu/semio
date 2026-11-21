# Replica store guard during incoming stage work

A replica under pressure reported a supervisor `loop-blocked-restart` and later received the authoritative stage while still applying 1,696 incoming items. The replica has two different safeguards. The supervisor detects event-loop blockage. The 10-second store turn guard abandons a turn only when its reachability probe and last confirmed work progress both exceed the deadline. These observations establish a store guard liveness gap with small items. They do not attribute the observed supervisor restart to that gap.

A validated journal slice now renews the guard only when the admitted cursor strictly advances from the cursor used to admit it, including after a snapshot adoption retry. A completed materialisation renews the guard when it actually resolves a pending queue item and the turn remains live. Empty polls, held future journal entries, dropped or still queued items and late completions after cancellation do not renew it. Existing byte-based renewals continue.

## Verification

The causal fixture uses zero-byte files, a serial materialisation queue, the actual store guard and a probe that stays pending. Before the patch, six completions spaced 80 ms apart expired against a simulated 200 ms deadline. The stall case observed no cursor or item progress beats before its deliberate deadline gap. With the patch, the uninterrupted batch finishes and the final unchanged-head follow adds no heartbeats. The stalled batch records its cursor advance and both completed items, expires, and emits no new heartbeat when the in-flight rename finally returns. The independent Python oracle evaluates the same cadence vectors and validates the fixture with jsonschema Draft7.

The full replica race suite passed all 32 cases with the repository's ticket artifact environment configured. The final late-return assertion passed in both focused cases.

## Durable state boundary

The journal cursor, merged registers and applied queue effects are persisted after the follow apply phase, and the index host settles completed entries in replica state. A process kill before that persistence can require rereading admitted spans. The observed live sequence provides no evidence of an incorrect index settlement, a repeated exact stage handoff or a lost authoritative stage. The patch changes guard renewals only. No service or live cache was edited.
