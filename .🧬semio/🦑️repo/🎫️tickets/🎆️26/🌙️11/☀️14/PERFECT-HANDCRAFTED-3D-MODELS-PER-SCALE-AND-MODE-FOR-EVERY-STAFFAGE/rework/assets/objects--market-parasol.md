# objects/market-parasol — running report

Earlier work on this asset: `T/rework/15.md`, section 3 (taut gores, crank below the knuckle, two mast bones). Source and catalog were identical when this pass started (`volumes` equal, 101 / 73 / 70 / 68 body solids).

## Pass 1 (second round), 2026-11-22

### Reference table (round crank parasol with collar or push-button tilt, 2.5 m, eight ribs, slab base)

| quantity | real | model before this pass | verdict |
|---|---|---|---|
| canopy | o2.5 m class, octagon of 8 gores, pitch 12 to 18 degrees, taut, hem 1.95 to 2.10 m above ground | o2.59 over rib tips, pitch 15.3 degrees, hem 2.0 m | ok (fixed in batch 15) |
| ribs / stretchers | 8 ribs 16 x 10 mm, 1.3 to 1.4 m; stretchers o11, 0.5 to 0.6 m, meeting the rib at 35 to 45 % of its length | ribs 1.3676 m, stretchers 0.55 m at 0.50 m (37 %) | ok |
| crown hub / runner | hub o80 to 110 with finial; runner sleeve o65 to 75, travel 0.6 to 0.7 m | hub o110, runner o72, travel 0.654 m | ok |
| mast | two-piece aluminium pole o48 below / o38 to 40 above, coupling at 1.0 to 1.2 m, overall 2.3 to 2.5 m | o48 to 1.07 m, o40 above, overall 2.46 m | ok |
| crank | winch housing with crank at 0.9 to 1.1 m, on the lower pole | 0.92 m | ok |
| tilt pivot | the knuckle sits in the upper pole 0.1 to 0.25 m below the open runner (about 1.75 to 1.85 m above ground); only the canopy head tilts, 20 to 35 degrees; the furled runner slides down over the straightened knuckle; the collar or button that releases it sits above the crank | knuckle in the coupling collar at 1.07 m, limit +-10 degrees: the whole upper 1.3 m of the mast leaned over like a broken pole | WRONG pivot |
| mast foot | pole stands 0.25 to 0.35 m deep in a welded socket tube (o55 to 60) on a flange, clamped by one or two knob screws | mast foot lug on a single 12 mm cross pin in a clevis 60 mm above the plate, joint declared fixed | WRONG load path: one pin is a hinge, the mast would fall over |
| ballast | slab 0.45 to 0.55 m square, 80 to 100 mm thick, 40 to 60 kg (low, wide: tipping arm) | block 0.40 x 0.30 x 0.20 m, 57.6 kg | wrong proportion (tall narrow block, short tipping arm across) |
| furled | ribs hang along the mast, tips about 1.0 m above ground, cloth gathered between the ribs | as real | ok |

### Rebuilt (`rework-market-parasol-socket-knuckle.py`, hand-chosen numbers in its docstring)

- **Rig.** `mast` is the whole standing pole (1.54 m, 2.5 kg) fixed in the socket; `mast-upper` is the mast head above the knuckle (joint at 1.80 m above ground, 0.55 m, 0.7 kg, revolute +-30 degrees); `crown` and `runner` re-seated on it without moving in the world (runner 0.1574 m above the knuckle when open, furled it slides below the knuckle along the straight mast). Pose `tilted-to-sun`: 20 degrees at the knuckle (was 8 degrees at mid mast); clip `wind-flap` gust key 3 degrees of knuckle play (was 9.5); `sun-tilt` text. Anchor `socket` footprint 0.5 x 0.5 m; base mass centre 0.048 m.
- **Body 1:50.** Concrete slab 0.50 x 0.50 x 0.096 m (57.6 kg as before) with 12 mm arris, steel flange 200 x 200 x 10 with four bolts, hollow socket tube o60 x 5.5 from 0.106 to 0.40 m with a rim ring, four 8 mm gussets, two clamp knobs; o48 pole from inside the socket (0.11 m) to the coupling, o40 pole (`mast-middle`) on to the knuckle; knuckle barrel o56 x 48 across the mast with its pin; mast head o40. Removed: clevis flange, two cheeks, foot pin, two nuts, mast lug.
- **Body 1:100 / 1:200 / 1:500.** Slab; socket as a solid o60 (1:100, with the 15 mm flange) or o70 (1:200) cylinder; one mast solid from the socket to the knuckle; knuckle barrel o56 (1:100) and o60 (1:200), none at 1:500; mast head 0.53 m. At 1:500 the 0.52 m mast head is drawn 75 mm thick (it is under the canopy and too short to stay a declared slender member).
- **Structure.** 1:50: pole tubes with their real wall as members from the socket up, socket and gussets as members of the base, clamp as the joint mast/base, knuckle barrel and pin as the joint mast/mast head, masses at the new centres, support plate 0.5 x 0.5. Coarser scales alike (1:100 joint capsule moved to the knuckle).
- **Thermal.** Lists follow by reference; removed parts dropped, new parts added to `frame-alu`; `base-concrete` area 0.69 m2 (slab top, bottom and four edges), characteristic length 0.5 m, horizontal plate.
- **2D.** Elevation and front at all four scales: slab, flange, bolts, socket with gussets and clamp knobs (1:50), mast drawn in one piece from the socket to the knuckle with the o48 / o40 step, knuckle as circle (side) and barrel (front), mast head.

