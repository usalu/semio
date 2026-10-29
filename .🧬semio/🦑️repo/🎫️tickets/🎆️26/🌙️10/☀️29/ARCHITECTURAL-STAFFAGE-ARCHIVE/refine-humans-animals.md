# Refinement pass: humans (12) and animals (6)

Files edited (catalog JSON directly, NOT the generator sources): `S/🗂️catalog/👤️humans/🔣️.json`, `S/🗂️catalog/🐕️animals/🔣️.json`.
A rebuild from `T/🧰️scripts/humans-a`, `humans-b`, `animals/*` would revert this pass. Scripts of this pass (all in `T/🧰️scripts/`, run in this order, one-shot, not idempotent unless noted):
`refine-ha-humans-provenance.py`, `refine-ha-humans.py rig|angles`, `refine-ha-toe-angles.ts`, `refine-ha-clean-keys.py`, `refine-ha-clip-toes.ts`, `refine-ha-humans.py clipkeys`;
animals: `refine-ha-animals-lib.py` (line-preserving editor for the hand-formatted file), `refine-ha-animals-{dog,cat,horse,pigeon,roe,cow,seams,seams2}.py`, `refine-ha-lift-swing.ts`, `refine-ha-pose-opt.ts`; probes `refine-ha-ground.ts` (lowest point of every pose and 40 clip instants), `refine-ha-review.ts` (review sheets), `refine-ha-anchor-bottoms.py`.
Check: `repo tools exec bun -- ...🩺️check... --file` prints a check mark for all 12 humans and all 6 animals.

