# 📓️ Design — Virtualised, lazily streamed panel trees (v1, final, 2026-09-16)

Inputs: 📓️audit-host-tree-pipeline.md, 📓️audit-surface-budgets.md, 📓️audit-app-panels-a.md, 📓️audit-app-panels-b.md,
📓️audit-paging-plumbing-and-schema.md, 📓️audit-paging-tests.md (all in this folder). Every name below is normative:
wave-1 packets implement exactly these signatures, wave-2 app packets call exactly them.

## 1. Problem (measured)
- Panel trees page with `+N` continuation rows (cad, puzzle 2d/3d navigable; fem, energy, generation3d catalogue inert),
  or truncate with a dead `+N` (inspection id lists), or render unbounded and HARD-FAIL past 32 rows with
  `ui.fixed-capacity` (flow, dag, vcs, note, sequence, remodel, imperative, stdio zip/xml/json …).
- Causes: `UI_BUILT_CHILDREN_MAX = 32` (fixed 32-slot `BuiltChildren`), `UI_VALUE_PAGE_ROWS = 31` argument-arena page
  (per plugin wasm instance), stale `PANEL_RECONCILE_NODE_BUDGET = 16`, and the guest never learning which containers are
  open or which rows are on screen (expansion is per-mount React state, lost on every refresh).

## 2. Goal
No `+N` anywhere. Every container reports its full `total`; the scrollbar spans the complete document. Children are
materialised only for open containers; only the rows intersecting the viewport (+ overscan) are materialised; scrolling
requests the next window; expansion is host-owned and survives refreshes. One framework mechanism for every app.

## 3. Contract — `semio_framework_ui_contract`
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs`:
```rust
/// 🪟️ The materialised slice of a logically `total`-long child list: `children` are entries
/// `[offset, offset + children.len())`. `total > 0` with no children = expandable, not yet loaded.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TreeWindow { pub total: u32, pub offset: u32 }
// TreeSectionProps: + pub window: Option<TreeWindow>          (serde/value default + skip_serializing_if none)
// TreeItemProps:    + pub window: Option<TreeWindow>
//                   + pub granularity: Option<UiText>         (row is a pick target of the tree's interaction domain)
```
`🏗️builder/🦀️.rs`: `TreeSectionBuilder::window(TreeWindow)`, `TreeItemBuilder::window(TreeWindow)`,
`TreeItemBuilder::granularity(UiText)`; both `credited_clone`s copy the new fields.
Constants: `UI_BUILT_CHILDREN_MAX = crate::UI_DOCUMENT_NODES` (=128: one built node fans out exactly as wide as one host
document child list, `UiNodeChildren`). `🎬️action/🦀️.rs`: `UI_VALUE_PAGE_ROWS = crate::UI_BUILT_CHILDREN_MAX` (the arena
prices one full viewport window of canonical rows; docstrings rewritten — the "four resident surfaces" text is stale,
the aggregate is `UI_RESIDENT_SLOTS × UI_RESIDENT_DOCUMENT_BYTES`). `UI_VALUE_LIVE_PAGES` stays 1.
TS mirror: `🧬️schema/🦀️.rs` `TYPES` gets `TreeWindow` (+1 entry → typegen test count 81) and the two props' new fields;
regenerate with `bun nx run @semio-tech/ui-contract-rs:generate`; conformance fixtures under
`🧫️fixtures/🧪️conformance/…/🌳️tree*` and retirement byte fixtures `♻️retirement/🌳️typed/…` updated by running them.

## 4. Host → guest request — `ViewModel` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4499`)
```rust
/// 🪟️ One tree container's host-known state: whether the user opened/closed it (None = author default)
/// and the row window on screen, overscan included.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")] #[value(rename_all = "camelCase")]
pub struct TreeWindowRequest { pub body_key: String, pub node_key: String,
    #[serde(skip_serializing_if = "Option::is_none")] #[value(skip_serializing_if = "Option::is_none")] pub open: Option<bool>,
    pub offset: u32, pub rows: u32 }
// ViewModel: + #[serde(default, skip_serializing_if = "Vec::is_empty")] pub tree_windows: Vec<TreeWindowRequest>
//            + #[serde(skip_serializing_if = "Option::is_none")] pub tree_viewport_rows: Option<u32>   // rows the tallest visible panel body fits
```
`for_panel()`/`for_window_instance()` use `..self.clone()` so both fields survive. TS mirror regenerated with
`bun nx run @semio-tech/framework:generate` (`🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`).
Why ViewModel and not a framework-reserved action: the only source of truth for scroll/open state is the host; ViewModel
is already threaded into every `render`, rebuilt per refresh, and the guest's `refresh` loop renders exactly the
`request.panels` the host names, hash-conditionally. A window change = host mutates its map → `refreshUi(session,
{ kind: "partial", panelBodies: [bodyKey] })`. No guest state, no action registration, no actor turn.

