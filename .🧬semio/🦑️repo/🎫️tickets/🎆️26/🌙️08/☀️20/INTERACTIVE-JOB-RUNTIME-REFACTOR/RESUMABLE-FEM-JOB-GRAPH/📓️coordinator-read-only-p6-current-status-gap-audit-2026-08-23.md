# Coordinator Read-Only P6 Current Status Gap Audit — 2026-08-23

## Verdict

**Phase 6 remains RED.** The repository contains substantial source-complete FEM job primitives and
historical focused native/release/Wasm evidence, but the current production FEM application mounts
none of those interactive jobs and none of their live visual data. Several supposedly bounded job
steps also still call internally run-to-completion numerical kernels whose cost grows with the
model.

No production, test, verifier, ticket status, or lifecycle file was edited by this audit.

## Current source census

The following implementations exist and retain useful accepted foundations:

- `FemJobGraph`, with ordered stage/checkpoint/progress state;
- first-party incremental `MeshJob` with coarse/refined/final preview vocabulary;
- deterministic partitioned `AssemblyJob`;
- persistent `PcgJob`, `LdltJob`, and `SubspaceIterationJob`;
- `Fem2dLiveVisual` plus deterministic layer encoding; and
- zero decorative `async fn` in the audited algebra/sparse/model/analyses boundaries.

The FEM engine and plugin production trees are clean relative to `HEAD`, and contain zero `[DEBUG]`
markers. Historical reports record focused exactness, cancellation, checkpoint, timing, release,
Wasm, and a 756-test native run; this audit does not invalidate those isolated results.

## Blocking findings

### 1. Zero mounted interactive production route

The complete production caller census is:

- `FemJobGraph::new`: tests only;
- `MeshJob::new`: tests only;
- `AssemblyJob::new`: the synchronous `assemble_system` batch adapter plus tests;
- `PcgJob::new`: the synchronous `pcg` batch adapter plus tests;
- `SubspaceIterationJob::new`: the synchronous `subspace_iteration` batch adapter plus tests;
- `LdltJob::new`: tests only; and
- `render_with_progress(..., Some(...))`: tests only. Production `render` always passes `None`.

There is no FEM tool/job factory, actor operation authority, generation-cancelling product session,
progress-overlay publication, commit validation, or exact close owner connecting model edits to
these jobs. Consequently edits cannot cancel a mounted stale solve, a coarse preview cannot reach a
frame, and the historical timing suites do not exercise a live application path.

### 2. LDLT one-column step is not a bounded micro-cursor

`LdltJob::step` consumes one fuel unit only after `ldlt_column` completes. That function builds a
`BTreeMap`, scans the entire sparse source column, scans every contributing earlier column, scans
each contributor's complete filled column, and emits every accumulator row without a
`StepContext` cursor. A high-fill column can therefore perform model-sized work inside one nominal
job step.

### 3. Subspace iteration is one whole numerical solve opportunity

`SubspaceIterationJob::step` calls `iterate()` once and consumes fuel afterwards. `iterate` applies
the operator and factor solve to the whole basis, orthogonalizes all basis-column pairs, builds and
multiplies dense projected matrices, runs the complete dense Jacobi eigensolve, sorts modes, and
updates all vectors. None of those loops can observe deadline, fuel, or cancellation. One iteration
is useful progress vocabulary but is not an 8 ms-bounded step for adversarial dimensions.

### 4. Mesh constraint recovery and element preparation retain indivisible work

One `MeshJob` opportunity calls `recover_constraint`, whose internal bound is triangle-count
squared; each iteration rebuilds the complete edge map and searches it. It has no retained flip/
edge cursor. Likewise `AssemblyJob::begin_element` computes the full element stiffness matrix,
allocates the complete local matrix backing, and copies every local cell before the cursor-driven
triplet emission starts. Large/high-order elements can exceed the step ceiling before the first
fuel unit is consumed.

### 5. Visual encoding is exposed but not incremental or live

`fem2d_live_visual_layers` sorts and traverses all regions, assembling elements, fields, loads, and
supports in one function. More importantly, no production caller supplies `Fem2dLiveVisual`. The
recorded 256-field timing fixture is positive focused evidence but does not establish the live
coarse-preview-under-50-ms or adversarial no-step-at-8-ms gates.

## Next bounded packets

1. **P6g mounted operation session:** one fixed, generation-tagged FEM job factory/session owned by
   the worker runtime; connect model revision changes to cancel/restart, bridge progress into the
   Phase 2 overlay, validate final commits, and retain all close owners. Mount 2D first, then 3D.
2. **P6h numerical micro-cursors:** split LDLT accumulator/contributor/row work, subspace operator/
   solve/orthogonalization/projected-eigen work, mesh constraint edge/flip work, and element local-
   stiffness preparation below the fuel/deadline checks.
3. **P6i visual publication job:** cursor the region/element/field/glyph encoding and transport the
   exact latest visual lease into the mounted renderer overlay.
4. Add permanent mounted-route predicates and mutations, then rerun focused numerical references,
   adversarial timing, cancellation, deterministic worker-count replay, native release, Wasm, and
   browser-visible preview gates under the single serialized build owner.

No Cargo, Nx, Wasm, browser, runtime, or network command ran during this read-only audit because
overlapping Rust source packets are still active.
