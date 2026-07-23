# Window Border Effect Follows Full Silhouette

## Problem

Introduction / loading / waiting border rings painted on the rectangular window body (`[data-slot="window"]` / `window_content_rects`), so Aggregator step 4 (and any loading/waiting window) showed a rectangle inside the chrome instead of the full dock silhouette:

```
┌───────────┐                    ┌─────┐
│  tabs     │      gap cutout    │ btns│
│           └────────────────────┘     │
│              body                    │
└──────────────────────────────────────┘
```

## Fix

### React / CSS
- Introduction elevation prefers `[data-slot="mode-dock-stack"]` so tabs + gap + controls + body rise together.
- `ModeDockStackSilhouetteBorder` SVG overlays the stack outline for introduced / loading / waiting.
- Helpers: `windowSilhouettePath`, `measureWindowSilhouetteMetrics`, `resolveWindowSilhouetteBorderKind`.
- CSS suppresses rectangular body rings inside a dock stack; silhouette stroke animations match introduced / loading / waiting timing.

### wgpu
- `WindowSilhouette` + `push_window_silhouette_border` paint the notched outline.
- Dock collect stores silhouettes beside content rects; tour pulse uses silhouette path for windows; cutout/anchor rects use silhouette bounds.

## Verified
- ui-react vitest: silhouette path/kind + dock-stack elevation/SVG (intro suite 7 passed)
- cargo test window_silhouette_border (pending compile in this session)
