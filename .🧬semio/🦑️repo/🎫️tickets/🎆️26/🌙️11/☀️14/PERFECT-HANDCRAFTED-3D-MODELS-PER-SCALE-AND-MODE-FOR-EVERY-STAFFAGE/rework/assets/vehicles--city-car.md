# vehicles/city-car — running author report

Earlier work on this asset: `T/rework/09.md` (sections "vehicles/city-car" and the connectivity round), scripts `rework-09-city-car-*.py`.

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published and `VALID`; the open list at the end is real.

### State found

- Source `T/volumes/vehicles/city-car.json` and the catalog `asset.volumes` were identical (no half-finished edit to reconcile).
- No section "Rule changes authors must know" exists in `T/validator.md` yet and `T/rig-lateral-rest.md` does not exist; the schema itself (`Bone.z`, `abductionDeg`, `twistDeg`) already allows laterally resting bones and joint axes that are not the sagittal one.

### Reference (B-segment five-door hatchback, e.g. VW Polo 6R / Suzuki Swift / Honda Jazz; from memory)

| quantity | real | model | verdict |
|---|---|---|---|
| length / width / height | 3.85–3.97 / 1.68–1.70 / 1.46–1.53 m | 3.90 / 1.68 / 1.50 m | ok |
| width over mirrors | about 1.90 m | 1.90 m | ok |
| wheelbase / front and rear overhang | 2.43–2.50 / 0.78–0.82 / 0.62–0.70 m | 2.45 / 0.80 / 0.65 m | ok |
| tyre 185/60 R15 | 603 mm diameter, 185 mm wide, rim 381 mm | 0.60 m, 0.186 m, rim well 0.37–0.42 m, five spokes | ok |
| wheel arch | circle about 0.36–0.37 m radius concentric with the wheel, lip 15–20 mm | 0.365 m; 1:200 was a 4-chord polygon | fixed at 1:200 |
| bonnet | hinged at the scuttle, opens 45–50°, leading edge 0.75–0.80 m, cowl 0.95–1.00 m | hinge (0.95, 0.97), 0.93 m long, leading edge 0.80 m, opens 48° | ok |
| engine bay | open space between the two front wings, the scuttle and the slam panel: transverse four-cylinder (block about 0.45 × 0.30 × 0.40 m, head cover at about 0.78 m) with the gearbox on its left end, on two front rails (about 70 × 100 mm, 0.84 m apart), radiator 0.62 × 0.36 m behind the grille, battery 0.24 × 0.18 × 0.19 m on a tray, air box | **none**: the nose was one closed loft, the open bonnet showed a second closed surface with a dark patch (`engine-cover`) | fixed at all four scales |
| lower intake | follows the bumper's plan curve | a flat 0.84 m box that stood up to 12 mm proud of the curved bumper at its ends | fixed (1:50) |
| tailgate | hinged at the roof, opens about 100° | hinge (-1.50, 1.44), 0.74 m, opens 100°, boot and seats visible | ok |
| suspension travel | ±0.08–0.10 m | prismatic ±0.10 m front and rear | ok |
| doors | four side doors on vertical hinges at the A- and B-pillar, opening 65–70° | shut lines only, no bones | **open** |
| steering | front wheels turn ±35° about the kingpin | none | **open** |

### Looked at (read at full size)

1:50 body parked (sheet and three-quarter close-up), bonnet-open (before and after, also from the front above), hatch-open (before), clip `drive-by` t 1.8 s, clip `park-and-open-hatch` t 5.8 s; 1:50 thermal cut (before) and thermal bonnet-open (after), 1:50 structure; 1:100 body bonnet-open (before and after) and structure; 1:200 body (before and after the arches), bonnet-open, thermal cut; 1:500 body hatch-open (before), bonnet-open (after), structure, thermal cut; 2D 1:50 bonnet-open and 2D 1:200 parked.

Rendered after the last change but **not read**: 1:50 poses `driving`, `crawl-in`, `braking-nose-down`, `lane-start`, `lane-cruise-entry`, `lane-exit`, `hatch-open` (after), clip `drive-by` t 3.8 s, clip `park-and-open-hatch` t 2 s, 2D 1:50 parked, 1:100 thermal cut, 1:200 structure. They only move the root, the wheels and the suspension slides, which this pass did not touch, but they were not looked at.

### Rebuilt

Scripts (hand-stated numbers): `T/rework-city-car-bay.py`, then `T/rework-city-car-arches.py` (run once each, in that order, on the source as author 09 left it).

- **Body, all four scales — engine bay.** The closed `nose` loft (1:500: the front of `hull`) is now four solids with the old outer faces: `scuttle` (to x 1.02 m), `wing-l` / `wing-r` (0.28 m wide front wings from 0.99 to 1.70 m, 0.30 m at 1:500, carrying the arch and the crown of the old nose) and `nose` (front end with the slam panel from 1.66 m). Under the bonnet the bay is open down to the road as on the real car. In it:
  - 1:50: `front-rail-l/r` (1.0 × 0.10 × 0.07 m at z ±0.42, y 0.47, the rails of the structure list), `engine-block`, `gearbox`, `exhaust-manifold` (the geometry of the hidden thermal region `engine-bay`), `valve-cover`, `air-box` on the gearbox, `engine-mount` between block and right rail, `battery` on its `battery-tray` on the left rail, `radiator` against the front end. `engine-cover` (the dark patch) is gone.
  - 1:100: rails, block, gearbox, mount, battery with tray, radiator. 1:200: rails (0.10 m wide at z ±0.40 so the one engine block of 0.75 m rests on both), engine, radiator. 1:500: the engine block on the dark underbody between the wings.
