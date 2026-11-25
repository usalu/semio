# 🔎 Audit 2: Web Code Against the Frame and Glassy-Chip Design

Read-only audit. No source file was edited, no git write was run, no build was started.

Paths are relative to `/Users/ueli/Documents/semio/`. Design reference: `design.md` (this ticket) and `summary.md` of `☀️24/UI-LAWS-FOR-CHIPS-HOVER-BANDS-DRAG-TOGGLES-ICONS-AND-LINES`. Pre-audit: `audit-unnamed-patterns.md` (this ticket). Tests and ticket folders were excluded.

## 🧭 Method and scope

- Scope: TS/TSX/CSS/HTML/MDX web code in `🧰️framework/🔨️modules/🖱️ui`, `🧰️framework/🛍️products/{📚️course,💻️os,🐾️pets,🎤️presentation}`, `🏢️semio-tech`, `♻️mit-bestand`, `🌎️hub`, `🎓️teaching`, `.storybook`. `🧰️framework/🛍️products/🖥️web` does not exist; `👴️leutwiler` contained no hits.
- Grep sweeps: negative offsets and `top`/`bottom` overlays, `clip-path`/`polygon`, old names (`silhouette`, `gap` regions, `cutout`), `rounded-*`/`shadow-*`/`box-shadow`/`border-radius`, hand-drawn `border*` classes, `z-[n]` arbitrary utilities, `max-w-*` and `mx-auto` blocks, `lifted`/`translate-y` hovers, `absolute` edge overlays.
- Read directly: `WindowFrame/🟦️.tsx`, `WindowFrame/shared-border.ts`, `CardStack/🟦️.tsx`, `CardStack/bands.ts`, `OverviewCard/🟦️.tsx`, `WindowChrome/🟦️.tsx` (chip surfaces, cap and footer rows), `Canvas/🦶️bottom-chips`, `Tabs/🟦️.tsx`, `Panel/🟦️.tsx`, `PanelTabBar` (tab-frame and `startDepth`), `Canvas/⚓️dock` corner bars, `UI 🎨️.css` (chip and frame rules, lines 7330-7660), `ui/🎨️chrome-classes`, OS `canvas-footer/footer-items`, pets stories CSS, presentation handles CSS, course styles and chooser.
- Not checked: every line of the four product `.css` trees, the Rust/wgpu and TUI targets (other lanes), LaTeX `.sty` (print lane, see section 4).

Severity: **H** = visibly breaks a stated law (floating framed box, gap or lift between framed cards); **M** = law breach that is visible on some state or breaks chip tint or stacking; **L** = dead class, cosmetic, or naming.

Note on `rounded-*` and `shadow-*`: the theme sets `--radius-*` and `--shadow-*` to `0`, so those utilities paint nothing. Their hits are therefore **L** (source hygiene), except where a real radius or shadow value is written (`border-radius: var(--spacing-single)`, `border-radius: 0 9px 0 0`, `box-shadow` rings).

---

## 1. Recheck of the pre-audit rows (`audit-unnamed-patterns.md`)

