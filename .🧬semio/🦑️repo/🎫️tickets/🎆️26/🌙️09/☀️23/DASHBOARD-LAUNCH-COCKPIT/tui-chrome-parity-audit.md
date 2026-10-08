# TUI chrome parity audit (rendering, layout, theming, text)

Scope: how closely the native Rust TUI target (`ui_tui`) and the dashboard that consumes it render "proper windows, same as the UI".
Input/interaction and the daemon are covered by `tui-interaction-audit.md` (same folder); where both audits found the same defect it is cross-referenced as `[I-Dnn]` instead of repeated.
Nothing in the repository was modified, nothing was compiled or run (`cargo`/`nx` forbidden, machine under load).

## 0. Method, evidence levels, path aliases

Evidence markers used on claims:

- **[R]** read from source, `file:line` cited.
- **[P]** reconstructed with a line-by-line Python port of the painting code (`window_chip_layout`, `layout_corner_tabs`, `build_corner_tab_interior`, `paint_window`, `paint_navbar`, `paint_footer`, scene `distribute`). The ASCII sketches in section 2 are the port's output, not screenshots. The port was not diffed against the Rust binary.
- **[D]** derived from Unicode data (Python `unicodedata` 13.0 East Asian Width) compared with the width tables parsed out of `TUI`. Real terminal behaviour was not measured.
- **[U]** unverified; needs a live probe (suggested probe given).

Aliases (all paths relative to `/Users/ueli/Documents/semio`):

| Alias | Path |
| --- | --- |
| `UI` | `🧰️framework/🔨️modules/🖱️ui` |
| `TUI` | `UI/⌨️tui/🦀️.rs` (5451 lines) |
| `TUIT` | `UI/⌨️tui/🧪️tests/🔬️unit/🦀️.rs` |
| `EL` | `UI/🧱️elements` |
| `WIN NAV FOO TABS CHIP LIST TBL WIZ INP SEL LOG LBL DIV` | `EL/{🪟️Window 🔝️Navbar 🔚️Footer 📑️Tabs 🪙️Chip 📃️List 📊️Table 🧙️Wizard ✏️Input 🔽️Select 🪵️Log 🏷️Label ➖️Divider}/🎯️targets/⌨️tui/🦀️.rs` |
| `DASH` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs` (941 lines) |
| `REACT` | `UI/🎯️targets/⚛️react/🟦️.tsx` |
| `CANVAS` | `EL/🎨️Canvas/🟦️.tsx` (dock tab bar and stack, `ModeDockTabBar` 968-1118, `ModeDockStack` 1135-1252) |
| `RWIN`, `WCHROME`, `WSIL`, `RTABS`, `RNAV`, `RFOOT`, `RTBL`, `RSEL` | `EL/{🪟️Window 🗂️WindowChrome 🔲️WindowSilhouette 📑️Tabs 🔝️Navbar 🔚️Footer 📊️Table 🔽️Select}/🟦️.tsx` |
| `CCP`, `IP`, `SBP`, `FCP` | `UI/🔨️modules/{🎛️chrome-control-presentation 🖱️interaction-presentation 🌀️status-border-presentation 📝️form-control-presentation}/🟦️.ts` |
| `TOK`, `CSS` | `UI/🎨️styling/🔤️tokens/🦀️.rs`, `UI/🎨️styling/🎨️palette/🎨️.css` |
| `DOCKW` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` |

TUI module line map (`TUI`): geometry 8, theme 87, text 205, cell 669, ansi 839, vt 1211, event 2109, scene 2178, layout 2428, widget 2968, chrome 3411, engine 3986, backend 4198, host 5356.

---

## 1. The reference design

### 1.1 Where the design lives

The React `Window` element (`RWIN:119-428`) is only a body plane: it paints the window-level surface, an optional top-right control group (external / focus-unfocus / close, `RWIN:271-289`, positioned `absolute top-1 right-1` at `RWIN:313`) and five anchored fold-panes (Actions top-left, Search top-middle, Window Options top-right, Utilities bottom-left, Projection bottom-right; `RWIN:317-423`, fixture `🐚️Shell/🧫️fixtures/🪟️window-pane-chrome/🔣️.json`). It has no title and no tab strip.

Title, tabs and the window outline belong to the **dock**: `ModeDockStack` (`CANVAS:1135-1252`) wraps each stack in `WindowChrome` (`WCHROME:395-621`) whose outline is one SVG path produced by `WindowSilhouette` (`WSIL`). The wgpu dock (`DOCKW`) is the second implementation and documents parity decisions in its comments; it is the most reliable "reference written by someone who already diffed against React".

The layout model is a tree of axis nodes (`row`/`column`, `size`) and stack nodes (`activeId`, children = window nodes with `id`, `title?: UiLabel`, `corner?`) (`REACT:7795-7815`). Instance titles live on the layout node and resolve as `child.title ?? descriptor.title ?? child.id` (`CANVAS:1155-1160`); wgpu reproduces it in `window_layout_instance_titles` (`DOCKW:826-850`) and records that dropping the node title "on the floor" was a parity bug.

### 1.2 Window anatomy

| Part | Reference behaviour | Source |
| --- | --- | --- |
| Outline | One continuous path over a U-cutout silhouette: body rectangle plus "chips" raised on the top and bottom edges; miter joins, 1 px inset, **no corner radius** (rounding exists only on the drag preview). | `WSIL:85,124-158`; `WCHROME:376-388`; `CANVAS:910` |
| Stroke kinds | `normal` = `--border-normal-color`; `active` = `--active-base`; `loading` / `waiting` = animated (spinning / dashed) border; `introduced` = secondary teal; `celebrated` = white mask. | `WCHROME:74-90`; `SBP:8-47` |
| Cap row | Height `min-h-medium` (22.4 px). Left: title chips (glass). Middle: transparent gap = the notch. Right: controls cell. Four corners map to `titleChips`, `capRightChips`, `footerLeftChips`, `footerRightChips`. | `WCHROME:507-550`; `CANVAS:1244-1247` |
| Tab content | Icon (small) + title (CSS `truncate`) + actions. Max width `12rem` (192 px). Actions in order: Focus/Unfocus (only when `!mobile && canMaximize`, i.e. more than one window), Close, drag grip. **There is no new-tab control.** | `CCP:35`; `CANVAS:1022-1099,1158`; `DOCKW:253-262,1202,1284-1293` |
| Tab states | inactive: transparent, `text-element`. hover: `bg-hover-interactive-fill` + `text-emphasized` (suppressed while the drag grip is hovered). stack-active and window-active: `bg-active-base border-active-base text-active-foreground` (solid fill). window-active adds the fill even when another stack holds the stroke. focus-visible: `ring-1 ring-inset ring-active-base`. disabled: `opacity-50`. | `IP:16-37`; `CCP:20-43,55-62`; `CANVAS:1034-1040` |
| Tab strip overflow | `overflow-x-auto overflow-y-hidden` (scrolls). Keyboard: roving tabindex, ArrowLeft/Right, Home/End, Enter/Space; `role=tablist/tab/tabpanel`, `aria-controls`. | `CANVAS:985-1009,1104` |
| Window controls (non-dock windows) | ActionGroup: external link, maximize/minimize icon swap, close. Each rendered only when its callback exists. | `RWIN:271-289`; fixture `windowControls` |
| Zoom | `Maximize2Icon` / `Minimize2Icon` swap by `isMaximized`; control hidden for a single window. | `CANVAS:1079,1158` |
| Split handles | `ResizableHandle` with join-corner hit zones between panels; panel `minSize = 8` (percent). wgpu: `SPLIT_MIN_FRACTION = 0.08`. | `CANVAS:1308-1311`; `DOCKW:20` |
| Focus / activity | Stack surface-active drives the silhouette stroke; `activeWindowId` drives tab fill; pressed/selected rows use `interactiveActiveFillClass`. | `CANVAS:1173-1175`; `IP:36-38` |
| Body | Base-level frame with `--padding-standard` (3.2 px) around a window-level content plane; the content is clipped to the silhouette. | `CANVAS:1240`; `REACT:5181`; `WCHROME:459-462` |
| Tooltips | `ChromeControlHint`: 400 ms hover delay, menu-tier glass surface at `z-menu`, every chrome control gets an accessible label. | `EL/💡️ChromeControlHint/🟦️.tsx:18-40` |
| Shared state model | `UiState` (introducing, celebrating, previewed, normal, disabled, hidden) x `UiStatus` (waiting, loading, idle, finished) x hover x selected; every renderer must distinguish them and carry text next to colour. | `UI/🎨️styling/🌓️theme/🟦️.ts:813-823`; `UI/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:55-75` |

