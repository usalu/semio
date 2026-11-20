# Worker Workspace Failures at 15:21 UTC

Durable records under /Users/Shared/semio-store/v1/origins were read on 2026-11-20. Timing records are terminal exit receipts, not liveness estimates.

| Origin | Workspace start UTC | End UTC | Duration | Result | Peak MB | Digest |
| --- | --- | --- | ---: | --- | ---: | --- |
| g1-m05 | 15:09:02.470 | 15:13:49.631 | 287 s | exit 1, attempt 10 | 777 | a3e7b1c5bbf6 |
| g1-m06 | 15:14:10.898 | 15:21:04.675 | 414 s | exit 1, attempt 16 | 4423 | a17ef0f43bdb |
| g1-m11 | 15:08:44.168 | 15:15:59.986 | 436 s | exit 1, attempt 15 | 1923 | a17ef0f43bdb |
| g1-m12 | 14:52:39.872 | 15:02:12.540 | 573 s | exit 1, attempt 21 | 2502 | a17ef0f43bdb |

The three macOS failed logs report identical Rust E0061 in semio-plugin-window-config: owner-registry/🦀️.rs:185 calls advance(publication, body, grant, receiving), but store-owners/🦀️.rs:194 requires a fourth RetainedCloneGrant argument before receiving. They failed compiling the norm-din4108-rs workspace target. This is a compile failure, not evidence of an individual resource stall. The logs describe the source at those attempts; current source may have changed.

Windows m05 has an independent failure: ENOENT opening C:\semio\.cache\b\g\m\snapshots\C:\semio\.cache\b\g\m\snapshots\d1959efcddc5610f7d6a928f81d31bb6~7b9a698a. In 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts, withRepositoryBuildGraphArtifactV1's publish constructs the temporary name with target.split("/").at(-1). On Windows the backslash path remains whole, and join appends that absolute-looking string beneath the snapshots directory. Use native basename at this boundary. The same file's retainSnapshotBudgetV1 extracts a digest using row.path.split("/"); that must also use native basename. A Windows path fixture should cover both publication temporary name and retention digest extraction; test actual storage on Windows as well. The reported path truncation to 64 characters follows the existing slice, explaining the incomplete digest.
