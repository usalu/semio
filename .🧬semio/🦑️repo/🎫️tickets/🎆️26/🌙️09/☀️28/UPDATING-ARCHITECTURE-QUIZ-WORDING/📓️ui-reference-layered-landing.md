# 📓️ UI reference: the layered landing of `🎡️play` and `🧺️demonstrator`, and one shared `LayeredOverview`

New requirement (user): the quiz page must be **multilayered like play** — the grid's content behind the glassy layer, and **on hover over a card the page shows clear**.
This report specifies exactly how the two landings do it (measured, not assumed), lists their duplicated code with line ranges, and recommends one
domain-neutral element that play, the demonstrator and the quiz home (nine cards) can all use. A working prototype of that element and of the quiz
home on top of it is checked in beside this report. Read-only exploration: the only repo files written are this report and two prototype files in the ticket folder.

## 0. Evidence and method

| Item | Detail |
|---|---|
| Live check of the real apps | **Not done.** The real dev servers need ~100 Nx tasks incl. wasm builds for dozens of plugins and write build outputs all over the repo (`dist`, generated registries), which read-only forbids; the cargo workaround (`CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false`, private `CARGO_BUILD_BUILD_DIR`/`CARGO_TARGET_DIR`) concerns the cargo build layout, not the size of the task graph or its side effects, so I did not attempt it. The layered mechanics do not depend on what a pane renders, so I extended the rig instead. |
| Rig | `🗑️generated/ui-reference/rig/` (Vite, port 6099, **stopped**). `real-demonstrator.html` and `real-play.html` import the **real, unmodified** `♻️mit-bestand/🧺️demonstrator/🟦️.tsx` and `🏢️semio-tech/🎡️play/🟦️.tsx` (real `🪧️brand.ts`, cards, footer, `globals.css`, `Navbar`, `UIIntroduction`, `ui-react`). Only the OS shell is faked: `stubs/framework-renderer-react.tsx` (`FrameworkOsShell` → a hue-coded page with a `<canvas>`, so posters can be captured and mounts/unmounts logged in `window.__rigShell`), `stubs/plugin-registry-catalog.ts`, `stubs/puzzle-js.ts`, `stubs/framework.ts` (real `@semio-tech/framework` + a stub `resolvePlaygroundBoot`), and a Vite transform that lets play's `🔨️modules/🧩️runtime/🟦️.ts` survive its stale generated registry (`Unknown play pane variant: stdio-txt`). Everything below marked *measured* was read from that running real landing code. |
| Caveat | The browser pane throttles `requestAnimationFrame` in this environment (a 500 ms glide took ~1–1.5 s and was sampled at a few frames), so per-frame sequences are inferred from the code and from the frames that were sampled; end states, timings of timers, DOM structure, counts and computed values are exact. |
| Working-tree state | While I worked, other workers applied parts of report 1: `🧱️elements/🃏️OverviewCard` (new, staged), `🧱️elements/🗂️WindowChrome`, the slim `@semio-tech/ui-react/chrome` sub-entry (`🎯️targets/⚛️react/🪟️chrome/🟦️.ts`), and both cards (`⚛️play-card.tsx`, `⚛️demonstrator-card.tsx`) are now thin wrappers of `OverviewCard`. The two landing `🟦️.tsx` files are unchanged, so all line ranges below are valid. |
| Incident | `🗑️generated` of this ticket was deleted once while I worked (screenshots and rig vanished). I restored report 1's screenshots from the tool cache and re-created the rig; the prototype sources are additionally kept **outside** `🗑️generated`: `layered_overview_prototype.tsx`, `layered_quiz_home_prototype.tsx` in the ticket folder. Screenshots: `🗑️generated/ui-reference/layered/` (list in §11). |

## 1. The layer stack (desktop, strip mode)

Both landings render the same five layers inside one root `div.relative.h-full.w-full.overflow-hidden.bg-background.text-foreground`
(`♻️…/🟦️.tsx:816-888`, `🎡️…/🟦️.tsx:783-845`). Measured on the real landing code at 1440×900:

| # | Layer | DOM / classes (verbatim) | Computed |
|---|---|---|---|
| 0 | **Backdrop strip** — every app pane, viewport-sized cells | `div.grid` with `gridTemplateColumns: repeat(C, 100vw)`, `gridTemplateRows: repeat(R, 100vh)` (play: `UI_AVAILABLE_HEIGHT`), `width: C*100vw`, `height: R*100vh` (play: `calc(R * var(--ui-available-height, 100dvh))`), `transform: translate(-Xvw, -Yvh)`; children = one pane `div.relative.h-full.w-full.overflow-hidden.bg-background[inert]` per app | demonstrator: strip **5760×1800 px** (4×2 cells of 1440×900), `z-index: auto`, `position: static`; play: `1300vw × 12·100dvh` (13×12 cells), 148 pane divs, **all `inert`** |
| 1 | **Glass veil** | `div.pointer-events-none.absolute.inset-0.z-30` > one `div.ui-veil.absolute` **per tint segment** with inline `top/left/width/height` in px | `z-index: 30`; `ui-veil` = `background: color-mix(in srgb, var(--surface-bg) 40%, transparent)` + `backdrop-filter: blur(0.5rem) saturate(1.45)` (`🖌️ui/🎨️.css:7056-7060`), measured light `rgba(247,243,227,0.4)`, `blur(8px) saturate(1.45)` |
| 2 | **Card grid** | `div.pointer-events-none.absolute.inset-0.z-[31].grid.items-center`, `gridTemplateColumns: repeat(C, minmax(0,1fr))`, `gridTemplateRows: repeat(R, minmax(0,1fr))`; per cell `div.flex.justify-center.px-double` (play: `min-w-0 px-single` + explicit `gridColumn/gridRow`, `pb-double pt-[calc(var(--size-workbench)*1.5)]`, `role="navigation" aria-label`) containing the card (`pointer-events-auto`) | `z-index: 31` (the arbitrary `z-[31]`, `z-[40]` utilities **are** generated in this build; measured 30/31/40) |
| 3 | **Chrome** | footer credits `div.pointer-events-none.absolute.inset-x-0.bottom-0.z-40` (demonstrator only), `Navbar` with `className="pointer-events-none absolute inset-x-0 top-0 z-40 bg-transparent"`, `UIIntroduction` (own `z-tutorial` = 10000 fixed veil + dialog), both `{!hoveredPaneId && …}` (intro not gated) | see §7: the `Navbar` does **not** end up where its class says |
| 4 | **Overview button** (only while a pane is opened) | `button.ui-glass.absolute.right-double.top-double.z-40 … Icon layout-grid + "Overview"/"Übersicht"` | replaces layers 1-3 |

