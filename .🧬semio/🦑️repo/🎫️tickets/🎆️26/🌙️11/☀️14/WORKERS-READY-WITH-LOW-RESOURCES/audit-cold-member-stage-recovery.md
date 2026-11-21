# Cold Member Git Stage Recovery

After restart the member reconstructs proof of its last applied index by folding the held stage registers. If the held fold failed, the apply step waited for missing objects even when the incoming author registers had already superseded those objects. A private Git fixture used an initialized member with a settled base and staged blob, removed that blob’s loose file, cleared the in-memory Git state and delivered the replacement registers. One blob genuinely needed by the current stage was initially absent. The member correctly waited, but still reported `objects-missing` after that blob arrived and all current inputs were available. A writer lock installed at that point made the failed attempt observable without letting it change the index.

When held proof fails, a member now collects and folds the current author stage. It requires the current inputs before attempting whole-stage adoption. Git verifies the target through a private index before the existing exclusive writer lock publishes it. The old index is backed up under that lock. The member settles only paths pending in this attempt, and records the exact adopted tree without assigning a signature to an index a concurrent process may already have changed. The author still requires held proof. Missing current objects still defer adoption, and cancellation before the index rename leaves the prior stage in place.

The schema-validated cold fixture records the old and new object contents. The regression verified that incomplete current inputs, an active writer lock and a cancelled invocation each preserve the old index. It then matched the recovered stage to an independently assembled Git tree, verified the backup byte-for-byte, and confirmed the worktree payload was intact.

The focused cold fixture passed 16 assertions. The Git module passed 66 tests and 541 assertions. Four selected replica integration tests passed 29 assertions for restart, missing objects, index recovery and pack hydration. These tests used private repositories. Live workers were left outside this verification.

## Hydration priority

The early retry of the held fold formerly called the supplied materializer even when the current stage was already complete. In the private reproduction the genuinely missing current blob prompted one call. Completing the stage triggered a second call for obsolete held proof. The materializer used by the replica can process unrelated Git objects, including large packs.

The author retains its held-fold retry. Members hydrate only on observed missing current dependencies. Direct object probes cover the base and author entries. A failed current fold can report absent subtrees from Git’s tree walk or absent blobs from its private index. Whole-stage verification can also report exact missing objects after releasing the writer lock. The callback is consumed once per apply.

The extended cold fixture supplied a missing subtree exclusively through a real Git pack and proved that the ready recovery requested no additional hydration. Three targeted Git tests passed 61 assertions. Four replica integration tests passed 33 assertions, including the advertised pack and the stage that settles before unrelated pack hydration.

The full Git module passed 66 tests and 546 assertions after the priority change.
