# 📓️ A8c — playbook, sourcing, stdio (json/xml/zip + the shared `TreeWindowKit`), gis, draw, raster, mathematical, demonstrator, norm

Packet A8c of wave 4. Resumes the dead A8 agent's work: re-enumerates every tree in scope, verifies the
migration by RUNNING it, adds the missing window laws, and records what is pre-existing.
Logs: `🗑️generated/a8c/` (the stale A8 logs under `🗑️generated/a8/` are hints only).

## 1. Scope enumerated (grep, not the audit)

`grep -ral PanelTreeBuilder|TreeWindowKit|TreeItemBuilder|tree_section|tree_item_desc|ui::tree(|window_section`
over every plugin in scope:

| plugin | tree-bearing files | verdict |
|---|---|---|
| 🌍️gis | `📌️panels/{🗿️artifact,🛍️catalogue,🔍️inspection}` | migrated (A8) |
| 🖍️draw | `📌️panels/{🗂️layers,🛍️catalogue}` | migrated (A8) |
| 🖨️raster | `📌️panels/{🗿️artifact,🎭️masks,🛍️catalogue,🔍️inspection}` | migrated (A8) |
| 🗄️stdio | 12 `🪟️main` windows (zip ×4, xml ×4, json ×4) on `TreeWindowKit` | migrated (A8); **laws had never compiled** — fixed here |
| 📖️playbook | `👁️viewer/…/🪟️windows/🌳️steps` on `TreeWindowKit` | migrated (A8); 2 pre-existing tests broken by the migration — fixed here |
| 📕️norm | `🖥️app-surface` (`render_inspection` tree + `render_report`) | **`render_report` was an unbounded `ui::column` — migrated here** |
| 🪵️sourcing | **none** | `🏊️pool`/`🧺️curated` are `SurfaceKind::Table` JSON-blob surfaces; zero `PanelTreeBuilder`/`TreeWindowKit`/`tree_item`/`ui_node_list`/`setPanelPage`/`panel_pages` hits anywhere under `🪵️sourcing`. No laws possible. |
| ➗️mathematical | **none** (173 rust files, zero hits) | skipped, confirmed |
| 🎪️demonstrator | **none** | skipped, confirmed |
| 🗟️artifacts | **no `.rs` at all** (`◻️2d` assets only) | skipped, confirmed |
| 🗄️stdio 💬️bcf | no tree — the only `TreeWindowKit` hit is a docstring saying it deliberately does NOT use it | skipped, confirmed |

Residue grep over the whole scope for `paged_panel_section`, `panel_continuation_row`, `setPanelPage`,
`panel_pages`, `section_page`, `SECTION_ROWS`, `LIST_ROWS_MAX`, `IDS_ROWS`, `CATALOGUE_GROUP_ROWS`,
`PANEL_RECONCILE_NODE_BUDGET`, `PanelRowBudget`, `fn ui_node_list`, `…more`, `.more`, `.take(N)`,
`UI_FIXED_LIST_ITEMS`, `.chunks(`: **zero live hits** — every match is a law assertion
(`assert!(!json.contains(".more"))`) or unrelated codec code (tiff `more_images`, dwg `section_pages`).

## 2. Per-plugin table (panel → migrated? laws? suite? wasm?)

