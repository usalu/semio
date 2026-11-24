# Glassy Chip Layer and Border-Preserving Hover Emphasis

Two UI laws, found on the architecture and technology course site, fixed in the shared UI module.

## 1. A chip never has the background of the surface it sits on

**Finding.** Chips already took the glass of the next level (`windowFrameChipLevel`), but measured on the cards overview the chip and the card body beside it had the same colour (chip `30,55,62`, body `32,55,61`). The next level's tint is 5 % lighter but 12 % more transparent, and the chip's own `saturate` darkens the already saturated body behind it, so the two cancel.

**Law.** A chip lies one layer above what it covers. Over its level glass it lays a veil of the foreground colour at `--chip-tint` (10 %). The veil shifts the chip away from whatever is beneath it by the same share at every level, so the difference no longer depends on the level formula, and the blurred content stays visible through it.

**Where.**

| What | File |
| --- | --- |
| Knob `--chip-tint` and the chip rule on `[data-window-frame-chip]`, `[data-glass-chip]` (forced colours: no veil) | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` |
| Token `levels.chipTintPercent`, projection, Rust constant `levels::CHIP_TINT_PERCENT` | `🎨️styling/🔣️.json`, `🎨️styling/📽️projection/🟦️.ts`, `🎨️styling/🔤️tokens/🦀️.rs` |
| `Level::above` and `Theme::chip_glass(content)`: level glass and veil folded into one tint and alpha | `🎨️styling/🎭️theme/🧩️component/🦀️.rs` |
| wgpu chip sites use `chip_glass(Level::Base)` | card stack paint, OS Shell pane chips, time-travel chips, Dock tab groups |
| Free-standing glass chips marked `data-glass-chip` | `🥞️LayeredOverview` hints and overview button |

## 2. A hover never adds a border line

**Finding.** The border compositor closed the whole perimeter of every stated box, hover included. A borderless box (tab, tree row, menu item, icon button) and a cell whose sides lie on undrawn edges of its owner (navbar and footer controls, grouped buttons) therefore got lines on hover that do not exist at rest.

**Law.** A hovered box emphasises the lines that lie on its perimeter without the hover, its own and those of other boxes (a seam, a divider, the frame line under a tab), and adds none. Focus, active, invalid and the other states still close all four sides. A borderless box answers a hover with its foreground.

**Mechanism.** The sides a hovered box does not draw are *emphasis* segments: they enter resolution, win the colour of every span they share with a drawn segment, and spans only emphasis contributes to are dropped (`withoutLoneEmphasis`, Rust `box_outline_emphasis_segments`).

**Where.**

| What | File |
| --- | --- |
| Law in TypeScript: `borderStateCloses`, `boxDrawnSides`, `withoutLoneEmphasis`, `boxOutlineSegments` | `🖱️ui/🔨️modules/📏️border-presentation/🟦️.ts` |
| Law in Rust: `border_state_closes`, `box_outline_emphasis_segments`, `box_outline_segments` | `🖱️ui/🔨️modules/📏️border-presentation/outlines.rs` |
| Language-neutral vectors (hover on undrawn parent edges, focus closing them, borderless hover, neighbour seam) | `outlines.cases.json` |
| React compositor: emphasis sides, `data-shared-border-hover` is a plain presence marker | `🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`, `🔝️Navbar`, `🔚️Footer`, `🃏️OverviewCard` |
| Foreground emphasis of hovered borderless boxes (components layer) | `🎨️styling/🖌️ui/🎨️.css` |

The wgpu and TUI targets were audited: no site adds a line on hover (controls and chips are framed at rest and only recolour). One wgpu paint-unit test now asserts the new law.

## 3. No floating element: every element is a rectangle whose edges are its neighbours' or its container's

**Finding.** Three leftovers of the old design system on the course site:

- `CardStack` had a `maxWidth` and centred itself (`mx-auto`), so every page behind a card (quiz start, learning clip, assignment, badges, leaderboard, learner) and every screen other than home (run, results) was a column floating between two empty margins.
- The scrolling panes (`course-home-grid`, `course-page-frame`, `course-page-scroll`) reserved the 10 px gutter of a native scrollbar, so the last column of a grid and every page card ended 10 px before the right edge.
- The modal `Dialog` was a padded, centred card of `max-w-md` or `max-w-2xl`.

**Law.** An element is a rectangle with a border that tiles its container; only the content of a rectangle is centred or aligned.

**Where.**

| What | File |
| --- | --- |
| `CardStack` is as wide as its container: `maxWidth` and the centring are gone | `🖱️ui/🧱️elements/🃏️CardStack/🟦️.tsx` and its test |
| `Page` and `PageFrame` lose `wide`; callers updated | course `📚️app`, `🪟️chrome`, `🏅️badges`, `🏆️leaderboard`, `📇️profile`, `📝️assignments` |
| The three scrollers carry `ui-scrollbar-hidden` | course `🏠️home`, `📚️app`, `🪟️chrome` |
| `Dialog` is a band from the left to the right edge of the client; its content keeps the reading width, centred inside (a `flush` dialog's rows span the band) | course `🪟️chrome` |

**Checked in the browser.** No scroller reserves a gutter, no card stack has a maximum width or margin, and of 62 card rows across the panes none ends before the right edge of its pane; the quiz start page spans the client and its last card ends on the footer line.

The identity dialog spans the client (0 to 1024 px of 1024) with its text centred and its chips in its corners.

**Open.** The first-visit language chooser (`📚️app`, `mx-auto max-w-xl` inside a padded `main`) is still a centred block. The hidden scrollbar gutter also removes the scrollbar thumb of these panes; an overlay thumb (`Scrollable`) would bring it back without a gutter.

## 4. The navbar and the footer are glass bands one layer above the canvas

**Finding.** Both bands painted the base surface and lay in the flow above and below the main area, so they had the colour of the canvas and no content ever ran beneath them.

**Law.** The bands lie on a layer above the canvas: they are glass with the chip tint, and the content runs beneath them from the top edge of the client to its bottom edge, visible through the glass while it scrolls.

**Where.**

| What | File |
| --- | --- |
| `Navbar` and `Footer` are `ui-glass` at panel level with `data-glass-chip`; the base-floor fill is gone from both | `🖱️ui/🧱️elements/🔝️Navbar`, `🔚️Footer` |
| The course footer is the same glass | course `⚖️legal` |
| The client publishes the band heights (`--course-band-top`, `--course-band-bottom`); the main area runs beneath both bands, and every scroller keeps that clearance as padding and scroll padding | course `📚️app`, `🎨️styles/⚖️footer-and-forced-colors` |
| The last card of a page fills down to the footer line (its frame now stretches with its section) | course `🎨️styles/🏁️results-and-preferences` |

Panel level was chosen over window level: window glass is 88 % opaque, through which scrolled content does not show; panel glass is 64 %.

**Checked in the browser.** Main area spans 0 to 768 px of 768 with the navbar (0 to 29) and the footer (743 to 768) over it. At rest the first card starts on the navbar's bottom line. With a page taller than the client, the scrolled content shows through the navbar and the page continues beneath the footer.

**Open.** While the pets are shown, the 48 px ground strip above the footer stays, so content ends above the strip and only the canvas lies beneath the footer. A band note under the navbar keeps the clearance itself; that case was not opened in the browser. The wgpu shell still paints its navbar and footer as before. The UI component suites that assert the bands' level were not run (they need the DOM suite runner).

## 5. A dragged element is detached and its drop places are shown

**Finding.** In the course tasks the dragged element stayed in its place, only dimmed to 40 %, while a copy followed the pointer; and only the one zone under the pointer was marked, so the places where it could be dropped were not visible.

**Law.** A dragged element leaves its place and travels with the pointer on a layer above the page; while it travels, every place where it can be dropped is shown, the one under the pointer more strongly.

**Where.**

| What | File |
| --- | --- |
| The source is hidden in place (it keeps its room, so nothing jumps under the pointer); the travelling copy is menu-level glass with the chip tint; the task of the source carries `data-course-dragging` for the length of the drag | course `🤏️drag/🟦️.ts` |
| The copy is outlined as the one floating rectangle; every `[data-course-drop]` of the dragging task is tinted (10 %), the one under the pointer stronger (32 %) | course `🎨️styles/🤏️drag-and-spider` |

All three draggable task kinds (classification, matching, sorting) use this one drag path.

**Checked in the browser.** A matching task of the heating quiz, dragged with synthesised pointer events: the source's place is empty, the copy floats outlined at the pointer, all six drop rows are tinted and the one under the pointer is stronger; Escape removes the copy, the tint and the hidden state. A classification task was not opened.

**Correction after the first round.** In a classification task the copy did not appear: the dragged element only vanished and came back where it was dropped. The copy's wrapper is an empty clone of the source's list, and the classification list is a shared-border group and cell, which the design system positions `relative`; that overrode the wrapper's `fixed`, so the copy lay in the page flow about 1235 px down, outside the view. The wrapper now keeps only the list's classes. The first check had used a matching task, whose list carries no such markers. Re-checked on the classification task of the physics quiz: the copy is `fixed` and sits at the pointer.

**Open.** The drags of the design system itself (panel dock, canvas tabs, tree rows) were not reviewed against this law.

## 6. Chips lie on top of the content, never beside it

**Finding.** On the learning clips grid the poster of a card ended 24 px below the top edge and 24 px above the bottom edge, so the title chip and the "coming soon" chip sat in two strips outside the picture. The card's content plane kept the chip row clearance that text content needs at rest.

**Fix.** The filling poster is edgeless content (`data-window-content-layout="edgeless"`, the design system's own marker): the content plane drops the clearance, the poster reaches every edge of the card and the chips lie on top of it. File: course `🎬️clips/🟦️.tsx`.

**Checked in the browser.** Poster spans 0 to 360 px of a 360 px card, with the chips at 0 to 24 and 336 to 360 on top of it.

**Open.** Cards whose content is text keep the chip row clearance at rest by design (the text would otherwise start under the title chip); they were not changed.

## 7. No rounded corner, no shadow

**Finding.** The theme's radius and shadow tokens were already zero, so `rounded-*` and `shadow-*` utilities paint nothing. What still rounded or shadowed:

- native tick boxes and radio buttons, which the browser draws itself (rounded box, round radio);
- explicit values: the hint symbol (`50%`), the peer badge and the mockup scroller radius in the course; a drop shadow on the introduction callout; and several `box-shadow` values used as rings or marker bars.

**Fix.**

| What | File |
| --- | --- |
| Native `input[type=checkbox]` and `input[type=radio]` are drawn by the design system: squares with the element line; a ticked box is filled with the active colour and carries its tick, a chosen radio holds a smaller square. Forced colours keep the system controls | `🖱️ui/🎨️styling/🖌️ui/🎨️.css` (base layer) |
| Introduction callout drop shadow removed; zoom ring is an outline | same file |
| Presence avatar and badge rings, the menu colour swatch line and the port marker ring are outlines instead of `box-shadow` | `👥️PresenceBar`, `🖱️ContextMenu/🎨️item-render`, react `🔌️host-ports` |
| Hint symbol, peer badge, mockup scroller: radius removed; own-row and alert marker bars are a background bar instead of an inset shadow; selected workbook cell is an outline | course `🎨️styles` |

**Checked in the browser.** On the quiz start page 347 visible elements of the client have a corner radius of 0; the challenge radios and the footer tick box render square. The only computed `box-shadow` values left are the zero-size transparent ones Tailwind composes for `shadow-none` and rings.

**Left as they are.** The Grasshopper mockup shapes in the quizzes (`.course-graph-node`, the parameter pill, the slider track) depict another program's interface. An `ellipse` port shape of diagrams (`rounded-[50%]`) is geometry of the drawn content. Both are content, not chrome; they keep their shapes until told otherwise.

## 8. Borders, chips and toggle cells instead of loose controls

**Finding.** On the quiz start page the challenge chooser was a padded block inside the quiz card: a bold legend, four radio buttons with text beside them, no line between the choices. Tick boxes elsewhere (preferences, the pets switch of the footer) were a small box beside a label.

**Law.** A named group is a card whose name is its title chip. Its choices are cells separated by the card's single lines. A choice and an on/off setting are toggle cells: the whole cell is the control, with an icon and its label, filled with the active colour while it is on, the way the task of a run is. No tick box or radio button is painted beside a label.

**Where.**

| What | File |
| --- | --- |
| `RadioChoice` (toggle cell of a radio group, icon, name, describing line) and `ToggleCell` (on/off setting, icon, label); the native control stays the real one and is visually hidden | course `🪟️chrome` |
| The challenge is a card of its own, titled by a chip, holding the four toggle cells and the start actions in its footer chips; the quiz card keeps the description and the facts | course `📖️quiz-page`, `⛰️challenge` |
| `CHALLENGE_ICONS`: hint, values in sight, values hidden, clock | course `⛰️challenge/🏷️labels` |
| Identity kinds use `RadioChoice` with icons under a heading cell | course `🪪️identity` |
| The four preference tick boxes and the footer's pets switch are `ToggleCell`s with icons | course `🎛️preferences`, `🐾️pets` |
| Toggle cell fill, hover and hint colour; choice grid without gaps | course `🎨️styles/🏁️results-and-preferences` |
| A native tick box or radio button that is a direct child of a cells owner is a cell of the border grid (no own line, as high as its row) | `🖱️ui/🎨️styling/🖌️ui/🎨️.css` |

**Checked in the browser.** Physics quiz page: the "Herausforderung" card with its title chip, four bordered cells with icons, the chosen one filled, the start action following the choice; the footer's pets switch is a filled toggle cell with an icon.

**Open.** The preferences page and the identity dialog were changed but not opened. "Everything has an icon" was applied to the toggles and choices touched here; other controls without an icon (text rows such as "Jetzt schon anzeigen", the footer link, some action chips) were not swept.

## 9. Every control carries an icon

**Law.** A button or link of the course never consists of words alone.

**Mechanism.** `icon` is a required prop of the four shared controls (`CardAction`, `BodyButton`, `CardLink`, `RowLink` in course `🪟️chrome`), which render it before their words, so a control without an icon no longer type-checks. All 64 call sites carry one; sites that held an inline `CardIcon` hand it over as the prop instead.

**Hand-built controls.** The footer's legal links (lock, external link), the page picker's open button, "stop following", the remove button of a matching row and the up and down buttons of a sorting row (icons instead of the characters ✕ ↑ ↓).

**Choices made.** Icons come from the design system's set: cancel, close and dismiss are `x`; start and watch `play`; resume `skip-forward`; submit and confirm `check`; retry and reload `rotate-cw`; results `bar-chart-3`; leaderboard and badges `award`; download `download`; open in a tab `external-link`; language `globe`; pet deeds `sparkles`; show or hide what others answered `eye` / `eye-off`.

**Checked in the browser.** Home, a quiz page and a running task compile and render; a scan of the visible buttons, links and toggle cells on those three screens found none without an icon except card title links (their icon stands beside them in the title chip), the task tabs (emoji glyphs) and the presence rows (colour mark).

**Open.** No TypeScript check was run, so the required prop is verified by the scan of call sites and by the pages rendering, not by the compiler. Results, leaderboard, badges, profile, preferences, assignments, the dialogs and the load-fallback screen were not opened. The fallback screen's two plain buttons and the leaderboard's sort headers were left as they are.

## 10. One line thickness everywhere

**Measured.** On the quizzes page every painted line is the compositor's SVG stroke at `--stroke-hairline` (1 px, non-scaling): 47 of 47 visible paths; no element of the client paints a CSS border or a pseudo-element line there. So the thickness is one value; what differed was how much of a line showed.

**Finding.** Since the navbar and the footer lie above the content, a line the content shares with a band (the navbar's bottom line, the footer's top line, and the top or bottom lines of the cards and chips that touch them) straddles the band's edge. While the band painted it, it showed whole. As soon as the content won it (a hovered card paints its outline in the hover state from its own overlay), half of it lay under the band's glass: the line looked half as thick, dimmer, and switched back and forth with the pointer.

**Fix.** A line shared with an owner inside an elevation root (`data-elevation-root`, which the navbar and footer of the design system carry and the course footer now carries too) is always painted by that owner, in whatever state won it. File: `🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`.

**Checked in the browser.** With a card under the navbar hovered, all nine horizontal segments on the navbar's bottom line are painted from the band (five in the hover state), none from the content. Compositor tests unchanged: 45, 11 and 9 pass.

**Not a thickness difference.** In the two screenshots the brighter lines belong to the hovered card and the greyer ones to cards at rest; both are 1 px.

**Open.** The footer's seam was not measured (its lines had not been painted when the script ran). Lines on other screens, at display scales other than 2, and the flicker the user saw elsewhere were not examined.

## Verification

- `outlines.test.ts` 9, `states.test.ts` 16, `segments.test.ts` 13 pass; `shared-border.test.ts` 45 pass, `focus-boxes.test.tsx` 11 pass. The remaining failures of those two files (9 and 2) and of the component suites are `document is not defined` under the graph-free `repo test-file` route and are independent of this change.
- Rust: not run yet. `repo cargo -- test -p semio-ui-border-presentation` refuses the crate (it has dev-dependencies and is not a workspace member); the graph route `repo run @semio-tech/ui-border-presentation-rs:test` was started and had not reported when this was written. The theme and wgpu crates touched by `chip_glass` are not compiled yet either.
- Third-party oracles: lodash interval union (closure and "hover leaves the merged line geometry unchanged"), Ajv for the vectors.
- Browser, course site: 74 hovered controls, cards and cells compared against the lines at rest, no hover line outside an existing line; chips on the cards overview are visibly lighter than the card body.

## Decisions

- The hover law is scoped to hover. Keyboard focus keeps the closed outline, since a borderless box has no other focus indicator.
- Chips keep the full outline on hover in the chip pass of the compositor: all four chip edges are frame or chip-region lines at rest.
- The hover fill (`--hover-interactive-fill`) of navbar and footer controls is untouched.
