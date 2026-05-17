# Investigation log — board canvas not interactive

## Symptom

Board canvases paint; chrome (windows, panels) works; **primary pointer** (click / select / deselect) does not reach the board.

## What we tried (chronological)

1. **Git 440→441 diff** — Found `ContextMenuController` rewrite with `document.addEventListener("pointerdown", …, true)` (capture). Hypothesis: dismiss ran before board surface listeners. **Later commits** addressed ordering (bubble / `window` + board `window` capture bridge).

2. **Vitest guard** — `still routes primary pointer down through WASM after the surface context menu opens` in `elements/client/lib/board/index.tsx`. **Passes**; does not cover “always dead” from layout/CSS alone.

3. **Golden Layout + `Window` options (441)** — Board panes gained `options` (Redraw zoom). `Window` renders `[data-slot="window-options-overlay"]` as **`absolute inset-0`** with Tailwind `pointer-events-none`, intended pass-through except the right rail (`pointer-events-auto`).

4. **`elements.css` rule** — `.lm_goldenlayout * { pointer-events: auto !important; }` **overrides** Tailwind `pointer-events-none` on that overlay (same specificity + `!important`; bundle order can matter). Full-bleed div can eat hits even when Tailwind says `none`.

5. **First CSS patch** — `[data-slot="window-options-overlay"] { pointer-events: none !important; }` after `.lm_goldenlayout *`. Playwright asserted computed `pointer-events === "none"` on three overlays — **passed** here; user still reported failure → **geometry** could still block, or other hosts/cache.

6. **Structural (this pass)** — Removed full-bleed: overlay is **`absolute top-0 right-0 bottom-0 left-auto z-panel flex w-max max-w-[min(11rem,calc(100%-0.5rem))] …`** so the shell **does not span the canvas**; even with Golden Layout forcing `pointer-events: auto`, the node is only on the **right rail** width.

7. **CSS belt-and-suspenders** — `.lm_goldenlayout [data-slot="window-options-overlay"]` **and** `[data-slot="window-options-overlay"]` with `pointer-events: none !important` (beats `.lm_goldenlayout *` on specificity for the first).

8. **Playwright `elementFromPoint` (rejected)** — Top hit was often a **Golden Layout `DIV`**, not `CANVAS`, and not always inside `[data-testid="board-event-surface"]` — too strict.

9. **Playwright `elementsFromPoint` stack** — Assert **first canvas** appears in `document.elementsFromPoint(x,y)` **before** any node under `[data-slot="window-options-overlay"]` (stacking / overlay order).

## Files touched (latest)

- `elements/client/lib/react/index.tsx` — `Window` options overlay layout.
- `elements/client/lib/styling/elements.css` — Golden Layout–scoped + global `pointer-events` override for `window-options-overlay`.
- `elements/client/lib/board/play/e2e/board-play-gpu.spec.ts` — overlay computed style + stack tests.

## If it still fails on your machine

- DevTools → pick point on dead canvas → **which element** is top (`$0` in console). Note `data-slot`, `class`, computed `pointer-events`, ancestors.
- Confirm `elements.css` is loaded (search for `window-options-overlay` rule in **Sources** / **Network**).
- Temporarily comment `.lm_goldenlayout *` in `elements.css` to confirm Golden Layout is the layer (expect interaction to return if that rule is the only cause).
