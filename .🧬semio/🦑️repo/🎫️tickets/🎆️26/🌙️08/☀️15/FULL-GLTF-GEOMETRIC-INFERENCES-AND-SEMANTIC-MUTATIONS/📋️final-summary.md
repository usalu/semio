# Full GLTF Geometric Inferences and Semantic Mutations

## Outcome

Replaced the ad-hoc GLTF bounds inference and index-splice mutations with one schema-first, scene-aware, deterministic inference and semantic mutation subsystem spanning Rust, TypeScript, GraphQL, JSON Schema, Proto, text grammars, binary protocols, native runtime registration, and the component host/guest boundary.

## Inference

- The public root is exclusively `GltfInference { geometry: GltfGeometricInference }`; legacy root bounds and duplicate counts are removed.
- The contract exposes 67 geometric indicators in 14 groups, plus per-part and pair results, policy, counts, diagnostics, validity, quality, and provenance.
- Accessors are decoded from resolved buffers with interleaving, sparse overlays, every legal integer normalization rule, and morph POSITION targets.
- Geometry is evaluated in static scene/world space across scene roots, node hierarchy, TRS/matrices, instancing, triangle/strip/fan modes, welding, and deterministic tolerances.
- Algorithms cover AABB/PCA bounds, projected areas, surface/material/enclosed/void volume, compactness, mass distribution, curvature, sharp features, ray thickness, convex hull/concavity, triangle clearance/contact, grid overlap/interference, adjacency, orientation, symmetry, repetition, smoothing deviation/roughness, and topology.
- Every measure carries unit, availability, validity, diagnostics, exact/estimate method, coverage/topology quality, and provenance. Unsupported or invalid computations are typed unavailable rather than fabricated.
- The dependency DAG separates resources, accessors, primitives, instances, materials, relations, and aggregate invalidation.

## Mutations and Diffs

- All structural commands validate bounds and the full GLTF reference graph before application.
- Insert/remove operations transport dependent indices, including node hierarchies, scenes, meshes, materials, accessors, morph targets, buffers, skins, textures, images, and animations.
- Semantic commands add node transforms, reparenting, node/mesh binding, and primitive/material binding.
- Invalid, cyclic, referenced, ambiguous, normalized-float, and buffer metadata/payload operations return typed `GltfMutationRejection`; accepted operations return sparse forward diffs.
- Diffs expose exact inverse operations, stable touched paths, typed regions, and minimal inference invalidation.

## Representation and Runtime

- Canonical inference text uses a versioned length/checksum envelope around RFC 8785-style JSON.
- Canonical inference binary uses the frozen 40-byte header with magic, versions, schema CRC, payload length/CRC, and header CRC.
- Mutation tags are exhaustive and stable at 0–27; all text/binary/schema facets agree.
- A domain-neutral native inference registry provides deterministic registration, conflict rejection, cold snapshot-pack inference, and canonical binary output.
- WIT guest exports and the host router list and invoke inference services with strict metadata, revision/generation echo validation, and stale-result rejection.

## Verification

- Combined GLTF Nx gate: 91 tests run, 91 passed, 3367 skipped.
- Advanced geometry: 13/13 passed.
- Inference: 23/23 passed.
- Mutations: 10/10 passed.
- Diffs: 9/9 passed.
- Conformance laws: 6/6 passed.
- Cold native parity, wire parity, touched-region, host stale-result, plugin registry, and focused codec tests passed.
- Runtime evidence captured canonical inference size, cold WIT/native binary parity, revision/generation echo, semantic touched regions, and inverse restoration with temporary `[DEBUG]` logs; the logs were removed afterward.
- Unfiltered stdio quick testing started 3448 tests and stopped fail-fast at the pre-existing BCF shipped-fixture honesty mismatch after 40 passes. The same BCF failure is recorded in the ticket baseline; the complete GLTF scope remains green.
- Scoped `git diff --check` is clean.

## Evidence

- `📋️implementation-plan.md`
- `📐️algorithms-report.md`
- `🔗️integration-report.md`
- `📋️final-conformance-audit.md`
- `📋️wave-5-b-runtime-evidence.md`
- `🦀️wave-1b-report.md`
