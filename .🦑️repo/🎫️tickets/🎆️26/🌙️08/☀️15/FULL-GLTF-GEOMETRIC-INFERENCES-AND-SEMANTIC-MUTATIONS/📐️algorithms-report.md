# Computational Geometry and Verification Plan

## Outcome

Replace the current metadata-only `bounds` fold with one deterministic, scene-aware geometric analysis pipeline. Decode the actual GLTF geometry once, form stable primitive occurrences, build reusable f64 topology and acceleration structures, and derive every requested indicator from those shared products. Never silently manufacture a value: every result carries applicability and quality (`exact`, `boundedApproximation`, `estimate`, or `undefined`) plus diagnostics and, where possible, an absolute error bound.

The implementation should be split between:

- a GLTF-specific adapter for accessor decoding, primitive expansion, scene traversal, morphing/skinning, identities and cache dependency bytes;
- a domain-neutral triangle-analysis kernel reused by GLTF and the existing Semio mesh/B-Rep artifacts;
- small indicator reducers over immutable kernel products;
- semantic GLTF mutations which preserve all index references and use copy-on-write for shared binary data.

The canonical inference scope is the static authored geometry of every scene, with the default scene identified separately. Results also expose uninstantiated mesh-asset analyses. An occurrence is a scene/node-path/mesh/primitive tuple, so one mesh instanced by three nodes contributes three world-space occurrences without decoding or analyzing its local geometry three times.

## Repository audit

### Current GLTF implementation and defects

- `.../gltf/.../💡️inferences/📦bounds/🦀️component.rs` trusts accessor `min`/`max`, folds every mesh regardless of scene reachability, remains in mesh-local coordinates, counts an accessor once per primitive reference, and has only determinism/default tests. It does not inspect actual bytes, indices, primitive mode, transforms or instances.
- `.../gltf/.../🚪️io/🦀️component.rs` already decodes dense, strided and sparse accessors into f64. It validates byte bounds through component reads. However it returns integer components unnormalized even when `normalized=true`; callers must not mistake the values for normalized normals/weights. External URI buffers intentionally remain unavailable and must yield `undefined`, not zero geometry.
- `GltfNode` models hierarchy, mesh, skin, matrix/TRS and weights. `GltfPrimitive` models attributes, indices, material and mode, but omits the standard `targets` morph-target array. Full default-pose analysis therefore requires restoring that field across snapshot/diff/mutation/codecs before morph metrics can be honest.
- The typed document preserves generic extensions but no typed geometry-compression adapters. `KHR_draco_mesh_compression`, `EXT_meshopt_compression` and GPU-instancing geometry must be decoded by registered extension interfaces or reported unsupported.
- Existing GLTF insert/remove mutations splice index-addressed arrays without remapping references. Inserting/removing nodes, meshes, accessors, materials or buffers can silently retarget valid references. Buffer views, skins, cameras, images, textures and samplers lack first-class mutation triads. Geometry mutations cannot be built on this behavior.

### Reusable internal implementations

- `🧰️framework/🔨️modules/🔺️mesh-engine/.../📦️glue.rs` has scene traversal, parent×local matrix composition, inverse-transpose normal transformation, default-scene selection, non-indexed fallback and correct triangle strip/fan expansion. Reuse the behavior, but not its f32/GLB/bin-0 limitations.
- `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️component.rs` has half-edge construction, polygon soup conversion, triangle tessellation, coincident-vertex welding, consistent face orientation, hole filling, normals and extensive topology tests. Generalize its immutable topology-building pieces to f64. Repair/edit operations must remain opt-in mutations; inference must not repair authored geometry.
- `✏️s/.../semio/.../✳️mesh/.../💡️inferences/📦aabb/🦀️component.rs` is the repository model for honest per-primitive `InferredField` dependency hashes and cache incrementality tests.
- `✏️s/.../semio/.../✳️brep/.../💡️inferences/📏mass-properties/🦀️component.rs` contains volume, area, center-of-mass, bounding-box, closest-point and solid-distance methods. `🌳bounding-volume` contains deterministic face/edge BVHs. Extract the triangle-generic math rather than depending on B-Rep arena types.
- `🧰️framework/🔨️modules/◻2d/🔀️booleans/🦀️component.rs` provides an internal interface over planar polygon booleans and can power projected-area unions and coplanar contact clipping.
- `🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️component.rs` provides basic vector/matrix/2D hull/polygon operations, but its render `Vec3`/`Mat4` are f32. Indicator accumulation must use new or extracted concise f64 primitives.
- `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs::EngineRep` defines the correct lifecycle: topology/BVH/derived working data are pure ephemeral representations, never authoritative hidden state.

## Semantic contract

### Analysis levels and identities

Return results at all levels, never one ambiguous document scalar:

1. `primitiveAsset`: mesh index + primitive index in local coordinates.
2. `meshAsset`: union/collection of its primitive assets in local coordinates.
3. `occurrence`: scene index + full node-index path + mesh index + primitive index in world coordinates.
4. `nodeOccurrence`: full path, aggregating primitives directly attached to that node.
5. `scene`: aggregating all reachable occurrences, including repeated instances.
6. `document`: one entry per scene, the designated default scene, and orphan mesh assets separately. Do not merge identical geometry across scenes into a fictitious world.
7. `relations`: unordered occurrence-pair keys in stable lexicographic order and a contact graph per scene.

