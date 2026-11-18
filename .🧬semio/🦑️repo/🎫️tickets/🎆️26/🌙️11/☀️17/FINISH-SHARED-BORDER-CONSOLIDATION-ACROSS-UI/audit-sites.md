# Audit: remaining top-level sites (shared-border consolidation)

Scope: `🌎️hub/**`, `🏢️semio-tech/**`, `👴️leutwiler/**`, `♻️mit-bestand/**`, React TSX + CSS only. Read-only audit, no source changed.

Excluded: `node_modules`, `dist`, `generated`, `🧪️tests` / `*test*` files, and the vendored saved website under `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/` (a browser "save page" copy of an external site with its own CSS, not hand-written; `🩹️overwrite.css` and the `_files` CSS were scanned by grep and contain no custom owner logic to fix here).

Mechanism reference: `nested-cells.md` (`SharedBorderRow` / `SharedBorderRowCell`, `data-shared-border-cells` owners, direct-child framed controls become cells, wrappers such as `<label>` break direct-child status and fall back to a floating frame). Table rows in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🟦️.tsx` always draw `borderNormalBottomClass` (L258 body rows, L432 head).

## Counts

| Site | Findings | High | Medium | Low | Notes |
|---|---|---|---|---|---|
| 🌎️hub | 8 | 2 | 4 | 2 | all in `🔨️modules/🛡️admin/🧱️elements/` |
| 🏢️semio-tech | 1 | 0 | 1 | 0 | bypass only, no border |
| 👴️leutwiler | 0 | - | - | - | no TSX or CSS (only `.ts`, Lean, TeX, JSON) |
| ♻️mit-bestand | 2 | 1 | 1 | 0 | `🧺️demonstrator` only |

## 🌎️hub

### H1. Box-in-box and stacked bordered cards: Connections

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔗️ConnectionsPage/🟦️.tsx`
- Lines: L75 (page `gap-single` column), L86 (`flex flex-col gap-single rounded border p-single` per space), L102 (`<Button ... variant="ghost">` kick action)
- Offending: `rounded border p-single` card per space; a ui `Button` (ButtonGroup frame, `border-transparent` for ghost) inside it.
- Why: (a) each space card is a bordered box inside a `gap-single` column, so the boxes are gap-separated siblings instead of one group; (b) the framed `Button` sits inside a bordered, padded parent, so its frame (visible on hover/focus) is a box inside a box. Ghost only hides the resting border.
- Fix: per space, one `SharedBorderRow orientation="vertical" framed` with a `SharedBorderRowCell` for the heading and each document/row line; the kick `Button` becomes a direct child cell of a horizontal row for that line (`className="flex-1 min-w-0"` on the text, not on the button). Remove the outer `rounded border p-single`. Nested `ml-single` indentation stays as cell padding. Do not put two owners on the same edge.
- Confidence: high for the box-in-box structure; medium for the visible hover/focus effect (depends on the ghost frame overlay).

### H2. Gap-separated stat tiles should be one group: Overview

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏠️OverviewPage/🟦️.tsx`
- Lines: L19 (`StatTile`: `flex flex-col gap-single rounded border p-single`), L109 (`grid grid-cols-2 gap-single md:grid-cols-3`), L110-116 (seven `StatTile`s)
- Why: seven separately bordered tiles separated by `gap-single`; the brief's "gap-separated bordered chips that should be one group". The comment at L17 says no stat-tile element exists; a shared-border owner now covers this.
- Fix: wrap each grid row in `SharedBorderRow` (horizontal, framed) with `SharedBorderRowCell` per tile (or one framed owner with cells for the whole set), drop `gap-single` between tiles and the tile's own `rounded border p-single`. On the narrow 2-column breakpoint, use one row per visual row (grid rows cannot carry flex dividers across wrap). Keep `StatTile` as the one shared helper (see H3 duplicate).
- Confidence: medium-high.

### H3. Duplicate bordered tile helper: Observability (same pattern as H2)

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📊️ObservabilityPage/🟦️.tsx`
- Lines: L40 (`Figure`: `flex flex-col gap-single rounded border p-single`, identical markup to `StatTile` in H2), L116 (`grid grid-cols-2 gap-single md:grid-cols-4` with two figures), L123 (`grid grid-cols-2 gap-single md:grid-cols-4` with nine figures, L124-132)
- Why: same gap-separated bordered-tile pattern, and a second copy of the same helper, which the repo rules say belongs next to its duplicate.
- Fix: one shared stat element (framework `🖱️ui` or a hub-local `StatTile` exported from one file) built from `SharedBorderRow` cells, one row per grid row, used by both Overview and Observability.
- Confidence: medium-high.

