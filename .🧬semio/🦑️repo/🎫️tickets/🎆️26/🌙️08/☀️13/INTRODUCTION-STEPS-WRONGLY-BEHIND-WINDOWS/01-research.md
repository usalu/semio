# Research: Introduction Steps Wrongly Behind Windows

## Problem Description
When `UIIntroduction` runs inside `ShellHost` (or any shell equipped with `ShellScope`), the introduction steps (info box card, gesture overlay) render behind target windows that have been stamped with `data-introduction-elevated="true"`.

## Root Cause Analysis
1. `ShellHost` mounts `data-semio-portal-layer` as a sibling before `FrameworkOsShellInner`:
   ```tsx
   <div data-semio-portal-layer ref={setPortalLayer} className="pointer-events-none absolute inset-0 z-tutorial" />
   <FrameworkOsShellInner {...innerProps} locks={locks} brand={brand} />
   ```
2. The `className="pointer-events-none absolute inset-0 z-tutorial"` on `data-semio-portal-layer` creates a new CSS stacking context at level `10000` (`var(--z-tutorial)`).
3. `UIIntroduction` portals its overlay elements into `data-semio-portal-layer`. The step info box has `style={{ zIndex: "calc(var(--z-tutorial) + 2)" }}` (10002 inside `data-semio-portal-layer`'s stacking context).
4. `FrameworkOsShellInner` comes after `data-semio-portal-layer` in DOM order. When `useIntroductionElevation` runs, it stamps `data-introduction-elevated="true"` on the target window or dock stack inside `FrameworkOsShellInner`.
5. In `ui.css`, `[data-introduction-elevated]` assigns `z-index: calc(var(--z-tutorial) + 1) !important;` (10001). Because `FrameworkOsShellInner` is not trapped in a lower z-index stacking context, the elevated window gets `z-index: 10001` in the root `semio-scope` stacking context.
6. In `semio-scope`'s root stacking context:
   - `data-semio-portal-layer` container is at `z-index: 10000`.
   - Elevated target window is at `z-index: 10001`.
   - Since 10001 > 10000, the elevated window paints on top of `data-semio-portal-layer`, causing the `UIIntroduction` info box card (which is trapped inside `data-semio-portal-layer` at 10000) to paint **behind** the elevated target window.

## Proposed Solution
1. Remove `z-tutorial` from `data-semio-portal-layer`'s container in `ShellHost/🟦️component.tsx` (and/or render `data-semio-portal-layer` after `FrameworkOsShellInner`) so `data-semio-portal-layer` does not create a z-index 10000 stacking context ceiling for its portaled children.
2. Portaled children inside `data-semio-portal-layer` (veil at 10000, step info box at 10002, demo overlay at 10002, popovers at 10003) will then participate directly in the `semio-scope` root stacking context alongside the elevated window (10001).
3. The resulting z-index hierarchy will be:
   - Radix popovers / select content inside elevated elements: `10003` (`var(--z-tutorial) + 3`)
   - `UIIntroduction` info box & demo overlay: `10002` (`var(--z-tutorial) + 2`)
   - Elevated window / target element: `10001` (`var(--z-tutorial) + 1`)
   - `UIIntroduction` veil: `10000` (`var(--z-tutorial)`)
4. Verify with automated tests in `🧰️framework` to ensure the elevated window is above the veil (10001 > 10000) and the introduction step info box is above the elevated window (10002 > 10001).
