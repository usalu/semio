# Semio first-party B-Rep kernel audit

**Repository:** `usalu/semio`  
**Branch requested:** `🐙ueli/⛳wip`  
**Commit audited:** `7ad363fd1ec91cb0c83cf716bc66522be99a4785`  
**Audit type:** static, source-level architecture and implementation review  
**Primary question:** can the in-monorepo B-Rep stack replace BRepJS, BRepKit, OpenCascade, and similar external kernels for procedural 3D, CAD, viewers, editors, interchange, and downstream geometry applications?

> **Bottom line:** not yet. The repository contains a credible first-party kernel foundation and a broad procedural API, but the production-reachable precise path still uses BRepJS/OpenCascade. The new Rust implementation is presently an MVP/approximate kernel: many apparently exact B-Rep operations degrade the model to tessellated triangle soup, use fixed sampling, use AABB or convex-hull substitutes, omit stable topological history, or are not connected to the artifact viewer/editor. It is suitable for experimentation, basic primitives, lightweight procedural previews, and protocol development—not yet for replacing an industrial CAD kernel.

---

## 1. Scope, method, and confidence

This report examines the exact commit named above, not a moving branch head. It traces:

- the TypeScript spatial-kernel interface and its BRepJS/OpenCascade adapter;
- the newer first-party Rust B-Rep implementation now housed in the stdio `semio/v1/brep` artifact;
- its snapshot/topology model, geometry evaluators, algorithms, handles, mutations, inferences, validation, tessellation, rendering/editor shells, STEP I/O, and tests;
- the Flow procedural B-Rep extension that exposes the kernel to applications;
- repository dependency declarations showing what remains production-reachable;
- the functional gap against the capabilities an OpenCascade-class replacement must provide.

The audit is **static**. I inspected the source and test code at the pinned commit, but could not execute the monorepo build or test suite in this environment because the repository could not be cloned into the execution sandbox. Therefore:

- “implemented” means source code exists and is connected at the code level;
- “tested” means a test exists in the source, not that it passed here;
- performance and readiness percentages below are reasoned estimates, not benchmark results;
- absence means “not found in the audited source paths,” not proof that no experimental code exists elsewhere.

Confidence is high for architectural conclusions, dependency status, and explicitly documented approximation paths; medium for behavior that would ideally be confirmed by execution.

---

## 2. Executive verdict

### 2.1 The current system is a dual stack

At this commit Semio has two materially different B-Rep paths.

#### A. Current production/application path

The TypeScript spatial kernel has a compact `SpatialKernel` contract and a `brepjs` implementation. That implementation identifies itself as `brepjs-opencascade`, imports the BRepJS/OpenCascade WASM stack, and exposes the expected modeling operations through a worker-backed adapter.

The repository dependency inventory marks both:

- `brepjs` `^18.20.3`
- `brepjs-opencascade` `^0.15.6`

as **production-runtime** and **productionReachable** for the CAD TypeScript package.

This means the precise, currently wired browser/CAD path is still external-kernel-backed. “Dependency-free” is false today in the most important operational sense.

#### B. First-party Rust path

A newer native implementation lives under the stdio Semio B-Rep artifact. It includes:

- a native B-Rep body/topology representation;
- analytic and NURBS curve/surface types;
- generational arenas;
- tolerances and persistent labels;
- a broad synchronous kernel trait—described in migration comments as 93 methods;
- primitives, transforms, sweeps, booleans, blends, offsets, classification, tessellation, mass properties, intersections, validation, and mesh/STEP I/O;
- a stable artifact snapshot and mutation protocol;
- a Flow extension exposing procedural nodes;
- viewer/editor/bridge/oracle/test scaffolding.

This is substantial. It is not a toy API surface. However, its implementation depth is far behind its interface breadth.

### 2.2 Replacement readiness

My source-based readiness assessment:

| Area | Readiness | Assessment |
|---|---:|---|
| Topological and geometric data model | **70%** | Good foundation; explicit coedges, optional p-curves, analytic/NURBS supports, arenas, labels. |
| Artifact schema, persistence, and primitive mutations | **60%** | Coherent snapshot/mutation protocol, but incomplete subshape identity and feature-history integration. |
| Basic primitives and elementary evaluation | **55%** | Boxes and several analytic supports exist; topology and sampling quality vary. |
| Procedural graph/API integration | **50%** | Broad Flow node surface and happy-path tests; inherits algorithmic limitations. |
| Tessellation for preview | **40%** | Serious implementation exists, but robustness around trims, projection, seams, and fallback triangulation is insufficient. |
| General free-form geometry | **25%** | NURBS storage/evaluation exists, but inverse evaluation, derivatives, intersections, and trimming are immature. |
| Exact booleans and production modeling features | **15%** | General operations are mesh/centroid/hull approximations; exact B-Rep reconstruction is absent. |
| STEP interoperability | **25%** | Useful AP214-shaped subset; important orientation, p-curve, unit, assembly, attribute, and healing gaps. |
| Interactive CAD viewer/editor | **5–10%** | Placeholder boxes and no-op edit commands, not actual B-Rep rendering/editing. |
| Industrial tolerance, healing, and robustness | **10–15%** | Validation exists, but there is no production-grade tolerance propagation, sewing/healing, singularity handling, or adversarial robustness. |
| **Overall OpenCascade/BRepJS replacement readiness** | **20–30%** | Strong foundation and integration experiment; not a production substitute. |

These percentages are qualitative and intentionally conservative.

### 2.3 Practical answer by application

- **Basic procedural 3D previews:** partially usable now for boxes, simple primitives, straightforward extrusions, and operations where approximation is acceptable.
- **Parametric CAD authoring:** not ready. Exactness, naming/history, robust booleans, feature regeneration, and tolerance management are missing.
- **Interactive CAD editor:** not ready. The B-Rep artifact’s viewer/editor do not render or mutate the real model.
- **STEP viewer:** parsing/export code exists, but the artifact viewer is not wired to real shape tessellation and the STEP subset is incomplete.
- **Manufacturing/CAM:** not ready. Manifold guarantees, geometric accuracy, healing, and deterministic exact boundaries are insufficient.
- **CAE/analysis:** not ready as a general source of analysis-grade geometry.
- **Browser CAD without OpenCascade WASM:** not yet demonstrated. The current precise browser path remains BRepJS/OpenCascade.
- **Native kernel R&D:** this is the strongest use today; the code is a useful first-party architecture and oracle-development base.

---

## 3. Architecture map

### 3.1 TypeScript spatial-kernel layer

The spatial-kernel engine is divided into:

- `geometry`: public model/value types, model diffing, and transfer structures;
- `spatial`: the small public kernel abstraction;
- `brepjs`: the concrete BRepJS/OpenCascade implementation and compatibility layer.

The public contract is intentionally small: kernel identity, operation metadata, box/volume/tessellation, and query/action entry points. That is a reasonable application boundary, but it hides whether an operation is exact, approximated, tessellated, or delegated.

The `geometry` model/diff layer is not itself a geometric kernel. It can mutate an application model and compute inverses, but it does not supply robust B-Rep algorithms.

