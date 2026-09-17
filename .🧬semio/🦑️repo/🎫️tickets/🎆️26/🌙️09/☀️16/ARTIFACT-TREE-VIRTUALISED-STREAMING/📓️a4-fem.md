# 📓️ A4 — 🏗️fem 2d + 3d migrated to windowed panel trees

Crates: `semio-s-artifact-fem-2d`, `semio-s-artifact-fem-3d` (both `--features component-app-assembly`).
Roots: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/`.

## 1. Panels migrated

| file (relative to each artifact's `✏️editor/`) | what changed |
|---|---|
| `📌️panels/🗿️artifact/🦀️.rs` | all nine sections → `PanelTreeBuilder::window_section_or_placeholder` (analysis → `window_section`); load-case rows and combination rows nest through `tree_window_item`; rows lost their per-row `interactionSelect` and gained `.granularity(...)`; tree binds `.interaction_domain(FEM{2,3}D_PLAY_CONTROLLER_ID, FEM{2,3}D_INTERACTION_DOMAIN)` |
| `📌️panels/🔍️inspection/🦀️.rs` | the three unbounded listings (a case's loads, a combination's terms, a wide selection's ids) → `window_section`; `paged_rows`/`LIST_ROWS_MAX` deleted |
| `📌️panels/📊️results/🦀️.rs` | untouched except the `ui_node_list` import, now the SDK's |
| `🦀️.rs` (editor) | `render_body` threads `TreeWindows::for_body(view_state, <panel>::BODY_KEY)` into both panels; the crate-local `ui_node_list` copy deleted |
| `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | paging laws replaced by window laws (below) |
| `📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | `render(...)` call sites take `&TreeWindows::unhosted()` |

The inspection body was already a `Component::Tree` built through `PanelTreeBuilder` (the audit's
"raw `ui::section`/`field`" note is stale), so the windowed listings are ordinary `window_section`s of
`treeItem` rows with their controls as row children — the generation3d control-placement trap does not
apply and no extra nesting was needed.

### Shape decisions worth knowing
- **Combination terms are open by default now.** A closed container materialises zero children under
  the window law, so `tree_window_item(..., default_open: true, ...)` keeps the nesting visible on
  first paint, matching what load cases already did.
- **The inspector's add-term / remove-term selects moved out of the `Terms` section** into the
  combination's own fields section. `Terms` is a window over the terms *alone*; a control that is not
  a term cannot sit inside it without claiming a slot in that window's extent.
- **Selection rows are keyed by the id, not the index** (`{ROOT}.selection.{id}`): an index key is not
  stable across a window offset.
- **Load rows nested under a case row carry `FEM{2,3}D_GRANULARITY_LOAD`**; combination term rows stay
  read-only `tree_item_desc` (a term is not a domain target).

## 2. Symbols deleted

Per artifact (both 2d and 3d, identical lists):

- artifact panel: `pick_action`, `ui_value_map`, `ui_value_text`, `nodes_section`, `elements_section`,
  `solids_section`/`regions_section`, `supports_section`, `load_cases_section`, `combinations_section`,
  `materials_section`, `sections_section`, `analysis_section`, `section_demands`, `section_quotas`,
  `with_quota`, and the `paged_panel_section` / `panel_page_rows` / `PanelRowBudget` /
  `tree_item_with_action` / `INTERACTION_SELECT_ACTION_ID` / `UiValue` / `ActionId` / `UiFixedList`
  imports. New: `entity_item` (returns a `TreeItemBuilder` so a nesting row can be handed to
  `tree_window_item`), `granularity_text`.
- inspection panel: `paged_rows`, `LIST_ROWS_MAX`, `selection_rows`, 2d's dead `text_node`, and the
  `panel_page_rows` / `PanelRowBudget` imports.
- editor module: the crate-local `pub fn ui_node_list` in both `✏️editor/🦀️.rs` (the SDK's
  `semio_framework_plugin::ui_node_list` is imported by the results panels instead).

Nothing else was in scope: fem has **no** `setPanelPage` command dir, no `panel_pages` state, no
manifest entry, no TS allowlist and no tree memo cache — confirmed by the audit and re-grepped here
(zero hits for `setPanelPage|set_panel_page|panel_pages|PANEL_RECONCILE_NODE_BUDGET|
artifact_tree_cached_from|CATALOGUE_GROUP_ROWS|SECTION_ROWS|IDS_ROWS` under `✏️s/🔌️plugins/🏗️fem`).
No generated manifest (`✏️s/🔌️plugins/🏗️fem/🔣️.json`) needed regeneration.

### Two `.take(N)` calls deliberately kept
- `MARKED_IDS_LIMIT = UI_FIXED_LIST_ITEMS` in `marked_ids()` — this is not paging: `PanelTreeBuilder::
  selected()/highlighted()` admit into a `UiFixedList<UiText>` (32), so an unclamped 80-id selection
  would *fault the render*. The law `selected_and_hovered_ids_are_marked_from_the_interaction_snapshot`
  still pins it.
- `SELECT_ITEMS_MAX = 24` in the inspector's `select_row()` — a `<select>`'s options are a
  `UiFixedList`, not a tree container, and nothing in the contract windows them.
- (`RESULT_SOURCE_OPTIONS` in the results panel is the same shape and the results panel is a form,
  out of packet scope.)

## 3. Tests

`📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — 12 laws per artifact. Replaced
`section_quotas_are_max_min_fair` and `an_oversized_document_pages_with_continuation_rows_instead_of_faulting`
(and the `omitted()` / `placed()` helpers that parsed `"{section}.more"` / `"+N"`) with the brief's
four window laws; kept everything else, updated in place.