The overview is shown only `!focusedId`; opening a pane removes layers 1-3 and un-`inert`s that pane. Cards are `pointer-events-auto` inside a
`pointer-events-none` overlay, so the pointer reaches the veil area only *between* cards — and that is what drives the pan (next section).

## 2. Strip geometry and how a card maps to its backdrop pane

**One card ↔ one pane, same cell.** Pane *i* sits on strip cell `(column, row)`; card *i* sits on the *same* `(column, row)` of the overlay grid.
Demonstrator: `column = i % 4`, `row = floor(i / 4)` (`♻️…:52-58`). Play: `playGridDimensions(148)` = **13×12** (fewest empty cells among near-square shapes with `columns - rows ≤ 2`, `🪧️brand.ts:99-118`), row-major with the short last row centred (`playGridRowSpan/playPaneGridCell`, `:120-136`; 148 = 11·13 + 5, so the last 5 cards sit in columns 4-8). The strip is therefore a **panorama of viewport-sized pages**; the overlay is a **contact sheet of the same layout squeezed into one viewport**. A card's own pixel position says nothing about where its pane is; only the `(column,row)` index does.

**Offsets are in viewport units** (`vw` horizontally, `vh` vertically): `ScrollOffset {x,y}`; page N of the strip is at `x = column*100`, `y = row*100`; `PLAY_MAX_SCROLL = ((C-1)*100, (R-1)*100)` (`🎡️:109-110`). Initial offset `(0,0)` (pane 0 fills the screen), or the hash pane's offset.

**Pointer pan** (only while nothing is hovered or opened, not on touch): `mousemove` on `window` sets the target to
`(clientX / viewportWidth) * MAX_SCROLL.x, (clientY / viewportHeight) * MAX_SCROLL.y` (demonstrator `:636-651`); play additionally clamps the target to *occupied* columns of the row band it is in (`clampScrollOffset` + `playOccupiedColumnRange`, `🎡️:112-118`, `🪧️brand.ts:138-145`) and reads `visualViewport`. Consequence: with the pointer in the top-left corner the backdrop is pane (0,0); in the bottom-right it is the last pane; in between it is a continuous, blurred panorama of neighbours (*measured* mid-pan screenshot). Because the card grid uses the same normalised layout, the pane under the veil at the pointer is approximately the pane of the card under the pointer — hover only has to finish the last fraction.

**Follow / glide loop** (one `requestAnimationFrame` chain per landing, started on demand, stops when settled; `🎡️:625-675`, `♻️:653-707`):
- `follow` (default): `next = lerp(current, target, 0.12)` per frame; settles when both axes are within `0.01` vw/vh (`PLAY_SCROLL_FOLLOW_LERP`, `PLAY_SCROLL_FOLLOW_EPSILON`).
- `glide` (programmatic: hover, focus, open, hash): `t = clamp((performance.now() - startedAt) / 500)`, `next = lerp(from, to, easeInOutCubic(t))` (`PLAY_SCROLL_GLIDE_MS`, `easeInOutCubic`: `t<.5 ? 4t³ : 1-(-2t+2)³/2`); on `t>=1` back to `follow` with `target = to`.
- An **epoch** counter is bumped on every drive change so a stale follow frame cannot repaint over a glide (`scrollEpochRef`, `commitScrollOffset`). The transform is written by React state (`setScrollOffset`) — **one React render of the whole landing per frame**; there must be **no CSS `transition` on that transform** (the doc comment says so).
- Measured end states: hover *Aggregator* (index 2) → `translate(-200vw, 0vh)`; *Statik* (index 7, col 3 row 1) → `translate(-300vw, -100vh)`; play *flow* → `-300vw`, *draw* → `-600vw`, *puzzle3d* → `-1100vw`.

## 3. Hover, focus, leave, click — exactly what happens

