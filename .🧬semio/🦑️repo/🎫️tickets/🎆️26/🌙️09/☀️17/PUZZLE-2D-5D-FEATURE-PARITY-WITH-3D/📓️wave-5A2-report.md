# 📓️ Wave 5A2 — puzzle 🖐️5d Artifact-lane + interaction/viewport verbs migrated off `BatchOnlyPendingRewrite`

Slice: 5A2. Editor root (EDITOR5):
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`.

## 1. Headline

All 21 verbs in this slice's remit are now `InteractiveJobClassification::Migrated`, registered in every
registry the executor rules list, and routed to real retained `PuzzleCommandWork` implementations. Combined
with slice 5A1 (window/config lane) the whole app is clean: **65 verbs, 0 `BatchOnlyPendingRewrite`, 0
`Unclassified`** — a `grep BatchOnlyPendingRewrite` over the production half of EDITOR5 returns nothing, and
a law pins that (`artifact_lane_verbs_are_registered_migrated_retained_tools`).

Most of the Work structs (`Puzzle5dTransformWork`, `Puzzle5dPatchPartWork`/`…GripWork`/`…FastenerWork`,
`Puzzle5dCreateFastenerWork`, `Puzzle5dEditFastenerWork`, `Puzzle5dRetargetFastenerWork`,
`Puzzle5dDeleteFastenerWork`, `Puzzle5dProximityConnectWork`, `Puzzle5dAddNodeWork`,
`Puzzle5dAddBrushPartWork`, `Puzzle5dBoardEventsWork`, `Puzzle5dWorldRelocateWork`,
`Puzzle5dSetActiveExampleWork`, `Puzzle5dFocusSelectionWork`) already existed in the tree and were already
routed by `build_tool_job` — they were simply **never registered**, so none of them could ever run. The bulk
of this slice is therefore registration plus repairing the silent/dead paths those works still carried.

## 2. Duplicate verbs resolved the greenfield way (no aliases)

| kept | removed | why |
|---|---|---|
| `focusSelection` | `zoomToSelection` | matches 3d; the context-menu row, the command variant, the tool-id list, the proof list, the publication contract and the `ActionDefinition` all moved to the survivor. Command directory renamed `🔎️zoom-to-selection` → `🎯️focus-selection` (module `focus_selection`; `🎯️focus-selection` is already an allowed taxonomy member name, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:9131`). |
| `addBrushPart` | `addBrushObject` | one placement verb; `.mutation`, `.action_args`, the command variant and the `build_tool_job` arm all dropped the twin. |
| `selectSameKindSelection` | `selectSameKind` | 3d's own surviving id, already the one `puzzle5d_context_menu_items` addresses, already in the fixtures. |

Law: `duplicate_verb_ids_are_removed_rather_than_aliased` asserts the removed ids appear nowhere in the
production source and have no `Puzzle5dCommand` variant.

## 3. What landed, per verb (file:line at time of writing — EDITOR5 `🦀️.rs` unless stated)

