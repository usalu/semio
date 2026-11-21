# Atomic Jobserver Reservations

Play generation `wX0lBAR` received four compiler slots. During six minutes leading to the second descriptor retry, 180 two-second samples showed four held slots while one job ran and ready stages waited. They recorded one rustc in 159 samples and zero in 21. The memory broker reported a freeze in 46 samples. These observations motivated investigation of slot accounting, and do not establish an overall speedup.

The jobserver previously let a whole-pool request at the head of the JavaScript queue hold every available slot while the active descriptor job retained the last one. Its external Cargo clients could no longer borrow the three published tokens. The new reservation path first checks whether existing JavaScript holders leave enough capacity. A probe that encounters an unavailable native token returns every tentative slot before yielding. The queue keeps its order. Opportunistic requests use the same reservation path, and client retirement retains its physical-token reconciliation.

The new fixture first failed with an independent Python borrower reporting `unavailable`. It passed after the fix and now checks physical borrowing while an exclusive waiter is pending, exact whole-pool admission, successor ordering, and cancellation. A second case leaves one native token outstanding and verifies that the failed probe leaves the remaining token available to a borrower, then that client retirement restores the full grant. Seven focused Bun tests passed on macOS. No Cargo build, Rust check, descriptor generation, or restart of the live release was performed.

## Driver restart

After the third confirmed descriptor source edit in generation `wX0lBAR`, the fourth descriptor attempt had just entered Cargo and no component had staged. Closed only the ticket's site driver with SIGTERM so the same supervisor could activate the tested scheduler fix on its next site attempt. The driver reported released leases. The descriptor child and Cargo descendants exited, and supervisor 8173 began catalog generation at 00:39 CET. No competing release was started. The replacement's actual compile throughput and full release outcome remain pending.

## Live reservation verification

After the intervening generation3d source-boundary failure, supervisor 65168 passed full preflight and opened `wSkckFq` with a four-slot grant. During initial jobs, telemetry showed three held with three active, then two with two active. With just the descriptor active and seven ready jobs it showed one held. The old driver recorded four held under those same queue counts. This verifies that the live scheduler leaves remaining slots available while the exclusive request waits. Whole-release performance and browser verification remain pending.

Four Rust compiler processes subsequently ran in parallel during the single active descriptor stage. The generation advanced from 142 to 266 completed units across the sampled minute. This is live evidence of access to all four granted slots. The comparison uses different source revisions and does not establish an end-to-end speedup.
