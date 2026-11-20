# Boot failure: "original Energy genesis phase or source changed"

## Cause
`EnergyNativeInitialSource::advance` (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/.../🌱️initialization/🌱️state/🦀️.rs`) performs one `EnergyModelInitialBirth::advance` (phase += 1, cursor.body = Some(grant)) and stashed the step, then settled the construction with `birth.built(...)`. It did `step_stash.take()` into a local BEFORE `built`, and recorded its own `body` only inside the construct closure. `admit_original_construction_settled` returns `Ok(false)` without running the closure when the turn must yield (`retirement_should_yield`, fuel below floor, quote not fitting the grant) and may settle across turns. Then:
- the performed step (and its pre-advance demand quote) was lost, the cursor was already one phase ahead;
- source `body` stayed `None`, so the next turn took the new turn's `maximum_copy_bytes` as body, differing from the cursor's frozen body -> `EnergyModelInitialBirth::demand` refused with `invalid()` ("phase or source changed").
Also the `cursor.ready()` branch ran before the stash was drained, and the quote `d` was recomputed from an already-advanced cursor (the last phase would error with phase >= 20).

## Fix
Same file: a `StagedStep{result,demand,metadata}` replaces the bare stash. The source `body` is frozen when the step is staged (as the cursor does), the quote and body metadata are stored with the step, the stash is taken only inside the construct closure (runs once, when settlement actually happens), and the take branch only runs with an empty stash. Retry turns reuse the staged quote and skip the cursor.

## Verification
By reasoning only (no builds). Existing cursor law tests (`🧬️original/🌱️genesis/🧪️tests`) cover the cursor unchanged. No source-level Rust test added: exercising a yield mid-settlement needs a mounted StepContext fixture with a tight fuel ceiling, not available cheaply here.

## Rebuild
Component `energy` hub (`🔋️energy`) only, plus any component embedding `semio-s-artifact-energy-model-snapshot` initialization state (check `repo components` consumers; e.g. an `energy` guest/playbook module if it exists). No browser bundle change.
