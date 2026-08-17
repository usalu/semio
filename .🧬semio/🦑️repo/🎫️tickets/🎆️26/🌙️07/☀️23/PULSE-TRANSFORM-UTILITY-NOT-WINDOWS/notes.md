# Pulse Transform Utility Not Windows

## Bug
Step 7 (`transform-utility`) pulsed the Top/Perspective window silhouettes instead of the Transform utility toggle.

## Root cause
1. `resolveWindowSilhouetteBorderKind` / ModeDock silhouette treated *any* `[data-introduced]` descendant as a window introduce — including `#transform`.
2. CSS blanked *all* introduced stamps inside `mode-dock-stack`, so the utility could not show its own inset pulse.

## Fix
- `isWindowChromeIntroducedTarget` — only `data-slot=window` or bare `framework.window.{segment}` (not `.action.*`).
- Silhouette kind uses that filter.
- CSS suppresses rectangular rings only for window-chrome stamps; nested utilities keep `introduced-border-pulse`.
