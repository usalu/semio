# 🔎 Round 3 Audit — ✏️s Plugins

## Findings: remaining shared-border violations in ✏️s

**Scope note.** ✏️s holds 53 `.tsx` files and no `.css`. It contains no OS (`s/OS`) code. The OS product lives at `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os`, outside ✏️s, so I did not audit it. The sweep covers the CAD, GIS, puzzle and dev-story files under ✏️s. Repo-wide, the `ring-1`, `divide-*`, `-ml-px`, `-mt-px`, `border-l-0` and `first:/last:border` patterns have zero hits in ✏️s. Canvas, 3D, display and interaction files (`🪩️canvas`, `🖼️display`, `🔖️interaction`, `🧲️geometry-targets`) are content art and are exempt. This is a static audit only; no live probe was run.

**Context.** `floatingPaneAsideClass`, `floatingFieldSurfaceClass`, `floatingMenuSurfaceClass` and `floatingTagClass` (`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts`) all carry a native `border`. `SharedBorderRow` has no wrap mode (confirmed by grep), so `flex-wrap` chip groups cannot currently use it.

---

### Tier 1: CAD REPL aside (always visible in the CAD editor)

File: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx`

**1. Aside frame has no cells; every control inside it is a free-floating box.** (L148, L149–476)
- Snippet: `<aside ... data-shared-border-frame="" data-shared-border-edges="top right bottom left" className={cn(floatingPaneAsideClass, ...)}>`
- Why: The aside is a compositor frame, but its children are not cells. The Buttons (L153), the command field (L166), the three Select triggers (L241, L277, L302), and the chip groups all draw their own frames inside it. That is box-in-box. Their spacing is `p-double` and `gap-single` from `floatingPaneAsideClass`, which is gap separation. The aside also keeps the native `border` from `shellChromeBorderClass` (`floatingPaneAsideClass`) on the same element as the compositor frame. That can paint a second line unless the compositor ownership layer hides it. Not verified.
- Fix: Make the aside (or an inner stack) `SharedBorderRow orientation="vertical"` as the cells owner. Each section becomes a cell, with padding inside the cells. Drop the native border from the aside and rely on the frame, after confirming the ownership rule.

**2. Transition Buttons in a gap-separated wrap.** (L151–165)
- Snippet: `<div className="flex flex-wrap gap-half">` containing `<Button variant="outline" ...>` per transition row.
- Why: Bordered Buttons separated by `gap-half`. Gap separation between bordered siblings is the core violation.
- Fix: Use `ButtonGroup` (`🔳️ButtonGroup`) or `ActionGroup`, which already tile their items. If a wrapping row is required, add wrap support to `SharedBorderRow` (framework gap, not yet present).

**3. Command field: bordered Button inset inside a bordered surface, plus an overridden Input border.** (L166–195)
- Snippet: `<div className={cn("relative grid overflow-visible", floatingFieldSurfaceClass)}>` → `<Input className="... border-0 ... shadow-none focus-visible:ring-0" />` (L181) and `<Button ... className="... mr-single size-medium ..."/>` (L183–195).
- Why: The surface draws a border. The Input is then stripped of its own border with `border-0`, which is an ad-hoc hack. The toggle Button sits inside the surface with `mr-single` inset, so it is box-in-box and inset. This is the top visual seam in the aside.
- Fix: Replace the surface with a horizontal `SharedBorderRow` owner. The Input becomes a flex-1 cell and the toggle Button a trailing cell. Remove `border-0`, `mr-single` and the surface's native border.

**4. Suggestion popover offset from its field by a gap.** (L202–208)
- Snippet: `className={cn("absolute top-[calc(100%+var(--spacing-double))] right-0 ... ", floatingMenuSurfaceClass)}`
- Why: `top-[calc(100%+var(--spacing-double))]` puts a gap between the bordered field and the bordered menu. The menu is also a native `border` box.
- Fix: Attach the menu flush to the field's bottom edge (zero offset) and let the compositor paint the shared line, or make the field and menu one framed group.

**5. Chips inside suggestion rows are bordered boxes inside a bordered menu.** (L212–218)
- Snippet: `<span className={cn(floatingTagClass, floatingTagOffClass, "inline-flex min-w-6 ...")}>{suggestion.key}</span>` inside `<MenuItem>`.
- Why: `floatingTagClass` includes `border`, so this is a box inside the bordered `floatingMenuSurfaceClass` menu.
- Fix: Render the key as an unframed text cell or `<kbd>`, not a bordered tag.

**6. Select blocks stacked with `gap-single` inside the aside.** (L236–262, L274–298, L299–323)
- Snippet: `<div className="flex flex-col gap-single text-xs">` containing `<label className="flex flex-col gap-half"><span/><Select><SelectTrigger className={cadFieldClass}/></Select></label>`.
- Why: Each Select trigger is a bordered control. The label-plus-control blocks are gap-separated from one another and from the neighbouring checkbox groups, and they sit inset in the aside.
- Fix: Make the column one vertical `SharedBorderRow`. Each label/Select pair becomes a row with the label as a label cell and the trigger as a `SharedBorderRowField`/cell, with no gap.

**7. Chip groups: many gap-separated bordered chips, each wrapping a Checkbox.** (L335–346, L369–387, L392–407, L429–450)
- Snippet: `<div role="group" className="flex flex-wrap gap-half">` → `<label className={cn(floatingTagClass, ...)}><Checkbox .../>{kind}</label>`.
- Why: Four chip groups with `gap-half`, each chip a `border rounded-full` box, and each chip holding a framework Checkbox. This is the largest volume of violations in the file. Checkbox nested in a bordered chip is box-in-box. I could not confirm from the framework source whether Checkbox draws its own frame. If it does, this is a double frame.
- Fix: Replace each group with a framework `ToggleGroup` (`type="multiple"`, or equivalent) whose items are cells. `ToggleGroup` already owns cells and a frame. If it does not wrap, add a wrap mode to `SharedBorderRow` (framework gap). Remove the Checkbox from the chip.

**8. Master-toggle rows stacked with gaps, with the chip groups.** (L331–334, L348–368, L388–428)
- Snippet: `<label className="text-foreground flex items-center gap-single ..."><SpatialChromeMasterToggle .../>Primitives · Show</label>`
- Why: Each master Checkbox is a bordered control, separated by `gap-single` from its chip group and from the next block.
- Fix: Make each master toggle and its chip group one row in the vertical `SharedBorderRow` from item 1, with no gap.

**9. Divider and last-response block: ad-hoc `border-t` with a gap above it.** (L452–462, L463)
- Snippet: `<div className="border-border text-xs border-t pt-single">`
- Why: A native `border-t` is used as a section separator, and it sits after a `gap-single` from the previous block. That creates a gap plus a second line (category 4).
- Fix: Make the response block a cell of the aside's vertical row. Remove `border-t` and let the cell seam draw the line.

---

### Tier 1: CAD attribute and stats panes (visible when an object is selected)

File: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/📋️repl-panes/🟦️.tsx`

