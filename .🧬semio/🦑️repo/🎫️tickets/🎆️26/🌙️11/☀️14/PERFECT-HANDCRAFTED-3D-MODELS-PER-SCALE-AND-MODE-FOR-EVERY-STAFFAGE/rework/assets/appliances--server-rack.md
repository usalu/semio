# appliances/server-rack — running report

Source `T/volumes/appliances/server-rack.json`, catalog `S/🗂️catalog/🔌️appliances/server-rack/🔣️.json`. Edit scripts: `T/rework-server-rack-coarse.py` (earlier round, coarse lists), `T/rework-server-rack-blanks.py` (this pass).

## Pass 1 (second round) — 2026-11-22

### State found

- No report of this asset existed; `rework/16.md` section 2 says "nothing changed", `--volumes` not finished. A later agent ran `rework-server-rack-coarse.py` at 04:10 (1:100, 1:200, 1:500 lists remodelled for their feature size, 2D redrawn) and left no report.
- Source and catalog `volumes` were identical in all twelve lists (50/100/200/500 x body/thermal/structure), so nothing to reconcile.

### Reference table (42U network/server cabinet, 600 x 1000 mm, EIA-310-E / IEC 60297, e.g. Rittal TS IT, APC NetShelter SX)

| Feature | Reality | Model before this pass | Verdict |
|---|---|---|---|
| Envelope W x D x H | 600 x 1000..1070 x 1991..2000 mm | 0.6 x 1.0 x 2.0 m | right |
| Rack unit, usable height | 44.45 mm, 42U = 1866.9 mm | equipment bone 1.8669 m from 0.10 m | right |
| 19-inch mounting | panel width 482.6 mm, clear opening 450 mm, hole centres 465 mm | fronts 482.6 mm, rails at +-225..244.5 mm | right |
| Frame | welded or bolted frame of four corner posts 35..45 mm, base and top rings, depth rails carrying four adjustable 19-inch rails | posts 35 x 40, rings 30 mm, depth rails, four rails | right |
| Side panels | 1.2 mm sheet, often split, removable | two halves per side with mullion, louvres at rear top | right |
| Front door | perforated sheet, 5 mm holes, 63..80 % open, swing handle, three hinges, opens 120..180 degrees | 1.5 mm sheet with 25 mm return, handle, lock, three hinges; perforation generalised to about 65 mm apertures; limit 100 degrees | perforation is an abstraction (stated below); limit too tight |
| Rear doors | split perforated leaves, open 120..180 degrees | two leaves on outer pins, limit 95 degrees | limit too tight |
| Roof | fan tray with two to four 120..172 mm fans, brush cable entry | two 172 mm fan housings with hub and guard, brush slot | right |
| Equipment | 2U server 87 x 443 x 700 mm, 4U storage, 1U switch, patch panel, brush panel; unused units closed by blanking panels (air management, ASHRAE TC 9.9) | storage U1-4, six 2U servers U5-16, patch/brush/switch U40-42; **U17-U39 open** | **defect: 23 empty units without blanking panels**, one sees through the rack and cold air would bypass the servers |
| PDU | two vertical zero-U strips 55 x 50 x 1650 mm at the rear | present | right |
| Casters | four swivel casters, 50 mm wheel, top plate, race, fork, axle | plate, race, crown, two cheeks, wheel; **no axle** | **defect: wheel hangs between the cheeks without axle** |
| Levelling feet | four M12 feet with 40..50 mm pads, screwed up when the rack rolls | **missing** (2D decisions say "omitted, screwed up behind the base frame") | **defect** |
| Mass | 110..130 kg empty, 234 kg here with 120 kg equipment | 234 kg | right |

### Rebuilt in this pass

