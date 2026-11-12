# energysession lane (11/12)
## Step 1: semio-s-artifact-energy-model-session GREEN
- W-energysession-1 (hub wasm, -p semio-s-artifact-energy-model-session) rc 0 (52 -> 0 errors).
- Root: `Construction` (resident OriginalConstructionAdmission + born marker; `run_into`/`run_flag` over admit_original_construction_settled, work 1, close via retire_step), `effect_move`, `settled_step` helpers.
- retained-input: ModelCapture owns a Construction; step_original settled.
- publication/prepared: ScalarRankCursor advance/close_step and PagedList release_empty_page run on `effect.nested(child grant)`; local moves charged with `effect.performed(1)`; poll advance via `cx.with_original_effect`; publication close_step/close_admitted effect-shaped (builder close_step(effect), source `ControlledRetirement::step(usize::MAX, effect)` polled by settled_step).
- weather: receive (grant, native), receive_effect (frame, decoder.step(effect)), receive_progress (advance_nested), effected (Local advance_local / take_into / close_step), advance_binding on OwnedAdvanceSelection; close via energy_effect_step; decoder closes via close_poll (no native retirement control).
- run-job: census/capture/admit stages on Construction; schedules/capture/template/writer close via ControlledRetirement::step(body, effect); snapshot close = `ControlledRetirement<ArcLease<EnergyModelSnapshot>>` (ArcLeaseRetirement) replacing SharedControlledRetirement::lease; all close steps now energy_effect_step (work charged by the calling turn's effect).
## Debts
- A construction that ends Failed keeps its causes in the Construction admission (no take_failure path): the job close cannot retire it; same behaviour as flowext Construction.
- Tests (🧪️tests, weather 🧪️.rs, publication tests) not compiled / not ported.
Rerun: "$C/1105b-tools/chk.sh" W-energysession-2 "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-energy-model-session
## Step 2: -p semio-hub-energy-component (W-energysession-2, wasm hub) rc 101
- 0 errors in semio-s-artifact-energy-model-session.
- Remaining reds by owner: semio-s-artifact-energy-model-native-mutations 323 errors (energynative); semio-s-artifact-stdio-xlsx-transitional mutations/🦀️.rs:83 `XlsxTransitionalMutation: OpBinary` x2 (stdio7). 🌱️initialization/🌱️state is hidden behind native-mutations (not yet reached).
Rerun: "$C/1105b-tools/chk.sh" W-energysession-3 "🌎️hub/Cargo.toml" wasm -p semio-hub-energy-component
STATUS: FIXED-verified W-energysession-1
