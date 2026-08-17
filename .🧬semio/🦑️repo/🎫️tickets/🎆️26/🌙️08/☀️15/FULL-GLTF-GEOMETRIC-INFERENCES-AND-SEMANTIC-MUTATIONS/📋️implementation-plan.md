# Full GLTF Geometric Inferences and Semantic Mutations

## Objective

Replace the GLTF 2.0 artifact's metadata-only bounds inference and partial generic mutation vocabulary with a deterministic, schema-first geometric-analysis and semantic-editing subsystem that works from decoded scene geometry, remains event-sourced, exposes quality and provenance, invalidates incrementally, round-trips through every schema representation, and is verified from analytic fixtures through the runtime boundary.

## Existing Defects to Remove

- `GltfBounds` reads accessor `min` and `max` metadata instead of decoded buffers.
- Bounds ignore node matrices, TRS, scene selection, hierarchy, instancing, skins, morph weights, primitive modes, sparse accessors, and missing/unresolved buffers.
- Empty and invalid geometry collapse to a fabricated zero-sized box without a validity state.
- No units, tolerances, provenance, approximation quality, diagnostics, configuration, per-part values, aggregate values, or cache dependencies are modeled.
- No surface, solid, mass, shape, curvature, thickness, concavity, clearance, contact, orientation, symmetry, roughness, or topology indicators exist.
- Mutations retain `NoMutation`, `SetSnapshot`, broad `Set*`, and generic index-based `Insert*`/`Remove*` operations; several GLTF collections remain reachable only through whole-document replacement.
- Mutation text/binary protocols duplicate manual dispatch and encode implementation-shaped operations rather than domain intent.
- Existing tests prove only determinism/default laws, local diff algebra, and codec round-trips; they do not prove geometric truth or runtime behavior.

## Locked Architecture

### Authored State and Derived State

- The GLTF snapshot remains the only persisted authored state.
- Indicators are derived and never written into `extras`, extensions, or the snapshot.
- Commands emit semantic mutations; mutations produce handcrafted sparse diffs and inverses; diffs are the only state transition material used by the builder/store.
- Inference reads the post-diff snapshot and uses content/dependency hashes. Cache entries are safe to discard and never affect results.

### Coordinate, Unit, and Scope Contract

- GLTF linear units are meters, matching the GLTF 2.0 convention.
- Areas use square meters, volumes cubic meters, curvature inverse meters, geometric second moments fifth-power meters, and normalized scores are dimensionless.
- Default scope is the default scene when present; otherwise all scenes; when no scene exists, all root nodes; unreferenced meshes are reported but excluded unless configuration requests them.
- Each node instance contributes separately to scene aggregates. Shared mesh resources are decoded once and transformed per instance.
- Matrix storage is GLTF column-major. A node matrix takes precedence for analysis when both matrix and TRS are illegally present; a diagnostic records the violation.
- TRS composition is `T * R * S`; quaternions are normalized deterministically or rejected when near zero.
- Negative determinant transforms flip orientation consistently before signed-volume and normal-sensitive computations.
- Skins and morph targets are modeled explicitly as unavailable until their evaluated pose inputs are present; static base geometry is never silently presented as an evaluated animated result.

### Precision and Determinism

- All analysis math uses `f64`, including source values widened from GLTF component types.
- Configuration carries relative and absolute length tolerances, angular tolerance, contact tolerance, sharp-feature threshold, raster/voxel resolution, symmetry samples, thickness samples, and work budget.
- Default effective length epsilon is `max(absolute_length_tolerance, relative_length_tolerance * characteristic_length)`.
- Stable ordering is resource index, node-instance path, primitive index, triangle index, vertex index. Hash maps are never allowed to determine public ordering.
- Accumulations use compensated summation where cancellation materially affects area, volume, centroid, covariance, or inertia.
- Every approximated indicator returns its method, resolution/sample count, error bound when available, and deterministic seed derived from the input dependency hash.

### Result and Quality Model

