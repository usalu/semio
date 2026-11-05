# Fleet Admission Snapshot

At 2026-11-04 22:14:25, the owned repo fleet status, leases status and capacity status routes report ordinary capacity queuing, with no confirmed lock or capacity cycle. This is one snapshot, not a liveness proof.

Four admitted jobs reserve 14,999 MB and observe 4,226 MB; ten requests wait. Memory pressure is normal, with 5,556 MB spare above the 4,096 MB floor. The head request (pid 3740) waits because CPU cores are fully booked; later requests wait behind that earlier request. Current grants reserve 15 jobs in total, including nine Cargo compiler CPU leases. Thus the UI wording memory wait does not mean actual host memory exhaustion here.

Lease diagnostics report 36 held, zero waiting and zero deadlock cycles. Cargo admission and the dependency-store owner are marked progressing. The parser-source run holds a grant while a child request waits for eight jobs, but other running grants can complete and release cores; this snapshot supplies no closed wait cycle. Ten recent lock-order violations are historical diagnostics, not current waiting edges.

The shooting chat session has zero grants and one queued request. No process was killed, no test or build was started, and no policy, cache, lease or capacity state was modified.

Separate infrastructure findings remain: fleet reports caches over budget by 27.6 GiB and a drifted sealed snapshot. These are distinct from the immediate CPU admission wait and are not changed by this audit. Native and React owners should retain their admitted queue positions.
