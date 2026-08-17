# glTF Atomic Inference Ownership and Census

## Ownership

- Owner: `gltf_atomic_inferences`.
- Writable scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/**/🧬️schema/💡️inferences/**` and glTF-only reusable geometry kernels below that artifact.
- Excluded: stdio registry and artifact-definition JSON, root declarations, framework, store, IO, and mutations.
- Gate owner: the coordinator owns the serial Cargo/Nx gate. This lane does not run Cargo until the declaration shards are frozen and the coordinator signals permission.

## Exact Baseline Census

The sole executable aggregate is `🧮️geometric-analysis/🦀️component.rs` (1,726 lines). It constructs `GltfInference.geometricAnalysis`, defines the aggregate `GltfGeometricInference`, builds `GltfEntityIndicators`, creates all part and pair records, owns policy/provenance/quality/availability helpers, gathers raw geometry, and executes every metric stage. It has seven aggregate cache fields, including the prohibited aggregate `s.stdio.gltf.inference.geometricAnalysis.aggregate`.

Existing public group stages and their preserved indicator totals are:

| Group | Indicators |
| --- | ---: |
| size | 7 |
| area-volume | 8 |
| compactness | 5 |
| proportion | 4 |
| mass-distribution | 5 |
| curvature | 4 |
| thickness | 4 |
| concavity | 4 |
| clearance | 4 |
| adjacency | 3 |
| orientation | 3 |
| symmetry | 6 |
| roughness | 5 |
| topology | 5 |
| **Total** | **67** |

All 14 group stages currently import shared geometry and selected calculation helpers from `super::geometric_analysis`; that coupling is the refactor target. `🟦️component.ts` has only aggregate output contracts and no executable calculator. The root inference TypeScript component is empty. The current inference manifest lists the 14 groups plus the obsolete `geometric-analysis` aggregate.

## Target Boundary

The inference root becomes assembly only: public type reexports, per-leaf descriptors, dependency DAG, and non-calculating result composition. Every one of the 67 semantic measures receives a canonical `s.stdio.gltf.inference.<slug>.v1` descriptor and an executable Rust and TypeScript surface. Leaf code owns its own measure record construction, dependency declaration, validity, quality, diagnostics, provenance, algorithm version, and incremental cache key. Pure vector, topology, bounds, BVH/distance, sampling, and statistics helpers move to named internal glTF geometry modules that cannot create public measurement records. No `bounds` or `geometric-analysis` public aggregate/alias/compatibility layer remains.

## Preserved Contract

- All 67 existing indicator field names and values remain in the assembled output.
- Existing unavailable/invalid/indeterminate behavior remains explicit; no unavailable calculation becomes a fabricated zero/exact result.
- Static-pose policy and current tolerance/sampling fingerprints remain the source of truth until a leaf carries its own versioned provenance.
- Rust/TypeScript/schema facets must remain structurally aligned for each leaf.