## 5. SDK — `semio_framework_plugin` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, region 🔖️PanelKit)
Region `🔖️PanelPaging` (`panel_page_rows`, `PanelRowBudget`, `panel_continuation_row`, `paged_panel_section`) is DELETED
(and its re-export at ~39485). Replacement, same region:
```rust
pub const TREE_WINDOW_DEFAULT_ROWS: u32 = 48;
/// 🪟️ What one container materialises this render.
pub struct TreeSlice { pub open: bool, pub offset: usize, pub len: usize, pub total: usize }
/// 🪟️ The host's windows for one body, read once per render from `ViewModel.tree_windows`.
pub struct TreeWindows<'a> { /* requests filtered to body_key, first-paint budget Cell<u32> */ }
impl<'a> TreeWindows<'a> {
    pub fn for_body(view: &'a ViewModel, body_key: &str) -> Self;
    pub fn unhosted() -> Self;                       // tests / no host state: author defaults + TREE_WINDOW_DEFAULT_ROWS budget
    pub fn is_open(&self, node_key: &str, default_open: bool) -> bool;
    pub fn slice(&self, node_key: &str, default_open: bool, total: usize) -> TreeSlice;
    pub fn window(slice: &TreeSlice) -> TreeWindow;  // { total, offset } as u32
}
/// 🪟️ One windowed section node: only `slice.len` rows built, `window` stamped, never a `+N`.
pub fn tree_window_section<T>(windows: &TreeWindows<'_>, id: &str, label: Label, default_open: bool, entries: &[T],
    row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;
/// 🪟️ Same, substituting one placeholder row when `entries` is empty.
pub fn tree_window_section_or_placeholder<T>(…, placeholder_label: Label) -> UiAssemblyResult<BuiltNode>;
/// 🪟️ A windowed group item (object › vortices, load case › loads): `item` already carries its id/label/icon.
pub fn tree_window_item<T>(windows: &TreeWindows<'_>, item: TreeItemBuilder, id: &str, default_open: bool, entries: &[T],
    row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;
impl PanelTreeBuilder {
    pub fn window_section<T>(self, windows, id, label: Option<Label>, default_open, entries: &[T], row) -> UiAssemblyResult<Self>;
    pub fn window_section_or_placeholder<T>(…, placeholder_label) -> UiAssemblyResult<Self>;
    /// 🕹️ Binds the domain AND stamps ONE tree-level Activate binding
    /// `ActionFactory::new(controller_id).action(INTERACTION_SELECT_ACTION_ID, {domainId})` so pick rows need no args.
    pub fn interaction_domain(self, controller_id: &str, domain: impl TryInto<UiText>) -> UiAssemblyResult<Self>;
}
/// 🌳️ The ONE `ui_node_list` (fallible iterator → UiFixedList<BuiltNode>) — the 35 per-app copies are deleted.
pub fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<UiFixedList<BuiltNode>>;
```
`slice` semantics: `open = is_open(node_key, default_open)` (host `open` wins, else author default). Closed → `len 0`,
`offset 0`, `total` stamped. Open with a host request → `offset = min(req.offset, total.saturating_sub(1))`,
`len = min(req.rows, UI_BUILT_CHILDREN_MAX, total − offset)`. Open without a request (first paint, or a container the host
has not seen yet) → `offset 0`, `len = min(total, UI_BUILT_CHILDREN_MAX, budget_remaining)` and the shared first-paint
budget (`tree_viewport_rows.unwrap_or(TREE_WINDOW_DEFAULT_ROWS)`) is decremented by `len` in document order, nested
containers included — so the first paint materialises about one viewport of rows and nothing more. A `row` refused with
`ui.fixed-capacity` ends the window early (shorter `len`), never faults; any other error propagates. `window` is stamped
whenever `total > 0` or a request exists, so the host always sees the full extent.
`ui_history_panel` (`HISTORY_COMMAND_ROWS` pager, `history_page` cursor, its `+N` row action) moves onto
`tree_window_section` with body key `FRAMEWORK_HISTORY_BODY_KEY`.
The UiTree-derived domain topology (`ui_tree_domain_topology`) now sees only materialised rows; no app declares
`HierarchyProvider::UiTree`, documented at the call site.

