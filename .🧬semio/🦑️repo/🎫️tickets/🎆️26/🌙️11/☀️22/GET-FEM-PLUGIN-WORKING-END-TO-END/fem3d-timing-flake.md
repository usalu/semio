# 📊 FEM 3D Timing Flake: fem3d_numerical_paged_owner_maximum_plus_one_refuses_unchanged

## Verdict

Timing flake. The 8 ms wall-clock bound guarded no behaviour and no performance budget. The test body is untimed except for that bound, so under debug-build load the elapsed time includes everything between the start and the assert: a loop of page admissions, a loop of page closes, and construction of three mounted models. The refusal itself is asserted deterministically (returned producer pointer identity, lengths, terminal state, admitted-byte totals).

## Change

File: `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d\🏅️standards\🔖️1\🪆️subsets\🌐️any\✏️editor\🧵️session\🧪️tests\🔬️unit\🦀️.rs`

- Removed `let started = std::time::Instant::now();` from `fem3d_numerical_paged_owner_maximum_plus_one_refuses_unchanged`.
- Replaced the final `assert!(started.elapsed().as_micros() < 8_000);` with `assert_eq!(scalars.len(), 0);`, an exact count after the rejected push.
- The state-unchanged assertions that already existed (`ids.len()`, `terminal_is_empty`, `model.nodes_len()`, pointer identity on returned producers, `released == allocated`) are unchanged.

## 2D twin

No 2D twin exists for this test. The 2D numerical child has no paged-owner refusal test. The closest 2D twin of the visual sibling is `fem2d_visual_job_replay_accessibility_and_each_step_are_bounded` (2D `🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs:318`), which has its own 8 ms bound. That bound is not touched here.

## Open: same flake class elsewhere

`fem3d_visual_...` sibling at the 3D unit test file, around line 442 (`assert!(started.elapsed().as_micros() < 8_000);` after `job.step_one`), has the same wall-clock pattern. It was not changed because it was out of this task's scope. It is a candidate for the same treatment.

## Verification: BLOCKED

Three runs were planned with `cargo test -p semio-s-artifact-fem-3d-editor --lib fem3d_numerical_paged_owner_maximum_plus_one_refuses_unchanged`, `-j 3`, private dirs `C:\t\fl` and `C:\t\fltmp`. Logs are in `$T22/🗑️generated/fl/`.

- Run 1: rc=101. Compile errors in `semio-s-artifact-fem-3d-editor-commands` (`add-combination/🦀️.rs`, 7 errors: unresolved `semio_s_artifact_fem_2d_load`, `str` unsized in a `for` loop) and in `semio-s-artifact-fem-3d-editor-session` (1 error).
- Run 2: still failing at the session crate. The error is at `🧵️session/🦀️.rs:26`: `use semio_s_artifact_fem_2d_model::...::structure::rebinds_generation;`. The 3D session crate does not depend on the 2D model crate, and `rebinds_generation` is defined only in the 2D crate (`◻️2d/.../structure/🔏️physics-fingerprint/🦀️.rs`). `session-api/🦀️.rs:108` and the unit test at lines 957 to 978 use the function through the parent module glob, so the import cannot be removed without another change.
- Run 3 was queued in the same launcher and will fail the same way unless the import is fixed.

The test edit is not affected by these errors; none of them are in the test file.

Not fixed by me: the `commands` `add-combination` errors and the session import. Both belong to a peer's in-progress work. The coordinator said a peer fixed the 3D import; the session import remains. Resolving it needs a decision about where `rebinds_generation` should live (3D model, or a shared crate), so I reported it instead.

## Pending

- Once the session crate compiles, run the test three times with the launcher at `C:\t\fl-run.sh` (its loop is `for i in 1 2 3`).
- Delete `C:\t\fl*` (`C:\t\fl`, `C:\t\fltmp`, `C:\t\fl-run.sh`) after the runs finish. Not deleted yet, because the runs are not finished.
- The launcher's background run may still be active; its PIDs could not be identified safely, so they were not killed.
