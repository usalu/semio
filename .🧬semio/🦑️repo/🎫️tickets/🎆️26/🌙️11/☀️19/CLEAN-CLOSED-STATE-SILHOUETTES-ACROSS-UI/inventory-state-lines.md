# 🔎 Inventory: State Lines Outside the Shared-Border Compositor

Read-only inventory for `design.md` and `live-baseline.md`. No source edited. Paths are repo-relative from `/Users/ueli/Documents/semio`.

## Method

- Tailwind state tokens (`hover:`, `focus:`, `focus-visible:`, `focus-within:`, `aria-*:`, `data-[…]:`, `group-*:`, `peer-*:`) carrying `border*`, `ring*`, `outline*`, `shadow*`, `underline`, `after:`, `before:` in `.tsx`, `.ts`, `.css`, `.html`.
- Unprefixed state-driven utilities (`ring-*`, `outline-*`, `border-b/t/l/r-*`, `underline`, `shadow-*`) in `.tsx` and presentation `.ts`, filtered to UI and element code. Geometry, IFC, 3D and test-only hits were dropped.
- CSS rules whose selector contains a state (`:hover`, `:focus*`, `:active`, `:checked`, `:has`, `[aria-*]`, `[data-state|active|selected|celebrated|introduced|loading|waiting|…]`, `.active`, `.selected`) and whose body sets `border*`, `outline*`, `box-shadow`, `ring*`, `text-decoration*` or an `animation` that draws a ring. A brace matcher marked which rules sit inside `@layer shared-border-ownership` and `@layer shared-border-priority`.
- Participation: `data-shared-border-*`, `useSharedBorderFrameRef`, `SharedBorderRow`/`SharedBorderRowCell`, `WindowSilhouette`. A "cell" means a direct child of a `SharedBorderRow` owner.
- Excluded: `variant="outline"` on `Button` (a static outlined button, not a focus or state line), `ring` as a type name (`kind: "ring"`), SVG `outline` geometry, test-only assertions, hover/reduced-motion icon animations, and static decorative borders with no state condition.

Legend for the proposed target:

- **(a)** Becomes a compositor box. Frame with its edges as today, or `data-shared-border-edges=""` for a borderless control whose silhouette is stated only in its state.
- **(b)** Fill-only state. Keep it. The line parts must go, not the fill.
- **(c)** Delete the line. The compositor already paints the state, or the rule is a removal that exists only to hide a browser or CSS line.

## 1. Compositor stylesheet outside the two owned layers

