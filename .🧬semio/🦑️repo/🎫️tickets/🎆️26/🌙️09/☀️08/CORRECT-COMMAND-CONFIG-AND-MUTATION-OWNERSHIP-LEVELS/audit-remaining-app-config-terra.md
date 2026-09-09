# Remaining App-Config Ownership Audit

## Scope and method

This is a read-only audit of the remaining entries in
`remaining-app-config-ownership-inventory.md`, with a focused trace through the
2D, 3D, and 5D Puzzle command, config-publication, retained-operation, and
render paths. It corrects the earlier 2D camera conclusion. The owner below is
derived from the producer, the retained publication path, and the reconstruction
or render consumer; a field name alone was not used as evidence.

Owner terms used here:

| Owner | Retention and scope |
| --- | --- |
| `Document` | shared artifact event stream; changing it changes the artifact |
| `AppConfig` | persisted locally for the whole app; only intentional app-wide user preferences belong here |
| `WindowConfig` | persisted locally for one concrete window instance |
| `AppTransient` | app-local computed/read-model state; never replayed as a preference |
| `WindowTransient` | ephemeral state for one concrete window instance |
| `Operation` | retained, cancellable job input/checkpoint/progress, keyed by operation identity |

No source or generated artifact was changed, and this audit did not run native
checks. Line references are from the working tree on 2026-09-09.

## P0: Puzzle 2D has one retained app-config snapshot behind every window action

The prior audit's statement that `set-camera` was window-local is false at the
publication boundary. `Puzzle2dPlayApp::scene_for` creates `scene.runtime` by
cloning the one `Puzzle2dConfig` at
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1017-1019`.
`puzzle2d_dispatch_emit` reconstructs that scene for every action at
`.../🦀️.rs:1585-1588`, and, after a command changes its runtime, publishes the
*whole* runtime as `Puzzle2dConfigMutation::Snapshot` at `.../🦀️.rs:1645-1650`.
The mutation replaces the global config at
`🎚️config/🦀️.rs:356-370`. Thus `set_runtime_camera` (`🦀️.rs:548-559`) has
durable, app-wide replay semantics despite the command's declared view scope.
`Puzzle2dActionCtx.window_id` (`🦀️.rs:895-912`) is not used to select a window
owner on this path.

The current fixed kind identifiers at `🦀️.rs:130-136` cannot make this correct:
they collapse instances of the same kind, while the product model permits a
concrete window instance to be split or spawned. A map keyed by kind would only
move the collision into the value type.

### Required 2D partition

| Current config fields | Evidence | Correct owner | Required change |
| --- | --- | --- | --- |
| `camera_x`, `camera_y`, `camera_zoom` | `set-camera` changes the scene runtime; it is rebuilt from the config snapshot as above | `WindowConfig` for the actual 2D board window | Read/write through the framework's exact window identity, and project that value into the board scene. Remove it from the global snapshot codec. |
| `lod_mode_by_pane` | `🎮️commands/🔭️set-lod-mode-for-pane/🦀️.rs:7-22` mutates a view LOD map | `WindowConfig` for the actual pane instance | Store one LOD value/group on the concrete window, never a pane-kind map in `Puzzle2dConfig`. |
| `engagement_input_by_pane` | `🎮️commands/⌨️engagement-input/🦀️.rs:7-13` changes an unsubmitted input | `WindowTransient` for the actual window | Route the draft through the ephemeral window view and clear it under the interaction lifecycle. |
| `brush_candidates`, `brush_candidate_source_handle_id`, `brush_candidate_index` | `🎮️commands/🎲️apply-board-events/🦀️.rs:132-141` receives candidates from interaction input, then the dispatcher snapshots them | `WindowTransient` for the board that produced the candidate set | Keep the computed candidate buffer, source handle, and cursor selection together; do not serialize a preview or handle reference into app config. |
| `fill_job_operation`, `fill_job_generation`, `fill_job_seed`, `fill_job_base_revision`, `fill_job_checkpoint_sequence`, `fill_job_accepted_count`, `fill_job_search_count`, `fill_job_stage`, `fill_job_lifecycle`, `fill_job_fault` | `🎮️commands/🧮️set-fill-count/🦀️.rs:1433-1451` creates `BoardFillJob` with a retained operation; `...:1608-1628` explicitly identifies the discarded runtime as the state a later verb resumes; complete currently emits `Puzzle2dConfigMutation::Fill` at `...:940-949` | `Operation` | Make operation identity the sole resume key. Retain input and checkpoint in the operation, publish progress/cancellation as an operation projection, and delete `Puzzle2dConfigMutation::Fill` as a job-state transport. |
| `example_load_generation`, `example_load_id` | retained `Puzzle2dActiveExampleWork` computes the next generation at `🦀️.rs:1905-1906` and publishes a global runtime snapshot at `...:1908-1911` | `Operation`; optional per-window completion notice is `WindowTransient` | Key the example-loading work by its operation. Only resulting document mutations are durable; the generation/id are operation status, not future app preference. |

`fill_count`, grid snapping/factor, suggestion offset, and node/handle kind weights
are input controls rather than progress. Keep each current control in the exact
window that owns it (`WindowConfig` for a persisted local tool preference), then
copy an immutable input snapshot into the fill operation at start. A subsequent
control change must not affect an already-running fill. If product explicitly
defines a preference as app-wide rather than tool-window-local, it may stay
`AppConfig`, but it must still be copied into the operation and must never share
the lifecycle/checkpoint lane.

The schema declaration to partition is
`🎚️config/🧬️schema/🦀️.rs:26-78`; its `#[state(config)]` annotations currently
give all of the above one global persisted lane. The matching Rust config/runtime
is at `🎚️config/🦀️.rs:180-235`.

