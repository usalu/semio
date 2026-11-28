# 📨️ PDF Open File stalls in the job receiving retirement scope (note from the PDF ticket)

From `26/11/22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END` (full trace: `🔁️media-in-livelock.md` there). The PDF editor's Open File never completes, in the browser and natively. It lands in the receiving and escrow-scope code this ticket is reworking, so it is handed over here instead of patched concurrently.

## Reproduction (native, about 6 min warm)
`sh <pdf ticket>/native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib reaches_its_mount_through_the_real_runtime -- --nocapture`

Pass `--nocapture`, because a thread-exit abort swallows the assertion otherwise. Both laws are red as of 13:50: `..._under_the_interactive_grant`, and since today's later edits `..._under_the_shard_budget`, which completed at 09:00.

## Where it stops
1. The MediaIn custody stays in `prepare_media_descriptor`. Its `OriginalPackReceiving<MediaArtifactDescriptor>` (`🔌️plugin/👷️job/🛬️receiving/📥️pack`) reaches phase 8, `close_sources`. The receiver is not terminal and not closing, so every call goes to `retired_native` with the demand `{copy 1, depth 1}`.
2. `retired_native` (`🛬️receiving/🏗️steps`) quotes `receiving_units(fuel)`, about 1,048,576 work units. `with_native_retirement_receiving` → `original_receiving_outcome` opens the scope and runs the closure with a body grant of `{items 1, copy 81, depth 1}`. Inside it, `native.with_original_effect(demand, units, …)` answers `None` ("work quote not admitted"), forever, with no refusal.
3. Capping that quote at 4096 units (tried, then reverted) clears the stall. The publication then refuses with `original publication full receipt of Retirement is smaller than its already received effects` (receipt 20 items, received 21; later 14 against 32). The second refusal panics at `original Reactor refusal slot cannot replace an earlier original output`.

The 1-item, 81-byte body grant matches the "loaned scope topped up inside the callback (1 item, 81 bytes)" in this ticket's notes.

## Kept on the PDF side
The command ingress `resume` repeats `settle_step` within the slice (`⌨️command-ingress-turn`). The law census names the armed more-work sources (`reactor_turn_laws::last_turn_more_work_sources`).

## Update 15:30
- With the retirement quote capped (experiment, reverted), the next fault comes from the typed-operation publication pump. One `advance_typed_operation_publication` window posted 52 items but carried only a Retirement receipt of 5. Preparation work and a retirement step ran in the same window, and `collect_original_receipts` binds one family per window. Escrow scope claims inside those windows were measured at zero.
- Both items remain open for their owners: (1) the size of the work quote in `retired_native`, which the pooled retirement scope declines; (2) publication windows that mix receipt families.

## Precise location of the receipt mismatch (15:35)
The pump (`advance_typed_operation_publication_turn`) runs one unit per window while operations exist, so the mismatched window was a single **retirement unit**. `retire_typed_operation_run` (`🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs`) posted 52 items, but `original_retirement_receipt` records only the `spent` of its granted-retirement loop (5 items). The postings of the same unit outside that loop are not in the receipt. That function was edited at 14:47 by the Puzzle ticket's agent (its `L3D_TRACE` "granted retire spent" probe sits exactly at the receipt), so it is left to them.
