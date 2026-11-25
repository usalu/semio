# Standalone websites

| Surface | Application of the shared design |
| --- | --- |
| Hub admin | The session-gate states share a window frame with a pane-level title chip. The main pages keep their flush navigation, table and section bands, whose shared dividers belong to the existing row compositor. |
| Play | The landing uses the shared LayeredOverview, OverviewCard, and CardGridCell. The app card fills its cell and the shared grid paints abutting edges. |
| Architecture course | Shared course cards, lists and data tables use shared edges. The workbook and graph previews use a framed figure with an inset caption. Workbook chrome adds an inset title, contiguous formula cells and contiguous sheet tabs. The authored workbook values and diagram shapes carry the teaching content. |
| Staffage | The surrounding site uses shared card, row and stack primitives. The three-view sheet uses window frames with pane-level glass for its viewpoint controls. The physical gaps between the orthographic projections carry the sheet layout. |
| Pets stories | The species roster, inspector and task data use single shared seams. Captions and sandbox fixtures use the generated shade, alpha, blur and tint values. The sandbox retains space between authored obstacles because it exercises the pets' climbing, falling and traversal. The dev command now loads the resolved menagerie so the gallery and sandbox open by default. |
| Demonstrator | The app cards already tile through shared UI components. Partner and funding links share the footer cell's chrome without a local rounded box. |

The 33. Projektetage mirror contains a captured third-party webpage. The maintained presentation shell is part of the presentation renderer work.

## Verification

- Chrome through Playwright rendered the Pets Stories Vite entry point at 1440 and 390 pixels. The two view controls and the roster cells touch and share one pixel-wide seam. The title and action chips paint only their inside edges. Computed glass alpha is 0.76 for the section label and 0.52 for the sandbox action, both under a 0.10 foreground veil. Both views fit the viewport. The repeatable check is `web-check.mjs`. The repo dev target stayed in the fleet capacity queue, so Vite ran directly with the site's owned config and the resolved menagerie.
- The pets stylesheet fixture passed using PostCSS and the repository test-file command.
- The Hub admin component suite passed 24 tests. A full package run also reached a command-routing test that requires the parent repo invocation context and failed in the direct Vitest fallback.
- The staffage viewpoint suite passed 35 tests. Its jsdom environment emitted warnings for Canvas 2D support when running flow-related cases.
- The Course mockup and shared-border suites passed 26 tests.
- The demonstrator shell-brand suite passed four tests, including the partner and funding footer.
- The Play landing-grid regression passed. Its full wasm release build was outside the focused verification.
