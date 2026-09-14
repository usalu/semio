# 🗨️ Popover contrast — floating surfaces resolve the shell's appearance scope (lane `popover-contrast`, 2026-09-14)

Fixes coordinator-walk item 5 (`📓️coordinator-walk-2026-09-14.md`): the navbar example picker opened a popover whose
10 rows were unreadable over the dark shell. **Not claimed** — nothing here is asserted without a measurement; every
number below comes from a run recorded under `🗑️generated/popover-contrast/`.

## Root cause

Appearance in this shell is a **scope**, not a document flag.
`applyElementsSurfaceChromeAppearanceDom` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`) paints `.dark`,
`data-ui-appearance` and the `--base`/`--foreground` inline pair on the **shell's own `.semio-scope` root**, and leaves
`document.documentElement` on the light palette (`documentElementClass: ""`, `body` computes
`rgb(247,243,227)` / `rgb(0,17,23)` while `.semio-scope` computes `rgb(0,17,23)`).

Two consumers still read the document root instead of that scope:

1. **DOM floating surfaces.** `SelectContent`, `PopoverContent` and `DialogPortal` portaled to `document.body`
   (`modalLayer.container ?? container ?? document.body`). Inherited custom properties resolve against the **computed**
   value of the DOM parent, so a surface mounted on `body` read the LIGHT `--base`/`--foreground` and painted its
   `data-level="menu"` glass + `text-popover-foreground` in the light appearance over a dark shell.
   `ContextMenu`, `Tree` and `ChromeControlHint` already portaled into `ShellScope.portalLayerRef` — the seam existed,
   three elements just did not use it.
2. **Canvas / WASM paints.** `currentStylingAppearanceName()` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts`)
   answered from `document.documentElement.classList.contains("dark")` — permanently `false` inside a scoped shell — and
   `probeCssComputed` appended its probe span to `documentElement`, so every `resolveColorHex` / `serializeCanvasThemeJson`
   answer was the light palette. Measured: the flow node graph cleared to `[240,236,221]` (`--color-light-8-9`) inside a
   dark shell. `useCanvasAppearanceSync` compounded it by observing `documentElement`, which never mutates on an
   appearance change, so nothing ever re-synced.

The token pair itself is fine: at menu level the shipped formula gives 4.84:1 (light) and 6.22:1 (dark). Only the scope
was wrong — composing the light pair over the dark shell ground yields **1.19:1**, exactly what the coordinator measured.

## The one owning layer

- **DOM surfaces** — `useShellFloatingSurfaceHost()` in `🧱️elements/🐚️ShellScope/🟦️.tsx`. Every floating surface now
  resolves its portal host through this single hook (shell portal layer, `document.body` only outside any shell), and it
  re-resolves once the portal layer attaches.
- **Canvas/WASM paints** — `setStylingAppearanceRoot` / `clearStylingAppearanceRoot` / `stylingAppearanceRootElement` /
  `subscribeStylingAppearanceRoot` in `🎨️styling/🌓️theme/🟦️.ts`. The surface-chrome lease registers the root it paints;
  everything that resolves a paint reads that root.

No element is special-cased; the example picker is just one `Select`.

### Two traps found while wiring it

- `clearElementsSurfaceChromeDom` initially unregistered unconditionally. The page's `documentElement` lease is released
  ~0.1 s **after** the shell registered its own root, so the shell's registration was dragged down with it and the flow
  canvas re-resolved light 40 s later. `clearStylingAppearanceRoot(root)` now clears only its own root.
- The portal layer is `pointer-events-none`; `SelectContent`/`PopoverContent` carried no `pointer-events-auto`
  (`ContextMenuChrome` and `DialogPortal` already did), so they needed it to stay clickable in their new host.

## Files changed

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🟦️.tsx` | new `useShellFloatingSurfaceHost()` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx` | portal host + `pointer-events-auto` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️.tsx` | portal host + `pointer-events-auto` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx` | portal host (command palette, every modal) |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx` | routed through the hook; dead `getDocumentBody` removed |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💡️ChromeControlHint/🟦️.tsx` | routed through the hook |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` | routed through the hook |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts` | appearance-root seam (register / clear / read / subscribe); `probeCssComputed` probes inside it |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | registers the root from the surface-chrome lease; `useCanvasAppearanceSync` observes it; re-export |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🧪️tests/🧩️component/🟦️.tsx` | **new** — the law |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers the law file |

## The law

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🧪️tests/🧩️component/🟦️.tsx` — 9 cases, four groups:

1. **appearance scope** — every floating surface (`select-content`, `popover-content`, `dialog-portal`, tooltip hint)
   mounts inside the shell's portal layer, never as a `document.body` child; two mounted shells keep their own.
2. **contrast** — parses the **shipped** stylesheets (`🎨️palette/🎨️.css`, `🖌️ui/🎨️.css`), re-derives the menu-level glass
   fill and `--color-popover-foreground` per appearance from the declared step knobs, composites the glass over its own
   shell ground and asserts ≥ 4.5:1 with the third-party `color` package as the WCAG oracle. A third case composes the
   opposite appearance's pair over the shell and asserts it is unreadable — the scope is load-bearing, not cosmetic.
