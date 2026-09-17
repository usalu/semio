# 📓️ A1 — 🧩️puzzle 3d migration to windowed panel trees

Packet A1 of wave 2 (📓️wave2-app-brief.md). Crate: `semio-s-artifact-puzzle-3d`,
`--features component-app-assembly`. Editor root
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` (`ED` below).

## 1. Panels migrated

| panel | what changed |
|---|---|
| `ED/📌️panels/🗿️artifact/🦀️.rs` | four sections → `PanelTreeBuilder::window_section`; object rows nest their vortices through `tree_window_item(windows, item, &object.id, false, &object.vortices, …)`; every row is now a `granularity` pick row (`pick_item`) keyed by the raw target id with NO per-row `interactionSelect` argument map; `.interaction_domain(PUZZLE3D_PLAY_CONTROLLER_ID, PUZZLE3D_INTERACTION_DOMAIN)` stamps the ONE tree-level pick binding; hide/lock `RowAction`s untouched on object/reference/target-volume rows |
| `ED/📌️panels/🛍️catalogue/🦀️.rs` | four windowed sections (`objects`/`vortices`/`cables`/`attractions`); each object-kind row is a `tree_window_item` over its rim-vortex templates (templates now stream and stamp their own `total`); rows keep their own `addObjectKind` binding and their drag data, so they are not pick targets; the dead page cursor (the `+N` that dispatched `setPanelPage` into a map nobody read back — 📓️audit-app-panels-a.md §4a) is gone |
| `ED/📌️panels/🔍️inspection/🦀️.rs` | the selected-id list is its own windowed section keyed `puzzle3d-play-inspector.ids` (`IDS_SECTION`), rows keyed by the RAW id (`{IDS_SECTION}.{id}`, never `ids.{index}` — an offset window must not renumber keys); the section is authored only when the selection names ids at all; entity field groups follow it in the same tree; `flag_row`'s `ids` argument list is now bounded by what the argument arena admits (push until refused) instead of by a row quota |
| `ED/📌️panels/⚙️settings/🦀️.rs` | imports the SDK `ui_node_list` (not a tree, otherwise untouched) |

`TreeWindows::for_body(view_state, <body key>)` is built per body in the app's single render funnel
`Puzzle3dPlayApp::render_body` (`ED/🦀️.rs`, which both `ArtifactEditor::render` and
`render_with_request_context` already funnel into) and threaded into the three panels as
`&TreeWindows<'_>`.

## 2. Symbols deleted

**`ED/📌️panels/🗿️artifact/🦀️.rs`** — the whole `🔖️Paging` region: `page_rows`, `page_rows_for`,
`PANEL_RECONCILE_NODE_BUDGET`, `SECTION_ROWS`, `SECTIONS`, `PANEL_PAGE_GUARD`, `RowBudget`,
`continuation_row`, `continuation_row_from`, `page_action`, `section_page`, `paged_section`,
`paged_section_from`, `render_from`; plus `select_action`, `selectable_item`, the `InteractionTarget`/
`serde_json` target encoding, the local `ui_node_list` and `ui_value_number`.

**`ED/📌️panels/🔍️inspection/🦀️.rs`** — `IDS_ROWS`, `ids_page`, `push_ids` (and its `setPanelPage`-carrying
`+N` row), the `pages: &BTreeMap<String, u32>` parameter on every `*_fields` function.

**`ED/📌️panels/🛍️catalogue/🦀️.rs`** — the `artifact::{page_rows, paged_section, RowBudget, SECTIONS}`
import and the eager `object_kind_vortex_items` builder.

