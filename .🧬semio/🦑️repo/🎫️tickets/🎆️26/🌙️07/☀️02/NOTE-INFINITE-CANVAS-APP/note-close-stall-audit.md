# Captured Math Import Close Stall

The Stage53 capture reports instance 1 close zero-progress at epoch 59309 and 56.138 seconds. Its WIT receipt has zero retained spend and does not identify the close stage or prove the elapsed stall credit. The fault message elapsed field is 0us.

Current cleanup-turns/🦀️.rs lines 155–182 and 282–295 count only uncontended Progress with an empty receipt and classify explicit Blocked and AwaitingInput as ExternalWait. Its current source mtime (16:47 UTC) is later than the captured Stage53 core (16:16 UTC), so the current two-currency elapsed-and-count verdict cannot be assumed present in that captured binary.

Cleanup-pumps/🦀️.rs lines 321–330, 344–359, 371–375 and 381–403 can each provide progress or wait paths. Inspect job.progress and contended at the stall verdict and identify which advance_source return produced them. Existing runtime close pending authority trace logs mounted-owner Blocked/AwaitingInput reasons on a doubling cadence; close phase timings and last-fault buffers are test or cleanup-probes gated. No captured evidence assigns this stall to a particular owner or external wait.