3. **one owning layer** — every portaling element mentions `useShellFloatingSurfaceHost` and no longer carries a
   `?? document.body` / `document.body : null` portal fallback.
4. **canvas appearance scope** — `currentStylingAppearanceName()` follows the registered `.semio-scope` root while
   `documentElement` stays light, and falls back to `documentElement` once that root leaves the document.

```
bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🐚️ShellScope"
 Test Files  1 passed (1)
      Tests  9 passed (9)
```

Full `@semio-tech/ui-react` suite: **728 passed, 14 failed** — the same 14 that failed before this lane touched anything
(12 are `/@fs`-prefixed `readFileSync` path failures in the big in-source suite, 1 a peer's navbar fullscreen-toggle
change, 1 the `UIDialog` "nested owned kind picker" case — that last one was **verified pre-existing** by running the
suite with `Select`/`Popover`/`Dialog` restored from `HEAD`, where it still fails).
`@semio-tech/ui-styling` suite (the peer `DARK-APPEARANCE-EMPHASIS-FOREGROUND` laws): **58 pass, 0 fail**.
Engine `🔬️engine-contract` at `SEMIO_TEST_LEVEL=long`: 610 pass / 4 fail, all four in selection-payload, ribbon-glass and
window-action-scoping cases owned by other in-flight lanes — none touch appearance or portals.
`typecheck` is clean for every file this lane touched.

## Before / after — measured in Chromium at 1600×1000 on `http://127.0.0.1:6021/?plugin=generation3d`

Probe: `🐍️popover-contrast-probe.mjs` (`SEMIO_PROBE_SCHEME=dark|light`). Colours are the **painted** values: the row's
`color` and the full composited background stack, both resolved through a canvas so `oklab()` / `color(srgb …)` are
handled exactly as the compositor does. Contrast is WCAG 2.x.

| Surface | Appearance | Text | Background | Contrast | Portal host |
|---|---|---|---|---|---|
| Navbar example picker (before) | dark shell | `#525d5c` (`oklab(0.468713 …)`) | `#465252` (light glass over dark shell) | **1.19:1** | `document.body` |
| Navbar example picker | dark | `#9fa39a` | `#142428` | **6.23:1** | shell portal layer |
| Navbar example picker | light | `#525d5c` | `#dadacc` | **4.83:1** | shell portal layer |
| Window Options → shading Select | dark | `#9fa39a` | `#142428` | **6.23:1** | shell portal layer |
| Window Options → shading Select | light | `#525d5c` | `#dadacc` | **4.83:1** | shell portal layer |
| Command bar panel row (`Set Driver…`) | dark | `#90958f` | `#001117` | **6.30:1** | shell subtree |
| Command bar panel row | light | `#626c69` | `#f7f3e3` | **4.88:1** | shell subtree |

Flow node-graph canvas clear colour inside the dark shell, read from the theme payload actually handed to the flow WASM
(temporary `[DEBUG]` log, since removed):

- before — `rasterClear [240, 236, 221, 255]` (`#f0ecdd`, `--color-light-8-9`), `appearance light`, root `""`
- after — `rasterClear [12, 28, 33, 255]` (`#0c1c21`, `--color-dark-6-7`), `appearance dark`, root `semio-scope dark`

### Notes on the walk-items that turned out not to be popovers

- **Window Options** is an inline measures rail on the window chrome, not a floating surface. Its own shading `Select`
  *is* one, and is covered above.
- **Command** (bottom bar) opens `framework.panelTab.framework.category.command`, a `data-slot="panel"` Tree rendered in
  the shell subtree — always in scope. What looked light in the first screenshot was the node-graph canvas behind it,
  i.e. cause (2).

## Screenshots

```
🗑️generated/popover-contrast/dark/00-shell.png
🗑️generated/popover-contrast/dark/01-example-picker.png      ← the fixed picker
🗑️generated/popover-contrast/dark/02-window-options.png
🗑️generated/popover-contrast/dark/03-command-panel.png
🗑️generated/popover-contrast/light/01-example-picker.png
🗑️generated/popover-contrast/light/02-window-options.png
🗑️generated/popover-contrast/light/03-command-panel.png
🗑️generated/popover-contrast/canvas/canvas.json             ← DOM behind the graph canvas is dark; the canvas was not
```

Probes kept in the ticket folder: `🐍️popover-contrast-probe.mjs`, `🐍️popover-contrast-recon.mjs`,
`🐍️canvas-appearance-probe.mjs`, `🐍️styling-module-identity-probe.mjs`.

## Left open (other lanes)

- `UIDialog` "keeps a nested owned kind picker focusable and dismisses it before its dialog" fails on the current tree
  independently of this lane (submit is never called after selecting `Terrain`). Verified against `HEAD` sources.
- The 12 `/@fs` `readFileSync` failures in `🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` are a path bug in that
  suite (vite serves the module under an `/@fs` prefix that `fileURLToPath` keeps verbatim) — the same trap this lane's
  law works around with a `sourcePath()` helper.
- The serve on `:6021` boots slowly after host-edit bursts; the probes needed `SEMIO_PROBE_BOOT_SECONDS=90…110`.
  The serve was **not** restarted.
