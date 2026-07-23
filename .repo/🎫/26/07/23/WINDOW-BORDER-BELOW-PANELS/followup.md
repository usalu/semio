# Follow-up: Screenshot still shows rectangular body border

## Observed
- Intro/active teal border cuts under "Top" tab and under Fokussieren/Schließen — body rectangle, not silhouette.
- Gap between tab and controls is bare canvas (grid shows through); glass must frost the cutout too.

## Root causes
1. **React:** Piecewise chrome borders (tab/gap/controls/body) still painted the horizontal body-top line; gap used `bg-canvas` instead of glass; silhouette SVG only for introduced/loading/waiting (and null without metrics).
2. **wgpu:** `render_stack` still painted cap/gap/controls piecewise borders plus body L/R/bottom — forming a rectangle under the tabs — while full-cap glass already covered the gap.

## Fix
### React / CSS
- Cap / gap / controls → `ui-glass-ribbon`, `border-0` (gap frosts the cutout).
- Body → fill only (`border-0 bg-canvas`).
- `ModeDockStackSilhouetteBorder` always on (`introduced|loading|waiting|active|normal`); pending placeholder when metrics unavailable.
- CSS `!important` zero borders on chrome slots; hover emphasizes silhouette stroke.
- Inactive tabs no longer get `border-b-active-base` U-frame baseline.

### wgpu
- Removed piecewise stack chrome/body borders in `render_stack`.
- Single `push_window_silhouette_border` after body fill.
- Full-cap glass already frosts the cutout (unchanged).

## Verified
- ui-react Mode/silhouette/intro vitest: 12 passed.
