# Faceted Shell Validation Reuse Research

The bounded source search found existing first-party geometry predicates, but no ready cooperative TypeScript 3D shell admission cursor. Do not duplicate these algorithms or treat a synchronous tick callback as an asynchronous original-host continuation.

The IFC first-party `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts` contains private `selfIntersections` (53–66), `shellWinding` (67–70), `validate` (72–80), and `admit`. `validate` checks oriented edge incidence, connected vertex links, nonconforming edge/vertex contacts, triangle self-intersections and component winding/nesting. `shellWinding` sums solid angles over each component, checks closeness to zero or 4π, and requires orientation parity by nesting depth. Public entry points are `booleanSolid`, `subtractSolid` and `subtractHalfSpace`; admission is not separately exported. Rust's adjacent implementation has the corresponding private predicates and public `boolean_solid` / `subtract_solid`.

The existing TS Boolean core uses a private synchronous Control, fixed normalized EPS=1e−9, normalization/welding, sorts, maps, recursion and temporary whole arrays. Calling subtraction with an empty operand merely to trigger validation would couple this feature to Boolean semantics, duplicate work, and would not satisfy async bounded traversal. A shared schema-neutral validation generator, extracted beside this core with a synchronous drain, would be the clean reuse route if this scope needs those predicates. Preserve exact source-ID admission separately: the Boolean core's coordinate welding cannot replace source identity topology checks silently.

Framework `🧰️framework/🔨️modules/📐️geometry/💥️collision/🦀️.rs` supplies public `Aabb`, `Bvh::build`, `triangle_contact`, `triangles_intersect`, `triangle_distance` and `clash`. It has private parity `inside` and containment, cancellation callbacks during traversal, but synchronous collecting/sorting BVH construction. Fixed EPS=1e−9, PLANE_EPS=1e−10 and degenerate area thresholds mean these are numerical predicates, not exact arithmetic certificates. `clash` deliberately distinguishes penetration from contact/clearance; absence of a hard clash is insufficient to certify disjoint shells, since forbidden touching must also be detected. This Rust API is not an existing direct TS cooperative interface.

Minimum additional laws (separate from the frozen18 inputs):

| Shape | Why edge counts alone are insufficient | Required outcome |
|---|---|---|
| cavity orientation reversed outward | each closed shell still has twice/opposing edges | reject incorrect inner-shell orientation |
| closed cavity translated outside outer | each component independently manifold | reject absent strict containment |
| cavity crossing outer with centre inside | centre parity passes | reject surface intersection / any outside portion |
| two intersecting or touching cavities | each independently manifold | reject cavity contacts/intersection |
| nested cavity within another cavity | orientation by generic parity may imply a material island, but an IFC Voids roster must meet its own disjointness contract | reject unsupported/non-disjoint void roster |
| one face removed | vertex incidence can remain ≥3 in richer meshes | reject non-two edge incidence |
| two disjoint boxes in a single ClosedShell | edge counts and each local link pass | reject disconnected shell |
| two closed boxes sharing only one source vertex | edge counts can pass | reject disconnected vertex link / pinched vertex |
| closed manifold with crossing nonadjacent triangles | edge and vertex-link checks pass | reject geometric self-intersection |

Existing Three182 is used only behind ticket test input `faceted-containment-point-oracle.ts`. Four independent finite BoxGeometry/Raycaster parity observations agree: an inner point is inside; a translated-outside centre is outside; a crossing cavity centre can be inside while its exterior corner is outside. This demonstrates the one-point-only containment trap. Generated `faceted-containment-point-oracle.json` is test-only corroboration, not general robust containment or geometry authority. The prior void certificate separately establishes the particular authored cavity's closed edges, inward orientation, strict box containment and 5.5 volume.

No production edits, frozen18 input changes, runtime dependency, native execution or full-solid validity claim were made. Original basic/Basin topology facts remain in the separate original-roster audit.
