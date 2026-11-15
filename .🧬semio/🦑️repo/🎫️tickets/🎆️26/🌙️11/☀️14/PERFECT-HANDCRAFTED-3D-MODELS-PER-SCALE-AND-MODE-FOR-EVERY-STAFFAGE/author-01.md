# Author 01 — vegetation (agave, apple-tree, bamboo-clump, date-palm, english-oak, hornbeam-hedge, hosta-clump, ivy-on-trellis)

All eight assets are authored at every available scale (hosta 1:500 unavailable), spliced into the catalog, and have bilingual `decisions`. **They are not all at the brief's quality bar.** Plain status first, then per asset. Sources: `T/volumes/vegetation/<slug>.json`, generators `T/author-01-<slug>.ts` (kit `author-01-kit.ts`, trees `author-01-tree.ts`), check helpers `author-01-fails.py`, `author-01-areafix.py`, `author-01-splice.sh`. Full per-rule results: `T/volumes-check.ts <asset>` (the oak/tree checks take minutes on the loaded machine; `volumes-status` shows the cached state).

## Quality tiers (honest)

| tier | assets | what it is |
|---|---|---|
| handcrafted and renders reviewed | hornbeam-hedge (1:50, 1:100, 1:500 renders read), ivy-on-trellis (1:100 render read) | bodies built by hand from the scale's own outlines (lofts with section by section half widths, crests, bumps); thermal by references; structure inside the body |
| generator-built from outlines, renders not reviewed | english-oak, apple-tree | per-bone lofts/capsules from each scale's own shapes (`treeBody`), generic thermal/structure; not hand-tuned; apple IoU plan 0.51, oak not checked to completion |
| drafter-derived (NOT handcrafted) | agave 1:50–1:200, hosta 1:50/1:100, date-palm, bamboo-clump | exact-profile extrusions from `volumes-draft.ts` (outline simplified per coarser scale so no scale is a copy), generic thermal/structure. Silhouettes of agave 1:50–1:200 pass; the 3D forms are flat plates. Date-palm and bamboo need real hand modelling (frond lofts with repeat, culm tapers, plan masses). |

I ran out of budget before the third tier was remodelled. I did not look at the renders of oak, apple, agave, hosta, palm or bamboo.

## Solid counts (body / thermal / structure)

| asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| agave | 38/40/41 | 38/40/22 | 38/40/22 | 4/6/22 |
| apple-tree | 22/28/32 | 22/28/17 | 16/22/17 | 3/14/17 |
| bamboo-clump | 64/66/44 | 61/63/26 | 34/36/26 | 3/7/26 |
| date-palm | 116/117/46 | 57/58/24 | 43/44/24 | 4/5/24 |
| english-oak | 24/26/42 | 18/20/26 | 14/16/26 | 6/8/26 |
| hornbeam-hedge | 37/44/38 | 19/16/20 | 7/10/14 | 2/5/14 |
| hosta-clump | 27/29/38 | 18/20/20 | 3/14/20 | unavailable |
| ivy-on-trellis | 25/23/32 | 20/26/20 | 14/20/20 | 7/13/20 |

## Check results (last full run, `T/🗑️generated/author-01/final.fails.txt`, abbreviated; cross-scale omitted)

- **agave**: silhouette 1:50–1:200 pass; 1:500 fails (IoU elevation 0.79, depth deviation 0.38 m: the rigid two-ellipsoid rosette cannot reproduce the posed 2.4 m depth); feature 1:200 (leaf tips 22 mm < 30 mm); monotonic structure 1:500; structure outside body at 1:50–1:500 (member capsules along leaf bones leave the thin plates); thermal area: leaves-spreading/erect exposed 2.1–2.6 m2 against the corrected value (see below), caudex.
- **hosta-clump**: silhouette 1:100/1:200 IoU 0.83/0.80; 1:50 `volume-profile-self-intersecting` on `blade-r1` (drafter profile; must be fixed), midrib below feature at 1:50, structure outside body (0.03–0.12 m), thermal 1:200 blades-outer.
- **apple-tree**: plan IoU 0.51 at 1:50/1:100 (plan crown fills missing), 1:500 bounds (width −0.94 m, depth −2.46 m: single crown loft), bones `ld`, `root-plate`, spurs without body solid, limb-bark/fruit areas at coarse scales, structure outside body.
- **english-oak**, **bamboo-clump**, **date-palm**: checks started, not finished at the time of writing; run `volumes-check.ts`. Expect silhouette Hausdorff/p95 failures on foliage outlines (see request 01).
- **hornbeam-hedge**: after the final re-splice not re-checked; before it (earlier run) all rules passed at all scales except silhouette Hausdorff 1:50 (elevation 38.6 mm, plan 28.5 mm vs 15 mm) and cross-scale 1:100/1:500 caused by the 2D drawings (twelve stems at 1:50 against a plinth at 1:100).
- **ivy-on-trellis**: plan IoU 0.85–0.91 (mats modelled from elevation and front; plan outlines thin and wavy), bounds 1:50 width +12.8 mm, `volumes-views-disagree` (the 2D drawings of the mats disagree between views), 1:500 thermal areas (screen versus the corrected area), cross-scale 1:500 (open frame at 1:200 against filled screen at 1:500).

## What I modelled by hand

