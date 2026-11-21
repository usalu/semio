# Tinted Navbar and Footer Overlays

## Shared UI

`Layout` offers a scrollable page canvas through `scrollCanvas`. The sticky bars reserve their own height in the scroll flow. The first content starts below the navbar, and the last content clears the footer at the end of the scroll range. The canvas content has its own stacking context, with docked panels retaining their shell-level stacking. The focusable scrollport measures the full height of its sticky bands, including the optional subnavbar, for keyboard and anchor scrolling. A shell status row stays below the scrollport. The Scrolling Page story shows the behavior using the shared Navbar and Footer.

The tint uses the current theme's base color and supports an opacity override through `--chrome-tint-opacity`. Reduced-transparency and forced-color preferences use solid backgrounds.

## Branded landing pages

Semio Play and the Mit Bestand Demonstrator use the same tint on their landing bars. Play already scrolls its card grid as a single page. The Demonstrator now scrolls its desktop card grid as a single page when it overflows, with space at both ends for the navbar and partner credits. The mobile swipe view retains its existing gestures and layout. Returning from a lower card restores focus and scrolls the card back into view.

### Isolated branded browser checks

An inline probe run through `repo tools exec --light bun -- -` compiled the repository Tailwind stylesheet and used `Bun.build` to bundle the real branded landing components. Chromium loaded the bundles through Playwright. The Demonstrator probe stubbed the OS catalog composition, framework shell renderer, puzzle session factories, and host ports. Play supplied a no-op pane renderer. Both probes excluded browser-unused in-source test imports.

At 1024 × 300, both first cards started at the 29 px navbar edge and moved to 13 px after a 16 px grid scroll. The last Demonstrator card ended at 270.656 px against the 271 px footer edge. The last Play card ended at the 300 px viewport edge. Enter opened the last card and Escape returned focus to it with the grid scrolled 126 px in the Demonstrator and 1649 px in Play.

The Demonstrator's official E2E target spent 340 seconds waiting for its 5245 MB host-capacity reservation and was cancelled while queued.

## Shared Layout browser check

The Layout test suite passed all seven tests using the repository's Vitest configuration and local Chrome. The desktop and mobile-width cases check Page Down, Home, and wheel scrolling. Browser geometry, hit testing, and screenshot pixels verify initial and final clearance, stationary bars, visible tint, and shell status row placement. Disabling sticky positioning or tint independently failed the expected assertion using `layout-mutation.vitest.ts`.

The stylesheet compiled with the repository's Tailwind installation. A separate Chrome check verified the default translucent tint and the solid backgrounds under reduced transparency and forced colors. The changed TSX files parsed successfully, and the whitespace check passed.

## Full application launch check

The fleet could not start the application servers within the verification window while the graph service was reading the workspace. Running the Vite configurations directly with the pinned Bun runtime reached the app prerequisites. Play required the `generation3d` activation receipt and the Demonstrator required the `generator` activation receipt. The full application acceptance suites require those receipts.
