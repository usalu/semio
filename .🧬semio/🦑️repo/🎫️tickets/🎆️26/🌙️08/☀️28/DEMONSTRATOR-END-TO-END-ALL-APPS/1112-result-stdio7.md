# 1112 stdio7
## Done (all hub wasm, `chk.sh W-stdio7-<n> "🌎️hub/Cargo.toml" wasm -p <crate>`)
- image-editor duplicate natural_file removed; cad-export step-snapshot dep (LOCK NEEDED).
- ifc-geometry mesh pages (with_original_effect), ifc-4-model (bespoke owned receiver replaced by owner.receive receive_pack; output-slot native; write-pack drive_cursor; Cargo deps ifc-geometry+ifc4-catalog, LOCK NEEDED; removed owned-graph.rs/owned-tests.rs/owned.feature, close = drop).
- png-snapshot 1.2, bcf-inferences, xlsx/docx/pptx base-model (decode_typed/encode_typed), xlsx/pptx/docx-transitional (removed OpBinaryReceiving macro: no OpBinary), gltf-snapshot-root (geometry pack receive/encode output-slot), ifc-2x3-base-model (+ifc2x3-catalog dep, LOCK NEEDED), png-tests + tiff mutation-regressions mirrors (close_step), docx-base, ifc4x3-schema/-step (receive_nested 3-generic form), tiff-snapshot, ifc-details-profile.
## Verified rc 0
W-stdio7-5 (ifc-4-model cad-export png-snapshot), -8 (bcf xlsx docx pptx), -10 (gltf-snapshot-root), -12 (ifc-2x3-base-model png-tests docx-base pptx-transitional xlsx-transitional), -14 (ifc4x3-schema ifc4x3-step), -16 (tiff-snapshot docx-transitional ifc-details-profile), -19 (tiff-mutation-regressions). xlsx-transitional: macro removed, covered by W-stdio7-18 (no error).
## Open (stdio, unported, W-stdio7-18)
- ifc4x3-mvd 572, ifc-2x3-base-details 119, ifc-details-wire 82, ifc4x3-population 6: framework shared-source API change (Arc<T>: RetireOwned gone -> ArcLease; StagedSourceAdmission/RetainedCloneSource/begin_granted/step_granted now effect-shaped; close_step(grant) -> (body, effect); also templates/apply/receive.rs callers). Large coordinated port, not started.
- Other lanes in W-stdio7-18: remodel-remodeling-images 64, remodel-remodeling-model 44, cad-cad-viewer 1, cad-cad-modes 16, playbook-playbook-inferences 8, energy-model-native-mutations-lifecycle 42 / -singletons 78 / mut-boundary 6.
- Debts: ifc-4 [[test]] original_ifc_native_receiving targets removed receiving API; gltf custom text-refusal context messages dropped.
STATUS: BLOCKED ifc4x3-mvd/ifc-2x3-base-details/ifc-details-wire/ifc4x3-population (shared-source effect port) keep semio-hub-stdio-component red; everything else assigned verified
