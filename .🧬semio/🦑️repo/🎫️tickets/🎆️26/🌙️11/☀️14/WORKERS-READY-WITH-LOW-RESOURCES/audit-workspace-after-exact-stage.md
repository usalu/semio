# Workspace Graph Progress After Exact Stage Publication

The 17:43 UTC stage-intake observation recorded nine reporting workers at coordinator tree `7d8c0b059bcaed05be881279fe59b4fa8305d514`. At 17:48 UTC their latest stage mirrors still recorded that tree and all nine Ready mirrors showed an adopted workspace stage running with `ready=false`. The repo replica fleet command then reported 0 of 12 current Ready, two stale Ready receipts and one missing. The separate attestation comparison considers records stale after 60 seconds. It reported g1-m02 as stale at 368 seconds and g1-m05 as lagging while settling its comparison. The Ready records are self-reported and have their own ten-minute freshness window.

| Worker | Physical-shaped progress from the rolling workspace log |
| --- | --- |
| g1-m03 | Across samples its shared fact-store leader advanced from 26 to 34 of 66. The Ready client still waited behind that load at 19,803 seconds of total slice time. |
| g1-m07 | Its slice advanced from 26 to 39 of 600 after transitioning from inventory to owners. |
| g1-m08 | Its slice advanced from 149 to 172 of 600 before the later owner-read sample. |
| g1-m10 | Across samples it advanced from 634 to 777 completed owner checks and from 725,440 to 966,400 verified memo rows. The distinct source count stayed at 308,509. |
| g1-m11 | It advanced from 21,616 to 22,186 completed owner checks and from 1,451,840 to 1,452,160 verified memo rows. The distinct source count stayed at 308,324. The last slice sample was 25,482 seconds. |
| g1-m12 | Its slice advanced from 34 to 64 of 600 before the later owner-read sample. |

The completed-check and verified-row qualifiers in the live inventory output and the `taken (pass 2)` line on g1-m02 show that the earlier reporting fixes are in physical use. A graph slice's elapsed count spans its phases and passes. It cannot measure the duration of the current owner's lookup. Completed-check counts can include repeated checks of one owner at distinct inputs.

Comparing commit `b4927d2c311` with enacted stage tree `7d8c0b059b` found no changes in the Ready caller, resident planner client or slice-report modules. The tree contains the newer pinned-implementation guard in the graph loader. On both successful and failed inference it checks the retained observations before forcing a fresh inventory. The guard fails when an observed input still matches the current disk bytes and those bytes disagree with the implementation pinned by that process. An ordinary mutable Source change can still go through the existing repair path. This guard saves another inventory after a proven pinned implementation mismatch. It does not skip normal inventory or owner work.

The sampled graph slice counters place the start of the work before the 17:42 UTC stage publication. The Ready report names an adopted stage and the replica attestation identifies replica code and staged bytes. Neither identifies the graph code already loaded by the detached child or the shared resident job. The rolling observed logs supply no newer planner reply, clean workspace completion or Ready transition. They do not isolate a single costly owner or prove the new guard executed. Renewed movement during this sample did not justify changing the Ready caller, watchdog or resource admission. The remaining acceptance evidence is a workspace graph reply and terminal success from the existing live attempt or a naturally launched attempt observed with the current graph implementation.

The focused graph-repair and Ready-watchdog suites passed 36 tests and 546 assertions through the repository Bun wrapper. They covered proven pinned drift after successful and failed inference, ordinary Source repair, bilingual inventory progress with a stationary source high-water, and the existing quiet-stop and adopting-process behavior. The first test invocation was refused before execution because the runner requires an explicit relative path. The rerun used explicit paths and private ticket scratch.

[Prior stage publication](audit-durable-replica-stage.md) · [Owner pass accounting](audit-workspace-owner-pass-progress.md) · [Inventory owner progress](audit-m06-inventory-owner-progress.md) · [Ready receipt freshness](audit-readiness-freshness.md)