**10. `fieldRow` stacks a bordered Clear button and a bordered control with `gap-half`.** (L72–84)
- Snippet: `<div key={defn.id} className="flex flex-col gap-half">` → header `<div className="flex items-center justify-between gap-single">` containing `<Button ... variant="outline" ... className="h-auto px-half py-0 text-2xs">` (L77), then `{control}` (L82).
- Why: The header Button and the bordered control below it are gap-separated bordered siblings. The Button is also inset in the header row.
- Fix: Make `fieldRow` a vertical `SharedBorderRow`. Use a label cell and the Clear Button as header cells, and the control as a field cell, all without gaps.

**11. Attribute field list is gap-separated.** (L86–87, L92–162)
- Snippet: `<div className="flex flex-col gap-single text-xs">` then `definitions.map(... fieldRow(...))`.
- Why: Each field row (Select L100–119, number Input L126–135, boolean label L142–145, text Input L151–160) is a bordered control separated by `gap-single`.
- Fix: One vertical `SharedBorderRow` owner for the whole pane. Each field is a row with cells, and the boolean `<label>` with its Checkbox becomes a cell. Use `SharedBorderRowField` for the native input.

Not violations (checked): `SelectionPropertiesPane` (L291–304), `ModelStatsPane` (L220–258). These contain only unbordered `pre`, `span` and `grid` text.

---

### Tier 1: GIS inference port (visible while an inference runs)