File: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`. Layers: `shared-border-ownership` at 1462–1466 and 6849–6863; `shared-border-priority` at 6814–6847. Rules below are outside those ranges and still paint state lines.

| # | Line | Selector / rule | State | Participant | Target | Note |
|---|---|---|---|---|---|---|
| C1 | 1270–1310 | `@utility border-loading` (`::after` masked ring, spinning) plus `border-loading-active` / `-element` | loading | n/a (utility) | (a) | Loading is a compositor state. Delete the `::after` ring. Non-participant users go to a box. |
| C2 | 1314–1353 | `@utility border-waiting` (`::after` dashed ring) plus `-active` / `-element` | waiting | n/a | (a) | Same as C1. |
| C3 | 1374–1387 | `[data-celebrated="true"]` `position:relative`, `::after` conic ring, `animation: celebrate-border-spin/burst` | celebrated | n/a | (a) | Celebrated is a compositor state. Remove the `::after` ring. |
| C4 | 1409 | `[data-window-silhouette] [data-slot="window"][data-celebrated="true"]::after { animation:none !important }` | celebrated | silhouette | (c) | Suppressor for C3. Delete with C3. |
| C5 | 1426–1427 | `[data-introduced="true"] { box-shadow: inset … introduced-border-color; animation: introduced-border-pulse }` | introduced | n/a | (a) | Introduced is a compositor state. Remove the inset ring and pulse. |
| C6 | 1434–1436 | silhouette suppressor for `[data-introduced]` `box-shadow:none !important` | introduced | silhouette | (c) | Delete with C5. |
| C7 | 1522–1534 | reduced-motion block; `[data-introduced="true"] { box-shadow: inset 0 0 0 var(--stroke-default) … }` | introduced | n/a | (a) | Static stroke fallback for reduced motion. Compositor must own it with the same stroke. |
| C8 | 1538 | silhouette suppressor `box-shadow:none !important` for `[data-introduced]` | introduced | silhouette | (c) | Delete with C7. |
| C9 | 1791 | `.introduction-demo-callout[data-double="true"] .introduction-demo-callout-chip:has(~ …) { border-block-end-width: var(--stroke-hairline) !important }` | demo chip state | review | review | Border width used as a state divider. Decide with the introduction owner; likely (c). |
| C10 | 7518–7521 | `[data-window-silhouette-chip] :is(button,…):focus-visible { outline:none; border-style:none; border-color: transparent !important }` | focus | silhouette chip | (c) | Removal only. Keep the removal; the chip silhouette paints focus. |
| C11 | 7611–7612 | `[data-slot="introduction-info-box"]:hover, [data-slot="dialog-content"]:hover { border-color: transparent !important }` | hover | dialog | (c) | Hides a border on hover. Delete. Compositor draws the dialog frame. |
| C12 | 7658 | `[data-celebrated="true"]:is([data-slot="button-group-item"], [data-slot="toggle-group-item"], [data-slot="action-group-item"], [data-slot="mode-dock-…"]) { animation: celebrate-border-spin … }` | celebrated | ButtonGroup, ToggleGroup (participants) | (a) | Remove the ring; the cell's compositor stroke carries celebrated. |
| C13 | 7706 | same family, second block | celebrated | participants | (a) | Same as C12. |
| C14 | 7748 | same family, third block | celebrated | participants | (a) | Same as C12. |
| C15 | 7771 | `[data-celebrated="true"]:is([data-slot="tree-item-row"], [data-slot="tree-section-row"], [data-slot="tree-property-item"], [data-slot="tree-row"], …)` `animation: celebrate-border-spin` | celebrated | Tree (cells) | (a) | Remove the ring. |
| C16 | 7808 | `:is([data-slot="tree-section-content"], [data-slot="tree-item-content"], …) { animation: celebrate-border-spin }` | celebrated | Tree | (a) | Remove the ring. |
| C17 | 7888–7891 | `[data-slot="window-measures-tree"] [data-slot="select-trigger"][data-state="open"]` and `:focus-visible` `{ border-color: var(--accent); box-shadow:none; outline:none }` | open / focus | select trigger | (a) | Open and focus recolour the trigger's border. Make the trigger a box with state `active` or `focus`. |
| C18 | 7895–7896 | `[data-slot="window-measures-tree"] [data-slot="toggle-group-item"][data-state="on"] { box-shadow:none }` | on | toggle item | (c) | Removal. Delete once the compositor draws on-state. |
| C19 | 7929–7931 | `.semio-scope[data-contrast="more"] :focus-visible { outline: 3px solid currentColor; outline-offset: 2px }` | focus (high contrast) | global | (a) | Global browser-style ring on every focusable. Replace with a compositor stroke weight token. Delete the rule. |
| C20 | 1083–1093 | hover fill (`--hover-ink`, element gray) | hover | n/a | (b) | Fill. Keep. |
| C21 | 7347–7350 | `[data-layered-backdrop] *` border colours and outlines transparent | backdrop | n/a | (c) | Removal. Keep. |
| C22 | 6594–6812 | `ShellParentHover`, shell chrome hover, tree value column, panel tab strip | hover | participants | (b) | No state border or outline declarations outside the owned layers. Nothing to change. |

Excluded from this file: 1893–1901 and 3546–3654 (hover icon animations), 6572 (reduced motion), 3580–3654 `icon-part` animations. None draws a state line.

## 2. UI module: presentation classes (`🧰️framework/🔨️modules/🖱️ui/🔨️modules/`)

These classes are reused by many elements. Changing one changes every consumer.

| # | File:line | Class | State | Consumers (tsx, in scope) | Target | Note |
|---|---|---|---|---|---|---|
| P1 | `🖱️interaction-presentation/🟦️.ts:36` `interactiveActiveBorderClass = "border-active-base"` | pressed / selected | Table, Canvas app tabs, Engagement option, PanelTabBar, Window ActionGroup, WindowSearch, ActionGroup, Tree row (via `interactiveActiveFillClass` line 39) | (b) for fill, (a) for border | Split the fill (`bg-active-base`, text) from the border. The border goes to the compositor box. |
| P2 | `🖱️interaction-presentation/🟦️.ts:17` `hover:not-data-[handle-hovered=true]:border-active-base` (`hoverExcludingHandleActiveBorderClass`, used in `interactiveActiveFillClass` line 39) | hover on active | same consumers as P1 | (c) | Hover border line on an active fill. Compositor hover draws it. |
| P3 | `🖱️interaction-presentation/🟦️.ts:44,47` `data-[state=on]:border-active-base` and `:hover:border-active-base` (`interactiveOnClass`) | on | ToggleGroup, ButtonGroup and others | (a) | Participants keep the state in the box. Delete the border classes. |
| P4 | `🖱️interaction-presentation/🟦️.ts:54,57` `data-[state=active]:border-active-base` and `:hover:border-active-base` (`interactiveTabActiveClass`) | active tab | `📑️Tabs/🟦️.tsx:213` | (c) | The selected trigger already carries `data-shared-border-frame`, edges, and `data-shared-border-state="active"`. The CSS border duplicates it. Keep only the fill. |
| P5 | `🔨️modules/🎛️chrome-control-presentation/🟦️.ts:57,60` `data-[active=true]:border-active-base` and `:hover:` (`chromeControlTabActiveClass`) | active tab | no tsx consumer found in scope (check for dead code) | (c) if live, otherwise delete | Same as P4. |
| P6 | `📝️form-control-presentation/🟦️.ts:14` `formControlFocusBorderClass`: `focus-visible:border-accent data-[state=open]:border-accent aria-invalid:border-destructive focus-visible:ring-0 shadow-none outline-none` | focus, open, invalid | `✏️Input` (513), `🔤️Textarea` (122), `🔽️Select`, `⚡️ActionGroup` (import) | (a) for the three colour states, (c) for `outline-none`, `ring-0`, `shadow-none` | Input and Textarea are compositor frames. Focus, open and invalid recolour the frame edges in the overlay. Delete the border-colour classes. Keep the removals. |
| P7 | `📋️menu-item-presentation/🟦️.ts:19` `data-[selected=true]:border-active-base` (`menuListItemClassName`) | selected | `🔽️Select` (793), `⌨️Command` (406), `⚡️ActionGroup` (23), `📋️MenuItem` (14) | (a) | Menu items are borderless controls. Make each item a box with `data-shared-border-edges=""`. Delete the border class. |
| P8 | `📋️menu-item-presentation/🟦️.ts:17` `focus:bg-hover-interactive-fill focus:text-emphasized` | focus | same | (b) | Fill and text only. Keep. |
| P9 | `🎨️chrome-class-names/🟦️.ts:34,39,97,99` `shadow-lg`, `border-t`, `border-l-accent`, `border-r-accent` | static chrome frame | chrome | review | Check whether these are conditional on state. If static, leave them. |
| P10 | `🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts:14,17,31,37` `border-b`, `border-t`, `shadow-sm` | static chrome | chrome | review | Same as P9. |

## 3. UI module: elements (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/`)

