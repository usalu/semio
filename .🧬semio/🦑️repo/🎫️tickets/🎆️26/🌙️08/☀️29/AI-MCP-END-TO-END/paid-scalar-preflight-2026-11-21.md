# Paid Scalar Preflight

The eligible reuse is the live Source remaining-stock read between the first and second deadline observations. The scalar loop currently uses the work balance from that read, then the final ordinary charge reads the same Source for retained and work balances. An instruction that reaches its final charge without an intervening mutation can use the first read in both places.

All three deadline observations have distinct consequences. At loop entry, the cancellation gate precedes the first deadline observation. The first expired, unavailable or regressing clock returns `Deadline`, even when fuel is already exhausted. The next observation is conditional on positive effective fuel and returns `Pending` before decoding. The final ordinary charge observes the clock after decoding and before retained posting. Its deadline refusal also returns `Pending`. Core startup and call runtimes propagate `Deadline` separately from `Pending`.

| Observation | Required behavior |
| --- | --- |
| First deadline | Short-circuits before the fuel lookup. An expired or missing clock gives `Deadline`. |
| Fuel and second deadline | Zero effective fuel gives `Pending` without observing the clock again. A failed second observation gives `Pending` before a malformed or trapping instruction can produce an error. |
| Final charge deadline | A failed observation prevents posting and PC movement. A decode error that occurs before this observation remains the result. The ordinary final charge still checks the deadline during authorized cleanup. |

`ClockStride::read` consumes a countdown even when it returns the cached time. Omitting a cached read changes which later instruction samples the clock. For example, with two cached reads left at time 9 and a deadline of 10, the first two reads pass. The third must call the clock and sees time 10 or an unavailable clock before charge. The first and second observations can likewise straddle a sample, and the second fails before decoding. The implementation needs each logical read at its existing point. Collapsing already-cached reads into one state update would be a separate optimization requiring proof of the same conditional countdown effects.

## Source authority

The return budget is held through an owning gate or an exclusive mutable loan. Between the top fuel check and a final charge reached through immutable planning, instruction decoding and trace marking change no Source stock, local fuel, claims, observer receipts, or policy. Trace marking reads retained progress and the supplied fuel count. There is no asynchronous suspension.

The smallest candidate is a one-use `StepContext` preflight holding the exclusive mutable context and the optional visible Source remaining tuple. Its entry keeps the existing first deadline, net fuel and second deadline checks in order. The interpreter keeps cancellation at the existing loop entry. The preflight exposes shared context access for immutable planning and trace marking. The scalar charge consumes it and observes the third clock at the present charge site, then uses the existing retained-post and work-debit routines. A path requiring a mutable context consumes the preflight and discards its observation before taking the existing route.

The eligible path is the final fixed-receipt scalar charge for `Nop`, `Drop`, `Local`, `Global`, `Numeric` and a `Push` with spare stack capacity. The memory-size opcode joins this path when the stack has room. Growth first posts its own capacity charge and consumes fuel, so its later scalar charge needs the new stock and its existing clock observation. Existing memory, frame, control, and import helpers also retain their own admission when they receive the mutable context. A copyable budget or stamp detached from the context would allow reuse after those mutations. The borrowed preflight prevents that.

The reused observation only gates admission. It still distinguishes a visible Source with exhausted or refused stock from the externally invisible birth state. The retained and work posts use the resident Source itself, including any pending birth state it may see internally. No cancellation observation, authorization, logical clock read or actual ledger posting is removed.

## Evidence and measurement

A lightweight Bun state model compared 2,592 combinations of cancellation, cleanup, work stock, three independently chosen clock readings, a decode error, retained shortage and intervening paid growth. Reusing only unchanged Source stock produced identical status, clock calls, decode and post order. This exercises the boundary logic. The Rust implementation and inlined cost of a preflight still need a real Source witness.

The existing native benchmark measured the preceding change from three admission reads to one within each charge. This candidate removes one additional read across the outer preflight and charge on eligible scalar paths. Its cost depends on the preflight transfer and how often growth or another helper consumes the context before charge. The existing 32.4% local median improvement does not estimate this follow-up. The smallest useful benchmark compares the current loop protocol against the guarded variant in the same pinned native fixture, with a genuine bound Source and active holds. It should record accepted and refused verdicts, exact receipts, final VM state, and clock samples for a no-growth scalar stream and a stream with growth and control fallbacks. No broad build or production edit was performed for this audit.
