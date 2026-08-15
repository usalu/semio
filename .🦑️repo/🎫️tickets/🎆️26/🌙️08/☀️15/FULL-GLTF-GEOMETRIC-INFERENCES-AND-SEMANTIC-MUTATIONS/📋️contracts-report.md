# GLTF Contract Replacement

## Decision

Replace the current “typed JSON plus index edits” surface with two schema-first contracts over the unchanged, lossless glTF 2.0 snapshot:

1. `GltfSemanticMutation` is a closed command vocabulary. Every command has one deterministic `GltfSemanticDiff` and one inverse command sequence. Collection edits repair all glTF index references atomically; invalid commands produce a typed rejected/no-change diff, never a partial document.
2. `GltfGeometricInference` is a versioned, dependency-aware analysis graph. Results exist at document, scene, node-instance, mesh, primitive, connected-component, surface-region, and entity-pair scopes. Every value carries units, validity, provenance and quality; unavailable geometry is data, not fabricated zeroes.

The persisted snapshot stays a faithful glTF/GLB representation. Stable analysis/mutation identity is a command-time `GltfAddress` (`kind`, current index/path, expected content fingerprint and base revision), not a private ID serialized into glTF. This preserves losslessness while detecting stale index addresses.

## Current Ad Hoc Surface

### Snapshot and codec

- `GltfSnapshot` couples `document.buffers[i]` to a second index-aligned `buffers[i]`; the invariant is documented but not represented or validated as a contract.
- `GltfNode` permits matrix and TRS simultaneously for lossless retention, but there is no canonical effective-transform or invalidity model for inference.
- Geometry is reachable only through string attributes, integer indices and decoded flat `Vec<f64>` values. `decode_accessor` loses integer/component provenance and provides no typed position/index/normal views.
- `validate_document` checks the asset version only. Dangling references, buffer ranges, accessor shape, primitive topology, cycles, non-finite coordinates, degeneracy and manifold/orientation conditions are not surfaced structurally.
- JSON/GLB parsing silently resolves absent/external buffers to empty bytes. Inference cannot distinguish “empty” from “unresolved”.

### Mutations and diffs

- `GltfMutation` contains `NoMutation`, `SetSnapshot`, `SetAsset`, and Insert/Remove/Set triples for only scenes, nodes, meshes, accessors, materials, buffers and animations.
- Buffer views, textures, images, samplers, skins and cameras have no command vocabulary and are reachable only through `SetSnapshot`.
- Every command is an index edit or whole-record replacement. There is no intent such as reparent, transform, bind material, replace attribute data, edit animation channel, or attach a scene root.
- Inserts clamp out-of-range indices; missing Set/Remove targets become empty effects. These are silent coercions rather than typed rejections.
- Removing or inserting an indexed entity does not express reference repair. Generic index transport only moves diff positions; it does not establish glTF referential-integrity semantics across scenes/nodes/meshes/accessors/buffer views/materials/textures/images/samplers/skins/animations/cameras.
- `SetBuffer` manually couples metadata and bytes, while direct diff fields can still change them independently.
- Mutation inverse is reconstructed opportunistically from the base; missing targets yield `NoMutation`. There is no precondition, rejection reason or explicit inverse payload.
- `GltfDiff` is a 4,221-line handcrafted structural patch/codec. Six collections have field diffs; the rest use whole-item “weak” diffs. `extensionsUsed`, `extensionsRequired`, extension JSON and extras replace whole values.
- Text/binary opcodes are declaration-order integers with duplicated handwritten dispatch. The committed TypeScript/GraphQL/JSON Schema/Proto describe shapes but do not drive Rust or codecs.
- Only the historical `set-snapshot` command has physical mutation/diff/inverse leaf files; the actual 22 other variants live monolithically in the family root.
- `GltfDiff` has no `DiffRegions` implementation, so a cache-aware inference cannot safely perform tier-1 invalidation.

### Inferences

- `GltfInference` has one field, `bounds`, and declares the coarse read-set `document`.
- `compute_gltf_bounds` ignores node transforms, scene selection, instancing, skins/morphs and primitive modes; it unions accessor-declared local `POSITION min/max` only.
- Missing/dangling/non-VEC3/min-max-less accessors are silently skipped. Empty or wholly invalid geometry returns an origin-sized box, indistinguishable from a genuine point at the origin.
- `vertexCount` double-counts shared accessors and counts accessor entries, not topological vertices; integer-to-`u32` casts can truncate. Mesh/primitive counts are mixed into a type named “bounds”.
- No buffer bytes are read, so sparse accessor overrides and dishonest/missing accessor extrema are not verified.
- There is no per-entity `InferredField`, dependency chain, cached implementation, tolerance policy, units, coordinate-space declaration, validity, warning/error diagnostics, provenance or confidence/quality.
- All generated inference facets and the descriptor merely repeat this single box.

