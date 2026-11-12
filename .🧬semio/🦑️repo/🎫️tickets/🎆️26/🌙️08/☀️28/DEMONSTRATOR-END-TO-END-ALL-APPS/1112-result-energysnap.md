# energysnap lane (11/12)
## State: semio-s-artifact-energy-model-snapshot, 374 -> 97 errors after macro fix (W-energysnap-1 rc101)
- Root cause 1 (235 errors): energy `state_patch!` macro (state/diff) never emitted `protocol::retained_empty_option_patch!`; `retained_row_patch!` requires `RetainedPatchEmpty` from the caller (framework macro is correct; fem calls the empty macro explicitly). Fixed in the energy macro + EnergyModelStatePatch.
- original/mod: hand `ImmutableRootProjection` impl (old `project_root`, u128 work adds) replaced by `#[derive(SnapshotProjection)]` on EnergyModelSnapshot (EnergyModelState already derived it).
- original/genesis: ported to effect shape (advance/close_step take `&mut RetainedAllocationEffect`, `SealedShared::admit_from`, close ladder on OriginalRetirementPoll, `work_demand()` added). Consumer `🌱️initialization/🌱️state` NOT yet adapted.
- Remaining (W-energysnap-1): state/fields (RetainedSealedField), fields/branch, fields/key, state/diff/apply (StateZoneApply), state/diff/branches: old grant-shaped hand protocol vs reply/effect-shaped.
## Step 2: W-energysnap-2 rc101: 97 -> 37 errors, all in state/fields/{🦦,branch,key} (RetainedSealedField, RetainedBranchProjection, RetainedEntityLookup)
- diff/branches: replaced the hand PreparedEnergyBranchesApply by `type PreparedEnergyBranchesApply=RetainedPatchDiffApply<EnergyModelState,EnergyModelStatePatch>` (framework generic is already effect-shaped).
- diff/apply: StateZoneApply now wraps RetainedPatchDiffApply and keeps only the one-unit identity/physics validation phase (preflight_reply with performed(1), refusal = Ready(_,false)).
- Genesis, original, diff, apply, branches: 0 errors in W-energysnap-2.
## Step 3 (10:10): ported fields trio to effect/reply protocol; source budget (262144 B / crate, refusal in W-energysnap-3/4) fixed without raising limits
- fields: RetainedSealedField / RetainedBranchProjection / RetainedEntityLookup rewritten reply-shaped (advance_reply/close_reply + four close entries via `close_entries!`, retained_routed!, take(effect)); consumers in the mutations crates (RowFieldsPreparation etc.) still on the old grant-shaped API.
- budget cuts: derive aliases + `records!` macro for the 48 state records (state/mod 30.7 -> 19.0 KB), genesis `seal!` macro (15.6 -> 12.2 KB), close entries macro.
## Step 4 (10:40): semio-s-artifact-energy-model-snapshot GREEN
- W-energysnap-7 (wasm hub, -p semio-s-artifact-energy-model-snapshot) rc 0. (W-energysnap-6 failed only on cadmodel's transient mesh-engine E0119, since fixed.)
- Framework edit: `semio-framework-replication` lib re-exports `retained_routed` next to `retained_row_patch` (claimed in claims log).
- Not done: native check, snapshot unit tests (state/fields/tests, key tests, original/genesis tests use the old grant API: need port to reply API), downstream energy crates (model-mutations: ~230 KB of RowFieldsPreparation / native-singletons / lifecycle code on the old grant-shaped API using RetainedSealedField/RetainedBranchProjection/RetainedEntityLookup), 🌱️initialization/🌱️state (EnergyNativeInitialSource calls EnergyModelInitialBirth::advance(body,grant)/close_step(grant); now effect-shaped).
Rerun: "$C/1105b-tools/chk.sh" W-energysnap-8 "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-energy-model-snapshot
STATUS: FIXED-verified W-energysnap-7 (snapshot crate only)
## Step 5 (11:20): diff + inferences GREEN; hub-energy-component map
- W-energysnap-10 (wasm hub, -p semio-s-artifact-energy-model-diff -p semio-s-artifact-energy-model-inferences) rc 0.
  - diff: EnergyDocumentApply ported (composite preflight_reply/commit(effect)/close_reply over PreparedEnergyBranchesApply + RetainedRowPatchApply; RetainedPatchEmpty for EnergyModelDiff, work 10).
  - inferences: `Default` was stale (cfg(test)-only `crate::model::Model`) -> zero census. Its unit tests (`from_model`, `EnergyModelSnapshot::empty()`) are stale and need a port (debt).
  - Framework edits (claimed): replication lib re-exports `retained_routed`; os-kernel facade re-exports RetainedDone/RetainedFlight/RetainedReply/retained_settle/RetainedPatchEmpty/retained_routed.
- W-energysnap-11 (wasm hub, -p semio-hub-energy-component) rc 101. Energy reds left:
  - semio-s-artifact-energy-model-native-mutations 281 errors (🧬️schema/🧬️mutations/🧳️native: value 42+9+8+5, leaf 35, membership 32, scalar 25+17+6, adjacency 21, name 17, reference 15, row 14+13+8+6+3, text validation 5). The RowFieldsPreparation family (grant-shaped) over RetainedSealedField/RetainedBranchProjection/RetainedEntityLookup, whose reply-shaped API now exists (`advance_reply`, `close_into`..., `take(effect)`, `exchange(.., effect)`); the ~230 KB of consumers must be re-shaped on them (and also hit `OpBinaryReceiving` / owner_slot rules).
  - semio-s-artifact-energy-model-session 52: simulation-session run-job (22: admit_original_construction now takes an OriginalConstructionAdmission and a status; SharedControlledRetirement.step/lease gone), weather (22: EPW owned receiving `.step`, RetirementStep.result, `map_err` shape), publication/prepared (5), publication (2: `close_step_granted`), retained-input (1).
  - Not energy (other lanes): stdio-xlsx-base-model 17, stdio-semio-image-editor 3 (W-energysnap-8).
- Also needed after native-mutations: 🌱️initialization/🌱️state EnergyNativeInitialSource must call EnergyModelInitialBirth::advance(body,effect)/close_step(effect) under `cx.with_original_effect` (pattern: plugin-ownership genesis `cursor.advance` stash) and use `work_demand()`.
Rerun: "$C/1105b-tools/chk.sh" W-energysnap-12 "🌎️hub/Cargo.toml" wasm -p semio-hub-energy-component
STATUS: FIXED-verified W-energysnap-10 (snapshot W-energysnap-7 + diff + inferences); semio-hub-energy-component still BLOCKED on native-mutations (281) and session (52)
