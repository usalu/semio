# 🔁 Role switch regression — 2026-09-14/15 (lane `role-switch-regression`, React :6028)

Owns `📓️coordinator-walk-2026-09-14.md` item 19 (23:32, :6028): *"Click on the navbar `Viewer` control does
nothing: navbar stays `Editor`, edit layout stays, no viewer preview. Worked at 17:10 today."* — and item
18's *"after clicking `Viewer` the navbar stays `Editor`"* on :6027.

---

## 0. The one-line version

The role switch was never broken. The navbar `Viewer` **button** was under the chrome panel tab rail: a
`centered` navbar item is absolutely positioned over the WHOLE bar, so it is invisible to the bar's own
flow row, and when the `Tool runs` panel tab (commit `f2fc008ee2`, 15:57) widened the `top-right`
`PanelChromeTabBar` its left edge moved from **1039 px to 947 px**, straight across the role group's right
half. At 1280 px `document.elementFromPoint` at the `Viewer` button's own centre (969, 15) resolved to
`framework.panel.inspection`, so every click reached the panel rail and the shell stayed in the editor
role. Fixed in the owning layer (`🖱️ui/🧱️elements/🔝️Navbar`): a centered item is still centered on the
bar, but is now placed inside the free band the flow row leaves, and clamped to it.

**Viewport click recon, `Viewer` click → role flipped: 2/5 → 5/5.** Law `🛟️ navbar centered band`
**19/19**. Battery `role-switch` **9/9 green**; `viewer-actions` 12/15, all three reds the pre-existing
`viewer-export` defect owned by `window-gaps-followup`.

---

## 1. Repro

`🐍️role-switch-regression-probe.mjs` (new) drives the converged
`?plugin=generation3d&example=sphere-cut-with-torus` shell on :6028 with a full console capture and a
per-second snapshot of both navbar groups, the mounted windows and the console's instance ids.

It did **not** reproduce at 1600×1000 (`🗑️generated/react-role/repro-1`), nor after a 150 s quiet dwell
(`repro-2`, 0 console lines in 150 s — no `toolRunPace` storm on :6028), nor after canvas click + `f` +
example pick (`repro-3`). In all three the role flipped in ~2 s with `draining=0`, `sealed=0`,
`noActor=0`, `pageErrors=0`. So the transactional switch itself (`🔀️surface-switch`,
`switchToPluginApp`) was healthy, and none of the day's suspects (mounted-window fetch / dropped cached
bodies, the ingress-generation gate, `useShellFloatingSurfaceHost`, the selection-prune lane, a guest
role change) was involved.

`🐍️role-switch-hit-recon.mjs` (new) then booted the same page at five viewports and clicked the `Viewer`
button with a **raw mouse click at its own centre** — the gesture a user makes, not a selector click:

| viewport | `elementFromPoint` at the `Viewer` centre | role flipped |
|---|---|---|
| 1600×1000 | `playground.navbar.roles.viewer` | ✓ |
| 1440×900 | `playground.navbar.roles.viewer` | ✓ |
| 1280×800 | **`framework.panel.inspection`** | ✗ |
| 1152×720 | **`framework.panel.top-right.fold`** | ✗ |
| 1024×768 | **`framework.panel.toolRun`** (and `Editor` → `…top-right.fold`) | ✗ |

`🗑️generated/react-role/hit-recon/results.json`. The coordinator's in-app browser pane is narrower than
1440 — that is the whole difference between their walk and a 1600-wide probe, and why the same click
worked at 17:10 (see §2) and not at 23:32.

## 2. What was actually wrong, measured

`🐍️navbar-rail-overlap-recon.mjs` (new), 1280×800, `🗑️generated/react-role/overlap-recon/`:

- `nav#ui.navbar` is `relative h-large z-base` (`z-index: 0`).
- Its flow row (`p-single flex gap-single items-center min-w-0 h-full`) carries, in order:
  `topLeftPanelTabs` (0…173), `navbarFillItem` (`flex-1 min-w-0`, **renders nothing**, 173…944), and
  `topRightPanelTabs` (947…1185, `data-slot="panel-tabs"`, `z-40`).
- The `centered` item — logo/title + role chip + example select + mode switcher + **role switcher** +
  `top-middle` tabs — is rendered in a SEPARATE `pointer-events-none absolute inset-0 flex items-center
  justify-center` layer, i.e. centered on the **full 1280 px** and reserving no space in the row. Measured
  cluster: 264…1017. Role group: 831…1016. `Viewer` button: 923…1016, centre **969**.
- 947 < 969, and the rail paints at `z-40` inside the dock's own stacking context, so the rail wins the
  hit test. `elementFromPoint(969, 15)` → `framework.panel.inspection`.

