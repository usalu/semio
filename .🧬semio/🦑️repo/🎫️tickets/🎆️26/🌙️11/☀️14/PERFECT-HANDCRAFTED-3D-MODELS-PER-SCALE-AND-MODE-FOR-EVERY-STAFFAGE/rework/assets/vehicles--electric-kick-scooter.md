# vehicles/electric-kick-scooter

## Pass 1 (second round) — 2026-11-22

State found: source `volumes/vehicles/electric-kick-scooter.json` identical to the catalog (no half-finished edit). No earlier report for this asset; `rework/09.md` lists it but never reached it. First full validation: INVALID, 25 findings (10 `volumes-feature` at 1:100 / 1:200, 5 `volumes-connected`).

### Reference (folding e-scooter of the Xiaomi M365 class, manufacturer figures and own knowledge)

| Quantity | Real | Model (rig / solids) | Verdict |
|---|---|---|---|
| Length × width × height unfolded | 1080 × 430 × 1140 mm | 1.09 × 0.43 × 1.11 m | ok (height 30 mm low, class value) |
| Folded height | 490 mm | about 0.48 m (stem 95° back, bell on the catch) | ok |
| Wheelbase | 0.80 to 0.86 m | 0.86 m (axles at x −0.43 / +0.43, y 0.108) | ok, upper end |
| Tyre 8½ × 2 | Ø 216 mm, section 50 mm | r 0.108, half width 0.025, rim seat r 0.068 | ok |
| Deck | 150 mm wide, about 55 mm deep, top at 150 mm, clearance about 90 mm | 0.15 wide, y 0.095 to 0.15, mat 0.132 × 0.484 | ok |
| Steering axis rake | 8 to 12° | 8° (fork −82°) | ok |
| Stem tube | Ø 38 to 44 mm, about 0.7 m above the hinge | Ø 44 → 38 mm, bone 0.719 m | ok |
| Handlebar | 430 mm, grips Ø 31 to 35 × 120 mm | 0.43 m, grips Ø 31/36 × 120 | ok |
| Hinge | lateral pin behind the steerer, 0.38 m above ground | joint of `stem` about w at fork (0, −0.024), y 0.38 | ok |
| Steering lock | about ±45° | joint had no twist limits | fixed: `twistMinDeg −45`, `twistMaxDeg 45` |
| Front hub motor | Ø about 150 mm, 66 mm wide, axle M12 | shell r 0.077 / half width 0.033 | ok |
| Rear wheel | cast five-spoke, hub Ø 40, rotor Ø 110 × 3 mm on a hub flange | rotor ring r 0.035..0.055 floated 8.5 mm beside the hub | fixed: hub flange `disc-flange-r` |
| Side stand | on a pivot block under the left deck edge | capsule 4.7 mm clear of the deck | fixed: `stand-pivot` |
| Mass | 12.5 kg | 12.5 kg (6.6 + 0.9 + 1.6 + 2.4 + 1.0) | ok |

### Defects found

- 1:50 body: side stand floated; brake rotor touched nothing. Otherwise the 1:50 body (50 solids: hollow deck extrusion, nose and tail castings, gooseneck loft, head tube with cups, fork with crown and blades, channel mudguards, hinge, tapered stem, cockpit, levers, wheels with tyre, rim, spokes, hub, bearings, axle) reads right from all six directions and in all poses; kept.
- 1:50 structure: the stem group (hinge pin, stem, bar) met the steerer nowhere (11 mm gap) because the hinge base was missing from the load path; the rotor was a floating "member".
- 1:100, 1:200, 1:500: both wheels were the 1:50 parts list (10 mm spoke web, 9 mm bearings, 14 mm axles) in all three lists; at 1:100 the rotor floated and the deck girder of the structure stopped 10 mm short of the stays.
- 1:500 body: the deck was a trapezoid tapering over its whole length (real deck is parallel-sided with two converging rear arms).
- Decisions of 1:100 / 1:200 / 1:500 carried a stale sentence about bearing cartridges and five spoke windows that those scales do not model.
- Rig: topology right (deck root; fork turning about the raked steering axis by twist; stem folding about the lateral hinge pin; both wheels about lateral axles). Only the steering limits were missing.