## 6. Host — React
### 6.1 `🌳️Tree/🟦️.tsx`
- `TreeDataItem.window?: { total: number; offset: number }`, `TreeDataSection.window?` (same). Both views render, when
  `window` is present: a `data-slot="tree-window-spacer"` div of `offset × treeRowHeightPx` before the rows and one of
  `(total − offset − items.length) × treeRowHeightPx` after; the container element carries `data-tree-window-key`
  (authored node key, supplied by the interpreter via a new `windowKey?: string` field) plus `data-tree-window-total`,
  `-offset`, `-length`. A windowed container is `expandable` when `total > 0`; open with `total > 0` and zero rows shows
  the existing `loading` ring. Open state stays controllable through the existing `openStates`/`onOpenStateChange`.
- Pure helper exported for tests: `treeWindowRequestsForViewport(containers: {key,total,offset,length,top,height}[],
  viewportTop, viewportHeight, rowHeightPx, overscanRows) → {key, offset, rows}[]` — offset = max(0, firstVisibleRow −
  overscan), rows = visibleRows + 2·overscan, clamped to `[1, min(total, 128)]`. `TREE_WINDOW_OVERSCAN_ROWS = 8`.
### 6.2 `🗣️Interpreter/🟦️.tsx` (`TreeView`, `treeItemToTreeData`)
- Map `window` and `windowKey = record.key` onto sections/items.
- New `TreeWindowContext` (provided per panel body by ShellHost, see 6.3): `{ bodyKey, openStates, setOpen(key, open),
  reportWindows(requests) }`. `TreeView` passes `openStates`/`onOpenStateChange` to `<Tree>`, and mounts
  `useTreeWindowObserver(rootRef)`: locates the scroll viewport (`closest('[data-slot="scroll-area-viewport"]')`, else
  the nearest scrollable ancestor), listens to `scroll` (rAF-coalesced) and a `ResizeObserver` on it, re-measures on
  store revision, computes requests with `treeWindowRequestsForViewport`, diffs against the last report, calls
  `reportWindows` on change. Also reports `viewportRows = ceil(viewportHeight / rowHeightPx)`.