### 2D non-finding: artifact camera

The Puzzle 2D artifact's `camera` field is not evidence that editor camera
belongs in a document. It is present in the artifact schema and fixtures (for
example `.../✳️any/🧬️schema/🦀️.rs:15`) and is distinct from the editor runtime
triple `camera_x/y/zoom`. This audit found no reason to remove that document
fixture field or its sparse-diff contract merely because it is named `camera`.

## P0: Puzzle 3D stores per-window data inside the wrong retained owner

Puzzle 3D has identified the correct *key*, but retains that map as a single
app config. `Puzzle3dWindowOptions` includes camera, LOD, grid, selection filter,
engagement, lighting, and display controls at
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs:76-94`.
`window_options` is then a `BTreeMap` in `Puzzle3dConfig` at `...:101-159`;
`load_window` and `save_window` retain it through that config at
`🎚️config/🦀️.rs:380-390`, and the `SetWindow*` mutation paths update the same
global record at `...:534-560`. This is an app-wide event/publication owner, not
the framework `WindowConfigOwner` required for isolation, restoration, and
window lifecycle.

| Fields | Correct owner | Required change |
| --- | --- | --- |
| `Puzzle3dWindowOptions.camera`, `sun`, LOD mode, grid view/snap/spacing, selectable-kind filter, proximity/chunk/voxel settings, transform display and vortex display/direction | `WindowConfig` for the exact Puzzle 3D window | Define/register a window config type and have actions/render read it directly; remove `window_options` and the config `SetWindow*` mutation family. |
| `suggestion_menu` | `WindowTransient` | It already carries a `window_id` (`schema:65-70`) and is a popup/position. Use the exact window ephemeral owner. |
| brush candidate cursor/index and `engagement_input` | `WindowTransient` | Keep interaction-derived candidate selection and typed draft in the producer's concrete window, not a shared config record. |
| `fill_apply_generation`, `fill_applied_count`, `fill_checkpoint` | `Operation` | Bind the checkpoint and progress to one retained fill operation; a resumer must load it by operation identity, not read mutable app config. |
| `active_tool_id`, `window_ids` | framework host/window state | Tool activation and live windows are host lifecycle facts. Derive them from the host rather than saving a plugin preference. |

There is a narrow legitimate `AppConfig` remainder: the source header at
`🎚️config/🦀️.rs:1-11` states that `overlap_budget`, `fill_count`,
`object_kind_weights`, and `vortex_kind_weights` form one shared precompute
plan. Retain those only if that is the intended product-wide local preference;
freeze their values in an operation input whenever a fill/precompute starts.
They must not be overwritten by progress or checkpoint events.

## P0: Puzzle 5D still assumes one instance per window kind

Puzzle 5D makes the same ownership error in a flat shape. Its config header says
the 2D and 3D windows are single-instance and stores all state flat at
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:1-8,63-113`.
Its schema lists the fields at `🎚️config/🧬️schema/🦀️.rs:41-65`. The event
reducer puts a board camera into the global runtime at `🦀️.rs:4103-4106`, while
render chooses only a window *kind* at `...:8778-8794`. The 2D renderer then
uses `envelope.runtime.camera2d` in the board fixture and view at
`🎭️modes/✏️edit/🪟️windows/◻️2d/🦀️.rs:117-151`.

