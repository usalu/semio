# 📓️ Wave 5A2 — puzzle 🖐️5d Artifact-lane + interaction/viewport verbs migrated off `BatchOnlyPendingRewrite`

Slice: 5A2. Editor root (EDITOR5):
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`.

## 1. Headline

Every verb in this slice's remit is now `InteractiveJobClassification::Migrated`, registered in every
registry the executor rules list, and routed to a real retained `PuzzleCommandWork`. Together with the other
5-slices the whole app is clean: **70 app verbs, 0 `BatchOnlyPendingRewrite`, 0 `Unclassified`** — a grep for
`BatchOnlyPendingRewrite` over the production half of EDITOR5 returns nothing, and a law pins it.

Most of the Work structs (`Puzzle5dTransformWork`, `Puzzle5dPatchPartWork`/`…GripWork`/`…FastenerWork`,
`Puzzle5dCreateFastenerWork`, `Puzzle5dEditFastenerWork`, `Puzzle5dRetargetFastenerWork`,
`Puzzle5dDeleteFastenerWork`, `Puzzle5dProximityConnectWork`, `Puzzle5dAddNodeWork`,
`Puzzle5dAddBrushPartWork`, `Puzzle5dBoardEventsWork`, `Puzzle5dWorldRelocateWork`,
`Puzzle5dSetActiveExampleWork`, `Puzzle5dFocusSelectionWork`) already existed and were already routed by
`build_tool_job` — they were simply **never registered**, so none of them could ever run. The bulk of this
slice is registration plus repairing the silent/dead paths those works still carried.

## 2. Duplicate verbs resolved the greenfield way (no aliases)

| kept | removed | why |
|---|---|---|
| `focusSelection` | `zoomToSelection` | matches 3d; the context-menu row, the command variant, the tool-id list, the proof list, the publication contract and the `ActionDefinition` all moved to the survivor. Command directory renamed `🔎️zoom-to-selection` → `🎯️focus-selection` (module `focus_selection`); `🎯️focus-selection` is already an allowed taxonomy member name (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:9131`). |
| `addBrushPart` | `addBrushObject` | one placement verb; `.mutation`, `.action_args`, the command variant and the `build_tool_job` arm all dropped the twin. |
| `selectSameKindSelection` | `selectSameKind` | 3d's own surviving id, already the one `puzzle5d_context_menu_items` addresses, already in the fixtures. |

Law `duplicate_verb_ids_are_removed_rather_than_aliased` asserts the removed ids appear nowhere in the
production source and have no `Puzzle5dCommand` variant.

## 3. What landed

### 3.1 New plumbing the whole slice rides on
- `Puzzle5dActionCtx` gained `view_state`, `effects: Vec<Effect>` and `interaction_writes: Vec<InteractionWrite>`,
  plus `replace_selection`, `clear_selection`, `notice`, `refuse_without_selection`, `refuse_when_locked`
  (EDITOR5 `🦀️.rs:4223-4287`) — ported 1:1 from puzzle 3d's `Puzzle3dActionCtx`. `handle_action_impl` threads
  both lanes into `Emit`, and an **aborted** arm still publishes the refusal notice it pushed (3d's fix:
  "refused" and "silently did nothing" must not look alike).
- `PUZZLE5D_LOCALIZATION_UNSUPPORTED` + `puzzle5d_notice_emit(view_state, message)` (`🦀️.rs:4289-4297`) — the
  terminal-`Emit` notice a retained Work completes with.
- `PUZZLE5D_FLAT_TO_WORLD: f64 = 1.0 / 48.0` (`🦀️.rs:≈699`) — the ONE board↔world scale, now used by
  `add_palette_part`, `Puzzle5dAddNodeWork`, `Puzzle5dAddBrushPartWork` and the transform work instead of three
  separate `1.0 / 48.0` literals.
- Terminology (`🗣️terminology/🦀️.rs:75-79`, EN+DE, compile-checked): `focus_selection` (replacing
  `zoom_to_selection`), `nothing_selected`, `selection_locked`, `example_too_large`, `edit_not_applicable`.

