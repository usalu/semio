# Native silhouette verification

The rerun covers border presentation, the draw contract, wgpu paint and events, both TUI suites, and the renderer Dock laws. Each crate uses its own manifest so Cargo includes that crate's development dependencies. [native-rerun.sh](native-rerun.sh) records the exit status and test counts from each command and returns a failure status if any suite fails.

## Rust results

| Suite | Passed | Failed | Exit |
| --- | ---: | ---: | ---: |
| Border presentation | 14 | 0 | 0 |
| Draw contract | 23 | 0 | 0 |
| wgpu paint laws | 73 | 0 | 0 |
| wgpu events laws | 91 | 0 | 0 |
| TUI unit laws | 100 | 0 | 0 |
| TUI companion laws | 102 | 0 | 0 |
| Renderer Dock laws, after correction | 69 | 0 | 0 |
| Total | 472 | 0 | |

The focus-priority laws in border presentation and wgpu paint both passed. The first confirms shared-seam priority from the silhouette resolver. The second confirms the resolved focus colour across the full wgpu perimeter beside a selected cell.


The first Dock run found three test defects. The two geometry laws used a depth-relative lookup that resolved to the framework directory, one level below the repository root. Both now read the existing shared fixture through `repo_text!`. The body-gap law now uses the same 0.001 px tolerance as the shared geometry oracle for its floating-point arithmetic. The first run had 66 passes and three failures. All 69 passed on the corrective run.

## Storage recovery

- At the start, the host APFS data volume had about 1.4 GiB free. The mounted Rust image had 52.58 GiB free internally. The backing file occupied 96 GiB, with about 43.42 GiB of APFS blocks in use inside the image.
- The Cargo CLI checks host and workspace storage before it resolves the Rust target. Moving the host cache root to the replica mount would still leave the workspace volume check in effect. The host data volume had no APFS snapshots.
- The targeted cache collector proposed zero eligible removals. The full dry run stopped with `Deferred closure read refused` before reaching image reclaim.
- Removed 870 MiB of regenerable Node compile-cache entries older than 24 hours. The host then reported about 10 GiB available because the graph image also disappeared during concurrent repository activity. The Node compile-cache entries were the only files manually removed to free disk.
- Ran Cargo with the repository's `SEMIO_DISK_RECOVERY_CHILD=1` switch to suppress the automatic collector that can retire whole idle images. The CAS guard, independent workspace-volume admission when applicable, and Cargo store leases still run.

## Cache reclaim admission

A manually invoked, exact `repo-cache-reset:cache-reclaim-*` target can reach the in-place image owner on macOS once the backing file is observed on its declared device. That owner still verifies its issued invocation, requires the exclusive non-waiting image lease, and checks that the image is eligible to reclaim. Automatic reclaim and regrow still require the measured pressure record for the exact device. A manual regrow still enters the ordinary disk-admission path.

The extended language-independent ingress fixtures were checked against SQLite and the actual TypeScript implementation. Manual reclaim failed before the fix and passed after it. The focused Bun suite passed with 63 assertions.

A live `repo run repo-cache-reset:cache-reclaim-products` reached the native image owner and exited 0 with `outcome: skipped`. The owner checked the products image directly and skipped reclaim.
