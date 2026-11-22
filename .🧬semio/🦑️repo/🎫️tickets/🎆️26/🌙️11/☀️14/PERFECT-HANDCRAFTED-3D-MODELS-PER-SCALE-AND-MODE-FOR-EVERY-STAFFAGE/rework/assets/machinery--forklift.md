# machinery/forklift — rework report

Earlier work: `rework/10.md`, section 3 (author 10: wheels of 1:100, 1:200, 1:500 turned per scale; it left 18 `volumes-connected` findings and the missing steering joint open). No report of this file existed before.

## Pass 1 (second round), 2026-11-22

### State found

- Source `volumes/machinery/forklift.json` (01:14) was newer than the catalog (21 Nov): the wheel rework of author 10 was only in the source. The source was the better state; it was kept, worked on and published, so catalog and source agree again.
- `r3-validate.ts --volumes` at the start: INVALID, 18 findings, all `volumes-connected` (floating armrest, steering cylinder, sheave, carriage, steering wheel, rear wheels, mast at 1:500; open load paths at all four scales).

### Reference (2.5 t electric counterbalance truck, Jungheinrich EFG 425 / Linde E25 class, duplex mast 3.1 m)

| quantity | real machine | model | verdict |
|---|---|---|---|
| length to fork face / width / guard height | 2.35–2.45 / 1.15–1.25 / 2.15–2.25 m | 2.441 / 1.20 / 2.24 m | ok |
| wheelbase | 1.55–1.72 m | 1.72 m | ok, upper end |
| drive tyre 23x9-10, steer tyre 18x7-8 | 0.59 x 0.225, 0.455 x 0.18 m | 0.5915 x 0.225, 0.455 x 0.18 m | ok |
| drive unit | two motors of about 260 mm diameter on the axle line, gear and brake housings through the frame plates, hub flange into the rim | none in the body (thermal cores only), wheels hung in the air | **defect, rebuilt** |
| steer axle | pendulum axle on a centre pivot under the counterweight, king pins, stub axles into the hubs, double-acting cylinder with links to the steering arms | beam, king pins and 70 mm stubs ending 95 mm short of the rim disc, cylinder 15 mm clear of the beam, no pivot | **defect, rebuilt** |
| mast pivot | brackets on the frame, 60 mm pins | present at 1:50; missing at 1:100 and 1:200 (the mast hung on the tilt rods only), mast 18 mm clear of everything at 1:500 | **defect, rebuilt** |
| lift chains | from the carriage anchor over a sheave at the inner mast head and down behind it | a straight bar in front of the sheave, one sheave 50 mm off its chain (mirror error), sheaves 15 mm under the head without carrier | **defect, rebuilt** |
| lift cylinders | stand on the lower cross member of the outer mast | 10 mm clear of the rails, no seat | **defect, rebuilt** |
| mast and carriage rollers | fill the channel (0.5 mm play) | 2 to 2.5 mm play on both sides, absent at 1:100 | **defect, rebuilt** |
| fork arm | 1150 x 120 x 45 mm | shank 40 mm, pallet 5 mm clear of it | **defect, rebuilt (45 mm)** |
| overhead guard | legs welded to frame and counterweight | rear legs 8 to 13 mm above the rounded counterweight top | **defect, rebuilt** |
| armrest, steering wheel | armrest on an arm from the seat frame; wheel on its hub | armrest 30 mm beside the seat without carrier; at 1:100 the wheel ring without hub or spokes, at 1:200 7 mm off the column | **defect, rebuilt** |
| steering | rear wheels turn about the king pins | no joint in the rig | **open** |

### Rebuilt (scripts `rework-forklift-p1-50.py`, `-coarse.py`, `-fix.py`, `-2d.py`, `-rig.py`; ruler `rework-forklift-bbox.ts`)

