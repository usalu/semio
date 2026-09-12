# Wave B54 — the mutation ceiling, round 2: where a 30-second budget actually goes

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B54, 2026-09-12 (report file dated per the coordinator's
naming). Written incrementally.

Predecessors: `📓️2026-09-13-wave-B44-mutation-latency-large-document.md` (the render fingerprint,
per-object residency, the `instancesDelta` lane, and the turn-count residual),
`📓️2026-09-13-wave-B52-reconcile-ladder-livelock.md` (the retirement-ladder livelock),
`📓️2026-09-13-wave-B48-nakagin-selection-lane.md` (the refresh hop),
`📓️2026-09-12-wave-B24-command-ingress-round-trips.md` (`TypedOperationGrant`).

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, shared live tree. No state-modifying git command,
  no worktree, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, every command foreground. The ticket is NOT
  closed or reopened by this wave. Nothing under `🗑️generated` was deleted.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- `RUST_MIN_STACK=134217728` on every law run.
- Machine load average 46–60 throughout. Every absolute timing is a loaded debug number; the wave's
  claims are ratios taken on the same machine minutes apart.
