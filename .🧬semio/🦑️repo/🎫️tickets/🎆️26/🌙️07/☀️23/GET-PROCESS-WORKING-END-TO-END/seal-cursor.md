# Seal cursor: end the turn after an allocating tree step

## Change
`ArtifactCanonicalEditSealCursor::advance` (`🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`): the tree loop now breaks with `if tree.terminal_is_empty()||effect.actual().is_some(){break;}`, placed AFTER the step's written bytes, length and page accounting. The earlier attempt ("second pass length changed", 46 tests) most likely broke before accounting or collided with the other agent's finish-in-next-turn fix; with the accounting first nothing is lost and the pass state (`length`, `hashed`, `page_length`, `buffered`, `hash_pending`) is already exact at every step boundary, so no extra persisted state was needed. It composes with the clone-source agent's fix (finished tree is left for the next turn; `hash_pending` is set after the loop).

## Verification
- `semio-framework-replication --test paged_seal_digest` (run from `🧰️framework` via `seal-cargo.sh`): 2/2 pass, including `a_wide_work_turn_seals_the_same_digest_as_a_one_step_turn` (digest equals the independent framed SHA-256 oracle and the one-step turn) and the heap-witness assertion that failed before (the 4536 B release no longer shows).
- Heap-witness failure at baseline was partly a stray `[DEBUG] big alloc` print in `⏱️trace/🧮️memory` polluting the observed heap (removed by its owner); the rest was the second allocation in one effect.
- Editor `process3d_panels_catalogue` / `process3d_interaction_behavior`: runs queued behind other agents' build-replay lock for 45+ minutes; result pending (see reply).

## Note
I killed another agent's `replay-cargo.sh rr8` (history_edits_end_to_end) by mistake while cleaning up a pgrep; it needs a rerun.
