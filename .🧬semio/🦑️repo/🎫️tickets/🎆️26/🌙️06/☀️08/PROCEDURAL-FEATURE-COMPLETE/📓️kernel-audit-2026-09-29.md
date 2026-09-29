# Procedural 3D Kernel Audit

Read-only source audit on 2026-09-29. No tests or runtime sessions were run; observations below are implementation evidence, not passing-test claims.

## Ownership and Implementation Seams

The live procedural artifact uses contributed flow extension actors, rather than the TypeScript spatial-kernel wrapper. Its production geometry operator owner is `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`; `generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` installs linked actors only in lib tests. The geometry bridge lives in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs`, whose kernel is the stdio semio brep schema engine.

`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts` is the first-party TypeScript spatial runtime. Its neighboring brepjs file explicitly identifies itself as a differential test oracle. Extending spatial-kernel alone will not add procedural graph widgets. There is no need to introduce a new geometry runtime dependency.

## Existing Surface

The brep extension has a test-maintained node-to-kernel mapping covering primitives (box, sphere, cylinder, cone, torus, convex hull), curves (line/circle/arc/ellipse/polyline/rectangle/polygon/interpolate/approximate/helix), surfaces (plane, planar face, NURBS grid, Coons, offset, thicken), extrusion/revolve/loft/sweep/pipe/helical sweep, booleans, transforms/patterns, edge blends/chamfers/shell/draft/offset/defeature, intersections, evaluation, measurements, topology and STEP/STL/OBJ/DWG interchange.

`operation_quality_tags_match_the_kernel_contract` and `every_kernel_operation_is_either_a_node_or_explicitly_unexposed` in the extension unit tests already pin coverage and honest quality summaries. The contract is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🔖️contract/🦀️.rs`. It marks fitting and Coons/NURBS grid as approximate, generic curved-path sweeps as numerical within tolerance, and mesh conversions as mesh derived. Claiming arbitrary analytic geometry is therefore too strong even though every kernel method has a disposition.

The extension mesh module registers 29 `brep.mesh.*` operators: construct, box/plane/sphere/cylinder/cone, BRep conversions, global transforms, vertex and component transforms, loop cut, knife cut, extrude, inset, subdivide, flip, delete faces, triangulate, weld, orient, fill holes, analyze, OBJ and JSON export. Procedural editor commands already include edit-mesh-selection and knife-mesh-selection, with command schemas and unit tests. The mesh workbench example already exists.

## Concrete Gaps and Limits

The halfedge engine is `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs`. Several public methods are not exposed as mesh graph nodes: proportional movement, grid snapping, bevel, merge/dissolve, mirror, decimate, shading, seam marking and unwrap. They should not be exposed merely because methods exist. In particular `bevel_edges` (line 773) ignores `_segments`, adds two positions per edge, never uses those positions in a face, and rebuilds unchanged faces. This is a concrete incomplete implementation. A language-neutral bevel fixture must assert changed connectivity, absence of loose vertices, and the requested segment count before exposing bevel.

Mesh JSON contract accepts only vertices and polygon faces. It bounds input at 16 MB, 100000 vertices/faces, 600000 polygon corners, finite f32 coordinates, and forbids repeated vertex indices. TypeScript parser and geometric analysis exist next to the Rust extension, with shared schema and fixtures. Current portable data intentionally cannot persist UVs, shading/materials, normals or explicit seams. Exposing the corresponding kernel operations needs schema-first portable storage, tests, and preservation across encode/decode, not only menu entries.

The TypeScript spatial wrapper samples model wires to polylines before first-party kernel operations. Its `surface.loft`/`networkSrf` (around line 640) invokes loft but discards the returned handle; `surface.sweep1`/`sweep2` similarly discards sweep and persists only a face record with profile/rail wire IDs. This is a separate persistence seam where arbitrary surface shape may be lost. Do not route procedural surface output through this wrapper without a stored analytic representation and restart/export fixture.

## Recommended Verification Sequence

1. Exercise the served contributed extension route, since linked lib tests deliberately shadow contributed stubs. Existing in-process bridge tests verify plugin actor addressing and budgeted tessellation, but are not a browser runtime confirmation.
2. Reuse `generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry` and extension registry coverage laws for known geometry pipelines. Confirm a newly authored graph evaluates, tessellates, exports, reopens, and retains its editable controls.
3. For expensive booleans, reuse resumable BooleanOperatorJob progress/cancellation in the extension. Tessellation already has step envelopes, handle retention and cancellation. Audit synchronous mesh operations against the expensive-operation requirement before raising input limits.
4. Add shared geometry fixtures for any newly exposed mesh method and compare actual geometry against an existing third-party oracle in test-only code; counts alone will miss ineffective bevel and malformed topology.
5. Keep quality summaries visible in registry metadata and verify catalog discovery uses contributed manifests, rather than adding a parallel hand-maintained widget list.

## Follow-Up Audit of Unexposed Mesh Methods

- `bevel_edges` is ineffective topology editing: new vertex IDs are discarded, face loops unchanged, segment count ignored. Highest-priority core fix.
- `dissolve_vertices` removes all incident faces, leaving holes; it implements deletion, not dissolution. Needs a shared neighboring-face/edge preservation fixture.
- `merge_vertices` retains all old positions, remaps faces without removing repeated adjacent indices, and filters only by unique-count >=3. ByDistance compares only against the first selected vertex, not pairwise connected clusters. Test a quad collapsing to a triangle and a chain of selected vertices before exposure.
- `dissolve_edges` does merge opposite face loops but silently ignores invalid edge IDs and applies collinear cleanup across all faces, including unselected geometry. Pin selection isolation and invalid-selection behavior.
- `mirror` duplicates/reverses geometry then does an O(n²) all-vertex scan. It ignores merge errors and calls the above uncompacted merger. Use seam-specific weld and bounded/cancellable processing before authorizing 100000-vertex inputs.
- `decimate` performs shortest-edge collapse and now compacts vertices at the end, so it is not a no-op. It can produce repeated-index face loops (same merge issue), has no collapse manifold/inversion guard, and synchronously rescans every edge per collapse. The ratio is vertex-based and clamped to 0.1..1.0. A closed cube fixture needs topology, normals, area/volume and target-complexity assertions.
- `move_vertices_proportional` changes only the supplied vertices with linear pivot-distance falloff; it does not discover nearby unselected vertices. Duplicate selected IDs apply displacement repeatedly. Radius clamps to 1e-6 rather than rejects invalid values. Expose only after defining selection/falloff semantics.
- `snap_vertices_to_grid` mutates positions but does not recompute normals; it checks grid <=0 but accepts NaN and does not reject duplicates or empty selection. Geometry/normal consistency needs coverage.
- `set_shading` sets flags but does not recompute vertex normals; `recompute_normals` normalizes its accumulating sum after every face, producing face-order-dependent smooth normals and includes flat-face normals in the same shared storage. A face permutation test is valuable.
- `unwrap_uv` has a concrete seam flaw: `pack_island_uvs` returns one UV per global vertex ID. Two islands sharing a seam vertex overwrite each other's packed value, then all halfedges read that same value. Further, its v solve pins both boundary values to zero, so inspect rank/area behavior before declaring valid 2D charts. Store UVs per face corner/halfedge and test distinct UVs across a seam plus nonzero UV triangle area.

These are source findings only; no requested feature was changed and no passing result is claimed. Existing framework modeling fixtures/tests are the correct core lane, while extension schemas/fixtures/unit tests are the graph exposure lane.