### Rebuilt (chisel scripts `rework-electric-kick-scooter-volumes.py`, `rework-electric-kick-scooter-rig.py`)

- Rig: steering limits ±45° on `fork`.
- 1:50: `disc-flange-r` (hub flange carrying the rotor, also thermal `rear-rim`), `stand-pivot` (also thermal), structure gains `hinge-base` as fork member and loses the rotor.
- 1:100 (15 mm): wheels re-authored: eight-point tyre ring, dished disc wheel (rim band + 16 mm web), hub barrel with rotor flange, axles nut to nut (18 mm), rotor as texture plate on the flange, one-piece motor shell; thermal and structure lists rebound; deck girder reaches the stays.
- 1:200 (30 mm): two turned parts per wheel (six-point tyre ring; rear wheel body with hub barrel into both deck arms and rotor flange; motor shell with axle bosses into both fork legs); rotor own solid in the thermal list; axles as own joint solids in the structure list.
- 1:500 (75 mm): tyre ring with flat tread plus one wheel body per wheel; rotor only in the thermal list; deck redrawn parallel-sided with converging arms.
- Decisions: one new en/de entry per scale; stale bearing sentence removed at 1:100 / 1:200 / 1:500.
- 2D: not redrawn. The changed forms do not alter any outline the drawings show (1:100 already draws `wheel-disc-r` as a disc, 1:200 draws wheel ellipses, 1:500 has no print drawing); `volumes-silhouette` and `volumes-views-disagree` pass; `r3-2d` 1:100 read and agrees.

### Rendered and read after the last geometric change

1:50 body (six views, left close-up), poses `braking-stop` and `folded-charging`, clip `fold-and-charge` t 1.8 s, clip `ride-loop` t 0.1 s (left), 1:50 thermal, 1:100 body / thermal / structure, 1:200 body (before the wheel change) and structure (after), 1:500 body / thermal / structure, 2D 1:100.

Not read after the last change (rendered only or not rendered): 1:50 structure, 1:200 body and thermal, `ride-loop` second instant, `fold-and-charge` t 4 s, pose `standby-unfolded` (same bones as default), 2D 1:50 and 1:200.

### Validation

`r3-validate.ts vehicles/electric-kick-scooter --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`; after `volumes-insert.py`: `--catalog-only --volumes` `VALID`.

### What remains imperfect

- Support solids in the structure list: tried (bearing blocks under both tyre patches) and withdrawn, because `volume-anchor` demands an entry in `structure.supports` and this asset is modelled with unilateral ground contacts (`supports: []`, pose `contacts`); the load path ends at the two `contact` anchors, which the rule accepts.
- 1:200 and 1:500 body have no mudguards (3 mm sheet, 10 mm off the tyre; below what those scales show) while the thermal lists keep them as the `fenders` region, so body and thermal sheet differ by the two arcs.
- 1:200 structure has no joint solids for the two head bearings (1:100 has them).
- 1:50: rotor is a plain ring on a plain flange (no spider cut-outs, no caliper bracket); no brake cable, charge-port cap or rear reflector; the front mudguard has no visible stay to the crown.
- 1:500 neck is a straight two-section loft; non-wheel solids of 1:100 / 1:200 / 1:500 were judged by rendering and kept as found.
- Views listed above as not read after the last change.

### Fundamental issues (outside this asset)

- None blocking. Observation: `volumes-feature` lets a 3 mm plate pass through `role: "texture"` alone at every scale (rotor), where the rule text speaks of `sheet: true` or a matching thermal layer; worth a line in `validator.md`.
