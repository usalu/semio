# Round 2: s/OS shell in the browser

## What could and could not be probed

- The real shell (ShellHost with plugin wasm) cannot run here: the generated plugin catalogue (`🤖️generated/🧩️plugins`) is absent and `wasm-pack` is not installed. The variant Vite configs fail at config load.
- Used instead: the root Storybook, scope `framework` (`STORYBOOK_SCOPE=framework`, port 5396, cache `.cache/b/p/m/border-os2`), driven through the preview channel with `os-run.js` (this folder) + `border-gap-probe.js`. 52 os/host stories probed. Both `os-shell` and `os-plugins` stories show "plugin artifact missing" and were not probed.
- Not covered (no story exists, so not probed): ShellHost chrome, docks, panel tab bars, palettes, settings, dialogs, context menus, popovers, ShellSearch, plugin panes (CAD/flow/DAG/ink/paint2d/block outside their host stories), TextEditor (renders empty without wasm), World3d/GraphTimeline/IconRender/TiledMap hosts (canvas art, count 0). Settings/ChromePanels recovery row was already converted by another agent; not touched.
- Gotcha: `@semio-tech/ui-react` is prebundled by Vite, so ui edits need a fresh cache dir; after a cache switch the browser must use another origin (`localhost` vs `127.0.0.1`).
- `resize_window` could not narrow the story iframe below ~887 px; the 375 pass is therefore only via the `phone` stories and a 887 px sweep (0 except the clipped Paint2d case below). A true 375x812 run is not done.

## Probe tables (1440 px, after fixes)

| View | Before | After |
|---|---|---|
| SpaceBrowser (default, first-run, loading, stale, invite-ready, redemption-refused, spectator, phone) | 1 (gap 39px section to field row) | 0 (re-run after PresenceBar fix: the last full sweep of this story set was not repeated at 1440, see below) |
| AgentDelegations (12 stories) | 1 each (section top vs refresh button, 17.8px) | 0 |
| HubSignIn (10 stories) | 0 at first, 3-4 once re-probed (row gaps 14-18px) | 0 |
| HubFirstRun | welcome: 1 | welcome: 1 (framework, see below); others 0 |
| TableHost (2) | 14 each | 0 |
| UiInterpreter button / panel / tree | 2 / 1 / 0 | 0 / 0 / 0 |
| BlockListHost editable / empty | 2 / 0 | 0 / 0 |
| InkCanvasHost (2) | 16 each | 0 (canvas objects marked art, 2 roots) |
| Paint2dHost composite | 1 | 2, clipped-scroll case, see below |
| Paint2dHost navigator, Canvas2d, NodeGraph (2) | 0 | 0 |
| ShellSync popover | not renderable | fixed in code, not probed |

## Fixes

- `🏘️SpaceBrowser`, `🤖️AgentDelegations`, `🔐️HubSignIn`: each pane is now ONE vertical `SharedBorderRow` section (header row with title cell + refresh button, status/description cells, forms as nested `as="form"` rows, members aside, errors as destructive cells). No gaps, no `p-4`, no `border-t`. Test updates: SpaceBrowser `aside` assertion.
- `📊️Table`: stepper, row-buttons and editable-text cells are `SharedBorderRow framed={false}` groups; controls lose `h-medium` and share the table row line.
- `🗣️Interpreter` (plugin property panels/inspectors): containers that hold bordered controls, sections, groups and fields render as shared-border rows; non-control children are cells, `separator` nodes in such rows are dropped (dividers exist). `containerIsRow`/`childIsFramed` in `🧱️node-tree`, `UiContainerFrame` `row` prop.
- `🖱️ui` `Field` (only consumer: the interpreter): now a vertical `SharedBorderRow` (label/description/error cells, control in a nested `data-slot="field-control"` row). The fixture-backed test `⚙️puzzle3d-settings-document` still asserts `gap-single` on the field and the wgpu renderer fixture still models a gap: update both (not run).
- `🧩️BlockListHost`: host is one horizontal frame [steps column (scroller inside) | palette]; steps are cells, header = label cell + Add Step button, native `border-l`/padding removed. Test `block-list-presentation` updated (4 pass).
- `🖋️InkCanvasHost`: item overlays carry `data-border-probe-art` (free-positioned canvas objects incl. user tables).
- `🖌️Paint2dHost`: closed `details` summary no longer draws a divider.
- `🔄️ShellSync` attach popover: content is a framed-less vertical row; path input + browse button, attach/detach buttons are cells, quarantined notice a cell (no inset boxes). Not browser-verified, no test run.

## Open items

1. `HubFirstRun` welcome (UIIntroduction card): `introduction-info-box` silhouette SVG vertical edge vs the footer "Next" button-group left border differ by 4.95px (OverviewCard/WindowChrome footer chip, framework agent). Same pattern expected for any OverviewCard footer chip.
2. `Paint2dHost` composite: 2 pairs, 8.5 and 16.5px, between the visible row line and the panel's bottom line where the scroller clips the next (partly visible) row. Probe false positive for scroll clipping (probe does not clip to overflow ancestors); not suppressed.
3. Tests: `vitest-os-rows-b.config.ts` 139/139 earlier except block-list (fixed, 4 pass); `vitest-os.config.ts` filtered run: SpaceBrowser, AgentDelegations, Table, ink pass except the Playwright-only Table case (Chromium missing); the 20 Interpreter suites fail at collection in this environment (pre-existing, not caused by these edits; unrun against the new container code). tsc not run.
4. A 375x812 run, the real shell and the uncovered views above need a working generated catalogue/wasm toolchain or new stories.

Storybook (port 5396) stopped; my browser tab closed.
