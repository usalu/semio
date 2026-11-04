# Actual App Story Follow-up

## Verified Causes

The initial localhost:6010 verification was wrong. That port belongs to a static inspection server launched from the separate `fix-storybook` checkout (process 215494 at inspection), not this branch. The attempted current-branch dev process exited because its required bounded compiler cache path was not supplied. HTTP 200 from an unrelated process was insufficient evidence of startup; this is corrected rather than attributed to the user's browser cache.

The current branch now serves live UI Storybook on **http://localhost:6011** with explicit canonical bounded compiler cache and localhost binding. Real App iframe inspection confirms updated source: `p-0` mode body, zero-width splitters, exact sibling contact at x=600, and registered shared-border segments.

That actual story also exposed remaining implementation gaps missed by isolated synthetic fixtures:
- Selected left tab is active, but both stack silhouette borders resolve to normal rather than active.
- Utilities collapsed pane chips remain independently outlined because their positioning leaves approximately 3.2px between the chip and containing window's left/bottom edges.

## Parallel Corrections

- `app-active`: actual Mode dock/WindowChrome active-state propagation and regression, preserving hover and continuous silhouette.
- `pane-join`: Window/WindowPane physical pane-chip attachment across anchors/RTL, without broad border erasure or arbitrary content-padding removal.
- Lead: live Storybook startup, actual App browser acceptance, integration and reports.

Evidence was captured from the real App story, not the earlier isolated browser fixture. `inspect-story.mjs` is retained as reproducible input. Generated JSON/screenshot files live under this ticket's generated directory and are temporary.

## Verified Corrections

- Controlled Mode chrome now uses activeWindowId plus selected stack ID; the actual left App outline is active before interaction, and hover wins without losing selection.
- Contained WindowPane positioning uses zero edge bases rather than Panel's default spacing-single; collapsed/expanded panes retain shape/padding and collision displacement. 32 Chromium anchor/fold/RTL scenarios passed.
- A third real-App bug appeared once the geometry touched: rounded offsetHeight values were incorrectly treated as transforms, leaving duplicate logical bottom spans despite coincident physical strokes. Measurement and registration now share one rounding-aware scale policy; fractional rectangular frames retain fractional dimensions. The new regression failed before and passed after the fix.
- ASSERT_APP_BORDERS=1 inspection now confirms abutting windows, Utilities/window contact, active/hover paint and absence of duplicate normal strokes along the shared bottom. Screenshot reviewed. The separate synthetic menu/forced-color/marker-only browser checks also pass.
- Focused implementation checks: 46 tests passed and strict modified-source TypeScript passed. Shared legacy assertions now follow the active/zero-inset contract; the static no-op controlled fixture was replaced by a real controlled host without weakening behavior.
- Final combined normal-runner result: **19 files, 718/718 tests passed**. Diff whitespace check clean. Live Storybook remains running on 6011. Generated evidence/cache outputs were removed; only its active server log is retained while the service runs.
