# Legacy suite fixes (owned-locale-detector-retirement aggregator)

Result: aggregator `🧪️tests/🧪️owned-locale-detector-retirement` 565/565 pass (was 8 failing). WindowChrome, PanelTabBar, Canvas, OverviewCard: 6 files, 62 pass, 1 skipped, 0 fail.

All 8 were stale tests; no product code changed. Each assertion now checks the current deliberate behaviour.

1. Panel silhouette cutout (`🎨️shell-components-1`): PanelTabBar wraps the scroller in a `panel-tab-frame`. Test now asserts the scroller is `flex-auto` and its parent frame has `data-slot="panel-tab-frame"` and `w-full`.
2. Mode close (`🗂️shell-components-5`): `closeWindow` in `🎨️Canvas/🖱️mode-drag-and-window-callbacks` now goes through an async close admission (`closeAdmissionRef.close`), so removal is not synchronous. Test is async and uses `waitFor`.
3-6, 8-9 tree guides: the guide design moved from per-row `data-tree-guide-line` (IndentationLines, now only for level>1 non-last-sibling continuation) to row-owned `tree-parent-guide` + `tree-branch-elbow` in `TreeDocumentGutter` (`🌳️Tree/📏️row-alignment`, commit 718; doc string in `🎨️chrome`: "rows own their terminating parent connector").
   - tree-helpers-1 fill on tree-row-content: guide count now counts `tree-parent-guide` (>=2).
   - tree-helpers-2 property label: `property-label-tree` class is now `flex min-w-0 items-center` (indent via padding style, still no placeholder gap).
   - tree-helpers-2 parent-level guides: now expects 2 `tree-parent-guide` rows in the expanded branch, first `top:0;bottom:0` (continues through), last stops at the row anchor (`bottom:calc(100% - ...)`); slice widened to 12k chars as rows are longer.
   - tree-helpers-3 guide wrappers: regex targets `tree-parent-guide` class.
   - uidriver WindowMeasuresTree hover-fill and nested groups: same swap to `tree-parent-guide` (hover colour `group-hover/tree-row:bg-emphasized` retained).

## Out of scope finding
`🌳️Tree/🧪️tests` (not in the requested list) has 5 failures in `🔤️text-flow` (3) and `🧩️component` focus retention (2). The Tree element has no working-tree changes from this ticket; not investigated.

Edited: files under `🧪️tests/🧪️owned-locale-detector-retirement/` `🎨️shell-components-1`, `🗂️shell-components-5`, `🪄️tree-helpers-1`, `🧵️tree-helpers-2`, `🎯️tree-helpers-3`, `🔗️uidriver-and-more` (`🟦️.tsx` each).
