# Declared Interchange Record Census

This is a read-only current-source census while phase8 and root geometry/Details are frozen for independent review and expanded readers. No production, neutral selected test or package registration edits occur here. The exact current before observations are in fem-zero-record-probe.ts and generated/fem-zero-record/before.json, with source hashes for fem.ts/geometry.ts/control.ts. The separate zero-force external input/oracle observes the freshly emitted unchanged STEP file.

BIM ModelSnapshot declares the required project,60 multiline optional Record families and3 additional merged-interface families supports/load_cases/loads:63 record families total. The spatial endpoint implements sites/buildings/storeys (3 families); the other60 nonempty source families receive explicit unsupported-family warnings and are not reconstructed. This remains an explicit partial spatial endpoint, not a silent whole-BIM roundtrip claim. Its source dictionary IDs are written into spatial ObjectType but imported IDs are canonical IFC-derived keys; stable original artifact identity remains a documented source-artifact concern.

| Declared supported fields | Current mapping and coverage | Remaining precision/semantic scope |
| --- | --- | --- |
| Project name/description/author/organization | IFC root name/description and actual FILE_NAME metadata; existing actual BIM fixture compares whole project | Arbitrary source name lengths are not exhaustively full-schema-admitted by this endpoint |
| Project phase_names | New phase8 exact ordered typed-text contract, independently closed | Native phase-list parity and expanded selected reader still separate |
| Site name/latitude/longitude/elevation/true_north | Existing actual angle/site/context fixtures and mismatch refusals | Compound geographic angle export rounds to integer microarcseconds; north is a direction, so raw angle periods/bits need not survive |
| Site boundary | Explicit site-boundary warning and omitted projected curves | A distinct future geometry feature; do not reinterpret the warning as preservation |
| Building site/name/origin/rotation/elevation | Explicit hierarchy and relative frame; actual placement/rotation tests | Raw rotation periods/bits are normalized by trigonometric frame reconstruction |
| Storey building/name/level/height/optional cut_height | Typed authored properties with qualified definitions/units, central edit/inverse and source-order cumulative laws | Identity and geometric tolerances are explicit; unsorted cumulative arithmetic preserves original evaluation order |

FEM3dSnapshot declares8 collections plus analysis. Nodes, frame/bar elements, material/section engineering values, fixed support subsets and nodal/memberUDL loads are mapped. Solids, area loads, self-weight and combinations explicitly refuse. Analysis modal/buckling/deformation display settings are intentionally omitted with a dedicated analysis-settings warning; importer defaults are projection defaults, not recovered source values. Units currently require SI metre/project and actual dimensioned SI engineering properties, explicitly refusing conversion cases rather than guessing. Source IDs become canonical IFC-derived destination identities. Fixed DOF arrays represent subsets and importer canonicalizes their order; nonzero frame roll reconstructs a periodic angle. Unreferenced sections are absent from the IFC section-definition graph because definitions are created at member use; broad partial fidelity does not claim a complete library exchange.

The concrete new selected concern is inside already-supported nodal loads, where zero is a legitimate finite Binary64 value. Public before observations:
- Positive zero: one declared load, one actual exported IfcStructuralPointAction/IfcStructuralLoadSingleForce, zero imported loads.
- Negative zero: same one-to-zero disappearance.
- Nonzero3: one-to-one.
- Central SetEntityArg force edit to5 makes all three produce one imported load. The exact central inverse restores the original IFC graph and its current projection in every case. Thus CQRS/inverse itself is intact; projection erases explicit zero records.
The current exporter writes all six components as explicit numeric zero except the selected value; importer emits only truthy components. This makes absent DOF and explicitly selected zero indistinguishable in its own output.

The pinned ADD2 source at10943–10953 declares all ForceX/Y/Z and MomentX/Y/Z OPTIONAL; it has no nonzero WHERE condition. A direct schema-shaped representation can keep one selected component present and leave the other five unset. This is preferable to inventing a custom property contract for information that already has an exact optional slot.

Fresh single-file external observation via owned ZeroForceOracle/IfcOpenShellOracle: actual IfcOpenShell0.8.5 / Python3.13 parses exactIFC4X3_ADD2, one named point action and one named load, six explicit zero components,61 entities and zero full-EXPRESS diagnostics. generated/fem-zero-record/external-before.json retains executable/version/source SHA/raw diagnostics. It corroborates that the source IFC action exists and is accepted by that implementation. It does not by itself prescribe semio's output record policy, certify the inverse scheduler, prove native parity or establish universal full-view validity.

Read-only native_fem_owned.rs evidence bounds parity further: its actual guard requires unloaded global3D analysis and refuses loaded/specialized structural semantics. The TS loaded engineering route is already a broader selected projection; native loaded parity is a separate unimplemented frontier, not a parallel passing result.

Select explicit-zero nodal record preservation as the next bounded feature; its schema-first plan is fem-zero-record-plan.md. No geometry/Brep, phase8, Details, control, build or native source was edited for this census.

