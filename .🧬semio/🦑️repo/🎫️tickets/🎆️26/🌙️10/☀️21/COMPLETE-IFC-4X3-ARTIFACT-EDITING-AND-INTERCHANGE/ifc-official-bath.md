# Official Bath CSG Geometry

Completed the bounded next prerequisites: IfcCsgSolid expression wrapper, placed IfcBlock primitive and IfcRoundedRectangleProfileDef in the common Rust/TypeScript geometry engine. Both IFC4 and IFC2x3 normative declarations include these types. The IFC4 pinned EXPRESS catalog and official IFC2x3 EXPRESS source confirm original argument order and positive radius limits; block axes extend positively from the primitive placement origin.

## Schema and tests first

Shared 📜️csg-solids.json defines the supported types, strict arities, standard availability, positive lengths/radius bounds and exact tangent endpoint sampling. Language-neutral csg-cases.json and three source fixtures isolate each feature. All three permanent source tests failed with their respective unsupported representation/profile reason before implementation. Rounded rectangles use four quarter arcs with 18 segments each, original source 2D placement and exact tangent endpoints; maximal radius equality is legal and consecutive equal tangent endpoints are removed. Original counters/cancellation apply at every emitted point/triangle and Boolean traversal.

Actual permanent source suite: 17 groups passed with zero strict TypeScript diagnostics. Coverage includes both standards, radius equal to half width/height, invalid radius, wrong-dimensional profile placement, rotated profile, block dimensions/origin/placement, unsupported CSG root, preexisting/following shape items beside temporary Boolean index spaces, and original cancellation. The kernel's 29 groups remain green. Rust equivalent source/official/radius regressions are authored; four changed Rust source/test syntax trees have no grammar errors. Native execution remains gated before tests by repo cache per-directory case-sensitivity allocation access denied; no build edits, command bypass or native success claim.

## Unchanged official example

official-bath.ifc remains exactly 3290 bytes, SHA-256 afb2fc8f0702c3d90ab9395ea0897301c98499160bb69dcda88484fec0375fbd, buildingSMART Standards-builds commit a8664407b63d42651e48a78a252c76d09dd6274c, CC-BY-ND 4.0. The source FILE_NAME authorship and comments are unchanged. Provenance and bounded expected output are retained beside the permanent fixture.

Actual transformed-source projection of the official mapped sanitary terminal: Complete, no issues, product 225, representation 223, 380 vertices, 756 triangles, signed volume 548147011.3103607 source mm³. Work 819892 original projection ticks under the approved 8m default. One measured source projection took 357.68ms in Node REPL, excluding compile/typecheck and without a UI/native benchmark claim.

Independent existing Three EllipseCurve and ShapeUtils construct the four rounded arcs from the official dimensions (1800×600, radius200), triangulate their double-precision area and obtain host box volume minus 700-deep cutter volume. The result matches within 0.001mm³. Independent Three BoxGeometry and ExtrudeGeometry give the same sampled measure within their Float32 coordinate tolerance (relative1e-7). The final first-party mesh also passes strict closed edge incidence and internal mandatory solid admission. Manifold runtime remains blocked by the tool's Wasm embedder; its authored Boolean oracle suite is retained for repo execution when available.

## Scope

CSG roots currently support original BooleanResult trees and Block primitives only; other primitives and halfspaces remain truthful unsupported. This is sampled finite closed-solid projection, not full CSG/IFC conformance or exact analytic curved Boolean surfaces. Boolean kernel work/polygon/depth failures now classify the projection as Limited. Original caller/context fuel caps and atomic product publication remain intact. No resumable cursor is claimed.
