# P6b Incremental Mesh Job

## Outcome

`fem::mesh::MeshJob` now owns a persistent, cancellable constrained-meshing operation with explicit `Validate`, `InsertBoundary`, `ConstrainBoundary`, `Classify`, `Refine`, `Finalize`, and `Complete` state. Boundary insertion, constraint creation, face classification, and result publication retain their cursors across `InteractiveJob::step` calls. Each ordinary step processes at most eight units and also observes fuel, deadline, operation/generation freshness, and cancellation.

The job publishes deterministic coarse, refined, and final preview tiers. Its binary payload is owned and versioned (`FEMMESH1`), uses stable face traversal and first-seen point numbering, and carries preview sequence, refinement progress, points, and triangle indices. Finalization emits a lossless checkpoint followed by a commit candidate. Existing batch meshing functions were de-asynced because they contain no suspension points.

## Refinement Boundary

The current internal constrained triangulator is still Spade. Interactive refinement constrains each invocation to one additional vertex and periodically rebuilds a replaceable preview in cursor-bounded face batches. This establishes the permanent resumable public contract without exposing Spade types. Phase 9 must still replace that internal seam with the owned incremental Bowyer-Watson implementation and differential-test it before the runtime-dependency gate can close.

## Verification

Added focused tests cover byte-identical repeated output, coarse preview publication, cancellation before mutation, and a 1,024-point boundary watchdog workload. The Nx quick gate was invoked with `bun nx run @semio-tech/fem-plugin:test-quick`; compilation is currently stopped before FEM by the active stdio de-async repair wall, so this packet does not claim a green compile or timing result yet. The focused tests must be rerun after the stdio dependency compiles.

## Files

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs`
