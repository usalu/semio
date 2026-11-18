# Round 2: one snapped source for navbar/footer band heights

## Root cause of the "1px navbar/footer pairs" in s/OS
Mostly a harness artifact plus one latent bug. In the showcase, `os-chrome-run.js` shims `cancelAnimationFrame` as a no-op. A `BorderCompositor` disposed on unmount still ran its already scheduled `commit` (it never checked `disposed`) and re-attached a second overlay SVG to the freshly mounted navbar/footer. The stale overlay painted the navbar line at 28 while the live compositor put the shared line at 29, giving the `navbar svg 28.0 || window svg 29.0` pairs. Evidence: first mount after a page load had 1 overlay and probe 0; every later `show()` had 2 overlays and probe 5. Fix: `commit` returns when `disposed` (`🗂️WindowChrome/shared-border.ts`). After it, repeated mounts in one page are clean.

## Single source of truth
Geometry bindings stay raw (theme-owned `--navbar-height`, `--footer-height`; contract/rust/python tests untouched). New derived vars, snapped once, in `🎨️styling/🖌️ui/🎨️.css` next to `--device-pixel`:

```
:root, [data-ui-theme] { --navbar-band-height: round(nearest, var(--navbar-height), var(--device-pixel)); --footer-band-height: ...footer... }
```
Declared on every theme root so a root with its own geometry re-snaps its own values. All consumers now read these and nothing rounds per consumer:
- `Navbar`, `Footer` (`h-[var(--navbar|footer-band-height)]`)
- `⚓️panel-anchor` `chromeHostedOpenPanelPositionStyle` (top/bottom offset, maxHeight, `--panel-header-height`)
- css `[data-slot=navbar|footer] --panel-header-height`
- `🏢️semio-tech/🎡️play/🏠️play-landing.tsx` (padding + insets) and `♻️mit-bestand/🧺️demonstrator` (overlay padding + insets): these used the raw 28.8 px while the bar was 29 px (0.2 px mismatch), now snapped.
Grep of `🧰️framework`, `🎓️teaching`, `🏢️semio-tech`, `♻️mit-bestand` shows no other consumer of the raw vars. Not consumers of the live DOM var: the presentation clip contract (`navbarHeight: 28.8` in its json, a fixed 1600x900 layout rendered by python/render.mjs) and print tokens; left alone. Course footer keeps its own content-height snapping (`calc-size`, round up to `--device-pixel`), unchanged. Course/staffage/hub use the shared Navbar/Footer.

Tests updated: geometry suite (new test: snap exists exactly once in css and no consumer re-rounds), shell-components-1 panel-anchor expectations, playgrid landing regexes.

## Button-group 1px overflow
Group is `h-medium` (22.4) with a 1px border box-sizing, item had `min-h-medium` (22.4) + `h-full max-h-full`; min-height beats max-height, so the item was taller than the group content box. `ButtonGroupItem` now adds `min-h-0` (ActionGroup already does). Live check on agent-chat, task-manager, hub-first-run, sync-attach: no item exceeds its group; agent-chat `overlaps` went 1 -> 0. Test added in shell-components-6 (Engagement item has `min-h-0`, not `min-h-medium`). ToggleGroup items use the same base class and the same pattern; not changed (vertical groups are `h-auto` and rely on the min height), not seen in the probe.

## Live (own tab, DPR 1 emulation)
s/OS showcase (native vite on 5397, killed) all 58 parts, 1440x900: probe count 0 everywhere except `settings-conflicts` (2) and `marketplace` (2): panel rows end 13-17 px before the panel edge vs the window edge behind (classic scrollbar gutter of the emulated pane; already listed as open item 2 in round2-os-final.md). `overlaps` remaining: layout/layout-many/layout-bottom (open panel cap's Workbench tab over the navbar title: design, the cap replaces the folded toggles), window-search, palette-scoped, find-scoped (intentional overlays). 375x812: all parts 0 except the desktop-only `layout`, `layout-bottom`, `layout-many` (two windows side by side at 375, not a real mobile layout; `layout-mobile*` are 0).
Course http://127.0.0.1:5391/?preview=1: home, #physics, run (Medium) at 1440x900 and 375x812 (run also scrolled to the bottom): count 0, overlaps 0, pending false after waking the tab with a screenshot.
Note: the s/OS showcase must be remounted via `show()` only with the fixed compositor; a hidden pane hangs the script (use screenshot first).

## Tests
- UI set (Navbar, Footer, WindowChrome, Layout, Panel, owned-locale-detector-retirement, OverviewCard, CardStack, LayeredOverview, WindowSilhouette, Window, window-pane-exclusivity, ButtonGroup, panel-anchor): 715 pass, 1 fail: Layout `keeps the window canvas full width under every open panel`, which needs a Playwright chromium that is not installed here (environment, unrelated). The "38 failed files" are driver/helper modules without suites.
- `vitest-os-rows-b.config.ts`: 139/139. `vitest-course.config.ts`: 1125/1125 (4 helper files without suites).
- The bun-only geometry suite and playgrid test cannot run here (no bun); edited by pattern, not executed.

## Files
`🖱️ui`: `🎨️styling/🖌️ui/🎨️.css`, `🧱️elements/🔝️Navbar/🟦️.tsx`, `🔚️Footer/🟦️.tsx`, `🔳️ButtonGroup/🟦️.tsx`, `🗂️WindowChrome/shared-border.ts`, `🔨️modules/⚓️panel-anchor/🟦️.ts`, tests: `🎨️styling/🧪️tests/🧩️suite/📐️geometry/🟦️.ts`, `🧪️tests/🧪️owned-locale-detector-retirement/🎨️shell-components-1/🟦️.tsx`, `🧩️shell-components-6/🟦️.tsx`. Also `🏢️semio-tech/🎡️play/🏠️play-landing.tsx` (+ `🧪️tests/🧪️playgrid/🟦️.ts`), `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`.
