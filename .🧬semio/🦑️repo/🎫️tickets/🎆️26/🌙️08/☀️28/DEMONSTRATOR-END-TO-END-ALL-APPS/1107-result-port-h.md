STATUS: GREEN (+ semio-framework-3d, 3d-brep-boolean, 3d-brep-mesh-io port-h-16; 3d-brep-engine port-h-17) wasm32-wasip2 lib check (rc 0): mesh-engine, graph-engine, tool-machine, tool-typing-edit (port-h-2/4; native port-h-5), 3d-brep-representation, 3d-mesh-clipping, 3d-mesh-document (port-h-9), 3d-brep-kernel-contract, 3d-brep-tessellation, 3d-brep-validation (port-h-11), 3d-mesh-reconstruction (port-h-12). UNVERIFIED: native for the 3d crates, all crate tests, downstream callers.

## Done
- graph-engine kinds: CoreEdge on `Leaf` seed; Node through a `NodeParts` mirror (foreign `Point` becomes a coordinate pair) on the canonical record cursor.
- tool-machine: ToolTransaction and ToolMachineRunner on `retirement_fields_type!/seed!`; scrub original close steps via `close_original_effect_step` with `effect.nested` (bool copy reserve kept); `SharedUtf8::admit` takes the actual marker; typing/gesture/scrub ledger `advance_retained_read`, `retain_original_read`, `return_retained_read` and the `OriginalToolLedgerSource` trait take `effect` and return `OriginalRetirementOutcome`.
- mesh-engine: MeshAttribute/MeshTexture/PolygonMeshSource via `artifact_retire_struct!`; PolygonSourcePreparation, MeshMetadataCursor, MeshJsonTask via `derive(RetireOwned)` (`send_only`); canonical decoder: record close step runs inside `cx.with_original_effect` and settles once through `receive_original_retained_work`, whole-close via `close_original_effect_step`; cold `PolygonSourcePreparation::step` keeps its signature and runs under an `AuthoringWallet` (no turn there), parser/projection/ticket steps take the wallet effect.
- value (additive): `ErasedCursor` pub and `impl RetireOwned for Box<dyn ErasedSnapshotRetirement>`.
- tool-typing-edit: `admit` marker, close via effect, `NativeAllocationIdentity` path.

## Open
- External callers of the changed ledger methods (os plugin tool-run tick, tool-machine follow) must pass an effect.
- mesh-engine/tool-machine tests are still on grant APIs (not ported).

Logs: 🗑️generated/1105b/port-h-1..3.txt


## Wave 2 (3d)
- brep-representation: arena Slot/Store, ValidationIssue on retirement_fields; `Store::reserve_insert_original` (reports page work), `reserve_insert_step` keeps its form; BodyCompactionJob `step(body,effect)`/`close_step(effect)` return `OriginalRetirementPoll` (sweep records `effect.performed(1)`); cold `Body::compact` runs under an AuthoringWallet.
- mesh-document: seeds via retirement_fields; tessellation `step(effect)`; `into_retirement(effect)` (error carries `Option<Self>`); cold `tessellate` under an AuthoringWallet.
- brep-validation: `step_funded`/`step_planning_funded` take `effect`; shared `funded_page` reads the funded page reservation.
- brep-tessellation: cold `into_mesh` retirement under an AuthoringWallet. kernel-contract: GeometryHandle seed. mesh-reconstruction: 7 job records on retirement_fields, SourceEqualityTask derive (types made pub+doc(hidden)).
- Also: dsl/schema/encoding `with_effect` closure takes (control,effect).
- Open: callers of changed signatures (brep engine retention job uses BodyCompactionJob step/close_step(grant); validation job callers; tessellation callers; tests).

## Wave 3 (semio-framework-3d, mesh-io, engine)
- modeling: 12 job records on retirement_fields, Work enum derive; MeshModelingJob `step(budget,effect,..)`, `into_retirement(effect)` (error `Option<Self>`); Subdivide/Knife tessellation + handoff on effect; translation production `step(.., effect)` (take via borrowed reply slot; work = effect delta); cold callers via `modeling_step` (AuthoringWallet).
- brep mesh-io `close_step(body,effect)`, healing engine `close_mesh_import_*(effect)`; boolean resumable job validation step via `cx.with_original_effect` + `receive_original_retained_work`.
- brep-engine: session `Entity` derive, retention job `step(kernel,effect)`.
- Open: external callers of these signatures (flow plugin, tests).