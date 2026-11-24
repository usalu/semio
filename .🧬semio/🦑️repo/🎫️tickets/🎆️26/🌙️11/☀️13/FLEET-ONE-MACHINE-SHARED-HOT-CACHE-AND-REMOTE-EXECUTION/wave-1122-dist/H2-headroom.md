# H2 Headroom: claims reserve measured use, units are admitted by their own size

B = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, RE = `B/🌐️remote-execution`.

## Changes (schema first)
1. **Stage claim = measured use x 1.25** (`RE/🌡️load/🪑️stage-claim/`): schema Policy `reservePercent` 125, `floorMegabytes` 256; Claim gets required `waiting`. `readyStageReserveV1` = 0 while waiting, else min(entitlement = learned peak or 30 % default, max(256, ceil(observed x 1.25))). Hold = reserve - observed (was entitlement - observed: 2457 MB on a 16 GB worker with 2.6 GB used, now about 650 MB; 0 while waiting). Publisher `observe(mb, waiting)` writes at once when waiting flips; `RE/🛰️sidecar/🚦️ready/🟦️.ts` passes `holder.snapshot()!==null` (the stage waits on a named holder: lease, dependency store, another build) at both sampler sites.
2. **Per-unit admission** (`RE/🌡️load/🟦️.ts`, new `🌡️load/🧬️schema/🔣️.json`): `evaluateRemoteExecutionAdmissionV1` takes the unit's `megabytes` and the peaks of the running units (`held`); the old constant 2048 is only the default for an undeclared unit. The load report's `freeMegabytes` is now the spare a unit may take above the floor (host decision `headroomMegabytes`), `headroom` is the verdict for the smallest unit (256 MB). The scheduler already compares peak with `freeMegabytes - committed`, so it now places by size. Task carries optional `peakMegabytes` (scheduler -> worker, schema `Task`); worker admits after the poll by it (`RemoteExecutionUnitAskV1`, `👷️worker`, `🛰️agent`).
3. **Learned peaks** (`RE/🚀️offload`): schema `policy.peak` (unknown 2048, margin 25 %, 64..16384, killedExit 137, killedFactor 2). `offloadUnitPeakMegabytesV1`: learned x 1.25 else unknown default (wasm linked 6144 stays the unknown default). `readUnitPeakV1/writeUnitPeakV1` (file `<crate>.peak` beside the cost record; larger at once, smaller halfway). Written from the broker provenance of every local compile (`⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts`). Every offloaded unit now declares `semio-peak-mb`. A unit killed on a worker (exit 137) falls back local (existing) and now records peak = 2 x placed in the infrastructure detail and the record.
4. **Pressure gate**: the "stage running and pressure != normal refuses" rule is gone. `remoteExecutionPressureGateV1`: critical refuses all; warn refuses only a unit above 1024 MB or while swap-out >= the policy's active rate (about 1 MB/s on 16 GB); else judged as normal. Hard swap stops of the host decision stay.

## Tests (`repo test-file`, one at a time, `taskpolicy -b`)
- new `RE/🧪️tests/🏋️unit-admission` 7/0 (Python oracle: 75 gate vectors, 700 admissions across available/held/size/pressure/swap, peaks, learns, kills; report under warn with stage; scheduler placement by peak; peak file).
- `RE/🧪️tests/🪑️stage-claim` 8/0 (waiting flips, hold vectors incl. waiting, reader, pool vs oracle, agent verdicts).
- dependants: why 11/0, placement 9/0, scheduler 22/0, worker 6/0, offload 41/0, offload failure 2/0, stage-claim run 1/0.
- `🌐️fleet` 2 pass / 8 fail in two runs under host load 40 to 70: the failures are timeouts (an `echo` action took 97 s, "first start" of the cluster); not judged, UNVERIFIED. Unit-cache `script.ts` and ready `🟦️.ts` only syntax-checked (`bun build --no-bundle`), the ready run test passes.

## Live (`repo fleet status`, once, after landing)
10 of 13 entries show headroom true (m03 m05 m06 m07 m08 m10 m11 m12 m13 g1-server); refusing: m01 and m02 (pressure warn, no-headroom), m04. Before: 2 of 13. Workers have not necessarily restarted on the new code (replica plus sidecar restart rule), so this is not proof of the change; re-read after the 300 s quiet restart.

## Open
- Learned peaks come from local compiles of the requester only; units only ever compiled remotely keep the unknown default 2048 (wasm 6144). Next step: workers return the measured peak RSS in the action result.
- `freeMegabytes` semantics changed (spare above floor, not raw available); a new worker reporting to a scheduler on old code still fits the closed schema (same field), but the scheduler places by it as before.
- Exit 137 is treated as a memory kill (typed signal, no text matching); it also fires for a manual kill.
