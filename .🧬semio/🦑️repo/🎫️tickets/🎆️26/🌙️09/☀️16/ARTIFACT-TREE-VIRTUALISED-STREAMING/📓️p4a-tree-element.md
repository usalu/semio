# 📓️ P4a — the React `Tree` element's virtual windows (design §6.1)

Packet: **P4a**, wave 1. Normative input: `📓️design-virtualised-tree.md` §6.1 (+ §7 for the 128 ceiling),
`📓️audit-host-tree-pipeline.md` §1–§3.

Files touched (all absolute):

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/📖️stories/🧪️.story.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — re-export list only
  (the module's `export { … } from "…/🌳️Tree/🟦️.tsx"` block, so `@semio-tech/ui-react` consumers — i.e. **P4b** —
  can import the new names). No logic there.

Nothing else was edited. No `🗑️generated` folder was swept; no ticket was opened, closed or reopened.

---

## 1. Public API added (`🌳️Tree/🟦️.tsx`, region `🪟️TreeWindow`)

### 1.1 Data props

```ts
/** The materialised slice of a logically `total`-long child list: the rendered `items` are entries
 *  `[offset, offset + items.length)`. `total > 0` with no items = expandable-but-not-yet-streamed. */
export interface TreeDataWindow {
  readonly total: number;
  readonly offset: number;
}

interface TreeDataItem {
  …
  window?: TreeDataWindow;   // this GROUP's child window
  windowKey?: string;        // the authored node key the host reports back (NOT the DOM `id`)
}

interface TreeDataSection {
  …
  window?: TreeDataWindow;   // this SECTION's child window
  windowKey?: string;
}
```

Both fields are optional; an unwindowed container renders exactly as before (no spacers, no attributes).

### 1.2 Constants

```ts
export const TREE_WINDOW_OVERSCAN_ROWS = 8;   // rows requested beyond each viewport edge
export const TREE_WINDOW_ROWS_MAX = 128;      // = guest UI_BUILT_CHILDREN_MAX / UI_DOCUMENT_NODES
export const treeRowHeightPx: number;         // was module-private; now exported (domSizePx("treeRowUiSpacing"))
```

`treeRowHeightPx` is newly exported **because P4b's observer needs the very same pitch** the spacers use —
there must not be a second copy of the row height on the host side.

### 1.3 Pure helpers

```ts
export interface TreeWindowContainerMeasure {
  readonly key: string; readonly total: number; readonly offset: number; readonly length: number;
  readonly top: number; readonly height: number;   // full virtual extent: spacers + rows + nested content
}
export interface TreeWindowRequest { readonly key: string; readonly offset: number; readonly rows: number }

export function treeWindowRequestsForViewport(
  containers: readonly TreeWindowContainerMeasure[],
  viewportTop: number,
  viewportHeight: number,
  rowHeightPx: number,
  overscanRows: number,
): readonly TreeWindowRequest[];

export function treeWindowSpacerRows(
  childWindow: TreeDataWindow | undefined, materialisedCount: number,
): { readonly leading: number; readonly trailing: number };

export function treeWindowDomAttributes(
  childWindow: TreeDataWindow | undefined, materialisedCount: number, windowKey: string | undefined,
): TreeWindowDomAttributes | undefined;
```

`treeWindowRequestsForViewport` rule, per container (exactly as design §6.1):

| step | value |
|---|---|
| skip | `total === 0`, or `rowHeightPx <= 0`, or the container does not overlap the viewport at all |
| `overlapPx` | `min(top + height, viewportTop + viewportHeight) − max(top, viewportTop)`; `<= 0` → **the container is dropped entirely** (no request, so a scrolled-away branch keeps what it has instead of churning) |
| `visibleRows` | `ceil(overlapPx / rowHeightPx)` |
| `firstVisibleRow` | `floor(max(0, viewportTop − top) / rowHeightPx)` |
| `rows` | `clamp(visibleRows + 2·overscan, 1, min(total, TREE_WINDOW_ROWS_MAX))` |
| `offset` | `min(max(0, firstVisibleRow − overscan), total − rows)` — so `offset + rows ≤ total` always holds |

`top`/`height` are taken **as the DOM measured them** (they cover the leading spacer, the materialised rows,
the trailing spacer and any expanded nested content); the helper does no geometry of its own.

---

## 2. DOM attribute contract — what P4b's observer reads

**Exact names. Nothing else on these elements is part of the contract.**

### 2.1 The container's *content element* (its top edge IS row 0 of the child list)

The element is the branch content div the `Tree` already rendered — `TreeBranchContent` — identified by its
existing slot:

| container | selector |
|---|---|
| section | `[data-slot="tree-section-content"]` |
| group item (default layout) | `[data-slot="tree-item-content"]` |
| group item (property layout, i.e. the row has a `control`) | `[data-slot="tree-property-content"]` |

A convenient single selector for the observer: **`[data-tree-window-key]`** — the attribute is present only on
windowed containers, and only on that content element.

Attributes stamped on it (all four, only when `window` is present):

