# Authoring report: appliances-b (12 assets)

All 12 assets have a hand-authored, spliced and validated `volume` (`volume-insert.py`, verified to change only `volume`). Authored sources: `T/volumes/appliances/<slug>.json`, generated from `T/volumes/appliances/author_<slug>.py` with the helper `_lib.py` (explicit data, each solid read off the 1:50 views). Tools written for this run (kept in `T`): `appliances-b-run.sh`, `appliances-b-validate.ts` (Ajv schema + `validateAsset` + pose/clip evaluation with weights), `appliances-b-solids.ts`, `appliances-b-inspect.ts`, `appliances-b-collect.py`. Tool output is in `T/🗑️generated/authoring-appliances-b/` (`summary.json`, `<slug>-validate.txt`).

Result: 9 assets meet every gate. 3 assets miss gates, all for reasons of the authored data, not of the 3D bodies: `laptop` and `vacuum-cleaner` (bounding box x and y at rest) and `shower-head` (elevation and front IoU).

## Table

bbox error is relative, per axis x (widthM) / y (heightM) / z (depthM) at the rest pose. IoU is elevation / front / plan at 1:50. "Validator issues" = `volume-*` findings of `validateAsset` (all assets: 0 findings in total) and Ajv errors under `/volume` (all 0). Poses checked = every pose of the asset plus each clip at 25 %, 50 % and 90 % of its duration, evaluated with the staffage `evaluate`, solids carried with `carry` (weights blended): bounding box, connectivity of the solid-touch graph, lost contacts.

| slug | solids | bbox error % x / y / z | IoU e / f / p | validator issues | poses checked | remarks |
|---|---|---|---|---|---|---|
| kitchen-extractor-hood | 7 | 0.0 / 0.0 / 0.0 | 1.000 / 1.000 / 1.000 | 0 | off, low, extracting + 2 clips x 3 | filter extrusion with four holes in which the lamp discs sit (2 x 2 lamp grid read from two views); canopy is a solid, not a shell |
| laptop | 10 | **-20.7 / -90.6** / 0.0 | 0.994 / 0.888 / 1.000 | 0 | open, open-idle, closed + 2 clips x 3 | gate miss by construction, see below; open pose bbox = 0.274 x 0.218 x 0.314 m |
| mvhr-unit | 12 | 0.0 / -0.5 / 1.0 | 1.000 / 0.992 / 0.993 | 0 | off, normal, boost + 3 clips x 3 | window wall is a 30 mm plate with opening so the impeller (9 repeated blades read from the path) is visible; Ajv reports 1 error outside `volume` (`thermal/emitters/1/vapourKgPerKg` < 0), not mine |
| pendant-lamp | 6 | 0.0 / 0.0 / -0.7 | 0.997 / 0.993 / 0.993 | 0 | on, dimmed, off + 2 clips x 3 | all six solids are revolves; the shade is a 2.5 mm shell |
| room-thermostat | 5 | 0.0 / 0.0 / 0.0 | 0.998 / 0.949 / 0.959 | 0 | idle, calling, set-high + 2 clips x 3 | dial turns about w in every pose |
| server-rack | 26 | 0.2 / 0.0 / 1.2 | 1.000 / 0.999 / 0.997 | 0 | idle, full-load, off + 2 clips x 3 | one repeated solid per equipment family (six servers, bays, vents, LEDs, ports); door is a solid plate (588 slots not modelled); depths of equipment are estimates |
| shower-head | 22 | 2.6 / -0.6 / 2.2 | **0.688 / 0.751** / 0.857 | 0 | off, running, eco + 2 clips x 3 | IoU miss caused only by the hose, see below; hose end blended to the handset bone |
| split-ac-indoor-unit | 7 | 0.2 / 1.1 / 0.0 | 0.994 / 0.981 / 0.998 | 0 | off, cooling, cooling-low, swing-high, swing-low + 2 clips x 3 | vane swings 0 to -55 degrees out of the recess; Ajv reports 2 errors outside `volume` (`thermal/regions/*/operating/heatW` < 0), not mine |
| thermostatic-radiator-valve | 5 | -1.5 / -0.2 / 5.6 | 0.985 / 0.986 / 0.975 | 0 | open, part-open, closed + 2 clips x 3 | z +5.6 % within the gate, see below |
| toaster | 13 | 0.0 / 0.3 / 2.1 | 0.996 / 0.984 / 0.982 | 0 | idle, toasting, popped + 2 clips x 3 | 16-section casing loft; slices lift with the carriage bone (lost contacts with the top plate in `idle` and the cool-down clip are the lowered slices, intended) |
| tower-server | 12 | 0.9 / 0.0 / 0.8 | 0.995 / 0.997 / 0.991 | 0 | typical, rated, standby, off + 2 clips x 3 | Ajv reports 288 errors outside `volume` (shapes `grille-slits`, `psu-slits`, `intake-slits` of the 1:50 geometry have no `tone`, and further view-shape fields), pre-existing and not mine |
| vacuum-cleaner | 37 | **+73.5 / -59.9** / 3.0 | 0.986 / 0.979 / 0.985 | 0 | parked, running, running-pulled + 2 clips x 3 | gate miss by construction, see below; running pose bbox = 1.502 x 0.762 x 0.308 m |