Stable keys must be length-prefixed tuples or typed structs, never delimiter-concatenated strings. A path, not merely node index, is necessary for shared nodes and robust invalid-document diagnostics. Array order remains GLTF identity; a content digest is a cache key, not public identity.

### Geometry domains

Each result declares its effective dimension:

- 0D: points only;
- 1D: line modes;
- 2D: triangle surface, open or closed;
- 3D: one or more validated closed orientable shells;
- mixed: more than one nonempty domain.

Counts, bounds and PCA can use all selected vertices. Length uses line segments and optionally boundary/feature edges as named subresults. Surface indicators use triangles only. Solid/material/void indicators require validated 3D shells. Returning volume `0` for an open sheet is forbidden; it is `undefined(openBoundary)`.

### Static pose

The primary inference is deterministic static authored pose:

- traverse a scene without applying animations;
- apply mesh default morph weights overridden by node weights;
- if a skin is present, apply joint global matrices × inverse-bind matrices in this static node pose;
- apply the occurrence node world transform;
- report undeformed/base geometry separately when morph/skin data are present.

Animation-time analysis is an explicit parameterized query, not part of snapshot inference. It samples a named animation at an explicit f64 time and loop policy. Never choose wall-clock time.

### Units and density

GLTF has no universal physical length unit or density. Scalar outputs use `gltfUnit`, `gltfUnit²`, and `gltfUnit³`. “Mass distribution” means uniform unit volumetric density for valid solids and is named `unitDensityMassProperties`. A separate surface-lamina mass result at unit areal density is allowed for open surfaces. Material density must come from an explicit extension/interface; PBR materials are not physical density data.

### Quality wrapper

Every metric or coherent metric group returns:

```text
value | null
quality: exact | boundedApproximation | estimate | undefined
absoluteErrorBound | null
algorithmVersion
applicability diagnostics[]
sampleCount | null
```

“Exact” means exact for the decoded piecewise-linear mesh up to documented floating-point predicates, not exact for the unknown source CAD surface. Histograms include explicit bin edges, normalization and under/overflow counts.

## Numerical policy

All analysis geometry and accumulators use f64. Decode source f32 exactly into f64; do not round through the f32 mesh engine.

For one analysis scope define:

- `C = max(1, max absolute finite world coordinate)`;
- `L = AABB diagonal`, falling back to the largest nonzero edge/coordinate span, then `1` for a singleton;
- numerical linear tolerance `εn = max(128·f64::EPSILON·C, 1e-12·L)`;
- weld tolerance `εw = max(1024·f64::EPSILON·C, 1e-9·L)`;
- contact tolerance `εc = 1e-6·L` (a semantic analysis threshold, distinct from numerical tolerance);
- degenerate triangle threshold `2·area <= εw²` and negligible volume `<= εw³`;
- angular equality tolerance `εa = 1e-8 rad`;
- default sharp-feature threshold `30°`, returned alongside a full dihedral histogram so clients can reclassify.

All constants are versioned in a `GltfGeometryAnalysisPolicyV1`. Custom tolerances belong to a parameterized query. Auto tolerances scale uniformly with geometry and are included in cache dependency bytes. Non-finite coordinates/accessor values are excluded with exact index diagnostics; a metric becomes undefined if exclusion changes required topology.

Robust orientation/insphere/segment predicates use adaptive expansion arithmetic at ambiguous signs. Comparisons sort by total-order f64 bits after canonicalizing `-0` to `0`; NaNs never enter geometry. Reductions use a fixed order and compensated pairwise summation. Parallel execution may compute independent stable chunks but must merge in key order, making output bit-stable across thread counts.

## Canonicalization pipeline

### 1. Validate and decode

Validate all referenced indices, accessor shapes and legal component types before geometry work. POSITION must be VEC3/f32 under core GLTF unless a registered extension explicitly supplies it. Indices must be unsigned SCALAR. Check buffer-view ranges including final element stride, alignment, sparse index monotonicity/uniqueness/range, count agreement for attributes used by morphing/skinning, and all finite values.

Extend the existing decoder with:

- spec normalization for signed/unsigned integer attributes;
- a `decode_indices_u32` path that rejects fractional/negative values instead of `as usize` conversion;
- byte-range dependency reporting;
- registered compression/extension adapters behind an internal interface;
- zero-copy borrowed dense slices when aligned, with the same observable f64 result as the general path.

Accessor `min`/`max` are validation hints only: recompute decoded bounds and emit a mismatch diagnostic. Never use authored bounds for final metrics.

### 2. Expand primitive modes

With explicit indices or `0..POSITION.count` when absent:

- POINTS (0): every index is a point;
- LINES (1): disjoint pairs; ignore and diagnose an incomplete last index;
- LINE_LOOP (2): adjacent pairs plus last-to-first when at least two;
- LINE_STRIP (3): adjacent pairs;
- TRIANGLES (4/default): disjoint triples; diagnose incomplete tail;
- TRIANGLE_STRIP (5): triples from sliding windows, swapping the first two indices on odd windows;
- TRIANGLE_FAN (6): `(first, i, i+1)`.

GLTF has no primitive restart. Repeated indices and near-zero-area triangles remain counted as authored degenerates in diagnostics but are excluded from geometric integrals and topology faces. Preserve source-corner provenance through expansion.

### 3. Scene traversal and transforms

