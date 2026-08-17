# UI Button Cycle Zero-Consumer Acceptance

## Lease Outcome

The zero-consumer ButtonCycle surface is removed. Button and ButtonProps remain the sole Button module contracts, and the React registrar keeps only their existing public mapping.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🟦️component.tsx | 2399db1122f0ac8871cabead437d57b3a0b51e24fb57f0e9b4c6c389675ae3a7 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🧪️story.tsx | 2bb18afa0e144ac929b2814e955a3b78252df33d77899adc66d6168bc2d64eaa |
| Registrar-owned update | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 8494f5da41ac9bcde40169278f6ad9a2749167b72ceef703b2eb31a6f606c906 |

Removed from the Button source contract: ButtonCycleItem, ButtonCycleProps, ButtonCycle, their exports, and the exclusive ButtonCycle story region. The Button directory remains because Button is retained.

## Static Acceptance

- An active UI scan found zero ButtonCycle, ButtonCycleItem, or ButtonCycleProps references; ticket history was excluded.
- The React barrel maps only Button and ButtonProps from the Button component.
- The retained Button implementation is unchanged in behavior and has three production module consumers: UIDialog, IconSelector, and Tree.
- Scoped ordinary and cached git diff --check commands completed cleanly for Button and the React target.

## Nx Validation

Each target was run exactly once in uncached Nx form with --skip-nx-cache.

| Command | Outcome |
| --- | --- |
| bun nx run @semio-tech/ui-react:lint --skip-nx-cache | Passed. |
| bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache | Failed on broad existing workspace and UI type errors. No ButtonCycle diagnostic was emitted. |
| bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache | Ran 520 tests: 510 passed, 10 existing failures, and 2 existing jsdom unhandled errors. No failure references ButtonCycle. |
| bun nx run @semio-tech/ui-react:build --skip-nx-cache | Failed independently because Storybook cannot resolve @semio-tech/coda-desktop/renderer from .storybook/stories/ui/✅ValidationTree.stories.tsx. |

## Blockers

No ButtonCycle removal blocker remains. The non-passing gates are blocked by unrelated workspace and UI type errors, existing quick-test failures in Scene, icon CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and jsdom event handling, plus the unresolved Storybook Coda renderer dependency.
