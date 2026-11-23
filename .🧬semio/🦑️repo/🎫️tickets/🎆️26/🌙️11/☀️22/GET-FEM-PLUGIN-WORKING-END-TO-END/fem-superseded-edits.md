# 🧩 FEM Superseded Edits — Reconciled onto the Fleet Versions

The four FEM files whose local edits went to `C:\semio\.cache\f\lost` during the g1-m05 catch-up were each merged three ways. The base was the version this machine saw when the edit was made: the replica froze at 2026-11-22 11:15:43Z, so the base is the latest fleet put before min(mtime, 11:15:43Z), or the git commit before it. The merge slots are in `C:\b\femmerge\<slot>` (local / base / current / merged), and the script is `FLEET-REPLICA-SYNC-HEALTH-ON-G1-M05/merge-lost.mjs`. All four conflicted with newer fleet work, so each was re-applied by hand on top of the current file. No newer fleet work was reverted: the fleet's `semio_plugin::Locale` parameters, the `locale` plumbing and `results_cache_resident` all stay.

## Results by file

| File (under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/`) | Already in the fleet version? | Action |
|---|---|---|
| `🧊️3d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` | **No.** The real product bug is still there: `render_static` falls back to the raw first case id, but `fem3d_solve_all` keys results by encoded identity, so `results.get` misses and the frame shows "result missing". The caption prints the encoded id. `render_buckling` passes the encoded `source_id` to `fem3d_buckling_mode_values`, which expects a raw case id. | Re-applied: `source_label` (encoded identity → plain id), the fallback encodes `("loadCase", id)`, the caption uses `source_label`, and buckling maps `source_id` through `source_label`. The local removal of `results_cache_resident` was **not** taken (the fleet tests still use it). |
| `…/📊️results/🧪️tests/🔬️unit/🦀️.rs` | No | Re-applied: `use crate::context::Retiring` and `Retiring::new(…)` guards around all 5 owned snapshots. The `"dead"` source ids become `encode_result_source_parts("loadCase", "dead")` (4×). Not taken (it belonged to the dropped `results_cache_resident` change): `rendered()` replacing `render_body`, `results_solve_count` replacing `results_cache_resident`, removal of the `close_step` assertion, and the label-based phase check. |
| `🧊️3d/…/👁️viewer/…/🧱️model/🧪️tests/🔬️unit/🦀️.rs` | No. The fleet version does not compile (`semio_plugin::artifact_app_laws` needs feature `artifact-app-testing`, which the viewer does not enable), and the node id assertion is outdated (instances are now `scope:…:node5:n00_g`). | Re-applied all three local changes: the `Fem3dSnapshotRetiring` guard, `semio_plugin_testing_laws::artifact_app_laws` (the viewer's dev-dependency), and the node id built with `fem3d_entity_target("node", "n00_g")`. |
| `◻️2d/…/👁️viewer/…/🧱️model/🧪️tests/🔬️unit/🦀️.rs` | No | Re-applied: `Fem2dSnapshotRetiring` guards (2×) and `built_surface_scene` in place of the hand decode. |

`3D mesh-preview fem3d_nodal_von_mises` (lookup by encoded identity) and `3D engine encode_result_source_parts` were already in the fleet version, unchanged.

## Tests (native cargo, `C:\t\rc` / `C:\t\rcb`, `-j 3`, one build at a time)

| Run | Result |
|---|---|
| 3D engine | 21 pass |
| 3D io | 16 pass |
| 3D model unit, cold-reference-graph, fixed-dofs | 5 + 2 + 1 pass |
| 3D model `original_fem_receiving` | 4 fail (`(72, 24)` vs `(0, 0)` at `original-receiving-tests.rs:27`). This is framework receiving/retirement, the area the snapshot-receiving agent is changing; not touched here. |
| 3D viewer (first run) | **4 pass** after the fix (the fleet version did not compile, then failed 1 on the outdated node id) |
| 3D editor `editor_app_2 -- results` (results window tests) | **not built**: `semio-plugin-decl-app` fails on `semio_framework_value::Slot/SlotGuard` missing (framework value API in flux by another agent) |
| 2D model + viewer | **not built**: `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs` calls a missing `recv_dbg` (another agent's `[DEBUG]` work in progress) |

Scripts: `fse-test.sh`, `fse-test-3d-rerun.sh`, `fse-test-3d-viewer.sh`. Logs: `🗑️generated/fse-*.log`.

## Open
- Re-run the 3D results window tests (`-p semio-s-artifact-fem-3d-editor --test editor_app_2 -- results`) and 2D model/viewer once the framework value and job-context edits compile again.
- `original_fem_receiving` 4 failures belong to the snapshot-receiving agent.