**`ED/🦀️.rs`** — the `set_panel_page` command import, `SetPanelPage = "setPanelPage"` enum variant, the
`TOOL_JOB_IDS` entry, the `"setPanelPage" => …` dispatch arm, the `PUZZLE3D_RETAINED_TOOL_IDS` entry, the
`build_tool_job` match arm, `ArtifactToolPublicationContract { tool_id: "setPanelPage", … }`,
`.view_action("setPanelPage", …)`, `.action_interactive_job("setPanelPage", Migrated)`, the crate-local
`pub fn ui_node_list`, and the whole outliner memo — `Puzzle3dArtifactTreeKey`,
`Puzzle3dPlayApp::artifact_tree_cache`, `artifact_tree_cached`/`artifact_tree_cached_from`,
`Puzzle3dSessionState::artifact_tree` (+ its byte charge and its check-in/check-out copies) and the
`PUZZLE3D_ARTIFACT_TREE_BUILDS` test counter. Rendering a window is cheap; the memo only existed to pay
for rebuilding a whole page.

**Command**: `ED/🎮️commands/📄set-panel-page/` (directory removed) and its
`#[path = …] pub mod set_panel_page;` declaration in `🗿️artifacts/🧊️3d/🦀️.rs`.

**State**: `Puzzle3dWindowConfig::panel_pages` (`ED/🪟️window/🦀️.rs`, the persisted DSL artifact
`s.puzzle.puzzle3d.windowconfig`) and `Puzzle3dRuntime::panel_pages` (`ED/🎚️config/🦀️.rs`), plus the
`BTreeMap` imports both files no longer need and the `panel_pages` copies in `from_runtime`/`runtime`.

**Fixtures**: `"setPanelPage"` removed from the `Puzzle3dPlayApp` window-config route group in
`✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` and from `toolIds` in
`…/🧊️3d/…/🧫️fixtures/🗄️retained-jobs/🔣️.json`.

**Schema — nothing to do.** 📓️audit-app-panels-a.md §4b already recorded that puzzle-3d's generated
window schema (`ED/🪟️window/🧬️schema/{🔣️.json,🔗️.graphql,🟦️.ts,🛰️.proto}`) never declared `panelPages`
even though the Rust struct carried it; verified again here (zero hits in all four). Removing the Rust
field therefore *closes* that pre-existing drift rather than opening one, and the TS
`publication-authority-audit`'s exact-record oracle for `Puzzle3dWindowConfig` (which already listed the
17 keys without `panelPages`) is now truthful.

**TS / manifest** — checked and clean: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript` and the plugin
manifest `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` carry zero `setPanelPage`/`panelPages` references (the manifest
never declared the action at all — 📓️audit-app-panels-a.md §0).

Post-migration grep over the whole 🧊️3d tree for
`setPanelPage|set_panel_page|panel_pages|panelPages|SECTION_ROWS|IDS_ROWS|PANEL_PAGE_GUARD|PANEL_RECONCILE_NODE_BUDGET|paged_section|continuation_row|artifact_tree_cache`
returns only two hits, both doc comments in the new tests naming the deleted behaviour.

## 3. Tests rewritten

All four files named in the packet, plus one editor-level law the packet inherited. Window requests are
built exactly as the brief prescribes —
`ViewModel { tree_windows: vec![TreeWindowRequest { body_key, node_key, open, offset, rows }], ..Default::default() }`
read back with `TreeWindows::for_body(&view, BODY_KEY)`; the cold path uses `TreeWindows::unhosted()`.
Body JSON comes from `artifact_app_laws::project_and_retire_fixture_tree`, so every asserted tree is
also retired.

| file | laws |
|---|---|
| `ED/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | **(a)** `an_oversized_document_stamps_every_containers_total_and_materialises_only_its_window` — the Nakagin fixture (180 objects / 358 vortices): the objects section stamps `window.total == 180`, materialises exactly the 10 requested rows keyed by raw object id, the opened object row stamps its own vortex total, and the body JSON contains no `.more` key and no `"+` label · **(b)** `a_closed_container_stamps_its_total_and_materialises_no_child` (host `open: false` AND an author-default-closed object row) · **(c)** `a_tree_window_request_materialises_exactly_its_slice` — `{offset: 120, rows: 6}` over 200 objects, and a nested `{offset: 2, rows: 2}` vortex window · **(d)** `every_row_is_a_domain_pick_target_and_only_the_tree_binds_the_pick` — all five granularities carry `granularity` and no activation binding while the root carries exactly one `interactionSelect`; hide/lock row actions still present · plus `the_first_paint_draws_one_viewport_and_still_reports_the_whole_document`, `the_outliner_assembles_its_four_document_sections`, `the_panel_tab_declares_…`, and the two inherited hide/lock laws (`…dispatch_the_inverse_of_the_current_flag`, `an_outliner_flag_row_undoes_itself_on_the_second_click`) rewritten onto the new render signature |
| `ED/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | **(a)** `an_over_wide_catalog_stamps_its_total_and_materialises_only_its_window` (200 kinds × 40 templates) · **(b)** `a_closed_catalogue_section_stamps_its_total_and_materialises_no_child` · **(c)** `a_tree_window_request_materialises_exactly_its_slice_of_the_catalog` (section `{90,4}` + a kind row's template window `{3,3}`) · the four inherited laws (drag data, `addObjectKind` binding, default-open, concrete-forest rows) kept, with their `.more` filters dropped |
| `ED/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | **(a)** `a_wide_selection_stamps_every_id_and_materialises_only_its_window` (120 ids, 8 requested) · **(b)** `a_closed_id_section_stamps_its_total_and_materialises_no_child` · **(c)** `a_tree_window_request_materialises_exactly_its_slice_of_the_id_list` (`{60,3}`, keyed by raw id) · `the_ids_section_appears_only_when_the_selection_names_ids` · the five inherited leftover-selection laws |
| `ED/🪟️window/🧪️tests/🔬️unit/🦀️.rs` | `window_config_pack_round_trips_every_persisted_option` no longer seeds `panel_pages` (the field is gone); every other persisted option still round-trips through pack AND text |
| `ED/🧪️tests/🔬️selection-scale/🦀️.rs` | `the_flagship_outliner_panel_body_carries_paged_object_rows` → `the_flagship_outliner_panel_body_windows_its_object_rows`: through the real `render_panel_body` route it now asserts the first object row is present, that some container stamps `window.total == 180` (new `stamps_window_total` helper), and that a continuation row is a REGRESSION |