### 1.3 Navbar and footer

- Both are `h-[var(--navbar-height)]` / `h-[var(--footer-height)]` = 28.8 px (9 spacing units), painted at the **base** level (`RNAV:226`, `RFOOT:37`; token `TOK:365`, `CSS:224`). Footer mirrors the navbar with the hairline on top (`RFOOT:28`).
- Content is a flex row of `NavbarItem { content, centered? }` with `gap = --padding-standard`. Centered items are absolutely positioned and **measured against the free band** the flow row leaves (`navbarFreeBandV1`, `navbarCenteredLeftV1`), so centred chrome never covers edge chrome (`RNAV:50-82,132-204`). A trailing slot hosts the fullscreen toggle (`RNAV:206-210`).
- The footer composition is bottom-left bar, bottom-middle bar, fill, presence pill, bottom-right bar (fixture `footerComposition`, e.g. `Display`, `Remote: detached`, `Tool`, `Command`, `No one else is here`, `Settings`).

### 1.4 Control states (list, table, tabs, input, select)

| Control | Reference | Source |
| --- | --- | --- |
| Tabs | trigger `text-element`, hover fill + emphasis, **active = `bg-active-base border-active-base text-active-foreground`**, `data-state` active/inactive, `border-transparent` otherwise. | `RTABS:196-232`; `IP:50-59` |
| Table | header `h-large text-element font-medium`, hairline `border-b normal`; body row `text-element` + hover fill; **selected = `interactiveActiveFillClass`**; drop-over `ring-2 ring-active`; sticky header; sort buttons; empty message centered muted; skeleton rows with pulse. | `RTBL:28-31,251-257,423-454,537-554` |
| Input | transparent control, `focus-visible:border-accent`, `aria-invalid:border-destructive`, no ring; collapsed display is grapheme-aware with an ellipsis. | `FCP:13`; `EL/✏️Input/🟦️.tsx:36-48,316-320` |
| Select | bordered trigger with chevron, popper listbox, `aria-activedescendant`, check mark on the selected option, `aria-selected`/`aria-disabled` rows. | `RSEL:411-459,671-730,774-806` |
| Tree row | 24 px row, 10 px indent per level, 14 px toggle. | `CSS:232`, `TOK:397-399` |

### 1.5 Design tokens that drive the above

Chrome palette (linear f32 in `TOK:619-713`; sRGB bytes computed with the same `linear_to_rgba8` formula as `UI/🎨️styling/🌗️mixing/🦀️.rs:15-23`):

| Role | Dark | Light |
| --- | --- | --- |
| level base / window / pane / panel / dialog / menu | `#001117` `#061a1f` `#112328` `#1c2d31` `#27373a` `#324143` | `#f7f3e3` `#e9e6d7` `#dad9cc` `#cccdc1` `#bec0b5` `#b0b4aa` |
| foreground / muted-foreground / element (window) | `#f7f3e3` / `#91968f` / `#828883` | `#001117` / `#3e494a` / `#737a76` |
| accent = active-base / active-foreground / active-hover | `#ff344f` / `#001117` / `#fd5a6d` | same |
| border-normal = border-element / border-emphasized | `#7b827d` / `#f7f3e3` | `#7b827d` / `#001117` |
| hover-interactive-fill | `#606966` | `#7b827d` |

Status colours exist only as brand constants in CSS, **not** in `ChromePalette`: success `#7eb77f`, warning `#fccf05`, info `#dbbea1`, danger `#a60009`, primary `#ff344f`, secondary `#34d1bf`, tertiary `#fa9500` (`CSS:173-179`). Contrast against the window surface [D]: danger 2.23:1 on dark / 6.39:1 on light; success 7.63 / 1.87; warning 11.97 / 1.19; info 10.12 / 1.41; accent 4.98 / 2.86. No single constant works in both appearances, so status needs appearance-aware roles.

Spacing scale: base unit `UI_SPACING_COMPACT_PX = 3.2` (`TOK:364`); navbar/footer 9 units (28.8 px); control height 7 (22.4 px, `--size-medium`); gap and padding 1 unit; panel inset 1; tree row 7.5 (`TOK:365-399`, `CSS:221-236`). Text sizes 11.2 / 12.8 / 14.4 px (`CSS:233-235`). Mapping to cells [U: depends on font; assume a cell of about 8-10 x 16-22 px]: control height is about one row; navbar/footer two rows (content plus hairline); 1-unit gaps and paddings collapse to **zero** cells.

---

## 2. What the TUI target actually draws

### 2.1 Glyphs, colours, geometry (per element)

Theme (`TUI:87-201`): six surfaces plus ten roles precomputed to truecolor. Roles actually painted: Foreground, MutedForeground, Accent, AccentForeground, ActiveBase, ActiveForeground, BorderNormal. **Never used by any painter**: `Role::HoverInteractive`, `Role::BorderEmphasized`, `Role::BorderElement`, `Surface::Pane`, `Surface::Dialog`, `Surface::Menu` (grep of `TUI` and all `EL/*/🎯️targets/⌨️tui` files). The theme drops palette fields that the reference uses: `element_*` (the default chrome text colour `text-element`), `active_hover`, `muted`, `panel`, `base`, and there are no status roles. Text attributes: only `BOLD` is ever set by a painter (Tabs active label, Table header); `DIM`, `ITALIC`, `UNDERLINE`, `REVERSE` exist in the cell model but no painter sets them.

**Window** (`WIN:108-198`, geometry `TUI:3732-3856`):

- Every window gets a raised top-left chip, even a single one: with no `stack_tabs` the effective tab is `"{number} {title}"` (`TUI:3616-3622`). Rows: `y` chip top border, `y+1` chip text, `y+2` body hairline with seam glyphs, content from `y+3` (`window_content_padding` = `[3,1,1,1]`, `TUI:3841-3856`), bottom border on the last row. If the rect is smaller than 4x4 the window degrades to a plain box with **no title and no controls** (`TUI:3745`, min heights `TUI:3765-3772`).
- Glyph set: light box drawing only (`┌ ┐ └ ┘ ─ │ ├ ┤ ┴`), never heavy, double or rounded. Controls are inline text in every tab chip: maximize `⤢` U+2922, new `⧉` U+29C9, close `✕` U+2715 (`TUI:3565-3567`, `3639-3653`), each followed by a space, one cell wide, no distinct colour or attribute.
- Colour: border = `ActiveBase` when `focused`, else `BorderNormal` (`WIN:113`); tab text = `Accent` when active, else `MutedForeground` (`WIN:45`), attrs 0. Fill: Window surface only over the silhouette (body plus chips, `WIN:76-105`); the notch keeps the canvas Base colour.
- Seam (`WIN:51-73`): the body hairline is closed under inactive tabs (`┴`/`├`) and open (spaces) under the active tab, bending into the body with `┘`/`└`.
- Tab strip budget (`TUI:3774-3777`): the top-left group may occupy columns `[x, x + width/2 + 1)`; each tab takes its **natural** width (label + 2 padding + 2 walls + 6 for three action glyphs, actions shed in the order new, maximize, close when room is short, `TUI:3624-3674`); tabs that no longer fit are **dropped silently** (`TUI:3685,3694`).
- Walls: `vline` at both sides from `y+1` (`WIN:137-143`). This also paints a `│` at (right edge, `y+1`), i.e. inside the notch to the right of the chip, one row above the body's `┐`. [P] visible as the trailing `│` on the tab text row in every sketch below; `TUIT:1175-1190` asserts the notch only at row 0, not row 1.

**Navbar** (`NAV:12-34`): two rows (content plus hairline). Left items `" label "` in Accent if `active` else Foreground; centre text = labels joined by one space, centred, MutedForeground; right items right-aligned; hairline `─` in BorderNormal. Surface Base. No collision handling between the three groups, no hit targets (`NavItem.id` is never read), no hover.

**Footer** (`FOO:11-26`): hairline row then content row. Each hint is `" key "` in Accent followed by `"label "` in MutedForeground (no chip background); status right-aligned in the remaining width, truncated with `truncate_to` (no ellipsis, no minimum gap to the last hint).

**Tabs widget** (`TABS:28-39`): flat `" label "` strip on Panel; active = Accent + BOLD; no fill, no underline, no overflow handling.

