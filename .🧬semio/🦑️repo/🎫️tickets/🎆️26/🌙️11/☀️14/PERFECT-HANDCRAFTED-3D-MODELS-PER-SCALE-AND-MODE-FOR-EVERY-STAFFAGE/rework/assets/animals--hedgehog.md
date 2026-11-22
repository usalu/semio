# Rework — `animals/hedgehog`

Earlier work on this asset: `T/r3-animals-hedgehog.md` (rounds 3 to 5: two-bone legs, flexing back, coat bowls, body, 2D) and `T/rework/08.md` section 4 ("reviewed, not changed"). After round 5 another pass replaced the tuft cones by "fascicle" polyhedra (four spines on a root patch) at all four scales; that pass left no report.

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published and `VALID`, but not everything was rebuilt and not every sheet was read after the last change (see "Remaining").

Script (the chisel, repeatable): `T/rework-hedgehog-girdle.py p1|p2|build|publish`; input `T/rework-hedgehog-before.json` (rig and volumes before this pass, taken from the git index, which held the state I found). It uses `r3-hedgehog-io.py` to write the `rig` member only and `volumes-insert.py` for the volumes.

### State found

- Catalog `volumes` and `T/volumes/animals/hedgehog.json` were identical, so there was no half-finished edit to reconcile.
- The catalog asset was **INVALID**: 272 `volumes-feature` findings at 1:100 (268 fascicles 9 mm thin under the 15 mm feature, eyes 7.6 mm, ears 7.8 mm). Cause: rule change 3 of `validator.md` (every scale judged on its own); the 1:100 coat was the 1:50 fascicle with fewer copies.
- With the measured rules more was wrong, none of it visible in the earlier "VALID": 1:200 and 1:500 still carried 212 and 148 fascicles of 1:50 size; the sternum touched no rib at any scale; the orbicularis was four separate rings.

### Reference (Erinaceus europaeus, adult, 1 kg; from my own knowledge, not from a cited osteometric table)

| measure | real | model before | model now |
|---|---|---|---|
| head and body / over spines | 0.20–0.27 / up to 0.28 m | 0.274 m | unchanged |
| height to spine tips, width over spines | 0.12–0.14 / 0.13–0.16 m | 0.123 / 0.146 m | unchanged |
| scapula (vertebral border to glenoid) | 32–35 mm | no bone; four 0.7 mm rods on the chest bone, flat at z 26–30 mm | bone `scapula-n/-f` 33.4 mm |
| vertebral border beside the midline / shoulder joint beside the midline | about 15–20 / 26–30 mm | 26 / 28 mm (blade parallel to the sagittal plane) | 20 / 28 mm (blade leans in over the rib cage, 13.8° abduction) |
| clavicle | present in Erinaceidae, about 20–25 mm | missing | 33 mm rod acromion to sternum (too long, see Remaining) |
| hip bone, total / ilium from sacroiliac joint to acetabulum | 45–50 / 22–28 mm | one pipe of 64 mm on the loin bone | ilium bone `ilium-n/-f` 29.4 mm, crest 10 mm beyond; ischium 20 mm; pubis to the symphysis |
| sacroiliac joint / hip joint beside the midline | about 8–10 / 22–25 mm | 12 / 24 mm | 9 / 24 mm |
| sacrum | 3 vertebrae, about 20 mm, between the ilia, cranial of the hip joints | 33 mm, wholly behind the hip joints | 28 mm, between the ilia, ending over the hip joints |
| thoracic inlet half width / widest rib half width | about 10 / 35–40 mm | rib 1: 22 mm, rib 3: 33 mm (shoulder joint inside the rib hoop) | rib 1: 11 mm, rib 3: 24 mm; widest unchanged |
| ribs to sternum | 8 true ribs with costal cartilage | no rib reached the sternum | every chest rib runs on to the sternum |
| humerus / radius-ulna / fore foot | 38–43 / 36–48 / 25–30 mm | 38 / 36 / 28 mm | unchanged |
| femur / tibia / hind foot | 40–43 / 44–46 / 40–45 mm | 40 / 44 / 40 mm | unchanged |
| joint order | scapula–humerus–radius/ulna–carpus/manus; ilium–femur–tibia–tarsus/pes, plantigrade | humerus and femur hung directly on chest and loin | scapula–humerus–forearm–paw; ilium–femur–shank–paw |
| orbicularis | one sphincter round the edge of the spiny skin, held by frontodorsalis and caudodorsalis | four rings of 42 mm radius, one per coat bowl | one oval ring in four quadrants plus the two pairs of bands |

### Rig

- New bones on lateral rest placement, solved with `rig-rest.ts --solve` (numbers in the script, `GIRDLE` and `LIMBS`): `scapula-n/-f` (parent `chest`, `angleDeg` -39.75, `abductionDeg` ∓13.85, limits ±15) and `ilium-n/-f` (parent `trunk`, `angleDeg` -142.12, `abductionDeg` ∓30.72, limits ±3). Their joint axis is the normal of the blade, so `angleDeg` swings the scapula in its own plane.
- `fore-n/-f` re-parented to the scapulae and `hind-n/-f` to the ilia without moving (`--solve --parent`); rest frames checked with `rig-rest.ts` on the candidate: equal to four decimals. Poses, clips, anchors, 2D shapes, thermal regions and limb members therefore keep their meaning; no pose moves the new bones.
- Joint limits were ±170 on every limb joint and ±110 on neck and head. Now: chest -75..15, neck -90..35, head -50..30, shoulder -40..60, elbow -45..70, wrist -45..160, hip -55..45, knee -65..40, ankle -50..25. The script asserts that every pose lies inside. The wrist limit of 160 only exists because `rolled` folds the fore paw by 158°, which no wrist does.
- Both sides are mirror images (z, abduction and twist negated). New bones carry no mass.

