# Native BIM and FEM Exchange Plan

## Owned Files and Interfaces

Add `any/🚪️io/native_graph.rs` for checked ADD2 graph access and construction shared by domain conversions. Add `native_bim.rs` and `native_fem.rs` for source-owned domain conversions, plus `native_domains.rs` for composer/control registry adapters. Mount these beneath the existing `native_exchange` module, keeping native STEP and EXPRESS validation ownership intact. Extend only IFC root declaration/assembly/registry and exact artifact-definition route claims. Add only repository-native model dependencies in the IFC4x3 Rust package; no build-system or enforcement changes.

BIM uses `semio-s-artifact-bim-model-snapshot::...::ModelSnapshot`, `semio-s-artifact-bim-model-schema` values and existing native codecs. Using the BIM artifact root would create an unnecessary plugin dependency; use the existing snapshot/schema component crates. FEM uses `semio-s-artifact-fem-3d-model::...::Fem3dSnapshot`, persistent owned nodes/materials and actual structure records. Domain data is never exchanged through untyped JSON or a new parallel snapshot model.

## Exact Admission

First finish native geometry audit fixes: exact finite Binary64 determinant orientation, exclusive matrix/TRS tiny-scale regression from official Khronos Box, and actual raw JSON/BIN canonical GLB padding. Keep audit repro inputs immutable and document the original tiny repro's matrix-plus-scale conflict.

Then implement native BIM spatial projection matching the already reviewed TypeScript subset: project metadata, geographic site fields, building planar origin/rotation/elevation and explicitly authored storey level/height/cut height. Emit and require `semio_BimStorey` authored properties. Preserve supported values/relationships and diagnose family/geometry omissions. Require identity project WCS/site frames and direct site→building→storey placements on import; reject contradictions instead of inventing level/height. Explicit property units must be compatible lengths or be refused. Unsupported nonspatial authored source content is diagnosed, never represented as invented families.

Next implement native FEM analytical projection with explicit smaller admission if required by the real owned interface: actual nodes, curve-member topology, canonical engineering material/section definitions, supports and global supported loads. No continuum, combinations, factored groups, self weight, member releases/extra connection conditions, unsupported action placements or dimension guessing. Preserve canonical shared section/material identities. Any unsupported necessary analytical semantic refuses atomically. Declare the admitted subset precisely; do not claim complete FEM interchange.

## Registration, Control and Tests

Declare both directions through normal artifact composers and central exact foreign route capabilities; controlled IO entries use the original caller's decoder/encoder and explicit lossy fidelity. Every expensive graph/domain loop checkpoints, with fixed work/count admission and atomic outcomes. Existing synchronous native codec/domain temporary allocation limitations remain documented, without claiming fully funded resumable jobs.

Write native executable tests and language-neutral `.feature` cases before production domain code. Use retained independent IfcOpenShell BIM/FEM generated fixtures for canonical import and emitted-output oracle hooks, official infrastructure/structural fixtures for accurate refusal boundaries, and source-owned snapshot pack roundtrips. Native Rust execution remains blocked by authentic compiler ingress; syntax checks and source/TypeScript oracles cannot be promoted to native execution evidence.