**Why it became a regression today.** The `top-right` rail's buttons at 1280 are
`framework.panel.inspection` (x 947, w 92) and `framework.panel.toolRun` (x 1039, w 85). `toolRun` is the
**Tool runs** panel introduced by peer commit `f2fc008ee2` (2026-09-14 15:57, after the last green React
battery at 15:18). Without it the rail starts at **1039**, and the `Viewer` button (923…1016) is entirely
clear of it — which is exactly what the coordinator saw work at 17:10, before that panel's tab was open in
their pane. The rail's left edge moving 1039 → 947 is the regression; nothing in the role-switch code
changed.

The `:6027` half-applied shape (item 18: instance 2 + fallback `Number → Math.add` graph) did **not**
reproduce: across four switches the shell mounts exactly one window instance at a time, `noActor=0`,
`sealed=0`. At the `Viewer` coordinates on a narrow pane a click lands on a panel tab or on
`framework.panel.top-right.fold`, which spawns nothing — that instance 2 came from the coordinator's own
earlier example switch, not from the role control. Not claimed as explained.

## 3. The fix — one owning layer

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️.tsx`

Three pure rules plus one shared body, and the `Footer` (which duplicated the same overlay markup, so it
had the identical defect at its own `bottom-middle` centered tabs) now renders through the same body:

- `navbarFlowChildOccupiesV1(child)` — whether a flow-row child actually occupies its box. A
  `navbarFillItem` renders nothing, so its wide `flex-1` box IS the room a centered item may use; every
  other child is chrome to stay clear of.
- `navbarFreeBandV1(width, occupied)` — the widest horizontal band of the bar no occupied span covers
  (spans clamped to the bar, overlaps merged, widest gap wins). A bar whose every pixel is occupied
  answers its whole width rather than a zero-width band: displacing an item into nothing is not an
  improvement.
- `navbarCenteredLeftV1(width, band, contentWidth)` — the bar's own centre whenever the item fits there
  without crossing the band's edges, otherwise the nearest position inside the band; an item wider than
  the band starts at the band's left edge. **True centering is kept** wherever there is room — the band
  only ever pulls the cluster in.
- `NavbarBandBody` — the flow row + one absolutely positioned layer per centered item, placed by the rule
  above against a live measurement (`ResizeObserver` on the row, its children and each centered cluster; a
  `MutationObserver` on the row subtree, because which chrome the bar carries — panel tab bars above all —
  is a runtime user choice, and a band computed once boots stale the first time a panel opens). Until the
  first layout pass the cluster centers by `translateX(-50%)`, so the first painted frame is already
  centered and nothing flashes in from the left edge. Each centered cluster carries
  `max-width: <band width>` and `data-slot="navbar-centered"`.

The clamp is a **fixed point**: the cluster is absolutely positioned, so it never feeds back into the
row's layout, and its measured width is `min(natural, band)` — placing that width again answers the same
left (covered by a law).

Files:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️.tsx` — rules + `NavbarBandBody`.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🟦️.tsx` — renders through `NavbarBandBody` (19 lines of
  duplicated overlay markup removed).
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — the three rules exported from the react barrel.
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔝️navbar-centered-band/🔣️.json` — new fixture.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔝️navbar-centered-band/🟦️.ts` — new law.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` —
  new `uiSuite(…)` helper + the suite registered, because the `ui` module declares no vitest project of
  its own and a suite no runner includes is a gate that reads green while measuring nothing.

## 4. Laws, with output

```
SEMIO_TEST_LEVEL=long bun node_modules/vitest/vitest.mjs run \
  --config 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts \
  🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔝️navbar-centered-band/🟦️.ts --reporter=verbose

 Test Files  1 passed (1)
      Tests  19 passed (19)
```

19 cases: the contract block, 4 occupancy cases, 6 band cases (the measured 1280 playground bar, an empty
bar, merging overlaps, widest-gap-not-first, a fully occupied bar, spans past the edges), 5 placement
cases (including the exact regression: a 753 px cluster in a `173…947` band lands at `left = 194`, so its
right edge is 947 — flush with the rail, never under it), plus the "never crosses its band", "is a fixed
point" and "the 1280 px playground navbar" invariants.

`bunx tsc --noEmit` on `@semio-tech/ui-react` reports nothing in the three files touched (the two hits
under `🔝️Navbar/📖️stories` and `🔚️Footer/📖️stories` are pre-existing: an undefined `defaultItems` and an
unknown `"check-circle2"` icon name).

## 5. Browser proof

**Viewport recon, after the fix** (`🗑️generated/react-role/hit-recon-fixed/results.json`), same five
viewports, same raw mouse click at the button's centre:

| viewport | hit at the `Viewer` centre | role flipped |
|---|---|---|
| 1600×1000 | `playground.navbar.roles.viewer` | ✓ |
| 1440×900 | `playground.navbar.roles.viewer` | ✓ |
| 1280×800 | `playground.navbar.roles.viewer` | ✓ |
| 1152×720 | `playground.navbar.roles.viewer` | ✓ |
| 1024×768 | `playground.navbar.roles.viewer` | ✓ |

