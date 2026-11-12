# cadrest (11/12)
- Installed dsl::original_pack_value_operation_receiving! on CadMutation, CadWorldWindowConfigMutation, PlaybookMutation (stl precedent).
- W-cadrest-1 (hub wasm cad-viewer cad-modes playbook-inferences) rc 0.
- Ported cad-editor: removed terminal_frame_bound! (impl already has terminal_frame_release_bytes), close_step(effect) -> OriginalRetirementPoll in artifact-/config-store-preparation, media-export-job via close_original_effect_step(RetainedWorkPoll).
- Cargo.toml: semio-framework-value added to hub compositions cad + playbook (LOCK NEEDED logged; check passed anyway).
- W-cadrest-3 (cad-editor, hub-cad-component, hub-playbook-component, hub wasm) rc 0.
- W-cadrest-4/5/6 (ext manifests, wasm): aec-building-energy (aec-energy-component), aec-building-structure (aec-structure-component), spatial-shape: rc 0 each.
## Not verified / other lanes
- cfg(test) targets not compiled; no cad/playbook test run. Debt: cad editor presence retirement-bytes test (from cadmodel) still missing.
- Other reds in W-stdio7-18 (not mine): ifc4x3-mvd (ifcmvd, since fixed), ifc-2x3-base-details / ifc-details-wire / ifc4x3-population (ifcdetails), remodeling-images/-model (remodel), energy-model-native-mutations-lifecycle/-singletons + mut-boundary (energylife/energynative).
- Rerun: chk.sh W-cadrest-7 "🌎️hub/Cargo.toml" wasm -p semio-hub-cad-component -p semio-hub-playbook-component
STATUS: FIXED-verified W-cadrest-3
