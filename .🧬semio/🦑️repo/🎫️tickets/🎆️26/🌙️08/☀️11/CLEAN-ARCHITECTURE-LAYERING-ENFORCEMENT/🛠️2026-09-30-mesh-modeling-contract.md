# Retained Mesh Modeling Contract

Implemented the missing canonical renderer-neutral production API in framework 3D: `MeshModelingJob`, `MeshModelingStep`, `MeshModelingProgress`, `HalfedgeMesh::bevel_job`, and `HalfedgeMesh::decimate_job`. Flow/S consumers receive only framework-owned types. No runtime dependencies, foreign public types, aliases, adapters, or test-only production facades were introduced.

## Retained Work

`step(budget)` advances at most `budget` retained work units. Each unit visits a source vertex/corner, checks one convexity point or halfedge, creates one bevel plane, clips one polygon corner, inserts/removes one heap item, evaluates one candidate corner, checks one edge incidence or vertex-link hop, or advances a reconstruction transition. The expensive geometry loops retain their cursors and intermediate buffers between calls. Clipped caps use ordered retained accumulation and a heap; decimation uses a distance-ranked heap and retains each candidate through remapping, normal/winding checks, adjacency checks, and vertex-link checks. Mesh compaction, halfedge reconstruction, and normal accumulation are also retained.

`step(0)` performs no work. Cancellation preserves the complete progress value and produces no geometry. A completed or failed job rejects further stepping. The immutable source is never updated by a job. Synchronous bevel and decimation now drive the same production jobs, replacing the duplicated synchronous algorithms and clipping helper.

Construction validates scalar parameters, checks selection handles, and captures an owned source snapshot with a normal Rust clone. Snapshot ownership capture is linear in source size; the step budget measures geometric work rather than allocations, collection deallocation, or a wall-clock deadline. `units_total` is a conservative workload estimate during execution and becomes the exact completed count on completion. These limits are explicit; the implementation does not claim deadline-bounded allocation or snapshot capture.

## Schema and Laws

Extended the existing language-neutral modeling fixture with the expected segmented-bevel volume, a minimum decimation volume, and all cancellable operation phases. Preserved every original slicing, zero-budget, minimum-step, cancellation, immutable-source, batch-parity, and retired-job assertion. Added cancellation checks for every listed phase, a Parry3d `TriMesh` mass-properties oracle for retained outputs, the existing Parry3d triangle-surface checks, and comparison of retained normal reconstruction with an independently rebuilt halfedge mesh.

The first green slicing run compared the new retained production jobs against the original untouched synchronous geometry implementation. Synchronous methods were subsequently changed to drive the retained implementation, then the whole crate was rerun.

## Actual Validation

All execution used the existing Bun/Nx targets with `NX_DAEMON=false`, `--skip-nx-cache`, and `CARGO_TARGET_DIR` set to the durable repository Cargo cache.

- Red baseline: `bun nx run semio-framework-3d:test --skip-nx-cache -- retained_modeling_jobs_slice_work_and_match_synchronous_geometry` failed compilation with 11 missing `bevel_job`, `decimate_job`, and `MeshModelingStep` errors.
- New retained implementation against original synchronous geometry: the same focused command passed 1 test, with 107 filtered out.
- Expanded fixture/cancellation/Parry3d law: the same focused command passed 1 test, with 107 filtered out.
- Runtime console verification: focused target with `-- --nocapture` passed and emitted temporary `[DEBUG]` observations: segmented bevel completed in 644 retained units with 16 vertices and 10 faces; decimation completed in 1051 retained units with 4 vertices and 4 faces. The temporary debug statement was removed afterward.
- Final source after debug removal and the final retained-state changes: `bun nx run semio-framework-3d:test --skip-nx-cache` passed all 108 tests, with zero skipped and native source compilation successful. Nextest run ID `70b3a77e-b0c5-4efe-a4c1-46eef16c4f73`.
- `bun nx run semio-framework-3d:lint --skip-nx-cache` ran and failed before reaching mesh because shared dependencies emitted two `semio-framework-trace` Clippy errors and 19 `semio-framework-value-derive` Clippy errors. Trace failures were an unused doc comment and a needless pass-by-value parameter. Value-derive failures included `map(...).unwrap_or_else(...)`, unnecessary unwraps, needless pass-by-value, elidable lifetimes, and unnecessarily wrapped results. Those unrelated shared files were preserved; lint is not claimed to pass.

## Exact File Manifest

Created:

- `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🛠️2026-09-30-mesh-modeling-contract.md`

Updated:

- `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs`
- `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🔬️modeling/🦀️.rs`
- `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json`

Deleted permanent files: none. Temporary mesh-modeling command logs were stored in the ticket's generated folder and removed after their observations were recorded here. Other agents' generated files were preserved. The parent owns eventual ticket closure and shared generated-folder cleanup.

No Git mutations, worktrees, AGENTS edits, launch commands, permanent scripts, manifest dependency changes, or goal/ticket mutations were performed by this subtask.

## Parent Integration

The Rust owner now contributes the real neutral-fixture/Parry3d retained-modeling law to canonical-architecture using the exact native law runner. Its existing test router now awaits the budgeted Cargo runner. Parent owns the additional package 📜️script.ts and 📋️project.json updates and derived launch registration. The aggregate contribution is one exact law; the earlier full crate proof remains 108 executed laws.

The new mesh canonical contribution actually passed through Nx in 21.9 seconds: one discovered, selected and executed native law with its neutral fixtures and Parry3d oracle. No cache was used.