State: `hoveredPaneId` (React), `hoveredPaneIdRef`, `revealRect` (px rect of the hovered pane's visible part, or `null`), plus the scroll refs.

**Veil geometry.** `tintSegments = playTintSegmentsPx(revealRect)` (`🎡️:181-196`, `♻️:142-158`): with `revealRect === null` → **one** rect = the whole viewport; else the viewport **minus** the reveal rect as up to **four** rects (top band, bottom band, left, right slice). Each rect is its own `div.ui-veil` (so up to four `backdrop-filter` layers). `revealRect = playPaneRevealRect(paneIndex, offset)`: per axis `start = max(0, cellIndex*100 - offset)`, `end = min(100, cellIndex*100 - offset + 100)` in %, converted to px with `visualViewport` (play) or `innerWidth/innerHeight` (demonstrator) (`:152-179` / `:114-140`). If the pane is not visible at all the rect is `0×0` and the veil stays whole.

**Mouse enter on a card** (`revealPane`, `🎡️:677-686`; inline in `♻️:862-868`):
1. `hoveredPaneIdRef = id`, `setHoveredPaneId(id)` → the card gets `lifted` (`-translate-y-0.5`), the **navbar and footer disappear** (`{!hoveredPaneId && …}`), pointer-pan is disabled (`onMove` returns early);
2. the pane is **booted/promoted** (`touch` / `promoteAndResume`; a suspended pane resumes);
3. `applyPaneScroll(index)` starts a **500 ms cubic glide** from the current offset to `scrollOffsetForPaneIndex(index)` (that pane exactly filling the viewport);
4. `refreshRevealRect(id, scrollOffsetForPaneIndex(index))` — computed from the **target** offset, so the reveal rect is the **entire viewport** and the veil disappears **immediately** (*measured*: `.ui-veil` count went 1 → 0 at the event, the strip still at its old offset). On the first animation frame `tick` recomputes it from the *current* offset each frame (`refreshRevealRect(hovered, next)`, `:636` / `:666`), so during the glide the veil is the viewport minus the sliding pane (*measured* mid-glide: veil rects `[0,0,1440,24]` + `[0,24,136,876]` = a 24 px top band and a 136 px left band around the pane that is still 24 px and 136 px short of filling the screen), and after the glide there is no veil at all (*measured* `veils = []`, `translate(-300vw,-100vh)`). At 60 fps the first frame's target-based hole is normally overwritten before it paints; it is still a latent one-frame flash.
5. **Result:** the cards stay (above), the pane of the hovered card is crisp behind them, the veil is gone. There is **no** transition on the veil (segments appear/disappear/resize per frame), 200 ms `transition-transform` on the card lift, 120 ms on the silhouette stroke colour (the generic `[data-window-silhouette]:hover > [data-window-silhouette-border] path { stroke: var(--border-emphasized-color); transition: stroke 120ms ease }` rule of `🖌️ui/🎨️.css`), 500 ms on the glide, `0.12` per-frame lerp for pointer follow.

**Mouse leave** (`concealPane`): only if the leaving card is still the hovered one → `hoveredPaneIdRef = null`, `setHoveredPaneId(null)`, `setRevealRect(null)` → the full veil returns **instantly** (*measured*: after a synthetic `mouseout`, one veil `[0,0,1440,900]`, footer back, strip **stays** at the pane's offset) and the pointer-follow resumes on the next `mousemove` (the backdrop then re-glides toward the pointer's position).

**Keyboard focus.** *Play*: `PlayCard` has `onFocus={() => revealPane(...)}` — focusing a card reveals it exactly like hover (*measured*: `focus()` → veil 0, strip `translate(0vw,-100vh)`), but **there is no `onBlur`**: after `blur()` the veil stayed removed (*measured* veils 0, navbar hidden) until a mouse enter/leave cycle. Tabbing through 148 cards triggers 148 glides. *Demonstrator*: no focus handler at all — keyboard users never see the reveal.

**Click / Enter on a card** → `focusPane(id)` (§5). **Reduced motion**: no handling in either landing (`prefers-reduced-motion` appears only in `ui-react` for border/ring animations, icons and the tutorial cursor; not for the glide, the lerp, the pan or the lift — the card lift got `motion-reduce:` variants in the new `OverviewCard` only).

## 4. Lazy boot, suspension, posters, placeholders

Common (`PlayPane` `🎡️:366-434`, `DemonstratorPane` `♻️:373-441`): a pane is `booted && !suspended` → live shell inside `PaneErrorBoundary`; else `booted && suspended && poster` → `<img src=poster alt="" aria-hidden class="h-full w-full object-cover">`; else the placeholder `div.flex.h-full.w-full.flex-col.items-center.justify-center.gap-double.bg-background.border-loading[role=status][aria-busy]` with a 40 %-opacity icon/logo and `CanvasSkeleton` (`h-full min-h-0 w-full max-w-4xl flex-1 p-double`). Every pane wrapper is `inert={!focused}` and marks itself **dirty** on `onPointerDownCapture/onKeyDownCapture` (dirty panes are never suspended: their document would be lost). The shell gets `suppressAutoIntroduction={!focused}` so only the opened app shows its own tour.

| | Play (`usePaneLifecycle`, `🎡️:199-342`; policy `🪧️brand.ts:170-190`) | Demonstrator (`useSequentialPaneBoot` `♻️:166-202`, `usePaneSuspension` `:204-347`) |
|---|---|---|
| Boot triggers | hash, hover/focus (`touch`), touch-list scroll, background **warm-boot queue** | hash, hover (`promoteAndResume`), touch-list scroll (current **and next**), background idle queue |
| Idle queue | grid order; first after **4 s** (`PLAY_WARM_BOOT_START_MS`), then every **35 s** (`PLAY_WARM_BOOT_INTERVAL_MS`); `schedulePlayIdle` = `setTimeout(delay)` then `requestIdleCallback(cb, {timeout: 1000})`; held while a pane is opened or the live budget is full (`playNextWarmBootPane`) | first after **1.5 s**, then every **35 s** (`DEMONSTRATOR_PANE_BOOT_INTERVAL_MS`); off when touch list or a pane is opened; a hover-booted pane leaves the queue |
| Live limit | **4 panes** (`PLAY_LIVE_PANE_BUDGET`): least-recently-touched *pristine* pane other than the opened one is released first (`playPanesOverBudget`) | none (up to 8) |
| Time-based release | pristine live pane untouched **2 min** (`PLAY_IDLE_SUSPEND_MS`), sweep every **5 s** | `DEMONSTRATOR_SUSPENSION_POLICY`: **30 s** offscreen while another pane is opened, **5 min** idle on the overview (the most recently opened pane is exempt), **60 s** with a hidden tab; sweep **5 s** |
| Poster | `capturePanePoster` (`:220-249`) | identical (`:227-259`) |

`capturePanePoster` composites **only the `<canvas>` elements** inside the pane into an offscreen 2D canvas and returns a PNG data URL, synchronously (a `preserveDrawingBuffer:false` GL back-buffer is cleared after the frame); it returns `null` when there is no canvas, so **a DOM-only page yields no poster** and falls back to the placeholder. Posters are large (measured 530–563 kB data URLs for a 1440×900 canvas with trivial content).

*Measured on the real landings* (fake shell logging mounts): **Demonstrator** — first pane mounted 5.3 s after load; hover-booted *Aggregator* at 32.3 s; idle queue booted *Koordinator* at **40.3 s** (= 5.3 + 35) and *Energie* at **75.3 s**; after clicking *Statik* at ~100 s the three panes older than 30 s unmounted together at 101.9 s and *Energie* (booted 75.3 s) at 107.0 s, each replaced by a poster; `Escape` returned to the overview and hovering *Generator* re-mounted it from its poster. **Play** — hovering `cad, flow, draw, raster, layout, shooting` in turn (1.5 s each): live set `[cad] → [cad, flow] → … → [flow, draw, raster, layout] → [draw, raster, layout, shooting]` — never more than 4, `cad` then `flow` released to posters. Play's cost when idle: **148 pane wrappers, 294 `[role=status]`, 147 `.border-loading` placeholders, 9,828 DOM nodes, 745 elements with `backdrop-filter`** (148 cards × 5 glass cells + veils), strip `1300vw × 12·100dvh`.

`PaneErrorBoundary` (`createUiErrorBoundary`, `🎡️:344-364`, `♻️:349-371`): a crashed pane shows `div[role=alert]` "… could not be loaded" and logs; the other panes and the landing chrome keep working.

## 5. Opening a pane full screen: hash routing, Escape, Overview

`paneIdFromLocationHash` (`🎡️:100-104`, `♻️:64-68`) reads `#<id>`; `initialFocusId` boots that pane first, sets `showIntroduction = false` and the initial offset to that pane (no glide) — *measured* `…/real-demonstrator.html#aggregator` → `translate(-200vw, 0vh)`, no cards, no veil, only *aggregator* mounted (at 1.8 s), the other 7 `inert`, `Übersicht` button present. `focusPane(id)` (`🎡️:507-525`): `touch/promoteAndResume(id)`, `setFocusedId`, `setShowIntroduction(false)`, clears hover, then glide (desktop) or `scrollListToPaneIndex` + lock (touch), and `history.replaceState(null, "", "#id")`. `returnToOverview` (`:527-538`): `setFocusedId(null)`, unlock/re-scroll the list, `replaceState` to the bare path. `hashchange` and `Escape` (only while a pane is opened) call the same functions. Opening removes the veil and card layers (cards unmount → **focus falls to `<body>`**; nothing restores it to the card on return — neither landing manages focus). *Measured*: click *Statik* → hash `#statik`, `Übersicht` button top right, only *statik* un-`inert`.

## 6. Touch / phone

`touchListMode = useMediaQuery("(max-width: 767px) and (hover: none) and (pointer: coarse)")` (`🎡️:43-44`, `♻️:44-45`; tablets keep the desktop strip). Layout (`🎡️:740-781`, `♻️:766-814`): a scroller `div.flex.w-full.flex-col.overscroll-y-contain` with `snap-y snap-mandatory overflow-y-auto` (or `overflow-hidden` while a pane is opened), height `UI_AVAILABLE_HEIGHT` (demonstrator `100dvh`), and per pane a `section.relative.w-full.shrink-0.snap-start.overflow-hidden` of the same height containing the pane, a **veil of its own** (`div.pointer-events-none.absolute.inset-0.z-30 > div.ui-veil.absolute.inset-0`; demonstrator adds `style={{opacity: 1}}` from `DEMONSTRATOR_MOBILE_OVERVIEW_VEIL_OPACITY`) and the card centred (`z-[31] flex items-center justify-center px-double pb-[5.5rem]`). There is **no hover on touch**: the veil is permanent and the backdrop stays blurred; tapping the card opens the page. Boot: demonstrator promotes the current and the next section (`:602-609`); play touches the current and warms the next only while the budget has a free slot (`:577-586`). *Measured* at 375×812: 8 sections of 812 px, all `ui-veil` at opacity 1, sections 0 and 1 live, `data-ui-device="mobile"`; the two footer credits collide (demonstrator). The intro dialog appears on first visit on top of everything.

## 7. Keyboard and accessibility as implemented

- Backdrop: all panes `inert` unless opened (removes focus and the accessibility tree); posters `alt="" aria-hidden`; placeholders `role="status" aria-busy aria-label="… is waiting to start"` (play only labelled). No landmark or heading in either landing; play's overlay is `role="navigation" aria-label="Every semio app"`, demonstrator's has no role; cards are `<button>`s (play: `aria-label="Open {label}: {tagline}"`).
- Missing: focus reveal in the demonstrator, blur-conceal in play, focus management on open/close, reduced motion, a label on the `nav` of `Navbar`.
- **Defect (measured on the real landing code):** `<nav data-slot="navbar" class="… relative h-large … absolute inset-x-0 top-0 z-40">` computes to `position: relative; z-index: 0` because `🖌️ui/🎨️.css` forces `[data-slot="navbar"] { position: relative !important; z-index: var(--z-base) !important }`; the brand bar therefore lies in flow after the strip at **y = 1800 px (demonstrator) / 10800 px (play)** and is clipped — invisible. Not related to layering itself, but a layered overview must place its chrome with its own wrapper, not with Navbar utility classes.

## 8. Performance safeguards and costs, as they are

Safeguards: lazy boot (hover/hash/idle), 35 s spacing so one 30 s plugin-load budget ends before the next starts, live budget 4 (play), time-based release (both), posters, `inert` backdrop, `pointer-events-none` overlays, a single rAF chain that stops when settled, hidden-tab release (demonstrator), `visualViewport` for mobile toolbars (play).
Costs: a **React render of the whole landing per animation frame** while panning/gliding (`setScrollOffset` in `commitScrollOffset`), up to four blurred veil rectangles, **745** backdrop-filter layers in play (every `WindowChrome` cap/body/footer cell is `ui-glass`), all 148 panes present as DOM (placeholders with animated borders), posters ~0.5 MB each as data URLs.

## 9. Everything duplicated between the two landings (exact ranges)

`♻️` = `♻️mit-bestand/🧺️demonstrator/🟦️.tsx` (892 lines), `🎡️` = `🏢️semio-tech/🎡️play/🟦️.tsx` (849 lines). "Same" = identical up to the pane array / constant prefix.

| Block | 🎡️ | ♻️ | Difference |
|---|---|---|---|
| touch-list media query | 43-44 | 44-45 | same |
| `paneIndexById`, `paneIdFromLocationHash` | 96-104 | 60-68 | same |
| `paneColumn/paneRow` | 88-94 | 52-58 | play reads a centred-row cell table; demonstrator `%`/`floor` |
| `ScrollOffset`, `MAX_SCROLL` | 106-110 | 70-74 | same |
| `clampScrollOffset` | 112-118 | — | play only |
| glide/lerp/epsilon constants, `ScrollDrive`, `easeInOutCubic`, `scrollOffsetForPaneIndex`, `lerpScrollOffset` | 120-150 | 76-112 | same (500 ms, 0.12, 0.01) |
| `PaneAxisBounds`, `paneAxisBounds` | 152-160 | 114-122 | same |
| rect types; viewport px | 162-168 | 124-126 | play adds `playViewportPx()` (`visualViewport`, `availableViewportHeightPx`) |
| reveal rect | 170-179 | 128-140 | same algorithm |
| tint segments | 181-196 | 142-158 | same algorithm |
| `capturePanePoster` | 220-249 | 227-259 | same |
| lifecycle (boot, release) | 199-218, 251-341 | 166-225, 261-346 | **different policies** (§4) |
| `PaneErrorBoundary` | 344-364 | 349-371 | play adds a `failedLabel` prop |
| pane component | 366-434 | 373-441 | play adds `style`, `data-play-pane`, labels |
| landing state + refs | 437-469 | 444-488 | |
| `commitScrollOffset` | 471-477 | 489-495 | same |
| `applyPaneScroll` | 479-496 | 497-521 | same |
| `scrollListToPaneIndex` | 498-505 | 523-530 | same |
| `focusPane` / `returnToOverview` | 507-538 | 532-563 | same (touch vs promote) |
| `hashchange`, `Escape`, initial list scroll effects | 540-564 | 565-589 | same |
| `handleListScroll` | 566-575 | 591-600 | same |
| touch-list warm effect | 577-586 | 602-609 | budget vs current+next |
| `refreshRevealRect`, tint memo | 588-593 | 611-624 | same |
| resize effect | 595-608 | 626-634 | play adds `visualViewport` |
| pointer-follow effect | 610-623 | 636-651 | play clamps + `playViewportPx` |
| **rAF follow/glide loop** | 625-675 | 653-707 | same |
| `revealPane` / `concealPane` | 677-693 | 858-877 (inline) | same |
| chrome (navbar, footer, intro) | 695-725 | 713-753 | demonstrator adds footer credits |
| overview button | 727-738 | 755-764 | same classes |
| touch-list JSX | 740-781 | 766-814 | demonstrator adds the veil opacity |
| strip JSX | 783-845 | 816-888 | play: `role`, cells, `pt`; demonstrator: fixed 4×2 |
| idle scheduler | `🪧️brand.ts:148-168` | `🪧️brand.ts:865-884` | same, renamed |
| card | `⚛️play-card.tsx` | `⚛️demonstrator-card.tsx` | already collapsed into `OverviewCard` (staged) |

Of 849 / 892 lines, roughly **620 / 660 are the same code**; what is genuinely different: the grid dimension math (play `🪧️brand.ts:99-145`), the lifecycle policies, the labels, the footer credits and the card content.

## 10. Recommendation: `LayeredOverview`, one shared element

### 10.1 Place and shape

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx` (React element) + pure geometry and policy in `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts` (no DOM), exported from the slim `@semio-tech/ui-react/chrome` entry (the quiz must not import the barrel) and re-exported by the barrel. It composes `OverviewCard`/`WindowChrome`; it contains **no** `FrameworkOsShell`, catalog, brand or i18n of a product. The element fills its positioned parent, so a page can put a header above it (quiz) or use the whole viewport (play).

### 10.2 Props API (exact)

```ts
/** 📍️ Zero-based strip cell of a pane. */
export interface LayeredCell { readonly column: number; readonly row: number }

/** 🥞️ One backdrop page: the real page a card opens. */
export interface LayeredPane {
  readonly id: string;                         // hash route, data-layered-pane, stable key
  readonly label: string;                      // accessible name of the opened page and of its placeholder
  readonly icon?: IconName;                    // placeholder glyph (40 % opacity)
  readonly render: (state: LayeredPaneState) => ReactNode;   // called ONLY while mounted (booted and not suspended)
  readonly poster?: string | null;             // still shown while not mounted (optional)
  readonly capturePoster?: (container: HTMLElement) => string | null;  // default: none; play/demonstrator pass `capturePosterFromCanvases`
}
export interface LayeredPaneState {
  readonly opened: boolean;                    // full-size and interactive
  readonly revealed: boolean;                  // its card is hovered/focused: shown clear
  readonly dirty: () => void;                  // page reports user work → never released (also wired to pointer/key capture)
}

export interface LayeredCardState {
  readonly mode: "strip" | "list";
  readonly revealed: boolean;                  // → OverviewCard `lifted` / data-revealed
  readonly opened: boolean;
  readonly open: () => void;                   // opens the pane (hash + glide)
}

export interface LayeredLifecycle {            // all ms
  readonly budget: number;                     // most panes mounted at once (play 4, demonstrator 8, quiz 9)
  readonly warmStartMs: number;                // play 4_000, demonstrator 1_500, quiz 600
  readonly warmIntervalMs: number;             // play/demonstrator 35_000, quiz 250
  readonly suspendIdleMs: number;              // play 120_000, demonstrator 300_000, quiz Infinity
  readonly suspendOffscreenMs: number;         // demonstrator 30_000 while a pane is opened, play = idle
  readonly suspendHiddenMs: number;            // demonstrator 60_000, play = idle
  readonly sweepMs: number;                    // 5_000
}

export interface LayeredOverviewProps {
  readonly panes: readonly LayeredPane[];
  readonly cells: Readonly<Record<string, LayeredCell>>;      // strip cell per pane id, for the CURRENT breakpoint
  readonly renderCard: (pane: LayeredPane, state: LayeredCardState) => ReactNode;  // an `OverviewCard`; the host wires reveal
  readonly overlayClassName?: string;          // the card grid is APP-owned CSS (responsive); host adds absolute inset-0 z-[31]
  readonly overlayStyle?: CSSProperties;
  readonly renderChrome?: (state: { readonly revealed: boolean; readonly opened: boolean }) => ReactNode;  // navbar/footer/intro above z-40
  readonly mode?: "strip" | "list";            // list = one snap section per pane (touch phones); default strip
  readonly pan?: "pointer" | "none";           // pointer-follow panorama (default pointer; off under reduced motion)
  readonly routing?: "hash" | "none";
  readonly openedId?: string | null;           // controlled
  readonly onOpenedIdChange?: (id: string | null) => void;
  readonly onRevealedIdChange?: (id: string | null) => void;
  readonly lifecycle?: Partial<LayeredLifecycle>;
  readonly windowing?: { readonly radius: number };   // mount pane CONTENT only within `radius` cells of the current offset (default 1)
  readonly reducedMotion?: "auto" | "always" | "never";
  readonly labels: { readonly grid: string; readonly overview: string; readonly waiting: (pane: LayeredPane) => string; readonly failed: (pane: LayeredPane) => string };
}
```
Helpers exported next to it (pure, in the geometry module, tested): `nearSquareGrid(count)` and `centeredLastRowCells(count)` (= play `🪧️brand.ts:99-145`, generic), `veilClip(cell, offset)`, `cellOffset(cell)`, `paneAxisBounds`, `easeInOutCubic`, `lerpOffset`, `panesOverBudget`, `nextWarmBoot`, `scheduleIdle`, `capturePosterFromCanvases`.

### 10.3 Behavioural specification (each item is a testable case)

1. **Structure.** `div[data-layered-overview]` (`relative h-full w-full overflow-hidden`) > strip `div[data-layered-strip]` (`grid will-change-transform`, `width: C*100%`, `height: R*100%`, `repeat(C,1fr)/repeat(R,1fr)`; percentages of the container, so no `vw/vh` and it fills any parent) > panes; veil layer `z-30`; card overlay `z-[31]`; chrome `z-40`. Unopened panes: `inert`, `aria-hidden`; opened: `role="region" aria-label`.
2. **Offsets** are in cell units; `translate3d(-x*100/C %, -y*100/R %, 0)`; written **imperatively** on the strip (no React render per frame). Follow: lerp `0.12`, epsilon `1e-4` cells; glide: **500 ms**, `easeInOutCubic`; epoch guard; reduced motion → no follow, no pointer pan, glide = snap.
3. **Veil** = **one** `div.ui-veil` whose hole is a `clip-path: polygon(evenodd, outer rect, hole rect)` in **percent** of the container, recomputed from the *current* offset at reveal start and per frame during a glide (`veilClip`): pane not visible → no hole; pane covers the container → `visibility:hidden`; else the polygon. One blurred layer instead of four; no `px`, no `visualViewport` read.
4. **Reveal.** `pointerenter` (mouse only) or `focus` on a card wrapper → boot/resume the pane, glide to its cell, punch the hole, set `state.revealed` (card lifts); `pointerleave` (mouse) or `blur` with `relatedTarget` outside the card → veil whole again, pointer pan resumes. Chrome hides while revealed if the app's `renderChrome` says so.
5. **Open.** Card `open()` / hash `#id` / hashchange → `openedId`, cards+veil unmount, the pane un-`inert`s, **focus moves to the pane** (`tabIndex=-1` region) and **returns to the card's heading link on close**; `Escape` and the `Overview` button (`ui-glass absolute right-double top-double z-40`) close; `history.replaceState` only (no history entries); deep link boots that pane first, no glide, no intro.
6. **Lifecycle.** Warm queue in cell order after `warmStartMs`, one pane per `warmIntervalMs` via `scheduleIdle`; never while a pane is opened; hover/focus boots immediately; `budget` releases the least-recently-touched pristine pane; time-based release per the three thresholds; `dirty` panes and the opened/revealed pane are never released; poster (if `capturePoster` yields one) else placeholder (`role=status`, `CanvasSkeleton`); `windowing.radius` limits how many pane bodies/placeholders exist at once (play would go from 148 placeholder subtrees / 9.8k nodes to ≲ 9 near the viewport).
7. **List mode.** `section` per pane (`snap-start`, container height), each with its own veil (`opacity 1`) and the card centred (`pb` for the browser toolbar); current + next pane warmed; opening locks the scroller; no reveal on touch.
8. **Errors.** Per-pane boundary → `role="alert"` `labels.failed(pane)`.
9. **Hooks for tests/styles:** `data-layered-pane|card|veil|strip|overview-button`, `data-revealed`, `data-opened` (replace `data-play-pane`, `data-demonstrator-pane-card`, `data-play-overview`).

### 10.4 Who uses it, how

- **Play**: `panes` = 148 with `render = ({opened}) => <FrameworkOsShell … suppressAutoIntroduction={!opened} />` inside its `resolvePlaygroundBoot` memo; `cells = centeredLastRowCells(148)` (13×12); `renderCard = (p, s) => <PlayCard pane lifted={s.revealed} onClick={s.open}/>` (`OverviewCard as="button"`, its `onMouseEnter/onMouseLeave/onFocus` props disappear); overlay `grid items-center pb-double pt-[calc(var(--size-workbench)*1.5)]`; `lifecycle = {budget: 4, warmStartMs: 4_000, warmIntervalMs: 35_000, suspendIdleMs: 120_000}`; poster = `capturePosterFromCanvases`. Deletes `🎡️:80-197` (keep only `playGridDimensions` etc. moved to the helper), `:199-342`, `:344-434`, `:437-693`, `:740-845`; the file shrinks to ≈ 120 lines (labels, brand→panes, chrome, intro).
- **Demonstrator**: same with `cells` 4×2, `lifecycle = {budget: 8, warmStartMs: 1_500, warmIntervalMs: 35_000, suspendIdleMs: 300_000, suspendOffscreenMs: 30_000, suspendHiddenMs: 60_000}`, footer credits in `renderChrome`.
- **Quiz home** (nine cards, prototype `layered_quiz_home_prototype.tsx`): panes `learner, physics, intro, heating, board, cooling, badges, demand, prefs` in that (row-major = DOM) order; each pane's `render` is the **real page of that card**: `learner` = the learner's profile and run history, `physics|heating|cooling|demand` = the quiz's own page (title, description, its tasks, best score, Start/Resume/Play again), `intro` = the full introduction, `board` = the full sortable leaderboard (today's `LeaderboardScreen`), `badges` = every badge with description and date, `prefs` = language/theme/text size. Cards are `OverviewCard as="section"` with real buttons in the footer chips (`OverviewCardAction`), the heading is a link `<a href="#physics">` (keyboard/AT way to open), plus a pointer `onClick` on the section for mouse users (ignoring clicks from `button, a`). `cells`: desktop 3×3 ring (learner 0,0 · physics 1,0 · intro 2,0 · heating 0,1 · **board 1,1** · cooling 2,1 · badges 0,2 · demand 1,2 · prefs 2,2), tablet 2 columns × 5 rows (board on row 2 spanning both columns in the card grid), phone `mode="list"`. The overlay grid is the app's own CSS:
  ```css
  .quiz-home-grid { display: grid; gap: var(--spacing-double); padding: var(--spacing-double); grid-template-columns: minmax(0,1fr); align-content: start; align-items: center; }
  @media (min-width: 768px)  { .quiz-home-grid { grid-template-columns: repeat(2, minmax(0,1fr)); } .quiz-home-grid [data-card="board"] { grid-column: 1 / -1; } }
  @media (min-width: 1024px) { .quiz-home-grid { align-content: stretch; grid-template-columns: minmax(0,1fr) minmax(0,1.5fr) minmax(0,1fr); grid-template-rows: minmax(0,1fr) minmax(0,1.4fr) minmax(0,1fr); } .quiz-home-grid [data-card="board"] { grid-column: auto; } }
  ```
  (descendant selector, because the host wraps each card in a `display: contents` element.) `lifecycle = {budget: 9, warmStartMs: 600, warmIntervalMs: 250}` — the pages are cheap DOM, so nothing is ever suspended in practice.

