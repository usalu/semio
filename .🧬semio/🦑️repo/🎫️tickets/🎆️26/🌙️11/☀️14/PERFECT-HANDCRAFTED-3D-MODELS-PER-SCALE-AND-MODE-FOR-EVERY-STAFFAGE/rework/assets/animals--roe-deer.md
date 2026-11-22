# animals/roe-deer — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Catalog and source agree and `r3-validate.ts animals/roe-deer --catalog-only --volumes` ends `VALID` (schema 0, validateAsset 0, checkVolumes 0; before this pass: validateAsset 80, plus 10 volume findings). The girdles are on the lateral rest placement. Not everything was rebuilt or looked at again; the open list is at the end.

State found: no earlier report, batch 07 lists the asset as "not started"; source and catalog `volumes` were identical. `rig-lateral-rest.md` existed at the start and was unchanged at the end (13:04).

### Reference (adult roe buck, Capreolus capreolus, 25 kg) against the model

| Measure | Reference | Model | Verdict |
| --- | --- | --- | --- |
| Shoulder height | 0.63–0.75 m | withers about 0.72 m (hip joint 0.66 m) | ok |
| Croup above withers | 20–40 mm | about 30 mm | ok |
| Head-body length | 0.95–1.35 m | 1.04 m | ok |
| Trunk breadth | 0.18–0.22 m | 0.203 m | ok |
| Scapula | 0.15–0.17 m | 0.1656 m (was 0.164 in the sagittal projection) | ok |
| Humerus | 0.15–0.16 m | 0.152 m | ok |
| Radius | 0.17–0.20 m | 0.195 m | ok |
| Metacarpus | 0.16–0.17 m | 0.169 m | ok |
| Femur | 0.18–0.19 m | 0.185 m | ok |
| Tibia | 0.22–0.24 m | 0.225 m | ok |
| Metatarsus | 0.20–0.21 m | 0.230 m | 20 mm long, kept (changing it moves every hind pose and clip) |
| Digit (fetlock to toe) | 0.08–0.09 m | 0.092 m, one bone for pastern and hoof | length ok, joint between pastern and hoof missing |
| Hip joints apart | 0.09–0.10 m | were 0 (both thighs on the median plane), now 0.100 m | fixed |
| Stifles apart | 0.13–0.14 m | were 0, now 0.136 m | fixed |
| Dorsal borders of the scapulae apart | 0.06–0.09 m | were 0, now 0.090 m | fixed, at the wide end |
| Shoulder joints apart | 0.13–0.14 m | were 0, now 0.136 m | fixed |
| Ear | 0.12–0.14 m | 0.135 m, both bases were on the median plane, now 60 mm apart | fixed |
| Antler | 0.20–0.25 m, three tines | 0.2 m, beam, brow and back tine | ok |
| Tail | 20–30 mm, not visible | none, white rump patch | ok |

Joint order found and kept: scapula – humerus – forearm – (carpus) – cannon – digit; femur – tibia – (hock) – cannon – digit. Both sides alike.

### Defects found

- Rig: every limb bone, both scapulae and both ears lay on the median plane; the limbs had no girdle and their solids were offset inside the bone frames.
- Skeleton 1:50: the ilia were two rails 45 mm beside a sacrum they never touched; a root joint marker hung in the air between the hips; second phalanx and pedal bone 5.7 mm apart; ear cartilage 2 mm beside the skull.
- 1:100, 1:200, 1:500 were the 1:50 parts with too-thin pieces: 8 + 12 + 20 body solids and 37 structure solids below the feature; rib 5 off the spine at 1:100; scapula unrelated to the rib cage at 1:200 and 1:500; three thermal regions undrawn at 1:500.
- Pose `grazing`: crouched on folded forelegs with the far upper arm pointing straight backwards and the trunk pitched 9°.

### Rebuilt