| Old row | Current code | Status |
| --- | --- | --- |
| 1 CardStack cap and footer lifted onto neighbour's line (`CardStack/🟦️.tsx`) | Cards abut with no lift, margin or band; `useCardInterlock` snaps heights (`CardStack/🟦️.tsx:14-80`) | Resolved |
| 2 `CardStack/bands.ts` band model (`none`/`shared`/`stacked`) | Band model removed. File keeps `CARD_BAND_MIN_GAP`, `chipGapReadsAsSliver`, `snapCardHeight` (`bands.ts:1-13`); name is stale | Resolved, name L (see lane K) |
| 3 OverviewCard title and footer chips | Chips are chrome props inside the frame (`OverviewCard/🟦️.tsx:77-97`); the body's `lifted` prop is new, see H-1 | Resolved, lift open |
| 4 PanelTabBar as framed row fed into cap | Tab-frame is a framed row inside the chip (`PanelTabBar/🟦️.tsx:589`); verify visually | Mostly resolved, L |
| 5 WindowFrame chip regions straddle the edge | Chip regions start at `y: 0` (top) or `height - depth` (bottom); all inside the frame (`WindowFrame/🟦️.tsx:113-125`) | Resolved |
| 6 Outline split around chip regions | `sharedOutlineSegments` emits the four frame edges plus inner chip edges only (`WindowFrame/shared-border.ts:161-188`) | Resolved |
| 7 Corner mode-dock tab bars as `titleChips`, `capRightChips`, footer chips (`Canvas/⚓️dock`) | Passed to WindowChrome props, rendered inside the frame (`Canvas/⚓️dock/🟦️.tsx:265-267, 330-333`) | Resolved |
| 8 Panel tab bar as `titleChips` (`Panel/🟦️.tsx:511-514`) | Inside the chip row. A second tab strip was added in the body (`Panel/🟦️.tsx:519`), see M-3 | Partly open |
| 9 WindowPane and Skeletons `titleChips` | Chips inside the frame | Resolved |
| 10 Introduction footer chips | Footer chips inside the chrome (`introduction/🟦️.tsx:421-422`) | Resolved |
| 11 Tabs: selected trigger outlines all four edges | Selected trigger draws three edges, the open side faces the panel (`Tabs/🟦️.tsx:231`); the list is an overlay chip, see M-2 | Mostly resolved |
| P1-P11 print `semio-window.sty` | Out of web lane. `\semio@window@cap@raise` no longer exists; `\semio@chrome@chip` macros remain (`semio-window.sty:3090-3140`) | Not re-audited, see section 4 |
| O1 OS canvas footer tab bars | Glass chips on the canvas frame's bottom edge (`canvas-footer/footer-items/🟦️.tsx`) | Resolved |
| C1, C2 course card chips and CSS | Inside the frame; course CSS `course-card-title-chip` is a chip rule (`📚️course/…/🏠️chrome-and-home/🎨️.css:5-11`) | Resolved |
| teaching browser title chip (`🌐️browser/🟦️.tsx:35,45`) | Title chip in `titleChips`; the video body is inset 3.2 px from every frame edge, see M-7 | Partly open |
| `🏢️semio-tech` play-card, `♻️mit-bestand` demonstrator-card, card CSS | Consumers of OverviewCard, resolved; their `lifted` prop is open, see H-1 | Partly open |
| pets `.card .tab { top: -16px }` (`🐾️pets/…/📖️stories/🎨️.css:426-429`) | Now `top: 0; right: 0`, inside the card. Still `border-radius: 0 9px 0 0`, see L-8 | Resolved, radius open |

---

## 2. Findings by lane

Lanes are disjoint in files so one agent can own each lane. Lane letters are referenced in section 3.