- **1:50 body** (156 → 168 solids): drive axle housing (motor barrel 260 mm, two gear housings 220 mm through the frame plates, hub flanges 140 mm into the rim discs); armrest arm; steering cylinder lugs, steering arms to the king pins, stub axles into the rim discs, pendulum pivot block and pin; sheave mirror corrected, two yokes per sheave under the inner head, chain as one extruded loop from the carriage anchor over the sheave to a rear anchor; cylinder seats; rollers 84 / 85 mm; pivot pin shortened so it no longer touches the tyre; rear guard posts let 40 mm into the counterweight; fork shanks 45 mm.
- **1:100** (122 → 143): the same parts sized for 15 mm (sheaves on one axle with a single hanger instead of 8 mm yokes, steering wheel as ring on a dished plate with hub), plus the pivot brackets and pins, king pins, stub axles, rollers and chain anchors that this scale lacked.
- **1:200** (62 → 67): axle barrel with hub flanges, pivot brackets, carriage side plates in the inner rails, stub axles, pendulum block, cylinder bolted to the beam, column into the wheel, armrest on the seat side.
- **1:500** (38 → 43): axle barrel, mast foot block on the axle line, tilt lug base and mast lug (the tilt rods ended 100 mm beside the rail in the air), pendulum block.
- **Thermal**: hidden regions `drive-motor` and `brakes` keep their own core solids inside the new housings (the rule forbids reusing body solids); mirror errors of the cores at 1:50 and 1:100 corrected (second motor and second gear housing lay inside the first). New body parts are in `chassis-paint`, `mast`, `fork-gear`. Areas stay inside the 15 % band.
- **Structure**: load path closed at all four scales: tyre patch → tyre → rim → hub flange → axle housing → frame plates; guard posts and tilt lug bases through fender decks and floor plate; tilt lug on the mast; mast through pivot bracket and pin; lift cylinder on its seat; carriage through rollers (1:50, 1:100) or side plates (1:200) into the inner rails; steer axle through pivot block into the counterweight and through king pins and stub axles into the wheels; at 1:500 the counterweight casting and the pallet are members.
- **2D** (1:50, 1:100, 1:200): axle housing drawn in elevation, front and plan (it shows in plan in the slot between front frame and mast); chain redrawn over the sheave in elevation and front (1:50, 1:100); fork shank 45 mm in elevation and front (1:50, 1:100).
- **Decisions**: one paragraph per scale, en and de.
- Rig and dimensions: unchanged. A 5 mm shift of `pallet-x` was tried and taken back (it made the 1:500 fork print below 0.2 mm); the pallet meets the forks through the 45 mm shank instead.

### Validation

`r3-validate.ts machinery/forklift --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0` (before publishing). Published with `volumes-insert.py`; catalog and source identical. Silhouette after the redraw: 1:50 IoU 0.997 / 0.997 / 0.996, p95 1.9 / 1.9 / 7.5 mm; 1:100 0.996 / 0.997 / 0.995; 1:200 0.992 / 0.993 / 0.993.

### Rendered and read after the last change

1:50 body (default, lifted-load, tilted-mast, clip drive-with-load t 6 s, underside three-quarter), 1:50 structure, 1:50 thermal cut, 1:100 body, 1:500 body, 2D 1:50, overlay sheet 1:50.

### Remains imperfect (stated plainly)

- **Rendered but not read after the last change**: 1:200 body; thermal and structure sheets of 1:100, 1:200, 1:500; poses approach, travel-accelerated, travel-cruised, travel-stopped; clip lift-pallet (t 2, t 6) and drive-with-load t 1.5; 2D sheets of 1:100, 1:200, 1:500 and of the lifted pose. They pass the measured rules, but nobody looked.
- **Rig**: no steering joint. The rear wheels are one bone `wheel-r`; real steering needs two knuckle bones about the vertical king pins (lateral rest, `rig-lateral-rest.md`), the wheels split into `wheel-r-l` / `wheel-r-r`, and all 2D shapes, thermal references, members and anchors rebound.
- **Chain** (1:50, 1:100): a rigid solid on `mast-inner`. In lifted-load the front run hangs 1.275 m below the risen carriage along the mast face, and the rear run ends at an anchor block on the inner mast that does not exist in reality (the real chain is anchored on the outer mast and changes its run lengths with the lift).
- **Axle barrel** 260 mm diameter touches the lower mast tie at 1:50 (tangent at x = 130 mm); the hidden motor core has the same radius as its housing (coincident surfaces).
- **1:500 drawings** do not show the new mast foot, tilt lugs and pendulum block (silhouette within tolerance without them); **1:200 drawings** do not show pivot brackets and carriage plates.
- **Body form** was not re-sculpted in this pass: battery hood and side panels are flat boxes, the counterweight is a superelliptic loft without the cast recesses, no dashboard display, no hydraulic hoses, no mast hose reel, no seat belt; the roof of 1:500 is a closed slab. Coarse scales other than the parts listed above are as author 10 left them (still many 1:50-identical solids at 1:100).
- Elevation Hausdorff distance at 1:50 is 274 mm at one spot (p95 1.9 mm), not located.

### Fundamental issues (outside this asset)

- The kernel has no solid whose length follows two bones (a chain, hose or cable between a fixed and a moving anchor); 2D has `followTip`, volumes have nothing comparable.
- `r3-validate.ts --volumes` needs about 3 min 40 s for this asset on the shared host; the fast loop was `rework-10-connected.ts` (5 s) and `volumes-check.ts --rules` (40 s per scale).
- `r3-project2d.ts --view plan --bone axle-f` left the motor barrel out of the printed outline (only gear housings and flanges came back).
