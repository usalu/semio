# 📓️ A8 — every remaining tree-bearing plugin, migrated onto the windowed panel-tree SDK

Packet A8 of wave 2. Inputs: `📓️wave2-app-brief.md`, `📓️design-virtualised-tree.md` §5/§8,
`📓️audit-app-panels-a.md` §9–§12, `📓️audit-app-panels-b.md` §10–§23 + cross-app + §-1.
The normative per-panel rule every file below follows is written out once in `📓️a8-migration-spec.md`
(same folder) — that file is the packet's own contract and the shape four parallel migration agents worked to.

## 1. Scope resolved

16 plugins were named. Three of them have **no tree UI at all** and are correctly skipped — confirmed by grep,
not by trusting the audit: `grep -rl 'PanelTreeBuilder|ui::tree(|tree_section|TreeWindowKit|tree_item'` returns
**zero** files in 📕️norm (all 15 artifacts, 45 panels), ➗️mathematical and 🎪️demonstrator.

The audit under-counted two plugins; both were found by inventorying trees built OUTSIDE `📌️panels`:

- **📖️playbook** is listed in `📓️audit-app-panels-b.md` §20 as having no tree. Its *viewer* has one:
  `👁️viewer/…/🪟️windows/🌳️steps/🦀️.rs` builds a `TreeWindowKit` tree of steps › blocks. Migrated.
- **🏛️architect** has **four** more tree-bearing windows beyond its three panels (`📋️register` viewer,
  `📓️report`, `🧭️trace`, `↔️adjacency` editor windows) — `🧭️trace` carried a live `.take(12)` truncation the
  audit never saw. All four migrated.

## 2. What changed, per plugin

