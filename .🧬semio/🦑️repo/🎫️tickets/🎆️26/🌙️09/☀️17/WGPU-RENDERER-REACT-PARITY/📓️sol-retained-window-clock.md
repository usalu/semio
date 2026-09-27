# Retained Window Clock

## Contract

The retained engine now accepts one process-monotonic `f64` timestamp for one exact window through `Ui::advance_window_clock`. Its `UiWindowClockStep` reports the surface token, tooltip transition, whether accepted interaction changed, and the next armed deadline. `Ui::window_next_clock_deadline` answers the same token plus deadline after input dispatch, while `Ui::clock_surface_at` exposes a stable fixed-slot cursor for bounded introspection.

The clock and every interaction deadline use `f64`: tooltip dwell/dismiss, NumberStepper hold repeat, Select typeahead timestamps, and Slider readout click timestamps. Paint receives a separate `f32` conversion; presentation timing never depends on that reduced value.

## Candidate and visibility rules

A monotonic clock update with no due work does not advance `presented_interaction_epoch`, so an already sealed candidate remains valid. A NumberStepper repeat or tooltip transition changes interaction exactly once, rebases the candidate, and invalidates a witness sealed against the prior epoch.

`seal_presented_input_candidate` is the accepted visibility boundary. It clears a hidden surface's repeat owner in both retained arenas. Reopening that surface cannot apply elapsed repeat ticks from a press that stopped being visible. Every clock result carries `UiSurfaceToken`, so a retired slot and its same-id successor cannot share a scheduled deadline identity.

## Neutral and independent oracles

The existing schema-first NumberStepper and Slider editing fixtures now include `clockOriginSeconds: 31536000.123456`. At that one-year origin an `f32` clock cannot distinguish the 100 ms repeat/double-click intervals, while the `f64` implementation can. The Stepper fixture owns 500 ms delay and 100 ms repeat cadence. The Slider fixture owns a 100 ms accepted second click and 500 ms expiry.

The actual React Stepper and Slider tests, plus both Ajv 2020 schema laws, pass:

```text
Test Files  1 passed (1)
Tests       4 passed | 10 skipped (14)
Duration    7.74s
```

## Native laws

The native laws awaiting the root-owned grouped run are:

- `number_stepper_editing_matches_the_neutral_react_contract`
- `slider_readout_editing_matches_the_neutral_react_contract`
- `window_clock_keeps_a_noop_candidate_sealed_and_invalidates_it_once_when_hold_repeat_changes_state`

The engine law proves stable-slot/token lookup, exact next deadline, no candidate invalidation before the deadline, one invalidation when repeat state changes, and repeat retirement on accepted visibility loss. Rust sources were formatted through the Nx-routed formatter. No native pass is claimed until the grouped run completes.