## Schema Taxonomy

The canonical Rust contract is mirrored exactly in TypeScript, GraphQL, JSON Schema and Proto.

```text
GltfGeometricInference
├── policy: GltfAnalysisPolicy
├── validation: GltfGeometryValidation
├── documents: GltfDocumentIndicators
├── scenes: Map<SceneAddress, GltfEntityIndicators>
├── instances: Map<NodeInstanceAddress, GltfEntityIndicators>
├── meshes: Map<MeshAddress, GltfEntityIndicators>
├── primitives: Map<PrimitiveAddress, GltfEntityIndicators>
├── components: Map<ComponentAddress, GltfEntityIndicators>
└── relations: Map<OrderedEntityPair, GltfPairIndicators>

GltfEntityIndicators
├── size: AABB/OBB dimensions, diagonal/characteristic length, projected footprint/area
├── measure: total/exposed/contact area; signed/enclosed/material/void volume
├── compactness: S/V, sphericity, hull fill and concavity/re-entrant measures
├── proportion: aspect ratios, PCA slenderness/flatness/elongation
├── mass: centroid, covariance, principal frame, inertia tensor/eigenvalues
├── curvature: mean/Gaussian summaries and histogram, sharp-feature ratio
├── thickness: mean/min/quantiles/variation and sample coverage
├── clearance: minimum/quantiles/histogram and nearest entities
├── adjacency: contacts, degree, connected components
├── orientation: main axis, area-weighted normal histogram, consistency
├── symmetry: reflection/rotation candidates and scores; repetition/modularity
├── roughness: smoothed deviation, normal variation, waviness
└── topology: boundary loops, components, Euler characteristic, genus/handles
```

Core value types:

- `GltfAddress`: typed scope plus indices/path, `content_fingerprint`, `base_revision`.
- `GltfCoordinateSpace`: `MeshLocal`, `NodeLocal`, `SceneWorld`; instance-sensitive metrics never masquerade as mesh-local metrics.
- `GltfQuantity<T>`: `{ value: Option<T>, unit, validity, provenance, quality }`.
- `GltfUnit`: `Unitless`, `Metre`, `SquareMetre`, `CubicMetre`, `Radian`, `InverseMetre`, `InverseSquareMetre`. glTF coordinates use the glTF metre convention; the result explicitly records it rather than assuming it in consumers.
- `GltfValidity`: `Valid`, `Approximate`, `Unavailable`, `InvalidInput`, `UnsupportedPrimitive`, `OpenSurface`, `NonManifold`, `Degenerate`, `UnresolvedResource` with diagnostic IDs.
- `GltfProvenance`: algorithm ID/version, dependency fingerprints, coordinate space, selected scene/instances, sampling mode/count/seed, tolerance-policy fingerprint.
- `GltfQuality`: coverage ratio, exact/estimated method, absolute/relative error bounds, sample count, watertight/manifold/oriented flags and warnings.
- `GltfHistogram`: fixed edges plus counts/area weights; never implementation-specific buckets.
- `GltfPrincipalFrame`: centroid, orthonormal axes, sorted eigenvalues and deterministic sign/tie convention.

## Canonical Geometry and Policy

`GltfAnalysisPolicy` is serialized in the result and salted into every cache key. Defaults are deterministic and schema-versioned:

- `absolute_length = max(scene_diagonal × 1e-9, 1e-12 m)`; `relative = 1e-9`; `angular = 1e-7 rad`.
- Contact/clearance epsilon is `max(absolute_length, characteristic_length × relative)`.
- Degenerate triangle area is `epsilon²`; near-zero volume is `epsilon³`; sharp edges use a recorded dihedral threshold (default `30°`).
- Histograms use fixed schema edges; stochastic estimators use a content-derived seed and record samples/error bounds.
- Density defaults to `1 kg/m³` only for normalized geometric inertia; results say `unitDensity`. Material mass is unavailable unless an explicit density resolver is supplied through the analysis service.
- Morph target weights may be evaluated; skeletal animation and time-varying animation require an explicit pose/time request. The default inference analyzes the static authored pose and records this.

