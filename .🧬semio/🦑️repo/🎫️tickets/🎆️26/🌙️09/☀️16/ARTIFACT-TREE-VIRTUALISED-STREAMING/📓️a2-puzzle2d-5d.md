# 📓️ A2 — 🧩️puzzle 2d + 5d migration to windowed panel trees

Packet A2 of wave 2 (📓️wave2-app-brief.md). Crates: `semio-s-artifact-puzzle-2d` and
`semio-s-artifact-puzzle-5d`, both `--features component-app-assembly`.

## 1. Panels migrated

| crate | panel | what changed |
|---|---|---|
| puzzle-2d | `…/◻️2d/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` | both sections → `PanelTreeBuilder::window_section_or_placeholder`; node/edge rows are now `granularity` pick rows keyed by the raw entity id, built through a local `pick_row` over `ui::tree_item`; `.interaction_domain(PUZZLE2D_PLAY_CONTROLLER_ID, PUZZLE2D_INTERACTION_DOMAIN)` |
| puzzle-2d | `…/📌️panels/🛍️catalogue/🦀️.rs` | three windowed sections (`nodes`/`handles`/`edges`); node rows keep `tree_item_with_action_draggable`, handle/edge rows keep `tree_item_with_action` — a catalogue row owns its own `addNode` binding and is not a pick target; section ids lifted to `ROOT`/`NODES_SECTION`/`HANDLES_SECTION`/`EDGES_SECTION` consts |
| puzzle-2d | `…/📌️panels/🔍️inspection/🦀️.rs` | the selected-id list became its own windowed section `puzzle2d-play-inspector.ids` (`ids_section`), rows keyed by raw id; the static `+N` row and the `IDS_ROWS = 8` truncation are gone; `node_fields`/`edge_fields` lost their `ids` parameter |
| puzzle-2d | `…/📌️panels/⚙️settings/🦀️.rs` | imports the SDK `ui_node_list` (no tree, otherwise untouched) |
| puzzle-5d | `…/🖐️5d/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` | both sections windowed; each part row is a `tree_window_item` over its grips (nested grip lists now stream and stamp their own `total`); part/grip/fastener rows are `granularity` pick rows keyed by raw id (`puzzle5d_grip_full_id` for grips); `.interaction_domain(PUZZLE5D_PLAY_CONTROLLER_ID, PUZZLE5D_INTERACTION_DOMAIN)`; `select_action`/`selectable_item`/`InteractionTarget` encoding deleted |
| puzzle-5d | `…/📌️panels/🛍️catalogue/🦀️.rs` | four windowed sections (`parts`/`grips`/`fasteners`/`ropes`); `kind_catalog_items` (eager `UiFixedList`) replaced by a per-row `kind_catalog_item` plus an `indexed()` helper that keeps the `{section}.{index}.{kindId}` key shape; part rows keep their `addPartKind` binding |
| puzzle-5d | `…/📌️panels/🔍️inspection/🦀️.rs` | no list — only switched to the SDK `ui_node_list` |

`TreeWindows::for_body(view_state, body_key)` is built once per render in each app's render entry and
threaded in as `&TreeWindows<'_>`:
- `…/◻️2d/…/✏️editor/🦀️.rs` `Puzzle2dPlayApp::render_body` (one funnel for both render entry points).
- `…/🖐️5d/…/✏️editor/🦀️.rs` `Puzzle5dPlayApp::render` **and** `render_with_request_context` (5d has two).

## 2. Symbols deleted

**puzzle-2d**
- `📌️panels/🗿️artifact`: `SECTION_ROWS`, `SECTIONS`, `section_page`, `continuation_row`, `paged_section`,
  `selection_args`, the per-row `ActionFactory … INTERACTION_SELECT_ACTION_ID` binding.
- `📌️panels/🔍️inspection`: `IDS_ROWS`, `push_ids` (and its static `"+{n}"` row).
- `📌️panels/🛍️catalogue`: the `artifact::paged_section` import, `PanelRowBudget`, `SECTIONS`, `BTreeMap`.
- `🎮️commands/📄set-panel-page/` (whole directory) and its `pub mod set_panel_page;` declaration in
  `🗿️artifacts/◻️2d/🦀️.rs`.
- `✏️editor/🦀️.rs`: the `set_panel_page` command import, the `"setPanelPage" => …` dispatch arm,
  `PUZZLE2D_RETAINED_TOOL_IDS` entry, `PUZZLE2D_GENERIC_TOOL_IDS` entry, the `TOOL_IDS` list entry in
  `build_tool_job`'s catalogue, `ArtifactToolPublicationContract { tool_id: "setPanelPage", … }`,
  `.action_with(puzzle2d_internal_action("setPanelPage", …))`,
  `.action_interactive_job("setPanelPage", Migrated)`, and the crate-local `pub fn ui_node_list`.
