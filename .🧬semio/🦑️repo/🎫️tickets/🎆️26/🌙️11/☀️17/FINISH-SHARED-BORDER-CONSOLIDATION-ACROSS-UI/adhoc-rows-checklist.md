# Ad-hoc bordered rows and cards: migration checklist

Read-only inventory. No source file was edited. Base: the audit in `audit-nested-controls.md` (F2 to F9).

## Method and scope

- Search: ripgrep over `*.tsx` for `border` tokens in `className` strings, `cn(...)` arguments and `border*Class` token constants. Border-0, border-t/b/l/r/x/y dividers, `border-transparent`, and tone tokens (`border-info-border` and similar) were filtered out as dividers or tokens.
- Included: `🧰️framework/**` (excluding `🛍️products/📚️course`, `node_modules`, tests, stories) and `✏️s/**`.
- Excluded as generated: `🧰️framework/🛍️products/💻️os/📦️packages/**` (a gitignored package mirror; `rg --no-ignore` finds no `rounded-sm border` hits there).
- Raw hits (after filters): 351 lines across 64+ files. Per area: `ui/elements` 175, `products/os` 135, `ui/targets` 26, `products/presentation` 8, `plugins (✏️s)` 6, other 1.
- Actionable items below: 55 across 16 files. Everything else in the raw hits was classified as fine (see the last section) or as part of the shared primitives that the audit's F1 fixes.

## Legend (target shape)

- **VL**: vertical shared list. One frame, cells stacked, single dividers (`data-shared-border-group` plus `data-orientation="vertical"`).
- **HR**: horizontal row of cells. One frame, cells side by side (`data-shared-border-group` horizontal, or the `shared-border-chips` pattern).
- **SC**: single framed card. Fine as a card, but its controls must become cells.
- **PRIM**: replace the native or raw control with the framework primitive (`Input`, `Textarea`, `Select`, `Button`). The primitive frames itself.
- **DROP**: remove the redundant native border class.
- **NEST**: a frame sits inside another frame. The inner frame goes.

## Summary by file (sorted by item count)

| # | File | Area | Items | Main shapes |
|---|---|---|---|---|
| 1 | `🏘️SpaceBrowser/🟦️.tsx` | products/os | 9 | PRIM x4, HR x2 (selects), SC + HR (invite box), VL + HR (space rows) |
| 2 | `🤖️AgentDelegations/🟦️.tsx` | products/os | 7 | PRIM x1, HR x2, SC x2, VL + HR (delegation rows), NEST (revoke confirm) |
| 3 | `🏛️ShellHost/🟦️.tsx` | products/os | 7 | VL x3 (stacked notices in one frame), NEST x3 (wrappers around framed panes), VL (stall list) |
| 4 | `🖌️Paint2dHost/✍️editing/🟦️.tsx` | products/os | 5 | NEST panel with ~30 framed `control` items, HR and grid cells |
| 5 | CAD `🎛️repl/🟦️.tsx` | plugins | 4 | DROP x3 (triple border), VL (aside), frame check |
| 6 | `✏️TextEditor/🟦️.tsx` | products/os | 4 | PRIM x3 (raw buttons in footer), HR (footer), PRIM x1 (rename input) |
| 7 | `🔐️HubSignIn/🟦️.tsx` | products/os | 4 | PRIM x4 (native inputs and select), HR (select + button) |
| 8 | `🧩️BlockListHost/🟦️.tsx` | products/os | 3 | HR (block row), NEST (step containing blocks), DROP (palette group) |
| 9 | `🖋️InkCanvasHost/🪟️ink-overlays/🟦️.tsx` | products/os | 3 | HR (format toolbar), PRIM x2 (native text surface, cell input) |
| 10 | `📌️ChromePanels/🎨️settings-theme/🟦️.tsx` | products/os | 2 | PRIM x2 (native color inputs with double border), HR (paint row) |
| 11 | `🎴️IconSelector/🟦️.tsx` (framework ui) | ui/elements | 2 | VL (outer card with `border p-2`), cell (preview box) |
| 12 | CAD `📋️repl-panes/🟦️.tsx` | plugins | 1 | DROP (triple border), plus import cleanup |
| 13 | `🛂️SpaceAdministration/🟦️.tsx` | products/os | 1 | SC with HR buttons, target dialog footer group |
| 14 | `🕸️NodeGraph/🖼️dag-overlays/🟦️.tsx` | products/os | 1 | HR (align toolbar, unframed buttons with gap) |
| 15 | `📝️staged-args/🟦️.tsx` | products/os | 1 | PRIM (native color swatch next to chip toolbar) |
| 16 | `🧱️window-chrome/🟦️.tsx` (targets/react) | ui/targets | 1 | Review: `windowMeasureTileClass` (nested measure leaves) |

