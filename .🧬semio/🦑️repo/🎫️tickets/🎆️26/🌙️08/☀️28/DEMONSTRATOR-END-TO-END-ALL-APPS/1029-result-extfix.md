# 1029 result extfix

STATUS: CHECKPOINT (usage cut). Final 21-batch sweep is RUNNING detached (script `C/1029-tools/extfix/run-final-resume.sh`, logs `C/🗑️generated/1029/extfix-<batch>-final.{txt,rc}`); do not restart it. Snapshot at checkpoint: GREEN cad-aec-structure, cad-spatial-shape, flow-dictionary, flow-draw, flow-list, flow-logic, flow-primitive, playbook-procedural; process-concrete running; not yet in sweep: process-metal, process-robotic, process-wood, sourcing-beams/slabs/windows. RED in sweep only from peers' mid-edit foreign crates or infra (ProgressDeadlineExpired/ENOSPC): flow-bim, flow-brep (framework tool-typing-edit), flow-math (value-retirement), flow-text (60 min idle queue), cad-aec-building/energy (plugin-ownership genesis / plugin-instance-jobs; the resume script reruns them last). Individually green earlier after my fixes: process-concrete, process-metal, cad-aec-building, flow-brep, playbook-procedural.
NEXT STEP: when `C/🗑️generated/1029/extfix-run-final.rc` exists, run `C/1029-tools/extfix/status.sh final`, then rerun any RED batch with `nohup bash C/1029-tools/extfix/run-retry.sh final2 flow-bim flow-brep flow-math flow-text ...` (one cargo at a time), inspect `^error` in its txt (peer-owned crates: report only), and write the GREEN/RED table here.

## Fixes
- process3d model (`process-metal`, `process-concrete`): `🪚️machines-and-catalogs/🦀️.rs` lost `use semio_framework_dispatch_macros::dyn_enum;` in the module split -> `#[dyn_enum]`/`__semio_dispatch_MachineCatalog!` unresolved. Import restored.
- cad aec-building: `CreateBuildingStorey::plan` moved from `CompositeMutationKind` (no longer has `plan`) to `ColdCompositeMutationKind<CadSnapshot,CadMutation>`.
- flow-brep-mesh (`playbook-procedural`, `flow-brep`): `mesh-operator-io::drive_job` rebuilt on the current `OperatorJob::step(&mut StepContext)` contract (fresh `StepContext` with per-step receipt, `work_error` maps `OperatorWorkFailure` to `EvalError`), `MeshModelingOps` imported, `MeshOperation` fields `pub(crate)` (struct is built in the sibling `registration` module).
- flow-brep tessellate/geometry: Pack wire is paged (`EvaluationWire`): `ready_wire`, `step` return and geometry-turn `wire` hold `PagedBytes`, single `to_vec_owner()` at the `ExtensionInvokeStep.payload`/`ArtifactInferenceExecution.canonical_payload` boundary (same as puzzlefin's os-flow-extensions fix). `TessellationResources::invoke` takes `&self` (it only uses its mutex).

- playbook-procedural: `learning-program-and-geometry` imported `text` from the playbook sqlite snapshot (stale) -> `semio_framework_ui_contract::text`; `build_tool_job` is no longer `async` in `ArtifactApp`; the preview's removed `SessionCapture::tessellate_geometry` replaced by a local `tessellated_mesh_json` that drives `tessellate_step` with a fresh `StepContext` per turn to `GeometryStep::Ready` and renders the mesh with `pack_json::to_json_string`.
- flow-bim / infinite-directed-presentation: the `NativeReceive` derive rejected `#[value(default = "path")]` (IoPortSpec `cardinality`/`visible`) although its own doc says a path default is canonical-wire only and natively required; the stale rejection is removed (SHARED CHANGE below).

## SHARED CHANGE
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs`: removed the "custom native defaults require an explicit original receiving port" refusal (path default = natively required field).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs`: explicit re-export of `EvaluationWire` (needed by flow-brep).

## Notes
- `stdio-step-mutations` (process-concrete) no longer fails (typedops fixed `semio_framework_pack_json`).
- ENOSPC on the rust cache image and "Original resource loan deadline exhausted" are infrastructure, not code; scripts `C/1029-tools/extfix/{run.sh,run-retry.sh,run-all.sh}` retry them.
