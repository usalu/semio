# Puzzle2d / UI theme desync

## Symptom

puzzle2d canvas and window options (select/combobox portals) sometimes render light tokens while the rest of the shell is dark.

## Cause

Multiple nested `applyElementsSurfaceChrome` / `useElementsSurfaceChrome` subscribers (e.g. `PlatformShell` + `PlatformView`). Each cleanup unconditionally removed `html.dark`, so a child effect re-run or unmount briefly (or persistently if order was wrong) dropped dark while another lease was still active. Theme also applied in `useEffect` (after paint) so first puzzle2d CSS probes could read light `:root` tokens.

## Fix

- Reference-counted lease stack in `@semio-tech/ui-react` surface chrome; DOM reflects only the top lease; releasing re-applies the previous lease.
- `useLayoutEffect` for hook subscribers so `dark` is set before canvas layout/probes.
- `color-scheme` on `html`/`body` for native controls in portals.
- puzzle2d CSS probe mirrors `dark` class when present.

## Play follow-up (dark → light flash)

- Async play boot left `html` without `.dark` until React; releasing the last chrome lease cleared `.dark` before remount (Strict Mode / shell refresh).
- `bootstrapElementsSurfaceChromeDocument` + inline `index.html` script + sync call in `puzzle/2d/play/index.ts` and `mountPlaygroundApp`.
- Deferred `requestAnimationFrame` clear when the last lease pops so remounts in the same frame keep dark.
- System `matchMedia` listeners stay installed for the page lifetime (not torn down with leases).
- `--color-background: var(--base)` so `bg-background` tracks semantic theme; play `body` uses `bg-base`.
