# 📓️ Wave 2C — puzzle ◻️2d panels, options and small verbs to 🧊️3d parity

Slice 2C. Paths below are relative to the repo root; `EDITOR2` =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
Line numbers are as of 2026-09-17 20:05; siblings 2A/2B/2D/2E/2F edit the same files continuously, so
treat them as anchors, not addresses.

## 1. Headline

Six new app verbs, one new window-option group, two new settings steppers, editable inspector
steppers, outliner hide/lock rows with authored display labels, a live-kind Add Node dialog, the
`setActiveTool`/`setActiveUtility` proof block 2d was missing, and the `Canvas2d`-vs-`Board2d`
surface-kind drift fixed at the root.

**One correction to the slice brief:** item 2 asked for inspector steppers "exactly where 3d's
object/vortex inspector fields are editable". Reading `EDITOR3/📌️panels/🔍️inspection/🦀️.rs` field by
field, **3d has ZERO editable numeric fields** — `object_fields` (:102-115), `vortex_fields`
(:117-126), `attraction_fields` (:128-140), `target_volume_fields` (:142-151) and `reference_fields`
(:153-163) are all `read_only`; only the two `hidden`/`locked` `flag_row`s mutate. Literal parity
would therefore have been "change nothing". I implemented the steppers anyway because
`patchInspectorNodes` was a **dead verb in 2d** — it is declared, classified, lane-contracted and
proof-carrying, but `grep patchInspectorNodes EDITOR2` found no dispatcher outside the registries.
2d is now AHEAD of 3d here, not at parity. Flagging so the wave-3 audit does not read it as drift.

## 2. What landed

### 2.1 Outliner hide/lock + display labels (item 1)
- `EDITOR2/📌️panels/🗿️artifact/🦀️.rs`
  - `flag_args` (:74) — `setSelectionFlag {flag, ids:[id], value}`. `value` is **always the inverse**
    of the row's current state (`!hidden` / `!locked`), ported from 2d's own context menu
    (`value: any_visible`) and `flag_row`, NOT from 3d's outliner. Map keys ascending (`flag`, `ids`,
    `value`) per the `UiMapBuilder` trap.
  - `flag_row_action` (:84), `with_hide_lock_actions` (:101) — graceful degradation copied from 3d's
    `with_hide_lock_actions`: a row the argument arena cannot afford **keeps the row and drops only
    its toggles**, so a section can never end at zero rows on Nakagin (180+179).
  - `node_row` (:135) gains `.dimmed(hidden)` + the two toggles. **Edge rows carry no toggles** —
    mirrors 3d's attraction rows and halves the per-document arena cost.
  - `node_label` (:48) now delegates to `puzzle2d_node_display_label`.
- `EDITOR2/🦀️.rs`
  - `puzzle2d_kind_catalog_label` (:565), `puzzle2d_node_display_label` (:579) — 3d's precedence:
    authored `label`, else the board's own `text`, else the kind's catalogue display name, else id.
  - `puzzle2d_next_node_label` (:614) — port of `puzzle3d_next_object_label`; first instance takes
    the catalogue name, further ones append ` 2`, ` 3`, … to the root taken from peers.
  - `puzzle2d_relabel_nodes` (:778) — **the one seam every batch-creation path calls**; clears each
    node's own label before its turn so it counts as one unlabeled peer, and later ids see what
    earlier ones were given.
  - Stamped at creation: `add_node_to_host_snapshot` (:~640, `"text": label`),
    `apply_brush_place_payload` (:~1030, `"text": label`).
  - `🎮️commands/👯️duplicate-selection/🦀️.rs` calls `puzzle2d_relabel_nodes` on the clones.
  - `🎮️commands/🚩️set-selection-flag/🦀️.rs` now accepts an explicit `ids` array (3d's
    `(entity, ids)` branch, minus `entity` — 2d ids resolve globally across nodes/handles/edges).

### 2.2 Inspector editable steppers (item 2)
- `EDITOR2/📌️panels/🔍️inspection/🦀️.rs` — `stepper_row` (:70) binds a `NumberStepper`
  (`uniform: true`) to `patchInspectorNodes {field, ids:[id]}` on `Trigger::Change`.
  Editable rows: `node.x` :163, `node.y` :164, `node.width` :166, `node.height` :167,
  `node.radius` :169, `handle.angle` :196, `handle.radius` :197.
  Edge fields stay read-only (id/kind/source/target are strings; 3d's attraction fields are
  read-only too — real parity).