### 3.2 Framework 3D layer

The Rust `framework/3d` code retains shared geometry/mesh types and migration commentary. The comments identify prior or shared modules for:

- BVH;
- spatial queries;
- offset;
- mesh I/O;
- classification;
- tessellation;
- booleans.

They also state that the consumer contract and 93 synchronous methods were moved into the stdio B-Rep artifact. This confirms a migration rather than a completed consolidation. Some kernel code still depends back on framework-level types.

### 3.3 Stdio Semio B-Rep artifact

The first-party implementation is organized as an artifact with:

- `schema/snapshot`
- `schema/mutations`
- `schema/inferences`
- `schema/diff`
- `schema/engine`
- `viewer`
- `editor`
- `bridge`
- `io`
- `generator`
- `oracle`
- fixtures and tests.

This is a strong product architecture: persistence and protocol are separated from algorithms and UI adapters. The problem is not the folder structure; it is fidelity, identity, and incomplete end-to-end wiring.

### 3.4 Flow procedural extension

The Flow B-Rep extension imports the stdio B-Rep kernel trait and exposes a broad node catalog across:

- primitives;
- curves;
- surfaces;
- transforms;
- sweeps;
- booleans;
- blends/features;
- intersections;
- evaluation;
- measurements;
- topology deconstruction;
- tessellation;
- import/export.

This gives the new kernel a real procedural application surface. It also makes the approximation issue more dangerous: broad API coverage can look like parity even when implementations silently convert to meshes or substitute hulls.

### 3.5 Viewer/editor artifact shells

The artifact has viewer and editor modules, but these are currently protocol/UI scaffolds. They do not form a real render/edit path for the B-Rep snapshot.

---

## 4. What exists: data model and kernel foundation

### 4.1 Native topology

The native body contains generational arenas for:

- vertices;
- edges;
- coedges;
- loops;
- faces;
- shells;
- solids;
- 3D curves;
- 2D curves;
- surfaces.

Important strengths:

1. **Explicit coedges.** Edge use is represented separately from the edge, with orientation and loop linkage.
2. **2D p-curve slot.** A coedge can carry a curve in surface parameter space.
3. **Loop rings.** Coedges have next/previous links.
4. **Outer and inner loops.** Faces distinguish an outer loop and holes.
5. **Face orientation and tolerances.**
6. **Shell and solid aggregation.**
7. **Generational IDs.** Arena recycling is guarded by generation values in the native structure.
8. **Seed/rebuild representation.** The code has a persistence-oriented seed that can recreate the body while preserving labels and high-water information.
9. **Persistent labels.** Major topological entities carry labels intended to survive mutation/history.

This is materially closer to a real B-Rep model than a half-edge triangle mesh.

### 4.2 Geometry supports

The model includes analytic and free-form geometry.

#### Curves

- line;
- circle;
- ellipse;
- Bézier/B-spline/NURBS-like representations;
- 2D and 3D curve forms;
- conversion/evaluation utilities.

#### Surfaces

- plane;
- cylinder;
- cone;
- sphere;
- torus;
- NURBS surface.

This is the right vocabulary for CAD and STEP.

### 4.3 Tolerance framework

Entities carry tolerance values and validators compare topology/geometry within tolerance. This is necessary groundwork.

However, the current implementation often bypasses that framework with hardcoded constants such as `1e-6`, `1e-4`, `1e-5`, and fixed mesh deflection values. Tolerance exists as a type-level concept but is not yet consistently propagated through all algorithms.

### 4.4 Persistent labels and history structures

The topology module contains persistent-label and history-related structures. This shows that topological naming was considered rather than ignored.

The gap is behavioral: high-level algorithms do not consistently emit a complete generated/modified/deleted mapping, and the engine’s public handles are not those persistent labels.

### 4.5 Euler and sewing modules

Low-level topology operators and sewing code exist. This is valuable because an exact B-Rep kernel ultimately needs topology-first operators rather than “rebuild every result from triangles.”

The audit did not find evidence that the major high-level operations—general booleans, transforms, sweeps, blends, offsets—systematically use those Euler operators to preserve topology, p-curves, tolerances, and history.

### 4.6 Broad kernel contract

The native engine exposes a large synchronous API covering:

- lifecycle and handle management;
- primitive construction;
- curve/surface construction;
- transforms and copies;
- linear/circular/grid patterns;
- extrusion, revolution, loft, sweep, pipe, helix;
- union/intersection/difference;
- fillet/chamfer;
- shell, offset, thicken, draft;
- section/split;
- curve/surface evaluation;
- volume, area, length, bounds, distance, classification;
- validation;
- topology deconstruction;
- tessellation;
- STEP and mesh serialization.

Breadth is a strength for application integration, but it should not be mistaken for implementation parity.

---

## 5. Handle, ownership, and persistence model

### 5.1 Public handles are ephemeral registry tokens

The native engine has a mutable `Brep` object containing:

- a `Body`;
- a `live` hash map from public handle to entity;
- a monotonically increasing counter.

A handle is minted by hashing:

- entity kind;
- counter;
- an entity tag derived from an arena ID.

Consequences:

1. Equivalent geometry recreated later receives a different handle.
2. Deconstructing the same body repeatedly can mint new handles repeatedly.
3. Public handles are not stable content hashes.
4. Public handles are not the same thing as persistent topological labels.
5. The counter is process/session state.
6. A serialized artifact cannot safely use these handles as durable references.
7. Importing a new body must carefully invalidate all old handles.

### 5.2 Import lifecycle defect

The STEP import path replaces the body but, in the audited implementation, does not clear the live handle map. Old handles can therefore remain registered against IDs from the previous body. Even if generation checks reject some accesses, this is the wrong lifecycle boundary and can cause stale identity, leaks, or accidental aliasing.

### 5.3 Dispose/retain do not reclaim topology

`dispose` and `retain` manipulate the public live-handle map. They do not garbage-collect unreferenced arena entities from the underlying B-Rep body.

For a long-running procedural or interactive CAD process, this means:

- transient operations can grow the body;
- disposing a handle is not equivalent to deleting geometry;
- memory behavior depends on full-body replacement/reset rather than ownership;
- object lifetime is not obvious to clients.

### 5.4 Missing handle kinds

The public handle enum covers vertex, edge, wire, face, solid, curve, and surface. Shell and compound are not first-class in the same way, despite the native topology model containing shells and high-level operations producing compound-like collections.

### 5.5 Required fix

A production design should separate three identifiers:

- **ephemeral process handle:** fast runtime token with generation;
- **document-scoped persistent label:** stable identity across edits and serialization;
- **geometric signature/history relation:** generated/modified/deleted correspondence after topology-changing operations.

They must not be conflated.

---

## 6. Algorithm audit

### 6.1 Primitive construction

#### Box

Boxes appear to be the strongest case: planar analytic faces and simple topology. Several fast paths elsewhere also recognize box/AABB geometry.

**Status:** useful, likely adequate for basic tests and procedural demos.

#### Sphere

The surface support is analytic, but the topology uses sampled/polyline-like equatorial structure rather than a fully robust periodic/singular parametric construction.

