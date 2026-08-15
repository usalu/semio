# glTF Geometric Analysis Discovery

## Decision

`💡️inferences/📐️geometry` is an umbrella, not one inference. It combines the registered aggregate result, fourteen metric contracts, decoding, transforms, topology, measurement, statistics, pair analysis, orchestration, and tests. Its identity must be removed rather than preserved under an alias.

## Target Ownership

- The registered aggregate becomes `💡️inferences/🧮️geometric-analysis` and owns aggregate assembly, diagnostics, quality, validity, provenance, `GltfGeometricInference`, `GltfPartInference`, `GltfPairInference`, `GltfInferenceCounts`, and aggregate-only collection/transform handling.
- The aggregate field is renamed from `geometry` to `geometricAnalysis` across Rust, TypeScript, JSON Schema, GraphQL, Proto, codecs, generator inputs, descriptors, fixtures, and direct consumers. No compatibility field or forwarding alias is permitted.
- The fourteen metric inference components remain their own leaves: size, area-volume, compactness, proportion, mass-distribution, curvature, thickness, concavity, clearance, adjacency, orientation, symmetry, roughness, and topology.
- `pub use geometry as bounds` in stdio Rust glue is an invalid forwarding alias and must be removed.

## Evidence-Based Shared Candidates

At the standard 2.0 / any-subset glTF owner, pending implementation validation:

| Candidate capability | Independent terminal production metric consumers |
| --- | --- |
| geometric context | all fourteen metric leaves |
| mesh topology | area-volume, clearance, adjacency, orientation, symmetry |
| pair geometry | area-volume, clearance, adjacency, orientation |
| distribution statistics | size, clearance, thickness, curvature, roughness, orientation |
| convex hull | compactness, concavity |
| triangle measurements | curvature, concavity |
| vector operations | curvature, roughness, size, orientation, concavity |
| inference measures | nearly all metric leaves |

Each is a candidate only when its resulting responsibility remains semantically coherent; helpers used by one resulting capability stay private. The lowest owner is glTF—not framework—unless a separate audit proves domain-neutral semantics including tolerance, coordinate, error, and performance rules.

Accessor decoding and GLB/document/data-URI codecs are specific I/O boundaries. The accessor decoder has independent aggregate-inference, mesh-import, animation-import, and animation-export production consumers, so it qualifies under glTF-specific I/O ownership.

## Atomic Runtime Surfaces

The stdio glue mounts the current umbrella. The root inference component registers descriptors, fields, codecs, and `ArtifactInferrer`; the artifact root registers cold inference and binary services. These mounts are glue and excluded from module consumer counts but must move atomically with contract renaming. Generated outputs are regenerated through Nx, never edited directly.

## Private Evidence

Thickness and roughness sampling, aggregate node-matrix/decode/fingerprinting helpers, and ray/triangle/shell internals have no demonstrated second terminal consumer; they remain private unless the implementation lease discovers otherwise.