- `🪟️window/🦀️.rs`: `Puzzle2dWindowTransient::panel_pages`, its `artifact_retire_struct!` member, its
  retained-bytes charge loop, and the `panel_pages` copies in `runtime()` / `split()`.
- `🎚️config/🦀️.rs`: `Puzzle2dPlayRuntime::panel_pages` (field + `Default`).
- `🪟️window/🧬️schema/`: `panelPages` removed from `🟦️.ts`, `🔣️.json`, `🔗️.graphql` and
  `map<string, uint32> panel_pages = 5;` from `🛰️.proto` — hand-fixed in all four consistently, because the
  puzzle plugin has **no** schema-generation nx target (`📦️packages/🦀️rust/📋️project.json` exposes only
  `wasm`, `test*`, `fixtures-lint`, `describe`; the TS project only `test` and `publication-authority-audit`).
- Fixtures: `"setPanelPage"` removed from the `Puzzle2dPlayApp` window-transient route group in
  `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` and from `toolIds` in
  `…/◻️2d/…/🧫️fixtures/🗄️retained-jobs/🔣️.json`.

**puzzle-5d**
- `📌️panels/🗿️artifact`: `ui_text_value`, `ui_map_value`, `select_action`, `selectable_item` and the
  eager `UiFixedList` loops over parts/grips/fasteners.
- `📌️panels/🛍️catalogue`: `kind_catalog_items`.
- `✏️editor/🦀️.rs`: the crate-local `pub fn ui_node_list`.
- 5d never had `setPanelPage`/`panel_pages` (confirmed: zero hits).

Repo grep after the migration over both artifact trees for
`paged_section|SECTION_ROWS|IDS_ROWS|panel_page_rows|PanelRowBudget|panel_continuation_row|setPanelPage|panel_pages|panelPages|section_page`
returns only one hit: a doc comment in the new 2d inspection test naming the deleted `IDS_ROWS` constant.

## 3. Tests added