Totals: products/os 47, plugins 5, ui/elements 2, ui/targets 1 (review), products/presentation 0.

---

## 1. SpaceBrowser

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏘️SpaceBrowser/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 88 | `w-full min-w-0 rounded-sm border px-single py-1 text-sm` | native `<input>` (space name), label above | yes, in a form column | PRIM `Input` |
| 93 | `w-full min-w-0 rounded-sm border px-single py-1 text-sm` | native `<select>` (kind) | yes, sits with line 100 in `sm:flex-row gap-2` | HR cells with 100, PRIM `Select` |
| 100 | same | native `<select>` (visibility) | same | HR cells with 93, PRIM `Select` |
| 187 | same | native `<select>` (role, invite) | yes, `sm:flex-row` with 194 | HR cells with 194, PRIM `Select` |
| 194 | same | native `<select>` (ttl, invite) | same | HR cells with 187, PRIM `Select` |
| 206 | `flex flex-col gap-2 rounded-sm border px-single py-1` (`role="status"`) | framed invite card: text, output link, copy and dismiss buttons in `sm:flex-row` | single card | SC; buttons (copy, dismiss) become HR cells; card becomes VL group |
| 237 | `w-full min-w-0 rounded-sm border px-single py-1 text-sm` | native `<input>` (redeem code) above submit Button | yes, form column | PRIM `Input` |
| 268 | `flex flex-col gap-1 rounded-sm border px-single py-1 sm:flex-row sm:items-center sm:justify-between` (`li`) | space row: open button (text) plus archive `Button` | yes, `li` list | VL list of rows; each row HR (open cell, archive cell); one frame per list, not per row |
| 348 | `w-full min-w-0 rounded-sm border px-single py-1 text-sm` | native `<input type="search">` | yes, field column | PRIM `Input` |

## 2. AgentDelegations

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentDelegations/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 97 | `w-full min-w-0 rounded-sm border ...` | native `<input>` (agent name) | yes, form column | PRIM `Input` |
| 102 | `w-full min-w-0 rounded-sm border ...` | native `<select>` (audience) | yes, `sm:flex-row` with 109 | HR cells with 109, PRIM `Select` |
| 109 | same | native `<select>` (ttl) | same | HR cells with 102, PRIM `Select` |
| 141 | `flex flex-col gap-2 rounded-sm border px-single py-1` (`role="status"`) | credential card: text, principal, download and dismiss buttons (`sm:flex-row`), command code token | single card | SC; buttons HR cells; `code` token stays |
| 184 | `flex flex-col gap-2 rounded-sm border px-single py-1` (`role="group"`) | MCP client card: title, install button, status, `pre` config, copy button | stacked controls in one card | SC; convert to VL group with buttons as cells; `pre` stays a token |
| 234 | `flex flex-col gap-1 rounded-sm border px-single py-1` (`li`) | delegation row: info block plus revoke `Button` | yes, `li` list | VL list of rows; each row HR (info cell, revoke cell) |
| 247 | `flex flex-col gap-2 rounded-sm border ...` (`role="group"`, revoke confirm) | confirm box inside the row `li`: title, body, confirm and cancel buttons | nested inside a framed row | NEST: make it a section of the row separated by a divider, buttons HR cells, no own frame |

## 3. ShellHost

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 217 | `flex flex-col gap-single rounded-sm border bg-base px-double py-single` | bootstrap status notices, each with a cancel control | yes, one frame with gap | VL shared group (notices as cells) |
| 224 | `... mt-double flex-col gap-single rounded-sm border ...` | execution-target notices, each with cancel | yes | VL shared group |
| 231 | `... max-w-[28rem] flex-col gap-single rounded-sm border ...` | artifact creation catalog notice and progress notices, with cancel and open controls | yes | VL shared group |
| 239 | `w-[28rem] rounded-sm border bg-base px-double py-single` | wrapper around `InstalledServicePanelV1` (controls) | NEST if the panel frames its controls | NEST: drop wrapper border, panel owns frame (audit F10) |
| 249 | `max-h-[80vh] w-full max-w-[44rem] overflow-auto rounded-sm border bg-base` | wrapper around `HubWorkspace` (framed SpaceBrowser and AgentDelegations) | NEST | NEST: drop wrapper border, workspace is the only frame |
| 264 | `max-h-[70vh] w-[36rem] overflow-auto rounded-sm border bg-base` | wrapper around `SpaceAdministrationPane` | NEST | NEST: drop wrapper border |
| 317 | `mt-double rounded-sm border border-normal bg-menu ...` (`data-semio-command-stall`) | stacked stall lines, each with an underline cancel button | yes, unframed `div` items | VL list (stall rows) inside the band; medium priority |

