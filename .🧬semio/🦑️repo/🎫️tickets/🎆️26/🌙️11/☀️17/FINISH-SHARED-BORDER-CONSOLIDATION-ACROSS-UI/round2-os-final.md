# Round 2: s/OS chrome, final pass

Showcase `os-chrome-live.tsx` (+ new `os-chrome-more.tsx`) served natively on 5397 by `vite-os-chrome.config.ts`, own tab, killed and closed. Runner `os-chrome-run.js` now always shims rAF on a MessageChannel pump (another agent fronting its tab stopped frames mid-run) and prints `pn` (anchored panels) and `ov` (overlap pairs from the probe another agent added). Machine load was 25-58 and the vite server wedged at 95 % CPU after several HMR edits (restarted 4 times, `kill -9`); the browser pane was often hidden by other agents, so each probe call ran right after `tabs_select`.

## 1. Folded quick-actions group wider than its "Actions" chip: fixed
`🪟️Window/🟦️.tsx`: the quick-actions group width is measured (`getBoundingClientRect`, ceil) into `--window-quick-actions-width` on the window body; the folded Actions pane gets `min-w-[var(--window-quick-actions-width)]`, its stack `w-full` and `data-pane-chip-fill`, and `🎨️.css` (`[data-pane-chip-fill] > [data-window-silhouette-chip]` and the toggle: `flex: 1 1 auto`) stretches the chip. The group now omits its own top edge (`data-shared-border-edges="right bottom left"`, `border-t-0`): the chip's bottom line is the shared line, so the two owners cannot render two lines 1.4 px apart (that happened at 375 even with equal widths). Engagement's own option group does the same when it is the first row (`🎮️Engagement`, `optionsFirst`): its top is the pane body top, which the silhouette draws (fixed the 1.39 px pair against the Search chip at 375).

## 2. 375: several panes open at once overlapped: fixed (one body at a time)
New module `🔨️modules/🪟️window-pane-exclusivity` (`🟦️.ts`, `exclusivity.cases.json` + `.schema.json`, `exclusivity.test.ts`): in a mobile window or a window body < 480 px only one pane body is shown (the one opened last, else priority engagement > search > measures > utility); the other chips stay as a row of tabs. Opening a pane folds the others (also through their controlled callbacks); the quick-actions group is hidden while a top-row body (search/measures) covers it. `Window` implements it (`togglePane`, `lastOpenedRef`); jsdom test `🧪️tests/🧩️component` + fixture `🔀️pane-fold-exclusive` (6 cases, en + de). Two windows side by side at 375 is not a real mobile layout (Mode has `mobile`, showing one window); the showcase now passes `mobile` like the shell does. Windows narrower than ~220 px on desktop still cannot hold three chips without overlap (not addressed).
Also: `📌️WindowPane` `chromePanelGapPx` default `-0.5` (safe-area offset rounds outward to whole pixels, left a 0.83 px gap between a window utility chip and a bottom-left panel = two lines 2.8 px apart; now one line).

## 3. Mounted and probe-clean (counts at 1440 and 375, tab fronted for each call, `pending:false`)
| Part | Fixes | 1440 | 375 |
|---|---|---|---|
| StagedActionDialog (error, pending de) | none needed | 0 / 0 | 0 / 0 |
| OwnedShellDialog (in a shell scope with a portal layer) | none | 0 | 0 |
| UtilityTree + SelectionUtilityOptions | wrapped rows had a row gap (8.4 px): `gap-y-0` | 0 / 0 | 0 / 0 |
| TaskManager panel + tasks + empty | action cells flush (`!px-0`, stretch, no gap), task rows are bottom-edge frames (no `divide-y`), no inset padding | 0 / 0 | 0 / 0 |
| AgentChatPanel (+ approvals, empty) | rows are bottom-edge frames, cancel/decision buttons flush in the row, composer textarea + full-width Send flush, no inset padding (`🤖️AgentApprovals`: buttons are one flush `flex-1` row) | 0 / 0 | 0 / 0 |
| ShellSync attach popover (file, remote) | popover docks flush (`sideOffset={0}`; was 6 px above the toggle group at 375) | 0 / 0 | 0 / 0 |
| HubFirstRun (steps 0, 1, 2, 3, de) | see 4 | 0 | 0 |
| UISearch / UIFind in a shell scope | none | 0 | 0 |
| CAD / flow chrome | at 375 mounted as the mobile panel (real shell), not the desktop panel | 0 (1440 pairs: see "Open") | 0 / 0 |
| DAG chrome | mock table was invalid (threw, so DAG never rendered in the previous round): fixed with real `table`/`tableRow` nodes; Interpreter table: lives directly in its section (`childIsFramed`), header and rows are bottom-edge frames instead of css `border-b`/`borderNormalTopClass` | 0 | 0 |
Window views at 1440 (all 6 rich states) 0; at 375 (folded, measures, actions, utility, open, many) 0. Also 0 at 1440 and 375: quad, mode-stack/mixed, app, context (+sub), command, popover, notices (stack/toast/error/pane), close dialogs, window search.

