# Author 21 — vegetation/japanese-maple, vegetation/lombardy-poplar

Both assets are modelled and checked at all four scales; **neither is spliced** (see "Why not spliced"). Nothing is "not started".

Generators (rerunnable, they merge into the source): `author-21-maple.ts`, `author-21-poplar.ts` on the kit `author-21-lib.ts` (visual hulls with oriented superellipse sections, growth and shrink, world-aligned ellipsoids and boxes, limb lofts with dense end sections). Helpers: `author-21-sum.py <slug> [scales]` (failing measurements of the full product check), `author-21-diag.ts` (structure and thermal outside the body per solid), `author-21-hard.ts`, `author-21-far.ts`, `author-21-blue.ts`, `author-16-ascii.ts` (overlay diagnostics). Sources: `volumes/vegetation/japanese-maple.json`, `volumes/vegetation/lombardy-poplar.json`. author-02 drafts were replaced; backups of the old sources are in `🗑️generated/author-21/*.author02.json`.

## vegetation/japanese-maple

Solids (body/thermal/structure): 1:50 130/142/31, 1:100 71/83/31, 1:200 40/52/21, 1:500 10/22/13. Bilingual `decisions` per scale (4, 2, 2, 2 entries).

Hand-modelled: 18 pads at 1:50 (14 at 1:100, 8 plus 6 tier masses plus 2 plan fills at 1:200, one crown hull at 1:500), each pad a visual hull of its own elevation, front and plan outline, split at 40 % of its height into a sun cap and a shade skin at 1:50 and 1:100 (thermal leaf-sun and leaf-shade); shade cores and top tuft; root collar loft on the drawn flare; three stems and the top stem along the drawn outlines with dense end sections; six tier branches (tapered lofts at 1:50 and 1:100, slender capsules at 1:200 and 1:500 because the load path crosses the gaps between the tiers); members clamped to 78 % of the half width of the body they run in; joints at 1:50 and 1:100; world-aligned mass ellipsoids; support pad.

Last full check (product engine, no `--fast`), failing measurements only:
- 1:50: `p95M:elevation` 24.4 mm (limit 15), `areaDeviation:limb-bark` 0.53 (limit 0.15).
- 1:100: `p95M:elevation` 37.5 mm (limit 30), `areaDeviation:limb-bark` 0.49 (limit 0.30).
- 1:200: `p95M:elevation` 97.5 mm (limit 60).
- 1:500: `p95SoftM:plan` 0.3375 m (limit 0.30), `unionVolumeBandExcess` 0.70.
Silhouette IoU elevation / front / plan: 0.931 / 0.978 / 0.939 (1:50), 0.928 / 0.968 / 0.937 (1:100), 0.906 / 0.923 / 0.917 (1:200), 0.928 / 0.948 / 0.906 (1:500); `p95SoftM` at most 0.0975 m against 0.105 to 0.15 m; bounds, feature, monotonic, bones, copied, views-disagree, structure coverage all pass; cross-scale passes except the 1:500 volume band.

Renders looked at (all four sheets): pads read as layered cushions with a visible cut into sun cap and shade skin, collar and stems below, thin blue branch lines cross the gaps between the tiers in the elevation overlay (real branches, the 2D does not draw them at 1:50 to 1:100), thermal cut-away shows nested cap and skin, structure shows members inside the pads. Imperfect, stated plainly: pad edges are smooth superellipses where the drawing has 30 to 60 mm scallops; the 1:200 elevation overlay still has red at the pad ends of the second tier; the 1:500 crown fills the gaps between the tiers.

Root changes in the asset (also in `author-requests.md`): `stem-bark.areaM2` 3.45 -> 2.3 (factor 1.5, C16); `limb-bark.areaM2` first set to 0.65 then reverted to 1.37 (beyond 1.5, row in `area-review.md`); representation 200 front: four shapes `f-t2-60/120/240/300` added (copied from 1:100; whether this stays under C11 is the coordinator's call, the audit lists it).

## vegetation/lombardy-poplar

Solids: 1:50 131/140/44, 1:100 49/58/32, 1:200 38/47/32, 1:500 11/20/24. Bilingual `decisions` (3, 2, 2, 2).

Hand-modelled: crown lobes (25 at 1:50, 17 at 1:100 and 1:200, one hull at 1:500) as visual hulls with the alternating left and right offsets kept; four trunk pieces with plugs (one bole at 1:100 to 1:500), circular root flare, eight branches along their drawn outlines (slender capsules at 1:500), members along the bole and branches inside the body, nine crown-bone members, joints at 1:50, world-aligned mass ellipsoids, support pad, nine emitter discs of scale-dependent size.

Last full check, failing measurements only:
- 1:50: `p95M:elevation` 24.4 mm (limit 15); `areaDeviation:stem-bark` 0.90 and `branch-bark` 1.00 (limit 0.15).
- 1:100: `p95M:elevation` 48.8 mm (limit 30), `unionVolumeBandExcess` 0.10, stem-bark 0.88, branch-bark 1.00 (limit 0.30).
- 1:200: `p95M:elevation` 97.5 mm (limit 60), `areaDeviationTotal` 0.92 (limit 0.30).
- 1:500: `unionVolumeBandExcess` 0.007, `areaDeviationTotal` 0.88.
IoU 0.958 / 0.988 / 0.947 (1:50), 0.953 / 0.957 / 0.918 (1:100), 0.943 / 0.966 / 0.915 (1:200), 0.953 / 0.934 / 0.942 (1:500).

Renders looked at (all four): a column of tufts at 1:50 to 1:200 (blocky lobe faces at 1:50 from the 250 mm hull levels), one smooth feather-shaped crown at 1:500, members and masses inside; the plan overlay has blue slivers at the lobe tips from the superellipse bounding boxes. Not changed in the asset (all corrections would be beyond the factor 1.5; rows in `area-review.md`).

## Why not spliced

Both fail rules that I could not satisfy without weakening them or editing shared tools: the hard `p95M` of foliage (tool defect, measured and explained in `author-requests.md`), hidden bark (poplar, maple limbs), and cross-scale volume bands where adjacent 2D drawings differ (maple 1:500, poplar 1:100). Splicing would put findings into the product asset, so `volumes-insert.py` was not run for either. Everything else passes at every scale. To splice after the coordinator's rulings: `python3 volumes-insert.py volumes/vegetation/<slug>.json "<catalog asset file>"`.