- Pick synthesis: a row with `granularity` and no own `activate` binding, inside a tree whose root record has an
  `activate` binding (the SDK's tree-level `interactionSelect`), gets `onClick(event, ctx)`: `merge =
  interactionMergeFromModifiers(event)`; targets = `[{granularity, id: record.key}]`, or for `merge === "range"` every
  `ctx.selectedIds` mapped through a domId→(key, granularity) table with `merge = "replace"`; dispatch
  `context.onIntent(store.buildIntent(rootRecord, rootActivateBinding, { merge, method: "pick", targets }))` — `targets`
  encoded exactly as `world3dSelectionActionArgs` encodes them (read that helper in 🌐️World3dHost first).
### 6.3 `🏛️ShellHost/🟦️.tsx` + `🛠️ShellHelpers/🟦️.tsx`
- `treeWindowsRef: Map<bodyKey, { open: Map<nodeKey, boolean>; windows: Map<nodeKey, {offset, rows}> }>` and
  `treeViewportRowsRef`. `uiNodeToTreePanelConfig(node, onAction, bodyKey, treeWindowContext)`; provide the context in
  `panelTabDefinitionToNode`. `setOpen` → update map → schedule refresh immediately; `reportWindows` → update → schedule
  with a 40 ms trailing debounce; one in-flight partial refresh per body, latest state wins when it settles.
- `baseDispatchViewState` and the refresh request's view state carry `treeWindows` (flattened over all bodies) and
  `treeViewportRows`.
- Refresh: `refreshUi(session, { kind: "partial", panelBodies: [bodyKey] })`.
- Narrow the `panelUiByKey` `useMemo`s (~8727/8779/8801) to per-tab memoisation so one body's window change does not
  re-parse sibling tabs.
- Delete `"setPanelPage"` from `WINDOW_CONFIG_RAIL_ACTION_IDS` (~4654).
### 6.4 wgpu (`🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`, `🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs`)
Read `window` on sections/items, carry it on the legacy tree nodes, paint spacer pitch (`TREE_ROW_HEIGHT × count`)
before/after materialised rows; expandable when `total > 0`. No host-side request wiring for wgpu this ticket
(documented gap; no `+N` fallback).

## 7. Budgets (decided)
- `UI_BUILT_CHILDREN_MAX` 32 → 128 (= `UI_DOCUMENT_NODES`, the host `UiNodeChildren` capacity). Fixture/test pins listed
  in 📓️audit-surface-budgets.md §6 (`page_arity == UI_BUILT_CHILDREN_MAX` fixture, retirement slots) are updated.
- `UI_VALUE_PAGE_ROWS` 31 → 128: 640 collections / 1792 items ≈ 1.9 MiB per plugin instance; `ui_value_headroom` remains
  the safety valve (a refused row shortens the window, never faults).
- `PANEL_RECONCILE_NODE_BUDGET` deleted; per-node host cost is `size_of::<FlatPresentedNode>() + semantic bytes`
  (page-exact since 2026-09-13). A new law in the reconcile tests prints `size_of::<FlatPresentedNode>()` and asserts a
  128-row window × 4 sections reconciles under `SURFACE_RECONCILE_SURFACE_BYTES`.
- Pick rows cost 0 arena (tree-level binding); only row actions and app bindings (catalogue `addWidget{kind}` …) pay.

## 8. App migration rule (wave 2, every plugin)
1. Every list section → `window_section` / `window_section_or_placeholder`; every group row with children →
   `tree_window_item`. `default_open` semantics unchanged. `TreeWindows::for_body(view_state, BODY_KEY)` is built in the
   app's `render_body` and threaded into the panel builder (panels gain a `&TreeWindows` parameter).
2. Domain-bound rows: delete per-row `interactionSelect` maps; call `.interaction_domain(CONTROLLER_ID, DOMAIN)` and
   `.granularity(g)` per row. Rows keyed by the raw target id (already the presence convention). Rows with their own
   app action (catalogue `addWidget`, workshop install, checkout …) keep their binding.
3. Delete: app-local `paged_section*`, `continuation_row*`, `section_page`, `SECTION_ROWS`/`IDS_ROWS`/`LIST_ROWS_MAX`/
   `CATALOGUE_GROUP_ROWS`/`PANEL_RECONCILE_NODE_BUDGET`, `.take(N)` truncations, static `+N` rows, `setPanelPage`
   (command dir, enum/const, dispatch arm, `.view_action`, `.action_interactive_job`, `ArtifactToolPublicationContract`,
   `toolIds`, config/window-transient `panel_pages` + puzzle2d proto field + generated schemas, retained-jobs and
   publication-authority fixtures, cad manifest `🔣️.json` regeneration, cad TS `retained-audit` allowlist), the puzzle3d
   `artifact_tree_cache`/`Puzzle3dArtifactTreeKey` memo, the local `ui_node_list` copy (use the SDK one).
4. Tests: replace paging laws with window laws — (a) oversized document: every container stamps `window.total ==
   entries.len()` and materialises ≤ its slice; (b) closed container: `total` stamped, zero children; (c) a
   `TreeWindowRequest{offset: k, rows: n}` materialises exactly entries `[k, k+n)` keyed by raw id; (d) no `.more` key and
   no `+` label anywhere in the body JSON. Run with the crate's feature flags (📓️audit-paging-tests.md §8) and
   `cargo check --target wasm32-wasip2` for the plugin crate.

## 9. Work packets
Wave 1 (parallel): P1 contract + ViewModel (§3, §4) · P3 SDK (§5) · P4a Tree element (§6.1) · P4b Interpreter + ShellHost
(§6.2, §6.3) · P5 wgpu (§6.4) + reconcile size law (§7).
Wave 2 (parallel, after wave 1 compiles): A1 puzzle 3d · A2 puzzle 2d/5d · A3 cad · A4 fem 2d/3d · A5 energy · A6
procedural + process3d + block · A7 flow/dag/vcs/note/sequence/writer/animate/imperative/remodel · A8 the remaining
tree-bearing plugins (shooting, draw, raster, gis, layout, forms, trinity, reasoning, architect, lowpoly, space,
sourcing, mathematical, stdio TreeWindowKit editors, demonstrator, playbook, norm if any tree).
Wave 3: browser verification on fem3d, puzzle3d and process3d React serves (no `.more`, spacer extents, scroll streaming,
lazy expand, expansion surviving a refresh), plus `bun nx run @semio-tech/framework-os:test`.