- `EDITOR2/🦀️.rs` `patch_inspector_nodes` (:938) + `patched_field_value` (:927) — an id naming a
  HANDLE now patches that handle nested inside its node, so the handle rows reach nested geometry
  through the same one verb. Empty `ids` keeps the pre-existing all-nodes behaviour.

### 2.3 Window options (item 3)
- `setGridVisible` — `EDITOR2/🎮️commands/👁️set-grid-visible/🦀️.rs` (new). Toggle at
  `EDITOR2/🎭️modes/✏️edit/☑️options/🌐️grid/🦀️.rs:28`, measure id `puzzle2d-play-grid-visible`.
- `setSelectableKind` — `EDITOR2/🎮️commands/☑️set-selectable-kind/🦀️.rs` (new). New group
  `EDITOR2/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs`, group id `puzzle2d-play-select`, children
  `puzzle2d-play-select-{nodes,handles,edges}`. Bound into
  `🪟️windows/👁️overview/🦀️.rs:46` `window_measures()`.
- Engine + hosts really honour both:
  - `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` —
    new `pub grid_visible: bool` (:2376, default `true`), `set_grid_visible` (:4125, bumps
    `content_scene_generation`), and `build_vector_scene` gates the whole grid-stroke block on it
    (:10622). The dag port already had this flag; the normal port (the one `BoardHost` resolves to)
    did not.
  - `EDITOR2/🦀️.rs` `sync_host_runtime_state` :1473 `set_grid_visible`, :1491
    `set_selection_options("rectangle", "replace", kinds.nodes, kinds.edges, kinds.handles)`.
    ⚠️ Argument order on the normal port is **(nodes, edges, handles)** — it differs from
    `🎲️board/🦀️.rs:955`'s `(nodes, handles, edges)`. It was `true, true, true` before, so the
    divergence was invisible until now.
  - `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` `Board2dScene` — `grid_visible`,
    `selectable_nodes`, `selectable_edges`, `selectable_handles`, all `#[serde(default =
    "board2d_default_true")]`, plus the `scene_pack_wire!` twin and `Board2dScene::base`.
  - TS twin `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` (optional fields; absent reads as ON).
  - React host `🖥️Board2dHost/🟦️.tsx:1024` (selection options) and `:1033` (`setGridVisible`).
  - wasm session bridge `EDITOR2/🌉️wasm/🦀️.rs` — `#[wasm_bindgen(js_name = setGridVisible)]`, and
    the `BoardSession` TS interface in `🪪️WasmSessionLoader/🟦️.tsx` declares it.
  - wgpu target `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — cache fields `selectable_kinds` /
    `grid_visible`, the sync block now pushes both, and the close cursor drains them.
- **Grid spacing naming: deliberately NOT renamed.** 3d's `setGridSpacing`/`grid_spacing` is world
  units `[0.5, 50]`; 2d's `setGridFactor`/`grid_factor` is a **multiplier** of the LOD band step
  `[0.25, 8]` (`♾️infinite` `GRID_WORLD_{LARGE,MEDIUM,SMALL,MICRO} * grid_factor`). They are not the
  same concept, and renaming the id would have touched every registry, both fixtures and three
  sibling slices' files for a cosmetic gain. Structurally the two apps already match (one settings
  stepper + one window-option slider over the same field).

### 2.4 Settings placement tuning (item 4)
- `EDITOR2/🎮️commands/🚧️set-brush-placement-contact-tolerance/🦀️.rs` and
  `EDITOR2/🎮️commands/🫂️set-brush-placement-overlap-budget/🦀️.rs` (both new), both via
  `puzzle2d_absolute_or_delta` (`EDITOR2/🦀️.rs:556`, the 2d twin of
  `puzzle3d_absolute_or_delta`) clamped to `[0, PUZZLE2D_PLACEMENT_MEASURE_MAX = 48.0]` (:552).
- Shared config fields `contact_tolerance` / `brush_placement_overlap_budget` on
  `Puzzle2dConfig` **and** `Puzzle2dPlayRuntime` (`EDITOR2/🎚️config/🦀️.rs`), threaded through
  `🪟️window/🦀️.rs` `runtime()` / `split()`. Same shape as 3d, which also keeps
  `contact_tolerance` in the shared config.
- Steppers `puzzle2d-play-settings.contact-tolerance` and `puzzle2d-play-settings.overlap-budget`
  (`EDITOR2/📌️panels/⚙️settings/🦀️.rs:74-75`), step 0.5, tagged with `windowId` like the others.
- **They are a real read, not decoration.** `EDITOR2/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:25`
  `RUN_SETTINGS_CONFIG` is now 5 pointers (`/contactTolerance`, `/brushPlacementOverlapBudget`
  added) so the tool-run driver rebuilds a live run when either changes;
  `EDITOR2/⏳️precompute/🪣️fill/🦀️.rs` `fill_bounds_overlap_with(left, right, slack)` replaced the
  untuned `fill_bounds_overlap`, `Puzzle2dFillRevalidateJob` carries `placement_slack`, and
  `EDITOR2/🦀️.rs` `build_tool_run_job` passes
  `request.config.contact_tolerance - request.config.brush_placement_overlap_budget`.
  Slack is clamped so a budget can never shrink a footprint past its own centre.

### 2.5 Add Node dialog (item 5)
- `openAddNodeDialog` → `Effect::OpenDialog { dialog_id: PUZZLE2D_ADD_NODE_DIALOG_ID }`
  (`EDITOR2/🦀️.rs:64`, dispatch arm :2759), `UiDirtyScope::None`, lane `HostOnly`.
- `DialogDefinition::new("addNode", …, ActionRef::new("addNode"))` at `EDITOR2/🦀️.rs:5356`.
- **The static-option bug is gone.** `puzzle2d_node_kind_rows` (:501) resolves a document's node
  kinds as `meta.kindCatalogs.nodes` → the manifest its `meta.manifestId` names →
  `inferred_kind_entries`; `puzzle2d_node_kind_options` (:523) unions the two shipped examples'
  rows, deduped, bounded by `PUZZLE2D_NODE_KIND_OPTIONS_MAX = 64` (:68). `puzzle2d_node_kind_arg`
  (:544) is used by BOTH the dialog and `.action_args("addNode", …)`, so the two forms cannot
  drift. The old single literal `ActionArgOption::new("node", …)` is deleted.
- Two entry points, the same pair 3d binds: the shell palette (`in_palette: true`, category
  `create`) and the **empty-board context menu** (`EDITOR2/🦀️.rs:1795`, row id `openAddNodeDialog`,
  EN "Add Node…" / DE "Knoten hinzufügen…").

### 2.6 `engagementRepeatLast` + tool/utility proofs (item 6)
- `EDITOR2/🎮️commands/🔂️engagement-repeat-last/🦀️.rs` — literal 3d semantics: when the armed tool
  is `fill`, ask for one more placement through the declared `setFillCount` verb rather than writing
  the config behind the tool-run driver's back. New helper `set_fill_count::request(count)`
  (`EDITOR2/🎮️commands/🧮️set-fill-count/🦀️.rs`), the 2d twin of 3d's.
- Wired as `on_repeat_last` on the engagement input (`EDITOR2/🎭️modes/✏️edit/🦀️.rs`, was `None`).
- **The proof-block split is the load-bearing part.** 2d had ONE
  `bounded_first_step_tool_proofs!` inline in `impl ArtifactEditor` and therefore **no proofs for
  `setActiveTool`/`setActiveUtility` at all**. Restructured exactly like EDITOR3 ≈7608-7641:
  `struct Puzzle2dRetainedCommandProofs` (`EDITOR2/🦀️.rs:~4654`, `factory_type:
  Puzzle2dRetainedCommandJobFactory`) and `struct Puzzle2dHostConfigurationProofs` (:4744, GENERIC
  proof, no `factory_type`, contract `resumable(8_192, 8, 1, 8_192, 7_500, 1, 1)`, tools
  `["setActiveTool", "setActiveUtility"]`), concatenated by
  `fn bounded_first_step_tool_proofs()` (:4907). Added `fn host_configuration_mutation` (:4903)
  returning `Ok(None)`, same as 3d.
  Per 3d's own doc comment this is what stops both verbs falling through to `admit_command_json`
  and failing closed with `interactive-job.missing-factory` — i.e. **the Fill tool tab could not be
  selected without it.** Worth a targeted battery assertion.
- **State clearing on tool/utility switch: NOT implemented, and 3d does not do it either.** E2 §
  verb-diff cites `EDITOR3 ≈6836-6838` as clearing `suggestion_menu`/`brush_candidate_index` on
  tool switch; that line range is now a different struct and `grep` finds no such clearing in the
  current 3d source. 3d's `host_configuration_mutation` returns `Ok(None)`. Structurally the seam
  cannot do it anyway: it returns a `ConfigMutation`, while 2d's brush slot lives in the
  **WindowTransient** lane. Leaving a second mechanism uninvented per rule 8. If the battery shows
  brush state leaking across a tool switch, the fix belongs in the framework seam, not here.

### 2.7 `WindowKindDefinition.surface_kind` drift (item 7)
All three window kinds declared `SurfaceKind::Canvas2d` while `edit::render_canvas` emits a
`SurfaceKind::Board2d` surface. Fixed at the root: `👁️overview/🦀️.rs:29`, `🔍️detail/🦀️.rs:24`,
`🎯️selection/🦀️.rs:24` now declare `Board2d`. Pinned by a law (§4).

## 3. Registries touched (every new verb, every registry)

Six verbs: `setGridVisible`, `setSelectableKind`, `setBrushPlacementContactTolerance`,
`setBrushPlacementOverlapBudget`, `engagementRepeatLast`, `openAddNodeDialog`.

| registry | done |
|---|---|
| `puzzle2d_command_variants!` | `EDITOR2/🦀️.rs:1687-1692` |
| `PUZZLE2D_RETAINED_TOOL_IDS` (= `OpBinary::TOOL_JOB_IDS`) | all six |
| `PUZZLE2D_GENERIC_TOOL_IDS` | all six (each completes through `puzzle2d_dispatch_emit` / `Puzzle2dWindowCommandWork`) |
| `bounded_first_step_tool_proofs!` tools list | all six in `Puzzle2dRetainedCommandProofs` |
| `build_tool_job` match | covered by the `generic if PUZZLE2D_GENERIC_TOOL_IDS.contains(&generic)` arm |
| `PUBLICATION_CONTRACTS` | `setGridVisible`/`setSelectableKind` → `WindowConfig`; the two placement measures → `Config`; `engagementRepeatLast` → `WindowTransient`; `openAddNodeDialog` → `HostOnly` |
| `.action_with` in `create_puzzle2d_app` | all six |
| `.action_interactive_job(…, Migrated)` | all six |
| `command_from_action` | via the macro's `try_from_action` |
| EN+DE terminology | `EDITOR2/🗣️terminology/🦀️.rs` — new fields `show`, `hide`, `lock`, `unlock`, `visible`, `overlap_budget`, `contact_tolerance`, `kind` (all four terminology×locale cells) |
| `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` | all six appended to `toolIds` |
| `PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json` | all six added to the matching lane groups |

Schema twins hand-updated (rule 6): `🎚️config/🧬️schema/{🦀️.rs,🟦️.ts,🔗️.graphql,🛰️.proto,🔣️.json}`
(`contactTolerance`, `brushPlacementOverlapBudget`, incl. the TS `measure()` guard and the JSON
`required` list) and `🪟️window/🧬️schema/{…}` (`gridVisible`, `selectableNodes`, `selectableHandles`,
`selectableEdges`).

## 4. Laws added

`EDITOR2/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` (region `🙈️RowFlagLaws`)
- `outliner_node_rows_toggle_hide_and_lock_to_the_inverse_state` — all four (hidden, locked)
  combinations; asserts icon AND the `value` the click asks for. This is the anti-regression for
  3d's hardcoded-`true` bug.
- `outliner_edge_rows_carry_no_inline_toggles`
- `an_oversized_outliner_still_materialises_rows_when_toggles_cannot_be_afforded` (200/199 rows)

`EDITOR2/🧪️tests/🔬️unit/🦀️.rs` (regions `🏷️DisplayLabels`, `🩹️InspectorEdits`, `🌐️WindowOptionVerbs`)
- `a_node_display_label_prefers_the_authored_label_then_the_catalogue_name_then_the_id`
- `the_next_node_label_numbers_duplicates_of_one_kind`
- `adding_nodes_stamps_the_next_display_label`
- `relabelling_a_batch_numbers_each_new_node_in_order`
- `patching_an_addressed_node_field_writes_only_that_node`
- `patching_an_addressed_handle_field_writes_the_nested_handle`
- `set_grid_visible_toggles_the_window_flag`
- `set_selectable_kind_is_a_view_verb_that_never_mutates_the_document`
- `placement_tuning_verbs_are_config_only_and_clamped`
- `engagement_repeat_last_never_mutates_the_document`
- `open_add_node_dialog_is_shell_only`
- `the_add_node_kind_select_enumerates_live_example_kinds`
- `every_window_kind_declares_the_surface_kind_it_renders`

`EDITOR2/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` (region `🚧️PlacementSlack`)
- `the_fill_collision_test_reads_the_placement_tuning_slack`

No new document mutation shape was introduced, so no new language-neutral fixture vector or Python
second implementation is owed by this slice.

## 5. Commands run — real verdicts

| when | command | verdict |
|---|---|---|
| 15:17 | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly` | **exit 0**, 0 errors, `Finished dev profile … in 3m 00s`, 8 crate warnings |
| 15:25 | scratch Ajv oracle over the 2d schema twins (read-only, `bun`) | **ok** `Puzzle2dWindowConfig`, `Puzzle2dWindowTransient`, `Puzzle2dConfig`; the app-config-leak probe is refused for both window definitions |
| 16:02 | `… cargo check … --target wasm32-wasip2` | **exit 0**, 0 errors, `Finished dev profile … in 22.39s`, 7 crate warnings |
| 15:22 | `bun ./📜️script.ts publication-authority-audit Puzzle2dPlayApp` | **FAILS, not mine** — aborts in `validateWindowOwnershipSchemas` on `Puzzle5dWorldWindowConfig neutral fixture failed Ajv validation: missing property 'voxelDims'`. That is a **5d** slice's in-flight schema edit; the script validates every artifact's window schema before it reaches any owner filter, so the 2d half never runs. Re-run after 5d settles. |
| 20:03 | `… cargo check … --message-format=short` (the one gated check the 19:55 addendum allows) | **inconclusive** — `rustc` compiling `semio-framework-os-infinite` was killed by `signal: 15, SIGTERM` (external, machine under 10+ concurrent cargos). **Zero `error[E…]` lines in the output** — no source error was reported before the kill. |