File: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🪟️presentation/🟦️.tsx`

**12. Raw `<button>` elements bypass the framework and sit as adjacent siblings.** (L55–59)
- Snippet: `{chrome.request ? <button type="button" onClick={...}>{control.request[locale]}</button> : null}` … `<button type="button" onClick={() => onAction({ kind: "close" })}>{control.close[locale]}</button>`
- Why: Five unclassed native buttons (request, cancel, reject, approve, close) sit directly in the `<section>` with no wrapper. They are unstyled by the framework and cannot join a shared row. They touch with no gap, which gives doubled or mismatched lines, or the browser default border (category 5). Cancel and reject both dispatch the same action, which is not a border issue but is worth a look.
- Fix: Wrap the actions in one `SharedBorderRow` (or `ButtonGroup`) with framework `<Button variant="outline">` cells, `data-shared-border-cells` on the group. Put the `<progress>` (L54) in a cell as well. Ignore the map overlay `svg` (L48–53), which is content art.

---

### Tier 2: Dev stories and harnesses (not product UI, lower visibility)

Stories under `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/**` and `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/**/📖️stories/**`. Each finding is the same pattern: raw `<button>` (category 5) in `display:flex; gap` rows (category 2).

| File (under ✏️s) | Lines | Pattern |
|---|---|---|
| `🧑‍💻dev/🏗️fem/📖️stories/🎭️2d-model/🧪️.story.tsx` | 71–82 | Raw buttons in `flex-wrap gap-4` |
| `🧑‍💻dev/🏗️fem/📖️stories/🎭️2d-results/🧪️.story.tsx` | 87–98 | Raw buttons in `flex-wrap gap-4` |
| `🧑‍💻dev/🏗️fem/📖️stories/🎭️3d-model/🧪️.story.tsx` | 67–74 | Raw buttons in `flex-wrap gap-4` |
| `🧑‍💻dev/🏗️fem/📖️stories/🎭️3d-results/🧪️.story.tsx` | 75–86 | Raw buttons in `flex-wrap gap-4` |
| `🧑‍💻dev/🧱️block/📖️stories/🎭️5d-board/🧪️.story.tsx` | 61–76 | Raw buttons in `flex-wrap gap-4` |
| `🧑‍💻dev/🧱️block/📖️stories/🎭️2d-board/🧪️.story.tsx` | 60–78 | Raw buttons in `flex-wrap gap-4` |
| `🧑‍💻dev/🧱️block/📖️stories/🎭️5d-world/🧪️.story.tsx` | 115–124 | Raw buttons in `flex-wrap gap-4` |
| `🧑‍💻dev/📸️remodel/📖️stories/🎭️panels/🧪️.story.tsx` | 55–57 | Raw buttons in `gap-6` |
| `🧑‍💻dev/📸️remodel/📖️stories/🎭️frames/🧪️.story.tsx` | 59–61 | Raw buttons in `flex-wrap gap-6` |
| `🧑‍💻dev/📸️remodel/📖️stories/🎭️model/🧪️.story.tsx` | 52–54 | Raw buttons in `flex-wrap gap-6` |
| `🧑‍💻dev/📸️remodel/📖️stories/🎭️report/🧪️.story.tsx` | 47–49 | Raw buttons in `flex-wrap gap-6` |
| `🧑‍💻dev/🧩️catalog/🧪️tests/🧪️multi-shell-harness/🟦️.tsx` | 41–45 | Raw toolbar button with inline `gap: 8` |

Fix for the table: use framework `Button` inside a `ButtonGroup` or `SharedBorderRow`.

Ad-hoc inline divider seams (category 4):
- `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️5d-timeline/🧪️.story.tsx:262`: `style={{ ... borderLeft: "1px solid var(--border, #3333)" ... }}` on the timeline pane beside the world pane. Fix: make the two panes cells of one horizontal shared row.
- `/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧱️block/📖️stories/🎭️2d-fixtures/🧪️.story.tsx:31`: the same `borderLeft` on a `section`. Same fix.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📖️stories/🎭️renderer/🧪️.story.tsx:165`: sidebar with inline `borderLeft`, then (L173–189) a `label`, a raw `<input>` (L175), and five raw `<button>` elements in a `gap: 8` column. Categories 2, 4 and 5 together. Fix: a vertical `SharedBorderRow` with framework `Input` and `Button` cells.

---

### Checked and clean (no finding)

- `🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/🟦️.tsx` (602 lines): code generator, no JSX.
- `🔌️plugins/🧩️puzzle/🧪️tests/🧪️compose5d-preparetopologymodel/🟦️.tsx`: test, no bordered markup.
- `🧪️tests/🏘️plugin-composition/🗺️gismap-inference-panel/🟦️.tsx`: test harness for the GIS panel, no classes of its own.
- `🧑‍💻dev/📖️stories/🧭️coordination/🟦️.tsx`: no bordered markup.
- CAD `repl-model` `WorldOrbitProjectionSwitchPane` (L789–811): delegates to framework `Pane`, no local border.
- CAD canvas, display, interaction, geometry-target and pick-layer files: content art, exempt.

### Not verifiable statically

1. Whether the compositor hides the native `border` from `floatingPaneAsideClass` on a `data-shared-border-frame` element (item 1).
2. Whether framework `Checkbox` draws its own frame. `☑️Checkbox` shows no border class in its own `🟦️.tsx`, so it may inherit one elsewhere (items 7 and 9).
3. Whether a global `button` CSS rule applies a border to the raw buttons in item 12 and the story rows.

### Priority order for fixing

1. Item 1 (aside frame as cells owner), since it frames the rest of the aside.
2. Items 7 and 5 (chip groups and suggestion tags), the largest volume.
3. Item 3 (command field), always visible.
4. Items 10–11 (attribute panel), visible on selection.
5. Item 12 (GIS raw buttons).
6. Items 2, 4, 6, 8, 9 as part of the same aside rewrite.
7. Dev stories, lowest.

I can run the same sweep on `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os` if that is the OS you meant.