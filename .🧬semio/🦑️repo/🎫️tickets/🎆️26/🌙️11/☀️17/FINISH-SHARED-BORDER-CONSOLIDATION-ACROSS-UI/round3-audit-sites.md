# 🔎 Round 3 Audit — Sites and Teaching

## Shared-border audit: remaining violations (read-only, static)

**Scope:** `.tsx/.ts/.css` under 🎓️teaching (about 480 files), 🌎️hub, 🏢️semio-tech, ♻️mit-bestand, 👴️leutwiler. The two launch-kit folders and 🧫️fixtures contain no TS/CSS. 🧪️tests and the vendored `📅️33.projektetage/🌐️public` are excluded. Slide art, canvases, 3D views and chart art are exempt.

**Method:** grep for the five categories, then manual reads of every hit in a UI file. I did not run the live `border-gap-probe.js`, so every finding below is static and needs a probe confirmation.

**Headline:** the hub and mit-bestand are essentially clean now. Every item in `audit-sites.md` (H1-H8, S1, M1, M2) is fixed: the admin pages use `SharedBorderRow`/`StatTiles`, and the language pickers use the shared `LanguageChoice`. The remaining violations are in teaching staffage.

### Ranked findings

**1. 🎓️teaching (staffage 3D toolbar): gap-separated bordered groups and box-in-box. High visibility, shown on every 3D model view.**
- File: `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧊️spatial/🖥️stage/🟦️.ts`
- `:46` `.semio-stage-bar{display:flex;flex-wrap:wrap;gap:.375rem;…}`, with `:156`, `:159`, `:162` creating three `semio-stage-group` children.
- `:47` `.semio-stage-group{…;border:1px solid …}`
- Why: three separately bordered groups sit side by side with a 0.375rem gap. That is the "gap-separated bordered siblings" case. The toolbar also paints a second frame per button (`:48` `.semio-stage button{border:1px solid …}`), overridden per group by `:49`.
- Fix: one `SharedBorderRow` (horizontal, framed) for the toolbar, with the scale, mode and preset buttons as direct cells. Drop `gap` and the group borders. The rule at `:50` (`.semio-stage-group button:first-child{border-inline-start:0}`) is the ad-hoc first-child hack and goes away with it.
- Also: `:58` `.semio-stage-progress{…;padding:.375rem;border:1px solid …}` is a bordered box holding the progress bar and the cancel `button`, which gets its own frame from `:48`. That is a box inside a box. Make the progress a `SharedBorderRow` whose cancel `Button` is a cell.
- Also: `:65` `@media (prefers-contrast:more){…border-width:2px}` makes 2px seams in high-contrast mode. Use the mechanism's hairline instead.
- Confidence: high for the gap and the box-in-box; medium on whether it reads as a double line in the live view.