| attribute | type on the wire | meaning |
|---|---|---|
| `data-tree-window-key` | string | the **authored node key** (`TreeDataItem.windowKey` / `TreeDataSection.windowKey`), i.e. what goes into `TreeWindowRequest.node_key` on the guest wire. Omitted only when the host supplied no `windowKey`. |
| `data-tree-window-total` | integer string | `window.total`, floored at 0 — the full logical child count |
| `data-tree-window-offset` | integer string | `window.offset`, clamped into `[0, total]` |
| `data-tree-window-length` | integer string | the number of rows actually rendered (`items.length`) |

Invariant: `offset + length ≤ total`.
Measuring: `element.getBoundingClientRect()` on that content element gives `top`/`height` for
`TreeWindowContainerMeasure` directly (subtract the scroll viewport's own `top`, add its `scrollTop`, per P4b).
A **closed** section still renders its `CollapsibleContent` with `hidden`, so its rect is zero — P4b should skip
zero-height rects (or read `[data-tree-owner-expanded="true"]`). A closed group item renders **no** content
element at all.

### 2.2 The spacers

```html
<div data-slot="tree-window-spacer"
     data-tree-window-spacer="leading" | "trailing"
     data-tree-window-rows="<n>"
     aria-hidden="true"
     class="w-full min-w-0 shrink-0"
     style="height: <n × treeRowHeightPx>px"></div>
```

- Rendered as a **direct child of the content element**, immediately before (`leading`) / after (`trailing`) the
  materialised rows.
- `leading` rows = `offset`; `trailing` rows = `total − offset − items.length`.
- **A zero-row spacer is not rendered at all** (no empty block in the DOM).
- They are plain blocks — no `TreeAlignedRow`, so they claim **no guide gutter**; the branch's own
  `IndentationLines` (absolutely positioned inside the same content element) paint straight through them.
  `TreeBranchContent`'s sibling-gap layout effect treats them as neither a row nor a branch, so no margin is
  applied to them and none is applied to the first real row after them.
- Pitch is exactly `treeRowHeightPx` per row — every tree row shell is `h-workbench min-h-workbench
  max-h-workbench` and every sibling gap constant (`treeCompactSiblingGapPx`, `treeSubtreeGapPx`,
  `treeSectionContentPaddingTopPx`, `treeItemContentPaddingTopPx`, `treeRowVerticalPaddingPx`) is `0`.
- In a `direction === "up"` tree the two spacers swap DOM order (the rows are reversed there), but each keeps
  its own `data-tree-window-spacer` value.

---

## 3. Rendering behaviour

`TreeDataSectionView` / `TreeDataItemView` (`🌳️Tree/🟦️.tsx`):

1. `windowTotal = max(0, floor(window?.total ?? 0))`.
2. **Expandable**: `isExpandable` (section) and `hasExpandableChildren` / `hasNestedTreeItems` (item) now also
   become true when `windowTotal > 0`, so an announced-but-unstreamed container still shows its chevron and can
   be opened. `TreeItemCollapsibleState.None` still forces non-expandable.
3. **Loading ring**: a new `isWindowPending = windowTotal > 0 && items.length === 0 && open` is OR-ed into the
   existing `isLoading` — the same `loading` prop path the async `getItems` load uses, so an unstreamed open
   window wears the existing `border-loading` ring on its header row. No new visual vocabulary.
4. Spacers are emitted around the rows via `treeWindowSpacerRows`; the window mirror via
   `treeWindowDomAttributes` goes onto the container's content element through a new
   `windowAttributes?: TreeWindowDomAttributes` prop on `TreeSectionProps` / `TreeItemProps`, which
   `TreeSection` / `TreeItem` forward to `TreeBranchContent` (all three branch-content call sites).
5. `alternatives` win over `window` on an item (a branch-navigated row is not a windowed list).
6. Open state is untouched: it still flows through `useTreeOpenState` → `TreeStateProvider`, so
   `openStates`/`onOpenStateChange` on `<Tree>` remain the host's controlled-expansion channel (P4b §6.2).

---

## 4. Tests

Added to `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`
(the file is already registered in the runner's `include` list —
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`), two new regions:

**`🪟️WindowedContainers`** (DOM, `@testing-library/react` + jsdom)

| test | asserts |
|---|---|
| `{total: 100, offset: 20, items: 10}` | leading spacer `20` rows / `20 × treeRowHeightPx`, trailing `70` rows / `70 × treeRowHeightPx`; all four `data-tree-window-*` attributes on `tree-section-content`; exactly 10 rows rendered |
| `{total: 4, offset: 0, items: 4}` | **zero** spacer elements (no zero-height blocks) |
| `{total: 5, items: []}` | section row is `role="button"`, `aria-expanded="true"`, carries `.border-loading`; one 5-row trailing spacer; zero rows |
| nested group `{total: 40, offset: 8, items: 1}` | same contract on `tree-item-content`; spacers `8` and `31` |

**`📐️WindowRequests`** (pure, `rowHeightPx = 20`)

| test | asserts |
|---|---|
| container fully above the viewport | skipped entirely; only the intersecting container gets a request |
| partially visible top (`top: -100`, viewport `0..200`, overscan 2) | `{offset: 3, rows: 14}` |
| viewport in the middle of a 1000-row container (rows 100…150, overscan 8) | `{offset: 92, rows: 66}` |
| total (6) smaller than `visibleRows + 2·overscan` | `{offset: 0, rows: 6}` |
| 300 visible rows of a 500-row container | `rows` clamped to `TREE_WINDOW_ROWS_MAX` (128), `offset` 192 |
| viewport at the very end | `{offset: 474, rows: 26}` — `offset + rows === total` |
| `total: 0`, and `rowHeightPx: 0` | `[]` in both cases |

**Story**: `WindowedSection` in
`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/📖️stories/🧪️.story.tsx` — an
`Entries` section streaming rows 40…51 of 400 (spacers 40 / 348, visible in the 320×420 scroll box) plus a
`Pending` section with `{total: 24, items: []}` showing the loading ring and a 24-row spacer. Its `play` asserts
the spacer row counts `40,348,24` and `data-tree-window-length === "12"`.

---

## 5. Commands run (foreground) and real results

Run from
`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript`
(this is exactly what `bun nx run @semio-tech/ui-react:test-long` / `:typecheck` invoke).

**1. Tree element tests only** — `bun ./📜️script.ts test long "🌳️Tree"`

```
 Test Files  1 passed (1)
      Tests  23 passed (23)
   Duration  10.60s
```

(13 pre-existing + 10 new. The default `test` level's 15 s budget kills the whole suite, hence `test long`.)

**2. Whole ui-react suite** — `bun nx run @semio-tech/ui-react:test-long`

```
 Test Files  2 failed | 24 passed (26)
      Tests  17 failed | 749 passed (766)
   Duration  55.68s
```

The 17 failures are **pre-existing and unrelated to this packet** — all of them live in the react target's own
in-source tests (`🎯️targets/⚛️react/🟦️.tsx`, plus one in `📨️UIDialog`), none in `🌳️Tree`. Evidence:

- Several fail with `ENOENT` on paths that **do not exist anywhere in the repo**:
  `🧰️framework/🎨️styling/🖌️ui/🎨️.css` and `🛂️manifest/🧫️fixtures/🖱️tutorial-local-interaction.json`
  (verified with `ls`) — stale fixture roots, not behaviour.
- The rest are icon-keyframes / navbar / celebrate / context-menu / WindowChrome assertions. The only edit this
  packet made to that file is adding names to one `import { … }` / `export { … }` list, which cannot change any
  of them.
- `… > tree helpers > WindowChrome stamps data-dim on the body glass surface for ghost dimming` **passes in
  isolation** (`test long -t "WindowChrome stamps data-dim"` → `1 passed | 767 skipped`), i.e. it is in-suite
  test pollution, not a regression.
- Full FAIL list captured at `🗑️generated/p4a/vitest-ui-react.txt`.

**3. Type-check** — `bun nx run @semio-tech/ui-react:typecheck` (`tsc --noEmit -p tsconfig.json`)

Exits non-zero with **661 errors, repo-wide and pre-existing** (top offenders: `🦑️repo/📚️library/🕸️dependencies`
59, `💻️os/🛢️db` tests 59, `🦑️repo/📚️library/🧹️normalization` 27, root `📜️script.ts` 98 …). Run twice, before and
after the story was added: **661 both times, byte-identical Tree lines.** The only `🌳️Tree` entries are six
pre-existing ones:

```
🌳️Tree/📖️stories/🧪️.story.tsx(66,31)  TS2769  (pre-existing)
🌳️Tree/📖️stories/🧪️.story.tsx(100,14) TS2741  (pre-existing)
🌳️Tree/🧪️tests/🧩️component/🟦️.tsx(111,72 / 153,68 / 154,66 / 169,89) TS2322 UiLabel  (pre-existing)
```

`🧱️elements/🌳️Tree/🟦️.tsx` and `🎯️targets/⚛️react/🟦️.tsx` produce **zero** type errors. Every line this packet
added type-checks clean.

---

## 6. Notes for the neighbouring packets

- **P4b** (`🗣️Interpreter`, `🏛️ShellHost`, `🛠️ShellHelpers`): import
  `treeWindowRequestsForViewport`, `TREE_WINDOW_OVERSCAN_ROWS`, `TREE_WINDOW_ROWS_MAX`, `treeRowHeightPx` and the
  types `TreeDataWindow` / `TreeWindowContainerMeasure` / `TreeWindowRequest` from `@semio-tech/ui-react` — they
  are already re-exported there. Query open containers with `root.querySelectorAll("[data-tree-window-key]")`,
  read the four attributes from §2.1 and the rect of that same element, and report
  `viewportRows = ceil(viewportHeight / treeRowHeightPx)`.
- The `getItems` lazy-children primitive is untouched and still works; `window` and `getItems` are independent
  (a container may use either, and the loading ring is shared).
- **wgpu (P5)** mirrors §2's numbers, not its attributes: spacer pitch is `TREE_ROW_HEIGHT × count` before and
  after the materialised rows, expandable when `total > 0`.
