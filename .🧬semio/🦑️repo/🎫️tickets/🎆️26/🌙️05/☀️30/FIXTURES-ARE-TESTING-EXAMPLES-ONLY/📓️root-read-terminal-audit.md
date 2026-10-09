# Read terminal physical receipt join

Read-only original Source, no compiler/native execution. A concrete physical release receipt is lost.

SnapshotReadRetirement::step returns the registry alias close result at line36. StoreRoot snapshot_registry_alias_close_step745 removes the original last alias, unwraps/drops the exact registry and returns Complete with copied_items1 and released_bytes=snapshot_registry_frame_bytes(). The shared-alias branch739 also returns nonzero copied_items1 Complete, though no physical frame release there.

SnapshotReadRetirement’s RetirementCursor adapter42 maps every Complete(_) to unit RetirementStep::Complete, discarding both receipts. Value ControlledRetirement157–170 consumes unit Complete as (0 copied bytes,0 released bytes,true). The later typed cursor terminal shell release131–134 is separate storage; it cannot restore lost registry frame release credit. Ownership terminal checks remain true, which makes this loss invisible to logical emptiness alone.

The same genuine Value Controlled adapter203–207 already gives the minimal mapping: only default empty Complete becomes unit Complete; nonzero Complete becomes RetirementStep::Progress(progress), preserving actual axes. The now-terminal child can publish empty completion on its next turn; do not repeat registry destruction or fabricate another birth/release. Keep full supplied grant and original alias race restoration745 intact.

Meaningful native test should join exact original registry-frame release and subsequent read-cursor/frontier shell release under System observation; verify denied release/depth preserves pointer/alias and no destructor work, shared alias transfer has item-only receipt, and last alias original backing+frame freed exactly once. Existing Presence backlog/cancellation/pointer and independent laws must remain. This audit identifies Source semantics, not a passing test.

Three current observer-only hashes are retained in 📥️root-read-terminal.json; shared full-file drift is not authored by this lane.