### 3.2 Per-verb repairs (not just registration)
- **`focusSelection`** — `Puzzle5dFocusSelectionWork` computed a camera target and then **threw it away**
  (`let _ = (target, divisor, config); … Emit::default()`). It now binds `view_state`/`window_config`,
  accumulates an axis-aligned bound in the one part scan it already declares, and publishes
  `window_config_mutations` via `window_ownership::addressed_config` — board pane → `camera2d`, world pane →
  `camera3d`. An EMPTY selection frames the WHOLE document (3d's `frames()` fix); with nothing to frame at all
  it completes with one notice.
- **`applyBoardEvents`** — four real defects fixed in `Puzzle5dBoardEventsWork`:
  1. the `camera` row was decoded and then **discarded** (`self.camera2d.take(); let config_mutations = Vec::new();`)
     → board pan/zoom never persisted; it now publishes the pane's `WindowConfig`;
  2. `select` rows were ignored → the LAST `select` row of a batch becomes one `Replace` interaction write;
  3. `nodeDelete`/`edgeDelete` left a dangling selection → removed ids are cleared with a **Subtractive** write
     naming exactly those ids (an empty `Replace` is a silent no-op in `protocol::next_selection`);
  4. a `brushPlace` row's created part is re-selected; a drag of a **locked** part is skipped and surfaces one
     notice. A board drag still moves ONLY the flat pose — that is what makes it a plan edit.
- **`translateSelection` / `rotateSelection` / `scaleSelection`** — `Puzzle5dTransformWork` moved only the 3d
  pose, so a world drag left the board pin behind. `translateSelection` now emits `move_part_2d` alongside
  `move_part_3d`, projecting the same world delta through `PUZZLE5D_FLAT_TO_WORLD`; rotate/scale leave the flat
  pose alone (neither moves a part's own origin). Locked parts are skipped and, when every addressed part was
  locked, the gesture completes with one notice and no edit. An empty selection completes with a notice. One
  history edit per gesture is unchanged (`coalesce_key` `gumball-translate|rotate|scale`).
- **`setActiveExample`** — extent used to return `None` for an example over the fixed work ceiling, which
  surfaces as the framework's opaque "exceeds fixed semantic work capacity" fault. It now declares one unit in
  that case and completes with the localized `example_too_large` notice, document untouched. It also clears the
  stale selection (Subtractive) — every part of the old document is gone.
- **`selectSameKindSelection`** — `puzzle5d_retained_reduce` **fail-closed** on this id
  (`"…ArtifactEditor has no route-specific interaction-selection publication primitive"`). That branch is
  deleted; `select_same_kind` emits one `InteractionWrite::replace` through `ctx.replace_selection`, and the
  contract moved `HostOnly` → `Interaction`.
- **`registerBrushMesh`** — was falling through to `BoundedFirstStepCommandWork`. New
  `Puzzle5dRegisterBrushMeshWork` pages the two number arrays in at `PUZZLE5D_MESH_PAGE_VALUES = 512` values per
  step and then calls the shared `register_brush_mesh::puzzle5d_install_brush_mesh` — the SAME install the
  synchronous `handle` arm uses, so there is one mechanism, not two. A rejected install surfaces one notice.
- **`addNode` / `addPartKind` / `addBrushPart`** — all three re-select the part they created
  (`InteractionWrite::replace`), so the next patch/transform/delete addresses the new part.
- **`patchPart`** — a patch that addressed nothing produced an empty `Emit` silently; it now completes with one
  `edit_not_applicable` notice.
- **`createFastener` / `deleteFastener` / `editFastener` / `retargetFastener` / `patchGrip` / `patchFastener` /
  `proximityConnect` / `worldRelocate`** — their Works were already correct and cursored; this slice registered
  them and declared their lanes.

## 4. Registries touched (all four agree — verified mechanically and by law)

- `puzzle5d_command_variants!` — `AddBrushObject`, `ZoomToSelection`, `SelectSameKind` removed.
- `PUZZLE5D_RETAINED_TOOL_IDS` (= `TOOL_JOB_IDS`) — 19 ids added, `zoomToSelection` → `focusSelection`.
- `PUZZLE5D_WINDOW_TOOL_IDS` — `zoomToSelection` removed (`focusSelection` owns its own Work).
- `bounded_first_step_tool_proofs!` `tools:` — same ids. The contract band
  `resumable(PUZZLE5D_RETAINED_RAW_BYTES = 262_144, PUZZLE5D_RETAINED_DECODED_ITEMS = 16_384, 1, 262_144, 7_500, 1, 1)`
  was **already raised by slice 5A1** (first-one-there sets it); 5A2 left it alone and verified both constants
  are threaded into `Puzzle5dRetainedCommandJobFactory::new` and its wire admission.
- `build_tool_job` — `registerBrushMesh` arm added; `addBrushObject`/`selectSameKind` arms removed.
- `PUBLICATION_CONTRACTS` — 19 rows added (lanes in §5).
- `.action_interactive_job(…, Migrated)` in `create_puzzle5d_app` — 19 rows flipped, 3 rows deleted.
- `.mutation`/`.action_with`/`.action_args`/`command_from_action` — the three removed twins dropped;
  `focusSelection` promoted to a `bounded_catalog` action with the `view` category (it is the keybinding /
  context-menu / engagement-bar verb).
- `puzzle5d_context_menu_items` — the Zoom row dispatches `focusSelection`.
- `🗣️terminology/🦀️.rs` — five label rows, EN + DE, all four locale×terminology cells.
- Fixtures:
  - `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` — stale `addBrushObject` cursor dropped; semantic cursors added
    for `registerBrushMesh` (`positionPage`/`indexPage`/`derive`/`closeOwner`), `selectSameKindSelection`
    (`interactionWrite`) and `setActiveExample` (a leading `capacityGate`); **13 new vectors**
    (`setActiveExampleNakagin`, `setActiveExampleConcreteForest`, `setActiveExampleOverCapacity`,
    `focusSelectionPublishesBothCameras`, `focusSelectionEmptyFramesDocument`, `boardNodeDeleteClearsSelection`,
    `boardCameraPublishesWindowConfig`, `boardNodeMoveLockedRefused`, `translateKeepsFlatPoseCoherent`,
    `translateLockedRefused`, `addPartKindReselectsCreatedPart`, `selectSameKindWidensSelection`,
    `registerBrushMeshPages`). **`capacities` deliberately stays at the shared puzzle defaults**
    (8 192 / 512 / 4 096 / 262 144 / 7 500 / 1): `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
    (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:140,152`) pins that array against the
    `PUZZLE_COMMAND_*` constants, and the `maxPlusOne` fingerprint `8193:…` pins the raw-byte vector. The widened
    band is a property of the FACTORY, not of this fixture — an earlier 5A2 edit raised it here and was correctly
    reverted by the 5d registry re-sync.
  - `🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` — the whole `Puzzle5dPlayApp` owner block regenerated
    mechanically from `PUBLICATION_CONTRACTS`: 10 groups, 74 routes, every group `migrated`, **no `blocker` rows
    left**. That regeneration also picked up slice **5G**'s five target-volume verbs (`addTargetVolume`,
    `deleteTargetVolume`, `relocateTargetVolume`, `setTargetVolumeFlag`, `setVoxelDims`), which had landed in the
    source but not in this fixture and were blocking the audit's bijection — their lanes are taken verbatim from
    5G's own contract rows, nothing invented.

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

