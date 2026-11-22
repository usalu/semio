# m07 Inventory Sample at 20:20 UTC

Two internal slice receipts at 20:19:24 and 20:19:46 UTC identify the same worker PID 54517 and Ready owner token for the workspace begun at 19:58:15.580. Both show scoped discovery with 194,411 distinct observations and 31 completed owner checks. Their active owner is `@semio-tech/dsl-derive-rs` at discovery operation `source-inputs`; that operation elapsed increased from 554,346 to 576,359 ms. This proves the same timed operation was reported across the sample, not that its internal work stopped or advanced. The denominator 600 in the separate progress text is expected seconds, not owner count.

The Ready supervisor PID 78094 reported the same inventory counters at 20:20:50. Its heartbeat is newer than the internal sample and does not itself establish new owner work. No capacity reservation wait was reported in those two slice receipts. The evidence does not establish a physical duration bound, an underlying cause, or a comparable rate with a different worker.

At 20:19:37 m05 separately renewed its dependencies disk-collecting status; a retained collector line reported zero bytes freed and 4.98 GiB free. Neither worker was Ready. No process or source was changed.

## Authored input scope

The selected owner declares a recursive DSL input and its three schema-index targets use the precision profile for JSON beneath schema directories across the repository. The local DSL subtree count was 162 files after skipping known runtime-output directories. The selector expands workspace-scoped globs and only reports the aggregate owner and operation in the physical receipt. The available mirror does not identify which target or query was active, how often it expanded each candidate set, or how much time it spent in directory enumeration versus path matching. The input profile and source evidence do not justify reducing the authored input closure or changing the live booking.

