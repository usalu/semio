# 🔎 Audit: Unnamed OLD Silhouette Chip Patterns

Read-only audit. No source file was modified and no state-changing git command was run.

Paths below are relative to `/Users/ueli/Documents/semio/`. Each row is a verdict on the shape only.

## 🧭 Method and scope

- Searched TS/TSX/CSS/STY/TEX/SVG (Python only for the print product) in the seven requested areas.
- Patterns: negative offsets (`-top-`, `-bottom-`, `-translate-y`, `top: -n`, `-mt-`/`-mb-`), `bottom-full`/`top-full`, `clip-path`/`polygon`, and chip/cap/tab/badge/heading/kind/footer rows passed as cap or footer props.
- Excluded on purpose: anything named `silhouette`, `WindowChrome`, `window-chrome-*`, `chip-abut`, `window-silhouette-*`, since other agents migrate those.
- Excluded as not chip shapes: `.py` teaching/bim and clip code (its `_chip` is a manim label), vendor CSS (`♻️mit-bestand/.../📐️layout.css`, Flickity/Bootstrap), and `.rs` declaration files (outside the requested extensions; same band model as `CardStack/bands.rs`).
- Named pattern owners (for reference, not in tables): `🗂️WindowChrome/🟦️.tsx` lines 98, 283-290 (`titleChips`, `capRightChips`, `footerLeftChips`, `footerRightChips`), `🔲️WindowSilhouette/🟦️.tsx`, CSS `window-silhouette-*` at `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` lines 1349-1435, and CSS `[data-window-silhouette-chip][data-chip-abut]` at lines 7351-7368.

Verdict legend: **OLD PATTERN** = chip/tab/cap/footer row sits outside or on the edge of its content rectangle, or is a sibling row above/below a bordered body. **Already inside** = chips are cells or overlays inside the rectangle. **Not a chip shape** = labels, cells, or geometry that is not a chip on a box edge.

---

## 1. UI module `🧰️framework/🔨️modules/🖱️ui` (OLD PATTERN hits: 11)

