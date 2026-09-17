# 📓️ Wave 5C — puzzle 🖐️5d PANELS to parity with 🧊️3d (and 2d where 2d is ahead)

Slice 5C. Repo root `/Users/ueli/Documents/semio`.
EDITOR5 = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
Line numbers are as of 2026-09-17 ~20:10; six sibling slices edit the same editor root concurrently, so
treat them as anchors, not addresses.

## 0. Verdict at a glance

| gate | verdict |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly` (native) | **EXIT=0** at 15:38, `Finished dev profile … in 12m 28s`, 8 lib warnings (7 pre-existing in 5E's `🪟️window/🦀️.rs`, 1 mine → fixed) |
| same, re-run after the warning fixes, `--message-format=short` | **EXIT=0** at 20:22, `Finished dev profile … in 17m 13s`, **0 errors**, `generated 3 warnings` — `🦀️.rs:9412` and `🪟️window/🦀️.rs:275` (5E) and `🦀️.rs:137 parse_example_dsl is never used` (5A2/5D's example path). **No 5C file appears in the diagnostics.** |
| same with `--target wasm32-wasip2` | **EXIT=0** at 15:41, `Finished dev profile … in 3m 06s`, 2 lib warnings, **neither in 5C files** |
| `bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` | **EXIT=0**, `validated Puzzle publication authority`, `setProximityRadius`/`setChunkSize` in the admitted set |
| `bun ./📜️script.ts publication-authority-audit` (all owners) | EXIT=1 — fails on **Puzzle2dPlayApp**, not 5d |
| `cargo test … --lib -- panels::` | **could not run**: the shared `lib test` target is red from SIBLING files only (see §5). Zero errors in `📌️panels/**` across four attempts (26 → 16 → 14 sibling errors as siblings landed fixes). **Owed to integration.** |

Logs: `TICKET/🗑️generated/5C/{check-native-1.txt, check-native-final.txt, check-wasm-1.txt,
test-panels-{1,2,3,4}.txt, publication-audit.txt}`.

## 1. What landed

### 1.1 Editor-root helpers (new, `EDITOR5/🦀️.rs`)

| region | helper | what it is |
|---|---|---|
| `🏷️Labels` | `puzzle5d_kind_catalog_label` `:1004` | a part kind's catalogue display name (`label` → `name` → id), else the kind id — the 5d twin of `puzzle3d_kind_catalog_label`; reuses the catalogue panel's one `catalog_kind_label` implementation |
| | `puzzle5d_part_display_label` `:1034` | **authored label → flat `text` → catalogue display name → id**. 3d's precedence with 5d's second authored carrier (the flat aspect's `text`) inserted after the label |
| | `puzzle5d_next_part_label` `:1049` | auto-numbering: the first instance of a kind takes the bare catalogue name, each further one appends ` 2`, ` 3`, … to the root its peers carry. Port of `puzzle3d_next_object_label` including the private `puzzle5d_label_root`/`puzzle5d_label_number` pair |
| `🧬️InferredKinds` | `puzzle5d_inferred_part_kind_rows` `:1079` | the part-kind rows a catalogue-less document IMPLIES — one row per distinct `partKind`, shaped from the first part carrying it (`shape`/`radius`/`width`/`height`/`iconKind`/`meshUrl` + that part's grips as templates). Mirrors 2d's `inferred_node_kind_rows` |
| | `puzzle5d_inferred_kind_rows` `:1127` | the same for the `grips`/`fasteners` slices (`{id, name}` per distinct kind); `ropes` has no document carrier and stays empty, so its section keeps its placeholder |
| `🙈️SelectionFlags` | `apply_puzzle5d_selection_flag` `:1148` | sets `hidden`/`locked` on exactly the named parts. Only a part carries these flags in 5d (grip/fastener have no such field), so any other `entity` is a no-op, not a fault |

Three new clamp bands: `EDITOR5/🦀️.rs:119` `PUZZLE5D_PROXIMITY_RADIUS_MAX = 25.0`, `:122`
`PUZZLE5D_CHUNK_SIZE_MIN = 0.25`, `:123` `PUZZLE5D_CHUNK_SIZE_MAX = 512.0`.

### 1.2 Inspection panel — live per-entity field groups
`EDITOR5/📌️panels/🔍️inspection/🦀️.rs`, 48 → **244 lines**, rewritten.

The 48-line version always rendered the document summary, with an explicit doc comment saying selection
threading was a framework gap (E3 §5: "the exact same defect 3d had before W-S", a two-layer read+write
gap). Both layers are now closed:

- **Read.** Slice 5E landed `Puzzle5dInteractionSnapshot` and put it on `Puzzle5dScene` while this slice
  ran; 5C threaded the live domain into the request-context render —
  `EDITOR5/🦀️.rs` `render_with_request_context` now takes `interaction` (was `_interaction`) and builds
  the envelope through `scene_from_projection_with_interaction(…,
  Puzzle5dInteractionSnapshot::from_interaction(interaction))`. The panel reads `envelope.interaction`.
- **Write.** Every mutating row dispatches the entity's own patch verb with the explicit id list that
  verb already accepts.

Shape:
- `selected_section` precedence **grip → part → fastener**, then three id-only fallbacks (a leftover
  selection naming a part, naming a fastener, naming a grip → that grip's part), then the summary.
- `part_fields`: id, kind, label, resolved display label, flat text, flat x, flat y, radius, volume
  origin, orientation, scale, grip count, `hidden`, `locked`.
- `grip_fields`: full id, owning part, kind, flat angle, radius, position, direction.
- `fastener_fields`: id, kind, source, target, gap, shift, rise, rotation, turn, tilt.
- Verbs: `patchPart {partIds, field, value}`, `patchGrip {gripFullIds, …}`,
  `patchFastener {fastenerIds, …}` — the exact arg shape
  `🎮️commands/{🩹️patch-part,✊️patch-grip,🪛️patch-fastener}/🦀️.rs` accept today.
  `patch_row` **sorts its map keys before admission**, because `UiMapBuilder::push` refuses a key that
  does not exceed the last one and `fastenerIds` sorts *before* `field` while `partIds`/`gripFullIds`
  sort *after* it — a hand-written order would have killed one of the three groups at runtime.
- The two flag rows dispatch `setSelectionFlag {entity, flag, ids, value}` with `UiValue::Bool(!pressed)`
  — **not** 3d's hardcoded-`true` outliner defect.
- The selected-id list is a `window_section` (`IDS_SECTION = "puzzle5d-play-inspector.ids"`) on the peer
  fleet's `TreeWindows` mechanism: it stamps the whole selection and materialises the host's slice. No
  `setPanelPage`, no paged section, no `+N`.

### 1.3 Artifact / outliner panel — display labels + hide/lock
`EDITOR5/📌️panels/🗿️artifact/🦀️.rs`, 108 → **165 lines**.

- The local `part_label` (flat text → volume label → kind) is **deleted**; part rows and both sides of a
  fastener row now read `puzzle5d_part_display_label`.
- New `flag_args` / `hide_lock_actions` / `with_hide_lock_actions`: two INLINE `RowAction`s per part row
  (`eye`/`eye-off`, `lock`/`lock-open`) dispatching `setSelectionFlag` with the **inverse** of the row's
  state; the row is `dimmed` while hidden. Grip and fastener rows carry no toggle (no such field exists).
- `with_hide_lock_actions` degrades instead of propagating: if the process-global argument arena refuses
  a row's toggles the row is still built as a pick target. This is exactly 3d's post-migration shape and
  the rules file's "row actions on every big-tree row starve sibling panels" trap — the refusal is
  absorbed per row, never turned into a section-wide `Err`.
- Row actions are not bindings, so the peer fleet's law "every row carries zero per-row bindings, the
  tree root carries the single `interactionSelect`" still holds unchanged.

### 1.4 Catalogue panel — inferred kind rows
`EDITOR5/📌️panels/🛍️catalogue/🦀️.rs`, 126 → **130 lines**.

- `render`'s `slice(key)` now falls back to `puzzle5d_inferred_kind_rows(document, key)` for **every**
  slice, replacing a flat id-dedup that only covered `parts`. None of 5d's three examples
  (`concrete-forest`, `nakagin-capsule-tower`, `capsule-dream`) authors `kindCatalogs`, so before this
  every 5d catalogue section was empty — E9 OWED 1, the highest-priority item on that list.
- `catalog_kind_label` made `pub` so the root's label helper reuses one implementation.
- Drag payload now carries `objectKind` and `meshUrl` beside `kindId`/`partKind`, so one catalogue row
  drags into the board pane (which reads `kindId`) and into the world pane (whose engine-side ghost
  reader resolves `objectKind` + `meshUrl`). MIME is unchanged and already the framework's
  `application/x-semio-catalogue-item`.
- Click still dispatches `addPartKind {partKind}` (5A2's verb).

**Correction to E9 OWED 1**: 5d's *fill* precompute already had an inference fallback —
`EDITOR5/🧠️precompute/🦀️.rs::puzzle3d_kind_catalogs`, the `authored == None` branch, builds object kinds
and vortex kinds from the document itself. Only the catalogue half was actually missing. No second
mechanism was introduced.

### 1.5 Settings panel — NEW
`EDITOR5/📌️panels/⚙️settings/🦀️.rs` (new, **91 lines**), tab id `puzzle5d.panel.settings`, body key
`puzzle.5d.play.settings`, group `Settings`. Seven `NumberStepper`s, all `uniform: true` (a `false`
renders MIXED: a blank box whose +/− bump starts from `defaultValue`), each tagging the pane the panel
resolves to via `panel_window_id` — render's own window → focused pane → first live instance → `None`;
an empty `windowId` is refused as an unknown instance, so it is omitted rather than sent empty (2d's
rule, ported from EDITOR2's own settings panel).

| field | step | verb | verb owner |
|---|---|---|---|
| fill count | 1 | `setFillCount` | pre-existing (Migrated) |
| suggestion offset | 4 | `setSuggestionOffset` | 5A1 |
| contact tolerance | 0.001 | `setBrushPlacementContactTolerance` | 5A1 |
| proximity radius | 0.1 | `setProximityRadius` | **5C (new)** |
| chunk size | 1 | `setChunkSize` | **5C (new)** |
| grid factor | 0.25 | `setGridFactor` | 5A1 |
| grid spacing | 0.5 | `setGridSpacing` | 5E |

This is 3d's four steppers (contact tolerance, proximity radius, chunk size, grid spacing) **plus** 2d's
three (fill count, suggestion offset, grid factor) in one panel.

### 1.6 Two new Config-lane verbs
`setProximityRadius` and `setChunkSize` had no owner anywhere in the master plan's slice table, so 5C
added them end to end.

- `EDITOR5/🎮️commands/📡️set-proximity-radius/🦀️.rs`, `EDITOR5/🎮️commands/🧱️set-chunk-size/🦀️.rs` — the
  absolute-`value`-or-`delta` shape `setBrushPlacementContactTolerance` uses
  (`puzzle5d_absolute_or_delta`), clamped to the new bands.
- **Lane decision**: both fields live in the SHARED `Puzzle5dConfig`, not per-window. 3d keeps them
  per-window despite its own panel doc calling them session-wide; 5d has TWO window kinds, so a
  per-window home would mean duplicating the field in both persisted structs or showing a default in
  whichever pane does not own it. 5d already keeps `fill_count`/`contact_tolerance` shared, and the
  placement search means the same distance whichever pane triggered it. `grid_spacing` stays per-window
  where 5E put it; 5C did not touch it.

### 1.7 `setSelectionFlag` accepts explicit ids
`EDITOR5/🎮️commands/🚩️set-selection-flag/🦀️.rs`: explicit `{entity, ids}` (the outliner's and the
inspector's rows) flags exactly those; otherwise the whole live part selection is flagged (the context
menu's path). Ported from 3d's `🎮️commands/🔖️set-selection-flag/🦀️.rs`. The `value` default also flipped
from `false` to `true` to match 3d, since an explicit arg always accompanies a real click.

### 1.8 Label stamping at creation
`puzzle5d_next_part_label` is called at the two non-retained creation sites 5C could touch without
colliding with 5A2/5B:
- `EDITOR5/🦀️.rs:773` `add_palette_part` (catalogue/board drop),
- `EDITOR5/🎮️commands/🖌️add-brush-part/🦀️.rs:74` `puzzle5d_place_brush_part` (the non-candidate branch).

## 2. Registries touched

For `setProximityRadius` + `setChunkSize` (`InteractiveJobClassification::Migrated`, lane `Config`) — all
in `EDITOR5/🦀️.rs`:

| registry | line |
|---|---|
| `puzzle5d_command_variants!` (`SetProximityRadius`, `SetChunkSize`) — this also supplies `command_from_action` and `command_id` through the macro | `:4101-4102` |
| `handle_action_impl` dispatch match | `:4499-4500` |
| `PUZZLE5D_RETAINED_TOOL_IDS` | `:4583-4584` |
| `PUZZLE5D_WINDOW_TOOL_IDS` (they are one-turn window-command work, exactly like `setBrushPlacementContactTolerance`, so `build_tool_job` routes them through `Puzzle5dWindowCommandWork` with no new arm) | `:4628-4629` |
| `PUBLICATION_CONTRACTS` → `ArtifactToolPublicationLane::Config` | `:8571-8572` |
| `bounded_first_step_tool_proofs!` tools list | `:8945-8946` |
| `.view_action(...)` EN+DE | `:9669-9670` |
| `.action_interactive_job(..., Migrated)` | `:9725-9726` |

Panel registration: `.panel_tab_def(settings_panel::definition())` `:9598`; render arms `:9356-9357` and
`:9389-9390` (both `render` and `render_with_request_context`); `pub mod settings` plus the two new
command mods in `🗿️artifacts/🖐️5d/🦀️.rs`'s `#[path]` tree.

Terminology (`EDITOR5/🗣️terminology/🦀️.rs`, EN+DE × native+reuse, compile-checked by `app_labels!`):
`fastener`, `orientation`, `scale`, `hidden`, `locked`, `settings`, `grid`, `grid_factor`, `spacing`,
`proximity_radius`, `chunk_size`.

Schema twins for the two new shared-config fields, hand-edited in **all five**
(`EDITOR5/🎚️config/🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts,🦀️.rs}`): `proximityRadius` and
`chunkSize` — including the JSON schema's `required` list and the TS guard's `parsePuzzle5dConfig`
(`finite(...)` per field), and `double proximity_radius = 5; double chunk_size = 6;` in the proto.

Fixtures: `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` `evidenceToolIds` (minimal two-line edit,
sorted position preserved); `🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` 5d `migrated`/`config`
routes.

## 3. Laws added

| file | laws |
|---|---|
| `📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` (rewritten) | a selected part renders its field group and BOTH flag rows and never the summary; a selected grip wins over its owning part; a selected fastener renders every joint scalar (gap/shift/rise/rotation/turn/tilt); a granularity-less leftover selection still resolves to the part group; a 120-id selection stamps every id and materialises exactly the host's 8-row window; plus the pre-existing empty-selection summary law |
| `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` (appended `🏷️LabelAndFlagLaws`) | authored labels + peer numbering (`puzzle5d_part_display_label` / `puzzle5d_next_part_label`, including "first instance takes the bare catalogue name"); every part row carries exactly two row actions, zero per-row bindings, and the already-hidden row asks for `hidden:false` |
| `📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` (appended `🧬️InferenceLaws`) | a catalogue-less document offers the kinds it already names (2 distinct part kinds out of 3 parts, 1 grip kind), each part row keeping its own `addPartKind` binding; an authored slice always wins over the inference |
| `📌️panels/⚙️settings/🧪️tests/🔬️unit/🦀️.rs` (new) | all seven steppers authored and each dispatching its own verb; `panel_window_id` resolution (none / focused / first instance); the registered app really serves the settings body (no "Unknown body" placeholder) |

The three pre-existing panel test files' `scaled_scene` helpers also gained the `interaction` field 5E
added to `Puzzle5dScene` — they did not compile without it and 5E had not reached them.

## 4. Commands run

1. `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly`
   → **EXIT=0**, `Finished dev profile [unoptimized] target(s) in 12m 28s`, 169 warning lines in the
   whole log, `semio-s-artifact-puzzle-5d (lib) generated 8 warnings`. Seven were pre-existing
   `unnecessary qualification` in 5E's `🪟️window/🦀️.rs` and `🦀️.rs:9411`
   (`window_engagements_with_request_context`); the one that was mine
   (`.map(crate::editor::puzzle5d::panels::catalogue::catalog_kind_label)`) was fixed to
   `.map(catalogue::catalog_kind_label)`. Log: `🗑️generated/5C/check-native-1.txt`.
2. `… --target wasm32-wasip2` (with `DEVELOPER_DIR=/Library/Developer/CommandLineTools`)
   → **EXIT=0**, `Finished dev profile … in 3m 06s`, `generated 2 warnings`, **neither in a 5C file**
   (`🦀️.rs:9411` and `🪟️window/🦀️.rs:276`, both 5E's). Log: `🗑️generated/5C/check-wasm-1.txt`.
3. `bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` → **EXIT=0**. The all-owners run
   exits 1 on `Puzzle2dPlayApp publication authority diverged from the fixture` — a 2d-side slice, not
   5d. Log: `🗑️generated/5C/publication-audit.txt`.
4. `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib
   -- panels:: --test-threads=1` — four attempts (15:41, 16:00, 16:06, 16:11), all `TEST_EXIT=101` at
   **compile** time of the shared `lib test` target, 26 → 16 → 14 errors as siblings landed fixes.
   **Zero of those errors is in `📌️panels/**`** — grouping the diagnostics by path across every attempt
   gives only `🧪️tests/🔬️target-volumes/🦀️.rs` (5G), `🪟️window/🧪️tests/🔬️unit/🦀️.rs` (5E),
   `🧬️schema/💡️inferences/**` (5G), `🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`,
   `🎮️commands/📡️proximity-connect/🧪️tests/🔬️unit/🦀️.rs` (5D), `🎭️modes/✏️edit/🪟️windows/{◻️2d,🧊️3d}`
   and `👁️viewer/**`. The only 5C diagnostics were three `unnecessary qualification` warnings, all fixed.
   Logs: `🗑️generated/5C/test-panels-{1,2,3,4}.txt`.
5. Final gated `CARGO_INCREMENTAL=0 cargo check … --message-format=short` (20:05→20:22, covering the
   three post-check warning fixes) → **EXIT=0**, `Finished dev profile … in 17m 13s`, **0 errors**,
   `semio-s-artifact-puzzle-5d (lib) generated 3 warnings`, none of them in a 5C file: `🦀️.rs:9412`
   (5E's `window_engagements_with_request_context`), `🪟️window/🦀️.rs:275` (5E), and
   `🦀️.rs:137 function parse_example_dsl is never used` (the example path, 5A2/5D). 5C's own
   `unnecessary qualification` warning from run 1 is gone. Log:
   `🗑️generated/5C/check-native-final.txt`.

## 5. NOT verified / owed to integration

- **The `panels::` unit tests have never been executed.** They compile clean; the shared `lib test`
  target is red from sibling slices' test files (§4 item 4). An integrator should re-run
  `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- panels::` once
  5E/5G/5D's test files compile.
- **`🗄️retained-jobs/🔣️.json`'s `toolIds` is badly out of sync with `PUZZLE5D_RETAINED_TOOL_IDS`** and
  5C deliberately did not resync it. When measured the fixture listed 16 ids against a source list of 65
  that was still churning under 5A1/5A2/5B/5E/5G, and it still named `importComposeKit`, `setFixtureJson`
  and `zoomToSelection`, which the source list no longer carries. The plugin-root law
  `🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
  asserts the array matches **exactly and in order**, so it stays red until someone regenerates it after
  the verb churn stops. Regenerating it mid-churn would have been stale within minutes and would have
  overwritten peers' pending rows. **Wave-2 integration item.**
- No browser/battery verification: 5C ran no Nx, no serve, no Playwright (coordinator owns those). The
  inspector's per-entity groups, the outliner's row toggles and the settings steppers are source- and
  law-level only.
- The inspector's numeric rows are `tree_item_with_action` rows carrying a staged `value`, not
  `NumberStepper` controls — the same shape 3d's inspector has. A typed edit therefore depends on the
  host's row-edit affordance, unverified in the browser for 5d.
- `patchPart`/`patchGrip`/`patchFastener` reachability depends on 5A2. If any is still
  `BatchOnlyPendingRewrite` at integration the inspector reads correctly but that write-back is dead.
  (At last measurement all three were in the publication audit's admitted set, so they look migrated.)
- The all-owners publication audit is red on 2d; someone on the 2d side owes that.

## 6. Hand-offs

- **5A2 / 5B — label stamping, the one thing 5C could not finish.** Call
  `crate::editor::puzzle5d::puzzle5d_next_part_label(&parts, &document, part_kind)` and put the result in
  `part_3d.label` at the RETAINED creation sites 5C did not touch (they build `crate::Puzzle5dPart`, the
  schema type, not the editor twin, inside work structs those slices are rewriting):
  `Puzzle5dAddNodeWork`'s `Create` stage, `Puzzle5dAddBrushPartWork`'s `Create` stage, the
  duplicate-selection path, and the fill run-ops if 5B materialises parts there. Without it the outliner
  falls back to the flat `text` those sites stamp, which is the raw kind id — exactly the 09-15
  `PUZZLE3D-OBJECT-TREE-LABELS` defect.
- **5A2 — accept-suggestion**, if it lands, needs the same call.
- **5E — grid spacing.** The settings panel binds `setGridSpacing`; that verb and the per-window
  `grid_spacing` field are yours and 5C introduced no duplicate. One shared-config default function,
  `default_grid_spacing`, vanished from `🎚️config/🦀️.rs` during a concurrent write around 12:40 and was
  restored at its `10.0` value — confirm that is still what you intend.
- **5E — `Puzzle5dScene::interaction`.** 5C threaded it into `render_with_request_context` and fixed the
  three panel test helpers that constructed `Puzzle5dScene` without the field. Plain `render` still
  builds an empty snapshot on purpose (there is no `InteractionView` there); that is 3d's own shape.
- **5G — voxel dims** landed in `Puzzle5dRuntime`/`Puzzle5dWindowConfig` while 5C ran; the settings panel
  deliberately does not expose them (they belong to the volume-brush option group).
- **Coordinator / wave 2.** Regenerate `🗄️retained-jobs/🔣️.json`'s `toolIds` from
  `PUZZLE5D_RETAINED_TOOL_IDS` and re-derive the publication-authority 5d entry from the classification
  table once every 5-slice has landed, then re-run
  `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit`
  (must exit 0 for all three owners) and the filtered `panels::` tests.
