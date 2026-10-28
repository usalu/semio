# Base Plane Halfspace Clipping Plan

Implement bounded first-party subtraction of an oriented plane halfspace from an admitted finite closed solid, then integrate IfcBooleanClippingResult DIFFERENCE with a supported swept-area or recursively clipped first operand and exact IfcHalfSpaceSolid over IfcPlane as the second. IFC4/IFC2x3 retain their explicit declaration restrictions. Other halfspace subtypes and primitive kinds remain unsupported.

## Normative side

Official IFC2x3 IfcHalfSpaceSolid documentation says AgreementFlag TRUE means the surface normal points away from material; FALSE means into material. Thus TRUE removes the negative plane side and FALSE removes the positive plane side. Position is an IfcAxis2Placement3D inherited by IfcPlane. IFC4 pinned EXPRESS rules require DIFFERENCE, first operand SweptAreaSolid/SweptDiskSolid/BooleanClippingResult and second HalfSpaceSolid; the current supported first leaf is ExtrudedAreaSolid. IFC2x3 first operand is SweptAreaSolid/BooleanClippingResult. Shared schema must record these narrower implemented capabilities.

## Exact bounded restriction

For an admitted finite host A inside its exact source bounding domain D, A minus H equals A minus (D intersect H). Construct D intersect H by clipping the six domain faces against the plane and adding its planar convex cap, with no arbitrary extension distance or guessed finite fallback. Then invoke the same mandatory closed-solid Boolean admission/reconstruction under the same work controller. This preserves general host cap rings and holes through the existing solid Boolean engine rather than flattening a holed cap. Full/empty/tangent cases must be exact bounded classifications after operand admission.

Normalize coordinates and plane normal before numeric predicates. Geometry outside representable finite ranges or ambiguous nonmanifold results must be refused atomically. Positive/negative source material side, mapped/reflected world transforms and nonuniform transform normals must be tested. Transform the plane covector by inverse transpose, not by treating its normal as a position vector.

## Tests and gates

Add a neutral halfspace contract and cuboid axial/oblique/full/empty/touch/through-tunnel cap-hole vectors before implementation. Compare actual accepted volume/topology with independent existing Three geometry/plane/triangulation operations and analytic restrictions. Add original cancellation and work/topology gates, invalid/open operand tests, source nested clipping graphs for both standards, reflected/mapped plane placement and truthful unsupported bounded/boxed halfspaces. Preserve existing Boolean/bath/slab tests. Seek a pinned unchanged official clipping example; currently all available retained IFC4 ADD2 TC1 Annex E .ifc sources searched have no clipping result.

Native Rust counterpart and regression source will be authored and syntax checked. Native compile/runtime remains gated by the existing repo cache access-denied failure; Node Wasm embedder disallows Manifold runtime. Neither is bypassed or claimed passed. All one-shot projection remains cancellable/bounded but not resumable.
