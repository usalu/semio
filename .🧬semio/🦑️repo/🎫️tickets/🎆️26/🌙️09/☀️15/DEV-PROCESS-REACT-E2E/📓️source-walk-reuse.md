# Selected Source Input Subtree Reuse

The selected owner source matcher previously reused identical glob questions and exact traversal roots. When an earlier recursive query had visited a later query’s root, the later query could still repeat its directory traversal. The matcher now stores start, immediate-file end and subtree end offsets for requested directories that it actually visits. The offsets refer to one candidate array per independent walk. A later query uses its own recursion depth, glob, exclusions and target classification over that array.

The DFS still collects a directory’s immediate files before descending through children in reverse listing order. The first requested root starts the first walk, including when that root is nested. The matcher indexes only directories it visits. An explicitly requested ignored directory can therefore still get its own walk, with the existing symlink, opaque-source and directory rules applied.

An indexed directory already produced its observed type and typed directory-listing facts in the same matching invocation. The matcher still probes explicit roots it has not visited, including missing and non-directory roots. The observation wrapper runs the physical directory read and records the fact from its outcome. The saved ranges live only for this invocation. Each visited directory, directory entry and candidate still reaches the existing cancellation check during scanning or matching.

## Verification

- The production matcher body, with counted filesystem ports, returned the same ordered freshness matches and per-target compile matches as the fixture’s picomatch oracle. The regression failed on traversal before the fix, recording 13 directory reads. It passed with 7 reads after the fix, removing six repeated reads in the fixture.
- The fixture exercises a recursive and direct-only query on the same root, a separate direct-only root, an empty subtree, an explicit ignored root, an opaque root and a symbolic link. It also checks the precise shallow candidates, a nested-first scan, and observation of a changed listing in a fresh invocation.
- The focused source-matches, source-demand and input suites passed all 40 tests through `repo tools exec --light bun -- test`, with `SEMIO_TEST_ARTIFACT_DIR` under this ticket. The source-matches suite also covers missing and non-directory roots, repeated-question filtering and cancellation, and compares classifications with Python’s `glob`.

Measurements are limited to operation counts on the focused fixture. Elapsed time and duplicate reads in the selected live Process3d request remain unmeasured.
