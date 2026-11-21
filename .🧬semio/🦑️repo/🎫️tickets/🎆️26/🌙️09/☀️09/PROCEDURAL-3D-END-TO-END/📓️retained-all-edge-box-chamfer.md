# Funded All-Edge Chamfer for Certified Boxes

The registered `brep.solid.chamfer` operator now admits an original retained job for certified rectangular boxes. The fixed planar boundary has six inset stock faces, twelve equal-distance bevel quadrilaterals, and eight triangular corner caps. The source owner checks the live primitive and validates that the distance leaves positive straight-side clearance and representable world coordinates.

The complete incidence table is checked at Rust compile time. Each edge has two opposite face uses. The runtime boundary certificate stores only its twelve axis coordinates. The existing solid transaction inserts vertices, curves, trims, faces, shells and the solid under grants, exposes progress and rolls back unpublished topology on cancellation. The live handle is published only at completion.

The Flow cold chamfer implementation closes each fully blended trihedral vertex with a planar triangle through the three face-tangency points. The retained implementation follows that geometry. The separate procedural inference fixture for a 2×2×2 box with a 0.25 distance describes twelve extended miter planes, which meet differently at the corners. The new Flow fixture uses the triangular-corner contract.

Three neutral rows cover a uniform cube, translated rectangular stock, and a near-limit chamfer. Manifold-3D constructed each result independently using the stock, edge and corner half spaces, matching the area, volume, bounds, and genus in 12 assertions. The disk-light Rust suite passed all eight tests and checks the actual fixed topology, planar pcurves, outward face winding, reciprocal incidence, exact measures, bounds, and invalid numeric and source cases.

The linked registered Flow test checks zero-item admission, one-item progress, unpublished cancellation, refusal of degenerate distances, a single live commit, 24 vertex and 48 edge labels, 26 face labels, and both preview and exact measurements. It awaits the shared Cargo lane. The variable and asymmetric feature operators are outside this retained box certificate.

Nightly Rust syntax, whitespace, and path-length checks passed. Source closures contain 3,995 active Construction lines, 3,999 Engine, 3,998 Native Registry, and 3,691 Solid Transaction.