Use column-major GLTF matrices. Local transform is authored `matrix`, otherwise `T·R·S`; normalize a finite quaternion, diagnose and use identity rotation only for a near-zero quaternion. If both matrix and TRS are authored, preserve the snapshot but choose matrix precedence with `invalidMutuallyExclusiveTransform` diagnostic. World transform is parent-world × local.

Traverse every scene in scene-index/root-index/child-list order. Use a path-local visited set to detect cycles; skip only the cyclic edge. Diagnose out-of-range children, duplicate children, multiple parents and roots reached more than once. If there are no scenes, analyze uninstantiated assets only rather than inventing a scene. The “default scene” is `document.scene` when valid; do not silently use the first scene, though a convenience view may label the first scene separately.

Transform points by the full affine matrix. Transform authored normals/tangents by inverse transpose and renormalize; a singular affine transform makes authored-normal comparisons undefined. Geometric triangle normals always come from transformed positions. A negative determinant naturally reverses signed orientation; for a bake-transform mutation, reverse triangle winding and tangent handedness so front-face semantics are preserved.

### 4. Morphing and skinning

Add typed primitive `targets`. Apply POSITION deltas with resolved weights. NORMAL/TANGENT morphs affect authored-normal quality but geometric normals are recomputed. For skinning, normalize weight sums above `εn`, ignore zero-weight joints, validate JOINTS against the skin joint list, and compute the standard weighted skin matrix. Joint/node cycles or unavailable inverse-bind accessors make posed output undefined while base output remains available. This stage is independently cacheable from topology because it changes positions, not connectivity.

### 5. Deterministic geometric welding

Never weld source data as an inference side effect. Build an analysis-only topology per occurrence/part:

1. Insert finite positions into integer spatial cells `floor(p/εw)` with checked wide integers.
2. Enumerate candidate pairs from the 27 neighboring cells in stable vertex-key order.
3. Union pairs whose Euclidean distance is `<= εw` using deterministic union-find whose representative is the smallest source key.
4. Emit vertices sorted by representative key and remap corners.

This transitive clustering is invariant to input hash-map order. Ignore UV/normal/color seams for geometric topology, but never weld across occurrence/part boundaries. Relation analysis keeps parts distinct even when coincident. Return raw and welded counts plus cluster displacement maximum. A parameterized `noWeld` diagnostic mode supports forensic comparison.

### 6. Half-edge topology and orientation

Build directed half-edges from nondegenerate triangles. An undirected edge with one incident face is boundary, two is manifold, more than two is non-manifold. Detect duplicate triangles independent of rotation and winding. Split face-connected components through manifold adjacency.

Propagate consistent orientation across two-face edges by BFS in stable face order. Do not alter inference triangles; store an orientation sign per face. A contradiction marks a non-orientable component. For closed orientable components choose outward sign using signed volume; near-zero volume uses a robust exterior probe/ray winding test. Preserve original orientation agreement as a quality indicator.

### 7. BVHs and shared products

Build deterministic median-split AABB BVHs for triangles, line segments and part AABBs. Split on longest centroid span, tie X/Y/Z, stable median by primitive key. Leaves have at most eight items. Reuse for ray hits, nearest points, self-intersections, thickness, clearance, symmetry and relation candidate pruning.

Build these immutable products once:

- decoded asset buffers and expanded connectivity;
- local welded topology and local triangle moments;
- world positions/moments per occurrence transform/pose;
- triangle/segment/part BVHs;
- convex hull;
- topology summary and oriented shell nesting;
- deterministic surface samples and normal histogram;
- pair relation candidates.

## Indicator algorithms

### Size, bounds and projection

| Indicator | Definition and deterministic algorithm | Applicability / complexity |
|---|---|---|
| Overall size | AABB diagonal. Also return bounding-sphere diameter using deterministic Ritter initialization followed by exact farthest-point expansion; label sphere bounded approximation. | Any finite point, O(V). |
| Bounding-box dimensions | World AABB component extents in X/Y/Z; local asset AABB separately. Return oriented bounding box aligned to principal axes and a tighter minimum-volume candidate OBB searched over convex-hull face normals/edge directions. | Exact AABB O(V); PCA OBB estimate; hull-candidate OBB bounded candidate, O(H²) capped with quality. |
| Characteristic length | `L=AABB diagonal` for tolerance normalization; also volume-equivalent diameter `(6V/π)^(1/3)` for solids, area-equivalent diameter `2√(A/π)` for surfaces and total line length for pure curves. Names prevent conflation. | Domain-specific. |
| Footprint/projected area | Project every triangle onto the requested plane, discard zero projected area, union projected polygons with the internal 2D boolean interface, then sum outer minus holes. Canonical footprint is world XY; also report projections normal to principal axes. | Exact for PL triangles if planar boolean succeeds, O(F log F + intersections); otherwise deterministic raster bound pair. |

Projected area is union area, not the sum of triangle projected areas. Return both when useful: the latter is orientation-weighted surface area and can exceed footprint through overlap.

### Area and boundary exposure

Triangle area is `0.5·|(b-a)×(c-a)|` with compensated sum.

