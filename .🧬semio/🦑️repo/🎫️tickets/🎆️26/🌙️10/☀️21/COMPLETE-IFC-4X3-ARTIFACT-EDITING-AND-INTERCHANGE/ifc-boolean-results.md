# IFC Closed-Solid Boolean Results

Implemented schema-first regularized difference, union and intersection in the shared first-party Rust and TypeScript closed-solid kernel and native IFC4/IFC2x3 graph projection. Public owned APIs are booleanSolid(operation,left,right,options) and boolean_solid(SolidBooleanOperation,left,right,limits,tick). subtractSolid/subtract_solid delegate to ordered difference. No external runtime dependencies or external API types.

## Normative evidence

Pinned IFC4 ADD2 TC1 EXPRESS in the retained IFC4 catalog defines three IfcBooleanResult positional arguments and IfcBooleanOperand alternatives, same-dimension rule and explicit closed tessellation flags. The official [IFC2x3 definition](https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcgeometricmodelresource/lexical/ifcbooleanresult.htm) describes regularized solids and ordered difference. IFC4's pinned official document source is authoritative when the public web URL returns 403.

Only supported closed operand representations are admitted. Both operands are validated even for empty/disjoint fast paths. IFC4 tessellation operands require Closed=.T.; IFC2x3 rejects IFC4-only tessellation. Faceted BRep operands require an IfcClosedShell. Halfspaces, clipping results, CSG primitives and CsgSolid wrappers remain explicit unsupported at this slice. Native IDs, world transforms and product/representation identities remain original; temporary operand geometry and final output consume the caller's original counters. Any operand/kernel failure omits the complete product. Source cycles and invalid operation tokens reject.

## Test-driven evidence

The schema operation enum and language-neutral operators.json were added first; the permanent source test initially failed on missing booleanSolid, and graph tests then failed on unsupported IFCBOOLEANRESULT before dispatch was implemented.

Actual source execution: 29 permanent kernel groups passed with zero strict TypeScript diagnostics; 10 permanent source groups passed with zero diagnostics (the tenth retains official fixture provenance and truthful pending scope). Ten neutral geometric relations exercise all three operations, reflected world coordinates, empty inputs, budget/cancellation, containment, identical solids, disjoint solids, crossings and coplanar contact. A union with only edge/point contact is refused because the welded boundary is nonmanifold; face contact is accepted. Independent existing Three BoxGeometry and ExtrudeGeometry output matches accepted known solid volumes and closed edge incidences. Independent Manifold APIs for 39 cases are authored and strict-typechecked but not executed: the Node REPL embedder disallows Wasm code generation and the actual repo route is blocked before native execution. No Wasm bypass was used.

Union tests exposed internal triangulation-edge vertices and collinear remainders. Output reconstruction now discards only tolerance-collinear triangulator remainders, refines every edge against existing output vertices under caller fuel, and still requires strict closed-manifold validation before publication. Operand admission checks were not relaxed. Existing nine subtraction fixtures and unchanged official slab opening tests remain green.

Rust kernel/operator and both graph-source regressions are authored. Native Rust syntax trees contain no grammar errors, but this is not a compile/test claim. Actual repo test routes remain blocked by Windows native-build cache per-directory case-sensitivity allocation access denied, previously reproduced by the main agent; no command bypass or build changes were made.

## Official unchanged model

Retained official-bath.ifc is exactly 3290 bytes, SHA-256 afb2fc8f0702c3d90ab9395ea0897301c98499160bb69dcda88484fec0375fbd, buildingSMART Standards-builds commit a8664407b63d42651e48a78a252c76d09dd6274c, CC-BY-ND 4.0. Original metadata is intact. It contains IfcCsgSolid, IfcBlock and a rounded rectangle profile; those prerequisites are not fabricated. The next separately scoped slice will implement them and replace the pending-scope assertion with an actual official geometric regression.

## Limits

This is bounded finite closed triangle-solid Boolean support, not arbitrary exact-solid, curved analytic CSG or full IFC geometry conformance. Kernel relative tolerance remains 1e-9 of the common coordinate extent. Ambiguous/invalid/nonmanifold results fail atomically. One-shot projection has cancellation and bounded fuel but no resumable cursor. Native compile/runtime and actual Manifold oracle execution remain required external gates.

## Subsequent completed prerequisite slice

The separately authorized CsgSolid/Block/rounded rectangle slice is now complete and its official fixture renders. The earlier pending-scope assertion was replaced with a real official solid regression; see ifc-official-bath.md for current evidence and unchanged source hash.