| # | File:line | Component | State | Class | Participant | Target | Note |
|---|---|---|---|---|---|---|---|
| E1 | `📑️Tabs/🟦️.tsx:212` (`contentId` trigger class) | Tabs trigger | focus-visible | `focus-visible:border-active-base focus-visible:outline-none focus-visible:ring-0` | yes (frame when selected) | (a) | Give the trigger `data-shared-border-edges=""` when unselected, and the full frame when selected. Delete `focus-visible:border-active-base`. Keep `outline-none` and `ring-0`. |
| E2 | `📑️Tabs/🟦️.tsx:212` | Tabs trigger | active | `data-[state=active]:…` via `interactiveTabActiveClass` | yes | (c) | See P4. |
| E3 | `🪜️Stepper/🟦️.tsx:257` | Stepper group | focus-within | `focus-within:border-accent` | yes (frame) | (c) | The group is not interactive. Focus goes to the innermost control, which is a box. |
| E4 | `🪜️Stepper/🟦️.tsx:271` | Stepper button cell | focus | `focus:outline-none focus-visible:bg-muted` | yes (cell) | (b) | Fill and removal only. The cell's compositor stroke is focus. |
| E5 | `🪜️Stepper/🟦️.tsx:321` | Stepper input | focus | `focus-visible:border-0 … outline…` (truncated) | yes (cell) | (c) | Removal. Confirm the full class string. |
| E6 | `🪜️Stepper/🟦️.tsx:352` | Stepper button cell | focus | `focus:outline-none focus-visible:bg-muted` | yes | (b) | Same as E4. |
| E7 | `🖱️ContextMenu/🔘️trigger/🟦️.tsx:33` | ContextMenu icon trigger | focus-visible | `focus-visible:outline focus-visible:outline-2` | no | (a) | Icon button. Box with `data-shared-border-edges=""`; the silhouette replaces the outline. |
| E8 | `📜️Scrollable/🟦️.tsx:36` | Scroller (tabindex region) | focus-visible | `focus-visible:ring-ring/50 focus-visible:ring-[length:var(--stroke-focus)] focus-visible:outline-1 transition-[color,box-shadow] outline-none` | no | (a) | Scroller is a focusable region. Box with edges "" or the owner's frame. Delete the ring and outline. |
| E9 | `↔️Resizable/🟦️.tsx:270` | Resize handle | focus-visible | `focus-visible:ring-ring focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none` | no | (a) | Handle is a box with stated silhouette on focus. Delete the ring. |
| E10 | `🎚️Slider/🟦️.tsx:40` | Slider thumb | focus-visible | `focus-visible:bg-active-base focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2` | no | (a), round | Thumb is round. Compositor silhouette must support a circle, or record an exception. The fill stays (b). |
| E11 | `🎚️Slider/🟦️.tsx:46` | Slider dial | invalid | `data-[invalid=true]:ring-1 data-[invalid=true]:ring-destructive` | no | (a) | Invalid state; same circle question as E10. |
| E12 | `🎚️Slider/🟦️.tsx:52` | Slider value text | focus-visible | `focus-visible:outline-2 focus-visible:outline-accent focus-visible:outline-offset-2 hover:text-emphasized` | no | (a) | Value is an editable control. Box edges "". Keep the text colour (b). |
| E13 | `💬️Dialog/🟦️.tsx:571` | DialogClose | focus | `focus:ring-ring focus:ring-2 focus:ring-offset-2 focus:outline-hidden ring-offset-background data-[state=open]:bg-accent` | yes (SharedBorderRow) | (a) for focus ring, (b) for open fill | Icon close box; the focus ring goes to the compositor silhouette. |
| E14 | `🥞️LayeredOverview/🟢️overview-button/🟦️.tsx:17` | Overview button (cell) | hover, focus-visible | `hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring outline-none` | yes (cell) | (c) for hover border, (a) for focus ring | Static `border-b border-l` stay as structure. |
| E15 | `🥞️LayeredOverview/💡️hints/🟦️.tsx:69` | Layered hint frame | focus-visible | `focus-visible:ring-2 focus-visible:ring-ring outline-none` | check | (a) | Hint is focusable. Box with stated silhouette. |
| E16 | `✏️Input/🟦️.tsx:299` | Input (stacked overflow) | invalid | `aria-invalid:border-destructive` | yes (frame) | (a) | Invalid recolours the overlay. Delete the class. |
| E17 | `✏️Input/🟦️.tsx:514` | Input element | invalid | `aria-invalid:ring-destructive/20 aria-invalid:border-destructive` | yes | (a) | Same as E16. The 20 % ring is a state line. |
| E18 | `⌨️Command/🟦️.tsx:405` | Command item | selected | `outline-hidden data-[disabled=true]…` plus `menuListItemClassName` | no | (a) | Item box with edges "". Keep `outline-hidden`. |
| E19 | `⌨️Command/🟦️.tsx:298` | Command input | focus | `outline-hidden` | field | (c) | Removal. Frame or compositor draws focus. |
| E20 | `🔽️Select/🟦️.tsx:681`, `🗨️Popover/🟦️.tsx:441` | Select and Popover content | focus | `outline-hidden` | no | (b) | Removal on a floating surface. Keep. |
| E21 | `🔽️Select/🟦️.tsx:791` | Select item | selected / focus | `menuListItemClassName` + `outline-hidden` | no | (a) | Same as P7. |
| E22 | `🕸️Diagram/⌨️keyboard/🟦️.tsx:102` `DIAGRAM_KEYBOARD_FOCUS_CLASS` `outline-solid outline-2 outline-offset-2 outline-accent` | Diagram node | keyboard focus | consumer `🧩️inner/🔶️force-config/🟦️.tsx:80` | no | (a) | Node box with stated silhouette in focus colour. Delete the outline. |
| E23 | `🌳️Tree/🎨️chrome/🟦️.tsx:117–124` `ringClass = loadingBorderStateClass(…) || waitingBorderStateClass(…)` | Tree row content | loading, waiting | via `border-loading` / `border-waiting` (C1, C2) | yes (cells) | (a) | Remove the ring class. Compositor draws loading and waiting on cells. |
| E24 | `🌳️Tree/🎨️chrome/🟦️.tsx:119` `interactiveActiveFillClass` (contains P1, P2) | Tree row selected | selected | fill + border | yes | (b) fill, (a) border | See P1. |
| E25 | `🌳️Tree/🟦️.tsx:249` ghost `border-primary … shadow-md` | drag ghost | drag preview | not an interactive state | no | (b) | Transient preview. Keep. |
| E26 | `📊️Table/🟦️.tsx:32` `tableRowSelectedClass = interactiveActiveFillClass` | Table row selected | selected | fill + border | yes (SharedBorderRow) | (b) fill, (a) border | See P1. |
| E27 | `⚡️ActionGroup/🟦️.tsx:176` | ActionGroup item | focus, selected | `outline-none` + `interactiveActiveFillClass` + `formControlFocusBorderClass` import | yes (cells) | (a) for border, (c) for `outline-none` | See P1, P6, P7. |
| E28 | `🎨️Canvas/🟦️.tsx:261` | Canvas app tab | active | `interactiveActiveFillClass` | check | (b) fill, (a) border | See P1. |
| E29 | `🎮️Engagement/🟦️.tsx:226` | Engagement option | active | `interactiveActiveFillClass` | yes (ToggleGroup) | (b) fill, (a) border | See P1. |
| E30 | `🎮️Engagement/🟦️.tsx:291` | Engagement session | active | `sessionActive && "bg-active-base shadow-sm"` | yes | (b) fill, (c) `shadow-sm` | State box-shadow. Delete the shadow. |
| E31 | `🔍️WindowSearch/🟦️.tsx:419` | Search field | active | `sessionActive && "bg-active-base shadow-sm"` | check | (b) fill, (c) `shadow-sm` | Same as E30. |
| E32 | `🔍️WindowSearch/🟦️.tsx:522` | Search result | active | `interactiveActiveFillClass` | check | (b) fill, (a) border | See P1. |
| E33 | `🧭️PanelTabBar/🟦️.tsx:22` | Panel tab | active | `interactiveActiveFillClass` | check | (b) fill, (a) border | See P1. |
| E34 | `🪟️Window/🟦️.tsx:20,395` | Window action group item | active | `interactiveActiveFillClass` | check | (b) fill, (a) border | See P1. |
| E35 | Every consumer of `interactiveOnClass` and `interactiveTabActiveClass` | on, active | see P3, P4 | mixed | (a) / (c) | Consistency pass after P3 and P4 land. |
| E37 | `🗂️WindowChrome/🟦️.tsx:63–89,339–343` `data-celebrated`, `data-introduced`, `border-loading`, `border-waiting` | window chrome | celebrated, introduced, loading, waiting | window silhouette | owner | (a) for the CSS ring classes, keep the state markers | Window chrome already passes the states to the silhouette. Remove the CSS ring classes (C1–C3, C5). |
| E38 | `🗂️WindowChrome/shared-border.ts:156–157` `"border-loading"`, `"border-waiting"` | compositor state feed | loading, waiting | owner | owner | keep | Compositor input. Not a CSS line. |
| E39 | `🎚️Slider/🟦️.tsx:515–516` `data-loading`, `data-waiting` | Slider | loading, waiting | no | (keep) | Attribute only; the CSS ring is C1/C2. |
| E40 | `📋️MenuItem` (import of `menuListItemClassName`) | MenuItem | selected | P7 | no | (a) | See P7. |
| E41 | `🪟️chrome`/`📣️display-components/🟦️.tsx:163`, `🚦️status-surface/🟦️.tsx:132` `hover:underline` | link-style text buttons | hover | `hover:underline` | no | decision | Text affordance, a line. Option (b) keep as typography, or (c) delete and use a fill. Needs a product decision. |