- A value is wrapped in a typed measurement containing value, unit, availability, quality, method, confidence/error metadata, and dependency provenance.
- Availability distinguishes `available`, `notApplicable`, `unavailableInput`, `invalidTopology`, `budgetExceeded`, and `unsupportedFeature`.
- Quality distinguishes `exact`, `discrete`, `boundedEstimate`, and `sampledEstimate`.
- Diagnostics contain stable code, severity, scene/node/mesh/primitive/accessor path, and concise parameters; prose localization occurs outside the inference schema.
- Aggregate results include global indicators, per connected part, per node instance, pairwise contact/clearance records, and input/topology counters.

## Canonical Normalized Geometry

The inference pipeline builds an internal immutable view, not persisted API:

1. Select scene roots and validate hierarchy cycles and out-of-range references.
2. Compose world transforms and enumerate node instances with stable instance paths.
3. Decode `POSITION`, optional `NORMAL`, and index accessors, including sparse values, byte offsets, strides, normalized integer values, and unresolved-buffer errors.
4. Expand GLTF TRIANGLES, TRIANGLE_STRIP, and TRIANGLE_FAN into oriented triangles. Points and lines contribute only counts/orientation where meaningful and are excluded from surface/solid metrics with diagnostics.
5. Apply world transforms to positions and inverse-transpose transforms to normals.
6. Remove non-finite positions, reject invalid indices, classify zero-area triangles, and preserve counters for every exclusion.
7. Weld topology vertices by tolerance through deterministic spatial buckets while retaining source-to-welded mappings.
8. Build edge incidence, vertex-face incidence, triangle adjacency, boundary loops, connected components, orientation propagation, and manifold/orientability classifications.
9. Build per-component AABBs, PCA frames, deterministic BVHs/spatial grids, and content hashes reused by indicator families.

## Indicator Definitions and Algorithms

### Size and Projection

- Overall size: AABB diagonal and maximum pairwise support extent.
- Bounding-box dimensions: world AABB and PCA-oriented bounding box dimensions, center, axes, and volume.
- Characteristic length: cube root of enclosed volume when valid, otherwise square root of surface area, otherwise AABB diagonal.
- Footprint/projected area: union area projected to XY/XZ/YZ and configured plane; exact triangle projection sum for non-overlapping monotone cases, deterministic raster union otherwise.

### Areas and Volumes

- Surface/total area: compensated sum of triangle areas.
- Exposed area: surface area not classified as shared/contact or buried by overlap, estimated by deterministic surface sampling with occlusion queries.
- Contact/shared area: coplanar opposing surface overlap within contact tolerance, with sampled fallback and quality marker.
- Enclosed volume: signed tetrahedral integral per closed orientable component, absolute after consistent orientation.
- Material volume: enclosed volume times material occupancy when an explicit occupancy source exists; otherwise equal to enclosed volume for watertight shell-as-solid interpretation with provenance.
- Void volume: convex/envelope volume minus material volume only when a declared envelope method is selected; never inferred ambiguously.
- Interference/overlap volume: exact zero for disjoint BVHs; otherwise deterministic adaptive voxel estimate with bounds.

### Compactness and Proportion

- Surface-to-volume ratio: `A / V` for valid nonzero enclosed volume.
- Sphericity: `pi^(1/3) * (6V)^(2/3) / A`.
- Compactness index: `36*pi*V^2/A^3`.
- Hull fill ratio: `V / V_hull`.
- Aspect ratios: sorted OBB/PCA dimensions `major:middle:minor`.
- Slenderness: `major / sqrt(middle * minor)`.
- Flatness and elongation: eigenvalue-derived normalized ratios with degeneracy classification.

### Mass Distribution

- Surface centroid and covariance are available for any nonempty triangle set.
- Solid centroid and inertia use closed-polyhedron tetrahedral integrals at uniform unit density.
- Principal axes/eigenvalues come from a deterministic symmetric 3x3 Jacobi eigensolver with sign and tie-breaking conventions.
- Results distinguish surface-weighted and solid-weighted measures.

### Curvature and Shape Complexity

- Vertex Gaussian curvature uses angle defect divided by mixed local area, boundary-aware.
- Mean curvature uses the cotangent Laplace-Beltrami vector.
- Statistics include area-weighted min/max/mean/RMS/quantiles and a fixed-edge histogram.
- Sharp-feature proportion is edge length above the configured dihedral threshold divided by total eligible edge length.
- Complexity also reports normal entropy and curvature entropy.

