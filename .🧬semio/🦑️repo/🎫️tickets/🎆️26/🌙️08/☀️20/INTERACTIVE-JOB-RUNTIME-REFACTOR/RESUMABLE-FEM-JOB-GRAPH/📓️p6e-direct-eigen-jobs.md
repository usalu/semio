# P6e Direct and Eigen Jobs

## LDLT

`LdltJob` shares the exact `ldlt_column` kernel with `ldlt_factor`. It owns the symmetric CSC input, completed `L` columns, diagonal `D`, symbolic row contributor lists, column cursor, and deterministic column batch size. Each step checks freshness/cancellation/deadline/fuel, checkpoints applied columns, reports negative-pivot progress, and emits a structured zero-pivot fault.

The focused tests prove checkpoint/resume equality with the direct factor reference and retain all prior dense-LU correctness and singular-matrix tests.

## Subspace Iteration

`SubspaceIterationJob` owns the factor, mass/geometric operator, current/final subspaces, eigenvalue history, per-mode relative residuals, converged count, iteration cursor, and checkpoint/preview state. One deterministic Bathe subspace iteration is the bounded numerical unit. Each successful iteration publishes eigenvalue estimates, mode shapes, residuals, and converged-mode count; the following step makes the state durable before more work.

`subspace_iteration` is a batch adapter over the same persistent job. Checkpoint restore/re-encode and resumed results are bit-exact with uninterrupted scheduling when `serde_json/float_roundtrip` is enabled.

## Verification

The same debug/release/wasm harness commands and evidence listed in `📓️p6d-pcg-job.md` cover these jobs.

Focused outcomes:

- LDLT checkpoint resume equals the direct reference factor exactly;
- subspace checkpoint bytes are stable and resumed eigenpairs equal uninterrupted eigenpairs exactly;
- injected zero-fuel calls and different step scheduling do not alter the result;
- analytic diagonal and dense-Jacobi non-diagonal eigen references remain green;
- debug 22/22, release 22/22, wasm check success.

The full FEM product gate remains blocked in upstream stdio before the FEM crate is reached; exact evidence and counts are in `📓️p6a-job-graph.md`.
