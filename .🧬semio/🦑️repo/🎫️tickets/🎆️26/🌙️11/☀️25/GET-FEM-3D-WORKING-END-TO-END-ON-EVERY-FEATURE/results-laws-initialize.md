# Results laws never initialize the FEM session (3D, and the 2D sibling)

## Root cause
The hub (`🌎️hub/🧩️compositions/🏗️fem/🦀️.rs::plugin()`) calls `session::initialize()` of the 2D and 3D editor session before building the plugin. That registers the bounded job kind (`FEM3D_MOUNTED_VISUAL_JOB_KIND`) in a thread-local registry. No native law did, so `SpawnJob` resolved a missing kind and the results caption stuck at "Preparing analysis".

## Fix (test root, so no law can forget it)
Every mounting law passes one `manifest` to `new_app_with_registry*`; that manifest now runs the same `initialize()` as the hub.
- 3D: new shared helper `✏️editor/🧪️tests/🚀️initialized-manifest/🦀️.rs` (`manifest()` = `initialize()` + `create_fem3d_app()`), included by `#[path]` in
  `🔬️app`, `🔬️app-2` (re-exported as `context::manifest`, also feeds `history_edit_acceptance_law!`), `🔁️live-cleanup` (also fixes its stale `manifest`/imports), `🪧️pane-smoke-target`.
  `🪟️window-config-ownership` (a lib test with a function-local module) calls `initialize()` in its own `manifest`.
- 2D (same gap): new helper `◻️2d/.../✏️editor/🧪️tests/🚀️initialized-manifest/🦀️.rs`, used by `🧰️context` (shared by editor_app, editor_app_2, unit) and `🌱️boot`; `🪟️window-config-ownership` calls `initialize()` inline.
- No production code changed.

## Runs (native, opt-level 1 for the later builds; debug for the first)
- `editor_live_cleanup`: compiles, `fem3d_live_cleanup_never_reaches_the_zero_progress_verdict` PASSES (0.50 s).
- `editor_app_2` (debug build, full): 64 of 71 laws passed, 7 did not finish: `context::history_edits_end_to_end` and 6 `results_window::*` settle laws
  (`add_solid_settles_the_results_3d`, `burst_of_edits_with_undo_redo_settles_3d`, `example_switch_during_a_running_solve_settles_on_the_new_example_3d`,
  `results_scene_captions_name_mode_and_factor_3d`, `results_window_surfaces_solver_error_without_panicking_3d`, `superseded_solve_settles_on_the_latest_revision_3d`). I stopped the run after ~14 min of CPU.
- `add_solid_settles_the_results_3d` alone, with a temporary `[DEBUG]` caption trace (removed again): the job now spawns (about round 100), the caption moves
  Analysis queued -> Preparing analysis -> Generating mesh -> Assembling analysis, and the initial example passes the first assertion. After the added solid it is still at
  "Generating mesh" after 600 rounds, so the law FAILS at its second assertion (not "Preparing analysis" any more). An opt-level-1 rebuild reaches exactly the same captions at the same round numbers,
  so the limit is turns, not CPU: the solver needs far more than 600 pump rounds (see `f3-root-cause.md`: per-unit turn cost in `MeshJob`/`AssemblyJob`, 249,763 assembly turns for the slab). No terminal "Case: ..." caption was reached.
  A failed law also aborts with "borrowed Pack symbol/frame backing requires its exact terminal-empty close witness" in the thread-local drop (a consequence of the assertion unwinding with a retained job).
- `editor_app` and `statik_pane_smoke`: NOT run to completion. Peers kept editing framework and stdio crates, which forced repeated rebuilds (each 25-40 min), and the run was still compiling when I stopped. 2D targets were not compiled.

## Follow-up
The remaining results-law failures are the solver turn cost (loop the delegated children until `should_yield()`, as listed in `f3-root-cause.md`), not initialization.

## Housekeeping
Private build dirs `C:\t\xi` and `C:\t\xo` (and their tmp dirs) deleted; the path `C:\ri`-style dirs were never usable because `\r` in `'C:\t\ri'` was mangled by the scripts (named `xi`/`xo` instead). A stray `C:\ti` from the first mangled run remains: the safety check refused deleting it, please remove it. Scripts kept: `xi-law.sh`, `xo-law.sh` (opt-level 1), `xi-retry.sh`, `xo-retry.sh`.