| Fields | Correct owner |
| --- | --- |
| `camera2d` | `WindowConfig` for the exact Board 2D window |
| `camera3d`, `sun`, `lod_mode` | `WindowConfig` for the exact World 3D window |
| grid snapping/factor, suggestion offset, and `fill_count` | `WindowConfig` for the source tool window; capture them into each operation input |
| `brush_candidate_index`, `engagement_input_by_window` | `WindowTransient` for the exact source window. The edit mode reads the latter by window at `🎭️modes/✏️edit/🦀️.rs:45-49`, but the config still makes it durable. |
| object/vortex kind weights | `AppConfig` only if they are intentionally shared generator preferences; otherwise the source tool's `WindowConfig`. In either case, freeze them into work input. |

The actual action already carries a window identifier, so the migration must use
that identity rather than select a single config value by kind. Do not remove
the artifact's own structural fixture camera based on the editor camera finding:
Puzzle 5D explicitly keeps document mutations out of its editor camera path.

## P0: remaining immediate interaction/job values outside Puzzle

### Wires drag

`WiresConfig` stores `drag_node_id`, `drag_last_x`, and `drag_last_y` in its
config schema at
`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs:7-13`.
`canvas-pointer-move` reads those values to calculate a node move and emits
`SetDrag` while it emits the document mutation at
`🎮️commands/↔️canvas-pointer-move/🦀️.rs:18-31`. The same three values appear in
the presence schema.

The document `move_node` event remains a `Document` mutation. The pointer's
last coordinate and node identity are `WindowTransient`, scoped to the exact
canvas. Optionally project a deliberate remote-drag representation to Presence;
do not write a config mutation or use its coalescing record as the drag-state
transport.

### Drawing camera and trace status

Drawing's editor config schema retains engagement text, camera, and trace
counters at
`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs:11-21`.
`🎮️commands/📷️set-camera/🦀️.rs:17-20` labels its update runtime-only while
still emitting a config mutation. The trace pointer work is retained by
`DrawingSession` (see the operation identity and cancellation tests at
`🎮️commands/🖱️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs:28-33,61-64`).

Move the camera to `WindowConfig`, unsubmitted engagement text to
`WindowTransient`, and `trace_pointer_generation`, `trace_pointer_completed_work`,
and `trace_pointer_pending_work` to the retained trace `Operation` projection.
These progress numbers must not be configuration just because they are used to
rebuild a UI after a step.

## P1: concrete remaining inventory groups

These findings have a direct config-field and render/command role, but are lower
risk than the P0 publication paths above.

