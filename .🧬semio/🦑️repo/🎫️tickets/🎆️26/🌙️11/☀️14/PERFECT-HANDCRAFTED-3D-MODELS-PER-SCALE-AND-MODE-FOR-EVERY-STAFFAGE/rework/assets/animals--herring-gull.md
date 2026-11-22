# animals/herring-gull — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published; `r3-validate.ts animals/herring-gull --catalog-only --volumes` ends `VALID` (schema 0, validateAsset 0, checkVolumes 0). Open items are listed under "Remaining".

Edit scripts (hand-chosen numbers, in `T`): `rework-herring-gull-girdle.py` (rig, poses, girdle, humeral feathers, thin vanes, load paths, decisions), `rework-herring-gull-coarse.py` (1:200 / 1:500 head, neck, humeral blades), `rework-herring-gull-thermal.py` (wing layer).

### State found

- Source `T/volumes/animals/herring-gull.json` and catalog `volumes` were identical (no half-finished edit).
- The earlier batch report (`rework/08.md`, section 5) had reviewed the asset and changed nothing; it called it VALID. Under the rules of 2026-11-22 it was **INVALID**: 98 `volumes-feature` (every primary and secondary vane at 1:100 and 1:200, 2.0 / 2.4 mm thick and undeclared; the three wing membranes at 1:200; the hand bones at 1:100 and 1:200) and 3 `volumes-connected` (scapula 1.6 mm short of the coracoid at 1:50; at 1:200 the wing chain and the spine chain were two separate groups).

### Reference table (adult Larus argentatus, own knowledge) against the model

| Measure | Real | Model before | Model now |
| --- | --- | --- | --- |
| Total length bill tip to tail tip | 0.55–0.67 m | 0.60 m | unchanged |
| Wingspan | 1.25–1.55 m | 1.37–1.45 m in `gliding` | unchanged |
| Mass | 0.7–1.5 kg | 1.0 kg | unchanged (humerus 30 -> 29 g, five humeral feathers 0.2 g each) |
| Humerus | 125–140 mm | 135 | unchanged |
| Ulna | 145–160 mm | 140 | unchanged (5 mm short of the range, kept) |
| Carpometacarpus + major digit | 125–135 mm | 135 | unchanged |
| Coracoid | 50–58 mm | no bone; solid on the trunk, 9 mm inboard of the shoulder joint | bone 57.5 mm, sternal facet to glenoid |
| Breadth across the shoulder joints | 65–75 mm | 72 mm (arm hung beside the trunk bone) | 72 mm, carried by the coracoids |
| Breadth across the hip sockets | 38–46 mm | 44 mm (thigh hung beside the trunk bone) | 44 mm, carried by the pelvis halves |
| Femur / tibiotarsus / tarsometatarsus | 58–62 / 105–115 / 60–70 mm | 58 / 105 / 66 | unchanged |
| Middle toe with claw | 58–65 mm | 58 | unchanged |
| Skull with bill / culmen | 115–125 / 50–60 mm | 125 / 59 | unchanged |
| Neck (15 cervicals) | 120–140 mm | 135 mm in two links | unchanged |
| Primaries / secondaries / humerals | 10 / about 20 / 6–8 | 10 / 12 / none (one fixed panel) | 10 / 12 / 5 on their own bones |
| Flight feather vane | 30–47 mm wide, under 1 mm thick | 2.0–2.4 mm thick at every scale | 1 mm at 1:100 and 1:200, declared sheets; 2.0–2.4 mm kept at 1:50 and 1:500 |

Joint order of the rig is right and both sides are alike: coracoid – humerus – forearm – hand – primaries; pelvis half – femur – tibiotarsus – tarsometatarsus – foot.

### Defects found by reading the sheets

1. No shoulder girdle and no pelvis in the rig: humerus and thigh were children of the trunk with a `z` offset.
2. Inner wing open in every flight pose: between body and elbow only a 56 mm covert lobe, a notch as deep as the secondaries behind it.
3. 1:200 body: neck top pushed through the crown, head cut off flat at the back.
4. 1:200 structure: two unconnected wire groups (no rib, coracoid ended 5 mm above the sternum).
5. Thermal: secondaries had no layer and drew grey beside layered primaries.
6. 2D plan and front in flight poses: stick wings (see Remaining).
7. 1:50 body: step at the nape where the head dome overhangs the cylindrical upper neck, collar bulge at the neck base (see Remaining).
8. 1:50 structure: neck is two straight bars, not a column of vertebrae (see Remaining).

### Rebuilt

**Rig** (lateral rest placement of `T/rig-lateral-rest.md`; angles solved in the script with the documented order `Rw · Rv · Ru` and checked with `rig-rest.ts`):