**Status:** analytic support exists; topology and seam/pole robustness are incomplete.

#### Cylinder and cone

Analytic supports and seam concepts exist.

**Status:** promising for elementary shapes; must still be validated for orientation, p-curves, singular/apex handling, and downstream booleans.

#### Torus

The torus primitive samples major/minor directions and rebuilds triangle-soup faces even though the model has an analytic torus surface type.

**Status:** visually useful, not an exact toroidal B-Rep result.

#### Triangle-soup reconstruction

The helper quantizes vertices around `1e-6` and creates a planar face for each triangle. This converts smooth analytic geometry into a faceted B-Rep:

- every mesh triangle becomes a face;
- tangent continuity is lost;
- original curves/surfaces are lost;
- topology count explodes;
- later feature selection and persistent naming become unstable;
- STEP export writes faceted planes rather than the intended analytic object.

This helper is central to many operations, making it one of the largest architectural blockers.

---

### 6.2 Transformations and copies

The general transform helper:

1. tessellates a solid at fixed deflection;
2. transforms mesh positions;
3. reconstructs a B-Rep from triangle soup.

It is used by rotation, scale, mirror, copies, and pattern operations. Translation also follows the mesh-transform path in the audited engine.

This is far below expected CAD behavior. A rigid transform should preserve:

- curve and surface types;
- trim domains;
- p-curves;
- topology graph;
- persistent labels/history;
- tolerances;
- face count.

Additional API issues:

- rotation uses the bounding-box center because the public API lacks an explicit origin;
- invalid/default axis behavior falls back to Z;
- pattern operations repeatedly transform and fuse, compounding approximation;
- copy/deconstruct can mint new handles without stable identity.

**Required:** exact affine transformation of all geometric supports and topological locations, with history mapping and no tessellation.

---

### 6.3 Extrusion, revolution, loft, sweep, pipe, and helix

The sweep module provides broad coverage, but uses sampled construction and triangle soup.

Observed behavior includes:

- fixed or capped section counts;
- revolve sampling in the approximate range of 8–64 sections;
- sweep sampling around 16 samples per edge;
- helix sampling around 16–128 sections;
- ad hoc moving-frame construction;
- loft requiring compatible/equal discretized profile vertex counts;
- `smooth` accepted but ignored;
- pipe guide accepted but ignored;
- fan caps;
- placeholder/raw topology IDs in intermediate structures;
- NURBS path evaluation in one path returning the origin rather than evaluating the curve.

The result is a mesh-like skin wrapped in B-Rep containers, not a true swept analytic/trimmed solid.

**Consequences:**

- section topology can be wrong;
- twisting/frame singularities are not robust;
- profiles with holes are problematic;
- guide curves and continuity requests are not honored;
- face provenance is lost;
- downstream booleans operate on already faceted results;
- parameter editing cannot preserve subshape identity.

**Required:** exact sweep surface construction, edge/face correspondence, trim generation, cap construction, singularity handling, continuity control, and generated-subshape history.

---

### 6.4 Boolean operations

This is the decisive parity gap.

The boolean module describes an exact pipeline aspiration, but the implemented general path is a mesh boolean:

1. tessellate both operands;
2. classify triangle centroids against the other solid;
3. keep or discard entire triangles by operation;
4. reconstruct triangle soup;
5. use AABB or convex-hull recovery/fallback paths in some cases.

It does **not** generally:

- calculate exact surface/surface intersection curves;
- create matching 3D curves and p-curves on both faces;
- split source edges and faces at intersection topology;
- classify exact cells/regions;
- select and stitch result patches;
- sew and heal the output with tolerance propagation.

An entire triangle is kept or removed based on its centroid. A triangle crossing the true intersection is not split at the true curve. Therefore output accuracy depends on tessellation density and can be topologically wrong.

There are special AABB/box fast paths. Those can make box tests look much stronger than general behavior.

Mesh deflection is clamped with a fixed lower bound, and booleans use hardcoded tolerance defaults.

**Status:** acceptable as an approximate CSG preview for simple meshes; not an exact B-Rep boolean kernel.

**Minimum replacement milestone:** general plane/cylinder/cone/sphere/torus/NURBS intersection, face splitting, region classification, topology reconstruction, p-curves, and history—without tessellation as the authoritative path.

---

### 6.5 Fillet and chamfer

The blend implementation explicitly characterizes itself as an MVP approximation. It samples a small number of stations and arc points, then uses convex hull / triangle-soup reconstruction. Exact rolling-ball surfaces and robust trimming are deferred.

Additional issues:

- variable radius is not established;
- continuity control is absent;
- corner/vertex blend resolution is absent;
- asymmetric chamfer accepts a second distance but ignores it;
- empty edge lists mean “all edges,” which is convenient but risky without deterministic selection/history;
- failure behavior is not surfaced with a precision/capability contract.

**Status:** shape-rounding visual approximation, not a production fillet/chamfer operator.

---

### 6.6 Offset, shell, thicken, and draft

#### Face offset

General implementation is planar-only. Non-planar faces error.

#### Solid offset

The generic path:

1. tessellates the shape;
2. offsets mesh points;
3. includes expanded AABB corners;
4. computes a convex hull.

This changes concave topology and does not represent an offset surface.

#### Thicken

Planar cases use extrusion. Non-planar cases use hull-like approximation.

#### Shell

The operation constructs/cuts hull-like tools and can silently continue when cuts fail.

#### Draft

The implementation is explicitly an MVP AABB shear for boxes. Non-box input can return a copied solid rather than a drafted result. The neutral point is ignored and only the first face is materially considered in the audited path.

**Status:** prototype approximations only.

A correct offset/shell/draft subsystem requires:

- analytic and NURBS offset surfaces;
- edge offset/intersection and corner resolution;
- p-curve recomputation;
- self-intersection detection and trimming;
- face removal with open-shell closure;
- tolerance propagation;
- draft about explicit neutral plane/line with face-chain propagation;
- deterministic failure diagnostics.

---

### 6.7 Intersection and projection

Strengths:

- analytic line-line, line-circle, and circle-circle logic;
- selected analytic surface pairs, such as plane-plane, plane-cylinder, plane-sphere, and sphere-sphere;
- NURBS/subdivision/Newton scaffolding;
- a general intersection-curve representation.

Limitations:

- general surface/surface cases fall back to fixed sampling;
- fallback grids use hardcoded domains and sample counts;
- resulting intersection curves carry a 3D curve but p-curves on both supports are deferred;
- projection often starts from `(0,0)` with a small fixed iteration count and finite-difference step;
- domain/periodicity handling is weak;
- public closest-parameter and closest-UV engine methods return `None`;
- NURBS UV projection in classification paths can fall back to the lower-domain corner.

Without reliable inverse evaluation and paired p-curves, robust face trimming and exact booleans cannot be built.

---

### 6.8 Curve and surface evaluation

The schema can store NURBS-like geometry and provides evaluation utilities. This is an important foundation.

Gaps:

- some interpolation simply treats input points as control points with clamped uniform knots rather than solving a true interpolation system;
- approximation downsamples rather than optimizing an error-bounded fit;
- NURBS surface derivatives are finite differences with a fixed step and are documented as unsuitable for tight Newton iteration;
- closest parameter/UV is not implemented at the engine API;
- periodicity, seam normalization, singularities, rational weight conditioning, and domain clamping are incomplete;
- one sweep path’s NURBS evaluation returns the origin.

**Required:** mathematically consistent curve/surface evaluation through at least second derivatives, robust inverse evaluation, periodic domains, rational conditioning, knot insertion/removal, degree elevation/reduction, trimming, and tolerance-aware solvers.

---

### 6.9 Tessellation

This is one of the more serious first-party modules.

Positive features:

- attempts to share edge boundary samples;
- triangulates in UV space;
- supports holes;
- refines curved interiors;
- evaluates surface normals;
- exposes face groups;
- includes tests for boxes, normals, single faces, and curved-density behavior.

Risks and gaps:

- surface projection is used to obtain UVs, inheriting inverse-evaluation weaknesses;
- periodic seams and singularities are not robustly handled;
- curved/non-planar interiors are refined, but planar trim quality still depends on polygon triangulation;
- ear clipping falls back to a triangle fan on failure, which is invalid for many concave or holed polygons;
- winding is inferred from an early triangle and then globally flipped;
- analytic segment counts are heuristic;
- NURBS/torus/cone refinement is rough;
- face-group identifiers are raw arena IDs rather than durable labels;
- no demonstrated crack-free multi-face adaptive refinement under arbitrary trim mismatch;
- no error certificate for chordal, angular, or normal deviation.

**Status:** promising preview tessellator, not yet a production CAD tessellator.

---

### 6.10 Point classification and BVH

The newer classifier builds a face BVH, but the classifier function receives it as an unused argument and scans every face. This means the acceleration structure is not actually used in the decisive query path.

Additional issues:

- one positive hit per face is used in ray counting;
- non-planar face/loop handling uses fixed edge sampling;
- stored coedge p-curves are ignored and 3D boundaries are reprojected;
- NURBS projection may use the lower-domain corner;
- non-planar intersection can default to empty on error;
- tolerance is often hardcoded.

A separate older classifier exists in the mass-properties module. It uses coarse surface-grid ray tests and very tight fixed thresholds. The coexistence of two classifiers is a correctness and maintenance risk.

**Required:** one authoritative classifier built on exact/interval surface intersections, trim-domain tests using stored p-curves, robust grazing/vertex rules, BVH traversal, and deterministic tolerance policy.

---

### 6.11 Mass properties and distance

The code contains:

- area;
- volume;
- centroid;
- bounding box;
- edge length;
- point/shape and solid/solid distance scaffolding;
- divergence-theorem/quadrature approaches;
- selected analytic special cases such as sphere volume.

Limitations:

- default tolerances are hardcoded;
- bounding boxes can be sampling-based;
- overlapping-solid distance can sample face points and miss edge-edge/interior extrema;
- non-planar closest points may be computed on the untrimmed support surface;
- classification used by mass properties is duplicated and coarse;
- there is no demonstrated certified error bound.

**Status:** useful metrics for simple geometry, not yet analysis-grade.

---

### 6.12 Validation, sewing, and healing

Artifact validation checks meaningful invariants:

- referential integrity;
- coedge ring consistency;
- loop closure;
- excessive edge valence;
- tolerance containment;
- sampled same-parameter checks when a p-curve is present.

But:

- missing p-curves are skipped rather than rejected or repaired;
- same-parameter sampling is sparse;
- general self-intersection is not fully checked;
- manifold orientation, shell closure, singularities, tiny/sliver topology, degenerate edges, periodic seams, and face-face overlap are incomplete;
- there is no comprehensive shape-healing pipeline;
- there is no robust gap closing/sewing with tolerance escalation and provenance;
- silent approximations can produce shapes the validator is not strong enough to reject.

Compared with industrial kernels, healing is a major missing subsystem, not an optional polish item.

---

## 7. Mutation model

### 7.1 Existing stable mutation verbs

The B-Rep artifact defines 13 entity-level mutation variants:

1. create vertex;
2. delete vertex;
3. move vertex;
4. create edge;
5. delete edge;
6. replace curve;
7. create face;
8. delete face;
9. replace surface;
10. create shell;
11. delete shell;
12. create solid;
13. delete solid.

This is a sensible minimal protocol for snapshot evolution.

### 7.2 Why loops and coedges are absent

The code explicitly avoids loop/coedge mutation verbs because those entities lack persistent labels and their arena IDs can recycle. That is an honest design constraint, but it means the most important incidence-level edits are not directly addressable.

### 7.3 High-level operations are not represented as semantic mutations

The intended architecture appears to be:

- execute a boolean/sweep/offset/fillet/Euler operation;
- compile its result into a grouped set of primitive mutations.

The audit did not find a complete end-to-end implementation that:

- maps every generated/modified/deleted subshape to persistent labels;
- emits the grouped mutation batch;
- applies it through the artifact;
- preserves selection references;
- supports undo/redo and feature recomputation;
- resolves concurrent edits or rebases.

The presence of history types does not by itself close this gap.

### 7.4 Referential safety

Representative create/delete mutations perform existence and duplicate checks, but local mutation validation is incomplete.

Examples:

- creating a face does not establish full local consistency of all referenced loops/coedges at mutation time;
- deleting an edge can leave referential questions to whole-snapshot validation;
- deletion inverse logic may remove and reconstruct later entities to preserve ordering, which is expensive and awkward for concurrency.

A production mutation system should enforce or transactionally restore all invariants before commit.

### 7.5 Undo/redo and topological naming

The generic artifact framework may supply log/commit mechanics, but CAD needs more:

- semantic feature operation records;
- deterministic regeneration;
- topological naming through generated/modified/deleted maps;
- selection rebinding;
- transaction grouping;
- rollback of failed geometric operations;
- merge/conflict policy.

These were not demonstrated for the B-Rep artifact.

---

## 8. Rendering and editor status

### 8.1 Viewer

The top-level viewer command enum contains only `Noop`.

The main viewer window does not tessellate and render the actual B-Rep snapshot. It:

- looks for the largest top-level collection;
- derives a small instance count;
- renders a built-in box mesh;
- spaces placeholder boxes along one axis.

There is no demonstrated:

- snapshot-to-kernel-body conversion;
- kernel tessellation call;
- real face/edge geometry;
- per-face persistent IDs;
- wire/edge overlays;
- selection/picking;
- highlighting;
- display tolerances/LOD;
- sectioning;
- hidden-line/silhouette processing;
- material/color mapping;
- incremental mesh invalidation.

### 8.2 Editor

The editor exposes a `SetVertex`-style action, but its handler emits no mutation. The source comments state that the action cannot honestly be backed by the current mutation schema and is therefore a no-op.

The editor uses the same placeholder-box representation as the viewer.

This means there is currently no real round trip:

`pick rendered subshape → resolve persistent label → preview edit → invoke kernel → validate → emit mutations → retessellate affected faces → commit/undo`