**List** (`LIST:34-49`): row selected only when `selected && focused` (ActiveBase fill, ActiveForeground text); prefix `"✓ "` or two spaces; `truncate_to` without ellipsis; `offset` is never updated by `list_on_key` (`LIST:12-32`).

**Table** (`TBL:82-155`): bold muted header, hairline underline, body rows separated by hairlines (two cells of height per row), tree indent plus `▾`/`▸`; selected row ActiveBase fill only when focused; body text MutedForeground; `"(empty)"` hard-coded English (`TBL:110`).

**Wizard** (`WIZ:73-113`): breadcrumb joined with `›`, filter row `"/ {filter}"` (no caret), options with `"› "` prefix on the selected row when focused; `"no matches"` hard-coded English (`WIZ:111`). No React twin; it is the TUI's command palette.

**Input** (`INP:37-47`): Panel fill, placeholder muted, caret = a `█` Accent cell **replacing** the glyph at the cursor; no border, no focus border, no horizontal scroll.

**Select** (`SEL:29-36`): one line `label: ‹ value ›`; no popup, no check mark.

**Chip** (`CHIP:11-17`): opaque fill, Accent/AccentForeground when `on`. **Label/Divider/Log**: plain text, hairline, tail-following log (`LBL`, `DIV`, `LOG`).

**Terminal pane** (`TUI:3322-3337`, VT `1211-2106`): fills with Window surface, then `VtScreen::blit_to` overwrites `screen.size` cells with the VT's own cells whose default colours are fixed **`[192,192,192]` on `[0,0,0]`** (`TUI:1218-1219`), regardless of theme. The VT cursor, selection and search matches are never painted (`TUI:1552,2017` flag is unused; `selection` unused by `paint_terminal`).

### 2.2 Sketches reconstructed from the drawing code [P]

Single window, 64 x 9, title = `Tasks` (unfocused and focused are identical glyph-wise; only the border colour differs):

```
┌─────────────┐
│ Tasks ⤢ ⧉ ✕ │                                                │
│             └────────────────────────────────────────────────┐
│                                                              │
│                                                              │
│                                                              │
│                                                              │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

Degradation by width (same title; 22, 16, 12, 9 columns) and the flat box below 4x4:

```
┌──────────┐   ┌───────┐   ┌─────┐   ┌───┐   ┌─┐
│ Ta ⤢ ⧉ ✕ │   │ T ⤢ ✕ │   │ T ✕ │   │ T │   │ │
│          └─  │       └─  │     └─  │   └─  └─┘
```

Three tabs, ids as labels (what `mount_window_layout` alone produces, `TUI:3923`), 80 x 10:

```
┌──────────┐┌──────────┐┌──────────┐
│ w2 ⤢ ⧉ ✕ ││ w3 ⤢ ⧉ ✕ ││ w4 ⤢ ⧉ ✕ │                                           │
├──────────┴┘          └┴──────────┴───────────────────────────────────────────┐
│                                                                              │
```

Three tabs with the dashboard's labels after its relabel hack, task tab active, 100 x 18 with navbar and footer (hint text from `DASH:575-577`, status from `DASH:923-925`):

```
 semio                                       dashboard
────────────────────────────────────────────────────────────────────────────────────────────────────
┌─────────────┐┌──────────────────────────────────┐
│ Tasks ⤢ ⧉ ✕ ││ [running pid 4242] bun nx  ⤢ ⧉ ✕ │                                                │
├─────────────┴┘                                  └────────────────────────────────────────────────┐
│$ bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev                                  │
│> nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev                                      │
│  vite v6 ready in 412 ms                                                                         │
│                                                                                                  │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
────────────────────────────────────────────────────────────────────────────────────────────────────
 C-B controls  C-B n new task  C-w close view  q detach 2 · connected · 148 commands · commands read