- **Body 1:50**: `blank-2u-front` (eleven 2U panels 88.1 x 482.6 x 8 mm, U17-U38), `blank-1u-front` (U39), their quarter-turn fasteners on the 465 mm hole line (`blank-*-fastener-a/b`, texture), `caster-axle-f/r` (12 mm axle through both cheeks and the wheel bore), `level-foot-f/r` (turned: 50 mm pad with chamfer, M12 stud, lock nut under the base frame, pad 12 mm above the floor, 85 mm inboard of the casters).
- **Body 1:100**: the same twelve panels at 15 mm thickness, feet as 50 mm pad on a 15 mm stud. **1:200**: one `blank-field` (1.022 x 0.4826 x 0.03 m) between server block and top group, feet as 50 mm cylinders. **1:500**: one 75 mm `blank-field` flush with the server block front; feet left out (50 mm < 75 mm).
- **Thermal**: `th-blank-2u`, `th-blank-1u` (1:50) and `th-blank-field` (coarser) as 1.5 mm sheets of region `blanks` (layer `blank-sheet`), axle and feet as references in region `feet`.
- **Structure 1:50**: `member-caster-axle-f/r` and `member-caster-wheel-f/r` added, so the load path plate - race - crown - cheeks - axle - wheel - support print is complete (the wheel was not a member before).
- Script: `T/rework-server-rack-blanks.py` (hand-chosen numbers; `--candidate` writes `🗑️generated/r3-server-rack/🔣️.json`, `--catalog` splices `rig`, `representations`, `thermal` into the catalog file, other members byte-identical).
- **Body 1:500, second fix**: `top-ring` (75 mm ring under the roof sheet). Before, a 32 mm slit under the roof was open right through the cabinet in the door view.
- **Rig**: topology kept (frame, feet, equipment, door-hinge with door, two rear leaves, all on vertical pins turned by `twistDeg`, which is the real pivot). Limits widened to what the hinges allow: front door 0..-130 degrees (was -100), rear leaves 0..+-130 degrees (was 95). Poses and clips unchanged.
- **2D**: elevation (door view) gets the panels behind the door field (`blank-17`..`blank-39` with fastener circles at 1:50, twelve rects at 1:100, one `blank-field` at 1:200 and 1:500); front view (cabinet side) gets `level-foot-front/rear` at 1:50, 1:100, 1:200. The feet stand 0.30 m from the centre (first try 0.34 m failed `PRINT range-end-gap` against the caster fork at 1:150).
- **Catalog facts**: `thermal.regions.blanks.areaM2` 0.043 -> 0.545 (measured 0.550 / 0.541), `feet` 0.072 -> 0.095 (measured 0.104 / 0.098), both titles extended; decisions of every scale (2D `retained`, volumes `decisions`) say what was added; the stale 1:50 text "20 mm holes at 28 mm pitch" corrected to the drawn 52 x 57 mm apertures with 10 mm webs.

### Rendered and read after the changes

1:50 body (front close-up, service-open three-quarter, `service-access` t = 2 s), 1:50 structure, 1:100 body, 1:200 thermal, 1:500 body (before and after the ring), 2D 1:50 and 1:500. Rendered without `NOT DRAWN` but not read: 1:50 thermal, 1:100/1:200/1:500 thermal and structure (except 1:200 thermal), 1:200 body, `service-open` at 1:500, `service-access` t = 6 s, 2D 1:100 and 1:200. `idle`, `off`, `full-load`, `load-ramp` and `power-up` move no bone (same geometry as the default pose).

### Validation

`r3-validate.ts appliances/server-rack --catalog-only --volumes`: `VALID appliances/server-rack (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (about 3 to 5 minutes; earlier 17 minutes under load).

### What remains imperfect

- 1:50 body, `door-field` / `rear-door-a/b`: the 5 mm perforation is a grid of 52 x 57 mm apertures with 10 mm webs, so the doors read as cage doors rather than perforated sheet. A finer grid (about 20 mm) means some 1200 holes per leaf in 3D and 2D and was not attempted (validation time, 2D redraw).
- 1:50 structure: `mass-post-a/b` and `mass-front-leaf`, `mass-rear-a/b` reuse the member solids as mass markers (posts and door sheets show magenta); no mass marker for patch panel, PDUs, blanking panels (about 6 kg) and roof fans; side panels and roof are not in the load path (correct) but carry no mass marker either.
- 1:200 and 1:500 structure were not re-read; 1:200 structure has no wheel/axle distinction (caster is one block, acceptable at 30 mm).
- Thermal: the servers are only their front bezels (region `servers` is declared as fronts); chassis, PDUs behind and the hot rear are not solids of the thermal list. 1:200 thermal side sheets end 27 mm under the roof (visible white band in the side view).
- Equipment detail at 1:50: no rear faces of the servers (PSU, fans, sockets), no cabling, no slide rails; side panel latches and the split-panel lock are missing; the door handle is a plain bar.
- 2D: the pads of the levelling feet stand 7 mm proud of the caster cheeks in the door view and are not drawn there; plan unchanged (feet hidden under the cabinet).
- The cabinet door faces the asset's right (-z), so the product's "elevation" is the door view and "front" the cabinet side: consistent in 2D and 3D, but unusual.
