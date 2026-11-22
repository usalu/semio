# animals/mallard-duck

- 2026-11-22 12:2x, rig verification (not the volume author): `poses` and `clips` re-solved with `rig-reoffset.ts animals/mallard-duck --write` because the rig now turns a pose offset about the bone's own rest axes instead of the parent's joint axis. The wing bones forearm-n/-f and hand-n/-f had moved by 1.2 mm in wing-stretch; no other pose or clip key turns them. Every bone keeps the posed frame it had before the rig change (largest difference of a bone tip after the re-solve under 0.01 mm). Only `asset.poses` and `asset.clips` were written; rig, shapes, volumes untouched. Do not run the tool on this asset again (`rig-reoffset.done.json`); new poses are authored about the bone's own axes (`rig-lateral-rest.md` 1.1).

## Pass 1 (second round) — 2026-11-22

State at the start: source `volumes/animals/mallard-duck.json` identical to the catalog; `r3-validate --volumes` INVALID (14 + 16 findings: webs 2.6 mm and hallux 2.0 mm at 1:100 and 1:200, tarsus 8.4 mm at 1:200, sternum and wing skeleton not joined to the spine at 1:100 and 1:200). Chisel of this pass: `rework-mallard-duck-wing.py` (hand-chosen joint points, feather tables and section rows; re-runnable on the written catalog; `--write` writes catalog and source).

### Reference (drake, own knowledge) against the model

| Measure | Real mallard | Model before | Model now |
|---|---|---|---|
| Length bill tip to tail tip, standing S-neck | 0.42–0.48 m (0.50–0.65 m stretched) | 0.417 m | unchanged (small end; body about 0.87 of a mean drake) |
| Body breadth with folded wings | 0.13–0.15 m | 0.136 m | 0.137 m |
| Humerus | 90–95 mm | carrier bone "wing" 249 mm from shoulder to primary tips, elbow 13 mm beside its axis | 93.4 mm, shoulder to elbow |
| Ulna / radius | 76–82 mm | 79.3 mm | 79.1 mm |
| Carpometacarpus + digits | 57 + 28 mm | 85.0 mm | 85.0 mm |
| Coracoid | 45–52 mm | no bone (one loft on the trunk) | own bone 40.8 mm from the sternum up and out to the shoulder joint (lateral rest placement: angle 104.4°, abduction −27.7°, twist) |
| Wing chord wrist to tip, folded | 257–285 mm (×0.87 = 225–250) | 233 mm | 230 mm (longest primary 142 mm on the 85 mm hand) |
| Primaries / secondaries | 10 / 10–11, plus tertials | one blade / one plate | 10 / 10 / 2 tertials |
| Shoulder joints apart | 60–70 mm | 60 mm | 60 mm |
| Hip joints apart (pelvis fused to the synsacrum) | 40–44 mm | 40 mm | 40 mm |
| Femur / tibiotarsus / tarsometatarsus / middle toe | 50 / 80–88 / 42–48 / 55–60 mm | 50 / 85 / 45 / 58 | unchanged |
| Cervical column | 150–170 mm | 165 mm | unchanged |
| Span, wings three quarters open | 0.6–0.7 m (0.81–0.98 m fully spread) | folded plate swung out | 0.64 m |

Defects found: (1) wrong wing skeleton: the "wing" bone was a carrier to the primary tips, forearm and hand hung beside it, secondaries and scapulars were bound to the plate, so `wing-stretch` lifted a folded plate; (2) no shoulder girdle bone; (3) 2D wing drawn as plates on the carrier bone; (4) the validator findings above.

### Rebuilt

