# Wave 1-B Rust Geometric Inference

## Result

The Rust glTF inference now exposes the frozen `GltfInference { geometry }` contract without legacy bounds fields. The implementation is hosted in the existing bounds leaf as required, but computes the complete regioned geometric result: policy, counts, overall indicators, per-instance parts, pairs, diagnostics, validity, quality, and provenance.

## Canonicalization

- Decodes real buffer-backed accessors through the shared decoder, including sparse overlays and normalized integer decoding supplied by the IO kernel.
- Validates base POSITION as FLOAT VEC3, morph POSITION as matching FLOAT VEC3, and indices as unsigned SCALAR.
- Applies static mesh/node morph weights before node transforms.
- Traverses every authored scene and node hierarchy, composes matrix or TRS transforms, detects hierarchy cycles, and preserves mesh instancing.
- Expands TRIANGLES, TRIANGLE_STRIP, and TRIANGLE_FAN with deterministic winding and diagnoses unsupported modes, invalid indices, unresolved resources, and malformed attributes.
- Welds by policy tolerance and derives edge incidence, connected components, boundary loops, orientability, manifoldness, Euler characteristic, and genus.

## Indicator Algorithms

Exact where the input topology permits:

- axis-aligned bounds, dimensions, diagonal size, triangle surface area, projected component areas;
- signed-tetrahedron shell volume and centroid, deterministic shell nesting, top-level enclosed volume, alternating material volume, and void volume for closed oriented manifold meshes;
- edge topology, components, boundary loops, Euler characteristic, genus/handles;
- face-normal angular distribution, sharp-edge length proportion, and internal orientation consistency;
- surface totals across preserved instances.

Deterministic estimates with explicit approximate quality:

- PCA frame, oriented dimensions, aspect/slenderness/flatness/elongation, and unit-mass point-distribution inertia;
- reflection and rotational symmetry sampled against the welded point cloud;
- angle-over-edge mean-curvature statistics and angle-defect Gaussian-curvature statistics;
- opposite-surface ray thickness distribution;
- umbrella-smoothed deviation, waviness, normal variation, and irregularity;
- supporting-plane convex hull area/volume, hull fill, hull gap, concavity, and re-entrant face area;
- triangle-triangle pair clearance and contact-area estimates;
- stratified intersected-AABB overlap/interference volume using three-direction closed-mesh parity;
- intrinsic quantized part-signature repetition and modularity.

The following remain typed unavailable because the geometry does not define them defensibly:

- deviation from an unspecified ideal geometry;
- orientation consistency across independent parts.

No bounding-box fill, bounding-box overlap, or invented zero is reported as any of those quantities.

## Adversarial Complexity Bounds

- Convex-hull supporting-plane enumeration is capped to at most `min(policy.samplingBudget, 32)` deterministic points, always retaining axis extrema; sampled results remain approximate.
- Thickness selects point and face strides whose Cartesian work is bounded by the policy sampling budget.
- Each pair's triangle-distance work is bounded by the policy sampling budget using deterministic two-axis stratification.
- Pair overlap grid size is bounded by the policy sampling budget divided by the three-ray triangle-classification cost.
- Nested-shell containment is refused as unavailable when shell-by-face parity work would exceed the policy sampling budget.
- Symmetry sampling is bounded by the policy sampling budget.
- Linear mesh passes cover decoding, transforms, welding inputs, topology, curvature, and roughness. Pair output itself is necessarily quadratic in the number of reported parts.

## Counts, Quality, and Provenance

- `primitiveCount` is the authored primitive count; `validPartCount` and `invalidPartCount` describe analyzed scene instances.
- Overall quality is deterministic-estimate because the result intentionally combines exact and sampled indicators; every measure carries its own exact/estimate availability and topology coverage.
- Coverage is valid parts divided by valid plus invalid parts.
- Provenance contains canonical world-geometry and per-buffer FNV-1a dependency fingerprints.

## Verification

Final focused verification:

- `cargo test -p semio-s-plugin-stdio --no-run`: passed.
- Direct bounds test module: 13 passed, 0 failed. Cases cover analytic cuboid bounds/area/volume/hull/topology; convex-versus-notched concavity; planar/closed Gaussian curvature; planar/corrugated smoothing deviation; analytic parallel-shell thickness; open and non-manifold availability; transformed instancing and intrinsic repetition; separated/contacting triangle distance; overlapping/separated box volume; hollow nested-cube enclosed/material/void volume; deterministic hull budget capping; and rigid-transform invariance.
- `bun nx run @semio-tech/stdio-plugin:test-quick -- gltf`: passed, 91 tests run, 91 passed, 0 failed, 3367 skipped.
