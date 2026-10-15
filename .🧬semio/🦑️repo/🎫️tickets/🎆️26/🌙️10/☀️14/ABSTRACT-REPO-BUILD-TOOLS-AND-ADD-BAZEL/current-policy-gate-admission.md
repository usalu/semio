# Policy Gate Lease Snapshot

Core requested one read-only live snapshot after the current whole graph gate reported an original 20-second admission refusal. At **2026-10-15 02:10:10 UTC**, the snapshot covered the global graph-facts resource, all 64 current graph-writer shard resources, graph-image custody, graph charge/store custody, canonical claim catalogs and lease-image custody.

Of 71 exact resource paths checked, 45 existed. Their queue directories contained no tickets; `lsof` found no matching live database descriptors (exit 1, no matches). No PID or holder mode was therefore inferred. No lease was acquired, lock probed, database opened, cache mutated, process controlled or peer output read.

This snapshot does not establish whether the earlier refusal had an external holder: the gate or holder may have settled before capture. It supplies no basis for changing original test budgets or production control. Evidence is `🗑️generated/current-policy-gate-lease-snapshot.json`.
