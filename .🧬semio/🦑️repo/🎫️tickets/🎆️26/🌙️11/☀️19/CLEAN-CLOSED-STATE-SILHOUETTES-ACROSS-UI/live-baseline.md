# 🧪 Live Baseline: State Silhouettes Before the Change

The baseline used the elements story site (`site/`, `elements-live.tsx`, `elements-restart.sh` on port 5452) with the pane shown at 919×564. The probe was `silhouette-probe.js`, in its final form, run through `silhouette.sweep()` over every story with at most 30 interactive targets each.

For each target the sweep:
- dispatches a synthetic `pointerover`, which drives the compositor hover; CSS `:hover` is not simulated;
- then calls `focus({ focusVisible: true })`;
- and checks the closure invariant from `design.md` on the painted overlay paths.

It also checks for state-dependent CSS borders, outlines and rings on the control and on its box.

## Result

126 stories with 382 targets produced **483 failing checks**.

| Issue | Count |
|---|---|
| Hover or focus painted on ancestor outlines | 186 |
| Hover or focus painted on ancestors of an unframed control | 26 |
| Side of the stated box left open (left, right, top, bottom) | 122 / 130 / 82 / 68 |
| Framed control without a stated outline | 107 |
| Browser focus outline instead of a seam (`1px auto`) | 113 |
| Other CSS outlines (2px / 1px solid) | 6 / 1 |
| CSS ring on focus | 15 |
| State-dependent partial CSS border (a cell divider recoloured on one or two sides) | 50 |

The worst stories are:
- Tree/ControlTreeStory: 60 failures out of 30 targets
- Icons/IconAnimations: 30/30, all from the browser focus outline
- LayeredOverview/ReducedMotion and LayeredOverview/Strip: 18/9 each
- Ribbon/RibbonLevels: 16/8
- VirtualFileSystem/Expanded: 14/14
- Compose/Groups: 13/10
- Ribbon/Default, Ribbon/MultipleZones and Ribbon/WithUndoRedo: 12/6 each
- ToggleGroup/Multiple: 12/6
- Layout/Mobile: 12/6
- ToggleGroup/Default: 10/5

Typical examples:
- **ToggleGroup:** the first item's right side is open, the group outline takes the hover, and the other items have no outline.
- **Tree filter input:** the bottom is open and three ancestor segments turn hover.
- **LayeredOverview "Open":** all four sides are partly open.