## 6. Laws added (EDITOR5 `🧪️tests/🔬️unit/🦀️.rs`, region `🧩️ArtifactLaneVerbs`, lines 1634-1888)

All the behavioural ones dispatch through `app_with_registry()` → `dispatch_typed` → the real
`Puzzle5dRetainedCommandJobFactory`, and assert the DOCUMENT (or the pane's own `WindowConfig`, or the live
`interaction_state()`) really changed:

1. `artifact_lane_verbs_are_registered_migrated_retained_tools` — the 18-row verb → exact-lanes table, the two
   multi-lane verbs, and "no `BatchOnlyPendingRewrite` anywhere".
2. `duplicate_verb_ids_are_removed_rather_than_aliased`.
3. `set_active_example_switches_the_document_and_never_faults_on_capacity` — each shipped example id reaches the
   document with that example's own part count, AND `extent` answers `Some(_)` inside the work ceiling for every
   one of them, so the verb can never produce the opaque capacity fault; the oversized path is the localized
   `example_too_large` notice.
4. `focus_selection_publishes_the_addressed_pane_camera` — world pane's orbit target and board pane's flat
   camera both move, no document edit.
5. `translate_selection_moves_both_poses_and_refuses_a_locked_part` — dual-pose coherence through
   `PUZZLE5D_FLAT_TO_WORLD`, and the locked refusal with a notice.
6. `board_node_delete_removes_the_part_and_its_fasteners` — an `edgeCreate` batch then a `nodeDelete` batch: the
   part, the incident fastener and the selection all go.
7. `add_part_kind_creates_a_part_and_reselects_it`.
8. `select_same_kind_widens_the_live_selection`.
9. `fastener_crud_reaches_the_document` (edit + delete).
10. `patch_part_writes_the_document_and_notices_an_inapplicable_edit`.

Helper `seeded_parts(app, count)` builds the document out of the app's OWN live verbs (`setActiveExample ""`
then N × `addPartKind`) rather than reading a shipped example file — see §8's first bullet for why that matters.

Pre-existing laws that named the removed twins were updated in place (`add_part_kind_route_is_cursorized`,
`every_dispatched_action_bridges_to_a_command`), and the `Puzzle5dScene` literals in
`🎮️commands/📡️proximity-connect/🧪️tests/🔬️unit/🦀️.rs` were carried to the new `interaction` field (that file is
one 5A2 edited).

## 7. Commands run — real verdicts