### Lane A: floating chrome surfaces and hand-drawn frames (`🎯️targets/⚛️react` and `🔨️modules`)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| A-1 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts:22` | `borderNormalFrameClass` = `box-border border border-solid …`, a hand-drawn 4-side frame class | Bypasses the shared border compositor. Consumers are only re-exports and `shellChromeBorderClass` | Delete the class and its re-exports, or make it a `data-shared-border-frame` marker | M |
| A-2 | same file, lines 14, 17 | `borderEmphasizedBottomClass` / `…TopClass` = `border-b`/`border-t` plus emphasis | Hand-drawn single lines bypass the compositor; no consumer in the web scope besides the re-export | Delete, or express as `data-shared-border-edges` | M |
| A-3 | same file, line 31 `floatingMenuSurfaceClass`, used by `🎯️targets/⚛️react/📋️canvas-pick/🟦️.tsx:10,82` | `rounded-md border shadow-sm` on a fixed floating menu | Hand-drawn border on a menu, plus dead `rounded`/`shadow` classes | Use `data-shared-border-frame` with `data-level="menu"`; drop `rounded-md border shadow-sm` | M |
| A-4 | same file, line 37 `floatingRibbonSurfaceClass` | `overflow-hidden rounded-md border shadow-sm` + `borderNormalClass` | Same as A-3 on a ribbon; the ribbon is a chip-row object | Express as a chip inside its frame (`data-glass-chip`), no border | M |
| A-5 | same file, line 34 `floatingPaneAsideClass` | `relative flex shrink-0 flex-col text-element z-[2]` | `z-[2]` arbitrary utility, not emitted by the stylesheet (WindowChrome comment says so) | Replace with inline `zIndex` or a `z-*` token | M |
| A-6 | `🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts` line 31, 37 (`glassClass` on floating surfaces) and `🔨️modules/🎨️chrome-class-names/🟦️.ts:34` `GLASS_OVERLAY_BOX_CLASS` | `max-w-sm rounded-lg p-double shadow-lg`, a fixed tooltip-like box | `rounded-lg`/`shadow-lg` are dead; `max-w-sm` floats it. OK for a transient tooltip | Drop the dead classes; keep the box if it stays transient | L |
| A-7 | `🧱️elements/💡️ChromeControlHint/🟦️.tsx:24` | `fixed z-menu w-max max-w-sm border p-single text-xs` | Hand-drawn `border` on a transient hint | Make the hint a glass chip with `data-glass-chip` and no border, or a compositor frame | M |
| A-8 | `🎯️targets/⚛️react/📣️display-components/🟦️.tsx:50` | `<aside className="my-small p-single border …">` | Hand-drawn border with a vertical margin: a floating box between other boxes | Replace with a compositor-framed `SharedBorderRow` with no margin; the margin goes to the container | H |
| A-9 | `🎯️targets/⚛️react/🎥️tutorial-ui/🟦️.tsx:210` | `<video className="fixed z-tutorial rounded-lg object-cover shadow-lg">` | Floating picture-in-picture with dead `rounded`/`shadow` classes | Drop the classes; if it is a window, use WindowChrome | L |
| A-10 | `🎯️targets/⚛️react/🎥️tutorial-ui/🟦️.tsx:123` | `size-1 rounded-full bg-foreground/50` chapter tick | A tick dot; dead radius | Drop `rounded-full` (renders square) or keep as a marker; decide | L |
| A-11 | `🎯️targets/⚛️react/📋️canvas-pick/🟦️.tsx:82` | `fixed z-tutorial w-layout-floating-menu-sm max-…` with `floatingMenuSurfaceClass` | Fixed menu with its own frame, no chip | See A-3 | M |

### Lane B: chip tint, stacking and framed rows inside chrome (`🧱️elements`)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| B-1 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️.tsx:182` | `absolute z-[2] inline-flex … start-0 top-0 h-large … glassClass`, `data-tabs-chip` | A chip overlay with `ui-glass` but **no** `data-glass-chip`, so no chip tint (`UI 🎨.css:7358`); `z-[2]` is dead | Add `data-glass-chip` (chip tint, chip level) and an inline zIndex | M |
| B-2 | `Tabs/🟦️.tsx:231` | Selected trigger: `right bottom left` (horizontal), `top bottom left` (rtl), `top right bottom` (vertical) | Three-side tab box above the panel: the open side faces the content, so the tab reads as a separate box, as in row 11 | Accept as tab-attached design, or draw the tab as a chip in the list. Verify visually | L |
| B-3 | `🧱️elements/🖼️Panel/🟦️.tsx:519` | Second `<PanelTabBar startDepth={1} …/>` inside `panel-body-stack`, above the `Scrollable` | A framed tab row sitting in the content plane beneath the cap chip; the body is not an edge chip | Make deeper tab rows chips on the body's top frame edge (`data-window-frame-chip`, `dock="top"`), or pass them to `titleChips` as a second chip row | M |
| B-4 | `🧱️elements/🧭️PanelTabBar/🟦️.tsx:589` | `data-slot="…-tab-frame"` with `data-shared-border-frame` only when `edge` is set | Framed tab row inside the chip; a frame in a frame | Check the edge set when the row is inside a chip; use `data-shared-border-cells` if it is only a row | L |
| B-5 | `🧱️elements/🗂️WindowChrome/🟦️.tsx:335, 361` | Border overlay `className="… absolute inset-0 z-[40] overflow-visible"` | `z-[40]` is an arbitrary class the comment says is not emitted; order relies on a sibling inline style | Set `zIndex` inline like `WINDOW_CHROME_CHIP_ROW_STYLE` | M |
| B-6 | `🧱️elements/🥞️LayeredOverview/💡️hints/🟦️.tsx:70` | `ui-glass absolute z-[32] …` with `data-glass-chip` (line 61) | Chip is tinted, but `z-[32]` is dead | Inline zIndex | M |
| B-7 | `🧱️elements/🔍️WindowSearch/🟦️.tsx:432` | `relative z-[1] min-w-0 flex-1 bg-transparent` on an input | Dead arbitrary z; the input sits in the chip row | Inline zIndex or DOM order | L |

