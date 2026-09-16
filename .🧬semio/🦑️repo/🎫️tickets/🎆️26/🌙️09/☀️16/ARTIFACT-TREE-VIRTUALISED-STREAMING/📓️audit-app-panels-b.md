# Audit: plugin panel tree builders and paging shapes — set B

Read-only investigation. All paths relative to repo root `/Users/ueli/Documents/semio`. Paths verified by direct `find`/`grep`/`Read`, not recalled.

Scope (per ticket assignment): 📕️norm (all 15 artifacts), 🌊️flow, 🕸️dag, 📏️layout, 📋️forms, ✒️writer, 🗒️note, 📸️remodel, 🌿️vcs, 🔱️trinity (jack + rewriting), 💡️reasoning, 🎬️sequence, 🎞️animate, 🏛️architect, 💠️lowpoly, 📜️imperative, 🪵️sourcing, ➗️mathematical, 🪐️space, 🎪️demonstrator, 📖️playbook, 🗄️stdio, 🗟️artifacts. Excluded (other auditor): 🧩️puzzle, 📐️cad, 🏗️fem, 🔋️energy, 🌀️procedural, 🏭️process, 🧱️block, 🎥️shooting, 🖍️draw, 🖨️raster, 🌍️gis.

## 0. Framework SDK primitives every panel funnels through (or doesn't)

All defined in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` unless noted.

**Row/tree builders (region `🔖️PanelKit`, ~5680–5991):**
- `tree_item(id, label)` (5681), `tree_item_desc(id, label, description)` (5688) — thin wrappers over `ui::tree_item`.
- `tree_group(id, label, default_open, children)` (5701) — non-interactive grouping row, no action/argument cost.
- `tree_item_with_action(id, label, description, (action, args))` (5708), `tree_item_with_action_draggable(..., drag_data)` (5727).
- `selection_ids(args)` (5761) — parses a selection action's `ids` array.
- **`PanelTreeBuilder`** (5784–5872) — the fluent skeleton almost every in-scope panel uses: `::new(namespace)`, `.section(id, label, default_open, items)`, `.section_or_placeholder(id, label, default_open, items, placeholder_label)` (substitutes one "(none)" item when `items` is empty), `.selected(ids)`/`.highlighted(ids)` (recorded but **not currently rendered** — doc comment at 5772–5783 explains presence moved to a separate `PresenceUpdate` channel this SDK layer can't publish yet), `.interaction_domain(id)`, `.drop_action(action)`, terminal `.build()`.
- Raw `ui::tree()` / `ui::tree_section()` / `ui::tree_item()` — defined in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs:1369/1408/1454`; used directly by a handful of inspection panels that don't need `PanelTreeBuilder`'s section/placeholder sugar.
- `FormPanelBuilder` (~5999+) — sibling namespaced builder for `Section > labeled Field rows > submit Button` form panels. **Zero usage found** anywhere in this audit's 23 plugins (`grep -rl FormPanelBuilder` across all of them → no hits), despite existing specifically to de-duplicate a pattern the doc comment says is "duplicated across plugin crates."

**Paging/virtualisation primitives (region `🔖️PanelPaging`, 5874–5987) — the mechanism this ticket is about replacing:**
- `UI_BUILT_CHILDREN_MAX = 32` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs:51`) / `UI_FIXED_LIST_ITEMS = 32` (`…/🎬️action/🦀️.rs:21`) — hard per-node child cap; every `UiFixedList::try_push` past the 32nd item returns `PluginAssemblyError::new("ui.fixed-capacity", …)`.
- `UI_VALUE_PAGE_ROWS = UI_BUILT_CHILDREN_MAX - 1 = 31` (`…/🎬️action/🦀️.rs:54`) — the "31 rows/section" ceiling named in the ticket.
- `UI_VALUE_ROW_COLLECTIONS`/`UI_VALUE_ROW_ITEMS`/`UI_VALUE_ADMISSION_SLOTS`/`UI_VALUE_AGGREGATE_ITEMS` (…/🎬️action/🦀️.rs:39,42,65,66) — the process-wide `UiValue` argument-arena costs one interactive row spends.
- `panel_page_rows()` (5888) = `UI_VALUE_PAGE_ROWS.min(ui_value_headroom().rows())` — the actual per-render row ceiling once other panels' live pages are accounted for.
- `PanelRowBudget` (5893–5928) — `.new(rows)`, `.spend()`, `.remaining()`, `.nested(reserved, build)` (lets a section reserve rows for siblings before it hands its remainder to a nested build — used by the framework's own history panel, not by any in-scope plugin).
- `panel_continuation_row(section_id, omitted)` (5933) — emits the `+{omitted}` row (id `{section_id}.more`, icon `more-horizontal`).
- `paged_panel_section(section_id, entries, section_rows, budget, row_fn)` (5952) — the actual "+N" builder: pages `entries` against `section_rows` and the shared `budget`, appending a `panel_continuation_row` for anything left over.
- **Important nuance directly confirmed by reading the code**: `panel_continuation_row` (5933-5941) itself builds a **bare, inert** `ui::tree_item(Label("+{omitted}"))` with an icon and an id — it attaches **no `Trigger` binding and no `RowAction` at all**. Read literally, the framework's own shared helper is not clickable. Every real consumer that wants a *working* "+N" (i.e. one that can actually reach the next page) hand-builds its own action-bearing row instead of calling this helper verbatim: `ui_history_panel` (below) constructs its own `more` tree_item with a `RowAction` wired to `SET_HISTORY_COMMAND_FILTER_ACTION_ID` + an incremented `page` arg (10229-10233), and the excluded 📐️cad plugin wraps the pattern in its own `cad_tree_item(..., cad_action("setPanelPage", Some(args)))` (`✏️s/🔌️plugins/📐️cad/…/📌️panels/🗿️artifact/🦀️.rs:163`) rather than calling `panel_continuation_row` directly. There is **no framework-level `setPanelPage` action or page-cursor type** — `grep -rn "setPanelPage\|panel_page_index\|PanelPageState"` across the whole framework returns zero hits; `setPanelPage` is a per-plugin action name/convention (`cad`, `puzzle`, `fem`, `energy`), not a framework primitive.
- **The only real caller of the shared `paged_panel_section`/`panel_continuation_row` functions inside the framework itself is `ui_history_panel`** (plugin/🦀️.rs:10139–10260, the generic undo/redo/commit-history panel every app gets for free) — `HISTORY_COMMAND_ROWS: usize = 16` (10117), a genuine `command_page: u32` cursor threaded in from `self.history_page` on the app struct (10139, 29955). `pub use app::{paged_panel_section, panel_continuation_row, panel_page_rows}` (39485) re-exports these for plugin use, but **grep across this entire 23-plugin scope for `paged_panel_section|panel_continuation_row|PanelRowBudget|setPanelPage` returns zero hits** — none of the 23 plugins in this audit call it. The `setPanelPage`-action idiom is real and fully worked, but only in the **excluded** 📐️cad/🧩️puzzle/🏗️fem/🔋️energy/🌀️procedural plugins (confirmed consumers: `📐️cad/…/📌️panels/🗿️artifact/🦀️.rs` + `🎮️commands/📄️panel/🦀️.rs`, `🌀️procedural/…generation3d…/📌️panels/🛍️catalogue/🦀️.rs`, `🧩️puzzle/…{🧊️3d,◻️2d}…/📌️panels/🗿️artifact/🦀️.rs`, `🏗️fem/…{🧊️3d,◻️2d}…/📌️panels/{🗿️artifact,🔍️inspection}/🦀️.rs`, `🔋️energy/…/📌️panels/🗿️artifact/🦀️.rs` — out of this audit's scope, for the other auditor).
- **A fourth, entirely separate tree-building primitive exists**: `TreeWindowKit` (framework `plugin/🦀️.rs:30897-30985`, `TreeView`/`TreeNodeView` types), used by a handful of 🗄️stdio format editors (see §22) for recursive document structures (zip entries, XML/JSON trees). `TreeWindowKit::render` walks `TreeView.roots` with an explicit stack and pushes each node's children via `parent.built.try_push(item)` into a **plain, uncapped-by-any-paging `UiFixedList` (cap 32)** — it does **not** call `paged_panel_section` either. This is the most severe unpaged-tree finding in the whole audit; see §22.
- **A fifth primitive, `TableWindowKit`**, renders `TableView{columns, rows}` by serializing the **entire** row set straight to a JSON string (`TableScene::base`), bypassing `UiFixedList`/`BuiltNode` admission (and therefore the 32-item cap) entirely — no hard failure on overflow, but also zero windowing/paging of any kind. Used by 🗄️stdio's csv/tsv/xlsx/epw/bcf-editor variants and by 🪵️sourcing's pool/curated panels (§21).
- `PanelGroup` enum (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3333`): `Workbench|Details|Display|Settings`, each anchoring to one of the four dock corners — this is the "panel kind" axis in the tables below (artifact tree/catalogue → `Workbench`; inspection → `Details` typically).

