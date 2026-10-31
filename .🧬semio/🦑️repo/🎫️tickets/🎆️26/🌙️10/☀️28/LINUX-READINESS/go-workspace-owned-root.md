# Workspace Go Owned Root Repair

The actual public workspace test target failed TestTestsRunAgainstAnOwnedRoot because two shared corpus readers in the unit suite called os.Getwd when FixtureOrigin was absent. The sandbox TestMain already supplies FixtureOrigin. Both readers now require that explicit origin, preserving the owned-root guard, complete neutral invocation corpus, and independent Windows encoder oracle.

Red evidence: generated/go-workspace-codebase-tree.log records both offending source lines 506 and 565. Validation pending the repaired actual public target.

## Actual Public Verification

The unfiltered public command `repo run @semio-tech/repo-workspace-go:test` successfully ran the test target and its source-policy dependency with zero cache hits (0/2). The workspace Go package passed in the owned sandbox, including TestTestsRunAgainstAnOwnedRoot. Nx target duration was 1m31s; aggregate target duration was 2m20s. Parent process 900222/session 94950 is completing cache postprocessing before the final command outcome.
