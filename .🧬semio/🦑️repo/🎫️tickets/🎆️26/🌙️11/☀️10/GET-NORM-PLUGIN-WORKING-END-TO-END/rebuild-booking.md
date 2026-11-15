# Rebuild Phase Booked 11634 MB For A Plain show

## Root cause
- `repo show` is phased; a run that rebuilds the graph enters phase `rebuild`, and `createHostPhaseResizerV1` (host-capacity/🪜️phases) lifted its grant to the learned global `phase:rebuild` size, capped only by the whole `capacityMegabytes` (host total minus floor, 14336 MB here).
- The learned rebuild size comes from tree readings of the graph builder; on a thrashing macOS host those count shared/compressed pages many times (gate saw 27488 MB for a 6.4 GB RSS worker), so the class learned 11634 MB. Rebuild booking therefore took 11.6 GB while the tree observed 470 MB, which put the host 6 GB below its floor.
- The slice-peak runtime file is not the source of this number (currently 1622 MB; earlier poisoning was reset).

## Fix
- Resizer: entering `rebuild` books at most the policy share `rebuildShareBasisPoints` (50 %) of capacity (7168 MB), the same share `hostRebuildSlotsV1` already uses for the slots. More real use grows through the gate.
- Python oracle resize updated, fixtures regenerated (`--check` equal).
- Tests (🧪️tests/🪜️phases): new test that a learned 11634 MB rebuild books exactly the share; the replay law now allows the memory beyond what a rebuild may book.
- Run: `SEMIO_HOST_CAPACITY=off repo tools exec bun -- test` phases + estimates + followup: 38 pass. Unrelated failing: graph upgrade-before-custody source-order test (another agent editing graph host source).

## Follow-up 1: clamped, decaying learning at record time (done)
- `recordHostPhasesV1` takes `ceilingMegabytes` and clamps every recorded peak (whole run and each phase segment, so the rebuild class too) to it before learning; the grant learner in library host-capacity passes `capacity` (host total minus floor). A thrashing tree reading (27 GB on a 14 GB host) can no longer teach more than the host has above its floor.
- Decay: the estimate store already decays by half-life (3 days) with an outlier-robust weighted quantile, and the slice-peak store is clamped on write and read (`sliceCeiling`, history tests); no second decay mechanism added.
- Measure: kept the footprint measure (it counts compressed memory, the real pressure); the ceiling is what bounds the overcount. `observeTreeMegabytesV1` supports `"resident"` if RSS-only learning is wanted later.
- Oracle: Python `record` clamps the same way; new `clampCases` vectors (thrashing 27 GB, sane builds, one 30 GB overcount among sane builds); TS test equals the oracle and bounds the learned rebuild by the ceiling plus margin.

## Follow-up 2: orphaned client releases its grant (done)
- Correction by the coordinator: no "parent changed / became init" rule, since detached runs (`nohup`, `setsid`, CI wrappers) are legitimately reparented and keep running. New `host-capacity/🎫️gate/👻️orphan/🟦️.ts` signals only a lost output reader: `EPIPE` on stdout (noted by an error listener) or stdout destroyed/ended; a stdout that is a file or the null device never breaks. `holdHostGrantV1` polls it on the heartbeat interval for non-adopted grants and, when lost, SIGTERMs itself, which runs the normal cleanup that gives the grant back. Port `orphan` in `HostCapacityOptionsV1` for tests.
- Tests (`🧪️tests/👻️orphan`): reparented with live stdout keeps its grant; broken pipe loses it; a real child process with a null-device stdout stays, one whose stdout pipe reader hung up is detected; a held grant is stopped by the heartbeat and given back.
- Runs: phases/estimates/followup/quiet/liveness/orphan pass with `SEMIO_HOST_CAPACITY=off`. The handover real-process tests fail only because the governor is off (as noted in quiet-grant.md), not re-verified with the gate on.

## Still held
- Service host restart, until the coordinator says so.
