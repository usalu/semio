# 📓️ Design — Virtualised, lazily streamed panel trees (draft v0, 2026-09-16)

Status: DRAFT — budget-dependent sections (§5) are finalised after 📓️audit-surface-budgets.md and 📓️audit-paging-plumbing-and-schema.md land.

## 1. Problem
Every plugin panel tree (artifact outliners, inspection id lists, catalogues, history) materialises ONE bounded page
(`paged_panel_section` / app-local copies) and closes truncated sections with a `+N` row that the user has to click
(`setPanelPage`). The user sees "7 objects, +173"; nothing streams. Constraints that produced this shape:
- `UI_BUILT_CHILDREN_MAX = 32` children per built node.
- `UI_VALUE_PAGE_ROWS = 31` interactive rows per render across the whole process (`UiValue` argument arena, one live page).
- Host reconcile byte cap (puzzle3d pins `PANEL_RECONCILE_NODE_BUDGET = 16` nodes).
- The guest never learns which nodes are expanded or which rows are on screen (`treeOpenStates` is host-only; `treeExpansion` is a tutorial change kind).

## 2. Goal
- No `+N` rows anywhere. Every section/group reports its full `total`; the scrollbar reflects the complete document.
- Children are materialised only for expanded containers (lazy on expand).
- Only the rows intersecting the panel viewport (+ overscan) are materialised; scrolling requests the next window from the guest.
- One framework mechanism: contract fields + SDK helpers + React host + wgpu target; every app funnels through it; zero per-app action registration.

## 3. Protocol (decided)
### 3.1 Contract (`semio-framework-ui-contract`, `🧩️component/🦀️.rs`)
```rust
/// 🪟️ The materialised slice of a logically `total`-long child list.
pub struct TreeWindow { pub total: u32, pub offset: u32 }
TreeSectionProps { …, pub window: Option<TreeWindow> }
TreeItemProps    { …, pub window: Option<TreeWindow> }
```
Semantics: `children` are entries `[offset, offset + children.len())` of `total`. A node with `window.total > 0` is
expandable even with zero children (lazy). `total == 0` with `window` present = genuinely empty (render the app's
placeholder row as today). Absent `window` = plain node (all children inline), unchanged behaviour.

### 3.2 Host → guest request (decided: ViewModel, not an action)
Host-owned UI state rides `ViewModel` exactly like `active_utility_by_window_id`:
```rust
ViewModel { …, pub tree_windows: Vec<TreeWindowRequest> }
/// 🪟️ One container's host-known state: expanded or not, and the row window on screen.
pub struct TreeWindowRequest { pub body_key: String, pub node_key: String, pub open: bool, pub offset: u32, pub rows: u32 }
```
- The React host derives the list from `treeOpenStates` + the panel scroll geometry (fixed row height `--size-workbench`).
- A change dispatches `refreshUi(session, { kind: "partial", panelBodies: [bodyKey] })` (existing lane, hash-conditional),
  throttled to one in-flight request per body with the latest geometry coalesced (latest-wins).
- Nothing is registered per app; no guest-side state; a panel body is a pure function of (document, view_state).
- Containers absent from the list use guest defaults: `default_open` decides expansion; the window starts at 0 with the
  panel's default viewport rows (`ViewModel.panel_viewport_rows`, host-measured; falls back to `TREE_WINDOW_DEFAULT_ROWS`).

### 3.3 SDK helpers (`semio-framework-plugin`, replaces the whole `🔖️PanelPaging` region)
```rust
pub struct TreeWindows<'a> { … }                       // built once per render from &ViewModel + body key
impl TreeWindows { pub fn for_view(view: &ViewModel, body_key: &str) -> Self; }
/// 🪟️ One windowed container: slices `entries` to the host window, builds only those rows, stamps `TreeWindow`.
pub fn tree_window_section<T>(windows: &TreeWindows, id: &str, label: Label, default_open: bool, entries: &[T],
    row: impl FnMut(&T, &TreeWindows) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;
/// 🪟️ One windowed group item (object › vortices, load case › loads): children only when expanded.
pub fn tree_window_item<T>(windows: &TreeWindows, item: TreeItemBuilder, id: &str, default_open: bool, entries: &[T],
    row: impl FnMut(&T, &TreeWindows) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;
```
`PanelTreeBuilder` gains `.window_section(...)` delegating to the above. `PanelRowBudget`, `panel_page_rows`,
`paged_panel_section`, `panel_continuation_row`, every app-local `SECTION_ROWS`/`IDS_ROWS`/`LIST_ROWS_MAX` paging and
every `setPanelPage` action + `panel_pages` state are deleted.

