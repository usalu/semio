# P9p Owned Async Combinators

## Scope

The framework async crate now owns the small combinators required by runtime consumers:

- `race2` returns the winner of two same-output futures using the existing deterministic `select2` primitive.
- `yield_once` performs one cooperative executor handoff without sleeping, blocking, or creating a runtime.
- `join2` polls left then right on every turn and retains completed output until both futures resolve.

Pack and the OS product were migrated from `futures-lite` to these primitives. Their process/test entrypoints use the feature-gated repository-owned `block_on`; interactive paths do not gain a blocking bridge.

## Verification

| Gate | Result |
| --- | --- |
| `cargo test -p semio-framework-async --all-features` | 45 passed, 0 failed |
| `cargo test -p semio-framework-pack` | 66 passed, 0 failed |
| async focused poll-order/yield test | passed |
| pack cancellation, retry timer, cross-thread wake, streaming recovery tests | passed in the full 66-test suite |
| `futures-lite` source/manifest census outside `compose` | zero |
| dependency ratchet | `rust:futures-lite` removed |

Both test commands used `CARGO_INCREMENTAL=0` and ticket-local target `🧪️target-owned-async`.

## Boundary

The owned entrypoint driver remains feature-gated. It is not an executor substitute for UI callbacks or worker job steps; those continue through `WorkerPool`, retained wakers, and bounded `InteractiveJob` turns.
