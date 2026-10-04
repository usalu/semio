# Process3D Current Initialization Phase Prerequisite

The authentic Root current32 compiler stopped before Process3D assertions on E0004 at initialization step match line3511. Full before log is generated/root-current-thirty-two-first-late-and-repaired-native.log, starting line366439.

Current source contains HashInverse, HashRedoForward and HashRedoInverse only in the phase enum: one reference each, no reachable transition, cursor or field implementing those stages. The actual applied path ends ApplyForward directly in CommitApplied, and FindRedo goes directly to CommitRedo. Both commit paths delegate the actual canonical ArtifactStoreInitializationRuntime::push_applied_edit/push_redo_edit. Those call effective_edit_digest over the full edit_digest; edit_digest_extending retains forwards, inverse and metadata chains, including supersessions. Removing the three unattached enum variants restores the actual exhaustive state graph without adding invented compatibility transitions, fault stubs or changing history digest authority.

Exactly three declaration lines were removed with occurrence guards against concurrent edits. All transitions, consumers, mutation/history bodies and the held new semantic-census law remain unchanged. Rustfmt parser checks syntax only; Root owns compilation and assertions. No Native or Source pass is inferred.
