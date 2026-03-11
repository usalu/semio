---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Aligned all right-panel inputs to one shared vertical guide by neutralizing nested tree content indentation in right panel scope.
## Changes
- Removed newly added `SidePanelProps` sizing controls.
- Added stable `data-slot` hooks in `SidePanel`:
  - `side-panel-tabs`
  - `side-panel-tab-button`
  - `side-panel-content`
- Used existing `className` on right side panel wiring in `Sketchpad.tsx` to resize:
  - tab bar height
  - tab button padding
  - tab icon size
  - content padding
- Extended existing right-side panel `className` selectors in `Sketchpad.tsx` to map internal element sizing to existing tokens:
  - `input` slot height + font size
  - `select-trigger` slot height + font size
  - `button-group-item` slot height + font size
  - `label` text size
  - tree item text size
- Added stable tree sizing hooks in `elements.tsx`:
  - `tree-section-row`
  - `tree-item-row`
  - `tree-label`
- Updated right-side panel sizing selectors in `Sketchpad.tsx` to use these tree slots and existing tokens:
  - row min height (`min-h-medium`)
  - tree label text size (`text-xs`)
- Re-aligned right panel sizing overrides in `Sketchpad.tsx` to old-detail density values:
  - tab row `h-medium` (was `h-large`)
  - tab button padding `px-small` (was `px-double`)
  - content padding `p-single` (was `p-double`)
  - removed forced text-size overrides on input/select/button to keep component-native values
  - removed forced tree row min-height overrides
- Applied old detail width bounds using existing `SidePanel` props in `Sketchpad.tsx`:
  - `minSize: 150`
  - `maxSize: 500`
  - applied to right details panel and single-tab right panels (chat/settings) for consistent behavior
- Added inspector-specific property row hooks in `elements.tsx`:
  - `property-row`
  - `property-label`
  - `property-control`
- Updated slider internals in `elements.tsx`:
  - added `slider-content`, `slider-row`, `slider-track-cell`, `slider-value` slots
  - switched slider value column to fixed `28px` and right-aligned text
  - prepared row geometry for fixed control/value alignment
- Updated stepper internals in `elements.tsx`:
  - added `stepper-group`, `stepper-minus`, `stepper-plus` slots
  - enforced `22x22` button geometry, `56px` numeric field width, connected segmented shape and `3px` corners
- Reworked right side panel scoped style map in `Sketchpad.tsx`:
  - panel padding `10px`
  - row height `24px`
  - label column `96px`
  - label-to-control gap `8px`
  - control height `22px`
  - slider value column `28px`
  - slider track target width `130px`
  - input/select/stepper border and background harmonization for reference look
- Converted connection slider row wrappers in `Design.tsx` from stacked label+control to single-row grid:
  - `grid-cols-[96px_minmax(0,1fr)]`
  - fixed row height and control alignment on Gap/Shift/Rise/Rotation/Turn/Tilt rows for both single and multi-connection editing paths
- Reset right-panel custom inspector color overrides in `Sketchpad.tsx`:
  - removed custom background/text color classes
  - removed custom input/select border color overrides
  - kept all layout/spacing/sizing constraints unchanged
- Enforced one global right-panel input alignment guide:
  - added `data-slot="tree-content"` in `elements.tsx`
  - applied right-panel scoped `tree-content` left-padding reset in `Sketchpad.tsx`
  - all nested input/control rows now start on the same vertical line

## Log
- Read right panel and side panel implementation details.
- Reworked implementation to avoid new sizing parameters.
- Applied right-panel sizing through existing `className` parameter.
- Ran `npx tsc --noEmit` in `semio/js`; command fails on pre-existing `PanelSection.content` typing errors in `Design.tsx`, `Docs.tsx`, `Quality.tsx` unrelated to this ticket.
- Reopened ticket to extend sizing scope to tree list items, fonts, inputs, and buttons with existing settings only.
- Updated `rightSidePanelElementSizingClassName` to use existing utility tokens and slot selectors; removed hardcoded icon pixel size.
- Reopened again to continue; semio-repo `tree` CLI command did not return in this shell session (timeout/hanging), proceeded with known file paths from prior ticket context.
- Replaced broad right-panel tree selector with explicit `data-slot` hooks in tree components and token-based right-panel selectors.
- Compared old detail panel build values (`Design.Details.tsx.old`) against current side panel wiring and adjusted only existing hooks/props to match old density and resize envelope.
- Re-ran `npx tsc --noEmit` in `semio/js`; command still fails on pre-existing `PanelSection.content` typing errors in `Design.tsx`, `Docs.tsx`, and `Quality.tsx` (no new errors from this patch).
- Reopened and applied inspector reference restyle constraints without changing element ordering.
- Introduced slot-level hooks for property rows, slider internals, and stepper internals to support strict relational alignment in a scoped manner.
- Enforced right panel layout tokens through the existing `rightSidePanelElementSizingClassName` with selector-based overrides.
- Updated connection section slider wrappers to row-grid layout so labels and controls keep fixed guides independent of label length.
- Ran `npx tsc --noEmit` in `semio/js`; same pre-existing type errors remain in `Design.tsx`, `Docs.tsx`, and `Quality.tsx`.
- Reopened ticket to reset panel colors on request.
- Removed only hardcoded color classes from `rightSidePanelElementSizingClassName`; kept geometry and alignment selectors.
- Reopened ticket to align all window inputs on one vertical line.
- Added a tree-content slot hook and removed nested content left padding only in the right panel scope.

## Todos
- [x] Identify right side panel element sizing points.
- [x] Ensure existing sizing parameters are used.
- [x] Wire right panel sizing via existing `className`.
- [x] Run validation.
- [x] Align tree/list text + control sizing with detail panel settings.
- [x] Stabilize tree row/label sizing selectors via tree-specific slot hooks.
- [x] Match old detail panel resize envelope and internal density with existing right-side structure.
- [x] Apply strict inspector grid tokens (padding/row/label/control/value).
- [x] Align slider rows to fixed control/value columns.
- [x] Enforce stepper segmented geometry (`22/56/22`) and fixed left alignment.
- [x] Keep inspector refactor scoped to existing elements and existing files.
- [x] Reset inspector custom colors to default theme colors.
- [x] Align all right-panel inputs to one shared vertical line.

## Plan
- Use existing `className` parameter to apply right-panel internal sizing.
- Keep left panel behavior unchanged.
- Validate with type check.
- Add slot hooks in shared elements where needed, then style only through right panel scoped class selectors.
- Convert only connection row layout wrappers to one-line grid rows, preserving existing element set and order.
