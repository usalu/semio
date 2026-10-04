# 📓️ UI reference: `🎡️play` and `🧺️demonstrator` — and how the quiz home should match

Scope: a precise, implementable design reference for the quiz website (`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react` renderer + site
`🎓️teaching/🏛️architecture/❓️quiz`) so that it looks and feels like `🏢️semio-tech/🎡️play` and `♻️mit-bestand/🧺️demonstrator`: a grid of
cards, one card per quiz, the leaderboard as the central card. Read-only exploration; the only files written are this report and
the ticket-local rig and screenshots under `🗑️generated/ui-reference/`.

## 0. How this was verified (and what could not be)

| Fact | Detail |
|---|---|
| Live dev servers | **Not runnable on this host.** `preview_start mit-bestand-demonstrator` (`.claude/launch.json`, port 6029) pulled 102 Nx tasks; `os/flow/core` `wasm-pack build` died with `STATUS_NO_MEMORY` (0xc0000017), i.e. the known rustc OOM of this machine. There is no `semio-tech-play` row in `.claude/launch.json` (only in `.vscode/launch.json`, port 6033). The Nx daemon that run started held 4 GB; I stopped it (`nx daemon --stop`). Working tree untouched (`git status` shows only the pre-existing ticket.json change and another agent's `📓️explore-cdn-docker-deploy.md`). |
| Substitute | A **rig** (`🗑️generated/ui-reference/rig/`, Vite on port 6099, now stopped) mounts the **real** `PlayCard` (`🏢️semio-tech/🎡️play/⚛️play-card.tsx`), the **real** `DemonstratorCard` + `⚛️footer.tsx`, the real `🎨️globals.css` of each app, the real `Navbar`/`ShellBrandLogo`/`WindowChrome` of `@semio-tech/ui-react`, the real brand data (`DEMONSTRATOR_PANES`, play catalog JSON) and the overlay JSX copied verbatim from each `🟦️.tsx`. The backdrop behind the veil is the landing's own *unbooted-pane placeholder* (logo/icon at 40 % + `CanvasSkeleton`), **not** live apps. Everything about cards, grid, veil, tokens, fonts and reflow is real; the blurred picture behind the cards is not. |
| Play brand module | `🎡️play/🪧️brand.ts` throws `Unknown play pane variant: stdio-txt` in the browser here: the generated registry (`🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts`) is stale relative to `🔨️modules/🧩️runtime/🔣️.json`. The rig therefore reads the catalog JSON and copies the three pure grid functions. Not a UI issue, but `bun nx run @semio-tech/semio-tech-play:prepare-dev` would fail the same way until the registry is regenerated. |
| Viewports | 1440×900 (browser pane scales the screenshot to 800×500), 768×1024 and 375×812 through `resize_window`; light and dark through `colorScheme`. Viewport reset to desktop, all servers stopped. |

Rig usage (for the implementer): `bun node_modules/vite/bin/vite.js --config "<ticket>/🗑️generated/ui-reference/rig/vite.config.ts"` then
`/demonstrator.html`, `/play.html`, `/quiz-mock.html` (query `?appearance=dark|light`, `?mode=list|grid` forces the touch list or the grid).
`rig/quiz-mock.tsx` + `quiz-mock.css` are a working **prototype of the recommended quiz home** (section 6) built from the real components.

## 1. What both apps are

Both landing pages are the same pattern with different sizes:

1. `#root` is one full-viewport React tree (`bg-background text-foreground`, `overflow-hidden`).
2. Behind: a **grid strip of live app panes** (`FrameworkOsShell` per pane) of `columns * 100vw` × `rows * 100vh`, panned with the mouse.
3. Above: a full-viewport **veil** (`ui-veil`: `rgba(base, 0.4)` + `backdrop-filter: blur(0.5rem) saturate(1.45)`), with a rectangular hole over the hovered pane.
4. Above the veil: an **overlay grid** `repeat(cols, minmax(0,1fr)) × repeat(rows, minmax(0,1fr))` that holds **one card per pane**.
5. Chrome: a `Navbar` brand chip, an `UIIntroduction` dialog on first visit, and (demonstrator only) partner/funding credits at the bottom.
6. Touch (`(max-width: 767px) and (hover: none) and (pointer: coarse)`): the strip is replaced by a vertical **scroll-snap list**, one pane-section per app of `100dvh`, each with its veil and a centred card.

|  | `🏢️semio-tech/🎡️play` | `♻️mit-bestand/🧺️demonstrator` |
|---|---|---|
| Cards | **148** (catalog `🔨️modules/🧩️runtime/🔣️.json`, 18 groups) | **8** (`DEMONSTRATOR_PANES` in `🪧️brand.ts`) |
| Overview grid | `playGridDimensions(148)` = **13 × 12**, short last row centred (`playPaneGridCell`) | fixed **4 × 2** (`DEMONSTRATOR_GRID_COLUMNS/ROWS`) |
| Card | `PlayCard`, compact: icon + label chip, one-line truncated tagline, `Open ›` chip, `max-w-xs` | `DemonstratorCard`, roomy: icon + label chip, bold tagline, 2 description paragraphs, `Demonstrator öffnen ›` chip, `max-w-sm` |
| Locale | English lock, labels registered EN + DE (`registerUiTranslationBundles`) | German lock, German strings inline |
| Footer | none | `aProjectOfLuhUdkFooterItem` (left) + `fundedByZukunftBauFooterItem` (right) |
| Height | `UI_AVAILABLE_HEIGHT` (`var(--ui-available-height, 100dvh)`) | `100vh` / `100dvh` |
| Hash routing | `#<variant>` focuses a pane, Escape / `Overview` returns | same |

The **8-card demonstrator is the relevant template** for the quiz (4 quizzes + 4 more cards = the 8 ring cells of a 3×3 with the leaderboard in the centre). Play shows how the same card scales down to a dense index.

## 2. Shared building blocks (what both really share)

There is **no shared grid component and no shared card component**. `PlayCard` and `DemonstratorCard` are near-duplicates; both landings also duplicate
`capturePanePoster`, the idle scheduler, `PaneErrorBoundary`, hash routing and the scroll-glide code. The grid is written inline twice.

What they do share, all from the package `@semio-tech/ui-react`
(entry `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx` → `🎯️targets/⚛️react/🟦️.tsx` + `🧱️elements/*`; the Vite configs alias it, it is not a `package.json` dependency of either app):

| Export | Defined | Role |
|---|---|---|
| `WindowChrome`, `windowChromeTitleChipClass` | `🎯️targets/⚛️react/🟦️.tsx:6029` and `:5893` | **the card**: U-cutout "window silhouette": title-chip cap top-left, body, footer chips bottom-left/centre/right, 1 px SVG outline traced round chips and body |
| `Navbar`, `ShellBrandLogo`, `navbarFillItem` | `🧱️elements/🔝️Navbar/🟦️.tsx` | brand bar (`<nav id="ui.navbar" data-slot="navbar" class="relative h-large …">`) |
| `Icon` (+ `IconName`) | `🧱️elements/🔣️Icons` + `@semio-tech/assets` | monochrome catalog icons |
| `UIIntroduction` | `🎯️targets/⚛️react/🟦️.tsx` | first-visit dialog from an `IntroductionDefinition` (paragraph hover emphasis, logos) |
| `CanvasSkeleton`, `loadingBorderClass` | `🧱️elements/🦴️Skeletons`, `🔨️modules/🌀️status-border-presentation` | loading placeholder of an unbooted pane |
| `cn`, `useMediaQuery`, `UI_MOBILE_MEDIA_QUERY` (`(max-width: 767px)`), `UI_TABLET_MEDIA_QUERY` (768–1023), `UI_AVAILABLE_HEIGHT` | `🖱️ui/📱️device/🟦️.ts`, `🎯️targets/⚛️react/🟦️.tsx:2469` | breakpoints and viewport height |
| `bootstrapElementsSurfaceChromeDocument`, `useElementsSurfaceChrome`, `readStoredUiChromeAppearance/Layout/Driver`, `initUiLocaleSync`, `registerUiTranslationBundles`, `useLabel`, `uiDataLabel` | `🎯️targets/⚛️react/🟦️.tsx:1898-2075` and i18n | `.dark` class + `data-ui-device` on `<html>`, locale lock, label registry |
| `@semio-tech/ui-react/runtime` (`mountUiRoot`, `useUiState/Memo/Callback/Effect/Ref`, `createUiErrorBoundary`) | `🎯️targets/⚛️react/🎠️runtime/🟦️.ts` | a **thin owned passthrough over plain `react`/`react-dom`** (same React instance), so plain React hooks and `createRoot` interoperate |

CSS chain (identical for both apps; `🎨️globals.css` is 1 import + 1 `@source` + card rules):

```css
/* 🏢️semio-tech/🎡️play/🎨️globals.css  (♻️…/🧺️demonstrator/🎨️globals.css is the same with demonstrator-* selectors) */
@import "../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎨️.css";  /* → 🖱️ui/🧵️styles/🎨️.css → 🎨️styling/🖌️ui/🎨️.css (tailwind, palette, tokens, levels, utilities) + 🌐️globals/🎨️.css */
@source ".";
[data-slot="play-pane-card-stack"] { background: transparent !important; background-color: transparent !important; }
[data-play-pane-card]:hover [data-window-silhouette-border][data-kind="normal"]:not([data-pending]) path { stroke: var(--border-emphasized-color); transition: stroke 120ms ease; }
/* demonstrator adds: paragraph hover → color: var(--border-emphasized-color); title chip emphasised when body/gap/title hovered */
```

`🌐️.html` of both: `<body class="h-screen w-screen overflow-hidden bg-background text-foreground"><div class="h-full w-full" id="root">`, `html, body, #root { height: …; margin: 0 }`.
The Vite configs use `semioHostHtmlVitePlugin(repoRoot, { title, entry, bodyClass, cnameHost })`, which **replaces the whole `🌐️.html`** with the shared boot head (system-appearance script, favicon, boot style); `@tailwindcss/vite` + `@vitejs/plugin-react`; `semioAssetsVitePlugin` serves `/🖼️assets/*` (fonts, cursors).
The quiz site's `🏗️builder/🌐️vite/🟦️.ts` already has `tailwindcss()`, `react()`, `semioHostHtmlVitePlugin`, `semioReferencedAssetsVitePlugin` and the `@semio-tech/ui-react` + `/i18n` aliases (no `/runtime` alias, not needed).

## 3. Card anatomy (measured in the browser)

`DemonstratorCard` (1440 px wide: cell 360 px, `px-double`, card **347 × 162 px**):

```tsx
<button type="button" data-demonstrator-pane-card data-pane-id data-hover-scope onClick …
  className="pointer-events-auto group w-full max-w-sm cursor-pointer border-0 bg-transparent p-0 text-left outline-none
             transition-transform duration-200 hover:-translate-y-0.5
             focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background
             {lifted && '-translate-y-0.5'}">
  <WindowChrome level="dialog" active={false} stackSlot="demonstrator-pane-card-stack" stackClassName="w-full min-w-0"
    titleChips={<div data-slot="…-title-chip" className={cn(windowChromeTitleChipClass, "flex min-w-0 items-center gap-single px-single")}>
                  <Icon icon={pane.icon} size="small" className="shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" title={pane.label}/>
                  <span className="truncate text-sm font-medium text-foreground">{pane.label}</span></div>}
    body={<div data-slot="…-content" className="w-full min-w-0 max-w-sm">
            <p className="mb-double text-xs font-medium leading-normal text-foreground">{pane.tagline}</p>           {/* bold one-liner */}
            <div data-slot="introduction-body" className="flex flex-col gap-double">
              <p data-slot="introduction-body-paragraph" className="whitespace-pre-line text-xs leading-normal text-muted-foreground">…</p> {/* × 2, hover → emphasised colour */}
            </div></div>}
    footerRightChips={<div data-slot="…-open-chip" className={windowChromeTitleChipClass}>
                        <span className="inline-flex items-center gap-single px-single text-xs font-medium text-muted-foreground transition-colors group-hover:text-foreground">
                          Demonstrator öffnen <Icon icon="chevron-right" size="small" className="transition-transform group-hover:translate-x-0.5"/></span></div>}
    bodyClassName="p-double" />
</button>
```

`PlayCard` differs only by: `max-w-xs min-w-0`, `aria-label="Open {label}: {tagline}"` (via `useLabel`), `onFocus` (reveals the pane like hover), tagline `truncate text-xs leading-normal text-muted-foreground`, no description, `bodyClassName="px-double py-single"`, chip text `Open`. At 1440×900 with 148 cards each cell is **110.8 × 71.5 px**, the card **104.4 × 64.8 px**.

DOM the `WindowChrome` produces (a stack with five layers): `data-window-silhouette` stack → `svg[data-window-silhouette-border data-kind=normal]` (`path` stroke `1px`, `var(--border-normal-color)`) → `window-chrome-cap` row (chip cap `ui-glass` 85×26 + transparent `window-chrome-gap` that is a real cut-out) → `window-chrome-body-surface` (absolute glass fill under the body) → `window-chrome-body` (`window-silhouette-content-plane`, clip-path so text never crosses the notch) → `window-chrome-footer` (right chip, `justify-end`). Left chip + right chip in the footer get a real gap between them (`hasFooterGap`), centre chip gets a 3-col grid.

Surface facts for `level="dialog"`: `--surface-bg = color-mix(in oklab, var(--base), var(--foreground) 20%)`, `--surface-alpha 0.52`, `--surface-blur 2rem`, `saturate(1.45)`. Measured light: fill `color(srgb 0.7455 0.7528 0.7115 / 0.52)`, `backdrop-filter: blur(32px) saturate(1.45)`. Veil: `color(srgb .9686 .9529 .8902 / 0.4)`, `blur(8px)`.

States: hover lifts 0.5 unit (`-translate-y-0.5`, 200 ms), outline `stroke` → `--border-emphasized-color` (foreground) in 120 ms, icon and chip text `muted-foreground → foreground`, chevron nudges right (`translate-x-0.5`); a global rule `[data-window-silhouette]:hover > [data-window-silhouette-border][data-kind="normal"] path` already emphasises the outline of **any** silhouette stack, so a non-button card needs no extra CSS. Keyboard focus: `focus-visible:ring-2 ring-ring ring-offset-2` on the whole card button; the chips of `WindowChrome` show an **inset 2 px primary (#ff344f) box-shadow** on `:focus-visible` (measured). `active` is never set on overview cards (that would paint the primary silhouette stroke).

Grid (desktop overview overlay, verbatim):

```tsx
// demonstrator
<div className="pointer-events-none absolute inset-0 z-[31] grid items-center"
     style={{ gridTemplateColumns: "repeat(4, minmax(0, 1fr))", gridTemplateRows: "repeat(2, minmax(0, 1fr))" }}>
  <div className="flex justify-center px-double"><DemonstratorCard …/></div> …
// play: same, plus `pb-double pt-[calc(var(--size-workbench)*1.5)]` (36 px top clearance for the brand bar), cell `flex min-w-0 justify-center px-single`,
//        explicit `style={{ gridColumn: col + 1, gridRow: row + 1 }}` so the short last row is centred, `role="navigation" aria-label="Every semio app"`
```
Cells are equal (`1fr`), gap comes only from `px-*`; cards are vertically centred in their cell (`items-center`), so cards of different heights sit ragged. There are **no** rounded corners, shadows or fills on the cell itself.

## 4. Visual system (tokens)

Fonts (`--font-sans` is the body font, from `🎨️styling/🎨️palette/🎨️.css`, files under `/🖼️assets/🔤️fonts/…`): **Anta** (techno sans; + Anta Extended/Math/Symbols) → **Noto Emoji** (monochrome outline emoji, 0–11 shards); `--font-serif` Kelly Slab; `--font-mono` Share Tech Mono. Root 16 px.
Type scale (`@theme inline`): `2xs .6rem`, **`xs .7rem` = 11.2 px (all card copy)**, `sm .8rem` = 12.8 px (card titles, brand), `base .9rem`, `lg 1rem`, `xl 1.125rem` … `9xl 7.2rem`.
Spacing: `--ui-spacing: 0.2rem` (`html.touch`, i.e. tablet and phone: `--spacing-touch .275rem`, plus `text-xs .75rem`, `text-sm .875rem`) → `single` 3.2 px, `double` 6.4 px, `tiny` 9.6, `small` 16, `medium` 22.4 (chip height), `large` 28.8 (navbar), `workbench` 24, `huge` 35.2 …; `--layout-touch-min 2.75rem`.
Shape: **radius 0 everywhere** (`--radius-*: 0rem`), **all shadows 0**, hairline strokes `1px` (`--stroke-hairline`), default 2 px, focus 3 px. Sharp corners, no shadow; depth = glass level + outline.
Elevation ("levels", `data-level` base/window/pane/panel/dialog/menu, each +5 % foreground mix, −0.12 alpha, +0.5 rem blur); z: base 0, window 10, pane 20, panel 30, dialog 40, menu 50, navbar 100, modal 1000. Fallbacks already in the CSS: no `backdrop-filter` support, `prefers-reduced-transparency`, `forced-colors`.

Palette (`--color-*`, generated from `🎨️styling/🔣️.json`): primary **#ff344f**, secondary **#34d1bf**, tertiary **#fa9500**, danger #a60009, warning #fccf05, info #dbbea1, success #7eb77f; neutrals dark **#001117**, light **#f7f3e3** and a 30-step olive-grey ramp (`gray #7b827d`, `dark-gray-2 #3e494a`, `light-5-9 #91968f` …).

| Semantic token | Light (`:root`) | Dark (`.dark`) |
|---|---|---|
| `--base` (`bg-background`) | `#f7f3e3` | `#001117` |
| `--foreground` | `#001117` | `#f7f3e3` |
| `--muted-foreground` | `#3e494a` | `#91968f` |
| `--border-normal-color` | `#7b827d` | `#7b827d` |
| `--border-emphasized-color` | `#001117` | `#f7f3e3` |
| `--accent` / `--active-base` | `#ff344f` | `#ff344f` |
| `--accent-foreground` / `--active-foreground` | `#001117` | `#001117` |
| `--hover-interactive-fill` | `#7b827d` | `#606966` |

`.dark` is set on `<html>` by `bootstrapElementsSurfaceChromeDocument` / `useElementsSurfaceChrome` (`system` follows `prefers-color-scheme`, listener installed); `html[data-ui-device]` = `mobile|tablet|desktop`, `.touch` on non-desktop. **Gotchas measured while building the prototype:** `--color-base` exists, so Tailwind's `text-base` is a *colour* (paper on paper = invisible text); use `text-lg`/`text-sm`. `border-normal` is a custom `@utility`.

## 5. Screenshots and what they show

Files in `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️28\QUIZ-PRODUCT-AND-TEACHING-PROCTOR\🗑️generated\ui-reference\`:

| File | Viewport | Content |
|---|---|---|
| `demonstrator-1440-light.jpg` | 1440×900 | **4 × 2 ring of 8 cards** on a cream (`#f7f3e3`) page, veil-blurred placeholder behind. Cards ≈ 347×162 px in cells of 360×450, vertically centred so tops are ragged; each card = tab chip top-left (icon + name), bold tagline, two paragraphs, `Demonstrator öffnen ›` chip bottom-right, outline notches around both chips. Bottom: footer credits left (`Ein Projekt von [LUH] und [UdK]`) / right (`Gefördert durch ZUKUNFT BAU`). |
| `demonstrator-cards-detail-light.jpg`, `demonstrator-cards-detail-dark.jpg` | zoomed | two cards at 1:1: 1 px grey-olive outline, sharp corners, translucent olive-grey glass body, Anta type; title chip and open chip are cut-outs of the same outline. |
| `demonstrator-1440-dark.jpg` | 1440×900 | same layout on `#001117`; glass = lighter teal-black, outline `#7b827d`, text `#f7f3e3` / muted `#91968f`. |
| `demonstrator-768-tablet-light.jpg` | 768×1024 | **same 4 × 2 grid**, columns 192 px, cards 179 px: text wraps to 6–9 lines, cards of very different heights hang in tall rows, ~40 % of the page is empty; footer credits still fit. No tablet layout exists. |
| `demonstrator-375-phone-light.jpg` | 375×812 | touch list: **one card per screen**, vertically centred in a `100dvh` scroll-snap section over a full-strength veil (each section is one app). Footer credits **collide** ("Ein Projekt von [logos]" overlaps "Gefördert durch"). |
| `play-1440-light.jpg` | 1440×900 | **13 × 12 = 148 tiny cards** (104×65 px): icon + truncated name, one truncated tagline, `Open ›`; last row (5 cards) centred; a compact index rather than a card wall. |
| `play-768-tablet-light.jpg` | 768×1024 | the same 13 columns at ~59 px: names cut to "Proce", "Puzzl", "Semio" — unusable; again no tablet layout. |
| `play-375-phone-light.jpg` | 375×812 | one compact card (`CAD · Model shapes and buildings · Open ›`) centred per screen. |
| `quiz-mock-1440-light.jpg`, `quiz-mock-1440-dark-crop.jpg`, `quiz-mock-768-tablet-light.jpg`, `quiz-mock-375-phone-light.jpg` | 1440 / 768 / 375 | **the recommended quiz home** built from the real components (section 6). |

**Not visible / not verified live:** the `Navbar` brand chip. In the rig (identical code + CSS) `<nav data-slot="navbar">` resolves to `position: relative; z-index: 0` because `🖌️ui/🎨️.css` forces `[data-slot="navbar"] { position: relative !important; z-index: var(--z-base) !important }`, which overrides the landing's `absolute inset-x-0 top-0 z-40` class. It therefore sits **in flow after the 200vh strip (y = 1800 px) and is clipped**: the "semio Play" / "Entwerfen mit Bestand" brand bar is currently not on screen. Either a regression of ticket 26/09/02 (navbar pinned to the base level) or a rig artefact I could not rule out without the live app; a live check is worth one minute. For the quiz this is irrelevant because I recommend the navbar **in flow** (its natural behaviour).

## 6. Recommendation: the quiz home

### 6.1 Reuse, do not re-skin

1. **Same components and tokens.** Render every card with `WindowChrome` (`level="dialog"`, `active={false}`) + `windowChromeTitleChipClass` chips; header with `Navbar` + `ShellBrandLogo`; icons with `Icon`; loading with `CanvasSkeleton`/`loadingBorderClass`; first-visit introduction with `UIIntroduction`. Load the tokens via the **same CSS chain**: in `🎓️teaching/🏛️architecture/❓️quiz/🎨️.css` replace the palette-only layer with
   ```css
   @import "../../../🧰️framework/🔨️modules/🖱️ui/🧵️styles/🎨️.css";   /* or the 💻️os engine 🎨️.css exactly like play/demonstrator */
   @source "../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react";
   ```
   (today it is `@import …/🎨️palette/🎨️.css theme(static); @tailwind utilities source(none)`), and call `bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(storage))` + `useElementsSurfaceChrome` so `.dark`/`data-ui-device` follow the learner's theme. Map the learner's text size to the **root font size** (`html { font-size: calc(16px * var(--quiz-text-scale)) }`): every token above is `rem`-based, so the whole system scales; then retire the `.quiz-*` visual rules of `🎯️targets/⚛️react/🎨️.css` (colours, borders, buttons, cards) in favour of the tokens (keep `.quiz-visually-hidden`/skip link as `sr-only`).
2. **Extract the duplicated card once** (schema-first / taxonomy: e.g. `🖱️ui/🧱️elements/🃏️OverviewCard` with a React target) instead of writing a third copy: props `icon`, `title` + `headingLevel`, `children` (body), `footerLeft`, `footerRight`, optional `as="section"|"button"`; PlayCard/DemonstratorCard/quiz cards become thin wrappers. `rig/quiz-mock.tsx` `Card` (12 lines) is that component. The grid stays CSS in each app (a quiz-specific layout).
3. **Bundle cost is the one real price** (measured with `vite build` of the mock, then deleted): importing the `@semio-tech/ui-react` barrel gives **1.63 MB JS (352 kB gzip) + 268 kB CSS (40 kB gzip)** plus a lazy 1.2 MB `pdf.worker` chunk, versus today's quiz site **414 kB JS (122 kB gzip) + 21 kB CSS (4 kB gzip)**. AGENTS.md says maximum performance: introduce a slim sub-entry (e.g. `@semio-tech/ui-react/chrome`: `WindowChrome`, chip classes, `Navbar`, `Icon`, `cn`, appearance helpers, i18n) or mark the barrel side-effect-free before switching the public site. Decision for the parent; the design does not depend on it.
4. Keep `createRoot`/plain React in the quiz renderer (the `ui-react/runtime` wrapper is a passthrough); no alias change needed.

### 6.2 Layout (the prototype's exact structure, verified at 1440 / 768 / 375)

Ring of eight cards around the leaderboard. **DOM order = row-major reading order = visual order at every breakpoint (no CSS `order`, no grid areas)**:

```
desktop ≥ 1024          tablet 768–1023            phone ≤ 767
┌────────┬──────────┬────────┐   ┌─────────┬─────────┐   ┌───────────┐
│ learner│  quiz 1  │ intro  │   │ learner │ quiz 1  │   │ learner   │
├────────┼──────────┼────────┤   │ intro   │ quiz 2  │   │ quiz 1    │
│ quiz 2 │LEADERBOARD│ quiz 3 │   ├─────────┴─────────┤   │ intro     │
├────────┼──────────┼────────┤   │   LEADERBOARD     │   │ quiz 2    │
│ badges │  quiz 4  │  prefs │   ├─────────┬─────────┤   │ LEADERBOARD│
└────────┴──────────┴────────┘   │ quiz 3  │ badges  │   │ quiz 3 …  │
                                 │ quiz 4  │ prefs   │   └───────────┘
                                 └─────────┴─────────┘
```

```tsx
<div className="flex h-full min-h-0 flex-col overflow-hidden bg-background text-foreground">
  <a href="#main" className="sr-only focus:not-sr-only …">Skip to content</a>
  <header><Navbar items={[brand chip, navbarFillItem(), connection status]} showFullscreenToggle={false} /></header>   {/* in flow, h-large = 28.8 px */}
  <main id="main" tabIndex={-1} className="min-h-0 flex-1 overflow-auto p-double">
    <h1 className="sr-only">…catalog title…</h1>
    <div className="quiz-home-grid"> {/* learner, quiz1, intro, quiz2, LEADERBOARD, quiz3, badges, quiz4, prefs */} </div>
  </main>
</div>
```
```css
.quiz-home-grid { display: grid; gap: var(--spacing-double); grid-template-columns: minmax(0, 1fr); }              /* phone: 1 column, page scrolls */
@media (min-width: 768px)  { .quiz-home-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); }
                             .quiz-home-grid > [data-card="board"] { grid-column: 1 / -1; } }                     /* tablet: board spans both, 5th of 9 = middle */
@media (min-width: 1024px) { .quiz-home-grid { height: 100%; grid-template-columns: minmax(0, 1fr) minmax(0, 1.5fr) minmax(0, 1fr);
                                               grid-template-rows: minmax(12rem, 1fr) minmax(12rem, 1.4fr) minmax(12rem, 1fr); }  /* desktop: board = centre cell, largest */
                             .quiz-home-grid > [data-card="board"] { grid-column: auto; } }
```
Breakpoints equal the design system's `UI_MOBILE_MAX_WIDTH_PX 767` / `UI_TABLET_MAX_WIDTH_PX 1023` (do not invent new ones). Measured in the prototype at 1440×900 (its rows were `minmax(0, 1fr) minmax(0, 1.4fr) minmax(0, 1fr)`; the `12rem` minimum above does not bind at this height and was not exercised in the screenshots): columns **404 / 606 / 404 px**, rows **249 / 348 / 249 px** (gap and page padding `--spacing-double` = 6.4 px, navbar 28.8 px); no card overflows; the board shows header + 5 rows + separator + own row without scrolling. On short desktop viewports the `12rem` row minimum lets `main` scroll instead of clipping. On tablet (768×1024) all nine fit in ~700 px; on phone the board sits about 1200 px down: add an optional header chip "Rank 8 · 620 points" that links to `#board`, rather than reordering the DOM.
Rationale: the demonstrator/play tablet and phone behaviour is the anti-pattern (4-col squeeze on tablet, one-card-per-screen on phone); the quiz needs a real 2-column tablet grid and a compact single column. Keep the touch list / veil / live-pane machinery, pan-on-mousemove and `100vw` strips **out**: they exist for live apps.

### 6.3 What each card shows

All cards: `<section aria-labelledby=…>` whose title chip contains an `<h2>` (icon + one-line truncated title); body text `text-xs`-`text-sm` (see 6.5) with `text-foreground` for the lead line and `text-muted-foreground` for facts; footer chips are **real `<button>`s** (`windowChromeTitleChipClass gap-single px-single text-xs font-medium`), primary action right (label + `chevron-right`, group-hover emphasis exactly like the open chip), secondary left.

| Card | Title chip | Body | Footer left / right |
|---|---|---|---|
| **Quiz** (×4, one per `catalog.quizzes`) | emoji glyph + `localized(quiz.title)` | `localized(quiz.description)` as lead; facts row: `n tasks`, `Best 82 %` or `Not played yet`, `Run open` when `openRunOf(...)` exists (all three strings exist: `quiz.home.tasks/best/notYet/open`); at the bottom a line for the badge earned for this quiz (`🧲 Physics Expert earned`) from `learnerView.badges` filtered by the quiz's `perfect-quiz` rule; optional thin score meter (native `<progress>` styled with `accent-color`, label = the score) | left `Last result` (only if `lastSubmittedRunOf` exists → `session.open({screen:"results", run})`); right **`Start` / `Resume` / `Play again`** (`quiz.home.start/resume/again`; disabled and replaced by the existing progress + `Cancel` while `pending`; errors as `role="alert"` above the grid as today) |
| **Leaderboard** (centre) | `list-ordered` icon + "Leaderboard" | compact `<table>` (`<caption class=sr-only>`): `# · Learner · Points · Badges`; **top 5** rows from `state.leaderboard` + a `⋯` separator (`aria-hidden`) + **own row** when own rank > 5 (`aria-current="true"`, primary inset bar `box-shadow: inset 3px 0 0 var(--active-base)` + `color-mix(--active-base 14 %)`, bold, "(you)" via `quiz.leaderboard.you`), rows keyed by public `tag`, never ids; loading → `CanvasSkeleton`, empty → `quiz.leaderboard.empty`; table body scrolls inside the card (`min-h-0 overflow-auto`); keep polling every `LEADERBOARD_POLL_MS`, **do not** announce refreshes to screen readers | left `Updated 14:02` (`quiz.leaderboard.updated`); right **`Full leaderboard ›`** → the existing sortable `LeaderboardScreen` (per-quiz best columns, badges, runs, last activity) as the expansion (route/dialog) |
| **Learner** | `user` icon + `learnerName(identity, tag)` | welcome line, total points as the one large number (`text-lg font-semibold tabular-nums`), facts: quizzes played x/4, badges y/7, rank r of n | right `Switch learner` (`session.forgetLearner()`) |
| **How it works** | `info` icon | first paragraph of the catalog introduction, 3–4 lines | right `Read more ›` → the full introduction as `UIIntroduction`/dialog; auto-open on first visit as today |
| **Badges** | `award` icon | one `size-workbench` tile per badge (`border-normal`, emoji; earned = foreground, locked = 50 % opacity + `sr-only` "earned/locked"), "3 of 7 earned" | right `All badges ›` → dialog with label, description and earned date (today's badge list) |
| **Preferences** | `settings` icon | segmented `aria-pressed` groups: Language (EN/DE, `LanguageSwitch`), Theme (System/Light/Dark), Text size (existing `TEXT_SIZES`) | — |

Emoji per quiz: the schema has none (`Quiz` = id/title/description/tasks), but the folders (`🧲️physics`, `🔥️heating`, `❄️cooling`, `📊️demand`) and each quiz's `perfect-quiz` badge already carry it. Either derive it from the badge (no schema change) or add an optional `emoji` to `Quiz` in `🧬️schema/🔣️.json` and both twins (schema-first, preferred). Emoji render in the monochrome **Noto Emoji** face; VS16 sequences such as `❄️` fall back to the colour system font (blue snowflake in the prototype): apply `font-variant-emoji: text` on the glyph spans to keep the look consistent.

### 6.4 Header / footer

Header: in-flow `Navbar` (`h-large`), left brand chip = `ShellBrandLogo` (`size-workbench`) + catalog title (`text-sm font-semibold`), right connection state (`quiz.connection*` strings; `truncate` it at 375 px, the prototype clips "Online · answers saved"). Optional footer chips (left connection/queue state, right partner credits) must be **icon-only or stacked on phone** (the demonstrator's two credits overlap at 375 px); reuse the `fundedBy…FooterItem` pattern (`text-2xs text-muted-foreground`, `iconOnly`).

### 6.5 Accessibility checklist (measured in the prototype unless noted)

- **Landmarks/headings:** `header` (banner) > `Navbar`; **note `Navbar` renders `<nav id="ui.navbar">` with no accessible name** — wrap in `<header>` and either give it an `aria-label` upstream or do not rely on it as navigation; one `main#main[tabindex=-1]` with skip link (the current `.quiz-skip`); `h1` (visible or `sr-only`), nine `h2` inside `section[aria-labelledby]` = 10 headings, jumpable. Prototype output: `H1 Quizzes, H2 Ann K., Physical Understanding, How it works, Heating, Leaderboard, Cooling, Badges, Energy Demand, Preferences`.
- **Do not copy the demonstrator card as a whole-card `<button>`**: it wraps `<p>`/`<div>` in a button (invalid content model, screen readers read a paragraph wall as one label) and can only carry one action; quiz cards need Start + Last result. Cards are sections; the actions are chips.
- **Focus order = DOM order = row-major grid order** (prototype: 21 tab stops: skip, learner, quiz 1 secondary + primary, intro, quiz 2 …, board footer, …, prefs segments). Focus indicator: the chips already show an inset 2 px primary ring; keep it, add `ring-ring` on any wrapper that becomes focusable.
- **Target size:** footer chips are `min-h-medium` = 7 × `--ui-spacing` = **22.4 px** high on desktop, just below WCAG 2.2 SC 2.5.8 (24 px). Give quiz chips `min-h-[24px]`. `html.touch` (set for `data-ui-device` tablet and mobile, `🖌️ui/🎨️.css:653`) switches `--ui-spacing` to `--spacing-touch` (0.275 rem → chips 30.8 px) and bumps `text-xs` to 0.75 rem and `text-sm` to 0.875 rem, so tablet/phone already get larger targets and type; only desktop needs the explicit minimum.
- **Text size:** on desktop card copy is `text-xs` = 11.2 px (12 px under `.touch`); use `text-sm` (12.8 px / 14 px) for descriptions and the table, and rely on the root-font-size scale for the learner's text-size preference. Contrast (hand-computed from the measured glass fills composited over the page background, not tool-verified): light muted `#3e494a` on card glass ≈ 6.5:1, dark muted `#91968f` on dark glass ≈ 4.9:1 — both pass AA for body text, so do not lighten the muted tokens.
- **Motion/transparency:** the hover lift (`transition-transform`) and chevron nudge should be wrapped in `motion-reduce:` variants; `prefers-reduced-transparency` and `forced-colors` fallbacks for `ui-glass` already exist in `🖌️ui/🎨️.css`.
- **Live regions:** keep the existing `LiveRegion`/`role="status"` for run start and errors; the leaderboard's 10 s polling must not announce.
- **i18n:** no default language; register every new string for `en` and `de` through the existing `QuizText` bundles (`🔨️modules/🌐️i18n`), set `lang` on `<html>` when the locale switches.

### 6.6 Tests to add (test-driven, language-agnostic)

DOM-structure case shared with the renderer tests: nine `section[aria-labelledby]` in the order learner, quiz1, intro, quiz2, leaderboard, quiz3, badges, quiz4, prefs; the leaderboard is the 5th; each quiz card exposes exactly its Start/Resume/Play-again action and Last-result only when a submitted run exists; own row has `aria-current`; at 767/768/1023/1024 px the computed `grid-template-columns` count is 1/2/2/3. Third-party oracle: `@testing-library/react` + axe (`jest-axe`) on the rendered home.

## 7. Key files

- Play: `C:\git\semio\🏢️semio-tech\🎡️play\{🟦️.tsx (overview 783-845, touch list 740-781), ⚛️play-card.tsx, 🪧️brand.ts (grid math 99-145), 🎨️globals.css, 🌐️.html}`
- Demonstrator: `C:\git\semio\♻️mit-bestand\🧺️demonstrator\{🟦️.tsx (overview 816-889), ⚛️demonstrator-card.tsx, ⚛️footer.tsx, 🪧️brand.ts (panes 911-992), 🎨️globals.css, 🌐️.html}`
- Shared UI: `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎯️targets\⚛️react\🟦️.tsx` (WindowChrome 6029, chip class 5893, appearance 1898-2075), `🧱️elements\🔝️Navbar\🟦️.tsx`, `🎨️styling\🖌️ui\🎨️.css` (tokens 754-880, levels 925-985, `ui-glass`/`ui-veil` 7050-7060), `🎨️styling\🎨️palette\🎨️.css` (fonts, colours), `📱️device\🟦️.ts` (breakpoints).
- Quiz today: `🧰️framework\🛍️products\❓️quiz\🎯️targets\⚛️react\{🟦️.tsx, 🎨️.css (.quiz-* rules), 🔨️modules\🏠️home\🟦️.tsx, 🔨️modules\🏆️leaderboard\🟦️.tsx}`, `🎓️teaching\🏛️architecture\❓️quiz\{🎨️.css, 🏗️builder\🌐️vite\🟦️.ts, 🔣️.json}`.
- Ticket-local rig and prototype: `…\QUIZ-PRODUCT-AND-TEACHING-PROCTOR\🗑️generated\ui-reference\rig\{vite.config.ts, vite.build.config.ts, play.*, demonstrator.*, quiz-mock.{html,css,tsx}}` (tool output caches deleted; screenshots kept beside it; delete the whole `🗑️generated` folder when the ticket closes).