## Cross-cutting
- Provenance (audit-provenance.md): Annex B -> Annex A (ISO 7730, all humans); cyclist `iso-8090-2007` -> `iso-8098-2014` (citation rewritten, EN 14764 named); wheelchair `cooper-2008` -> `cooper-2014`; jogger `cavanagh-1982` -> `cavanagh-1980`, `mcmahon-1986` replaced by a correct `dalleau-1998` entry (about 5.1 m/s, values authored), umbrella `rain-wetting` removed and its value re-tagged authored-estimate; running-child Jensen fractions (15) -> authored-estimate, citation and summary say "guided by Jensen (1986)"; animals: `hoy-zernicke-1985` title corrected to "Modulation of limb dynamics in the swing phase of locomotion" (DOI 10.1016/0021-9290(85)90044-2 confirmed by web search; volume/pages left out), `gebremedhin-2001` pages removed (not confirmable by search; all values already authored-estimate and now stated as guidance only), `hutchinson-1969` removed (not confirmable) with its three values and the Hutchinson/Brown text dropped, horse `halter-webbing` re-tagged authored-estimate, dog summary "footfall order after Hildebrand (1976)" (the dog's own reference; the 1965 paper is the horse paper). No "per brief"/"to be verified" strings exist in these two files.
- Hands (all 12 humans, 24 hands): new silhouette with four finger tips and a separate protruding thumb shape (`role: body`, lod 2, so it appears from 1:100 and finer), three finger-separation lines (lod 2); wheelchair user keeps bare fingertips as skin region `hand-skin` (glove cut at the knuckles).
- Toe-off sink: the six humans-b assets got a forefoot bone `toe-l/-r` (joint 0..70 deg) hinged at the ball contact; shoe path and sole/toe-cap strokes were split exactly (Bezier split) between foot and toe bone. Toe angles of every pose were solved (smallest dorsiflexion that keeps the toe level, clamped to the joint). Three clips got smooth override keys where interpolation sank geometry: seated-reader `stand-up` (toe plus hip-lift raise), worker `squat-lift`, jogger `stretch`.
- Animals: continuous silhouette. Neck/head coat regions merged into the trunk coat (dog, cat, roe `coat-head-neck` -> `coat-trunk`; horse `coat-neck` -> `coat-trunk`; areas and metabolic shares added, insulation/length area-weighted), rump shapes moved onto the lumbar bone where pelvis and thorax were not adjacent (dog hindbody, horse rump, roe rump), horse mane bands, mane strands and trunk strokes moved to lod 3. The depiction hides inner outlines only for the same region on adjacent bones, so the seams at the neck base and across the torso are gone at 1:100 and 1:200 (looked at). Cow and pigeon were left as they are (cow already one trunk path).

## Per asset

### walking-commuter
Defects: mitten hands. Fix: new hands with thumb. After: no ground violation. Limit: thermal region colours plausible (face and hands warmest, coat cooler).
### seated-reader
Defects: stand-up clip: toe tip and foot sinking up to 27 mm. Fix: hands; 13 override keys (toe level, hip-lift raised). Limit: stand-up keys are an interpolation patch, not new IK.
### running-child
Defects: mitten hands, Jensen tagging. Fix: hands, provenance. Limit: flight poses are real flight (ballistic); small 1:50 figure.
### wheelchair-user
Defects: ramp ground line started 84 mm below ground (the ground polyline rotates with the chair); fingerless-glove tips lost. Fix: polyline now starts at the wheel contact (x = -0.02), hands with skin fingertips, `cooper-2014`. Limit: flat poses rely on the depiction ground line left of -0.02.
### elderly-with-cane, cyclist
Hands, cyclist reference renamed. No ground violation.
### worker-carrying-box, parent-with-stroller
Defects: toe-off foot sank 40-57 mm, mittens. Fix: toe bone plus solved toe angles, hands. After: all poses and clips within 5 mm of ground. Limit: shoe seam (rear/toe) is a faint vertical stroke at 1:20.
### jogger
Toe-off sank 37 mm, stretch clip 30 mm. Fix: toe bones, override keys at 1.6-1.9 s and 7.5-7.6 s, provenance (cavanagh-1980, dalleau-1998). Limit: stretch-hamstring rests 4.4 mm below ground (inside tolerance).
### person-with-umbrella
Defects: furled umbrella ferrule 24 mm below ground. Fix: shaft angle -10 -> 12 deg (carried tilted forward), toes. After: ok.
### leaning-person, tour-guide
Toe bones and hands. No other defects.

### labrador-dog
Defects: trunk seams (three overlapping trunk shapes, kinked belly, bulge at the croup), zig-zag fur strokes, paws 13 mm below the contact. Fix: one hind-body path on lumbar plus a chest path on thorax with matching belly line, hind thigh top lowered, fur strokes redrawn, paw contact anchors lowered to the sole (-0.02/-0.0255). Limit: slim, pointer-like build; trunk is two shapes (merged outline), the thorax -10 deg in sniffing moves the chest 4 cm against the legs.
### domestic-cat
Defects: sitting tail lay behind the haunch, below ground (-19 mm); paws 7 mm low in stalk. Fix: tail now runs from the rump down the near flank and forward along the ground around hind and fore paws, tip curled up (descriptions EN/DE updated); paw anchors -0.0145. After: no violation anywhere.
### horse
Defects: boxy hind thigh/gaskin (straight L edges, thigh hidden behind trunk), mane bands and strokes crossing the neck, swing hooves up to 39 mm below ground in the walk. Fix: smooth thigh/gaskin curves, near thigh and gaskin drawn above the trunk (visible stifle/haunch), neck coat merged, mane/strokes lod 3, 18 swing-flexion override keys (carpus/hock flexed until clear), check stays on advanceM 1.25. Limit: hind limb still simple; stance-foot drift of about 3-4 cm in the hand-built walk keys (not touched).
### pigeon
Defects: wings 149 mm and body 21 mm below ground in take-off/peck poses. Fix: lift-off wing posture folded (20/130/-140), pecking legs less flexed, airborne roots raised (0.27, 0.34 m), contact anchors -0.0095. After: all poses within 5 mm. Limit: silhouette is crude (small body, looped legs); not redrawn in this pass.
### roe-deer
Defects: swing/bound hooves 29-40 mm below ground, bounding poses not flagged as dynamic, box seams in the torso. Fix: 18 + 12 + 2 override keys (walk, bound, alarm), bounding-pushoff far hoof and bound-phase-5 hind flexion, rump moved to lumbar, coat merged; every bounding pose (named and key) now ends with "Dynamic snapshot: the centre of mass is outside the support, so the moment is not statically balanced." (German too); bound-cycle `advanceM` 0.6775 -> 0.6484 (planted travel after the flexion keys). Limit: far-fore stance drift about 2-4 cm remains (plant solver vs authored IK keys; a re-solve of the gait keys is needed).
### dairy-cow
Defects: lying udder 173 mm below ground (the 0.56 m wide bag cannot fit above the 0.13 m udder pivot), hindlegs 42 mm and tail 17 mm below ground in lying, pasterns 6 mm below ground. Fix: udder bag narrowed to 42 % of its width (teats moved), tail draped over the flank, hind metatarsus/hoof angles re-solved, hoof contacts lowered to -0.052, `lie-down` got 9 override keys. After: poses standing/grazing/lying within tolerance. Limit: kneeling-front pastern 10.6 mm low (joint limit), lie-down 1.6-3.6 s far fore pastern up to 76 mm below ground (interpolation between standing and kneeling keys; needs new IK keys), udder narrower in standing than before.

## Evidence
Before/after probe outputs: `T/🗑️generated/refine-ha/ground-before-*.txt` (deleted at the end) versus the final probe; the six kept after-sheets are in `T/🗑️generated/refine-ha/`.
