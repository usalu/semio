# 1029 result extfix

STATUS: IN PROGRESS. Green so far (wasm32-wasip2 check, this session): process-concrete, cad-spatial-shape, process-metal, cad-aec-building. Pending: playbook-procedural, flow-brep (fixes landed, rerun queued), then final 21-batch sweep.

## Fixes
- process3d model (`process-metal`, `process-concrete`): `🪚️machines-and-catalogs/🦀️.rs` lost `use semio_framework_dispatch_macros::dyn_enum;` in the module split -> `#[dyn_enum]`/`__semio_dispatch_MachineCatalog!` unresolved. Import restored.
- cad aec-building: `CreateBuildingStorey::plan` moved from `CompositeMutationKind` (no longer has `plan`) to `ColdCompositeMutationKind<CadSnapshot,CadMutation>`.
- flow-brep-mesh (`playbook-procedural`, `flow-brep`): `mesh-operator-io::drive_job` rebuilt on the current `OperatorJob::step(&mut StepContext)` contract (fresh `StepContext` with per-step receipt, `work_error` maps `OperatorWorkFailure` to `EvalError`), `MeshModelingOps` imported, `MeshOperation` fields `pub(crate)` (struct is built in the sibling `registration` module).
- flow-brep tessellate/geometry: Pack wire is paged (`EvaluationWire`): `ready_wire`, `step` return and geometry-turn `wire` hold `PagedBytes`, single `to_vec_owner()` at the `ExtensionInvokeStep.payload`/`ArtifactInferenceExecution.canonical_payload` boundary (same as puzzlefin's os-flow-extensions fix). `TessellationResources::invoke` takes `&self` (it only uses its mutex).

## SHARED CHANGE
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs`: explicit re-export of `EvaluationWire` (needed by flow-brep).

## Notes
- `stdio-step-mutations` (process-concrete) no longer fails (typedops fixed `semio_framework_pack_json`).
- ENOSPC on the rust cache image and "Original resource loan deadline exhausted" are infrastructure, not code; scripts `C/1029-tools/extfix/{run.sh,run-retry.sh,run-all.sh}` retry them.