Checked and fine (not in the count): 244 (directory bootstrap, single standalone notice), 278 (route admission banner), 295 (plugin install band, text plus progress), 382 (checkin band, text and underline text buttons, standalone), 416 (fixed corner context trigger, intentional).

## 4. Paint2dHost: editing panel

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 34 | `const control = "rounded border border-border bg-background px-2 py-1 ..."` | shared class applied to about 30 controls (selects, number inputs, buttons, grid tools) at lines 402 to 438 | inside a framed panel | PRIM `Select`, `Input`, `Button`; remove the per-control `border` from the shared class |
| 401 | `... rounded border border-border bg-background/95 p-...` | floating panel (`section`) holding all controls | NEST (controls framed inside a framed panel) | VL shared group: sections (target, layer, tools, color and size, selection, adjustment) as cells |
| 409 | `role="toolbar" grid grid-cols-3 gap-1` | tool buttons, each with `control` (own frame) | grid of framed buttons | grid of cells in one frame (gap removed) |
| 410 to 411 | `flex gap-2` | color input and size input, each framed | HR of framed controls | HR cells |
| 430 to 435 | `grid grid-cols-2 gap-1` | label and input pairs with `control` | grid of framed controls | grid cells in one frame |

## 5. CAD REPL (plugin)

Path: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 148 | `aside` with `data-shared-border-frame` and `floatingPaneAsideClass` | floating editor pane, label rows with selects | yes, `flex-col gap-single` | VL group (single frame). Check: `floatingPaneAsideClass` (`🎨️chrome-classes/🟦️.ts` L34) includes `shellChromeBorderClass`, which may draw a second native border on the same element |
| 251 | `cadFieldClass, "rounded-md border px-single py-half", borderNormalClass` | `SelectTrigger` (model definition) | yes, label row | DROP the three border classes (primitive frames it), keep as cell |
| 286 | same | `SelectTrigger` (transfer from) | same | DROP, cell |
| 311 | same | `SelectTrigger` (incoming) | same | DROP, cell |

## 6. TextEditor (product)

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 663 | `rounded border border-border px-3 py-1 text-xs` (raw `<button>`) | discard button in a `border-t` footer row | one button, footer row | PRIM `Button` as a cell of the footer HR group |
| 670 | `shrink-0 rounded border border-border px-3 py-1 text-xs ...` (raw `<button>`) | pending cancel button in footer | footer row with 671 | PRIM `Button`, HR shared group (drop `gap-2`) |
| 671 | same | discard draft button | same | PRIM `Button`, HR cell |
| 230 | `pointer-events-auto absolute z-50 min-w-[12rem] rounded border border-border px-2 py-1 font-mono ...` (`<input>`) | rename input, floating | standalone | PRIM `Input` |

Checked and fine: 256 (suggestion menu surface, `rounded border border-border p-1`), provided its items are not framed themselves.