### 10.5 Quiz-specific rules that fall out of this design

- **A card's backdrop pane must be a pure view over session state.** Never mount `RunScreen` as a backdrop (`startRun` is a command that creates a run); the quiz needs a read-only *quiz page* (`screen: "quiz"`), which is what Start/Resume/Play again lead from. Leaderboard polling (`usePolling`, 10 s) runs only while the pane is mounted **and** (`opened || revealed`); a backdrop shows the last snapshot. No `role="status"`/live regions inside backdrop panes (they are `inert` + `aria-hidden` anyway).
- **Cards must be compact and centred in their cells (`align-items: center`)**, as in play/demonstrator. My first cut stretched the cards to fill the 3×3 cells (`proto-1440-rest-light-first-cut.jpg`) and the cleared page was visible only in slivers between the cards (`proto-1440-hover-first-cut-slivers.jpg`); with content-height cards (`proto-1440-rest-light.jpg`, `proto-1440-hover-leaderboard-clear.jpg`) about half of the page shows around them.
- The page behind must be laid out for the **container** (header excluded), be responsive and use the same root-font-size text scale; `hash` routes: `#learner`, `#physics` … `#prefs`.
- Keep the header (`Navbar` in flow above the element) always visible; use `renderChrome` only for what should hide on reveal.
- Glass count: 9 cards ≈ **32** `backdrop-filter` elements + 1 veil (measured in the prototype: 704 DOM nodes) versus 745 in play — fine.