### Lane C: lifted cards (the card's frame moves, so it no longer abuts its neighbours)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| C-1 | `🧱️elements/🃏️OverviewCard/🟦️.tsx:52, 116-121` | `lifted` prop: `-translate-y-0.5` on the card's paint layer of a button card | Moves the whole framed card 2 px up while revealed, so it overlaps or gaps against its neighbour in `CardStack`/`CardGrid`; breaks the abut law | Remove `lifted`; express the reveal as a glass or emphasis change on the card's own frame | H |
| C-2 | `♻️mit-bestand/🧺️demonstrator/⚛️demonstrator-card.tsx:28`, `♻️mit-bestand/🧺️demonstrator/🟦️.tsx:125` | `lifted={state.revealed}` | Consumer of C-1 | Drop the prop with C-1 | H |
| C-3 | `🏢️semio-tech/🎡️play/⚛️play-card.tsx:36`, `🏢️semio-tech/🎡️play/🏠️play-landing.tsx:110` | `lifted={state.revealed}` | Consumer of C-1 | Drop the prop with C-1 | H |
| C-4 | `♻️mit-bestand/🧺️demonstrator/🟦️.tsx:197, 208`; `🏢️semio-tech/🎡️play/🏠️play-landing.tsx:162` | `pointer-events-none absolute inset-x-0 top-0 z-40` wrappers around `Navbar`/`Footer` | Band overlays, not chips. Glass bands are a design law (section 4 of summary) | Keep. Note the z-40 is a Tailwind token class, verify it is emitted | L |

### Lane D: course product (`🧰️framework/🛍️products/📚️course`)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| D-1 | `🎯️targets/⚛️react/🔨️modules/📚️app/🟦️.tsx:636` | `<div className="mx-auto w-full max-w-xl"><LanguageChoice/></div>` inside a padded `<main id="course-main" … p-double>` | First-visit language chooser still a centred floating block (open item in summary, section 3) | Make the chooser a framed, full-width band or a `SharedBorderRow` that tiles the main area; centre only its content | H |
| D-2 | `🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css:32` | `.course-tree-panel { margin: 0.5rem; border: 1px solid currentColor; max-width: 26rem; }` | Floating panel with margin, a hand-drawn border and a max width | Use `data-shared-border-frame` and remove margin; if it depicts another program's UI, keep the depicted geometry but make the outer box a frame | M |
| D-3 | `🎯️targets/⚛️react/🎨️styles/👥️peer-marks/🎨️.css:48-49` | `.course-peer-focus > .course-peer-label { top: -1.25rem; left: -2px }` | A name label sticks out of the control's top edge on focus | Put the label as a chip inside the control's frame, or accept as a focus label (decide) | M |
| D-4 | `🎯️targets/⚛️react/🔨️modules/🏁️results/🟦️.tsx:38-39`, `🏆️leaderboard/🟦️.tsx:188, 244, 255`, `📇️profile/🟦️.tsx:139, 188`, `🕸️radar/🟦️.tsx:384-386` | `border-b border-normal` / `border-t border-normal` on table cells and rows | Hand-drawn table rules bypass the compositor; the rest of the course uses cells | Convert to `SharedBorderRow`/`SharedBorderRowCell` rows, or to a table with `data-shared-border-cells`; decide per table | M |
| D-5 | `🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css:53` | `.course-slider-thumb { top: -0.3rem }` | Thumb overhangs the track; a control part, not a chip | Keep (control geometry) | L |
| D-6 | `🎯️targets/⚛️react/🔨️modules/📚️app/🛟️load-fallback/🟦️.tsx:34` | `flex max-w-xl … text-center` section, two plain buttons | Centred content inside a band; buttons have no icons (open item section 9) | Give the buttons icons; keep centring | L |
| D-7 | `🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx:711` | `mx-auto w-full max-w-md|max-w-2xl` around a non-flush dialog body | Dialog content keeps reading width, as design section 3 specifies | Keep, by design | L |
| D-8 | `🎯️targets/⚛️react/🔨️modules/🪪identity` and `⛰️challenge` controls | Summary section 8 open: text rows (`Jetzt schon anzeigen`), footer link and some action chips still have no icon | Not verified in code this pass | Sweep the call sites of `RowLink`/`CardLink`/`BodyButton` for text-only controls | L |

