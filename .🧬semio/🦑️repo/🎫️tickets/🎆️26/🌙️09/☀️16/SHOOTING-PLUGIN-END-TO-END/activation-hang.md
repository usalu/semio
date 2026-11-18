# Activation hang: findings (status: two defects fixed and tested, activation not completed)

## Where the 9.4 h run stalled
Rerun of `repo run @semio-tech/framework-os-dev:activate-shooting-react-prototype --lang=en --output=stream` reproduced a silent stall within seconds of "Planning 1 targets.":
- The front door (pid 6726) has no child process and one open unix socket to the resident graph service (pid 25029).
- The service has one slice worker, busy with another agent's cold graph inference of the same checkout (about 269 of 4774 projects, hours left).
- The slice pool (`slice-pool/🟨️.mjs`) runs loads of one checkout strictly one after another (they share one fact store). A load queued behind another one sent no progress and had no deadline or way out, so the client printed only "[repo] still working after N s; last phase started: door.script".
- No task started: the `repo cargo test` processes seen in `ps` belonged to a peer (their stdout is that peer's `run-base.txt`), not to this activation. The claim that a nested cargo test inside activation streams nothing is therefore NOT confirmed; the cargo build and assert steps use `stdio: inherit` (`testing/execution`), so their output reaches the process that runs them. No change was made there.
- The capacity growth waits (2206 s, 8627 s) in the old log are a different thing: the grant grows while the run is alive; they did not block anything. The old log's slice progress ended at 2216 s and nothing else is known about the last 7 h, because that process tree was gone when I looked. A second, rarer cause was found by reading the code (below).

## Defects fixed
1. Silent queue in the slice pool: a queued load now reports a heartbeat decision (`queued-root`, `queued-pool`) with bilingual texts, leaves the queue when its client leaves, and an aborted load keeps its place so a later load cannot overtake the running one.
2. Lost wakeup in the host service: a client attaching to a job while it was binding replies to the clients attached before it was resolved with an empty outcome and never got a reply, leaving its connection open for ever. Every attached client is now answered, and a client still unanswered when the job ends re-attaches to a fresh job.

## Files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`
- `.../🏡️resident/🏛️service/🌐️host/🟨️.mjs`
- `.../🏡️resident/🧬️schema/🔣️.json` (keys `host-slice-owners-queued-root`, `host-slice-owners-queued-pool`, en and de)
- new tests `.../🏡️resident/🧪️tests/⏳️queue-progress/🟦️.ts`, `.../🧪️tests/🤝️late-attach/🟦️.ts`

## Test results
`repo tools exec bun -- test` on `⏳️queue-progress` and `🤝️late-attach`: 4 pass, 0 fail, 25 expect() calls, exit 0. I did not run them against the unfixed code, so that they fail without the fix is not shown.

## Activation
- First rerun (old code): after the peer's load ended, the slice worker restarted 4 times with exit 75 (the replica code changed under it, caused by these edits) and the run ended with exit 1 and the typed refusal "try again once the replica has settled". Not a hang.
- Second rerun (fixed code, `🗑️generated/activate-rerun.txt`): the run now prints "Reading the project graph slice (1807 s): waiting for the load of this checkout that is running ahead of this one (1 load(s) of 2 workers busy) ..." every 2 s instead of silence. After 30 min it was still queued behind another agent's load, so there is no activation exit code yet.
- The shooting lib tests (`native-fixtures-green-2.md` of the other agent) did not exist at this time.

## Open
- Let the second rerun finish and record its exit code; rerun once the graph slice is available.
- A queued graph read still has no upper deadline by design; it is now audible.

## Third hang (8.6 h, pids 11698/11799/11855): diagnosis, fix, results
### What the live state showed (before anything was killed)
- 11855 is the graph service itself (descriptor `host-4bdd0c801786.json`), 11799 its launcher, 11698 the front door. The door holds exactly one connection to the service (peer fd pairing via `lsof`); both socket queues were empty. "[repository:command] still running" in the service log is the service's own watchdog, not our job.
- A status request to the old service (`🗑️generated/old-status.mjs`) answered: `plans.total 4`, `budgetRefusals 3`, `clients 2` (door plus the probe), `queue 0`, `inFlight 0`, `workspaces []`. So the service held NO running job: nothing could still be working for the door.
- Service log timeline (seconds since service start): slice progress reached the door until about 2583 s; at 2571 s a worker refused a heavy owner with exit 75 and the pool handed it off and started further loads. The door printed nothing after that (no handoff text, no later slice lines, no refusal), while the service kept loading until 13343 s and logged three `graph-plan-too-heavy` refusals (5542 s, 8963 s, 13343 s), then went idle. The earlier hang in this file also stopped printing at about 2216 s, again after a cut/handoff.
- The service key file was replaced at 05:11, so the old service rejects every new client ("capability forged"); a second probe attaching to the same job was therefore impossible.
- Root cause: NOT proven. What is proven: a request can be unknown to the service (no job, `inFlight 0`) while the door waits on an open connection with no deadline, and `exchange` in `🚪️client/🛰️plan` waited for ever for a frame that never came. A end-to-end probe with the real service, real pool and a fake worker (owner-too-heavy handoff, then done) delivered every progress frame and the reply, so the happy handoff path is not the loss; the trigger of the orphaned request remains open (candidates: a client dropped from its job without an answer, or a refusal sent to a connection the door did not process).

### Fix (wait always carries a liveness check; schema first, bilingual)
- Schema `🏡️resident/🧬️schema/🔣️.json`: policy `requestKeepaliveMilliseconds` 10000 and `planSilenceMilliseconds` 120000; progress phase `alive`; texts `host-alive` and `host-silent` in en and de.
- Service `🏛️service/🌐️host/🟨️.mjs` (+ `.d.mts`): every plan request gets a keepalive frame after a quiet interval, but only while a job holds the client or before it is attached (`keepaliveDueV1`), so a request no job holds falls silent instead of being masked.
- Client `🚪️client/🛰️plan/🟨️.mjs` (+ `.d.mts`): `exchange` fails with the bilingual `HostUnavailableErrorV1("silent")` when no frame arrives for the declared silence; `requestPlanV1` asks again like after a lost connection (bounded by `reattachMaximum`), then reports it. `🧭️invocation/🛰️plan/🟦️.ts` shows the bilingual detail in the door refusal.
- New test `🏡️resident/🧪️tests/💓️keepalive/🟦️.ts`: policy bounds, `keepaliveDueV1` truth table, lost reply (fake server) ends `silent` in both languages while the same request without a deadline is still pending after 0.8 s, and a slow real job stays alive under a short deadline thanks to keepalives. No third-party oracle was added (the contract is timing/protocol behaviour).

### Test results
- keepalive: 4 pass, 0 fail. `🧭️invocation/🧪️tests/🛰️plan`: 13 pass. late-attach, queue-progress, protocol, plan-refusal, failure: pass.
- Pre-existing failures found in neighbours, not touched: `🌐️host` (4: pool cut levels make `overBudget` 7 not 1 in two memory-guard tests, disk refusal text lacks `repo cache cas gc`, separate-process env test ENOENT) and `🛑️cancel` (4 cgroup limit tests expect fixed limits, host scales them).

### Activation
- Killed 11698, 11799, 11855 (TERM, gone after 8 s).
- Rerun 2 (`activate-rerun-2.txt`): exit 1, typed bilingual refusal "slice worker ended 4 times with exit code 75, code kept changing" (replica churn by concurrent edits).
- Rerun 3 (`activate-rerun-3.txt`): exit 1, "resident service met an unexpected failure (5572a035)": `Resource lease timed out: graph-custody ...generations`, another agent's process 39598 held the exclusive graph-custody lease about 10 min. A bounded, loud wait, but shown as a generic failure.
- Rerun 4 (`activate-rerun-4.txt`): see below.
- Rerun 4 (`🗑️generated/activate-rerun-4.txt`, fixed code): after about 1 h it is still reading the graph slice (774 s into the current cut level, progress lines every 2 s, one cut after the worker grew to 10447 MB). No exit code yet and the shooting cargo tests have not been reached. The process (pid 48883) is left running; the exit code belongs under this line once the file ends with `EXIT <code>`.
