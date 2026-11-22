# animals/sheep — author report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Rig and girdles are on the lateral rest placement, the skeleton is connected at all four scales, the asset is `VALID` with the measured rules and published. The body and thermal solids, the 2D drawings and the poses were only re-bound and checked, not re-sculpted; what is still wrong is listed at the end.

State found: source `volumes/animals/sheep.json` and catalog were identical (no half-written edit). `r3-validate.ts animals/sheep` was INVALID with 58 `volumes-feature` findings (1:200 claws, 1:500 claws, ears, and a 1:500 structure list that was the 1:50 sections with entries deleted). Batch report `rework/07.md`: "not started".

### Reference (ewe in full fleece, 70 kg, withers about 0.72 m; own knowledge of ovine anatomy, values rounded)

| Quantity | Reference | Model before | Model now |
| --- | --- | --- | --- |
| Hip joint height | 0.57–0.60 m | 0.590 | 0.590 |
| Stifle / hock / fetlock height | 0.42 / 0.26–0.28 / 0.08–0.09 | 0.425 / 0.262 / 0.085 | same |
| Shoulder joint / elbow / carpus height | 0.47–0.50 / 0.38–0.40 / 0.21–0.22 | 0.530 / 0.403 / 0.222 | same (shoulder about 30–40 mm high, see open list) |
| Scapula / humerus / radius / metacarpus | 0.17–0.18 / 0.15 / 0.17–0.18 / 0.13–0.14 | 0.179 / 0.150 / 0.182 / 0.137 | 0.185 (3D) / 0.150 / 0.182 / 0.137 |
| Femur / tibia / metatarsus with tarsus | 0.19 / 0.21–0.22 / 0.17–0.18 | 0.193 / 0.215 / 0.177 | same |
| Digit: pastern (P1+P2) / claw (P3) | 0.065–0.07 / 0.035–0.04 | one bone 0.110 / 0.107 | 0.070 + 0.040 (hind), 0.068 + 0.039 (fore) |
| Joint order fore | scapula–humerus–radius–carpus–cannon–pastern–claw | pastern and claw one bone | as reference |
| Joint order hind | hip bone–femur–tibia–hock–cannon–pastern–claw | no hip bone, pastern and claw one bone | as reference |
| Scapula: dorsal border / shoulder joint beside the median plane | 0.04–0.05 / 0.09–0.10 | bone in the median plane (z 0) | 0.050 / 0.095, 14° abducted |
| Hip bone: sacroiliac joint / hip joint beside the plane | 0.03–0.04 / 0.08–0.095 | none; thigh in the median plane | 0.035 / 0.095 |
| Track half-width fore / hind | 0.09–0.10 | solids 0.090 / 0.103, bones 0 | 0.095 / 0.095 |
| Rib cage half-width, ribs 1 … 8 | narrow inlet, 0.04 rising to 0.12 | 0.055, 0.070, 0.085, 0.098, 0.108, 0.116, 0.122, 0.127 | 0.045, 0.055, 0.066, 0.078, 0.090, 0.102, 0.112, 0.122 (1:50, 1:100) |
| Ilium | body from sacrum to acetabulum, wing to tuber coxae | bar 70 mm beside the plane, never meeting the sacrum; pubis 3.7 mm short of the acetabulum | body sacral end → acetabulum, wing → tuber coxae, pubis into both acetabula |

### Defects found

- Rig: all 28 bones lay in the median plane; limbs were placed sideways only by offsets inside their solids. No shoulder or pelvic girdle bone ran laterally. Pastern and claw were one bone.
- Skeleton 1:50: ilium not connected to sacrum or pubis; root joint marker floating; cranial rib cage as wide as the shoulder joints (olecranon 4.9 mm from rib 4).
- Skeleton 1:100: rib heads 9 and 11 short of the spine by 7–8 mm.
- Skeleton 1:200 and 1:500: hip bone ran tuber coxae → acetabulum and never reached the sacrum; scapula bars loose beside the ribs.
- Skeleton 1:500: 1:50 sections with entries removed (39 findings); ribs without sternum.
- Body 1:200 / 1:500: two 20 mm claws per foot below the feature; ears and hind cannon of 1:500 below the thinnest drawing.

### What was done (scripts in `T`, each states its hand-chosen numbers)

1. `rework-sheep-rig.py` — rig on the full 3D rest placement (`rig-lateral-rest.md`):
   - `scapula-fn/-ff`: from the dorsal border (thorax 0.2404, 0.0887, ∓0.05) to the shoulder joint 0.095 beside the plane (`abductionDeg ∓14.1`, 0.185 m); humerus re-solved under it with its joint axis lateral.
   - new `ilium-hn/-hf` (hip bone half): sacroiliac joint (pelvis 0.095, 0, ∓0.035) to the hip joint 0.095 beside the plane (0.112 m, `abductionDeg ∓32.3`, joint ±3°); thighs hang from its tip.
   - new `pastern-*` between cannon and `hoof-*`; `hoof-*` is now the claw (coffin joint −15…25°). The mass of the old digit is split 57 / 43.
   - every limb is a straight parasagittal column at z ∓0.095 (see "decision" below); all limb solids of body, thermal and structure at all four scales were re-expressed in the new frames and straightened onto the column (joint centres were at 0.088–0.115).
   - re-bound: poses and clip keys (`hoof-*` offsets → `pastern-*`), contact anchors (toe tip of the claw bone), 2D shapes of the feet (→ `pastern-*`), front and plan shapes of the limbs shifted by the same lateral amounts, structure `member` fields, joint markers for the new bones.
   - Decision: bones tilted joint-to-joint in 3D (femur 8° out, tibia 5° in) were tried first and are anatomically closer, but `validateAsset` then reports `lateral-ambiguous` for all 14 flat elevation shapes of the legs. They would have to be redrawn as extruded solids; not done in this pass, so the columns are straight.