**5/5, 0 page errors.** At 1280 the `Viewer` button moved 923…1016 → 853…946, clear of the rail at 947. At
1024 the cluster's `max-width` lets the title truncate rather than run under the rail; every control stays
hit-testable.

**Click-driven round trip at 1280×800** (the regression viewport),
`🗑️generated/react-role/proof-1280/`, sphere-cut-with-torus, each step waiting for the role AND for the
example to converge (`data-status-json` `phase: idle`, `meshes > 0`):

| step | gesture | roles | window | meshes | verdict |
|---|---|---|---|---|---|
| 1-boot | — | editor | `procedural-preview` | 1 | ✓ 9 s |
| 2-click-viewer | **mouse click** on `Viewer` | viewer | `procedural-view-preview` | 1 | ✓ 15 s |
| 3-chord-editor | `⌘⌥E` | editor | `procedural-preview` | 1 | ✓ 24 s |
| 4-chord-viewer | `⌘⌥V` | viewer | `procedural-view-preview` | 1 | ✓ 32 s |
| 5-chord-editor-back | `⌘⌥E` | editor | `procedural-preview` | 1 | ✓ 44 s |
| 6-mode-step | `⌘⌥→` | edit → **generate** | `generation3d-generate-preview` | 0 | ✓ 47 s |
| 7-mode-back | `⌘⌥←` | generate → **edit** | `procedural-preview` | 1 | ✓ 49 s |

`pageErrors=0`, `surface-switch draining=0`, `sealed-instance` drops `0`, `no actor for instance` `0`,
`actor-activation.revoked` `0`. Mode switching (Edit ⇄ Generate) works by chord and by button; the
generate preview legitimately carries 0 meshes (no generation added).

**No instance leak.** Instance ids named by the console across the run: 1 → 5, i.e. exactly one new
instance per role switch, as `runSessionAppSwitchV1` specifies (create the successor, then retire the
sealed predecessor). At every snapshot exactly ONE `[data-window-instance-id]` set is mounted, and the
`typed-operation slots` ledger reads `live=0/64` for each retired instance.

**Battery** —
`SEMIO_BATTERY_URL=http://127.0.0.1:6028/?plugin=generation3d SEMIO_BATTERY_ROOT=react-role bun 🐍️react-battery.mjs --only=role-switch,viewer-actions`
(`🗑️generated/react-role/scoreboard.json`):

| row | result |
|---|---|
| `role-switch` | **9/9 green**, 33 s — boot, mid-chain chord, post-editor, post-viewer, generate, `no actor` 0, page errors 0, shell faults 0 |
| `viewer-actions` | 12/15, 803 s, 0 page errors — viewer role, measures, Show×2, LOD×2, Sun toggle + azimuth, example switch from the viewer, export row all green |

## 6. Open, NOT claimed

- **`viewer-actions` 12/15** — all three reds are one pre-existing defect owned by `window-gaps-followup`
  (`📓️react-window-gaps-2026-09-14.md` §7, "only `viewer-export` red"): `viewer-export` never produces a
  download (`exportDocument {format: stl|obj}` fails `typed-operation failed: retained command reducer
  rejected operation`), and the format menu lists 7 labels of which the declared `txt` has no row. Nothing
  to do with the navbar or the role switch.
- **One retirement of four failed** during the 1280 proof:
  `switchToPluginApp: predecessor procedural/s.procedural.generation3d@1/*#editor retirement failed Error:
  plugin-ui.owner-close-budget-exhausted` at `retireInstanceLifecycle` (`🔌️PluginRuntime/🟦️.tsx:1225`).
  The switch is unaffected by design (the close ladder is started, never awaited) and no later gesture
  failed, but the close budget is genuinely exhausted on a converged editor instance. Not investigated by
  this lane.
- **`aria-keyshortcuts` disagrees with the badge**: the role buttons render `Viewer ⌘️⌥️V` while publishing
  `aria-keyshortcuts="Control+Alt+V"` — a screen-reader user is told the wrong chord on macOS. Owned by
  the a11y lane, untouched here.
- The centered-band rule is measured in the browser only; jsdom has no layout, so the DOM half of the law
  is the pure rule plus the `elementFromPoint` recon above rather than a rendered assertion.

## 7. Artifacts

- `🐍️role-switch-regression-probe.mjs` — repro/proof driver (viewport, dwell, interact, convergence-gated).
- `🐍️role-switch-hit-recon.mjs` — per-viewport `elementFromPoint` + real-click recon.
- `🐍️navbar-rail-overlap-recon.mjs` — navbar/rail geometry, stacking and overlap dump.
- `🗑️generated/react-role/{repro-1,repro-2,repro-3,hit-recon,hit-recon-fixed,overlap-recon,proof-1280,role-switch,viewer-actions,scoreboard.json}`.
