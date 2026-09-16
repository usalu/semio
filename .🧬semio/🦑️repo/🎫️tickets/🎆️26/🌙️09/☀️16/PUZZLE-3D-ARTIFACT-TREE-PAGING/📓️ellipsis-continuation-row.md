# Puzzle 3D artifact tree — truncated "ellipsis" row

## Symptom

After ~7 object rows in **Dokument** (artifact tree), a single odd row appears (reported as "ipsi…") with no hide/lock actions; the remaining ~90 objects are not listed.

## Root cause

The outliner is **virtualised**: large documents (Nakagin-scale, or ~100 placed objects) render one bounded page per UI contract (`PANEL_RECONCILE_NODE_BUDGET` ≈ 11 interactive rows shared across four sections). The objects section closes with a **continuation row** whose label is `+N` (omitted count) and whose icon id was `ellipsis`.

`ellipsis` is **not** in the vendored icon catalog. The React `Icon` codec falls back to `data-icon-kind="text"` and paints the raw id in a narrow tree glyph slot, so users see garbled text ("ellipsis" truncated) instead of a ⋯ affordance. The row is not a broken object — it is the pager.

Paging is wired: `continuation_row_from` dispatches `setPanelPage`; `artifact_tree_cached_from` keys the memo on `panel_pages` and passes cursors into `render_from`.

## Fix

- Map `ellipsis` → `more-horizontal` in `CATALOG_ICON_ALIASES` and in asset icon projection aliases.
- Emit `more-horizontal` from `panel_continuation_row` and puzzle3d/cad continuation builders.

## User expectation

Click the `+N` row (⋯ icon) to advance the objects section page. Full scroll-virtualisation of the tree remains future work; the immediate bug was misleading rendering of the pager row.
