# Retained Rotated Box Cut and Fuse

The retained Boolean route now cuts and fuses rectangular stock with one shared principal axis and a common extrusion span. A bounded two-dimensional arrangement splits each source side at its proper crossings, selects directed material edges from the requested set operation and assembles closed cycles. It uses space for eight original corners and up to eight crossings. The signed loops preserve outer contours, holes, and up to four separate material pieces.

A connected simple result publishes the existing planar prism with its complete nonconvex profile. A strictly contained tool cut publishes one perforated prism with 16 vertices, 24 reciprocal shared edges, eight side walls and two planar caps. Each cap has an outer ring and a correctly oriented inner ring. The native boundary contains no internal cell faces. Both paths keep their exact authored support geometry and use the existing one-solid transaction, paid publication, and planar preview and measure paths. Classification measures distance to the finite caps and material contours, including the concave crossings and complete hole wall.

The subsequent [compound publication wave](📓️disconnected-rotated-box-publication.md) covers the retained multi-piece result. The same-span admission also refuses uncertain footprint contacts and near-degenerate spans. The previous non-overlap, exact containment, and shared-axis Intersect cases continue through their existing recipes.

## Verification

- The neutral fixture covers a partial notch, a through-hole, and connected union and intersection outcomes for cuts yielding two and four islands. Manifold independently matched status, volume, surface area, connected component count and genus for all 12 operations in 60 assertions.
- The compact Rust suite compiled the source arrangement, Solid Boolean dispatch, native prism recipes, and exact point-classification logic. It passed 21 tests covering closed coedge incidence, planar pcurves, outward support loops, complete inner cap loops, exact area and volume, empty and ambiguous inputs, and connected and disconnected results. The shared-axis Intersect regression suite separately passed all 22 tests.
- The registered `original-brep-booleans` integration exercises both operators with preview and measures, exact single live publication, and disconnected publication after rollback. The linked target remains pending under the host's constrained managed build capacity. Canonical source measurement reported 3,999 active engine lines, 3,994 native registry, 3,952 solid transaction, and 136,731 active bytes for the Boolean fixture target. Rust source parsing and scoped whitespace checks passed.

## BRep Query Source Admission

The six original OBJ, STL, and STEP cases now live in the registered `original-brep-exchange` integration target. The 24 other cases remain in `original-brep-queries`. The tests and referenced fixtures were preserved. The canonical Rust source reader now reports 188,825 active bytes for queries and 103,137 for exchange, within the 262,144-byte ingress ceiling. Both integration source roots passed Rust AST parsing.
