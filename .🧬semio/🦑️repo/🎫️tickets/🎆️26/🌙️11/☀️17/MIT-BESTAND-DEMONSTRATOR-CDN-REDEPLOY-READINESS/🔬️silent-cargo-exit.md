# Silent cargo exit 1 in native release component builds

## Evidence (run wtJlpQa, demonstrator release-00)
- `l/cargo-release-00.jsonl`: 15691 records (1050 compiler-artifact, 17 build-script-executed, 14624 compiler-message), no `build-finished`, no message with level error; last record is a compiler-artifact.
- `l/cargo-release-00.log` stops mid "Compiling semio-s-artifact-stdio-ply-inferences"; no "could not compile", no stderr.
- Report: `Failed units | 0`, no "Killed units" section, no "Execution failure" section.
- A cargo with a failed unit exits 101 (see the other logs: 101). Exit 1 with empty output is the driver's own fallback.

## Root cause
Two defects in `component-pass/🟦️.ts` combine:
1. `nativeBuildChildTerminalV1` resolved `code ?? 1`: a Cargo ended by a signal (close code null) was reported as plain "exit 1", indistinguishable from an ordinary failure, with no signal name.
2. Inside the per-attempt `compile` closure, a local `const stop = (): void => terminateOwnedChildTree(child)` shadowed the driver-wide `stop(reason)`. The storage watchdog (`storageCheck`, every <=1 s, `nativeBuildStorageFloor` of `min(statfs, host volume)` vs the admitted reserve, the "compiler image 95% full" situation) called `stop(failure)` / `stop(error)`, which hit the shadow: it killed the Cargo process tree and silently discarded the ENOSPC reason. `stopped`/`stopReason` were never set, so `checkPass()` did not throw, the unit killer saw no unit signal (cargo itself was terminated), stderr was empty, and the result was a bare "cargo exited with 1". This matches: no ENOSPC logged, empty stderr tail, mid-"Compiling" stop, ~500 s in when image headroom shrinks (swap 3.5 GiB, frozen samples).
- Other stop paths (abort signal, drift, deadline, outer `stop`) already record a reason; an external signal (jetsam, lease/capacity sweep) remains possible and is now named.

Not proven which exact event crossed the floor in those four runs (the reason was discarded); the evidence fits the storage watchdog, and the fix makes the next occurrence name it.

## Fix
- `component-pass/🟦️.ts`: renamed the shadowing local to `endChild` (four uses) so the storage watchdog calls the driver `stop(reason)`: the ENOSPC error is recorded, children are ended, `check()` throws it, and the run report gets "Execution failure".
- `nativeBuildChildTerminalV1` now maps a signal end to `128 + signal` (the convention `unitExitSignalV1` reads) instead of 1.
- New `cargoSilentExitV1` in `component-pass/🩺️failure-diagnosis/🟦️.ts`; the driver adds its text to the failure lines when Cargo failed with no compiler error, error block or stderr: names the signal or states that no stop of the driver was recorded.
- Test added in `failure-diagnosis/🧪️tests/🟦️.ts` ("a Cargo exit without any diagnostic is named instead of left silent"). NOT executed: no bun on PATH here and I did not start builds; run via repo test for the process module.

## Operational note
If the storage floor was the trigger, the next run now fails with the ENOSPC message (free MiB vs required MiB) instead of exit 1; remedy is image reclaim headroom, not a cargo problem.
