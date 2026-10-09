# 🔍️ r9 audit: BIM editor, viewer and render UI (read-only)

Scope: `S/✏️editor`, `S/👁️viewer`, `S/🖌️render`, where `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.
Sources read: `r2-design.md` §6, `r3-recipe-ui.md`, `r5-exec-u-tools.md`, `r4-exec-u-viewer.md`, `r8-exec-z-close.md`, `AGENTS.md`, and the code.
Path shorthand: `E` = `S/✏️editor`, `V` = `S/👁️viewer`, `R` = `S/🖌️render`, `W` = `E/🎭️modes/✏️edit/🪟️windows`, `C` = `E/🎛️chrome/🦀️.rs`.
Method: static reading and `rg` only. Nothing was built, run or clicked, so every runtime claim below is unverified. The working tree is shared and moving; line numbers are as read on 2026-10-08.

## 1. Findings table

Status: OK = present and wired, PARTIAL = present with a gap, MISSING = absent, DEAD = present but has no effect.

### Item 1: design §6 presence and wiring

| Item | Status | Where | Fix |
|---|---|---|---|
| Plan window (Canvas2d, storey, Viewport2d) | OK | `W/🗺️plan/🦀️.rs`, config `W/🗺️plan/🎚️config/🦀️.rs:10-17` | none |
| Plan cut height display | DEAD | config `:14` (`cut_height`); read only by chrome `C:71` and set-view `E/🎮️commands/🪟️set-view/🦀️.rs:42`. `plan::render_over` never reads it. The real cut height is the storey's authored `cut_height` (`E/🧩️entities/🦀️.rs:344`) | Drive plan-linework from the window value, or drop the measure and show the storey value read-only |
| World window (World3d, orbit, projection, storey isolation) | OK | `W/🧊️world/🦀️.rs`, `C:74-92` | none |
| World section "box" | PARTIAL | `W/🧊️world/🦀️.rs:104-115`: one capped plane, not a box | Amend design to "section plane", or add the box |
| Section window (Canvas2d, section line config) | PARTIAL | `W/📐️section/🦀️.rs`, config `:12-17` | see next rows |
| Section line: UI to set it | MISSING | `C:94-96` offers only `depth`. The line is reachable only through `setView` field `line` (`set-view:80`), i.e. host args | Add two-point numeric measures, or a pick gesture |
| Section depth | DEAD | config `W/📐️section/🎚️config/🦀️.rs:17`, set-view `:87`, measure `C:95`. `cuts` and `records` (`W/📐️section/🦀️.rs:72-115`) never read it | Clip cuts beyond depth, or remove the measure and the field |
| Outliner panel | OK | `E/📌️panels/🌳️outliner/🦀️.rs` | stale docstring, see item 7 |
| Properties panel | OK | `E/📌️panels/🔍️properties/🦀️.rs` | none |
| Library panel | OK | `E/📌️panels/🛍️library/🦀️.rs` | none |
| Schedule | PARTIAL | `W/🧮️schedule/🦀️.rs`, registered as a window kind (`E/🦀️.rs:799`), not a panel as design §6 says | Amend design, or move to `📌️panels` |
| Tools: select, wall, arc wall, curtain wall, column, beam, slab, slab-from-walls, roof, window, door, opening, stair, railing, space, grid, measure | OK | `E/🧵️gestures/🦀️.rs:48-70` (`build_tool`), rows `E/🪛️utilities/🦀️.rs:36-54` | none |
| Tools: move, rotate | PARTIAL | `build_tool` has them (`:52-53`). Utility rows `E/🪛️utilities/🦀️.rs:37-38` have no hotkey and no arm command | Add keys and `arm-*` rows |
| Tool: slab from walls | PARTIAL | row `E/🪛️utilities/🦀️.rs:45` has no hotkey and no arm command | Add key and arm command |
| Gestures emit only existing mutation kinds | OK | `E/🧵️gestures/*` (r5 §2) | none |
| Preview in window transient (plan, section) | OK | `E/🦀️.rs` `render_with_request_context`, `E/🫧️transient/🦀️.rs:9` | none |
| Preview in window transient (world) | MISSING | `E/🦀️.rs:528`: `world::render` takes no preview. Matches r5 §5 | Add a world overlay seam (3D marks) |
| `BimPresence { camera, storey, engagement_input }` | PARTIAL | `E/👥️presence/🦀️.rs:13-17`. `storey` and `camera` are written (`:223-230`). `engagement_input` is never written by any command | See next row |
| `engagement_input` (presence and transient) | DEAD | presence `:14, :224-229`; transient `E/🫧️transient/🦀️.rs:9`. Design wants a rename input | Delete, or implement the rename input through presence |
| Terminology `BimLabels`, en + de | OK | `E/🗣️terminology/🦀️.rs`, single `app_labels!` block | see item 3 for bypasses |
| Keybindings: undo, redo, delete, backspace | OK | `E/🦀️.rs:820` | none |
| Keybindings: escape, enter | OK | `E/🧵️gestures/🦀️.rs:45` | none |
| Keybindings: W, D, N, C, S, R | OK | `E/🪛️utilities/🦀️.rs:36-54` | none |
| Keybindings: A, U, B, O, T, L, P, G, M, V | OK | same rows (extra, not in design) | none |
| Viewer `BimModelViewer`, mode `view`, world plus plan, camera-only | OK | `V/🦀️.rs:508-560`, `V/🎭️modes/👁️view/🦀️.rs:17` | none |
| Viewer layout tab labels | MISSING (i18n) | `V/🎭️modes/👁️view/🦀️.rs:17`: `Some(&["World".into(), "Plan".into()])`, English only | see item 3 |

### Item 2: ModelMutation kinds reachable from the UI

Method: 88 `ModelMutation` variants (`S/🧬️schema/🧬️mutations/🦀️.rs`). A variant counts as reachable if its payload name appears in non-test `E` source (field `write`, `create`, `delete`, `rename` closures, or a gesture). That is static evidence, not a runtime check.

| Result | Kinds | Notes |
|---|---|---|
| 76 reachable | all others | via create-entity, delete-selection, rename-entity, set-field, or a gesture |
| No UI path (12) | `SplitWall`, `FlipWall` | no command |
| | `SetSlabBoundary`, `SetRoofFootprint` | no vertex or footprint edit |
| | `SetProjectInfo` | project name shown in outliner title (`E/📌️panels/🌳️outliner/🦀️.rs:155`) but not editable |
| | `PlaceElements` | not used |
| | `DeleteElements` | intentional per r5 §2: delete stays per-kind `delete-*` (`E/🎮️commands/🗑️delete-selection/🦀️.rs:47`) |
| | `SetElementProperty`, `RemoveElementProperty`, `SetElementClassification`, `RemoveElementClassification` | no property or classification editor |
| | `RenameStorey` | superseded: storey rename uses `RenameElement` (`E/🧩️entities/🦀️.rs:340`) |

Parameters wired in the mutation but read-only in the properties panel:

| Field | Status | Where |
|---|---|---|
| Roof `shape` | MISSING | `E/🧩️entities/🦀️.rs:407` is read-only. `SetRoofShape.shape` exists |
| Stair `flight` | MISSING | `E/🧩️entities/🦀️.rs:442` is read-only. `SetStair.flight` exists |
| Wall type `layers` and `thickness` | read-only | `E/🧩️entities/🦀️.rs:483-484`. Editing layers needs a richer control; document it |

### Item 3: i18n

| Item | Status | Where | Fix |
|---|---|---|---|
| Labels through `app_labels!` with en and de | OK | `E/🗣️terminology/🦀️.rs` (windows, panels, kinds, groups, fields, measures, statuses, empty states) | none |
| Command table labels and descriptions (en, de) | PARTIAL | `E/🦀️.rs:116-145` via `LocalizedLabel::native` in the macro rows | move into `app_labels!` |
| Action arg labels | PARTIAL | `E/🦀️.rs:776-785` (`Kind`, `Container`, `Name`, `Entities`, `Parameter`, `Value`, `Setting`) | move into `app_labels!` |
| Fault notices (`bim.*`) | PARTIAL | `E/🦀️.rs:458-487` | move into `app_labels!` |
| Utility labels | PARTIAL | `E/🪛️utilities/🦀️.rs:36-54` (`UtilityRow.label` tuples) | move into `app_labels!` |
| Interaction domain labels "Elements", "Library" | PARTIAL | `E/🕹️interaction/🦀️.rs:46, 54` | move into `app_labels!` |
| Default-language fallback "Column", "Space", "Stair" | PARTIAL | `E/🧵️gestures/📍️point/🦀️.rs:40, 49, 58` and `E/🧵️gestures/🧭️session/🦀️.rs:125-126` | take the fallback from the labels, not English |
| Axis labels "X", "Y", "Z" by `to_uppercase()` | low | `C:84` | move into terminology |
| Viewer: mode label, window labels, verb labels, domain labels | MISSING (i18n) | `V/🎭️modes/👁️view/🦀️.rs:12`, `V/🦀️.rs:530-542`, `V/🪟️windows/🧊️world/🦀️.rs:28`, `V/🪟️windows/🗺️plan/🦀️.rs:28`. `BimViewerLabels` (`V/🗣️terminology/🦀️.rs:6-8`) defines `window_world` and `window_plan`, but nothing uses them | use `BimViewerLabels` |
| Viewer: "Unknown body" | MISSING (i18n) | `V/🦀️.rs:324` | use the terminology `unknown_body` label |
| Hard-coded strings in render code | none beyond the rows above | checked by rg over `E`, `V`, `R` | none |

### Item 4: accessibility

| Item | Status | Where | Fix |
|---|---|---|---|
| Tree rows, field rows, inferred rows, add rows have labels | OK | outliner, properties, library | none |
| Field inputs have labels; text commits on blur | OK | `E/📌️panels/🔍️properties/🦀️.rs:70-80` | check Enter-to-commit at runtime |
| Tool buttons have labels and icons | OK | `E/🪛️utilities/🦀️.rs` | none |
| Keyboard: tools with a hotkey | PARTIAL | 16 of 19 utilities (see rows `E/🪛️utilities/🦀️.rs:36-54`). Missing: move, rotate, slab-from-walls | add keys |
| Keyboard: placing geometry | MISSING | every gesture needs a pointer. No keyboard cursor or coordinate entry | add keyboard cursor (arrows, Enter) to plan and section gestures |
| Keyboard: handles, marquee, storey top | MISSING | pointer only | keyboard equivalents, or document the gap |
| Canvas, 3D and table surfaces: accessible name | MISSING (static) | `E/🧰️kit/🦀️.rs:215` (`canvas_surface`) and `E/🧮️schedule/🦀️.rs:161` set no label. Framework may name them from the window label | verify at runtime; set a label if not |
| `role` values in canvas records | n/a | `overlay`, `node`, `meta` are render roles, not ARIA | none |

### Item 5: customizability

| Item | Status | Where | Fix |
|---|---|---|---|
| Per-window config for plan, world, section | OK | `E/🎭️modes/✏️edit/🪟️windows/*/🎚️config/🦀️.rs` | none |
| User controls: plan storey, plan navigation | OK | measure `C:71`, `setCamera` | none |
| User controls: plan cut height | DEAD | see item 1 | see item 1 |
| User controls: world projection, isolation, section toggle, axis, offset | OK | `C:74-92` | none |
| User controls: world hidden storeys | MISSING | `set-view:61` accepts `hidden_storey`; no measure in `C:74-92` | add per-storey toggles, as the viewer has |
| User controls: section line | MISSING | see item 1 | see item 1 |
| User controls: section depth | DEAD | see item 1 | see item 1 |
| Projection value validation | PARTIAL | `E/🎮️commands/🪟️set-view/🦀️.rs:51-54` takes any string | validate against the five kinds |
| Utility table and hotkeys | static | `E/🪛️utilities/🦀️.rs` is a compile-time table | none unless user remapping is required; framework keybinding config covers it |
| Layout | framework | `E/🎭️modes/✏️edit/🦀️.rs:26-28` (`create_default_layout`) | unverified: framework layout persistence |

### Item 6: progress and cancellation

| Item | Status | Where | Fix |
|---|---|---|---|
| Full re-inference | MISSING | `E/🦀️.rs:519` calls `with_inference` synchronously inside render. `E/🔮️inference/🦀️.rs` has no progress or cancel. The viewer does the same (r4 §5.3) | run as a bounded job with progress and cancel |
| Progress labels | DEAD | `E/🗣️terminology/🦀️.rs:198-199` (`progress_inference`, `cancel_inference`) are defined and used nowhere | wire them with the job above |
| Operation progress scope | PARTIAL | `E/🦀️.rs:702-704` names only the outliner panel | extend when the job exists |
| Plan and section generation | MISSING | `W/🗺️plan/🦀️.rs` `render_over` and `W/📐️section/🦀️.rs:98-115` run per render. `cuts` (`:72`) is recomputed on every render, with no memo | memoise per inference revision; make it a job |
| Gestures | OK | bounded single-step (r5 §1) | none |
| IFC import and export (out of this scope) | MISSING | `S/🚪️io` has no progress or cancel identifier (rg). Flagged, not audited | add progress and cancel to the IO jobs |

### Item 7: AGENTS.md style

| Item | Status | Where | Fix |
|---|---|---|---|
| Comments inside definitions | OK | rg found none in `E`, `V`, `R` | none |
| Docstrings start with an emoji | OK | 318 docstring blocks checked, 0 without an emoji | none |
| `[DEBUG]` lines | OK | none in `E`, `V`, `R` | none |
| TODO, `todo!`, `unimplemented!`, stubs | OK | none | none |
| Stale docstring | PARTIAL | `E/📌️panels/🌳️outliner/🦀️.rs:27` documents a constant that no longer exists, sitting above `OPEN_GROUP_LIMIT` (`:29`) | delete the line |
| Empty placeholder folders | low | `W/🧊️world/🪛️utilities/📌️.empty.md`, `W/🧊️world/🎬️actions/📌️.empty.md`, `W/🧊️world/👥️presence/📌️.empty.md`, `E/🫧️transient/📌️.empty.md`, `E/🎮️commands/📌️.empty.md`, plus many in `V` | remove where no node is planned |
| Dead configuration fields | PARTIAL | section `depth`, plan `cut_height`, `engagement_input` | see items 1 and 5 |

## 2. Prioritized fix list

P0, functional gaps against design §6:
1. Plan cut height is a dead control. Drive plan-linework from it, or remove the measure and show the storey value read-only. (`C:71`, `E/🎮️commands/🪟️set-view/🦀️.rs:42`, `W/🗺️plan/🎚️config/🦀️.rs:14`)
2. Section depth is a dead control. Apply it in `cuts` or remove it. (`W/📐️section/🎚️config/🦀️.rs:17`, `W/📐️section/🦀️.rs:72-115`, `C:94-96`, `set-view:87`)
3. Section line has no UI. Add numeric measures or a pick gesture. (`C:94-96`, `set-view:80`)
4. World gesture preview is missing. Add a 3D overlay seam, then pass the preview (`E/🦀️.rs:528`). This is the r5 §5 issue.
5. Hidden storeys have no UI. Add per-storey toggles. (`C:74-92`, `set-view:61`)
6. Roof `shape` and stair `flight` are read-only although their mutations carry them. Wire the field writes. (`E/🧩️entities/🦀️.rs:407, 442`)
7. Delete the dead `engagement_input`, or implement the rename input through presence. (`E/👥️presence/🦀️.rs:14, 224-229`, `E/🫧️transient/🦀️.rs:9`)
8. Decide per unreachable mutation kind: implement, or document as intentionally not in the UI. The 12 kinds are listed in item 2. Priority: `SetSlabBoundary`, `SetRoofFootprint`, `SetProjectInfo`, and the element property and classification set.
9. Full re-inference, plan and section generation, and their cut recomputation have no progress or cancel. Make them bounded jobs. Wire `progress_inference` and `cancel_inference`. (`E/🦀️.rs:519`, `E/🔮️inference/🦀️.rs`, `E/🗣️terminology/🦀️.rs:198-199`, `E/🦀️.rs:702`)

P1, accessibility and i18n:
10. Hotkeys for move, rotate and slab-from-walls. (`E/🪛️utilities/🦀️.rs:37, 38, 45`, plus `arm-*` commands and rows)
11. Keyboard drawing: cursor movement and Enter for plan and section gestures. Document any gap that remains.
12. Accessible names for canvas, world and table surfaces. Verify at runtime first. (`E/🧰️kit/🦀️.rs:215`, `E/🧮️schedule/🦀️.rs:161`)
13. Move command, arg, fault, utility, interaction-domain and fallback labels into `app_labels!`, en and de. (`E/🦀️.rs:116-145, 458-487, 776-785`, `E/🪛️utilities/🦀️.rs:36-54`, `E/🕹️interaction/🦀️.rs:46, 54`, `E/🧵️gestures/📍️point/🦀️.rs:40, 49, 58`)
14. Viewer: use `BimViewerLabels` for the mode, layout tabs, windows, verbs, domains and unknown-body text. (`V/🎭️modes/👁️view/🦀️.rs:12, 17`, `V/🦀️.rs:324, 530-542`, `V/🪟️windows/*/🦀️.rs:28`)
15. Hidden storey toggles in the world chrome (see 5).

P2, hygiene:
16. Delete the stale docstring at `E/📌️panels/🌳️outliner/🦀️.rs:27`.
17. Validate the projection value in `set-view` (`set-view:51-54`).
18. Schedule: decide window or panel, and align design §6 or the registration (`E/🦀️.rs:799`).
19. `domain_granularity_id` is hard-coded to `"wall"` in world (`W/🧊️world/🦀️.rs:132`) and schedule (`W/🧮️schedule/🦀️.rs:160`). Derive it per row, or leave it unset.
20. Remove empty `📌️.empty.md` placeholders that no node will use.
21. Verify at runtime: Enter-to-commit in properties, focus order, framework window labels. None of this was run.
22. Out of scope but flagged: IFC import and export have no progress or cancel (`S/🚪️io`).

## 3. Verification status

Static only. No build, no test, no browser. The `rg` counts above are from the tree as read today. Nothing in `E`, `V` or `R` was modified.
