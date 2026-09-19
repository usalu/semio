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

## 🧮️ Grid for 58 panes

`columns = ceil(sqrt(n))`, `rows = ceil(n / columns)` → 8×8. Overview cards are compact (icon, label,
tagline). Boot is on demand (hover/focus/hash) plus a slow idle queue; a live-pane budget suspends the
least recently used pristine pane so the page never holds dozens of wasm shells at once.
