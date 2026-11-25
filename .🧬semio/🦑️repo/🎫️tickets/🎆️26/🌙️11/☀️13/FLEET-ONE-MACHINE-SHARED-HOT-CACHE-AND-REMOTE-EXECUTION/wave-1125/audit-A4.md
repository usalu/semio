# 🧭️ Audit A4: Does g1-server Publish ready.json?

Read-only audit, wave 1125. No code edits, no git writes, no process kills.

## Verdict

Intended, not a defect in the readiness supervisor. The group server is excluded on purpose. g1-server is nevertheless a registered remote-execution executor with headroom, and nothing in the readiness path provisions it. That gap is worth a decision, but it is not a missing `ready.json` writer.

## Evidence: the exclusion is intended

- `🚦️ready/🟦️.ts:660`: `isServer = origin === group + P.serverSuffix` (`-server`), so `g1-server` with group `g1` is the server.
- `🚦️ready/🧬️schema/🔣️.json:96`: `"serverSuffix": "-server"`.
- `🚦️ready/🟦️.ts:662-663`: `readyIsServerV1` doc: "the group server of its group, which is never set up by this service".
- `🚦️ready/🟦️.ts:1397-1403`: `runReady` reads the group from `.cache/f/machine.json` when the env has none, then `if(ports.server){ ... "the group server is never set up by this service" ; return; }`. The function returns before `runReadyV1`, so it never writes `ready.json`.
- `🚦️ready/🟦️.ts:450`: "The group server returns at once and writes nothing."
- `🚦️ready/🟦️.ts:287`: decision rule "the group server never".
- `🚦️ready/🟦️.ts:1305`: `server:isServer(input.origin,input.group)` in the ports.
- The machine's own config, `/Users/ueli/Documents/semio/.cache/f/machine.json`: `{"originId":"g1-server","groupId":"g1",...}`. So the role is derived from the origin name, not from a separate topology file. I found no separate topology file under `/Users/Shared/semio-store/v1/fleet/`, which holds only `cache.json`.

## Evidence: state on this Mac

- `ls /Users/Shared/semio-store/v1/origins/g1-server/`: has `attest.json`, `fleet-auth.json`, `head.json`, `home.json`, `pool.json`, `refused.json`, `segments/`, `view.txt`. No `ready.json`, no `memory.json`, no `ready-log-*`.
- The worker origins g1-m01 to g1-m14 each have `ready.json` (g1-m09 has none). The g1-m10 origin also has `memory.json` and `ready-log-*`.
- `ls .cache/f/native-bootstrap.done`: absent. `.cache/f/native-bootstrap.log` exists from 2026-11-15. Consistent with the server never running the loop.
- `ps`: no ready supervisor process is running on this Mac. The bun processes are the agent, scheduler, graph, pool, spool and cache services. That is expected, because the server never starts the loop.

## Evidence: remote execution

`repo fleet status --json` (`fleet.machines`):
- `g1-server running 4 queued 0 headroom True worker idle why None cores 10 load 37.01 pressure normal age 2`
- The fleet `offloadedIn` for g1-server is 0 in this snapshot.

`repo remote-execution status` (`server.workers`), entry for origin `g1-server`:
- `pin g1-server`, `platform darwin-arm64`, `concurrency 10`, `sandbox darwin-sandbox`, `confinement full`, `network denied`, `headroom True`, `draining False`, `offloadedIn 0`, `refused 0`, `lastSeen 3`.
- Toolchain, linker, target and wasm-opt properties are identical to g1-m01 and g1-m10 (`semio-toolchain 312cc5b2932fbb31`, `semio-linker 0fbcef956b2adb56`, `aarch64-apple-darwin`, `wasm-opt fa49ec32a92b`).

Headroom caveats:
- Memory: `totalMegabytes 32768`, `availableMegabytes 13842`, `floorMegabytes 4096`, `spareMegabytes 4715`, `pressure normal`, `swapUsedMegabytes 710`. Headroom flag is memory-based.
- CPU: `load 37.01` on 10 cores, about 3.7x oversubscribed. The `headroom True` flag does not account for this.

## Answer to the three questions

1. Coordinator exclusion from readiness: intended, by name convention `<group>-server` in `🚦️ready/🟦️.ts:660`, and by the early return at `🟦️.ts:1403`.
2. Supervisor on this Mac: not running (no ready process in `ps`), and it should not run for the server. `fleet status` and `remote-execution status` both report g1-server as a live, idle worker.
3. Remote execution: g1-server is a registered executor with concurrency 10 and headroom True. It has received 0 offloaded jobs. Its CPU load is high.

## Possible defect (not in the readiness supervisor)

- The server is registered as a worker (`pin g1-server`) with a full toolchain fingerprint, but no path in the readiness loop provisions or marks it. Its toolchain matches the peers, so it was presumably set up outside the loop. The code does not verify this. If it was not set up by hand, the worker advertises a toolchain it may not have.
- The `headroom` flag ignores CPU load (37 on 10 cores), so the scheduler may route work to a saturated coordinator. The fleet shows g1-server with 0 offloaded jobs so far, so this has not bitten yet.

Decision needed (not a code change in this audit): either keep g1-server as an executor and document that its toolchain is provisioned by hand, or remove its worker registration and let it be coordinator-only.

## Limits

- The `repo remote-execution status` command streams script progress and hit a 5 s reader timeout for the `workers` and `storage` sources, so the worker list may be partial in time. The g1-server entry was present.
- I did not find a separate topology config file. The role comes from the origin name and `.cache/f/machine.json`.
