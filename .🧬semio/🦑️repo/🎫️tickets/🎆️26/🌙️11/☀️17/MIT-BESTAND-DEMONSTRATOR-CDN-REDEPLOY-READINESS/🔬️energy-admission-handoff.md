# Energie numerical admission handoff

The existing static release panics in EnergySimulationRunJob while handling the Energie run/cancel test. The failing assertion at run-job line 278 requires the numerical admission holder to be terminal after moving out its job. The holder still owns the original constructor state.

The constructor can produce a job before it finishes settling its Source result. The old `ready` predicate tracked the physically present job, so the session moved the job ahead of settlement. The holder then reached its terminal check with its Source outcome, binding or completed control marker still present.

Numerical admission now records the constructor's explicit Ready result, retires its remaining control slots under the original caller, and then exposes a job or numerical rejection. Cancellation continues to close the job, model and config it owns, then reports the exact constructor retirement quote and progress when that is the remaining frontier. The outer session keeps its terminal assertion.

The focused native test uses the authored portable zone and weather fixture, compares the admitted census with the independent observer, drives a real original caller to the handoff, checks immediate terminal emptiness, and closes the numerical job and decoder. Managed Cargo ran `numerical_admission_handoff_leaves_terminal_owner` and passed. The initial crate-wide unit-test compilation stopped at seven existing API mismatches in the close and batch test modules. The focused public-API test is a standalone Cargo integration target.

A matching Energy release rebuild and the bilingual `energie run-cancel` browser case are pending.