2. `rework-sheep-coarse.py` — 1:200: one horn block per foot (sole on the ground plane); 1:500: horn block on the pastern, ears and hind cannon at the drawable thickness, skeleton as slender true shafts (spine, sacrum, hip bone, scapula, long bones, mandible, tail), digits one bar per foot.
3. `rework-sheep-pelvis.py` — 1:50 ilium body + wing, pubis into the acetabula; 1:100/200/500 hip bone sacroiliac joint → acetabulum → ischial tuber; rib heads 9 and 11 of 1:100 to the spine; coffin joint markers sized to the foot.
4. `rework-sheep-joints.py`, `rework-sheep-scapula.py`, `rework-sheep-ribs.py` — fore digits reach the pedal bones; 1:500 sternum; cranial ribs 1–8 narrowed at 1:50 and 1:100; scapula bars of 1:200 / 1:500 moved onto the ribs. Two of these steps partly cancel: the 1:50 blade was first moved 15.5 mm towards the ribs (wrong diagnosis: the reported 4.9 mm gap was the olecranon, measured afterwards by sampling the surfaces) and then put back on its bone line by `rework-sheep-ribs.py`. Run in the order listed they give the published state.
5. `rework-sheep-decisions.py` — one decisions entry (en, de) per scale for the above.

### Validation

`r3-validate.ts animals/sheep --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; published with `volumes-insert.py`; `r3-validate.ts animals/sheep --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`.

Rendered and read after the last geometry change: structure 1:50 (default, lying), 1:100, 1:500 (before the sternum and the last scapula shift); body 1:50 grazing, 1:200, 1:500; thermal 1:50; 2D 1:50. Rendered but not read: thermal 1:100/200/500, structure 1:200, body 1:100, walk-cycle t 0.9, graze-cycle t 3.2, ruminate-lying t 1, body lying (read only on the rig candidate before the skeleton edits; walk-cycle t 0.4 likewise). No solid failed to tessellate in any of them.

### Open (for the next pass)

- All scales, rig: limb columns are straight at z ∓0.095; true stifle-out / hock-in needs the 14 leg elevation shapes as extruded solids first (`lateral-ambiguous`).
- Rig: shoulder joint 0.53 m above ground, about 30–40 mm high against the reference; not changed (would move every fore-limb pose and the walk cycle).
- Rig: ears are still sagittal bones with the ear modelled sideways inside the solid (should be lateral rest placement); neck is one bone (no cervical curve in grazing); no carpus / tarsus bones of their own.
- 1:200, 1:500 structure: rib cage keeps the old wide cranial half-widths (narrowed only at 1:50, 1:100); scapula bars sit 13 mm / 25 mm medial of their bone line so that they touch the ribs (`volumes-connected` wants contact or a clear feature of distance; the real connection is muscular).
- 1:50 structure: cervical spine is one tube without vertebrae, skull one loft without orbit or separate mandible rami, no carpal / tarsal bones, sacrum without wings; scapula has no spine.
- 1:50 body: not re-sculpted. Trunk fleece is three noisy polyhedra with a visible seam at mid trunk; legs are plain tapered tubes without carpus / hock form; neck is rigid, so in `grazing` the neck–chest and neck–head junctions overlap coarsely; `lying-ruminating` hides the folded legs inside the fleece and the belly dips about 40 mm below the ground line.
- Thermal: only re-bound; layers and areas not reviewed, `--cut` sheets not read.
- 2D: only re-bound and shifted; `fold-stifle` / `fold-elbow` are crude hexagons in the elevation; 1:100, 1:200, 1:500 drawings not rendered after the change.
- Decisions: older entries per scale still describe the earlier model in places (1:200 "one wedge for pastern and hoof", 1:500 "real widths").

### Fundamental issues (outside this asset)

- `volumes-insert.py` runs `main()` on import and its formatter does not reproduce the catalog file byte for byte, so a root edit of `rig`, `poses`, `representations` re-formats those members (done here member-wise; all other members byte-identical).
- `lateral-ambiguous` rejects any limb bone that leaves the sagittal plane by more than about 5° when it carries a flat elevation shape; with full 3D rest placement this blocks anatomically tilted long bones until `r3-project2d` can emit extruded elevation shapes.
- `volumes-connected` has no way to state a muscular suspension (scapula on the thorax): the skeleton must touch or stay a full feature apart, which at 1:200 / 1:500 forces contact that does not exist in the animal.
- A structure bone shorter than two features can be neither `slender` nor thick enough at 1:500 (tail segments 0.10 / 0.09 m, coverage demands a member): solved here with two overlapping tail shafts.