### H4. Member panel border next to the table's last-row border: Spaces

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏛️SpacesPage/🟦️.tsx`
- Lines: L330-338 (`<Table>` of spaces; body rows carry `borderNormalBottomClass`), L339 (load-more `Button`, conditional), L341 (`<MembersPanel>` rendered after), L119 (`<div className="flex flex-col gap-single border-t p-single" data-slot="admin-space-members">`)
- Offending: `border-t` on `MembersPanel` directly below the Table whose last row already draws its bottom border.
- Why: two hairlines stacked in the same column (table row bottom, then the panel's top) with only a gap between them, i.e. "border + border of neighbour" and two lines where one is wanted.
- Fix: drop `border-t` from `MembersPanel`; let the Table row bottom border be the single divider. Or, if the panel must read as part of the expanded space, render the Table and the panel as cells of one vertical `SharedBorderRow` (the table row as a cell, the panel as a cell with its own padding and no `border-t`).
- Confidence: medium-high (depends on the table's last row being visible; it is when the table has rows).

### H5. Gap-separated framed controls in one row: add-member form

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏛️SpacesPage/🟦️.tsx`
- Lines: L132 (`flex items-end gap-single`), L133-135 (`<label>` wrapping `<Input>`), L137-141 (`<Select>` with `SelectTrigger w-32`), L149-150 (`<Button id=admin-space-member-add-...>`)
- Offending: three framed controls (Input, SelectTrigger, Button) separated by `gap-single` in one flex row, and the `Input` wrapped in a `<label>`.
- Why: gap-separated bordered controls that should be one group. The `<label>` wrapper makes the Input not a direct child, so it keeps its own floating frame and cannot become a cell.
- Fix: `SharedBorderRow orientation="horizontal" framed` with the `Input`, `Select` trigger and `Button` as direct children, `className="flex-1 min-w-0"` on the Input, and no `gap-single`. Move the visible field label out of the wrapping `<label>` (keep `htmlFor`/`aria-labelledby`, or use `Field` above the row) so the Input stays a direct child.
- Confidence: high.

### H6. Gap-separated ghost row actions: space row

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏛️SpacesPage/🟦️.tsx`
- Lines: L280 (`flex items-center gap-single`), L281-315 (four `Button variant="ghost"`: rename, visibility, archive, delete)
- Why: four framed (transparent-border) buttons with gaps. Little visible seam at rest, but each shows its frame on hover/focus, and the design system has a grouping element for exactly this.
- Fix: `ActionGroup` (`⚡️ActionGroup` element) instead of a `gap-single` flex row, so the group shares one set of dividers.
- Confidence: low (ghost: no resting border).

### H7. Gap-separated primary and ghost buttons: rebuild row

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏠️OverviewPage/🟦️.tsx`
- Lines: L118 (`flex items-center gap-single`), L119 (rebuild `Button`, framed), L121 (cancel `Button variant="ghost"`, shown while rebuilding)
- Why: a framed button and a second framed (ghost) button side by side with a gap; both render frames while rebuilding.
- Fix: `ActionGroup` or `SharedBorderRow` (horizontal) with both buttons as cells.
- Confidence: low.

### H8. Hand-rolled bilingual language buttons bypass the ui Button: admin gate

- File: `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx`
- Lines: L359 (`<div className="flex gap-single">`), L360-361 (`<button type="button" lang="en">English</button>`, `<button type="button" lang="de">Deutsch</button>`)
- Offending: two native, unstyled `<button>` elements (no class; styling depends on a global reset for native border and padding). Not a 🖱️ui element.
- Why: bypasses the 🖱️ui `Button`/`ButtonGroup`; if the reset is absent these render with browser-default borders next to each other (a gap-less seam question).
- Fix: `ButtonGroup` with two `ButtonGroupItem`s (same as the mit-bestand locale picker M1). Ideally one shared language-choice element for the hub, semio-tech play and mit-bestand gates.
- Confidence: medium.

## 🏢️semio-tech