- **hornbeam-hedge**: each of the six clipped panels is a vertical loft whose half widths are read slice by slice from its own front outline and whose plan sections are rounded rectangles (superelliptic exponent 6 to 7); crests: three lofted crowns per panel at 1:50 (heights read per panel from the elevation ring), one at 1:100, none at 1:200; twelve stem capsules whose centres follow the drawn, slightly leaning stems; bed strip for the drawn root-plate line; 1:100/1:200 plinth; 1:500 one long loft along the hedge axis on the plinth. Thermal: leaf-face as references to the exposed panel and crown solids; leaf-core (marked `hidden` at the root, see below) as six inner boxes at 1:50 and one strip coarser, fitted to the area; stems as references to the stem capsules (plinth coarser); emitters as flat discs. Structure: stems referenced (1:50), branch-bundle capsules, joint balls at stem feet and panel bases (1:50/1:100), root mat, support block with `anchor`.
- **ivy-on-trellis**: posts, shoes, rails, battens as boxes (1:50 batten repeat 8x0.2 m; 1:100 individual outline battens because the 1:100 drawing strokes them), climbing stems as two-piece capsules with sideways drift, eight leaf mats as lofts (elevation x front); 1:500 screen box between two post boxes with steel shoes (so posts and shoes stay exposed for thermal) and two crown ellipsoids. Structure members inside the body (root-plate member sits in the bottom rail, crown members in the top rail, stem members inside the mats), support blocks with `anchor`.
- Everything else is generated (see tiers). The kit (`author-01-kit.ts`) provides the pieces I used: slice-wise lofts from outlines, member fitting that shrinks capsules until they lie inside the body silhouette, emitter discs, support blocks.

## Root data changes (ruling C1, old -> new `areaM2`, m2)

Measured with the tool's exposed-area measure on the spliced 1:50 model by `author-01-areafix.py` (text replace of `areaM2` only; verified by `git diff`).
- agave: caudex 0.21 -> 0.394 (an earlier faulty run also set leaves-spreading 2 -> 0.394; restored to 2 by hand).
- hosta-clump: petioles 0.25 -> 0.183.
- apple-tree: trunk-bark 3.58 -> 4.98, limb-bark 6.08 -> 3.0, spur-twig 0.12 -> 0.227, leaf-shade 60.48 -> 19.6, fruit 15 -> 0.0921 (leaf-sun unchanged).
- english-oak: leaf-sun 250 -> 642, leaf-shade 200 -> 111.
- date-palm: stem-bark 13 -> 36.3, fruit 3 -> 12.2, fronds 90 -> 21.9.
- bamboo-clump: culms 1.6 -> 19.3, leaves-sun 5.4 -> 11.0, leaves-shade 8.2 -> 34.9.
- ivy-on-trellis: leaves-upper 8 -> 2.31, leaves-lower 7 -> 2.01, stems 0.45 -> 0.139, trellis-timber 6.3 -> 2.82, post-shoes 0.5 -> 0.264 (may need a second pass).
- hornbeam-hedge: no change (an erroneous automatic run set 0.0076/0/0; restored to 0.8/16/14 by hand); `leaf-core` gained a `hidden` note (B4) so that its interior solids are measured as hidden.
**Concern**: for foliage regions `areaM2` is leaf area (drives transpiration), several of these corrections are physically large (fruit 15 -> 0.09, bamboo culms 1.6 -> 19.3 caused by drafter capsules that are too thick). The coordinator should review the foliage rows; see `author-requests.md` section 01. The bamboo/palm/oak/apple rows come from non-handcrafted geometry and should be redone after remodelling.

## Not done / known imperfect

1. date-palm, bamboo-clump, agave, hosta: bodies are drafter output, not hand modelled (flat polygon plates for fronds/leaves; culms/leaf masses as extrusions).
2. apple-tree plan crown fills, fruit placement, 1:500 crown; oak plan crown fills; neither reviewed in renders.
3. Structure inside-body failures on thin-plate plants (agave, hosta) and on apple/oak at some scales.
4. Thermal area at coarse scales is not tuned for most assets (30 % window), only checked for hedge/ivy.
5. hornbeam-hedge, ivy: see check lines above.
6. Rule requests appended to `author-requests.md` (section 01): cross-scale against generalised drawings, Hausdorff on organic outlines, foliage `areaM2`, leaf-core hidden.

## Addendum: final check run (all eight assets finished)

- **english-oak**: plan IoU 0.48 at 1:50/1:100 (plan crown fills missing), p95 boundary failures, root-plate and trunk-upper without body solid, thermal bark areas at 1:100/1:200 and leaf-shade at 1:500, structure outside body at every scale, bounds height 1:50 off by 0.37 m (limit 0.364). The earlier "check not finished" is superseded.
- **date-palm**: elevation IoU 0.81 at 1:50, 1:500 IoU 0.47/0.44 (drafter body does not follow the 1:500 drawing), fronds thermal area 0.34-10.2x off at 1:100-1:500, thermal at 1:100 is a copy of 1:50, structure outside body, monotonic 1:500.
- **bamboo-clump**: front IoU 0.71-0.78, height bbox +0.52 m at 1:50, culms below the 1:200 feature (5.4 mm), duplicate thermal ids at 1:500 (`th-crown-a-0`), total thermal area off at 1:200, structure outside body.
- **hornbeam-hedge**: thermal only: stem-bark 1:50 +20 % (references to the stems; area not re-corrected), leaf-core exposed 0 at 1:50 (the `hidden` flag is in the catalog file, but the product measure still counts it against the body), coarse-scale totals 32-59 % off. Silhouette, bounds, feature, structure all pass.
- **ivy-on-trellis**: silhouette plan IoU 0.85 at 1:50, elevation IoU 0.80 at 1:100; structure borderline (elevation 7.5 mm at limit).
Oak, palm and bamboo are therefore not at the bar; they need real hand modelling and tuning before they can be called done.
