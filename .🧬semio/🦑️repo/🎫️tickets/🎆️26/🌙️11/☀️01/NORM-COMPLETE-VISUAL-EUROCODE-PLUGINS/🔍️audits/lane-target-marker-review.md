# Lane target marker review

The target marker reader checks regular-file type, link count, bounded size, descriptor identity and the final named inode, canonical serialized contents and target identity. The selected shared build is checked against the physical compiler store layout.

The retention pass nevertheless reads destination and marker evidence separately. In retention source line 130, targetDestinationV1 first chooses shared destination A; a later digestV1 and publishV1 can observe a replacement marker naming valid destination B. The later digest recheck validates B against itself, not against the marker that chose A. The preliminary build-to-destination ambiguity map is likewise populated from an earlier marker read than the later publication. A marker replacement between these operations can thus associate evidence or a ledger with a different destination than the one authenticated for it.

Capture parsed destination, file identity and digest together and verify the same captured marker remains named and unchanged through association and before ledger retirement. Refuse if it changes. This is a source-level race analysis; no runtime test was run.

## Follow-up

The revised capture validates the shared destination and derives the digest from the same parsed marker value, and revalidates captured inode and digest around publication and ledger passes. The reported A/B association race is addressed in the current source. Runtime validation remains pending.
