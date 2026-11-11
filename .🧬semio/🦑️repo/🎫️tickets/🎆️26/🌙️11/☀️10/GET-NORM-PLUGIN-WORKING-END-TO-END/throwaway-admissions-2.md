# Throwaway construction admissions, round 2

Rule as before: each caller or owner keeps a resident `OriginalConstructionAdmission`, retires it with cx before terminal. New framework helpers: `OriginalConstructionAdmission::retire_step(cx)` (close step carrying the exact retired receipt) and `::matches(cx)`.

## Fixed
| Site | Resident admission |
| --- | --- |
| inference `🫴️receiving` `admit_original` | caller's admission passed in; residue moved into owner `construction` |
| inference `take_pair` | owner `construction`, retired in `close_step`, counted in `terminal_is_empty` |
| infinite fill `BoardFillJob/SnapshotIngress/SnapshotCapture::admit_original`, `Capture::step`, `begin_board_fill_snapshot` (+host) | caller admission param; editor `Puzzle2dFillRunJob.construction` and `FillCapture.construction`, retired in close |
| `admit_original_reserved_producer` | caller admission param |
| retained-command-job `test_admit_original_source` | `source_construction` field, retired in `close_step` |
| `admit_reserved_work` + `build_reserved_tool_job` (trait, editor, viewer, EditorApp/ViewerApp, 10 editor overrides) | new `construction` parameter; instance `ReservedPreparation.wire_construction` |
| instance `ReservedPreparation::admit` slot birth | `PendingReserved.reserved_birth` (field 31 of the initial inventory), residue adopted by the preparation, `close_slot`/`slot_close_demands`/`owns_birth` retire it |
| tool-run `🎞️tick/🛬️native` `built` | `OriginalToolRunNativeState.construction`, retired in `close_with` |
| tests: testing-caller fixture, slot-and-publication-fairness, job-close `admit_registered_fixture`, natural-file-reserved-job-laws, board-host fixture, puzzle2d brush fixtures | local or fixture-resident admission, retired explicitly |

## Not fixed
None remaining. `build_tool_job` (trait, editor/viewer app, 158 implementors, fixtures) now takes a resident `construction` admission: preview `AgentLanePreview.producer_construction` (retired in its close), retained-command `reserved_construction.admission()`, fixtures pass it into `admit_fixture_tool_job`. Production overrides name it `_construction` (unused).

## Coordinator item
`build_media_export_job` kept its 3-parameter form: the media export bodies never use a settled admission, so the earlier extra parameter in the app trait was dead and was reverted (all implementors repo-wide match again).

## Verification
- `check -p semio-plugin-decl-surface --all-targets` (root workspace): passes, 0 errors (log `throwaway2-surface.log`), before the build_tool_job threading finished.
- Every later run (`throwaway2-retained.log`, `throwaway2-all.log`) is refused by the repo's source admission before compiling: crate `semio-framework-job-context`, `📦️owner/🦀️.rs` 3702 bytes observed against a 1960 byte maximum. That file and crate are untouched by this work; the source-admission tool itself has uncommitted edits from another agent. So retained-command, instance crates, tool-run, infinite fill, testing-caller and the tests are NOT yet compiled or run after the build_tool_job and ReservedPreparation edits.
- Plugin crates belong to the root workspace (`/home/deck/semio`), not `🧰️framework`.
