# Puzzle Window and Operation Ownership Handoff

## Scope

Read-only follow-up to `audit-remaining-app-config-terra.md`, limited to the
current Puzzle 2D, 3D, and 5D command, configuration, and render paths. This
report records the source state observed on 2026-09-09. It does not claim a
native build or runtime check.

The required owner partition is:

| Semantic state | Owner |
| --- | --- |
| Shared authored board/world content | document mutation stream |
| Restorable local view/tool preference for one concrete host window | framework `WindowConfig` keyed by the exact window instance id |
| Draft text, popup, candidate buffer/cursor, hover, live interaction | `WindowTransient` keyed by the exact window instance id; optional peer publication is Presence |
| Work input, identity, cancellation, checkpoint, progress, result/fault | retained `Operation` keyed by operation identity |
| Active tool and live-window registry | framework host/window lifecycle state |

The existing plugin config record is an app-level retained publication lane. A
map within that record does not become a `WindowConfig`; it still has the
wrong lifecycle, replacement, undo, and restoration owner.

## P0 — Puzzle 2D has no per-window publication boundary

`Puzzle2dPlayApp::scene_for` clones the complete `Puzzle2dConfig` into each
scene at
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1017-1019`.
Every command retains its `window_id` (`:800`, `:813-827`) and passes it to
`Puzzle2dActionCtx` (`:1585-1606`), but the post-command path emits the whole
scene runtime as `Puzzle2dConfigMutation::Snapshot` (`:1645-1650`). Thus a
view action from one window changes the one persisted app record read by all
later window actions.

The source schema confirms that all candidates share that lane:
`🎚️config/🧬️schema/🦀️.rs:26-78`. The renderer uses the LOD map by
`WINDOW_KIND_ID`, for example at `🦀️.rs:671-674`; this fixed kind is not an
instance identity.

| Current fields / mutation lane | Correct owner | Implementation boundary |
| --- | --- | --- |
| `camera_x`, `camera_y`, `camera_zoom`; `lod_mode_by_pane`; view controls `grid_snap_enabled`, `grid_factor`, `suggestion_offset`, `fill_count` | `WindowConfig` for the originating concrete board/tool window | Replace the `scene.runtime` clone/snapshot dependency in `🦀️.rs:1017-1019,1585-1650`. Commands in `🎮️commands/📷️set-camera`, `🔭️set-lod-mode-for-pane`, `🧲️set-grid-snap-enabled`, `📐️set-grid-factor`, `🧭️set-suggestion-offset`, and `🧮️set-fill-count` must read/write the exact framework window owner. Freeze a control value in a new operation when work starts. |
| `engagement_input_by_pane`; `brush_candidates`, `brush_candidate_source_handle_id`, `brush_candidate_index` | `WindowTransient` | Route drafts and computed brush candidates through the concrete window. They are interaction output, so they must not be restored by `Puzzle2dConfigMutation::Snapshot`; clear them on abort/commit/window disposal. |
| `fill_job_operation`, `fill_job_generation`, `fill_job_seed`, `fill_job_base_revision`, `fill_job_checkpoint_sequence`, `fill_job_accepted_count`, `fill_job_search_count`, `fill_job_stage`, `fill_job_lifecycle`, `fill_job_fault_code` and `Puzzle2dConfigMutation::Fill` | retained `Operation` | Delete the fill runtime transport from `🎚️config/🦀️.rs:121-165,300-370`. Bind checkpoint and progress to the operation identity; reconstruct a resume only from that identity, immutable input, and operation checkpoint. |
| `example_load_generation`, `example_load_id` | retained example-loading `Operation` | Keep only a transient completion indication in the initiating window. The retained load must publish document mutations, never a configuration snapshot that represents job status. |
| `node_kind_weights`, `handle_kind_weights` | explicit local preference: `AppConfig` only if product deliberately shares generator defaults; otherwise source `WindowConfig` | In both cases snapshot values into fill input. A control edit after start must not alter a running job. |

The Puzzle 2D artifact's structural `camera` is not in this migration. It is a
document fixture contract distinct from the editor triple.

## P0 — Puzzle 3D uses instance keys inside the app record

Puzzle 3D proves the data is per-instance while preserving it in the wrong
owner. `Puzzle3dWindowOptions` contains camera, LOD, grid, selectable kinds,
engagement input, precompute controls, transform/display controls, sun, and
camera at
`🎚️config/🦀️.rs:278-303` and `🎚️config/🧬️schema/🦀️.rs:76-94`.
`scene_for` clones config and calls `load_window(window_id)` at
`🦀️.rs:2575-2582`. The action path appends the live id to
`config.window_ids`, loads it, then calls `save_window` at
`🦀️.rs:2602-2614,2643`. Render repeats that same config materialization at
`:7382-7405`.

All `SetWindow*` mutations still mutate the plugin config map at
`🎚️config/🦀️.rs:534-560`. This is an app configuration event stream, not the
framework window's persisted local state. `active_tool_id` and `window_ids`
are likewise retained in the plugin config (`🎚️config/🦀️.rs:196-204`) while
the action/render paths use them as host facts (`🦀️.rs:2608-2614,7385-7391`).

| Current fields / mutation lane | Correct owner | Implementation boundary |
| --- | --- | --- |
| Every `Puzzle3dWindowOptions` field except the typed input choice below: camera, sun, LOD, grid, selectable kind filter, proximity/chunk/voxel controls, transform flags, vortex display/direction | `WindowConfig` | Create/register an exact-window config type at the framework boundary. Make render and actions read it directly. Remove `window_options`, its temporary flat materialization fields, and the `SetWindow*` config-mutation family. Never substitute another map inside `Puzzle3dConfig`. |
| `Puzzle3dWindowOptions.engagement_input`, `suggestion_menu`, `brush_candidate_index` | `WindowTransient` | `suggestion_menu` already carries `window_id` in `🎚️config/🦀️.rs:111-128`. Preserve that scope in the framework transient owner and retire it with the popup/interaction. |
| `fill_apply_generation`, `fill_applied_count`, `fill_checkpoint` | retained fill `Operation` | The action path restores and updates these around precompute (`🦀️.rs:2619-2641`); render restores them too (`:7392-7404`). Move the checkpoint, generation and progress projection to the fill operation keyed by its identity. |
| `fill_count`, `overlap_budget`, `object_kind_weights`, `vortex_kind_weights` | `AppConfig` only as deliberate shared generator inputs | The current code explicitly treats these as one shared plan (`🎚️config/🦀️.rs:279-282`). Keep them as preferences only if that product choice stands, and copy immutable values into fill input at operation start. |
| `active_tool_id`, `window_ids` | framework host/window lifecycle | Derive from host state. Do not retain a window registry in an artifact editor configuration. |

## P0 — Puzzle 5D collapses two window kinds into one global runtime

The 5D schema puts cameras, grid, LOD, engagement drafts, brush candidate
cursor, preferences, and sun in one `Puzzle5dConfig`
(`🎚️config/🧬️schema/🦀️.rs:36-66`). Dispatch accepts an exact `window_id` but
creates the scene from `config.clone()` and snapshots all changed runtime at
`🦀️.rs:4178-4206`. Camera commands write the global runtime directly:
`🎮️commands/🖼️set-camera-2d/🦀️.rs:7-12` and
`🎮️commands/🎦️set-camera-3d/🦀️.rs:7-12`.

Render selects `board2d::WINDOW_KIND_ID` or `world3d::WINDOW_KIND_ID` only
from the body key and passes one cloned config into both renderers
(`🦀️.rs:8778-8794`). The default layout therefore does not make the values
window-owned; it merely assumes one board and one world kind.

| Current fields / mutation lane | Correct owner | Implementation boundary |
| --- | --- | --- |
| `camera2d`; board-side `lod_mode`, `grid_snap_enabled`, `grid_factor`, `suggestion_offset`, `fill_count` | `WindowConfig` for the exact 2D board window | Replace global reads in `🎭️modes/✏️edit/🪟️windows/◻️2d` and the global snapshot in `🦀️.rs:4205`. |
| `camera3d`, `sun`; world-side LOD/grid/tool display values | `WindowConfig` for the exact 3D world window | Replace global reads in `🎭️modes/✏️edit/🪟️windows/🧊️3d`; do not key restored state by `WINDOW_KIND_ID`. |
| `engagement_input_by_window`, `brush_candidate_index` | `WindowTransient` | The engagement renderer reads the map at `🎭️modes/✏️edit/🦀️.rs:44-49`, and brush options render the index at `☑️options/🖌️brush/🦀️.rs:140-143`; neither is a restore preference. Use exact transient window identity instead. |
| `object_kind_weights`, `vortex_kind_weights`, `overlap_budget` | `AppConfig` only if intentionally shared generator defaults; otherwise source `WindowConfig` | Freeze the selected values in operation input. The current schema contains no fill checkpoint/progress fields, so do not add job state back to configuration during the migration. |

## Required implementation order

1. Add framework-backed exact-window config and transient projections before
   deleting the plugin config fields. Do not use a fixed window-kind id or a
   plugin-owned map as an identity surrogate.
2. Make each Puzzle render path consume those projections; then redirect every
   view-only action to its new owner. Retain document mutations only where the
   source action changes board/world content.
3. Move 2D/3D job state to retained operations. Capture input at start,
   preserve cancellation/checkpoint/progress by operation identity, and prevent
   an unrelated preference update from overwriting a checkpoint.
4. Remove the obsolete snapshot/mutation routes after callers and renderers no
   longer read them. Validate two simultaneous same-kind instances, draft and
   popup retirement, and cancellation/resume by operation id.

## Remaining-family inventory pointers

The following records were not re-traced in this bounded handoff because their
preliminary producer/consumer audit is already in
`audit-remaining-app-config-terra.md`:

| Family | Existing recorded owner direction |
| --- | --- |
| Architect Program | register/filter/graph camera → `WindowConfig`; search draft → `WindowTransient`; reports/results/analysis → `AppTransient` or search `Operation` |
| Home Space | active tab → `WindowConfig`; directory projection/authorization receipt and client identity → OS account/session projection, not artifact app config |
| CAD | visual controls → exact `WindowConfig`; selection/hover/drafts/engagement session → framework interaction or `WindowTransient`; preview identity/generation → `Operation`; host contributions → host program context |
| Flow | camera/LOD/proximity/grid/preview-off → `WindowConfig`; duplicate checkpoint → `Operation`; contribution catalogue → host program context; generation preview → local derived projection |
| Forms | step → `WindowConfig`; try values → `WindowTransient`; contributions → host program context |
| DAG and FEM 2D/3D | viewport/result-display state remains a window-owned configuration candidate; re-check each command/render path during its execution lane before changing source |

The generic ownership scan found no new Puzzle artifact schema copy of
`WiresRetainedCommandRoutes`. The already reported Wires move to
`✏️editor/🎮️commands/🧬️schema` remains the correct physical direction.
`ArtifactDialect` is a shared framework value type; duplicate physical
definitions in an artifact schema should be replaced by the framework schema
reference only when the field is genuinely a document child/reference contract.
