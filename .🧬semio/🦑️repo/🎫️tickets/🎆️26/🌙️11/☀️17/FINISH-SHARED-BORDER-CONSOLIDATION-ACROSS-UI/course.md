# Course border verification

The screenshot shows the German Course sorting result. Its inset crowd table was registering with the compositor as a cell inside an already bordered result section. The chart table now shares the figure’s surrounding cell. The table header owns its divider, and the score histogram has one continuous baseline. The page picker, pet-action grid and numeric guess field use shared-border cells for their control dividers.

## Course component tests

All 367 tests passed across shared borders, home grid, crowd results, pets, quantity formatting and keyboard task controls. The CSS checks used Lightning CSS to resolve declarations with custom properties. The final run used the installed Vitest runner after the repository capacity queue held the retry for more than 90 seconds.

## Course browser check

The German dark-mode adaptive journey passed in Chromium at 375 and 1920 CSS pixels. It ran against the actual Course app using the ticket’s local Vite preview and the cached development proctor with a fresh database. Both journeys submitted the quiz, verified the answer table’s border ownership and checked the single 1 CSS pixel histogram baseline. Pointer entry over the sorting result caption recolored every painted card edge. SVG bounds, edge geometry, count and 1 CSS pixel computed stroke width matched at rest and hover, with no repeated path spans.

The pointer check initially found a move-only path on desktop and two half-pixel segments that retraced the phone card’s existing edges. The compositor now discards spans whose snapped endpoints cross and skips move-only paths. Five neutral junction cases cover crossing, collapsed spans, perpendicular joins and ordinary spans. The targeted shared-window suites passed 94 tests.

The original `repo run` gate was cancelled after 230 seconds waiting for the project slice. The local Vite preview omits assignment PDF delivery, which the result journey does not use. A separate Chromium check confirmed the histogram baseline at device pixel ratios 1 and 2.

## Course forced colours

The clock and drop zones already belong to the compositor. Their forced-colour states now use paired system colors for the clock and drop-target labels. The leaderboard’s personal-row outline remains because its row uses native table separators. The Course contrast, clock and pointer suites passed 58 tests. Chromium confirmed that the warning clock, expired clock and drop-target label use contrasting system colour pairs with `outline-style: none`.

## Related UI smoke tests

- Hub Admin passed 28 tests across administration, routing and localization.
- Staffage passed 8 shared-border tests.
- Presentation passed 12 deck rendering and DOM interaction tests.
