# Receipt Guard Frontier Audit

## Observed facts

- `🗑️generated/native-note-source-frontier-sdk.log:383–385` reports an unframed Input `WorkLimit` and only `receipt output reservation`; it does not identify the substep or establish that the table was full. The later close grant at line 386 is a different phase and cannot establish the failed phase's remaining grant.
- Receipt `prepare_step` can return false before the guard: initial home `pay`; yield/cancellation; four `prepare_place` calls (admit, preissue, or their local grant/fuel preflight); and exposure preparation. Exposure deliberately returns false after owner metadata initialization and after an admission attempt even when that attempt succeeded; its metadata and preissue can also yield.
- The guard can return false after the new row-reclamation call: yield/cancellation, unadmitted retained recipient, insufficient current items/copy/depth/fuel, or `table.reserve_guard` finding no zero state. Its snapshot accounting and `plan_guard` failures return errors rather than false. A prior permit instead qualifies and returns true or errors.
- `reserve_original_return_rows` returns true with no table, although this guard subsequently requires a table; in normal guard flow the publisher admission has already occurred. It returns false for no reclaimable row, stop condition, insufficient reclaim copy/fuel, or `reclaim_one` false; the wrapper now labels all of these `header_guard_row_unavailable`.
- `last_claim_reason` is thread-local, initialized empty and set by `claim_why`; successful claims and unrelated false exits do not clear it. Thus an empty detail does not isolate the guard, and a nonempty detail may be stale. The `for 0..items` loop can also exhaust its bound with no yield, then return `None` if `prepared != 15` or guard absent. Receipt preparation has progress-making false returns, so the final bit mask and exposure state matter.

## Fast discriminator if retry fails

Temporarily record at each false return its exact substep, `prepared` bit mask, exposure admission/preissue state, guard presence, remaining items/copy/depth/fuel, and yield/cancel/recipient flags; for row reservation also record free rows, reclaimable status, and reclaim copy demand. Clear the claim-reason slot immediately before each `prepare_step` (or report a direct local reason) so a prior claim cannot masquerade as the current exit. Distinguish loop exhaustion from yield. Capture the values at the failed turn, not from subsequent cleanup. This is an observation proposal, not a diagnosis of the trace.

## Candidate regression law

With a real Source and core-wire publisher, fill the escrow table with acknowledged settled rows, then prepare a fresh receipt under sufficient current grant and fuel. Repeated preparation must reclaim paid rows and eventually produce all four preissued places, exposure and one guard; account for reclaim work and copy, and verify the guard reserves a genuinely free row. Separately constrain current grant below a specific substep demand and assert a resumable false with no unintended state mutation; replenish it and assert completion. Include both no-reclaimable and loop-bound cases. The current trace alone does not prove which case occurred.
