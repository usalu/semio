# W1-C — artifact tree panel, inspector panel, and the commands they need

Lane W1-C of `26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR`. Everything below is in the energy editor
subset `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

## 1. The diff gap (`✏️editor/🦀️.rs`)

`model_edit`'s probe used to do `probe.fenestrations = model.fenestrations.clone()` and there was no
`diff_fenestrations` at all, so a window field edit reduced cleanly and emitted **nothing** — neither
a mutation nor a `kind-unavailable` fault. Closed:

- **`diff_fenestrations(base, model, steps) -> Vec<Fenestration>`** (beside `diff_surfaces`). It
  covers every field of the record — `name` → `rename-fenestration`, `surface_id` →
  `change-fenestration-surface`, the thirteen scalars → their own `change-fenestration-*` kinds,
  `vertices_m` → `replace-fenestration-vertices` (lane A's kind, picked up as soon as it landed), and
  `glazing_construction_id` → `bind-`/`clear-fenestration-glazing-construction`. A window present in
  `model` but not in `base` emits `create-fenestration` plus, when it has real corners, its own
  `replace-fenestration-vertices` (the create kind carries no polygon). Deletion is still cascaded by
  `diff_surfaces` — unchanged.
- It **returns the projection of `base` through exactly the steps it emitted**, and `model_edit` now
  assigns that to `probe.fenestrations`. A Fenestration field nobody names makes `probe != *model` and
  faults loudly through `kind_unavailable` — the masking is gone by construction, not by promise.
- Same probe-exact treatment for the two catalogues lane A unlocked: **`diff_glazing_materials`** and
  **`diff_gas_materials`**, whose projections feed `probe.glazing_materials`/`probe.gas_materials`
  (which the probe previously did not set at all). Glazing covers `name`/`thickness`/`conductivity`/
  `solar_transmittance`/`visible_transmittance`/`infrared_emissivity_{front,back}`; the four
  reflectances and `infrared_transmittance` still have **no mutation kind**, so editing one now
  faults loudly instead of vanishing.

Audit of the other diffs:

| Collection | Verdict |
|---|---|
| `diff_zones` | already complete — all five `Zone` scalars (name/volume/multiplier/conditioned/part_of_total_floor_area) |
| `diff_surfaces` | already complete — all nine `Surface` fields including `class` and `outside_boundary_condition` |
| `diff_materials` | was missing `name`; **added `rename_material`**. `roughness` has no mutation kind and `probe.materials` still clones, so a roughness edit is still masked — see §7 |
| `diff_thermostats` | was missing `zone_id`; **added `change_thermostat_zone`** |
| site | **no `diff_site` needed** — `model_edit` already emits `update_site` inline on `base.site != model.site`, and `SetSite` goes through it |

## 2. New editor commands (`✏️editor/🦀️.rs`)

Five new variants on `EnergyModelEditorCommand`, **appended** (never inserted) so no existing
`OpBinary` ordinal moves:

| Action id | Payload | Properties |
|---|---|---|
| `set-surface-property` | `{surface: u32, property: String, value: String, partner_surface: u32}` | name, class, boundary, construction, sunExposed, windExposed, multiplier |
| `set-fenestration-property` | `{fenestration, property, value: String}` | name, uValueWM2K, shgc, vlt, areaM2, heightM, sillHeightM, frameConductanceWK, dividerConductanceWK, overhangDepthM, overhangOffsetM, finDepthM, finOffsetM, glazingConstruction (empty value = clear) |
| `set-zone-property` | `{zone, property, value: String}` | name, volumeM3, multiplier, conditioned, partOfTotalFloorArea |
| `set-glazing-material-property` | `{material, property, value: String}` | name, thicknessM, conductivityWMK, solarTransmittance, visibleTransmittance, infraredEmissivityFront, infraredEmissivityBack |
| `set-gas-material-property` | `{material, property, value: String}` | name, thicknessM, gas |

`value` is TEXT for all five because one verb has to carry a name, an enum spelling, a flag and a
scalar alike — which is also the shape a rendered control's own value arrives in. `partner_surface`
is the `Interzone` boundary's other half, `0` meaning none (`OutsideBoundary::from_parts` refuses an
interzone without a partner and a partner offered to any other arm).

Wired into every roster the framework demands set-equality over:
`ENERGY_MODEL_RETAINED_TOOL_IDS`, `ENERGY_MODEL_DOCUMENT_TOOL_IDS`, `action_id()`,
`args_bridge::command_from_action`, `reduce()`, `PUBLICATION_CONTRACTS` (all five `Artifact` lane),
`bounded_first_step_tool_proofs!`'s `tools:` list, and — see §6.1 — the APP-level action list, which
`build_definition` copies onto every window kind. `create_energy_model_editor` panics on an
unclassified id and `retained_roster_is_exact_and_exhaustive` asserts every retained id reaches a
window's action list.

`args_bridge` reads **both** spellings of the payload: the palette's `{property, <entity>}` and the
inspector's `{field, id}` + host-merged `value`.

## 3. `📌️panels/🗿️artifact/🦀️.rs` — the outliner

`BODY_KEY = "energy.model.artifact"`, `TREE_NAMESPACE = "energy-model-artifact"`, 11 sections:
site · zones (each zone nests its spaces and surfaces, each surface nests its windows) · shading
surfaces · materials · glazing materials · gas gaps · constructions · internal loads
(people/lighting/equipment/infiltration) · controls (thermostats + humidistats) · HVAC (ideal loads +
air loops + plant loops) · schedules.

- Rows keyed by `energy_target_id(id)` (raw `EntityId`), bound to `ENERGY_MODEL_INTERACTION_DOMAIN`
  with the matching `ENERGY_GRANULARITY_*`, picking through the framework-reserved
  `interactionSelect` — the same effect a 3d viewport pick dispatches.
- **One action per row** (its pick) and no row verbs — the argument-arena law; delete lives in the
  inspector.
- Human labels: `"Wall South · exteriorWall"`, `"South Window West · 6.00 m²"`, `"Zone · 129.60 m³"`.
- `paged_panel_section` + `PanelRowBudget` + fem2d's max-min-fair `section_quotas` (ported, generalised
  to 11 sections), truncated sections close with `+N`.
- Selection/hover come from `EnergyModelInteractionSnapshot` via `.selected()`/`.highlighted()`
  (capped at `UI_FIXED_LIST_ITEMS`; note the SDK's own caveat that those do not paint onto the built
  tree yet).
- **Read-only rows** for the families `energy_entity_kind` does not resolve — humidistats, air loops,
  plant loops, schedules (schedules key off `ScheduleId`, a *separate* id space that would collide
  with `EntityId` on the wire) — and for the site, which is a singleton with no `EntityId`. They bind
  nothing, so they cost no arena credit.

## 4. `📌️panels/🔍️inspection/🦀️.rs` — the inspector

`BODY_KEY = "energy.model.inspection"`, `ROOT = "energy-model-inspection"`. Body is
`ui::column` of `ui::section`s of `ui::field(label) > control` — **never** `PanelTreeBuilder`
(§2.1 trap); three tests assert `!carries_a_tree(json)`.

Forms per resolved kind: surface (name text, class select, boundary select + read-only interzone
partner, construction select over the model's constructions, sun/wind toggles, multiplier stepper,
read-only area/tilt/azimuth from `geometry::surface_area_m2`/`surface_tilt_azimuth`), fenestration
(all fifteen fields + glazing-construction select with an explicit clearing option), zone (five
fields), material (7 scalars editable; name + roughness read-only, see §7), glazing material and gas
gap (editable now that lane A's kinds landed; the five fields with no kind stay read-only),
construction (name + layer list, read-only), thermostat, shading surface (read-only), and the site
(inside the no-selection summary, since it has no `EntityId` and no other home).

Every control binds `Trigger::Change` through a ported `bind()` helper with the flat `{field, id}`
map — the host merges `value`. Two exceptions carry their whole payload minus the edited key, because
their verbs replace several fields at once and the host merges only one value: `set-thermostat-setpoints`
and `set-site`. Multi-selection header, document summary fallback, and one grouped Delete action row
for zone/surface (`delete-zone`/`delete-surface`).

## 5. Manifest, render, mounts

- `create_energy_model_editor`: `.panel_tab_def(artifact_panel::definition())` +
  `.panel_tab_def(inspection_panel::definition())`.
- `render_body`: two new arms threading the interaction snapshot and `view_state.locale`. Lane B had
  already split `render`/`render_with_request_context` and renamed `_interaction` → `interaction`, so
  nothing there needed changing.
- Crate root `🗿️artifacts/🔋️model/🦀️.rs`: `editor::model::panels::{artifact, inspection}`, mirroring
  fem2d's mount shape.
- `📦️packages/🦀️rust/Cargo.toml`: added `semio-framework-ui-contract` (the panel bodies need the
  builder *functions*, not just the types `semio-framework-plugin` re-exports).
- TS mirrors kept in step: `✏️editor/🟦️.ts` (roster + both body keys; `ENERGY_MODEL_DOCUMENT_TOOL_IDS`
  changed from `slice(0, 12)` to a filter, since the document ids are no longer a prefix) and
  `🌳️structure/🟦️.ts` (five new action rows + the five property union types).

## 6. Follow-up round (coordinator's browser-probe defects)

### 6.1 The shell refused every inspector command

`input #11 set-fenestration-property refused: dispatch-failed (user window=energy.model.3d)` /
`window kind energy.model.3d does not own action set-fenestration-property`.