### Rendered and read

- Before: 1:50 body open, `furled`, `tilted-to-sun` (the broken-pole tilt and the pin foot).
- After (candidate file): 1:50 `tilted-to-sun` six views: only the canopy head tilts under the runner, the pole stands in the socket; `furled` three-quarter: slab, flange with bolts, gusseted socket with two knobs, coupling collar, crank at the hem of the hanging cloth.
- After publishing (catalog): 1:50 body three-quarter; clips `wind-flap` at 1.5 s and 4.5 s (3 degrees of knuckle play at the gust key) and `sun-tilt` at 2 s and 4 s; 1:50 structure (right side) and thermal with `--cut`; 1:100 body six views, structure six views, thermal, `furled`; 1:200 body `tilted-to-sun` six views, structure, thermal, `furled`; 1:500 body six views, `furled` six views, structure, thermal. 2D (`r3-2d`): 1:50 open and `tilted-to-sun`, 1:100, 1:200. Read: 1:50 tilted (3D six views, 2D), 1:50 furled three-quarter, 1:50 structure, 1:50 thermal cut, wind-flap 4.5 s, 1:100 body and structure, 1:200 tilted and thermal, 1:500 body, furled and structure, 2D 1:50 tilted and 1:100. Rendered but not opened for lack of budget: sun-tilt 2 s / 4 s, wind-flap 1.5 s, 1:100 thermal and furled, 1:200 structure and furled, 1:500 thermal, 2D 1:50 open and 1:200.
- 2D correction after reading the tilted front elevation: the knuckle was drawn over the cloth; its layer is now just above the mast head (`rework-market-parasol-knuckle-z.py`).

### Texts (`rework-market-parasol-texts.py`)

`volumes[scale].decisions` rewritten per scale (what is modelled, what is merged or left out and why; structure sentence; tilt sentence), en and de. Summary, provenance notes, representation decisions and thermal texts: every mention of block, cover plate, clevis, pin, crank at 1.38 m and 8 degree tilt replaced by slab, flange, socket tube, knuckle, 0.92 m and 20 degrees; the script lists stale words and ends with none.

### Validation

`r3-validate.ts objects/market-parasol --catalog-only --volumes` (after the last change): `VALID objects/market-parasol (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. Source `T/volumes/objects/market-parasol.json` equals the catalog `volumes` (body 101 / 74 / 71 / 68 solids).

Findings on the way: 1:500 mast head 0.52 m at o44 failed `volumes-slender` (12:1), so it is 75 mm thick there; 1:100 old pin capsule moved to the knuckle stuck 22.5 mm out of the body and left the load path open, replaced by a joint on the knuckle barrel.

### What remains imperfect

- Canopy at 1:100, 1:200, 1:500 keeps the 48 facet solids of the 1:50 cloth (6 per gore, needed for the furled pose) and the true-size ribs and stretchers as slender bars; only hubs, mast and base are simplified per scale. Not re-authored in this pass.
- The furled cloth is six stiff facets per gore (a star in plan), not gathered and strapped cloth; no tie strap.
- No wind vent cap, no valance, no rib pockets at the tips, no cord from winch to runner (it runs inside the pole; the structure list says so in its decision text); the winch housing is a plain rounded box.
- The knuckle is a plain barrel with a pin; no fork and tongue, no release button; the runner sleeve does not show the slot that passes the knuckle.
- Coupling collar and its knob keep the id `mast-tilt-collar` (it is the collar-tilt release ring and pole splice).
- Plan drawing dimension reads 2.55 to 2.60 m against `dimensions.widthM` 2.59 (octagon turned 11 degrees), unchanged.
- Thermal: `base-concrete` layer still says 51 mm thermally active for a 96 mm slab (48 mm to the mid-plane); `frame-alu.areaM2` 1.12 was not recomputed for the socket tube and passed the measured check.
- 2D at 1:50: the two clamp knobs are drawn in the side elevation only (in the front they point at the viewer and are not drawn as circles).
- `r3-validate --volumes` takes about 15 minutes for this asset (48 cloth solids times three lists times four scales), so only two full runs were possible.

### Fundamental issues (outside this asset)

- Sheets rendered with `r3-sheet.ts --file` into `🗑️generated/r3-sheet/file/` disappeared within about a minute while another agent cleaned that folder; outputs have to be copied out at once (`rework-market-parasol-render.sh` does).
- `python3` on the host has no PIL, so several single-view sheets cannot be tiled into one image for reading.