**The dominant unpaged idiom actually found across this scope**: a per-plugin, copy-pasted `ui_node_list(values) -> UiFixedList<BuiltNode>` helper that does nothing but `try_push` every value with no cap, no `.take()`, no continuation row — overflow past 32 items is a hard `PluginAssemblyError` (render failure), not a graceful "+N". Found independently defined **35 times** across the plugins tree (`grep -rn "fn ui_node_list" ✏️s/🔌️plugins --include="*.rs" | wc -l` → 35; in this audit's scope alone: ✒️writer, 🌊️flow, 🌿️vcs, 🎞️animate, 🎬️sequence, 🏛️architect, 💠️lowpoly, 💡️reasoning, 📋️forms, 📏️layout, 📜️imperative, 📸️remodel, 🔱️trinity, 🕸️dag, 🪐️space, 🪵️sourcing — the last one dead code, never called). See §6 (norm) for the one plugin in scope with **no tree UI at all**, and §7 for plugins whose editors don't build a tree by any mechanism.

---

## 1. ✒️writer (artifact: ✒️writer)

Base: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Bindings | Paging | Growth risk |
|---|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 84 | `WRITER_PLAY_BODY_ARTIFACT` | artifact tree (jack AST outline) | `PanelTreeBuilder.section_or_placeholder` + raw `ui::tree_item`, **self-recursive** `jack_ast_to_tree_item` (line 52) | 1 section; recursive AST nesting via `try_children`; `default_open` true only for `query\|match\|pattern\|return` node kinds | `.interaction_domain("ast")`, no per-row Activate action, no drag | none | Medium — AST width/depth is per-query, typically small, but nothing caps a wide node |
| `🔍️inspection/🦀️.rs` | 51 | `WRITER_PLAY_BODY_INSPECTION` | inspection | `PanelTreeBuilder` + `tree_item` | ≤2 sections ("document" fixed, "diagnostics" conditional) | none | app-local `.take(8)` on lint diagnostics (line 37) — a cap with **no "+N" indicator at all**, extra diagnostics are silently dropped | Low |
| `🛍️catalogue/🦀️.rs` | 33 | `WRITER_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder` + `tree_item` | 1 section, 1 hand-authored static item | none | n/a | None |

Tests: `🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`, `🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`, `🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` — none assert row/page counts (one asserts tab `children.len() == 2`, unrelated to row paging).

## 2. 🌊️flow (artifact: 🌊️flow)

Base: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Bindings | Paging | Growth risk |
|---|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 54 | `FLOW_PLAY_BODY_ARTIFACT` | artifact tree (widgets+synapses) | `PanelTreeBuilder.section_or_placeholder` ×2 | 2 flat sections ("widgets" open, "synapses" closed), no nesting | no per-row action; `.interaction_domain(FLOW_INTERACTION_GRAPH)` drives select/hover generically | **none** — `ui_node_list(live.widgets…)`/`ui_node_list(live.synapses…)` (`✏️editor/🦀️.rs:119-125`) push unbounded | **High** — node-graph widgets/synapses are open-ended and user-growable; >32 of either hard-fails the render |
| `🛍️catalogue/🦀️.rs` | 112 | `FLOW_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder`, dynamic sections from `host.catalogue_json()` | N sections (registered widget/operator kinds) + installed-extension section(s) | `tree_item_with_action_draggable` → `addWidget` (MIME `application/x-flow-widget`); extension rows → `toggleExtension`/`runExtensionAction` | none | Low-medium — bounded by registered kinds, not document size |
| `🔍️inspection/🦀️.rs` | 45 | `FLOW_PLAY_BODY_INSPECTOR` | inspection | `PanelTreeBuilder.section_or_placeholder`, always empty (`UiFixedList::default()`) | 1 section, always the empty placeholder — selection view is a documented SDK gap (lines 29-35) | none | n/a | None (currently dead) |

Tests: 3 unit test files, none with paging/row-count assertions.

## 3. 🌿️vcs (artifact: 🌿️vcs)

Base: `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/` (only `artifact` + `inspection` — no `catalogue`)

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Bindings | Paging | Growth risk |
|---|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 81 | `VCS_PLAY_BODY_ARTIFACT` | artifact tree (checkpoints+alternatives) | `PanelTreeBuilder.section_or_placeholder` + `.section` | 2 flat sections: "checkpoints" (`history.columns.iter().rev()`, all, open) + "alternatives" (deduped ids, open) | `tree_item_with_action` → `checkoutCheckpoint{id}` / `switchAlternative{id}` | **none** — `ui_node_list(history.columns…)` (line 38) unbounded | **High** — VCS commit history is unbounded by construction; the single clearest "growth is guaranteed, no cap" case in the audit |
| `🔍️inspection/🦀️.rs` | 62 | `VCS_PLAY_BODY_INSPECTION` | inspection | `PanelTreeBuilder.section` | 1 section, 5 fixed metadata fields | `Trigger::Change` → `patchSnapshot{field}` | n/a | None |

Tests: 2 unit test files; `document_lists_checkpoints` only asserts substring presence, not a count.

## 4. 🎞️animate (artifact: 🎬️presentation)

Base: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Bindings | Paging | Growth risk |
|---|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 52 | `PRESENTATION_PLAY_BODY_ARTIFACT` | artifact tree (deck tiles) | `PanelTreeBuilder.section_or_placeholder` + a locally re-implemented `ui_node_list` (lines 27-33) | 1 flat "tiles" section | `.interaction_domain(PRESENTATION_INTERACTION_DOMAIN)`, no per-row action | none | Medium-high — a real deck's tile count can exceed 32 |
| `🔍️inspection/🦀️.rs` | 52 | `PRESENTATION_PLAY_BODY_DETAILS` | inspection ("other", not a tree) | raw `column`/`section`/`field` | 1 section, 2 fixed fields (schema, tile_count summary) — per-tile editing is a documented removed capability | none | n/a | None |
| `🛍️catalogue/🦀️.rs` | 76 | `PRESENTATION_PLAY_BODY_CATALOGUE` | catalogue ("other") | raw `column`/`section`/`button`/`input` | 2 sections, 8 fixed rows total | `Activate` → `seedGrid`/`addTile`/`clearTiles`/`setSource` | n/a | None |

Tests: 4 files; a semantic-contract test cross-checks `tiles.len()` against a layer count, not a page cap.

## 5. 🎬️sequence (artifact: 🎬️sequence)

Base: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Bindings | Paging | Growth risk |
|---|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 103 | `SEQUENCE_PLAY_BODY_ARTIFACT` | artifact tree (steps+edges, **recursive control-flow slots**) | `PanelTreeBuilder.section_or_placeholder` ×2 + self-recursive `build_step_tree_item` (line 45) | 2 top sections ("steps" open, "edges" closed); **real recursion**: each control-flow step (`if`/`while`) gets one nested tree_item per slot (line 66-67, `default_open(true)`), holding a `Change`-bound collapse `toggle` + that slot's child steps, itself recursing | toggle → `setStepCollapsed{id}`; steps use `.interaction_domain(SEQUENCE_INTERACTION_STEPS)` for select | **none at any level** — top-level and every nested slot's `ui_node_list` push unbounded, independently | **High, 3 independent axes**: total steps, total edges, and per-slot child-step count at every depth — the most structurally complex unpaged case in the audit |
| `🔍️inspection/🦀️.rs` | 68 | `SEQUENCE_PLAY_BODY_INSPECTOR` | inspection | `PanelTreeBuilder.section` | 1 section, ≤3 fixed fields for first selected step | none | n/a | None |
| `🛍️catalogue/🦀️.rs` | 56 | `SEQUENCE_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder.section` | 1 section: 5 fixed action rows **+** one row per (control-flow-step × slot) pair, sharing one `UiFixedList` | `tree_item_with_action` → `addStep{kind}` / `addStepToSlot{...}` | none | Medium — scales with control-step×slot count, competing with the 5 fixed rows for the same 32-slot cap |

Tests: 5 files across the 3 panels; none assert row/page counts.

## 6. 🕸️dag (artifact: 🕸️dag, crate `semio_framework_artifact_infinite_dag`)

Base: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Bindings | Paging | Growth risk |
|---|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 59 | `DAG_PLAY_BODY_ARTIFACT` | artifact tree (nodes+edges) | `PanelTreeBuilder.section_or_placeholder` ×2 | 2 flat sections ("nodes" open, "edges" closed) | `.interaction_domain(DAG_PLAY_INTERACTION_DOMAIN)`, no per-row action | none — `ui_node_list(scene.nodes…)`/`ui_node_list(scene.edges…)` unbounded | **Highest in the whole audit** — the artifact crate is literally named "infinite DAG"; node/edge count is unbounded by definition |
| `🔍️inspection/🦀️.rs` | 130 | `DAG_PLAY_BODY_INSPECTOR` | inspection ("other") | raw `column`/`section`/`field` | ≤2 groups (conditional "slider" group + "base" group) | `Change` → `patchDagNodes{nodeIds,field}` (multi-select-aware); `renameDagNode` when exactly 1 selected | n/a | None — one field editor regardless of selection size |
| `🛍️catalogue/🦀️.rs` | 47 | `DAG_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder.section` | 1 section, 6 fixed node-kind rows | `tree_item_with_action` → `addNode{kind}` | n/a | None |

Tests: 3 files, none assert row/page counts.

## 7. 🗒️note (artifact: 🗒️note)

Base: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Bindings | Paging | Growth risk |
|---|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 96 | `NOTE_PLAY_BODY_ARTIFACT` | artifact tree (all blocks, recursive groups + quick-add rows) | `PanelTreeBuilder.section` + local `fixed_nodes` helper (52-58) + raw `tree_item`/`tree_item_desc`/`tree_item_with_action` | **1 single section**: 5 fixed "Add {Text,Table,Math,Image,Group}" rows **followed by** every top-level block, **sharing the same `UiFixedList`**; `NoteBlockNode::Group` recurses via self-recursive `block_tree_item` (34-36) | Add-rows → `addBlock{kind}`; block rows: `.interaction_domain(NOTE_INTERACTION_BLOCKS)`, **`draggable = Some(true)`** on every block row, `dimmed`/`default_open` per block state | **none**, and uniquely worst: fixed chrome (5 add-buttons) competes with content for the same 32-slot budget, so the practical content threshold is lower than 32; every nested Group level independently faces the same cap | **High** — blocks (and each Group's children) are open-ended and user-growable |
| `🔍️inspection/🦀️.rs` | 63 | `NOTE_PLAY_BODY_PROPERTIES` | inspection ("other") | raw `section`/`text` | 1 section, 4 fixed summary fields (incl. `flatten_blocks(&doc).len()` as a *count*, not a list) | none | n/a | None |
| `🛍️catalogue/🦀️.rs` | 43 | `NOTE_PLAY_BODY_CATALOGUE` | catalogue | `PanelTreeBuilder.section` | 1 section, 6 fixed static reference rows | none (read-only) | n/a | None |

Note: the ticket brief mentions "backlinks" for note — no such concept exists in this codebase (`grep -rl backlink ✏️s/🔌️plugins/🗒️note` → no hits); the model is recursively-nested blocks only.

Tests: 6 files; the artifact-panel test only asserts the 5 "Add X" labels + placeholder text render, not a row count.

---

## 8. 📸️remodel (artifact: 📸️remodeling)

Base: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/` — **7 panels**, the most of any single-artifact plugin in scope.

| Panel | LOC | BODY_KEY | Kind/Group | Sections | Row actions/drag/domain | Overflow risk |
|---|---|---|---|---|---|---|
| `⚙️parameters/🦀️.rs` | 68 | `remodeling.play.parameters` | App, Details | 1, fixed 7 rows | none | never (closed 8-subgroup set) |
| `✅️quality/🦀️.rs` | 56 | `remodeling.play.qc` | App, Settings | 1 | none | **yes** — ~11 fixed rows + unbounded `qc.warnings` loop (line 42-44) |
| `🎯️calibration/🦀️.rs` | 49 | `remodeling.play.calibration` | App, Details | 2 (cameras, gcps) | none | **yes** — unbounded `scene.calibration.cameras` (29-31) and `scene.gcps` (33-35); a real photogrammetry rig can have dozens |
| `🏃️tracks/🦀️.rs` | 41 | `remodeling.play.tracks` | App, Details | 1 | none | unbounded `scene.results.tracks` loop (28-30), but motion tracking is gated by an unused `self.motion_enabled` flag so currently ≤2 rows in practice |
| `🗂️media/🦀️.rs` | 49 | `remodeling.play.media` | App, Workbench | 1 | **panel-level `.drop_action(...)`** → `Trigger::Drop` → `importFramePayload` (line 41); no per-row action | unbounded `scene.streams` loop (27-38); multi-camera import could realistically exceed 31 |
| `🗿️artifact/🦀️.rs` (pipeline tab) | 87 | `remodeling.play.pipeline` (+ nested framework `Run` panel tab, lines 25-41) | App, Workbench; **nested `children` tabs** | 1, fixed 7 rows (2 status + 5 keybinding) | none | never — fixed |
| `🧵️results/🦀️.rs` | 48 | `remodeling.play.results` | App, Workbench | 1, fixed 5 rows | none | never — fixed |

All 7 build rows through the shared `crate::editor::remodeling::ui_node_list` helper (`✏️editor/🦀️.rs:95-102`) pushing into a default `UiFixedList` (cap 32); overflow is `Err(PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))` — a hard render failure. **None ever calls `paged_panel_section`/`panel_continuation_row`; none sets `.interaction_domain(...)`.** Tests exist under each panel's `🧪️tests/🔬️unit/🦀️.rs` (7 files); grep for `ROWS|\.take(|"more"|_MAX|page` — **zero hits**, no paging tests anywhere, none exercising the 32-row ceiling.

**Real risk**: `✅️quality` (QC warnings) and `🎯️calibration` (cameras/GCPs) are the two most likely to exceed 31 rows in a real remodeling/photogrammetry project; `🗂️media` (source streams) close behind for multi-camera imports.

## 9. 📜️imperative (artifact: 📜️procedure)

Base: `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind/Group | Sections | Row actions/domain | Paging | Growth risk |
|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 64 | `imperative.play.artifact` | App(Artifact), Workbench | 1 (`.section_or_placeholder`, open) | no per-row action; **`.interaction_domain(IMPERATIVE_INTERACTION_STEPS)`** (line 55) drives select | **none** — local `ui_node_list` (37-43) pushes one `tree_item_desc` per `path.steps` entry into a default `UiFixedList` (cap 32) | **High and concrete**: a procedure with ≥33 steps makes `render()` return `Err("ui.fixed-capacity", "imperative step admission failed")` — a hard render failure, not a "+N" row. The single most actionable finding for this plugin. |
| `🔍️inspection/🦀️.rs` | 48 | `imperative.play.inspection` | App(Inspection), Details | 1 fixed summary field (`imperative-play-inspector.steps`, 33-36) | none | n/a | None — doc comment (24-30) documents a deliberately-reduced per-selected-step field group (framework selection gap) |
| `🛍️catalogue/🦀️.rs` | 39 | `imperative.play.catalogue` | App(Catalogue), Workbench | 1, closed set of 5 built-in step kinds (`state.set`, `log.print`, `control.if`, `control.while`, `math.add`, lines 26-37) | `tree_item_with_action` → `addStep{kind}` via `ActionFactory` | n/a | None — never exceeds 31 |

Tests: `🗿️artifact/🧪️tests/🔬️unit`, `🔍️inspection/🧪️tests/🔬️unit` + `…/🔬️semantic-contract` — grep for "more"/"page"/`*_ROWS` finds only a false-positive "anymore" substring in a doc comment (`🗿️artifact/🦀️.rs:49`). **No test pins row/paging counts, and none exercises >32 steps.**

---

## 10. 🏛️architect (artifact: 🏛️program)

Base: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Paging | Growth risk |
|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 73 | `architect.document` | artifact tree | `PanelTreeBuilder` — sections: `meta` (3 fixed rows), `registers.{page}` (chunked, see paging), `elements` (`section_or_placeholder`, flat) | rows bound to `program` interaction domain; register rows carry `Activate` → `selectRegister` | **App-local chunking-into-sections**: `for (page, registers) in summary.by_register.chunks(UI_FIXED_LIST_ITEMS).enumerate()` (line 54) makes one section per 32-row page labeled "Registers {start}–{end}". The `elements` section (line 65, open-ended program elements) has **no chunking at all**. | `elements` (`Vec<ProgramElement>`, `🧬️schema/📸️snapshot/🦀️.rs:36`) is unbounded and unpaged — a real program with >32 spaces/rooms hits `ui.fixed-capacity` today; the register lists (37-66 fixed catalog entries) are already safely chunked |
| `📚️catalogue/🦀️.rs` | 64 | `architect.catalogue` | catalogue | `PanelTreeBuilder` — `actions` (10 fixed) + `registers.{page}` chunked | same chunk-into-sections idiom over a 66-entry fixed `REGISTER_IDS` array → 3 pages ("Registers 1–32", "33–64", "65–66") | Fixed catalog, safe |
| `🔍️inspection/🦀️.rs` | 59 | `architect.inspection` | inspection | raw `section`/`ui_children` (not `PanelTreeBuilder`) | 6 fixed summary rows | None |

**This is the one plugin in the whole B-set with an existing app-local chunking idiom** (`.chunks(UI_FIXED_LIST_ITEMS)` → multiple labeled sections), distinct from both the framework's `paged_panel_section` (which pages within one section via a continuation row) and the unbounded `ui_node_list` idiom used everywhere else. It still isn't "+N" streaming — it's static multi-section splitting computed once per render, with no lazy/viewport behavior.

Tests: `📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs:21-24` — asserts `registerPages` (per-page chunk sizes) and `registerCount` (66) against a fixture (`🧫️fixtures/🔣️panels.json`). This is the **one test in the entire B-scope that pins a paging/chunk shape**.

## 11. 💠️lowpoly (artifact: 💠️lowpoly)

Base: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Paging | Growth risk |
|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 110 | `lowpoly.play.artifact` | artifact tree (mesh outline) | `PanelTreeBuilder`, one `meshes` section | **3-level nesting**: object → {vertex/edge/face} group → **one leaf row per individual vertex/edge/face id** (`for id in 0..count`, line 42); `default_open(object.id == active_id)`; face rows carry a `RowAction` (flip-normal, menu placement); rows bound `Activate` → `mesh_select_action`; `.interaction_domain(MESH_INTERACTION_DOMAIN)` | **none** — leaves pushed into a capacity-32 `UiFixedList` per group | **Most severe overflow risk in the whole B-scope**: one row per raw mesh element means any object with >32 vertices/edges/faces *of one kind* fails immediately — real lowpoly meshes routinely have hundreds |
| `🔍️inspection/🦀️.rs` | 165 | `lowpoly.play.inspection` | inspection | raw `section`/`field` (not tree) | object/transform/11-fixed-numeric-param groups | n/a | None — fixed-shape |
| `🗂️layers/🦀️.rs` | 40 | `lowpoly.play.layers` | layers list | `PanelTreeBuilder`, flat section | one `tree_item_with_action` per paint layer, `.selected([active_layer])` | none | Low-medium (paint layers are usually few) |
| `🛍️catalogue/🦀️.rs` | 43 | `lowpoly.play.catalogue` | catalogue | `PanelTreeBuilder`, flat section | fixed 5-entry `PRIMITIVE_CATALOG` | n/a | None |

Tests: none found pinning row/paging counts across any of the 4 panels.

## 12. 💡️reasoning (artifact: 🔌️wires)

Base: `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Paging | Growth risk |
|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 104 | `reasoning.wires.document` | artifact tree | `PanelTreeBuilder`, 2 flat `section_or_placeholder`s: `identities` (open) + `relationships` (closed) | flat rows, bare ids matching canvas hit-testing, `Activate` → `INTERACTION_SELECT_ACTION_ID`; `.interaction_domain(WIRES_INTERACTION_GRAPH)` | **none** | **High** — `wires_identities(wires)`/graph edges are open-ended for a real reasoning/mindmap document |
| `🔍️inspection/🦀️.rs` | 241 | `reasoning.wires.properties` | inspection | `PanelTreeBuilder`, 1 flat section, 4 fixed rows | fixed | n/a | None |
| `🛍️catalogue/🦀️.rs` | 72 | `reasoning.wires.catalogue` | catalogue | `PanelTreeBuilder`, 2 flat sections (identity-kinds, relationship-kinds) | driven by a fixed DSL kind catalogue | n/a | None — bounded by kind vocabulary, not document size |

Tests: a semantic-contract test pins section *labels* and summary *values*, not row counts.

## 13. 📋️forms (artifact: 📋️forms)

Base: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Paging | Growth risk |
|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 67 | `forms.play.artifact` | artifact tree | `PanelTreeBuilder.section_or_placeholder("steps")` | **2-level nesting**: step → question via `try_with_children` (line 48); steps default-open + draggable; questions draggable, icon `help-circle`; `.interaction_domain(FORMS_INTERACTION_FIELDS)` + `.drop_action(...)` | **none at either level** | **High** — a real form can have many steps × many questions per step, unbounded at both levels |
| `🔍️inspection/🦀️.rs` | 44 | `forms.play.inspection` | inspection | `PanelTreeBuilder`, 1 flat section, 3 fixed rows | fixed | n/a | None |
| `🛍️catalogue/🦀️.rs` | 62 | `forms.play.catalogue` | catalogue | `PanelTreeBuilder`, 2 flat sections: `kinds` (draggable question-kind palette) + `actions` (2 fixed) | bounded by fixed question-kind vocabulary | n/a | None |

Notably, `📋️forms` itself does **not** use the framework's `FormPanelBuilder` (which exists precisely for "Section > labeled Field rows" forms) — it builds its own `PanelTreeBuilder`-based step/question tree instead, since its panel is an outline of the *document's* form structure, not a settings form.

Tests: none pin row/paging counts.

## 14. 📏️layout (artifact: 📏️layout)

Base: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | BODY_KEY | Kind | Builder | Sections/nesting | Paging | Growth risk |
|---|---|---|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 216 | `layout.play.artifact` | artifact tree | `PanelTreeBuilder`, **9 flat sections**: `document`, `spreads`, `pages`, `frames`, `parentPages`, `layers`, `stories`, `links`, `styles` | all flat, no nesting; `frames` flattens every frame across every page into one list (137-143); `.interaction_domain(LAYOUT_INTERACTION_ELEMENTS)` | **none in any of the 9 sections** | **Second most severe in B-scope** — most flatly-enumerated panel found: `frames`/`links`/`styles`/`layers`/`stories` are all genuinely open-ended for a real print/layout document |
| `🔍️inspection/🦀️.rs` | 67 | `layout.play.inspection` | inspection | raw `section`/`try_child` | 4 fixed rows | n/a | None |
| `🛍️catalogue/🦀️.rs` | 68 | `layout.play.catalogue` | catalogue | `PanelTreeBuilder`, flat, 4 fixed draggable items | fixed | n/a | None |
| `🚦️preflight/🦀️.rs` | 251 | `layout.play.preflight` | issue list | `PanelTreeBuilder`, flat `issues` section from `run_layout_preflight` (all pages × all frames) | one row per detected issue | **none** | Unbounded — grows with document complexity (overset text, missing assets, out-of-bounds) |

Tests: `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:12-21` pins the **9-section id set** (not row counts, a shape test); `🚦️preflight/🧪️tests/🔬️unit/🦀️.rs:14-66` pins the set of issue *codes* from an 8-frame/4-link fixture (content, not a paging contract).

## 15/16. 🔱️trinity — two artifacts, 🔌️jack and ♻️rewriting

### 15. 🔌️jack

Base: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | Kind | Builder | Notes |
|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 26 | document tree | `PanelTreeBuilder`, 2 flat sections: `nodes` (open, bare-id `tree_item_desc` for "ast"-domain matching) + `edges` (closed, `tree_item` via `builder.item_id("edge", …)`) | **No paging.** BODY_KEY constants live centrally in the parent `editor/🦀️.rs` (`TRINITY_JACK_PLAY_BODY_*`), dispatched by `match body_key` there — not in the panel file itself |
| `📚️catalogue/🦀️.rs` | 39 | catalogue | `PanelTreeBuilder`, 3 flat fixed sections (2 fixtures, 8 example queries, 3 kinds) | Fixed-shape |
| `🔍️inspection/🦀️.rs` | 16 | inspection | **not a tree** — static `built_text_node(Label::data("Select one or more pieces"))` | Documented framework gap: no `InteractionView`, can't show per-selection fields |

**Zero `🧪️tests` directories exist under any jack panel** — no tests of any kind for these 3 files.

### 16. ♻️rewriting

Base: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/`

| Panel | LOC | Kind | Builder | Notes |
|---|---|---|---|---|
| `🗿️artifact/🦀️.rs` | 21 | document tree | `PanelTreeBuilder`, 1 flat `nodes` section from `fixture.nodes()` | No paging; bare ids for `graph` domain, mirrors jack's convention |
| `📚️catalogue/🦀️.rs` | 38 | catalogue | `PanelTreeBuilder`, 3 flat fixed sections (`kinds`×3, `lhs`×1, `rhs`×5) | Fixed-shape "add rule clause" shortcuts |
| `🔍️inspection/🦀️.rs` | 16 | inspection | Same static placeholder as jack | Same framework-gap comment |

Same story — no `🧪️tests` directories under rewriting panels either.

**Growth risk (both)**: node/edge lists are open-ended graph-document content with zero paging; realistic rule/graph fixtures in this test-oriented tool are usually small, but nothing caps them — same overflow risk as everywhere else once a fixture grows past 32.

## 17. 🪐️space — two distinct panel locations

### 17a. `⚙️engine/🪐️space/📌️panels` (unusual location — under the engine, not under `artifacts/…/editor/panels`; flagged per the ticket's "not under 📌️panels" ask, though here it's the reverse — panels *without* the usual artifact-editor ancestry)

| Panel | LOC | BODY_KEY | Kind | Builder | Notes |
|---|---|---|---|---|
| `🔢️parameters/🦀️.rs` | 272 | `S_PLAY_PARAMETERS_BODY_KEY` | workflow-parameters editor | **not `PanelTreeBuilder`** — raw `column()` of one `section()` per workflow parameter (`UiFixedList`, line 179), each with editable fields | `default_open(true)` everywhere; `projection.parameters` (`Vec`) flattened into sections, capped at 32 total (31 real parameters + 1 header) before `"parameters section admission failed"` |
| `🔍️inspection/🦀️.rs` | 267 | `S_PLAY_INSPECTOR_BODY_KEY` | selection inspector | raw `column`/`section` driven by `selected_node_ids` | Bounded by selection size + fixed per-app `parameter_fields` schema — lower risk |
| `🛍️catalogue/🦀️.rs` | 121 | `S_PLAY_CATALOGUE_BODY_KEY` | **whole-framework app catalogue** | `PanelTreeBuilder` + **recursive** `app_catalogue_item` (line 44) | Enumerates `workflow_palette()` — every registered plugin app in the entire framework — nested by document breadcrumb. `default_open(!children.is_empty())` (line 55) means **every branch auto-expands** — the full app registry renders fully expanded on every render, with no paging at any depth. **The single clearest "should be lazy-expand" candidate in the whole audit**, since it isn't document-bound at all — it scales with how many plugin apps are installed, not with any one document's content. Verified directly: `app_catalogue_item` recurses over `node.children: BTreeMap<String, AppCatalogueNode>` with no cap besides the per-level 32-child `UiFixedList`. |

### 17b. `🗿️artifacts/🪐️space/…/✏️editor/📌️panels` (the normal artifact-editor location)

| Panel | LOC | BODY_KEY | Kind | Builder | Notes |
|---|---|---|---|---|
| `👥️members/🦀️.rs` | 94 | `s.space.members` | space-membership editor | `PanelTreeBuilder`, 1 flat section: 3 fixed action rows (invite/share-link/visibility-toggle) + one row per `config.members` entry | **No paging.** `config.members` is open-ended (real collaboration spaces can have many members); each member row carries a `removeMember` action; capacity 32 total including the 3 fixed rows (~29 members max before admission fails) |

Tests: none in either location pin row/paging counts (`🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:11` uses `.iter().take(2)` only to build a small test *fixture selection*, not a paging contract).

---

## 18–23. Plugins with NO 📌️panels directory — trees (where present) built inline in the editor crate, or not built at all

### 18. ➗️mathematical (artifact: ➗️equation)

**No tree UI.** `✏️editor/🦀️.rs` (1419 lines) dispatches exactly two body keys (line 1347-1354): `MATH_PLAY_BODY_GRAPH` → `graph_window::render(...)` (a node-graph canvas), `MATH_PLAY_BODY_GEOMETRY` → `geometry_window::render(...)` (a geometry viewport). `grep -rln "ui::tree|tree_section|tree_item|PanelTreeBuilder"` across the whole plugin: zero hits. Not applicable to this audit.

### 19. 🎪️demonstrator (artifact: 🎪️playground)

**No tree UI.** `✏️editor/🦀️.rs` (441 lines) dispatches its one body key to `main::render` (line 411) → a plain `TextWindowKit::render(&TextView{...})` editing the document's single `schema` string field. Zero tree-API hits anywhere in the plugin. Not applicable.

### 20. 📖️playbook (artifact: 📖️playbook)

**No `ui::tree()`/`PanelTreeBuilder` anywhere.** `✏️editor/🦀️.rs:501-506` dispatches its one body key (`PLAYBOOK_PLAY_BODY_BUILDER`) to `✏️editor/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️.rs:73-76` (verified directly: `pub fn render(spec, config) -> UiAssemblyResult<BuiltNode> { ... semio_framework_plugin::scene_surface(PLAYBOOK_PLAY_SURFACE_BUILDER, SurfaceKind::BlockList, &crate::playbook::build_playbook_list_scene(&kernel, &build_palette(config), None)) }`) — a **drag/drop Blockly-like block-list scene surface**, a distinct rendering primitive from any tree API, driven by `build_playbook_list_scene`. That function's row/block-count admission behavior lives outside this plugin crate (framework-side support module) and was not traced further; flag separately if virtualization work extends to `BlockList` surfaces. `addStep`/`removeStep`/`moveStep`/`addBlock`/`removeBlock`/`moveBlock` mutations exist on the editor (lines 519-524), confirming the underlying document supports an open-ended step/block list even though it isn't rendered as a `ui::tree()`.

### 21. 🪵️sourcing (artifact: 🗂️curation)

**No `ui::tree()`/`PanelTreeBuilder` anywhere** (confirmed independently: the plugin defines a `ui_node_list` helper at `✏️editor/🦀️.rs:152` but **never calls it** — dead code). `✏️editor/🦀️.rs:1082-1090` dispatches 4 body keys to window modules: `🏊️pool` and `🧺️curated` use `SurfaceKind::Table`; `👁️preview`/`🔢️grid` use `SurfaceKind::World3d` (mesh viewers, not lists).

- **Pool** (`🏊️pool/🦀️.rs`, 154 lines) renders the entire filtered stock catalogue as table rows — `pool_kinds(document, cfg).iter().map(pool_row).collect()` (line 141), no `.take(N)`, no cap. Per-row actions via `UiTreeItemAction`/`UiTreeActionPlacement::Row` (61-67) despite the surface being `Table`, not `Tree`.
- **Curated** (`🧺️curated/🦀️.rs`, 96 lines): same shape, `curated_rows(document, cfg)` unbounded (46-59), collected wholesale (74-88).
- Both go through shared `sourcing_table`/`sourcing_table_row` helpers (`✏️editor/🦀️.rs:83-104`) that serialize rows to a JSON string (`protocol::json::to_json_string`, line 99) for a `TableScene`, **bypassing `UiFixedList`/`BuiltNode` admission entirely** — no hard 32-row cap, but also zero windowing: an arbitrarily large catalogue is serialized into one JSON blob per render.

**Growth risk**: a curation stock/material catalogue routinely runs into hundreds of SKUs — a strong virtualization candidate, but via the Table/TableScene JSON-blob mechanism, not the tree "+N" pattern this ticket centers on; needs its own fix path.

### 22. 🗟️artifacts (artifact: ◻️2d)

**Not implemented at all.** `✏️editor/` contains only an empty `🧵️session/` directory — **zero files anywhere in the entire plugin tree** (confirmed independently via `find ✏️s/🔌️plugins/🗟️artifacts -type f` → no results). No Cargo.toml, no schema, no editor, nothing to audit.

### 23. 🗄️stdio — ~40 file-format artifacts

`grep -rl "TreeWindowKit|PanelTreeBuilder|ui::tree_item|ui::tree("` across every top-level stdio artifact directory found tree UI in **exactly 4 of ~40 artifacts**, all via the fourth primitive `TreeWindowKit` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30897-30985`), never `PanelTreeBuilder`:

| Artifact | Editor file(s) | Tree shape | Paging |
|---|---|---|---|
| 🎒️zip | `…/2.0/🪆️subsets/{🌐️iso21320,🧱️base}/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` (50 lines each) | root = archive `comment`; one flat leaf per `ZipEntry` (`document.entries.iter().enumerate()...`, line 40) | **none** |
| 📰️xml | `…/1.0/🪆️subsets/{✅️valid,🧱️base}/…/🪟️main/🦀️.rs` (65 lines each) | fully recursive `node_view` mirroring `XmlNode::Element{children}` (38-57), unbounded depth and fan-out | **none** |
| 🧾️json | `…/rfc8259/🪆️subsets/{🛜️i-json,🧱️base}/…/🪟️main/🦀️.rs` (92 lines each) | fully recursive `node_view` mirroring `JsonValue::{Object,Array}` (56-85), unbounded | **none** |
| 💬️bcf (viewer only) | `…/2.1/🪆️subsets/🖊️markup/👁️viewer/…/🪟️main/🦀️.rs` | tree of BCF markup topics | none (editor itself uses `TableWindowKit` instead) |

**This is the single most severe unpaged-tree finding in the whole audit.** `TreeWindowKit::render` walks `TreeView.roots` with an explicit stack and, per node, does `parent.built.try_push(item).map_err(|_| ui_assembly_error("tree-window.siblings"))?` into a **plain, uncapped-by-any-paging `UiFixedList` (cap 32)** — `TreeWindowKit` never calls `paged_panel_section`. Consequence: **any single node with more than 32 children — 32 zip entries, a JSON array/object with >32 elements/members, an XML element with >32 children — makes the entire editor fail to render** (`Err("ui.fixed-capacity")` propagates out of `render()`). This is trivially reachable with ordinary real-world files (any moderately sized zip archive, JSON array, or XML document). None of the 6 editor-side `🧪️tests/🔬️unit/🦀️.rs` files for these variants test past a handful of nodes (`grep -n "32|31|fixed-capacity|siblings"` on the zip/json unit tests: no hits).

**A fifth pattern, also unpaged**: `TableWindowKit` (framework, renders `TableView{columns, rows}` by serializing straight to a JSON string via `TableScene::base`, bypassing `UiFixedList` admission) is used by 🌦️epw, 📊️csv, 📑️tsv, 📕️xlsx, and 💬️bcf's editor. Verified in full for csv: `📊️csv/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:29-38` builds a `TableView` from **all** `document.records`, no `.take(N)` — doesn't hard-fail like `TreeWindowKit` (JSON serialization has no 32-cap), but has no windowing/paging either; a CSV/XLSX with thousands of rows produces an arbitrarily large per-render JSON payload.

**Remaining ~36 stdio artifacts confirmed to have no list-shaped UI at all** (via `*WindowKit` import scan): `MeshWindowKit` (3D viewers — ☁️las, 🏗️ifc, 📐️step, 🔺️stl, 🖊️dwg, 🖋️dxf, 🗽️obj, 🧊️gltf, 🧱️ply, 🧿️semio, 💬️bcf-mesh), `ImageWindowKit` (🎞️gif, 🎨️svg, 📷️png, 📸️jpg, 🖼️tiff, 🪟️bmp), `MediaWindowKit` (🎥️mp4, 🎵️mp3, 📼️avi, 🔊️wav), `TextWindowKit` (🌐️html, 💾️binary, 📝️md, 🔤️txt, 🗜️deflate), `DocumentWindowKit` (📖️pdf, 📜️docx, 📽️pptx), and 4 with no `*WindowKit` match at all (🏃️commands, 📇️inventory, 🕸️graph, 🛂️contract — likely unimplemented/non-visual).

---

## Cross-app patterns (covers all of §1–23: norm + writer/flow/vcs/animate/sequence/dag/note/remodel/imperative + architect/lowpoly/reasoning/forms/layout/trinity/space + mathematical/demonstrator/playbook/sourcing/artifacts/stdio)

**Pattern A — no tree at all.** 📕️norm (all 15 artifacts): every panel (`artifact`/`catalogue`/`inspection`) renders a flat text summary or a single-item text node via a shared `app_surface.rs` module — zero `ui::tree()` usage anywhere. See §-1 below for the full norm write-up (carried over from the parallel norm audit, included verbatim in this file's norm section).

**Pattern B — `PanelTreeBuilder` + unbounded per-app `ui_node_list` (the dominant pattern, ~15 of 17 plugins/artifacts above).** A namespaced `PanelTreeBuilder::new(...).section(...)`/`.section_or_placeholder(...)` skeleton, fed by a per-plugin copy-pasted `fn ui_node_list(values) -> UiFixedList<BuiltNode>` that does nothing but unconditional `try_push`. **35 independent copies** of this exact helper exist across the plugins tree (this scope: writer, flow, vcs, animate, sequence, architect, lowpoly, reasoning, forms, layout, imperative, remodel, trinity, dag, space; sourcing has a copy too but never calls it — dead code). None of these 35 call the framework's `paged_panel_section`/`panel_continuation_row`/`PanelRowBudget`/`setPanelPage`. Overflow past `UI_FIXED_LIST_ITEMS = 32` is a hard `PluginAssemblyError::new("ui.fixed-capacity", …)` — the render fails, it does not show a "+N" row. This is the shape that most needs replacing, and unlike the ticket's framing ("+N row the user has to click"), for this scope the honest baseline is "no graceful degradation exists yet."

**Pattern C — app-local static chunking into multiple sections (🏛️architect only).** `.chunks(UI_FIXED_LIST_ITEMS)` splits a >32-entry fixed catalogue into N labeled sections ("Registers 1–32", "33–64", …) computed once per render. Distinct from both A/B and from the framework's `paged_panel_section` (which pages *within* one section via a continuation row) — this instead multiplies sections. Still not lazy/viewport-based.

**Pattern D — framework `paged_panel_section`/`setPanelPage` (the mechanism the ticket's brief assumes is pervasive) — found ONLY in the framework's own generic `ui_history_panel` (undo/redo/commit history, every app gets this for free, and even there it hand-builds its own action-bearing `+N` row rather than calling `panel_continuation_row` verbatim — see §0) and in the **excluded** plugins (📐️cad, 🧩️puzzle, 🏗️fem, 🔋️energy, 🌀️procedural) per that auditor's scope. Zero uses inside this ticket's 23-plugin scope.**

**Pattern F — `TreeWindowKit` (a fourth, distinct tree primitive, framework `plugin/🦀️.rs:30897-30985`), used only by 🗄️stdio's zip/xml/json editors (§23).** Fully recursive, unbounded document structures (archive entries, arbitrary JSON/XML trees) pushed into a plain `UiFixedList` (cap 32) with **no paging call at all** — not even the inert `panel_continuation_row`. **This is the single most severe finding in the entire audit**: any ordinary zip archive, JSON array, or XML document with more than 32 siblings anywhere in its structure makes the whole editor fail to render today, and unlike the `ui_node_list` plugins above (open-ended but often realistically small user documents), zip/JSON/XML files with >32 entries/elements are common, unremarkable, everyday inputs.

**Pattern G — `TableWindowKit`/ad hoc `TableScene` JSON serialization** (🪵️sourcing pool/curated panels §21; 🗄️stdio csv/tsv/xlsx/epw/bcf-editor §23). Rows serialize straight to a JSON string bypassing `UiFixedList` admission — no hard 32-cap and no crash, but also zero windowing: an arbitrarily large table (a stock catalogue, a multi-thousand-row CSV) is fully serialized on every render. A different fix path than tree virtualisation, but the same underlying problem (unbounded document content rendered in one shot).

**Pattern H — non-`ui::tree()` surfaces flagged but not fully traced**: 🪐️space's `🔢️parameters` panel (raw `column`/`section`, capped at 32 sections, own mechanism); 📖️playbook's `SurfaceKind::BlockList` builder surface (§20) — admission behavior lives in a framework-side support module outside the plugin crate and wasn't traced to its cap; flag separately if virtualisation work extends past trees/tables to `BlockList`.

**No tree UI at all**: 📕️norm (§-1, flat text/placeholder panels), ➗️mathematical (§18, graph/geometry canvas only), 🎪️demonstrator (§19, single text field), 🗟️artifacts (§22, entirely unimplemented stub, zero files).

**Which plugins/panels genuinely need virtualisation, ranked by urgency:**

1. **🗄️stdio zip/xml/json editors (`TreeWindowKit`, §23)** — recursive, fully unbounded, *zero* paging of any kind (not even the inert continuation row), and triggered by ordinary everyday files. Highest priority: a live landmine, not a hypothetical edge case.
2. **🕸️dag** (nodes/edges of an "infinite DAG" — worst case among `ui_node_list` plugins), **🌿️vcs** (unbounded commit/branch history — grows by construction, no ceiling), **💠️lowpoly** (per-vertex/edge/face leaf rows, 3 levels deep — worst raw-count case among document-driven trees), **📸️remodel** `✅️quality`/`🎯️calibration` (unbounded QC findings / cameras+GCPs), **📜️imperative** `🗿️artifact` (procedure steps, concretely fails past 32).
3. **🎬️sequence** (steps/edges + recursive per-slot nesting, 3 independent unbounded axes — most structurally complex), **📏️layout** (9 flat sections incl. cross-page-flattened frames + unbounded preflight issues), **📋️forms** (2-level step→question, both unbounded), **💡️reasoning** (identities/relationships), **🏛️architect** `elements` (its register lists are already app-chunked, but `elements` isn't), **🪐️space** engine catalogue (whole installed-app registry, auto-expanding — not even document-bound, arguably the clearest "lazy children on expand" case since it doesn't scale with any one user's document) and members list, **🌊️flow** (widgets/synapses), **🗒️note** (recursive blocks, uniquely sharing capacity with 5 fixed chrome rows), **🎞️animate** (deck tiles), **🔱️trinity** jack/rewriting document panels (lower realistic scale, same unguarded shape), **✒️writer** (AST, lowest risk of this group).
4. **Different mechanism, same underlying problem**: 🪵️sourcing pool/curated (§21) and 🗄️stdio csv/tsv/xlsx/epw/bcf editors (§23) — unbounded `TableWindowKit`/JSON-blob rendering, needs a table-specific windowing fix, not `paged_panel_section`.

**Small, fixed-shape, no virtualisation needed**: every catalogue panel driven by a fixed kind/action vocabulary rather than document content (writer, flow, vcs n/a, animate, sequence, dag, note, architect action-shortcuts, lowpoly, reasoning, forms, layout, trinity jack/rewriting, space engine), every inspection panel editing a bounded field set for the current selection rather than listing rows (present in nearly every plugin above), all of norm's 45 panels, 📜️imperative's catalogue/inspection, 📸️remodel's parameters/pipeline/results/tracks (tracks currently gated off), and the ~36 stdio artifacts with no list-shaped UI at all.

**Memoization/caching**: none found anywhere in this entire 23-plugin scope. Every panel's `render()` is a pure function rebuilt from scratch on each dispatch from the current document/config snapshot; no `Memo`, no cached `BuiltNode`, no equality-gated skip, in any plugin or in the framework's `TreeWindowKit`/`TableWindowKit`/`PanelTreeBuilder` machinery itself.

**Minimal SDK helper signatures every one of these panels funnels through today** (the actual replacement surface for the ticket's design):
- `PanelTreeBuilder::new(namespace) -> Self`, `.section(id, label, default_open, items: UiFixedList<BuiltNode>) -> Self`, `.section_or_placeholder(id, label, default_open, items, placeholder_label) -> Self`, `.interaction_domain(id) -> Self`, `.drop_action(action) -> Self`, `.build() -> UiAssemblyResult<BuiltNode>` — used by the large majority of panel files audited in this file.
- The per-app `fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<UiFixedList<BuiltNode>>` — **35 duplicate copies** across the plugins tree, all functionally identical (unconditional `try_push`, `ui.fixed-capacity` on overflow). This is the single highest-leverage place to introduce a windowed/lazy replacement: one shared SDK function signature change (e.g. accepting a `total` count + a host-provided window offset/length, per `design-virtualised-tree.md` §3.3's `tree_window_section`/`tree_window_item`) would flow into all 35 call sites with minimal per-app code change, since every caller already isolates "build all the rows" behind this one function.
- `TreeWindowKit::render`'s node-stack/`try_push`-per-sibling loop (framework `plugin/🦀️.rs:30897-30985`) — the second highest-leverage site: fixing this one function fixes zip/xml/json (and any future format editor built on it) in one place.
- `TableWindowKit`'s `TableScene::base` JSON-serialization path — the site to fix for sourcing's pool/curated and stdio's table-format editors.
- Recursive nesting (lowpoly mesh groups, sequence control-flow slots, note blocks, writer AST, space app-catalogue, stdio's xml/json `node_view`) all use the same `try_children`/`try_child` mechanism on a `tree_item`/`TreeItemBuilder` (or, for `TreeWindowKit`, its own explicit stack) — any lazy-children-on-expand design needs to handle recursion depth, not just one flat list, for at least 6 of the call sites in this scope.

---

## §-1. 📕️norm — all 15 artifacts (carried over, no tree UI at all)

Scope: en1990(⚖️)/en1991(🏋️)/en1992(🏛️)/en1993(🔩️)/en1994(🧩️)/en1995(🪵️)/en1996(🪨️)/en1997(🌍️)/en1998(🫨️)/en1999(🪶️)/din4108(🧱️)/din16798(🌬️)/din18599(⚡️)/iso16757(📇️)/vdi3805(🏭️), each at `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji><variant>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/{🗿️artifact,📚️catalogue,🔍️inspection}/🦀️.rs`.

**Bottom line: norm has no tree-paging mechanism to virtualise, because it has no tree at all.** Grep for `paged_panel_section|panel_continuation_row|PanelTreeBuilder|tree_group|_ROWS|setPanelPage|\.more|PanelRowBudget|ui::tree` across the entire `📕️norm` plugin tree returns **zero hits**.

**Shared mechanism**: every one of the 45 panel files (3 kinds × 15 artifacts) is a ~27-29 line stub delegating into one shared module, `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` (865 lines):
- `render_summary<F: NormFamily>` (line 183) — one-line headline text (`"{family} — {n} checks, worst u={:.2}, all pass={}"`), used by every `🗿️artifact` panel.
- `render_catalogue(label)` (189) — a hardcoded placeholder string `"{label} catalogue"` — the file's own doc comment states "no norm family ships a browsable clause catalogue yet."
- `render_inspection(report, selected_check_index)` (195) — selects **exactly one** `CheckResult` by clamped index and renders it as a single text node; never lists multiple rows.
- `render_report(report)` (155) — renders **all** checks as a flat `ui::column()` of one `ui::text()` per check, no windowing, no `+N` — but this is the **`📊️results` window** (edit mode), not a panel, so out of the panels-only scope; flagged since `CheckReport.checks: Vec<CheckResult>` (`⚖️compliance/🦀️.rs:180-181`) is exactly the kind of open-ended collection that would need paging/virtualisation if this work later extends to windows.

Per-artifact deltas: **none** — all 15 artifacts share identical panel logic; only the underlying `NormFamily`/`Document` schema differs, never the panel-rendering path. No sections, no nesting, no `try_children`, no `default_open`, no `Activate` bindings, no `RowAction`/`draggable`/`interaction_domain` in any of the 45 files.

**Virtualisation need**: **none of the 15 artifacts need it** — `artifact` (single summary line), `catalogue` (unimplemented placeholder), `inspection` (exactly 1 row, index-selected) are all bounded regardless of document size.

**Tests**: all 45 panel unit tests assert (1) `definition().body_key` matches the `BODY_*` constant, (2) a substring smoke-test on rendered text. The `inspection` test additionally checks index-clamping (out-of-range `selected_check_index` falls back to 0) — the closest thing to a "bounds" test, but about selection-index clamping, not row/page counts. **Zero tests anywhere in norm pin a row cap or "+N" shape**, because the mechanism doesn't exist.

**Recommendation for the audit**: record norm as a clean negative result for panel virtualisation; flag `render_report` (the `📊️results` window, not a panel) separately if the scope ever extends past panels to windows.
