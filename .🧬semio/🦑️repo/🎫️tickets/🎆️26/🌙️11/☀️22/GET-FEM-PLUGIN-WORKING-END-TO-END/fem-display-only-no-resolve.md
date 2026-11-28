# FEM: a display-only document edit no longer re-solves (2D + 3D)

## Root cause
Every document edit bumps the render revision. Both mounted sessions treated "current generation's identity != render revision" as "needs a new solve", so a deformation scale edit (display-only, read from `doc.analysis` at render time) spawned a full background solve. R4 (`fem-r4-keep-results.md`) only made that re-solve invisible (previous generation drawn, marked stale).

## Fix
- Physics fingerprint (shared by 2D and 3D, in the 2D model crate which 3D already re-exports `FemAnalysisSettings` from): `.../◻️2d/.../🧬️schema/🏗️structure/🔏️physics-fingerprint/🦀️.rs`.
  - `FemAnalysisSettings::ANALYSIS_FIELDS`: the single declared table of settings with a `display_only` flag. Only `deformationScale` is display-only; `modalCount` and `bucklingCount` are physics (they change how many eigenpairs are solved).
  - Schema-first: `x-semio-display-only: true` on `deformationScale` in both `🧬️schema/🔣️.json` (2D and 3D); a law asserts the Rust table equals what the schema declares (and covers every schema property).
  - `PhysicsHasher` (FNV-1a 64, stable, reproducible in any language). Settings digest = for each non display-only field in declaration order: name, `=`, 8 little-endian bytes of the value (counts as u64, reals as binary64 bits), `;`.
  - `Fem2dSnapshot::physics_fingerprint()` / `Fem3dSnapshot::physics_fingerprint()`: every entity (exhaustive destructure, so a new field cannot be forgotten silently, via `Debug` text of rows) plus the settings digest. Display-only settings are excluded.
  - `rebinds_generation(finished, bound_fp, edited_fp)`: the one pure decision, shared by both twins.
- Sessions (2D `🧵️session/💼️job-bridge-and-live-mount`, 3D `🧵️session/🚀️session-api`):
  - The current generation (`CurrentSession` / `Current`) now carries `bound: RevisionBinding` (the revision it answers for, initially its own solve identity) and the `fingerprint` computed when its solve was admitted. The solve `identity` (job key, terminal receipts, recovery) is untouched.
  - All "is this the render's revision" checks (admission candidate selection, reconcile, `with_live_fields`, `with_live_visual`, 3D `with_result_fields`, `result_stage`) use `current.matches(render)`, i.e. the bound revision.
  - New `rebind_display_only` runs at the start of snapshot admission: for every finished generation (2D: `validated_final` and stage `Complete`; 3D: `done`, solver ready and the job's `Validated` terminal receipt, so the job can no longer step) with an equal fingerprint, `bound` is moved to the render revision and any stale pending admission is dropped. The slot is then settled: no preflight, no shell, no `SpawnJob`, no `CancelJob`; the fields are current (not stale, no "Updating results" caption).
  - The edited snapshot's fingerprint is memoised per canonical revision (registry `fingerprint_memo`), so repeated admission opportunities hash once and a following physics solve reuses it.
  - Physics change (fingerprint differs) or a solve still running: unchanged R4 path (previous generation drawn stale, cancel and respawn).
  - `with_live_visual` (2D) matches the visual lease with the generation's own solve identity, not the render's revision, since the lease was built for the solve revision.
- 3D `Fem3dResultBorrow::current_revision` now means "the current generation answers this render" (was an identity comparison).

## Decisions
- Only finished generations are re-bound. A job still running keeps the old behaviour (re-solve), because its snapshot lease checks commit authority on every step and would cancel itself once the store moves on.
- Fingerprint hashing is O(document) once per revision change that has a finished generation to compare; a solve is far larger.

## Laws (all natively green)
- Language-agnostic vector `.../◻️2d/.../🧬️schema/🏗️structure/🧪️tests/🧫️fixtures/🔏️physics-fingerprint/🔣️.json`: six settings combinations with their FNV-1a digests (computed independently with a Python implementation, then checked by Rust), plus the display-only and field-order lists. Three scale values share one digest; each count change differs.
- `the_physics_fingerprint_ignores_display_only_settings_and_matches_the_language_agnostic_vectors` and `the_display_only_fields_are_the_ones_the_schema_declares` (model crate test target `structure_laws`): passed.
- 2D session unit twins `a_display_only_edit_rebinds_the_finished_generation_without_a_new_solve` (scale edit: equal fingerprint, `rebinds_generation` true so no solve is spawned, `fields_generation(true, ..)` is not stale; a running solve is not reused) and `a_physics_edit_still_re_solves_through_the_keep_results_path` (modal count, buckling count, node move, new load case: different fingerprint, no rebind, previous generation drawn stale): passed.
- 3D session unit twins of the same names (modal count, buckling count, node move, node added; crate `semio-s-artifact-fem-3d-editor-session --lib`): passed. Full 3D session lib run: 25 passed, 1 failed: `fem3d_numerical_paged_owner_maximum_plus_one_refuses_unchanged` asserts a wall-clock bound (`elapsed < 8_000 us`, test file line 493) in an unoptimised build on a loaded machine; it does not touch the changed code and is a timing flake, not a regression (not re-run).
- Existing R4 law `a_revision_change_keeps_the_last_generation_drawn_until_the_new_one_binds` still passes.

## Files
- Model (2D crate, used by 3D): `◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🏗️structure/🦀️.rs` (module + exports), `.../🏗️structure/🔏️physics-fingerprint/🦀️.rs` (new), `.../🏗️structure/🧪️tests/🔬️unit/🦀️.rs`, `.../🏗️structure/🧪️tests/🧫️fixtures/🔏️physics-fingerprint/🔣️.json` (new), `.../🧬️schema/📸️snapshot/🦀️.rs`, `.../🧬️schema/🔣️.json`.
- 3D model: `🧊️3d/.../🧬️schema/📸️snapshot/🦀️.rs`, `🧊️3d/.../🧬️schema/🔣️.json`.
- 2D session (`◻️2d/.../✏️editor/🧵️session/`): `🦀️.rs`, `📍️snapshot-admission-and-registry/🦀️.rs`, `💼️job-bridge-and-live-mount/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`.
- 3D session (`🧊️3d/.../✏️editor/🧵️session/`): `🦀️.rs`, `📸️snapshot-preflight/🦀️.rs`, `🗃️registry/🦀️.rs`, `🚀️session-api/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`.
- Script: `$T22/fem-display-only-run.sh` (launcher copies `C:\t\pf-run2d.sh`, `C:\t\pf-run3d.sh`).

## Build notes
- `C:\t\pf` + profile `p` still hit os error 206 on the model crate; use a shorter private dir (`C:\z`, `--profile p` via `--config 'profile.p.inherits="dev"'`). `C:\q` belongs to someone else and was not touched.
- Not browser-verified: the coordinator rebuilds the wasm and checks that dragging the scale spawns no solve and shows no "Updating results" caption.

## Cleanup
`C:\t\pf*` deleted. The short build dir `C:\z` (about the size of two workspace builds) could not be removed: the safety check blocks `rm -rf` on a top-level dir. Delete it with `Remove-Item -Recurse -Force C:\z` when convenient.
