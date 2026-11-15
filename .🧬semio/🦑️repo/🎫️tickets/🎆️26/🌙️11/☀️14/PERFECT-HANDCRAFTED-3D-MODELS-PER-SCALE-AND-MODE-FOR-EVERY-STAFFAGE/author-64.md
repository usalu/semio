# Author 64: copied scales, template decisions, small defects

Scope: the two catalog-wide defects of `audit-authored.md` and its small defects. Sources were worked on only when `T/volumes/<category>/<slug>.json` had not been touched for 20 minutes (busy assets were done later by `author-64-wait.sh`). Scripts (all in `T`): `author-64-decisions.py` (decision composer), `author-64-notes.py` (hand-written notes), `author-64-walls.py`, `author-64-furniture.py`, `author-64-append.py`, `author-64-fix-regions.py`, `author-64-blind.py`, `author-64-dropbody.py`, `author-64-splice.sh`, `author-64-wait.sh`. Output only in `🗑️generated/author-64/`.

## Result in numbers (audit re-run at the end)

| defect | before | after |
|---|---|---|
| template clusters of `decisions` | 35 clusters, 212 entries, 28 assets | 4 clusters, 26 entries, 8 assets; 6 of the 8 are assets of other authors (elderly-with-cane, jogger, kneeling-gardener, leaning-person, person-with-umbrella, skateboarder: "Figure kit adoption" text); the last cluster is person-cooking against standing-phone-user at the thermal entry of 1:50, 1:100, 1:200 (same clothed figure, see section 2) |
| undefined or missing thermal `region` | 17 asset-scales (guitarist, pig, red-fox, red-squirrel, sheep) | 0 (audit section 6 is empty) |
| pure-copy adjacent pairs (audit heuristic, section 3b) | 10 | 10 by the heuristic, each judged below; one real remodel (venetian blind 1:500); `volumes-copied` is green at every scale of every asset I touched |
| asset-scales with fewer than 2 member solids | 19 | 19 by count, each justified in `decisions` (one rig bone with one section) |

## 1. Scales that are copies

The audit's "near copy" only means that numbers like thickness or radius changed. I compared every pair with the 2D drawings of the two scales (shape ids and geometry per view) and with the 3D lists.

| pair | what it is | decision |
|---|---|---|
| laptop 50 to 100 | not a copy: nine solids (two base boxes, feet, hinge shoulder, keys, track pad, display, hinge, lid) merge into two slabs exactly as the 2D goes from 12 elevation shapes to 4 | unchanged, the 1:100 `decisions` already say what was merged |
| folding-deck-chair 200 to 500 | genuinely minimal: the 2D draws the same true-size 45 mm outlines (C13), only the canvas stroke is forced to 75 mm | already has "Minimal:" with the reason, unchanged |
| height-adjustable-desk 200 to 500 | minimal by C13 (same filled true-size outlines in 2D), 17 to 14 solids (two brackets and the monitor neck gone, monitor one block) | added a "Minimal:" entry naming what the 1:500 drawing leaves out |
| etics-brick-wall-12 and -30, 100 to 200 | not copies: the 20 mm render is under the 30 mm feature and merges into the insulation (three slabs to two), as the 2D goes from 3 to 2 elevation shapes | decisions written by hand (section 2); "Minimal:" names the field boundary the 1:200 drawing keeps |
| solid-brick-wall-1960 100 to 200 | not a copy: 380 mm masonry plus render to 370 plus a 30 mm render skin, as drawn | existing "Minimal:" kept |
| external-venetian-blind 200 to 500 | a real copy: the 1:500 drawing strokes only every fourth lamella (0, 4, 8, 12, 16, bottom rail; 2D shapes 24 to 12) but the 3D had all 17 as 75 mm bars | remodelled: body 24 to 12 solids (five lamella bars); thermal keeps all 17 lamellae (5 by reference, 12 as thermal-only plates on their own bones) so the lamella area 3.67 m2 stays. `--product --scale 500`: PASS. Render sheet `🗑️generated/volumes-tools/render/components/external-venetian-blind.s500.png` opened: overlays green apart from the lamella ends, the body reads as a lowered blind, thermal shows the 17 lamellae in the wall cut-away |
| glulam-beam 100 to 200, 200 to 500 | a prism on two bearings: the 2D changes only the plates (a line instead of a slab) and the skin thickness; at 1:500 it is one rectangle | the "Minimal:" entries now carry the reason; check PASS |
| wheelchair-ramp 200 to 500 | the slab wedge is the same outline; the rails change (1:500 pen); kerbs and posts are gone (12 to 4 solids) | added a "Minimal:" entry with the reason |