### Lane E: teaching (`🎓️teaching`, staffage and course sites)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| E-1 | `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/📦️packages/🟦️typescript/📚️app/🟦️.tsx:34` | `<SharedBorderRow orientation="vertical" className="my-auto w-full max-w-xl shrink-0 bg-panel">` | Framed box with a max width, centred in its container: a floating card | Make it full width (no `max-w-xl`), or move the width to its content | H |
| E-2 | `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx:35-45` | `VideoPane`: `WindowChrome` whose body is `left:3.2, right:3.2, bottom:3.2`, `top: calc(var(--window-frame-top-clearance) + 3.2px)` | Content is inset 3.2 px from every frame edge: a gap ring between content and frame. Also hard-coded `900`, `navbarHeight` and `footerHeight` band maths | Make the body full-bleed (`inset: 0`, edgeless layout) and read band heights from the contract constants | M |
| E-3 | `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🏠️overview/🟦️.tsx:97` | `absolute inset-x-0 border-t border-foreground/50` ground line in a frieze drawing | Drawn content (a ground line), not chrome | Keep. Recorded so it is not mistaken for a hand-drawn frame | L |

### Lane F: OS product (`💻️os`)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| F-1 | `…/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🔷️diagram-node/🟦️.tsx:92-93` | `rounded border border-border …` and `border-b border-border` on a node card | Node card with a hand-drawn frame and a hand-drawn header line (both bypass the compositor) | Use a `SharedBorderRow` frame and cells; drop `rounded` | M |
| F-2 | `…/🌐️World3dHost/🟦️.tsx:233-234` | `rounded px-2 py-1` labels (terrain feature name, attribution) and the scalar-field status `rounded` | Hand-rolled chips over the 3D view: no `data-glass-chip`, so no chip tint | Add `data-glass-chip` and drop `rounded`; or render through the glass chip primitive | M |
| F-3 | `…/🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx:25` | `{ key: "footerLeadingGap", className: "w-huge", content: null }` and `footerTrailingGap` | Empty spacer items named "gap" inside the footer band; the design says gap elements do not exist | Rename to `footerLeadingSpacer` / `footerTrailingSpacer`, or use `navbarFillItem` | L |
| F-4 | `…/🧱️elements/🖋️InkCanvasHost/🧱️ink-block-views/🟦️.tsx:126`, `…/🔺️DiffViewHost/🟦️.tsx:127` | `border-b`/`border-l` cells in a table and a divider | Hand-drawn lines in a table and a divider | Convert to shared-border cells | L |
| F-5 | `…/🧱️elements/👕️canvas-presence/🟦️.tsx:149, 195` | `top: -2` name label | Floating presence label outside the cursor box | Keep as a floating label, or move it inside the presence chip | L |
| F-6 | `…/🧱️elements/✏️TextEditor/🟦️.tsx:257-262` | `absolute z-50 … rounded px-2 py-1` autocomplete popup | Floating menu with a hand-drawn surface | Use `data-level="menu"` and a compositor frame | L |

### Lane G: presentation product (`🎤️presentation`)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| G-1 | `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:10, 92, 146, 206, 215` | `border-radius: var(--spacing-single)` on handles and action buttons | A real corner radius on chrome (`--spacing-single` is a spacing token, not zero) | Set `border-radius: 0` | M |
| G-2 | same file, line 13 | `box-shadow: 0 0 0 var(--stroke-hairline) var(--base)` ring on the resize handle | A ring drawn with `box-shadow`, the design says rings are outlines | `outline: var(--stroke-hairline) solid var(--base); outline-offset: calc(-1 * var(--stroke-hairline))` | M |

