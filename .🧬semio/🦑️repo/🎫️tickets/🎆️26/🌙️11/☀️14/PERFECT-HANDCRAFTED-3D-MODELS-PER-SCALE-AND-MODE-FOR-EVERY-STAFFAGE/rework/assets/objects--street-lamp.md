# objects/street-lamp — running report

## Pass 1 (second round), 2026-11-22

Author: claude-code, one agent, this asset only. Status: **partial** (see "Open").

Scripts of this pass (ticket folder): `rework-street-lamp-shroud.py [--write]`, `rework-street-lamp-chassis.py`, `rework-street-lamp-zoom.py <scale>` (close-up candidates of column top and foot for `r3-sheet.ts --file`; its profile cut was wrong for solid cones and is now a real polygon clip). Earlier in this round another agent ran `rework-street-lamp-edit.py` (foundation head 0.50 x 0.50 m, coarse scales freed of 1:50 hardware, stay kept at 1:500); source and catalog were identical when I started (`volumes` equal), so there was nothing to reconcile.

### Reference (written before any change)

| item | real subject | model before this pass | verdict |
|---|---|---|---|
| column | conical steel lighting column to EN 40, light point 6 m, base diameter 140-170 mm, top 60-80 mm, taper 13-15 mm/m, wall 3-4 mm | cone 170 -> 80 mm over 5.88 m (15.3 mm/m), 4 mm wall, hollow at 1:50 | right |
| inspection door | flush door 400-500 x 85-100 mm, lower edge 0.5-0.7 m above ground, one triangular lock | 500 x 90 mm at 0.67-1.17 m, lock at the top, faces the elevation viewer | right |
| base | flange plate 300 x 300 x 20 mm on four M20 anchors (nut AF 30), bolt circle inside the plate corners, on a concrete foundation head; flared base section welded to the shaft | plate 300 x 300 x 20, nuts AF 30 x 19 at 120 mm from the axes, flare 240 -> 169 mm over 180 mm, head 0.50 x 0.50 x 0.10 m | dimensions right; **flare was bound to the foundation** |
| bracket | clamp-on single arm, outreach 0.8-1.0 m, tube 48-60 mm, diagonal stay 25-30 mm to a band 0.4-0.6 m below | sleeve 100 mm, arm 60.4 x 3 mm x 0.82 m, stay 27 mm to a band 0.455 m below | right |
| luminaire | flat LED lantern, side entry 48-60 mm, 505-600 x 240-280 x 100-160 mm, 4-5.5 kg, flat glass, cooling fins over the LED board, gear hatch | 505 x 274 x 98 mm, 4.5 kg, cover and tray lofts (13 stations), six fins, flat lens, hatch seam and catch | right |
| luminaire tilt | spigot knuckle adjustable about -5 to +15 degrees, set once at installation | `head` joint `fixed` at 0 degrees | simplification, see Open |
| sway | tip deflection under the design gust up to about 4 % of the height (EN 40-3-3 class 1); the shaft bends, it does not hinge | one `pole` bone turning 1.8 degrees about the plate (tip 0.185 m = 3.1 %) | magnitude right, shape idealised, see Open |
| mass | 80-100 kg without foundation | 90.1 kg | right |

### Looked at (every image read at full size)

- 1:50 body: whole sheet with rig, close-up sheets of the top (bracket, luminaire) and the foot, six directions each.
- 1:50 structure top and foot, 1:50 thermal top with `--cut`.
- 1:100 body top; 1:200 body top and foot; 1:500 body top; 1:200 thermal top (`--cut`); 1:500 structure top.
- Poses: `lit-in-wind` whole sheet and foot close-up at 1:50 and 1:100 (before and after the change). `lit-night`, `dimmed-night`, `day-off` have no bone offsets (thermal states only): rendered, identical in form to `dusk-off`, not read one by one.
- Clips: `ignition-at-dusk` t = 500 s (thermal list) read; t = 60 s and `pole-sway-in-gust` t = 2 s / 4 s rendered, form identical to the poses they key, not read one by one.
- 2D: `r3-2d` 1:50 in `lit-in-wind` read; 1:100 default rendered, not read.

### Defects found

1. **`lit-in-wind`, 1:50 and 1:100, body and thermal:** the flared base shroud stood on the `foundation` bone while the shaft turns 1.8 degrees on the `pole` bone: the shaft broke through the shroud wall on the lee side (5.7 mm at the shroud top against a 3 mm wall) and left a step on the other. Visible in the foot close-up.
2. **1:500 structure:** the head member was a separate three-station prism with flat ends inside the luminaire (hexagon in plan) that followed neither the casting nor the 1:200 chassis.
3. Not defects, checked: the yellow stay and arm on the coarse sheets are the renderer's mark for `slender: true`; the 1:200 luminaire outline differs at most 8 mm from the 1:50 loft (feature 30 mm); the flare is left out at 1:200 because it exceeds the shaft by 35 mm only at the rim (mean about 10 mm).

### Rebuilt

- **Rig binding / body / thermal, 1:50 and 1:100:** `base-cover` and `th-base-cover` moved to the `pole` bone (revolve about the bone axis, profile in height above the plate), skirt extended 5 mm into the plate so the sway (rim travel 3.8 mm) opens no joint. Shaft and shroud now move as the one welded part they are. Body decisions (en, de) say so.
- **2D, 1:50 and 1:100:** the `base-cover` shapes of elevation, front and plan are bound to `pole`; the elevation outline is redrawn in the bone frame with the same stations as the solid, the front outline takes the skirt.
- **Structure 1:500:** `member-head-chassis` references the head loft (at a 75 mm feature the die-cast body is the member); decision text updated.
- Rig bones, limits, poses and clips unchanged.

### Validation

`r3-validate.ts objects/street-lamp --catalog-only --volumes` after the last publish: `VALID objects/street-lamp (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### Open (for the next pass)

- **Rig, all scales:** no tilt joint at the luminaire. A true one needs a knuckle: arm tube ending at the pivot, spigot socket on the arm, `head` origin at the pivot, limits -5 / +15 degrees; this moves every head solid and 2D shape by 0.10 m. Not started.
- **Rig, all scales:** the sway is one rigid rotation at the plate. A shaft of three or four bones with growing rotation would show the bending line; needs the cone split per bone in all three lists and the 2D pole shapes per bone. Not started.
- **1:50 structure:** the head member is the tray alone; the spigot socket casting and the cover that close the section are not members. Judged acceptable, not changed.
- **1:50 thermal:** the plume emitter is a 3 mm curved plate lying on the fin tops; it reads as a lid over the fins. Not changed.
- **1:200 body:** base plate is 30 mm thick instead of 20 mm (thickened to the feature by the earlier pass, stated in the decisions). Not changed.
- **Not read after the last change:** 1:100 and 1:200 structure, 1:100 and 1:500 thermal, 1:500 foot, the 2D sheets of 1:100, 1:200, 1:500, and the three thermal-only poses one by one. The last change (1:500 chassis reference) was rendered but its picture not read.

### Fundamental issues (outside this asset)

- `sharp`-based montage scripts in the ticket folder (`r3-street-lamp-montage.ts`, also seen with `rework-standing-adult-montage.ts` of another author) hung at 0 % CPU under host load 27; no PIL / ImageMagick on the host, so several sheets cannot be tiled into one image.
- `r3-sheet.ts` has no zoom or region option: a 6 m column is 12 px wide on the sheet, so tall slender assets can only be judged through cut candidate documents (`rework-street-lamp-zoom.py`).
