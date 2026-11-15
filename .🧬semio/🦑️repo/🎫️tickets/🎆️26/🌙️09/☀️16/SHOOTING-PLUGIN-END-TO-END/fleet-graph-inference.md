# Fleet graph inference delivers nothing

## Root cause
Graph requests (`graph-state/wanted`) are answered only by `runWantedServerV1`, run by the sidecar `remote-execution/sidecar/graph`. It claims nothing unless `host.totalMegabytes >= serveTotalMegabytes`, which was 24000. Every fleet machine is a 16 GB Mac mini (hw.memsize 17179869184 here), so no machine ever served: `repo fleet status` shows "Graph: no builder; 0 computer(s)", "worker idle" (that is the cargo worker, not the graph server), and requests expire silently (300 s) with 0 delivered. Platform keys and queues are not the cause: the request slot is per requester and the platform class matches (macos-arm64 machines).
A second gate, a request read only starting at 9400 MB free (`serveHeadroomMegabytes`), could not be met on a busy 16 GB host either.

## Estimates
"about 9854 MB" is the recorded wave-peak cost (managed MB) times `footprintFactor` 2 (display in physical MB). Wave evidence attributes the peak above the survivor floor to the single project at the peak, so concurrent allocation and uncollected garbage inflate it. It is an overcount, not a real need; heavy owners are never refused when a solo grant exists (`planSoloV1`), they are only asked from the fleet first. With serving machines present, the heavy ones are inferred there and the estimate no longer blocks a busy host. The installed `repo` binary also reports "code older" and prints a refusal (36 local, wave > 0) that the current `fleetReadV1` source does not raise (it refuses only when no wave fits), so activation of current code is needed.

## Fix
- `graph-state/wanted/schema`: `serveTotalMegabytes` 24000 -> 15000; new `requestHeadroomMegabytes` 6144 (a requested read runs in waves that give way, so it needs less than a refresh at 9400).
- `graph/generations/remote-exchange/schema`: `servingTotalMegabytes` 24000 -> 15000 (same gate, second copy, head-rank test pins both equal).
- `graph-state/wanted/server`: a held request starts a read at `requestHeadroomMegabytes`.
- Tests: `wanted/tests/server` (new test: 6143 MB no read, 6144 MB read), `sidecar/tests/graph` (new test: a 16384 MB host serves, 8192 MB does not).

## Results
`repo tools exec bun -- test`: wanted server 15 pass, priority 3, read 16, two-platforms 5, sidecar graph 9, head-rank 5, all 0 fail.

## Open
Serving machines must restart their graph sidecar on the new code (sidecar restarts on closure change). Not verified live.

# Plan stage "result" refusal (unit-cache-test)

## Finding
The text "the plan stage result (during result) grew to 7050 MB" is not the in-process plan guard. It is the slice pool (`resident/composition/host/slice-pool`), which measures the footprint of the whole worker process tree and, above the 40 % limit (6553 MB), settles once and then judges with `planGuardVerdictV1`. "result" is only the last stage the worker reported. Earlier stage logs show the `result` stage itself costs 0 MB; the growth is the inference of the stale slice (167 owners, because live replica edits dirty many owners), and the tree (7050 MB) is larger than managed memory (about 3.3 GB) through native memory and children. A `repo run` of one target needs a current graph, so its plan is the stale slice, not the target's closure; it cannot be bounded to the closure without changing what a current graph means.
The refusal carried `available 2998 MB` which is above the 2048 MB reserve, so the only reason that can have fired is `pressure` (`warn` refused), the normal state of a busy 16 GB Mac. It killed a read that had run 2000+ s with all its work lost.

## Fix
`planGuardVerdictV1` (`slice-pool/limit`): only `critical` pressure refuses by pressure; `warn` falls through to the reserve check (refuses when available is below 2048 MB). Test `resident/tests/plan-guard`: warn with 2998 MB continues, warn with 1500 MB refuses `memory`, critical refuses `pressure` (10 pass); `plan-refusal` 4 pass.

## Not done
No per-stage memory breakdown of the slice (needs a live run with `SEMIO_GRAPH_PHASES=1`). The real run is in `generated/unit-cache-test-3.txt`; a resident service started before the change keeps the old code until it restarts.

## Real runs after the fix
- Run 3 (`generated/unit-cache-test-3.txt`): past the plan-stage verdict; stopped by `graph.owner-too-heavy` for `@semio-tech/async-macros-rs` (4250 MB physical estimate vs 3512 MB solo grant, which is capped by the memory available at that moment).
- Run 4 (`generated/unit-cache-test-4.txt`): the heavy owner now ran locally; the worker tree grew to 9992 MB during inference, above the absolute ceiling (60 % = 9830 MB), with 2804 MB available. The ceiling refuses always. The host is saturated (several Claude sessions, builds, load 15); no fleet machine answered, no fleet wait line appeared.
- Conclusion: a 16 GB host that is also busy cannot read the stale slice locally. The remaining fix is operational/deployment: the fleet graph sidecars must run the new serving gate so heavy owners are served remotely. Not verified.

# Graph plan waits for the full 6553 MB while 5515 MB are free

Cause: `decideHostCapacityV1` (`host-capacity/decision`) skipped every admission, including the shrunk start with `minimumMegabytes`, whenever a soft pause was active (`pressure-warn`, the normal state of a busy 16 GB Mac), and then reported the reason as "memory" (since the full ask did not fit). Only the override after 120 s (`overrideAfterMilliseconds`) admitted the shrunk start. Fix: a request that can shrink and fits at its minimum starts at once under a soft pause when nothing of ours runs or was admitted before it. A full ask that fits is unchanged. Test `host-capacity/tests/partial-start` (2 pass; the warn case fails without the change); `host-capacity/tests` decision and SQLite oracle suite unchanged (the oracle does not model `minimumMegabytes`).

# Profile of a live graph read (not obtained) and the stage label

- `repo show project @semio-tech/shooting-js` fails before any graph read in `source.capture` ("Original Native terminal record changed", exit 1; unrelated to memory). `repo show projects` with `SEMIO_GRAPH_PHASES=1 SEMIO_GRAPH_FLEET_READ=off` sat in `source.served` for over 30 min behind other repo work on this saturated host and never reached the plan stages, so no stage or tree breakdown (heap vs native vs children) was measured. No before/after peak exists; no memory reduction was made. The sampler script is `profile-tree.sh` (ps RSS by command, 10 s).
- What the activation log does show: the worker spent its time in the load phases `verification` (4510-4553 owners), `rederivation`, `facts` (120475) and `flush`, i.e. before the plan stages, and the tree already passed 6553 MB there. The next step is to run one read with `SEMIO_GRAPH_PHASES=1` on a quiet machine.
- Fixed: the label "?" came from the slice pool using only the plan-stage message; before the plan stages begin it now falls back to the worker's progress phase (`verification`, `facts`, ...). Test `resident/tests/plan-guard` (11 pass).