```

Note: the part of the label that identifies the task (`dev-puzzle3d-react-dev`) is cut off, there is no `…`, the controls repeat on each tab, and the footer status loses its tail.

Six tabs, active = the sixth (80 x 10): only two tabs are drawn, the active one is invisible, nothing indicates the hidden ones; at 40 columns the second tab is cut to `de`:

```
┌─────────────┐┌────────────────────────┐              ┌─────────────┐┌────┐
│ Tasks ⤢ ⧉ ✕ ││ dev puzzle3d rea ⤢ ⧉ ✕ │      and     │ Tasks ⤢ ⧉ ✕ ││ de │
├─────────────┴┴────────────────────────┴───...        ├─────────────┴┴────┴──...
```

Horizontal split (row of two stacks, 50/50, 100 x 18): adjacent borders (`││`), no gutter, no drag handle; the vertical split (column) stacks two full boxes, each paying 4 rows of chrome:

```
┌─────────────┐                                   ┌────────────────────────┐
│ Tasks ⤢ ⧉ ✕ │                                  ││ [running pid 424 ⤢ ⧉ ✕ │                       │
│             └──────────────────────────────────┐│                        └───────────────────────┐
│/                                               ││$ bun nx run ...                                │
```

Zoomed window: pixel-identical chrome to the unzoomed one, the `⤢` glyph does not change (`TUI:3565` is a constant), and nothing says it is zoomed.

Navbar and footer at 60 columns, and the footer at 40 columns (hints starve the status):

```
 semio                   dashboard
────────────────────────────────────────────────────────────
────────────────────────────────────────────────────────────
 C-B controls  C-B n new task  C-w close view  q detach 2 ·
────────────────────────────────────────
 C-B controls  C-B n new task  C-w close
```

---

## 3. Parity table (element x feature)

Status: **matches** / **approximated** / **missing** / **wrong**. Cites: TUI side first, reference second.

### 3.1 Window and stack chrome

| Feature | Status | TUI | Reference | Concrete defect |
| --- | --- | --- | --- | --- |
| Silhouette outline (U-notch, rectilinear, no radius) | matches | `WIN:108-198`, `TUI:3732-3838` | `WSIL:124-158`, `WCHROME:376-388` | geometry is faithful |
| Stray right wall in the notch | wrong | `WIN:137-143` (walls start at `y+1`) | `WSIL:124-158` (notch open) | `│` drawn beside the chip on the text row, above `┐` [P] |
| Stroke normal / active | matches | `WIN:113` | `WCHROME:85-88` | none |
| Stroke loading / waiting / introduced / celebrated | missing | none | `WCHROME:74-90`, `SBP:8-47` | no activity border for a running task |
| Focus cue | approximated | border hue only (`WIN:113`) | stack stroke + tab fill (`CANVAS:1034-1040`) | `#ff344f` vs `#7b827d` luminance ratio 1.10:1 [D]; invisible in monochrome / NO_COLOR / colour-blind use; tab label and body unchanged |
| Active tab styling | wrong | Accent **text** on window surface (`WIN:45`) | solid `active-base` fill + `active-foreground` (`CANVAS:1037-1038`, `CCP:55-62`, `RTABS:205`) | inconsistent with the TUI's own list/table selection (which does use the fill); light-mode contrast 2.86:1 (`TABS` Accent on Panel 2.23:1) [D] |
| Inactive tab text | approximated | MutedForeground | `text-element` | slightly different grey; no hover state |
| Tab hover | missing | `Role::HoverInteractive` unused (`TUI:118,193`); mouse mode 1002 only (`TUI:902`) | `IP:16-37` | no hover possible |
| Tab icon slot | missing | none | `CANVAS:1060-1065` | no verb / status glyph |
| Tab label source | wrong | `label = window_kind_id` (`TUI:3923`), then dashboard string-match rewrite (`DASH:364-377`) | `title ?? descriptor.title ?? id` (`CANVAS:1155-1160`, `DOCKW:826-850`) | see section 4 |
| Tab max width + ellipsis | missing | none (`TUI:3624-3674`); `truncate_to` drops the tail | 12rem cap and `…` (`CCP:35`, `DOCKW:1202,1237-1263`) | one long title takes the whole strip |
| Tab strip overflow | wrong | half-width budget (`TUI:3774-3777`), silent `break` (`TUI:3685,3694`) | scrollable strip (`CANVAS:1104`) | later tabs and even the active tab vanish [P] |
| Per-tab close | approximated | glyph per chip (`TUI:3649-3653`); `WindowClose` has no tab index (`TUI:3538-3540`) | `CANVAS:1083-1095` | acts on the active window, `[I-D09]` |
| Per-tab focus / maximize | wrong | always shown when `maximizable` (`TUI:3628,3639-3643`); constant glyph | only when `canMaximize` (>1 window); icon swaps (`CANVAS:1067-1082`, `DOCKW:253-262`) | shown for a single window, no restore state |
| "New tab" glyph | wrong | `⧉` on every chip (`TUI:3566,3644-3648`); dashboard maps it to a "Commands" window (`DASH:681-692`) | none; wgpu removed it as a dead button (`DOCKW:1274-1283`) | invented, repeated per tab, steals width |
| Drag grip / reorder / drop preview | missing | none | `CANVAS:880,1096` | cannot reorder tabs |
| Four corner stacks | approximated | enum and painter exist (`TUI:2572-2590`, `WIN:37-48`) | `CANVAS:1244-1247` | dashboard only uses TopLeft; no bottom chips used |
| Right controls cell | approximated | controls inline in each chip | `WCHROME:521-549`, `RWIN:313` | no separate window-level cell |
| Minimum size | wrong | `<4x4` => plain box, no title or controls (`TUI:3745,3765-3772`) | resizable min 8% (`CANVAS:1311`) | tiny windows cannot be closed or identified |
| Split gutter / handle | missing | `solve_axis` tiles edge to edge (`TUI:2652-2667`), mount boxes gap 0 (`TUI:3947`) | `ResizableHandle` (`CANVAS:1308`) | doubled `││` borders, no drag |
| Split minimum | approximated | `size.max(0.05)` (`TUI:2892`) | 8% (`CANVAS:1311`, `DOCKW:20`) | wrong constant |
| Zoom indicator | wrong | `TUI:3565` | `CANVAS:1079` | no state change |
| Pane chips (Actions, Search, Options, Utilities) | missing | none | `RWIN:317-423` | task actions live only behind `Ctrl+B` |
| Body inset | approximated | 1 cell wall, 0 gutter | `--padding-standard` 3.2 px | acceptable; text touches the wall |
| Control tooltips / labels | missing | none | `ChromeControlHint` 400 ms | `⤢ ⧉ ✕` are unlabelled |
| Disabled / hidden | missing | none | `UiState` | no dim anywhere |
| Roles, a11y | missing | none | `CANVAS:985-1009,1042-1047` | no screen-reader mode, hardware cursor hidden (`TUI:902`), `[I-D03]` |

### 3.2 Navbar, footer, tabs, lists

| Element | Feature | Status | TUI | Reference | Concrete defect |
| --- | --- | --- | --- | --- | --- |
| Navbar | left / centre / right bands | approximated | `NAV:22-34` | `RNAV:177-201` | static text |
| Navbar | collision avoidance for centred items | missing | `NAV:27-29` | `RNAV:50-82` | centre text overwrites left items when narrow |
| Navbar | clickable items, hover | missing | `NavItem.id` unread | `RNAV` | none |
| Navbar | brand mark | approximated | text `semio` (`DASH:634`) | `SemioLogo` svg | fine |
| Navbar / footer | height, surface, hairline | matches | `NAV:23,33`, `FOO:12-14` | `TOK:365`, `RNAV:226` | none |
| Footer | key hints | approximated | `FOO:17-22` key text Accent, no chip | footer = pills / tab bars | Accent on Base in light = 3.22:1 [D] |
| Footer | status text | wrong | `FOO:23-25`, `DASH:923-925` | status pills | truncated without ellipsis or gap; `"{running} · ..."` has no label; mixed languages |
| Footer | centre / presence band | missing | none | fixture `footerComposition` | no notion of peers or sync |
| Tabs widget | active state | wrong | Accent + bold (`TABS:34-36`) | fill (`RTABS:205`, `IP:50-59`) | see window row |
| Tabs widget | overflow, hover, disabled | missing | `TABS:28-39` | `RTABS:130-160` | none |
| List | selected row | approximated | only when focused (`LIST:40-41`) | always shows selection | an unfocused list shows no selection |
| List | viewport follow | wrong | `LIST:12-32` never updates `offset` | scroll into view | selection can leave the viewport |
| List / Table / Wizard / Log | scroll bar, position indicator | missing | none | `Scrollable` | no sense of length (44k options in the launcher) |
| List / Table / Wizard / Label | truncation | wrong | `truncate_to` cuts (`LIST:46`, `TBL:72`, `WIZ:81,106`) | CSS ellipsis | identifying tail of long labels lost |
| Table | header, hairlines, tree indent | matches | `TBL:82-155` | `RTBL:423-431` | body text MutedForeground vs `text-element` |
| Table | selected, sort, sticky, skeleton, drag | approximated / missing | `TBL:129-130` | `RTBL:251-257,433,537-554` | selection fill only when focused |
| Wizard | command palette | approximated | `WIZ:73-113` | `EL/⌨️Command` (React-only) | no caret on filter row; hard-coded `"no matches"` |
| Input | focus border, caret, scroll | wrong | `INP:37-47` | `FCP:13` | caret replaces the glyph; no scroll; byte cursor `[I-D21]` |
| Select | popup listbox, check mark | missing | `SEL:29-36` | `RSEL:671-806` | cycler only |
| Log | follow / pageup | approximated | `LOG:14-50` | n/a | no ANSI colour, no wrap, first PageUp is a no-op |
| Terminal pane | theming | wrong | `TUI:1218-1219,3322-3337` | window surface / element tokens | black `#000000` block with `#c0c0c0` text in both appearances (16.76:1 against the light window surface) |
| Terminal pane | cursor, selection, search highlight, scrolled indicator | missing | `TUI:3322-3337` | n/a | invisible caret, nothing says "scrolled back" |
| Status | success / danger / warning colours | missing | none in `Theme` | `CSS:173-179` | session state only as bracket text |
| Spacing | gaps, paddings | approximated | zero cells | 3.2 px | acceptable |

Constants that are dead or misleading in the consumer: `padding: [2,1,1,1]` in `add_launcher_window` (`DASH:398`), `shell` (`TUI:3881-3890`), `mount_stack` (`TUI:3930`) and the zoom branch (`TUI:3965`) are ignored for `ChromeState::Window` because `layout_node` substitutes `window_content_padding` (`TUI:2531-2534`); `gap: 1` is the only live part and affects nothing with a single child. `WindowState.number` is never set by the dashboard. `WindowLayoutWindowNode.title` is set to `None`/`"Commands"`/`"shell"` and then overwritten (`DASH:149,225,684,775,788`).

---

## 4. Tab and window naming

### 4.1 Trace of what happens today

1. **Launcher activation** (`handle_view_signal` -> `spawn_output`, `DASH:447-452,257-293`): the launcher window itself becomes the output window. `open_output` (`DASH:230-249`) removes the wizard, adds the terminal widget and sets `title = "{cmd} {args}"`, e.g. `bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev`. The command label the user picked (`verb / owner / ... / target`, built in `inventory::commands`, `📚️inventory/🦀️.rs:90-97`) is discarded.
2. **Daemon `SessionChanged`** -> `update_session` (`DASH:331-362`): sets `title = "[{status} {detail}] {cmd} {args}"` (`DASH:357`) with `detail = "exit N"` or `"pid N"` or empty. Empty detail yields `"[running ] bun ..."`. Status words are localized, the bracket syntax is not.
3. **Restore / attach** (`ServerMsg::Sessions`, `DASH:513-523`): for unknown sessions `update_session` creates a window via `add_launcher_window(.., "task")` (`DASH:339`) -> `attach_launcher` sets `"Commands"`/`"Befehle"` -> `open_output(.., "task")` (`DASH:345`) -> `"task"` -> then `"[status detail] cmd args"`. Two `remount`s and a discarded `render_full` (`DASH:341-342`) happen in between.
4. **Mount** (`mount_stack`, `TUI:3919-3940`): every window of a stack receives `stack_tabs = [label = window_kind_id ...]` (`TUI:3923`, labels like `w3`) and `ws.title = layout title or window_kind_id` (`TUI:3935`), which **overwrites** the dashboard's title.
5. **Dashboard `remount`** (`DASH:364-377`): saves every window title from the scene, calls `shell.remount`, restores the titles, and for each tab whose label **equals a known window id** replaces the label with that window's title. This is a string-match patch over a framework default; `WindowLayoutWindowNode.title` is never written, so the layout (the source of truth in React and wgpu) never knows the names.
6. **Paint**: the tab chip shows that title; there is no other place where the title is displayed. Tab text is cut by `truncate_to` without ellipsis (`TUI:3634-3635`).

### 4.2 Defects

- Tab label == long title: `[running pid 4242] bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev` (82 cells) in a strip budgeted at half the window width; the discriminating tail is lost and later tabs vanish (section 3.1).
- Status as text inside the label (`[running ]`, `[exited exit 0]`), changing length on every transition so tab geometry jumps; no colour, glyph or spinner.
- Four overwrite sources (`Commands`, `task`, `cmd args`, `[status] cmd args`) plus `w3` pre-fix labels; transient wrong names are rendered on screen between steps 3 and 5.
- Relabel-by-string-match: breaks if a title equals another window id, and only fixes tabs whose label is exactly an id; stack siblings keep a stale copy of the label list (`ws.stack_tabs = tabs.clone()` per window, `TUI:3933`).
- Layout titles `"Commands"` (`DASH:684`) and `"shell"` (`DASH:775,788`) are hard-coded English and then discarded; `Locale` is ignored there, `"Commands"` also passed to `add_launcher_window` (`DASH:685,776,789,821`).
- No stable identity for the same task across restart: `same_task` compares argv/env (`🌀️daemon/🦀️.rs:110-113`), but the label is rebuilt from `cmd args`, so two sessions of the same target with different env look identical.
- `SessionCommand` carries only `cmd, args, cwd, env, cols, rows` (`🌀️daemon/🦀️.rs:66-73`): the structured launcher record (verb, taxonomy owner, target, variant) is not in the session, so a restored dashboard cannot name tasks properly even if it wanted to.

### 4.3 Proposed deterministic rule

Inputs (add as a schema-first `LaunchLabel` on `ClientMsg::Spawn` and `SessionInfo`, persisted by the daemon with the session):

```
LaunchLabel { verb, owner: [string], target, variant?: { plugin, variant, renderer, example? }, members?: u16, argv_digest }
```

Derivation at launch time from data the dashboard already has:

- `verb`: first launcher segment (`VERBS` in `🌳️command-tree/🦀️.rs`, else `task`).
- `owner`: emoji-stripped taxonomy keys (`segment_key`, `🌳️command-tree/🦀️.rs:346-350`); the **short project name** is the Nx project name after the scope (`@semio-tech/ui-tui` -> `ui-tui`).
- `target`: Nx target or script name. `variant`: playground rows (`inject_playground_dev`, `🌳️command-tree/🦀️.rs:352-370`).
- `members`: compound size.

Rules (pure function `name(record, budget_cells) -> (tab, title)`):

1. `tab = verb + " " + subject [+ " " + qualifier]` where `subject = plugin·renderer` when `variant` exists else `owner.last()`; `qualifier` = `target` with the `verb-` prefix removed when non-empty and different from `subject` (`build-report` -> `report`), or `#example`.
2. `tab` budget 24 cells (the 12rem cap, `CCP:35`) including glyph, `×` and padding; if wider, elide the **middle of `subject`** with `…`, keeping its last 6 cells (renderer/target), never the verb. Truncation uses the grapheme-aware width of section 5.
3. `title = verb · owner path joined by " / " · target [· variant details]`, no status text. The raw argv is shown in a secondary line of the body (first row of the pane) or in a tooltip, not in the chip.
4. Status is a **glyph and role**, not text: prefix glyph in the tab, same glyph in the Tasks list. Mapping to the shared `UiStatus` (`THEMETS:822`): pending connection -> `waiting` `◌`; `Running` -> `loading` `◐◓◑◒` (frame from a 100-150 ms tick, only while visible); `Stopping` -> `waiting` `◌`; `Exited(0)` -> `finished` success `✓`; `Exited(n)` -> `finished` danger `✗` (code goes to the title's right-aligned chip, not the label); `Failed` -> danger `!`. Colour roles must be added to `Theme` per appearance (section 1.5: danger `#a60009` is 2.23:1 on the dark window).
5. Duplicates: if two live tabs in one stack share the same `tab` string, suffix ` ·2`, ` ·3` by ascending session creation time (the daemon nonce in `session_id`), stable across restarts because it is derived from persisted data.
6. Localization: `verb` strings are language-neutral identifiers mapped through a label table (en/de); `owner`/`target` stay verbatim. Titles live on `WindowLayoutWindowNode.title` (a `UiLabel`, as `REACT:7797-7803`); `mount_stack` reads that and the string-match hack in `DASH:364-377` is deleted.

Examples (real inputs from `🌳️command-tree/🦀️.rs`):

| Source command | tab (<=24) | window title |
| --- | --- | --- |
| `bun nx run @semio-tech/ui-tui:test` (label `test / 🧰️framework / ... / test`) | `✓ test ui-tui` | `test · ui-tui` |
| `bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev` (playground `dev / puzzle3d / puzzle3d / react / all examples`) | `◐ dev puzzle3d·react` | `dev · puzzle3d · react · all examples` |
| same with example `sphere` | `◐ dev puzzle3d·react #sphere` -> elided `◐ dev puzzle3d·rea…#sphere` | `dev · puzzle3d · react · sphere` |
| `bun run build:wgpu` (workspace script) | `✗ build wgpu` | `build · workspace scripts · wgpu` |
| launch compound `dev: API + Web` | `◐ dev API+Web ×2` | `dev · API + Web (2 processes)` |
| two sessions of `test ui-tui` | `✓ test ui-tui`, `◐ test ui-tui ·2` | same title |

---

## 5. Text correctness

### 5.1 Width model (`TUI:205-664`, `cell` 669-835)

Facts [R]: a cell stores one `char` (`TUI:685-691`). `put_str` **skips every zero-width scalar** (`TUI:755-759`), so U+FE0F (VS16), U+200D (ZWJ), combining marks, U+20E3 and tag characters are never emitted. `char_cells` = 0 for controls and 351 zero ranges (`ZERO_WIDTH_RANGES`, `TUI:207-559`), 2 for 61 wide ranges (`is_wide`, `TUI:567-632`), else 1. `display_width` sums scalars (`TUI:647-649`). `TUIT:515-525` locks "the cell renderer retains one glyph per emoji scalar and skips the joiner".

Consequence [D, not measured on a terminal]: whenever the table's width for a base character differs from what a terminal computes for that same bare character (VS16 having been stripped), the rest of the run is shifted.

| Class | Rule in the table | What the terminal sees | Effect | Reach |
| --- | --- | --- | --- | --- |
| A: text-default pictographs inside `U+1F300-1F64F` and `U+1F680-1F6FF` | width 2 for **all** of the block | EAW Neutral, no VS16 => width 1 | text after the glyph is drawn 1 column left of the engine's grid for the rest of the run; right border and hit columns disagree on that row | 237 code points in the scan, among them `🖥 🖱 🛍 🗂 🏷 🕸 🕰 🖼 🖋 🛡 🗜 🎚 🏗 🏛 🗄 🎛 🗺 🎞 🛰 👁 🛠 🗣 🖌 🗃 📽 🎟` (`🎛️dashboard`, `🖥️terminal`, `🖱️ui`, `🛍️products` are in this class) |
| B: BMP text-default symbols + VS16 | width 1, VS16 dropped | text-style glyph width 1 | no desync; coloured emoji look lost (`⌨️ ⚛️ ✏️ ☑️ ⚙️ ↔️ ↕️ ⏱️ ❤️ ⚠️`) | fidelity only |
| C: ZWJ sequences | scalars 2 + 0 + 2 | terminals with clustering draw one 2-cell glyph; the TUI emits two emoji (4 cells) | consistent but wrong glyph (`🧑‍💻dev` => `🧑💻dev`) | 11 names in `🧰️framework` |
| D: emoji modifiers `U+1F3FB-1F3FF` | width 2 | clustered into the base glyph (0 extra) | -2 drift in clustering terminals | output only |
| E: combining marks | width 0, **not emitted** | n/a | NFD input loses accents (`e` + U+0301 draws `e`) | child output on macOS |

Measured [D]: scanning every name under `🧰️framework` (excluding `node_modules`, `target`, `dist`, `temp`, `.git`, `🗑️generated`; depth <= 7) gives 6 938 distinct names, 5 863 carry VS16, **782 of 5 887 emoji-bearing names (13.3%)** have a class A mismatch (`🛍️products` believed 10, drawn 9; `🖥️platform` 10 vs 9; `🛠️tool-machine` 14 vs 13). The most frequent offenders: `🖋 🖼 🛡 🏷 🗜 🎚 🕸 🏗 🏛 🗂 🗄 🖥 🎛`. The dashboard prints these names in launcher labels (`inventory::commands` joins raw directory names, `📚️inventory/🦀️.rs:90-97`), in the Tasks list and in terminal output.

Fix direction: grapheme-cluster cell model (`Cell { cluster, width }`), width from a table that distinguishes `Emoji_Presentation` and VS16; probe the terminal at start (DECRQM `?2027`, or print a probe glyph and read `CSI 6n`) and either emit VS16 and count 2 or normalise class A/B to a safe presentation; unit test by feeding the emitted ANSI back through the repo's own `VtScreen` and comparing grids (the existing tests grep raw bytes only, e.g. `dashboard` tests assert `rendered.contains("exit 0")`).

### 5.2 Wide cells, continuations, half cells

- `CellBuffer::put` documents that it blanks orphaned continuations on either side (`TUI:728`) but only clamps width at the right edge and pairs continuation cells on write (`TUI:729-749`). Overwriting a wide lead with a narrow glyph leaves the `\0` continuation, and `emit_runs` skips width-0 cells without advancing the terminal (`TUI:889-892`): the rest of the run shifts one column. Overwriting a continuation with a narrow glyph leaves the lead at width 2 and the narrow glyph is lost under the wide one. The current painters mostly overpaint with `fill_rect`, so the trigger is rare (single-cell `put`, e.g. the input caret `█` over a wide glyph, `INP:45`); [P] it is a latent defect.
- `diff` on differently sized buffers returns one run of `w*h` cells (`TUI:809-811`, locked by `TUIT:610-618`) but `emit_runs` iterates one row only (`next.get(x, y)` is `None` past the row), so that path would paint a single row; the engine avoids it because `resize` sets `full_redraw`.
- Truncation never inserts an ellipsis anywhere (grep `…`/`u{2026}`: no hit in `TUI` or any `⌨️tui` element). `truncate_to` stops before a wide char that does not fit (`TUIT:526-535`), so a wide glyph at the boundary leaves an unpainted cell.

### 5.3 Colour emission and appearance

- Every cell is emitted as `ESC[0;...;38;2;r;g;b;48;2;r;g;bm` (`TUI:855-879`); there is no 256-colour or 16-colour fallback and no detection (no `COLORTERM`, `TERM`, `NO_COLOR` anywhere in `ui_tui` or the dashboard; grep). Terminals without 24-bit SGR (Linux console, old conhost, tmux without RGB, `screen`) will mis-render [U: verify on the target terminals]. `[I-D31]` (children get no `TERM`/`COLORTERM`).
- Appearance is `dark`/`light` only; the dashboard maps `preferences.appearance == "light"` (`DASH:632`); no `system`, no OSC 11 background query; changing appearance forces a full redraw (`TUI:4039-4042`). All cells are opaque, so the user's terminal theme is overridden everywhere, including the embedded terminal default colours (section 2.1).
- Contrast [D]: light appearance Accent text on the window surface 2.86:1, Accent on Panel 2.23:1, Accent on Base 3.22:1 (tab labels, Tabs widget, footer keys, navbar active item); dark appearance passes (4.98 / 3.99 / 5.36). Muted text passes in both.
- VT colour mapping uses fixed xterm RGB for the 16 ANSI colours (`TUI:1223-1242`) instead of theme roles, so `green`/`red` output ignores the palette (acceptable) but black-on-black and white-on-cream combinations are possible in light mode.

### 5.4 Other text defects

- English strings inside framework painters violate "no default language": `"(empty)"` (`TBL:110`), `"no matches"` (`WIZ:111`), Select format (`SEL:33`); the dashboard localizes via `Dashboard::text` but not `connection_status` literals (`DASH:498,508,543`), the navbar (`DASH:634`), or `"Commands"` (`DASH:684-685`).
- `Input` cursor is a byte offset (`INP:25-31`); Left then typing on `ü` panics at `String::insert`; `TUIT:956-991` asserts the byte step `[I-D21]`.
- Combining marks and ZWJ are dropped in the embedded VT as well (`TUI:1798-1803`).

### 5.5 Embedded VT fidelity (rendering side)

`finish_csi` (`TUI:1447-1491`) handles `A B C D H f J K L M @ P X S T r m`; `feed_escape` (`TUI:1391-1427`) handles `[ ] P X ^ _ 7 8 c`. Not handled: `CSI G` (CHA), `d` (VPA), `E`/`F`, `s`/`u`, `b`, `ESC M` (reverse index), `ESC D`/`E`, charset designators (`ESC ( B` falls into `_ => Ground`, so the following `B` is **printed**), colon sub-parameters in SGR (`38:2::r:g:b` terminates the CSI and prints the rest, `TUI:1443`), SGR 5/8/9/21/53. Node's `readline.cursorTo(x)` emits `CSI x+1 G`, `tput sgr0` emits `ESC ( B`; both are common in the tools the dashboard hosts [U: confirm against a recorded nx/bun session replay]. Query responses (DSR/DA) are `[I-D14]`.

---

## 6. Layout engine

- **Constraint model** (`TUI:2432-2463`): `Dimension::{Auto, Cells, Weight}`, `Direction::{Row, Column, Stack}`, `gap`, `padding`. No min/max, no shrink, no wrap, no alignment, no absolute positioning. `Auto` measures only `Text` and widgets; boxes and chrome measure `(0,0)` (`TUI:2465-2474`), so an `Auto` child of a window or box collapses to 0.
- **Algorithm** (`distribute`, `TUI:2476-2521`): fixed and auto first, remaining space by weight with largest-remainder rounding; if fixed sizes exceed the total, weighted children get 0 cells and fixed children overflow the parent; weighted children can be 0 wide. No overflow policy beyond `inset_sides` saturation and `put_str` clipping. Children are not clipped to their parent's rect (a widget paints into its own rect only).
- **Two solvers**: `solve_window_layout`/`solve_axis` (f64 weights with `round()` and `min(extent-offset)`, `TUI:2652-2715`) and the scene's `distribute` (u16 weights from `round(size*100).max(1)`, `TUI:3911-3913`). `shell()` only uses the first one to enumerate window ids on a zero rect (`TUI:3881`); rounding rules differ (leftover cells can stay empty in `solve_axis`).
- **Shell regions** (`TUI:3868-3894`): navbar `Cells(2)`, canvas `Weight(1)`, footer `Cells(2)`; below 5 rows the canvas has no room and windows collapse (`paint_window` returns below 2 cells, `TUI:109`). No minimum terminal size screen.
- **Tiny terminals** [R]: chip drops controls at width < 16 and the whole chip below 4x4 (section 2.2). A 24-row terminal spends 4 rows on navbar+footer plus 4 on one window's chrome; a vertical split of two windows leaves 2-3 content rows each.
- **Resize**: `Tui::resize` reallocates both buffers and forces a full redraw (`TUI:4031-4037`), but the native backends never produce `Event::Resize` (`TUI:5386` is the only producer; `DASH:631` the only `size()` call) `[I-D01]`.
- **Embedded terminal size**: `open_output` calls `tui.render_full()` just to read a rect (`DASH:236`, `342`; the patch is discarded), then sizes the VT as `chrome.rect - 4` in both axes (`DASH:238`). Real inner size is `width-2` x `height-4` (`window_content_padding`), so the VT and PTY start **2 columns narrow** (PTY gets `cols = term_size.width`, `DASH:275`). `resize_terminals` (`DASH:550-566`) runs only on the dead `Event::Resize` arm and on `ReplayComplete`; it is not called after split, zoom, close, new tab or tab switch, and it reads `tui.scene.rect()` from the *previous* layout pass. `TerminalState::resize` copies the top-left region and cuts the bottom rows on shrink (`TUI:1621-1650`); no reflow `[I-D13,I-D35]`.
- **Zoom** (`TUI:3961-3968`): only the zoomed window is reparented under the mount box; all other windows were already reparented to the canvas (`TUI:3900-3906`), keep their `visible` flag, and are laid out and painted at full canvas size underneath `[I-D32]`.
- **Scroll regions**: only inside the VT (DECSTBM, `TUI:1977-1988`); scrollback pushes only when `scroll_top == 0` (`TUI:1708-1717`). The UI has no scroll container; `List/Table/Wizard/Log` each hand-roll an offset (`List` never updates it).
- **Z-order and overlays**: painting is a single DFS in child order (`TUI:4137-4158`), hit-testing reverses it (`TUI:2354-2368`). There is no overlay layer, no popup, no menu, no tooltip, no modal, no toast, no command palette overlay: `Surface::{Pane, Dialog, Menu}` exist as colours but nothing uses them. Adding an overlay needs an absolute-position layout primitive (the root is a Column of navbar/canvas/footer, so a fourth child would steal space) and clip/shadow rules.

---

## 7. Rendering pipeline cost

- `Tui::render` = layout-if-dirty -> paint -> `diff` -> `emit_runs` (`TUI:4161-4186`); `render_full` forces the full-frame branch (`TUI:4169-4179`, `4189-4192`). The dashboard **only** calls `render_full` (`DASH:236,342,645,929`). `diff`, run merging (`MERGE_GAP = 4`, `TUI:807`) and the incremental path are never exercised by the dashboard; only `WasmHost::render` uses `render()` (`TUI:5389-5391`).
- Reason the incremental path cannot be used: `mark_dirty` stops at the first ancestor that already carries the flags (`TUI:2335-2337`), while `take_dirty` clears **only the root** (`TUI:4162`); every other node keeps `LAYOUT_DIRTY|PAINT_DIRTY` forever (initial value `TUI:2225`), so any later mutation of a non-root node never reaches the root and `render()` returns an empty patch [R, by reading; demonstrate with a 6-line test: add a `Label` under root, `render_full`, `set_text`, `render()` => empty]. `TUIT:1441-1449` only tests the unmutated case. `[I-D08]`.
- Per `render_full`: whole-tree relayout, whole-tree repaint into a `back` buffer that is never cleared (every region must be overpainted), `front = back.clone()` (a 12-byte `Cell` per position), every row emitted from column 0 with a fresh SGR state. Estimated 35-60 KB per frame at 200x50 (about 10 000 chars plus 600-1 500 transitions of ~36 bytes) [estimate, assumptions: typical colour churn]; up to ~12 frames per second because the loop sleeps in `term.poll(80 ms)` (`DASH:651`).
- **Wizard cost**: `visible_indices` lowercases every option on every call (`TUI:3110-3114`), 44k options in the real workspace (`control-plane-completion.md`), at least once per paint (`WIZ:75`) and per key; `refresh_views` clones all labels per window on every session event (`DASH:118-137,361`) `[I-D11]`.
- **Flicker sources**: no synchronized output (`?2026h/l`; absent from `setup_sequence`, `TUI:901-903`, and from `present`, `TUI:4545-4548`); one `write_all` of the whole frame; no per-frame clear (good); identical cells rewritten each frame; the bottom-right cell is written every frame (auto-wrap is not disabled, `?7l` not sent). `[I-D19]`.
- **Latency**: output and echo wait for the next loop turn (up to 80 ms, `DASH:650-651`) because the tty and the daemon socket are not multiplexed `[I-D02]`.
- `emit_runs` re-sends both colours on every change (`TUI:855-879`); no `CSI K`/`REP` use; no reset at patch end.

---

## 8. Elements that exist in React but have no TUI target

Matrix from `EL/*` (react = `🟦️.tsx`; wgpu/tui = `🎯️targets`): TUI targets exist for 13 elements (Chip, Divider, Footer, Input, Label, List, Log, Navbar, Select, Table, Tabs, Window, Wizard). Chip, Divider, List, Log, Wizard are TUI-only (no React twin).

Needed for a faithful dashboard, with the React source to project from:

| Element (React) | Why the dashboard needs it | Priority |
| --- | --- | --- |
| `📜️Scrollable` (scroll area + bar) | log/list/terminal position, thumb | P0 |
| `↔️Resizable` + `🧱️DragHandle` (split handle, tab grip) | gutters, drag-resize, tab reorder | P1 |
| `🖱️ContextMenu`, `📋️MenuItem`, `🗨️Popover` | per-task actions (restart, stop, kill, copy) | P1 |
| `⌨️Command` (palette) | replace the launcher-as-window; overlay | P1 |
| `💡️ChromeControlHint` (tooltip) | label `⤢ ✕`, show key hints | P1 |
| `💬️Dialog` / `📨️UIDialog` | confirm kill, shutdown, close running task | P1 |
| `⭕️Ring`, `🦴️Skeletons`, `🪜️Stepper` | activity/progress for discovery and tasks | P1 |
| `🔘️Button`, `🔳️ButtonGroup`, `⚡️ActionGroup` | window controls as real hit targets | P1 |
| `🌳️Tree` | grouping of the command tree and task list | P2 |
| `☑️Checkbox`, `🔀️Toggle`, `🎛️ToggleGroup` | settings window (today a cycler list) | P2 |
| `📝️Field`, `🧾️Form`, `🔤️Textarea` | settings and filters | P2 |
| `🧭️PanelTabBar`, `🖼️Panel`, `🎀️Ribbon` | window pane chips (Actions, Utilities) | P2 |
| `🔣️Icons` | glyph registry with ASCII fallback | P1 |
| `↕️Collapsible` | fold panes | P2 |

Not present anywhere in React either: toast, badge, breadcrumb (`grep` of `🧱️elements`/`🎯️targets` finds only incidental text hits); a status badge and notification toast would need new schema-first elements (the shared `UiPresence` model already defines the states).

---

## 9. Prioritized rendering defects

P0 = prevents a faithful window system or corrupts the display. Fix directions are minimal and architecture-neutral; each needs a VT-projection test (feed `Tui` output into `VtScreen` and compare to `Tui::frame()`), not byte greps.

| Id | Defect | Evidence | Fix direction |
| --- | --- | --- | --- |
| R01 | Native TUI never learns the terminal size changed | `TUI:4545-4569,4756-4771,5386`; `DASH:631,657-661` | poll `size()` per tick plus SIGWINCH self-pipe / console buffer event; resize `Tui`, then relayout, then PTYs. `[I-D01]` |
| R02 | Tab strip drops tabs; active tab can be invisible; one long title consumes the strip | `TUI:3685,3694,3774-3777` | React semantics: per-tab max 24 cells with `…`, strip uses the full width minus window controls, active tab always scrolled into view, `‹ N ›` overflow chips; hit-test the same geometry |
| R03 | Tab/title naming (ids, long text, status text, string-match hack) | `TUI:3923,3935`; `DASH:331-377` | section 4.3: `LaunchLabel` in the spawn schema, titles on layout nodes, delete `remount` hack |
| R04 | Terminal widget/PTY size not derived from the laid-out rect; starts 2 columns narrow; stale rects | `DASH:236-238,275,535,550-566,659` | single `sync_terminal_sizes(tui)` after every layout pass, diffing `rect` against `screen.size`; remove the `-4` guess and the discarded `render_full` calls (expose `Tui::layout()`) `[I-D13]` |
| R05 | Text-width model desyncs rows for ~13% of repo names (class A) and strips VS16/ZWJ/combining marks | `TUI:567-632,752-773`, section 5.1 | grapheme-cluster cells, presentation-aware table, terminal probe, normalisation fallback, VT-projection test |
| R06 | Per-tab controls and signals carry no tab index; always-on maximize and invented new-tab | `TUI:3538-3546`; `DASH:666-692` | `WindowClose(tab)`, `WindowMaximize(tab)`; show focus only if >1 window; drop per-tab `⧉` (one `+` chip at strip end if a new-tab is wanted) `[I-D09]` |
| R07 | Incremental render path dead; dashboard repaints the full frame on every event | `TUI:2328-2347,4161-4192`; `DASH:929` | clear dirty during layout/paint (or epoch counter), add mutate-then-render test, then use `diff` and `?2026` `[I-D08,I-D19]` |
| R08 | Active tab/label colour-only and low-contrast; focus cue only a hue change | `WIN:45,113`; contrast numbers in 1.5 and 5.3 | filled active tab (`ActiveBase`/`ActiveForeground`), heavy line set for the focused stack, dim unfocused chrome, keep a no-colour mode |
| R09 | Terminal pane ignores theme; no cursor, selection, search highlight, scrolled indicator; view drifts with new output | `TUI:1218-1219,1675-1706,3322-3337,3218-3239` | map VT default fg/bg to Window/Foreground roles, draw cursor from `cursor_visible`, anchor viewport to absolute row, scroll bar `[I-D03,I-D15]` |
| R10 | No ellipsis; tail loss; footer status unbounded | `TUI:652-664`; `LIST:46`, `TBL:72`, `FOO:23-25` | `truncate_ellipsis(s, w, Middle|End)` shared helper; reserve status minimum width |
| R11 | No status roles/glyphs; running state is text | `Theme`, `DASH:123,353-357` | `Role::{Success,Warning,Danger,Info}` per appearance, glyph set `✓ ✗ ! ◐ ◌` with ASCII fallbacks, spinner tick |
| R12 | Colour capability not detected | grep section 5.3 | read `COLORTERM`/`TERM`/`NO_COLOR`; quantise to 256/16; monochrome attributes (bold/reverse/underline) for focus and selection |
| R13 | Zoom: no state, other windows still laid out and painted; maximize shown for one window | `TUI:3961-3968`, `3565` | unmount hidden windows or set `visible=false`, restore glyph, hide control when only one window `[I-D32]` |
| R14 | No hover, tooltips, gutters, scroll bars | section 3 | mouse 1003 + `Move`; `Tooltip` overlay; split gutter with drag; `Scrollable` element `[I-D07]` |
| R15 | VT gaps: CHA/VPA, charset designators, RI, colon SGR | `TUI:1391-1491` | add sequences; table-driven conformance test replaying recorded nx/bun/vite sessions |
| R16 | Compact chrome for small terminals (4 rows of chrome per window) | `TUI:3841-3856` | `<30` rows: one-row cap with tabs on the top border (`┌ ◐ dev × │ ✓ Tasks × ┐`), no bottom chrome |
| R17 | Stray right wall in the notch | `WIN:137-143` | start walls at `top_body_y + 1` when a top chip exists |
| R18 | `put` continuation handling and `diff` size-change run | `TUI:728-749,809-811` | blank orphaned halves; return per-row runs |
| R19 | Dead roles/constants, duplicated solvers, unused fields | section 3 tail | delete or wire (`HoverInteractive` for hover, `BorderEmphasized` for focus); one solver |
| R20 | Hard-coded English in painters and dashboard literals | section 5.4 | label ids resolved through the i18n table |
| R21 | `Input` byte cursor, caret hides glyph, no horizontal scroll | `INP:12-47` | grapheme cursor, real hardware cursor or reverse-video cell `[I-D21]` |
| R22 | `List` never scrolls selection into view; selection hidden when unfocused | `LIST:12-48` | persist viewport; unfocused selection style (outline or dim fill) |
| R23 | Navbar centre/left/right overlap; footer hints can starve status | `NAV:27-33`, `FOO:17-25` | free-band placement like `navbarFreeBandV1` |
| R24 | Glyph font coverage for `⤢ U+2922`, `⧉ U+29C9`, `✕ U+2715` | `TUI:3565-3567` | WGL4-safe set or ASCII fallback table (`ui.glyphs=unicode|ascii`) [U: check Menlo, Consolas, Cascadia] |
| R25 | `"[running ]"` when pid/code absent | `DASH:355-357` | disappears with section 4.3 |

Priority split: P0 = R01-R07; P1 = R08-R16; P2 = R17-R25.

---

## 10. Proposed target window anatomy (faithful terminal projection)

Rules the mock encodes (each maps to the reference):

1. Silhouette is the same rectilinear path as `WSIL`; the **focused stack uses the heavy line set** (`┏━┓┃┗┛┣┻`), unfocused stacks the light set, so focus is visible without colour and without a hue-only cue.
2. Cap row = chip row + seam row (2 rows) for >= 30 rows, compact one-row variant below that (R16). Tabs sit in the top-left corner group; per tab: status glyph, label (<= 24 cells including glyph and `×`), `×` close. The **active tab is a solid `ActiveBase` fill with `ActiveForeground` text** (not representable in plain text; marked below by `■`), inactive tabs `text-element`, hover = `HoverInteractive` fill. Overflow uses `‹`/`›` chips and keeps the active tab visible.
3. A single window-level control cell at the right end of the cap row: `⤢`/`⤡` only when more than one window exists; no per-tab `⧉`; a `+` chip after the last tab if a new-tab is wanted. All controls are real hit targets with a tooltip row in the footer on hover.
4. Body: window-level surface; a vertical scroll bar column (`▲ █ ░ ▼`) inside the right wall for scrollable content; terminal panes use `Window`/`Foreground` default colours, a visible cursor, and a `↑ 120 lines · End` badge when scrolled back.
5. Split: a one-cell gutter between stacks (`GAP`), drag handle highlights on hover; min share 8%.
6. Navbar/footer at base level; centred items placed in the free band; hints are chips (`ActiveBase` fill on the key), status pills right-aligned with minimum gap and ellipsis.

```
 semio  ▸ dashboard                                               ◐ 2 running · ● connected · dark
────────────────────────────────────────────────────────────────────────────────────────────────────
┌───────────┐                     ┏■■■■■■■■■■■■■■■■■■■■■■■■┓┏━━━━━━━━━━━━━━━━━┓┏━━━━━━━━━━━━━━━━┓
│ ✓ Tasks × │                     ┃ ◐ dev puzzle3d·react × ┃┃ ✗ test ui-tui × ┃┃ ✓ build wgpu × ┃ ⤢
│           └───────────────────┐ ┃                        ┗┻━━━━━━━━━━━━━━━━━┻┻━━━━━━━━━━━━━━━━┻━━┓
│ /  filter commands…           │ ┃$ bun nx run @semio-tech/framework-os-dev:dev-puzzle3d…        ▲┃
│                               │ ┃  vite v6.0.1 ready in 412 ms                                  █┃
│   New task…                   │ ┃  ➜ Local:   http://localhost:6058/                            █┃
│   Settings                    │ ┃  ➜ Network: use --host to expose                              █┃
│   Refresh commands            │ ┃12:01:07 [vite] hmr update /src/App.tsx                        ░┃
│   Cancel discovery            │ ┃                                                               ░┃
│ ────────────────────────────  │ ┃                                                               ░┃
│ ◐ dev puzzle3d·react   12s    │ ┃                                                               ░┃
│ ✗ test ui-tui       41s       │ ┃                                                               ░┃
│ ✓ build wgpu         9s       │ ┃                                                               ░┃
│                               │ ┃                                                               ░┃
└───────────────────────────────┘ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
────────────────────────────────────────────────────────────────────────────────────────────────────
 Ctrl-B controls  n new task  x close  z zoom  / search  q detach        ● connected · 148 commands
```

Legend: left stack unfocused (light lines, one tab `Tasks` showing the overview list with session rows `glyph · label · duration`); right stack focused (heavy lines), three tabs, first active (`■` = filled `ActiveBase` chip), seam open under the active tab and closed (`┻`) under the others; the one-cell gutter between stacks is the drag handle; `⤢` is the stack-level focus control. Tab strings come from the naming rule in 4.3. The mock was assembled and width-checked with the same width table as the engine (all rows 100 cells).

---

## Appendix A: cross-reference to the interaction audit

Same defect found independently in both audits: resize never detected `[I-D01]`, dirty propagation `[I-D08]`, tab controls act on the active window `[I-D09]`, PTY/VT size not synced `[I-D13]`, scrollback drift `[I-D15]`, full-frame emit and no `?2026` `[I-D19]`, zoom leaves windows `[I-D32]`, input latency `[I-D02]`, invisible cursor `[I-D03]`, `/` search trap `[I-D05]`, `Input` byte cursor `[I-D21]`. This audit adds: width model (class A-E), tab strip geometry and naming, theme/contrast/status tokens, terminal pane theming, ellipsis, reference-design parity matrix, VT sequences that affect rendering (CHA, charset, colon SGR), and the target anatomy.

## Appendix B: reproduction notes (kept out of the repo)

- Width comparison: parse `ZERO_WIDTH_RANGES` and `is_wide` out of `TUI`, compute `display_width` per name, and compare with `unicodedata.east_asian_width` (W/F = 2 else 1, controls/combining = 0) after dropping zero-width scalars (what `put_str` emits). Scan: `os.walk` of `🧰️framework` skipping `node_modules target dist temp .git 🗑️generated`, depth <= 7. Result: 782 of 5 887 emoji-bearing names differ.
- Palette numbers: `CHROME_DARK`/`CHROME_LIGHT` (`TOK:648-713`) through `linear_to_rgba8`; WCAG relative luminance contrast.
- Sketches: Python port of the painters listed under [P]; widths use the repo's own tables so alignment matches the engine's belief, not a terminal's.
- To prove R07 in Rust (not run here): `Tui::new`, add a `Label` widget child of root, `render_full()`, `node_mut(label).set_text("x")`, assert `render().0` is empty.