## 4. HubFirstRun 4.95 px: not reproducible, no change
Welcome card, steps 0-3, en and de, at 1440 and 375: 0 pairs. Geometry: Next group `850.31..918.28` flush with the info box right edge 918.28 and bottom 118.34. The framework agents' zero placement gap and flush docking (UIIntroduction) removed it.

## 5. Probe review (`border-gap-probe.js`, `pending` kept)
The previous rule skipped every pair across any `[data-slot=panel][data-anchor]`, which also hid docked panels vs panel, vs window content beside them, and vs navbar/footer. New rule: panels are opaque overlays, so only an edge hidden BENEATH a panel it is not part of (midpoint of the shared span inside the panel rect, 1.5 px shrink) is ignored; panel vs band, panel vs panel, panel vs visible neighbours and panel-internal edges are all checked. Result field `panels` replaces nothing else. I kept the modal-layer and `overlaps` additions another agent made to the same file concurrently.

## Tests
- `vitest-ui.config.ts` regression (owned-locale-detector-retirement driver, WindowChrome, OverviewCard, CardStack, LayeredOverview, WindowSilhouette, Window, window-pane-exclusivity): 705/705 (was 702 before this pass; added the exclusivity cases and one Window test).
- `vitest-os-rows-b.config.ts`: 139/139 (re-run now). `vitest-interpreter.config.ts`: 195/195.
- New `vitest-os-final.config.ts` (TaskManager, AgentChatPanel, AgentApprovals, ShellSync, UtilityTree, HubFirstRun): 158/158. The `bun:test` files (ShellHost staging session/lifecycle/execution, ActionGroup spacing) cannot run under vitest or here (no bun); untouched.

## Open / not done (with evidence)
1. Concurrent changes by another agent break the navbar/footer seam: `Navbar`/`Footer` now `h-[round(nearest,var(--navbar-height),var(--device-pixel))]` (19:26) and `createSharedBorderSnap` tolerance became 0.75 while I worked. Result: every desktop view with a navbar and a window directly under it (layout, layout-bottom/many, plugin-cad/flow/dag at 1440, all PanelShell settings/display/marketplace views at 1440, marketplace at 375) reports 1 px pairs (`navbar svg 28.0 || window svg 29.0`, same at the footer, 3-5 per view), though boxes coincide exactly (`navbar 0..29`, `mode-body 29..783`). Same views were 0 earlier in the session and `layout` flips between 0 and 7 between runs. Not caused by this ticket's edits (I reverted my own snap experiment: tolerance 1 made it worse). Needs the owner of that change.
2. settings-conflicts/marketplace at 1440: panel-internal rows end 13-17 px before the panel edge (scrollbar gutter of the overflowing panel body) vs the window edge behind it.
3. `overlaps` (new probe field): button group items overflow their group by 1 px (item `66.52..88.91` in group `65.52..87.91`: group has a 1 px border and `h-medium`, item `min-h-medium`), flagged in agent-chat against the next row; UISearch/UIFind Close button over the input; window search suffix text over its input (intentional overlay). Generic ButtonGroup geometry, not changed.
4. Not mounted: the real ShellHost (no wasm engine/plugin catalogue).
5. Showcase files stay in the ticket: `os-chrome-live.tsx`, `os-chrome-more.tsx`, `os-chrome-run.js`, `vite-os-chrome.config.ts`, `vitest-os-final.config.ts`. HMR edits wedge vite; restart it after edits instead of waiting.