| law | what it pins |
|---|---|
| `an_oversized_document_stamps_every_extent_and_materialises_one_viewport` | (a) 60-node / 40-load document, `TreeWindows::unhosted()`: every container stamps `window.total == entries.len()`, materialises ≤ its slice and ≤ `UI_BUILT_CHILDREN_MAX`, the whole first paint stays inside one `UI_DOCUMENT_NODES`, and the body JSON contains neither `.more` nor a `"+` label |
| `a_closed_container_stamps_its_extent_and_builds_no_child` | (b) `TreeWindowRequest{open: Some(false)}` on the nodes section and on the `wind` case row: extent stamped, zero children, no first-paint budget spent |
| `a_window_request_materialises_exactly_its_own_range` | (c) `{offset: 20, rows: 8}` on the nodes section → exactly `g20..g28`; `{offset: 5, rows: 4}` on the nested `wind` row → exactly `wl5..wl9`; both `TreeWindow { total, offset }` stamps asserted exactly |
| `rows_declare_their_granularity_while_the_tree_binds_the_one_interaction_select` | (d) a pick row has **no** binding of its own, carries `granularity`, carries no row actions; the tree root carries exactly one `Trigger::Activate` `interactionSelect` under the fem controller, and the projected body names it exactly once |
| `demo_document_lists_every_section_with_its_own_count` | headers name their count AND each section stamps that count as its extent |
| `load_case_rows_nest_their_loads_and_combination_rows_nest_their_terms` | nesting preserved, plus each nesting row's own extent |
| `an_empty_document_renders_placeholders` | "(none)" placeholder per empty section, extent 0 (an empty container publishes no window) |
| `selected_and_hovered_ids_are_marked_from_the_interaction_snapshot`, `rows_are_keyed_by_the_raw_entity_id_and_bound_to_the_fem{2,3}d_domain`, `rows_carry_the_human_label_of_their_entity`, `scalars_and_overlong_labels_stay_inside_the_ui_text_envelope`, `german_labels_resolve_across_the_whole_tree`, `the_app_declares_the_artifact_panel_under_its_body_key` | unchanged laws, kept |

