# Round 3 — staffage fixes (audit findings 1, 2, 3, 4, 7, 8)

Item 6 (hub ADMIN_ACTION_CELL) skipped as instructed. Paths are under `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/`.

## Changes
1. **Stage toolbar** (`🧊️spatial/🖥️stage/🟦️.ts`, plain DOM, also used in shadow/standalone, so self-contained CSS instead of the React mechanism): bar, progress and notice are one row each (`semio-stage-row`) of cells. No `gap`, no group border, no first-child hack. Cells draw only `border-inline-start` and `border-block-start`; the row clips the outer-edge seam (`clip-path`), the view draws one top seam when a block precedes it. Progress row = progress cell, text cell, cancel cell; notice = words cell + retry cell. Stage root has `gap:0`; outer frame only when `framed` (default true for `<staffage-figure>`; `ModelFigure` (React) passes `framed:false` because its cell owns the edges, found live: the stage's bottom/right line sat 1px next to the parent cell's line = 2px seam). High contrast now switches `--semio-edge` to `CanvasText` instead of `border-width:2px`; the dashed `aria-disabled` rule is gone; focus outline moved inside (`outline-offset:-2px`) so the clip does not cut it.
2. **Notices**: sheet unavailable cell now `<Notice kind="unavailable" plain live={false}>` (no dashed box; `live` prop added so the sheet keeps its single live region). Figure refusal (`🧍️figure`), gate error (`🧊️spatial/🚪️gate`, raw button now unframed tint) and the 4 web-component innerHTML boxes (`🧩️element`) lost the `1px dashed` box and use a tint (`color-mix` of currentColor) instead; figure/gate/element stay free of the ui-react chrome dependency since they are embeddable.
3. **Collections** (`⭐️collections`): the two `style={{borderBlockEndWidth:0}}` hacks replaced by `edges={GRID_BELOW}` (`top right left`) on the sections that end in a card grid; the grid owns the bottom line.
4. **Detail tables** (`🔬️detail/🧱️tables`): `last:border-b-0` removed (framework already zeroes the last row inside a shared frame: `table > tbody > tr:last-child`).
7. **Native controls**: verified in the framework CSS (`[data-shared-border-field] > select|input` lose border/radius/background; range excluded, has no frame): no change needed.
8. **Notice plain**: now a vertical unframed `SharedBorderRow` — words cell, then `Bar framed={false}` with `asCells(actions)`.

## Tests
- `🧪️tests/🧷️shared-borders/🟦️.tsx` (+ `.feature`): plain Notice actions are cells of one group; stage stylesheet has no gap/group frame/first-child/2px; no `dashed` in sheet/element/figure/gate; no `borderBlockEndWidth` in collections; no `last:border-b-0` in tables.
- `🧪️tests/🔭️viewpoints`: unavailable cell asserts the Notice group and that only notice icons are svgs.
- Full staffage react suite natively (`vitest-teaching.config.ts`, cwd `📦️packages/🟦️typescript`): 272 pass, 2 failed: `catalog-loading` (73 s, pre-existing) and one viewpoints test caused by my Notice change, which I fixed (viewpoints + shared-borders re-run: 41/41).

## Live verification
Server on 127.0.0.1:5471 (cache `border-staffage-r3`, `r3-staffage-start.sh`), own tab, probe `border-gap-probe.js` via `r3-staffage-views.js`.
- Done live (1440x900, domestic-cat 3D model): count 0 in preparing (progress row with cancel cell), ready, cancelled, and with an injected notice row (words + retry cells). Computed borders confirm one line per seam; found and fixed the stage-edge/parent-cell 2px seam (framed prop).
- NOT done live: collections (with/without assets), detail tables, sheet unavailable state, other asset categories, and everything at 375x812. After my later edits (Notice plain/live, sheet, collections, tables) the Browser pane stopped answering (every `mcp__Claude_Browser__*` call: "PreToolUse hook did not respond"), and headless Chrome via playwright-core cannot start in the command sandbox (`Failed to get the path for 1001`). Ready-to-run script for a later pass: `r3-staffage-pw.mjs [desktop|mobile] [routes]` (playwright, needs a working Chrome) or the same probe in the pane via `r3-staffage-views.js`.
- Dev server note: with `TEACHING_ARCHITECTURE_STAFFAGE_WATCH=off`, edits are not picked up until the server is restarted (cost me a stale-CSS detour).
- Server on 5471 killed.

## Not changed / notes
- Viewpoint label chip on the sheet figure and the 3D canvas itself are art; unchanged.
- Another agent's playground figure-picker and the framework `data-shared-border-edges` rule were not touched; the collections fix relies on the `edges` prop of `SharedBorderRow` (compositor), so please confirm live that the section with a grid shows exactly one bottom line.
