# Audit: How Every UI State Is Drawn Today (Shared-Border Silhouettes)

Read-only audit by static reading. Nothing was run in a browser or in the native targets, and no source file was edited.

## Legend and roots

Module root: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui` (written `UI/` below).

- `CSS` = `UI/🎨️styling/🖌️ui/🎨️.css` (8047 lines)
- `WC` = `UI/🧱️elements/🗂️WindowChrome/`
- `WS` = `UI/🧱️elements/🔲️WindowSilhouette/`
- `BP` = `UI/🔨️modules/📏️border-presentation/`
- `IP` = `UI/🔨️modules/🖱️interaction-presentation/🟦️.ts`
- `CCP` = `UI/🔨️modules/🎛️chrome-control-presentation/🟦️.ts`
- `FCP` = `UI/🔨️modules/📝️form-control-presentation/🟦️.ts`
- `MIP` = `UI/🔨️modules/📋️menu-item-presentation/🟦️.ts`
- `CCN` = `UI/🔨️modules/🎨️chrome-class-names/🟦️.ts`
- `EL` = `UI/🧱️elements/`

Line numbers refer to the files as they stand in the shared worktree at audit time.

---

## 1. CSS state rules (`CSS`)

### 1.1 Layer model that decides everything

- `CSS:2` declares `@layer shared-border-ownership, shared-border-priority;` before the Tailwind import, so ownership is the earlier layer. For `!important`, the earlier layer wins. So an `!important` in `shared-border-ownership` beats an `!important` in `shared-border-priority`, which beats Tailwind utilities.
- `CSS:1462-1466` (ownership): `.window-silhouette-border { stroke-width: hairline }`.
- `CSS:6863-6927` (priority, the state layer): focus, active, loading, waiting, introduced, celebrated, hover, forced-colours.
- `CSS:6930-6943` (ownership, the suppression layer): transparent native border colour on every side the overlay draws.
- Unlayered: `CSS:6738-6742` (native width zero for undrawn sides), `CSS:6945-6947` (segment `stroke` per state).

Consequence: for a frame whose sides are drawn by the overlay, every native border colour is transparent `!important` from the earlier layer. All `!important` border-colour state rules on frames (`CSS:6876-6878`, `6889-6892`, `6911-6915`) therefore do nothing visible on attached sides. The visible recolour is always the overlay segment stroke (`CSS:6879`, `6946-6947`) chosen by the compositor.

### 1.2 Hover

| Selector / rule | Lines | Property | Target |
|---|---|---|---|
| `[data-slot="navbar"]:hover::after`, `[data-slot="footer"]:hover::before` | `CSS:6630-6633` | `background-color` of the 1px pseudo-edge | navbar/footer band edge (suppressed while attached, `CSS:6936-6937`) |
| `[data-slot="navbar"] [data-level]:not([data-state="on"])…:hover`, and item variants | `CSS:6705-6714` | `background-color`, `color`, `--hover-ink`, `--muted-foreground` | shell-band controls; fill and text only, no line |
| `…[data-state="on"]:hover` | `CSS:6690-6697` | `background-color: active-hover` | selected shell-band controls |
| `[data-slot="tree-item-control"] [data-shared-border-frame/cell]` (stretch only) | `CSS:6636-6638` | layout only | tree value column |
| `.shared-border-cell:hover:not(…)` and `.shared-border-cell:has(+ cell:hover)` | `CSS:6872-6875` | `border-color: border-emphasized !important` on the cell's own inline-end divider | the cell's own seam, and the seam before a hovered next cell |
| `.shared-border-frame:hover:not(…)` / `[data-shared-border-frame]:hover` | `CSS:6876-6878` | `border-color` (no visible effect on attached sides, see 1.1) | frame |
| `[data-shared-border-segment][data-shared-border-state="hover"]` | `CSS:6879` | `stroke: border-emphasized !important` | overlay segment |
| `[data-slot="mode-dock-tab"]:is(:hover, :has(+ …:hover))` | `CSS:6907-6909` | `border-inline-end-color` | mode dock tab seam |
| `[data-window-silhouette]:hover > [data-window-silhouette-border][data-kind="normal"]:not([data-pending]) path` | `CSS:7521-7523` | `stroke: border-emphasized` (not important) | every path of a normal window silhouette |
| `[data-slot="introduction-info-box"]:not([data-active="true"]):hover [data-window-silhouette-border] path` | `CSS:7710-7712` | `stroke` | introduction box silhouette |
| `[data-slot="introduction-info-box"][:hover], [data-slot="dialog-content"]:hover` | `CSS:7701-7702` | `border-color: transparent !important` | own border already transparent |
| `[data-hover-scope]:hover [data-slot="drag-handle"]` and tab focus/close | `CSS:7743-7745` | `color` | grip and icons |
| `[data-slot="introduction-info-box"]:is(:has(…:hover))[…chip]` | `CSS:7716-7735` | `color` | title chip and grip |
| Icon animation on hover | `CSS:1895-1914`, `3557-3660`, `6576-6583` | `animation` | icons only |
| Scrollbar and scroll-area hover | `CSS:257-324` | scrollbar | not borders |
| React Flow handle and resizable corner hover | `CSS:564`, `601` | `cursor` | not borders |
| Prose link hover | `CSS:838` | colour | not borders |

### 1.3 Active, pressed, selected and on

| Selector | Lines | Property | Target |
|---|---|---|---|
| `[data-state="on"]`, `[data-slot="…-group-item"]`, `panel-tab-button` (`:not(:disabled)`) | `CSS:6676-6684` | `background-color: active-base`, `border-color: active-base !important`, text | shell-band toggle items (not shared cells here) |
| `.shared-border-cell:where([data-active="true"], [data-state="on"], [data-state="active"], [aria-selected="true"], [aria-pressed="true"], :has(> active child), :has(+ active cell), :has(+ cell > active child))` | `CSS:6866` | `border-color: active-base !important` | cell's own divider, or the seam before an active neighbour |
| `.shared-border-cell:where(.border-waiting …)`, `.border-loading`, `[data-introduced]`, `[data-celebrated]` | `CSS:6867-6870` | `border-color` (waiting/loading/introduced/celebrated colour) | cell divider |
| `[data-shared-border-segment]:is([data-shared-border-state="active"], [data-shared-border-state="focus"])` | `CSS:6947` | `stroke: active-base` | overlay segment |
| `[data-shared-border-attached~="top"…]:is(.border-loading, .border-waiting, [data-celebrated])::after` | `CSS:6938-6940` | `content: none` | suppresses a pseudo-edge |
| `[data-state="open"]` on select trigger in window measures | `CSS:7984-7986` | `border-color: var(--accent)`, `outline: none` | specific to window measures, dead for attached frames |
| `[data-state="on"]` toggle items in window measures | `CSS:7993-7995` | `box-shadow: none` | specific |
| `[data-slot="mode-dock-tab"][data-active="true"] :is(focus, close)` | `CSS:7752-7754` | `color: border-emphasized` | icons |
| Tab trigger `data-state=active` fill | `TSX: EL/📑️Tabs/🟦️.tsx:213` via `IP:52-59` | `background-color: active-base` | tab |
| `:active` | `CSS:281-282` (scrollbar thumb only) | scrollbar | no bordered element has an `:active` border or fill rule in CSS |

There is no CSS `:active` rule for any bordered control. Pressed is expressed only through `data-state="on"`, `data-active`, `aria-pressed`, or the element's own `active:cursor-grabbing` utilities (`EL/🖼️Panel/🟦️.tsx:136`, `EL/🧱️DragHandle/🟦️.tsx:48`, `EL/📊️Table/🟦️.tsx:195`).

### 1.4 Focus

| Rule | Lines | Property | Target |
|---|---|---|---|
| `.shared-border-cell/frame:focus-visible` (and `[data-shared-border-cells] > frame`) | `CSS:6884-6887` | `outline: none !important; box-shadow: none !important` | kills native ring on cells and frames |
| `:has(> input/textarea/select-trigger:focus-visible)` on frame/cell | `CSS:6885-6886` | same as above | kills ring on frame when inner control has keyboard focus |
| `.shared-border-frame:focus-visible` and `:has(> …:focus-visible)` | `CSS:6889-6892` | `border-color: active-base !important` (dead for attached sides, see 1.1) | frame |
| `.shared-border-cell:where(:focus-within, :has(+ cell:focus-within))` | `CSS:6865` | `border-color: active-base !important` (priority layer) | cell's own divider, or the seam before a focused cell. Note: `:focus-within`, not `:focus-visible` |
| `[data-overview-card]:is(button):focus-visible [data-window-silhouette-border] path` | `CSS:6881` | `stroke: active-base !important` | every path of the card's own silhouette only |
| `[data-window-silhouette-chip] :is(button, [role=button], [tabindex], a[href], input, select, textarea):focus-visible` | `CSS:7609-7612` | `outline: none; box-shadow: none` | chip controls; focus is painted by the silhouette (`WC/chip-focus.ts`) |
| `:is([data-state="active"], [data-state="on"], [aria-selected], [aria-pressed], [data-active]):not(…):focus-visible` and `> :is(button, …):focus-visible` | `CSS:6899-6904` | `text-decoration: underline 2px` | label of an accented control, because its seam cannot show focus |
| `[data-shared-border-segment][data-shared-border-state="focus"][data-focus-on-accent]` | `CSS:6906` (and forced colours `6923`) | `stroke-width: 2×hairline` | focus path of a chip in an active window |
| `[role="tabpanel"]:focus-visible` | `CSS:6894-6897` | `outline: 1px solid active-base !important; outline-offset: -1px` | tabpanel edge |
| `[data-window-silhouette-chip] > …` and other focus rules in `FCP` | `FCP:14` | `focus-visible:border-accent`, `focus-visible:ring-0`, `outline-none`, `shadow-none` | native controls in `CCP:24` base class |
| `[data-window-measures…select-trigger]:focus-visible` | `CSS:7987-7989` | `border-color: accent; outline: none` | dead for attached frames |
| `.semio-scope[data-contrast="more"] :focus-visible` | `CSS:8027-8030` | `outline: 3px solid currentColor; outline-offset: 2px` | high contrast. Not `!important`, so it does not hit the shared-border exclusions in `CSS:6884` and draws outside the element on every non-shared focus target |

### 1.5 Drag and drop

There are no `:drag*` or `data-dragging` CSS selectors on borders. Drag and drop is expressed in TSX classes:

- Slider: `data-[dragging=true]:bg-active-base` (`EL/🎚️Slider/🟦️.tsx:41`). A fill only.
- Panel drop zone: `dropZoneReadyFillClass` (`CCN:24`, `CCN:27`) as a background fill, plus `data-active` (`EL/🖼️Panel/🟦️.tsx:137`, `310-311`). A fill only.
- Panel tab drop target: `isChildDropTarget && "bg-accent/20 !border-accent"` (`EL/🧭️PanelTabBar/🟦️.tsx:470`). The `!border-accent` is an `!important` utility on a `shared-border-cell`. The `shared-border-priority` `!important` rules (`CSS:6865-6870`) are in an earlier layer than utilities, so for `!important` they win. The drop-target border is expected to lose on the seam.
- Table row over-state: `isOver && !isSelected && "bg-hover-interactive-fill text-emphasized !border-b-[color:var(--active-base)]"` (`EL/📊️Table/🟦️.tsx:260`). Table rows are not shared cells. They use `borderNormalBottomClass` (`border-b !border-normal`).
- Tree row: no drag state. Selected is `bg-accent text-accent-foreground` (`EL/🌳️Tree/🟦️.tsx:584`).
- Overlay `data-shared-border-state` is also set explicitly for the loading/waiting ActionGroup (`EL/⚡️ActionGroup/🟦️.tsx:223`) and for Canvas dock insert preview as `active` (`EL/🎨️Canvas/⚓️dock/🟦️.tsx:104`).

### 1.6 Aria and data hooks in CSS (complete list)

`aria-selected`, `aria-pressed`, `data-active`, `data-state=on|active|open|closed`, `data-highlighted` (none found in CSS), `aria-expanded` (none found in CSS). Lines: `CSS:6669-6714`, `6866`, `6899-6900`, `6912-6913`, `7710`, `7752`, `7986-7993`. `data-highlighted`, `aria-expanded` and `data-drop*` appear nowhere in `CSS`.

---

## 2. Compositor and silhouette TypeScript

### 2.1 Which state an outline gets (`WC/shared-border.ts`)

- `WC/shared-border.ts:34`: state order `normal, focus, active, waiting, loading, introduced, celebrated, hover`.
- `WC/shared-border.ts:74-82` `sharedBorderStroke`: hover `--hover-base`, focus and active `--active-base`, introduced, loading, waiting use their tokens, normal `--border-normal-color`. These attributes are overridden by CSS at `CSS:6879` and `6946-6947` for `[data-shared-border-segment]`. So `--hover-base` in the attribute is dead in the browser; hover is `--border-emphasized-color` via CSS.
- `WC/shared-border.ts:224-236` `state(element, fallback)`. Candidates are combined and the highest state wins:
  - `:225` if the pointer is inside the element (and the element is not `:disabled` nor under `aria-disabled`), the result is **hover, immediately**. Hover wins over everything, including active and focus.
  - `:226-228` explicit `data-shared-border-state` attribute, validated against the list.
  - `:229-230` `data-celebrated`, `data-introduced`.
  - `:231-232` class `border-loading`, `border-waiting`.
  - `:233` `data-active="true"`, `data-state="on|active"`, `aria-selected="true"`, `aria-pressed="true"` **on the element itself only**. Descendant state does not promote the outline. CSS `:has()` does that for cells, not for frames.
  - `:234` `element.contains(document.activeElement)` gives **focus**. This is `:focus-within` semantics. It does not check `:focus-visible`, so a pointer click into a frame's control also paints the frame in focus colour until blur.
- `WC/shared-border.ts:166-178` pointer tracking: `pointerover` and `pointerout` on the root set `hovered` and mark every element that contains the pointer target. For a frame, **any descendant hover** makes the whole frame hover. Hovering one button of a ButtonGroup recolours all four sides of the group's outline.
- `WC/shared-border.ts:180-184` and `185-191`: transition start and end are tracked (transform, translate, scale, width, height, top, left, right, bottom, flex-basis, grid-template) and trigger re-measurement.
- `WC/shared-border.ts:193-203`: `schedule`, `scheduleGeometry`, `scheduleState` (state changes repaint from cached geometry).
- `WC/shared-border.ts:237-265` `clipOf`, `visibleSources`: every overflow-clipping ancestor (hidden, clip, scroll, auto) trims the segments to its clip rectangle. A trimmed outline is open at the clip edge.

### 2.2 Segment ownership and resolution

- `WS/shared-border.ts:144-158` `sharedOutlineSegments`: each rectilinear logical edge of an outline becomes one `SharedOutlineSegment` with owner `${owner}:${index}`. Endpoints are snapped and carry `paintFrom`, `paintTo`, `transform`. So a closed rectangle is four owned segments; a window silhouette is one segment per silhouette edge.
- `WC/shared-border.ts:325-327` frames: `edges = sharedOutlineSegments(frame.id, logical, paint, this.state(element, "normal"), …)`. The frame's `data-shared-border-edges` decides which sides are drawn (`:326`, through `flushWithUndrawnParentEdge`).
- `WC/shared-border.ts:330` `visibleSharedOutlineSegments` removes clipped candidates at the canonical centreline.
- `WC/shared-border.ts:333` `resolveBorderSegments(sources)`: for collinear coincident spans, the winner is the highest state priority (`BP/🟦️.ts:34-36`, `74`), ties broken by owner text and then id. So a seam shared by a hovered owner and a normal owner is drawn hover, and the other owner's segment is dropped on that span.
- `WC/shared-border.ts:340-368`: contributors are merged onto one centreline (`lineOf`), so a shared seam is one line. Each owner's output is its winning spans, split at junctions (`WS/shared-border.ts:78-102` junction resolver).
- `WC/shared-border.ts:369-378`: a window gets its complete `borderPath` in its own state only when all its owned spans are won by it, all share one state, are unclipped and not `interacting`. Otherwise it gets its per-span pieces, which may be in several states.
- `WC/shared-border.ts:379-419`: frame overlay SVG. Each output segment becomes a `path` with `stroke` attribute, class `window-silhouette-border window-silhouette-border-${state}`, and `data-shared-border-state`. `celebrated` uses a mask and a conic `foreignObject` (`:401-415`).
- `WC/shared-border.ts:418`: the frame's `data-shared-border-attached` is set to its drawn sides. That attribute is what suppresses native borders in `CSS:6931-6935`.
- `WC/shared-border.ts:487-509` `paintSharedWindowBorder` (windows): the main path takes the outline of segments in the window's own kind, extra paths take every other state. So a window's own silhouette is one `path` plus an `extra` layer.

### 2.3 Can a cell's four edges be one closed loop?

No. A `data-shared-border-cell` inside a frame has no overlay of its own. Its divider is a native `border-inline-end`, painted by CSS (`CSS:6804-6827`, `6866-6875`). Its top, bottom and outer left/right edges belong to the owner frame's overlay. So a cell's visible outline is split between a CSS border owned by the cell and overlay segments owned by the frame. The overlay state comes from the frame's pointer and focus containment, not from the cell. Only frames with `data-shared-border-frame` and no `cell` ancestor can be recoloured as one closed loop. The `SharedBorderRow` cells (`data-shared-border-cells`) follow the same split.

### 2.4 Window silhouettes (`EL/🗂️WindowChrome/🟦️.tsx`)

- `WC/🟦️.tsx:35` kinds: `celebrated, introduced, loading, waiting, active, normal`.
- `WC/🟦️.tsx:61-78` `resolveWindowSilhouetteBorderKind`: no hover, no focus. Kind comes from celebrated, introduced, loading/waiting class, else `active` when `stackActive`, else `normal`.
- `WC/🟦️.tsx:81-95` paint per kind: class and stroke token.
- `WC/🟦️.tsx:314-436` `WindowChromeSilhouetteBorder`:
  - `:373-377` segments painted through the compositor (`useSharedWindowBorder`, `:375`).
  - `:385` `focusPaint`: a separate `path` in focus colour (`sharedBorderStroke("focus")`) from `focusedWindowChipPath`, drawn last in the SVG. It sits on top of the silhouette line, it does not replace it.
  - `:391-409` and `:418-428`: main `path` with `data-shared-border-segment` and `data-shared-border-state={kind}`.
- `WC/chip-focus.ts:37-52` `focusedWindowChipPath`: only when `:focus-visible` (`:39`). It builds a closed rectangle on the chip's inset centreline, snapping each coordinate to an existing silhouette line within 2 px (`:13`, `28-32`). When the snap misses, the rectangle sits beside the silhouette line: a second line.
- `WC/chip-focus.ts:17` excludes focus paths from the snap lines.
- `CSS:1495-1501`: `.window-silhouette-border-active` and `-normal` class strokes. These are class rules and beat presentation attributes, so `CSS:7521-7523` (hover) and class strokes override attribute strokes.
- Conflict: `CSS:7521-7523` (`stroke` on every `path` of a normal silhouette while hovered, not important) overrides the `focusPaint` attribute stroke. A keyboard-focused chip in a hovered normal window loses its focus colour while the pointer is over the window. This is a code-reading finding and should be confirmed in a browser.

### 2.5 Card stack and overview card

- `EL/🃏️OverviewCard/🟦️.tsx:71-…` renders `WindowChrome` **inside** the `button[data-overview-card]` (`:117` `outline-none`, `:121` `group-hover:-translate-y-0.5` lift).
- Because the silhouette stack is a descendant of the focused button, the compositor's `state()` containment test (`:234`) never sees the card's focus. Focus on the card is therefore CSS-only: `CSS:6881` recolours every path of the card's own silhouette, and does not take part in ownership resolution. A seam owned by a neighbouring card keeps its normal colour.
- Hover on the card is different: `CSS:7521-7523` plus the compositor's hover state on the card's own segments. Hover does take part in resolution (hover beats normal on a coincident span), so the seam follows the hovered card.

---

## 3. Per element (`EL/*/🟦️.tsx`)

Element TSX mostly delegates to shared class maps. Those maps are listed with their own lines.

| Element | State mechanism in its code | Shared-border cell or frame? |
|---|---|---|
| Button (`EL/🔘️Button/🟦️.tsx`) | No own state class. Wraps `ButtonGroup` and `ButtonGroupItem` (`:12`). | Via ButtonGroup: group frame, item cell |
| ButtonGroup (`EL/🔳️ButtonGroup/🟦️.tsx`) | Group frame `data-shared-border-frame` and `edges="top right bottom left"` (`:71-72`). Item variants `buttonGroupItemVariants` (`:26-31`) are `chromeControlItemClass` (`CCP:29`): base (`CCP:20-26`, which includes `formControlFocusBorderClass`) plus `interactiveHoverClass` (`IP:33`: `hover:bg-hover-interactive-fill hover:text-emphasized`). Item: `data-shared-border-cell` (`:128`), `focus:z-panel focus-visible:z-panel` (`:137`). | Frame plus cells |
| Toggle (`EL/🔀️Toggle/🟦️.tsx`) | Item `data-shared-border-cell` (`:184`), group frame (`:209`). Dropdown items `hover:bg-hover-interactive-fill outline-none focus-visible:bg-hover-interactive-fill` (`:334`). Open and closed are motion only (`:323`). | Cell and frame |
| ToggleGroup (`EL/🎛️ToggleGroup/🟦️.tsx`) | Frame `:171`, item shell cell and group `:208`, item cell `:227`, action cells `:258`, `focus:z-panel` (`:231`). Selection comes from `interactiveOnClass` (`IP:42-49`) through `CCP:29`. | Frame, nested cells, `data-shared-border-cells` on the action |
| ActionGroup (`EL/⚡️ActionGroup/🟦️.tsx`) | Item `chromeControlGroupSiblingDividerClass` (`:107`, `CCP:12` = `shared-border-cell`), `focus:z-panel focus-visible:z-panel`. Frame state `data-shared-border-state` = loading or waiting (`:223`). Option rows `outline-none` plus `interactiveActiveFillClass` when selected (`:176`). | Frame and cells |
| Tabs (`EL/📑️Tabs/🟦️.tsx`) | Trigger is a frame **only when selected** (`:205-206`) with `data-shared-border-state="active"` (`:207`). Trigger class: `focus-visible:border-active-base focus-visible:outline-none focus-visible:ring-0` (`:212`), `interactiveTabActiveClass` (`:213`, `IP:52-59`), `interactiveHoverClass` (`:214`). Tabs list is a group (`:154-155`). | Frame only when selected. Unselected tabs have no outline |
| PanelTabBar (`EL/🧭️PanelTabBar/🟦️.tsx`) | Tab button `data-shared-border-cell` (`:430`), `aria-pressed={isActive}` (`:440`), `data-active` (`:438`), `data-drop-nest` (`:435`). Class: `panelTabButtonDividerClass` (`CCN:59`), `panelTabButtonClass` (`CCN:63-71`: `outline-none` plus hover fills), `isChildDropTarget && "bg-accent/20 !border-accent"` (`:470`), `isUnitDropReady && dropZoneReadyClass` (`:471`). Tab frame `data-shared-border-frame` on the strip edge (`:563`), group `:564`. | Cells, group and edge frame |
| Select (`EL/🔽️Select/🟦️.tsx`) | Trigger frame (`:422-423`), content frame (`:671-672`). Content `data-[state=open|closed]` animation (`:681`), item `focus:text-emphasized outline-hidden` and `data-[disabled]` (`:791`), item hover fill plus text (`:848`, `871`). | Frame |
| Input (`EL/✏️Input/🟦️.tsx`) | Root `data-shared-border-frame` (`:491`). Native `<input>` gets `formControlFocusBorderClass` (`:370`: `focus-visible:border-accent`, `aria-invalid:border-destructive`) and `aria-invalid:ring-destructive/20` (`:514`). A box-shadow ring on the inner input is not suppressed by `CSS:6748` (which only zeroes border width and radius). | Frame |
| Textarea (`EL/🔤️Textarea/🟦️.tsx`) | Root frame (`:113`). No own state class. | Frame |
| Checkbox (`EL/☑️Checkbox/🟦️.tsx`) | No state class in TSX. Native painter is `paint_checkbox` (`🖌️paint`). | Not a shared border |
| MenuItem (`EL/📋️MenuItem/🟦️.tsx`) | `outline-none` (`:14`) plus `menuListItemClassName` (`MIP:14-20`): hover fill and text (`interactiveHoverClass`), `focus:bg-hover-interactive-fill focus:text-emphasized` (**`:focus`, not focus-visible**, so a click also shows it), `data-[active=true]` fill, `data-[selected=true]:bg-active-base data-[selected=true]:border-active-base`. Border colour has no visible effect (no border width). | Not a shared border |
| List (`EL/📃️List/`) | No TSX component file. Targets only. | Not a shared border |
| Tree (`EL/🌳️Tree/🟦️.tsx`) | Row class `baseClasses` with `hover:bg-hover-interactive-fill` (`:583`). Selected row: `bg-accent text-accent-foreground` (`:584`). Row is `data-shared-border-cell` (`:605`). Tree files frame (`:641`). | Cells (rows) and frame |
| Table (`EL/📊️Table/🟦️.tsx`) | Row class: `borderNormalBottomClass` (`border-b !border-normal`), `tableRowInteractiveClass` (`:29`, hover fill), `tableRowSelectedClass = interactiveActiveFillClass` (`:32`), drop state (`:260`). Rows are **not** shared cells. Selected: `border-active-base` in the fill loses to `!border-normal` (important) so the bottom line stays normal. | Not shared |
| HistoryTable (`EL/🕰️HistoryTable/🟦️.tsx`) | `ring-2 ring-background` on avatars (`:135`). Not a state. | Not shared |
| CardStack (`EL/🃏️CardStack/🟦️.tsx`) | No state class. Interlocking cards: compositor paints each coincident line once (`:121`). | Via cards |
| OverviewCard (`EL/🃏️OverviewCard/🟦️.tsx`) | Button root `outline-none` (`:117`), `data-hover-scope`, `group-hover:-translate-y-0.5` lift (`:121`). Footer chips `group-hover:text-foreground`, `group-hover:translate-x-0.5` (`:136-138`). Flush content is a vertical shared-border group (`:92`: `data-shared-border-group`, `data-shared-border-cells`). | Flush: group with cells |
| Window (`EL/🪟️Window/🟦️.tsx`) | Group and cells (`:312`), edges (`:393`). | Frame and cells |
| WindowPane (`EL/📌️WindowPane/🟦️.tsx`) | No state class. `active:` matches only prop names (`:84`, `144`). | Via WindowChrome |
| Panel (`EL/🖼️Panel/🟦️.tsx`) | `active:cursor-grabbing` (`:136`). Empty drop zone is a `data-shared-border-cell` with `data-active` and fill (`:309-311`). | Cell |
| Navbar (`EL/🔝️Navbar/🟦️.tsx`) | Frame `edges="bottom"` (`:229`). Pseudo-edge hover (`CSS:6630`) is suppressed while attached (`CSS:6936-6937`). Band controls use `CSS:6669-6714`. | Frame |
| Footer (`EL/🔚️Footer/🟦️.tsx`) | Frame (`:39`). Pseudo-edge as Navbar. | Frame |
| ContextMenu (`EL/🖱️ContextMenu/🟦️.tsx`) | No state class. Items use `MIP`. | Not shared |
| Popover (`EL/🗨️Popover/🟦️.tsx`) | Content frame and cells (`:437-438`), `outline-hidden` (`:441`), open and closed animation (`:441`). | Frame and cells |
| Slider (`EL/🎚️Slider/🟦️.tsx`) | Thumb `hover:bg-emphasized group-hover:bg-emphasized` (`:39`), `focus-visible:bg-active-base focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2` (`:40`), `data-[dragging=true]:bg-active-base` (`:41`). Value readout `focus-visible:outline-2 focus-visible:outline-accent focus-visible:outline-offset-2` (`:52`). Invalid dial `ring-1 ring-destructive` (`:46`). | Not shared (ring around thumb) |
| Stepper (`EL/🪜️Stepper/🟦️.tsx`) | Group frame with `focus-within:border-accent` (`:257`), edges (`:254`). Minus and plus cells `data-shared-border-cell` (`:262`, `275`), `hover:bg-muted focus:outline-none focus-visible:bg-muted` (`:271`, `352`). Input `outline-none focus-visible:border-0` (`:321`). Focus-within border on a frame is dead for attached sides. | Frame and cells |
| Ribbon (`EL/🎀️Ribbon/🟦️.tsx`) | Group only (`:23`, `35`, `83`, `91`, `93`). No state class. | Group |
| DragHandle (`EL/🧱️DragHandle/🟦️.tsx`) | `hover:text-emphasized active:cursor-grabbing` (`:48`), `hover:text-emphasized` (`:24`). Parent hover via CSS (`CSS:7743-7748`). | Not shared |
| Diagram (`EL/🕸️Diagram/🟦️.tsx`) | No state class in TSX. React Flow CSS only (`CSS:551`, `564`, `601`), cursor only. | Not shared |
| Ports (`EL/🔌️Ports/🟦️.tsx`) | No state class. Re-exports. | Not shared |
| SharedBorderRow (`EL/🪢️SharedBorderRow/🟦️.tsx`) | Owner attributes (`:113-117`): `data-shared-border-cells`, `data-shared-border-wrap`, `data-shared-border-frame` (if framed), `data-shared-border-group` (if not framed), `data-shared-border-edges`. Cells `data-shared-border-cell` (`:138`, `150`, `168`). No state class; state is compositor and CSS. | Owner and cells |

Other elements with explicit `data-shared-border-state` or state attributes (not in the list above): Skeletons row cell `loading` (`EL/🦴️Skeletons/🟦️.tsx:28`), Engagement (`EL/🎮️Engagement/🟦️.tsx:290`), WindowSearch (`EL/🔍️WindowSearch/🟦️.tsx:416`), Canvas dock insert preview (`EL/🎨️Canvas/⚓️dock/🟦️.tsx:104`).

Element-level class maps that matter:

- `CCP:20-26` `chromeControlItemBaseClass` includes `formControlFocusBorderClass`.
- `CCP:46` `chromeControlGroupShellClass`: `shared-border-frame box-border flex items-stretch overflow-visible border w-fit shrink-0`, `borderNormalClass` (`!border-normal`), `glassClass`. The native `border` is kept for layout and made transparent by `CSS:6931`.
- `CCP:55-62` `chromeControlTabActiveClass`: `data-[active=true]:…`.
- `FCP:14` `formControlFocusBorderClass`: `outline-none`, `focus-visible:border-accent`, `data-[state=open]:border-accent`, `aria-invalid:border-destructive`, `focus-visible:ring-0`, `shadow-none`.
- `IP:15-18`, `24`, `30`, `33`: hover fill and text, `hover:not-data-[handle-hovered=true]:…` variants.
- `IP:39` `interactiveActiveFillClass`: `bg-active-base`, `border-active-base`, `text-active-foreground`, active-hover variants.
- `IP:42-49` `interactiveOnClass`: `data-[state=on]:…`.
- `IP:52-59` `interactiveTabActiveClass`: `data-[state=active]:…`.
- `CCN:59` `panelTabButtonDividerClass = "shared-border-cell"`.
- `CCN:113-114` `panelWindowInactiveTabClass`: transparent; the chip-cap glass is the boundary.
- `CCN:122-123` `modeDockActiveTabClass`: `border-0` so the silhouette owns the outline.
- `MIP:14-20` `menuListItemClassName`: described above.
- `BP/cases.json`, `BP/🟦️.ts:9-15`: `borderNormalClass = "!border-normal"`, `borderElementClass`.
- `🌀️status-border`: `border-waiting`, `border-loading` (and `-active`, `-element`) are class names read by `WC/shared-border.ts:231-232` and by `WC/🟦️.tsx:61-78`.

---

## 4. Native (Rust) paint

### 4.1 Shared-border model in native

- Enum: `UI/🔨️modules/📏️border-presentation/🦀️.rs:10` (`Normal, Focus, Active, Waiting, Loading, Introduced, Celebrated, Hover`). Same order as TS (`BP/🟦️.ts:34-36`).
- Resolution: `UI/🔨️modules/📏️border-presentation/🦀️.rs:328` `resolve_shared_border_segments` (mirror of `BP/🟦️.ts:39-85`).
- Recording: `UI/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🖊️draw-list-queries-and-push/🦀️.rs:203-213`. `push_shared_border` records four edges; `push_shared_border_edges` records selected edges; `push_shared_border_effect` adds an animated effect for Normal, Waiting, Loading, Introduced, Celebrated, and returns for Focus, Active, Hover.
- Chrome helpers: `UI/🎯️targets/🧊️wgpu/🖥️chrome/🦀️.rs:22-25` (`push_chrome_border`), `:30-35` (`push_control_border`: solid background if alpha > 0, then `push_shared_border` with the state), `:49-60` (`chrome_item_bg`: active and hovered uses `accent_hover`, active uses `selected`, hovered uses `button_hover`, else transparent).

So native paints each control as a closed rectangle of four owned segments in one state, resolved by the same priority as the web. No cell-level seams exist in native beyond what `push_shared_border_edges` records.

### 4.2 Where states are chosen

- Hover: `UI/🎯️targets/🧊️wgpu/⚡️events/🏗️router-construction-and-intents/🦀️.rs:352-381` sets `NodeFlags::HOVERED` on the old and new hover bubble chain. It is containment-based, like the web compositor: a child's hover flags its ancestors.
- Keyboard focus: `UI/🎯️targets/🧊️wgpu/🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:126-128` `focus_ring_visible(flags) = FOCUSED && FOCUS_VISIBLE`. This is keyboard-only, matching CSS `:focus-visible`. Doc comment at `:120-125` states the same.
- Control border choice: `UI/🎯️targets/🧊️wgpu/🖌️paint/🎨️node-painting-with-driver/🎛️control-nodes/🦀️.rs:22`, `74`, `134`, `300`. Hovered takes `border_emphasized` and `SharedBorderState::Hover`; else focus ring takes `accent` and `Focus`; else `border_normal` and `Normal`.
- Presence overlay: `UI/🎯️targets/🧊️wgpu/🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:17-40` `presence_overlay`:
  - Status Loading, Waiting, Finished paint effect borders on `bounds` (`:22-26`).
  - **Selected** paints an **outset** ring: `Rect::new(bounds.x - 1.0, bounds.y - 1.0, bounds.w + 2.0, bounds.h + 2.0)` with `SharedBorderState::Active` (`push_chrome_border`). This is a second line 1 px outside the element's own box, not on it.
  - Previewed paints `Focus` on the exact bounds.
  - Introducing paints `Introduced`; Celebrating paints `Celebrated`.
- Checkbox: `:210` `border = focus_ring_visible ? accent : border_normal` (`UI/🎯️targets/🧊️wgpu/🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:206-211`).
- Selected: `node_selection_outline` (`:44-47`) is true for any selected node except a Checkbox toggle. So the selected outset ring applies to every selected node, not only shared-border cells.
- Reference painter (`UI/🎯️targets/🧊️wgpu/🖌️paint/🔬️reference/🦀️.rs:259-263`, `298-300`, `343-346`, `379-382`, `455-457`, `582-590`) mirrors the same choices with `Hover` and `Focus`.

### 4.3 TUI

- Window chip painting: `UI/🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs:15-50`. The focused stack uses heavy line glyphs (`HEAVY`, `:43-49`); the others use light glyphs. Focus is a weight change, not a colour change.
- Chip paint: `:61-80` `chip_paint`. Active tab is bold (`attr::BOLD`) on the window background. Hovered or drop-target tab uses `Role::HoverInteractive` background. Inactive is muted.
- Chip border role: `:82-93` `chip_border_role`. Emphasized or hovered gives `BorderEmphasized`; the active stack tab gives `ActiveBase`; otherwise `BorderNormal`.
- Widget-level: `UI/⌨️tui/🪀️widget/🦀️.rs:274-277` paint dispatch with `focused: bool`. Tree row and Table row hover use `HoverInteractive` (`UI/⌨️tui/📜️rows/🦀️.rs:156`). Focus on a select uses the accent foreground (test `UI/🧪️tests/🔬️unit/🎨️tui-widget-paint-and-window-silhouette-laws/🦀️.rs:27`).
- The TUI has no seams. Each window draws its own closed glyph outline; junctions are glyphs.

### 4.4 Native parity gaps

- Selected ring is outset 1 px in native; web selected is on-edge (`CSS:6866`, `6946-6947`). Not the same geometry.
- Native hover is containment-based (bubble chain), like the web compositor, so both have the same frame-level hover behaviour.
- Native focus is keyboard-only (`focus_ring_visible`), which matches the intent of `CSS:6883`. The web compositor's focus is not keyboard-only (see 2.1 `:234`), so the two renderers disagree for pointer focus.
- Window chip focus: web uses a heavier stroke on accent-active windows (`CSS:6906`); TUI uses heavy glyphs; native not examined for chips.

---

## 5. Failure classes with concrete examples

### A. Open silhouette at a shared edge (the outline is not closed)

1. **Overview card focus.** `CSS:6881` recolours only the card's own silhouette paths. The compositor's focus test (`WC/shared-border.ts:234`) cannot see the card because the silhouette is a descendant of the focused button (`EL/🃏️OverviewCard/🟦️.tsx:71`). The seam owned by the neighbouring card stays `normal` (owner order in `BP/🟦️.ts:74`). Result: the focused card is open at the seam it shares with the neighbour.
2. **Clipped by an overflow ancestor.** `WC/shared-border.ts:256-265` and `WS/shared-border.ts:33-51` trim segments to the clip rectangle. A frame inside a scroller, or a window whose ancestor clips, loses the side outside the clip, so the outline is open exactly where the clip cuts it. Hover or focus on that element does not close it.
3. **Cell is not a loop.** A hovered or active cell recolours only its own divider (`CSS:6872-6875`, `6866`) and its own inline-end border. Its top, bottom and outer sides are the frame overlay, which recolours with the frame, not the cell (`WC/shared-border.ts:325`). So a cell's state is never one closed shape (see 2.3).
4. **Wrapped rows.** `CSS:6839-6863` drops the right divider at line ends and the lower divider on the last line, and the owner paints the edge. A cell at a line end gets a divider on one side only; its state is drawn on that one side.

### B. Double line next to the seam

1. **Native selected ring.** `presence_overlay` (`UI/🎯️targets/🧊️wgpu/🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:17-40`) paints a 1 px outset `Active` ring. Beside a neighbour's seam, or against a parent edge, this is a second line.
2. **Focus path beside the silhouette.** `WC/🟦️.tsx:385` draws the focus rectangle on top of the silhouette. If the snap in `WC/chip-focus.ts:28-32` misses (reach 2 px, `:13`), the focus rectangle and the silhouette line are two lines.
3. **Framed input ring.** `EL/✏️Input/🟦️.tsx:514` puts `aria-invalid:ring-destructive/20` (a box-shadow) on the inner `<input>`. `CSS:6748` only zeroes width and radius for `[data-shared-border-frame] > input`, not box-shadow. For an input that fills its frame, the ring falls on the frame's own line. Candidate; verify in browser.
4. **Tabpanel outline.** `CSS:6894-6897` draws a 1 px inset `outline` on a focused tabpanel, which can sit on the window edge the window's own silhouette already draws.
5. **High-contrast ring.** `CSS:8027-8030` draws a 3 px outline outside the element with an offset for every `:focus-visible` that is not excluded by `CSS:6884`. Non-shared controls get a ring outside their box.

### C. Clipped by a parent

- Same mechanism as A.2 on the compositor side (`WC/shared-border.ts:256-265`).
- Generic CSS `outline` and `box-shadow` on a control inside `overflow: hidden` are cut at the parent's edge. The shared-border rules `CSS:6884-6887` remove those for shared cells and frames, so this affects non-shared controls (for example Slider thumb ring at `EL/🎚️Slider/🟦️.tsx:40`).

### D. Corner mismatch

- Overlay outlines are square (miter joins, square caps: `WC/shared-border.ts:397-398`, `WC/🟦️.tsx:427`).
- Cells and frames set `border-radius: 0` (`CSS:6746`, `6753`, `6765`, `6770`, `6781`). So a square overlay matches square native boxes.
- Native effects use `theme.border_radius` (`UI/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🖊️draw-list-queries-and-push/🦀️.rs:213-214`, `presence_overlay` `:22-35`). Radius is not zero by definition; if the token is nonzero, native introduced and celebrated effects round where the compositor does not. The native progress comment (`🖌️paint/📶️…/🦀️.rs` doc above `paint_progress`) says the radius tokens are pinned to 0 in this design system, so the risk is latent.
- `rounded-sm` on menu items (`MIP:15`) and select items (`EL/🔽️Select/🟦️.tsx:791`) has no effect while there is no border, but is a corner mismatch if a border is added.

### E. z-order covered by a neighbour

- Overlay z-index is 40 (`WC/shared-border.ts:314` inline `z-index:40`; `WC/🟦️.tsx:394` and `421` `z-[40]`). Tokens: `--z-panel: 30` (`CSS:951`), `--z-dialog: 40` (`CSS:952`), `--z-menu: 50` (`CSS:953`), `--z-tutorial: 10000` (`CSS:957`).
- Focused cells (`focus:z-panel`, `EL/🔳️ButtonGroup/🟦️.tsx:137`) rise to 30, which is below the overlay (40). So a focused cell does not cover its frame line.
- Overlays share z 40 with `--z-dialog`. Siblings in the same stacking context tie, and DOM order decides. Candidate for covered lines inside dialogs; not reproduced.
- Navbar and footer pseudo-edges are z 50 (`CSS:6619`), above the overlay (40). Suppressed while attached (`CSS:6936-6937`), so the overlay is the visible line.
- Cell fills: backgrounds sit inside each cell's box after the native border, and the overlay is above them. A cell fill does not cover a frame line.

### F. Inconsistent per-element mechanisms

1. **Hover granularity.** Frame hover recolours the whole outline on any descendant hover (`WC/shared-border.ts:166-178`, `:225`). Cell hover recolours only the hovered cell's divider (`CSS:6872-6875`). A ButtonGroup with three buttons: hovering the middle one recolours the whole outer outline and only the adjacent dividers.
2. **Focus granularity and kind.** Frames use `:focus-within` (`WC/shared-border.ts:234`). Cells use `:focus-within` (`CSS:6865`). Chips use `:focus-visible` (`WC/chip-focus.ts:39`). Overview card uses `:focus-visible` (CSS only, `CSS:6881`). Native uses FOCUS_VISIBLE (`focus_ring_visible`). Menu items use `:focus` (`MIP:17`).
3. **Pointer focus leaves a frame in focus colour.** `WC/shared-border.ts:234` makes a click into any control of a frame paint the whole frame in focus colour until blur. The CSS comment at `CSS:6883` says "Pointer focus keeps the controls' own treatment". The two disagree. Code reading only; confirm in a browser.
4. **Selected state, three ways.** Tab: frame plus explicit `data-shared-border-state="active"` (`EL/📑️Tabs/🟦️.tsx:205-207`). Tree row: background only (`EL/🌳️Tree/🟦️.tsx:584`). Table row: fill plus bottom border that `!border-normal` overrides (`EL/📊️Table/🟦️.tsx:29-32`, `CCP`/`BP:9`). Menu row: fill only (`MIP:19`).
5. **Dead focus rules.** `CSS:6889-6892` (`border-color` on focused frame), `CSS:7987-7989` (window-measures select trigger border), and `EL/🪜️Stepper/🟦️.tsx:257` (`focus-within:border-accent`) set border colours that the ownership layer's `!important` transparency hides on attached sides. They do nothing visible.
6. **Selected child does not select a frame.** `WC/shared-border.ts:233` reads state from the element only. A ToggleGroup frame with one `data-state="on"` child is not active; the on cell's divider is (`CSS:6866` `:has(> on)`). So the frame's outline and the cell's divider disagree on the active state.
7. **Hover beats active on lines.** `WC/shared-border.ts:225` returns hover before any other state, and `BP/🟦️.ts:35` ranks hover above active. A selected element under the pointer shows the hover colour on its outline and the active-hover fill inside (`CSS:6690-6697`). Intentional, but the accent disappears while hovered.
8. **Two hover colours.** The attribute `sharedBorderStroke("hover")` is `--hover-base` (`WC/shared-border.ts:75`), while CSS paints `--border-emphasized-color` (`CSS:6879`). Browser output uses the CSS colour; any path without CSS (tests, native parity) uses the other.
9. **Window hover is a separate mechanism.** Normal windows get CSS hover (`CSS:7521-7523`, non-important `stroke`) and compositor hover (`WC/shared-border.ts:225`). Non-normal windows get only the compositor. The CSS path also overrides `focusPaint` (`WC/🟦️.tsx:385`) as described in 2.4.
10. **Pressed has no line.** No `:active` rule for any bordered control in CSS. Pressed is `data-state=on` or `data-active`. Native uses `chrome_item_bg` (`🖥️chrome/🦀️.rs:49-60`) with `selected` and `accent_hover`, not an outline.
11. **Hover lift moves geometry.** `EL/🃏️OverviewCard/🟦️.tsx:121` and `:136-138` lift the card by a transform on hover. The compositor measures transform transitions (`WC/shared-border.ts:181`, `185-191`), so the seam is re-resolved during the motion. Not a failure as such, but it is a per-frame geometry change on hover.
12. **Drop target border loses to layers.** `EL/🧭️PanelTabBar/🟦️.tsx:470` (`!border-accent` on a shared cell) is overridden by `CSS:6865-6870` (`!important` in an earlier layer). The drop target is shown by fill (`bg-accent/20`) only.

---

## 6. Not examined and open questions

- Not examined: Storybook stories, `EL/⚡️ActionGroup`'s `styleVariants` detail, every `chromeControlItem` variant, native CSS `:focus` in `UI/🧱️elements/*/🎯️targets/🧊️wgpu` beyond the paint functions listed, and the TUI widgets other than Window, Tabs, Tree, Table, Toggle, Select.
- Not verified in a browser or native run. Items marked "candidate" or "confirm" need a rendering check: pointer focus on frames (A.3, F.3), the input ring (B.3), the window focus-path loss under hover (2.4), the card focus seam (A.1).
- Test pins that already guard parts of this: `UI/🧱️elements/🗂️WindowChrome/focus-seam.cases.json` (CSS string pins), `UI/🧱️elements/🗂️WindowChrome/compositor.cases.json`, `UI/🔨️modules/📏️border-presentation/cases.json`, `UI/🧪️tests/…` (not read in full). The TUI paint law test is `UI/⌨️tui/🧪️tests/🔬️unit/🎨️tui-widget-paint-and-window-silhouette-laws/🦀️.rs`.
