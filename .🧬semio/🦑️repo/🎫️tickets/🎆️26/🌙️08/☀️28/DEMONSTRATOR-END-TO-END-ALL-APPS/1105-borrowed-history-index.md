# Borrowed History Index

The current public first-party path is protocol::causal::transition::HistoryFoldIndexRead, explicitly reexported by replication. Its defining owner is replication/causal/transition/fold/index/read. The new, next_demands, step, and matched methods operate over the actual borrowed index and query; the cursor binds actual index address and mutation epoch and retains bounded comparison byte position. A matched token borrows the genuine value through value(index). Rank is internal and has no public getter.

The current step returns Result<(Option<Option<usize>>, RetainedCloneProgress), ValueError>. It does not expose raw work or last_turn_work. An actual original-context consumer must first add a rightful defining raw-outcome seam; progress/error must not be used to guess CPU work. This audit made no production changes. The replication owner is outside the captured lower Ordered/Equality graph.
