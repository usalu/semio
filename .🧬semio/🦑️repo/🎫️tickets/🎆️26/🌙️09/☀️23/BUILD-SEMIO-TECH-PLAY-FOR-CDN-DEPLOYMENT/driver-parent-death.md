# Driver Outliving Its Launcher (Parent Death) and Silent Lane Wait

## Cause

1. `native-build-launch.ts` runs the driver with `spawnSync`. SIGTERM kills that bun without any handler, the blocked
   `spawnSync` never forwards it, and the driver (own process group, `detached`) was re-parented to pid 1.
   Nothing in the child noticed: the only orphan rule (`host-capacity/gate/orphan`) watches for a broken stdout pipe
   (EPIPE) and only while it holds a host grant, and a quiet cargo build never writes. The driver stayed alive, so its
   pid-liveness claims (`semio.compiler.cpu/v1/*`, `semio.cargo.custody-lease`, product lane) were legitimately held.
2. The next job did not wait for "5 of 32 lanes". The message came from `acquireProductLaneV1`: the product lane
   (`q<digest>`) is exclusive, one holder per product key. The claim of that one lane was held by the zombie driver.
   The message printed `live` = claimed lanes (5) and the pool maximum (32), which is not the bound that applied.
   The poll sleep (`sleepV1` in `unit-cache/lock`) was not a declared wait, so the script-process watchdog saw no
   progress, no live child, and exited 124 after the stall period.

## Changes

- `process/routing/invocation/process/watchdog` (existing liveness module, schema `Policy`, `OwnerFacts`, `Notice.ownerLost`):
  - `scriptProcessEnvironment` (the envelope carrier every owned spawn with an envelope goes through) now hands over
    `SEMIO_SCRIPT_PROCESS_OWNER=<spawner pid>`.
  - `scriptProcessOwnerLostV1` (pure rule), `readScriptProcessOwnerV1`, `startScriptProcessOwnerWatchV1`: a pid probe
    plus parent pid every 2 s (`process.kill(pid,0)` and `process.ppid`, so identical on POSIX and Windows).
    A process nobody handed an owner to (`nohup`, daemon) is never touched; the documented reparenting exception stays.
  - `withScriptProcessEnvelope` (process birth) starts the watch on the main thread: on loss it prints the bilingual
    notice, runs the existing `terminate` path (abort of the invocation signal, owned children terminated, leases
    released by the normal cleanup, guard exit 143 after the grace) and, on exit, terminates remaining descendants
    with the existing `terminateOwnedProcessTree` (`taskkill /T` on Windows).
- `unit-cache/lock`: `sleepV1` and `watchChangesV1().wait` declare a wait (`enterScriptProcessWaitV1`), so a lane,
  claim or slot wait is "waiting", never stalled (bounded by the existing 4 h wait ceiling).
- `unit-cache/lock`: `claimHolderV1`; `unit-cache/lane/pool`: `laneHoldersV1`; schema messages `poolBusy` (now names
  holders) and new `productLaneHeld` (names pid, agent, session, age, states that the limit is the one exclusive lane
  and not the pool of max 32), `pool.listedHolders = 4`.

Not done: Windows Job Objects and POSIX `PDEATHSIG` are not reachable from Bun without a native addon, so the
cross-platform mechanism is the poll (worst case 2 s). The `repo capacity run` -> launcher hop is covered only if the
launcher receives the envelope env from its spawner (the launcher itself already hands it on).

## Tests

- `watchdog/tests/owner/` : neutral vectors (`json`) checked against the schema and an independent SQLite oracle;
  env hand-over; watch fires once / never without owner; subprocess test: a middle process spawns an Original child
  (own grandchild), the middle is SIGKILLed, child and grandchild must be gone (passes in about 2.5 s).
- `unit-cache/lane/pool/product/tests`: waiter on a held product lane declares a wait, reports `productLaneHeld`
  with holder pid/agent/session, limit 32, both languages; wait count returns to 0.
- Ran with `repo test-file`: watchdog (15), owner (4), product (8), pool-claims (7), routing, channel, testing
  invocation: all pass.

The ticket was already open (reopen refused), so it was not closed here.