### Lane H: pets (`🐾️pets`)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| H-1 | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css:426-438` | `.card .tab { top: 0; right: 0; border-radius: 0 9px 0 0; … }` | Old protrusion fixed, but a 9 px radius remains on a chip | Set `border-radius: 0` | L |

### Lane I: dead utilities and radius cleanup (`🧱️elements` and `🎯️targets` controls)

| # | File:line | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| I-1 | `🧱️elements/📋️MenuItem/🟦️.tsx:15` | `menuItemClassName` has `rounded-sm` on every menu row | Dead class (theme radius 0); menu rows are chrome | Drop `rounded-sm` | L |
| I-2 | `🧱️elements/🖱️ContextMenu/🎨️item-render/🟦️.tsx:49`, `🔘️trigger/🟦️.tsx:35` | `rounded-sm` swatch; `rounded` trigger | Dead classes | Drop | L |
| I-3 | `🧱️elements/👥️PresenceBar/🟦️.tsx:115, 125` | `rounded-full` agent and activity badges, `size-3` | Dead radius; the badges are outlines already | Drop `rounded-full` | L |
| I-4 | `🧱️elements/🎚️Slider/🟦️.tsx:39, 47, 525` | `rounded-full` thumb and track | Control geometry for a round thumb. Dead class renders square | Decide: keep as the slider's own geometry, or drop for the square chrome law | L |
| I-5 | `🎯️targets/⚛️react/🔌️host-ports/🟦️.tsx:530` | `rounded-sm bg-background/80` badge on a port | Dead radius; the badge is an overlay chip without tint | Use `data-glass-chip` | L |
| I-6 | `🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx:49` | `Badge`: `rounded-full border …` | Dead radius, plus a hand-drawn border on a badge chip | Badge in a chip (`data-glass-chip`), no border | L |
| I-7 | `🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx:100` | `StatCard`: `flex flex-col gap-single border p-single` | Hand-drawn border on a stat card (hub uses cells instead) | Make it a `SharedBorderRow` cell; drop `border` | M |
| I-8 | `🧱️elements/🕰️HistoryTable/🟦️.tsx:149, 160` | `rounded px-1.5` level and checkpoint badges | Chip-like labels without tint | Use `data-glass-chip` chips or cells | L |
| I-9 | `🧱️elements/🦴️Skeletons/🟦️.tsx:22, 82, 148`, `📊️Table/🟦️.tsx:577`, `🔣️Icons/🟦️.tsx:356`, `📻️TableAvatar/🟦️.tsx:61-89`, `⚙️VirtualFileSystem/🟦️.tsx:549` | `rounded`, `rounded-full`, `rounded-sm` on placeholders, avatars, missing-icon marker, icon buttons | Content placeholders and avatars (images) are content; the icon button is a control | Drop the dead radius on chrome (`VirtualFileSystem`, `Icons`); keep avatars | L |
| I-10 | `♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx:27, 59, 85` | `rounded-sm` on footer links | Dead class on chrome | Drop | L |

### Lane J: contract leftovers in `🧱️elements/🔲️WindowFrame` and naming

| # | File | Shape | Verdict | Recommended fix | Sev |
| --- | --- | --- | --- | --- | --- |
| J-1 | `🧱️elements/🔲️WindowFrame/junctions.schema.json`, `junctions.cases.json`, `snap.schema.json`, `snap.cases.json`, `snap.test.ts` | Junction cases and snapping. The design removes "junction cases" and "full-span snapping" | Leftover contract files | Delete with their tests when Lane J's owner confirms no consumer; check `shared-border.test.ts` imports first. Non-web (schema) | M |
| J-2 | `🧱️elements/🃏️CardStack/bands.ts` and `bands.cases.json` reference | Name `bands` for a file that now only snaps heights and tests sliver gaps | Stale name | Rename to `card-snap.ts` and update the two imports | L |
| J-3 | `🧱️elements/🗂️WindowChrome/chip-abut.ts` and `[data-chip-abut]` (`UI 🎨.css:7622-7632`) | Chip abutting is still a rule | Consistent with work package 2 ("chip abut") | No action | — |

---

## 3. Lane assignments (disjoint files)

