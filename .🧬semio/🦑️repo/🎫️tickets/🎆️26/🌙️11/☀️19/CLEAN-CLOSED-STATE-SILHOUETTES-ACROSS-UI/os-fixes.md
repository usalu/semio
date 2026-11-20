# OS Closed-State Silhouette Fixes (4 live defect classes)

All paths relative to `🧰️framework/`. Compositor: `🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`.

## 1. Mode-dock tab silhouettes
Causes:
- The tab box `div[data-slot=mode-dock-tab]` holds 3+ buttons (select `role=tab`, focus, close), so it was never interactive (not itself interactive, no single interactive child). Hover and focus therefore fell through to the enclosing chip (interactive: holds interactive elements) and stated the chip and window: open right side and 4-6 leaked segments.
- Even when the tab was stated, its paint lines were its own box edges (bottom = rect.bottom - 1 = 57), while the window body top lies at depth + inset (59): two parallel lines two pixels apart joined by a 1 px staircase through the junction resolver. A same-window box also took part in the shared-line centerline average and moved the window's left line (jog 1 -> 0 where the neighbour stack shares x).
Fixes:
- `shared-border.ts:332` the implicit mode-dock tab is interactive (delegates to itself) when it contains `[role=tab]`.
- `shared-border.ts:525-550` `windowOf(box)` finds the window of a box inside a chip; its paint lines snap to the window outline lines within 2 px (`snapToLine`, the rule chips already used), so the tab bottom equals the body top line and the right edge equals the chip edge; box ids are recorded as `followers`.
- `shared-border.ts:568-590` `leadingOf`: followers do not contribute to the centerline or the per-source line of a shared span (the window's own line wins), so cap and body left edge stay one x.
Tests (`🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.test.ts`, describe "state silhouettes of product shapes", from line 977):
- "hovering any control of the tab states the tab box alone, and its closed bottom lies on the window body line" (mocked rects, window geometry depth 30: tab bottom y 31, right x 60, no hover outside the tab).
- "the closed tab bottom and the window body top are one line without staircase jogs, whatever the tab edge offset within two pixels" (tab bottom 29, 30, 31).
- "a focused tab states only itself ...".
- Updated `🎨️Canvas/🧪️tests/shared-border-dock.test.tsx`: tab rect mock added, hover paths read from the window border plus the tab overlay, hover left edge on the shared line x=0.
Verified the first two and class 2 test fail without the fixes.

## 2. owned-dialog `input#name`
Cause: the field is a bare `<input>` that is a direct child of the argument row (`SharedBorderRow data-shared-border-cells`). Direct children of a cells owner are cells (probe, CSS, SharedBorderRow doc), but the compositor required a frame/edges marker (`element.matches(selector)`), so a bare field was no box at all. The void-element painter path (`holdsNoOverlay`) only handles boxes outside cells owners and never ran. Real `Input` (framed `input-root`) was unaffected.
Fix: `shared-border.ts:39,123` bare `input` (not hidden), `select`, `textarea` children of a cells owner are cells (`bareFieldSelector`), painted in the row's overlay. A concurrent edit of the file also reuses the selector at line 43/394.
Test: "a bare input that is a direct child of a cells owner is a cell" and "a hidden input and an input outside a cells owner are not cells".

## 3. task-manager buttons (TaskManagerPanel actions column)
Cause: the buttons are `ButtonGroup` frames with a cell, correct boxes (jsdom with the real component and a mocked scroller paints a closed outline). They sit in the last of 12 columns. The table's `Scrollable` (`overflow-x: hidden`, `size-full`) swallowed the horizontal overflow of the outer `overflow-x-auto` region, so the column lies outside the visible clip; the compositor correctly paints nothing for a wholly clipped stated box. The sweep dispatches synthetic events without scrolling, hence the 12 findings (2 rows x 3 buttons x hover/focus).
Fix (`🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🟦️.tsx:450-451,470`): the actions column and header are `sticky right-0 z-panel bg-background`, and the table scroller itself scrolls horizontally (`className="overflow-x-auto"`). The controls stay in view and become reachable.
Tests: component test "keeps the actions column in view ..." (TaskManager), compositor test "a control scrolled out of its scroller paints nothing and states no ancestor, and closes fully once it is in view".
Not verified live (no browser): that the sticky column fits the live panel width.

## 4. agent-chat approval `section`
Cause: the section was forced to be an interactive borderless box (`sharedBorderlessBoxProps` + `data-shared-border-interactive`) although it is a static text block with `tabIndex=-1` (a programmatic focus target); design.md: a box is interactive only if it is itself a control, and `tabindex=-1` is not interactive. Hovering any text closed the whole section, which spans the scroller (`ol overflow-y:auto`) and its owning `li` edges (bottom edge of the `li`, clip edges at its sides), producing the open sides.
Fix (`🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🟦️.tsx:168`): no box props, import removed. The decision buttons inside remain boxes. Trade-off: a programmatic focus on the section shows no silhouette (outline is suppressed globally); the focused control inside shows its own.
Test: "is a static programmatic focus target, never a shared-border box ..." (`🤖️AgentApprovals/🧪️tests/🧩️component/🟦️.tsx`).

## Test counts
- ui (`vitest-ui.config.ts`, `🧱️elements` + `border-presentation`): 710 passed, 4 failed (Tree text-flow x3, Layout x1: need Chromium, same as before), 20 non-suite files fail (bun:test and helper fixtures, same as before). `WindowChrome` 46 passed, dock test 17 passed.
- os-final (`vitest-os-final.config.ts`): 6 files, 160 passed (+2 new).
- os-elements / os-engine: only `bun:test` import errors and Chromium tests fail (unrelated to these files); not re-baselined.