## 4. Verification (foreground-equivalent; logs under `🗑️generated/a1/`)

| command | result | log |
|---|---|---|
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --no-run` | `Finished` in 6m 48s, 0 errors, 154 warnings (proof the crate expanded) | `build.txt` |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly` | **737 passed; 8 failed** — all 27 panel laws + the migrated selection-scale law + all 7 window laws green; the 8 failures are pre-existing/peer-caused, §5 | `test2.txt` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --target wasm32-wasip2` | `Finished` `dev` in 1m 02s, no errors | `wasm.txt` |
| `bun ./📜️script.ts publication-authority-audit` (`@semio-tech/puzzle-js`) | **validated**, all three owners; `admitted=` no longer lists `setPanelPage`; `windowOwnershipCases=7` | `pub-audit.txt` |

Every named panel/window law, verbatim from `test2.txt`:

```
panels::artifact::tests::{the_panel_tab_declares_the_framework_artifact_slot_and_this_body_key,
  the_outliner_assembles_its_four_document_sections,
  an_oversized_document_stamps_every_containers_total_and_materialises_only_its_window,
  a_closed_container_stamps_its_total_and_materialises_no_child,
  a_tree_window_request_materialises_exactly_its_slice,
  every_row_is_a_domain_pick_target_and_only_the_tree_binds_the_pick,
  the_first_paint_draws_one_viewport_and_still_reports_the_whole_document,
  outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag,
  an_outliner_flag_row_undoes_itself_on_the_second_click}                       ... 9 ok
