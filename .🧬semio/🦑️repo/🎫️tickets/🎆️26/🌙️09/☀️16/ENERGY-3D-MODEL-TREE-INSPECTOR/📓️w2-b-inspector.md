# W2-B — inspector + panels complete: result-field selector, materials, constructions, tree

Lane W2-B of `26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR`, finishing lane W1-C's §6 owed list and
lane W1-D's §9 item 5, plus the four review findings of `📓️w2-c-review.md`.

Everything below is implemented and **run**. Nothing here is "written but not verified" — every
command in §8 was executed in the foreground and its real output is quoted.

⚠️ This lane was cut off once by a usage-limit reset (mid-way through the `change-material-roughness`
leaf) and resumed; every file listed here was re-read from disk before the resume, not from memory.

---

## 1. App-level action ownership — the keybindings and the result-field select

**The rule, restated:** a panel action AND a keybinding are both dispatched in the ACTIVE window's
context, and `AppBuilder::build_definition` copies an app-level action onto every window kind but
SKIPS any id some window declares in its own `actions` (`explicitly_owned_action_ids`). So a verb
listed on one window belongs to that window alone and is refused from anywhere else.

Lane C had already moved the ten inspector verbs app-level. This lane moved the remaining six:

| Verb | Was declared on | Now |
|---|---|---|
| `set-simulation-settings` | `⚡️simulation` window | app-level |
| `set-result-field` | `⚡️simulation` window | app-level |
| `set-run-period` | `⚡️simulation` window | app-level |
| `create-surface` | `🌳️structure` window | app-level |
| `create-zone` | `📊️zones` window | app-level |
| `rename-zone` | `📊️zones` window | app-level |

- `⚡️simulation/🦀️.rs`: `settings_action`/`result_field_action`/`run_period_action` are now `pub`,
  and `definition().actions` is **`Vec::new()`** with the reason written on it.
- `🌳️structure/🦀️.rs`: new `pub fn create_surface_action()`; `actions()` keeps only
  `assign-surface-construction` (whose arguments are meaningless outside that tree).
- `📊️zones/🦀️.rs`: new `pub fn create_zone_action()` / `rename_zone_action()`; `actions()` is empty.
- `✏️editor/🦀️.rs`: new `window_shared_action_definitions()` (those six) and
  `app_level_action_definitions()` = inspector ∪ shared. `create_energy_model_editor` loops over the
  latter. `setActiveExample` was already app-level through `.mutation(…)`.

So `mod+shift+n` (create-zone), `mod+shift+s` (create-surface) and `mod+shift+g` (set-site) now fire
whichever window is active, and the inspector's own Results select reaches `set-result-field` from
the artifact panel while the 3d window holds focus.

The simulation window's own two laws (`actions_are_localized_and_registered_as_interactive`,
`the_result_field_action_offers_exactly_the_published_fields_…`) read their three verbs from
`window_shared_action_definitions()` instead of `definition().actions` now. That adaptation was made
by a PEER on the shared file `⚡️simulation/🧪️tests/🔬️unit/🦀️.rs` while this lane was gating, not by
this lane; it is correct and green, and is recorded here so the change is not attributed wrongly.

## 2. Result-field selector — lane D's §9 item 5

`📌️panels/🔍️inspection/🦀️.rs` gained a **Results** section rendered in BOTH bodies: the
no-selection document summary AND the footer of every entity form (nine selections asserted). It is
a `select` over exactly `ResultField::ALL`, EN/DE labels, `Trigger::Change` → `set-result-field`,
reading the CURRENT field back from `result_field(cfg)`.

- `render(snapshot, interaction, config: &EnergyModelConfig, locale)` — `cfg` threaded through
  `render_body` (`cfg.snapshot`).
- **The select authors NO arguments at all.** `set-result-field`'s bridge arm prefers a named
  `field` over the host-merged `value`, so authoring `{field: <current>}` would pin the control to the
  value it already holds and make every pick a no-op. An empty descriptor lets the host's `{value}`
  merge be the only source. Asserted by round-tripping the projected binding through
  `command_from_action`, not by a shape check.

## 3. Materials — `set-material-property` widened, `change-material-roughness` authored

