# Windows Graph Snapshot Paths

The g1-m05 workspace receipt at 15:13:49 UTC on 2026-11-20 recorded an `ENOENT` opening a path with the absolute snapshots directory repeated in its own filename. Both publication and eviction derived a digest by splitting an absolute path on `/`. On Windows the backslashes left the entire path in the first 64 characters.

The graph artifact owner now extracts the digest from the final path component with the system path grammar that accepts both separators. Publication uses the digest and nonce to create a relative staging name in the snapshot directory. Eviction authenticates the original manifest against the same digest before deletion. Publication uses the repository's native directory metadata barrier after renaming the files. The barrier runs with the original issued control inside the existing graph writer lease. On Windows the barrier uses the directory flush port. The physical suite now derives directories and separators from the host path module.

The neutral fixture covers Windows compressed records, Windows manifests, and a POSIX manifest. An independent SQLite expression derives both the digest and the staging name from that fixture. The regression reproduced the malformed g1-m05 digest and staging name with the old extraction. It passed after the correction, along with the namespace assertion. The shared patch passed `git diff --check`.

The direct Bun route lacks the original test invocation required by the three physical cases. The owner-backed `repo run repo:test-graph-bounded --output=stream` was still planning with PID 73132 when this report was written. The native graph-plan process was PID 73220. Windows storage on g1-m05 still needs a current worker receipt.
