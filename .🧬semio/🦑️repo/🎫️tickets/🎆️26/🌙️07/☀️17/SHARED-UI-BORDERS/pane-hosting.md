# Pane Hosting Geometry

## Root Cause

`WindowPane` calls `anchorPositionStyle` then `chromePanelSafeAreaStyle` without an explicit base. The latter overwrites both physical anchor edges with `var(--spacing-single)` (3.2px), even with no occupied shell panel. This directly explains Utilities at x19.1875 rather than x16, bottom412.8125 rather than416. It also retains panel-oriented double-inset width/height clamps.

## Scoped Fix

Use a zero-inset positioning policy for contained Pane only, preserving Panel defaults and collision displacement. Explicit zero bases in the existing safe-area API are sufficient; no WindowChrome/shared runtime API or CSS change is needed. Keep content/body padding and full silhouette geometry untouched. Cover actual Window collapsed and expanded utilities/actions/measures and all eight Pane anchors with browser geometry tests, including RTL.

## Changed Files

Source:
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📌️WindowPane/🟦️.tsx`: zero anchor inset/safe-area base and host-width clamps, retaining collision displacement, padding, silhouette/chips/glass, resize/drag and event handling.
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/⚓️panel-anchor/🟦️.ts`: parameterized inset with unchanged Panel default and matched width/height clamps.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/🧩️component/🟦️.tsx`: added actual Window-with-Pane integration; Decimal numeric oracle. Existing concurrent focus-indicator hunk preserved, not authored here.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/🧫️fixtures/pane-edges.json`: language-neutral physical-edge/fold/direction vectors shared by component and browser tests.

Ticket inputs/reports: `pane.config.ts`, `pane-browser.tsx`, `check-pane-browser.mjs`, this report. Window, Tree toggle, CSS, WindowChrome, WindowSilhouette and Canvas sources were inspected as needed but not modified.

## Validation Commands

Launcher: `/home/clstb/.local/share/semio/launcher/bin/repo`.

- `SEMIO_UI_TEST_CONFIG='<ticket>/pane.config.ts' <repo> tools exec --light node -- '<ticket>/run-ui-tests.mjs'` — 13 passed.
- `<repo> tools exec --light node -- '<ticket>/check-pane-browser.mjs'` — 32 Chromium scenarios passed, using live 6011 and actual Mode/Window/Pane composition.
- `ASSERT_APP_BORDERS=1 <repo> tools exec --light node -- '<ticket>/inspect-story.mjs'` — actual App physical attachment/activation/hover checks passed; raw paths expose separately owned compositor overlap.
- Scoped `git diff` inspected and `git diff --check` passed through repo launcher; no modifying Git commands.

`<ticket>` is `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS`. Owner requested generated evidence preservation for runtime integration: `🗑️generated/pane-browser.json`, `🗑️generated/app-story.json`, `🗑️generated/app-story.png`. Existing ticket/run infrastructure reused; no installs/configuration/service changes.

## Validation Results

- Red: actual Window integration fails on Utilities' default spacing-single inset.
- Green: all 13 Window component tests pass after zero-base Pane placement.
- Chromium: 32 actual Mode/Window geometry scenarios pass (eight contributed anchors × two folds × LTR/RTL). Checks actual WindowChrome silhouette bounds, not just overlays, against Window body edges.
- Real App 6011: Utilities now x16/bottom416 and x600/bottom416; unchanged 70.34375×22.390625 chip dimensions, zero page errors. `ASSERT_APP_BORDERS=1 inspect-story.mjs` passes.

## Compositor Handoff (Resolved by Lead)

Although physical bounds now coincide, App output still contains normal-state utility bottom paths on top of the parent active bottom path. Parent active bottom is local y369.234375 + root y45.765625 =415; utility normal bottom is local y21.390625 + root y393.609375 =415. They are the same painted centerline, but both remain in `app-story.json`. Left parent active path spans x17..600; nested normal bottom spans x17..85.34375. The Utility left edge IS deduplicated, while the bottom is not. Runtime also emits tiny x0.002455 junction offsets. This is fractional size normalization/ownership, outside Pane placement ownership: likely registry scale using integer offsetHeight while measureWindowSilhouetteMetrics treats <=1px differences as unscaled. No runtime source was edited. Lead explicitly accepted placement and took ownership of fractional runtime normalization. Lead subsequently fixed the mismatch with one shared measurement/registration scale policy. The actual App paint-path acceptance now rejects duplicate normal bottom strokes and passes; see fractional-followup.md.
