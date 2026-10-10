# Original Issued Shard Batch Ledger

Host retry 6 reached the specific E0599 declaration failure for `IssuedShardTurn::new` at full log line 25028. The earlier search missed the associated-method diagnostic; the original report is corrected explicitly.

The new non-Copy, non-Clone ledger owns the exact original input and pure scheduling budget, tracks cumulative accepted receipts, loans one remaining grant at the declared envelope index, validates exact receipt identity/epoch/currencies before mutation, and returns only once after the last declared envelope. Forged, duplicate, unissued, and out-of-order completion preserve the active checkout. The borrowing retained recipient forwards accepted operation spans into that same issued ledger and validates the final return.

Actual queued authority and execution callers still use the old copied budget architecture and require reconciliation to mandatory envelope descriptors and this original recipient. This source is not declared ready or accepted. Exact registered Host retry 7 is live (PID 74360, session 66560), with unchanged limits and roster; log `🗑️generated/host/original-issued-batch-oct10-red7.log`.

The concrete recipient now implements Services `ComputeRetainedRecipient` directly. Active accepted deltas live in the original ledger, so dropping a borrowing async driver cannot lose those receipts; remaining grant is computed from that same active span. Native caller/queued-envelope migration remains open. Rustfmt parsed the new ledger/recipient source successfully; this is parse evidence only.
