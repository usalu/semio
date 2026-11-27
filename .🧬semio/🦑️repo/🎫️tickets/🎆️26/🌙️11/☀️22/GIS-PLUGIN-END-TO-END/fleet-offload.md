# Fleet offload of Cargo units on g1-m07

## How offload works (no opt-in needed)

- `SEMIO_OFFLOAD` / `--offload=never|distribute|pin:<origin>`; policy default is `distribute` (`...remote-execution/🚀️offload/🧬️schema`). `repo cargo` and `repo run` wrap rustc and build scripts; every unit worth a round trip (recorded cost >= 2.5 s or none) goes to a fleet worker. Endpoint, instance and credentials come from the fleet facts (`.cache/f/cache-facts.json`) and this machine's signing identity. Nothing is configured by hand.
- Evidence of attempts: `.cache/b/r/m/u/x/offload.jsonl` (per unit) and `.cache/f/offload-counters/reason/*` (one byte per decline).

## Why the GIS runs did not offload (bug, not policy)

`~/.zprofile` exports a stale operator token `SEMIO_REMOTE_CACHE_TOKEN` (plus stale `SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.70:6090`, which does not answer). `openOffloadSessionV1` took any environment token as the bearer for the gRPC execution call and signed with the machine identity only when no token existed. The scheduler (fleet gate) accepts only its write token or a `submit` signature, so every `Execute` answered `16 Unauthorized`, classified `infrastructure`, and the unit compiled locally:

- 4841 `offload-local-fallback ... 16 Unauthorized` since 2026-11-23 (149 in the last 6 h, all lanes with the token in their environment); the 76 successes were runs without it.
- Probe (`fleet-offload-probe.ts`, GetCapabilities on grpc://192.168.178.70:8980): no credential FAIL, bearer env token FAIL (Unauthorized), signed `submit` OK.
- Other counters (`cheap`, `local-slot`, `not-portable`) are normal declines (cheap units, own slots free, build scripts that spawn processes).

## Fix

`openOffloadSessionV1` (`🚀️offload/🟦️.ts`) now returns `executionToken`: undefined (so the call is signed) when the endpoint comes from the fleet facts (no `SEMIO_REMOTE_EXECUTION_ENDPOINT`), no explicit port token is given and this machine holds a `submit` identity; an explicit endpoint keeps the operator token. Both execution sites (rustc units, commands/build scripts) use it.

Test: `🧪️tests/🌐️cluster/🔏️signed-fleet/🟦️.ts` new test "a stale operator token of the environment never replaces the machine's signature ..." (passes; the existing end-to-end offload test still passes).

Remaining caveat: the unit-cache uploader (`🛰️fleet/📡️client`) still sends the environment token as bearer to the cache, so a stale token can lose result uploads (seen as one missing action-cache put in a probe run). Unset the token for lanes (recipe).

## Recipe for lanes

```sh
unset SEMIO_REMOTE_CACHE_TOKEN SEMIO_REMOTE_CACHE_ENDPOINT   # stale values from ~/.zprofile
# SEMIO_OFFLOAD defaults to distribute; set it explicitly to be sure:
SEMIO_OFFLOAD=distribute repo cargo --store=shared --configuration=prototype -- test -p <crate> --lib -j 12
```

Equivalent per command: `env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute repo ...`. Do not set `SEMIO_REMOTE_EXECUTION_ENDPOINT`.

## Proof run

`repo cargo --store=shared --configuration=prototype -- test --manifest-path <gismap Cargo.toml> -p semio-s-artifact-gis-gismap --lib -j 12` with the stale token still exported, under the shared lock (note: `-p` alone does not resolve the gismap package, the `--manifest-path` of `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml` is required):

- `.cache/b/r/m/u/x/offload.jsonl` in the run window: 0 `16 Unauthorized`, 20 `offload-run` (wgpu, wgpu-hal, naga, ... build scripts and units, on workers `ueli-workstation-mac-mini-m4-16gb-256gb-08-lan` and `Uelis-Mac-mini-4`; part of them by other lanes), 45 `not-portable` build scripts that spawn processes (expected).
- Scheduler (`repo remote-execution status`): `counters.completed` 15 -> 102, sum of workers' `offloadedIn` 1928 -> 2017, `failed` 0 -> 1, `retried` 0 -> 2.
- Local memory during the run: free percentage never below 36 % (lowest while the gismap crate itself compiled locally, see below), swap flat at about 6.5 GB.

## Next blocker (not offload, not fixed here)

The crate `semio_s_artifact_gis_gismap` itself is declined `not-portable`: its mutation derive (`...🧬️mutations/🎥️set-camera/🦀️.rs`) reads workspace files that no compile of the unit has recorded yet, so the first compile must run here (`offload/🗺️paths/🟦️.ts:253`: "it compiles here first, then it offloads"). That local compile peaks at 4584 MB, above the final link ceiling of 4096 MB of the warden (`🛡️warden/🧬️schema`, `ceiling`, closed carving list), which cancelled it; the run therefore ended with `host warden cancelled 1 of its processes`. Once one local compile of the unit finishes (dep file and input manifest recorded) it offloads. Options: carve the gismap crate along the domain taxonomy so the unit fits 4096 MB, or put it on the carving list; run it once on a calm machine or a worker (`--offload=pin:<origin>` does not help for the first compile).