- **Rig** (`rework-roe-deer-rig.py`): new bones `hip-hn` / `hip-hf` from the sacroiliac joint (24 mm beside the plane) to the hip joint (50 mm), 89.7 mm, 16.9° out of the sagittal plane; thighs re-parented with unchanged sagittal frames; stifle 18 mm outside the hip joint (`shank.z`); scapulae from the dorsal border (45 mm) to the shoulder joint (68 mm), tilted 8.0°; humeri re-solved to the sagittal frame; ear bases 30 mm beside the plane. Every solid of every scale and list was re-expressed in the new frame of its bone and stays where it was in the world; ilium and ischium moved from `pelvis` to the hip bones; hip-bone members and joints added. Poses and clips needed no change (all limb rest axes kept).
- **Structure**: 1:50 sacral wings carry the hip bones, floating root marker removed, phalanx bent into the pedal bone, the two pedal bones of a foot meet, ear cartilage rooted in the skull. 1:100 own sacral wings, ilium through the sacroiliac joint, rib 5 on the spine. 1:200 hip-bone bars, first rib under the scapula. 1:500 true slender shafts for spine, long bones, ribs, mandible, scapula bar; one digit per foot; pelvis as a block of sacrum and two hip-bone halves.
- **Body**: 1:100, 1:200, 1:500 one hoof form per foot; 1:200 pastern and hoof at 30 mm; 1:500 antler as one beam, ear, upper arm, forearm, hock and hind cannon at their drawn widths. 1:50 body unchanged.
- **Thermal**: follows the body (`of` references); hoof area at 1:200 brought back inside the 30 % limit by a shorter hoof form.
- **2D**: scapula shapes at 1:50, 1:100, 1:200 are extruded solid shapes on the tilted bone (side elevation unchanged; front and plan now show a slab); at 1:500 the head shape stands for nose leather and eyes and the hind body for the rump patch.
- **Pose** `grazing` (`rework-roe-deer-poses.py`, solved on the rig): four soles on one ground (residual 0.2 mm), near foreleg 27° ahead, far foreleg 20.9° back, back rounded 10°, root 0.95°, neck -107°, head 64° (limit 75°), muzzle 60 mm above the ground.
- **Decisions**: girdle sentence added to the structure decision of 1:50, 1:100, 1:200; body, structure and foot decisions of 1:500 rewritten (they still described four tines and eight claws).

### Rendered and read

Before: 1:50 body and structure (six views). After the rig: 1:50 structure (six views and top), 2D 1:50 `bounding-gathered` before and after. After the solids: 1:500 structure, 1:500 body, 1:50 `bounding-gathered`, 1:50 thermal cut, 2D 1:50 and 1:200, `walk-cycle` 0.3 s, `alarm-freeze` 2.1 s, `grazing` (old pose, two intermediate solutions).

### Remaining (not done or not looked at)

- Rig: pastern and hoof are one bone; no joint between them.
- Rig: the femur is sagittal with a lateral step at the stifle instead of a true 5.6° abduction (a tilted thigh makes its flat elevation shape `lateral-ambiguous`).
- Rig: metatarsus 20 mm longer than the reference.
- 1:50 body: not re-sculpted. In `bounding-gathered` the croup stands as a bulb behind the forward-swung thigh; the hock point reads as a small spike.
- 1:50 structure: ilium is still a narrow blade, not an oblique wing from tuber sacrale to tuber coxae; cervical spine one tube without atlas and axis; skull a wedge; no caudal vertebrae.
- 1:500 structure: pelvis is three 76 mm sausages; ear cartilage and antler are fat cones; scapula bars meet over the withers (rule-driven, see below).
- 1:200 and 1:500 structure: first and second rib widened under the scapula by hand, not checked against the 1:50 rib cage.
- Thermal: one coat shell per region, no nested layers; sheets of 1:100, 1:200, 1:500 not rendered.
- 2D: scapula slab in front and plan at 1:50, 1:100, 1:200 is boxier than the former hand drawing; hooves at 1:100, 1:200, 1:500 not redrawn after the 3D merge (validator passes); 2D 1:100 rendered but not read, 1:500 read only before the region change.
- Not read after the last change: final `grazing` (rendered), `bounding-pushoff` (rendered), `alert-foreleg-raised`, the 16 walk and bound phase poses one by one, `bound-cycle`, second instants of `walk-cycle` and `alarm-freeze`, 1:100 and 1:200 body, 1:100 and 1:200 structure (rendered).
- `rig-lateral-compare.ts` reports up to 204 mm between before and after; I attribute it to the anchors (now at the hoof, 68 mm beside the plane) and the replaced scapula front and plan shapes, judged from the 2D sheets, not measured per shape.

### Fundamental issues (outside the asset)

- `rig-lateral-standing-adult.py` `splice` refuses `representations` of this asset ("unknown layout"). I wrote `rig`, `representations` and `poses` in the `rework-01-lib.catalog_text` layout; the catalog file grew from 16 735 to about 36 000 lines with the same content.
- `lateral-ambiguous` forces extruded shapes for an 8° tilted scapula (open request in `validator.md`).
- `volumes-connected` asks the scapula to touch the trunk skeleton or stay a feature away at 1:200 and 1:500; the real connection is muscular, so the skeleton has to be bent to pass.
- A hip bone of 90 mm must have a member at 1:500 but can be neither slender (under 150 mm) nor thinner than 75 mm.

Scripts: `rework-roe-deer-rig.py`, `rework-roe-deer-solids.py`, `rework-roe-deer-2d.py`, `rework-roe-deer-poses.py`.
