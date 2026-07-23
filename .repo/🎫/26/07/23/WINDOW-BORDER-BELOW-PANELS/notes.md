# Window Border Effect Follows Full Silhouette

## Problem

Introduction / loading / waiting / active borders painted a rectangle under the dock tabs (and gap glass had seams), instead of one continuous silhouette:

```
┌───────────┐                    ┌─────┐
│           │                    │     │
│           └────────────────────┘     │
│                                      │
└──────────────────────────────────────┘
```

## Fix

### React / CSS
- Elevation prefers `[data-slot="mode-dock-stack"]`.
- **One** `ui-glass-ribbon` on `[data-slot="mode-dock-tabbar"]`; cap/gap/controls are transparent (no per-cell backdrop-filter seams).
- Body is fill-only; silhouette SVG always paints the notched outline.
- Intro kind-id stamps the inner scroll surface (`framework.window.{kind}`) — suppress any `[data-introduced]` inside the stack; resolve silhouette `introduced` from descendants.

### wgpu
- Full-cap glass + single `push_window_silhouette_border` (no piecewise body/cap borders).

## Verified
- ui-react Mode/silhouette/intro vitest: 13 passed (includes kind-id-on-scroll-surface intro case).