**2. 🎓️teaching: dashed or bordered "unavailable/error" notices drawn as ad-hoc boxes. Medium-high, appears whenever a viewpoint or model is unavailable.**
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🔭️viewpoints/🪟️sheet/🟦️.tsx:100`
  - Snippet: `<div data-notice="unavailable" className="flex h-full … overflow-auto border border-dashed border-border bg-muted/40 p-double pt-6 …">`
  - Why: a dashed bordered box inset inside the sheet's figure cell, with its own padding. It bypasses the `Notice` element, which draws a shared `SharedBorderRow`.
  - Fix: render `<Notice kind="unavailable" plain …>` or a `SharedBorderRow` (vertical) whose words and details are cells. Remove `border border-dashed` and `p-double`.
- `:117` the viewpoint label chip `absolute left-1 top-1 border border-border px-single …` sits on the figure. It is a bordered button inset inside the figure frame. Figure drawing is art, so this is low, but confirm the frame is not a bordered container.
- `🎓️teaching/…/🔨️modules/🧍️figure/🟦️.tsx:139` `style={{ display: "block", border: "1px dashed currentColor", padding: "0.5em" }}` (figure refusal).
- `🎓️teaching/…/🔨️modules/🧊️spatial/🚪️gate/🟦️.tsx:27` `style={{ border: "1px dashed currentColor", padding: "0.5em" }}` (model error, with a raw `<button>` at `:29` that is not a `Button`).
- `🎓️teaching/…/🧩️element/🟦️.ts:181`, `:237`, `:299`, `:314` the same inline dashed box inside innerHTML for the web component's problem and unavailable states.
- Why: these are dashed-border notices with padding, which is category 6, and they bypass the shared notice. The web-component ones are a separate render path and need the same treatment.
- Fix: use the `Notice` element (or the same `SharedBorderRow` cells) in the React paths. For the `.ts` innerHTML, emit `data-shared-border-*` markup or a single `SharedBorderRow`-equivalent block, with no dashed border and no padding box.
- Confidence: high for the presence; medium on whether these count as "content" (they are error states of figure content, so the team may choose to exempt them).

**3. 🎓️teaching (collections): ad-hoc bottom-border kill. Medium, visible on the Collections page for each saved and curated collection.**
- File: `🎓️teaching/…/🔨️modules/⭐️collections/🟦️.tsx`
- `:47` `<Column as="section" … {...(collection.assets.length === 0 ? {} : { style: { borderBlockEndWidth: 0 } })}>`
- `:146` `<Column as="section" key={collection.id} … style={{ borderBlockEndWidth: 0 }}>`
- Why: category 4 hack. The section draws its own bottom border and the code removes it inline to avoid a double line with the grid below. The seam is owned in two places, which is the root cause.
- Fix: pass `edges` to the `SharedBorderRow` (or make the grid below the bottom owner), so the section paints no bottom edge. Remove the inline `style`.
- Confidence: high.

**4. 🎓️teaching (staffage detail tables): hand-rolled table rules and `last:` hack. Medium-low, visible on the detail page's dimension, model and timeline tables.**
- File: `🎓️teaching/…/🔨️modules/🔬️detail/🧱️tables/🟦️.tsx`
- `:13` `<caption className="border-b border-border bg-muted/40 …">`
- `:17` `<th … className="… border-b border-border bg-panel …">`
- `:32` `<tr className="border-b border-border/60 align-top last:border-b-0">`
- Why: category 4 (`last:border-b-0`) plus hand-drawn rules in a cell that is already a shared cell (`:48` `data-shared-border-cell`). The rule set is not owned by the mechanism, so the bottom of the last row and the next cell's divider can double up.
- Fix: build `DataTable` rows as a vertical `SharedBorderRow` of `SharedBorderRowCell`s (or reuse the framework `Table` row rules), with one owner for each row line. Drop `last:border-b-0`.
- Confidence: medium.

**5. ♻️ and 🏢️: none. Verified clean.**
- The language choice (`locale-picker.tsx:23`, `:32`; `🏢️semio-tech/🎡️play/🟦️.tsx:67`) and the admin `LanguageChoice` (`📚️I18n/🟦️.tsx:362`) all use the shared element. No `gap-`, `divide-` or `border` is left in the demonstrator navbar or footer.
- `🏢️semio-tech/🎡️play` uses `CardGrid`, `OverviewCard` and `centeredLastRowCells`, with no gap or border.

**6. 🌎️hub admin: residual low-severity items. Low.**
- `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📄️AdminPage/🟦️.tsx:7` `export const ADMIN_ACTION_CELL = "!px-0 [&>div]:items-stretch [&>div]:!gap-0";`
  - Why: `!important` overrides (`!px-0`, `!gap-0`) that reach into the child's own `gap`. It is a hack that patches the table cell instead of the row. Used by `🏛️SpacesPage` and `🙋️UsersPage` action cells.
  - Fix: render the action cell as a `SharedBorderRow framed={false}` directly (it already is) and have the `Table` cell adopt it, with no class override.
- `🌎️hub/…/📊️StatTiles/🟦️.tsx:30` empty `aria-hidden` filler cells.
  - Why: no visible box, but they make the final row look bordered at full width. Acceptable, low.
- No other hub hits. The one remaining `rounded` chip (`🔑️AdminSession/🟦️.tsx:270`, `:282`) is an unbordered code chip, so no finding.

**7. 🎓️teaching (staffage raw native controls inside shared rows): low, category 5.**
- File: `🎓️teaching/…/🔨️modules/🪟️chrome/🟦️.tsx`
- `:150` `<select … className="min-w-0 flex-1 text-xs text-foreground">`
- `:165` `<input … className="min-w-0 flex-1 text-xs text-foreground">`
- `:211` `<input type="range" … className="min-h-6 w-full accent-…">`
- Why: raw native controls. They sit inside a `SharedBorderRowField` cell, so the cell draws the frame. The native `<select>` and text input's own border depends on a global reset, which I did not verify.
- Fix: confirm the global reset zeroes `select`/`input` border. If it does, nothing to change; otherwise add the reset or use the framework `Input`/`SelectTrigger`.
- Confidence: low (depends on the reset).

**8. Notice `plain` mode with unwrapped actions. Low, no current caller.**
- File: `🎓️teaching/…/🪟️chrome/🟦️.tsx:288`
- `{actions === undefined ? null : <div className="flex flex-wrap gap-single">{actions}</div>}`
- Why: in `plain` mode the actions are not passed through `asCells`, so framed `Button`s would be gap-separated. Non-plain mode (`:306`) uses `Bar(asCells(...))`, so it is correct. The only `plain` callers (`overview/🟦️.tsx:166`, `:205`, `collections/🟦️.tsx:29`, `compare/🟦️.tsx:66`, `:68`, `:73`, `previews/🟦️.tsx:58`, `:61`) pass no actions, so no visible bug today.
- Fix: run `actions` through `asCells` and a `Bar` in plain mode, or drop the plain actions branch.

**9. Exempt or art (listed, not counted as violations):**
- `🎓️teaching/…/🔬️detail/🎞️clips/🟦️.tsx:101`, `:105`: timeline phases and event markers, bordered, inside a group marked `data-border-probe-art` (`:94`). These are the chart art of the timeline.
- `🎓️teaching/…/🎛️viewer/🧾️reading/🟦️.tsx:53`: the thermal gradient legend bar, `data-border-probe-art`.
- `🎓️teaching/…/🧪️playground/🧊️model/🟦️.tsx:242`: a bordered 3D model wrapper.
- `🎓️teaching/…/🧪️playground/🖼️stage/🟦️.tsx:130`, `:131`, `:136`, `:140`: drawing guides for the planar stage.
- `🎓️teaching/…/🏠️overview/🟦️.tsx:257`: `grid grid-cols-2 gap-double` of two inert, `aria-hidden` 3D miniature previews. The gap is between art, not between bordered elements.
- `🎓️teaching/…/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧪️playground/🎞️timeline/🖼️panel/🟦️.tsx:130`: per-row `border-t` dividers in a timeline. These are row lines, not boxes.

**10. Not fully reviewed. Low.**
- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx:35`, `:49`, `:68`: raw `<button>` chips in the recorded presentation's navbar and footer, with `windowChromeTitleChipClass`. This is presentation chrome for a recorded clip. I did not find a border or gap class in it, so I am listing it but not counting it.

### Previously reported items confirmed fixed (skip)
- Hub: H1 Connections, H2 Overview tiles, H3 Observability tiles, H4 Spaces `border-t`, H5 add-member row, H6 ghost row, H7 rebuild row, H8 language gate. All use `SharedBorderRow`, `StatTiles` or `LanguageChoice` now.
- Semio-tech: S1 play language gate, fixed via `LanguageChoice`.
- Mit-bestand: M1 locale picker (now `LanguageChoice`) and M2 navbar switcher.
- Teaching: the course chrome (`🪟️chrome`, `ButtonGroup`, `Group`, `Notice`, `ProgressNotice`, `CardGrid`) and the staffage Button and Bar paths are already on the mechanism.

### Suggested order of work
1. Fix the stage toolbar and progress (item 1). It is the most visible and is one stylesheet plus one element.
2. Replace the dashed notices with `Notice` or shared cells (item 2).
3. Remove the collections `borderBlockEndWidth` hack by making the grid the bottom owner (item 3).
4. Move the detail tables onto shared rows and drop `last:border-b-0` (item 4).
5. Confirm the native-control reset (item 7), then clean up the hub `ADMIN_ACTION_CELL` override (item 6).

I did not change any files, and the report is not written to the ticket folder, since this was a read-only pass.