**Root cause, confirmed in the framework:** a panel action is dispatched in the ACTIVE window's
context, and `AppBuilder::build_definition`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5324-5348`) copies every APP-level action onto
every window kind — but skips any id in `explicitly_owned_action_ids`, i.e. any id some window kind
declares in its own `actions`/`action_refs`. So the moment a verb is listed in `structure::actions()`
it belongs to the structure window ALONE.

**Fix:** new `crate::editor::model::inspector_action_definitions()` holds the ten verbs the inspector
can dispatch (`set-surface-property`, `set-fenestration-property`, `set-zone-property`,
`set-glazing-material-property`, `set-gas-material-property`, `set-material-property`,
`set-thermostat-setpoints`, `set-site`, `delete-zone`, `delete-surface`), and
`create_energy_model_editor` declares each with `builder.action_with(...)` — app-level. They were
REMOVED from `structure::actions()` (which now keeps only `create-surface` and
`assign-surface-construction`) and from `zones::actions()` (which keeps `create-zone`/`rename-zone`).
Lane B's `energy.model.3d` window needed no edit at all: it inherits them like every other window.

Three new laws in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`🪟️ActionOwnership`):
`every_inspector_verb_is_owned_by_every_window_kind`,
`no_window_kind_declares_an_inspector_verb_itself`,
`every_inspector_verb_stays_a_classified_retained_tool`.

### 6.2 A selected wall's windows collapsed into `…windows.more`

**Root cause:** `paged_panel_section` reserves exactly one row per remaining sibling and hands the
whole rest to the current one, so under a tight page the FIRST surface consumed the slack and the
later ones — or, with a small page, that first surface's own windows — were lost behind a `+N` marker
no tree row can expand. The page itself shrinks whenever the arena is loaded, which is exactly what
the inspector's own form of bound controls does.

**Fix:** `surfaces_with_windows` replaces that helper at the surface level with a two-phase
allocation — **breadth first** (one row per surface, document order, so a truncation never hides a
whole wall behind the windows of the wall before it), then **depth** (the leftover buys windows, the
MARKED surface's first — marked = selected, hovered, or owning a marked window). Rows are still BUILT
in document order, so the tree never reshuffles under the reader's cursor; only who survives a
truncation changes. `zones_section` now takes the interaction snapshot for this.

Laws: `a_marked_surface_keeps_its_windows_under_a_tight_page` (case 620, one window per side, a page
with room for exactly one: the marked wall keeps its own and the unmarked one gives way; with nothing
marked, document order decides) and `the_surface_rows_stay_in_document_order_whatever_is_marked`.

### 6.3 Test-harness defect found while fixing the above

The first cut of the panel tests built rows against an explicit `PanelRowBudget` and DROPPED them.
A built argument map dropped without retirement never returns its `UiValue` arena credit, and since
all 6 200-odd tests share one process that starved the panels other tests were assembling — it showed
up as `ui.value.map.entry` refusals in the inspector's glazing form. Every panel test now projects
AND retires through `project_and_retire_fixture_tree` and asserts over the projected JSON. Any law
about which rows survive uses an explicit budget, never `panel_page_rows()`, which reads the
process-wide arena.

## 7. Still owed / honest notes

- `set-material-property` still takes `value: f64`, so a material's **name** and **roughness** are not
  settable from the inspector (roughness has no mutation kind at all; `rename-material` now exists in
  the diff but no command reaches it). Widening that command to `value: String` would change an
  existing wire shape and three call sites in the shared editor test file — left alone deliberately.
- `probe.materials`/`probe.thermostats`/`probe.zones`/`probe.surfaces` still clone verbatim in
  `model_edit`, so an undiffed field in those four is still masked. Only fenestrations, glazing
  materials and gas materials are probe-exact. Making the rest exact is mechanical but touches code
  other lanes are editing.
- Glazing `solar_reflectance_{front,back}`, `visible_reflectance_{front,back}` and
  `infrared_transmittance` have no mutation kind — the inspector reports them and
  `set-glazing-material-property` refuses them.
- `PanelTreeBuilder::selected()/highlighted()` do not reach the built tree in this SDK wave; tree-row
  selection highlighting is future wiring, not something this lane can prove.
- `✏️s/🔌️plugins/🔋️energy/🔣️.json` (the committed package descriptor) still lists the old action set;
  it is generated by the packaging tool, not by these gates, and was left untouched.
- No spaces exist in the BESTEST examples, so the zone→space nesting is exercised only by the demand
  table, not by a fixture with real spaces.
- `create-surface`, `create-zone`, `rename-zone`, `set-run-period` and `set-simulation-settings` are
  still declared on one window each, so the keybindings that reach them (`mod+shift+n`,
  `mod+shift+s`) are refused while another window is active. Pre-existing, same root cause as §6.1,
  out of this lane's scope — worth one follow-up.
- Only the four gates below were run. No restage, no dev server, no browser probe from this lane.

## 8. Gates — commands and results

All from `/Users/ueli/Documents/semio`.

| # | Command | Result |
|---|---|---|
| 1 | `cargo check -p semio-s-artifact-energy-model --lib` | green, 0 errors |
| 2 | `cargo check -p semio-s-artifact-energy-model --lib --tests` | green, 0 errors |
| 3 | `cargo test -p semio-s-artifact-energy-model --lib -- panels editor::model::component::tests interaction` | **70 passed, 0 failed**, 6157 filtered out, 4.03 s |
| 4 | `cargo test -p semio-s-artifact-energy-model --lib` | **6222 passed, 3 failed, 2 ignored**, 28.09 s |
| 5 | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-energy --lib --target wasm32-wasip2` | green, 0 errors |