- `totalSurfaceArea`: sum of all nondegenerate authored triangle areas, including internal, duplicate and interpenetrating part surfaces; duplicate area is also reported.
- `closedBoundaryArea`: area of validated closed shell boundaries after intra-part duplicate removal.
- `externalExposedArea`: area of the boundary of the material union of all parts. Exact path uses triangle arrangement/CSG: BVH candidate intersections, robust triangle-triangle intersection, split faces along intersection segments, classify fragment centroid with generalized winding numbers against other closed parts, and retain fragments bordering exterior. Coplanar fragments use dominant-plane 2D clipping. If topology is invalid or arrangement exceeds the budget, use adaptive octree boundary integration and return lower/upper bounds.
- `contactSharedArea(a,b)`: coplanar, oppositely facing surface intersection area within `εc`, counted once. Project candidate triangle pairs to the dominant plane and polygon-clip; union fragments per plane group to prevent double counting. Near-contact area can be a separate tolerance-based estimate, never merged into exact shared area.
- `exposedArea(part)`: retained external fragments belonging to that part; contact faces and fragments inside another solid are excluded.

Self-intersection prevents exact closed-boundary semantics until the fragment arrangement resolves it. Open surfaces still have exact total area but external/contact semantics are undefined or explicitly two-sided estimates.

### Volume, void and compactness

For each oriented closed triangular shell, translate coordinates near the shell centroid and sum signed tetrahedron moments `(a×b)·c/6`. Use exact/adaptive sign predicates at ambiguity and compensated summation. Build a shell containment forest using BVH exterior probes and generalized winding. Material alternates by nesting parity independent of unreliable authored winding.

- `enclosedVolume`: sum of outermost shell interior volumes, including nested voids inside each envelope.
- `materialVolume` and unqualified `volume`: parity-filled material measure: outer shells add, depth-1 cavities subtract, depth-2 islands add, etc. Overlapping parts at scene level use the material union, not a naive sum; also return `sumOfPartVolumes`.
- `voidVolume`: enclosed volume minus material volume within outer envelopes.
- `interferenceVolume(a,b)`: volume of material intersection from the same exact triangle arrangement; adaptive octree lower/upper bounds are the fallback.
- `convexHullGap`: hull volume minus material volume.
- `reentrantVolume`: same as hull gap for a valid single material body; for assemblies additionally report inter-part empty hull volume separately.
- `reentrantArea`: hull surface area minus area of original boundary fragments coincident with hull boundary; never simply hull area minus surface area, which can be negative.
- `concavityIndex`: `(Vh−Vm)/Vh`, with 0 for a positive-volume convex body and undefined if `Vh<=εn³`.
- `surfaceToVolumeRatio`: external exposed area / material volume; per-part closed form also returned using closed-boundary area.
- `isoperimetricCompactness`: `36πV²/A³` in [0,1] for a valid single solid.
- `sphericity`: `π^(1/3)(6V)^(2/3)/A`, the cube root of compactness.
- `hullFillRatio`: material volume / convex-hull volume.

Use QuickHull with adaptive 3D orientation predicates and deterministic farthest-point/tie selection. Deduplicate hull inputs at `εw`. Coplanar inputs produce a 2D hull and no hull volume.

### Proportion and principal shape

Compute volume covariance for valid solids from exact tetrahedral second moments; use area covariance for open triangle surfaces and vertex covariance only as a named fallback for points/lines. Never let mesh tessellation density silently bias a solid PCA.

Diagonalize the symmetric 3×3 covariance with fixed-sweep Jacobi iteration, sort eigenvalues descending, and canonicalize eigenvector signs by making the largest absolute component positive (tie X/Y/Z). Enforce a right-handed basis by flipping the third axis. If eigenvalue gaps are below `εeig=max(1e-12·trace,128eps·trace)`, mark the affected axes non-unique and return the invariant subspace.

- `aspectRatios`: sorted principal extents `d1/d2`, `d2/d3`, `d1/d3` plus world AABB ratios.
- `slenderness`: `d1/max(d2,d3)`; `flatness=d2/max(d1,εn)` and `elongation=d3/max(d1,εn)` are also returned with explicit convention. Because literature reverses these names, raw extents/eigenvalues are authoritative.
- `mainAxisDirection`: first unique principal eigenvector; undefined for spherical/degenerate covariance.
- `principalAxes`: all three axes, eigenvalues, extents and uniqueness flags.

### Centroid and inertia

- `centroid`: volume center for solids; surface-area centroid for open surfaces; length-weighted centroid for lines; arithmetic point centroid otherwise. Return each applicable centroid separately and one dimension-selected primary centroid.
- `momentsOfInertia`: exact unit-density volume inertia tensor about origin from tetrahedral polynomial integrals, shifted to centroid by the parallel-axis theorem; eigenvalues/eigenvectors are principal moments/axes. For open surfaces compute a named unit-areal-density lamina tensor from triangle integrals.
- Scene aggregation sums part mass, first moments and origin inertia tensors, including instances, then shifts once. Material-union inertia requires arrangement fragments/tetrahedralization; return both additive-part and union values when overlaps exist.

Negative signed tetrahedra contribute algebraically after consistent shell orientation/nesting. A non-watertight or non-orientable solid has no volumetric centroid/inertia.

### Curvature and shape complexity

For a welded two-manifold triangle surface:

- mixed Voronoi area per vertex, using barycentric area for obtuse triangles;
- mean-curvature normal `Hn=(1/(2Ai)) Σ(cot α+cot β)(vi−vj)` and signed mean curvature from oriented normal;
- Gaussian curvature `K=(2π−Σθ)/Ai`, or `(π−Σθ)/Ai` at manifold boundaries;
- per-face curvature by area-weighted vertex interpolation;
- exact integrated Gaussian curvature from angle defect, useful for Gauss–Bonnet topology checks.

