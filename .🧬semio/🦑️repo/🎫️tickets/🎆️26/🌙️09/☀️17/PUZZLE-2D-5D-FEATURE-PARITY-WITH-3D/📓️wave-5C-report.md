# 📓️ Wave 5C — puzzle 🖐️5d PANELS to parity with 🧊️3d (and 2d where 2d is ahead)

Slice 5C. Repo root `/Users/ueli/Documents/semio`.
EDITOR5 = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
Line numbers are as-of this write; six sibling slices edit the same editor root concurrently, so treat
them as anchors, not addresses.

## 1. What landed

### 1.1 Editor-root helpers (new, `EDITOR5/🦀️.rs`)

| region | helper | what it is |
|---|---|---|
| `🏷️Labels` `:976` | `puzzle5d_kind_catalog_label` `:979` | a part kind's catalogue display name (`label` → `name` → id), else the kind id — the 5d twin of `puzzle3d_kind_catalog_label` |
| | `puzzle5d_part_display_label` `:1009` | **authored label → flat `text` → catalogue display name → id**. 3d's precedence with 5d's second authored carrier (the flat aspect's `text`) inserted after the label |
| | `puzzle5d_next_part_label` `:1024` | auto-numbering: first instance of a kind takes the bare catalogue name, each further one appends ` 2`, ` 3`, … to the root its peers carry (port of `puzzle3d_next_object_label`, including the `label_root`/`label_number` pair) |
| `🧬️InferredKinds` `:1048` | `puzzle5d_inferred_part_kind_rows` `:1054` | the part-kind rows a catalogue-less document IMPLIES — one row per distinct `partKind`, shaped from the first part carrying it (`shape`/`radius`/`width`/`height`/`iconKind`/`meshUrl` + that part's grips as templates). Mirrors 2d's `inferred_node_kind_rows` |
| | `puzzle5d_inferred_kind_rows` `:1102` | the same for the `grips`/`fasteners` slices (`{id, name}` per distinct kind); `ropes` has no document carrier and stays empty |
| `🙈️SelectionFlags` `:1123` | `apply_puzzle5d_selection_flag` `:1123` | sets `hidden`/`locked` on exactly the named parts. Only a part carries these flags in 5d (grip/fastener have no such field), so any other `entity` is a no-op, not a fault |

Two new clamp bands, `EDITOR5/🦀️.rs:119` `PUZZLE5D_PROXIMITY_RADIUS_MAX = 25.0` and `:121-122`
`PUZZLE5D_CHUNK_SIZE_MIN = 0.25` / `PUZZLE5D_CHUNK_SIZE_MAX = 512.0`.

### 1.2 Inspection panel — live per-entity field groups
`EDITOR5/📌️panels/🔍️inspection/🦀️.rs` (48 → 232 lines, rewritten).

The 48-line version always rendered the document summary because `ArtifactApp::render` had no selection.
Slice 5E landed `Puzzle5dInteractionSnapshot` and put it on `Puzzle5dScene` while this slice ran; the
panel now reads `envelope.interaction` and this slice threaded the live domain into
`render_with_request_context` (`EDITOR5/🦀️.rs`, `scene_from_projection_with_interaction(…,
Puzzle5dInteractionSnapshot::from_interaction(interaction))` — the parameter was `_interaction`).

- `selected_section` precedence **grip → part → fastener**, then three id-only fallbacks (leftover
  selection naming a part, naming a fastener, naming a grip → that grip's part), then the summary.
- `part_fields`: id, kind, label, resolved display label, flat text, flat x, flat y, radius, volume
  origin, orientation, scale, grip count, `hidden`, `locked`.
- `grip_fields`: full id, owning part, kind, flat angle, radius, position, direction.
- `fastener_fields`: id, kind, source, target, gap, shift, rise, rotation, turn, tilt.
- Mutating rows dispatch `patchPart {partIds, field, value}` / `patchGrip {gripFullIds, …}` /
  `patchFastener {fastenerIds, …}` — the exact arg shape those three command files already accept
  (`🎮️commands/{🩹️patch-part,✊️patch-grip,🪛️patch-fastener}/🦀️.rs`). `patch_row` **sorts its map keys**
  before admission, because `UiMapBuilder::push` refuses a key that does not exceed the last one and
  `fastenerIds` sorts before `field` while `partIds`/`gripFullIds` sort after it.
- The two flag rows dispatch `setSelectionFlag {entity, flag, ids, value}` with `UiValue::Bool(!pressed)`
  — **not** 3d's hardcoded-`true` outliner defect.
- The selected-id list stays a `window_section` (`IDS_SECTION = "puzzle5d-play-inspector.ids"`), stamping
  the whole selection and materialising the host's slice, on the peer fleet's `TreeWindows` mechanism.
  No `setPanelPage`, no `+N`.

### 1.3 Artifact / outliner panel — display labels + hide/lock
`EDITOR5/📌️panels/🗿️artifact/🦀️.rs`.

- `part_label` (flat text → volume label → kind) **deleted**; part rows and both sides of a fastener row
  now read `puzzle5d_part_display_label`.
- New `flag_args` / `hide_lock_actions` / `with_hide_lock_actions`: two INLINE `RowAction`s per part row
  (`eye`/`eye-off`, `lock`/`lock-open`) dispatching `setSelectionFlag` with the INVERSE of the row's
  state, and the row is dimmed while hidden. Grip and fastener rows carry no toggle (no such field).
- `with_hide_lock_actions` degrades: if the process-global argument arena refuses a row's toggles the row
  is still built as a pick target, instead of propagating the refusal and ending the section at zero rows.
  This is 3d's own post-migration shape, kept so a big tree cannot starve sibling panels.
- Row actions are not bindings, so the existing law "every row carries zero per-row bindings, the root
  carries the single `interactionSelect`" still holds.

### 1.4 Catalogue panel — inferred kind rows
`EDITOR5/📌️panels/🛍️catalogue/🦀️.rs`.

- `render`'s `slice(key)` now falls back to `puzzle5d_inferred_kind_rows(document, key)` for **every**
  slice, not just a flat id-dedup for `parts`. None of 5d's three examples (`concrete-forest`,
  `nakagin-capsule-tower`, `capsule-dream`) authors `kindCatalogs`, so before this every 5d catalogue
  section was empty (E9 OWED 1).
- `catalog_kind_label` made `pub` so the editor root's label helper reuses the one implementation.
- The drag payload now carries `objectKind` and `meshUrl` beside `kindId`, so one catalogue row drags
  into the board pane (which reads `kindId`) and the world pane (whose engine-side ghost reader resolves
  `objectKind` + `meshUrl`). MIME is unchanged and already the framework's
  `application/x-semio-catalogue-item`.
- Click still dispatches `addPartKind {partKind}` (5A2's verb).

Note: 5d's **fill** precompute already had an inference fallback
(`EDITOR5/🧠️precompute/🦀️.rs::puzzle3d_kind_catalogs`, the `authored == None` branch builds objects and
vortex kinds from the document), so only the catalogue half of E9 OWED 1 was actually missing.

### 1.5 Settings panel — NEW
`EDITOR5/📌️panels/⚙️settings/🦀️.rs` (new, 91 lines), tab id `puzzle5d.panel.settings`, body key
`puzzle.5d.play.settings`, group `Settings`. Seven `NumberStepper`s, all `uniform: true`, each tagging
the pane the panel resolves to via `panel_window_id` (render's own window → focused pane → first live
instance → `None`; an empty `windowId` is refused as an unknown instance, so it is omitted rather than
sent empty — 2d's rule, ported):

| field | step | verb | owner |
|---|---|---|---|
| fill count | 1 | `setFillCount` | existed (Migrated) |
| suggestion offset | 4 | `setSuggestionOffset` | 5A1 |
| contact tolerance | 0.001 | `setBrushPlacementContactTolerance` | 5A1 |
| proximity radius | 0.1 | `setProximityRadius` | **5C (new)** |
| chunk size | 1 | `setChunkSize` | **5C (new)** |
| grid factor | 0.25 | `setGridFactor` | 5A1 |
| grid spacing | 0.5 | `setGridSpacing` | 5E |

### 1.6 Two new Config-lane verbs
`setProximityRadius` and `setChunkSize` had no owner in the master plan's slice table, so this slice
added them end to end.

- `EDITOR5/🎮️commands/📡️set-proximity-radius/🦀️.rs`, `EDITOR5/🎮️commands/🧱️set-chunk-size/🦀️.rs` — the
  absolute-`value`-or-`delta` shape `setBrushPlacementContactTolerance` uses, clamped to the new bands.
- **Lane decision**: both fields live in the SHARED `Puzzle5dConfig`, not in a per-window config.
  3d keeps them per-window despite its own panel doc calling them session-wide; 5d has TWO window kinds,
  so a per-window home would mean either duplicating the field in both persisted structs or showing a
  default in whichever pane does not own it. 5d already keeps `fill_count`/`contact_tolerance` shared,
  and the placement search means the same distance whichever pane triggered it. `grid_spacing` stays
  per-window where 5E put it.

### 1.7 `setSelectionFlag` accepts explicit ids
`EDITOR5/🎮️commands/🚩️set-selection-flag/🦀️.rs`: explicit `{entity, ids}` (the outliner's and the
inspector's row actions) flags exactly those; otherwise the whole live part selection is flagged (the
context menu's path). Ported verbatim from 3d's `🎮️commands/🔖️set-selection-flag/🦀️.rs`. The default
`value` also flipped from `false` to `true` to match 3d, since an explicit arg always accompanies a real
click.

### 1.8 Label stamping at creation
`puzzle5d_next_part_label` is called at the two non-retained creation sites this slice could touch
without colliding with 5A2/5B:
- `EDITOR5/🦀️.rs::add_palette_part` (catalogue/board drop),
- `EDITOR5/🎮️commands/🖌️add-brush-part/🦀️.rs::puzzle5d_place_brush_part` (the non-candidate branch).

## 2. Registries touched

For `setProximityRadius` + `setChunkSize` (both `InteractiveJobClassification::Migrated`, lane `Config`):
- `puzzle5d_command_variants!` (`SetProximityRadius`, `SetChunkSize`) — this also gives
  `command_from_action` and `command_id` for free via the macro.
- `handle_action_impl`'s dispatch match.
- `PUZZLE5D_RETAINED_TOOL_IDS` and `PUZZLE5D_WINDOW_TOOL_IDS` (they are one-turn window-command work,
  exactly like `setBrushPlacementContactTolerance`).
- `PUBLICATION_CONTRACTS` → `ArtifactToolPublicationLane::Config`.
- `bounded_first_step_tool_proofs!` tools list.
- `.view_action(...)` + `.action_interactive_job(..., Migrated)` in `create_puzzle5d_app`.
- EN/DE labels are on the `ActionDefinition`s (`Set Proximity Radius` / `Näherungsradius festlegen`,
  `Set Chunk Size` / `Blockgröße festlegen`).
- Fixtures: `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` `evidenceToolIds`, and
  `🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` 5d `migrated`/`config` routes.

Panel registration: `.panel_tab_def(settings_panel::definition())` in `create_puzzle5d_app`, the two
render-body match arms, and `pub mod settings` + the two new command mods in
`🗿️artifacts/🖐️5d/🦀️.rs`'s `#[path]` tree.

Terminology (`EDITOR5/🗣️terminology/🦀️.rs`, EN+DE+reuse, compile-checked): `fastener`, `orientation`,
`scale`, `hidden`, `locked`, `settings`, `grid`, `grid_factor`, `spacing`, `proximity_radius`,
`chunk_size`.

Schema twins for the two new shared-config fields, hand-edited in all five
(`EDITOR5/🎚️config/🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts,🦀️.rs}`): `proximityRadius` and
`chunkSize`, including the TS guard's `parsePuzzle5dConfig` and the JSON schema's `required` list.

## 3. Laws added

| file | laws |
|---|---|
| `📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | selected part renders its group + both flag rows; selected grip wins over its part; selected fastener renders every joint scalar; a granularity-less leftover selection still resolves; a 120-id selection stamps every id and materialises only its window; the empty-selection summary (kept) |
| `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | authored labels + peer numbering (`puzzle5d_part_display_label` / `puzzle5d_next_part_label`); every part row carries exactly two row actions and the already-hidden row asks for `hidden:false` |
| `📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | a catalogue-less document offers the kinds it already names (2 distinct part kinds from 3 parts, 1 grip kind), each row keeping its `addPartKind` binding; an authored slice always wins over the inference |
| `📌️panels/⚙️settings/🧪️tests/🔬️unit/🦀️.rs` (new) | all seven steppers authored and each dispatching its verb; `panel_window_id` resolution (none / focused / first instance); the registered app really serves the settings body |

The three pre-existing panel test files' `scaled_scene` helpers also gained the `interaction` field 5E
added to `Puzzle5dScene` (they did not compile without it).

## 4. Commands run

_pending — see §6._

## 5. NOT verified / known gaps

- **`🗄️retained-jobs/🔣️.json` `toolIds` is badly out of sync with `PUZZLE5D_RETAINED_TOOL_IDS`** and this
  slice deliberately did not resync it. The fixture lists 16 ids; the source list is now 65 and still
  churning under 5A1/5A2/5B/5E/5G. The plugin-root law
  (`🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`)
  asserts the array matches **exactly and in order**, so it is red until someone regenerates it once the
  verb churn stops. It also currently lists `importComposeKit`, `setFixtureJson` and `zoomToSelection`,
  which are no longer in the source list. **Wave-2 integration item, not 5C's to land mid-churn.**
- The publication-authority fixture's 5d entry is likewise stale for every verb 5A1/5A2 migrated; 5C only
  added its own two `config` routes.
- No browser/battery verification: this slice ran no Nx, no serve, no Playwright (coordinator owns those).
- The inspector's numeric rows are `tree_item_with_action` rows carrying a staged `value`, not
  `NumberStepper` controls — same as 3d's inspector. A typed edit therefore depends on the host's row-edit
  affordance, which is unverified in the browser for 5d.
- `patchPart`/`patchGrip`/`patchFastener` reachability depends on 5A2 finishing their migration; if they
  are still `BatchOnlyPendingRewrite` at integration, the inspector reads correctly but the write-back is
  dead.

## 6. Hand-offs

- **5A2 / 5B — label stamping.** Call
  `crate::editor::puzzle5d::puzzle5d_next_part_label(&parts, &document, part_kind)` and put the result in
  `part_3d.label` at the three RETAINED creation sites this slice did not touch (they build
  `crate::Puzzle5dPart`, the schema type, not the editor twin): `Puzzle5dAddNodeWork`'s `Create` stage,
  `Puzzle5dAddBrushPartWork`'s `Create` stage, and the duplicate-selection path. Also the fill run-ops if
  5B materialises parts there. Without it the outliner falls back to the flat `text` those sites already
  stamp, which is the raw kind id.
- **5E — grid spacing.** The settings panel binds `setGridSpacing`; that verb and `grid_spacing` are
  yours (per-window config). No duplicate was introduced. One shared-config default function,
  `default_grid_spacing`, went missing from `🎚️config/🦀️.rs` during a concurrent write and was restored
  at its 10.0 value — check that is still what you intend.
- **5G — voxel dims** landed in `Puzzle5dRuntime`/`Puzzle5dWindowConfig` while this ran; the settings
  panel does not expose them (they belong to the volume-brush option group).
- **Coordinator — wave 2.** Regenerate `🗄️retained-jobs/🔣️.json`'s `toolIds` from
  `PUZZLE5D_RETAINED_TOOL_IDS` and the publication-authority 5d entry from the classification table once
  every 5-slice has landed, then run
  `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp`.