- Rig: `coracoid-n/-f` (lateral rest placement, limits ±4°), `humerus-n/-f` (renamed from `wing-*`, re-aimed shoulder to elbow), `forearm`, `hand` on their joints; feather tract bones `p1..p5` on the hand, `s1..s5` on the forearm, `t1` on the humerus, both sides mirrored by the rule of `rig-lateral-rest.md` 1.1 (z, abduction, twist negated). Frames checked with `rig-rest.ts --file`.
- Pose `wing-stretch`: solved from target directions (humerus back-up-out, elbow 133° open, wrist nearly straight); the tract bones fan the feathers in the wing plane (primaries 24°–80°, secondaries 14° to −12°, tertials −30° from the trailing direction). No other pose or clip turned a wing bone.
- Body 1:50: per side upper wing, forearm and hand as lofts on their bones, forearm and hand covert shields, 10 primaries, 10 secondaries, 2 tertials as vanes of 1 mm (`sheet`) laid along the flank hull with their calamus on the bone; scapulars re-bound to the trunk. 1:100: one blade per tract; 1:200: three primary, three secondary and one tertial blade, arm as thin panels in the wing plane; 1:500: the same blades, no shields.
- Thermal: every new solid is a `wing-plumage` contour reference; scapular down layer re-bound to the trunk; declared `areaM2` unchanged and accepted by the measured rules.
- Structure: coracoid on its own bone, humerus with heads, ulna and radius, carpometacarpus with minor metacarpal, wing digit and alula digit, shoulder / elbow / wrist joints (1:50); one forearm member and one manus from 1:100; a feather shaft per tract bone at every scale; one rib pair added at 1:200 so that sternum and girdle reach the spine.
- 2D (1:50, 1:100, 1:200; elevation, front, plan): folded wing plates removed, scapulars re-bound to the trunk, every tract bone draws its vane pair as two `solid` (extruded, sweep w) shapes, so the drawings fold and open with the rig (`wing-stretch`: 0.37 m high, 0.64 m span in plan).
- Baseline findings closed: webs thickened to 3.25 mm and hallux to 3.3 mm at 1:100 and 1:200, tarsus 8.9 mm at 1:200.
- Decisions: one wing sentence per scale on the body entry and one girdle sentence on the structure entry (en, de).

Validation: `r3-validate.ts animals/mallard-duck --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`.

Rendered and read after the last body change: 1:50 body standing (six views and right close-up), wing-stretch, upending; 1:50 structure standing and wing-stretch; 1:50 thermal; 1:100 standing; 1:200 and 1:500 wing-stretch; 2D 1:50 standing and wing-stretch. Rendered but not read: resting-ashore, swimming, dabbling, waddle t 0.3, paddle t 0.35, dabble t 2 (wings stay folded there). Not rendered: thermal and structure at 1:100, 1:200, 1:500; `--cut` thermal; 2D at 1:100 and 1:200 in the new state (1:200 wing-stretch was drawn, not read); waddle-0..7 and paddle-0..5 one by one.

### Remains imperfect

- Pelvic girdle: the pelvis is fused to the synsacrum, so no pelvis-half bone was added; the hips are at the true 40 mm, but the femur still runs sagittally and the 18 mm to the knee are a lateral step of the shank origin instead of a femur splayed about 21°.
- 1:200 coracoid is a 2 mm rod (true 5–8 mm): a 41 mm bone can be neither `slender` (needs 60 mm) nor 30 mm thick.
- 1:100 forearm and hand are thickened to 15 mm (true 13 and 10 mm).
- Folded primaries bend up to 30 mm inwards to follow the rump, so the open wing is cupped more than a real one.
- 2D: no shapes for upper wing, forearm, hand and covert shields (the open wing is drawn by the vanes alone); no feather detail strokes.
- Wing tones: one grey-green for all tracts except the darker secondaries; no white speculum borders.
- Body, neck, head, bill, tail and legs were judged from the sheets and the table only and not re-modelled; the whole bird stays at 0.87 of a mean drake while the leg bones are at mean size.

### Fundamental issues (outside this asset)

- `volumes-feature` uses the full smallest feature for a bone without 2D shapes but the thinnest drawing for a bone with shapes, so the same solid passes or fails depending on whether a drawing exists on its bone.
- `volumes-slender` / `volumes-feature` leave no valid statement for a short true bone at coarse scales (coracoid 41 × 5 mm at 1:200).
- `rework-brief.md` suggests PIL-free tools only; `numpy` and `PIL` are not installed for `python3` on this host.