Return min/max/mean/RMS/quantiles for signed mean, absolute mean and Gaussian curvature, all area weighted. Curvature histograms use 64 symmetric log bins after dimensionless normalization `H·L` and `K·L²`, with explicit zero bin. Quantiles use deterministic weighted selection.

`sharpFeatureProportion` has two outputs: fraction of manifold interior edge length with unsigned dihedral above 30°, and fraction of surface area incident to at least one such edge. Boundary and non-manifold edge proportions are separate. `shapeComplexity` returns normalized integrated absolute mean curvature, curvature entropy and sharp-area fraction rather than hiding them in one arbitrary scalar.

### Thickness

Thickness is defined only for oriented closed material boundaries.

- `minimumThickness`: branch-and-bound over BVH node pairs to find the minimum distance between non-adjacent, non-incident boundary triangles. Validate the closest connecting segment midpoint as material with winding classification and require opposing local normals; otherwise continue. This is exact for the PL boundary when the search completes.
- `localThicknessSamples`: deterministic area-stratified samples per triangle using a geometry-digest-seeded low-discrepancy barycentric sequence. From each sample cast a fixed symmetric cone of inward rays, reject grazing/same-surface hits, take first exit distances, and use the weighted median shape-diameter value. Include both direct-normal thickness and cone-robust thickness.
- `meanThickness` and `thicknessVariability`: area-weighted mean, standard deviation, coefficient of variation, min/max and fixed quantiles over local samples.

Adaptive sample doubling stops when successive means/quantiles change below 0.5% or the policy cap is reached; the change is the empirical error estimate. Thin double surfaces without closed side walls remain open and therefore undefined, not assigned their separation as solid thickness.

### Clearance, interference and connectivity

Build a scene-level BVH over occurrence AABBs inflated by `εc`.

- `minimumDistanceToNeighbors`: exact triangle-triangle distance through best-first BVH pair traversal, zero for touching/intersecting surfaces. Return nearest points and part IDs.
- `clearanceDistribution`: for deterministic area samples on each part, closest distance to every other part via BVH; area-weighted histogram and quantiles. Distances are unsigned; signed penetration depth is not conflated with clearance.
- `interferenceVolume`: exact/bounded algorithm described above. Also return overlapping pair count.
- `numberOfContacts`: graph edges with positive exact shared area, or a separately classified near-contact edge when distance `<=εc`.
- `contactGraphDegree`: per occurrence exact-contact degree and near-contact degree.
- `connectedComponents`: graph components for exact contacts and for exact+near contacts, both in stable key order.
- mesh-topology connected components are a separate indicator and must not be confused with assembly contact components.

Broad phase is O(P log P + K); nearest narrow phase is normally O(log F) but worst-case quadratic; pairwise interference is restricted to overlapping part AABBs.

### Orientation and normal distribution

- `faceNormalDistribution`: area-weighted normals binned in a fixed equal-area octahedral map (8×8 per hemisphere, explicit mapping/version). Return directed and axial (`n≡−n`) distributions. Authored vertex normal agreement is an area-weighted angular-error histogram against geometric normals.
- `mainAxisDirection`: PCA definition above.
- `orientationConsistencyAcrossParts`: compare each part’s unique principal frame with the scene frame using absolute dot products and solve the 3×3 axis assignment by enumerating six permutations. Report mean alignment and non-unique exclusions. Also report normal-histogram Jensen–Shannon similarity.
- authored face winding consistency is the fraction of manifold adjacency constraints already satisfied before orientation propagation, plus outward-shell agreement.

### Symmetry, repetition and modularity

Generate deterministic candidate symmetry frames from the centroid, unique PCA axes, convex-hull face-normal clusters and normal-histogram peaks.

- `reflectionSymmetryScore`: reflect deterministic area samples across each candidate centroid plane; query closest point and compatible normal on the original surface BVH. Score is the area-weighted robust kernel `exp(-(d/(0.01L))²)` times normal agreement, computed bidirectionally. Report best plane, RMS/max residual, coverage and score. Exact score 1 is certified only if reflected welded vertices/faces biject under `εw`; otherwise it is an estimate.
- `rotationalSymmetryScore`: test orders 2..12 about each candidate axis, using every nonidentity step and the same bidirectional surface test. Return best axis/order and per-order scores. For continuous axial symmetry, report high agreement across all tested angles but do not claim an infinite group from a polygon mesh.
- `repetitionModularityRatio`: first cluster exact GLTF instances by shared mesh/primitive plus equivalent pose-independent geometry. Then cluster geometrically congruent parts using scale-normalized invariant fingerprints (volume/area/eigenvalues/radial and normal histograms), followed by rigid ICP verification with deterministic starts. Ratio is occurrence count or external area belonging to clusters of size ≥2 divided by total. Return exact-instancing and inferred-congruence ratios separately.

### Roughness, deviation and waviness

The “ideal” must be explicit. Produce three named comparisons:

1. `smoothedDeviation`: feature/boundary-preserving Taubin cotangent smoothing, fixed `(λ,μ)=(0.5,-0.53)` and 10/50 iterations; symmetric closest-surface RMS, median, P95 and max, normalized by L.
2. `planarOrQuadricFitDeviation`: per curvature-segment patch, deterministic least-squares plane or quadric fit with robust fixed iterations; area-weighted residual statistics.
3. `authoredNormalDeviation`: authored versus geometric normal angular statistics.