Test fixtures build host state with `ViewModel { tree_windows: vec![TreeWindowRequest{…}], .. }` and
`ViewModel { tree_viewport_rows: Some(512), .. }` (a viewport tall enough that the label / nesting /
keying laws read the whole demo rather than one screenful).

## 4. Verification

All four runs foreground, `DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0 … -j 2`
(the shared target dir was swap-thrashed by the rest of the fleet — several attempts were SIGKILLed at
exit 137 before landing; raw logs under `🗑️generated/a4/`).

| command | result |
|---|---|
| `cargo test -p semio-s-artifact-fem-3d --features component-app-assembly` | **ok. 1127 passed; 0 failed** — all 27 fem3d panel tests green |
| `cargo test -p semio-s-artifact-fem-2d --features component-app-assembly` | **1257 passed; 4 failed** — all 29 fem2d panel tests green; the 4 failures are not this packet's (below) |
| `cargo check -p semio-s-artifact-fem-3d --features component-app-assembly --target wasm32-wasip2` | **Finished** (also compiles fem-2d's lib for wasm) |
| `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --target wasm32-wasip2` | **Finished** |

Also run first as a fast gate: `cargo check -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d
--features …/component-app-assembly --tests --keep-going` → **Finished, 0 errors, 81 warnings**
(warnings present, so expansion really ran; none of them in the files this packet touched after the two
`unnecessary qualification` ones were fixed).

### The 4 fem-2d failures, none of them A4's
- `analyses::tests::assembly_job_one_fuel_steps_stay_below_eight_milliseconds` (11.7 ms) and
  `mesh::tests::mesh_job_large_boundary_never_runs_to_completion_in_one_step` (22.4 ms) — wall-clock
  budget laws in the shared `✏️s/🔨️modules/🏗️fem` engine, failing because the machine was at
  ~20 GB/22 GB swap with five peer cargos running. No panel code on either path.
- `editor::fem2d::component::unit_tests::every_route_declares_the_lane_its_handler_emits` — fails on
  `setTransformGumballFlag` → `fem2d.gumball-flag.window-context-required`, introduced by commit
  `f7fef5746d` (2026-09-16 21:35), hours before this packet started.
- `editor::fem2d::component::unit_tests::two_instances_converge_on_disjoint_edits` — fails in the
  framework's `interactive-job.catalog-authority` proof (`generated_migrated=false` for every fem2d
  tool). A peer is concurrently landing fem2d results/playback-clock work (set-playback-clock
  mutations, result-animation-tick) in the same crate; tool-factory classification is untouched here.

### One fixture correction worth recording
`materials`, `sections` and `analysis` are authored `default_open: false`. Under the old pager a
collapsed section still built all its rows; under the window law a closed container materialises ZERO
children. Four fem-2d panel laws initially failed on exactly that, and the fix was in the test fixture,
not the panel: `wide_view()` now files `TreeWindowRequest{open: Some(true), rows: 128}` for those three
containers. `default_open` semantics in the panel are unchanged, per §8 rule 1.

## 5. Gate note

`📓️p1-contract.md` and `📓️p3-sdk.md` never appeared in the ticket folder inside the brief's 45-minute
poll window (polled from 00:40 to 01:22). Both packets' **code** is landed and was used as the source
of truth instead of §5: `TreeWindow` + `TreeItemProps::granularity` + `TreeSectionProps::window` in
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/`, `ViewModel::tree_windows` / `tree_viewport_rows` +
`TreeWindowRequest` in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4581`, and region `🔖️PanelWindowing`
(`TreeWindows`, `tree_window_section[_or_placeholder]`, `tree_window_item`, `ui_node_list`,
`PanelTreeBuilder::window_section[_or_placeholder]`, two-argument `interaction_domain`) plus the
re-export at `🔌️plugin/🦀️.rs:39495`. Two signatures differ from §5 and this packet follows the code:
`window_section*` take `id: &str` (not `impl AsRef<str>`), and `interaction_domain(controller_id:
&'static str, domain)`.
