# Report V1: Vegetation (agave, apple-tree, bamboo-clump, date-palm, english-oak, hornbeam-hedge, hosta-clump, ivy-on-trellis)

## State
- All eight assets have front and plan views at every authored scale; hosta 1:500 stays unavailable. `presentation.viewpoints` is removed.
- `checkAssetDetailed` passes for all eight, and every scale is suitable under `office-laser`. It ran with physics off and 5 samples.
- The rest-pose consistency script passes for all eight.
- Not yet run: the three-views vitest and `consistency.ts`.
- **Bug fixed:** apple, oak, bamboo, ivy and palm used `scale >= 100` where `scale <= 100` was meant.

## Method
- **Coordinates:** front is lateral × height; plan is forward × lateral. All values are rest-pose model metres with the rig root at y = 0.
- **Scales:**
  - 1:500 is one or two silhouettes.
  - 1:200 shows the main masses.
  - 1:100 adds articulation and detail strokes.
  - 1:50 adds bark, leaf arcs, midribs and branch hints.
- **Bindings:**
  - Front shapes bind to the bone at the same height, using `followTip`.
  - Plan shapes bind to the foliage or leaf bones, with two-segment `followTip` capsules.
  - Regions and hatches are copied from the elevation.
- **Solid placements are not used.** They would turn the handcrafted fronts into mirrored side views or boxes. Forward sway therefore reads only in plan; the front reads droop, lift and sag.

## Per asset
- **agave:** 11 spiral leaves, foreshortened in front; plan has knee-and-tip capsules.
- **apple-tree:** 7 foliage masses and lobes, scaffolds and leader; fruit from 1:100.
- **english-oak:** 8 masses, bole with root flare, limbs and branches.
- **bamboo-clump:** 6 culms with node ticks and 6 leaf sprays; plan shows culm rings.
- **date-palm:** 8 fronds plus 7 short ones, with chevron bases; plan is a frond star.
- **hornbeam-hedge:** the front is the end view; plan shows six clipped tops.
- **hosta-clump:** 9 spiral blades over petioles (tight tolerance, 0.026 m).
- **ivy-on-trellis:** the front is an end section; plan is a strip with posts, rail and leaf bands.

## Doubts
1. Agave in drought and frost: in plan the leaves lean instead of shrinking; in front, the drought-infolded tip looks boxy.
2. The hedge plan's core rectangles at 1:100 and 1:50 read a little technical.
3. Several previews are unseen: frost-sag, gust and other clips, the 1:200 and 1:500 sheets, ivy, and the hosta collapse. QA3 will check them.