- **Body 1:50:** `intake` is a loft along the bumper's plan curve (x 1.958 m on the centre line to 1.946 m at z ±0.42), 24 × 90 mm, instead of the flat box.
- **Body 1:200:** wheel arches as six 30° chords of the 0.365 m circle (stations ±0.183 m and ±0.316 m from the axle) in `scuttle`, `wing-l/r`, `side-wall-l/r` instead of four 45° chords (sagitta 28 mm, one feature; now 12 mm).
- **Thermal:** 1:50 `th-nose-sheet/-paint` replaced by sheet and paint solids for `nose`, `scuttle`, `wing-l`, `wing-r` (sheet 0.08 mm inside the paint, as before); 1:100, 1:200, 1:500 `th-scuttle`, `th-wing-l/r`, `th-nose` in `body-panels`. The hidden `engine-bay` solids stay solids of their own (the rule forbids a hidden region to reuse a body solid; tried, reverted). The thermal sheet with the bonnet open shows the engine region in the bay.
- **Structure:** unchanged this pass (the body rails now coincide with the structure's front rails).
- **Rig, poses, clips, 2D:** unchanged. The 2D drawings were checked against the new form: in `parked` nothing visible moved; the 2D bonnet-open plan and elevation agree with the 3D bonnet (plan: bonnet from 0.94 to 1.69 m).
- **Decisions:** one sentence pair per scale (bay), one more at 1:200 (arches).

### Validation

- After the bay: `r3-validate.ts vehicles/city-car --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; published with `volumes-insert.py`.
- After the arches: the same line `VALID`; published again. The final `--catalog-only --volumes` line is in the returned status of this pass.

### Remaining, plainly (for the next pass)

1. **Rig, all scales: no door bones.** Four doors need bones hinged about a vertical axis at x ≈ 0.72 m (front, at the A-pillar foot) and x ≈ -0.20 m (rear, at the B-pillar), z ±0.82, with `twistDeg: ±90` so the joint axis is vertical, limits 0…68°. Then at every scale: `side-wall-l/r` split into rear quarter, hinge pillar stub, front and rear door panel (the side wall is linear between its stations, so a cut at any x keeps the face), a B-post below the belt, the side glass, handles, mirror (front door) and shut-line capsules rebound to the door bones in bone-local coordinates, thermal copies, a door-open pose with its complete `thermal.regions`, 2D shapes bound to the door bones in three views, and at 1:500 the `hull` split for the door openings. Not started: it does not fit into what was left of this pass, and half of it would leave the catalog invalid.
2. **Rig, all scales: no steering.** `wheel-f` is one bone for both front wheels turning about the lateral axis only; steering needs a kingpin bone per side (vertical axis, ±35°) between `axle-f` and a wheel bone per side, and the front wheel solids, knuckles, calipers and discs rebound.
3. **Structure, all scales:** the powertrain (about 130 kg) is not in the load path: no engine mounts as members and no mass marker of its own (it is inside the 850 kg chassis point mass). Members remain tube idealisations of the pressed body-in-white sections.
4. **Thermal 1:50 / 1:100:** the new `radiator`, `battery`, `air-box`, `valve-cover` and rails in the bay have no thermal solid; the hidden region `engine-bay` is still block, gearbox and manifold only (adding the radiator needs `thermal.regions[engine-bay].areaM2` re-derived at the root).
5. **Body 1:50:** the bay has no undertray, no strut-tower domes on the inner wings, no bulkhead detail (brake servo, wiper motor), no hoses; the wings' inner faces are plain. Headlamps are 90 mm slits (real units are 150–200 mm tall); enlarging them needs the three 2D views redrawn at 1:50, 1:100, 1:200. Seat cushions are rounded boxes without bolsters. The bonnet lies on the wings (10 mm proud) instead of flush between them.
6. **Body 1:500:** `hatch-open` still shows the filled hull behind the tailgate (no boot opening); the arches are four 45° chords (sagitta 28 mm, under the 75 mm feature, so left).
7. **Body 1:100:** no lower intake and no number plates (present at 1:50 only); not judged this pass beyond the sheets listed above.
8. The sheets listed under "not read" above.

### Fundamental issues (outside this asset)

- `T/validator.md` has no section "Rule changes authors must know" and `T/rig-lateral-rest.md` does not exist, although the author task refers to both; authors cannot tell which rule defects are open or how laterally resting bones are meant to be used in 2D (what `evaluate` draws for a bone turning about a vertical axis in elevation, front and plan).
- No tool tiles several sheets into one image (no PIL, ImageMagick or `rsvg-convert` on the host): every sheet costs one read, which is what limits how many of the 12 scale × list sheets plus poses an author can look at per pass.

## Repair by the umbrella finisher, 2026-11-22 22:15

Found by the catalog sweep of 22:10: `INVALID … validateAsset 8`, all `volumes-feature`: the 12 mm plain brake discs (`rl-disc`, `rr-disc` at 1:100 and 1:200, all four at 1:500) are below the smallest feature of their scale (the per-scale feature rule of 2026-11-22; the asset was `VALID` when pass 1 reported).

`rework-city-car-coarse-discs.py` (new, one-shot on the volumes source): rear discs 15 mm at 1:100 and 30 mm at 1:200 (as the front discs already are), at 1:500 each brake one 75 mm body between the faces of the disc hat (−45 to +30 mm), radii 75 to 150 mm unchanged; one decision sentence per scale (en, de). Published with `volumes-insert.py`.

Proof: `r3-validate.ts vehicles/city-car --volumes` and, after publishing, `--catalog-only --volumes` → `VALID vehicles/city-car (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ city-car (1/1)`, four scales suitable. Sheet read: 1:500 body six views (the brakes stay hidden behind the wheel discs; the car reads as before). Rendered, not read: 1:100 body `--cut`. The open list of pass 1 (steering bones and the rest) is untouched.