## 7. HubSignIn (product)

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔐️HubSignIn/🟦️.tsx`

Class at line 107 belongs to the `<input>` opened at line 92. Line 181 belongs to the `<select>` opened at line 175. Line 236 belongs to the `<input>` opened at 223. Line 255 belongs to the `<input>` opened at 242.

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 107 | `w-full min-w-0 rounded-sm border px-single py-1 text-sm` | native `<input>` (sign-in field) | form column | PRIM `Input` |
| 181 | `w-full min-w-0 rounded-sm border px-single py-1 text-sm sm:flex-1` | native `<select>` next to a submit `Button` (line 188) | yes, row with the button | HR cells (select and button), PRIM `Select` |
| 236 | `w-full min-w-0 rounded-sm border px-single py-1 text-sm` | native `<input>` | form column | PRIM `Input` |
| 255 | same | native `<input name="invitation">` directly above the redeem `Button` (line 260) | yes, form column with button | PRIM `Input`, HR cell with the button |

Checked and fine: 204 (error callout, token colours on a single standalone box).

## 8. BlockListHost (product)

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧩️BlockListHost/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 83 | `semio-block-card flex items-center gap-2 rounded border border-border bg-background p-single` | block card: drag handle, selection label, delete `Button` | yes, blocks stacked in a step | HR row (handle, label, delete) inside a VL list; remove own frame when the step already frames it |
| 115 | `semio-step-card flex flex-col gap-2 rounded border border-border bg-background p-single` | step card: header row (handle, label, delete), description, nested blocks | NEST (blocks are framed inside a framed step) | VL group: header cell, description cell, blocks as nested cells without their own frame |
| 197 | `flex flex-col rounded border border-border` with `data-shared-border-group` | palette group (entries already cells, line 168) | group | DROP the native `rounded border border-border` (keep the group attribute) |

Checked and fine: 168 (palette entry cell, `border-0`, correct reference).

## 9. InkCanvasHost ink-overlays

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🪟️ink-overlays/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 37 | `bg-background/95 mb-1 flex gap-1 rounded border p-1 shadow-sm` | formatting toolbar: B, I and other buttons (unframed) | HR of buttons separated by `gap-1` | HR shared group (cells, no gap) |
| 84 | `text-foreground bg-background h-[calc(100%-2rem)] w-full overflow-auto rounded border p-2 outline-none` | editor body (native text surface, tag not verified) | standalone | PRIM `Textarea` (verify tag) |
| 119 | `bg-background ring-primary absolute z-30 rounded border px-2 py-1 text-sm ring-2 outline-none` (`<input>`) | cell editor input, floating | standalone | PRIM `Input` |

Checked and fine: `ink-block-views` 90 (block frame around table content), 123 and 133 (table grid borders), 184 and 188 (selection and handles).

## 10. ChromePanels: settings-theme

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🎨️settings-theme/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 19 | `cn(borderElementClass, "h-small w-16 shrink-0 rounded border bg-background")` (`type="color"`) | native color input, double border (native plus token) | control in a paint row | PRIM color control, DROP the native `border` |
| 175 | `cn(borderElementClass, "h-small w-10 shrink-0 rounded border bg-background")` | native color input (alpha row) | row with the alpha input | HR cells, DROP the native `border` |

## 11. IconSelector (framework ui elements)

Path: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 199 | `flex min-w-0 flex-col gap-2 rounded-md border p-2` | outer card holding Select, Textarea, preview and the import and clear buttons | yes, `gap-2` stacking | VL shared group (single frame, drop `p-2` padding); children become cells |
| 240 | `bg-muted/30 flex min-h-peta items-center justify-center overflow-hidden rounded-sm border px-1 py-2` | preview box (non-control content) | stacked child | cell of the VL group, drop its own border |

Note: Select (mode) and Textarea inside this card are primitives and self-frame (audit F1). Once the card is a group, they must become cells.

## 12. CAD repl-panes (plugin)

Path: `✏️s/🔌️plugins/📐️cad/.../✏️editor/⚙️engine/📺️renderer/📋️repl-panes/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 110 | `cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)` | `SelectTrigger` (enum field) inside attribute rows | yes, attribute rows | DROP the three border classes, cell. Also remove the `borderNormalClass` import on line 9 once unused |

## 13. SpaceAdministration (product)

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 366 | `flex flex-col gap-2 border p-2` (`role="alertdialog"`) | delete confirmation: title, warning, confirm and cancel buttons (`flex gap-2`) | single card with buttons in a row | SC for the alert; buttons as cells of a horizontal dialog-footer shared group (as `Dialog` footer does, `💬️Dialog/🟦️.tsx` L596). Also `border` without `rounded` is inconsistent with the other cards |

## 14. NodeGraph dag-overlays (product)

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🖼️dag-overlays/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 842 | `pointer-events-auto absolute z-50 flex gap-0.5 rounded border border-border p-0.5 shadow-sm` | align toolbar, six unframed `size-5` buttons | HR with `gap-0.5` | HR shared group (cells, no gap) |

## 15. staged-args (product)

Path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 100 | `h-medium w-medium shrink-0 cursor-pointer rounded-sm border` (`type="color"`) | native colour swatch next to the chip toolbar | row with chips | PRIM colour control, cell in the reference row |

Checked and fine: 60 (reference chips, intentional tokens).

## 16. window-chrome measure tile (targets/react): review

