# History Replay Grant

## Decision
Raise the production grant; do not split the replay preparation. The copy grant of `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` is derived, not hard-coded: `covering_copy_bytes(LARGEST_INDIVISIBLE_STEP_COPY_BYTES)`. The constant is 377,696 B, so the grant is 524,288 B (512 KiB). This was already in the tree (another agent, GIS ticket `gap5-history-receive-fix.md`), and it covers the 33,976 B replay preparation step, the 277,538 B record receive and the 377,696 B member open. The test-only 65,536 ceiling in the acceptance policy is gone: `🎛️acceptance-policy-pump-and-drive` reads `INTERACTIVE_COMPONENT` directly, so tests and production share one grant.

Why not splitting: the 11 KiB `ArtifactDerivedOperation<P,M>` is an inline value the store replay owns. Reshaping it means editing the store-engine replay while another agent is mid-refactor there. The only cost of the larger grant is a larger per-step loan ceiling, and `covering_loan` and the item/capacity limits stay unchanged. The schema has no twin: the mounted policy is a Rust const, and no JSON/TS source defines INTERACTIVE_COMPONENT (grep over json/ts/schema files found none).

## Files
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🦀️.rs`: docstring corrected (it still said "32 KiB copy").
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧪️tests/🦀️.rs`: regression law `the_interactive_copy_grant_covers_the_largest_indivisible_step_with_power_of_two_headroom` now also asserts the grant funds each measured step (33,976 replay preparation, 277,538 receive, 377,696 member open).

## Results
- `semio-plugin-mounted-owner` law `interactive_copy_grant`: 1 passed (via MCP cargo, shared, prototype).
- `semio-s-artifact-process-process3d-editor` is not in the root workspace that MCP cargo plans, so the MCP call failed with "did not match any packages". Native run `replay-cargo.sh hist test -p semio-s-artifact-process-process3d-editor --lib history_edits_end_to_end` (build dir `build-replay`, log `🗑️generated/replay-hist.txt`) was still compiling the framework crates when this note was written (about 100 crates in 90 min on a shared machine). NOT yet verified end to end: framework history-edit acceptance and process3d `history_edits_end_to_end`.