Of the 8 warnings in the 15:17 run exactly one was mine (`fill_bounds_overlap is never used` after
the slack rewrite); I deleted the dead wrapper and the 16:02 wasm run shows 7. The remaining seven
(`unnecessary qualification` ×6, `unused imports: Buildable and HasBase` in
`🎭️modes/✏️edit/🦀️.rs:12`) are pre-existing or sibling-owned.

## 6. NOT verified — owed to integration

1. **The final native check never completed.** The 15:17 green covers every `semio-s-artifact-puzzle-2d`
   source edit of this slice *except* those made after it: the `🌉️wasm` `setGridVisible` bridge
   (wasm-gated — but **is** covered by the 16:02 green wasm check) and the framework-side edits below.
2. **`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` has never been compiled by any check I ran** — it lives
   in the renderer crate, which is not a dependency of `semio-s-artifact-puzzle-2d`. My edits there
   are three: two cache fields, the sync block, the close-cursor drain. **An integrator must check
   `semio-framework-os-renderer` (or whatever crate owns that file).** Highest-risk item in this slice.
3. `🪪️WasmSessionLoader/🟦️.tsx` and `🖥️Board2dHost/🟦️.tsx` are TypeScript — no `tsc` was run.
4. Filtered unit tests (`cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly
   --lib -- <filter>`) were **not run** — the 20:03 check was not green so the addendum forbids
   proceeding to them. Suggested filters: `display_label`, `next_node_label`, `relabel`,
   `patching_an_addressed`, `outliner_node_rows`, `outliner_edge_rows`, `placement_tuning`,
   `open_add_node_dialog`, `add_node_kind_select`, `surface_kind`, `placement_tuning_slack`.