### 3.1 New plumbing the whole slice rides on
- `Puzzle5dActionCtx` gained `view_state`, `effects: Vec<Effect>` and `interaction_writes: Vec<InteractionWrite>`
  (≈ `🦀️.rs:4060-4080`), plus `replace_selection`, `clear_selection`, `notice`,
  `refuse_without_selection`, `refuse_when_locked` (≈ `🦀️.rs:4118-4190`) — ported 1:1 from puzzle 3d's
  `Puzzle3dActionCtx`. `handle_action_impl` now threads both lanes into `Emit`, and an **aborted** arm still
  publishes the refusal notice it pushed (3d's fix: "refused" and "silently did nothing" must not look alike).
- `PUZZLE5D_LOCALIZATION_UNSUPPORTED` + `puzzle5d_notice_emit(view_state, message)` — the terminal-`Emit`
  notice a retained Work completes with (`🦀️.rs:4189-4200`).
- `PUZZLE5D_FLAT_TO_WORLD: f64 = 1.0 / 48.0` (`🦀️.rs:≈699`) — the ONE board↔world scale, now used by
  `add_palette_part`, `Puzzle5dAddNodeWork`, `Puzzle5dAddBrushPartWork` and the transform work instead of
  three separate `1.0 / 48.0` literals.
- Terminology (`🗣️terminology/🦀️.rs`, EN+DE, compile-checked): `focus_selection` (replacing
  `zoom_to_selection`), `nothing_selected`, `selection_locked`, `example_too_large`, `edit_not_applicable`.

### 3.2 Per-verb repairs (not just registration)
- **`focusSelection`** — `Puzzle5dFocusSelectionWork` computed a camera target and then **threw it away**
  (`let _ = (target, divisor, config); … Emit::default()`). It now binds `view_state`/`window_config`,
  accumulates an axis-aligned bound in the one part scan it already declares, and publishes
  `window_config_mutations` via `window_ownership::addressed_config` — board pane → `camera2d`, world pane →
  `camera3d`. An EMPTY selection frames the WHOLE document (3d's `frames()` fix) instead of no-oping; with
  nothing to frame at all it completes with one notice.
- **`applyBoardEvents`** — four real defects fixed in `Puzzle5dBoardEventsWork`:
  1. the `camera` row was decoded and then **discarded** (`self.camera2d.take(); let config_mutations = Vec::new();`)
     → board pan/zoom never persisted; it now publishes the pane's `WindowConfig`;
  2. `select` rows were ignored → now the LAST `select` row of a batch becomes one `Replace` interaction write;
  3. `nodeDelete`/`edgeDelete` left a dangling selection → removed ids are now cleared with a **Subtractive**
     write naming exactly those ids (an empty `Replace` is a silent no-op in `protocol::next_selection`);
  4. a `brushPlace` row's created part is re-selected; a drag of a **locked** part is skipped and surfaces one
     notice. A board drag still moves ONLY the flat pose — that is what makes it a plan edit.
- **`translateSelection` / `rotateSelection` / `scaleSelection`** — `Puzzle5dTransformWork` moved only the 3d
  pose, so a world drag left the board pin behind. `translateSelection` now emits `move_part_2d` alongside
  `move_part_3d`, projecting the same world delta through `PUZZLE5D_FLAT_TO_WORLD`; rotate/scale leave the flat
  pose alone (neither moves a part's own origin). Locked parts are skipped and, when every addressed part was
  locked, the gesture completes with one notice and no edit. An empty selection completes with a notice.
  One history edit per gesture is unchanged (`coalesce_key` `gumball-translate|rotate|scale`).
- **`setActiveExample`** — extent used to return `None` for an example over the fixed work ceiling, which
  surfaces as the framework's opaque "exceeds fixed semantic work capacity" fault. It now declares one unit in
  that case and completes with the localized `example_too_large` notice, document untouched. It also clears the
  stale selection (Subtractive) — every part of the old document is gone.
- **`selectSameKindSelection`** — `puzzle5d_retained_reduce` **fail-closed** on this id
  (`"…ArtifactEditor has no route-specific interaction-selection publication primitive"`). That branch is
  deleted; `select_same_kind` now emits one `InteractionWrite::replace` through `ctx.replace_selection`, and the
  contract moved `HostOnly` → `Interaction`.
- **`registerBrushMesh`** — was falling through to `BoundedFirstStepCommandWork`. New
  `Puzzle5dRegisterBrushMeshWork` pages the two number arrays in at `PUZZLE5D_MESH_PAGE_VALUES = 512` values per
  step and then calls the shared `register_brush_mesh::puzzle5d_install_brush_mesh` — the SAME install the
  synchronous `handle` arm uses, so there is one mechanism, not two. A rejected install surfaces one notice.
- **`addNode` / `addPartKind` / `addBrushPart`** — all three now re-select the part they created
  (`InteractionWrite::replace`), so the next patch/transform/delete addresses the new part (3d's
  `addObjectKind`/`addBrushObject` behaviour).
- **`patchPart`** — a patch that addressed nothing produced an empty `Emit` silently; it now completes with one
  `edit_not_applicable` notice.
- **`createFastener` / `deleteFastener` / `editFastener` / `retargetFastener` / `patchGrip` /
  `patchFastener` / `proximityConnect` / `worldRelocate`** — their Works were already correct and cursored;
  this slice registered them and declared their lanes.

## 4. Registries touched (all four agree — verified mechanically)

- `puzzle5d_command_variants!` — `AddBrushObject`, `ZoomToSelection`, `SelectSameKind` removed.
- `PUZZLE5D_RETAINED_TOOL_IDS` (= `TOOL_JOB_IDS`) — 19 ids added, `zoomToSelection` → `focusSelection`.
- `PUZZLE5D_WINDOW_TOOL_IDS` — `zoomToSelection` removed (`focusSelection` owns its own Work).
- `bounded_first_step_tool_proofs!` `tools:` — same 19 ids. The contract band
  `resumable(PUZZLE5D_RETAINED_RAW_BYTES = 262_144, PUZZLE5D_RETAINED_DECODED_ITEMS = 16_384, 1, 262_144, 7_500, 1, 1)`
  was **already raised by slice 5A1** (the rules said first-one-there sets it); 5A2 left it alone.
- `build_tool_job` — `registerBrushMesh` arm added; `addBrushObject`/`selectSameKind` arms removed.
- `PUBLICATION_CONTRACTS` — 19 rows added (lanes in §5).
- `.action_interactive_job(…, Migrated)` in `create_puzzle5d_app` — 19 rows flipped, 3 rows deleted.
- `.mutation`/`.action_with`/`.action_args`/`command_from_action` — the three removed twins dropped;
  `focusSelection` promoted to a `bounded_catalog` action with the `view` category (it is the keybinding /
  context-menu / engagement-bar verb).
- `puzzle5d_context_menu_items` — the Zoom row now dispatches `focusSelection`.
- `🗣️terminology/🦀️.rs` — five label rows, EN + DE, all four locale×terminology cells.
- Fixtures: `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` (capacities raised to 262 144 / 16 384, stale
  `addBrushObject` cursor dropped, cursors added for `registerBrushMesh` / `selectSameKindSelection` /
  `setActiveExample`, 13 new vectors) and `🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` (the whole
  `Puzzle5dPlayApp` owner block regenerated from source: every group `migrated`, no `blocker` rows left).

## 5. Declared publication lanes (the honest set — an undeclared lane is a runtime fault)

| verb | lanes |
|---|---|
| `addBrushPart`, `addNode`, `addPartKind` | artifact + interaction |
| `applyBoardEvents` | artifact + window-config + interaction |
| `createFastener`, `deleteFastener`, `editFastener`, `patchFastener`, `patchGrip`, `patchPart`, `proximityConnect`, `retargetFastener`, `rotateSelection`, `scaleSelection`, `translateSelection`, `worldRelocate` | artifact |
| `focusSelection` | window-config |
| `registerBrushMesh` | host-only |
| `selectSameKindSelection` | interaction |
| `setActiveExample` | artifact + config + interaction |

`applyBoardEvents` deliberately declares **three** lanes, not 2d's four: 5d's board-events work never writes a
`WindowTransient` (its brush candidate index rides the separate `cycleBrushCandidate` route). If a later slice
makes it write one, that lane must be added or the route faults at runtime.

## 6. Laws added (EDITOR5 `🧪️tests/🔬️unit/🦀️.rs`, region `🧩️ArtifactLaneVerbs`)

All of them dispatch through `app_with_registry()` → `dispatch_typed` → the real
`Puzzle5dRetainedCommandJobFactory`, and assert the DOCUMENT (or the pane's own `WindowConfig`, or the live
`interaction_state()`) really changed:

1. `artifact_lane_verbs_are_registered_migrated_retained_tools` — the 20-row table of verb → exact lanes, plus
   "no `BatchOnlyPendingRewrite` anywhere".
2. `duplicate_verb_ids_are_removed_rather_than_aliased`.
3. `set_active_example_loads_every_example_and_refuses_the_oversized_one` — empty / `concrete-forest` /
   `nakagin` each really replace the document (asserted against each example's own part count);
   `capsule-dream` refuses with exactly one notice and an unchanged document.
4. `focus_selection_publishes_the_addressed_pane_camera` — world pane's orbit target and board pane's flat
   camera both move, no document edit.
5. `translate_selection_moves_both_poses_and_refuses_a_locked_part` — dual-pose coherence and the locked refusal.
6. `board_node_delete_removes_the_part_and_its_fasteners` — the delete-node fold (part + every incident
   fastener) and the selection clear.
7. `add_part_kind_creates_a_part_and_reselects_it`.
8. `select_same_kind_widens_the_live_selection`.
9. `fastener_crud_reaches_the_document`.
10. `patch_part_writes_the_document_and_notices_an_inapplicable_edit`.

Three pre-existing laws that named the removed twins were updated in place
(`add_part_kind_route_is_cursorized`, `every_dispatched_action_bridges_to_a_command`).

## 7. Commands run / verdicts

See §9 — this section is filled in from `🗑️generated/5A2/`.

## 8. NOT verified / hand-offs

- **`capsule-dream` is now unreachable via `setActiveExample`.** Its switch costs 2 880 parts + 2 865
  fasteners ≈ 5 749 semantic units against the shared ceiling
  `crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS = 4 096`
  (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:12`). Per the slice brief it now **refuses with a
  notice instead of faulting**, which is the honest behaviour under the current ceiling — but the example
  itself stays unloadable. That constant is shared by 2d/3d/5d and their fixtures all declare `workItems: 4096`,
  so raising it is a **coordinator decision**, not a slice edit. Nakagin (180 parts / 179 fasteners) is well
  inside the ceiling and loads.
- **The publication-authority audit could not be run green** — `bun ./📜️script.ts publication-authority-audit`
  aborts in `validateWindowOwnershipSchemas` on `Puzzle2dWindowConfig` (missing `areaBrushWidth`/
  `areaBrushHeight`), i.e. slice **2F**'s area-brush schema landing mid-flight; that gate runs before any owner
  is inspected, so no `Puzzle5dPlayApp` verdict is obtainable until 2F settles. The 5d half of the oracle was
  replicated locally instead (pairs ↔ fixture routes, retained ids ↔ migrated, contracts ↔ fixture lanes,
  proof ids ↔ migrated, host-only exclusivity, non-empty groups) and all six checks pass.
- **No browser/battery evidence.** This slice is source + laws only; the coordinator owns activate/serve/battery.
- **`focusSelection`'s framing distance** is half the diagonal of the framed parts' axis-aligned bound times
  three plus two (3d's constant); it has not been eyeballed in a live pane.
- **5B** owns the fill/brush tool definitions; 5A2 owns the `addBrushPart` + `registerBrushMesh` Works and the
  shared `register_brush_mesh::puzzle5d_install_brush_mesh` install helper — call that, do not re-derive.
- **5C** consumes `patchPart`/`patchGrip`/`patchFastener`: all three are live, Artifact-lane, and `patchPart`
  now surfaces `edit_not_applicable` when a field is not writable. 5C still owes the render-side
  `InteractionView` threading in `📌️panels/🔍️inspection`.
- **5D** consumes `addPartKind`: live, Artifact + Interaction, re-selects what it creates, and its declared
  `partKind` arg option list is still the hard-coded single `"Part"` row (5D's row to widen).
- **5E/5G**: `applyBoardEvents` now owns the board `select` row as an interaction write — a hover/option slice
  adding new board event names must extend `Puzzle5dBoardEventsStage::Dispatch`, not a second reducer.