## 4. Product and app UI code (tsx and ts)

Scope: `🧰️framework/🛍️products/**`, `🎓️teaching/**`, `🌎️hub/**`, `♻️mit-bestand/**`, `✏️s/**`. Tests, stories and fixtures are excluded. `🌎️hub/🔨️modules/🛡️admin/🧱️elements` and `✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧱️elements` have no state lines.

Participants refer to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/` unless stated.

| # | File:line | Component | State | Class / rule | Participant | Target | Note |
|---|---|---|---|---|---|---|---|
| R1 | `🗣️Interpreter/🧱️node-tree/🟦️.tsx:451` | node-tree row | focus-visible | `hover:bg-muted/40 focus-visible:bg-muted/60 focus-visible:ring-1 focus-visible:ring-primary outline-none` | yes (`data-shared-border-frame edges=bottom`) | (a) for ring, (b) for bg | Row is a frame; ring goes to compositor. |
| R2 | `🗣️Interpreter/🖼️surface-views/🟦️.tsx:117` | surface view binding | focus-visible | `focus-visible:ring-primary focus-visible:ring-2 outline-none` | no | (a) | Box with edges "" when focused. |
| R3 | `🖥️Board2dHost/🟦️.tsx:66` | 2D canvas | focus | `outline-none focus:outline-none` | no | (a) | Focusable canvas removes the browser focus with no replacement. Box with stated silhouette. |
| R4 | `🧭️TiledMapHost/🟦️.tsx:30` | tiled map canvas | focus | `outline-none focus:outline-none` | no | (a) | Same as R3. |
| R5 | `🧩️BlockListHost/🟦️.tsx:71` | SelectionLabel button | focus-visible | `focus-visible:outline focus-visible:outline-2 focus-visible:outline-accent` | yes (cells) | (a) | Cell focus. Delete the outline; the cell's silhouette paints it. |
| R6 | `🏛️ShellHost/🗂️local-catalog/📂️files/🟦️.tsx:48` | local catalog file button | focus-visible | `focus-visible:outline` | no | (a) | Box with edges "". |
| R7 | `🕸️NodeGraph/🪄️catalogue-spotlight/🟦️.tsx:223` | spotlight input | focus-visible | `shadow-none focus-visible:ring-0` | no (inside frame) | (c) | Removal. Keep. |
| R8 | `🖌️Paint2dHost/✍️editing/🟦️.tsx:34` `cell` class | paint tool cell | focus-visible, disabled | `focus-visible:outline focus-visible:outline-2 focus-visible:outline-accent` | yes (16 `data-shared-border-cell`) | (c) | Cell already has compositor focus. Delete the outline. |
| R9 | `🖌️Paint2dHost/✍️editing/🟦️.tsx:384` | paint application region | focus-visible | `focus-visible:outline focus-visible:outline-accent` | yes (cells) | (a) | Region box with focus silhouette. |
| R10 | `🎛️control-port/container.tsx:31` | control-port section | selected | `selected && "ring-primary ring-1"` on `SharedBorderRow` | yes (SharedBorderRow) | (c) | Selected ring on a participant. The owner's state goes to the compositor. |
| R11 | `🎛️control-port/container.tsx:45–46` | control-port record | selected, hovered | `selected && "ring-primary ring-1"`, `hovered && "outline-primary/50 outline-1"` plus `activityBorderClass(record)` | yes (`SharedBorderRow`) | (a) | Selected and hovered are compositor states. Delete both classes. `activityBorderClass` (loading/waiting) goes via C1/C2. |
| R12 | `🎛️control-port/🟦️.tsx:15`, `🎛️declarative-controls/🟦️.tsx:47,92` | `ring` control kind | n/a | type name | n/a | exclude | Not a class. |
| R13 | `🖋️InkCanvasHost/🧱️ink-block-views/🟦️.tsx:91` | ink block | selected, hovered | `selected && "ring-primary ring-2"`, `hovered && !selected && "ring-primary/60 ring-1"` | no | (a) | Block box with `edges=""` and a compositor stroke. Keep `shadow-sm` static. |
| R14 | `🖋️InkCanvasHost/🧱️ink-block-views/🟦️.tsx:42,48` | ink block text | underline | static `underline` | no | exclude | Static. |
| R15 | `🖋️InkCanvasHost/🪟️ink-overlays/🟦️.tsx:60,63` | ink overlay link | hover/focus | `underline` | no | decision | See E41. |
| R16 | `🛠️ShellHelpers/⏪️time-travel/🟦️.tsx:421,514` | time-travel buttons | underline | `underline` | no | check | Likely static link style. Verify. |
| R17 | `🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx:67,251` | staged args action | underline | `self-start text-xs underline` | no | exclude | Static link style. |
| R18 | `🏛️ShellHost/🍞️transient-notice/🟦️.tsx:20`, `🔖️session-notice/🟦️.tsx:19`, `🚪️close-guard/🟦️.tsx:58` | notice action | underline | `underline` | no | exclude | Static. |
| R19 | `🏛️ShellHost/🟦️.tsx:412` | ShellHost chrome | static | `border-l border-t` | no | exclude | Static. |
| R20 | `🏛️ShellHost/🟦️.tsx:245` and `🏛️ShellHost/🧱️notice-bands/🟦️.tsx:7,21` | notice band | static | `shadow-sm` | no | exclude | Static. |
| R21 | `🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx:454` | route chrome | focus | `ring` (truncated) | check | review | Confirm whether `ring` is a state. |
| R22 | `🤖️AgentApprovals/🟦️.tsx:168` | approval section (`tabIndex=-1`) | focus-visible | `focus-visible:ring-2 focus-visible:ring-emphasized` | no | (a) | Programmatic focus target. Box with focus silhouette. |
| R23 | `🔗️HubConnection/🏛️workspace` and `🔐️HubSignIn`, `🛂️SpaceAdministration`, `🤖️AgentDelegations`, `🏘️SpaceBrowser`, `🧵️TaskManager` | Buttons `variant="outline"` | static | `variant="outline"` | yes/no | exclude | Static outlined button, not a state line. |
| R24 | `✏️TextEditor/🟦️.tsx:229,257` | editor shadow | static | `shadow-md` | no | exclude | Static. |
| R25 | `✏️TextEditor/🟦️.tsx:664,672,673` | editor buttons | static | `variant="outline"` | no | exclude | Same as R23. |
| R26 | `🧩️BlockListHost/🟦️.tsx:93,141,279` | block action | static | `variant="outline"` | yes (cells) | exclude | Same as R23. |
| R27 | `🌐️World3dHost/📏️modelling/🟦️.tsx:385`, `⏯️tool-run-trace/🟦️.tsx:460`, `🎨️mesh-render/…:108` | 3D outline | n/a | LineSegments / geometry | no | exclude | Not CSS. |
| R28 | `🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx:126,148,156`, `🌐️World3dHost/🟦️.tsx:197,199`, `📏️modelling:482` | shadows | static | `shadow-sm` | no | exclude | Static. |
| R29 | `🏘️SpaceBrowser/🟦️.tsx` and `📊️Table`, `🌐️World3dHost` `outline` tokens | variant buttons / geometry | n/a | `outline` | yes/no | exclude | Same as R23. |
| R30 | `🕸️NodeGraph/🔷️diagram-node/🟦️.tsx:92,93` | diagram node | static | `shadow-sm`, `border-b` | no | review | `border-b` may be a selected-state line. Verify. |
| R31 | `🔺️DiffViewHost/🟦️.tsx:127` | diff line | static | `border-l` | no | review | Verify. |
| R33 | `🏛️ShellHost/🟦️.tsx:282,377` | close, arrow buttons | static | `variant="outline"` | no | exclude | Same as R23. |
| R34 | `🌐️World3dHost/🎨️mesh-render/…` | highlight | n/a | geometry | no | exclude | Not CSS. |
| R35 | `🪟️chrome` (course) `🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx:96` `card` link | course card link | hover, focus-visible | `hover:underline focus-visible:underline` | no | decision | See E41. |
| R36 | `🎓️teaching/…/🪟️presentation/🌐️browser/🟦️.tsx:35` | window chrome title chip | focus-visible, active | `focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-active-base` plus `interactiveActiveFillClass` when active | yes (`WindowSilhouette`, `data-shared-border-state`) | (a) for ring and border, (b) for fill | Chip is a window tab. Compositor chip focus. |
| R37 | `🎓️teaching/…/🧪️playground/🖼️stage/🟦️.tsx:72` | playground stage item | selected | `selected && "outline-2 outline-dashed outline-offset-2 outline-[var(--accent)]"` | no | (a) | Selection outline. Box with selected silhouette. |
| R38 | `🎓️teaching/…/🧪️playground/🎞️timeline/🖼️panel/🟦️.tsx:109,113,130,153` | timeline | static | `border-t`, `border-l` | no | review | Verify whether conditional. |
| R39 | `🎓️teaching/…/🔬️detail/🎞️clips/🟦️.tsx:97,98,115` | clip row | static | `border-l`, `border-l-2` | no | review | `border-l-2` may be selected. Verify. |
| R40 | `🎓️teaching/…/🔬️detail/🧱️tables/🟦️.tsx:13,17,32` | table | static | `border-b` | no | exclude | Static. |
| R41 | `🎓️teaching/…/🔭️viewpoints/🎚️selector/🟦️.tsx:50` | selector | static | `decoration-dotted` | no | exclude | Static. |
| R42 | `🎓️teaching/…/🏠️overview/🟦️.tsx:97,101` | overview | static | `border-t`, `border-l` | no | review | Verify. |
| R43 | `🎓️teaching/…/🪟️chrome/🟦️.tsx:353` | link title | hover, focus-visible | `hover:underline focus-visible:underline` | no | decision | See E41. |
| R44 | `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:254,263` | reset control | hover | `border-color: var(--border-normal-color)` on `:hover` | no | (a) | Make the control a box. |
| R45 | `♻️mit-bestand/🧺️demonstrator/🟦️.tsx:213` | app name `h1` (`tabIndex=-1`) | focus | `focus:outline-none` | no | (b) | Programmatic focus target with no interactive state. Keep the removal, or decide the focus treatment. |

## 5. CSS outside the compositor

| # | File:line | Selector / rule | State | Participant | Target | Note |
|---|---|---|---|---|---|---|
| S1 | `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/⚖️footer-and-forced-colors/🎨️.css:10–13` | `.course-skip-link:focus { outline:none; border: hairline solid var(--active-base); text-decoration: underline … }` | focus | no | (a) | Focus is a border line plus underline. Make the skip link a box. |
| S2 | same file `:74` | `.course-app [aria-disabled="true"], .course-app :disabled { border-color: GrayText }` | disabled, forced colours | no | (a) | Map the disabled stroke in the compositor. |
| S3 | same file `:80–82` | `.course-app :focus-visible { outline: 2px solid Highlight; outline-offset:2px }` | focus, forced colours | global | (a) | Global browser-style ring on every focusable. Replace with the compositor stroke. |
| S4 | same file `:85–87` | `.course-skip-link:focus { border-color: Highlight }` | focus, forced colours | no | (a) | Same as S1. |
| S5 | `🧰️…/📚️course/🎨️styles/📊️plots-and-crowd/🎨️.css:219–220` | `.course-card-link:focus-visible { outline:none }` | focus | no | (a) | Removes focus with no replacement. Box with focus silhouette. |
| S6 | `…/🏁️results-and-preferences/🎨️.css:118–121` | `.course-input:focus-visible, .course-app select:focus-visible, .course-field > input.course-input:focus-visible + .course-input { border-color: var(--active-base); outline:none }` | focus | no | (a) | Field focus is a frame-edge stroke. |
| S7 | `…/🏁️results-and-preferences/🎨️.css:125–127` | `.course-cell:focus-visible, .course-cell:focus-visible + .course-cell { border-inline-start-color: var(--active-base) }` | focus | cells | (a) | Cell focus is a divider stroke. |
| S8 | `…/🏁️results-and-preferences/🎨️.css:130–131` | `.course-cell:focus-visible { outline:none }` | focus | cells | (c) | Removal. Keep only if S7 is compositor-driven. |
| S9 | `…/🖥️mockups/🎨️.css:6` | `.course-mockup-scroll:focus-visible { outline: 2px solid currentColor; outline-offset: 3px }` | focus | no | (a) | Scroller box. |
| S10 | `…/🖥️mockups/🎨️.css:16` | `.course-workbook td[data-selected] { box-shadow: inset 0 0 0 2px currentColor }` | selected | cell | (a) | Selected cell is a compositor state. |
| S11 | `…/🖥️mockups/🎨️.css:19` | `.course-workbook-sheets [data-active] { font-weight:700; border-bottom: 3px solid currentColor }` | active tab | tab | (a) | Active sheet tab underline indicator. Box with active silhouette. |
| S12 | `…/🖥️mockups/🎨️.css:33` | `.course-graph-relations summary:focus-visible { outline: 2px solid currentColor }` | focus | no | (a) | Summary box. |
| S13 | `…/🖥️mockups/🎨️.css:41` | `.course-workbook td[data-selected] { border: 2px solid Highlight }` | selected, forced colours | cell | (a) | Compositor stroke. |
| S14 | `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🎨️.css:3` | `.staffage-app :focus-visible { outline: 2px solid var(--accent); outline-offset: 2px }` | focus | global | (a) | Global fallback. Delete after each control is a box. |
| S15 | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css:124` | `.roster a[aria-current="page"] { outline: 2px solid var(--accent); outline-offset: 2px }` | current page | nav link | (a) | Active nav item. Box with current silhouette. Story-only, check whether shipped. |
| S16 | `🧰️framework/🛍️products/🎤️presentation/…/🔲️interaction-handles-and-controls/🎨️.css:254,263` | `.reveal .presentation-interaction-slide-reset-host:hover … { border-color }` | hover | no | (a) | See R44. |
| S17 | `♻️mit-bestand/…/📐️layout.css` (vendored Bootstrap export, 16 297 lines) | `.form-control:focus`, `.btn:focus-visible`, `.nav-tabs .nav-link.active`, `.accordion-button:focus`, `.page-link.active`, `.list-group-item.active`, and about 100 more state rules with `border-color`, `outline`, `box-shadow` | focus, hover, active, checked | no | exclude or decide | Vendored third-party CSS. Do not patch it. Leave to a decision, or replace the export. |
| S18 | `♻️mit-bestand/…/🏠️main.css:623` | `.cb-accordion__trigger:focus { outline: 2px solid #007bff; outline-offset: 2px }` | focus | no | (a) | First-party theme CSS. |
| S19 | `♻️mit-bestand/…/🏠️main.css:858` | `.tocWrapper ul li a:hover { border: 1px solid var(--zb-slate-400) }` | hover | no | (c) | Hover border line. |
| S20 | `♻️mit-bestand/…/🏠️main.css` (`.cb-cards__link--link:hover`, `--download:hover`) | `border: 1px solid …` on hover | hover | no | (c) | Same as S19. |
| S21 | `♻️mit-bestand/…/🎨️Basic.css` | `.deleteAllFiles:hover { text-decoration: underline }` | hover | no | decision | Text affordance, as E41. |
| S22 | `🧰️framework/🔨️modules/🖱️ui/🌐️globals/🎨️.css:172–176` | `html[data-ui-chrome-reveal="hover"] [data-ui-reveal-region]:not([data-ui-revealed="true"]):not(:focus-within) { border-width: 0 }` | hidden / revealed | reveal region | (a) | Visibility state drawn as absence of border. Make the region's silhouette empty while hidden. |