5. `publication-authority-audit` exit 0 — blocked on 5d (§5).
6. No runtime/browser evidence of any kind; the coordinator owns activate/serve.

## 7. Battery hooks — exact ids for slice 2G

Verbs to dispatch: `setGridVisible`, `setSelectableKind {kind: "node"|"handle"|"edge"}`,
`setBrushPlacementContactTolerance {value}`, `setBrushPlacementOverlapBudget {value}`,
`engagementRepeatLast`, `openAddNodeDialog`, `setSelectionFlag {flag, ids, value}`,
`patchInspectorNodes {field, ids, value|delta}`.

Measure ids (window-options rail): `puzzle2d-play-grid-visible`, `puzzle2d-play-grid-snap`,
`puzzle2d-play-grid-factor`, group `puzzle2d-play-select` with
`puzzle2d-play-select-{nodes,handles,edges}`.

Panel node keys: outliner rows are keyed by **raw entity id** under
`puzzle2d-play-document.nodes` / `.edges`; inspector steppers are
`puzzle2d-play-inspector.node.{x,y,width,height,radius}` and
`puzzle2d-play-inspector.handle.{angle,radius}`, each with a `.control` child carrying the
`Change` binding; settings steppers are `puzzle2d-play-settings.{contact-tolerance,overlap-budget}`
(plus the pre-existing `fill-count`, `suggestion-offset`, `grid-factor`).