- `coracoid-n/-f`: parent `trunk`, origin at the sternal facet [0.124, -0.028, ∓0.013], tip at the glenoid [0.095, 0.016, ∓0.036], 57.5 mm, `angleDeg 123.3885`, `abductionDeg ∓23.5791`, `twistDeg ∓104.7697`, revolute ±4°.
- `ilium-n/-f`: parent `trunk`, from the edge of the dorsal shield [0.004, 0.014, ∓0.016] to the hip socket [0, 0, ∓0.022], 15.75 mm, joint `fixed` (the avian pelvis is fused to the synsacrum).
- `humerus-*` re-parented to the coracoid tip and `thigh-*` to the ilium tip without moving: rest frames identical to four decimals (`rig-rest.ts`), so their solids, 2D shapes, anchors, masses, poses and clips keep their meaning.
- `t1`–`t5` per side on the humerus (humeral feathers / tertials, 90–122 mm, bases 14–110 mm from the shoulder), folded along the humerus at rest, fanned to 74°–100° behind the humerus in the seven flight poses (scaled by how far the pose opens the secondaries: `touchdown` 0.5, `landing` / `flare-key` 0.72, glides 1).

**Body**: five humeral vanes per wing at 1:50, 1:100, 1:200 (34–37 mm wide, 1.2 mm, `sheet`), two tapered blades 78 mm wide at 1:500 (on `t2`, `t4`); folded they lie inside the covert panel at every scale (checked at 1:50, 1:200, 1:500), in flight they close the inner wing. 1:100 and 1:200 flight feathers thinned to 1 mm true vanes and declared sheets; 1:200 membranes 1.4 mm sheets; P10 widened from 26 to 31 mm at 1:200. 1:200 and 1:500: neck ends inside the head, occiput rounded.

**Structure**: coracoid re-authored on its own bone at all four scales, running joint to joint; scapula (1:50, 1:100) and furcula top end at the same shoulder socket; acetabular wall of the pelvis on the ilium bone at 1:50; one rachis per humeral feather at every scale; hand bones declared slender at 1:100 and 1:200; at 1:200 a rib per side and a coracoid set into the sternum make one load path.

**Thermal**: one entry per humeral vane in `wing-plumage`; secondaries and humerals take the `wing-contour` layer of the primaries at every scale.

**Decisions**: body and structure texts of all four scales extended, English and German.

**2D**: nothing redrawn. The standing drawings still agree with the model (silhouette rule passes at all scales).

### Read after the last geometric change

1:50 body `standing` (six views), `gliding`, `touchdown`, clip `landing` at 1.6 s; 1:50 structure three-quarter; 1:200 body `standing`, 1:200 structure; 1:500 body `standing` and `gliding` (top); 1:100 thermal `gliding`; 2D 1:50 `gliding`. Rendered but **not read**: 1:50 `landing`, clip `soar` at 3 s, 1:100 body `gliding`, 1:200 right view after the head fix. Not rendered after the change: `gliding-up`, `gliding-down`, `approach-a`, `flare-key`, `settled`, 1:50 thermal, thermal and structure at 1:500, structure at 1:100, any second clip instant. The thermal layer change was validated but not rendered.

### Remaining (next pass)

- All scales, 2D `plan` and `front`, flight poses: the wing is drawn as bone lines and a brush of primary strokes; secondaries and the new humeral feathers have no plan / front surface. Needs `solid` shapes per feather (or grouped fans at 1:100 and 1:200) so the vane turns from edge-on to flat with the wing.
- 1:50 and 1:100 body, head and neck: nape step where the head overhangs the upper neck, collar bulge at the neck base, neck reads as a post; needs new lofts and then the matching elevation / front / plan shapes at three scales.
- 1:50 structure, neck: two straight bars; model the cervical column as vertebrae in its S, and the thoracic column likewise.
- Rig: neck has two links for 15 cervicals, tail one link, foot one bone without toe joints.
- 1:50 body, humeral feathers: plain grey vanes without the white tertial tips; no covert rows over the feather bases in flight; the fixed `near-/far-tertials` panel on the humerus still exists beside the new feathers.
- 1:500 body, flight: the two humeral blades read as two lobes; 1:50 and 1:500 flight feathers are still 2.0–2.4 mm thick without a sheet declaration (they pass there, I did not find out why 1:500 passes).
- 1:200 body: legs are thin strokes; head still a blunt cone.
- Ulna 140 mm against 145–160 mm.
- Scales 1:100 structure and 1:500 structure / thermal were changed by script and validated but not looked at.
- Everything listed above as not read / not rendered.

### Fundamental issues (outside this asset)

- `rig-lateral-standing-adult.py` `splice()` refuses the `volumes` member of this asset ("unknown layout"); rig and poses splice fine. Volumes went through `volumes-insert.py`.
- `r3-sheet.ts --list thermal`: an entry without `layer` in a layered region is drawn grey and has no legend entry, and the validator does not report a region that mixes layered and unlayered solids.
- `volumes-feature` passes 2.0–2.4 mm undeclared feather vanes at 1:500 while it rejects the same solids at 1:100 and 1:200; cause not investigated.