### 3.4 Host (React `🌳️Tree`, `🗣️Interpreter`)
- `TreeDataItem`/`TreeDataSection` gain `window?: { total, offset }`.
- A windowed container renders: spacer(offset × rowHeight) · materialised rows · spacer((total − offset − len) × rowHeight);
  spacers are plain fixed-height divs (no per-row DOM), so scroll extent equals the complete document.
- A windowed item with `total > 0` and no children is `expandable`; opening it (host `treeOpenStates`) marks it
  `loading` until the next body carries its children.
- `TreeWindowObserver` (one per panel scroll container): on scroll/resize (rAF-coalesced) computes for every windowed
  container in the DOM the intersecting row range ± overscan (`TREE_WINDOW_OVERSCAN_ROWS`), diffs against the last
  request, and calls `onTreeWindowsChange(bodyKey, requests)`; ShellHost merges into `treeWindowsRef`, updates the
  ViewModel field, and triggers the partial panel refresh.
- The old body stays mounted while the next one is in flight (no flash); rows outside the new window unmount.

### 3.5 wgpu target
`🌳️Tree/🎯️targets/🧊️wgpu`: render spacers for `window` the same way; expansion/scroll requests are out of scope for wgpu this ticket (documented gap, no `+N` fallback).

## 4. Row picks without argument maps (decided in principle, sized in §5)
Domain-bound trees (`TreeProps.interaction_domain`) author one 4-entry map per row only to say
`{domainId, merge: replace, method: pick, targets: [{granularity, id: key}]}`. Move that to the contract:
`TreeItemProps.granularity: Option<UiText>`; the host synthesises the `interactionSelect` intent from
`(tree.interaction_domain, item.granularity, item.key, modifiers → merge)`. A pick row then costs nothing of the
argument arena, and shift/ctrl-click multi-select works on every tree for free. Row actions keep their bindings.

## 5. Budgets (pending audits)
- Is `UI_BUILT_CHILDREN_MAX = 32` a wire/struct-size fact or a constant? A window must be able to hold a full viewport (≥ 64 rows).
- Real per-node reconcile bytes; whether the 8 MiB surface cap admits a 64-row window.
- Arena: with §4, remaining arena consumers are row actions only; decide `UI_VALUE_LIVE_PAGES` and whether row actions on
  big trees move to the context menu.

## 6. Work packets (Opus fleet) — filled after audits

## 7. Host facts folded in from 📓️audit-host-tree-pipeline.md
- Guest-tree expansion is today per-mount local React state (`useTreeOpenState` fallback) and is lost whenever a
  refresh re-mints the `UiDocumentStore` (`uiNodeToTreePanelConfig` builds a fresh store per call). The shell's
  `treeOpenStates` is wired only to the always-empty outer `Tree`. ⇒ Expansion becomes host-owned per
  `(bodyKey, nodeKey)` in ShellHost (`treeWindowsRef`), passed into the guest `<Tree>` as controlled `openStates`
  /`onOpenStateChange`, and mirrored into `ViewModel.tree_windows`. This also fixes the lost-expansion bug.
- The real scroll container is the `Panel`'s `Scrollable` viewport (`data-slot="scroll-area-viewport"`), two layers above
  `TreeView`. The observer attaches there (context from `Panel` → `TreeView`), listens to `scroll` + a `ResizeObserver`.
- Row pitch is one token (`treeRowUiSpacing`, React `treeRowHeightPx`, wgpu `TREE_ROW_HEIGHT`).
- `"setPanelPage"` sits in `WINDOW_CONFIG_RAIL_ACTION_IDS` (ShellHelpers ~4654) forcing a FULL refresh — delete with the action.
- Panel refresh is whole-body, hash-conditional per body key (`buildUiRefreshRequest` / `SET_PANEL_UI_BY_KEY`). A window
  change therefore costs one `refreshUi(session, { kind: "partial", panelBodies: [bodyKey] })`; the guest answers with the
  new body only for that key.
- `useMemo`s over the whole `panelUiByKey` rebuild every tab's tree on any body change (ShellHost ~8727/8779/8801). Narrow to
  per-tab memoisation so a streamed window on one tab does not re-parse its siblings.
- Host-side `UiDocumentLimits.max_children = 4096` already admits a long flat section; the 32 is guest-builder only (pending §5).