`normalVariation` is area-weighted RMS neighbor dihedral plus total variation `Σ edgeLength·dihedral / area`. `surfaceWaviness` is the low-frequency 50-iteration smoothed deviation; high-frequency roughness is the difference between 10- and 50-iteration residual energy. Preserve sharp edges and boundaries in smoothing; otherwise intentional corners become “roughness.” Approximation/sample metadata is mandatory.

### Topology

On welded nondegenerate triangles return raw and cleaned counts:

- V, E, F and Euler characteristic `χ=V−E+F` per face-connected component and total;
- boundary edge count and boundary loops, found by deterministic traversal of boundary half-edges; branching boundary vertices yield chains plus a non-manifold diagnostic rather than fabricated loops;
- manifold connected components;
- duplicate faces, isolated vertices, bow-tie vertices, non-manifold edges/vertices and self-intersection pairs;
- orientability and closedness;
- genus `g=(2−b−χ)/2` for each connected orientable manifold component with `b` boundary loops. Require the numerator to be a nonnegative even integer within exact integer arithmetic. Genus is undefined for non-manifold/non-orientable components.

Gauss–Bonnet check `Σ angle defects ≈ 2πχ` is a verification diagnostic, not the primary topology calculation.

## Concavity and assembly edge cases

- Multiple disconnected closed solids: additive per-part metrics; scene material union separately.
- Nested shells: containment parity determines material/void, not authored winding alone.
- Coplanar duplicate faces: total authored area includes them, geometric boundary area deduplicates them and reports duplicate area.
- Self-intersections: simple signed volume is diagnostic only. Exact arrangement may recover a material boundary; otherwise solid metrics are bounded/undefined.
- Non-manifold closed-looking soups: area/topology are available; volume/material/curvature/thickness requiring a manifold are undefined unless the exact arrangement yields a valid boundary.
- Zero/negative scale: zero scale collapses dimension and invalidates topology; negative determinant is supported.
- Extremely large/small coordinates: centering/scaling for predicates and moment accumulation prevents cancellation; outputs transform back.
- Points/lines mixed with surfaces do not affect surface/solid metrics unless a named `allVertexBounds`/PCA is requested.
- Transparent/double-sided materials do not alter geometry. Alpha is rendering semantics, not exposure.
- Orphan meshes remain asset results but do not contribute to a scene.

## Caching and invalidation

The current assertion that a flat fold is too cheap to cache no longer holds. Use separate `InferredField`s with exact dependency bytes and stable typed keys.

### Cache DAG

1. `DecodedAccessorField(accessorIndex)`: accessor descriptor, referenced buffer-view descriptor, exact dense byte range, sparse descriptors and exact sparse byte ranges.
2. `PrimitiveAssetField(mesh,primitive)`: mode, POSITION/index/morph/skin attribute references and parent decoded-accessor hashes.
3. `LocalTopologyField(mesh,primitive,policyVersion)`: expanded indices, local positions and weld policy.
4. `NodeWorldField(scene,nodePath)`: local transform/weights/skin, parent node-world hash and joint dependencies.
5. `OccurrenceGeometryField(occurrence)`: primitive asset + node world + pose.
6. Independent occurrence indicator fields: bounds/moments, topology, hull, curvature, samples, BVH. A rigid transform may analytically transform cached local moments/AABB corners; non-uniform/skinned transforms recompute necessary world integrals.
7. `SceneAggregateField(scene)`: ordered occurrence parents.
8. `RelationField(scene,unorderedPair)`: both occurrence geometry/BVH hashes and contact policy.
9. `SceneGraphField(scene)`: ordered relation parents.

Materials, images, samplers, animation records not selected by an animated query, names/extras and `sourceForm` must not invalidate geometry. A byte edit outside a referenced buffer slice must remain a cache hit. Buffer-view/accessor/reference changes invalidate only dependents. Scene/node transform changes retain decoded/local topology caches. One instance transform change invalidates only that occurrence, its scene aggregate and its candidate relations.

Cache tests must prove disabled-cache transparency, identical-snapshot hits, one-primitive incrementality, unrelated-field hits, exact-byte-range hits, parent invalidation and deterministic results under eviction. Cache values are immutable derived data; no warm mutable topology survives inference.

## Semantic mutation algorithms affecting geometry

The geometry implementation depends on repairing mutation semantics first.

### Referential index transaction

Every insert/remove of an indexed GLTF collection must be one atomic semantic mutation whose diff includes reference remapping across all dependent objects. Define a generated, exhaustively tested reference table. Examples:

- node: scenes.nodes, nodes.children, skins.joints/skeleton, animation channel targets;
- mesh: nodes.mesh;
- accessor: primitive attributes/indices/targets, skins.inverseBindMatrices, animation sampler input/output;
- bufferView: accessors, sparse indices/values, images;
- buffer: bufferViews;
- material: primitive.material;
- texture/image/sampler/camera/skin: every respective reference.

On removal, require a policy: reject when referenced, cascade owning objects, or clear only nullable references. Never decrement a reference that equals the removed index into a different object. Inverse captures the exact prior values and insertion position.

### Copy-on-write geometry edits

`MoveVertex`, `WeldVertices`, `OrientFaces`, `RemoveDegenerates`, `BakeNodeTransform` and repair mutations must not modify an accessor shared by unrelated primitives/semantics. Compute a reference graph, clone the accessor/bufferView/minimal bytes when shared, then retarget only the selected primitive. Repack deterministically into aligned buffer views, update byte lengths and recompute accessor min/max. Preserve untouched interleaved attributes or deinterleave the edited primitive explicitly.