Both crates had **no** paging/window coverage before (2d's artifact-panel test was 23 lines of label
assertions; the 2d catalogue and inspection panels had no test file at all; 5d had three one-line smoke
tests). Every new law drives the panel `render(&scene, labels, &windows)` directly over a synthetic
document and walks the returned `BuiltNode`, with a `drain_retired_ui_owners()` pump around each build
(the process-wide `BuiltNode`/`UiValue` page credit is shared, as in puzzle-3d's suite).

| file | laws |
|---|---|
| `…/◻️2d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | (a) `the_outliner_stamps_every_container_and_never_pages` · (b) `a_closed_outliner_container_stamps_its_total_and_builds_no_row` · (c) `a_host_window_materialises_exactly_its_slice_keyed_by_raw_id` · (d) `outliner_pick_rows_are_domain_bound_without_a_per_row_binding` |
| `…/◻️2d/…/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` (new file + `mod tests`) | (a)+(b) `the_catalogue_stamps_every_section_and_never_pages` · (c) `a_catalogue_window_materialises_its_slice_with_row_bindings_intact` |
| `…/◻️2d/…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` (new file + `mod tests`) | (a) `the_inspector_ids_section_stamps_the_whole_selection` · (b) `a_closed_inspector_ids_section_builds_no_row` · (c) `an_inspector_ids_window_materialises_exactly_its_slice` |
| `…/🖐️5d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | (a)/(b)/(c)/(d), (c) covering a nested grip window as well as the section window |
| `…/🖐️5d/…/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | (a)+(b) and (c); the four-section smoke test now asserts against the new section consts |

Request fixtures are built exactly as the brief prescribes:
`ViewModel { tree_windows: vec![TreeWindowRequest { body_key, node_key, open, offset, rows }], ..Default::default() }`,
read back with `TreeWindows::for_body(&view, BODY_KEY)`; the unhosted first-paint path uses
`TreeWindows::unhosted()`.

## 4. Verification

All runs foreground-equivalent (harness background tasks, waited to completion), output under
`🗑️generated/a2/`. Fleet contention on the shared debug artifact lock made each run take 10-40 min of
wall clock; two earlier attempts died on my own 600 s command timeout (exit 137), not on a build fault.

| command | result | log |
|---|---|---|
| `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- panels:: --test-threads=1` | **ok. 11 passed; 0 failed** (715 filtered out) | `🗑️generated/a2/2d-panels.txt` |
| `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- panels:: --test-threads=1` | **ok. 9 passed; 0 failed** (338 filtered out) | `🗑️generated/a2/5d-panels.txt` |
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --target wasm32-wasip2` | **Finished** `dev` in 1m 02s, no errors | `🗑️generated/a2/2d-wasm.txt` |
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2` | **Finished** `dev` in 29.72s, no errors | `🗑️generated/a2/5d-wasm.txt` |
| `bun nx run @semio-tech/puzzle-js:publication-authority-audit -- Puzzle2dPlayApp` | **validated**; `admitted=` no longer lists `setPanelPage`; `windowOwnershipCases=7` (the TS `Puzzle2dWindowTransient` exact-record oracle already expected the four keys without `panelPages`) | `🗑️generated/a2/2d-publication-audit.txt` |

Two framework changes from wave 1 surfaced during the runs and were fixed inside this packet:
- **Built trees no longer serialize directly.** `serde_json::to_string(&BuiltNode)` now fails with
  `BuiltChildren requires retained page transport`, which broke the puzzle-2d test context's
  `render_body_with_view` (and therefore its two pre-existing label tests, before this packet touched
  anything else). Switched it — and the five new body-JSON assertions — to
  `artifact_app_laws::project_and_retire_fixture_tree(built_to_component_tree(node))`, the helper
  puzzle-5d's context already used.
- **`ActionId` is a record, not a string.** The root-binding assertion reads `action.name.as_str()`.

Also fixed, in the two panel tests this packet inherited: they built an app and dropped it without the
context's `close_app`, which the framework's store-drop witness now aborts on
(`artifact store reached Drop without its exact terminal-empty shallow-shell witness`). Added the
`close_app` calls; both are green.

## 5. Notes / deviations

- **Wave-1 gate.** `📓️p1-contract.md` and `📓️p3-sdk.md` never appeared in this ticket folder (polled well
  past the brief's window). The gate's substance was satisfied directly from the tree: the contract
  (`TreeWindow`, `TreeSectionProps::window`, `TreeItemProps::window`/`granularity`,
  `TreeItemBuilder::window`/`granularity`) and the whole SDK region `🔖️PanelWindowing`
  (`TREE_WINDOW_DEFAULT_ROWS`, `TreeSlice`, `TreeWindows::{for_body,unhosted,is_open,slice,window}`,
  `tree_window_section[_or_placeholder]`, `tree_window_item`, `ui_node_list`,
  `PanelTreeBuilder::{window_section,window_section_or_placeholder}` and the two-argument
  `interaction_domain(controller_id, domain)`) are landed in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, the old `🔖️PanelPaging` region is gone, and a peer
  packet (a3-cad) had already run cargo against it. Code was written against §5 and matches the landed
  signatures exactly.
- **Audit correction.** 📓️audit-paging-tests.md §1 claims the puzzle-2d window round-trip test
  (`…/◻️2d/…/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:176-197`) seeds `panel_pages` with
  `("objects",3)`/`("references",1)`. It does not — the committed file has zero `panel_pages` hits, so no
  round-trip test needed updating. (Checked against `git show HEAD:…`.)
- **Pre-existing, unrelated abort still open in `semio-s-artifact-puzzle-2d` (outside the panels).**
  `editor::puzzle2d::component::unit_tests::ingest_operations_is_idempotent` SIGABRTs the whole test
  binary: `artifact store reached Drop without its exact terminal-empty shallow-shell witness` /
  `mutation dag reached Drop before every exact envelope and identity owner was cursor-retired`. That test
  (and its neighbour `instance_a`/`instance_b` convergence test) never calls the context's `close_app`, so
  the framework's store-drop witness fires on `Drop` — the same law that hit the two panel tests, which
  this packet did fix. Nothing here touches the artifact store, the mutation DAG or those convergence
  tests, so they are left to their owner; because the abort kills the binary, the panel laws above are run
  under the `panels::` filter (`--lib`) rather than as a whole-crate `cargo test`.
- **Left to packet A1 (puzzle 3d), deliberately untouched:** the remaining `"setPanelPage"` entries in
  `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` (the `Puzzle3dPlayApp` window-config
  group) and `…/🧊️3d/…/🧫️fixtures/🗄️retained-jobs/🔣️.json`.
- **Pre-existing fixture/const order mismatch (not introduced here).** `PUZZLE2D_RETAINED_TOOL_IDS` and the
  2d `🗄️retained-jobs` `toolIds` array disagree on the *position* of `setSelectionFlag`/`setSuggestionOffset`
  relative to `translateSelection…openImportFixture`, and
  `🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
  compares them order-sensitively. Verified against `git show HEAD:…` that the same disagreement exists in
  the committed tree; this packet removed `setPanelPage` from both sides and changed nothing else.
