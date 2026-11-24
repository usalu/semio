# 🪞️ Multiview Fixer Report: Batch MS (In-Band Derived Views)

Written 2026-11-24 by the fixer of `U/🧰️matrix/batches/MS.tsv`. Every number was measured in this session. `S` is `🎓️teaching/🏛️architecture/🧍️staffage`, `T` is `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder, `W` is `U/🧰️matrix/MS`.

## Result

| Asset | Decision | Hard findings before | Hard findings after | State |
|---|---|---|---|---|
| `ceiling-diffuser` | allowlist `front` | 3 | 3 until the allowlist is merged | asset untouched |
| `decentralised-hrv-fan` | redrawn (1:50 front, plan) | 1 | 0 | clean |
| `pendant-lamp` | repaired (solids) and allowlist `front` | 7 | 3 until the allowlist is merged | clean but for the allowlisted view |
| `used-park-lantern` | allowlist `front` | 1 | 1 until merged | asset untouched |
| `used-playground-roundabout` | allowlist `front` | 2 | 2 until merged | asset untouched |
| `waste-bin` | allowlist `front` | 1 | 1 until merged | asset untouched |
| `boxwood-ball` | allowlist `front` | 1 | 1 until merged | asset untouched |
| `european-larch` | redrawn (1:500 body and three views) | 1 | 0 | clean |
| `norway-spruce` | allowlist `front` **proposed, against the list of the task** | 2 | 2 until decided | asset untouched |
| `windmill-palm` | allowlist `front` **proposed, against the list of the task** | 1 | 1 until decided | asset untouched |
| `saucer-magnolia` | anchor repaired; 1:500 front **open** | 3 | 1 | open |
| `rowan` | 1:200 front and plan redrawn; rest **open** | 20 | 8 | open |

Sum of the scan over the twelve assets: 43 hard findings before, 23 after (`[scan] 12 assets: 2 clean, 10 with a hard finding, 23 hard findings`); 14 of the 23 are the allowlisted views (11 in the six agreed assets, 3 in the two proposed ones, see below), 9 are open in rowan (8) and magnolia (1). The lamp's 7 before are 3 derived views and 4 pose findings of `sync-bone-height`.

"Hard findings" are those of `T/🔎️scan.ts` (poses of every authored scale, one per code, view, bone or anchor and scale). Before: the scan of this session before the first edit (`catalog-scan.ts`, the same functions) and `🔎️scan.ts` for rowan and magnolia. After: `🔎️scan.ts --language en --quiet` over the twelve assets after the last edit.

Skipped, they belong to MB-1 or MB-2 (also listed in `MB-1.tsv` or `MB-2.tsv`): `duct-fan`, `etics-brick-wall-12`, `etics-brick-wall-30`, `glulam-beam`, `solid-brick-wall-1960`, `steel-column`, `tealight`, `used-bollard-light`.

No rule, threshold, test, tool, fixture, engine or schema file was edited. The manifest was not edited.

## Allowlist for the coordinator

`W/allowlist.json`: eight entries `{ asset, view, reason: { en, de } }`, the shape the rule text of `S/🧫️fixtures/🔭️multiview-coverage/🔣️.json` (`allowlists.rule`) states for `derivedViewAllowlist`.

Six follow the task (a body of revolution or a point-symmetric body):

| Asset | View | Measured (front against side elevation) | Physical reason in short |
|---|---|---|---|
| `boxwood-ball` | front | 0 at 1:500 (0.18 to 0.26 at the finer bands) | one sphere of 0.8 m at 1:500 |
| `ceiling-diffuser` | front | 0 at 1:50, 1:100, 1:200 | square plate, round neck: four-fold symmetric about the duct axis |
| `pendant-lamp` | front | 0.009 at 1:50, 1:100, 1:200 | rose, cord, holder, shade, bulb: bodies of revolution about the cord (now `revolved` solids) |
| `used-park-lantern` | front | 0 at 1:500 (0.03 to 0.26 finer) | turned column and lantern on a square foundation head, 0.98 mm wide on paper |
| `used-playground-roundabout` | front | 0.008 at 1:200 and 1:500 (0.04, 0.11 finer) | round platform, ring and four handrails 90 degrees apart |
| `waste-bin` | front | 0.010 at 1:500 (0.06 to 0.11 finer) | square in plan to 10 mm, 1.0 mm wide on paper |

Two are my decision per asset and differ from the list in the task, which names all five trees as redraws. They need the coordinator's yes or no:

- **`norway-spruce` front** (0.0239 at 1:200, 0.0178 at 1:500). The front is not a copy: it is the true view of the body, 13 % narrower than the side (7.41 m against 8.48 m), with an own offset and ellipse per whorl already at 1:200. The check normalises by the 30 m height, so its threshold is 0.9 m of difference in the outline; a whorled conifer on a straight stem does not have that between two sides. Measured with `W/outline.ts` (outer silhouettes only): 0.023 to 0.025 at all four scales; 1:50 and 1:100 pass only through the leading sprays drawn inside the crown (0.051, 0.066). A redraw that passes needs those sprays as solids and shapes at 1:200 and 1:500: about 30 capsules and their three views per scale, with thermal references, monotony against 1:100 (77 solids) and the tier radii brought back from the spray envelope (4.25 m) to the tier itself (3.77 m). Not done.
- **`windmill-palm` front** (0.0279 at 1:200; 0.037 at 1:500, 0.039 at 1:100, 0.049 at 1:50). Front and side are two true views of the 201 solids of the 1:200 body and draw other leaves; the crown of twenty fan leaves in four tiers round one stem is rotationally symmetric by nature. The 1:200 band misses the threshold by 0.002 because it draws three blade outlines and the ribs per leaf, which lie closer to each other than the single blade outlines of 1:500.

If the coordinator refuses the two entries, both assets stay failing with these numbers (spruce 2 scale entries, palm 1) and need the redraw described.

## Per asset

Proof lines are the output of `W/prove.sh` (kept below as printed). "gate" is `U/🧰️matrix/MR/gate.ts`: the same checks over every clip instant and transition of the matrix. "derived" is `W/derived.ts`.

### `pendant-lamp`

- Before: `derived-view-in-band` front at 1:50, 1:100, 1:200 (distance 0); `sync-bone-height` bone `shade` at 1:100 and 1:200 in `swinging` and `swinging-back` (19 mm against a tolerance of 16 mm; 24 findings over the clip instants); at 1:50 the same as reported group drift (18 mm). The moments cue (9) belongs to the shared code and is gone since the other worker's change.
- Cause: the cord bone turns 8 degrees in the pendulum poses and the shade, holder, cord and bulb were flat front shapes (cause D of the diagnosis).
- Changed (`W/fix-pendant-lamp.ts`, 16 shapes): every filled front shape on `cord`, `holder`, `shade`, `bulb` is the side-elevation shape of the same id with `solid: { kind: "revolved", axisBone: <its bone>, axis: "u", axisX: 0, mirror: true }`; the plan disc `plan-shade` is the revolved shade profile. Rose and open detail strokes stay.
- After: gate 0 hard, 0 reported. Scan: 3 findings, all `derived-view-in-band` front (0.00935), the allowlisted view. Matrix: 252 cells, 196 ok, 21 unavailable, 35 fail, all 35 `derived-view-in-band` front (1:50, 1:100, 1:200 and the two alternatives of 1:500, 7 modes each).
- Sheets looked at: 1:50 `swinging` (front shows the shade tilted towards the viewer, plan the shifted disc), 1:100, 1:200 default.

```
pendant-lamp | scan: exit 1 [scan] 1 assets: 0 clean, 1 with a hard finding, 3 hard findings
pendant-lamp | gate (clips, transitions): exit 0 hard findings: 0
pendant-lamp | derived: 3 flagged scale(s)
pendant-lamp | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
pendant-lamp | validateAsset: pendant-lamp: 0 issues, 0 before, 0 new
pendant-lamp | check: ✓ pendant-lamp (1/1); 3 authored scale(s) suitable
pendant-lamp | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
pendant-lamp | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
pendant-lamp | breathing: exit 0  Test Files 1 skipped (1); Tests 290 skipped (290);
pendant-lamp | asset-validation: exit 0  Test Files 1 skipped (1); Tests 83 skipped (83);
pendant-lamp | asset-power: exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
```

### `decentralised-hrv-fan`

- Before: `derived-view-in-band` at 1:50: the plan equals the front under a quarter turn (distance 0). Reported only: group drift of bone `rotor` at 1:100 and 1:200 (86 to 113 mm).
- Cause: seen from outside, the square tray on the round sleeve is the same T from ahead and from above, and 1:50 drew only that (three rects), less than 1:100, which also draws a shaft and a blade.
- Changed (`W/fix-decentralised-hrv-fan.ts`; 1:50 front and plan, 3 shapes each before, 15 after; decisions `silhouette` and `omitted` of 1:50): the tray as floor and rim; inside the sleeve the ceramic core, the three struts, the motor can with its cap and the five blades, each the side-elevation shape of the same id with `solid: { kind: "extruded", sweep: "w", z0, z1 }` over the depth its 1:50 body solid has (read from the body by the script). Three struts 120 degrees apart and five blades 72 degrees apart read differently from ahead and from above, and the rotor parts follow the spin in every pose.
- After: plan against front 0.0734; scan clean; gate 0 hard; 1:50 has no reported drift. The reported drift at 1:100 and 1:200 (flat shaft and blade sections on the spinning rotor) is as before; it is the reported tier and those bands were not flagged, so I left them. The same `extruded` twins would remove it.
- Sheet looked at: 1:50 (`extract`).

```
decentralised-hrv-fan | scan: exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
decentralised-hrv-fan | gate (clips, transitions): exit 0 hard findings: 0
decentralised-hrv-fan | derived: 0 flagged scale(s)
decentralised-hrv-fan | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
decentralised-hrv-fan | validateAsset: decentralised-hrv-fan: 0 issues, 0 before, 0 new
decentralised-hrv-fan | check: ✓ decentralised-hrv-fan (1/1); 3 authored scale(s) suitable
decentralised-hrv-fan | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
decentralised-hrv-fan | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
decentralised-hrv-fan | breathing: exit 0  Test Files 1 skipped (1); Tests 290 skipped (290);
decentralised-hrv-fan | asset-validation: exit 0  Test Files 1 skipped (1); Tests 83 skipped (83);
decentralised-hrv-fan | asset-power: exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
```

Matrix (`W/matrix.sh`): `decentralised-hrv-fan | matrix: exit 0 252 cells {'unavailable': 21, 'group-drift': 189, 'ok': 42} failing cells by check {}`. No failing cell. The 189 `group-drift` cells are the reported rotor drift of the 1:100 and 1:200 representations (their presets, range ends, the upper range end of 1:50, which resolves to 1:100, and the two alternatives of 1:500); 1:50 and its lower range end are `ok`.

### `european-larch`

- Before: `derived-view-in-band` front at 1:500 (0.0242).
- Cause: the drawings were true views, but of a 1:500 body that had lost the individual: two solids of revolution with one common axis offset and one lateral scale (0.8392), so the front was the side squeezed by one factor. The 1:200 body has an own offset and ellipse per whorl (0.76 to 1.10) and passes (0.0373).
- Changed (`W/fix-european-larch.ts`; `volumes.500.body` 3 solids before, 12 after; `volumes.500.thermal` 2 references before, 11 after; the two crown shapes of each view of 1:500; decisions of the representation and of the volume): the eleven storeys of the stepped envelope are eleven solids `crown-01` … `crown-11` on `trunk-2`, each with the offset and ellipse of `tier-01` … `tier-11` of 1:200 and its radii brought from the common envelope back to the reach of its whorl; each storey but the lowest laps 0.08 m into the one below (touching caps left a slit in the guard's raster, measured 0.43 m). Elevation, front and plan are the exact outlines of these solids: storeys 5 to 11 as `crown-sun`, 1 to 4 as `crown-shade` (storey 5 was split at 14.7 m between the two before; it is sunlit as a whole as at 1:200, the boundary is at 13.8 m). Declared dimensions hold: body 9.2227 × 30.0000 × 7.7728 m.
- After: front against side 0.0379; scan clean; gate 0 hard. Silhouette guard of 1:500: IoU 0.998, 0.999, 0.997, largest distance 19, 27, 27 mm of 150 mm.
- Sheet looked at: 1:500 (the storeys now step differently left and right; the plan is an egg, not an ellipse).

```
european-larch | scan: exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
european-larch | gate (clips, transitions): exit 0 hard findings: 0
european-larch | derived: 0 flagged scale(s)
european-larch | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
european-larch | validateAsset: european-larch: 0 issues, 0 before, 0 new
european-larch | check: ✓ european-larch (1/1); 4 authored scale(s) suitable
european-larch | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
european-larch | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
european-larch | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
european-larch | asset-validation: exit 0  Test Files 1 skipped (1); Tests 83 skipped (83);
european-larch | asset-power: exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
```

Matrix: `[matrix] 252 cells: 252 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing` (261 s).

### `saucer-magnolia` (open: 1 finding)

- Before: `anchor-landmark` `tp-shade-3` in the front at 1:50 and 1:100 in `spring-gale` (144 mm against 130 mm); `derived-view-in-band` front at 1:500 (0.0180).
- Changed (`W/fix-saucer-magnolia.ts`, one anchor): `tp-shade-3` lay 0.22 m beside the centre of its own leaf patch `th-tp-shade-3`, at the rim of the shaded core where the front draws a sunlit mass of another bone in front of it. It moved 0.21 m to (0.1914, 2.2738, 0.3912) in the frame of `fol-core`, 0.05 m from the patch centre; `W/anchor-search.ts` measures 0 mm there over every pose and scale.
- After: gate 0 hard; scan 1 finding.
- **Open: `derived-view-in-band` front at 1:500, 0.0180.** The 1:500 body is 16 sunlit masses round four stems, a cap and a core; its outer silhouettes from ahead and from the side are 0.013 to 0.018 apart (`W/outline.ts`), and the drawing is one crown outline and the stems. The threshold is 0.19 m on this 6.5 m tree. No drawing of this body tells the views apart; the body has to take the lopsided masses of the 1:200 body (front against side 0.060 there), then the three views are redrawn. Not done.

```
saucer-magnolia | scan: exit 1 [scan] 1 assets: 0 clean, 1 with a hard finding, 1 hard findings
saucer-magnolia | gate (clips, transitions): exit 0 hard findings: 0
saucer-magnolia | derived: 1 flagged scale(s)
saucer-magnolia | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
saucer-magnolia | validateAsset: saucer-magnolia: 0 issues, 0 before, 0 new
saucer-magnolia | check: ✓ saucer-magnolia (1/1); 4 authored scale(s) suitable
saucer-magnolia | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
saucer-magnolia | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
saucer-magnolia | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
saucer-magnolia | asset-validation: exit 0  Test Files 1 skipped (1); Tests 83 skipped (83);
saucer-magnolia | asset-power: exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
```

Matrix: `saucer-magnolia | matrix: exit 0 252 cells {'ok': 175, 'fail': 7, 'group-drift': 70} failing cells by check {'derived-view-in-band': 7}`. The 7 failing cells are the front of the 1:500 preset in the seven modes; the 70 `group-drift` cells are the reported tier (foliage mass `fol-1`, 165 mm in `spring-gale` at 1:50 and 1:100).

### `rowan` (open: 8 findings)

- Before (scan, 20 hard): `sync-bone-height` 9 and `sync-bone-forward` 3 at 1:200 (foliage masses `fol-5` … `fol-9`, `fol-top`, up to 695 mm in `gale`); `sync-forward` 2 and `sync-height` 2 at 1:50 and 1:100 in `gale` (357 and 392 mm against 200 mm); `anchor-landmark` 3 (`tp-sun-1` plan 433 mm at 1:50 and 1:100, `tp-sun-3` front 225 mm at 1:100, in `gale`); `derived-view-in-band` front at 1:500 (0.0211). Reported: 51 group drifts.
- Cause of all but the last: the foliage masses ride on limbs that the wind and snow poses turn by up to 25 degrees, and their front and plan shapes are flat sheets (cause D). The outlets leave their mass because the sheet does not follow (cause C, second kind).
- Changed (`W/make-rowan.sh`: `🧰️silhouette/R4-P/redraw.ts rowan --scales 200 --views front,plan --slab 0.6 --tones`): front (27 shapes before, 93 after) and plan (23 before, 120 after) of 1:200 are plates 0.6 m apart in depth, one stack per bone and tone, that the bone carries; in the default pose the plates of a part are exactly its silhouette (the real silhouette test passes; measured on the variant with 0.4 m slabs: front IoU 1.000 and 0 mm, plan IoU 1.000 and 8 mm of 60 mm). The catalog file grows from 1.66 MB to 1.84 MB.
- After (scan, 8 hard): the 12 `sync-bone-*` findings of 1:200 are gone (gate over clips and transitions at 1:200: 0 hard; 17 reported, worst 221 mm). Left: `sync-forward` 2, `sync-height` 2, `anchor-landmark` 3, `derived-view-in-band` 1. The gate over all clip instants counts 75 hard findings at 1:50 and 1:100 (the same four kinds).
- **Open at 1:50 and 1:100.** The same redraw clears every hard finding there too (measured on a work file with `--slab 0.45`: 0 hard and 0 reported over the poses of all four scales, silhouette guard 0 failing checks, `validateAsset` 0 issues). I did not keep it: the plates of neighbouring masses interleave in depth and their outlines read as contour lines in the solid view of the default pose, and the berries lose their circles in the line view (sheets of 1:50 compared with the state before). Redrawing only the masses that set the extents and the outlets (`fol-top`, `fol-1`, `fol-2`) also gives 0 hard, with the same look in those masses. A repair that keeps the lobes needs plates that carry no outline of their own inside the crown, or solids per lobe; the anchors `tp-sun-1` and `tp-sun-3` follow from it and were therefore not moved (moving them means moving their leaf patches at four scales).
- **Open: `derived-view-in-band` front at 1:500, 0.0211.** The 1:500 body is rings of 6, 8, 8 and 4 equal ellipsoids round the leader and a top: the same from two sides by construction. Even the 1:200 body has outer silhouettes only 0.02 apart; its views differ through the masses inside (0.056). Needs a 1:500 body with the masses of this individual and a drawing that shows them. Not done.
- Sheets looked at: 1:200 default and `gale` (default as before but for fewer inner lobe lines in the front; in `gale` the front shows short depth lines at the windward edge, sizes now right), 1:50 and 1:100 of the rejected variants.

```
rowan | scan: exit 1 [scan] 1 assets: 0 clean, 1 with a hard finding, 8 hard findings
rowan | gate (clips, transitions): exit 1 hard findings: 75
rowan | derived: 1 flagged scale(s)
rowan | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
rowan | validateAsset: rowan: 0 issues, 0 before, 0 new
rowan | check: ✓ rowan (1/1); 4 authored scale(s) suitable
rowan | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
rowan | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
rowan | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
rowan | asset-validation: exit 0  Test Files 1 skipped (1); Tests 83 skipped (83);
rowan | asset-power: exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
```

Matrix: `rowan | matrix: exit 0 252 cells {'ok': 77, 'fail': 112, 'group-drift': 63} failing cells by check {'derived-view-in-band': 7, 'sync-forward': 70, 'sync-height': 70, 'anchor-landmark': 56}` (a cell can fail several checks). No cell fails `sync-bone-*` any more.

### Untouched assets (allowlist only)

`ceiling-diffuser`, `used-park-lantern`, `used-playground-roundabout`, `waste-bin`, `boxwood-ball`, `norway-spruce`, `windmill-palm`: the scan and the gate of this session report no other finding than `derived-view-in-band` front (gate over clips and transitions of the first five: 0 hard besides it). Their files were not edited, so their other tests stand as they were. Sheets looked at for the decision: bin 1:200 and 1:500, roundabout 1:100 and 1:500, lantern 1:200 and 1:500, spruce 1:100 and 1:200, larch 1:500, palm 1:200 and 1:500, rowan and magnolia 1:200 and 1:500.

## What stays open, plainly

1. Until `W/allowlist.json` is merged into the fixture, the six agreed assets keep their `derived-view-in-band` cells (the lamp: 35 cells measured).
2. `norway-spruce` (1:200, 1:500) and `windmill-palm` (1:200): my allowlist entries contradict the task's list; without them both keep failing and need new solids at the coarse scales.
3. `saucer-magnolia` 1:500 front: 0.0180, needs a 1:500 body with the lopsided masses of 1:200.
4. `rowan`: 8 scan findings (75 over clip instants) at 1:50 and 1:100 from flat foliage sheets, and 1:500 front 0.0211.
5. `decentralised-hrv-fan` 1:100 and 1:200: reported group drift of the rotor (86 to 113 mm), not a failing cell, not repaired.
6. Matrix runs of this session, each to the end (`W/matrix.sh`, one asset at a time): `european-larch` 252 of 252 ok; `decentralised-hrv-fan` 0 fail; `pendant-lamp` 35 fail (allowlisted view only); `saucer-magnolia` 7 fail; `rowan` 112 fail. The seven untouched assets were not run through the matrix: their files did not change, and their only scan finding is the allowlisted view.

## Files

Created (ticket, all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`):

