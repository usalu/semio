# Store receipt progress during publication

The editor law on the bounded scene-splice build reached a Nakagin publication-stalled fault. The debugger stopped at that fault, after 12,718 mounted units and 11,155 exact Store advances. The final six Store calls were in PreflightingCommit and returned PreparationProgress with the same checkpoint (13, 39). They each performed two Work units and included real copy or release receipts. One retired 392 bytes and another copied 1,856 bytes. The Store was moving through commit preflight while the published row checkpoint was stationary.

The mounted witness now counts a validated nonempty physical receipt or a changed checkpoint. A Blocked step still supplies neither, and repeated empty steps still reach the 4,096-unit fault. The independent SQLite test covers a stream with only periodic physical receipts and two unchanged streams, and checks the real preparation caller wiring. Both tests passed.

The earlier Capsule Dream run had also reached a publication-stalled fault after 39,669 Store units, following successful asset admission and worker completion. After the fault the fixture continued retirement up to its bound. The earlier board and world history law reached board editing after its first document creation. The current native run includes the per-target patch backing and the physical publication witness together.

A debugger rerun of the older Capsule Dream executable reproduced the fault on fixture turn 40,617. It stopped after 300 calls to the mounted retirement pump. All 15 sampled returns succeeded, and each stopped before the granted owner quote and step. The earlier retirement prerequisites include release of the acknowledged fault page. The trace input is `capsule-publication-retirement.gdb`.
