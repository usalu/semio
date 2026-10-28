# Base Plane Halfspace Clipping

Implemented schema-first owned SolidPlane/subtractHalfSpace and native SolidPlane/subtract_half_space plus original IFC4/IFC2x3 IfcBooleanClippingResult graph projection. The public plane API removes the positive signed side. Source AgreementFlag TRUE removes the negative side because the base normal points away from material; FALSE removes the positive side. Source position must be an original IfcAxis2Placement3D over an IfcPlane. Only DIFFERENCE and supported swept-area/nested-clipping first operands are admitted. Boxed/polygonally bounded halfspaces and non-plane surfaces remain explicit unsupported.

## Exact bounded domain

For an admitted bounded host A in its exact source bounding domain D, A−H=A−(D∩H). The kernel clips D's six oriented faces, builds its convex plane cap, admits that closed cutter, and uses the existing solid subtraction/reconstruction. It does not choose a finite arbitrary extent. General host cap holes, inward cavity shells and disconnected components are preserved by the same mandatory closed-manifold engine. Construction, admission, traversal and reconstruction share one Boolean work controller and original projection callbacks. Output publication is atomic per product.

Mapped plane normals use an inverse-transpose equivalent: transform both plane tangent columns, normalize their individual scales, cross, and use the third column's sign to retain the original source normal direction through reflection. Plane origin is transformed as a position. This covers nonuniform affine scaling instead of treating a normal as a position vector. Original world placements, mappings and product identities are preserved.

## Tests before implementation and actual evidence

Descriptive source contract and language-neutral halfspaces.json were written before implementation; the first actual test failed on missing subtractHalfSpace. Source AgreementFlag test then failed on unsupported IFCBOOLEANCLIPPINGRESULT before dispatch was added. A strict owned request schema at Boolean/✂️halfspace/🔣️.json is validated against the neutral requests through existing AJV.

Actual permanent kernel suite: 43 groups passed with zero strict TypeScript diagnostics. Actual permanent source opening/CSG/clipping suite: 25 groups passed with zero diagnostics. Coverage includes nine neutral axial/oblique/full/empty/tangent relations, exact ordered material side, normals scaled1e−120 through1e120, reflection, holed and cavity caps, disconnected hosts, original caller work/polygon/cancel/completion gates and invalid/open operands. Source neutral cases run through both IFC4 and IFC2x3: half cube32, annular cap24, nested clips16, nonuniform2×3 mapped oblique plane192, reflected plane32. Unsupported operators/surfaces/halfspace variants and source cycles omit the product.

Independent existing Three ConvexGeometry constructs the tetrahedron for x+y+z≤1 inside the cube; its volume1/6 matches the accepted first-party result. Independent Three Matrix4+ConvexGeometry constructs the transformed triangular prism matching the nonuniform mapped source output192. Existing independent Three official slab/bath sampled-volume regressions remain green.

A peer audit reproduced accepted-empty loss of a positive1e−10 retained slab under an EPS-padded full-side classification. Fixed in both languages: full/empty domain classification now uses exact signed comparisons; a true crossing with nonzero corner distance within the normalized tolerance explicitly refuses ambiguous-halfspace-domain-intersection. Permanent1e−12/1e−10/1e−9 positive sliver cases assert refusal, not erasure. This is conservative finite tolerance geometry, not an exact arbitrary-solid claim.

Rust kernel, source normal mapping and both standards' source regressions are authored; all five affected Rust source/test syntax trees have no grammar errors. Native compile/runtime remains gated by repo's Windows per-directory case-sensitivity allocation access denied before tests. Independent actual Manifold trimByPlane APIs are authored for the same nine neutral planes (48 total independent Manifold cases now) and strict-typechecked; runtime is unexecuted because Node REPL Wasm code generation is disallowed by the embedder and actual repo route is blocked. No Wasm or command bypass was used.

## Official example research

Pinned buildingSMART Standards-builds IFC4 ADD2 TC1 available Annex E .ifc sources were searched and contained no BooleanClippingResult. The peer auditor also searched all25 IFC2x3/IFC4 .ifc files in pinned official Certification-datasets/Sample-Test-Files and found no clipping or halfspace instances. No unchanged eligible official clipping fixture has been claimed; authored neutral sources are labelled as such. Existing unchanged official slab and bath files keep their exact hashes and licenses.

## Limits

Finite closed sampled host solids and base unbounded plane halfspaces only. Ambiguous, nonmanifold or unrepresentable output refuses atomically. Kernel tolerance remains1e−9 of normalized extent. This is not boxed/bounded halfspace, other analytic surface or full CSG support. Original caller/context caps remain in force; one-shot execution has no resumable cursor. Native compilation/runtime and independent Manifold execution remain required external gates.
