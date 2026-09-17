# 📓️ P4b — Interpreter + ShellHost/ShellHelpers wiring (design §6.2, §6.3)

Wave-1 packet P4b, implemented 2026-09-17. Normative source: `📓️design-virtualised-tree.md` §6.2/§6.3;
map of the existing pipeline: `📓️audit-host-tree-pipeline.md`.

Consumes P4a (`🌳️Tree` element: `TreeDataItem/Section.window`/`windowKey`, the `data-tree-window-*`
attributes, spacers, `treeWindowRequestsForViewport`, `TREE_WINDOW_OVERSCAN_ROWS`,
`TREE_WINDOW_ROWS_MAX`) and P1 (`TreeWindow` + `window`/`granularity` on the tree props in the
generated `📜️ui-contract` TS; `PluginViewState.treeWindows`/`treeViewportRows` +
`ViewTreeWindowRequest` in `🛂️manifest/🟦️.ts`). Both landed before the final type-check.

---

## 1. The context API

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`, region
`🪟️TreeWindows` (~1490–1700):

```ts
export type TreeWindowReportV1 = { readonly nodeKey: string; readonly offset: number; readonly rows: number };

export type TreeWindowContextValue = {
  readonly bodyKey: string;
  readonly openStates: Readonly<Record<string, boolean>>;                      // AUTHORED node key → open
  readonly setOpen: (nodeKey: string, open: boolean) => void;                  // AUTHORED node key
  readonly reportWindows: (requests: readonly TreeWindowReportV1[], viewportRows: number) => void;
};
export const TreeWindowContext = createContext<TreeWindowContextValue | null>(null);
export function useTreeWindowContext(): TreeWindowContextValue | null;
```

**Where it lives and why.** In the Interpreter, not in `🛠️ShellHelpers`. The import edge already runs
ShellHelpers → Interpreter (`InterpretedUiNode`, `wireLabel`), so the provider side imports the context
and the consumer side declares it; the reverse would have needed a third module for one type. (The two
files are already mutually importing — the Interpreter pulls `shellLabel` back — so no NEW cycle is
introduced either way; this direction is simply the one where the consumer owns the declaration.)

**Key identity.** Everything crossing this boundary is the AUTHORED node key (`UiNodeRecord.key`), never
`uiNodeDomId`'s `<surface>/<key>` and never the DFS-minted `UiNodeRecord.id`. That is what lets the
host's map survive a body refresh, a `UiDocumentStore` re-mint and the renumbering both cause — the
defect §2 of the audit describes (expansion was per-mount React state and died on every refresh).

**Two DOM-id translations happen inside `TreeView`, nowhere else:**

1. `uiNodeDomId(surface, key, id)` — authored key → `<Tree>` row id.
2. `🌳️Tree` keys its own expansion map by a ROLE-PREFIXED form of that row id
   (`getTreeSectionStateId` → `tree-section-<id>`, `getTreeItemStateId` → `tree-item-<id>`), and neither
   helper is exported. `treeOpenStateIdsForDomIdV1(domId)` therefore writes all three spellings into the
   controlled map, and `treeDomIdFromOpenStateIdV1(stateId)` strips the prefix on the way back. This is
   unambiguous because a panel DOM id is always `panel:…/…` and can never itself begin with one of the
   prefixes. **This is the one place P4b duplicates a private convention of the `🌳️Tree` element** — see
   open issue O1.

`TreeView` reads the context with `useTreeWindowContext()`. When it is `null` (stories, fixtures, the
wgpu mount, any `uiNodeToTreePanelConfig` call without a host channel) `openStates`/`onOpenStateChange`
are **not** passed to `<Tree>` at all and the element keeps its existing uncontrolled per-mount state —
a covered test case, so a non-shell mount behaves exactly as before this packet.

## 2. `treeItemToTreeData` / `TreeView`

- `treeItemToTreeData` is now **exported** (it is a unit under test) and takes one new optional trailing
  parameter, `walk?: TreeWalkContextV1`. Everything else in its signature is unchanged.
- It stamps `window: props.window ?? undefined` and `windowKey: record.key || undefined`; the section
  mapper in `TreeView` does the same from `TreeSectionProps.window`. An unwindowed row stamps neither,
  so P4a renders no spacers and no `data-tree-window-*` for it.
- `TreeWalkContextV1` is filled WHILE walking and shared by every row's closures, so a row converted
  early can still resolve a pick against ids the walk had not reached when that row was built:
  `openStates` (in), `domOpenStates` (out, the controlled map), `keysByDomId` (out, the inverse),
  `pick` (the tree root's own `activate` binding, when it has one) and `pickTargets`
  (out, DOM id → `{key, granularity}`).
- `TreeView` returns `{ sections, walk }` from ONE `useMemo` (previously `sections` alone) and wraps
  `<Tree>` in `<div ref={rootRef} className="contents">`. `display: contents` generates no box, so the
  `Panel`/`Scrollable` layout chain above and the `Tree` root's own classes below are unchanged; the
  wrapper exists solely because `<Tree>` exposes no ref and the observer needs a DOM handle.

## 3. Observer behaviour

`useTreeWindowObserver(rootRef, windows, revision)` (Interpreter, mounted from `TreeView`):

- **Viewport.** `treeWindowScrollViewport(root)` → `root.closest('[data-slot="scroll-area-viewport"]')`,
  else the nearest ancestor whose computed `overflow-y` is `auto`/`scroll`. Per audit §3 the real
  bounded scroll container for a guest tree is the ancestor `🖼️Panel`'s `📜️Scrollable` viewport, two
  component layers above `TreeView`, not the guest `<Tree>`'s own `overflow-auto` root.
- **Triggers.** One measurement on mount, a `scroll` listener (`passive`) and a `ResizeObserver` on the
  viewport, all coalesced through one `requestAnimationFrame` handle (falls back to `setTimeout(…, 0)`
  where rAF is absent, e.g. jsdom). The effect's dependency list carries the store `revision`, so it
  re-attaches and re-measures after every body refresh — a refresh changes what is there to measure.
- **Measurement.** `treeWindowContainersUnder(root, viewport)` reads every `[data-tree-window-key]`
  element with `getBoundingClientRect()` and reports `top` in the viewport's own scroll-content space
  (`rect.top − viewportRect.top + viewport.scrollTop`), paired with `viewportTop = viewport.scrollTop`.
  Only the DIFFERENCE `viewportTop − top` reaches `treeWindowRequestsForViewport`, so this is identical
  to the viewport-relative reading and cannot disagree with P4a's helper. Containers with
  `data-tree-window-total <= 0` are skipped (asking for rows that do not exist).
- **Row pitch.** `treeWindowRowHeightPx()` = `domSizePx("treeRowUiSpacing")`, the same design token the
  `🌳️Tree` element's own module-private `treeRowHeightPx` and the wgpu target's `TREE_ROW_HEIGHT` are
  computed from (audit §6). Clamped to ≥ 1 so the row arithmetic can never divide by zero.
- **Reporting.** `treeWindowRequestsForViewport(containers, scrollTop, viewportHeight, rowHeight,
  TREE_WINDOW_OVERSCAN_ROWS)`, mapped `key → nodeKey`, plus
  `viewportRows = max(1, ceil(viewportHeight / rowHeight))`. Diffed against the previous report through
  `treeWindowReportSignatureV1`; an identical answer calls nothing. Without that diff, the revision
  dependency alone would make every refresh re-report, and every report would request another refresh.

## 4. Scheduling rules

`createTreeWindowSchedulerV1` (`🛠️ShellHelpers/🟦️.tsx`, region `🪟️TreeWindows`) — pure of React, with
time injected (`setTimer`/`clearTimer`), so all of it is testable without a DOM or fake timers.

| gesture | rule |
| --- | --- |
| `setOpen(body, node, open)` | value already equal → no-op. Otherwise update + **flush immediately** (a chevron must not wait on a debounce). |
| `reportWindows(body, requests, rows)` | windows map and viewport rows both unchanged → no-op. Otherwise update + arm a **40 ms trailing debounce** (`TREE_WINDOW_REPORT_DEBOUNCE_MS`). |
| in flight | at most **one** partial refresh per body. A body that moved while its refresh was crossing stays `pending` and is re-sent — with the LATEST state — when the host calls `settled(bodyKey)`. |
| `reset()` | drops every body, the pending/in-flight sets and `treeViewportRows`. |

`viewStateFields()` flattens every body's `open ∪ windows` into `ViewTreeWindowRequest[]`, sorted by
(bodyKey, nodeKey) so the request is byte-stable across renders, and emits `treeViewportRows` (the max
over reporting bodies — the design's "rows the tallest visible panel body fits"). A node with an OPEN
toggle but no measurement yet asks for `treeViewportRows ?? TREE_WINDOW_DEFAULT_ROWS` (48, mirroring the
SDK constant) rather than `rows: 0`, which under §5's `slice` would materialise nothing and leave the
row the chevron just promised permanently absent. A node explicitly closed asks for `rows: 0`.

`ShellHost` owns the `settled` half: its `refresh` closure calls
`refreshUi(session, { kind: "partial", panelBodies: [bodyKey] })` — the existing lane, the existing
scope — and `.finally(() => scheduler.settled(bodyKey))`.

## 5. Pick synthesis encoding

A row gets a synthesised pick when it has `props.granularity`, has **no** `activate` binding of its own,
and the tree root record **does** have one (the SDK's tree-level `interactionSelect`, bound once by
`PanelTreeBuilder::interaction_domain`). Precedence on `onClick` is: own `activate` binding → pick →
activatable inline control. A row with its own binding is untouched (covered by a test).

```ts
merge = interactionMergeFromModifiers(event)                 // the ONE shared modifier→merge policy
targets = merge === "range" ? ctx.selectedIds mapped through walk.pickTargets, then the clicked row
                            : [{ granularity, id: record.key }]
