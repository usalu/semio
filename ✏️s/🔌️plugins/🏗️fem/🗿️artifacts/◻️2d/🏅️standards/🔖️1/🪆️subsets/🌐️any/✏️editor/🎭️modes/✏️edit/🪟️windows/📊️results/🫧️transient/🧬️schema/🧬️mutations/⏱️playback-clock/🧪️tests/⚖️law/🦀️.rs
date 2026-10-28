//! ⚖️ `set-playback-clock` — the inverse diffs sum to the negative of the forward diff (L3).

use super::*;

/// ⚖️ Publishing, advancing and clearing the clock each restate the clock slot and the inverse carries the base clock back.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let running = FemResultsWindowTransient { clock: Some(FemPlaybackClock { phase: 0.75, reverse: true }) };
    let stopped = FemResultsWindowTransient { clock: None };
    for (base, clock) in [
        (stopped.clone(), Some(FemPlaybackClock { phase: 0.25, reverse: false })),
        (running.clone(), Some(FemPlaybackClock { phase: 0.5, reverse: false })),
        (running.clone(), None),
        (running.clone(), running.clock),
        (stopped, None),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&FemResultsWindowTransientMutation::from(SetPlaybackClock { clock }), &base).await;
    }
}
