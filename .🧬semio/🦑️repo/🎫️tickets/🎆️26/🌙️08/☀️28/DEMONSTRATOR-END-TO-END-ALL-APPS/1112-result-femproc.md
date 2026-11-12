# femproc (11/12)
## Done (edits)
- fem2d results-window config: DslReceiving derives + original_ordinal_operation_receiving! (W-femproc-3 rc0: fem-2d-results-state).
- app-surface ResultMode/FemLoopMode/FemWaveform/FemResultsAnimation: DslReceiving derive.
- fem2d/fem3d `Fem*Mutation`: dsl::original_pack_value_operation_receiving!; fem3d model-window + results-window config: same recipe as 2d; Fem3dGumballConfig DslReceiving.
- fem3d visual-job staged-step: RetainedWorkOutcome::<_, ValueError> turbofish.
- process3d host: snapshot/mutation decode ported to 3-arg close_step / reserve_retained_allocation via cx.with_original_effect, artifact_retirement_box_close_step(.., grant.maximum_copy_bytes, effect); Process3dMutation original_pack_value_operation_receiving!.
- W-femproc-7 (run during a reverted StoreState experiment): process3d-host, process3d-viewer, fem-3d-editor-visual-job show NO errors.
## Open
- fem-2d-viewer (and by symmetry fem-3d-viewer/editors): `Fem2dSnapshot: Clone` unsatisfied (StoreState = Clone+..). Snapshot holds non-Clone PersistentList<FemNode|FemMaterial>. StoreState edit REVERTED per coordinator. Energy pattern: wrap persistent groups in SealedShared<Group> (Clone + Deref). Needs a fem snapshot type decision (nodes/materials as SealedShared<PersistentList<..>>, ~11 2d sites + 3d), or PersistentList alias Clone (1029-result-hubfix.md decision A, no longer in tree).
- Not re-verified after the 3d results-window fix: rerun chk.sh W-femproc-8 hub wasm -p fem-3d-editor-results-window -p fem-3d-viewer -p fem-2d-viewer.

## Round 2 (coordinator ruling: SealedShared wrap, StoreState untouched)
- Fem2dSnapshot + Fem3dSnapshot: nodes/materials are `SealedShared<PersistentList<..>>` (identical in 2d/3d), `Clone` derived, manual `Default` via `from_reference(Default::default())`. fem-receiving cold gained `sealed_list` / `discard_sealed`; cold.rs 2d/3d and native.rs aliases ported; reads use `(*x.nodes).get(i)` (SealedShared::get clashes), loops use `.iter()`.
- process3d-editor: store-preparation (missing trait-impl brace, effect close_step), retained-commands (terminal_frame_bound! removed, ValueError turbofish).
- fem 2d/3d editors: store-preparation receiving stubs + effect close_step, terminal_frame_bound! removed, 3d `dsl` extern un-cfg(test)ed, transient preparation via ArtifactEphemeralTransferPreparationFactory (FemGumballTransientMutation got original_pack_value_operation_receiving!).
- LOCK NEEDED: hub compositions process + fem gained semio-framework-value.
## Verified
- W-femproc-21 (wasm hub, rc 0, 0 errors): semio-hub-fem-component, semio-hub-process-component, fem-2d-results-state, fem-2d-viewer, fem-3d-viewer, fem-3d-editor-visual-job, process3d-host, process3d-viewer, process3d-editor.
## Not verified
- Native/test targets (cold-tests, snapshot tests, receiving runtime behaviour of SealedShared-wrapped list receivers: native.rs phase demands are compile-only).
Rerun: "$C/1105b-tools/chk.sh" W-femproc-22 "🌎️hub/Cargo.toml" wasm -p semio-hub-fem-component -p semio-hub-process-component
STATUS: FIXED-verified W-femproc-21