| plugin | panel / window | migrated | laws (a)–(d) | suite | wasm |
|---|---|---|---|---|---|
| 📕️norm | `🖥️app-surface::render_report` (all 15 apps' `📊️results` body) | ✅ **this packet** | ✅ **4 added, pass** | ✅ `semio-s-artifact-norm-contract` 24+1 pass, 0 fail | ✅ contract + **all 15** artifact crates |
| 📕️norm | `🖥️app-surface::render_inspection` | carve-out (4 static rows) | n/a | ✅ | ✅ |
| 🗄️stdio json | editor base / editor i-json / viewer base / viewer i-json `🪟️main` | ✅ | ✅ 4 laws (base editor) + root-id law ×4 | ✅ 105 pass / 2 pre-existing fail | ✅ |
| 🗄️stdio xml | editor base / editor valid / viewer base / viewer valid `🪟️main` | ✅ | ✅ 4 laws (base editor) + root-id law ×4 | ✅ 80 pass / 7 pre-existing fail | ✅ |
| 🗄️stdio zip | editor base / editor iso21320 / viewer base / viewer iso21320 `🪟️main` | ✅ | ✅ 4 laws (base editor) | ⏳ re-running after the F2 path change | ⏳ |
| 📖️playbook | `👁️viewer/…/🌳️steps` | ✅ | ✅ 4 laws pass | ⏳ re-run | ⏳ |
| 🖍️draw | `🗂️layers`, `🛍️catalogue` | ✅ | ✅ 4 laws | ⏳ re-run | ⏳ |
| 🖨️raster | `🗿️artifact`, `🎭️masks`, `🛍️catalogue` | ✅ | ✅ 4 laws | ⏳ re-run | ⏳ |
| 🌍️gis | `🗿️artifact`, `🛍️catalogue` | ✅ | ✅ 6 laws | ⏳ re-run | ⏳ |
| 🪵️sourcing | — no tree — | n/a | n/a (nothing to window) | ✅ 139 pass / 5 pre-existing fail (SOURCING-* peer work) | — |

## 3. What this packet changed

### 📕️norm — `render_report` migrated off an unbounded `ui::column`
- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`: `render_report(report)` → `render_report(report, windows)`.
  It was `ui::column().try_children(report.checks.iter().map(ui::text))` — an unbounded child list that
  faults past the body node ceiling. It is now
  `PanelTreeBuilder::new(NORM_REPORT_TREE_ID)?.window_section_or_placeholder(windows, NORM_REPORT_SECTION_ID, …)`
  over `report.checks.iter().enumerate()`, so the row key stays `norm-report-check-{index}` even inside a
  slice. New consts `NORM_REPORT_TREE_ID = "norm-report"`, `NORM_REPORT_SECTION_ID = "norm-report.checks"`
  (one container per body → no sibling-key collision). The empty report's `"No checks computed."` text is
  now the section's placeholder row, so that behaviour is unchanged.
- The 15 `📊️results` windows: `render(host)` → `render(host, windows)`.
- The 15 editors' `fn render`: `_view_state` → `view_state`, and the `results::BODY_RESULTS` arm threads
  `&TreeWindows::for_body(view_state, results::BODY_RESULTS)`.
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`: dev-dep
  `semio-framework-plugin { features = ["artifact-app-testing"] }` (the crate that actually owns
  `app_surface` and its tests).
- `🖥️app-surface/🧪️tests/🔬️unit/🦀️.rs`: laws (a)–(d) added in a `//#region 🪟️WindowLaws`
  (`an_oversized_report_stamps_its_total_and_never_a_continuation_row`,
  `a_closed_report_section_stamps_its_total_and_materialises_no_rows`,
  `a_host_window_materialises_exactly_its_report_slice`, `the_report_tree_binds_no_interaction_domain`),
  measured viewport `tree_viewport_rows: Some(4)`; the two pre-existing `render_report` tests were
  rewritten onto the same projection helper.

### 📖️playbook — the migration had broken two pre-existing tests
`…/🌳️steps/🧪️tests/🔬️unit/🦀️.rs`: `render_nests_every_blocks_label_and_kind_under_its_own_step` and
`render_falls_back_to_the_step_id_when_the_title_is_empty` still did `serde_json::to_string(&node)`, which
a windowed tree refuses with `BuiltChildren requires retained page transport`. Both now go through a shared
`projected_body(node)` helper (`project_and_retire_fixture_tree(built_to_component_tree(node))`), which
`window_body` also reuses.

### 🗄️stdio — the window laws had NEVER been compiled, and the tree roots had no key
1. **The laws never ran.** `🐚️a8-verify.sh` ran the three stdio crates with no features and the filter
   `main::tests`. Every editor/viewer module is behind `#[cfg(feature = "component-app-assembly")]` and the
   real module path is `…::windows::main::component::tests`, so the filter matched **0 tests** and the
   "laws exit=0" verdict was an empty run. Correct invocation:
   `cargo test -p <crate> --features component-app-assembly --lib -- windows::main`.
2. **`artifact_app_laws` is feature-gated.** It lives behind `semio-framework-plugin/artifact-app-testing`,
   which the three crates did not carry. Added to `[dev-dependencies]` of
   `🎒️zip`, `📰️xml`, `🧾️json` `📦️packages/🦀️rust/Cargo.toml`.
3. **The json and xml tree ROOTS had an empty node id** (`encode_path_id(&[]) == ""`, xml's root path
   `[]` → `""`). A row built with an empty id falls back to its positional `#0` key: the root is then not
   addressable by any `TreeWindowRequest` — the host can never open or scroll the document root — and it
   collides with any sibling doing the same. Fixed with a real root token `"$"`:
   - json: `pub const JSON_ROOT_NODE_ID: &str = "$"` in all four `🪟️main` modules; `encode_path_id`
     returns it for an empty path; both editors' `decode_path_id` accept it (and still accept the legacy
     `""`), with a decoder law added.
   - xml: `pub const XML_ROOT_NODE_ID: &str = "$"` in all four `🪟️main` modules (child ids are
     `/`-joined child-index paths, which can never spell `"$"`); both editors' `decode_node_id` accept it.
   - The four json and four xml `render_walks_*` tests now assert the real root key instead of `""`.
