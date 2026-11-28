# H16 result: fleet as one machine, end-to-end probe (Rust slice measured; TS slice running)

## Probe (kept): `T/🧰️scripts/H16/probe.py`, `ts-probe.py`, `gate.py`
`python3 probe.py rust --crate <package> --out <dir> [--only up,down,down-wipe]` (python3 + `repo` only). Each phase runs in its own agent session; per-phase counts come from the unit-cache event logs (`.cache/b/r/m/u/x/<lane>.jsonl`, `offload.jsonl`), lane found by the primary crate's events in the phase window. Fresh key without editing the tree: `cargo rustc -p <crate> --lib -- --cfg h16_<nonce>`. Local entries cleared per crate with the unit-cache operator command (`bun <unit-cache script.ts> --semio-unit-cache forget --crate X --root .cache/b/r/m/u`). Gate: memory only (coordinator addendum), niced.
Crate: `semio-framework-pets-clearance` (3049 lines, 3 path deps; pets tree untouched for hours). TS: `@semio-tech/pets:typecheck`.

## Rust table (ground truth from event logs; wall includes admission queues)
| phase | units dispatched | local compile | fleet hit | offloaded | unit compile / cache ms | wall (s) | result |
|---|---|---|---|---|---|---|---|
| cold natural key, remote up (11-27 00:54) | 3 (contract, kinematics, clearance) | 3: 67.6 s, 4.6 s, 4.2 s | 0 (fleet had never seen them) | 0 | publish 0.2-0.6 s each, upload 0.08-0.4 s | 179 | ok, all 3 now in the fleet |
| fresh key, remote up (01:30) | 1 | 1 (289 s total) | 0 | tried 1, fell back after 243.9 s: `infrastructure`, "deadline 240000 ms" | 5 x `upload-capacity: the fleet cache is full` over ~9 min, then uploaded | 888 | ok but slow, FAULT 1+2 |
| fresh key, remote up (run 4) | 1 | 1 (704 s compile incl. waits) | 0 | 0 | published 15 s | 1293 | ok (queue) |
| fresh key, remote up (run 5, "up") | 1 | 1 (13.1 s) | 0 | 0 | upload 0.14 s | 223 | ok |
| warm, cold cargo lane, local entries present | 14 (3 + externals) | 0 | 0 | 0 | local hit 114-217 ms per unit | 1080 (queue) | ok |
| hot (cargo fresh) | 0 dispatched | 0 | 0 | 0 | none | 82-125 | ok |
| remote DOWN (cache + execution endpoints closed, with `SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE=1`) | 1 | 1 (4.0 s) | 0 | 0 (no offload attempt) | `remote-skip fault` in 9 ms, no claim wait | 160 | ok, exit 0 |
| remote DOWN + closure local entries forgotten | 1 | 1 (3.0 s) | 0 | 0 | `remote-skip fault` 11 ms | 128 | ok, exit 0 |
Down vs up for the same size of work: 4.0 s / 3.0 s local compile versus 13.1 s with the remote up (run 5): the down path adds no measurable time (fault answer 9-11 ms, no retries on the build path); the uploader keeps `upload-retry` every 15 s in the background without blocking.

## Faults and findings
1. FAULT (H4, offload): an offloaded 4 s unit waited the full 240 s deadline ("compiling on another machine") and then failed with `infrastructure`; the build lost 4 min. Needs an early liveness/progress check from the worker (heartbeat missing after N s -> fall back), and a deadline proportional to the measured local compile time (here ~4 s).
2. FAULT (H3/H14, cache server): `upload-capacity: the fleet cache is full` (507) at 01:30-01:40; the first publish of a unit took ~9 min to land (retry every 15 s). The cache server evicts too late; hand to H14 (findings-H16.md).
3. Env trap, fixed in the probe: a closed-port `SEMIO_REMOTE_CACHE_ENDPOINT` alone is ignored (fleet facts supersede a pointer that names none of the healthy endpoints); remote-down simulations need `SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE=1`. Without it my first "down" run silently used the live fleet (claim granted, remote-miss in 119 ms). Same for any future test of the down path.
4. `repo cargo` refuses `clean`, and forwards only some rustc flags (`--diagnostic-width` refused: "compiler forwarding belongs to original repository admission"), and a new agent session of the same client reuses the same lane with its warm cargo target. Consequence: with the shared lane, a unit with an unchanged key is never redispatched, so a cold-target fleet fetch of closure units and a byte-for-byte fleet-vs-local sha check cannot be produced from outside without deleting lane files (forbidden by the rules). Needs a tool-side switch (e.g. `repo cargo --fresh-lane` or `repo cache forget-lane`) to run this proof zero-touch.
5. Wall times are dominated by admission: each `repo cargo` took 8-20 min under load 50-100 (cargo-preparation exclusive lock, then `semio.compiler.cpu/v1/<n>` slots); H31 owns it.

## Not verified
- Fleet-fetched artifact sha versus a local build (item 4): no phase produced a `remote-hit` (units never redispatched with a key that is in the fleet but not local). The uploader's divergence list (1038, H15) remains the evidence on cross-machine determinism; round-trip integrity is covered by the uploader/restore tests, not by this probe.
- Offload to an idle machine: not demonstrated (one attempt, failed on deadline, finding 1).
- TS slice: see below.

## TS slice (`@semio-tech/pets:typecheck`): NOT measured
`ts-probe.py` (cold `--skip-cache`, warm, fresh `NX_CACHE_DIRECTORY` for a fleet fetch, endpoints closed for down) started at ~03:25; the first `repo run` sat 30 min at 0 % CPU in admission (no task started), so I stopped my own pids (probe 42626, repo child 42642) per the coordinator rule. The script is kept and untested beyond that first call; rerun after H31's cargo-preparation / graph-read contention fix: `python3 ts-probe.py @semio-tech/pets:typecheck <scratch> <out.json>`.

## Still running
Nothing of mine. Sessions closed. Two nonce entries of this probe remain in the uploader spool (`upload-retry` every 15 s) and expire by themselves.
