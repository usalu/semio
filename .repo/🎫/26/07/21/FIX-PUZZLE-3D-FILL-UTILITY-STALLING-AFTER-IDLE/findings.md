# Findings

## Symptom

Puzzle 3D Fill utility starts planning, then after ~15s of waiting the utility stops responding.

## Root cause

Two compounding bugs:

1. **Host queue flood** (`framework/renderer/react/index.tsx` World3dHost): `setInterval(..., 120)` fired `fillBuildTick` on every beat without waiting for the previous tick's serialized plugin `handleAction` + `refreshUi` to finish. After ~15s the WASM action queue was deep enough that user actions (slider / utility switches) appeared dead.

2. **Full UI refresh every tick** (`puzzle/plugin/rs/lib.rs`): `fillBuildTick` defaulted to `UiDirtyScope::Full` and was incorrectly declared as an `Operation`. Every 120ms tick re-fetched the entire shell UI.

## Fix

- Host: `createInFlightSkippingInterval` — drop overlapping ticks while one is in flight.
- Plugin: demote `fillBuildTick` to `View`, emit narrow `Partial` scope (main window body + measures).
