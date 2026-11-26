# LDLT p6h hang: root cause

The hang was never in the production path, and `run_turn` is not at fault.

- The "final checkpoint loop" on `zero_fuel` does complete: instrumented, it reached `Seen::Checkpoint` at turn 457 (column 1, entries 1, floor 1), at roughly 11 us per turn. Fuel is charged, and reservation lanes 0..5 finish by turn ~60.
- The real hang is a few lines later, in the test-local `fresh_checkpoint` helper. It builds `LdltJob::new(..)` for a 6x6 matrix and loops `until Seen::Checkpoint`. After the cadence amortization the default floor is `LDLT_CHECKPOINT_MINIMUM_ENTRIES = 4096` entries, so such a small factor never checkpoints. The job completes, keeps yielding and the loop spins forever.
- Fix: the helper builds its source with `.with_checkpoint_floor(1)`, as the other checkpoint-expecting jobs in the test already do.
- Pcg and Subspace use `run_turn` too, but only the LDLT test depended on a checkpoint cadence, so they cannot spin the same way.

Files changed: `🔺️ldlt/🧪️tests/🔬️unit/🦀️.rs` (helper fix, all `[DEBUG]` markers and the 100k bound removed).

Laws: ldlt 11 passed, pcg 27 passed, subspace 17 passed.