### Structure

- 1:50: scapula as a thin plate (`sheet`, 0.8 mm) with spine and acromion, clavicle, ilium, ischium, pubis (meeting at the symphysis), sacrum restated between the ilia, lumbar column shortened to it, tail vertebrae prolonged to it, cranial ribs narrowed, costal cartilages, one orbicularis ring, frontodorsalis bands on the head bone and caudodorsalis bands on the loin bone. The four wire-triangle rods per scapula and the hip pipes are gone.
- 1:100, 1:200, 1:500: one slender rod per scapula and per ilium on the new bones, the same rib and ring changes, pelvis block kept (sacrum, ischium and pubis are under the feature there).

### Body and thermal

- 1:50: not changed.
- 1:100: 170 tuft cones of 15.6 mm base instead of 268 fascicles (two of every three per ring, each keeps root, lean and tip of the fascicle it replaces); eyes and ears 9.2 mm. Thermal references follow.
- 1:200 and 1:500: fascicles removed, the dome halves enlarged by 6 mm to the envelope of the spine tips (29 and 25 body solids). Thermal references follow; no region of its own was remodelled.

### 2D, decisions

- 2D not redrawn. The silhouette rules pass at 1:50 and 1:100 with the existing drawings (1:100 front failed at 0.876 with half the tufts, passes with two thirds). 1:200 and 1:500 representations are `unavailable` as before.
- `decisions`: first entry rewritten at 1:100, 1:200, 1:500 (coat); one girdle entry appended at every scale (en, de). The older second entries still mention "pelvis block under the sacral rod", "the spine of each scapula" and "the orbicularis ring of each bowl"; they were not rewritten.

### Validation

`VALID animals/hedgehog (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`r3-validate.ts animals/hedgehog --catalog-only --volumes`, after the last publish). Catalog `volumes` equal the source.

### Sheets read

Before: 1:50 body and structure (default pose). After my changes: 1:50 structure (six views, top and right close-ups, `rolled`), 1:50 body `forage-walk` t 0.3, 1:100 body, 1:200 body, 1:500 structure. The last change (dorsalis bands moved to head and loin) was read at 1:50 structure in the default pose (right) and `rolled`.

Rendered but not read: 1:50 body `rolled`, `alert`, `roll-up` t 1.1; 1:50 structure `forage-walk` t 0.5; 1:500 body; 1:200 body `rolled`; 1:100 and 1:200 structure. Not rendered after the changes: any thermal sheet, the 2D drawings, a second instant of each clip.

### Remaining (next pass)

1. Rig, both sides: elbow and knee still carry a lateral step (`forearm` z ∓6 mm, `shank` z ∓12 mm) instead of an abducted humerus (about 9°) and femur (about 17°). Needs the limbs' front and plan shapes shifted and the walk keys re-checked for foot plant.
2. Rig: one mid-back joint instead of a flexing column; no carpus/tarsus or toe bones (paw is one bone each); neck one bone.
3. 1:50 structure, `rolled` pose: the four ring quadrants are rigid on the coat bowls and stick out of the ball as rods; the purse-string needs its own treatment (ring bones or a ring per pose).
4. 1:50 structure: skull is still a plain wedge with a slab mandible (no braincase, orbit, teeth row); column is three 10 mm pipes without vertebrae or spinous processes; rib 11–15 unchanged; clavicle 33 mm against about 22 mm because the sternum starts 24 mm ahead of the shoulder joint (sternum too long cranially); ilium stands 56° to the horizontal because the hip joint lies 28 mm under the column.
5. 1:200 and 1:500 body: brow of the dome is a blunt wall; the coat bowls show as ragged skirts under the dome at front and rear; `rolled` not judged.
6. 1:100 body: tufts are cones of 15.6 mm, a coarse reading of a spine coat; head still one cone with jaw.
7. Thermal, all scales: not looked at in this pass; at 1:200 and 1:500 the spine regions now rest on the dome and bowl surfaces alone.
8. 2D at 1:50 and 1:100 not re-read; `decisions` second entries at the coarse scales describe the old skeleton.
9. `rolled` pose: wrist at 158°.
10. `r3-hedgehog-rig.ts`, `r5-hedgehog-model.ts` and the fascicle pass (script unknown) still describe the old rig and lists; running them would undo this pass.

### Fundamental issues (outside this asset)

- `volumes-feature` in `validateAsset` flagged the sub-feature fascicles only at 1:100; the identical solids at 1:200 and 1:500 (features 30 and 75 mm) raised nothing there. It looks as if the analytic rule runs only where a 2D representation is authored; `volumes-monotonic` was the only rule that caught those lists.
- `volumes-connected` reports a member that passes within one feature of the rest without touching, but not a member that is farther away: the sternum floated free of every rib at all four scales and passed.
- `volumes-insert.py` runs its main at import (no `__main__` guard), so it cannot be used as a module for its `dump`.
- `rig-lateral-compare.ts` threw at line 11 when given the round-3 baseline file and a candidate; I did not look into it.