- `multiview-MS.md` (this report)
- `🧰️matrix/MS/allowlist.json`
- `🧰️matrix/MS/fix-pendant-lamp.ts`, `fix-decentralised-hrv-fan.ts`, `fix-european-larch.ts`, `fix-saucer-magnolia.ts`, `make-rowan.sh` (the repairs, each idempotent: a second run changes nothing, compared byte by byte)
- `🧰️matrix/MS/lib.ts`, `spans.ts` (copy of `🧰️silhouette/SV-2/spans.ts`, extended by anchors and decisions)
- `🧰️matrix/MS/derived.ts`, `where.ts`, `outline.ts`, `anchor-search.ts`, `validate.ts`, `inspect.py`, `dump.py` (measuring)
- `🧰️matrix/MS/env.sh`, `x.sh`, `look.sh`, `vt.sh`, `prove.sh`, `matrix.sh` (running)

Updated (catalog, `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`):

- `🔌️appliances/pendant-lamp/🔣️.json`
- `🔌️appliances/decentralised-hrv-fan/🔣️.json`
- `🌳️vegetation/european-larch/🔣️.json`
- `🌳️vegetation/saucer-magnolia/🔣️.json`
- `🌳️vegetation/rowan/🔣️.json`

Removed: `🗑️generated/matrix/MS` (tool output, backups of the twelve assets before the first edit, sheets).

For the coordinator's manifest sync: no declared dimension changed.
