# Report V2: Vegetation (japanese-maple, lombardy-poplar, london-plane-pollarded, miscanthus-grass, multi-stem-birch, scots-pine, small-leaved-lime, weeping-willow)

## Scope
- 32 representations (8 assets × 4 scales) now have front and plan views.
- `presentation.viewpoints` is removed.
- The decisions texts are extended in en and de.
- The plan symbols are untouched.
- The scripts are in `T/🧰️V2/` (one `spec_<asset>.py` per asset, with hand-placed masses).

## Method
- **Crowns:** composed of hand-placed leaf masses. Front uses (lateral, height); plan uses (forward, lateral). The tiers are rotated against the side view, so the front is not a mirror of it.
- **Occlusion:** front masses are ordered by depth; in plan, higher masses sit over lower ones.
- **Detail per scale:** 1:50 shows every mass plus inner arcs and limb lines; 1:100 drops the diagonal masses; 1:200 shows the main masses only; 1:500 has one silhouette per view.
- **Bindings:**
  - Trunks and limbs in the front view are `poly` with `followTip`.
  - Willow curtains and miscanthus blades use `followTip` in both front and plan.
- **Wind sway** runs along the elevation x axis. It therefore shows in plan as an x shift and in front only as a change in height. No `solid` placement is used (accepted).

## Checks
- Three-views guard (vitest filtered to these assets): 16/16 pass. Earlier lateral and miscanthus failures are fixed.
- Check CLI: all eight assets are suitable under `office-laser` at all four scales. The minimum stroke is 0.10 mm and the minimum feature 0.38 mm.
- Previews: only the rest-pose composites from the Python previewer were inspected. Poses and clips are covered by the guard sweep alone.

## Doubts
- Clip instants and non-default poses have not been inspected by eye.
- The front and plan crown outlines are smoother than the scalloped elevation.
- The tests ran through bun and vitest directly (the fallback in `ts-status.md`).
