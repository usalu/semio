# Round 2: s/OS chrome probe

## Setup
- Real shell not runnable: no wasm-pack/bun, no engine wasm, no generated plugin catalogue (nothing downloaded). Fallback: ticket-local showcase `os-chrome-live.tsx` (mounts real ui-react / os React components with mock props), served by `vite-os-chrome.config.ts` + `os-chrome-site/` (port 5412, stopped), driven by `os-chrome-run.js` + `border-gap-probe.js`.
- Probe changes: violations now report `at` positions; floating shell panels (`[data-slot=panel][data-anchor]`) are a separate layer: pairs between a panel and outside content are skipped unless one side is navbar/footer. Tab must be fronted (compositor commits in rAF); runner shims rAF/transitions in a hidden pane.
- Plugin chrome (CAD, flow, DAG) is mocked as Interpreter documents (toolbar, feature/neuron tree, property sections, inspector, run/progress, results table) in a right panel; canvas marked `data-border-probe-art` (CAD, flow: 1 art root each).

## Fixes (root causes)
- Navbar trailing chrome (fullscreen toggle) was `h-medium`, floated inside the band -> `h-full`, stretched.
- Chrome-hosted open panels: cap is now exactly band height (`--panel-header-height` = navbar/footer height, top = -band height); CSS sets the same var inside navbar/footer so collapsed tab strips fill the band. Trailing-end reserve no longer adds a 3.2px gap (`chrome-safe-area`).
- `anchorPositionStyle` default inset 0 (panels flush with navbar/footer/edges); pane/quick-action safe-area gaps 0; engagement quick actions flush under the Actions chip (`top: var(--size-medium)`, left 0).
- Tree rows: value-column controls stretch to full row height (`self-stretch` + CSS) so consecutive controls share lines; panel/pane scroll areas no longer inset by a hairline (content touching the frame shares its line).
- ChromePanels: keybinding, conflicts (+diff cell), marketplace controls are `SharedBorderRow`s, no gaps.
- Tests updated (intentional geometry change): Panel chrome-hosted position (`--panel-header-height` now a custom property, top `calc(-1 * var(--navbar-height))`), reserve `96px` (was `96 + 3.2`), `panel-inset` assertions `0px`, cap row == band, conflicts buttons selector. `owned-locale-detector-retirement`: 566/566 pass.

## Probe tables (count; 1440x900 unless noted)
| View | 1440 | 375 |
|---|---|---|
| Layout shell: top/bottom docked panels, nested tab bar, collapsed+open, overflowing 14 tabs | 0 (3 views) | mobile layout 0; mobile settings 0 |
| Settings General/Theme/Hotkeys/Defaults/Conflicts, Display, Marketplace, bottom-docked settings | 0 each | 0 (general, theme, keys, conflicts, marketplace, bottom) |
| Mode quad / tab stack / mixed, App modes | 0 | 0 |
| Window with search, engagement, measures, utilities (open one pane at a time) | measures 0, actions 0, utility 0, all-open 0, two windows 0 | see open items |
| Context menu (+fused submenu opened), command (cmdk), UISearch palette, UIFind, window search w/ suggestions, popover | 0 | 0 |
| Notices: stack, toast, error toast, floating pane; close dialog (+failed) | 0 | 0 |
| CAD / flow / DAG plugin chrome | 0 (edges 43/39/>0) | not run |

## Open items (not done)
1. Folded window with engagement options: the quick-actions group is ~12px wider than the "Actions" chip it hangs from (1 pair, 22.39px, 1440 and 375). Fix idea: publish quick-actions width and min-width the chip.
2. 375: several floating window panes open at once overlap each other (up to 8 pairs); one-pane-at-a-time views are 0 except `measures` (14.39, quick-actions vs measures overlap) and `actions` (1px search-zone vs group).
3. Not mounted: StagedActionDialog/OwnedShellDialog, UtilityTree palette, TaskManager, AgentChatPanel, ShellSync popover, CAD/flow/DAG at 375, true ShellHost.
4. HubFirstRun welcome 4.95px (info-box silhouette vs Next button): NOT done.
5. Browser pane became unresponsive at the end (navigate timeouts); the last full 1440 sweep was run in chunks before the final runner edit, then not repeated. Tab-21 left open/stuck; server 5412 and Storybook stopped.
