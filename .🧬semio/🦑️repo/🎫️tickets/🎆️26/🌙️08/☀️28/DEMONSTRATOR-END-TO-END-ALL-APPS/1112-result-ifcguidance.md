# 1112 ifcguidance (11/12)
## Step 1 ifc-details-guidance GREEN (W-ifcguidance-3 hub wasm rc 0, baseline 150 errors)
- profile: `retire_self!` macro (reply-shaped self-cursor from inherent begin_close/close_demands/close_owned(body,effect)/empty), `closed_in`, `settled_construction`.
- wire source_loan: `closed_cursor` (effect-shaped clone-cursor close), `settle_cursor` pub.
- guidance files ported: type_guidance_request.rs (RequestCopy, effect-shaped admission/cursor/take_authority_into), type_guidance_candidate.rs, type_input_choices.rs, type_input_{list,set,typed}_copy.rs (shared `OriginalConstructionAdmission` passed as `adm`), type_input_set_controls.rs, type-guidance.rs (Work owns `construction: Option<OriginalConstructionAdmission>`; settled construction; `demands()` wraps `step_demands()` with `settled_construction`).
- Debts: behaviour not run; cfg(test) code on old API.
## Step 2 ifc-details GREEN (W-ifcguidance-5 hub wasm rc 0, baseline 87 errors)
- work.rs: SnapshotLease + StagedSourceAdmission, cursor copy via source_loan advance_cursor/take_cursor/closed_cursor, close via closed_in; command_wire.rs: JSON grammar step under profile `receive_effect`, effect-shaped close; admission.rs: codec advance/close under `effected`.
## Step 3 ifc4x3-exchange GREEN (W-ifcguidance-9 hub wasm rc 0, baseline 25-30 errors)
- new `🚪️io/native_receiving.rs`: `decoded/encoded/nested/nested_value/refused` run a value body as one funded `native_snapshot_turn` on the new 3-generic `receive`/`receive_nested`; native_pack/native-gltf-owned/-write/-export/-import and native_exchange (IoError cause into NativeSnapshotFailure) ported; gltf pack path now `gltf_io::snapshot_codecs::pack_codec::geometry`.
## Step 4 SnapshotLease hoist (W-ifcguidance-12 rc 0: manifest-view-model, ifc-details-wire, ifc-2x3-base-details, ifc4x3-mvd)
- No ifc crate is shared by all three (wire/2x3 vs mvd depend on different models), so the one definition is the framework `ArcLease<T>`: `get()`, `addr()` (retirement arc-custody) and the generic `OriginalSourceProjection<T> for ArcLease<T>` (value retained-clone projection.rs). The fourth copy (`ArcLease<ViewModel>` in manifest view-model `shared-source`) is deleted.
- `SnapshotLease` = `ArcLease<IfcSnapshot>` (wire), `ArcLease<Ifc2x3Snapshot>` (2x3 details), `Lease` = `ArcLease` (mvd concepts/lease.rs). The three `funded_turn` copies remain (not requested).
## Step 5 ifc4x3-fem GREEN (W-ifcguidance-14 hub wasm rc 0, baseline 49 errors)
- exchange exports the receiving adapters (`decoded`, `decoded_nested`, `scoped`, `settled`, `nested`, `refused`, `io_error`); fem build/owned/load_build/domains use them; identity-index `lookup`/`receive` and `Fem3dSnapshot::receive_authoring` via output-slot helpers (`rank`, `identities`, `authored`).
## Open (W-ifcguidance-15, hub wasm -p semio-hub-stdio-component, rc 101)
- semio-s-artifact-stdio-ifc-4x3: 464 errors, UNSTARTED, FREE. Files: 🧬️schema/🏷️templates ✍️apply (factory.rs ToolJobFactory admission_demands/admit_job + ToolOperationSpec::new, receive.rs, work.rs, staging-native/🦀️.rs + drafts.rs: RetainedCloneOwner close_step/advance/admit_borrowed + hand RetireOwned for Page), ✏️editor (guided-work.rs 74, 🦀️.rs 65, work.rs 51, render-work.rs 46, receive.rs 14, details.rs 1), 👁️viewer 🦀️.rs (MeshWindowKit KIND_ID/window_kind), action_staging STAGED_FORM_BODY_KEY missing. 117 errors are `Arc<IfcSnapshot>: RetireOwned` (use `SnapshotLease` + `StagedSourceAdmission` as in ifc-details work.rs); the NonNull<...Cursor> Sync errors cascade from them.
- Other lanes' reds: none seen on this path (bim-model-snapshot no longer errors).
## Verified
- W-ifcguidance-3 ifc-details-guidance; -5 ifc-details; -9 ifc4x3-exchange; -12 manifest-view-model + details-wire + 2x3-base-details + ifc4x3-mvd (lease hoist); -14 ifc4x3-fem (all hub wasm, rc 0).
## Not verified / debts
- Behaviour never run (no tests/--tests/native). cfg(test) code of guidance/details/exchange/fem still on old APIs (source_loan_tests, type_guidance tests, native-gltf-owned-tests only path-fixed).
- Guidance `demands()` now adds `settled_construction` overhead; the settled-construction fuel floor and `actual` allocation marker were not exercised.
- Receiving adapters (`native_receiving.rs`) report one performed unit per frame and drop delegated child outcomes' work (children commit to the native ledger); revisit if work parity with the framework meter style is required.
- Rerun: "$C/1105b-tools/chk.sh" W-ifcguidance-<n> "🌎️hub/Cargo.toml" wasm -p semio-hub-stdio-component
STATUS: BLOCKED semio-hub-stdio-component still red on semio-s-artifact-stdio-ifc-4x3 (464 errors, templates apply/editor/viewer, free); ifc-details-guidance, ifc-details, ifc4x3-exchange, ifc4x3-fem and the SnapshotLease hoist FIXED-verified (W-ifcguidance-3/5/9/14/12)
