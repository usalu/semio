# Graph slice read that did not converge (2026-11-23)

Evidence: `/tmp/claude-501/devhost.txt` (progress stream of `repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype`) and the service log `host-659f709f2f10.log` (`[graph-retry]`, `[graph-settle]`, `request-failed`).

## What the log shows (three separate things)

1. **Cold first pass, monotonic (0 to 3043 s).** 217,413 source observations, 535 admitted owners, 587 owner checks (`memo 0, checkpoint 0, discovery 587`). The "n of 600" counter is the elapsed time of the current step against its 600 s bound and restarts with every settle round; it is not an owner count. The counters did advance (3, 15, ..., 587 completed checks). This pass is only slow, because after the reboot no owner fact was reusable (empty memo, no checkpoint hit) on a saturated machine. It is not a restart loop.
2. **The non-converging part (3043 s to the end): inventory follow-up rounds on one path.** From round 1 on, every round names the same single path (a `Cargo.toml` below `✏️s/.../mutations/remove-mark-info/packages/rust/`, a directory that other agents create and delete) with `dirtyOwners: []`, `streak` 1, 2, ... 23 (round 23 at `elapsed 16.7 s`; the `streak` kept growing although `inventoryVolatileRounds` is 3). After 24 rounds (`2 * inventoryRounds`) the read died with `BUILD_SOURCE_UNPARSEABLE: ENOENT lstat .../Cargo.toml (imported by .../os/dev/packages/typescript/project.json)` ("The resident service met an unexpected failure"). The slice worker was also restarted by the replica-settle supervisor ("exit code 75 ... Another process kept editing ...; the replica has not settled for 0 of 900 s"), and later the whole run exited with 124 ("no progress for 10 min and has no live child").

## Cause

`settleInventoryRoundsV1` (`🕸️graph/📥️load/🔁️inventory-rounds/🟦️.ts`) makes a path volatile after `volatileRounds` and the tolerance reaches only the membership walk and hash (`workspaceBuildFilesV1`, `buildGraphMembershipV1`). The failing read here is not the membership walk: an **owner observation** reads the file (the project.json of an owner imports it) and fails with `ENOENT`. Three details turned that into an endless, then fatal, loop:

- `observeOwnerFactsMemoizedV1` was called with `foreign: followedPathsV1(root, error).length > 0`, so every vanished path of the tree was "another layer's failure" and was thrown back to the inventory round, even after that very path had been declared volatile. The owner could never fail with its own named failure.
- A failed owner is not memoized, so `forget(paths)` finds no owner (`dirtyOwners: []`): the round re-observes the same owner, fails the same way, and tolerating the path changes nothing. Streak and tolerance were meaningless for this failure.
- The loop therefore ran to its outer bound (24 rounds, each 1.5 to 25 s plus a growing pause, each re-walking the tree) and threw the raw `BUILD_SOURCE_UNPARSEABLE` as an unexpected service failure instead of a named owner failure that the admission can withhold or report.

Not the cause: restart-on-change of the first pass (it was monotonic), a perpetually dirty owner (no owner was dirty), a missing sealed generation as such (a selected scope that was never sealed has no generation to plan against; its owner facts only persist at the end of a successful read, which is why a failed round leaves the next run cold again).

## Fix

- `🕸️graph/📥️load/🧲️membership-follow/🟦️.ts`: new `untoleratedFollowedPathsV1(root, error, volatile)`: the followed paths of a failure that are neither tolerated nor below a tolerated directory.
- `🕸️graph/📥️load/🟦️.ts`: the owner observation is foreign only when the inventory still follows one of its paths (`untoleratedFollowedPathsV1(...)`). Once the path is volatile the owner fails with its own named bilingual failure (`BUILD_OWNER_FAILED`), which `closeBuildOwnerAdmissionV1` withholds when the selection does not need it and reports by name when it does.
- `🕸️graph/📥️load/🔁️inventory-rounds/🟦️.ts`: a failure that names only already tolerated paths cannot be helped by another round, so the read ends at once (or, with `orNothing` and a membership failure, answers nothing and marks the volatile paths) instead of repeating until `2 * inventoryRounds`. Owner checks are bounded by `volatileRounds + 1` rounds.
- `🕸️graph/📁️workspace/🟦️.ts`: `BUILD_MEMBERSHIP_CHANGED` names its `paths` in a deterministic (locale) order; the parallel `lstat` pushed them in completion order, which made the volatile-directory test flaky.

## Tests (bun, module conventions)

- `🔁️inventory-rounds/🧪️tests/🟦️.ts`: new "a failure that keeps naming only a tolerated path ends the read at once with a bounded number of owner checks" (rounds and checks equal `volatileRounds + 1`; `orNothing` variant returns the volatile path). The "volatile directory" test compared against a hash without its directory records; it now compares with a second real walk.
- `🧪️tests/🧲️membership-follow/🟦️.ts`: new test for `untoleratedFollowedPathsV1` (file, directory prefix, sibling with common prefix, error without path).
- Run: `bun test ./<path>/🟦️.ts` for the two files plus `🧪️tests/📁️inventory` and `🛟️owner-isolation/🧪️tests` with `SEMIO_TEST_ARTIFACT_DIR` set: 45 pass, 0 fail.

## Not done / follow-ups

- The cold first pass (48 min for 535 owners) is only reusable if its owner facts persist before the read ends. Today the admission memo and the owner checkpoint packs are published at the end of the settle (`oc` directories of earlier scopes are empty), so a run that dies in the follow-up rounds starts cold again. Publishing the memo before the follow-up rounds (the first captured admission is already consistent per owner) is the next step; it touches the admission memo format and was not changed here.
- The replica-settle supervisor (exit code 75 restart) re-spawns the slice worker, losing its in-memory memo; with the persistence above the restart would be cheap.
- Not verified against a live `repo run` (machine saturated, another agent's run was in flight); the `repo` MCP server did not connect in this session, so no ticket tools were used.