### 8.3 Artifact inferences do not expose tessellation

The B-Rep artifact’s inference layer intentionally exposes validation but omits tessellation and mass properties because the stdio layer cannot evaluate curves and surfaces without depending back on `framework-3d`.

This is an important architectural contradiction:

- kernel algorithms exist;
- artifact UI needs derived mesh;
- dependency direction prevents the artifact contract from exposing it.

Until that is resolved, real viewer/editor wiring remains blocked even before algorithm quality is addressed.

---

## 9. Procedural Flow integration

The Flow extension is broad and materially useful. It exposes the first-party kernel to procedural graphs and includes tests for several happy paths such as:

- box/line creation;
- extrusion;
- area/volume;
- fillet/translation;
- handle retention;
- tessellation memoization;
- topology deconstruction;
- several import/export wrappers.

This is the best integrated first-party application path.

However, it inherits all kernel approximations:

- transforms facet;
- general booleans classify mesh triangles;
- blends use hulls;
- sweeps sample;
- offsets can convex-hull the result;
- topology identity is unstable;
- tolerances are not consistently propagated.

### Procedural-use classification

| Use | Current suitability |
|---|---|
| Visual shape exploration | Moderate for simple models |
| Low-poly / mesh-oriented procedural output | Moderate |
| Exact parametric solids | Low |
| Re-editable feature tree | Very low |
| Manufacturing geometry | Very low |
| Automated CAD generation with STEP delivery | Low |
| Geometry oracle development | Moderate to high |
| Differential testing against OpenCascade | High strategic value |

The Flow integration should remain, but node results must advertise quality and failure mode.

---

## 10. Interchange and serialization

### 10.1 Native codecs

The engine includes code paths for:

- STEP;
- STL;
- OBJ;
- GLB.

Mesh formats are naturally compatible with approximate/tessellated output. STEP is the critical exact-format test.

### 10.2 STEP exporter

The exporter builds an AP214-shaped Part 21 graph with entities for:

- vertices;
- edge curves;
- oriented edges;
- loops and bounds;
- advanced faces;
- shells;
- manifold solids;
- voids;
- analytic and NURBS curve/surface supports.

This is a meaningful implementation, not just a file stub.

Information-loss and standards gaps include:

- shell-face orientation is dropped/normalized;
- axis placements lack full in-plane reference direction because the model does not carry it;
- artifact IDs are not preserved as STEP identity;
- `same_sense` and bound orientation are hardcoded in important places;
- no complete p-curve/same-parameter representation;
- shells are exported as closed shells even when that may not be justified;
- units are not comprehensively negotiated;
- assembly/product structure is absent;
- colors, layers, names, materials, and presentation are absent;
- AP242 PMI/GD&T is absent;
- validation properties and external references are absent.

### 10.3 STEP importer

The importer performs a real entity-graph walk and supports a useful analytic/NURBS subset.

Explicitly documented limitations include:

- omitted polyline/Bézier/offset/trimmed-curve/curve-on-surface representations;
- no assembly/product structure;
- no presentation/colors;
- no validation properties;
- no non-manifold surface representation or geometric sets;
- expectation of SI metre/radian units;
- approximation of B-spline knot information because the internal artifact schema lacks explicit knots in the needed form.

Face/shell orientation is normalized in places and IDs are synthesized.

### 10.4 Snapshot mismatch

There is a representational mismatch between:

- the native `Body`, with generational arenas, coedges, optional p-curves, tolerance, and history structures; and
- the artifact snapshot/STEP-facing schema, which is simpler and ID-keyed.

Any field lost between those models becomes impossible to preserve across:

- load/save;
- mutation replay;
- viewer/editor;
- Flow serialization;
- STEP round trip.

The two representations should be unified or connected by a lossless, versioned mapping with invariant tests.

---

## 11. Dependency status

### 11.1 External runtime CAD dependencies

At the audited commit, the dependency inventory explicitly marks BRepJS and BRepJS/OpenCascade as production-reachable. Therefore the monorepo has **not** removed its external CAD-kernel runtime.

No `brepkit` package entry was found in the checked dependency inventory. In this report “replace BRepKit” is treated as a functional parity target rather than removal of an active package at this commit.

### 11.2 Internal dependency inversion

The first-party B-Rep engine still imports framework-level 3D types such as:

- vector;
- AABB;
- parameter domain;
- mesh transfer.

Topology also imports an OS-kernel representation trait.

The artifact inference code then declines to expose tessellation/mass properties because it cannot depend back on framework 3D. This is a textbook dependency-cycle symptom.

### 11.3 What “dependency-free” should mean

A literal “zero third-party crates/packages” target is neither necessary nor especially valuable. The useful goal is:

> **No runtime dependency on another CAD/B-Rep kernel, no hidden fallback to one, and a kernel core that is independent of application/UI/plugin frameworks.**

Third-party general-purpose utilities can remain if licensed, portable, auditable, and not geometric engines. OpenCascade/BRepJS can remain temporarily as **test-only differential oracles**, but not production runtime.

### 11.4 Dependency-free acceptance criteria

The stack is dependency-free only when:

1. no app imports BRepJS, `brepjs-opencascade`, OpenCascade WASM, or BRepKit at runtime;
2. direct legacy imports are CI-forbidden outside an oracle/compatibility package;
3. the first-party kernel builds as an independent core crate;
4. artifact, Flow, native, and WASM clients use the same implementation;
5. no operation silently delegates to an external kernel;
6. file I/O does not require an external CAD kernel;
7. browser deployment uses a first-party WASM ABI;
8. tests can optionally compare against OCCT, but production bundles exclude it;
9. the artifact can infer tessellation/measurements without violating dependency direction;
10. all supported operations have explicit exactness/capability metadata.

---

## 12. Gap against an OpenCascade-class kernel

An industrial replacement requires more than the presence of similarly named functions.

### 12.1 Geometry mathematics

Missing or insufficient:

- robust rational B-spline/NURBS interpolation and approximation;
- exact derivatives through second order;
- curvature and continuity analysis;
- knot insertion/removal and degree operations;
- periodic/seam-aware evaluation;
- robust closest point/parameter/UV;
- certified root finding and interval methods;
- complete analytic and general curve/curve, curve/surface, and surface/surface intersections;
- singularity and degeneracy treatment;
- offset curves/surfaces;
- complete trimming and p-curve maintenance.

### 12.2 Topology construction

Missing or insufficient:

- exact edge/face splitting;
- common-parameter enforcement;
- p-curves on every relevant coedge;
- deterministic orientation propagation;
- cell/region construction after intersection;
- non-manifold policy;
- shell closure and void orientation;
- complete Euler-operator usage by all high-level algorithms;
- local topology repair after operation.

### 12.3 Modeling operations

Missing or insufficient:

- exact general booleans;
- robust sewing;
- rolling-ball fillet and variable-radius fillet;
- full chamfer variants;
- exact shell/offset/thicken;
- general draft;
- robust sweep/pipe/loft/revolve with continuity and guide laws;
- face removal/defeaturing;
- local operations;
- surface extension and trim repair;
- feature provenance.

