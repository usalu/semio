# 📓️ A5 — 🔋️energy (`semio-s-artifact-energy-model`)

Packet A5 of wave 2. Crate `semio-s-artifact-energy-model`, no extra feature (📓️audit-paging-tests.md §8).
Editor root: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`.

## 1. Status

Done. Both panels are windowed, all app-local paging is gone, both test files are rewritten/adjusted, and the two
required commands were run in the foreground.

| Command | Result |
|---|---|
| `cargo test -p semio-s-artifact-energy-model` | **6289 passed, 3 failed, 2 ignored** — the 3 failures are `sim::tests::p7c*` (see §6), untouched by this packet |
| `cargo test -p semio-s-artifact-energy-model -- editor::model::panels` | **36 passed, 0 failed** (20 artifact + 16 inspection) |
| `cargo check -p semio-s-artifact-energy-model --target wasm32-wasip2` | **exit 0**; 1 pre-existing warning, in a non-packet module (§6) |

Output captured under `🗑️generated/a5-energy/{full-test.txt,panels-test.txt,wasm-check.txt}`.

## 2. Files changed

| Path (absolute) | What |
|---|---|
| `…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` | rewritten: 11 windowed sections, 3-level `tree_window_item` nesting, domain-bound pick rows |
| `…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | rewritten: paging laws → window laws |
| `…/✏️editor/📌️panels/🔍️inspection/🦀️.rs` | windowed layer stack + windowed multi-selection header; `LIST_ROWS_MAX` gone |
| `…/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | `render(…, &TreeWindows::unhosted())` |
| `…/✏️editor/🦀️.rs` | `render_body` builds `TreeWindows::for_body(view_state, BODY_KEY)` per panel body; crate-local `ui_node_list` deleted |

## 3. Artifact panel (`📌️panels/🗿️artifact`, body `energy.model.artifact`)

**Sections.** All eleven (`site`, `zones`, `shading`, `materials`, `glazing-materials`, `gas-materials`,
`constructions`, `loads`, `controls`, `hvac`, `schedules`) are now `PanelTreeBuilder::window_section` /
`window_section_or_placeholder`. `default_open` semantics unchanged (`site`/`zones` open, the rest closed).
The audit called this "nine sections"; the file declared `SECTIONS = 11` and eleven `.section*` calls — all eleven
migrated.

**Nesting.** zone › {spaces, surfaces} › windows is now two levels of `tree_window_item`:

- a zone's children are ONE logical list (`enum ZoneChild { Space, Surface }`, spaces then surfaces) so the zone
  container stamps a single honest `total = spaces + surfaces`;
- each surface is itself a `tree_window_item` over its own fenestrations.

This **removes the hand-paged `surfaces_with_windows` starvation workaround** (old L269-354: the marked-first
breadth/depth allowance passes and the `…surfaces`/`…windows.more` markers). The defect it existed for cannot recur:
a container never truncates, it reports its extent and the host asks for the slice it shows.

**Rows.** `entity_row`/`entity_row_marked` lost `pick_action` entirely. A row is now built by `entity_item(…)` →
`ui::tree_item(label).icon().description().dimmed().granularity(<ENERGY_GRANULARITY_*>)` keyed by the raw
`EntityId`, with **no binding and no argument map**. The tree carries the single
`.interaction_domain(ENERGY_MODEL_EDITOR_CONTROLLER_ID, ENERGY_MODEL_INTERACTION_DOMAIN)` binding.

Kept as-is per the packet brief: the `●`/`○` `MarkedAs` label-prefix workaround; the site row's own
`clear_selection_action`; schedule/humidistat/air-loop/plant-loop rows read-only (now expressed as
`RowEntry.granularity == None`).

**Symbols deleted:** `pick_action`, `site_section`, `zones_section`, `surfaces_with_windows`, `is_marked`,
`shading_section`, `materials_section`, `glazing_materials_section`, `gas_materials_section`,
`constructions_section`, `loads_section`, `controls_section`, `hvac_section`, `schedules_section`,
`section_demands`, `section_quotas`, `with_quota`, `SECTIONS`, `entity_row_marked`, `icon_text`, and the imports of
`paged_panel_section`, `panel_page_rows`, `PanelRowBudget`, `tree_item_with_action`, `UiFixedList`, plus the
`semio_framework_plugin::panel_continuation_row` call site.

**Symbols added:** `entity_item`, `entity_row`, `RowEntry`/`entry_row`, `ZoneChild`, `zone_children`, `zone_row`,
`surface_row`, `site_row`, `load_entries`/`control_entries`/`hvac_entries`/`schedule_entries`, `ui_text`.

## 4. Inspection panel (`📌️panels/🔍️inspection`, body `energy.model.inspection`)

- The `.take(LIST_ROWS_MAX)` construction-layer loop plus its static `read_only_row("More layers")` is replaced by a
  **windowed section** `energy-model-inspection.construction.layers`: `enum LayerRow { Pick, Remove }`, two rows per
  layer over one flat list, so the section stamps the stack's whole extent. The `Add layer` select and the U-value
  row stay in the fixed `…construction` form section. Row ids are unchanged
  (`…construction.layer.{i}.select`, `…construction.layer.{i}.remove.button`), so the existing inspector tests still
  pin them.
- The multi-selection header (`…selection`) was the other `.take(LIST_ROWS_MAX)`; it is now a `window_section` over
  `interaction.selected_ids`, keyed by the selected id itself (index keys would have broken windowing).
- `const LIST_ROWS_MAX` and `fn selection_rows` deleted. `SELECT_ITEMS_MAX` kept: it bounds a `select` control's
  option list against `UiFixedList`, not tree rows.
- Dead `fn surface_options` removed (it had no caller before this packet either; it was the crate's only
  `dead_code` warning).
- `render(…)` gained `windows: &TreeWindows<'_>`.

