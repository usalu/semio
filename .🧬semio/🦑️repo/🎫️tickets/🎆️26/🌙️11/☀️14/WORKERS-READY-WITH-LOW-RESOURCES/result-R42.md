# R42: a running graph read survives every restart (detached runner, adoption)

Written in the replica `/Users/ueli/Documents/semio`: at 14:50Z the live tree `/Users/Shared/semio` disappeared completely (not by me: my only recursive deletes are scratch copies under /private/tmp), so `N` there could not be written.

## Cause, measured
Every restart of the build agent (closure watch, exit 75, or any stop) stopped its graph child with SIGTERM; the graph child answered with SIGINT to `repo show projects`. R31's deferral only covered the graph child's own closure restart. So the read was lost on every agent restart and every stall stop. Option (a) chosen (restart-proof for every cause); (b) not needed.

## Built (landed 14:33Z, `T/🧰️scripts/R42/`, `landings.md`; second docstring-only landing 14:47Z)
- Runner (`sidecar/🕸️graph/🏃️runner`): started detached (own session), spawns the read, appends its stderr to `.cache/f/graph-server/read.log`, rewrites `read-record.json` (schema `ReadRecord` in the wanted schema) every 2 s, records the exit status, stops the read when its watch file is not touched for 30 min (bounded orphan).
- `sidecar/🕸️graph/🔭️read`: `inferGraphReadV1` (start or attach, never a second read), `adoptGraphReadV1` (attach at server start, also to a read that ended unwatched), `liveGraphReadV1`. Cancelling the server only lets go ("detached"). Replay of the old log tells no wave twice.
- Wanted server: `adopt` input asked before the first round (works with no headroom), keeps the read's priority file when the held list is identical so `pending.json` stays valid and fresh projects are told in the first round after a restart.
- Cold reads: `createColdPendingV1` (steering) + hook in `discoverBuildGraphPartV1` write `pending.json` as each owner finishes (rate limited 1/s, trailing), so held requests get `N of M projects ... are fresh` before the generation exists. Previously pending was written only after a generation (`if(!cold)`).

## Proof
- Tests (all green on the live tree after landing, 17 directories): `🕸️graph/🔭️read` 8 (SIGKILL of the starting process: runner and read live, next process attaches, 1 start for 12 waves, generation 12; detach and adopt; orphan expiry; ended unwatched; crash then new read; stale/damaged records; schema; env and args), `🙋️wanted/🧪️tests/🔭️adopt` 3 (mutation check: without priority reuse it fails), steering `🌡️cold` 4. Existing restart (15), drain, resume, restart-work, server suites unchanged and green. One flake in the landing run (orphan test, 900 ms too tight), fixed to 2500 ms, 3 x 8/8.
- LIVE, build-agent.log: 14:49:34.696Z `agent code changed, restarting` (my docstring landing) -> `graph-server: left the graph read (pid 57538) running` -> 14:49:35.782Z `attached to the graph read (pid 57538) that has run for 899 s; it is not started again`; the read kept its pid and its 15 minutes. This is the case from the brief (agent restart with read in flight) and it no longer restarts from zero.
- 14:50:24Z a second "left the read" was logged and at 14:50:25Z the record shows status 143 (SIGTERM to the read itself). I could not see the cause: the live tree vanished at about that time. Unverified whether the SIGTERM came from the disappearing tree or from something else.

## Not verified
- Whether the next read resumes at owner k: np records exist (7815 files, ~100 written per hour) and R36 saw owners 1-11 in 20 s on a repeat, but no read reached the owners phase during my watch (all reads were in "Verifying input closures"/facts, 8-15 min, R41's work, not landed when I looked). Not measured.
- A real `... of ... projects of <origin> are fresh` answer from the cold hook: tested by law tests only; the discovery hook is covered by a source-contract test (the repo's own pattern), not by a real cold read. Told-fresh means requesters skip those projects and take records from the fleet cache; the upload timing per owner was not checked.
- R31's serve deferral still defers a graph-child restart for code changes while a request read runs (up to 60 min); with adoptable reads it is no longer needed and could be removed (would change 6 R31 tests). Not done.
- Windows/Linux, `tsc`, other machines, a real `repo` build through the path.
- Orphan guard: a runner killed with SIGKILL leaves its read without a runner; the next server would start a second read (the capacity gate serialises).
