# Host Spend: Release Axis and Lifetime Fuel

Status: measured, NOT fixed. All temporary `[DEBUG]` instrumentation is removed again (grep-checked). No product or fixture file changed.

Method: temporary per-renew deltas in `fixture_renew_turn`, a `post_retained` backtrace, a close-ladder rung/progress trace and a scope-body print (logs `🗑️generated/replay-h2.txt`, `replay-hl*.txt`).

## Goal 1: `history_edits_end_to_end` release axis

The 64 MiB is not a repeated receiving-scope hold. It is physical backing release at app close.

- The release-spending turns are close-ladder turns: 64 items and 64 work per turn, each item releasing 27,136 B (one causal DAG page, `MutationDagFixedSlots`, 64 slots), so 1,736,704 B per turn. About 197 such turns (about 340 MB) plus the larger OwnedStores/FinalBackings turns make up the 538,684,036 B.
- Per app close (rung trace): five `ClosingChildMember` runs of about 4.85 MB each (24.4 MB), `OwnedStores` about 19.8 MB, `FinalBackings` (156 steps) about 18.5 MB: about 62.9 MB per app, identical on every close (62.87 M, 63.12 M, ...). That is the earlier "63-68 MB per load/close cycle"; it equals `UNQUOTED_RECEIVING_RELEASE_BYTES` only by coincidence.
- Each step is a real free (`close_backing_step` of eager 128-page DAG backings per store, resident rings, snapshot lease backings). Capacity on the same caller ledger is only about 1-2 MB per app (cap 20,816 at close start, 0.8 MB at its end): births of these backings are charged to the construction/receiving-child ledger (`receiving_child_progress`, `self.construction`), not to the caller `spent`, while their release lands in the single business `spent` of the one acceptance caller (`with_acceptance_caller`, one per scenario).
- Arithmetic: 62.9 MB per app x about 8.5 app closes in one caller = 538 MB > 512 MiB fixture release stock. Nothing retries or re-claims; the scope refund fix is already in and idle scopes hold nothing, so no settle change can reduce it.

Root fix options (owner decision, not done):
1. Charge close release to the ledger that funded the birth (construction credit refunded at close), not the business `spent`.
2. Run each app's close as its own initiating event (own stock), like the host, instead of one caller per scenario.
3. Shrink the eager backings (lazy DAG pages; `MUTATION_DAG_CAPACITY` 8,192 x 3 structures x 5+ stores per app is about 62 MB).
The fixture stock must not be raised: the arithmetic only proves the limit, not that the spend is necessary.

## Goal 2: `bearbeiten_pane_loads_applies_undoes_and_closes` fuel

- Not retries. Each decode chunk's own scope-observed work is 43,819 units, but `OriginalRecordMemberSnapshotOpen::publish` (`🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs`) charges `copied_items + copied_bytes` of the step as fuel (repo-wide convention, 1 unit per copied byte). Chunks: 30,379 items / 16,775,687 B copy / 6.35 MB capacity / 5.98 MB release = 17.48 M work each (capped by the 16 MiB unquoted copy share); three of them plus one 5.6 MB chunk per load = about 56 MB copy, about 57 M work for one `SemioBrepSnapshot` member open (stack: `settle_document_archive_load` -> member open -> `original_record::received`). Lifetime fuel is 58,720,256: the first locale's load alone exhausts it (work 17.7 M, 38.5 M, 58.4 M, 60.2 M at consecutive renews).
- Source data is 78,180 B (`🌲️concrete-forest/📐️.stp`); the decoded brep snapshot is about 56 MB (about 100 k items of about 550 B) and copy is about 2.6x retained capacity (transient copy). Fixing means shrinking the decoded representation or the transient copy in the record receive path (or funding the load as its own event); it is not a loop.

## Cleanup

Temporary edits reverted: `⏱️context/🦀️.rs` (`post_retained`), `🛬️receiving/🎟️state/🦀️.rs`, `🧪️testing/🎟️caller/🦀️.rs`, `🔒️close-ladder/🦀️.rs`, `🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`. Generated logs to delete at ticket close: `🗑️generated/replay-h2.txt`, `replay-h3..h6`, `replay-hl*.txt`, `🗑️generated/h-loop.sh`.
