# CommonMark Native Retirement Runtime Correction

Root's actual current run executed 24 selected laws: **22 passed, 2 failed**, with 47 ordinary tests outside the selector, 2.223 seconds of assertions and 22.1 seconds in Nx. Nextest: `959691eb-42a0-4ed4-997a-534626aaddbe`. Evidence: `🗑️generated/root-authentic-md24-owned-controls-current-typed-terminals.log`.

All original 22 laws passed. The allocator-refusal retirement law measured 25,192,449 visits in its alternating list/quote case against the unchanged 114,696 ceiling. This disproves the preceding source-level linear inference: synthetic enum continuation nodes were revisited and rewrapped. The final SQL cancellation law also failed through its isolated child. Neither failure is a compiler or fixture-layout prerequisite.

The final SQL return now holds `OwnedSnapshot` across the fallible final checkpoint and moves its snapshot out only after success. This was mounted only after the new genuine boundary RED.

Retirement now uses a fixed intrusive continuation frame in the **spare slot of the vector whose real node was just popped**. The frame retains that same vector's ownership/header and the previous frame link; block frames also retain the original pending list-item vector. It does not manufacture additional block/inline nodes. Every real node is popped once, each listitem once, and each branch continuation is resumed once. Depth adds links stored in the already-owned allocations, not call frames or fresh scratch vectors.

The unsafe ownership boundary is explicit and local: pop first makes the slot inactive; compile-time size/alignment assertions prove the frame fits that node slot; pointer write retains the original backing Vec inside the frame; pointer read moves the frame out before restoring its Vec; normal Vec drop sees only its actual remaining node length, never the retired frame slot. The nonfallible traversal has no allocation, reserve or callback that can abandon the live raw-link chain. Metadata strings and leaf payloads still drop through their actual variants.

The existing public tests keep the all-allocation-refused condition, complete live-byte release, 64 KiB stack, depth 8,192, width 4,096, and unchanged strict visit ceiling. **No threshold was weakened.** Two actual mounted files parsed through rustfmt with exit 0; generated receipts are `md-linear-intrusive-retirement-current-parsed.rs` and `md-final-sql-guard-current-parsed.rs`. Typechecking and runtime of the revised algorithm remain pending Root's sole owning Native 24-law rerun. No Cargo ran in this lane.

The new HTML lifecycle draft was updated to the same corrected fixed-frame pattern, but remains UNMOUNTED and has no runtime claim.

## Compiler Layout Follow-up

Root's next native attempt stopped before assertions on `E0080`: the original block continuation's two vector headers plus link exceeded the actual `MdBlock` slot. The compile-time fit guard correctly refused that representation. This is a compiler prerequisite, not a new runtime verdict. The separate newly added allocation law's import prerequisite belongs to the Rust executor.

The mounted block continuation now holds one block-vector header, the previous link, and a pointer to its pending-item vector header. The item header lives in the spare outer-vector slot created when the current block group was popped. Its saved header owns that same outer allocation. Separate size/alignment guards prove both overlays fit their actual slots, with no weakened assertion.

A zero-capacity pending-item vector uses a null pointer and retains no allocation. A positive-capacity active item vector always has a spare slot because it yielded the active block group through `pop`; the code asserts this ownership invariant before writing. Nested branches move that header into its spare slot, leaving the active header empty. Resuming a block frame reads its item header exactly once before the saved vector is restored. Exhausted or empty lists need no item frame: their own empty vectors drop before the parent resumes. Every retained header and backing vector consequently has one owner and one final release.

The source parsed with rustfmt exit 0 after this topology change; receipt: `🗑️generated/md-split-continuation-layout-current-parsed.rs`. Existing neutral fields and Source behavior are unchanged, so the prior registered 12-law/90-assertion Source receipt remains the relevant evidence. No Cargo ran here.

## Actual Follow-up Runtime

Root's owning rerun executed **25 selected laws: 24 passed, 1 failed**, with 47 ordinary tests outside the selector, 750 ms of assertions and 18.8 seconds in Nx. Nextest: `cf89b1c9-3b33-4c12-b438-40410e58db11`.

All original 24 laws now passed, including the all-allocation-refused linear retirement law and the isolated final SQL cancellation law. This provides actual runtime proof for the corrected continuation topology and final snapshot guard. The sole new public allocation law genuinely failed because native ownership admission used the semantic value ceiling; Root owns the shared Store bridge repair and its baseline. This result does not claim the whole ordinary owning package passed.
