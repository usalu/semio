# Round 3: os product fixes (findings 2, 3, 5, 6, 7, 8, 9, 10, ShellHost staging buttons)

Base: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`.

## Changes
- F2 staged-args: reference chips are cells of a `SharedBorderRow` (`role=toolbar`), no `gap-tiny`, native `border`/`rounded-sm` dropped. `StagedReferenceField` now exported (for the harness).
- F3 Paint2d: panel pinned `left-0 top-0 max-h-full max-w-full`. `!border-b-0` removed; the generic rule now lives in framework `🎨️.css` (`details:not([open])[data-orientation=vertical]... > summary` drops its block-end divider). The `details` disclosure stays (tests drive it) as the one nested owner.
- F5 ShellHost bands: new `NoticeActionRow` in `🧱️notice-bands` (text cell + framework `Button` cell). Route admission = horizontal `NoticeStack` docked at `bottom-[var(--footer-band-height)]`. Check-in band = framed `SharedBorderRow` with `Button`s. Folder reconnect band (`📎️local-folders`) = `SharedBorderRow` with two framework `Button`s. Bottom-bands wrapper: no padding/gap, `items-stretch` (kept the `<div data-semio-bottom-bands="" className=...>` shape the geometry test regexes). Corner trigger: no inset, `right-0`, above the footer, `border-l border-t` only.
- F6: install band and command stall folded into one `NoticeStack` at `top-[var(--navbar-band-height)]` (no `mt-double`, no `translate-y-full`); stall rows are a nested vertical row keeping `data-semio-command-stall*` attributes.
- F7: nothing needed; framework popover default is already 0 (`sideOffset ?? 0`), no hack added.
- F8: TiledMap popup offset 12px removed (`-translate-y-full`), root marked `data-border-probe-art` (map chrome over art, documented).
- F9: NodeGraph peers legend = vertical `SharedBorderRow` with `SharedBorderRowCell` rows, pinned `right-0 top-0`; exported `PresencePeersOverlay`.
- F10: time-travel chip lives inside the window controls silhouette chip; dropped its own `rounded-sm border border-accent`, now a `data-shared-border-cell` with `text-accent`.
- Item 15 staging dialog: Cancel/Execute are framework `Button`s in one `SharedBorderRow`.

## Tests
- Updated `📎️local-folders` phone test; new `NoticeActionRow` test in notice-bands.
- vitest: `vitest-os-final.config.ts` 158/158, `vitest-os-rows-b.config.ts` 139/139; os-wide filter (notice-bands, staged-args, time-travel component, local-folders, Paint2d, NodeGraph, TiledMap) 159 pass; remaining failed files are pre-existing `bun:test` collection errors.

## NOT verified live (probe count not obtained)
The built-in browser was unreachable (hook timeouts, then navigation denied). Headless Chrome via playwright-core also fails to launch (`Failed to get the path for 1001`). Prepared and left for a later run: `round3-os-live.tsx`, `round3-os-parts.tsx` (parts `r3-bands-top`, `r3-bands-stall-only`, `r3-route-admission`, `r3-subfooter`, `r3-subfooter-busy`, `r3-corner-trigger`, `r3-staged-reference(-empty)`, `r3-paint2d(-more)`, `r3-nodegraph-peers`, `r3-time-travel`), `round3-os-run.js`, `round3-os-probe.mjs`, `vite-round3-os.config.ts`, `round3-os-site/`; vite on 5481 was started and killed. Probe at 1440x900 and 375x812 still owed. Risks to check: wrapping of the reference chip row on narrow widths (no wrap now), full-width stretch of bottom bands, the time-travel cell inside the controls chip, `bg-menu` surfaces, geometry Playwright test (`⏪️time-travel/🧪️tests/📐️geometry`) not runnable here.
