# Tool-Machine Gesture Issuer and Scrub-Host Test Fixes

## Result
- `semio-plugin-tool-machine`: 2 passed, 0 failed (`gesture_issuer` tests).
- `semio-plugin-scrub-host`: 3 passed, 0 failed.

## Root causes (all fixture-side; no production code changed)

### gesture_issuer
1. The earlier diagnosis was incomplete. `release_birth` did lack a producer invocation, but the failing assertion was in `drain`, which stepped `ControlledRetirement` under a bare `EffectBank::run`. Controlled/sealed/derived retirement cursors require `effect.invocation()`.
   Fix: both `drain` and `release_birth` now run under `effect::with_effect`, the shared fixture that carries a genuine original producer invocation. It is included with `#[path]` from `value/retirement/owner/work/effect.rs`, like the other crates' tests. `release_birth` now funds the close with the birth's own `next_close_demand` and `next_close_work_demand`. The old `funded()` grant had `maximum_release_bytes: 0`, so the close could never progress.
2. Cancellation test: `request_cancellation` and `slot.state.lock()` lazily box a 64-byte `pthread_mutex_t` on macOS. It is freed during retirement when the mutex is dropped, so a step released 64 bytes it did not report. The strict per-step heap equality failed on macOS only.
   Fix: the test measures that lazily allocated backing with a probe lock (0 on platforms with futex or unfair-lock mutexes). `drain_unreported` credits only that measured amount against step releases, and `unreported == 0` is asserted at the end.
3. Neutral-admission test: the final accounting hid a missing term (`held=0` and `*0+0`). It now measures the slot's own strings (37 bytes) when it creates the slot, and asserts `held + reserve + publish + drained births == releases`.

### scrub-host
The fixture's `turn_with_decoder` bills the decoder's origin-refresh service (9 items / 6064 bytes steady state, 15 / 7362 on a fresh decoder) in every turn. The tests still expected `Default::default()` for denied business turns.
- `service()` now runs one warm-up turn and returns the steady-state receipt.
- Denied `advance` turns and the first denied registry admission compare with the `service` baseline of their own grant, so the property is still "a denied business step adds nothing to the service bill".

## Changed files
- `🛠️tool-machine/🖐️gesture/🎟️issuer/🧪️tests/🦀️.rs`
- `🛠️tool-machine/🫧️host/🧪️tests/🦀️.rs`
- `$T/gesture-cargo.sh` (script for these runs)