### Thickness

- Thickness samples cast inward and outward rays along robust vertex/face normals against nonincident triangles.
- Mean, minimum, maximum, standard deviation, median, lower quantiles, sample coverage, and censored misses are reported.
- Closed convex analytic fixtures establish expected convergence; open sheets return not-applicable or one-sided thickness rather than zero.

### Concavity and Convexity

- A deterministic 3D convex hull is built from unique welded positions, with coplanar/collinear degeneracy states.
- Convex hull gap is `V_hull - V`; concavity index is the normalized gap.
- Re-entrant area is surface area incident to locally concave dihedral edges; re-entrant volume uses hull gap with its explicit method.

### Clearance, Contact, and Connectivity

- BVH-pruned triangle-triangle distance yields minimum part-pair clearance and distribution samples.
- Contact classification uses distance, opposing normals, and projected overlap.
- Contacts produce an undirected graph with stable part ids, contact kind, area, clearance, and overlap estimate.
- Aggregate adjacency reports contact count, degree distribution, isolated parts, and connected components.

### Orientation

- Main-axis direction is the stable first PCA axis.
- Face-normal distribution is area-weighted over an equal-area spherical binning.
- Orientation consistency reports orientable components, required flips, inconsistent/non-manifold edge counts, and cross-part principal-axis dispersion.

### Symmetry and Regularity

- Reflection candidates are the three centroidal principal planes plus configured planes.
- Rotational candidates are principal axes and detected repeated angular signatures.
- Scores use symmetric surface-distance matching normalized by characteristic length and report sample coverage/error.
- Repetition/modularity clusters per-part invariant descriptors and reports repeated-part area/volume/count ratios.

### Roughness and Deviation

- A deterministic cotangent-Laplacian smoothing reference is computed without mutating source geometry.
- Deviation reports area-weighted point-to-reference distances, RMS/quantiles/max, normal variation, and multi-scale surface waviness.
- Ideal-geometry deviation is unavailable unless an explicit ideal primitive/model is provided; no shape is guessed silently.

### Topology

- Counts include valid/source vertices, edges, faces, degenerate faces, boundary/non-manifold edges, and components.
- Boundary loops are traced in stable order.
- Euler characteristic is `V - E + F` per welded component.
- Genus for each closed orientable component is `(2 - chi) / 2`; with boundaries it is `(2 - b - chi) / 2` when valid.
- Number of holes/handles is the summed genus only for components where the formula applies; unavailable components retain reasons.

## Semantic Mutation Vocabulary

Whole-document replacement moves to the store reset lane and is removed from event history. `NoMutation` is removed. Mutation variants are verbs grouped by aggregate:

- Document: `ChangeAssetMetadata`, `ChooseScene`, `DeclareExtension`, `ForgetExtension`.
- Scene: `CreateScene`, `RenameScene`, `IncludeSceneRoot`, `ExcludeSceneRoot`, `DeleteScene`.
- Node hierarchy: `CreateNode`, `RenameNode`, `ReparentNode`, `DetachNode`, `DeleteNode`.
- Node geometry: `AttachMesh`, `DetachMesh`, `TranslateNode`, `RotateNode`, `ScaleNode`, `SetNodeTransform`, `ClearNodeTransform`.
- Mesh: `CreateMesh`, `RenameMesh`, `AddPrimitive`, `RemovePrimitive`, `AssignPrimitiveMaterial`, `ChangePrimitiveMode`, `DeleteMesh`.
- Geometry payload: `CreateGeometryBuffer`, `ReplaceAccessorValues`, `MoveVertices`, `TransformVertices`, `ReversePrimitiveWinding`, `RecalculateAccessorBounds`.
- Material: `CreateMaterial`, `RenameMaterial`, `ChangeBaseColor`, `ChangeMetallicRoughness`, `ChangeAlpha`, `MakeDoubleSided`, `DeleteMaterial`.
- Animation and remaining GLTF collections receive their own intent verbs so no collection requires snapshot replacement.

Each mutation owns a typed mutation/diff/inverse triad, validates references before producing a diff, updates shifted references atomically when indexed collections change, and declares the exact inference dependency regions it invalidates. Invalid commands produce diagnostics and no event; they never clamp indices silently.

