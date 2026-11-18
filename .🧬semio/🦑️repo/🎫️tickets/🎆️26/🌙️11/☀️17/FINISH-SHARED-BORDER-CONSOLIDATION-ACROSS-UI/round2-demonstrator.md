# Round 2 — mit-bestand demonstrator

## How it was served (native)

- `repo run` and the project's own `dev` chain cannot run here: `readDemonstratorActivation` needs the nine lane activation receipts (cargo + wasm builds, owned by ticket `MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS`), and the Vite config only loads with `NODE_OPTIONS=--experimental-transform-types` plus a generated plugin catalog.
- Recipe used (port 5393, cache `.cache/b/p/m/border-demonstrator`, killed afterwards):
  1. Catalog: `demonstrator-native-lanes.sh catalog` (copy of the CDN ticket's `native-lanes.sh`, prototype mode, logs in this ticket's `🗑️generated`, deleted) wrote `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`.
  2. `NODE_OPTIONS=--experimental-transform-types SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=… node_modules/.bin/vite --configLoader native --config vite-demonstrator.config.ts --mode prototype --port 5393 …` where `vite-demonstrator.config.ts` (this folder) wraps the real factory with `command: "build"` (no activation lookup) and drops the browser-artifact plugin.
- Consequence: no pane plugin (generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik) can boot, so every opened pane shows the OS shell in its "Plugin Recovery / This program crashed" state (navbar, footer with credits, panel dock, recovery buttons). The healthy internals of the eight apps (canvases, 3D, panels, toolbars, dialogs) were NOT probed; they are the OS shell and plugin UIs, probed by the OS/product rows. Re-run `demonstrator-probe-run.js` once an activated prototype or the CDN release build is served.
- Probe helper: `demonstrator-probe-run.js` (imports `border-gap-probe.js`, skips the introduction, returns compact JSON). Test config: `vitest-demonstrator.config.ts`.

## Changes (all demonstrator-owned)

- `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`
  - Card grid: overlay is `grid items-stretch`, rows `minmax(0,1fr)`, `padding-block: var(--navbar-height) var(--footer-height)` so the first row starts on the navbar's bottom line and the last row ends on the footer's top line; the cell wrapper is `CardGridCell` (no `justify-center`, no `px-double`, no centring).
  - Partner credits: the absolutely positioned wrapped row (`gap-single`, `px-double py-single`) is now the real `Footer` (`navbarFillItem` between the two credits), band height `--footer-height`; swipe insets use `--navbar-height` / `--footer-height`.
  - Opened pane: the framework's floating "Overview" button (`right-double top-double`, sat on the shell's Fullscreen toggle) is gone (`labels.overview` omitted); a demonstrator `Navbar` band with an Overview `Button` (keeps `data-layered-overview-button`, `aria-keyshortcuts="Escape"`) sits above the shell, sharing its bottom line with the shell navbar. `openedId` is controlled (initial value from the hash). The shell stays mounted across open/close (same wrapper tree, only classes change).
  - The overview-state live pane previews (behind the glass) are marked `data-border-probe-art` (miniature app previews = content art); the opened pane is not marked.
- `⚛️demonstrator-card.tsx`: dropped `max-w-sm` (card and content), `h-full`, `data-demonstrator-card`.
- `🎨️globals.css`: paint layer and chrome stack of `[data-demonstrator-card]` stretch to the cell (`height: 100%`) so neighbours share one line.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx` (shell, tiny, shown in every crashed pane): the plugin-recovery "Restart App / Disable Plugin" `flex gap-single` row is a `SharedBorderRow`.
- Test: `🧪️tests/🎭️acceptance/🟦️.ts` (Playwright, not runnable here) "overview chrome" now asserts that cards of one row abut (< 1 px). Unit tests (`🏛️shell-brand`, pane branding, brand in-source): 3 files, 29 passed via `vitest-demonstrator.config.ts`.

## Probe table (`count`)

| View | 1440x900 | 375x812 |
|---|---|---|
| Overview with introduction (info box, Next/Skip) | 0 | 7 (framework, below) |
| Overview, introduction skipped (8 cards, navbar + language choice, footer credits) | 0 | 1 (framework, below) |
| Opened generator | 0 | 0 |
| Opened koordinator | 0 | not run |
| Opened aggregator | 0 | not run |
| Opened energie | 0 | not run |
| Opened aussuchen | 0 | not run |
| Opened bearbeiten | 0 | not run |
| Opened verfolgen | 0 | not run |
| Opened statik | 0 | not run |

All opened panes share the crashed-plugin shell state (see above); mobile was run on generator only as the shell is identical.
Before this ticket: overview 1440 = 13 / 17 pairs (cards 347 px wide centred in 360 cells, 15 px gaps, 140 px+ between rows, navbar/footer credits floating over cards), opened pane 8 (Overview button over Fullscreen toggle and others).

## For the framework card/layout agent (not patched)

1. `LayeredOverview` swipe mode (touch phones, also what the 375x812 emulation gets): cell padding `inset + 44px` leaves the card 14.3 px below the navbar band (`toggle-group «EnglishDeutsch» css:bottom || demonstrator-pane-card-stack svg:h`, 14.3). Cards should start on the navbar bottom line / end on the footer top line, hints sharing bands with them (hints are already flush in the latest build).
2. `UIIntroduction` on the overview at 375x812 (6 pairs, 6-40 px): the `introduction-info-box` is not flush to the cards it overlaps (12 px), its `Next/Enter` button group sits 6.4 px inside the box's bottom edge and 9.3 / 12.5 px from its sides, and the demo ripple floats. On 1440x900 the intro is clean (0) in the latest build.
3. `OverviewCard as="button"`: stack/paint layers are not `h-full` (section variant is). The demonstrator stretches them with `[data-demonstrator-card]` CSS; a root fix would make `as="button"` fill its cell like `section`.
4. OS shell: the Fullscreen toggle and the demonstrator's Overview band are two stacked navbars while a pane is open; the clean solution is a shell prop for extra trailing navbar items (the Overview action as a cell of the shell navbar).
5. Shell brand footer credits at 375 px overflow horizontally (Zukunft Bau logo cut) in opened panes because `shellFooterNavbarItem` ignores a compact/icon-only mode on phones (content issue, not a border pair).

## Content art
`data-border-probe-art`: the live pane previews behind the overview glass (`DemonstratorShell` wrapper, only while not opened).

## Not verified
Healthy pane interiors (see top), German locale, tablet, the Playwright acceptance spec, and the static release build (`build`) were not run; the CDN session owns the build and none of its files were touched.