| # | Confidence | File (absolute) | Lines | Shape | Verdict | Excerpt |
|---|---|---|---|---|---|---|
| 1 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️CardStack/🟦️.tsx` | 11-13, 29-34, 85-90, 159-165 | Cards stacked; each card's cap row (above) and footer row (below) are lifted onto the neighbour's bottom line, one shared band | OLD PATTERN | `* The next card's cap row starts on the previous card's bottom line, sharing one band with the previous card's footer chips` |
| 2 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️CardStack/bands.ts` | 16, 43-46, 55 | Band model (`none`/`shared`/`stacked`) for cap and footer chip rows outside the card body | OLD PATTERN | `* cap row below, so the lower card's cap row starts on the upper card's bottom line` |
| 3 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx` | 15-17, 60-65, 77-78, 96-97, 121, 128-129 | Title chip on the card's top edge, footer chips below, built on the named WindowChrome | OLD PATTERN | `title chip (glyph and title), a body, and optional chips at the bottom left and right` |
| 4 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx` | 595-608 | Tab row as its own framed shared-border row, fed into the chrome cap (`titleChips`) | OLD PATTERN | `<div ref={frameRef} data-slot={`${tabSlot}-tab-frame`} ... data-shared-border-edges` |
| 5 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowFrame/🟦️.tsx` | 9-16, 38-40, 117-122 | Frame with top/bottom chip regions docked on the frame edge (chips straddle the border) | OLD PATTERN | `for (const chip of normalized.top.chips) regions.push({ ... dock: "top" })` |
| 6 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowFrame/shared-border.ts` | 161-185 | Outline split around the chip regions so chips replace the frame edge | OLD PATTERN | `Registers the four frame edges and the inner edges of every chip region` |
| 7 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/⚓️dock/🟦️.tsx` | 265-267, 330-333 | Corner mode-dock tab bars passed as `titleChips`, `capRightChips`, `footerLeftChips`, `footerRightChips` | OLD PATTERN | `titleChips={renderCornerBar("topLeft")}` |
| 8 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` | 511, 514, 547 | Panel tab bar used as the chrome's `titleChips` cap | OLD PATTERN | `titleChips={<PanelTabBar anchor={anchor} activePath={resolvedPath} ...` |
| 9 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📌️WindowPane/🟦️.tsx` (307) and `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🟦️.tsx` (146) | 307 / 146 | Pane and skeleton chrome passing `titleChips` | OLD PATTERN (usage) | `titleChips={` |
| 10 | High | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎓️introduction/🟦️.tsx` | 352, 421-422 | Introduction info box with title chip and footer chips (button groups as footer chips) | OLD PATTERN (usage) | `footerLeftChips={stepIndex > 0 ? <div data-shared-border-cells="" ...` |
| 11 | Medium | `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️.tsx` | 186-208, 248 | Selected tab trigger outlines itself on all four edges; tabpanel below is borderless, so the active tab reads as a separate box sitting above the content | OLD PATTERN | `data-shared-border-edges={selected ? "top right bottom left" : ""}` |

Notes, not OLD:
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🥣️chrome/🟦️.tsx` lines 8, 20-21: title chip is overlaid on the top row of one rectangle with glass. **Already inside.** The `top: calc(y - var(--chip-row-height))` placement in `.../🖱️ContextMenu/🟦️.tsx` lines 313, 355 moves the whole rectangle, not the chip out of it.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow/🟦️.tsx` lines 17-24, 85-90: framed row with cells. **Already inside.**
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎬️introduction-demo/🟦️.tsx` lines 412-431 with CSS `🎨️styling/🖌️ui/🎨️.css` lines 1647-1700: kbd chips stacked as cells in one frame inside the callout. **Already inside** (medium).
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx` line 14 and `.../🔨️modules/🥞️layered-overview-geometry/🟦️.ts` line 227: glass veil with a `clip-path` hole. **Not a chip shape.**
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx` line 389: safe-area reservation `block: "var(--chip-row-height)"`. **Not a chip shape** (padding only).
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` lines 73-82: `--chip-row-height` band sizing. Supports the old band model, but is not a shape by itself.

---

## 2. Print product `🧰️framework/🛍️products/📓️print` (OLD PATTERN hits: 11, plus 1 consumer)

All in `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty`, unless marked.

| # | Confidence | Lines | Shape | Verdict | Excerpt |
|---|---|---|---|---|---|
| P1 | High | 426-428 | Title tab sits on the component's top edge; frame overlapped onto the tab | OLD PATTERN | `% LR-2.14 seam: overlap the content frame onto the tab ... the tab must SIT ON the component, not float above it.` |
| P2 | High | 886-889 | Title tab welded on immediately above the content box, zero `before skip` | OLD PATTERN | `% LR-2.14 seam: the title tab is welded on immediately above this box` |
| P3 | High | 1319-1321 | Title tab welded onto the frame for the whole-keep split start | OLD PATTERN | `% Title tab welded onto the frame, for the whole-keep split start.` |
| P4 | High | 2445-2447 | `\semio@window@cap@raise`: raisebox upward by full cap height, so the cap stands above the body | OLD PATTERN | `\raisebox{0pt}[\semio@window@cap@titleh][0pt]{#1}` |
| P5 | High | 2501-2521 | `\semio@window@cap` title/accent tab: side strokes and cap box above the frame top rule | OLD PATTERN | `\newcommand{\semio@window@cap}[3]{ ... \semio@window@cap@accent@vbox{#1} ...}` |
| P6 | High | 2523-2540 | `\semio@window@cap@open` open-bottom tab that shares the row baseline | OLD PATTERN | `% Open-bottom solid chip for pairs ... the row's shared full-width rule supplies the chip's bottom edge` |
| P7 | High | 2657-2694 | Muted caps for window, table, figure, contents and heading titles (`\semio@heading@cap@muted@core`, `@muted`) | OLD PATTERN | `\semio@window@cap@raise{\semio@window@cap@muted@vbox{semio-chrome-canvas}}` |
| P8 | High | 2725-2755 | Section and appendix heading chips (`\semio@heading@chip`, `@open`), solid and outline | OLD PATTERN | `\newcommand{\semio@heading@chip}[3]{%` |
| P9 | High | 3202-3221 | Header gap connector joining the title tab and the number/key tab on the component's top edge | OLD PATTERN | `% Open connector for MUTED title bars ... the two chips are tabs sitting on the component's top edge` |
| P10 | High | 3626-3650 | `\semio@chrome@chip`: navbar and footer chips with side strokes, standing on a rule | OLD PATTERN | `\hbox{\semio@chrome@chip@rule} ... \hbox{\usebox{\semio@chrome@chip@body}}` |
| P11 | Medium | 3751-3767 | `\semio@chrome@navbar@bar` / `\semio@chrome@footer@bar`: chip row stacked above the bar rule (navbar) or below it (footer) | OLD PATTERN | `\hbox to \headwidth{#1}%` then `\hbox{\semio@window@stroke@h{...}{\headwidth}}` |

Consumer (not a shape, but the window kind tab that carries the number):
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty` lines 1342-1348: `\semio_kind_chip_number:nn` inside window kind tabs (figure/table tab text). **OLD PATTERN via P5/P7**, counted as a consumer.

Not chip shapes:
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-core.sty` lines 354-363 and 530-568: heading/kind chip text macros. **Not a chip shape** (text only, the shape lives in semio-window.sty).
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-table.sty`: table header and register cells; no tab shape. **Not a chip shape.**
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-composition.sty` lines 634-720: chain "chip" is a table cell kind. **Not a chip shape.**
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-api/🔓️viz-api.tex` and the `♻️mit-bestand` `.tex` files: consumers of `\SemioTableLong` only. **Not chip shapes.**

---

## 3. Presentation product `🧰️framework/🛍️products/🎤️presentation` (OLD PATTERN hits: 0)

| File (absolute) | Lines | Shape | Verdict |
|---|---|---|---|
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🔨️modules/🔗️shared-border-chips/🟦️.tsx` | 1-25 | `SharedBorderChipGroup`: one four-edge frame, chips are cells, seams are single hairlines | **Already inside** |

Only the shared-border file was read in detail. The other presentation hits (about 12 raw) were not individually read.

---

## 4. Course product `🧰️framework/🛍️products/📚️course` (OLD PATTERN hits: 2)

| # | Confidence | File (absolute) | Lines | Shape | Verdict | Excerpt |
|---|---|---|---|---|---|---|
| C1 | High | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx` | 2, 14, 20-29 | Card with title chip (cap) and action chips in the footer, built on `OverviewCard` and `CardStack` | OLD PATTERN | `/** 🧷️ What lets a chip in a card's footer give way ...` |
| C2 | High | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📚️course/🎨️styles/🏠️chrome-and-home/🎨️.css` | 5-11 | `course-card-title-chip` cap styling on the card title chip | OLD PATTERN (usage) | `.course-app [data-slot="course-card-title-chip"] {` |

Not chip shapes or not verified:
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/👥️peer-marks/🎨️.css` lines 31-49: name label above a peer control (`top: -1.25rem`). **Floating label, not a box chip.** Review separately.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css` line 3 and `.../🧩️task-layouts/🎨️.css` lines 100-110 (`.course-chip`): chips inside rows. **Already inside** (selector only read).
- `.../🏅️badges/...` hits: the word "badge" only. **Not a chip shape.**

---

## 5. Operating system `🧰️framework/🛍️products/💻️os` (OLD PATTERN hits: 1)

| # | Confidence | File (absolute) | Lines | Shape | Verdict | Excerpt |
|---|---|---|---|---|---|---|
| O1 | Medium | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx` | 22-23, 50 | Panel tab bars (`bottomLeftPanelTabs`, `bottomMiddlePanelTabs`) placed as tab rows in the canvas footer band | OLD PATTERN | `{ key: "bottomLeftPanelTabs", ...` |

Not chip shapes:
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/👕️canvas-presence/🟦️.tsx` lines 146-149, 192-195: presence name label at `top: -2`. **Floating label, not a chip shape.**
- `.../🏛️ShellHost/📋️panel-hosting/🔶️brand/🟦️.tsx` line 118: `surface-role-chip` in a header line. **Not a chip shape** (inline pill, not on an edge).
- Other OS hits (`ChromePanels`, `notice-bands`, `ShellHost`) are mostly code names (`capability`, `captions`). **Not chip shapes.**

---

## 6. Plugins `✏️s/🔌️plugins/*` (OLD PATTERN hits: 0)

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/.../✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx` line 31: doc comment about a toggle section. **Not a chip shape.**
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/.../🪟️presentation/🟦️.tsx`: `<h2>` heading only. **Not a chip shape.**
- `🧩️puzzle`, `🗄️stdio`, `📕️norm`, `🏙️bim`: hits are code identifiers or `.py`/`.ts` geometry. **Not chip shapes.**

---

## 7. Other areas

| Area | File (absolute) | Lines | Shape | Verdict |
|---|---|---|---|---|
| `🏢️semio-tech` | `/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play/⚛️play-card.tsx` | 1-4, 41 | Play card as an `OverviewCard`; open chip in footer right | OLD PATTERN (usage of OverviewCard, medium) |
| `♻️mit-bestand` | `/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/⚛️demonstrator-card.tsx` | 1-4, 34 | Demonstrator card as an `OverviewCard`; open chip in footer right | OLD PATTERN (usage, medium) |
| `♻️mit-bestand` | `/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🎨️globals.css` | 5-18 | Title chip hover rule on the card (`demonstrator-pane-card-title-chip`) | OLD PATTERN (consumer, medium) |
| `🌎️hub` | `/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/` (`🏛️SpacesPage`, `🔗️ConnectionsPage`, `📄️AdminPage`, `📊️StatTiles`, `🏠️OverviewPage`) | hits | `SharedBorderRow` cells and labels (`SharedBorderRowLabel`) | **Already inside** (no OLD hits) |
| `🎓️teaching` | `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx` | 35, 45 | Title chip and `OverviewCardOpenChip` in footer of a presentation card | OLD PATTERN (usage, medium) |
| `🎓️teaching` | `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🎨️.css` | 62-68 | `staffage-card-title-chip` cap styling | OLD PATTERN (consumer, medium) |
| `🎓️teaching` | `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx` | 12, 17, 74-270 | `SharedBorderRow` and fields | **Already inside** |
| `🧰️framework/🛍️products/🐾️pets` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css` | 426-429 | `.card .tab { position: absolute; top: -16px; right: 24px; background: var(--page); }` — tab protrudes above the card | OLD PATTERN (high) |

Pets is listed here because it is a product outside the seven requested folders (checked as a related product). The pets tab is the one unnamed protruding tab outside the UI and print code.

---

## 🏁 Summary, ranked by confidence

1. High, outside tests: UI `CardStack` cap/footer band model (`CardStack/🟦️.tsx`, `bands.ts`), `OverviewCard`, `WindowFrame` chip regions, `PanelTabBar` as chrome cap, `Canvas/dock` corner rows, print `semio-window.sty` tabs and heading chips (P1-P9), pets `.card .tab`.
2. High usage: `titleChips`/footer-chip consumers (`Panel`, `WindowPane`, `Skeletons`, `introduction`, course chrome, teaching browser, `play-card`, `demonstrator-card`).
3. Medium: `Tabs` selected box (UI, row 11), print navbar and footer chip bars (P10, P11), OS canvas footer tab bars (O1).
4. Low, review: peer-marks name label (course), presence label (OS), `chip-row-height` band sizing (UI CSS). These are floating labels, not chip shapes.

## 🔢 OLD PATTERN hit counts per area

| Area | OLD PATTERN hits |
|---|---|
| UI module (`🖱️ui`) | 11 |
| Print (`📓️print`) | 11 (+1 consumer) |
| Presentation (`🎤️presentation`) | 0 |
| Course (`📚️course`) | 2 |
| OS (`💻️os`) | 1 |
| Plugins (`✏️s/🔌️plugins`) | 0 |
| `🏢️semio-tech` | 1 |
| `♻️mit-bestand` | 2 |
| `🌎️hub` | 0 |
| `🎓️teaching` | 2 |
| Other: `🐾️pets` (related) | 1 |
| **Total** | **31** (plus 1 print consumer) |

## ⚠️ Limits of this audit

- Not every raw grep hit was read. Files whose only hit is a word match (`badge`, `label`, `SharedBorderRow`, `capability`) were classified from the matched line, not the whole file.
- Presentation (12 raw hits) and OS (about 330 raw hits, mostly identifiers) were sampled, not exhausted.
- Pattern owners named `silhouette`/`WindowChrome`/`window-chrome-*` are intentionally excluded, so the migration list for them is outside this report.
