# 1030 result resid

STATUS: PORT WAVE in progress, nothing red in my lane. Hub wasm chk312: only `os-store-one-item-publication` (12), `s-fem-analyses-assembly` (4), `s-plugin-energy-engine-sim` (75) fail (framework/fem/energy lanes); part21 ported. Driver `final2.sh` (pid final2.pid) re-probes hub every 10 min while any framework crate is red, then runs gis.sh (hub-gis+inference, plugin-host), the cad-editor check and sweep x4 (13 batches; `status.sh x4`, marker `extfix-run-x4.rc`).


## Fixes
- store `field_set_mutations!`: Mutation impl split, `diff`/`inverse` now `ColdMutation<$record>` (config macro stale vs trait); kernel store root re-exports `field_set_mutations_{text,binary}` (they lived unexported in text-io crate).
- flow presence: dropped duplicate `RetireOwned` derive; flow-mutations: binary `diff` codec mounted in the diff crate (orphan impl) and re-exported; editor host `host.dag.set_*` -> `FlowHost` methods; flow-eval-tick `preview_source: None`.
- pptx-strict: text/binary mutation codecs mounted in the strict-mutations crate (orphan E0117).
- tiff: config mutation ported to DslEnum+DslReceiving (+RetainedClone) with `variants_text`; `select_ifd(n)` helper; no `.expect` left in that mutation (audit row 5).
- gisterrain editor: `ColdMutation` import, `Fault.into()`.
- raster: asset `value::bytes` (replaces missing `asset_data_base64`); `raster_document_store_owners()` now returns `Result` (audit row 4), 10 test callers `.expect`.
- cad: transient `extern crate pack` (lib name is `pack`; E0463 was real); `CadComposedPane` made `pub` (cad-kit privacy error).
- plugin-host: `BackboneDocument.transitions` is now `ArtifactHistoryLedger<MutationEnvelope>` (matches envelope owners); test helper takes `HistoryTransitionEvents`. UNVERIFIED (native framework check not run).

## Open (not mine / not done)
- `semio-hub-gis` inference.rs: 21 errors, stale hub-gis binding API (genesis share_snapshot/digest, ArtifactInferenceExecutionStep, MemberStoreOwner...). Needs its own port.
- gen-owned: procedural 2d host/3d io-geometry, playbook-inferences, playground-editor. fem-owned: fem-3d-editor-*, fem-2d-io.

## hub-gis port (done, unverified by compile)
- `MapInferenceProposalBindingV1`/`ComposedMapCommitBindingV1::member_stores` now returns `Result` (owner catalogue funding can refuse); committer maps it to `Rejected`.
- inference.rs: stepped `infer_gis_map_controlled` driven by `run_controlled` (fresh `StepContext`); request carries `operation` = hash of job id, `generation` 1, `retained` = grant sized from the request budget (review item 6a: the one-turn executable ignores operation/generation/grant today, verified in `🧠️inference-service`, so values are informational; a multi-turn executable would need the runtime to thread real ids); stores built by one `member_store` with `ArtifactPackGenesisProducer`, `genesis.facts().{share_snapshot,digest}`, `funded_bounded_artifact_store_owners`.
- 2d-path-stroke dev-dep `tiny-skia` (peer, 19:20) used workspace inheritance, which broke Cargo.lock refresh in every plugin workspace root lacking it (flow-list x2 red): now explicit `"0.11.4"`. Also stdio step-geometry Cargo.toml gained the brep/session/value deps its source imports.

## cad-editor (rtpanes request)
- Added `semio-framework-dsl-record(-derive)` deps to cad-editor Cargo.toml and imported `CadDispatchCtx, CadPlayRuntime, CadPlayView` from cad-kit in the lib. chk20 (cad workspace native): no error in cad-editor itself, but its dependency chain is blocked by peers: `semio-framework-pixels` (`CompositeRetirement::advance` missing, compositing/layers:130) and `semio-framework-os-flow-drawing` (`semio_framework_2d::trace::trace_bitmap_paths` missing). Rerun: `C/1030-tools/resid/chk.sh <n> '✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml' native -p semio-s-artifact-cad-cad-editor`.

## PORT WAVE (10/31)
- stdio `contract/part21/receiving`: hand-written `NativeReceive`/`NativeReceiver` ported to `receiver()` + `RetainedWorkOutcome` (advance/take/close_step; `bind`/`close_one`/`step_granted`/`begin_granted` outcomes) and its test; chk33/34 show part21 no longer red. Other lane crates (gis/flow/cad/raster/pptx/tiff) not yet reached: hub chk34 still stops in ~17 framework/fem/energy crates. `final.sh` keeps re-probing (chk35+), then gis.sh, cad-editor, x4.
