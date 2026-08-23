# P6h FEM Numerical Microcursor Repair Contract

Date: 2026-08-24
Owner: `/root` coordinator
Verdict: **PREPARED — implementation begins only after P6g source acceptance.**

## Purpose

Remove the model-sized numerical calls that still occur inside nominal one-fuel FEM job steps.
P6h retains the accepted algorithms and numerical order while exposing a persistent cursor for one
semantic scalar, sparse entry, matrix cell, vector element, graph edge, or flip per worker grant.

This packet covers LDLT, subspace iteration, mesh constraint recovery, and element local-stiffness
construction. It does not create a new scheduler and does not replace the mounted P6g operation
session.

## Governing Rule

Fuel is consumed before an admitted semantic unit and no stage falls through after that unit. Every
inner loop with cost depending on nodes, elements, DOFs, nonzeros, basis columns, constraint edges,
or local element order becomes retained state. Deadline and cancellation are observed between
units, including on zero fuel.

Opaque calls are acceptable only when a fixed schema maximum proves their cost independent of
input size and hostile timing confirms the proof. A post-call `consume(1)` is not boundedness.

## P6h1 — LDLT Column Cursor

### Current defect

`LdltJob::step` calls `ldlt_column` and consumes one unit after it returns. That function allocates
a `BTreeMap`, scans the source column, scans all contributing prior columns and each filled entry,
then emits the entire output column. One high-fill column can run model-sized work and allocate
unadmitted map nodes in a single opportunity.

### Required state

Replace the live `ldlt_column` path with a retained state machine:

1. `ReserveColumn`;
2. `SourceEntry`;
3. `ContributorLookup`;
4. `ContributorEntry`;
5. `PivotRead`;
6. `DiagonalCommit`;
7. `EmitRow`;
8. `PublishColumn`; and
9. `CompleteColumn`.

The accumulator is an owned fixed/page sparse workspace indexed by admitted row slots. It must not
use `BTreeMap`, `HashMap`, or a standard-map entry-size estimate. Row presence, value, touched-list,
and generation marks are fixed and credited before the column begins.

One grant reads/accumulates one source/contributor entry, computes one pivot scalar, emits one
output entry, or transfers one column owner. Contributor lookup itself is cursorized; no whole
symbolic list scan is hidden in a helper.

The original live factor remains immutable until one complete candidate column publishes. A stale,
cancelled, singular, overflow, or faulted column retires its workspace one owner per grant.

### Numerical contract

Preserve the exact deterministic source/contributor/row order of the accepted reference kernel.
Checkpoint/restore captures the complete cursor and workspace generation without cloning all prior
columns. Resumed and uninterrupted factors must be bit-identical to the batch adapter for supported
inputs.

## P6h2 — Subspace Iteration Cursor

### Current defect

One `SubspaceIterationJob::step` calls a whole `iterate`: operator/factor application to the basis,
all pairwise orthogonalization, dense projected matrix construction/multiplication, complete Jacobi
eigensolve, mode sorting, vector updates, residuals, and convergence. One iteration is meaningful
progress but not a bounded scheduling opportunity.

### Required stages

Persist at least:

- `ReserveIteration`;
- `ApplyOperatorColumnRow`;
- `FactorForwardEntry`;
- `FactorDiagonalEntry`;
- `FactorBackwardEntry`;
- `OrthogonalizePairElement`;
- `NormalizeColumnElement`;
- `ProjectedMatrixCellEntry`;
- `JacobiFindPairCell`;
- `JacobiRotateCell`;
- `JacobiConvergenceCell`;
- `ModeSortCompare`;
- `ModePermuteElement`;
- `ResidualColumnRow`;
- `ConvergenceMode`; and
- `PublishIteration`.

Every dense/sparse matrix and basis allocation uses exact observed capacity/page admission for the
simultaneous current-plus-candidate iteration. Jacobi search, rotation, and convergence scans have
their own row/column cursors. Sorting uses an owned deterministic incremental sort cursor, not a
whole standard-library sort call.

Preview publication occurs only after a consistent full iteration, but construction of its
eigenvalue/mode/residual owner is separately cursorized and bounded. The former valid preview stays
visible until atomic publication.

### Numerical contract

Preserve stiffness-metric basis semantics, finite `f64::MAX` null/nonpositive sentinel, ordering,
relative residual definitions, and convergence counts. Uninterrupted, restored, and worker-count
variants must be bit-identical where the existing accepted contract is bit-exact; otherwise they
must meet the exact recorded numerical tolerance with a deterministic reduction order.

## P6h3 — Mesh Constraint Recovery Cursor

### Current defect

One mesh opportunity calls constraint recovery whose internal work can be triangle-count squared.
It repeatedly rebuilds the full edge map, searches all edges, and flips without a retained
edge/search cursor.

### Required stages

Retain:

1. `ReserveConstraintWorkspace`;
2. `IndexTriangleEdge`;
3. `SearchConstraintEdge`;
4. `ClassifyIntersection`;
5. `SelectDeterministicFlip`;
6. `ValidateFlip`;
7. `ApplyFlip`;
8. `RetireFormerEdge`;
9. `PublishConstraintProgress`; and
10. `ConstraintComplete`.

