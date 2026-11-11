# Lane matcher: record matcher KeyAcknowledgement zero step

## Diagnosis (W-matcherack-5, probe)
Fem 3d `original_job_receives_typed_record_into_caller_slot`: `[DEBUG] ack ready work=0 next=KeySettlement actual=829`. The acknowledgement effect had already reported its unit of work in an earlier `Pending{work}` poll (`claim_child_work` returns the unreported delta); the finalizing poll is `Ready{work:0}`, mapped to `idle(0)` = zero progress, zero work, which the snapshot cursor loop refuses.

## Fix
`record/🔎️schema/🎛️drive.rs`: new `advance_acknowledgement` settles the acknowledgement inside one `advance_nested` call: it keeps polling while the effect reports `Pending{work>0}` (summing the work), returns `Pending{total}` on `Pending{0}` (state kept), and returns `idle(total + ready_work)` on `Ready`. The finalizing step now carries the work performed by the same call.

## Law
`record/🔎️schema/🎟️work/🧪️tests/🦀️.rs` (`record_matcher_driver`): nested turns run under an exact-quote grant (items 1, copy/capacity/release/depth = `demands()`); an admitted turn that starts at KeyAcknowledgement must report work>0 or progress.

## Open / status
Verification runs in progress (see below).
