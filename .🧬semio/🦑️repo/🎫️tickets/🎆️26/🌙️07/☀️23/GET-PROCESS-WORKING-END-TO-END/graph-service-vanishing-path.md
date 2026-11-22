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
`vanishedMemberPathV1(root, error)` (`🕸️graph/📥️load/🧲️membership-follow/🟦️.ts`) names the absolute member path below the root
for an ENOENT/ENOTDIR failure and answers nothing for the root itself, for paths outside the root and for other codes.
`followedPathsV1` uses it on the failure chain, so `settledInventory` in `🕸️graph/📥️load/🟦️.ts` re-observes exactly the owners
that read that path (path-moved follow-up). The earlier `BUILD_MEMBERSHIP_CHANGED` wrapper was dropped: `settledInventory` never
read its code, and the wrapper let the root path through as a member. Test: `🕸️graph/📥️load/🧪️tests/🧲️membership-follow/🟦️.ts`.
Caveat: I could not reproduce which exact reader threw; the fix is at the step boundary, not at one lstat.

## Fix 2: "still recorded as running"

Every start writes exactly one start row. The launcher writes it at spawn: with the pid for a direct child, or without a pid
and with `via:"launchd"` for a launchd job. The service then completes that pidless row with its pid
(`completeServiceStartRowV1` in `🏡️resident/🧾️exit-log/🟨️.mjs`, called from `🏗️composition/🌐️host/🟨️.mjs`); it never appends a
second row. `explainServiceLossV1` reports "the service is still starting under launchd" for a pidless launchd row younger than
120 s, and treats an older pidless row like a dead pid (killed without exit record, oom count from the last known cgroup).
"Still recorded as running" remains only for a start row whose pid is alive.
Test: `🏡️resident/🧪️tests/🧾️exit-log/🟦️.ts`.

## Validation

The repo MCP hooks timed out and `repo` CLI stalled at `door.capacity` on the loaded host, so native bun 1.4.2 from the
repo tool cache ran both test files: 7 pass, 0 fail (log: `🗑️generated/graph-vanishing-path-bun.txt`). The running graph
service and `dev playground` process were not touched. The service has to be restarted by its owner to load the new code.