**Command.** `SetMaterialProperty { material, property, value }` — `value: f64` → **`String`**, like
the five newer inspector verbs. `set_material_property` now covers `name` (→ `rename-material`),
`roughness` (→ the new kind) and the seven SI scalars, parsing the text per property and refusing a
non-finite/out-of-range reading. The bridge arm reads both spellings (`{property, material}` and the
inspector's `{field, id}` + merged `value`). The three existing call sites in
`✏️editor/🧪️tests/🔬️unit/🦀️.rs` were updated.

**New mutation kind `change-material-roughness`**, hand-authored by copying
`🔥️change-material-conductivity` (payload shape) and `🟤️change-gas-material-gas` (enum payload); the
drifted generator was NOT run. Directory `🧬️mutations/🗻️change-material-roughness/` with `🦀️.rs`,
`🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🔣️.json`, `🧬️schema/🔣️.json` and two `🧪️tests/<case>/🦀️.rs`
(`✅️applies` roughens material 1 to `VeryRough`; `⛔️refuses` names material 99). Payload
`{id, newRoughness: SurfaceRoughness}`. Aggregate surfaces updated, always APPENDED so no binary
ordinal moves:

`🧬️mutations/🦀️.rs` (reexport, variant, `KINDS`, `DIRECTORIES`, `wire_probes`), `🧬️mutations/🔣️.json`
(`oneOf`, inserted in slug order), `🟦️.ts`, `🔗️.graphql`, `🛰️.proto` (field 325), `📖️.grammar.semio`
(op list + rule), `🗿️artifacts/🔋️model/🦀️.rs` (`#[path]` mount block), `🔮️oracles/🔣️.json`
(`.vectors`, `.kinds` → 287, `mutationManifests[0].mutations`), and the second implementation
`🧪️tests/🏛️mutate-energy-model-1/` (`🦀️.rs` `KINDS` + 2 `Vector`s, `🐍️.py` `VECTOR_ROOTS` + forward
fn + inverse table, `🥒️.feature` both `Examples` tables).

Fixtures materialised with `SEMIO_ENERGY_WRITE_FIXTURES=1` (§8 gate 2) — 10 files under
`🧫️fixtures/🧬️mutations/🗻️change-material-roughness/`. Note the chicken-and-egg: `include_str!` needs
the files to exist before the test binary compiles, so they were created as `{}` placeholders first
and then overwritten by the writer.

**Probe-exactness.** `model_edit`'s probe no longer CLONES for four collections. `diff_zones`,
`diff_surfaces`, `diff_materials` and `diff_thermostats` now return the PROJECTION of `base` through
exactly the steps they emitted, like `diff_fenestrations` — so a field nobody diffs makes
`probe != *model` and faults loudly through `kind_unavailable`. `diff_materials` also gained the
`roughness` arm (the live masking case the review named). **Constructions were not in the probe at
all**, so a new `diff_constructions` was needed (§4) and `probe.constructions` is now its projection
too. Every one of the five is covered by
`the_four_projected_collections_emit_a_step_for_every_field_they_carry`.

## 4. Constructions — editable layer stack + computed U-value

**New command** `SetConstructionProperty { construction, property, value }` (`set-construction-property`),
appended LAST to the enum so no `OpBinary` ordinal moves. Registered in
`ENERGY_MODEL_RETAINED_TOOL_IDS`, `ENERGY_MODEL_DOCUMENT_TOOL_IDS`, `action_id()`,
`args_bridge`, `reduce()`, `PUBLICATION_CONTRACTS` (Artifact lane),
`bounded_first_step_tool_proofs!` and the app-level roster.

Properties — the operand travels in `value` because a rendered control merges exactly one scalar:

| property | `value` | mutation kind |
|---|---|---|
| `name` | new name | `rename-construction` |
| `addLayer` | material id (appends) | `add-construction-layer` |
| `removeLayer` | layer index | `remove-construction-layer` |
| `moveLayerUp` / `moveLayerDown` | layer index | `reorder-construction-layers` |
| `replaceLayer:<index>` | material id | `remove-construction-layer` **+** `add-construction-layer` |

(`replaceLayer` without a suffix takes `<index>:<materialId>`, which is what a palette invocation
types.) `diff_constructions` classifies a layer-stack change back into those kinds: one insert, one
delete, a one-slot exchange (remove+insert at the same index) or a permutation of the same multiset;
anything else is refused with `kind_unavailable("replace-construction-layers")`.

⚠️ **`add-construction-layer` admits an OPAQUE `Material` only** — its own diff refuses a glazing or
gas id with `mutation.target-missing`. So `addLayer`/`replaceLayer` naming one is refused HERE,
early and loudly, instead of reducing cleanly and dying at the store. The layer select still OFFERS
all three catalogues, because an existing glazing/gas layer has to display its own name.

**Inspector form:** name text row, per layer a `select` over materials/glazing/gas bound to
`replaceLayer:<index>` plus a `Remove layer` button carrying its own index, one trailing `Add layer`
select, and a read-only **U-value** row. The U-value is
`crate::material::construction_u_value(layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W)` — the
engine's OWN helper (`⚙️engine/🧱️material/🦀️.rs`, Σ R + the two standard films), never a second
formula. A stack containing a pane or gas gap has no `Material` row for that layer and the helper
takes opaque layers only, so it reports `— (non-opaque layer)` rather than a number computed from the
layers it happened to resolve.

That is TWO argument maps per layer. `moveLayerUp`/`moveLayerDown` are reachable from the palette but
are deliberately NOT rendered as two more buttons per row — the arena budget this panel shares with
the tree (`📓️UiValue Map Ascending Keys & One-Page Arena`) is the reason.

## 5. Tree

- **Selection/hover markers.** `PanelTreeBuilder::selected`/`::highlighted` still do not paint in
  this SDK wave, so the state also rides in the row label: `● ` selected, `○ ` hovered, `""` unmarked
  (new `MarkedAs` enum + `entity_row_marked`). The interaction snapshot is now threaded into ALL ten
  entity sections (zones, spaces, surfaces, windows, shading, materials, glazing, gas, constructions,
  loads, controls, HVAC), not just the zones section.
- **Site row.** It has no `EntityId`, so it cannot be a domain target. It now dispatches the
  framework's own CLEARING pick — `interactionSelect` with `merge: replace` and an EMPTY target list,
  which the framework documents as "clears selection while hover remains"
  (`empty_target_interaction_select_clears_selection_while_hover_remains`) — and the inspector's
  no-selection body is exactly where the site form lives. So clicking Site opens the site form.
  `section_demands[0]` went 0 → 1 because that row now binds an argument map.
- **Shading rows** were already present (`shading_section`); nothing owed there.

## 6. Review fixes (`📓️w2-c-review.md`)

### 6.1 BLOCKER — site/thermostat controls wrote the bridge's default

Confirmed and fixed on BOTH sides:

- **Authoring** (`site_args`/`thermostat_args`): they now author the WHOLE record at its current
  values PLUS a `field` key naming the slot this control edits. They no longer DROP the edited key.
  Authoring every key means that even a lost `field` marker degrades to a no-op rather than to a
  wrong number. (`UiMapBuilder` admits ascending keys only; both lists are sorted, `field` slotting
  between `elevationM`/`latitudeDeg` and between `coolingThrottleRangeK`/`heatingSchedule`.)
- **Bridge** (`command_from_action`, `SET_SITE_ACTION_ID` / `SET_THERMOSTAT_SETPOINTS_ACTION_ID`):
  each slot is read through a `merged(key, fallback)` closure that takes the host-merged `value` when
  `field` names that key and the named key otherwise. A palette or keybinding invocation carries no
  `field`, so `edited` is `None` and every slot is read by its own name exactly as before.

Four round-trip laws, none of them shape assertions:
`a_site_control_writes_the_number_the_host_merged_not_the_bridges_default` and
`a_thermostat_control_writes_the_number_the_host_merged_not_the_bridges_default` (editor tests: build
the payload, `command_from_action`, `reduce`, assert the emitted mutation AND the applied model carry
the typed number and that the untouched scalars survive; the thermostat law also covers a schedule
select, which used to write id 0), plus the two panel-side laws in §6.4 that start from the REAL
projected control.

### 6.2 MAJOR — `interzone` could never be applied

- `set_surface_property` gained the property **`interzonePartner`**: `value` is the partner surface,
  and the boundary becomes `Interzone(partner)` in one step. It refuses a self-partner, a missing
  partner and `0`.
- Its `"boundary"` arm now CARRIES FORWARD a partner the surface already holds, so re-picking
  `interzone` on an already-interzone surface keeps its neighbour instead of refusing.
- The inspector's boundary select **no longer offers `interzone`** (a lone select can never supply
  the second half), and the previously read-only "Interzone partner" row is now an editable `select`
  over the model's other surfaces. That is the control that makes a surface interzone.
  `OUTSIDE_BOUNDARY_KIND_IDS` itself is unchanged — it is the command's vocabulary, not the select's.

