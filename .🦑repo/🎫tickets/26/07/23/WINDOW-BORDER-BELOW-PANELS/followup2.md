# Follow-up 2: Glass seam + intro pulse still body-rect

## Observed
1. Glass cutout shows a visible seam (tab | gap | controls are separate `backdrop-filter` regions).
2. Introduction pulse still only the inside body rectangle.

## Root causes
1. **Seam:** Cap, gap, and controls each had their own `ui-glass-ribbon` → adjacent backdrop-filter stacking contexts create seams.
2. **Intro rect:** Aggregator `introduce: windowElementId("puzzle3d-main")` stamps `data-introduced` on `ChromeAwareWindowScrollSurface` (`id="framework.window.puzzle3dMain"`) — the *inner* scroll surface — not `[data-slot="window"]`.
   - CSS suppress only targeted `[data-slot="window"][data-introduced]`, so the scroll surface kept the inset box-shadow pulse.
   - Silhouette kind looked only at the window element's own attribute, so it never became `introduced`.

## Fix direction
- One glass layer on the whole tabbar; cap/gap/controls transparent.
- Suppress any `[data-introduced]` inside `mode-dock-stack`; resolve silhouette kind from introduced descendants too.
