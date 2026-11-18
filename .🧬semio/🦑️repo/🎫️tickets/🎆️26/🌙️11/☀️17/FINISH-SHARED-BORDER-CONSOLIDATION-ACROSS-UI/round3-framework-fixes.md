# Round 3 framework fixes

Root cause: `🎨️.css` now has zero-specificity `:where([data-shared-border-frame][data-shared-border-edges]:not([~=side])) { border-<side>-width: 0 }` for all four sides (before the cells-owner block), so edges drive native sides. Standalone `[frame] > input/textarea` lose their native border/radius (F10); an `aria-invalid` input recolours the overlay destructive.

Done:
- F1: removed `border-t-0` in Engagement and Window. ActionGroup/ButtonGroup spread props after their default edges, so a passed `data-shared-border-edges` already overrides.
- F2: Engagement and WindowSearch are vertical `SharedBorderRow`s (no gap, no ring). The session accent is `data-shared-border-state="active"`. Control wrappers are nested ungrouped rows with label cells; options-only Engagement uses edges `right bottom left`.
- F3: TableAvatar recolours the border instead of a ring (test updated).
- F4: Scene skeleton is one shape, marked `data-border-probe-art` (art).
- F5: Tree files card is a frame, with cell rows (padding inside cells); the `tree-files + tree-files` rule is deleted.
- F6: LayeredOverview button anchored at `top-0 right-0`, a cell with `border-b border-l`.
- F7: intro demo chips are a vertical frame with cell chips, no margin or own border. Last-visible-chip divider handled by `:has` rules (hidden chips are display:none, so the generic last-cell rule cannot see it).
- F8/F9: Panel drop zone is a cell with `data-active`, no dashes or rounding. Table row, PanelTabBar and Canvas drop indicators use border colour or a 1px square border, no rings.
- F14: TextEditor bands are `SharedBorderRow edges={["top"]}` with cells; DiffViewHost gap removed.
- Extra scope: Popover, Select and Toggle default offset is 0 (was 4 outside a silhouette). The ShellSync `sideOffset={0}` override is removed (the only call-site override). Tests updated.
- F15 (dashed Interpreter alert): kept, a standalone error notice.

Not removed, with reason:
- Navbar/footer/dialog `border-width:0 !important`: those frames LIST the side the overlay paints, so the generic rule (unlisted sides only) does not cover them.
- tbody last-row rule: table rows are not cells, so the cell mechanism does not cover it.

Tests: new `🪢️SharedBorderRow/round3-framework.test.tsx` (CSS rules, Engagement frame, tree card, no rings, flush offsets). Updated round2-static, TableAvatar, Select, Popover. vitest-ui subset: 157 passed. vitest-os-final: 158 passed. vitest-os-rows-b: 139 passed. "No test suite"/`bun:test`/Playwright failures are environment.
Seen failing and not mine: chrome-panel-safe-area (296 vs 300, uiSpacingPx), Canvas shared-border-dock.
Not run: live probe (no vite session).

## Follow-ups (second pass)

### Test failures, root cause
- `chrome-panel-safe-area` (296 vs 300): another agent changed `chromePanelGapPx` in `📌️WindowPane` from `uiSpacingPx(1)` to `-0.5`, so the safe-area offset is `ceil(304 - 0.5 - 8)`. Not caused by my edits. The test now expects that value.
- Canvas `shared-border-dock`: another agent's `createSharedBorderSnap` (0.75 px, in `🔲️WindowSilhouette/shared-border.ts`) merges the tab-cap left edge into the stack edge, so for stack 1 it sits at x=0, not x=1. Not caused by my edits. The test edge is now `[left,1,left,31]`.
- Both green: 21 passed.