## 6. Browser default focus outline (no focus styling)

The baseline reports 113 hits for the default `1px auto` outline. The main sources in this inventory:

- Icon buttons and `Button size="icon"` without an outline class, from the compositor's view (inherit `outline: auto`). Confirm in the live probe; the list of stories in `live-baseline.md` names `Icons/IconAnimations` (30/30).
- Focusable `canvas` elements with `outline-none` and no replacement: R3, R4.
- Focusable `[tabindex]` regions and `role` containers without a focus class: R2, R9, E8.
- Course and staffage global `:focus-visible` rules (S3, S14) apply only where no control class wins.

Action: after each (a) item is a box, the global `:focus-visible` fallbacks (S3, S14, C19, `🎨️.css` 7518) must be removed so the browser default is never painted.

## 7. Summary

- Row counts: compositor 22 (C1–C22), presentation 10 (P1–P10), UI elements 40 (E1–E41, E36 removed), product tsx 44 (R1–R45, R32 and R46 removed), CSS outside compositor 22 (S1–S22). Total 138.
- Compositor (C): 12 rows are (a), 7 are (c), 2 are (b), 1 needs review (C9).
- Shared classes with the widest reach: `interactiveActiveBorderClass` (P1, P2) feeds every active or selected fill, and `formControlFocusBorderClass` (P6) feeds Input, Textarea, Select and ActionGroup. Splitting fill from border in these two presets resolves most UI-module items at once.
- Global fallbacks to delete after migration: C19, S3, S14, and the `:focus-visible` rule at 🎨️.css 7929 (C19 is the same as the high-contrast form).
- Decisions needed: text underline on hover (E41, R15, R35, R43, S21); round thumb and dial silhouettes (E10, E11); whether the vendored Bootstrap export S17 is patched or replaced.
- Verification gaps: rows marked "check" or "review" (P5, P9, P10, E28, E31–E34, R16, R21, R30, R31, R38, R39, R42) need the full class string read before editing.
- Excluded from counts: static `variant="outline"` buttons, SVG and 3D outlines, `ring` type names, icon animations, test and story files.
