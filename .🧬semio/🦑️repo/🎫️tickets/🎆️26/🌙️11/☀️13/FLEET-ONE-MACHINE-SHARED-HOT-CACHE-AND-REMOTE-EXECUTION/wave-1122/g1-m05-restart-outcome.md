# g1-m05 Replica Restart Outcome

Restarted the `semio-fleet-replica` scheduled task on 2026-11-22, following findings-E6.md. The task entry identifies `C:/semio` as the replica worktree. The separate `C:/git/semio` checkout was not targeted. No Git-changing commands, cache deletion, worktree reset, stash, or broad process kill was performed; the local replica WAL and checkpoint were retained.

## Result at handoff

- The scheduled task remains running; the replica attempted its own watchdog restart after the manual restart.
- The latest health record remains `store: stalled` with PID 28024, a zero code digest, and no pending operation. The shared attestation is still stale at wall time `1795320387404`, with `queue.incoming: 122` and zero publish samples; convergence is not verified.
- The last readable entry state is PID 8348 in `starting` from the worktree, with no digest; its last attempt reports exit 137 after the watchdog fell silent. The stable heartbeat file was absent at the final read; the last observed heartbeat snapshot is captured separately.
- The shared Ready record continues to update and reports the `dependencies` stage as running/adopted. That stage and its processes were not stopped or restarted.
- `C:/semio/.cache/f/boot/run.log` is absent in this installation. The task shim log at `%LOCALAPPDATA%/semio/fleet/entry/run.log` supplied the last-200-lines capture.

## Diagnostics

- [health.json](../🗑️generated/g1-m05-restart-20261122-145503/health.json)
- [health-latest.json](../🗑️generated/g1-m05-restart-20261122-145503/health-latest.json)
- [run-tail.txt](../🗑️generated/g1-m05-restart-20261122-145503/run-tail.txt)
- [entry-state.json](../🗑️generated/g1-m05-restart-20261122-145503/entry-state.json)
- [loop-blocked-restart.json](../🗑️generated/g1-m05-restart-20261122-145503/loop-blocked-restart.json)
- [beat-last-observed.json](../🗑️generated/g1-m05-restart-20261122-145503/beat-last-observed.json)
- [beat-unavailable.txt](../🗑️generated/g1-m05-restart-20261122-145503/beat-unavailable.txt)

The manual restart was performed, but the replica is not healthy yet. Leave its automatic recovery running; escalate the stalled store/boot issue rather than retrying Git synchronization or killing dependency-stage processes.