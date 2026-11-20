# Descriptor retries in wqJ4HZO

All seven `job:describe-tool` attempts ran Cargo once and yielded with exit 75 for a typed input change. The attempt logs and retained tails under `.cache/b/r/m/components/runs/wqJ4HZO/l` show the following Cargo-phase durations. These are elapsed time inside the compiler phase, before the driver's interattempt delays.

| Attempt | Cargo seconds | Reported changed input |
| --- | ---: | --- |
| 1 | 1,234.331 | Store child |
| 2 | 23.760 | Store delegated operations and publication |
| 3 | 54.700 | Store child |
| 4 | 22.385 | Store child |
| 5 | 236.517 | Replication transition fold index |
| 6 | 152.148 | Store cursor admission dispatch and apply batch |
| 7 | 27.051 | Store snapshot clone |

Total Cargo time was 1,750.892 seconds. Four of six retries had 22–55 second Cargo phases. Capacity wait dropped from 69.516 seconds on the first attempt to 0.110–0.222 seconds on the later attempts. Each later attempt still entered Cargo. It did not use a previously published descriptor tool to complete the job.

## Early detection evidence

Each attempt has one Cargo phase, no stage, provenance, custody or consumption phase, and two build-level snapshots retried for the same path after 2 and 8 seconds. There is no completed source digest mismatch progress event. The current monitor runs only on this retained artifact path, converts a verified physical SHA-256 mismatch into a build-level input error, stops the owned Cargo tree and skips unsuccessful receipt observation. The post-completion mismatch route runs after staging and receipt creation and explicitly publishes its mismatch event. An ordinary compiler failure keeps its primary compiler error even if fallback receipt observation fails. Together with the typed input failure on every attempt, the recorded phase boundary strongly identifies live monitor cancellation. The logs render only the churn path/reason and do not directly print the watcher's mismatch payload or terminal signal. They cannot distinguish whether a directory event or rolling metadata probe produced the evidence, nor measure latency from the first changed byte.

The rolling monitor can only raise an early failure for inputs in its admitted Rust roster and within its caps. It neither pins the full source epoch nor provides a chance of success while admitted content changes in every attempt. The monitor bounded the work when it found a change. The driver's initial attempt plus six restarts still reached the prescribed terminal failure.

## Published tool and existing warm paths

The published release Darwin compiler receipt at `.cache/b/p/m/browser/release/tools/descriptor-emitter/darwin/compilation/receipt.json` records a successful compiler start at 05:16:56 UTC and final observation at 05:35:41 UTC, before this generation exhausted its retries. An independent Bun hash pass checked the currently marked inputs in the published index. It found 3,340 matching files, 31 changed files, 6,545 expected absences and no unstable observations. The index inode, size and modification and change times held across the pass. Differences include all five distinct paths reported by the restart logs, other Rust sources, and Cargo manifests and locks. Stabilization on these later bytes would still leave that published receipt stale.

Play's `describe-tool` job is unconditional and feeds the describe stage. `withDescriptorEmitterTaskV1` already validates the executable digest, exact retained inventory and successful compiler receipt under its artifact reader and current Cargo configuration scope. That verifies historical retained bytes and current configuration custody. A hit path would also need to prove the exact retained source claims match current source, including required absences and completeness. Only then could it bypass compilation while preserving the actual emitter-to-component check. The existing downstream describe-stage cache uses the component and emitter digests and runs after acquiring the verified reader. It cannot skip the gated emitter job.

The current repository compiler-store product lane and source admission memo already support warm retries after source stabilizes. The optional native unit cache has separate direct verification for cross-lane reuse, but this ticket's release launcher defaults the unit cache off and its actual release hit rate has not been established. The existing CAS revision materializer and incremental workbench provide private source views for their own consumers. The Play release still opens shared-checkout source, so switching generation or warming the local compiler store does not pin the release's source inputs. The separate release-source snapshot audit records the missing integration.

Only retained logs, source and the current publication's input claims were read. No build or Cargo command was run.