### S1. Hand-rolled bilingual language buttons bypass the ui Button: play boot

- File: `🏢️semio-tech/🎡️play/🟦️.tsx`
- Lines: L243-246 (`<main ... data-slot="play-language-choice">` with `<button type="button" lang="en">English</button>` L245 and `<button type="button" lang="de">Deutsch</button>` L246)
- Offending: native unstyled `<button>` pair, no class.
- Why: same as H8; no shared border logic in this file (the file has no `border`/`rounded` at all, checked).
- Fix: `ButtonGroup` with `ButtonGroupItem`s, or the same shared language-choice element as H8 and M1.
- Confidence: medium.
- Checked, no finding: `⚛️play-card.tsx` (uses `OverviewCard` / `OverviewCardOpenChip`, no own border); `🎨️globals.css` (import only); the Navbar (`bg-transparent`).

## 👴️leutwiler

- No TSX or CSS in scope (`💤️realparts-of-powers-z-n` has `.ts`, Lean, TeX, JSON only). No findings.

## ♻️mit-bestand

### M1. Two hand-rolled bordered buttons separated by a gap: locale picker

- File: `♻️mit-bestand/🧺️demonstrator/⚛️locale-picker.tsx`
- Lines: L21 (`<div role="group" className="flex gap-double">`), L23 (`<button ... className="cursor-pointer rounded-sm border border-border px-double py-single text-sm hover:bg-hover-window">`, one per locale)
- Offending: two hand-rolled `border border-border rounded-sm` buttons with `gap-double` between them. Not a 🖱️ui element.
- Why: two separately bordered boxes instead of one grouped control (gap-separated bordered siblings), and a hand-rolled control where the ui `ButtonGroup`/`Button` exists.
- Fix: `ButtonGroup` with two `ButtonGroupItem`s (one shared divider, no gap, shared hover/focus priority), or `SharedBorderRow orientation="horizontal" framed` with `Button` children. Remove `border border-border rounded-sm px-double py-single` from the native buttons.
- Confidence: high.

### M2. Hand-rolled pressed-state language switcher where ToggleGroup exists

- File: `♻️mit-bestand/🧺️demonstrator/⚛️locale-picker.tsx`
- Lines: L37 (`<div role="group" ... className="pointer-events-auto inline-flex items-center gap-tiny">`), L39-49 (`<button aria-pressed ... className={cn("cursor-pointer rounded-sm px-single py-half text-xs ...")}>`)
- Offending: hand-rolled `aria-pressed` buttons (no border; navbar switcher).
- Why: not a border seam (no border on these), but a hand-rolled toggle group bypassing the 🖱️ui `ToggleGroup` (`🎛️ToggleGroup`); its pressed styling is custom.
- Fix: `ToggleGroup` (single selection) with `Toggle` items, so pressed state, focus and hover priority come from the design system.
- Confidence: medium.
- Checked, no finding: `⚛️footer.tsx` (partner credit links use hover background only, no border; L26, L57, L83); `🟦️.tsx` (`Navbar`, `OverviewCard` via `⚛️demonstrator-card.tsx`, no own border); `🎨️globals.css` (colour rules only).

### M3. Presentation folder: no finding

- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎨️.css`: no `border` rules. Its `🧪️tests` `.tsx` files are excluded.

## Cleared (checked, not findings)

- `🌎️hub/.../🛡️AdminApp/🟦️.tsx` L55 and L66 (`header ... border-b`): the header bottom border is not coincident with any framed edge. The tab list and locale `SelectTrigger` sit inside the header's `p-single` padding, and the active `TabsTrigger` frame sits inside `TabsList` padding, so there are two lines but no seam. Not a double line; re-check if the header padding changes to 0.
- `🌎️hub/.../📦️packages/🟦️typescript/` (`🎨️.css`, `🟦️.tsx`): no border rules.
- `🌎️hub/.../🔑️AdminSession/🟦️.tsx` L270, L282: `rounded bg-muted` code chips, no border.
- Custom CSS `border:` on neighbours: none in any in-scope CSS file.

## Notes for the shared mechanism

- The same language-choice gate appears in three sites (H8, S1, M1). One shared `ui` element built on `ButtonGroup` would fix all three.
- `StatTile` (H2) and `Figure` (H3) are the same component; one shared stat-tile on `SharedBorderRow` covers both.
