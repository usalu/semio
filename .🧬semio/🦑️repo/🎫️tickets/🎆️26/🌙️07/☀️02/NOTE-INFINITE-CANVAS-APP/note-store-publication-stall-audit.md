# Note Store publication stall trace

Browser attempt 42 records the first Add Text operation, 192, in Publishing with a completion, an emission and a pending artifact owner. The progress witness stays at 694 across 4,096 idle observations and yields `interactive-job.publication-stalled`. A second add, operation 256, pauses at 578 for at least 2,048 observations and subsequently advances to 738 before retirement. The recorded flags alone leave the pending Store phase and receipt state unknown.

## Returned snapshot reads

The first batch item leases the current Store snapshot. Each later item leases the preceding staged post root through the same Store registry. A returned lease remains in the registry until one exact retirement admission moves its root into a disposer. Live maintenance stage 8 drives this admission, rotating between document, config, interaction and transient returns. The disposer closes after it releases or retires its Arc custody. A shared root's Arc election relinquishes the alias in one step, even when a different owner retains the live root.

The mounted artifact path returns early when the document registry has more than one returned read. Its additional reclaim predicate requires zero returned reads, making the pressure branch equivalent to an early return based on the count. That branch runs before the Store batch step and can produce the recorded stagnant witness whenever live maintenance is slow to lower the count. It applies to the whole pending artifact owner, including batch phases that issue no further read. Live maintenance has no special priority for returned reads. The available attempt 42 trace lacks the registry counts needed to establish whether this branch ran.

## Batch and receipt flow

The Store resumes its one-item owner through source census, preparation, fold, item retirement, cursor validation and commit. Direct Store step denial returns `Blocked`; initial snapshot authority or registry saturation instead returns a domain error. Store checkpoints account for semantic work and omit individual custody transitions during the fold, commit and retirement. Current mounted code additionally counts nonzero paid receipts, so an unchanged checkpoint during a paid unit should remain visible in that build.

On successful commit the Store immediately records its receipt and enters AwaitingAck. The mounted operation can still show Publishing until it reserves and delivers the artifact result page. Its publisher checks the accepted Store receipt before checking the returned-read count. Store phase, accepted receipt, returned and outstanding read counts, and original turn grant together distinguish whether the next reproduction is before Store entry, inside the batch, or after commit.

The unquoted receiving scope can also defer entry while it reserves its original stock and Work. The callback's grant derives from that scope. The mounted Store route currently passes its outer body grant into Store preparation and ignores the callback grant. Individual Store operations that admit construction must still use the receiving control's actual stock. The attempt 42 trace has no receiving-scope observation to tie this discrepancy to the current pauses.

No Store source changed during this trace.

## Attempt 43 and correction

Attempt 43 places Add Text operation 192 in mounted Publishing while the pending artifact Store is already Closing. The Store reports no accepted receipt at that phase, as its receipt accessor exposes only AwaitingAck. Both snapshot registry counts are zero at every logged observation, and the publishing caller still has items, bytes, depth and fuel. The returned-read gate therefore had no role in this failure.

The Store moves from AwaitingAck to Closing when the host acknowledges the artifact result, and the mounted operation returns to Publishing. After the publisher disposes the producer's acknowledged result page, it meets the Closing Store owner. The previous publisher returned an idle result there and depended on a separate live-maintenance caller. That caller handles the same owner at GrantedMounted, but the reactor does not treat this pending owner as maintenance pressure. Repeated typed publication turns could make no progress until live maintenance got enough turns to dispose the owner.

The mounted publisher now runs the existing funded retirement step after result-page custody and before any additional Store publication. It reuses the current caller's grant and adopts the step's exact ownership receipt. The independent maintenance path continues to support teardown. The Rust fixture drives a Closing document Store with only publication turns, checks that an unfunded turn preserves custody, and requires paid release before the operation can continue. The Bun SQL and Tree-sitter oracle verifies the fixture and routing order. The browser rerun and the Rust fixture execution are still pending.