| command | verdict |
|---|---|
| `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --all-targets --message-format=short` | **exit 0**, 0 errors, `Finished dev profile in 17m 05s`; 36 warnings for `semio-s-artifact-puzzle-5d (lib test)` (3 duplicates), **0 of them inside slice 5A2's lines** (all are `unnecessary qualification` / unused-import in 5G's `🔬️target-volumes`, the window/viewer tests, the fill precompute tests, and one pre-existing `parse_example_dsl is never used`). → `🗑️generated/5A2/check-native.txt` |
| `CARGO_INCREMENTAL=0 cargo check … --target wasm32-wasip2 --message-format=short` | **exit 0**, `Finished dev profile in 5.40s`, 3 lib warnings, none in 5A2's lines. → `🗑️generated/5A2/check-wasm.txt` |
| `CARGO_INCREMENTAL=0 cargo test … --lib -- <the 10 law names>` | **exit 0 — 10 passed, 0 failed**, 560 filtered out. → `🗑️generated/5A2/test-5a2.txt` |
| `CARGO_INCREMENTAL=0 cargo test … --lib -- language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle retained_publication_contracts_are_an_exact_nonempty_tool_bijection` | **exit 0 — 2 passed** (both fixtures still match the production catalogs). → `🗑️generated/5A2/test-fixtures.txt` |
| `bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` | **exit 0** — `validated Puzzle publication authority; owners=Puzzle5dPlayApp; admitted=73 routes; windowOwnershipCases=7; schema=Ajv; oracle=independent`. → `🗑️generated/5A2/publication-audit.txt` |
| `python3 🗑️generated/5A2/publication-oracle-5d.py` | 8/8 PASS (owner oracle + 6 hostile mutations). Written while the shipped audit was red on slice 2F's 2d window schema; kept as a fast local re-check. → `🗑️generated/5A2/publication-oracle-5d.txt` |

## 8. NOT verified / owed to integration / hand-offs

- **🔴 PRE-EXISTING, NOT THIS SLICE: `🌙️capsule-dream` and `🏗️nakagin-capsule-tower` load as EMPTY documents.**
  `document_from_json` (`EDITOR5 🦀️.rs:403-405`) answers `empty_document()` on any serde failure, so an example
  whose JSON does not deserialize into `Puzzle5dDocument` silently becomes a zero-part document. The already-red
  law `clipboard_verbs_cover_every_part_of_the_largest_example` (`🧪️tests/🔬️unit/🦀️.rs:1199`) fails with
  `left: 0, right: 2880` **on its own filter**, independent of anything 5A2 changed. Consequences the integrator
  must plan for: `setActiveExample nakagin`/`capsule-dream` succeed but land nothing; any battery step that
  expects Nakagin's 180 parts will see 0. My laws therefore build their documents from live verbs
  (`seeded_parts`) so they prove the VERBS rather than the example files. **Owner: whoever owns 5d example data
  — the fix is to make `document_from_json` fail loudly (or the example JSON deserialize).**
- **The `capsule-dream` capacity gate is proven by contract, not by data.** With the example loading empty its
  switch costs a handful of units, so the oversized branch cannot be exercised end to end today. The law asserts
  the property that matters (`extent` is never `None`) plus the notice path; the fixture vector
  `setActiveExampleOverCapacity` records the intended 2 880-part / 2 865-fastener case. Once the example data is
  fixed, the switch will cost ≈5 749 units against
  `crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS = 4 096`
  (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:12`) and will refuse with the notice. Raising that shared
  constant is a **coordinator decision** — it is shared by 2d/3d/5d and all three fixtures pin it.
- **No browser/battery evidence.** Source + laws only; the coordinator owns activate/serve/battery.
- **`focusSelection`'s framing distance** is half the diagonal of the framed parts' axis-aligned bound × 3 + 2
  (3d's constant); not eyeballed in a live pane.
- **Stray `[DEBUG] maintenance idle probe …` logging** floods the 5d test output (hundreds of lines). It is not
  5A2's and not in 5A2's files; AGENTS.md rule 6 says it must be removed before that slice finishes.
- **Hand-offs.** 5B owns the fill/brush tool definitions; 5A2 owns the `addBrushPart` + `registerBrushMesh` Works
  and the shared `register_brush_mesh::puzzle5d_install_brush_mesh` install helper — call it, do not re-derive.
  5C consumes `patchPart`/`patchGrip`/`patchFastener`: live, Artifact-lane, and `patchPart` now surfaces
  `edit_not_applicable` when a field is not writable; 5C still owes the render-side `InteractionView` threading in
  `📌️panels/🔍️inspection`. 5D consumes `addPartKind`: live, Artifact + Interaction, re-selects what it creates.
  5E/5G: `applyBoardEvents` now owns the board `select` row as an interaction write — a slice adding new board
  event names must extend `Puzzle5dBoardEventsStage::Dispatch`, not add a second reducer.
