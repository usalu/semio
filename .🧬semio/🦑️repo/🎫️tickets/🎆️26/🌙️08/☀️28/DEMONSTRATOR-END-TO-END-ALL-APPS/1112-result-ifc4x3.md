# 1112 ifc4x3 (11/12)
## Step 1 semio-s-artifact-stdio-ifc-4x3 GREEN (W-ifc4x3-5 hub wasm rc 0, baseline 464 errors)
Root-cause groups and fixes:
- [x] Arc<IfcSnapshot>: RetireOwned (117) -> templates apply work.rs: SnapshotLease + StagedSourceAdmission<IfcSnapshot,SnapshotLease>, funded_turn advance, close_into; semantic close wrapped with charged.
- [x] apply factory.rs: retained_command_factory_admission! + admit_retained_tool_producer; build(request, output, cx) -> Result<bool>; editor build_tool_job new signature; bounded_first_step u64 work units.
- [x] apply receive.rs: effect-shaped receiver close (closed_in/owned_step/settled), receiver.close_demands() 0-arg.
- [x] apply staging-native: raw clone cursors + advance_cursor/take_cursor/closed_cursor; StagedSourceAdmission<DslValue,StagedFormRequest>; settled construction admission (copy_text/reserve take it); retire_self! for IfcTemplateSourceWork; IfcPreparedTemplateSource seed via Page; Page pub; plan::guid pub(super).
- [x] editor guided-work.rs / render-work.rs: derive(RetireOwned) needs Sync, cursors are !Sync -> owner slots as Options and hand-closed via retire_self!; build_body via receive_effect; construction admission field on Ifc4x3RenderWork; details now Option.
- [x] editor 🦀️.rs: reference_birth replaces initial_snapshot; viewer imports WindowKit (KIND_ID/window_kind).
- [x] orphan impl IfcDetailsTypeMetadata for StaticTypeExpression: trait + IfcTypeProjection + IfcTypeExpression impl moved into ifc-declarations (catalog), profile re-exports, express crate implements it. No Cargo edits.
- [x] STAGED_FORM_BODY_KEY: const exists in arg-schema, was missing from semio_plugin::action_staging re-export list -> re-added to the list (framework 📦️packages/🦀️rust/🦀️.rs).
- [x] arg-schema: impl OriginalSourceProjection<DslValue> for StagedFormRequest (args projection).
- [x] ifc4x3-fem native_fem_build.rs: nodes is now SealedShared<PersistentList> (femproc) -> nodes.get().get(i).
## Debts / not verified
- Behaviour not run; cfg(test) code in apply (work.rs test_planning, tests.rs), staging-native tests, editor tests still on old APIs (only tests.rs initial_snapshot path fixed).
- Template/guided close order and quotes never exercised; staged-body build uses body.demand() as the receive_effect quote.
- Rerun: "$C/1105b-tools/chk.sh" W-ifc4x3-<n> "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-stdio-ifc-4x3
## Step 2 semio-hub-stdio-component GREEN (W-ifc4x3-7 hub wasm rc 0)
Remaining reds after the ifc crate were two, both fixed:
- [x] stdio html facade (`🌐️html/🦀️.rs`) did not re-export editor/viewer (they live in html-mutations/inferences after the split) -> added tsv-shaped `editor::html` / `viewer::html` re-export modules.
- [x] hub compositions/stdio Cargo.toml: `semio-framework-value` moved from dev-dependencies to dependencies (dyn_enum_close! expansion needs it). Same package/path; no lock change was needed for the check (LOCK NEEDED only if the coordinator's lock gate flags it).
## Verified
- W-ifc4x3-5 semio-s-artifact-stdio-ifc-4x3 rc 0; W-ifc4x3-7 semio-hub-stdio-component rc 0 (wasm).
## Not verified
- No native, no --tests, no runtime behaviour; cfg(test) code of ifc-4x3 apply/staging-native/editor still on old APIs.
STATUS: FIXED-verified W-ifc4x3-7