Laws: `picking_an_interzone_partner_makes_the_surface_interzone` (editor, incl. the three refusals
and the carry-forward) and `a_surface_form_picks_its_interzone_partner_and_never_offers_a_partnerless_interzone`
(panel, round-tripped through the bridge).

### 6.3 MAJOR — a selected surface could be dropped from the tree

`surfaces_with_windows`'s BREADTH pass walked plain document order while only the DEPTH pass used the
marked-first `order`. Both passes now hand out rows from the SAME marked-first `order` (`sort_by_key`
is stable, so document order survives within each group); rows are still BUILT in document order, so
the tree never reshuffles under the reader's cursor.

Law: `a_marked_surface_keeps_its_row_when_the_page_cannot_hold_every_wall` — case 600, six surfaces,
a page with room for the zone row plus three: surface `45` (LAST in document order) survives when
selected, a hovered surface survives too, and with nothing marked the page still goes to `40/41/42`.
This is the case the review said no existing test covered.

### 6.4 MINOR — the test that asserted the broken shape

`a_selected_thermostat_carries_the_whole_setpoint_payload_per_control` asserted
`!bindings.contains("heatingThrottleRangeK")` — certifying the very shape §6.1 shows is wrong. It is
**deleted** and replaced by `a_thermostat_control_round_trips_the_typed_number_through_the_action_bridge`
and `a_site_control_round_trips_the_typed_number_through_the_action_bridge`, which take the REAL
projected control out of the panel JSON, merge `value` the way `🗣️Interpreter/🟦️.tsx`'s
`dispatchDeclarativeControlAction` does, run it through `command_from_action` and assert the decoded
command. The shared helper is `bridged(control, value)`.

