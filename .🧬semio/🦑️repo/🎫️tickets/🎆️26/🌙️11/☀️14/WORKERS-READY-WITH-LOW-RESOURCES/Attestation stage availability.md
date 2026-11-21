# Attestation stage availability

The replica view regression reported a single difference in `indexTree` while the archivist had just returned a transient null tree. The peer had a known tree. The checkpoint comparator treated the null as a different stage, and the local attester could replace prior Git evidence with a checkpoint that contained that null.

The local attester withdraws its Git checkpoint until a present repository can produce a stage tree. It still writes the current observed HEAD. A genuinely absent repository continues to produce a checkpoint with two null values, so two absent repositories can agree. The on-wire attestation schema retains its existing representation.

With comparable cuts, the checkpoint comparator continues to compare HEAD. A null stage alongside a known HEAD leaves stage agreement undecided. Comparisons of actual tree IDs still produce a stage difference when the IDs differ.

The new regression was red before implementation for both unavailable-stage comparison vectors and the persisted checkpoint withdrawal. After the change, 21 focused tests passed with 65 assertions, including the independent Python agreement oracle and fixture schema check. The only schema edits define the new language-neutral fixture cases. The scoped diff check passed.