| plugin | panels / windows migrated | nesting windowed | domain → granularity | notable deletions |
|---|---|---|---|---|
| 🎥️shooting | `🗿️artifact`, `🛍️catalogue` (`🔍️inspection` = static rows, carve-out) | flat | stays **unbound** by design (shot+asset ids share one tree under two namespaces) — rows keep `setShotSelection` / their hand-built `interactionSelect` | crate-local `ui_node_list` |
| 🖍️draw | `🗂️layers`, `🛍️catalogue` (`🔍️properties` = a bare text node) | **unbounded, both recursive kinds**: `Group` and `Boolean` children each a `tree_window_item`, via a recursive `layer_tree_item(windows, …)` | `.interaction_domain(DRAWING_PLAY_CONTROLLER_ID, DRAWING_INTERACTION_DOMAIN)` + `.granularity("stroke")` per layer row | crate-local `ui_node_list`, the `try_children` build path |
| 🖨️raster | `🗿️artifact`, `🎭️masks`, `🛍️catalogue` (`🔍️inspection` carve-out) | **unbounded** recursive `Group` | `.interaction_domain(RASTER_PLAY_CONTROLLER_ID, RASTER_INTERACTION_DOMAIN)` + `.granularity("layer")`; `🎭️masks` stays **unbound** as designed | crate-local `ui_node_list`; bare domain literals promoted to consts |
| 🌍️gis | `🗿️artifact`, `🛍️catalogue` | flat | `.interaction_domain(GIS2D_PLAY_APP_ID, GIS2D_INTERACTION_DOMAIN)` + `.granularity(...)`; **row keys are now the raw layer id**; catalogue stays unbound with `toggleLayerVisibility` | crate-local `ui_node_list`, `builder.item_id(...)` |
| 📏️layout | `🗿️artifact` (**all nine** sections), `🚦️preflight`, `🛍️catalogue` | flat (frames/layers flattened across pages) | `.interaction_domain(LAYOUT_PLAY_APP_ID, LAYOUT_INTERACTION_ELEMENTS)` + `.granularity(element)` on frame rows; page rows keep `setActivePage` | crate-local `ui_node_list`, 4 `UiFixedList` accumulators, hand-rolled empty rows |
| 📋️forms | `🗿️artifact`, `🛍️catalogue` | **2 levels** — every step row is a `tree_window_item` over its questions | `.interaction_domain(FORMS_PLAY_APP_ID, FORMS_INTERACTION_FIELDS)` + `.granularity("section"/"field")`; `.drop_action` and both draggable rows preserved | crate-local `ui_node_list`, `try_with_children` |
| 🔱️trinity (jack + rewriting) | both `🗿️artifact` + both `📚️catalogue` | flat | `.interaction_domain(<controller>, "ast"/"graph")` + `.granularity("node")`, rows keyed by raw id | 2 crate-local `ui_node_list`; catalogue tuples → const rosters |
| 💡️reasoning | `🗿️artifact` (both `section_or_placeholder`s), `🛍️catalogue` | flat | `.interaction_domain(WIRES_PLAY_APP_ID, WIRES_INTERACTION_GRAPH)` + `.granularity("node"/"edge")` | the whole per-row `fn selection_args` `{domainId, merge, method, targets}` map; crate-local `ui_node_list` |
| 🏛️architect | `🗿️artifact`, `📚️catalogue` + **4 non-panel windows** (`📋️register`, `📓️report`, `🧭️trace`, `↔️adjacency`) | 1 level in `↔️adjacency` and `📋️register` | `.interaction_domain(ARCHITECT_APP_ID, ARCHITECT_INTERACTION_PROGRAM)` + `.granularity("entity")`; register rows keep `selectRegister` | **the `.chunks(UI_FIXED_LIST_ITEMS)` "Registers 1–32/33–64/65–66" idiom** (the audit's Pattern C, now one windowed section), `🧭️trace`'s `.take(12)`, the `registerPages` fixture field + its assertion |
| 💠️lowpoly | `🗿️artifact`, `🗂️layers`, `🛍️catalogue`, `🔍️inspection` | **2 nested levels** — object › {vertex,edge,face} group › one row per raw element id | `.interaction_domain(LOWPOLY_PLAY_CONTROLLER_ID, MESH_INTERACTION_DOMAIN)` + `.granularity("object"/"vertex"/"edge"/"face")`; face rows keep their `flipFaces` `RowAction` | per-row `mesh_select_action` bindings and the fn itself; crate-local `ui_node_list` |
| 🪐️space | engine `🛍️catalogue`, engine `🔍️inspection`, engine `🔢️parameters`, artifact `👥️members` | **every depth** of the app-registry catalogue, via a recursive `fn` on `tree_window_item` | all stay unbound (mixed id namespaces) | crate-local `ui_node_list`; **branch `default_open` flipped to `false`** so the installed-app registry no longer auto-expands whole |
| 🪵️sourcing | none — its `🏊️pool`/`🧺️curated` are `SurfaceKind::Table` JSON-blob surfaces, a different mechanism | — | — | the **dead** crate-local `ui_node_list` (zero callers, verified by grep) |
| 🗄️stdio | **12 window files** (zip ×4, xml ×4, json ×4) ported onto the SDK — see §3 | **unbounded, every level** | n/a (document structure, not a pick target) | — |
| 📖️playbook | `👁️viewer/…/🌳️steps` (`TreeWindowKit`) | steps › blocks | n/a | — |
| 📕️norm, ➗️mathematical, 🎪️demonstrator | **no tree UI — confirmed and skipped** | — | — | — |

`TreeWindows::for_body(view_state, <that body key>)` is built per dispatch arm in every editor's and viewer's
`fn render(body_key, doc, cfg, view_state)` and threaded in; every migrated panel takes `windows: &TreeWindows<'_>`.

**Sections deliberately left on plain `.section(...)`** are exactly the spec §1 carve-out — a handful of
hand-written heterogeneous rows with no entry slice to window: every `🔍️inspection` panel, forms'/trinity's
2–3-row `actions` sections, architect's 3-row `meta`, and space members' 3 fixed action rows. Nothing else.

## 3. `TreeWindowKit` — the audit's "single most severe finding", closed

`📓️audit-app-panels-b.md` §23 called this a live landmine: `TreeWindowKit::render` walked `TreeView.roots` with
an explicit stack and pushed every sibling into a plain 32-slot `UiFixedList`, so **any** zip archive, JSON
array or XML element with more than 32 children made the whole editor fail to render with `ui.fixed-capacity`.

It is a genuinely different surface from a panel tree — a `WindowKit` whose trait `render(view)` takes no host
state — so I did **not** convert it to `PanelTreeBuilder`. Instead I windowed it with the same `TreeWindow`
contract, as the packet brief allows:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, region `🔖️TreeWindowKit`: the explicit-stack walk is
  replaced by `TreeWindowKit::render_windowed(view, windows)` — the root list goes through the SDK's
  `tree_window_section`, and every node through `tree_window_item` via a recursive `windowed_node` guarded by
  `COMPONENT_TREE_PRODUCER_DEPTH`. The trait's `render(view)` is now `render_windowed(view, &TreeWindows::unhosted())`,
  so nothing that composes the kit without host state changes behaviour.
- All 12 stdio window modules (`🎒️zip` base+iso21320, `📰️xml` base+valid, `🧾️json` base+i-json, each editor and
  viewer) take `windows: &TreeWindows<'_>`; all 12 dispatches thread `TreeWindows::for_body(view_state, main::BODY_KEY)`.
- 📖️playbook's steps window rides the same port.

**Residual, stated honestly:** the fan-out ceiling is gone and the render is now bounded and streamable, but the
`TreeView`/`TreeNodeView` value itself is still built eagerly by each editor before `render_windowed` sees it —
a 100 MB JSON document still materialises its whole `TreeNodeView` tree in guest memory. Windowing the *view
model* as well needs a lazy `TreeView` (a children-provider closure instead of a `Vec`), which is a separate
design change, not a packet-A8 edit.

## 4. Tests — the four window laws

Every artifact/layers/document panel in the packet carries laws (a)–(d) from the brief, driving the panel's
`render` **directly** (no app fixture) against a locally built oversized document, then projecting with
`project_and_retire_fixture_tree(built_to_component_tree(node))`. Law (d) has an "unbound tree" variant for the
surfaces that deliberately bind no domain (shooting, raster masks, gis catalogue, stdio, playbook, space,
architect `🧭️trace`): it asserts no domain, no stamped `granularity`, and that rows keep their own action.

Test modules **created from nothing** (the audit flagged these as coverage-location gaps): draw `🗂️layers`,
raster `🗿️artifact`, trinity jack `🗿️artifact`, trinity rewriting `🗿️artifact` — each with the
`//#region 🧪️Tests` + `#[cfg(test)] #[path = …] mod tests;` registration its sibling panels use.

**One law-design correction worth carrying forward.** My first cut of law (a) let the first paint materialise a
full `TREE_WINDOW_DEFAULT_ROWS` (48) viewport. Each materialised interactive row costs real `UiValue`
argument-arena credit, and the arena is **process-wide**: running beside the sibling panel tests in the same
binary, the 48-row laws starved shooting's inspection panel into a hard
`ui.fixed-capacity: fixed UI map admission failed`. The laws now report a measured viewport
(`tree_viewport_rows: Some(4)`) instead — a sharper assertion (total stays 200/300, materialised ≤ 4) at
negligible arena cost. Any later packet writing these laws should do the same rather than pin the default.

## 5. Verification

Runner: `🐚️a8-verify.sh` (this folder) — `cargo test -p <crate> [--features …]` then
`cargo check -p <crate> --target wasm32-wasip2 [--features …]`, one crate at a time so the shared
fine-grain-locked build dir is queued for once per crate. `DEVELOPER_DIR=/Library/Developer/CommandLineTools`
(Xcode-licence link gate). Never sets `CARGO_TARGET_DIR`. Per-crate output under `🗑️generated/a8/`.

<!-- VERIFICATION RESULTS -->

## 6. Open items handed on

1. **🪐️space `🔢️parameters` is bounded but cannot stamp.** That surface is not a tree: it is a raw `column()` of
   `section()` containers holding editable field rows, and `🧰️framework/🔨️modules/🖥️platform/🟦️.ts` states
   `TreeView`/`treeItemToTreeData` never renders a non-`treeItem` child as an inline row control, so converting it
   to `PanelTreeBuilder` would produce an unrenderable body. It now takes `TreeWindows::slice(...)` and builds only
   that slice (the hard fail past 32 parameters is gone), but **`TreeWindow` was added to `TreeSectionProps` and
   `TreeItemProps` only — `ContainerProps` has no `window` field**, so the host never learns `total` and files no
   requests for that body. Closing it needs either `window` on `ContainerProps` + a host reader, or a renderer for
   inline row controls inside tree rows. Wave-1/wave-3 follow-up.
2. **draw/raster pick rows keep composite keys.** Spec §3 wants a pick row keyed by the raw target id. Draw's rows
   are keyed `drawing-play-layers.<kind>.<id>` and raster's `raster-play-layers.<segment>.<id>`, because those keys
   are also the drag/drop payload contract read by `🚚️move-layer` / `📥️drop-layer-kind` and by
   `drawing_play_layer_id_from_tree_row_id` / raster's `layer_id_from_tree_row_id` — all outside this packet's file
   set. **Host-side pick synthesis will not resolve these two trees until that re-key lands.**
3. **architect `↔️adjacency` changed shape, not just paging.** It used to fan one root `.section` per matrix row;
   there is no SDK affordance to window a `PanelTreeBuilder`'s root section list, so it is now one windowed `rows`
   section with a `tree_window_item` per matrix row. Its semantic-contract test's fixed index path
   (`children[1].children[1].bindings[0]`) became a recursive `first_binding` helper. Worth a reviewer's eye.
4. **architect's *editor* register window is a `BlockListScene` surface**, not a tree — no windowing applies.
5. **Peer-owned failure in layout's `🔍️inspection`.** While this packet ran, another agent rewrote that panel from
   raw `section`/`text` onto `PanelTreeBuilder` + `tree_item_desc` (ids `layout-inspector.*` → `layout-play-inspector.*`)
   while its semantic-contract test still asserts the old shape. Not a windowing change and not touched here.
6. **The cargo gate's two report files were never written.** `📓️wave2-app-brief.md` says to wait for
   `📓️p1-contract.md` and `📓️p3-sdk.md`. Neither exists after ~45 min of polling — but the wave-1 **code** has
   landed, so before building I verified the real signatures directly against §5: `TreeWindow` +
   `TreeSectionProps.window`/`TreeItemProps.{window,granularity}` in the UI contract, `ViewModel.tree_windows`/
   `tree_viewport_rows` + `TreeWindowRequest` in `🛂️manifest`, and the whole `🔖️PanelWindowing` region
   (`TREE_WINDOW_DEFAULT_ROWS`, `TreeSlice`, `TreeWindows`, `tree_window_section{,_or_placeholder}`,
   `tree_window_item`, `ui_node_list`, `PanelTreeBuilder::window_section{,_or_placeholder}`, two-argument
   `interaction_domain`). All match §5 exactly.