### 10.6 Accessibility of the element (beyond what play/demonstrator do)

Backdrop `inert` + `aria-hidden` (as now); overlay `role="group"` with `aria-label` (not `navigation`: quiz cards are regions, not links to pages only); **reveal on focus and conceal on blur** (play conceals only on mouse leave); opened pane is a labelled region and receives focus; focus returns to the card on close; no CSS `order`/grid areas so DOM order = reading order = tab order; reduced motion (§10.3.2) plus `motion-reduce:` on the card lift (already in `OverviewCard`); `prefers-reduced-transparency` and `forced-colors` fallbacks of `ui-veil` already exist; 24 px minimum chip height (`OverviewCardAction`); hover is an enhancement only — touch gets the permanent veil and keyboard gets focus reveal, so no information is available only on hover.

### 10.7 Performance improvements built into the element (each measured or derived above)

Imperative transform and veil (no per-frame React render); one veil layer with `clip-path` (was up to four); percent geometry (no `visualViewport`/resize listeners); `windowing` (play: 9.8k → ≲1k DOM nodes, 147 animated placeholders → ≲9); `contain: layout paint` and `will-change: transform` on strip/panes; posters only when a poster function exists (DOM pages need none); `scheduleIdle` for warm boots; pointer pan is `passive` and rAF-driven only while unsettled.