Dialog id `addNode`; context-menu row id `openAddNodeDialog` (only on an EMPTY selection).

Two assertions worth their own lane:
- **Fill tool tab arms.** Click the Fill tool and assert it stays on — this is what
  `Puzzle2dHostConfigurationProofs` exists for, and it was previously unprovable.
- **Outliner toggle round-trips.** Hide a node from its row, then Show it from the same row. Before
  this slice there was no row affordance at all; a hardcoded-`true` regression would pass the hide
  and silently fail the show.

## 8. Hand-offs to siblings

- **2B (hover/suggestions) and 2D (clipboard/createEdge/proximity):** call
  `crate::editor::puzzle2d::puzzle2d_relabel_nodes(&mut fixture, &new_ids)` after any path that
  mints nodes (paste, accept-suggestion batch, proximity-created clones). For a single node built
  from a kind id, use `puzzle2d_next_node_label(fixture_nodes(&fixture), &fixture, kind)` **before**
  taking the mutable borrow and write it into the node's `"text"`. Read a label back with
  `puzzle2d_node_display_label(node, fixture)` — never `node["text"]` directly.
- **2D:** `setProximityRadius` is yours. `contact_tolerance` and `brush_placement_overlap_budget`
  are already on `Puzzle2dConfig` + `Puzzle2dPlayRuntime` and in all five config schema twins;
  put `proximity_radius` on `Puzzle2dWindowConfig` (where 3d keeps it) and add its stepper next to
  mine at `📌️panels/⚙️settings/🦀️.rs:75`. `puzzle2d_absolute_or_delta` (`EDITOR2/🦀️.rs:556`) is the
  shared stepper-arg parser — reuse it.
- **2E (board engine / Board2dHost):** `BoardHost::set_grid_visible` and the `grid_visible` draw
  gate are new in `➕️normal/🦀️.rs`; `Board2dScene` gained four fields; `Board2dHost` gained two
  effects. Watch the **(nodes, edges, handles)** argument order of the normal port's
  `set_selection_options` — it is transposed relative to `🎲️board/🦀️.rs:955`.
- **2F (target regions):** the outliner's `with_hide_lock_actions` /`flag_args` helpers are
  reusable for a target-region row; keep region flags on their own verb as you already do
  (`setTargetRegionFlag`), since `setSelectionFlag` addresses node/handle/edge ids only.
- **Coordinator:** items 2 and 6 of the slice brief rested on E2 claims about 3d that the current
  3d source does not support (see §1 and §2.6). Neither is a gap I could close by porting, because
  there is nothing in 3d to port.
