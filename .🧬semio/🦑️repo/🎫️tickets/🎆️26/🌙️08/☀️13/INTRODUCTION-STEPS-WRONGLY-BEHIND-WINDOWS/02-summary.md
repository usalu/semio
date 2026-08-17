# Summary: Fix Introduction Steps Rendering Behind Windows

## Result Summary
The issue where introduction steps (info box card, gesture overlays) rendered behind target windows has been resolved.

## Root Cause
`ShellHost` mounted `data-semio-portal-layer` with `z-tutorial` (`10000`) before `FrameworkOsShellInner`. This created a `10000` CSS stacking context ceiling for all portaled children. When target windows were elevated (`[data-introduction-elevated]`, `10001`), they painted above the `10000` portal layer, placing the portaled `10002` step info box card behind the elevated windows.

## Resolution
1. Removed `z-tutorial` from `data-semio-portal-layer`'s container in `ShellHost/🟦️component.tsx` and moved the portal layer after `FrameworkOsShellInner`.
2. Portaled elements now participate directly in `semio-scope`'s root stacking context:
   - Fullscreen veil: `z-index: 10000`
   - Elevated target window: `z-index: 10001`
   - Step info box card & gesture overlay: `z-index: 10002`
   - Floating popovers: `z-index: 10003`
3. Added unit test to `🧪️index.test.ts` verifying the container is unconstrained by `z-tutorial`.

## Files Updated
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
