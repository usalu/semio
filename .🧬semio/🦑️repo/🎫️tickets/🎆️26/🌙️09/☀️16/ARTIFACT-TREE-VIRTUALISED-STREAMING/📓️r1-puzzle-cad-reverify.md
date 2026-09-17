# 📓️ R1 — re-verification of 🧩️puzzle 3d / 2d / 5d and 📐️cad

Packet R1 of wave 4 (📓️wave4-resume-brief.md). Re-audits and re-runs what 📓️a1-puzzle3d.md,
📓️a2-puzzle2d-5d.md and 📓️a3-cad.md reported green ~7 h earlier, against the SDK as it stands now
(F2's body-wide node ledger in `TreeWindows`) and after peer churn.

Crates: `semio-s-artifact-puzzle-3d`, `semio-s-artifact-puzzle-2d`, `semio-s-artifact-puzzle-5d`
(all `--features component-app-assembly`) and `semio-s-artifact-cad-cad` (no extra feature).

> ⏳️ VERIFICATION SECTION IS BEING FILLED IN — see §4.

## 1. SDK state this packet was verified against

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` region `🔖️PanelWindowing` (lines ~5923-6139) now
carries F2's ledger:

- `TREE_WINDOW_FIXED_NODE_HEADROOM: usize = 16`;
- `TreeWindows { requests, budget: Cell<u32>, nodes: Cell<usize> }` with
  `LEDGER = UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM` (= 128 − 1 − 16 = **111**),
  `nodes_remaining()`, private `charge(n)`;
- `tree_window_section` charges 1 for the section node, `tree_window_section_or_placeholder` charges 2
  on the empty path, `tree_window_item` charges 1 for the group item, and `slice()` clamps `len` to the
  ledger remainder for a host request exactly as for a first-paint default.

Live contract constants: `UI_DOCUMENT_NODES = 128`, `UI_BUILT_CHILDREN_MAX = UI_DOCUMENT_NODES`,
`UI_VALUE_PAGE_ROWS = UI_BUILT_CHILDREN_MAX` — the bisect P3 flagged is still reverted.

`cargo check -p semio-framework-plugin` → **Finished `dev` in 17.40s, 0 errors** (40 warnings), so F2's
region compiles as this packet's app work was written against it.

## 2. Completeness audit — every Tree in every panel of the four apps

### 2.1 Panel inventory (production code, `🧪️tests` excluded)

| app | body | panel file | containers | windowed? |
|---|---|---|---|---|
| puzzle-3d | `puzzle.3d.play.artifact` | `…/🧊️3d/…/📌️panels/🗿️artifact/🦀️.rs` | 4 sections (`objects`/`references`/`target-volumes`/`attractions`) + one nested vortex group per object row | ✅ `window_section` ×4, `tree_window_item` for vortices |
| puzzle-3d | `puzzle.3d.play.catalogue` | `…/📌️panels/🛍️catalogue/🦀️.rs` | 4 sections (`objects`/`vortices`/`cables`/`attractions`) + one nested rim-template group per object-kind row | ✅ `window_section` ×4, `tree_window_item` for templates |
| puzzle-3d | `puzzle.3d.play.inspector` | `…/📌️panels/🔍️inspection/🦀️.rs` | selected-id list (`IDS_SECTION`) + ONE entity field group (≤10 author-fixed rows) or the 3-row summary | ✅ ids list `window_section`; field groups are fixed lists inside the headroom |
| puzzle-3d | `puzzle.3d.play.settings` | `…/📌️panels/⚙️settings/🦀️.rs` | no list — fixed stepper/toggle controls (69 lines) | n/a, uses the SDK `ui_node_list` |
| puzzle-2d | `puzzle2d.play.layers` | `…/◻️2d/…/📌️panels/🗿️artifact/🦀️.rs` | 2 sections (`nodes`/`edges`) | ✅ `window_section_or_placeholder` ×2 |
| puzzle-2d | `puzzle2d.play.catalogue` | `…/📌️panels/🛍️catalogue/🦀️.rs` | 3 sections (`nodes`/`handles`/`edges`) | ✅ `window_section_or_placeholder` ×3 |
| puzzle-2d | `puzzle2d.play.properties` | `…/📌️panels/🔍️inspection/🦀️.rs` | selected-id list (`IDS_SECTION`, only when >1 id) + one field group (node 12 / edge 6 / handle 5 rows) or the 4-row summary | ✅ ids list `window_section`; field groups fixed and inside the headroom |
| puzzle-2d | `puzzle2d.play.settings` | `…/📌️panels/⚙️settings/🦀️.rs` | no list (77 lines) | n/a |
| puzzle-5d | `puzzle.5d.play.artifact` | `…/🖐️5d/…/📌️panels/🗿️artifact/🦀️.rs` | 2 sections (`parts`/`fasteners`) + one nested grip group per part row | ✅ `window_section_or_placeholder` ×2, `tree_window_item` for grips |
| puzzle-5d | `puzzle.5d.play.catalogue` | `…/📌️panels/🛍️catalogue/🦀️.rs` | 4 sections (`parts`/`grips`/`fasteners`/`ropes`) | ✅ `window_section_or_placeholder` ×4 |
| puzzle-5d | `puzzle.5d.play.inspector` | `…/📌️panels/🔍️inspection/🦀️.rs` | 4 fixed summary rows, no entity list at all | n/a (SDK `ui_node_list`, inside the headroom) |
| cad | `cad.play.artifact` | `…/📐️cad/…/📌️panels/🗿️artifact/🦀️.rs` | 9 sections (4 pane object sections + 4 per-pane reference sections + `nodes`) + one nested primitive group per object row | ✅ `window_section` ×5, `window_section_or_placeholder` ×4, `tree_window_item` for primitives |
| cad | `cad.play.catalogue` | `…/📌️panels/🛍️catalogue/🦀️.rs` | 1 section (`typologies` over `TYPOLOGY_CATALOG`) | ✅ `window_section` |
| cad | `cad.play.properties` | `…/📌️panels/🔍️inspection/🦀️.rs` | the selected-object block (grows with the selection) + reference (13 rows) / node (2 rows) / summary (3 rows) fixed groups | ✅ selected-object block `window_section`; the three fixed groups are inside the headroom |

Every app builds ONE `TreeWindows::for_body(view_state, <body key>)` per render, in its single render
funnel: `Puzzle3dPlayApp::render_body` (`…/🧊️3d/…/✏️editor/🦀️.rs:8285-8287`),
`Puzzle2dPlayApp::render_body` (`…/◻️2d/…/✏️editor/🦀️.rs:1219`), `Puzzle5dPlayApp::render` **and**
`render_with_request_context` (`…/🖐️5d/…/✏️editor/🦀️.rs:7909` / `:7939`) and `CadPlayApp::render_body`
(`…/📐️cad/…/✏️editor/🦀️.rs:1311-1315`).

**No tree lives outside these panel modules.** `grep -a` for `PanelTreeBuilder|tree_section(` over each
app's whole `✏️editor` tree returns hits only under `📌️panels` (cad's two extra hits are the
`cad_tree_item`/`cad_tree_item_static` row helpers the panels call). `🎭️modes` carries no tree in either
cad or puzzle-3d. None of the four apps calls `ui_history_panel` — the history body is the framework's,
migrated by P3; the apps only name `FRAMEWORK_HISTORY_BODY_KEY` in dirty scopes.

### 2.2 Residue greps (all `grep -a`, `dist`/`node_modules`/`target` excluded)

Over `✏️s/🔌️plugins/🧩️puzzle` and `✏️s/🔌️plugins/📐️cad`, all file types (Rust, `🧬️schema/{🔣️.json,🔗️.graphql,🟦️.ts,🛰️.proto}`,
plugin manifests `🔣️.json`, fixtures, `📦️packages/🟦️typescript`):

| pattern | hits |
|---|---|
| `setPanelPage` / `set_panel_page` / `panel_pages` / `panelPages` | **2**, both doc comments in new tests naming the deleted behaviour (`🧊️3d/…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:198`, `🧊️3d/…/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs:121`) |
| `SECTION_ROWS` / `IDS_ROWS` / `PANEL_PAGE_GUARD` / `PANEL_RECONCILE_NODE_BUDGET` / `CATALOGUE_GROUP_ROWS` / `LIST_ROWS_MAX` / `HISTORY_COMMAND_ROWS` | **1**, a doc comment (`◻️2d/…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:5`) |
| `paged_section` / `paged_panel_section` / `panel_page_rows` / `PanelRowBudget` / `panel_continuation_row` / `section_page` | 0 |
| `.more` / `"+` / `continuation` in PRODUCTION panel code | 0 — every hit is a test assertion (`assert!(!json.contains(".more"))`) or a doc comment |
| `.take(N)` / `.truncate(N)` list truncation in panel code | 0 (the only `.take()`s are `Option::take` in owner/close paths) |
| `fn ui_node_list` (crate-local copy) | 0 — every panel imports the SDK's |

Manifest/fixture cross-checks re-run mechanically (not from the earlier reports):

- cad `🧫️fixtures/🗄️retained-jobs/🔣️.json`: `routeCount 37`, `routes[] 37`, `admittedRoutes 31`,
  `activation.proofRows 31`; Rust `CAD_RETAINED_TOOL_IDS` = **31**; `setPanelPage` absent from
  `…/✏️editor/🦀️.rs`. cad TS `📜️script.ts` pins `=== 37` twice and its `expectedAdmitted` no longer lists
  `setPanelPage`.
- puzzle `🗄️retained-jobs` `toolIds`: 3d 63, 2d 36, 5d 16 — none names `setPanelPage`; the shared
  `🧫️fixtures/🔏️publication-authority/🔣️.json` does not either.
- puzzle-3d/2d window+config `🧬️schema/{🔣️.json,🔗️.graphql,🟦️.ts,🛰️.proto}` and the cad config schema carry
  zero `panelPages`/`panel_pages`.

### 2.3 The one residue the audit DID find, and its fix

**cad's inspector keyed its selected-id rows by list index, and mixed them into the object's own field
window.** `…/📐️cad/…/📌️panels/🔍️inspection/🦀️.rs::selected_object_section` built ONE windowed section
`cad-play-inspector.object` whose entries were `ids.{index}` rows followed by the nine object field
rows:

```rust
let mut rows: Vec<(String, String, String)> = selected.iter().enumerate()
    .map(|(index, (_, selected))| (format!("ids.{index}"), …, selected.id.clone())).collect();
rows.push(("object.label".into(), …));   // … eight more
… .window_section(windows, &format!("{ROOT}.object"), …, &rows, …)
```

Two defects, both the ones 📓️wave2-app-brief.md §2/§4 and 📓️a1-puzzle3d.md §1 call out explicitly for
every other app's id list:

1. **Index keys, not raw ids.** puzzle-3d and puzzle-2d both key their ids rows `{IDS_SECTION}.{id}`
   precisely so the host can address a row across a selection change; cad's `ids.{index}` renames every
   row below a removed id.
2. **The object's fields were the TAIL of the window.** With a selection wider than the slice, the nine
   field rows — the whole point of the inspector — are never materialised, and the section's `total`
   was `selection + 9` rather than the selection's extent.

Fixed the same way the other three apps do it: a windowed section of its own
(`IDS_SECTION = "cad-play-inspector.ids"`, new `pub const`) keyed by the RAW object id, followed by the
nine field rows as a plain author-fixed `.section` (inside `TREE_WINDOW_FIXED_NODE_HEADROOM`). New law
`a_wide_selection_windows_its_ids_by_raw_id_without_starving_the_field_group` in
`…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` pins both halves.

Apart from that, a1/a2/a3's deletions all survived the peer churn of the last seven hours, and no Tree
in any of the four apps is unwindowed.

## 3. What this packet changed

One new "whole document" law per app in each app's outliner test file, plus the cad inspector fix of
§2.3 (the only production edit this packet made) and its law.

| file | law added |
|---|---|
| `…/🧊️3d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | `a_whole_open_document_stamps_every_total_and_stays_inside_the_body_node_ceiling` (+ a `node_records` helper) |
| `…/◻️2d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | same name (+ `node_records`) |
| `…/🖐️5d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | same name (+ `node_records`) |
| `…/📐️cad/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | same name (+ `node_records`) |
| `…/📐️cad/…/📌️panels/🔍️inspection/🦀️.rs` | **production**: `IDS_SECTION` const; `selected_object_section` splits into a raw-id-keyed windowed ids section + a fixed nine-row field section (§2.3) |
| `…/📐️cad/…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | `a_wide_selection_windows_its_ids_by_raw_id_without_starving_the_field_group` |

Shape of the law, per app: one container holds **more than `UI_DOCUMENT_NODES` entries**
(3d: 200 objects; 2d: 200 nodes + 168 edges; 5d: 200 parts + 144 fasteners; cad: 300 + forest nodes),
**every** container in the body is opened at once by an explicit `TreeWindowRequest{open: Some(true),
rows: 512}` (3d also opens 24 object groups, 5d 24 part groups, cad every pane section, every reference
section and every object group), and the law then asserts

1. every container still stamps `window.total == entries.len()` — including the ones the exhausted
   ledger could seat no rows in;
2. `node_records(body) <= UI_DOCUMENT_NODES` — the same counting rule as
   `SurfaceReconcileLimits::max_nodes` and as the SDK's own `body_nodes` helper;
3. the projected body contains no `.more` key and no `"+` label.

This is the app-level mirror of the SDK law `nine_windowed_sections_share_one_body_wide_node_ledger`.

## 4. Verification

⏳️ pending — filled in below once the runs complete.