Path: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧱️window-chrome/🟦️.tsx`

| Line | className (excerpt) | Contents | Stacked? | Target |
|---|---|---|---|---|
| 51 | `windowMeasureTileClass = cn("... rounded-sm border", borderElementClass/40, "px-tiny py-tiny")` | measurement tile; comment at line 68 says nested measure leaves have no outer tile border | nested possible | REVIEW: no in-scope consumer found; confirm nested measure tiles do not frame twice |

---

## Fine as-is (skipped)

- Standalone text cards and callouts: `🚦status-surface` StatCard (line 100, verify it is not a stacked grid), `📣display-components` callout (line 50), `🌳Tree` TreeFiles card (line 638, links only, per audit), ShellSync quarantine pill (`🔄️ShellSync/🟦️.tsx` 160), Interpreter unknown-component box (`display-views` 152).
- Standalone floating bands and notices: ShellHost session-notice, transient-notice, close-guard (verify its buttons are not in a framed row), local-folders reconnect banner, check-in band, route admission, plugin install band, local-catalog list card (list items are unframed text buttons), ChromePanels conflict diff preview box (line 131).
- Chips and tokens: Badge and StatusChip (`status-surface` 49), `LayeredOverview` overview-button (16) and hints (67), `ContextMenu` colour swatch (`item-render` 49), `TableAvatar`, `PresenceBar` avatar rings, staged-args chips (60).
- Overlays and drag previews: `Tree` drag ghost (248), `Tree` drop-preview (`drag-drop` 63), `Canvas` drag preview (`drag-preview` 9), `Canvas` activity highlight (132), `Panel` drop zone (`Panel` 310, dashed placeholder), `Scene` decorative frames (504 to 506), `ChromeControlHint` tooltip, `panel-dock` tutorial tooltip (218), `NodeGraph` presence legend (182) and message pill (589), `diagram-node` body (92), `WindowSilhouette`, `Skeletons`.
- Dividers and grid lines: `Table` row and header borders, `HistoryTable` row divider, `Command` input wrapper, `Canvas` footer dividers, `fullscreen` titlebar, `Navbar` and `Footer` rules, `TaskManager` `divide-y` list, `AgentChatPanel` `divide-y` list, Interpreter node-tree header row, ink table cells (123, 133).
- Already shared-border (reference): `Stepper` (`🪜️Stepper` 257, 262, 275, 343), `shared-border-chips` (`🔗️shared-border-chips` 16), `Dialog` footer (596), `Dialog` and `Popover` frames, `ToggleGroup`, `ActionGroup`, `Tabs`, `PanelTabBar`, `UIDialog` group (135), `Ribbon`, `Navbar` group (195), `Layout` frame (69).
- Overlays on canvas and presentation: `presentation` `morph-views` (`boxSizing` only), `presentation` `shared-border-chips`, `Canvas2dHost` gumball and `wasm-graph` selection rectangles, `Paint2dHost` overlay (705).
- Primitives (F1 scope, not ad hoc): `ButtonGroup` 63 to 77 and `Button`, `Input` 297 to 299, 382, 489, 510, `Textarea` 48, 111, 119, `Select` 401 to 430, 557, 663 to 681, `Toggle` 136, 184, 209, `ToggleGroup` 90, 155 to 162, `ActionGroup` 60, 208 to 236, `Popover` 320, 412 to 425, `Dialog` 437, 538 to 552, `Navbar`, `Footer`, `PanelTabBar`, `Tabs`, `UIDialog`.
- Products with no ad-hoc frame found: `ChromePanels` settings-defaults (only the diff box at 131), `World3dHost` (only geometry, no className frames in scope), `TiledMapHost`, `Board2dHost`, `ShellSync` (pill only), `Interpreter` control-port.

## Not found or out of scope

- `products/📚️course`: excluded by the brief, not scanned.
- `✏️s/**` outside `🔌️plugins`: only stories and dev fixtures carry border classes, which are out of scope.
- Audit items F9 to F12 that are not border-class hits (TaskManager buttons in cells at 452 to 454 and 717 to 722, `Table` cell action at 230) are framed `Button` primitives. They are covered by F1 and are not listed here.

## Suggested migration order

1. F1 primitives (nesting-aware `Button`, `Input`, `Textarea`, `Select`, `Toggle`): fixes most PRIM and DROP items at once.
2. Shared VL and HR group helpers for the ad-hoc cards: SpaceBrowser, AgentDelegations, SpaceAdministration, BlockListHost, HubSignIn, TextEditor footer.
3. NEST cleanup: ShellHost wrappers (239, 249, 264), AgentDelegations confirm (247), BlockListHost step (115), Paint2d panel (401).
4. Local fixes: IconSelector (199, 240), CAD REPL triple border (251, 286, 311, repl-panes 110), ink-overlays toolbar (37), dag-overlays align toolbar (842), ChromePanels colour inputs (19, 175), staged-args swatch (100).