Gate 4's three reds are EXACTLY the pre-existing ones named in this lane's brief, all in the
simulation engine and untouched by this lane:

```
sim::tests::p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one
sim::tests::p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total
sim::tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology
```

An earlier run of gate 4 also failed `setCamera` (lane B) and `set-result-field` (lane D) in the
SHARED editor/simulation-window test files; both were green again by the final run, so those lanes
landed their fixes in between.

### Tests added by this lane — 33

| File | Count | Covers |
|---|---|---|
| `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 17 | panel-tab route, all eleven sections, entity-id keying of zones/surfaces/windows and of every catalogue, the single pick action per row, zone→surface→window nesting, shading, read-only schedule/site rows, empty-model placeholders, max-min-fair `section_quotas`, oversized-document continuation rows, dangling-reference dimming, German headings, the marked-ids cap, the demand table, and the two paging laws of §6.2 |
| `📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | 13 | panel-tab route, one form law per entity kind, the `input`-inside-`field`-row law with its binding payload, the `!carries_a_tree` law (three tests), the whole-payload thermostat binding, the editable site in the summary, unresolvable-id fallback, multi-selection header, German labels |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` § `🔍️InspectorVerbs` | 13 | the `diff_fenestrations` regression (exactly one `change-fenestration-u-value`), every fenestration/surface/zone/glazing/gas property round-tripping to its own kind, the interzone partner, the refusal triple per command, unchanged-value no-ops, the `{field, id, value}` bridge, and the material-rename / thermostat-zone diff additions |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` § `🪟️ActionOwnership` | 3 | §6.1's three laws |