Ajv "other errors" and `validateAsset` for the three assets above: `validateAsset` returns 0 findings for every one of the 12 assets (including non-volume rules).

## Gates not met and why

1. `laptop`, x -20.7 % and y -90.6 %. `dimensions` (0.275 x 0.218 x 0.313 m) are those of the open laptop (poses `open`, `open-idle`, lid 105 degrees; measured with the volume: -0.165 to 0.109 in x = 0.274 m, 0.218 m high), whereas the rig rest (lid angle 0) and every 1:50 shape are the closed laptop (0.218 x 0.019 x 0.313 m). The volume follows the rig and the shapes; the open poses reproduce the declared dimensions within 0.4 %. Elevation, front and plan IoU pass (0.994 / 0.888 / 1.000).
2. `vacuum-cleaner`, x +73.5 % and y -59.9 %. The rig rest is the straight chain (hose segments, grip, wand, nozzle in a line along x, 2.6 m long, 0.305 m high) and the 1:50 shapes are drawn at this rest; the declared dimensions (1.5 x 0.76 x 0.3 m) are those of the pose `running` (volume measured there: 1.502 x 0.762 x 0.308 m). IoU 0.986 / 0.979 / 0.985 against the rest shapes.
3. `shower-head`, IoU elevation 0.688 and front 0.751 (plan 0.857 passes). The hose is the only cause: it is authored as an open stroked polyline (`penMm` 0.24) in the elevation, front and plan, so the checker (closed body, detail and glazing shapes only) does not count it as drawn area, yet the declared height (1.12 m, hose hanging to y = -0.30) and width need it (without the hose the bounding box misses by -16 % / -28 % / -32 % and the IoU is 0.996 / 0.990 / 0.981). The hose is a 3D curve rebuilt from the shared parameter of the front and plan polylines (12 capsules of the stroke width 12 mm); I kept it and did not thin it to chase the number.

## Other 2D inconsistencies and things a reviewer should look at

- `thermostatic-radiator-valve`: declared depth 0.090 m, but the authored front and plan shapes span 0.095 m (collar 71 mm + knob 24 mm). Volume follows the shapes (+5.6 %, inside the 8 % gate).
- `tower-server`: the plan puts the CPU grille edge at z = +0.092 to +0.1 (left side panel) while the elevation, seen from the right (-z), draws it; the grille is modelled on +z. The PSU vent has different extents in elevation (rear, 26 mm) and plan (top, 90 mm): modelled as a rear plate and a top plate.
- `split-ac-indoor-unit`: the display window is drawn at three different places (elevation y 0.13 to 0.18; front y 0.12 to 0.155, z 0.32 to 0.38; plan z 0.30 to 0.38); front taken.
- `kitchen-extractor-hood`: lamps are drawn at two x positions (elevation) and two z positions (front); read as a 2 x 2 grid.
- `vacuum-cleaner`: `handle-opening` is a closed `detail` shape in the elevation but is a hole in the handle, so the checker reports 0 % coverage for it (IoU unaffected: 0.986). Exhaust grille width is not drawn (estimate 0.18 m).
- `server-rack`: equipment depths, the rear panel and the 15 mm gap behind the door are not drawn (estimates); the door perforation (588 slots) is not modelled; handle protrudes 12 mm (depth 1.012 m).
- Lofts follow the kernel as implemented (section plane given by `rotation`), not the tangent-frame wording of the schema description; see `T/authoring-requests.md` (new file, one entry). Revolve profiles are open polylines (the validator rejects closed loops), hence the pendant-lamp shade runs out and back as an open profile.
- Lost contacts in the pose evaluation are only the intended separations: laptop (lid leaves the deck when opening) and toaster (slices lowered below the top plate in `idle`). No solid left its neighbours in any other pose or clip; the shower-head and vacuum-cleaner chains stay connected (components = 1 everywhere).