Repairs consume the same analysis topology but emit authored GLTF changes only through a named mutation. Each supports dry-run diagnostics, exact inverse and postcondition validation. Applying then inverting must restore byte-identical snapshot state, including buffer padding, URIs and extension/extras fields.

### Transform mutation laws

- Node transform edit changes only node matrix/TRS and preserves mesh assets/instances.
- Bake transform applies the affine transform to POSITION, inverse transpose to NORMAL/TANGENT, flips winding/tangent handedness for negative determinant, resets node transform and clones shared meshes/accessors first.
- Geometry-changing edits invalidate accessor/primitive/occurrence/relations; material-only edits do not.
- Mutations must reject unsupported compressed accessors unless a registered extension encoder is available; never silently drop the extension.

## Verification program

### Analytic fixtures

Build fixtures through existing typed GLTF construction helpers, extending existing test files rather than creating test files.

| Fixture | Exact expected values |
|---|---|
| Box `a×b×c` | AABB `(a,b,c)`, footprint `ab`, `A=2(ab+bc+ca)`, `V=abc`, centroid center, `A/V`, hull fill 1, concavity 0, inertia `M/12·diag(b²+c²,a²+c²,a²+b²)`, genus 0. |
| Unit cube variants | Indexed/non-indexed/list/strip/fan where representable, duplicated UV seams, reversed face, missing face, internal duplicate face, zero-area face. Exact invariance or expected diagnostic. |
| Sphere radius r | Analytic targets `A=4πr²`, `V=4πr³/3`, `A/V=3/r`, sphericity 1, `H=1/r`, `K=1/r²`, inertia `2Mr²/5`; tessellated values must converge monotonically/in bounded error, not equal analytic values. |
| Regular tetrahedron side a | `A=√3a²`, `V=a³/(6√2)`, χ=2, genus 0. |
| Cylinder r,h | `V=πr²h`, `A=2πr(r+h)`, centroid and textbook axial/transverse inertia; tessellation convergence. |
| Torus R,r | `A=4π²Rr`, `V=2π²Rr²`, χ=0, genus 1; convergence. |
| Hollow concentric cubes a,b | enclosed `a³`, material `a³−b³`, void `b³`, total boundary area `6a²+6b²`, external exposed `6a²`, two boundary components. |
| Concave L prism | known polygon area×height volume, convex-hull gap and fill ratio. |
| Two boxes | separated known clearance; face-touch known shared area; partial face contact; positive overlap known intersection volume; containment. |
| Topology adversaries | open cube (`b=1`, genus formula valid for disk-like surface), non-manifold three-triangle edge, bow tie, two components, Möbius strip (non-orientable), self-intersecting triangles. |

### GLTF representation equivalence

For identical geometry assert equal results for:

- dense versus sparse accessor;
- tight versus interleaved stride and nonzero offsets;
- u8/u16/u32 indices;
- indexed versus non-indexed;
- triangle list/strip/fan;
- data URI versus GLB BIN bytes;
- matrix versus equivalent TRS;
- parent transform composition versus baked transform;
- one mesh instanced by nodes versus duplicated mesh data;
- positive and reflected scales;
- accessor bounds correct, absent and deliberately wrong;
- normalized integer normals/weights versus equivalent floats;
- default morph weights and static skin pose once schema support lands.

### Metamorphic laws

- Translation: scalar shape metrics unchanged; centroids/AABBs translate; inertia about centroid unchanged.
- Rotation: scalar metrics unchanged; axes/normals/centroid rotate; world AABB may change.
- Uniform scale s: length×`|s|`, area×`s²`, volume/mass×`|s|³`, centroid×s, inertia×`|s|⁵`, curvature `H/|s|`, `K/s²`; dimensionless ratios unchanged.
- Reflection: unsigned metrics unchanged; directed normals/orientation sign reflect; material volume stays positive.
- Triangle subdivision/retriangulation: PL area, volume, centroid, inertia and topology unchanged when no new geometric deviation is introduced.
- Vertex/index/primitive ordering permutation: public identity mapping changes only where GLTF indices define identity; numeric aggregate values remain bit-stable after stable-key normalization.
- Welding seam duplication within tolerance leaves geometric topology/mass unchanged and changes only raw counts.
- Adding points/lines leaves triangle-surface/solid metrics unchanged but changes named all-geometry bounds.
- Instance duplication doubles additive part mass/area but not material-union values for coincident instances; repetition ratio changes predictably.

### Property and differential tests

Use the existing seeded internal RNG to generate convex polytopes, transformed boxes, triangle subdivisions and scene trees. Properties include nonnegative areas, `0<=fill/compactness/symmetry<=1`, `material<=enclosed<=hull`, hull containment of every vertex, covariance/inertia positive semidefinite within tolerance, Euler/genus integrality, BVH nearest result equal to brute force for small meshes, and contact graph symmetry.

External libraries may be test oracles only behind test-local interfaces: compare eigenvalues with nalgebra already present elsewhere, point/triangle distances with parry3d already present in the repo, and imported GLTF scene expansion with the existing `gltf` crate. Production result types must not expose those libraries. Differential fuzz failures are minimized and checked in as typed fixtures inside existing test modules.

### Mutation laws

For every semantic mutation test:

- diff application equals direct intended post-state;
- apply followed by inverse restores exact snapshot;
- text and binary mutation codecs round-trip;
- all GLTF references are in range and still target the same semantic object after unrelated insertion/removal;
- copy-on-write leaves other users byte-identical;
- only declared inference cache dependencies miss;
- runtime decode of the mutated output confirms the intended coordinates/indices, with temporary `[DEBUG] ` logs removed after verification.

### Error-path tests

Cover missing external bytes, truncated views, stride overflow, sparse duplicates/out-of-range indices, nonfinite f32, mismatched counts, illegal primitive mode, incomplete mode tails, invalid node roots/children/cycles, singular matrices, zero quaternions, skin joint errors, unsupported compression, coordinate overflow and resource caps. No panic, hang or fabricated zero metric is acceptable.

## Performance and resource budgets

Budgets are acceptance targets to measure through `bun`/`nx` targets and the repository benchmark harness, not claims about current performance.

For a native release build on the project reference machine:

- 100k triangles / 100 parts: decode+scene expansion ≤100 ms, weld+topology ≤250 ms, moments/bounds/PCA ≤75 ms, BVH ≤150 ms, hull ≤300 ms; quick inference total ≤750 ms.
- 1M triangles / 1k parts: quick inference ≤6 s and peak derived memory ≤512 MiB; no recursive stack proportional to scene depth or face count.
- Full curvature ≤1.5 s/100k triangles; deterministic 64k-sample roughness/symmetry/thickness pass ≤3 s each; exact pair relations ≤5 s for 100 non-overlapping/contacting parts.
- Interactive mutation preview on a 100k-triangle selected occurrence should reuse decode/local topology and complete transformed bounds/centroid ≤16 ms, while exhaustive relations run asynchronously from immutable snapshot state.
- Cache re-inference after one node transform touches one occurrence and candidate pairs only; target ≤50 ms for 100k total triangles when that occurrence is ≤10k.

Resource guards are deterministic policy inputs: maximum decoded elements, BVH nodes, arrangement fragments, octree cells, samples and relation candidates. Reaching a cap returns bounded/estimate quality with a diagnostic; it never truncates and labels the result exact. Benchmark adversarial all-coplanar, all-overlapping and high-valence inputs, not only clean solids.

Complexities:

- decode/expand/weld/topology/moments/curvature: expected O(V+F), weld expected O(V+k);
- BVH/hull: O(F log F), hull worst O(V²);
- exact nearest pair: BVH-pruned, worst O(Fa·Fb);
- contact/CSG arrangement: output-sensitive and potentially quadratic; guarded fallback;
- PCA 3×3 eigensolve: O(1) after moments;
- topology: O(V+E+F);
- symmetry/thickness/clearance samples: O(S log F).

## Parallel implementation work packages

The following packages can be assigned independently once the shared contract and fixture builder are frozen. Each owns regions in existing files; no package creates production side files.

1. Snapshot completeness: morph targets and geometry extension adapter interfaces across every facet.
2. Referential mutation transaction table and invariant validator.
3. Accessor decoding/normalization/range hashes/compression adapters.
4. Scene occurrence traversal, transforms, morph and skin static pose.
5. f64 vectors, robust predicates and deterministic reductions.
6. Primitive expansion, weld and half-edge topology extraction from the existing mesh kernel.
7. BVHs, exact distances, ray/winding classification and self-intersection.
8. Bounds, projections and planar union.
9. Triangle mass moments, shell nesting, material/void semantics and inertia.
10. QuickHull, concavity and OBB.
11. PCA/proportion/orientation and normal histograms.
12. Curvature, sharpness and Gauss–Bonnet verification.
13. Thickness and its adaptive sampling/error estimates.
14. Contact/shared/exposed/interference arrangement with bounded octree fallback.
15. Clearance/contact graph/connectivity.
16. Symmetry/repetition/congruence verification.
17. Smoothing/roughness/waviness/deviation.
18. Inference facet schemas, quality wrappers and deterministic aggregation.
19. Fine-grained `InferredField` cache DAG and incrementality instrumentation.
20. Copy-on-write geometry edits and transform baking.
21. Analytic fixtures and representation equivalence tests.
22. Metamorphic/property/differential/error fuzz tests.
23. Native/WASM cross-platform performance, memory caps and `launch.json`/Nx wiring.
24. End-to-end runtime validation and documentation synchronization across Rust/TypeScript/GraphQL/JSON/Proto facets.

Integration order is 1–6, then 7; packages 8–13 can proceed concurrently; 14–17 depend on 7 and shared samples; 18–20 integrate results; 21–24 run continuously and close the ticket only after runtime verification.

## Definition of done

- Every requested indicator above is represented, dimensionally defined and tested, or returns a typed undefined reason on inapplicable geometry.
- Scene transforms, hierarchy, instancing, primitive modes, sparse/interleaved accessors, morphing and static skinning are honored.
- Open, degenerate, duplicated, self-intersecting, non-manifold, nested and overlapping inputs have deterministic documented outcomes.
- Exact and approximate values are distinguishable; approximations expose sample count/error or bounds.
- Results are bit-deterministic across repeated runs and thread counts on a platform, and numerically equivalent across native/WASM within explicit tolerances.
- Cache dependency and incrementality laws pass.
- Semantic geometry mutations maintain all references, are copy-on-write, invert exactly and round-trip through every facet.
- The analytic, metamorphic, differential, fuzz, performance and end-to-end runtime suites all run through registered `bun`/`nx` targets and launch configurations.
