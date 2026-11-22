# Fleet rollout of the wasm cdylib offload code

## Sanctioned mechanism
There is no command to run and nothing to kill: every sidecar restarts itself for new code (`🛰️sidecar/🟦️.ts`, `createSidecarRestartV1`, numbers in `🛰️sidecar/🧬️schema/🔣️.json`). Each role (agent = worker, scheduler, cache, graph, pool, ready) watches the content digest of its own code closure. After a change it waits for a quiet period (agent 300 s, at most 1800 s of continuous edits), then exits with status 75 and its launcher starts it again on the new code. The agent first drains: `admit:false, reason:"draining"` for new actions, finishes the actions in flight (up to 7200 s, never killed), then restarts; a minimum interval of 1800 s between restarts applies. The scheduler role is aborted at once on its own due time. Other users' in-flight actions are therefore never killed. The replica daemon already delivers the code, so the rollout is automatic and was not touched (no processes stopped, no remote shell, no credentials needed).

## Evidence
- This host's agent log (`.cache/f/sidecars/build-agent.log`): `agent: code of the sidecar edited (RE/🚀️offload/📐️policy/🟦️.ts, RE/🚀️offload/🧬️schema/🔣️.json)` at 13:49:03Z, new agent child (pid 54831) started 13:53:03Z, i.e. after the edit, so it runs the new code. A later edit (resident schema, 13:56Z) schedules a further restart (min interval 30 min).
- `repo fleet status` shows no per-worker implementation digest for sidecars (only per-run `code current/older/unknown`), so the version of the other 10 workers and of the scheduler on g1-server cannot be read from here. Run-level freshness: this host's script run `code current`, the graph-plan run `code older`.
- Versions before: workers and scheduler as of the 14:5x build (old policy, 1800 s cap, cdylib refused). After: see proof; a remote worker that is not this host executed the cdylib, which the old worker filter refuses, so at least that worker runs the new code. Which of the 11 workers and whether g1-server's scheduler already restarted is not individually readable; those that did not restart yet will within about 30 min after edits go quiet. Remaining un-restarted machines would refuse a cdylib (fall back locally, no harm).

## Proof
Script: `fleet-proof.sh <a|b> <always|never>` (tiny cdylib `fleet-proof`, release profile opt-level s, thin LTO, 16 CGU, strip, wasm32-unknown-unknown, flags `-Z threads=1 -Z no-parallel-backend`, `--remap-path-prefix=<dir>=.`, `--remap-path-scope=all`, max-memory and stack-size link args), driving plain pinned cargo through the unit-cache shim (`RUSTC_WRAPPER` = `📜️script.ts`), because `repo cargo` only accepts repository packages. Flags go through `--config` (an environment `RUSTFLAGS` naming the cwd is declined as unmapped-path).
- Local (`SEMIO_OFFLOAD=never`, unit cache off, dir b): sha256 `0aca3d50bc4b5cfc4c657d7500b8cddc80b750deaa9e4ddc78d6c8106b4de620`.
- Remote (`SEMIO_OFFLOAD=always`, dir a): event `offload-run`, origin `ueli-mac.fritz.box-75332` (another machine than this host), 4998 ms total (queue and execute 4612 ms), 364 bytes shipped, 400 bytes returned. sha256 identical `0aca3d50...4de620`.
- Second request from a different checkout path (dir b): origin `fleet (cache)`, same action digest (action cache hit), identical sha256.
Result: remote wasm cdylib offload works and is byte-identical to the local build.

## Not shown
- Peak-sized placement and the 3600 s limit are not separately observed (the fixture is tiny). Not run: a real 100+ MiB component.
- Per-machine implementation of the other workers and the scheduler (see above).
- Incident note: a stray `repo remote-execution workers --join=...` was started once by mistake (it joins this host as a worker) and was stopped within seconds, no action had been taken by it.
