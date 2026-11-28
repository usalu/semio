# 🧊️ 3D selection, gumball and matrix cascade in the live browser

Scope: 3D matrix rows `3d.sel.*`, `3d.util.*`, `3d.gumball.*` and the wider cascade (rows 46-112: `3d.del.*`, `3d.hist.*`, `3d.ckp.*`, `3d.focus.entity`, `3d.edit.*`). Own headless Chromium on http://127.0.0.1:6087/ (published wasm 06:11), runs written to `🗑️generated/sg/run{1..4}` (never the coordinator's `matrix-v3/3d`; `run.mjs` got `--out <dir>` for this).

## Verdict

One product defect (framework, TS, live in vite) and several harness defects. No Rust change, so nothing waits for a wasm publish. The 3D selection and the gumball work in the live build; the failures were the harness reading the wrong tree and measuring too early.

## Root cause of the cascade (rows 46-112)

1. Harness: `inspTree()` was `getByRole("tree").last()`. Any other tree later in the DOM (the Actions palette after `hist.palette-undo-redo`, the Command/Language popover after `shell.command-locale`) was read as "Inspection". That is the `inspection="ACTIONS | Add Node… | Add Bar…"` text, `no summary counts`, every `edit.*` "field missing". Fix: scope to `[id="framework.panelTab.framework.panel.inspection"]`.
2. Product: `PanelTabBar` set `aria-pressed={isActive}`. A folded panel's active tab is still `isActive`, so Inspection, History and Results all reported `aria-pressed="true"` while only one (or none) was mounted (checked in the DOM: the `framework.panelTab.framework.panel.inspection` node was absent while its button said pressed; clicking it opened it, a second click closed it, pressed stayed true). The harness (`isPressed`) and a screen reader both got a false state, so `ensureInsp`/`panel()` clicked blind and desynchronised (History open, Inspection folded: `inspection=""`, locator timeouts in `hist.buttons`, `ckp.switch-alternative`, `focus.entity`). Fix: `aria-pressed={isActive && showActiveColor}` (what the component's own docstring and `data-state="on"` already say). Law: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/aria-pressed.test.tsx` (2 tests, green; the folded case fails on the old code). Status: live in vite, verified live (run2/run4).
3. Harness: `panel()`/`inspOnly()` toggled and waited a fixed 600-1200 ms, but the UI settles in 1-6 s under load, so the next click re-toggled. Now polls `aria-pressed` until it matches (`settled()`).

## Per failure

| Check | Cause | Fix | Status |
|---|---|---|---|
| `sel.click-replace` timeout 120 s | harness: wrong tree (Command popover left open by `shell.command-locale`), `pickAt` returned instantly for B because a selection already existed | scoped `inspTree`; `pickAt(spec, {differs:true})` waits for a changed id; per-attempt window 16 s | PASS (run2) |
| `sel.shift-additive/ctrl-subtractive/shift-ctrl-invertive/empty-click` | harness: `toSummary` clicked empty after only 600 ms hover and waited 15 s, selection round trip is up to 7 s under load | hover 1.8 s, wait 30 s | PASS (run2) |
| `sel.marquee`, `sel.marquee-tiny` "no Marquee Select" | by design: the 3D host marquee is Shift (or Ctrl/Meta) + left drag, a plain left drag orbits (`world3dLeftPressOrbits`, `viewport-drag.md`); fem3d declares only the Transform utility. Not a defect | 3D branch drives Shift+drag; tiny = Shift + 2 px | PASS (44 selected, `N SELECTED`) |
| `sel.lasso`, `util.pan` | no such 3D utility by design (pan = right/middle drag, covered by `cam.*`) | stay SKIP | n/a |
| `util.transform-gumball` `pick=false options=[]` | harness: `cam.*`/`wheel-zoom` ran first and left the camera moved, calibrated pixels stale | check is `fresh` (reload, recalibrate) | PASS |
| Option checkboxes missing / `no gumball handle` (translate, plane, rotate, scale) | harness: (a) the Transform options panel sits at the bottom-left of the Model window, under the docked Artifact panel when it is open (real mouse cannot reach it); (b) the image diff was dominated by the options panel and taken 0.9 s after the toggle while the gumball redraw takes 1-5 s; (c) re-arming Transform toggled it off again; (d) earlier drags moved the entity under the calibrated pixel | `optionsLayout()` closes Artifact for gumball work (`freeLayout`, reset per check), `gumClip` x 0-800 / y 60-780 excludes the panel, `changed()`+`stable()` polling, `armed` test before `util("Transform")`, all `gumball.*` drag checks `fresh` | translate, plane, rotate-small, rotate-large, scale-axis, scale-uniform and all 5 option checks PASS (run2, run4); `scale-uniform` no longer times out |
| `del.key-tree`, `del.backspace-tree` | harness: a focused tree row button owns Delete/Backspace by design (`controlOwnsKeyboardEvent`), so the global `removeSelection` hotkey does not fire | blur focus and hover the Model before the key | PASS (run4) |
| `hist.buttons` | harness: fixed 3 s waits plus an empty history in isolation | poll counts, Undo/Redo after an edit | PASS (run4) |
| `hist.command-log` 0 rows | wrong tree/panel state (same cascade) | cascade fix | PASS (run3) |
| `focus.entity`, `edit.node-x`, `edit.element-kind` | cascade | cascade fix | PASS (run2) |

## Open, not fixed here

- Product UX (no change): Transform options are hidden behind the Artifact panel when it is open.
- Selection round trip is 3-7 s with several agents loading the machine (1.1-1.5 s earlier, `first-click-pick.md`); Rust publication, needs profiling after a publish.
- Still failing in run2 and not triaged: `del.inspection-support/loadNodal/section/material` (Delete on referenced entities, may be intentional refusal), `del.palette-remove-selection` (70 s, no effect), `del.combination-term-zero`, `ckp.switch-alternative` (locator timeout), `inspect.solid-readonly` (state leak from an earlier edit), `shell.fullscreen` (button hidden behind the Command popover).

## Files

Harness: `matrix-v3/lib.mjs`, `checks-core.mjs`, `apps.mjs` (`gumClip`), `run.mjs` (`--out`, `freeLayout` reset), helpers `sg-driver.mjs`, `sg-probe.mjs`, `sg-vitest.config.mjs`. Product: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx` and `aria-pressed.test.tsx`.