### 10.8 Tests (language-agnostic first, per AGENTS)

Fixture `🧫️fixtures/layered-overview/🔣️.json` with input/expected pairs read by every implementation: `nearSquareGrid(148) = 13×12`, `centeredLastRowCells(148)` (last row columns 4-8), `veilClip` for (pane not visible / partial / full / corner overlap) as polygon vertex lists, `easeInOutCubic` samples, `panesOverBudget`, `nextWarmBoot` (focused → hold, budget → hold, complete → hold, else boot), glide sampled at 0/250/500 ms. Third-party oracle for the polygons: a boolean-difference library (`polygon-clipping`) comparing the area of `viewport − hole` with the area of our `evenodd` polygon. DOM cases (vitest + Testing Library + `axe`): nine `section[aria-labelledby]` in order, hover/focus reveal and blur conceal (veil `visibility`/`clip-path`), `inert`/`aria-hidden` of unopened panes, hash deep link (only that pane mounted, no cards), Escape closes and focus returns to the card heading, reduced motion snaps, budget releases the least recently touched pane (`live ≤ 4`), a DOM-only pane never gets a poster.

### 10.9 Defects found in the landings that the extraction fixes (list for the tickets)

1. Brand `Navbar` renders off-screen (`position: relative !important` beats `absolute`), §7. 2. Play: focus reveal never concealed. 3. Demonstrator: no keyboard reveal. 4. Focus lost on open, not restored on close. 5. Hole computed from the target offset at hover start (latent flash). 6. Whole landing re-rendered every frame. 7. Four veil rectangles, 745 blur layers, 148 placeholders mounted. 8. No reduced-motion handling. 9. Posters only for canvas apps. 10. The veil hosts carry no `data-level` although `🖌️ui/🎨️.css:7037-7044` says `ui-veil`'s host must (the landings render the base-level tint, `rgba(247,243,227,0.4)`, which is what they look like today).

