# m07 tools timeout under older Ready observer

Physical wrapper 68477 started at 16:10:09.265 UTC and exited at 20:10:10.623, after 14,401 seconds. The durable timing receipt records stage-timeout, exit 1 and 2,058 MiB peak. Its launch and terminal watcher was Ready PID 68450, which started at 16:10:07.776 and still owned the ordinary successor at 20:14. The process predates the tools grace and terminal evidence landings.

The current replica attestation at 20:13:59 UTC reports sync false, 179 pending incoming rows, and server cursor 163:1206146. The coordinator had reached segment 170:1030728 and published the new observer evidence and exact code there. The m07 cursor confirms those source changes had yet to reach this origin. The receipt does not establish why sync is false.

The historical failed pane retains 96 provider-check messages reporting 7,756 edges, owned SIGTERM, lease release, and a dependency cancellation record with 15 completed Source changes, no unresolved Source transition and no whole allocation retirement. A repeated edge total supplies no evidence of strict counter advancement. The active tools mirror has moved to the ordinary successor, so the available files lack the previous raw terminal window and pre-exit registry records. At 20:13:39 the successor had built and published the scheduler compiler helper and reached the compiler-helpers ready marker.

This timeout cannot verify the new observer guard because the watcher and the replica predate it. The original producer changed actual Source bindings, placing that observation within dependency completion. The terminal pane supplies no narrower authenticated final hold history. This inspection was read-only.


## Natural successor reached Ready

The next physical tools stage started at 20:12:24.231 UTC under the same Ready observer and exited normally at 20:16:42.307. The durable tools receipt is exit 0, outcome `ok`, 258 seconds, 875 MiB, digest `57d7264ca1ac`. The later Ready status was `ready:true` with no current stage under PID 68450. The host is the separate macOS arm64 16 GB / 256 GB Mac mini M4 reported as `ueli-workstation-mac-mini-m4-16gb-256gb-02-lan.fritz.box`.

The timing history adds only the successful tools stage. The latest native stage receipt was already accepted at 01:25:30.363 UTC on November 20 and the latest workspace success ended at 04:06:22.611 on November 18. The unchanged published `readyMs` is November 18 at 04:06:22.830. No fresh repo-mcp, dependencies, native or workspace run was credited during the fast successor. The tools log independently records the newly built and published scheduler helper, successful tool verification and Bazel 8.4.2 readiness on darwin/arm64.

Replica status stayed separate. At 20:17:14 the fresh attestation still showed sync false, incoming 179 and server cursor 163:1206146. This is a fresh Ready state for the worker's locally accepted stage digest while its Source replica remains behind the coordinator.