### 12.4 Tolerances and healing

Missing or insufficient:

- per-operation tolerance budget;
- tolerance propagation and escalation;
- gap analysis and sewing;
- small-edge/sliver collapse;
- seam and orientation repair;
- same-parameter repair;
- self-intersection diagnosis;
- imported-shape healing;
- validated closed/manifold solid guarantee.

### 12.5 Persistence and document model

Missing or insufficient:

- document-scoped stable subshape IDs;
- complete generated/modified/deleted history;
- deterministic regeneration;
- undo/redo integrated with feature semantics;
- selection rebinding;
- transaction/rollback across failed kernel operations;
- versioned schema migrations;
- stable cross-process handles.

### 12.6 Visualization

Missing or insufficient:

- actual B-Rep-to-mesh inference in the artifact;
- crack-free adaptive tessellation;
- persistent face/edge IDs in render buffers;
- edge curves, silhouettes, hidden-line rendering;
- picking and selection;
- incremental retessellation;
- display modes, section planes, clipping, annotations;
- browser/native renderer integration.

### 12.7 Interchange

Missing or insufficient:

- fuller AP203/AP214/AP242 coverage;
- units and uncertainty;
- assemblies and shared instances;
- names/colors/layers/materials;
- PMI/GD&T and validation properties;
- p-curves/orientation/same-sense fidelity;
- healing and diagnostic import reports;
- broad STEP conformance corpora.

### 12.8 Quality engineering

Missing or insufficient:

- differential regression against the legacy/OCCT stack;
- adversarial geometry corpus;
- property-based topology tests;
- fuzzing parsers and operations;
- metamorphic tests;
- scale sweeps from micro to very large models;
- tangency/coincidence/sliver/seam cases;
- deterministic output checks across native and WASM;
- performance and memory benchmarks;
- crash-free long-running document tests.

---

## 13. Architectural risks

### 13.1 API breadth masks implementation quality

A node or method called `fillet`, `boolean`, `offset`, or `sweep` implies exact CAD behavior to most clients. In the current implementation, the same name may mean “sample and convex hull” or “tessellate and classify centroids.”

This is a product correctness problem, not just an implementation detail.

### 13.2 Silent fallback and silent cloning

Several paths continue after failure, ignore invalid handles, or return a copied input for unsupported geometry. Silent behavior makes downstream defects hard to diagnose.

Every operation should return:

- result;
- quality class;
- achieved tolerance/error;
- warnings;
- provenance/history;
- explicit unsupported/failure status.

### 13.3 Faceted B-Rep contamination

Once an operation converts an analytic solid to per-triangle planar faces, every later operation sees a much larger and lower-quality topology. Repeated transforms/patterns/booleans can cause combinatorial face growth.

### 13.4 Duplicate algorithm generations

Classification exists in multiple modules. STEP shape logic is split between native and artifact representations. Algorithms were migrated from framework 3D into stdio but shared types remain behind. This increases drift risk.

### 13.5 Box-biased test success

AABB/box fast paths and many simple tests can create an overly optimistic impression. Production robustness must be measured on curved, trimmed, tangent, coincident, thin, periodic, and mixed-scale models.

---

## 14. Recommended target architecture

### 14.1 Kernel-owned neutral core

Create a kernel-owned layer independent of stdio, Flow, viewer, and generic framework 3D:

- `brep-math`: scalar policy, vectors, matrices, intervals, roots;
- `brep-geometry`: curves, surfaces, derivatives, inverse evaluation;
- `brep-topology`: arenas, labels, coedges, loops, shells, solids, Euler ops;
- `brep-history`: generated/modified/deleted relations and persistent naming;
- `brep-algorithms`: intersections, split, classify, boolean, sew/heal, features;
- `brep-mesh`: adaptive tessellation and mesh transfer;
- `brep-io-step`: STEP graph and mappings;
- `brep-api`: stable service trait and capability metadata;
- `brep-wasm`: browser ABI;
- adapters for artifact, Flow, renderer, and legacy compatibility.

`Vec3`, `Aabb`, `ParamDomain`, and `MeshTransfer` should live in this neutral layer or a lower math crate, not in an app-oriented framework module.

### 14.2 One authoritative model

Choose one canonical in-memory/persistent shape model.

Preferred approach:

- native `Body` remains authoritative;
- artifact snapshot is a lossless serialization of it;
- coedges, p-curves, tolerances, labels, history, knot vectors, orientations, and high-water/generation information all round-trip;
- STEP and viewer consume the same model.

### 14.3 Explicit capability contract

Each operation should publish:

- supported input classes;
- exact/analytic vs numerical vs mesh fallback;
- tolerance semantics;
- deterministic behavior;
- output manifold guarantee;
- history guarantee;
- platform availability;
- version.

Suggested result quality enum:

- `ExactAnalytic`
- `ExactNumericalWithinTolerance`
- `ApproximateBRep`
- `MeshDerivedBRep`
- `PreviewOnly`
- `Unsupported`

Mesh fallback must be opt-in for CAD callers.

---

## 15. Prioritized replacement roadmap

### Phase 0 — Make the current truth explicit

1. Add capability/quality metadata to all 93 methods.
2. Remove silent copies, ignored arguments, and swallowed failures.
3. Mark mesh/hull implementations as preview-only.
4. Add a CI rule blocking direct BRepJS/OpenCascade imports outside the adapter/oracle.
5. Keep BRepJS/OpenCascade as a test oracle until parity gates pass.
6. Publish an app capability matrix from source, not marketing names.

**Exit gate:** no caller can mistake an approximate result for an exact one.

### Phase 1 — Self-containment and identity

1. Move shared math/mesh transfer types into kernel-owned neutral crates.
2. Remove `framework-3d` and OS-kernel dependency inversion.
3. Unify native Body and artifact Snapshot losslessly.
4. Separate runtime handles from persistent labels.
5. Clear/invalidate handle maps on load/reset.
6. Implement arena reachability/garbage collection.
7. Add shell/compound first-class handles.
8. Make deconstruction idempotent with stable labels.
9. Add transaction-scoped handle validity.

**Exit gate:** native, artifact, Flow, and WASM can all operate on one first-party kernel model without external CAD dependencies.

### Phase 2 — Geometry correctness

1. Complete rational NURBS curve/surface representation, including explicit knots.
2. Implement stable first/second derivatives.
3. Implement robust closest parameter and closest UV.
4. Implement periodicity, seam normalization, and singularity handling.
5. Implement error-bounded interpolation/approximation.
6. Implement analytic and interval/subdivision intersection solvers.
7. Produce paired p-curves for intersection curves.
8. Implement same-parameter enforcement.

**Exit gate:** geometry primitives and intersections pass a differential corpus against OCCT within declared tolerance.

### Phase 3 — Exact topology and booleans

1. Split edges and faces at exact intersection curves.
2. Classify regions/cells using one authoritative classifier.
3. Use the BVH in actual traversal.
4. Construct and orient result shells.
5. Sew and heal output.
6. Propagate tolerances.
7. Emit complete generated/modified/deleted history.
8. Remove centroid-triangle boolean from exact mode.