| Lane | Files it owns | Findings |
| --- | --- | --- |
| A | `🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts`, `🔨️modules/🎨️chrome-class-names/🟦️.ts`, `🧱️elements/💡️ChromeControlHint`, `🎯️targets/⚛️react/📣️display-components`, `🎯️targets/⚛️react/📋️canvas-pick`, `🎯️targets/⚛️react/🎥️tutorial-ui` | A-1 to A-11 |
| B | `🧱️elements/📑️Tabs`, `🧱️elements/🖼️Panel`, `🧱️elements/🧭️PanelTabBar`, `🧱️elements/🗂️WindowChrome`, `🧱️elements/🥞️LayeredOverview/💡️hints`, `🧱️elements/🔍️WindowSearch` | B-1 to B-7 |
| C | `🧱️elements/🃏️OverviewCard`, `♻️mit-bestand/🧺️demonstrator/⚛️demonstrator-card.tsx`, `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`, `🏢️semio-tech/🎡️play/⚛️play-card.tsx`, `🏢️semio-tech/🎡️play/🏠️play-landing.tsx` | C-1 to C-4 |
| D | `📚️course/…/📚️app/🟦️.tsx`, `…/🎨️styles/🖥️mockups`, `…/🎨️styles/👥️peer-marks`, `…/🏁️results`, `…/🏆️leaderboard`, `…/📇️profile`, `…/🕸️radar`, `…/📚️app/🛟️load-fallback` | D-1 to D-8 |
| E | `🎓️teaching/…/📚️app/🟦️.tsx` (staffage), `🎓️teaching/…/🌐️browser/🟦️.tsx` | E-1, E-2 |
| F | `💻️os/…/🕸️NodeGraph/🔷️diagram-node`, `…/🌐️World3dHost/🟦️.tsx`, `…/🏛️ShellHost/🖼️canvas-footer/🟠️footer-items`, `…/🖋️InkCanvasHost`, `…/🔺️DiffViewHost`, `…/👕️canvas-presence`, `…/✏️TextEditor` | F-1 to F-6 |
| G | `🎤️presentation/…/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css` | G-1, G-2 |
| H | `🐾️pets/…/📖️stories/🎨️.css` | H-1 |
| I | `🧱️elements/📋️MenuItem`, `🧱️elements/🖱️ContextMenu/{🎨️item-render,🔘️trigger}`, `🧱️elements/👥️PresenceBar`, `🧱️elements/🎚️Slider`, `🎯️targets/⚛️react/🔌️host-ports`, `🎯️targets/⚛️react/🚦️status-surface`, `🧱️elements/🕰️HistoryTable`, `🧱️elements/🦴️Skeletons`, `🧱️elements/📊️Table`, `🧱️elements/🔣️Icons`, `🧱️elements/📻️TableAvatar`, `🧱️elements/⚙️VirtualFileSystem`, `♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx` | I-1 to I-10 |
| J | `🧱️elements/🔲️WindowFrame/{junctions,snap}.*`, `🧱️elements/🃏️CardStack/bands.ts` | J-1, J-2 |

Each file appears in one lane only.

---

## 4. Not verified, and out of the web lane

- Print LaTeX (`🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty`): the pre-audit P1-P11 rows need a print-lane recheck. Quick grep: `\semio@window@cap@raise` is gone; `\semio@chrome@chip` (P10, lines 3090-3140) remains with edge options `lr`.
- wgpu and TUI targets (Rust): not in this lane. The `chip_glass` and `WindowFrame` Rust crates are unverified (summary section, Verification).
- Browser verification of lines flagged `verify`: B-2, B-4, B-5, B-6, C-1 lift state, D-3, E-2 inset.
- Pets ground strip above the footer (summary section 4, open): not found in the grep; not checked.
- Summary section 5 open: tree rows (`🌳️Tree/🧲️drag-drop`) and canvas (`🎨️Canvas/🖱️mode-drag…`) already use `pointer-drag/ghost.tsx`, so the drag law is applied there; the panel dock drags were not traced.

## 5. Order of work

1. H items first: C-1 (remove `lifted`, which also fixes C-2 and C-3), A-8, D-1, E-1.
2. M items by lane, in parallel: A, B, D, E, F, G, I-7.
3. L items last, as a single cleanup pass per lane.