The edge authority is an owned fixed/page index with stable keys and exact backing admission. It is
built incrementally and updated per flip; the live route never rebuilds a complete standard map in
one grant. One grant indexes one edge, checks one intersection, updates one adjacency owner, or
performs one deterministic flip transition.

The combined boundary/edge/triangle/refinement/candidate-plus-live working set is admitted before
the constraint starts. Region-scoped cancellation leaves the former valid coarse mesh visible and
retains every candidate owner for one-unit close.

## P6h4 — Element Stiffness Cursor

### Current defect

P6g cursorizes element indices/positions and later reclamation, but Phase 6 acceptance also requires
the element kernel itself to be bounded. A `Stiffness` phase that calls a full high-order element
matrix and consumes one unit afterwards still performs quadrature, Jacobian, material, and every
local cell before yielding.

### Required stages

For every mounted element family, retain:

- reference/quadrature point;
- shape-function derivative scalar;
- Jacobian cell;
- determinant/inverse cell;
- strain-displacement cell;
- constitutive cell;
- local stiffness multiply/accumulate cell;
- body/traction/load contribution cell;
- local-to-global triplet cell; and
- element candidate publication.

One grant advances one scalar/cell/entry. Fixed-size small element families may use a proven opaque
kernel only when the schema maximum and timing mutation demonstrate input-independent work below
the ceiling. High-order/dynamic elements always use the retained path.

Move matrix/vector backing between stages; do not clone a complete local/global stiffness matrix.
Exact candidate and displaced backing credit remains held until publication/retirement.

## Shared Admission and Close

Extend the P6g concrete owner inventory with every simultaneously legal P6h owner:

- LDLT accumulator/touched/symbolic/candidate column;
- subspace current/candidate bases, projected matrices, Jacobi workspace, permutations, residuals;
- mesh edge/triangle/constraint/refinement workspace;
- element quadrature/shape/Jacobian/strain/constitutive/local matrix/triplet owners;
- preview/checkpoint/result/fault owners; and
- every fixed control/page/box/arc/string backing.

Admission counts actual fixed backing or observed allocation capacity, never requested length or a
standard-container estimate. Maximum +1 returns the exact producer and leaves all counters
unchanged.

Every job exposes take/resume/close and an exhaustive terminal-empty witness. Dropping a public
handle during partial close leaves a generation-addressable registry authority. One close grant
retires one semantic owner/control/page; no recursive Drop or fixed-turn bailout is allowed.

## Checkpoint and Publication

Checkpoints are fixed-page retained cursors. They serialize one field/page entry per grant and
preserve the same operation/model/generation/numerical-contract identity. Restore validates lengths,
counts, revisions, and numerical schema before allocation, then reconstructs one owner per grant.

Previews are latest-wins only through retained close of a displaced preview. Checkpoints, terminal
results, and commits are lossless within declared fixed admission. Full queues return the exact
producer or keep it discoverable; they never deep-drop or livelock.

## Hostile Fixtures

Add focused fixtures and matching verifier mutations for:

- zero/max/max+1 rows, nonzeros, contributors, fill entries, basis columns, Jacobi cells, triangles,
  edges, flips, element nodes, quadrature points, and local cells;
- an adversarial high-fill LDLT column with low nonzero fuel and near deadline;
- deep/wide subspace bases with cancellation during every nested stage;
- a constraint requiring many deterministic flips and interruption after every edge phase;
- high-order elements cancelled during every stiffness stage;
- singular/negative/zero pivots, invalid Jacobian, nonconvergent Jacobi, and unflippable constraints;
- stale operation/model/document/surface generation before every publication;
- checkpoint interruption/restore at every phase;
- panic/fault before and after every owner transfer;
- full preview/checkpoint/result/terminal registries;
- dropped handle during partial close and exact registry rediscovery;
- exact process/page/item/byte/control counters returning to zero;
- deterministic 1/2/4/default worker replay and reference numerical parity; and
- no worker step at or above 8 ms under admitted adversarial maxima.

Verifier mutations must restore each old whole-kernel call, omit each cursor stage/credit/close
owner, weaken checked generation, or reintroduce dynamic map/sort work and make the focused gate
fail.

## Owned Files and Collision Boundary

Expected ownership is limited to FEM analyses LDLT/subspace sources, mesh constraint source,
element-family stiffness sources, narrow P6g inventory/session composition, focused fixtures, root
`📜️script.ts`, and this report. Re-census before editing; do not overlap active P6g audit or unrelated
FEM document/stdio work.

## Acceptance Gates

Source acceptance requires scoped rustfmt/diff, exact call/loop/container census, verifier
self-test/live focused success, deterministic ledgers, and independent Terra audit. Final acceptance
requires serialized debug/release/strict-warning, native/both Wasm, real mounted product, numerical
reference, worker replay, allocation/cancellation/fault/close stress, and timing gates.

P6h and Phase 6 remain RED until all gates pass.
