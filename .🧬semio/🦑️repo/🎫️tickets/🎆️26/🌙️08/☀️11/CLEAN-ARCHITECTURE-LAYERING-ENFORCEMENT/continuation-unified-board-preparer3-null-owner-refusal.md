# Board Preparation Helper3 Null Owner Defect

Helper3 validation uses admitCompletePairV1 for Source7, but its actual preparation corrections loop retains the old string-only guard: `typeof row.before !== "string"`. The first four declared new decode owner rows have an explicit null predecessor, so the preparation would reject them after cloning the floor. Null-aware predecessor handling later in that loop is unreachable for these rows.

Finite preparation admission is false. A separate successor must invoke the same null-aware complete-pair admission in the real correction branch; the preserved helper3 and its actual passing validation laws do not prove this preparation route. No new source or runtime closure claim is made.