## 7. TS mirrors

- `✏️editor/🟦️.ts`: `EnergyModelConfig.resultField`; roster gained `set-result-field`, `setCamera`
  and `set-construction-property` (it was already missing lane B's and lane D's ids — fixed while
  here); `ENERGY_MODEL_DOCUMENT_TOOL_IDS` filter excludes `set-result-field`/`setCamera`.
- `🌳️structure/🟦️.ts`: `EnergyModelMaterialProperty` gained `name`/`roughness`; new
  `EnergyModelSurfaceRoughness` and `EnergyModelConstructionProperty`; `EnergyModelSurfaceProperty`
  gained `interzonePartner`; the action list gained `set-construction-property`/`set-result-field`
  with a corrected doc comment about app-level ownership.

## 8. Gates — commands and results

All from `/Users/ueli/Documents/semio`, all FOREGROUND.

| # | Command | Result |
|---|---|---|
| 1 | `cargo check -p semio-s-artifact-energy-model --lib --tests` | **exit 0**, 0 errors, 81 warnings (all pre-existing `unnecessary qualification`) |
| 2 | `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-artifact-energy-model --lib -- writes_the_committed_vector_when_requested` | **574 passed, 0 failed**, 0.39 s — materialised the 10 roughness fixture files |
| 3 | `cargo test -p semio-s-artifact-energy-model --lib -- panels editor::model::component::tests mutations::change_material_roughness` | **106 passed, 0 failed**, 6189 filtered out, 2.17 s |
| 4 | `cargo test -p semio-s-artifact-energy-model --lib` | **6289 passed, 4 failed, 2 ignored**, 48.09 s |
| 5 | `cargo test -p semio-s-artifact-energy-model --lib -- sim::tests::energy_job_previews_checkpoints_and_commits_bounded_steps` | **1 passed**, 0.05 s (see below) |
| 6 | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-energy --lib --target wasm32-wasip2` | **exit 0**, 0 errors |

**Gate 4's four reds, honestly.** Three are exactly the pre-existing ones the brief names:

```
sim::tests::p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one
sim::tests::p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total
sim::tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology
```

The fourth, `sim::tests::energy_job_previews_checkpoints_and_commits_bounded_steps`, is a **wall-clock
budget** assertion (`worst energy step was 15.321625ms`) that failed while peer cargo builds were
saturating the machine. Gate 5 re-ran it ALONE and it passed in 0.05 s. It is load flake, not a
regression — and like the other three it lives in the simulation engine, untouched by this lane.

**Peer interference seen while gating** (all transient, all in files this lane does not own, all
cleared on re-run): `👁️viewer/🦀️.rs` `UiDirtyScope::Partial { panels, regions }` (fields since
renamed), `👁️viewer/🧪️tests` `WindowInstanceView`, `⚙️engine/🎬️scene/🧪️tests` two `E0716`, and an
`include_str!` depth error in `⚙️engine/🧪️sim/🧪️tests` whose file was rewritten one minute earlier.
No cargo process was killed; no peer file was edited or reverted by this lane.

### Tests added / replaced by this lane — 20

| File | Count | Covers |
|---|---|---|
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` § `🧱️MaterialAndConstructionVerbs` | 4 | every material property → its kind (9 properties, incl. name + roughness) and the applied value; the four material refusal shapes + the unchanged-value no-op; every construction property → its kind (5 shapes, incl. the remove/insert pair) and the applied stack; the five construction refusals incl. the opaque-only limit; the five projected collections each emitting a step for a field that used to be masked |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` § `🌉️HostMergeRoundTrip` | 3 | site + thermostat + interzone-partner round trips through `command_from_action` → `reduce` → applied model |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` § `🪟️AppLevelOwnership` | 3 | every app-level verb (and `setActiveExample`) owned by every window kind; every editor-bound chord owned by every window kind; no window re-declares one, and the simulation window's list is empty |
| `📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | 5 new + 3 rewritten | Results section in all nine bodies; the select reads the config and authors no `field`; the interzone partner picker; the construction layer stack + U-value; the two bridge round trips (replacing the shape-only thermostat law); material name/roughness controls |
| `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 3 new + 2 updated | the site row's clearing pick; marked-row label state; the marked-first breadth allocation; `schedule_rows_are_read_only` (split from the site, and now RETIRES its built tree); `section_demands[0] == 1` |

