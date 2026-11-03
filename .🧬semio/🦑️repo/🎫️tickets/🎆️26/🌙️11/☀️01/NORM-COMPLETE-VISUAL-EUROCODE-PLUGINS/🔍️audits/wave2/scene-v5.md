# SCENE v5 (wave 2). Nothing compiled, no cargo run.

APP = `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface`.

## 1. Host viewport drag path through the shared placement commit
- New `APP/🕹️direct-manipulation/📍️placement-drag` (live-core): `standing_pose` (anchor of the placement grips + grip bases, yaw 0), `placement_edits` (calls `PlacementCommit::of_writes`; Insert -> one `NormEdit::InsertItem` at index = row count, Set -> `SetField` per changed leaf), `resolve_placed_fields` / `handle_placed_fields` (placement leaves via the commit, other leaves via `resolve_set_fields`, one emit = one undo step), `complete_preview` (drag preview of a row-less entity gains the other leaves of the standing pose), `overlay_writes` (inserts the row when all four leaves are present, refuses a partial set). Unit tests in `📍️placement-drag/🧪️tests`.
- `🫧️live-lanes/🎛️editor-wiring`: `live_set_fields` commits through `handle_placed_fields` (handles of the gesture, else of every entity when the bundle has placement leaves); `lane_events` completes `DragPreviewed` writes (`complete_preview`, needs `A::Snapshot: ToValue`, also on `live_ephemeral`).
- `🫧️live-lanes/🧵️lane-runtime::overlay_document` uses `overlay_writes`, so `LiveView::with_lanes` paints the preview document (and `live_scene_of` re-projects only that entity per frame) even for an entity without a row; the old path failed with an unknown-selector error.
- Shared for all ten artifacts (no per-artifact code): they all have the flat `placements` collection, `change-placement-x|y|z|yaw`, `insert-placement`, `remove-placement`. Rule selectors/payload names checked against the leaf structs: `id` (en1990/91/93/94/96/97/98) vs `placementId` (en1992/95/99), value `newX|newY|newZ|newYaw` (en1995 `newValue`), insert item `item` (en1990/91/94/96/97/98) vs `placement` (en1992/93/95/99), en1999 removes by `IdRemoval`. All consistent.
- Integration tests (stamped into every editor by `norm_live_acceptance!`): `a_synthetic_placement_drag_previews_the_complete_row_commits_one_event_and_reloads_to_the_same_pose` (stream -> overlay row, commit -> exactly one typed edit, insert when no row, replay on the value tree gives the previewed pose, each edit resolves to one mutation, `live_set_fields` raises exactly one mutation). The shared `transient_after` helper now goes through `lane_events` (so existing drag tests also cover placement grips). Feature scenario appended to `editor-wiring/🧪️tests/.../🥒️.feature`.

## 2. Field handles
Field handles already run the same pipeline: `HandleDragTool` applies snap rules/limits/quantise, preview via the lane, release -> `commit_writes` -> one `setFields` -> `SetFieldRule` kind. New test `every_handle_binding_of_the_default_document_resolves_to_a_typed_kind` resolves every binding of every handle of the default document through `A::live_resolve`. Static check (`🧰️scripts/handle-edit-rules-check.py`, heuristic, leaf and path names) found handles WITHOUT a typed kind:
- en1999 `connections[].bolts.boltsPerRow` (handle `per-row`): no rule/leaf (only `bolts.rows`). Request filed.
- en1991 wave-3 collections `windBridges[]` (deckWidth, deckDepth, heightAboveGround, span), `dynamicElements[]` (height, width, logDecrement), `internalExplosions[]` (volume, ventArea), `thermalSections[]` (length, depth), `impactActions[]` (x, y, height, assumedForce), `specialVehicles[]` (x, y): no `SetFieldRule` yet (known from editability-2, field-adder in progress).
- All other paths with a static prefix match an existing rule (en1990/92/93/94/96/97/98 clean; the path builders with dynamic prefixes are covered only by the runtime test).

## 3. en1992 crate rename (by hand, no cargo)
`mut-member-placement` -> `mut-member-prestress-steel` (leaf `change-member-prestress-steel-id`), `mut-anchors-placement` -> `mut-anchors-head` (leaf `change-anchor-head-diameter`). Dirs `🏛️member-prestress-steel`, `⚓️anchors-head`; en1992 `Cargo.toml` (members, workspace deps), `Cargo.lock` (renamed, packages and dependency lists re-sorted), `📦️packages/🦀️rust/📜️script.ts`, mutations root and `Cargo.toml`, io `Cargo.toml` and text codec `use`, root `📋️build-owners.json` (names/roots/manifests). `repo ./📜️script.ts verify build-owners sync` NOT run (runner owns it); needs one run to normalise owner order.

## 4. Imports and selectors
`super::scene::projection_kit::{Placement, grip_base}` in en1991/en1995 handles resolve: `scene` is `extern crate semio_s_plugin_norm_structural_scene as scene` in the scene root, child modules reach it via `super::scene`, `projection_kit` is `pub mod` and `placement::*` is re-exported. Generated edit-rule selectors for placements verified (see 1).

## Open / risks
- Standing yaw of a row-less entity is 0 (grips expose x/y only); if a derived layout yaw is non-zero the first drag resets it. Fix would be a yaw binding or a `live_standing` hook.
- `dispatch_live_command` (dry-run in `lane_events`) still resolves placement leaves without rows as errors; harmless (events empty), only the document lane and preview matter.
- Compile risks: `Option::is_none_or`, `DslValue: ToValue` in the new unit tests, `A::Snapshot: ToValue` bound on `lane_events`/`live_ephemeral` (editors are concrete).
- Nothing built; requests in `📌️important/requests.md`; rebuild lines in integrate and build-en1990..en1999.