Concentration list (copy-share ranking), compared per pair with the 2D shape ids:

- diesel-generator: genuine abstraction at every step (louvre slats to four panels to nothing; castings, doors, edge plates, fuel fill, eyes drop out; 31, 27, 22, 12 solids). The "identical" solids are the container boxes, hood plates and silencer, whose outlines do not change. No change.
- excavator 200 to 500 (22 to 18, 15 identical): I tried the remodel the 1:500 2D asks for (drop the six hydraulic cylinder parts, which the 1:500 drawing does not draw). `volumes-structure-coverage` then fails (`outsideBodyM:plan=0.2625 > 0.075`): the cylinders' member solids lie outside the plan silhouette of the remaining body, and the cylinders are load-bearing bones that need a member. I restored the committed catalog values (`--scale 500`: PASS as before). A real fix needs a rule or 2D decision (the plan drawing would have to draw the cylinders, or the members fold into the boom): left as is, the existing decision already says why the cylinders stay.
- tower-crane 100 to 200 (111 to 102 solids, 55 identical): the 2D goes from 71 to 62 elevation shapes (-9) and the 3D from 111 to 102 (-9), so the 3D follows the 2D; 200 to 500 (102 to 60) follows 62 to 30 shapes. No case where the 3D keeps what the 2D drops; not remodelled.
- cafe-chair, tram, forklift, retractable-bollard, charcoal-grill and the rest of the ranking: the identical solids are rigid slender tubes and boxes at true size (C13, C24) and the counts fall with the 2D; cafe-chair 500 has "Minimal:". Not changed.
- Not touched (still edited by their authors or unspliced): tilt-window, tram, vintage-car-1929, the vegetation, human and vehicle assets still on the old `volume`.

Tool note: the product's `copiedRule` (`volumes-copied`) does not read the "Minimal:" declaration; only `volumes-monotonic` does. Not needed now (no scale I touched is identical to a finer one); written to `tool-requests.md`.

## 2. Template decisions

Method. `author-64-decisions.py` writes, from the actual solid lists and the asset's own rig and thermal data, one English and one German entry for body, thermal and structure at each authored scale: counts per solid kind, how many rig bones the body stands for and which (bilingual bone titles), what is gone or new against the next finer scale (by ids and bones), average loft sections, per thermal region the solid count and declared area, which thermal solids are references and which have own geometry (emitters named), the outer layer of the largest regions, which body solids each region reuses, member, joint, mass and support counts, thinnest and thickest member, total rig mass and heaviest bones, plus the asset's own `summary`, `structure.assumptions[0]` and `thermal.assumptions[0]` in both languages. On top sits a hand-written note per asset (what the solids stand for and what merges first, English and German: `author-64-notes.py`, for 1:50 and for the coarser scales, per scale where I checked the ids). "Minimal:" and "[props]" entries are kept. Geometry untouched.

Honest limit: the lists and sentence frames are composed, not literary; the German is composed from the asset's own German titles plus hand-written notes and is grammatical, with "(each side)" and "(beidseitig)" replacing side adjectives. The vegetation notes run in plain mode (ids only), because bones are re-assigned between scales.

Assets rewritten (all authored scales) and re-spliced with `volumes-insert.py` after the rewrite (`author-64-splice.sh`):