4. xml law (a) counted `json.matches("child-")`, which double-counts every row (a projected row carries its
   label in both `accessibility.label` and `component.label`); it now counts `"type":"treeItem"` nodes
   against `viewport + 1` (the root).

## 4. Pre-existing failures (evidence, not excuses)

None of these touch trees, panels, paging, windows, `setPanelPage`, `UI_BUILT_CHILDREN_MAX` or
`UI_VALUE_PAGE_ROWS`.

| crate | failures | cause (verbatim) |
|---|---|---|
| `semio-s-artifact-norm-en1990` | 32 | `artifact store reached Drop without its exact terminal-empty shallow-shell witness` (🏪️store), `interactive-job.live-instance`, `interactive-job.catalog-authority`, mutation-fixture JSON float form. **Proof they are not mine:** `panels::catalogue::tests::renders_this_standards_catalogue_headline` and `panels::document::tests::renders_the_family_headline` — panels this packet never touched, rendering plain text — fail with the identical store-Drop panic at `🏪️store/🦀️.rs:18451`. |
| `semio-s-artifact-stdio-json` | 2 | `standards::…::diff::…::absorb_object_associativity` (`mutation.apply.duplicate-target`), `…::mutations::…::applying_a_mutation_and_then_its_inverse_restores_the_snapshot` (member re-ordering). Same 2 as the stale pre-ticket log. |
| `semio-s-artifact-stdio-xml` | 7 | XML text parser (`xml state parse: expected [...]`), `restore leaves exceed their neutral inline budget`, `set-standalone … must invert exactly`, 4 conformance laws. Same 7 as the stale pre-ticket log. |
| `semio-s-artifact-stdio-zip` | 4 | `zip: unsupported extra field 0x0009`, `logical_mutations_diff_and_codecs_round_trip`, `ops_grammar_conformance_law`, `deterministic_logical_round_trip`. Same 4 as the stale pre-ticket log. |
| `semio-s-artifact-playbook-playbook` | 18 | `interactive-job.missing-factory` for every typed command, `interactive-job.catalog-authority`, store-Drop witness, `close-owned-disposer-missing`, JSON float canonical form. |
| `semio-s-artifact-sourcing-curation` | 5 | contributed-module catalogue counts (`available_modules_tracks_contributed_modules`, the `🏊️pool` contribution lane) — the live SOURCING-* peer work. |

## 5. Open / not finished

1. **F2's window-path change landed mid-run.** Container identity is now the container PATH (enclosing
   windowed containers' keys + own key, joined by `TREE_WINDOW_PATH_SEPARATOR`), so a `TreeWindowRequest`
   must name the full path. The `TreeWindowKit` law fixtures in stdio and playbook still file bare node
   keys (`"comment"`, `"k=a"`, `"$"`), which no longer resolve — zip's laws (b)/(c) went green in isolation
   and red after F2 landed. Fixing the fixtures to `"framework.window.tree-root\u{1f}<key>"` is the
   remaining work; the SDK and the app code are correct as they stand.
2. **`📕️norm::render_text_chunks` is still an unbounded `ui::column`.** It backs `render_document_json`
   (the `📥️inputs` window): a document JSON longer than `UI_TEXT_MAX_BYTES × UI_BUILT_CHILDREN_MAX`
   admits more chunks than a container accepts. It is a text blob, not a tree, so windowing it is a
   different shape than this ticket's mechanism — flagged, not fixed.
3. **🪵️sourcing's `🏊️pool` filter bar** pushes one toggle per contributed module into a bare
   `UiFixedList` row. Bounded today (3–4 authored modules) but unbounded in principle. It is a `Table`
   surface under active SOURCING-* peer edits — left alone deliberately (surgical-edits instruction).
4. **draw/raster pick rows keep composite keys** (`drawing-play-layers.<kind>.<id>`,
   `raster-play-layers.<segment>.<id>`) instead of the raw target id, because those keys are also the
   drag/drop payload contract read by `🚚️move-layer`/`📥️drop-layer-kind` and by
   `drawing_play_layer_id_from_tree_row_id`. Carried forward from A8 §6.2, unchanged.
5. Window laws live once per stdio crate (in the `🧱️base` editor module); the sibling subset modules
   (`🌐️iso21320`, `✅️valid`, `🛜️i-json`) and the viewers carry the shape tests only. They share the same
   `TreeWindowKit` render path, so the mechanism is covered, but not per-module.