## 11. Screenshots (`C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️28\QUIZ-PRODUCT-AND-TEACHING-PROCTOR\🗑️generated\ui-reference\layered\`)

| File | What it shows |
|---|---|
| `real-demonstrator-1440-first-visit-intro.jpg` | first visit: `UIIntroduction` dialog (own fixed veil, heavier blur) over the whole landing |
| `real-demonstrator-1440-rest-veil.jpg` | overview at rest: pane 0 (the live fake *generator* page, big green title, canvas rays) blurred behind the veil, 4×2 cards above, footer credits |
| `real-demonstrator-1440-pointer-pan.jpg` | pointer between cards: strip mid-pan, backdrop = panorama (the booted *aggregator* page and neighbours) |
| `real-demonstrator-1440-hover-aggregator-clear.jpg` | hover *Aggregator*: veil gone, the *aggregator* page (blue, big title, canvas rays) crisp behind the still-visible cards, footer/navbar hidden, hovered card lifted with emphasised outline |
| `real-demonstrator-1440-hover-statik-clear.jpg` | same for *Statik* (bottom-right cell: strip at `-300vw,-100vh`) |
| `real-demonstrator-1440-opened-statik.jpg` | after click: hash `#statik`, cards/veil gone, page full size, only the `Übersicht` glass button top right |
| `real-demonstrator-375-touch-list.jpg` | phone touch list: one section per app, permanent full veil, live fake page blurred behind the centred card, colliding footer credits |
| `real-play-1440-hover-puzzle3d.jpg` | play hover: veil removed but 148 blurred cards cover most of the page; only gaps show it |
| `proto-1440-rest-light-first-cut.jpg`, `proto-1440-hover-first-cut-slivers.jpg` | prototype, cards stretched to the cells (rejected: after hover the cleared page shows only in slivers, mid-glide frame) |
| `proto-1440-rest-light.jpg` | **prototype quiz home at rest**: 3×3 compact cards, leaderboard centre, learner page blurred behind the veil |
| `proto-1440-hover-leaderboard-clear.jpg` | hover the centre card: the full sortable leaderboard page (10 rows, per-quiz columns, own row highlighted) crisp behind the cards |
| `proto-1440-opened-heating.jpg` | opened page `#heating` with the `Overview` button; the header stays |
| `proto-768-rest.jpg`, `proto-768-hover-leaderboard-clear.jpg` | tablet: 2-column card grid, board spanning; hover clears the leaderboard page (strip `translate3d(0%, -40%)`, 2×5 cells) |
| `proto-375-list-leaderboard.jpg` | phone list mode, leaderboard section: card centred over the blurred full leaderboard page |
| `proto-1440-dark-rest.jpg`, `proto-1440-dark-hover-clear.jpg` | dark theme, rest and hover |

Prototype files: `layered_overview_prototype.tsx` (the element: contracts, `veilClip`, imperative follow/glide loop, reveal/conceal with focus/blur, hash routing, warm queue and budget, list mode, per-pane boundary; it imports `Icon`, `CanvasSkeleton`, `loadingBorderClass`, `cn` from `@semio-tech/ui-react`, to be replaced by the chrome entry plus a local placeholder) and `layered_quiz_home_prototype.tsx` (the nine cards and nine real-looking pages, `Navbar` in flow above). Measured on the prototype: strip `translate3d(-33.3333%, -33.3333%, 0)` for the centre pane with the veil `visibility: hidden`; mouse leave → veil `visible`, `clip-path: none`; keyboard `focus()` on the badges heading link → strip `translate3d(0%, -66.6667%, 0)` and veil hidden, `blur()` → veil back; opened `#heating` → only that pane un-`inert` with `role="region" aria-label="Heating"`, no cards, no veil; 704 DOM nodes, 1 veil, 32 backdrop-filter elements.
