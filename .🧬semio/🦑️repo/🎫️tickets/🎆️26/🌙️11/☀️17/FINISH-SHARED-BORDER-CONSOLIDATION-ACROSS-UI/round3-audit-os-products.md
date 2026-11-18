# 🔎 Round 3 Audit — Products and OS

# Remaining shared-border violations in `🧰️framework/🛍️products` (static audit)

**Coverage:** grep-driven across the product tree (635 `.tsx`/`.css`), excluding tests, stories and content art. I read the flagged sites and the framework CSS and popover code they depend on. I did not run the live probe, and I did not read every course or presentation stylesheet line by line, so treat the lower-ranked items as leads to verify. Paths are relative to `/Users/ueli/Documents/semio/🧰️framework/🛍️products/`.

**Clean areas:** `🖥️server`, `🦑️repo`, `📓️print` and `🐾️pets` have no border usages in `.tsx` outside tests and stories. `💻️os` has no `-ml-px`, `-mt-px`, `border-l-0`, `first:`/`last:` border or `divide-*` hacks left.

## Ranked findings

### 1. Systemic: framework popovers and dropdowns default to a 4px gap from their trigger (rule 6)

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️.tsx:329`: `const gap = sideOffset ?? (anchor.closest("[data-window-silhouette]") ? 0 : 4);`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:574`: same `? 0 : 4` default.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🟦️.tsx:268`: `dropdownSideOffset = 4`.
- Why: every bordered popover, select listbox and toggle dropdown outside a silhouette floats 4px away from its trigger. Products patch this per call site (`ShellSync` passes `sideOffset={0}`) instead of fixing the root.
- Fix: in the framework, default to 0 when the trigger is a shared-border cell or frame (`closest("[data-shared-border-cell],[data-shared-border-frame]")`), and make `dropdownSideOffset` default to 0 under the same condition. Then remove the per-call overrides.

### 2. Staged reference chips: bordered buttons in a gapped flex row (rules 2, 5)

- `🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx:60`:
  `<button ... className="inline-flex items-center gap-tiny rounded-sm border px-single text-xs" ...>`
- The wrapper a few lines up is `<div role="toolbar" className="flex flex-wrap gap-tiny">` (line ~57). Each chip is a native bordered box, so neighbouring chips are separated by `gap-tiny` and each draws its own border.
- Fix: render the chips as cells of `SharedBorderRow` (or `SharedBorderChipGroup`) with `data-shared-border-cell`, drop `gap-tiny`, the native `border` and `rounded-sm`. Use a framework chip or `Button` variant rather than a raw `<button>`.

### 3. Paint2d editing panel: floating frame inset from its host and ad-hoc hacks (rules 1, 4, 5)

- `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.tsx:402`: `<SharedBorderRow as="section" ... className="absolute left-2 top-2 z-40 max-h-[calc(100%-1rem)] max-w-[calc(100%-1rem)] bg-background/95 ... shadow-lg">`
  - Why: the panel is inset 8px inside the canvas, so the frame sits apart from the edge it floats over. This is an overlay on content art, so it may be exempt, but the inset is a gap by the rule's definition.
  - Fix: pin it to the canvas edge with a shared frame, or mark the host `data-border-probe-art` and list it in the report.
- Line 425: `<summary data-shared-border-cell="" className={cell+" cursor-pointer [details:not([open])>&]:!border-b-0">` plus the `details` with `data-shared-border-group` nested inside a vertical `SharedBorderRow`.
  - Why: `!border-b-0` is an ad-hoc override, and a group nested in a group is the pattern the mechanism is meant to avoid.
  - Fix: make the `details` body a plain cell list, or use the `SharedBorderRow` disclosure primitive so the summary needs no override.
- Lines 403-439: the `cell` constant is `bg-background px-2 py-1 ...` with no border, so these raw buttons are acceptable cells. Medium-low, only because they bypass `Button`.

### 4. TextEditor footers: bordered controls and frames inset in a bordered, padded, gapped row (rules 1, 2)

- `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx:662`: `<div className="flex items-center gap-2 border-t border-border px-3 py-2">`, containing `<Button ... variant="outline">` (a bordered control) at ~line 664.
- Line 668: `<div className="flex flex-wrap items-center justify-end gap-2 border-t border-border px-3 py-2">` wraps `<SharedBorderRow className="shrink-0">` (line ~674). The shared row is inset by `px-3 py-2` and `gap-2` inside a bordered footer.
- Line 679: `<div className="border-t border-border px-3 py-2 ...">` (diagnostics) is a padded native-border band, not a shared row.
- Fix: make each footer one `SharedBorderRow` (`as` footer, `data-shared-border-edges="top"`) with its buttons as direct cells and the status text as a `SharedBorderRowCell`. Move the padding into cells, and remove `gap-2`, `px-3`, the native `border-t` and the nested `Button outline` borders.

### 5. Floating ShellHost bands and buttons: native borders, separate from the shared mechanism (rules 3, 5)

- `🏛️ShellHost/🟦️.tsx:279`: `<div role="status" ... className="pointer-events-auto absolute bottom-double left-1/2 ... gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">` (route admission).
- `🏛️ShellHost/🟦️.tsx:383`: `<div ... data-semio-checkin-band ... gap-single rounded-sm border border-normal bg-menu px-double py-single ...">` (checkin band).
- `🏛️ShellHost/📎️local-folders/🟦️.tsx:96`: `... gap-single rounded-sm border border-normal bg-menu ...` (folder reconnect), with a raw `<button>` at line 97 (`min-h-medium px-tiny underline`).
- `🏛️ShellHost/🟦️.tsx:417`: `className="fixed bottom-2 right-2 z-50 border bg-background/80"` (corner trigger).
- Why: the bands are three native-bordered boxes that can stack at the same bottom-centre spot, which the previous round already flagged for install and stall bands. The corner trigger is inset 8px from the viewport edge. That may be fine if nothing bordered sits beside it, so verify.
- Fix: route all three through `FloatingPane` or `NoticeStack` (one frame per stack, shared edges), with the stack laid out by the compositor. Use the framework button for the folder reconnect action.

### 6. Command-stall band offset from the navbar by a margin (rule 3, page-margin rule from the brief)

- `🏛️ShellHost/🟦️.tsx:318`: `className={cn("top-workbench left-1/2 mt-double -translate-x-1/2 bg-menu", installingPluginIds.length > 0 && "translate-y-full")}`
- Why: `mt-double` leaves a gap between the workbench edge and the stall frame. `translate-y-full` makes it stack under the install band, which the previous round left as two native borders.
- Fix: dock the stall `NoticeStack` flush to `top-workbench` (no margin). Fold it into the same stack as the install band so they share one frame.

### 7. Interpreter draft-conflict popover offset from its trigger (rule 6)

- `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/✍️input-views/🟦️.tsx:248-250`: `<Popover>` / `<PopoverTrigger asChild><Button ... className="shrink-0 text-destructive" />` / `<PopoverContent ... className="max-w-sm whitespace-normal">`.
- Why: no `sideOffset`, so the framework default of 4px applies (finding 1). This is the most visible case, since it sits in a field row.
- Fix: fix finding 1 at the root. Until then pass `sideOffset={0}` here.

### 8. Tiled-map feature popup floats 12px above its point (rule 6)

- `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🟦️.tsx:35`: `className={cn("pointer-events-none absolute z-10 max-w-56 -translate-x-1/2 -translate-y-[calc(100%+12px)] px-2 py-1.5", floatingMenuSurfaceClass)}`
- `floatingMenuSurfaceClass` includes `rounded-md border shadow-sm` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts:31`).
- Why: a bordered popup offset 12px from the feature it describes. It is map chrome over content art, so it may be exempt, but the offset is an explicit gap.
- Fix: anchor at 0 (the feature point with a pointer tail) or treat the map as content art and document it.

### 9. NodeGraph peer legend: native bordered box inset in the canvas (rules 4, 6)

- `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:182`: `<div className={cn("pointer-events-none absolute right-2 top-2 z-panel flex max-w-[14rem] flex-col gap-1 rounded border border-border/60 px-2 py-1 text-xs shadow-sm", glassClass)} data-level="pane">`
- Why: `rounded border` native frame, `gap-1` between rows, and `right-2 top-2` inset from the canvas edge. It is a chrome overlay.
- Fix: use a shared frame with rows as cells and no `gap-1`. Pin it to the canvas corner, or document it as content overlay.

### 10. Time-travel indicator chip: standalone native bordered chip (rule 5)

- `🛠️ShellHelpers/⏪️time-travel/🟦️.tsx:401`: `<span role="note" ... className="pointer-events-auto inline-flex items-center gap-tiny rounded-sm border border-accent bg-base px-single text-xs">`
- Why: a raw bordered chip in the window header. If it sits next to the window's silhouette chips (used at `🏛️ShellHost/🎮️focus-and-input/🎯️focused-program-and-introduction/🟦️.tsx:112`), it produces two lines at the seam.
- Fix: render it as a silhouette chip (`data-window-silhouette-chip`) so the silhouette owns the outline.

### 11. Course: load-fallback retry/reload buttons use an ad-hoc child-border hack (rules 4, 5)

- `📚️course/🎯️targets/⚛️react/🔨️modules/📚️app/🛟️load-fallback/🟦️.tsx:38`: `<span className="flex justify-center border border-border [&>button:not(:last-child)]:border-e [&>button:not(:last-child)]:border-border">` wrapping two raw `<button>`s (lines 39-44).
- Fix: `SharedBorderRow` with the two buttons as `data-shared-border-cell` cells. Drop the arbitrary-variant divider hack and the native `border`.

### 12. Course: framed containers carry both a native border and the frame attribute (verify)

- `📚️course/🎯️targets/⚛️react/🔨️modules/🚏️navigation/🟦️.tsx:76`: `<div role="group" ... data-shared-border-frame="" data-shared-border-edges="top right bottom left" className="flex w-fit border border-normal">`
- `📚️course/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx:161`: `RowList` with `framed && "border border-normal"` plus `data-shared-border-frame`.
- `📚️course/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx:414`: `Segments` (non-flush) `"w-fit border border-normal"` plus `data-shared-border-frame`.
- Why: the framework CSS zeroes only unlisted sides. Listed sides keep their native border, so the overlay and the native border paint the same coordinates. The brief asks for one hairline. This is redundant at best and could double at sub-pixel sizes.
- Fix: drop `border border-normal` from framed elements and let the compositor paint the frame. Confirm with the probe at 1440 and 375.

### 13. Course: presentation chips and handles with native frames and a raw inset button (rules 1, 3, 5)

- `🎤️presentation/🎯️targets/⚛️react/🔨️modules/🔗️shared-border-chips/🟦️.tsx:16`: `SharedBorderChipGroup` renders `data-shared-border-frame`. The CSS at `🎤️presentation/🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:82-95` also sets `border: var(--stroke-hairline) solid` on `.presentation-interaction-actions`, the same double-frame pattern as finding 12.
- The same stylesheet, around line 205-215, gives `.presentation-interaction-slide-reset` its own `border` and `border-radius`. It is a raw bordered button placed at `top` and `right: var(--spacing-single)` of a host, beside the actions group. If the two sit one cell apart they form a gap.
- Fix: remove the native `border` on the chip group (frame owns it). Make the slide-reset button a cell of the same group, or place it inside the actions frame.

### 14. Course access status: native bordered status chip (rule 5, low)

- `📚️course/🎯️targets/⚛️react/🔨️modules/🔓️access/🟦️.tsx:50`: `<p className="m-0 flex items-center gap-single border border-normal bg-background px-double py-single text-sm" role="status">`
- Why: a standalone bordered chip with its own padding. Only a problem if it sits next to another bordered element.
- Fix: check placement. If it stands alone, use the shared-border frame with the message as the cell. If it stacks, use a `SharedBorderRow`.

### 15. Lower visibility

- `🔺️DiffViewHost/🟦️.tsx:126-127`: `grid grid-cols-[minmax(0,1fr)_1px_minmax(0,1fr)] gap-x-single` with `<div className="border-border border-l" />` as a divider. The `gap-x-single` puts space between the divider and the text. The text is not bordered, so this is not a shared-border gap, but a divider cell would be cleaner. Low.
- `🗣️Interpreter/🖋️InkCanvasHost/🧱️ink-block-views/🟦️.tsx:124,134`: `border border-border` on `th`/`td` inside a `border-collapse` table (content view). Low, probably exempt.
- `🖌️Paint2dHost` and `🖋️InkCanvasHost`: raw native `input`/`select` in `SharedBorderRowField` cells are covered by the CSS rule at `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` (~L6680-6740). Not a finding.
- `🏛️ShellHost/📝️staging/🟦️.tsx:17`: `<div className="flex justify-end gap-single"><button ...>Cancel</button><button ...>Execute</button></div>`. Two raw native buttons with a gap and no framework `Button`. Medium-low, bypasses the framework controls (rule 5).
- `👥️presence/🟦️.tsx:392-427` (course) and `🎨️.css` `.course-presence` peer chips: `gap-single` between bordered peer marks. Peer marks are content art (avatars). Exempt.
- Stories excluded: `🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css` has `border-left`/`border-top` accents. Documentation only.

## Notes

- Most of the os shared-border work from earlier rounds is already in place (SpaceBrowser, AgentDelegations, HubConnection rows, NoticeStack, FloatingPane, TaskManager, AgentChatPanel). The items above are what remains by static reading.
- Root-cause fixes first: finding 1 covers the 4px popover and dropdown default for every product, and findings 2, 5 and 13 would each remove a class of raw bordered controls.
- Verification is still needed. The probe at 1440x900 and 375x812 should run on the TextEditor, Paint2d, staged-args, stall-band and course navigation states before anyone trusts these as fixed. Items 12 and 13 need the probe to confirm whether the double-line is visible.