The geometry normalization stage decodes actual buffers (including stride, sparse overlays, normalized components and indices), triangulates triangles/strip/fan with winding preserved, retains lines/points as unsupported for surface/volume indicators, applies morph then node-world transforms, and emits diagnostics for every skipped primitive. No metric reads accessor `min/max` as authoritative; extrema are an optional validation cross-check.

## Inference Dependency Graph

```text
snapshot document + resolved buffers + policy
→ validation/resource status
→ decoded accessor views (key: accessor + buffer/bufferView dependencies)
→ canonical primitive geometry (attributes + indices + mode + morph weights)
→ half-edge topology + BVH + triangle normals/areas
→ connected components + boundary/manifold/orientation classification
→ node-instance world geometry (node-parent transform DAG + mesh primitive geometry)
→ basic measures (AABB/OBB, area, signed volume, centroid, covariance/inertia)
→ hull/PCA/curvature/thickness/roughness/topology indicators
→ pair candidates from BVH
→ clearance/contact/interference/adjacency
→ scene/document aggregation
→ orientation consistency, symmetry and repetition/modularity
```

Each stage is an `InferredField` keyed by the narrowest stable address and hashes exactly what it reads. Parent hashes express the graph above. `GltfDiff::touches()` emits fine paths such as `document/meshes/3/primitives/1`, `document/accessors/8`, `buffers/0`, and `document/nodes/5/transform`. `ArtifactInferrer::infer_cached` runs the graph through `infer_field_after_diff`; cache-disabled and cold/warm-cache outputs must be byte-identical.

Indicator validity rules are explicit: enclosed volume/sphericity/material-vs-void require consistently oriented watertight shells; genus requires orientable manifold components; contact area/interference require closed surfaces for volumetric results; thickness requires two-sided ray coverage; curvature on boundary/nonmanifold vertices is marked partial. Approximation never silently becomes an exact scalar.

## Semantic Mutation Contract

The public enum is `GltfSemanticMutation`. Every payload contains `target: GltfAddress` and `precondition: GltfPrecondition`. Dispatch returns `GltfMutationOutcome { diff, inverse, diagnostics }`. A rejected command has an empty diff, empty inverse, and a typed diagnostic. Applied diffs include `touched_paths` and reference-remap records.

Required semantic command families (each is a mutation/diff/inverse triad even when implemented as regions in the existing files):

| Aggregate | Commands |
|---|---|
| document/scene | change asset metadata; create/delete/rename scene; choose/clear default scene; attach/detach/reorder scene root; declare/undeclare/require/unrequire extension |
| node graph | create/delete/rename node; reparent/detach/reorder child; translate/rotate/scale/transform node; attach/detach mesh, camera or skin; change morph weights |
| mesh/primitive | create/delete/rename mesh; create/delete/reorder primitive; bind/unbind attribute; bind/unbind indices/material; change topology mode; change mesh weights |
| accessor/storage | create/delete/rename accessor; bind/unbind buffer view; change layout/normalization/extrema/sparse overlay; create/delete/rename/reframe/retarget buffer view; add/delete/rename buffer; replace/splice buffer bytes; change/clear URI |
| material/texture/image/sampler | create/delete/rename and bind/unbind each resource; change PBR factors/textures, normal/occlusion/emissive inputs, alpha policy and sidedness; replace image source/MIME; change sampler filters/wrap |
| skin/animation/camera | create/delete/rename skin; bind skeleton/joints/inverse matrices; create/delete/rename animation; add/delete/reorder/edit channel and sampler; create/delete/rename camera; change perspective/orthographic projection |
| extension data | set/clear/merge extension or extras at any typed owner address; reject invalid JSON path/type operations |
| atomic geometry | replace primitive geometry and transform geometry as coordinated commands that author accessors, views, buffers and references in one transaction |

Delete/insert triads carry a `GltfReferencePlan`: `RejectIfReferenced`, `Cascade`, or an explicit replacement address. Diff contains the exact removed value/bytes, insertion anchor, global `GltfIndexRemap`, repaired references and before/after fingerprints. Inverse consumes that captured diff, not a fresh best-effort lookup. Laws: `apply(diff(m,b),b) == apply(m,b)`; `apply(inverse(m,b),apply(m,b)) == b`; diff inverse/absorb laws; no dangling references after any accepted command; serialization round-trips for every variant.

Public API:

```rust
pub fn validate_gltf_geometry(snapshot: &GltfSnapshot) -> GltfGeometryValidation;
pub fn infer_gltf_geometry(snapshot: &GltfSnapshot) -> GltfGeometricInference;
pub fn infer_gltf_geometry_with(snapshot: &GltfSnapshot, policy: &GltfAnalysisPolicy) -> GltfGeometricInference;
pub fn infer_gltf_geometry_cached(snapshot: &GltfSnapshot, policy: &GltfAnalysisPolicy, cache: &mut InferenceCache, session: &mut InferenceSession) -> GltfGeometricInference;
pub fn plan_gltf_mutation(base: &GltfSnapshot, command: &GltfSemanticMutation) -> GltfMutationOutcome;
pub fn apply_gltf_diff(base: &GltfSnapshot, diff: &GltfSemanticDiff) -> Result<GltfSnapshot, GltfMutationRejection>;
```

Root re-exports expose snapshot, semantic mutation/diff, inference, policy, quantities/diagnostics and the two services. External math/acceleration libraries, if used, remain behind local traits; no foreign type appears in these APIs.

## Existing Files That Must Change

No new production file/folder is required by this workstream; the repo instruction requires extending existing files with regions. Conceptual modules above are region boundaries.

Exact path prefix `G` below is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any`. “Every facet” means every existing `component.*` file in the named exact directory and its existing `📝️text/` and `💾️binary/` children; this is intentionally exhaustive across Rust, TypeScript, GraphQL, JSON Schema, Proto, EBNF, ANTLR, Semio grammar/protocol, ABNF, Kaitai and Spicy.

1. Snapshot and validation: `G/🧬️schema/📸️snapshot/` every facet. Keep the persisted glTF shape; add local address/resource-status/typed-view contracts outside serialized document state where possible.
2. Accessor/GLB codec: `G/🚪️io/🦀️component.rs`, `G/🚪️io/🟦️component.ts`, `G/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`, and `G/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`. Add resolved-resource states, typed accessor/index decoding, exhaustive validation and exact normalization/sparse/stride behavior.
3. Inference root and numeric kernel: `G/🧬️schema/💡️inferences/` every facet, including existing `📦bounds/🦀️component.rs` and `📦bounds/🟦️component.ts` (repurpose as the regioned geometry-analysis kernel and types).
4. Diff: `G/🧬️schema/🔺️diff/` every facet. Replace weak/index-only patches; add rejection/remap/reference repair/touched paths and `DiffRegions`.
5. Mutations: `G/🧬️schema/🧬️mutations/` every facet. Retire `SetSnapshot` and all Insert/Remove/Set whole-record variants. Reuse the six existing `📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/{🦀️component.rs,🟦️component.ts}` files as the regioned semantic triad service/contract rather than leaving a forbidden whole-document mutation.
6. Artifact/schema/public assembly: `G/🧬️schema/` root `🦀️component.rs`, `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`; `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️component.rs`; `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🟦️component.ts`; and generated `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`. Descriptor must expose the new inference schema/version and public re-exports; glue is regenerated/updated after leaf contracts settle.
7. Tests/fixtures: extend tests in the same Rust roots above and `G/📚️examples/🌱️metabolism/🧪️tests/🦀️test.rs`; do not add test files. Cover cube/open plane/disconnected shells/concave mesh/hollow shell/nonmanifold/degenerate/sparse/interleaved/instanced/transformed/unresolved-resource cases using the existing fixture/example locations.
8. Task registration: if new executable verification commands are needed, extend `✏️s/🔌️plugins/🗄️stdio/📦️packages/{🦀️rust,🟦️typescript}/📜️script.ts`, route through nx/bun, and register the command in the existing `.vscode/launch.json` ordering. No standalone scripts.

## Ownership Boundaries and Integration Order

- **Contracts/schema owner:** snapshot auxiliary types, all Rust/public cross-language shapes, IDs/versions, descriptor and facet parity. Does not implement geometry algorithms or command application.
- **Geometry owner:** accessor normalization, canonical primitive/topology/BVH stages and all indicators. Consumes the contract; does not edit mutation/diff codecs.
- **Mutation owner:** semantic commands, referential-integrity planner, diff/inverse/absorb, touched paths and op codecs. Does not define metric math.
- **Integration owner:** GLTF/GLB resource codec, artifact declaration/re-exports, glue, fixtures and nx/bun/launch verification. Integrates only after contract IDs and command tags freeze.

Order: freeze value schemas and validity rules → implement decoded geometry/validation → implement dependency stages and indicators → implement semantic mutation/diff/inverse → regenerate every facet/glue → run unit/law/schema/grammar/protocol tests → run nx target through bun and verify runtime logs on real GLB fixtures. Schema version must bump whenever numeric semantics, tolerances, histogram edges or dependency reads change.