- animals: pig, sheep, red-fox, red-squirrel, horse, dairy-cow, labrador-dog
- humans: standing-adult, pregnant-person, runner-with-blade-prosthesis, parent-with-stroller, person-cooking, person-with-rollator, person-with-white-cane (it joined the clusters after the audit), standing-phone-user, walking-commuter, sleeping-person
- vegetation: apple-tree, date-palm, bamboo-clump, hosta-clump, agave, english-oak
- components: etics-brick-wall-12 and -30 (by hand, `author-64-walls.py`: layers, resistances 3.43 and 8.57 m2K/W, 120 and 300 mm insulation, 719 and 730 kg, why one member is right; every "Minimal:" has its own reason), solid-brick-wall-1960 (1:100 and 1:500 body entries)
- furniture: single-bed, picnic-table, operating-table (the line "Thermal and structure follow the 1:100 body." replaced by sentences naming the regions, members, joints, supports and masses of each)

Several assets were rebuilt by other authors while I worked (horse, dairy-cow, labrador-dog returned with the old template text; standing-adult, pregnant-person, parent-with-stroller were re-templated with "Figure kit adoption" text). I rewrote their decisions again from their then-current solids, so the text describes the source as at the last write. If an author regenerates decisions again, `python3 author-64-decisions.py <asset> --force-age` then `author-64-splice.sh <asset>` reapplies mine.

Remaining similarity: person-cooking and standing-phone-user share their thermal entry at 1:50, 1:100 and 1:200 by the audit's measure (not exact), because it is the same clothed figure with the same ten regions and the same thermal basis. The six "Figure kit adoption" assets are other authors' and not in my list.

## 3. Small defects

- seated-guitarist: the undefined thermal region `face-skin` of the two breath emitters is `face` (8 solids over four scales). pig, red-fox, red-squirrel, sheep: the one regionless thermal solid is the back plume emitter; it now carries the region its emitter declares (`skin-trunk`, `coat-trunk`, `fur-back`, `fleece-trunk`; 4, 3, 2, 4 solids). Emitters are excluded from the exposed area, so no area changes. `author-64-fix-regions.py`.
- date-palm 1:100: the thermal list had exactly one real solid (the transpiration disc), identical to 1:50, so `volumes-copied` failed; its half-thickness is now 9 mm, the pattern of the other scales (1.2 x feature). `volumes-copied` green.
- fewer than two member solids (19 asset-scales): ceiling-diffuser 50, 100, 200; etics-brick-wall-12 and -30 at 100, 200, 500; glulam-beam 50 to 500; panel-radiator 200; solid-brick-wall-1960 100, 200, 500; steel-column 200, 500. Each is right for the object (one rig bone with one section, a masonry leaf that is the only load-bearing layer, a merged shaft) and each `decisions` now says so ("One member solid is right: ...", via `author-64-append.py` and in the hand-written wall texts). No geometry changed.

## Checks (`volumes-check.ts <asset> --product --quiet` after the last change; output `🗑️generated/author-64/checkall.out`)

My edits changed `decisions`, regions and the blind and date-palm geometry only. PASS: glulam-beam, red-fox. The failures that remain are outside this task and were present before: area deviations in thermal-coverage on small regions (etics-brick-wall-12 and -30 at 1:50, steel-column 50 and 100, panel-radiator 50 to 200, ceiling-diffuser 50 to 200, single-bed, operating-table, height-adjustable-desk, external-venetian-blind at 1:50, standing-adult, pregnant-person, runner-with-blade-prosthesis, pig, sheep, red-squirrel, seated-guitarist); other rules: wheelchair-ramp `volumes-cross-scale` 1:500, picnic-table `volumes-cross-scale` 1:100, red-squirrel `volumes-structure-coverage` 1:100, seated-guitarist `volumes-views-disagree` 1:50 and 1:100, person-with-white-cane `volumes-silhouette` 1:200 and 1:500. Noticed and not fixed (other authors' assets): apple-tree `volumes-bones` (6 bones without a solid at 1:50 and 1:100, 2 at 1:200), bamboo-clump 1:500 duplicate thermal id `th-crown-a-0` and a triangle count that grows at 1:200, date-palm thermal area.
