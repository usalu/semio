# 📋️ Semio Tech Play — Design

## 🎯️ Task

`🏢️semio-tech/🎡️play` at the repository root: the general, extended version of
`♻️mit-bestand/🧺️demonstrator`. It shows **every app** of the playground registry in one live grid,
locked to **English + native terminology** (the demonstrator is German + `reuse`).

## 🔎️ Findings

- The demonstrator is one Vite page that mounts one `FrameworkOsShell` per grid cell (4×2, eight branded
  panes). It needs the component union of all its panes and gets it by merging several os-dev activation
  lanes (`generator`, `energy`, `fem3d`).
- Every pane boots its own plugin closure, so play's union is the union of the 58 per-pane closures:
  53 components. The `s` host lane would cover them but also needs `stdio`, whose wasm component cannot
  link (1,000,000-function ceiling) — its activation fails. Play therefore activates the fewest PANE
  lanes covering the union (greedy, deterministic): 26 lanes, merged into one play-owned receipt
  (`dist/♻️activation/dev`), extensions served per extension from the lane that installed them. A unit
  test pins `📋️project.json` `dependsOn` to the computed lanes.
- Locks: `ShellTerminology = "native" | "reuse"`, `ShellLocale = "en" | "de"`.
- Registry catalog rows carry no labels. Plugin descriptor `🔣️.json` files at each plugin root carry the
  `manifest.apps` list (editor/viewer). Their editor apps are the ground truth of "every app".

## 🧭️ App set

Every `PLAYGROUND_BUILD_TARGETS` row except:
- the six `entwerfen-mit-bestand-*` branded rows (they are German re-skins of other rows: generator,
  koordinator, aggregator, aussuchen, bearbeiten, verfolgen);
- the `s` host row (the OS shell itself, i.e. the launcher play replaces).

That is 58 panes. Rows without an `app` column (animate, architect, dag, flow, imperative, playbook,
reasoning-wires, sequence, vcs, writer) boot their plugin's default editor app exactly as os-dev does.
A unit test pins the pane list to this rule (no drift when a new playground appears) and checks every
source descriptor editor app (except the space host apps) is reachable through some pane.

## 🧱️ Structure (mirrors the demonstrator)

```
🏢️semio-tech/🎡️play/
  package.json, 📋️project.json, 📜️script.ts, 🌐️.html, 🎨️globals.css, public/.nojekyll
  🟦️.tsx                  landing: grid, overview cards, focus/hash routing, paced boot, suspension
  ⚛️play-card.tsx          compact overview card
  🪧️brand.ts               play brand (en/native), pane specs from the runtime catalog
  🏗️builder/🌐️vite/🟦️.ts
  🔨️modules/🧩️runtime/     🔣️.json (+🧬️schema), 🟦️.ts, 📜️script.ts, ♻️activation, 📦️assets, 🧪️e2e
  🔨️modules/📦️site/       📜️script.ts (static release build)
  🔨️modules/🧪️e2e/        📜️script.ts, 🎚️config/🟦️.ts (playwright)
  🧪️tests/                 🎚️config (vitest), unit tests, 🎭️acceptance (boots every pane)
```

## 🧮️ Grid

Never a constant: `playGridDimensions(n)` takes the shape with the FEWEST empty cells among those no more
than two columns wider than tall (ties to the squarest), so the grid follows the pane count as apps are
added — 8 panes give the demonstrator's own gapless 4×2, the 60 panes of 2026-09-20 give 9×7. A short
trailing row is centred (`playGridRowSpan`) and the free pan is clamped to the occupied columns of the
rows in view (`playOccupiedColumnRange`), so an empty cell is never a viewport of its own. Overview cards
are compact (icon, label, tagline, description, open chip). Boot is on demand (hover/focus/hash) plus a
slow `schedulePlayIdle` warm-boot queue; a live-pane budget suspends the least recently used pristine
pane so the page never holds dozens of wasm shells at once. Landing chrome reads its strings through
`useLabel` from en+de bundles (`playLandingUiLabel`, `playCardUiLabel`) — no default language, even
though every pane shell is locked to English.
