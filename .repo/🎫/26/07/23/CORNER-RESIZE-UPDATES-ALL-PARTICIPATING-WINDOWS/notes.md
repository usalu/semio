# Corner resize — all participating windows

## Bug
CAD 2×2 (row of two columns) shared center corner: dragging only resized one cross axis (e.g. right column heights). Left column heights stayed wrong. Live preview therefore only moved two of four windows.

## Cause
`ResizableJoinCornerSpec` / `onJoinCornerResize` / `applyModeJoinCornerResize` only touched `spec.crossAxisPath`. At a `+` junction both perpendicular children share the corner and must receive the same cross-axis delta.

## Fix
`resolveJoinCornerPeerCrossAxes(layout, spec)` walks the main-axis separator’s prev/next children and returns every perpendicular join with the same `alongFraction` (including the dragged axis). Resize applies main delta once and cross delta to all peers (live `setLayout` + persisted layout).