## Cache Dependency Graph

- `decoded-accessor/<index>` reads accessor, buffer view, buffer bytes, and sparse dependencies.
- `mesh-resource/<index>` reads primitive metadata and referenced decoded accessors.
- `node-instance/<path>` reads hierarchy, node transform, mesh resource, scene selection, and evaluated pose inputs.
- `topology/<component>` reads normalized triangles and tolerance configuration.
- Local families read the minimal derived node: bounds/projections, surface, solid properties, hull, curvature, thickness, BVH contacts, symmetry, roughness, topology.
- Aggregate indicators read only per-part results and pairwise relations.
- Diffs expose affected regions so metadata/material-only edits do not invalidate geometry, node transforms invalidate only dependent instances and aggregates, and accessor/buffer edits invalidate only referencing meshes and downstream instances.

## Schema and Integration Surface

- Rust is authoritative and uses internal interfaces for decoding, normalized geometry, spatial queries, eigensolver, hull, sampling, and measurements.
- Existing TypeScript, GraphQL, JSON Schema, protobuf, text grammar, and binary protocol leaves are updated to the same public result and mutation vocabulary.
- The artifact inference descriptor version increments and enumerates field-level read regions.
- GLTF declaration/runtime registry exposes the complete inference descriptor without side effects.
- Builder, store, event log, undo/redo, text op, binary op, and diff codecs accept the semantic operations and preserve mutation/diff/inverse laws.
- Existing inference UI renders groups from descriptors; English and German labels are supplied with no default-language assumption, keyboard/screen-reader semantics, units, quality badges, unavailable reasons, and per-part/contact drill-down.
- Executable verification commands are added to the existing ordered `launch.json` through existing `📜️script.ts` routing only if no suitable command already exists.

## Verification Matrix

### Analytic Fixtures in Existing Tests

- Empty, point, line, open triangle, square sheet.
- Unit cube and rectangular cuboid with exact bounds, area, volume, centroid, inertia, Euler characteristic, genus, and sharp-edge proportions.
- Tetrahedron, octahedron, cylinder/cone approximations, UV and icosphere convergence.
- Torus with genus one.
- Concave L/prism and hollow shell for hull gap/re-entrant behavior.
- Two separated cubes, touching cubes, overlapping cubes, rotated/scaled/mirrored instances, repeated modules.
- Non-manifold edge, inconsistent winding, duplicate vertices, degenerate triangles, invalid indices, sparse/interleaved/normalized accessors, unresolved buffers.
- Scene hierarchy, shared mesh instancing, default-scene selection, orphan nodes, transform matrix versus TRS, negative scale.

### Laws and Metamorphic Properties

- Determinism, cache transparency, incremental/full equivalence, serialization round-trip.
- Translation invariance for scalar shape metrics; expected centroid/bounds translation.
- Rotation invariance for scalar metrics; equivariance for axes/orientation.
- Uniform-scale powers: length `s`, area `s^2`, volume `s^3`, geometric inertia `s^5`, curvature `1/s`.
- Tessellation invariance within tolerance and winding reversal behavior.
- Part permutation and GLTF resource reindexing invariance with stable semantic identities.
- Mutation diff law, inverse law, absorb law, invalid-command no-op law, reference-integrity law, and exact cache invalidation law.

### End-to-End Gates

1. Focused Rust unit tests for GLTF inference, mutations, diff, IO, and example fixtures.
2. `bun nx` project targets for stdio check/test and generated/schema validation.
3. Workspace policy/verify gates with unrelated failures separated by path and reproduced against current shared-tree state.
4. Runtime import of real `.gltf` and `.glb`, inference request, semantic mutation, incremental reinference, undo, redo, export, reimport, and equality checks.
5. `[DEBUG]` console evidence for cache hit/miss and runtime values during verification, removed from production paths after capture.
6. Performance measurements for 10k, 100k, and 1M triangles, cold versus cached and transform-only edits, with bounded-memory assertions.

## Maximum-Parallel Workforce Plan

The execution environment supports one orchestrator plus three simultaneous workers. Every wave therefore fills all three worker slots; the orchestrator performs reconciliation, shared-file edits, gate runs, and assignment of the next wave. Reports and logs stay in this ticket.