input   = { merge: merge === "range" ? "replace" : merge, method: "pick", targets: JSON.stringify(targets) }
context.onIntent(context.store.buildIntent(rootRecord, rootActivateBinding, input))
```

- `targets` is a **JSON string** of `{granularity, id}` records and `method` is `"pick"` — byte-for-byte
  the shape `world3dSelectionActionArgs` puts on the wire (`🌐️World3dHost/🟦️.tsx:5325`), minus
  `domainId`: the tree's own binding already carries `{domainId}` in its authored args and
  `uiIntentPayload` merges this map OVER them.
- `merge` is a raw `MergeMode` word, never translated — the framework's `parse_merge_mode` accepts
  exactly the five schema words and faults on anything else.
- `"range"` has no ordered topology on the wire, so the host resolves it into the full id set and sends
  `"replace"`. `ctx.selectedIds` is the tree's NEXT selection (`handleSelectItem` passes
  `nextSelection.selectedIds`, already including the clicked row); the clicked row is appended anyway as
  a floor, so a selection the table cannot resolve still picks something.
- Targets are deduplicated on `(granularity, id)`, mirroring `world3dSelectionActionArgs`' own id set —
  "a `Select` must be idempotent per id".

Exported for tests: `treePickTargetsV1`, `treePickIntentInputV1`.

## 6. Files and regions touched

| file | what |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` | new region `🪟️TreeWindows` (~1490–1700): context, `treeWindowRowHeightPx`, `treeWindowScrollViewport`, `treeWindowContainersUnder`, `treeWindowReportSignatureV1`, `useTreeWindowObserver`, `TreeWalkContextV1`, `treePickTargetsV1`, `treePickIntentInputV1`, `treeOpenStateIdsForDomIdV1`/`treeDomIdFromOpenStateIdV1`, `registerTreeWalkRow`. `treeItemToTreeData` exported + `walk` param + `window`/`windowKey`/pick click. `TreeView` rewritten (walk memo, controlled open states, observer, `contents` wrapper). Imports: `useEffect`, `RefObject`, `MouseEvent`, `ActionBinding`, `MergeMode`, `domSizePx`, `interactionMergeFromModifiers`, `treeWindowRequestsForViewport`, `TREE_WINDOW_OVERSCAN_ROWS`, `TreeDataActivationContext`. Test registration for `🪟️tree-windows`. |
| `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | new region `🪟️TreeWindows` (~2130–2250): `TREE_WINDOW_REPORT_DEBOUNCE_MS`, `TREE_WINDOW_DEFAULT_ROWS`, `createTreeWindowSchedulerV1`, `TreeWindowHostV1`, `PanelTreeConfigCacheV1`, `cachedTreePanelConfigV1`, `pendingPanelUiNodeV1`. `uiNodeToTreePanelConfig(node, onAction, bodyKey, treeWindows?)` builds the per-body context value and wraps `InterpretedUiNode` in `TreeWindowContext.Provider`. `panelTabDefinitionToNode(…, treeWindows = null, cache?)` threads the tab's own `bodyKey` (`tab.bodyKey ?? tabId`) and the cache. `"setPanelPage"` deleted from `WINDOW_CONFIG_RAIL_ACTION_IDS`. |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx` | region `🪟️TreeWindows` at ~2315–2355: `treeWindowSchedulerRef`, `refreshUiRef`, `panelTreeConfigCacheRef`, `treeWindowGeneration`, `treeWindowHost`. `refreshUiRef.current = refreshUi` (5167). Session-switch reset inside `runUiRefreshPass` (4891–4892). `...viewStateFields()` spread into the refresh view state (4939) AND `baseDispatchViewState` (6815). The three panel-tab memos (8789/8841/8863) pass `treeWindowHost` + the cache and list `treeWindowHost, treeWindowGeneration` in their deps. |
| `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers `elementSuite("🛠️ShellHelpers", "🪟️tree-windows", "tsx")` — a co-located suite in no include list is a gate that reads green while measuring nothing. |
| `…/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx` | new (4 laws). |
| `…/🧱️elements/🛠️ShellHelpers/🧪️tests/🪟️tree-windows/🟦️.tsx` | new (7 laws). |
| `…/🧪️tests/🔬️engine-contract/🟦️.ts`, `…/🛠️ShellHelpers/⏯️tool-run-panel/🧪️tests/🧩️component/🟦️.tsx` | existing `uiNodeToTreePanelConfig` call sites given the now-required `bodyKey` (7 call sites). |

### Narrowing the `panelUiByKey` memos

`cachedTreePanelConfigV1` keys one cache entry per tab id on `(body node identity, onAction identity,
bodyKey, treeWindows identity, that body's open-state signature)` and returns the previous
`TreePanelConfig` by reference on a hit. The three memos still re-run when the `panelUiByKey` record
changes, but a tab whose own body did not change now keeps its `UiDocumentStore`, its JSX and therefore
its mounted `<Tree>` — audit §8's "a whole-panel-body refresh invalidates every open tab's tree at
once". With a windowed tree that is not merely wasteful: a remount throws away the `<Tree>` instance the
scroll observer is attached to, on every scroll of a sibling panel. `pendingPanelUiNodeV1()` memoises
the pending body, which otherwise minted a fresh object per call and missed the cache forever.

## 7. Tests and results (all run in the foreground)

New laws:

- `🗣️Interpreter/🧪️tests/🪟️tree-windows` (in-source registration, runs at `test long`/`exhaustive`) —
  (a) `treeItemToTreeData` maps `window`/`windowKey` and stamps neither for an unwindowed row;
  (b) a `granularity` row's click dispatches the TREE's binding with
  `{merge:"replace", method:"pick", targets:"[{\"granularity\":\"piece\",\"id\":\"seed-left-001\"}]"}`
  and the tree's authored `{domainId}` args intact; (c) a row with its own `activate` keeps it;
  (d) range resolution + dedupe + verbatim merge words.
- `🛠️ShellHelpers/🧪️tests/🪟️tree-windows` — scheduler: immediate open vs 40 ms trailing coalescing
  (three scroll reports → one refresh carrying the last offset), no-op on an unchanged report,
  one-in-flight + latest-wins re-send with the `TREE_WINDOW_DEFAULT_ROWS` floor, `reset()`. Panel body:
  a host-closed container renders `data-state="closed"` against the author's `defaultOpen: true`; a fold
  reports back under the AUTHORED key; no host channel → uncontrolled fold still works.

| gate | result |
| --- | --- |
| `@semio-tech/framework-renderer-react:lint` | **pass** (`region/host-contract lint passed`) |
| `@semio-tech/framework-renderer-react:typecheck` | 457 errors repo-wide, **0 in any file this packet touches** (Interpreter, ShellHelpers, ShellHost, engine-contract, both new suites). Pre-existing/peer-owned red: 155 in `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement`, 98 in a root `📜️script.ts`, 21 in `🌍️world/🎨️r3f`, etc. (full log: `🗑️generated/p4b/typecheck-2.txt`). Before P1's TS regeneration this file showed 6 errors in the Interpreter for `props.window`/`props.granularity`; all gone. |
| `…test long "ShellHelpers/…/🪟️tree-windows"` | **7 passed** |
| `…test long --testNamePattern="interpreted tree windows"` | **4 passed** |
| `…test long "tool-run-panel"` | **2 passed** |
| `…test long "engine-contract"` filtered to the 6 panel-tree laws | **6 passed** |
| `…test long` (whole corpus) | **killed by the harness at the 300 s budget** — pre-existing: `engine-contract` ALONE also exceeds it. See O3. |
| `@semio-tech/framework-os:test` | fails, **not this packet**: every failure is in the store worker / directory administration / backbone, and that project's `includeSource` lists only `💻️os/🟦️.ts`, `🏪️store/👷️worker`, `🔌️plugin/⚡️effect-backbone` and the browser-bundle handoffs — no file P4b touches is in it. |

## 8. Open issues

- **O1 — the role-prefixed open-state id is duplicated.** `getTreeSectionStateId`/`getTreeItemStateId`
  are private to `🌳️Tree/🟦️.tsx`, which P4b must not edit, so `TREE_OPEN_STATE_ID_PREFIXES` restates
  the convention and the controlled map carries all three spellings of every row. Cleanest fix is for
  the Tree element to export one `treeOpenStateId(kind, id)` (or accept an already-prefixed map) and for
  the Interpreter to call it. Until then a rename inside the Tree element silently unbinds host-owned
  expansion — the ShellHelpers suite's "host-closed container" law is the tripwire.
- **O2 — `TreeSection`/`TreeItem` report one fold TWICE.** The data view owns the controlled `open` prop
  AND the row component calls `useTreeOpenState` again under the same id, so both push through the
  provider. Harmless here (`setOpen` returns on an unchanged value) and documented at the call site, but
  it is a duplicated dispatch in the element, not something P4b fixed.
- **O3 — the renderer-react `test long` corpus exceeds its own 300 s harness budget.** Pre-existing and
  independent of this packet (`engine-contract` alone exceeds it); my suite adds ~20 s. Wave 3 wants a
  green full run, so the corpus needs splitting or a higher level — the harness message says so itself.
- **O4 — `TreeView` still renders `selectionMode="single"`.** A `range` pick therefore resolves to a
  one-element set in practice (`normalizeTreeSelectedIds` truncates). The synthesis is correct and
  tested at the unit level; whether guest trees should become `"multiple"` is an app/design question
  outside §6.2 and was deliberately not changed here.
- **O5 — `treeViewportRows` is the max over reporting bodies, not "the tallest VISIBLE body".** The host
  has no visibility signal per panel body at this layer; a collapsed or hidden panel that reported once
  keeps contributing its number until the session resets. Harmless (it only widens the guest's
  first-paint budget) but not literally the design's wording.
- **O6 — wgpu is unwired**, as §6.4 documents: no host-side window request path for that target.
- **O7 — the concurrent World3dHost change breaks 7 `world3d-interaction` laws**
  (`expected granularity 'handle' to deeply equal 'object'`). Landed in auto-commit `a4cda597ea`
  alongside a +98-line `🌐️World3dHost/🟦️.tsx` edit that is not part of this packet. Flagged for
  whoever owns it.
