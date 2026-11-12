# energylife lane (11/12)
## Step 1: mut-boundary GREEN
- W-energylife-1 (wasm hub, -p semio-s-artifact-energy-model-mut-boundary) rc 0.
- Fix: derive `mutations/✨️derive/🧩️facet/🔲️boundary` hand-implemented the old ScalarStorage; now `FixedScalarMapping` for the plan + `MappedSharedScalar<Self>` (same pattern as the setpoint-manager tagged union).
Rerun: "$C/1105b-tools/chk.sh" W-energylife-N "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-energy-model-mut-boundary
## Step 2: singletons crate (W-energylife-3)
- single (SingleFieldPath/SingleFieldPreparation/TransformedSinglePreparation) ported to Bound/Held/Staged + advance_reply/quote/take(effect); compiles (no errors in 🔹️single).
- native-mutations edit (claimed): `#[macro_export]` on `std_cursor!`/`custody_impl!` (std_cursor inlines the close-entry match) so downstream crates can derive owner close surfaces.
- TODO singletons: outputs, root, network(+inverse,close), action. lifecycle crate: deletion*, construction*.
## Step 3: singletons GREEN
- W-energylife-6 (wasm hub, -p semio-s-artifact-energy-model-native-mutations-singletons) rc 0.
- Ported to the reply/effect shape on energynative's helpers: single (+transform) on Staged/Bound/Held, outputs (inverse strings now copied in the forward turn via RetainedFieldPreparation, no retained projections), root (+equality via native_child), network (+inverse; close module removed, custody chain closes), action (`NativeActionReceiving` now forwards NativeReceivingOriginal's witness/effect API: advance(control,witness,body,max,g), take(output,effect), begin_close_into/close_into...; the editor consumer `✏️editor/🎮️commands/🛬️action` must adopt it).
- Unit tests under singletons/*/🧪️tests and tests.rs still use the grant API (debt, not compiled by `check`).
## Step 4: lifecycle GREEN
- W-energylife-8 (wasm hub, -p semio-s-artifact-energy-model-native-mutations-lifecycle) rc 0.
- construction: RowFieldsPreparation (Cursor + quote/advance_reply/take_quote/take(effect)), RowBirthForm (quote + materialize like ScalarStorage::admit, admit_from), RowBirthPreparation/Pair/Fixed/Sealed/Owned fields, CreateRowPreparation, RowCreationValidation family (named via TextValidation::quote_original/advance_original_reply, fixed, rules).
- deletion: DeleteRowPreparation (row retained from lookup.take), guards (reference guard reads the list directly, no retained projection; All/Root/No), Multi (MultiDeletePath unchanged for the derives), CascadeFilter (source authority via take_authority_into), SurfaceDeletePreparation (geometry source is now `Sourced<SealedShared<Geometry>>`).
- native-mutations edits (claimed): text/validation gained row-less quote_original/advance_original_reply.
- Tests under lifecycle/*/🧪️tests and tests.rs still use the grant API (debt).
## Step 5: mut-model GREEN, hub-energy-component downstream (W-energylife-10/11/12)
- W-energylife-12 (wasm hub, -p semio-s-artifact-energy-model-mut-model -p semio-s-artifact-energy-model-mut-boundary) rc 0.
- mut-model (🔋️model/🧳️retained): IdentityChangePreparation<L,M> is now a RowFieldPreparation (Bound/Held/Staged, inverse text copied in the forward turn because the inverse protocol gets no original), and IdentityLeafPreparation = the shared native `RowLeafPreparation<L,M,IdentitySpec<L,M>,1>`; hand-written leaf (14 KB) removed; IdentityOutcome removed (RowOutcome).
- W-energylife-11 (wasm hub, -p semio-hub-energy-component) rc 101. Remaining reds, none in my crates:
  - semio-s-artifact-energy-model-editor-commands 33 errors (✏️editor/🎮️commands/🛬️action 32, 🕹️interaction 1): action producer still on old `admit_original_construction` (6 args), `begin_close`/`close_step` on NativeActionReceiving, `.progress()` on poll. Owner: unclaimed (editor lane). Needs the new `NativeActionReceiving` (singletons/🎯️action): advance(control,witness,body,max,g) -> RetainedWorkPoll, take(&mut Option<M>,effect), begin_close_into/close_into(effect,reply).
  - semio-s-artifact-energy-model-initialization 3 errors (🌱️initialization/🦀️.rs: EnergyModelSnapshot FromValue, SnapshotProjection::Output != EnergyModelSnapshot, EnergyModelMutation: OpBinaryReceiving). Owner: energynative/energysnap (EnergyNativeInitialSource + derive(SnapshotProjection) on EnergyModelSnapshot).
  - semio-s-artifact-energy-model 2 errors (StoreState / StoreMutationFor not satisfied): cascade of initialization.
- Framework-adjacent edits (all claimed): native-mutations 🛡️custody `#[macro_export]` on std_cursor!/custody_impl!; text/✅️validation row-less quote_original/advance_original_reply; derive 🧩️facet/🔲️boundary now FixedScalarMapping + MappedSharedScalar.
- Debts: unit tests under singletons/lifecycle/mut-model (tests dirs, tests.rs) still use the grant API (not compiled by check); runtime quote==actual not exercised; surface inverse and ranked scans not run.
Rerun: "$C/1105b-tools/chk.sh" W-energylife-N "🌎️hub/Cargo.toml" wasm -p semio-hub-energy-component
STATUS: FIXED-verified W-energylife-12 (mut-boundary, mut-model; singletons W-energylife-6, lifecycle W-energylife-8); hub-energy-component BLOCKED on editor-commands, initialization, energy-model (not mine)