**Exit gate:** general curved booleans produce manifold, validated B-Reps without tessellation as the authoritative representation.

### Phase 4 — Exact transforms and modeling features

1. Preserve analytic geometry through affine transforms.
2. Replace sampled extrude/revolve/loft/sweep/pipe/helix.
3. Implement robust guide/frame laws.
4. Implement exact fillet/chamfer, then variable-radius blends.
5. Implement general offsets, shelling, thickening, and draft.
6. Implement defeaturing and local operations.
7. Preserve history and labels through every feature.

**Exit gate:** a representative parametric part corpus regenerates deterministically and retains selections.

### Phase 5 — Real rendering and editing

1. Expose `Snapshot -> MeshTransfer` as an artifact inference.
2. Include persistent face/edge labels in render buffers.
3. Replace placeholder boxes with real tessellation.
4. Add edge overlays, picking, highlighting, sectioning, and LOD.
5. Compile editor commands into validated mutation batches.
6. Add preview/commit/cancel and undo/redo.
7. Incrementally retessellate only affected faces.

**Exit gate:** the B-Rep artifact can open, display, select, edit, undo, save, reload, and preserve subshape identity.

### Phase 6 — STEP and interoperability

1. Preserve p-curves, orientation, same-sense, units, and uncertainty.
2. Support assemblies and shared instances.
3. Add names, colors, layers, materials, and validation properties.
4. Add required AP203/AP214/AP242 subsets.
5. Add import healing and structured diagnostics.
6. Run broad conformance and round-trip corpora.

**Exit gate:** real-world STEP files round-trip with quantified information loss and no external CAD kernel.

### Phase 7 — Remove legacy runtime

1. Route all apps through first-party native/WASM adapters.
2. Compare output against BRepJS/OpenCascade in CI.
3. Freeze legacy feature development.
4. Remove productionReachable flags.
5. Delete runtime dependencies only after app-specific parity gates pass.
6. Retain optional oracle tooling outside production bundles.

**Exit gate:** no production artifact or application contains BRepJS/OpenCascade/BRepKit runtime code.

---

## 16. Verification program

### 16.1 Differential tests

Run each first-party operation against the existing BRepJS/OpenCascade path while it remains available.

Compare:

- validity;
- solid count;
- topology counts;
- area/volume/bounds;
- Hausdorff distance;
- point classification;
- manifoldness;
- STEP round-trip;
- generated/modified/deleted history;
- deterministic IDs.

Differences should be categorized, not merely snapshot-updated.

### 16.2 Property tests

Examples:

- transform followed by inverse returns equivalent shape;
- `A ∪ A = A`;
- `A ∩ A = A`;
- `A − A = empty`;
- boolean commutativity where applicable;
- volume monotonicity;
- tessellation converges as tolerance tightens;
- STEP export/import preserves declared invariants;
- every closed solid has expected edge valence and consistent orientation;
- mutation followed by inverse restores canonical snapshot.

### 16.3 Adversarial corpus

Include:

- tangent and nearly tangent surfaces;
- coincident faces/edges;
- tiny sliver faces;
- very short edges;
- periodic seams;
- sphere/cone poles;
- high-degree rational NURBS;
- holes and nested loops;
- thin walls;
- disconnected compounds;
- non-manifold inputs;
- scales from approximately `1e-9` to `1e9`;
- malformed STEP;
- repeated edit/regenerate cycles.

### 16.4 Fuzzing

Fuzz:

- mutation streams;
- STEP parser/entity graph;
- curve/surface parameters;
- boolean operand placement;
- trim loops;
- tessellation tolerances;
- handle lifecycle;
- serialization/deserialization.

### 16.5 Performance and memory

Track:

- operation latency by topology complexity;
- peak memory;
- arena growth and reclamation;
- tessellation cache behavior;
- BVH build/reuse;
- native/WASM parity;
- long-running interactive session growth;
- deterministic output across thread counts/platforms.

---

## 17. App-specific release gates

### 17.1 Procedural 3D preview

Can ship earlier with explicit `PreviewOnly` quality when:

- operations never crash;
- approximations are disclosed;
- result meshes are manifold where promised;
- tolerance visibly controls quality;
- no silent clone-on-failure;
- topology explosion is bounded.

### 17.2 Parametric CAD

Do not replace the legacy kernel until:

- exact transforms;
- exact booleans for target surface classes;
- stable persistent naming;
- feature history and regeneration;
- robust fillet/chamfer/offset;
- import healing;
- undo/redo;
- deterministic results;
- selection persistence.

### 17.3 Interactive viewer/editor

Require:

- real snapshot tessellation;
- persistent render IDs;
- picking/selection;
- editor commands that emit valid mutations;
- incremental updates;
- save/reload identity;
- large-model performance.

### 17.4 STEP interchange

Require:

- units;
- orientation and p-curve fidelity;
- target AP coverage;
- structured unsupported-entity diagnostics;
- conformance corpus;
- quantified round-trip loss;
- import healing.

### 17.5 CAM/CAE/manufacturing

Require the strictest gate:

- validated watertight solids;
- exact or tolerance-certified boundaries;
- robust offsets;
- no preview-only fallback;
- deterministic classification;
- feature-size/tolerance policy;
- audited STEP fidelity.

---

## 18. Recommended immediate backlog

The highest-leverage near-term work is:

1. **Add operation-quality metadata and eliminate silent fallbacks.**
2. **Make the B-Rep core independent of framework/app crates.**
3. **Unify native Body and artifact Snapshot.**
4. **Implement stable document labels and complete operation history.**
5. **Implement exact affine transforms without tessellation.**
6. **Finish closest parameter/UV and p-curve infrastructure.**
7. **Wire artifact tessellation to the real viewer.**
8. **Replace the no-op editor action with a valid mutation path.**
9. **Use the existing BVH in classification and delete the duplicate classifier.**
10. **Build differential CI against BRepJS/OpenCascade.**
11. **Implement one narrow but truly exact boolean slice end to end—e.g. planar/quadric faces—before adding more API names.**
12. **Strengthen STEP units, orientation, p-curves, and diagnostics.**

A narrow exact kernel is more valuable than a broad approximate kernel marketed under exact CAD operation names.

---

## 19. Final assessment

The WIP Ueli branch at commit `7ad363f` contains the beginnings of a serious first-party B-Rep platform:

- the topology model is credible;
- the artifact protocol is thoughtfully structured;
- the kernel API is broad;
- procedural integration exists;
- STEP code is nontrivial;
- there is enough code to support sustained kernel development.

But it is not yet a replacement for BRepJS, BRepKit, or OpenCascade because:

- the current precise production path still links BRepJS/OpenCascade;
- the first-party engine remains internally coupled to framework types;
- general transforms, booleans, sweeps, blends, and offsets are mesh- or hull-derived;
- inverse geometry and p-curve infrastructure are incomplete;
- stable topological naming/history is not connected end to end;
- artifact rendering/editing is placeholder/no-op;
- STEP fidelity and healing are incomplete;
- tests are weighted toward simple/happy paths;
- no industrial robustness or parity gate has been demonstrated.