panels::catalogue::tests::{…}                                                   ... 8 ok
panels::inspection::tests::{…}                                                  ... 8 ok
window::component::tests::window_config_pack_round_trips_every_persisted_option ... ok
selection_scale::the_flagship_outliner_panel_body_windows_its_object_rows       ... ok
```

## 5. The 8 remaining crate failures are NOT this packet's

Evidence: `git diff df7a4916bd HEAD -- <ED/🦀️.rs>` (the auto-commit that carried this packet) contains
**only** this packet's edits — no subject of any failing test is in the diff — while the same commit
carries 2 749 inserted framework lines from wave 1. None of the eight names a panel, a tree or a window.

| failing test | why it is not ours |
|---|---|
| `…mutation_latency::one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` | the publication ladder's STORE units grow with the document (22 vs 332); the store/publication lane, untouched here (the memo we deleted was a render cache, not a store) |
| `…unit_tests::every_context_menu_row_dispatches_a_declared_action` | 5 granularities now emit a `zoom` row, 3 expected; `puzzle3d_context_menu_items` is byte-identical to `df7a4916bd` |
| `…unit_tests::the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind` | window addressing falls back to `puzzle3d-main-top` instead of `puzzle3d-main`; `puzzle3d_addressed_window_id` is unchanged — wave 1 changed `ViewModel::for_panel`/`for_window_instance` to `..self.clone()` (📓️design-virtualised-tree.md §4) |
| `…unit_tests::window_options_are_local_to_the_window_instance_not_shared_across_split_panes` | `load_window_config_pack` → framework fault `window-config.typed-state`. Our own struct's pack AND text round trip is green in the SAME run (`window_config_pack_round_trips_every_persisted_option`), so the rejection is in the framework's retained loader, not in the shape we changed |
| `…unit_tests::two_instances_converge_disjoint_object_edits_via_backbone` | `module.vcs`: "remote snapshot merge is fail-closed until the app-owned streaming envelope decoder … are terminal-authorized" — a declared framework gap |
| `…precompute::brush::…_stays_below_the_interactive_ceiling_for_nakagin` · `…precompute::fill::…` | µs-deadline laws (23 485 µs vs 2 000 µs; 2.11 ms vs 2 ms) measured while ~17 peer cargo jobs and 18 rustc processes were saturating the machine |
| `…schema::mutations::binary::wire_format_guard::engine_command_rows_keep_their_pre_migration_wire_bytes` | the frozen hex moved by a constant variant offset (`0102…` vs `0104…`); `Puzzle3dEngineCommand` and the test are both unmodified, and `dsl::variants_binary` was rewritten in commits 633–635 today |

## 6. One pre-existing defect repaired (needed for a green audit on the owner we touched)

`bun …:publication-authority-audit` was **already red for `Puzzle3dPlayApp`** before this packet: the
Rust contract `ArtifactToolPublicationContract { tool_id: "addObjectKind", lanes: &[Artifact, Config,
Interaction] }` has carried the `Interaction` lane since at least commit `6105063a33`, while the fixture
grouped `addObjectKind` with `setActiveExample` under `["artifact","config"]`. (A2 only ran the audit
scoped to `Puzzle2dPlayApp`, so it went unnoticed.) Fixed by splitting that fixture group so
`addObjectKind` declares `["artifact","config","interaction"]` and `setActiveExample` keeps
`["artifact","config"]`, matching the Rust exactly. The audit is now green for all three owners.

## 7. Nothing left unfinished in this packet

`📓️p3-sdk.md` landed late (after the runs) and was read: every signature in its §1 matches what this
packet calls, including the `&'static str` controller id, `label: Option<Label>` on the
`PanelTreeBuilder` methods and `placeholder_label` last — no call site needed a change. `📓️p1-contract.md`
never appeared (polled ~45 min). Until p3 landed, the gate's substance was taken from the tree: `TreeWindow` / `TreeSectionProps::window` /
`TreeItemProps::window`+`granularity` are in the contract, the SDK region `🔖️PanelWindowing`
(`TREE_WINDOW_DEFAULT_ROWS`, `TreeSlice`, `TreeWindows`, `tree_window_section[_or_placeholder]`,
`tree_window_item`, `ui_node_list`, `PanelTreeBuilder::window_section[_or_placeholder]`, two-argument
`interaction_domain`) is landed, the old `🔖️PanelPaging` region is gone, and peer packets (a2, a3) had
already run cargo against it. All code was written against 📓️design-virtualised-tree.md §5 first and
matched the landed signatures with one adjustment: `window_section`'s `id` is `&str`, so the call sites
pass `&format!(…)`.
