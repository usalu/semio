# Investigation log — board canvas not interactive

## Symptom

Board canvases paint; chrome (windows, panels) works; **primary pointer** (click / select / deselect) does not reach the board.

## What we tried (chronological)

1. **Git 440→441 diff** — `ContextMenuController` used `document.addEventListener("pointerdown", …, true)` (capture). Addressed later on mainline (bubble / `window` + board `window` capture bridge).

2. **Vitest guard** — `still routes primary pointer down through WASM after the surface context menu opens` in `elements/client/lib/board/index.tsx`. Passes; not sufficient for “always dead”.

3. **Golden Layout + `Window` options (441)** — Board panes gained `options` (Redraw zoom). `Window` added `[data-slot="window-options-overlay"]` as **`absolute inset-0`** + Tailwind `pointer-events-none` (pass-through) + right rail `pointer-events-auto`.

4. **Identified `.lm_goldenlayout * { pointer-events: auto !important; }`** in `elements/client/lib/styling/elements.css` — Overrides **every** descendant’s `pointer-events`, including intentional `none` on pass-through overlays.

5. **First CSS patch** — `[data-slot="window-options-overlay"] { pointer-events: none !important; }` after `.lm_goldenlayout *`. Playwright: computed `pointer-events === "none"` on three overlays — passed locally; user still “still” → cascade order / other layers / stale build.

6. **Structural `Window` overlay** — Replaced `inset-0` with **`top-0 right-0 bottom-0 left-auto w-max max-w-[min(11rem,calc(100%-0.5rem))]`** so the shell does not geometrically cover the canvas center.

7. **Higher-specificity override (while `*` existed)** — `.lm_goldenlayout [data-slot="window-options-overlay"]` + attribute selector with `pointer-events: none !important`.

8. **Playwright `elementFromPoint` (too strict)** — Top node was often a Golden Layout `DIV`, not `CANVAS`.

9. **Playwright `elementsFromPoint` stack** — Assert canvas appears in the stack before any `window-options-overlay` subtree.

10. **Removed `.lm_goldenlayout *` entirely (definitive)** — The universal selector was the fundamental bug: **no** amount of per-component `!important` is reliable while `*` exists. **Replacement:** explicit `pointer-events: auto !important` only on known interactive Golden Layout chrome (`.lm_dragProxy`, `.lm_popin`, `.lm_header`, `.lm_tabs`, `.lm_tab`, `.lm_controls`, `.lm_splitter`, `.lm_splitter .lm_drag_handle`, `.lm_close`, `.lm_maximise`, `.lm_popout`, `.lm_tabdropdown`, `.lm_tabdropdown_list`). Removed the redundant `[data-slot="window-options-overlay"]` override block after `*` deletion.

## Files touched (cumulative)

- `elements/client/lib/react/index.tsx` — `Window` options overlay layout (narrow strip).
- `elements/client/lib/styling/elements.css` — Delete `.lm_goldenlayout *`; add targeted GL `pointer-events: auto` list + comment.
- `elements/client/lib/board/play/e2e/board-play-gpu.spec.ts` — overlay computed style + `elementsFromPoint` stack tests.

## If it still fails on your machine

- DevTools → dead click → top element (`$0`): `data-slot`, classes, computed `pointer-events`, ancestor chain.
- Confirm built CSS contains **no** `.lm_goldenlayout * { pointer-events: auto` (hard refresh / rebuild).
- If a Golden Layout control is dead, add its class to the explicit list in `elements.css` (do not reintroduce `*`).