## 5. Tests — old paging laws → new window laws

| Old (deleted) | New |
|---|---|
| `section_quotas_are_max_min_fair_and_never_exceed_the_page` | — (the quota splitter no longer exists) |
| `an_oversized_document_pages_with_continuation_rows` (asserted `…materials.more`) | `an_oversized_document_stamps_every_containers_total_and_materialises_only_its_window` — 412 materials, host asks for 10: section stamps `window {total: 412, offset: 0}`, builds exactly entries `[0,10)` by raw `EntityId`; the nested zone stamps its own total; body JSON contains no `.more` and no `"+` label |
| `section_demands_count_every_interactive_row_including_the_nested_ones` | — |
| `a_wide_selection_marks_only_its_first_page` | `a_wide_selection_still_assembles_the_whole_tree` (keeps the `MARKED_IDS_LIMIT` cap assertion, drops the page one) |
| `a_marked_surface_keeps_its_windows_under_a_tight_page` / `a_marked_surface_keeps_its_row_when_the_page_cannot_hold_every_wall` | `a_wide_zone_keeps_every_surface_reachable_and_no_window_is_ever_collapsed` — 126-surface zone, scrolled to the tail (`offset 122, rows 4`) still reports `total 126`; a wall reached through its own window keeps its opening |
| — | `a_closed_container_stamps_its_total_and_materialises_no_child` — `TreeWindowRequest{open: Some(false)}` on the zone, plus an author-default-closed section |
| — | `a_tree_window_request_materialises_exactly_its_slice_of_a_surface_group` — `{offset: 1, rows: 1}` on surface `40` → children exactly `["51"]`, window `{2, 1}` |
| `every_entity_row_picks_into_the_energy_model_domain` (kept, rewritten) | root carries **exactly one** `interactionSelect`; zone/surface/window rows carry `component.granularity` and an **empty** `bindings` |
| `windows_nest_under_their_surface_and_surfaces_under_their_zone` (kept) | now read off the rendered tree instead of a budgeted section call |

Also kept and adapted to the whole-tree render (the per-section builders they used are gone):
`the_catalogue_sections_key_their_rows_by_entity_id`, `a_shading_surface_is_listed_when_the_model_has_one`,
`schedule_rows_are_read_only`, `an_empty_model_renders_placeholders_rather_than_refusing`,
`a_dangling_construction_reference_dims_its_surface_row`, `german_resolves_every_section_heading`,
`the_surface_rows_stay_in_document_order_whatever_is_marked`, `the_site_row_opens_the_site_form_by_clearing_the_selection`,
`a_marked_row_carries_its_state_in_the_label`, `every_section_is_assembled_over_the_document`,
`every_zone_surface_and_window_is_a_row_keyed_by_its_entity_id`, `the_panel_tab_declares_…`.

Fixtures are built the brief's way: `ViewModel { tree_windows: vec![TreeWindowRequest{…}], ..Default::default() }`
→ `TreeWindows::for_body(&view, BODY_KEY)`; the no-host-state path uses `TreeWindows::unhosted()`.

## 6. Notes for the coordinator

1. **`📓️p1-contract.md` and `📓️p3-sdk.md` never appeared.** I polled the ticket folder past the brief's 45-minute
   cap (three loops) and they are still absent. The wave-1 *code* is fully landed, so I wrote the panels against
   §5 and then read the final signatures straight from
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (region `🔖️PanelWindowing`) and
   `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`. **Three signatures differ from design §5** — later app packets should
   know:
   - `PanelTreeBuilder::window_section` / `window_section_or_placeholder` and the free
     `tree_window_section*` / `tree_window_item` take **`id: &str`**, not `impl AsRef<str>` — an `id` built with
     `format!` must be passed as `&format!(…)`.
   - `PanelTreeBuilder::interaction_domain(controller_id: &'static str, domain)` — the controller id must be a
     `&'static str` (a `const`, not a `String`).
   - `tree_window_section_or_placeholder` on an **empty** container always materialises the placeholder row
     (whatever the open state) and **stamps no `window` at all**. A test asserting `window.total == 0` on an empty
     section will fail; assert `component.window` is absent instead.
2. **3 unrelated failures in the full-crate run**, identical before and after this packet and in no way panel code:
   `sim::tests::p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one`,
   `sim::tests::p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total`,
   `sim::tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology`. They live in
   `✏️s/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/` and fault on the `RetainedJobPayload` "one-page close to
   terminal-empty" law in `🧰️framework/🔨️modules/🧵️job/🦀️.rs` (last touched 2026-09-16 04:42, before this
   ticket's work). Flagged, not fixed — out of packet scope.
3. **One remaining warning in the crate**, pre-existing and outside this packet's files: `unnecessary qualification`
   at `…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs:40`. The two panel files are warning-free.
4. **Nothing else to retire for energy**: a repo-wide grep over `✏️s/🔌️plugins/🔋️energy` finds zero
   `setPanelPage`, `panel_pages`, `SECTION_ROWS`, `PANEL_RECONCILE_NODE_BUDGET`, `paged_panel_section`,
   `panel_continuation_row`, `panel_page_rows`, `PanelRowBudget` or `.more` hits. No plugin manifest
   (`✏️s/🔌️plugins/🔋️energy/🔣️.json`) regeneration was needed — no action catalogue changed.
5. **Out of scope, still open**: the `●`/`○` selection-prefix workaround stays until `PanelTreeBuilder::selected`/
   `::highlighted` paint; the ticket-scoped probe
   `…/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR/🐍️energy-panels-probe.mjs` still asserts `/\.more$/i` rows and will need
   rewriting for wave-3 browser verification.
