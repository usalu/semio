# 👆️ Layered Overview: the Same Grid and Swipes on Mobile

Request (owner, 2026-10-05): play, demonstrator and the quiz UI showed phones an ad-hoc scrollable list. They must show the same grid instead, and swiping in both directions must move through it.

## Decision

`LayeredOverview` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview`) has two modes now: `strip | swipe`. The `list` mode is gone, with no compatibility layer.

- **Same grid, same machinery.** Swipe mode keeps the one strip, the one glass and the percent `translate3d` used on desktop, and the same `cells`. The view is one cell. The card layer is a second grid the size of the strip, and `paint` moves it with the same transform, so every card rides on its own page. A rotated phone stays on its cell, because the transform is in percent.
- **Gestures in JS, not native scroll-snap.** A probe in real Chromium (`snap_probe.ts`) showed that native 2D `scroll-snap-type: both mandatory` fails in three ways:
  - short swipes did not move;
  - a long fling skipped two rows despite `scroll-snap-stop: always`;
  - one swipe landed on an empty cell, because each axis snaps on its own.
  
  So `useSwipe` drives the gestures itself:
  - **Axis lock.** A touch locks to its longer axis past a 10 px slop.
  - **Drag.** The strip follows the finger, on a UIKit-style rubber band (0.55) where no cell follows.
  - **Release.** The strip moves on after a quarter of the view, or after a flick of at least 0.3 px/ms. It settles in 320 ms with a cubic ease-out.
  - **Short rows.** A swipe into a short row lands on its occupied columns, like `clampOffset` for the desktop pan.
  - **Tall cards.** A tall card takes the gesture first, the way a nested scroller latches natively, and is flung with UIKit's 0.998/ms deceleration.
  - **Other input.**
    - The wheel takes one step per gesture (Shift turns it sideways; a quiet gap of 250 ms ends a gesture).
    - The click a swipe ends in is swallowed.
    - A second finger is handed to pinch zoom.
    - Mouse drags are ignored.
- **touch-action.** The root and every card cell carry `touch-action: pinch-zoom`, so zoom keeps working for accessibility. Found in the real-browser run: a cell that may scroll is a scroll container. Browsers read `touch-action` only up to the nearest scroll container, so without its own value the browser took every swipe.
- **Lost pointer events.** Found in the landscape run: once a touch locks, the root captures the pointer. A new primary touch always starts a new gesture, and only non-primary pointers count as a second finger. Before this, a lost `pointerup` turned the next swipe into a "second finger".
- **Lifecycle.** The page the view rests on is touched. Its four neighbours are warmed one at a time within the budget (`neighbourWarmBoot`), never evicting a page.
- **Motion.** The settle runs whatever the device says about reduced motion, like pan and glide (`reducedMotion="never"` default). See the RDP note in memory.

## Apps

- **Play** (`🏢️semio-tech/🎡️play/🟦️.tsx`) and **demonstrator** (`♻️mit-bestand/🧺️demonstrator/🟦️.tsx`): `PLAY_/DEMONSTRATOR_SWIPE_MEDIA_QUERY` (the same touch-phone query as before) → `mode="swipe"`; the card renderer returns the bare card when `state.mode === "swipe"`.
- **Quiz home** (`❓️quiz/🎯️targets/⚛️react/🔨️modules/🏠️home`):
  - `HomeLayout` is now `{ grid: HomeGrid; mode: LayeredMode }` with the pure `homeLayout(matched)`.
  - Phones swipe through the desktop ring of nine (3 × 3, leaderboard in the centre).
  - Short windows swipe through the grid of their width: tablet 2 × 5, desktop 3 × 3.
  - `.quiz-home-grid` lost its one-column phone rule, since the card grid only ever appears in strip mode.

## Fixtures and tests

- `🖱️ui/🧫️fixtures/🥞️layered-overview/🔣️.json` and its schema gained ten sections:
  - `nearestCells`, `restingCells`
  - `swipeAxes`, `swipeSteps`, `swipeTargets`, `swipeDrags`
  - `rubberBands`, `settles`, `flings`
  - `neighbourWarmBoots`
  
  The values come from an independent Python twin (`swipe_vectors.py`) and were written by `insert_vectors.py`. Oracles: `d3-ease` `easeCubicOut` for the settle, and three.js `MathUtils.damp` for the fling's travel.
- `❓️quiz/🧫️fixtures/🏠️home-grid/🔣️.json`:
  - `layouts` and `heights.vectors` now carry `grid` and `mode`; `cells[].layout` became `grid`.
  - Two new vectors: a portrait phone (375 × 812) and a landscape phone (812 × 375).
- The element's component tests cover:
  - the swipe structure;
  - both axes, follow, distance and flick;
  - the rubber band and short rows;
  - the tall-card latch, click swallowing, mouse, pinch and wheel;
  - a lost `pointerup`;
  - neighbour warming, and opening and closing.
- Quiz tests: home-grid, adaptive-layout (the only width media query left is `.quiz-pair`) and learner-journey.
- Site e2e:
  - The `📱️phone` spec swipes with real CDP touch events through the learner driver's `swipeTo`, `touchSwipe` and `restingPage`. It covers the spring-back, a short slow swipe, the way to the cooling quiz (right, right, down) and back again.
  - `🐕️pet-walk` reaches the quiz with `swipeTo`.

## Verification

| What | Result |
| --- | --- |
| `@semio-tech/ui-react` full unit suite (vitest) | 1110 passed (LayeredOverview 35, geometry incl. schema 179) |
| `@semio-tech/quiz-react` full unit suite | 922 passed |
| typecheck ui-react, quiz-react, site | no errors in touched files; only `🛂️manifest` (another session's half-generated manifest) fails |
| real Chromium, phone 375 × 812, reduced motion on (`swipe_shots.ts` against the dev site) | left → physics, up → board, right → heating, down → learner; pulls past the first column/row spring back; no page scroll, no page error (`🖼️phone-swiped-up.png`, `🖼️phone-swiped-right.png`) |
| real Chromium, landscape 812 × 375 (tablet grid 2 × 5) | every swipe lands on its neighbour in two runs after the capture fix |
| real Chromium, swipe starting on the learner card's presence list (`touch_action_check.ts`) | moves to the next page; the only scroll container inside the cells computes `touch-action: pinch-zoom` |
| site e2e `📱️phone` (real CDP touch) | 2 passed — failed before the cell and descendant `touch-action` fixes and the capture fix, each time on a real defect |
| site e2e `desktop` `🥞️layered-home` (strip mode unchanged) | 5 passed |
| site e2e `pets` phone test | 1 passed |

Nx is pinned to bun 1.3.14 by another session's change, so its targets refuse this shell's bun 1.4.2. Tests ran through each owner's own `📜️script.ts` with the repository's Vitest policy (`owner_test.ts`). Play's and the demonstrator's dev servers need Nx-built activation lanes, so they were not started. Both only pass `mode="swipe"` to the element that the quiz runs verify.

## Follow-up: neighbour hints (owner, 2026-10-05)

Request: "Show which pages are neighbours, so the user doesn't need to guess what is coming after swiping in a certain direction."

- **What.** In swipe mode, every edge of the view behind which a page lies carries a hint: an arrow and that page's label.
  - Top and bottom hints run across, centred.
  - Side hints run along the edge as vertical text.
  - Each hint is a button that settles the strip there, so every swipe also has a single-pointer way (WCAG 2.5.1).
  - Each hint is at least 28 px (WCAG 2.5.8).
  - While the strip moves, the hints fade out and leave the focus and accessibility tree, so they always name the neighbours of the page at rest.
- **Same rules as the swipe.** `swipeNeighbours(from, cells)` (geometry, fixture section `swipeNeighbours`, Python twin) is `swipeTarget` in the four directions. A unit test proves for every cell of grids of 1–60 panes that a hint names exactly the page the swipe lands on, including short rows.
- **API.**
  - `LayeredLabels.neighbour(pane, direction)` is required (no default language). It is the hint's accessible name, e.g. "Go right to Physics" / "Nach rechts zu Physik". Play and the demonstrator add a beginner variant that explains the swipe.
  - New prop `insets: { top?, bottom? }` (CSS lengths): how far the app's chrome covers the overview. Cards and hints stay inside it.
    - Play: top `calc(var(--size-workbench) * 1.5)` for its navbar.
    - Demonstrator: the same top, plus bottom `5.5rem` for the partner credits.
    - Quiz: none (its navbar and footer lie outside the overview).
  - Cells keep px room for the hints (36 px at the sides, inset + 44 px above and below; px, so large learner text never narrows the card). This replaces the former `pb-[88px]`.
  - New exports: `LayeredDirection`, `LayeredInsets`.
- **Verification.**
  - ui-react: 1120/1120.
  - quiz-react: 923/923, including the hints in English and German at 375 × 812.
  - Typecheck: clean apart from the other session's `🛂️manifest`.
  - Real Chromium (`swipe_shots.ts` reports each hint's box): on portrait and landscape phones the hints sit at their edges, 28 px or more, and are hidden while moving. The first browser run found that `--spacing-half`, `left-half` and `right-half` do not exist: the hints stood off-screen or at the wrong edge, and were fixed.
  - Phone e2e: checks the hints and their German names and that they are fully in the viewport; a real tap on the bottom hint goes there.

## Follow-up: swiping across the edges (owner, 2026-10-05)

Request: "Make sure that swiping over the edges is possible and that when on top it jumps to the bottom etc. It feels like an infinite canvas but the grid index just wraps."

- **Geometry.**
  - `swipeWrapTarget(from, axis, step, cells)` returns a landing `{ cell, slot }`. Inside the strip the slot is the cell.
  - Across an edge the index wraps: past the last occupied column of a row to its first; past the bottom row to the top row, landing on its occupied columns like a pan. The slot stays right beside the view.
  - `swipeNeighbours`, `swipeDragOffset` and `neighbourWarmBoot` all follow the wrapping strip. The rubber band remains only where a row (or the column) holds no other page.
  - New fixture section `swipeWrapTargets`. `swipeDrags`, `swipeNeighbours` and `neighbourWarmBoots` were regenerated by the Python twin (`insert_vectors.py` now replaces changed sections). The schema gained a `landing` definition.
  - An exhaustive test over grids of 2–60 panes checks four things: every landing is occupied; its slot lies exactly one step beside the view; nothing wraps except where a row or column holds a single page; a swipe back along the rows returns.
- **Element.** Each page exists once, so the page that comes in across an edge is a ghost:
  - its pane and its card cell get `transform: translate(Δ·100%)` to the slot;
  - once the strip rests there, the shift is removed and the offset jumps to the page's own cell in the same frame (`settleGhost`), so nothing visibly moves.
  
  Rules that keep this consistent:
  - **Aiming.** The ghost is aimed when the touch locks and again whenever the drag changes direction, so each side shows the page it leads to.
  - **Release.** The release aims the step it takes.
  - **Interruptions.** A new touch, a wheel step, a hint tap, a keyboard reveal or an opened page first completes a running wrap.
  - **Clipping.** The strip and the card layer drop `paint` containment in swipe mode, so a ghost outside the strip box is not clipped.
  - **Readiness.** The pages one swipe away (wrapped too) keep their placeholder or poster ready.
- **Verification.**
  - ui-react: 1138/1138.
  - quiz-react: 923/923.
  - Typecheck: clean apart from the other session's `🛂️manifest`.
  - Real Chromium (`wrap_probe.ts`, phone, reduced motion):
    - a finger held halfway across the left edge shows "How it works" sliding in beside the learner (shifted by −300 %); after release the index is column 2 and no shift remains;
    - the same holds at the top edge with the bottom row.
  - Phone e2e (2/2, real CDP touch):
    - right from the learner wraps to "how it works" and back;
    - up from the learner wraps to the badges and back;
    - the learner's hints name all four neighbours (badges, how it works, quiz 1, quiz 2);
    - the cooling quiz is reached by swipes.