### Wave 0 — Discovery and Contract Freeze (parallel now)

- W0-A: current GLTF schema/inference/mutation inventory and replacement contracts.
- W0-B: algorithm/internal-library inventory and mathematical definitions/test oracles.
- W0-C: runtime/generated/UI/test integration map and verification matrix.
- Orchestrator: baseline, ticket, exhaustive plan, collision map, acceptance gates.

### Wave 1 — Foundations

- W1-A: public measurement/config/diagnostic/result schema and representation leaves.
- W1-B: accessor decoding normalization, scene traversal, transforms, primitive expansion.
- W1-C: analytic fixture builders and baseline truth table in existing tests.
- Orchestrator: glue/descriptor reconciliation and focused compile gate.

### Wave 2 — Core Geometry

- W2-A: topology/welding/components/orientation/boundary/Euler/genus.
- W2-B: bounds/PCA/projections/proportions/centroids/inertia.
- W2-C: surface/volume/compactness and analytic exactness tests.
- Orchestrator: cross-family aggregate model and metamorphic gates.

### Wave 3 — Advanced Shape

- W3-A: convex hull/concavity/re-entrant metrics.
- W3-B: discrete curvature/sharp features/normal distributions.
- W3-C: thickness/roughness/waviness/deviation.
- Orchestrator: deterministic sampling/error contract and performance probes.

### Wave 4 — Multi-Part Relations

- W4-A: BVH/spatial indexing and clearance distributions.
- W4-B: contact/shared/exposed area and overlap volume estimates.
- W4-C: adjacency graph/components/degrees and cross-part orientation.
- Orchestrator: relation aggregation and two-part analytic scenarios.

### Wave 5 — Symmetry and Modularity

- W5-A: reflection symmetry.
- W5-B: rotational symmetry.
- W5-C: repetition/modularity clustering.
- Orchestrator: invariance tests and budget/quality fallback review.

### Wave 6 — Semantic Mutations

- W6-A: document/scene/node hierarchy and transform mutation triads.
- W6-B: mesh/primitive/accessor/buffer geometry mutation triads.
- W6-C: material/animation/remaining collection mutation triads and reference reindexing.
- Orchestrator: remove generic/snapshot/no-op variants and reconcile root dispatch.

### Wave 7 — Codecs and Incrementality

- W7-A: semantic mutation text grammar/printer/parser.
- W7-B: semantic mutation binary protocol/codec and diff codec compatibility within the new schema only.
- W7-C: dependency-region declarations, merkle cache nodes, invalidation tests.
- Orchestrator: builder/store/undo/redo laws and descriptor versioning.

### Wave 8 — Generated and User Surfaces

- W8-A: TypeScript/GraphQL/JSON Schema/protobuf parity.
- W8-B: inference UI grouping, quality/provenance presentation, accessibility, English/German localization.
- W8-C: runtime/WIT/registry wiring and real-file scenario harness in existing files.
- Orchestrator: launch command ordering and cross-platform zero-touch review.

### Wave 9 — Exhaustive Verification

- W9-A: focused and full stdio Rust gates plus analytic/property suites.
- W9-B: nx policy/schema/workspace gates and unrelated-failure attribution.
- W9-C: runtime scenarios, cache traces, performance and memory measurements.
- Orchestrator: independently inspect all diffs, remove debug code, write final report, close ticket through repo MCP.

## Completion Criteria

- Every requested indicator exists in the authoritative schema and is either computed with declared quality or unavailable with a typed reason; no fabricated zeros or silent skips.
- Scene transforms, hierarchy, instancing, sparse/interleaved accessors, primitive modes, degeneracy, open/non-manifold topology, and multi-part relations are covered.
- No `NoMutation`, `SetSnapshot`, generic broad `Set*`, or collection-only snapshot escape hatch remains in GLTF mutation history.
- Every semantic mutation has handcrafted diff and inverse, preserves reference integrity, and invalidates only declared inference dependencies.
- All schema representations and runtime descriptors agree.
- Analytic, law, metamorphic, real-file, runtime, cache, and performance evidence is captured in the ticket.
- Focused gates pass; any workspace failures are reproduced and shown unrelated to touched paths.
- Ticket summary names every changed file and the repo MCP closes the ticket.