| App/config root | Fields and correct owner | Evidence |
| --- | --- | --- |
| Lowpoly `🎚️config/🧬️schema/🦀️.rs:11-47` | `active_object_id`, paint layer/parameters/colour, world camera, show-edges, and sun are `WindowConfig`; paint utility is host window state; engagement text is `WindowTransient`. | `🧭️view/🦀️.rs:6-11,30-35,91-125` already consumes framework interaction selection/hover. Do not recreate those fields in config. |
| Architect Program `🎚️config/🧬️schema/🦀️.rs:10-30` | graph camera, active register, and adjacency filter are `WindowConfig`; `search_query` is `WindowTransient`; active report/result/analysis JSON are `AppTransient` or a retained search `Operation`. | These outputs are command results/read models, not preferences. Keep `search_history_json` as `AppConfig` if it is intentional user history. |
| Home Space `🎚️config/🧬️schema/🦀️.rs:7-21` | active panel tab is `WindowConfig`; directory read model/session binding/auth generation/receipt projection and client identity are OS/session-owned local projection, not artifact-app config. | `🎮️commands/📬️apply-directory-event-page/🦀️.rs:28-31` folds directory pages; `🎚️config/🦀️.rs:165-189,367-379` persists them. `🎮️commands/🪪️set-client/🦀️.rs:14-21` stores identity even though studio creation consumes it. |
| CAD `🎚️config/🧬️schema/🦀️.rs:40-85` | selected/hovered references belong in framework interaction/presence; engagement input and pane step are `WindowTransient`; cameras/sun/dislocate controls are `WindowConfig`; preview operation JSON/generation/finalized-interaction id are `Operation`/`AppTransient`; example selection belongs to its loading operation. | Current schema still co-locates interaction, visual preferences, and job state. Contributions require the existing host-program owner rather than a new plugin config. |
| Flow `🎚️config/🧬️schema/🦀️.rs:8-32` | camera/LOD/proximity/grid and preview-off node controls are `WindowConfig`; generation and duplicate-widget progress JSON are `Operation`/`AppTransient`. | Catalogue sections and automation enablement are legitimate `AppConfig` if product-wide local workspace preferences; do not move them without a different producer trace. |
| Forms `🎚️config/🧬️schema/🦀️.rs:8-14` | step index is `WindowConfig`; unsubmitted `try_values` are `WindowTransient`; `contributions_json` belongs to the host program/view context. | `🦀️.rs:153-169` merges try values into the displayed form. Current reopen tests prove the old config persistence, not the correct semantic owner. |

## No additional finding from the requested artifact and metadata spot audit

The existing `artifact-contract-ownership-audit.md` identifies its source-level
contract coverage and native-validation limitation. This audit found no further
artifact sparse-diff field to remove from that evidence: the Puzzle 2D document
camera and Puzzle 5D structural fixture camera are not editor window cameras,
and no trace showed a computed preview being published as an artifact mutation.

`plugin-preference-ownership-implementation.md` already assigns Block 3D brush
preview to `WindowTransient` and Procedural 2D/3D generation preview to
`AppTransient`; they are excluded from this report's implementation work to
avoid duplicating that active change. The presentation metadata report's explicit
per-definition metadata direction is also consistent with this spot audit; no
new generic metadata-inference finding was observed.

## Dependency-ordered implementation groups

1. **Use existing framework exact-window owners at all call sites.** Require the
   action context's concrete window identity for every read/write. Do not replace
   app config with a plugin-owned map keyed by a kind or by a synthetic fixed
   identifier. Wire `WindowConfig` and `WindowTransient` projection into the
   relevant view model before deleting a config field.
2. **Puzzle 2D, then Puzzle 3D and 5D.** First split configuration schemas,
   codecs, and action outputs according to the tables. Then alter reconstruction
   (`Puzzle2dPlayApp::scene_for`, P3 `load_window`/`save_window`, and P5 runtime
   rendering) to consume the new owners. Finally replace fill/example
   config-mutation resume with retained operation state. This order prevents a
   window action from being rebuilt from an obsolete global snapshot.
3. **Interaction and job-state removals.** Migrate Wires drag and Drawing
   camera/trace state, followed by Lowpoly, Architect Program, Home, CAD, Flow,
   and Forms. Preserve a document mutation only where the trace shows it changes
   shared artifact content; route optional peer visibility through Presence.
4. **Contract tests by owner boundary.** Add schema-first language-agnostic
   fixtures and a third-party implementation comparison for each new owner
   partition. Cover: two concrete windows of the same kind preserving distinct
   restored camera/LOD; reload with no draft/candidate buffer; fill/example
   cancellation and resume by operation id; no config event for Wires drag or
   Drawing trace progress; and no artifact sparse-diff change for a view-only
   action. Run the repo's native Nx/Bun validation after source changes.

This sequencing deliberately leaves active work alone: locale OS-global
canonical SDK/window work, the named camera/LOD and family work, Block 3D and
Procedural preview work, Rewriting 2D, and root artifact-contract changes have
separate ownership assignments in the ticket plan.
