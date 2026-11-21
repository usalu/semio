# Original line and conic contacts

The registered `brep.intersect.curveCurve` operation accepts original line, circle and ellipse curve handles and analytic edge handles. A standalone line remains an unbounded supporting line. An edge uses its stored parameter range, including for periodic conics whose trim crosses the angular seam. Coplanar queries solve signed distance to the supporting line in the conic frame. A transverse query checks only the plane crossing. World-space distance verifies each candidate against the bounded line and conic.

The query returns at most two contacts in line-parameter order regardless of input order. The existing `wire` output publishes a vertex for one contact and a segment with two exact contact endpoints for two. A miss refuses without publishing. The source quotes its live registry lookup and, for an edge, the original edge and supporting curve arena depths. Both original intersection operations use the retained native vertex and segment publication path.

The detached untrimmed solver shares these analytic equations. General curve pairs and rational NURBS arcs remain outside the fixed original line and conic family.

## Verification

- Compiled the actual intersection crate against cached native geometry libraries. The focused witness passed all 15 portable cases and checked the returned parameter on each original curve. It also compared both math entry points on untrimmed inputs.
- Three.js reproduced the portable points independently by transforming the ellipse to unit coordinates and intersecting rays with a unit sphere in the conic plane. The witness covered secants, exact tangency, order reversal, misses, rotated planes, oblique transverse cases, both operand trims and periodic seam crossing.
- Added a registered original operator regression law that constructs public curves and imports trimmed edges, validates misses and the cancellation of a partially allocated two-contact publication, then checks native vertex or wire topology and funded preview coordinates. The full Flow BRep test crate was not built under the disk limit.
- Rust AST parsing and scoped whitespace checks passed. The repository source closure reader measured 3,982 Engine lines, 3,929 native registry lines, 2,567 intersection lines, and 2,072 lines with 255,935 source bytes for the original query test target.

The scripts are `🐍️line-conic-source-check.py` and `🧮️line-conic-oracle.mjs` in this ticket.
