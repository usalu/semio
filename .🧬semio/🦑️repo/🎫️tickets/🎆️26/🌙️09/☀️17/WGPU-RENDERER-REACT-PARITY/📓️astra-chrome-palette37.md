# Semantic Chrome Palette Projection

W28 showed cream selected text in React while WGPU used the dark foreground declared by the shared theme. Source inspection found that `applyUiThemeToRoot` applied primitive colors and metrics but omitted all semantic chrome paints. The stylesheet also hardcoded older dark-mode foregrounds and selected-control hover colors.

Added a neutral fourteen-token CSS mapping and custom light/dark palette fixture. An actual Chromium test resolves the production stylesheet on two separate theme roots and compares the colors against the shared palette, then changes appearance twice without reapplying the theme and verifies cleanup. The original implementation failed; the updated implementation passed 2/2 (Nx exit 0, 10.2 seconds).

Both resolved appearance palettes are now applied as scoped CSS variables, and local semantic aliases choose their light or dark values. Selected toggle controls use the active foreground and authored active-hover paint. The extended Chromium coverage passed 2/2 (Nx exit 0, 11.6 seconds), including mixed appearances on separate roots and actual selected-control hover. The existing live React projection tree still reports cream selected text; its distinct tree styling is under investigation. Full paired app runtime validation remains pending.

## Live Refresh and Shared Control Classes

Reloading the React page applied the new palette variables: the active foreground now resolves to `rgba(0,17,23,1)`. Selecting Curvilinear still painted cream text because the shared interaction classes explicitly used `text-emphasized`. A new actual Tailwind compilation plus Chromium test reproduced the mismatch (2 passed, 1 failed; Nx exit 1, 21.7 seconds). The shared selected, on and active-tab classes now use the active foreground and authored active hover token. Its verification is running.

The strengthened palette suite passed 3/3 (Nx exit 0, 20.0 seconds), including compiled production Tailwind classes for selected trees, toggles and tabs under actual Chromium hover. The existing pressed-navbar expectation was updated to the semantic foreground; its separate suite is pending.

The live React server also reports refused trusted-catalog proxy connections after refresh. Earlier browser warnings concern stale/unactivated guest modules; no new renderer JavaScript error was returned by the scoped browser log read. These integration issues remain distinct from the semantic paint repair.
# Live Nested Label Follow-Up 39

The live React DOM confirms active row/tab containers now use RGB(0,17,23), but the selected tree label remained RGB(247,243,227). Its data-tree-selection-path/data-tree-hover-path rule in UI globals directly overrode descendant labels/icons with border-emphasized-color. A new browser case includes the production global stylesheet and nested slots; it failed on that exact cream-vs-dark mismatch. The selected content override now passes: 4/4 tests, Nx exit 0, 27.8 seconds. Scope to the row's direct layout is being tightened to prevent selected ancestors coloring unselected descendants. Live console error list was empty at inspection; full renderer parity is not implied.
# Live Confirmation 39

After a deliberate React reload and Projection → Curvilinear interaction at localhost:6013, DOM readback reports Curvilinear aria-selected=true with RGB(0,17,23); Orthographic is aria-selected=false with its muted foreground. The screenshot visibly shows dark text on the selected red row. The console error list is empty. The active WGPU page remains W28; its replacement artifact has not been built or published.

The expanded browser law also caught selected-ancestor color leakage (3 passed, 1 failed). The production override now targets the selected row's direct tree-row-layout/tree-row-content only. The final four-law rerun is tracked separately. The older navbar string assertion could not run through ui-react:test because its monolithic owned-locale-detector-retirement file is not registered in that project's test include list; no pass is claimed for it.
The final direct-row scope rerun passed 4/4 tests (Nx exit 0, 31.5 seconds), including selected descendants, unselected nested rows, guide colors, toggle/tab colors, both appearances, and authored palette overrides. No additional palette changes were made after this pass.
