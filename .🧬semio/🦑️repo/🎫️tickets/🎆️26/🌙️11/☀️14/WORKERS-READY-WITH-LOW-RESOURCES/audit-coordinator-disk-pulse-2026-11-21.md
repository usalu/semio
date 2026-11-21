# Coordinator Disk Pulse — 2026-11-21

At 02:14:19 UTC the ticket filesystem had 33,366,179,840 available bytes against the existing lab admission requirement of 60,129,542,144, a 26,763,362,304-byte deficit. This is a point sample, not a reservation. No alternate authorized volume was established.

Metadata-only backing allocation / mounted used bytes were: Rust 22,711,402,496 / 20,599,332,864; Graph 13,355,540,480 / 7,178,326,016; Dependencies 14,794,481,664 / 14,339,887,104; Tools 3,786,579,968 / 3,517,710,336; Nx 935,665,664 / 829,493,248; Products 62,865,408 / 61,886,464; Diagnostics 67,133,440 / 8,863,744; Leases 66,060,288 / 5,701,632. The six larger images are below the existing 3× ratio. Diagnostics and Leases exceed that arithmetic ratio but their combined entire backing allocation is below 134 MB; this is not a determination of production reclaim eligibility or a measured return.

The exact lease-directory registry contained shared holds naming graph, Rust, dependencies, tools and diagnostics; some registry entries were old, so the listing alone is not a proof of their current process liveness or an idle image. The leases image backs the lease infrastructure. Neither custody nor the policy establishes a nonwaiting, meaningful reclaim candidate sufficient for the lab. The arithmetic differences do not promise reclaimable blocks. No reclamation, exclusive queue request, lease change, broad content scan or workload intervention was performed.

## 02:54 UTC Recheck

At 02:54:18 UTC available space had risen to 40,121,397,248 bytes, still 20,008,144,896 below the lab floor. Rust backing allocation / mounted used was 22,694,490,112 / 20,581,154,816 bytes; Graph was 13,355,540,480 / 7,245,238,272. Dependencies, Tools, Nx and Products likewise remained below the 3× ratio. Diagnostics and Leases still exceeded that arithmetic ratio but their combined total allocation was under 134 MB; fresh registry rows named shared Diagnostics custody, while Leases is infrastructure. Fresh rows also named shared Rust, Graph, Dependencies and Tools custody. No exact meaningful idle reclaim opportunity was established.

A shallow metadata listing of `.cache/b/c` found only small direct files. The cache-server and remote-execution roots are active directories, without a demonstrated idle claim or measured reclaimable amount. `/Volumes` contained Macintosh HD and semio-peers, both resolving to the same device 16777234 with approximately 40.28 GB available; neither establishes a distinct authorized backing volume. No deep walk, reclaim, exclusive request or lease mutation was performed.

## 08:39 UTC Recheck

At 08:39:01 UTC the host had 52,816,564,224 bytes available, a 7,312,977,920-byte gap to the 60,129,542,144-byte gate; the sample is not a reservation. Rust backing inode 686284559 allocated 12,336,611,328 bytes with 4,641,349,632 used on its mounted filesystem (2.66×), below the existing 3× predicate. Graph inode 711799973 allocated 13,639,426,048 with 7,421,714,432 used (1.84×). Dependencies, Tools, Nx and Products were likewise below 3×. Diagnostics and Leases together had only 133,193,728 bytes of backing allocation. These arithmetic differences are not measured reclaimable host blocks. No current exclusive custody was established.

Macintosh HD and semio-peers shared device 16777234. The separate semio mount had only 65,196,032 bytes available and was not established as an authorized alternative for the lab. No meaningful eligible compaction or separate authorized volume was demonstrated. No removal, signal, reset, exclusive request or queue action was made.

## 09:28 UTC Recheck

At 09:28:33 UTC available host space was 55,958,122,496 bytes, a 4,171,419,648-byte gap to the lab gate. Rust backing inode 686284559 had 13,773,246,464 bytes allocated and 5,499,719,680 bytes used inside (2.50×); Graph inode 711799973 had 13,639,426,048 allocated and 7,215,550,464 used (1.89×). Both are below the existing 3× reclaim predicate. Dependencies, Tools, Nx and Products were also below that ratio; Diagnostics and Leases combined were only 133,193,728 backing bytes. Arithmetic slack is not demonstrated reclaimable host space.

Fresh raw per-user lease-registry rows named shared custody of the checkout leases image by Cargo processes 31052 and 27658. No fresh Rust or Graph image hold appeared in this registry scan; absence is not proof of idleness, exclusive eligibility or actual host return. Stale entries were not pruned. Macintosh HD and semio-peers resolved to the same host device, and no separate authorized backing volume was established. No maintenance, removal, signal or additional lab was initiated.
