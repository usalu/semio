# 🔭️ Claude Discovery — Layout Plugin, Host and Workflows (2026-11-01)

## Plugin and host

- Plugin: `semio:layout` / pluginId `layout`. Component cdylib `🌎️hub/🧩️compositions/📏️layout/🧩️component/🦀️.rs` (crate `semio-hub-layout-component`), manifest `🧩️component/🔣️.json` + `🛂️.descriptor.semio`, lib crate `semio-hub-layout` (`📦️packages/🦀️rust`). Artifact code `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/`.
- The plugin manifest/descriptor moved from `✏️s/🔌️plugins/📏️layout/` to `🌎️hub/🧩️compositions/📏️layout/🧩️component/` in commit `4e36b2b5012` (repo-wide hub restructure); older ticket scripts referencing the old paths, the `dev` profile and `activate-layout-react-dev` are obsolete.
- Host: semio OS browser shell (`@semio-tech/framework-os-dev`), React renderer, guest built as `wasm32-wasip2` component → jco transpile → wasm-opt → staged module.

## Launch chain (inferred targets, `🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`)

| Target | Purpose |
| --- | --- |
| `semio-hub-layout-component:component` | build/describe/transpile/optimize/publish guest (`🧾️component.json` receipt) |
| `@semio-tech/framework-os-dev:prepare-layout-react-prototype` | stage boot closure into `.cache/b/p/m/browser/prototype/modules/📏️layout/` |
| `@semio-tech/framework-os-dev:activate-layout-react-prototype` | publish `.cache/b/p/m/browser/prototype/runtime/react/layout/activation/🔣️receipt.json` |
| `@semio-tech/framework-os-dev:dev-layout-react-prototype` | activate then serve |
| `@semio-tech/framework-os-dev:serve-layout-react-prototype` | Vite on `127.0.0.1:${S_OS_PORT:-6079}` (ports from component `Cargo.toml` `react=6079, wgpu=6179`) |

URL: `http://127.0.0.1:6079/?plugin=layout`. Freshness: serve prints `[stale] …` when staged module ≠ receipt; receipt row `artifactSha256` covers staged module files.

## Browser automation

- Playwright 1.64.1 + chromium-headless-shell-1248 present (`~/.cache/ms-playwright`, `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`). Run with bun (`repo tools exec bun -- <probe>`).
- React probe precedent: `🐍️layout-interact-probe.mjs` (DOM `[id="action.*"]`, `[role="treeitem"]`, `[data-surface-id]` with `data-status-json`), `☀️18/…/🐍️layout-export-pdf-probe.mjs` (download capture + `🐍️validate-pdf.py`). macOS-only flags (`--use-angle=metal`) must be dropped.
- Boot readiness: `html[data-semio-os-ready]==="layout"`, `html[data-semio-os-error]` absent.

## Intended workflows (manifest + `✏️editor/🔀️args-bridge/🦀️.rs`)

Units: page-local points, y down. Demo example: page-1 400×500 with `frame-1` rect (10,10,40,40), `frame-text-1` (156,220,80,40), `frame-image-1` (136,435,60,40, missing link), inherited `frame-inherited` (50,50,100,80) from `parent-1`; page-2 blank.

- `addFrame {kind,x,y}` → `frame-(n+1)`, rect 120×64 default at (48,48), selected.
- `addPage` → `page-(n+1)` copying active page geometry.
- `patchFrame {frameId,field,value}` / `patchPage {field,value}`; non-finite refused by mutation layer; locked frame/layer refuses geometry.
- Selection: `interactionSelect`, `selectAll` (mod+a), `clearSelection` (Esc), `deleteSelection` (skips locked).
- `translateSelection {dx,dy,phase}` stream/commit/abort, `rotateSelection`, `scaleSelection`.
- `undo` (mod+z) / `redo` (mod+shift+z), alternatives, history edit.
- `exportPdf` (all pages, `Demo.pdf`), `exportSvg`/`exportPng` (active page), `exportPackage`.

Known: `setActiveExample refused: undeclared-action` at boot; live document persistence across reload is host-owned and unverified.
