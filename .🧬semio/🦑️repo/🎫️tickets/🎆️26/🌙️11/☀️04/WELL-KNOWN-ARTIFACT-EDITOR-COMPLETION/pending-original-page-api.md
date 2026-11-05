# Pending Original Page API and Fixed Sink

Production FullOp fields and publication pumps remain untouched. Two independent owners now supply the future integration seam: central JsonBorrowedSinkCursor below pack JSON's existing controlled/borrowed bridge, and PendingPublicationPage below retained-command/full-operation/📃️pending.

## Central Traversal

The existing String cursor and new fixed-slice cursor share one advance_source traversal. Existing String output remains its owned output path; fixed-slice framing retains no output String or source reference. The new cursor separately admits frame/path allocations, retains a fixed scalar atom, and flushes positive prefixes into a caller slice. One call performs at most one existing central traversal transition or one output flush. maximum_work=0 pauses unchanged; copied_items and CPU work remain independent. No complete-source premeasure or prefix restart is used.

Public API: empty(maximum_depth), next_demands(output_bytes,body), step(source,output,grant,maximum_work,control), written_bytes, pending_bytes, is_complete, failed, close_demands and close_step. Work and receipts retain failed-action evidence. Source error/cancellation keeps the accepted caller prefix and physical frontier for funded close. The parent must keep the borrowed semantic source immutable while a cursor is active.

## Pending Intent

PublicationPagePlan freezes exact five-field token plus scalar PublicationPageIntent. Supported intent is two-generation lane receipt, accepted count/lane, Ui selector, Download selector and raw reason/lane. PublicationPageSource borrows actual Ui/Download/raw producer data synchronously; no references, cloned Ui scope or ToValue tree enter retained state.

Capture happens after semantic acceptance and generation update. capture_demands/capture quote the real Self extent. backing_demands/advance_backing forward the actor's original fixed4096 backing birth and bounded initialization. write_demands/write_step retain framing state and bridge NativeDecodeControl::with_encoding_receiver to NativeEncodeControl::new_forwarded using the actual remaining authority and allocation port. No replacement control or ceiling is created.

PublicationPageWrite always carries this call's written byte count, actual retained progress and work, including error. PublicationPageWriteFailure keeps primary and forwarding refusals separately. A refused write prevents handoff; the accepted plan and accumulated byte count remain owned until paid closure. The caller must preserve and account both refusals, post exact progress even on error, and close retained custody rather than replaying semantic acceptance.

After framing, traversal_close_demands/close_traversal physically return traversal allocations. handoff_demands/take_page use the retained token and lane, move the original backing and clear plan custody only on successful handoff. A denied handoff preserves the same plan. The caller must queue or retire the returned page, never regenerate the Store commit or pop another outbox item. close_demands/close_step return cursor, backing and plan in that order under original retirement authority. Cancellation creates no fresh budget.

The parent must service accepted pending intent before generation freshness checks or new semantic actions. Later document generation/cancellation cannot rewrite the frozen accepted receipt. Raw fault/terminal input is borrowed already-framed source data; any upstream fault-framing work must itself be financed by its defining owner.

## Validation and Actual Native Targets

TDD neutral pending law initially failed while owner was absent:1 pass/1 fail/1136 assertions. Central source law also failed before its owner existed. Latest pending+central neutral checks:4 pass/1164 assertions/262 ms. New owner/test files parse under pinned Rust AST. These do not prove native semantics or physical receipts.

Native tests are separately declared Cargo compilation units using public APIs, because broad preexisting unit roots exceeded unchanged Source limits after additions. No checks or quotas were bypassed. Authoritative readRustSourceClosureV1 results:

| Unit | Files | Bytes | Active lines |
|---|---:|---:|---:|
| retained-command normal lib |17|239553|2543|
| borrowed-publication-wire test |2|27152|77|
| pending-original-page test |2|31633|95|
| pack-json normal lib |18|260330|3420|
| borrowed-json-sink test |2|5150|48|
| actor lib test |28|213592|2634|

All selected units fit262144 bytes/32 files/4000 lines. Existing broader retained-command/pack-json unit-test roots remain oversized; no broad-unit success is claimed.

Exact revised native commands are package semio-framework-pack-json --test borrowed-json-sink (2 tests), package semio-framework-plugin-retained-command --test borrowed-publication-wire (3 tests), and --test pending-original-page (2 tests). No --lib or artifact-app-testing feature is needed for these targets; the existing first-party job/testkit is a direct test dependency. Actor --lib original_result_backing_ remains unchanged. The editor's controlled driver owns execution; this agent launched no heavy job.

Pending native tests use genuine OriginalJobFixtureCaller, decoder, actual original construction/retirement ports and heap-versus-caller receipt checks. They cover the neutral payload corpus, denied write/handoff, frozen token, separately paid frontier close, exact output bytes and cancelled partial framing. Central tests exercise one-work/zero-work resumption through seven-byte slices and independent serde/ToValue oracles. Native results remain pending.

## Accepted Receipt After Cancellation

A fourth native case now uses the genuine fixed fixture caller and its same decoder to capture an Artifact7→8 receipt, cancels after zero, one or four framing turns, and finishes the accepted page through the new original retirement encoding bridge. It checks original token generation8, unchanged decoder ledger/recipient, independent serde JSON receipt contents, paid traversal/handoff and physical page closure. It adds no fixture stock or per-turn renewed control. The test file parses; native execution remains queued. Production integration owns selection of this bridge for accepted pages and retains the exact committed expected_revision rather than querying a later live Store revision.
