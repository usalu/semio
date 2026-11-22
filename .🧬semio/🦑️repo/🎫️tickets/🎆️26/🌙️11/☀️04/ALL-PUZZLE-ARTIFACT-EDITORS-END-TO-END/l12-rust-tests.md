# L12 Rust tests (3D top-level, 2D, 5D/hub)

Runner: `l12-run.sh <3d|2d|5d|hub> <steps-file>` (persistent request loop afterwards: `🗑️generated/l12-req-<dim>`, line `quit` ends). 4g tmpfs (overlay on prior 5d cache, effectively cold), `--jobs 2`, opt-level 1 + debug-assertions + overflow-checks, `-Zembed-metadata=no`, no debuginfo, symbols stripped. Test crates' build dirs (exes ~120 MB each) are bind-mounted from disk `🗑️generated/l12-spill/` (deleted afterwards) because the full graph does not fit 4g. Full cold 3D build: about 55 min.

## 3D (log `🗑️generated/l12-3d-results.log`)
| target | passed | failed |
|---|---|---|
| puzzle-3d lib | 10 | 0 |
| puzzle-3d `snapshot-sqlite` | 12 | 2 |
| puzzle-3d-editor lib | 1 | 0 |
| editor-action-producer | 1 | 1 |
| editor-budgets | 59 | 13 |
| editor-defects | 40 | 12 |
| editor-gestures | 41 | 7 |
| editor-manifest | 46 | 16 |
| editor-panels | 63 | 28 |
| editor-surface | 40 | 25 |
| editor-task-resume | 1 | 3 |

(see sections below for causes and fixes)
