# Native Storage and Retention Evidence

Measured on 2026-09-08 while several independent builds were active. The actual `repo:disk-report` target passed in 22.5 seconds: Nx task results occupied 2.05 GiB allocated, the default Cargo store 138.59 GiB, and the repository cache tree 44.00 GiB. This is a bounded snapshot of those three roots, not a complete machine inventory. Later `du` samples differ as active compilers keep writing.

| Measured path | Allocated GiB (`du`) |
| --- | ---: |
| `target/wasm32-unknown-unknown` | 31.98 |
| `target/wasm32-wasip2` | 36.87 |
| `target/debug/incremental` | 65.26 |
| `.🧬semio/🦑️repo/⚡️cache/agents` | 5.92 |
| `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules` | 5.19 |
| `.🧬semio/🦑️repo/⚡️cache/cargo/browser/wasm32-unknown-unknown/debug/incremental` | 29.11 |

The largest measured default debug incremental namespace was a Stdio instance at roughly 6.91 GiB. Several other Stdio disambiguator namespaces each occupy multiple GiB. This establishes their sizes, not that those namespaces are obsolete or interchangeable. The browser Cargo store contains about 29.11 GiB of target-specific incremental state, while the default Cargo store also contains a substantial `wasm32-unknown-unknown` tree. Actual compiler flags, features, profiles and producers must be compared before concluding they can share one namespace.

Current storage handling remains incomplete. `repo:disk-prune` delegates test-evidence cleanup and reports that native state is preserved; it does not enforce the proposed aggregate native budget. Artifact staging has an exclusive publication file that fails immediately on contention and does not yet support stale-lock recovery. Dependency synchronization also needs a cross-invocation resource boundary: an earlier Bun installation transiently removed an Nx module while another task finalized. That failure was repaired by rerunning after synchronization, but the race still needs a permanent concurrency contract.

Next implementation must make store ownership, compatibility, active use, budget groups and cleanup handlers explicit. Native cleanup must obtain the native tool/resource boundary and preserve live operations. Age or file size alone is insufficient proof that a compiler namespace can be removed. No native compiler state, active process, source directory or dependency installation was deleted during this inspection. Nx log retention is already separately implemented and tested; it does not bound Cargo incremental storage.
