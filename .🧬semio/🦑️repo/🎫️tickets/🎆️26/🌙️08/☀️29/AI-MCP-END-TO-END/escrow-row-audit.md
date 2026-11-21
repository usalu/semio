# Escrow Row Audit

The escrow table has 32 physical rows. A full table without a reclaimable row makes a sized return claim or row reservation return pending; it does not itself produce a WorkLimit. A sized claim independently returns WorkLimit when the raw remaining lifetime work is below three, before its table-full check. The observed unframed WorkLimit therefore does not establish row starvation without additional diagnostics.

The current receipt preissues four return-place claims, an exposure claim, and a guard before the producer runs. The producer's retained and work loans partition currency, but do not reserve physical rows for the following receipt. A saturated table can therefore obstruct the next receipt. Checking free rows once does not hold them against later producer claims.

Accepting a returned header acknowledges only tombstones in the snapshot captured at borrow time. Settlement after that snapshot needs a later accepted snapshot before paid reclamation. A potential fix must enforce Source-owned row headroom throughout producer allocation and staging, grant the receipt access to that headroom, and preserve producer progress and multi-claim atomicity. This remains a hypothesis until actual row states and work currency at the failing frontier are observed.
