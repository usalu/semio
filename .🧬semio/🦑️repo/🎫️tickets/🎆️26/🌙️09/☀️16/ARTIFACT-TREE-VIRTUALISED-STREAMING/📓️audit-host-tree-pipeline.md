# Audit: host-side pipeline for a guest-authored `BuiltNode` panel tree

Read-only investigation. All paths relative to repo root `/Users/ueli/Documents/semio`. File emoji names verified by direct `ls`/`grep`.

## 1. From `BuiltNode` snapshot to the rendered `Tree`

**`uiNodeToTreePanelConfig`** — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2051`

```
export function uiNodeToTreePanelConfig(node: BuiltNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  const store = new UiDocumentStore(`panel:${node.key}`);
  store.loadSnapshot(builtNodeToSnapshot(`panel:${node.key}`, node));
  return {
    sections: [],
    emptyState: (
      <ShellFaultBoundary boundaryId={`panel-${node.key}`} fallbackLabel={shellLabel("ui.common.renderError")}>
        <div className="min-h-0 min-w-0 w-full flex-1">
          <InterpretedUiNode store={store} onAction={onAction} onIntent={(intent) => onAction(uiIntentToActionDescriptor(intent))} />
        </div>
      </ShellFaultBoundary>
    ),
    className: "min-w-0 w-full",
    sortableSections: false,
  };
}
```

Key facts:
- A **brand-new `UiDocumentStore` is constructed on every call** (not memoized inside this function itself — see §4 on how often it's actually called).
- The whole guest body is hosted as the outer `TreePanelConfig`'s `emptyState`, with `sections: []`. The doc comment above it (line ~2038) explains why: parking the interpreted body in a `TreeDataItem.control` broke property-layout rows (ticket 26/08/01). This means the **outer** `<Tree>` that `TreePanelConfig` eventually feeds (see §2) never has real sections/items for a guest tree — it always falls into its own `{resolvedSections.length === 0 && emptyState}` branch.
- Caller: `panelTabDefinitionToNode`, same file, line **1904**: `tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tabId] ?? pendingPanelUiNode(), onAction))`.

**`builtNodeToSnapshot`** — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx:379`. Does a fresh pre-order DFS mint of every node, `nextId = 1, 2, 3…`, over the **entire** `BuiltNode` tree the guest returned — there is no partial/lazy walk here; whatever the guest put in `node.children` is fully minted into `UiNodeRecord`s up front.

**`UiDocumentStore`** — same file, class at line **411**.
- `loadSnapshot` (line 436): replaces the whole internal `UiDocumentState` wholesale — explicitly documented as *not* the validated/incremental path (`"a UiSnapshot ... is not the untrusted-input boundary this contract defends"`).
- `applyPatch` (line 446): a transactional, **fine-grained** patch API that notifies only the per-node listeners whose record reference actually changed. This exists in the contract, but `uiNodeToTreePanelConfig` never uses it — panel trees always go through `loadSnapshot`, i.e. whole-body replace, never `applyPatch`. (Patch application is exercised elsewhere for live/continuous retained surfaces — `RetainedUiPatchCursor`/`RetainedUiSurfaceOwner`, same file — not for panel tabs.)

**`InterpretedUiNode`** — corrected location vs. the initial guess: it is **not** under `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`; it lives in the **os renderer's own Interpreter element**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1791`.
```
export const InterpretedUiNode = memo(function InterpretedUiNode({ store, onAction, onIntent, requestContextMenu }) {
  const surfaceActions = usePluginSurfaceActions();
  const root = useUiDocumentRoot(store);
  if (root === null) return null;
  return <UiNodeView store={store} id={root} context={{ store, onAction, onIntent, requestContextMenu: requestContextMenu ?? surfaceActions }} />;
});
```
`memo`'d, but its only real re-render trigger is `store`'s own identity (stable per call to `uiNodeToTreePanelConfig`, i.e. **not** stable across successive host-driven refreshes — a new store is minted each time `uiNodeToTreePanelConfig` runs) plus `useUiDocumentRoot(store)` (root id). Node-level updates below the root are handled by `UiNodeView`'s own per-id `useUiNode` subscription (line 1762), which is the flat-table "each mutation re-renders exactly its own node" mechanism the header doc for this file describes.

`renderComponent` (line 1729) switches on `record.component.type`; `case "tree": return <TreeView store={store} record={record} context={context} />;` (line 1752).

