# H40 result: offload liveness, cache 507, `repo cargo --fresh-lane`

Landed 05:00 (slot), script `T/🧰️scripts/H40/land.py`, landings.md entry written, locks released 05:03. Typecheck (11 files, 166 importers): passed. dangling-imports 0.

## (1) Why the offloaded 4 s unit waited 240 s
- 240 000 ms = `offload.deadline.unknownMilliseconds`: the unit had no recorded local cost. `offloadCostRecordV1` never records a first compile that was starved (CPU/wall < 0.5, `learnContendedFirst=false`), and every compile under load 50-100 is starved, so units stay "unknown" and got the 4 min value. The scheduler's own liveness already exists (worker heartbeat, lease 30 s -> `worker-lost`/requeue; `assigned` is the dispatch ack; the client re-attaches after 20 s of stream silence). What was missing: (a) the client never noticed a requeue (stage EXECUTING -> QUEUED), so a dropped worker with no replacement waited for the whole deadline; (b) the deadline for unmeasured/short units was far too long; (c) a worker that keeps heartbeating but never makes progress is only bounded by the deadline.
- Nothing reported earlier: the scheduler log has no per-assignment line (only refusals/declines), and the stream keepalive rows come from the server, not the worker, so they say nothing about the worker.
- Fix: floor 60->30 s, unknown 240->90 s (schema const + vectors), new `deadline.requeueMilliseconds` 20 s: when the operation goes back to QUEUED while executing, the client waits at most 20 s for another machine, then falls back (`infrastructure`, detail `requeue`, counter cause `requeue`). Worst case dropped worker: lease 30 s + 20 s. Tests (timeouts 19/0): requeue without machine ends < 4 s; requeue then another machine finishes; deadline numbers (policy-vs-python vectors updated).
- Not done: a worker "started" signal (spawned compiler) to cut the case "worker holds the task but never starts it" below the deadline; the deadline (30 s for measured, 90 s unknown) bounds it now. Idea: scheduler already holds `worker.usage` per task from heartbeats (only attached processes appear), usable as the start ack.

## (2) "fleet cache is full" with ~80 GiB free
- The 507 detail in `.cache/f/sidecars/build-cache.log` shows the cause: "store holds 84.46 GiB of its 84.5 GiB cap in force (74.23 GiB free, hard reserve 74.11 GiB)". Free space was only 0.1-10 GiB above the hard reserve (the "80 GiB" is not usable), the cap in force follows free-reserve, and the two unit-cache holders hold 56 GiB that nothing evicts. The remote blob bound (50 % share = 34 GiB) ignored those holders, so 56+34 > cap and every upload was refused until a collection by chance (trigger only after 5 % of the bound was admitted and 30 s).
- Uploader: a refused put is NOT lost: outcome `capacity` -> `retryJobV1` with exponential backoff up to `maximumBackoffMilliseconds`, dropped only after `maximumJobAgeMilliseconds` (counted `upload-expired`). H17's fresh-measure fix is landed but the server runs old code until hand-over; H17's census fix does not change the bound formula.
- Fix: `remoteCacheBlobBoundV1` = min(share, max(floor, cap - held - headroom)) from the census (held+pinned+leased+linked; census older than 6 h ignored), policy `blobRoom` {headroom 8 %, floor 5 %, 21600 s} (schema + policy + 3 call sites); `RemoteCacheStoreV1` calls `relieve()` after a `CasCapRefusalV1` (collection at most every 5 s) so the retry meets room. Tests: blob-bound 7/0 (room formula incl. Python model, relief after refusal).
- Takes effect at the cache server's next hand-over (not verified live).

## (3) `repo cargo --fresh-lane`
- Schema `🚪️launcher/🧬️schema` (option, Request.freshLane), parse (shared store only, repeat refused, bilingual errors), `acquirePooledLaneV1({fresh})` forces a new lane, the lane is removed after the call (`removeLaneV1` + retention), catalog flag + example en/de. Tests: launcher 11/0 (commander oracle + 3 fixture rows), pool-claims 8/0.
- Not covered by a test: the removal after the call (needs a real cargo run). MCP `cargo` tool has no fresh-lane parameter yet.

## Measured remote hit + sha identity (probe `T/🧰️scripts/H40/remote-hit.py`, `semio-framework-pets-clearance`, 05:40-05:45)
Command both phases: `repo cargo --store=shared --configuration=prototype --fresh-lane -- rustc -p semio-framework-pets-clearance --lib -- --cfg h40_<nonce>` (fresh key).
| phase | wall | units | local hit | fleet hit | compiled | primary |
|---|---|---|---|---|---|---|
| publish (fresh lane) | 138.4 s | 14 | 13 | 0 | 1 | miss, published, uploaded to the fleet within seconds |
| 5 local entries of the closure forgotten (operator command), hit (new fresh lane) | 161.1 s | 14 | 11 | 1 | 2 | `remote-hit` |
Byte identity: sha-256 of the unit entry outputs written by the local compile equals that of the entry the remote hit restored: rlib 6e112cd1..., rmeta 8a4e51ac..., semio-deps 24c5d39f... (identical: true). Wall is admission-dominated (the unit itself restores in well under a second); an earlier first run (461.9 s) hit an probe matcher bug (waited for the uploader under the local key instead of the crate), fixed.
Not verified: the fresh lane was removed after the call (no leftover checked), the offload fix and blob-bound live (sidecars run old code until restart/hand-over), no offload to an idle machine demonstrated.