The correct strategic framing is:

> **Semio already has a viable kernel architecture and a useful first-party experimental implementation. It does not yet have an industrial B-Rep kernel.**

The safest migration is to keep OpenCascade/BRepJS as a quarantined differential oracle while developing the first-party core, expose approximation explicitly, and remove the legacy runtime only after app-specific correctness gates pass.

---

## Appendix A — Capability matrix

| Capability | Exists | Current implementation class | Production replacement blocker |
|---|---|---|---|
| Vertex/edge/coedge/loop/face/shell/solid topology | Yes | Native generational arenas | Lossless artifact mapping and complete invariant enforcement |
| Analytic curves | Yes | Basic evaluators | Robust inverse/derivatives/intersections |
| NURBS curves | Yes | Storage/evaluation; weak interpolation/fit | Full knot/rational math, periodicity, solvers |
| Analytic surfaces | Yes | Plane/cylinder/cone/sphere/torus | Trim/seam/singularity and exact operations |
| NURBS surfaces | Yes | Storage/evaluation; finite-difference derivatives | Exact derivatives, inverse UV, trimming |
| P-curves | Partial | Optional in coedge model | Producers and algorithms do not consistently generate/use them |
| Tolerances | Partial | Stored and validated | Hardcoded tolerances and no propagation budget |
| Persistent labels | Partial | Topology structures | Not aligned with public handles or operation history |
| Primitive box | Yes | Strong special case | General consistency tests |
| Sphere/cylinder/cone | Partial | Analytic support with sampled topology | Seam/pole/pcurve/boolean robustness |
| Torus | Partial | Sampled triangle soup | Exact toroidal topology |
| Transform | API yes | Tessellate-transform-rebuild | Must preserve analytic B-Rep |
| Extrude | Partial | Sampled/triangle-soup route | Exact side/cap surfaces and history |
| Revolve | Partial | Fixed sampling | Exact revolution surfaces and poles |
| Loft | Partial | Compatible sampled profiles | Continuity, guides, topology correspondence |
| Sweep/pipe | Partial | Sampled ad hoc frames | Exact frames/laws/guides |
| Helix | Partial | Fixed sampling | Exact/controlled helical curves/surfaces |
| Boolean | Partial | AABB fast path + centroid mesh boolean | Exact intersection/split/classify/stitch |
| Fillet | Partial | Sample/hull MVP | Rolling-ball, corner resolution, variable radius |
| Chamfer | Partial | Approximate; second distance ignored | Complete definitions and exact trimming |
| Offset/shell/thicken | Partial | Planar or convex hull | General offset surfaces/self-intersection |
| Draft | Partial | Box AABB shear | General neutral-plane draft |
| Section/split | Partial | Mesh/hardcoded tolerance | Exact section curves and topology |
| Tessellation | Partial | UV triangulation and refinement | Robust trims/seams/holes/error bounds |
| Point classification | Partial | Ray logic; BVH unused | One robust authoritative classifier |
| Mass properties | Partial | Quadrature/special cases | Certified accuracy and trimmed supports |
| Distance | Partial | Sampling/support-surface methods | Exact constrained extrema |
| Validation | Partial | Referential/topological checks | Healing, self-intersection, manifold guarantees |
| STEP import/export | Partial | AP214-shaped subset | Units, pcurves, orientation, assemblies, AP242 |
| STL/OBJ/GLB | Yes/partial | Mesh-oriented codecs | Determinism and metadata |
| Artifact mutations | Yes | 13 primitive verbs | Loop/coedge identity and feature history |
| Artifact viewer | Scaffold | Placeholder boxes | Real tessellation/render IDs/picking |
| Artifact editor | Scaffold | No-op command | Real mutation/compiler/undo |
| Flow procedural nodes | Yes | Broad API | Exactness and reliability |
| Native deployment | Partial | Rust code exists | Build/test/release validation |
| Browser/WASM replacement | Not demonstrated | Legacy precise path remains OCCT WASM | First-party ABI, performance, app wiring |

---

## Appendix B — Source index

Paths below are semantic paths with decorative emoji prefixes omitted for readability. All refer to commit `7ad363fd1ec91cb0c83cf716bc66522be99a4785`.

### Legacy/current path

- `s/modules/spatial-kernel/engine/geometry/*.ts`
- `s/modules/spatial-kernel/engine/spatial/*.ts`
- `s/modules/spatial-kernel/engine/brepjs/*.ts`
- root `dependencies.json`

Key evidence:

- small `SpatialKernel` interface;
- BRepJS/OpenCascade implementation and worker wiring;
- production-reachable `brepjs` and `brepjs-opencascade` dependency entries.

### Migration/shared layer

- `framework/modules/3d/engine/*.rs`
- `framework/modules/3d/packages/rust/Cargo.toml`
- root `Cargo.toml` / `Cargo.lock`

Key evidence:

- migration of the 93 synchronous API to stdio B-Rep;
- remaining shared framework types and duplicated/moved modules.

### First-party B-Rep artifact

- `s/plugins/stdio/artifacts/semio/v1/subsets/brep/schema/engine/*.rs`
- `.../schema/snapshot/*.rs`
- `.../schema/snapshot/topology/*.rs`
- `.../schema/snapshot/curve/*.rs`
- `.../schema/snapshot/surface/*.rs`
- `.../schema/snapshot/arena/*.rs`
- `.../schema/snapshot/tolerance/*.rs`
- `.../schema/mutations/*.rs`
- `.../schema/inferences/*.rs`
- `.../schema/diff/*.rs`

Algorithm submodules include:

- boolean;
- blend;
- sweep;
- intersection;
- offset;
- primitives;
- Euler/topology;
- sew;
- BVH;
- classification;
- tessellation;
- mass properties/distance;
- mesh I/O;
- STEP I/O.

### Artifact UI and integration

- `.../brep/viewer/*.rs`
- `.../brep/viewer/main/*.rs`
- `.../brep/editor/*.rs`
- `.../brep/editor/main/*.rs`
- `.../brep/bridge/*.rs`
- `.../brep/io/*.rs`
- `.../brep/tests/*`
- `.../brep/oracle/*`
- `.../brep/generator/*`

Key evidence:

- `Noop` viewer command;
- no-op editor mutation;
- placeholder-box rendering;
- bridge as descriptor/protocol inventory;
- IO composer and grammar/conformance tests.

### Procedural application integration

- `s/plugins/flow/extensions/brep/*.rs`

Key evidence:

- broad procedural node surface;
- direct use of stdio B-Rep kernel;
- mostly simple/happy-path tests.

---

## Appendix C — Decision

**Do not remove BRepJS/OpenCascade from production yet.**

Adopt this policy instead:

- external kernel: quarantined compatibility/oracle only;
- first-party kernel: default for explicitly supported exact operations;
- preview algorithms: opt-in and visibly marked;
- legacy runtime removal: only after per-app parity gates;
- every unsupported operation: fail explicitly rather than silently approximate.

That approach protects users while allowing the first-party kernel to mature into the dependency-independent platform the monorepo is clearly aiming to build.
