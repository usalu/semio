# Graph service died on a vanishing path

## Producer of the scratch directory

`🧰️scratch` is not a temporary directory. It is a source module, `build/🌐️remote-cache/🧶️nx/🧰️scratch/🟦️.ts`
(`provisionPristineNxV1`, `createScratchNxWorkspaceV1`, `wipeScratchNxCacheV1`, `runScratchNxV1`), created in the working tree
by another agent (fleet wave E10) and imported by the fleet bench runner and the remote-cache tests. For a moment it did
not exist (the runner and tests imported a missing module); it exists again (re-created 02:20). The graph service walked it
while it came and went. Its scratch workspaces and pristine Nx tree are already created below `tmpdir()`
(`fleet-*`, `semio-pristine-nx-<version>`), so no producer writes temporary directories next to source files. Nothing to move.

## Fix 1: vanished paths in the inventory

The workspace walk, the membership hash and the observation probes already treat ENOENT/ENOTDIR as a membership change.
A raw `lstat` ENOENT from any other reader in the inventory step still surfaced as an internal error.
`vanishedMemberAsChangeV1(root, error)` (`🕸️graph/📥️load/🧲️membership-follow/🟦️.ts`) turns ENOENT/ENOTDIR of a path below the
root into the typed `BUILD_MEMBERSHIP_CHANGED` refusal naming that path; `settledInventory` in `🕸️graph/📥️load/🟦️.ts` applies it,
so the existing follow-up rounds treat the path as absent. Paths outside the root and other codes pass unchanged.
Test: new case in `🕸️graph/📥️load/🧪️tests/🧲️membership-follow/🟦️.ts`.
Caveat: I could not reproduce which exact reader threw; the fix is at the step boundary, not at one lstat.

## Fix 2: "still recorded as running"

A service started through launchd (the macOS route) wrote a start row without a pid and never wrote its own start row, so
a dead service fell through to "still recorded as running". The service now appends its own start row with its pid once it
runs (`🏗️composition/🌐️host/🟨️.mjs`), and `explainServiceLossV1` (`🏡️resident/🧾️exit-log/🟨️.mjs`) treats a last start row without
a pid like a dead pid (killed without exit record, oom count from the last known cgroup). "Still recorded as running" remains
only for a start row whose pid is alive. Test: extended case in `🏡️resident/🧪️tests/🧾️exit-log/🟦️.ts`.

## Validation

The repo MCP hooks timed out and `repo` CLI stalled at `door.capacity` on the loaded host, so native bun 1.4.2 from the
repo tool cache ran both test files: 7 pass, 0 fail (log: `🗑️generated/graph-vanishing-path-bun.txt`). The running graph
service and `dev playground` process were not touched. The service has to be restarted by its owner to load the new code.
