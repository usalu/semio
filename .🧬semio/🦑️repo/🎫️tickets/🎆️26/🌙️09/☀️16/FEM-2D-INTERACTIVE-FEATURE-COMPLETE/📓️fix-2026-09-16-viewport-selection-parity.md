# 🖱️ Fem 2D viewport selection parity (2026-09-16)

Ticket: `FEM-2D-INTERACTIVE-FEATURE-COMPLETE` follow-up.

## Summary

Fem2d now exposes the same canvas selection utilities as draw/note: **Direct**, **Marquee (rectangle)**, **Lasso**, plus **Pan**. The shell’s selection method toggle (`rectangle` / `lasso`) and merge modes (`selective` / `additive` / `subtractive` / `invertive` via shift/ctrl/meta) apply to viewport picks the same way as in other apps.

## Behaviour

| Utility | Gesture |
|---|---|
| `selectDirect` | Primary click picks immediately (unchanged semantics) |
| `selectMarquee` | Drag rectangle; commit on release; `method: rectangle` |
| `selectLasso` | Drag freeform path; commit on release; `method: lasso` |
| `transformMove` | Host pan (middle button still pans globally) |

Marquee hit-testing covers viewport-visible entities: nodes, supports, loads, members, regions. Overlay layers draw during an active drag on both Model and Results windows.

## Files

- `🕹️interaction/🖱️canvas-gesture/🦀️.rs` — gesture state, marquee geometry, pointer routing
- `🕹️interaction/🦀️.rs` — `SelectionMethod::Lasso`, `interactionSelect` carries `method`
- `🎮️commands/🖱️canvas-pointer-{down,move,up}/🦀️.rs` — delegate to canvas gesture
- `✏️editor/🦀️.rs` — utility registry on both canvas window kinds
- `🧱️model/🦀️.rs`, `📊️results/🦀️.rs` — marquee overlay layers

## Verification

```bash
cargo nextest run -p semio-s-artifact-fem-2d --features component-app-assembly -E 'test(/interaction::/) or test(/canvas_pointer/) or test(/canvas_gesture::tests::/)'
```