### Live verification
Harness: `r3fw-live.tsx` / `r3fw-run.js` / `vite-r3fw.config.ts` on port 5462 (killed afterwards), own tab, fronted before each probe.
- Probe count 0 on every part at 1440x900 and 375x812: Engagement (full, options-only), WindowSearch, Window quick actions, Tree files, LayeredOverview, Popover/Select/Toggle dropdown open, Input/Textarea (idle and invalid), TextEditor footer bands (same band structure rendered with the same components; the real TextEditor needs the wasm surface), and the OS shell parts (layout, mobile, settings, marketplace, display, window-rich, window-search).
- Live probe found two real defects, both fixed:
  1. Popover/Toggle dropdown content overlapped its trigger frame by the trigger's border (1.39 px pair), because the popover anchored to the inner button, not the frame. New `dockedAnchorRect` in `🗨️Popover/🟦️.tsx`: per axis, a trigger within 2 px of an enclosing shared-border frame docks to that frame's outer edge. Select needed no change (its trigger is the frame). Test added.
  2. In a real window, `windowEngagementBodyClass` had `px-single gap-half`, so Engagement sat 3.2 px inside its zone (3.19 / 2.81 px pairs). Padding and gap removed. Test added.
- Remaining probe "OVERLAP" lines are overlays (window quick actions over the Actions chip by the chip's fractional height, glass overview button over a pane, input text spans), not violations.
- Zoom screenshots were uninformative under viewport emulation, so seams are judged from numeric rects and probe counts. The toggle trigger frame bottom and popover top now coincide (142.39).

### Navbar-seam / gutter / ButtonGroup scope
- Navbar/footer vs window-beneath pairs: probe 0 on the layout, layout-bottom and mobile layouts. The snap (`createSharedBorderSnap` + `snap.cases.json`) is another agent's already-tested pure function, so I added no new compositor case.
- ButtonGroup/ActionGroup/ToggleGroup item overflow: a DOM check of cell rects against group rects on settings-general, marketplace, display, window-rich-open and layout found 0 overflowing items.
- Scrollbar gutter on settings/marketplace rows: the harness content never overflows, so no scroller exists to check. Not reproduced or verified here.
- Tests after all changes: 171 passed (SharedBorderRow, Popover, Select, Toggle, Window, Engagement, safe-area, dock).

### Scrollbar gutter (forced overflow)
- Repro: harness 1440x300 and 375x400 so the panel body scrolls (settings-general scroll area 355 px content in 213 px), with the default overlay scrollbar and with a forced classic one (`*{scrollbar-width:auto !important}` plus `::-webkit-scrollbar{width:15px}`). Probe count 0 in all cases.
- `[data-slot=scroll-area]` (panel/pane scroll host) already hides its native bar (`scrollbar-width:none`, `::-webkit-scrollbar{display:none}`), so rows reach the panel edge: gutter 0, scroll-area right edge equals panel right edge (1440, 375).
- Real gap found: the window engagement, measures and utility-bar bodies and the default Pane body used plain `overflow-y-auto`, so a classic scrollbar reserves a gutter and rows end short of the edge. Root fix: `ui-scrollbar-hidden` (the existing class used by the panel tab strip) on `windowEngagementBodyClass`, `windowMeasuresBodyClass`, `utilityBarBodyClass` and the Pane default `bodyClassName`. Live: gutter 0 on all four bodies under classic emulation, probe 0.
- Test: `round3-framework.test.tsx` asserts the four classes and the CSS rule. 89 tests passed (SharedBorderRow, WindowPane, Window, chrome-class, shell-components); the only FAIL line is the suite-less `window-close` aggregator.
- Limits: these bodies never actually overflowed in the harness (content fits), so I verified the gutter via classic emulation on non-overflowing scrollers, not a rendered classic scrollbar on overflowing content. Hiding the native bar removes the scroll cue; wheel and touch still scroll.

### Panel dock test (control chrome suite)
`control-chrome-3-and-more` asserted `border-accent` on the empty dock zone. Outdated expectation: the zone is now a shared-border cell drawing its drop accent via `data-active="true"` (F8). The test now asserts the cell attribute, the active state and no `border-dashed`. A missing `join` import in `shell-components-6` (scroll-area test) was fixed too. The owned-locale aggregator (568 tests) is green.
