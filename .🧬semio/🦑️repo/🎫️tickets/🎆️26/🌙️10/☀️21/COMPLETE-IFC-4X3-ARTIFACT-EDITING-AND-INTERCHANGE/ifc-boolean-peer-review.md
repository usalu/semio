# First-Party Boolean Peer Review

Actual-source read-only audit of IFC/📐️geometry/🧊️boolean/🟦️.ts. Ticket input ifc-boolean-peer-probes.mjs reproducibly strips current source into owned generated module and runs15small probes. No production edits and no native execution. Current exported Boolean API is subtractSolid only; no union/intersection execution or completeness claim is possible from this module.

## Proven Guard Inconsistency

At review time, subtractSolid(unit cube[0,2]^3,disjoint cube[4,5]^3,{maxPolygons:1}) returns12triangles. Same cube minus empty returns12triangles. Identical limit with contained cutter rejects boolean-polygon-limit. Thus empty/disjoint early returns bypass the cap enforced by BSP/output paths. If maxPolygons is the module output/intermediate capacity limit, apply consistent fast-path admission/emission checks. If intentionally only a BSP intermediate limit, make that narrower meaning explicit. Fast-path clone maps also have no per-item tick or final original control callback, unlike normal output. High was notified with exact reproductions.

## Executed Rejection Evidence

All expected malformed operands rejected: entire cube inverted => invalid-shell-winding; zero-thickness cube => degenerate-triangle; duplicated coincident closed cube sheets => operand-not-closed-manifold; one reversed triangle => operand-not-closed-manifold; deformed cube vertex6=(-1,-1,-1), creating crossing faces => self-intersecting-solid; two cubes joined on a complete face as an unclean two-shell mesh => operand-not-closed-manifold. Existing permanent source tests additionally cover overlapping shells, wrongly oriented nested shells, pinched vertex links and open operands. No accepted invalid operand was found in these concrete probes.

## Executed Valid Geometry Evidence

Separate edge/point/full-face tangent cutters retain cube volume8 and12triangles. Disconnected closed outward components accept. Inner cavity retains volume7 with inward shell and survives disjoint subtraction. Cavity minus identical cavity returns a valid empty mesh. A crosscut through two disconnected components produces volume4.499999999999998, matching independent box arithmetic4+.5. These probes validate expected operation outcomes; no general third-party Boolean oracle was executed here. Existing manifold WASM embedder block remains separate from native build constraints.

## Source Acceptance Review

Current validator explicitly checks finite coordinates, indices, degenerate faces, opposite edge incidence, single-cycle vertex links, interior edge vertices, triangle intersections, connected-shell solid-angle nesting and shell orientation. Both operands and nonempty generated output receive it. This materially closes the earlier edge-count-only false-positive class. BSP operation is atomic to caller: validation/control exceptions throw before returning output. Control ticks cover splits, traversals, most conforming triangulation work and acceptance sweeps; callers still need to preserve atomic host publication and original-context cancellation outside this pure mesh function.

No claim of exhaustive proof, native compilation, union/intersection support or actual manifold-3d execution is made.

## Fresh Guard Resolution

High fixed operand maxPolygons admission plus per-item/final controlled fast-path copying and finite world-output checks. Re-ran actual latest source with the retained15probe harness: both previously bypassing maxPolygons cases now reject boolean-polygon-limit; all prior valid/malformed results remain correct. The proven inconsistency above is resolved in fresh source. Added permanent final-cancel regression reported by author; this audit rerun checks the neutral probes directly.