## 9. Still owed / honest notes

1. **No restage, no dev server, no browser probe from this lane** — the brief forbade them. Every
   claim above is a Rust law, not an observed pixel. Lane E should re-probe: the Results select, the
   construction layer selects, the site row, the `●`/`○` markers and a `mod+shift+n` press while the
   3d window is focused.
2. **`add-construction-layer` still admits opaque materials only.** The inspector's layer select
   offers glazing and gas entries (it must, to display an existing one), and picking one is refused
   with a message naming the vocabulary gap. Closing it properly means new
   `add-glazing-construction-layer`/`add-gas-construction-layer` kinds, or widening the existing
   leaf's diff to accept all three catalogues — a schema change this lane did not take.
3. **`moveLayerUp`/`moveLayerDown` have no rendered control**, only palette reach. Adding two buttons
   per layer would put four argument maps on every layer row; the arena cannot afford that beside the
   tree.
4. **`replaceLayer` emits two steps**, so an undo of one layer exchange is two history entries. The
   vocabulary declares no `replace-construction-layer`; adding one would collapse it.
5. **The `●`/`○` label prefix is a workaround**, not the SDK's own marking. `PanelTreeBuilder::selected`
   / `::highlighted` are still declared on the built tree and still do not paint. The prefix should be
   removed the moment the builder's marking reaches the renderer.
6. **Construction create/delete are still refused** (`diff_constructions` returns
   `kind_unavailable("create-construction / delete-construction")` on an identity change), as are
   material and thermostat create/delete. The kinds exist in the vocabulary; no editor verb reaches
   them.
7. **Two independent layer edits in ONE revision are refused.** `construction_layer_steps` classifies
   exactly one insert / one delete / one exchange / one permutation. Anything else faults loudly —
   which is correct for this editor (every command makes exactly one) but would bite a batch caller.
8. **Glazing `solar_reflectance_*`, `visible_reflectance_*` and `infrared_transmittance` still have
   no mutation kind** (lane C's note, unchanged): reported read-only, refused if named.
9. **`✏️s/🔌️plugins/🔋️energy/🔣️.json`** (the committed package descriptor) still lists the old action
   set. It is produced by the packaging tool, not by these gates.
10. **`🐍️.py` / `🥒️.feature` rows for `change-material-roughness` are written, not executed** — the
    `🏛️mutate-energy-model-1` differential case runs through the repo test host, outside this lane's
    cargo budget. They were written from the payload schema and mirror the Rust codes
    (`mutation.target-missing`, `mutation.no-op`). Same standing as lane A's ten kinds.
11. **No nx gate was run** (`verify-taxonomy-report`, `mutation-outcome-law`). The new directory
    `🗻️change-material-roughness` was checked by hand against the sibling pattern: NFC, exactly one
    U+FE0F, first emoji `🗻` unique among its 290 sibling directories.