**`TreeView`** (guest tree, Interpreter file, line **1474**):
```
function TreeView({ store, record, context }) {
  const revision = useUiDocumentRevision(store);
  const overlay = useUiPresenceOverlay();
  ...
  const sections = useMemo((): TreeDataSection[] => {
    const state = store.getState();
    const sectionRecords = (record.children ?? []).map(id => state.nodes.get(id)).filter(r => r?.component.type === "treeSection");
    return sectionRecords.map(sectionRecord => ({
      id: uiNodeDomId(...), label: ..., defaultOpen: sectionProps.defaultOpen ?? undefined,
      loading: ..., waiting: ...,
      items: items.map(item => treeItemToTreeData(store, state, item, context, overlay, leftoverIds)),
    }));
  }, [store, record, revision, context, overlay, leftoverIds]);
  ...
  return (
    <Tree
      className="min-h-0 min-w-0 flex-1 overflow-auto"
      sections={sections.length > 0 ? sections : [treeStatusSection(store, record)]}
      selectionMode="single"
      showLines
      dragAndDropController={dragController}
      sortableSections={sections.length > 1}
    />
  );
}
```
`treeItemToTreeData` (line 1445) **recurses eagerly and synchronously over the whole subtree already present in the (fully-minted) snapshot**:
```
items: childItems.length > 0 ? childItems.map((child) => treeItemToTreeData(store, state, child, context, overlay, leftoverIds)) : undefined,
```
It maps each `treeItem` `UiNodeRecord` to a `TreeDataItem`: `label`, `description`, `icon` (via `resolveControlIconNode`), `defaultOpen` (from the node's own authored `Component::treeItem.defaultOpen`), `isSelected`/`isHighlighted` (from a presence overlay keyed by `record.key`), `loading`/`waiting` (from `record.activity`), `isHidden` (dimmed), `draggable`/`dragData`, `control` (non-`treeItem` children mounted live via `UiNodeView`, e.g. an inline "Undo" button), `onClick`/`onPointerEnter` (bound from `record.bindings`), and `actions` (from `props.rowActions`, each wired to `context.onIntent(context.store.buildIntent(record, action.action))`).

**`Tree`/`TreeDataItem`/`TreeDataSection`** — `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` (4369 lines total). Interfaces at lines **697** (`TreeDataItem`) and **733** (`TreeDataSection`); `Tree` component body at line **3125**; row components `TreeSection` (1498), `TreeItem` (2069), and the "data-driven" render path used by `TreeView` above, `TreeDataSectionView`/`TreeDataItemView` (~2950-3113).

Row expansion: `useTreeOpenState` (line 116) — see §2.

**Existing lazy-children hook that the guest path does NOT use.** `TreeDataItem`/`TreeDataSection` both carry an optional `getItems?: () => Promise<TreeDataItem[]>` (lines 705, 738). `TreeDataItemView`/`TreeDataSectionView` call `loadItemItems`/`loadSectionItems` (`Tree`'s own `useCallback`s, lines 3231/3247) from a `useEffect` gated on `hasDynamicChildren && treeOpenState.open` (lines 2964-2978 for items, 3062-3072 for sections) — i.e. **the `Tree` element already has a per-item/per-section async "fetch my children on first expand" primitive**, entirely local to the React tree (no host/guest round trip wired to it). `treeItemToTreeData`/`TreeView` never populate `getItems` — they always populate `items` directly and synchronously from the already-fully-materialized snapshot. This is the natural seam for lazy-loading children, but it is currently unused by the guest-tree path.

## 2. Expansion state ownership

There are **two independent, non-interacting** expansion mechanisms in the codebase, and the guest-tree path uses neither of the "host-owned" ones:

**(a) Shell-owned `treeOpenStates`** (`ShellState.layout.treeOpenStates`, reducer action `SET_TREE_OPEN_STATE`) — used exclusively by the **legacy manifest-tree path** (`uiTreeNodeToTreePanelConfig`/`PanelTreeNode`, Interpreter file ~line 261) via `Panel`'s `onTreeOpenStateChange`/`treeOpenStates` props (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx:217,250,277`) and `PanelTreeUnitsPane` (same file, ~232-300), which feeds the **outer** `<Tree openStates={unitOpenStates} onOpenStateChange={onUnitOpenStateChange} sections={config.sections} emptyState={config.emptyState} .../>` (line ~283).
  - For a guest `BuiltNode` panel, `config.sections` is always `[]` (see §1), so this outer `Tree`'s `openStates`/`onOpenStateChange` are **wired but inert** — no `TreeSection`/`TreeItem` ever mount under it to read them, only `emptyState` renders.
  - `treeOpenStates` is also captured/replayed by the tutorial recorder: `captureTutorialUiSnapshot` (`ShellHelpers/🟦️.tsx:1041`, `expandedTreeIds: Object.entries(state.layout.treeOpenStates)...`), diffed by `diffTutorialUiSnapshot` (`ShellHost/🟦️.tsx:1071-1072`, emitting `{kind:"treeExpansion", id, expanded}`), and replayed by `applyTutorialUiChangeToShell`'s `case "treeExpansion": dispatch({type:"SET_TREE_OPEN_STATE", id: change.id, open: change.expanded})` (`ShellHelpers/🟦️.tsx:2691-2692`). This is a **host-only, tutorial-recorder concept** — it never reaches the guest and never touches the inner guest `<Tree>` described in (b).

**(b) The guest tree's own `<Tree>` (`TreeView`, Interpreter file line 1474)** does **not** pass `openStates`/`onOpenStateChange` to `<Tree>` at all. Each row's `useTreeOpenState(itemId, defaultOpen)` (line 116) therefore falls back to **local, uncontrolled React state** (`useState(defaultOpen)`, line 118) inside whichever `TreeStateProvider` context is in scope — and since the enclosing outer `Tree` (a) always wraps its children in its own `TreeStateProvider` with `openStates=undefined` (`Tree`'s body, line 3582, since `TreeView`'s inner `<Tree>` is rendered inside that outer `emptyState`), the guest tree's expand/collapse state is a **fresh, per-mount, host-only React state map with no connection to shell state, no persistence across a full-body `loadSnapshot` refresh (see §4 — the store is discarded and reminted), and no signal sent back to the guest**.

**Does the guest learn about expansion? No.** Confirmed at the contract level: the `Trigger` enum (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:1691`) is a closed set — `Activate, Change, Commit, Delta, Drop, Submit, Abort, RepeatLast, HoverPreview` — there is no `Expand`/`Toggle`/`Scroll`/`Viewport` trigger. Grepping `🧰️framework/🔨️modules/🖱️ui/🧬️contract` and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`/`🧰️framework/🔨️modules/🧬️schema/📽️projection` for `treeExpansion`/`TreeExpansion` finds **no Rust hits at all** — the term is purely a TypeScript/tutorial-recorder concept (`TutorialUiChange`), unrelated to the wasm guest wire.

## 3. Scrolling and row sizing

Scroll container chain for a floating/docked `Panel`'s tab body (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx:534-538`):
```
<Scrollable className="relative flex-1 min-h-0" viewportClassName={isBottom ? "flex min-h-full flex-col justify-end" : undefined}>
  {activeTabTrees && bodyLeaf ? <PanelTreeUnitsPane anchor={anchor} tabId={bodyLeaf.id} units={activeTabTrees} treeOpenStates={treeOpenStates} onTreeOpenStateChange={onTreeOpenStateChange} treeContentRevision={treeContentRevision} /> : null}
</Scrollable>
```
`Scrollable` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🟦️.tsx:20-46`) is a plain native-overflow `<div>` (explicitly documented as avoiding Radix `ScrollArea` ref-update loops): `overflow-y-auto overflow-x-hidden` on the outer div, a plain `min-h-0 min-w-0 w-full` inner `data-slot="scroll-area-viewport"` div with **no scroll position tracking, no `IntersectionObserver`, no `ResizeObserver`** anywhere in the file.

`PanelTreeUnitsPane`'s outer `<Tree>` root div (`🧱️elements/🌳️Tree/🟦️.tsx:3591`, class `w-full min-w-0 overflow-hidden ${className}`) does not itself scroll (natural height, `overflow-hidden`). The **guest** inner `<Tree>` from `TreeView` is given `className="min-h-0 min-w-0 flex-1 overflow-auto"` (Interpreter file line ~1497) — but because its parent chain up to `Scrollable`'s viewport is a natural-height (non-flex) stack, `flex-1`/`min-h-0` have no effect there in practice; the **outer `Panel`'s `Scrollable` is the real, bounded scroll container** for a guest tree panel, not the guest `<Tree>`'s own root div. (Worth confirming live in a browser probe before relying on it, but it follows directly from the CSS/DOM nesting read here.)

Row height: `treeRowHeightPx = domSizePx("treeRowUiSpacing")` (`🧱️elements/🌳️Tree/🟦️.tsx:212`), token `treeRowUiSpacing: 7.5` (× the compact spacing unit) in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json:359` and the generated token file `🎨️styling/🤖️generated/🔤️tokens/🟦️.ts:352`. The **same** token drives the wgpu target's `TREE_ROW_HEIGHT` (see §6), so row pitch is already a single shared constant usable for a fixed-row-height virtualiser on both targets.

No virtualization/windowing primitive exists anywhere in `🧰️framework/🔨️modules/🖱️ui` or the os renderer's `📺️renderer` tree: repo-wide `grep` for `IntersectionObserver`/`ResizeObserver`/`react-window`/`react-virtual`/`virtualiz` inside `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` returns nothing; the same terms across the whole `🖱️ui` module return only unrelated files (`Popover`, `Select`, `Diagram`, `Navbar`, `Window`, a bundled `pdf_viewer.mjs` from `node_modules`) and across the os renderer only scene-host / wgpu-boot files (`Board2dHost`, `Canvas2dHost`, `TiledMapHost`, `Paint2dHost`, the wgpu browser-boot loader) — none of it panel/tree related.

## 4. Action round trip (guest ⇄ host ⇄ guest)

A tree row's `onClick`/`actions[].onClick` calls `context.onIntent(...)` (a `UiIntent`) or directly `context.onAction(...)` (an `ActionDescriptor`). `uiIntentToActionDescriptor` (`ShellHelpers/🟦️.tsx:2039`) bridges a `UiIntent` onto an `ActionDescriptor` (`{controllerId, action, args}`); `uiNodeToTreePanelConfig` wires `onIntent={(intent) => onAction(uiIntentToActionDescriptor(intent))}` so every row action ultimately becomes one `onAction(ActionDescriptor)` call into `ShellHost`'s dispatch machinery (outside this file's scope, but reached uniformly regardless of trigger kind).

The panel-body refresh cycle is driven by `refreshUi` (`ShellHost/🟦️.tsx:5105`), which coalesces requests through a lane (`uiRefreshLaneRef`) and calls `program.refreshUi(instanceId, request)` (line ~4904) / `plugin.refreshUi(spawned.instanceId, request)` (line ~5192) — i.e. a message into the **wasm guest** (PluginRuntime's `refreshUi` entry point), not a per-node diff request. `buildUiRefreshRequest(scope, windows, panels, viewState, cache)` builds one **batched, hash-conditional** request across every window and panel section; the guest's response (`PluginUiRefreshSectionResponse[]`) omits `value` for any section whose hash still matches what `cache` already holds (comment at line ~4886: *"the plugin omits payloads for any section whose hash still matches what `cache` already holds"*).

Applying the response (`applyUiRefreshResponseToCache`, then `dispatch({type: "SET_PANEL_UI_BY_KEY", ...})`, `ShellHost/🟦️.tsx:4991-4998`) is **whole-panel-body replacement**: each panel key maps 1:1 to one cached `BuiltNode` value (`cache.get(\`panel:${panelTabKindId(tab.kind)}\`)?.value`), reused by reference (`mergeRecordPreservingIdentity`) when the hash didn't change, replaced wholesale when it did. **The round trip is whole-panel replacement, not a diff** — there is no per-tree-node patch coming back from a panel refresh (contrast with `UiDocumentStore.applyPatch`, which exists in the contract for continuous/live retained surfaces but is never invoked on this path — see §1).

Because `uiNodeToTreePanelConfig` mints a **fresh `UiDocumentStore` and calls `loadSnapshot`** every time it runs (§1), and because the surrounding `useMemo`s that call `panelTabDefinitionToNode` (e.g. `ShellHost/🟦️.tsx:8727,8779,8801`) list the **whole** `panelUiByKey` record as a dependency, **any single panel body changing invalidates and re-parses every panel tab's tree**, not just the one that changed — each such rebuild also throws away whatever local expand state existed in the previous `<Tree>` instance's `TreeStateProvider` (a fresh store/mount), unless React happens to keep the same `<Tree>` component instance alive across the rebuild (same component identity, new props) — in which case `TreeStateProvider`'s own `openStates` map (component-local `useState`, `🧱️elements/🌳️Tree/🟦️.tsx:84-108`) does survive, since it isn't reset by a new `BuiltNode`/snapshot per se, only by an actual remount.

Typical latency path for one row click: `onClick` → `onAction(ActionDescriptor)` → ShellHost's action dispatch (not read in this pass) → guest actor turn → (if the action is in `WINDOW_CONFIG_RAIL_ACTION_IDS`, `ShellHelpers/🟦️.tsx:4649-4671`, which includes `"setPanelPage"`) a synthesized `{kind:"full"}` `UiDirtyScope` → `refreshUi(session, {kind:"full"})` → `program.refreshUi` (worker/wasm boundary) → hash-conditional response → cache → `SET_PANEL_UI_BY_KEY` → `uiNodeToTreePanelConfig` reruns → new `UiDocumentStore`/`loadSnapshot` → `InterpretedUiNode`/`TreeView` re-renders the whole guest subtree for that panel.

## 5. Existing "+N" paging (what this ticket replaces)

`panel_page_rows()` / `PanelRowBudget` / `paged_panel_section` / `panel_continuation_row` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5888-5995` (re-exported at line 39485).
- `panel_page_rows()` (5888) returns `UI_VALUE_PAGE_ROWS.min(ui_value_headroom().rows())` — the page size is a **process-wide `UiValue` argument-arena budget**, not a viewport/row-count computed from the panel's rendered height. It is a resource-admission limit, unrelated to how many rows physically fit on screen.
- `paged_panel_section` (5950) walks `entries`, spends 1 budget unit per row via `PanelRowBudget::spend()`, and stops early — on quota exhaustion or on an admission refusal (`error.code == "ui.fixed-capacity"`) — appending `panel_continuation_row(section_id, omitted)` (5933), a `tree_item` whose label is literally `"+{omitted}"` with icon `"more-horizontal"`.
- The continuation row's action wiring is app-specific (not in this file); `"setPanelPage"` is registered host-side in `WINDOW_CONFIG_RAIL_ACTION_IDS` (`ShellHelpers/🟦️.tsx:4654`), which forces a **full** shell refresh (`browserActorWindowConfigDispatchUiScopeV1`) whenever that action's actor-dispatch reports `mutationCount: 0` (a page-number bump alone doesn't touch the document, so it would otherwise not trigger any refresh at all).
- Contract limit context: `UiDocumentLimits.max_children = 4096` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs:39-41`, doc: *"covers the largest legitimate flat list (an unpaginated tree section...)"*) and `max_nodes = 20000` (line ~30, doc: *"comfortably covers the largest known tree (a fully expanded product tree view or timeline)"*) — these are the ceilings the current "ship everything, then page/continuation-row" model is implicitly designed against.

## 6. wgpu target of `Tree`

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs` (257 lines). Mirrors the React data-driven path structurally, not virtualized either:
- `render_tree` (line 115) iterates every `TreeSection`, calling `render_tree_section_header` then, for a non-collapsed section, `render_tree_item` for every top-level item (line 127).
- `render_tree_item` (line 152) recurses into every child (`render_tree_item(child, ...)`, line 253) when the item is not collapsed, using a plain `collapsed: &HashMap<String, bool>` passed down from the caller — the **wgpu target's own client-local expand-state map**, independent from and unaware of both host mechanisms in §2.
- Row pitch: `TREE_ROW_HEIGHT` (imported from `crate::wgpu::widgets`, defined in `🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:192`, doc: *"the ONE tree row pitch, straight off `dom.treeRowUiSpacing`"*) and `🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:73` (*"the same `--size-workbench` React's Tree rows use"*) — same design-token source as the React side (§3), so a virtualisation contract field mirrored here has an existing, shared row-height constant to key off.
- `measure_tree_item_height`/`measure_tree_sections_state` (lines 100, 84) compute total content height by walking every (non-collapsed) row — again, full-tree walk, no windowing.

## Design constraints & hooks for virtualisation

Facts only — no proposed design — but these are the concrete places a virtualised/streaming tree would need to touch on the host:

1. **Snapshot ingestion is whole-body, not incremental.** `uiNodeToTreePanelConfig` (`ShellHelpers/🟦️.tsx:2051`) always does `new UiDocumentStore(...)` + `loadSnapshot(builtNodeToSnapshot(...))` — a fresh sequential re-mint of every node on every call, discarding the prior store. A lazy-children/viewport model needs either (a) a persistent store across refreshes (keyed stably, not reminted per call) plus `applyPatch`-style incremental updates, or (b) an explicit contract for partial subtrees that `builtNodeToSnapshot` can mint against.
2. **The one existing lazy-children primitive is unused by the guest path.** `TreeDataItem.getItems`/`TreeDataSection.getItems` + `Tree`'s own `loadItemItems`/`loadSectionItems` (`🧱️elements/🌳️Tree/🟦️.tsx:3231-3266`, triggered on expand at lines 2964-2978 / 3062-3072) is the natural per-row "fetch children on demand" seam already built into the `Tree` element; `treeItemToTreeData` (Interpreter file line 1445) would need to populate `getItems` (an async host→guest round trip) instead of eagerly walking `record.children` recursively.
3. **Expansion state must move from purely-local to host-observable (or guest-observable).** Today `useTreeOpenState` (`🌳️Tree/🟦️.tsx:116`) is either the inert shell `treeOpenStates`/`SET_TREE_OPEN_STATE` path (dead for guest trees — wired only through the empty-sections outer `Tree`) or genuinely local per-mount React state (guest trees, via `TreeStateProvider`'s own `openStates`, `🌳️Tree/🟦️.tsx:84-108`). Either channel could be extended to notify a host callback per node id/authored `key` (`uiNodeDomId`, Interpreter file line 1399) on toggle — there is currently no code path from a row's chevron click to anything the guest or a viewport tracker can see.
4. **No trigger exists in the contract for expand/toggle/scroll/viewport.** `Trigger` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:1691`) is closed at 9 variants; adding a host→guest "which rows are visible" or "this row expanded" signal needs either a new `Trigger` variant + `ActionBinding`, or a dedicated side-channel outside the `Trigger`/`ActionBinding` model (the refresh request already carries a `ViewModel`/`UiDirtyScope`, `ShellHost/🟦️.tsx:~4870-4905`, which is one plausible carrier for a per-panel visible-range hint).
5. **The actual scroll container is the ancestor `Panel`'s `Scrollable`** (`🖼️Panel/🟦️.tsx:534`), not the guest `<Tree>`'s own root `<div>` — a viewport/visible-range observer needs to attach to that `Scrollable`'s DOM node (or its `data-slot="scroll-area-viewport"` inner div), which is two component layers above `TreeView`/`InterpretedUiNode`. No `ResizeObserver`/`IntersectionObserver`/scroll-position tracking exists anywhere in `Scrollable` or `Tree` today — this would be new code, not a hook into existing plumbing.
6. **Row height is a single shared design token already used identically by both targets** — `treeRowUiSpacing` (`🎨️styling/🔣️.json:359`) → `treeRowHeightPx` in React (`🌳️Tree/🟦️.tsx:212`) and `TREE_ROW_HEIGHT` in wgpu (`🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:192`) — a fixed-row-height virtualiser can rely on one authoritative constant across both renderers.
7. **The "+N" paging today is a resource-budget artifact, not a viewport computation** (`panel_page_rows`/`PanelRowBudget`, `🔌️plugin/🦀️.rs:5888-5995`) — it truncates based on the process-wide `UiValue` argument arena, with no knowledge of how many rows the panel can actually display. Replacing it with true virtualisation removes the coupling between "how much host memory a page of interactive rows costs" and "how many rows are shown," which today are the same knob.
8. **A whole-panel-body refresh currently invalidates every open tab's tree at once** (`ShellHost/🟦️.tsx` `useMemo`s over the full `panelUiByKey` record, e.g. lines 8727/8779/8801, feeding `panelTabDefinitionToNode`/`uiNodeToTreePanelConfig`) — any per-tab or per-node granularity for virtualised loading will also need to narrow this dependency, or a lazy/paged tab will keep getting rebuilt by unrelated sibling-tab updates.
9. **wgpu mirror needs the same contract field.** Whatever new field/flag drives host-side lazy loading and visible-range reporting in the React `TreeDataItem`/`TreeDataSection` types (`🌳️Tree/🟦️.tsx:697,733`) has a structurally parallel spot to mirror in the wgpu target's `TreeItem`/`TreeSection` structs and `render_tree_item`/`render_tree` (`🎯️targets/🧊️wgpu/🦀️.rs:115,152`), which currently assume full recursive materialization gated only by a local `collapsed: HashMap<String,bool>`.
