# 🔍 Shared Border Audit: Products and Plugins (excluding 📚️course)

Read-only audit. No source file was edited. Scope: `🧰️framework/🛍️products/*` (presentation, print, pets, os, repo, server, tests), `✏️s/🔌️plugins/**` (UI only), `🌎️hub`, `🎓️teaching` (non-course sites), `🏢️semio-tech`. Ignored: `node_modules`, generated output, tests, fixtures, stories where noted.

Mechanism referenced: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` (`[data-shared-border-cell|frame|group]` L6655-6740, `@layer shared-border-priority` and `shared-border-ownership` L7335-7380) and `🧱️elements/🗂️WindowChrome/shared-border.ts`.

## Method and coverage

- Grepped every in-scope `.tsx/.ts/.css/.html` for border tokens (`border*`, `divide-*`, `-ml-px`, `border-[lrtb]-0`, `inset` shadows, `shared-border*`), about 400 hits, most of them in non-UI code (SQLite snapshots, standards text, 3D mesh/glb bounds, `box-sizing: border-box`).
- Read the UI hot spots in full: presentation interaction and PDF-pager CSS and TSX, teaching `chrome`, `stage`, `playground`, `browse`, `tables`, `export-panel`, hub admin, CAD REPL aside, OS `BlockListHost`, `TextEditor`, `ShellHost` floating layer, `canvas-footer`, `AgentChatPanel`, `dag-overlays`, `ink-overlays`, pets stories CSS.
- Long tail not read line by line: the 3D `World3dHost` internals (mesh, instance-layer and glb code, where borders are per-mesh geometry, not DOM seams) and the `stdio`/`bim` artifact code (non-UI).
- Product usage of the mechanism: `data-shared-border-*` is used by no in-scope product or plugin UI. The only in-tree users are the UI library `Tabs` (`🧱️elements/📑️Tabs/🟦️.tsx` L155, L205-207) and the teaching energy browser clip, which reads `data-shared-border-root` set by the compositor. So every product group below is hand-rolled `gap` plus per-cell `border`, and it does not take part in the seam colour priority (hover/focus/active).

## Summary of priorities

1. High-visibility, medium-high confidence: the presentation PDF pager (prev/next are two fully bordered chips with a `0.5rem` gap) and the presentation interaction actions (reset and enlarge, two bordered chips with a gap).
2. Medium-high confidence: the CAD REPL aside overrides its shared frame per side (`border-l-0 border-t`) instead of compositor attachment.
3. Medium, needs a runtime check: the OS block palette and its block-card column, and the teaching `Button` rows that sit next to each other.
4. Clean reference implementations to copy: the `Tabs` trigger, the teaching `stage` group (`border-0` plus `border-inline-start` with `:first-child` reset), the `ink-overlays` and `dag-overlays` toolbars (bordered wrapper, unbordered children), the teaching `browse` grid (bordered grid, `fieldset border-0`), and the teaching `tables` rows (`last:border-b-0`).

---

## 🎤️ Presentation (`🧰️framework/🛍️products/🎤️presentation`)

### P1. PDF page pager: two bordered chips separated by a gap (medium-high)

- Markup: `🎯️targets/⚛️react/🔨️modules/🎭️morph-views/🟦️.tsx:741-766`. Wrapper `.presentation-pdf-page-nav` (`role="group"`), buttons `.presentation-pdf-page-nav__button--prev` and `--next`.
- CSS: `🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:132-147` (`display:flex; gap:0.5rem`) and `:148-156` (`__button { border: var(--stroke-hairline) solid var(--border-normal-color); width/height 2rem }`).
- Why it duplicates: each chip paints its own four hairlines, and the group's gap puts two full boxes side by side. It reads as two separate buttons, not one segmented pager, and the seam (both chips' inner edges plus the gap) does not follow the hover, focus or active priority.
- Fix: make `.presentation-pdf-page-nav` the group frame (`data-shared-border-group`, horizontal), mark the two buttons `data-shared-border-cell`, set the gap to `0`, and drop the per-button `border` in CSS (the cell mechanism gives the divider). The priority layer then lights the seam on hover of either button.
- Confidence: medium-high that it should touch. The duplication (two full borders with a gap) is certain.

### P2. Interaction actions: reset and enlarge chips with a gap (medium)

- Markup: `🎯️targets/⚛️react/🔨️modules/🫳️interaction/🧲️interactive-disposition/🟦️.tsx:82-92` (`.presentation-interaction-actions` containing `.presentation-interaction-reset` and `.presentation-interaction-enlarge`).
- CSS: `🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:81-91` (`gap: var(--spacing-single)`), `:97-116` (both buttons get `border: var(--stroke-hairline) solid var(--border-normal-color)`).
- Why: two 1.5rem bordered icon chips with a gap form one toolbar of two actions, so there are two full frames where a shared divider is expected.
- Fix: mark the actions row as `data-shared-border-group` (horizontal), mark the two buttons `data-shared-border-cell`, gap `0`, remove the per-button border. Note that the reset button is conditional (`canResetPosition`), so the cell mechanism's last-cell rule must hold when only enlarge is rendered.
- Confidence: medium (depends on whether the two are meant to read as one control).

### P3. Interaction handles (reviewed, no action)

- `🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:4-80` `.presentation-interaction-handle` (eight corner and edge handles, each fully bordered). Handles sit at the bounding-box edges and do not touch each other, so no seam. Keep.

### P4. Arrangement canvas and media teaser (reviewed, no action)

- `🎨️styles/🧱️arrangement-canvas-and-embodiments/🎨️.css:74,119` (`border: 0`) and the `box-sizing: border-box` lines. Clean.

---

## 💻️ OS product (`💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/*`)

The OS product is the largest UI surface in scope (about 170 border lines). Most of it is `rounded-sm border bg-base` floating overlays, form fields (`rounded-sm border px-single py-1` on `input` and `select`), and per-mesh 3D geometry. The findings below are the ones with a real seam or box-in-box risk.

### O1. Block palette column and its entries (low-medium, needs a runtime check)

- `🧩️BlockListHost/🟦️.tsx:195` `semio-palette ... border-l border-border` (the palette panel's left divider).
- `🧩️BlockListHost/🟦️.tsx:168` palette entries `rounded border border-border p-single` inside `gap-1` (`PaletteEntryRow`).
- `🧩️BlockListHost/🟦️.tsx:83` and `:115` block and step cards `rounded border border-border` inside `gap-2`.
- Why: a column of fully bordered entries separated by gaps (rows each fully bordered with gaps). The palette's `border-l` also sits against the block list column; if that column has a right border or a frame, the two lines form a double seam. I did not find the list column's border in this file, so this is unconfirmed.
- Fix: `data-shared-border-group data-orientation="vertical"` on the palette and `data-shared-border-cell` on each entry, gap `0`. Keep the palette's left divider as the group frame and drop `border-l` in favour of it. The step and block cards are deliberate cards, so leave those as they are.
- Confidence: low-medium.

### O2. Agent chat header uses a raw token, not the shared helper (low)

- `💬️AgentChatPanel/🟦️.tsx:162` `border-b border-border` on the header, while the feed `ol` at L166 has `divide-y divide-border`.
- Why: no duplicate line (the header's bottom and the first feed item have no top border), but the header bypasses `borderNormalBottomClass`, so its colour does not get the shared-border priority treatment.
- Fix: use `borderNormalBottomClass` (`📏️border-presentation/🟦️.ts:12`) for consistency. No geometry change.
- Confidence: high that it is inconsistent, low that it is visible.

### O3. Text editor footers and action buttons (low, reviewed)

- `✏️TextEditor/🟦️.tsx:661`, `:667`, `:676` are three footer bars each with `border-t border-border`. Stacking two of them gives one hairline between them (the upper bar has no bottom border), so no double line. Clean.
- `✏️TextEditor/🟦️.tsx:663`, `:670`, `:671`, `:672` `rounded border border-border px-3 py-1` discard, cancel and apply buttons in `gap-2` rows. They are actions, not a segmented group, so the gap is defensible. Low; only convert if they are later made a group.

### O4. Floating notices stacked with a margin (low, intentional)

- `🏛️ShellHost/🟦️.tsx:217`, `:224` (`mt-double`), `:231`, `:239`, `:244`, `:249`, `:264`, `:278`, `:295`, `:317`, `:382`; `🏛️ShellHost/🔖️session-notice/🟦️.tsx:15`, `🍞️transient-notice/🟦️.tsx:17`, `🚪️close-guard/🟦️.tsx:36`, `🗂️local-catalog/📂️files/🟦️.tsx:45`.
- Each is an absolutely positioned card with its own full border. Two are stacked with `mt-double`, which is a gap between bordered cards. These read as separate notices, so no change is needed. If the stacked pair is meant to read as one panel, use the group mechanism.
- Confidence: low.

### O5. Toolbars with a bordered wrapper and unbordered children (clean reference)

- `🕸️NodeGraph/🖼️dag-overlays/🟦️.tsx:842` `SelectionAlignChrome`: `rounded border border-border p-0.5 gap-0.5` wrapper, children `size-5 rounded` with no border.
- `🖋️InkCanvasHost/🪟️ink-overlays/🟦️.tsx:37` toolbar: `rounded border p-1 gap-1`, children unbordered.
- Both are already correct: the wrapper owns the outline and the children draw no divider. Use them as the pattern for P1 and P2.

### O6. Space rows and member lists (low, reviewed)

- `🏘️SpaceBrowser/🟦️.tsx:268` each space row is `rounded-sm border` inside `ul gap-1`. Cards, not a segmented control. Keep.
- `🏘️SpaceBrowser/🟦️.tsx:172` `section ... border-t pt-3` (invite panel). Its previous sibling is `CreateSpaceForm` with a gap, so no seam. Clean.

### O7. Diagram node header (reviewed, clean)

- `🕸️NodeGraph/🔷️diagram-node/🟦️.tsx:92-93` node frame `border` and header `border-b border-border`. The header's bottom edge is interior to the node, not on the frame edge, so no duplicate. Clean.

### O8. Node tree header and rows (low, consider)

- `🗣️Interpreter/🧱️node-tree/🟦️.tsx:372` sticky header `borderNormalTopClass`, and `:424` rows `border-b border-border/40`. Each row draws its own bottom divider, the header only its top, so each seam is one line. Clean. Optional: a `data-shared-border-group` vertical would give one divider per row and let hover light it.
- Confidence: low (no duplicate).

### O9. Diff view divider (reviewed)

- `🔺️DiffViewHost/🟦️.tsx:127` `border-border border-l` divider between panes. Only a problem if the right pane also has `border-l`. Not checked; low.

---

## 📐 CAD plugin (`✏️s/🔌️plugins/📐️cad/…/✏️editor/⚙️engine/📺️renderer`)

### C1. REPL aside overrides its shared frame per side (medium-high)

- `🎛️repl/🟦️.tsx:146`: `cn(floatingPaneAsideClass, fillHost ? "max-h-[45%] w-full shrink-0 border-l-0 border-t" : "...")`.
- `floatingPaneAsideClass` (`🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts:34`) contains `shellChromeBorderClass`, which is `borderNormalFrameClass` (`🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts:28`), a full four-side frame.
- Why: in full-height host mode the aside is pinned to the bottom of a host. The per-side override (`border-l-0 border-t`) is a hand-made seam: the aside still paints its right and bottom edges on the host edges, so a host border plus the aside frame gives box-in-box. The override also depends on class order in the cascade.
- Fix: do not override classes. Attach the aside to its host through the compositor (`useSharedBorderFrameRef` and `data-shared-border-attached` with `left right bottom`, which the ownership layer makes transparent), or make the host own those edges. Remove `border-l-0`.
- Confidence: medium-high that this is a per-side hack that should be a shared attachment. The visible result depends on whether the host paints its own border.

### C2. Select triggers combine a raw border with the helper (low)

- `🎛️repl/🟦️.tsx:249`, `:284`, `:309` and `📋️repl-panes/🟦️.tsx:110` `SelectTrigger ... "rounded-md border px-single py-half", borderNormalClass`. The helper is applied on top of a raw `border`. No seam, but redundant and inconsistent. Consolidate to one class.
- `🎛️repl/🟦️.tsx:461` `border-border text-xs border-t pt-single` footer inside the aside: single top rule, clean.

---

## 🎓️ Teaching sites (non-course: `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react`, `🎓️teaching/…/⚡️energy/…`)

### T1. Button chips in action rows (medium-low, needs usage check)

- `🔨️modules/🪟️chrome/🟦️.tsx:15` `CONTROL` gives every `Button` and link `border border-border`. Any row of buttons in a `gap-single` flex (for example the `PageFrame` actions slot in `🪟️chrome/🟦️.tsx`, `flex flex-wrap gap-single`) shows bordered chips with a gap between them, not one segmented control.
- Fix: where a row of buttons is one logical control (Back/Next, or a set of filters), wrap it in `data-shared-border-group` and mark each button `data-shared-border-cell`, which also gives the priority hover. Standalone buttons in a header stay as they are.
- Confidence: medium-low. I did not verify every call site.

### T2. Segmented control hand-builds its frame (low, migration)

- `🔨️modules/🪟️chrome/🟦️.tsx:70` `role="radiogroup"` `inline-flex max-w-full flex-wrap border border-border` wrapper, and `:85` options `border-0`. There are no internal dividers, so no duplicate. It is a hand-made frame. Migrating it to `data-shared-border-group` plus `data-shared-border-cell` makes the seam follow hover and focus like the rest of the system.
- Confidence: low (cosmetic and consistency).

### T3. Stage group segments (reviewed, clean but hand-built)

- `🔨️modules/🧊️spatial/🖥️stage/🟦️.ts:47-50`: `.semio-stage-group` owns the outer border, its buttons are `border:0` with `border-inline-start: 1px` dividers, and `:first-child` drops the leading divider. Correct: one hairline per seam, no duplicate. It is a hand-written version of the cell mechanism, so migrate later for hover consistency.
- `:156-162` three groups in `.semio-stage-bar` (`gap:.375rem`): separate controls, so the gap is legitimate. No change.
- `.semio-stage-view` (L55) and `.semio-stage-progress` (L58) are separated by the root `gap:.5rem` (L44). No seam.

### T4. Playground panels and palette rows (low, reviewed)

- `🔨️modules/🧪️playground/🟦️.tsx:291`, `:310`, `:334`, `:350`, `:368` are bordered panels in a `gap-double` column and grid, so no touching seam. Clean.
- `:376` palette entry buttons `border border-border` in a grid with gaps: cards, not segments. Low.

### T5. Browse grid with border-0 fieldsets (clean reference)

- `🔨️modules/🗃️browse/🟦️.tsx:124` grid wrapper `border border-border`, with `fieldset ... border-0` (L126-147). Child frames do not repeat the outline. Clean; this is the pattern to copy.
- `:251` `border-b` header block above the page grid. No duplicate.

### T6. Tables (clean)

- `🔨️modules/🔬️detail/🧱️tables/🟦️.tsx:13` caption `border-b`, `:17` th `border-b`, `:32` `tr ... border-b border-border/60 last:border-b-0`, `:48` mobile row `border-b`. Each seam is one line and the last row has no bottom rule. Clean.

### T7. Export-panel format cards (low)

- `🔨️modules/🔬️detail/📤️export-panel/🟦️.tsx:119` `li ... border border-border` in `ul grid gap-double md:grid-cols-2` (L110). Cards with a gap. Deliberate. Keep.
- `:150` component card with `:153` inner `code ... border` (child frame inset inside a padded parent; the edges do not coincide). Clean.

### T8. Viewer reading and legend bars (reviewed, clean)

- `🔨️modules/🎛️viewer/🧾️reading/🟦️.tsx:23`, `:53`, `:69`. Standalone bordered panels with gaps. Clean.

### T9. Overview ruler and timeline (reviewed, clean)

- `🔨️modules/🏠️overview/🟦️.tsx:97-101` and `🧪️playground/🟦️.tsx:315-318`: absolutely positioned tick and ground lines, not framed boxes. Clean.
- `🔨️modules/🔬️detail/🎞️clips/🟦️.tsx:97-115`: phase blocks `inset-y-0 border` inside a `relative h-6` track with no border on the track. No box-in-box. Clean.

### T10. Energy browser clip (clean)

- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️settled.ts:21-23` reads `[data-shared-border-root]` and `data-shared-border-pending`, both set by `BorderCompositor` (`shared-border.ts:83`, `:129`, `:298-305`). The attribute names match. Clean.

### T11. Teaching app shell (low, reviewed)

- `📦️packages/🟦️typescript/📚️app/🟦️.tsx:107` `border-t` above the clear-data block and `:205` `footer ... border-t`. No adjacent border-b found in the shell, so no duplicate. Clean.

---

## 🌎️ Hub admin (`🌎️hub/🔨️modules/🛡️admin`)

### H1. Tab header and active trigger (clean reference)

- `🧱️elements/🛡️AdminApp/🟦️.tsx:55` and `:66` headers `border-b`. The tabs use the mechanism: `🧱️elements/📑️Tabs/🟦️.tsx:155` (`data-shared-border-group` on the list) and `:205-207` (`data-shared-border-frame`, `edges="top right bottom left"`, `state="active"` on the selected trigger), with `h-[calc(100%-var(--stroke-hairline))]` so the active trigger does not paint over the header's bottom edge. Clean. This is the reference for every bar in the suite.

### H2. Stat tiles (low)

- `🧱️elements/🏠️OverviewPage/🟦️.tsx:19` and `📊️ObservabilityPage/🟦️.tsx:40` `StatTile` `rounded border p-single` in a grid. Cells each fully bordered with gaps. If the tiles are laid out as one strip, use a group. As a card grid, keep.
- Confidence: low.

### H3. Space member section (clean)

- `🧱️elements/🏛️SpacesPage/🟦️.tsx:119` `border-t p-single` members section. Single divider, no adjacent border. Clean.

---

## 🐾 Pets (`🐾️pets/🎯️targets/⚛️react/📖️stories`)

Pets has no UI components with border class hits outside the stories CSS. `🔨️modules/🪞️lifting/🟦️.ts` lists computed longhand border properties, which is a copy routine and not UI.

### PT1. Controls header and story rule (clean)

- `📖️stories/🎨️.css:37-45` `.controls` `border-bottom`, then `🐾️species-gallery/🟦️.tsx:211-222`: `form.controls`, then `nav.roster` (no border-top in CSS), then `section.story`, whose `border-top` is `🎨️.css:129-131`. The roster sits between them, so there is no double line. Clean.

### PT2. Card tab note (low)

- `📖️stories/🎨️.css:417-424` `.card .note` with dashed `border` and `border-bottom: 0`, placed at `bottom:100%` on the card. It is a hand-drawn tab. The card's solid top border runs under it, so the tab is not an attached silhouette. Low; could use silhouette attachment if this pattern spreads.

### PT3. Story section title rule (low)

- `.story-section h3` (`🎨️.css:151`) `border-top: 1px dashed` inside `.story` (border-top at L130). The h3 sits below `.story-head` padding, so it is not edge-coincident. Clean.

### PT4. Card buttons and prop boxes (clean)

- `.card button` (`🎨️.css:72`), `.prop` (L320), `.specimen-box` (L202), `.shelf` (L434) are child frames inset by padding inside parents. Clean.

---

## 🦑️ Repo, 📓️ Print, 🖥️ Server, 🧪️ Tests, 🏢️ Semio-tech

- `🦑️repo/🔨️modules/📚️library/🎨️styling/📇️source/🟦️.ts` and `⚖️laws/🔬️interactivity-audit/…/🔍️chrome-and-maintenance-checks/🟦️.ts`: source-analysis code and audit rules, not UI. No findings.
- `📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts`: design-token paint resolution, not DOM. No findings.
- `🖥️server`, `🧪️tests`, `🏢️semio-tech/🎡️play` (`🎨️globals.css`, `⚛️play-card.tsx`, `🟦️.tsx`): zero border hits. No findings.

## ✏️ Other plugins

- `🗄️stdio`, `🏙️bim`: border hits are SQLite snapshot schemas, PDF and DWG standards subsets and schema text, not UI. No findings.
- `🌍️gis`, `🎞️animate`, `📕️norm`, `🌊️flow`, `📸️remodel`: no UI border hits in the in-scope files. No findings.
- All other plugin folders: zero border hits.

---

## Cross-cutting observations

1. Products and plugins do not use the shared mechanism. Every group I found (P1, P2, T1, T2, T3, O1, O8) is a hand-rolled `gap` plus per-cell `border`. The mechanism is used only by `Tabs` in the UI library. Fix at the root by moving these groups onto `data-shared-border-group` and `data-shared-border-cell`, not with per-product overrides.
2. Per-side overrides are the one hack pattern present (C1, `border-l-0 border-t`). Attach through the compositor instead.
3. Two token styles coexist: the raw `border-border` (O2, O3, teaching `CONTROL`) and the helpers `borderNormalClass` / `borderNormalBottomClass` / `borderNormalTopClass` (O2, C2, `ShellHost`, `canvas-footer`). Only the helpers feed the seam colour through `!important` and the priority layer, so raw tokens silently miss hover and focus on seams. Standardise on the helpers for any border that sits on a seam.
4. Non-issues confirmed: stacked `border-t` footers (one hairline), `border-b` above a `border-t` section with a gap between them, and children with `border-0` inside a bordered wrapper. These are the patterns already correct in the codebase.

## Suggested order of work

1. P1 (PDF pager) and P2 (interaction actions): convert to `data-shared-border-group` and cell; add a test case in `compositor.cases.json` or a presentation test for the seam count.
2. C1 (CAD REPL aside): replace the per-side override with compositor attachment; add a case where the host has a border.
3. T1 (teaching `Button` rows) after a usage survey; O1 (palette) after a runtime check of the list column.
4. T2, T3, O2, C2: consistency migrations, low priority.

Nothing in this audit was changed in the source tree. Temporary grep outputs written to `🗑️generated/` were removed; other workers' logs in that folder were left alone.
