# components/wheelchair-ramp — rework report

## Pass 1 (second round), 2026-11-22

State found: no earlier report for this asset; `T/volumes/components/wheelchair-ramp.json` was identical to the catalog `asset.volumes`, and `r3-validate.ts components/wheelchair-ramp --volumes` ended `VALID` before any change. The asset was already authored per scale (34 / 12 / 9 / 7 body solids), not a cut-down 1:50 list.

### Reference table (ADA 405 / DIN 18040-1 / EN 10219, own knowledge) against the model

| item | reference | model | verdict |
|---|---|---|---|
| running slope | 1:12 max (ADA), 6 % (DIN) | 0.30 m per 3.60 m run = 1:12 | right (ADA ramp, stated in the summary) |
| rise per run | at most 0.76 m | 0.30 m | right |
| landings | at least 1.50 m long | middle 1.50 m, top 1.50 m, at-grade landing = ground | right; the at-grade landing has no solid (it is the ground) |
| clear width | at least 1.20 m between kerbs / rails | 1.20 m between kerbs, 1.258 m between rails, 1.40 m overall | right |
| wheel guard | 0.10 m high | 100 x 100 mm, 10 mm chamfer at 1:50 | right |
| handrail | tube 30-45 mm, top 0.85-0.90 m, both sides, 0.30 m level extensions, ends returned | CHS 42.4, axis 0.85 m above the surface, 0.30 m extensions, 60 mm bend into a 100 mm leg | right at 1:50; return was missing at 1:100 and 1:200 |
| posts | CHS 48.3, spacing at most about 2 m, base plate with two anchors | 48.3 mm at 1.7 m, plate 120 x 70 x 10, two M10 nuts, cap and 16 mm stem | right |
| structure | 150 mm slab on two 200 mm walls over fill, dowelled joints | hollow slab-on-walls members, three dowel rows, four bearing layers, lumped mass | right |

### Defects found

1. 1:100 and 1:200 body: the handrails ended as plain cut tubes; the turned-down return (42 mm tube, 100 mm leg) is well above the 15 / 30 mm feature. The 2D elevations had no return either and listed it as omitted.
2. 1:500 body: no kerbs, although the kerb (100 mm) is above the 75 mm feature and the thermal list of the same scale carried kerb strips over a body that had none; posts stood on the slab edge instead of the kerb top. The 2D elevation mass ended at the walking surface although its decision text said the kerb was merged into it.
3. 1:500 2D decision text spoke of a "0.075 m toe step" that is not drawn.
4. 1:50: nothing wrong enough to rebuild (rendered body, thermal, structure, the 2D sheet and the pose drawing).

### Rebuilt (script `T/rework-wr-wheelchair-ramp-returns-kerbs.py`, hand-chosen stations)

- 1:100 body `rail-a/b`: leg, bend in three chords (0, 30, 60, 90 degrees), runs, bend, leg (14 sections). Structure: `m-rail-*-ext-start/end` shortened to the straight part and new `m-rail-*-bend-start/end` members (6 sections), because the old straight ends lay 36 mm outside the new body silhouette.
- 1:200 body `rail-a/b`: leg, bend in two chords (0, 45, 90 degrees), 12 sections, still `slender`.
- 1:500 body: `kerb-a`, `kerb-b` added, posts on the kerb top (foot +0.10 m, length 0.733 m, 75 mm). Thermal `th-kerb-a/b` now refer to the body kerbs; structure gets `m-kerb-a/b` and the post joints move to the kerb top.
- 2D: 1:100 elevation `handrail` band with both returns (26 points), front `handrail-a/b` down to 0.75 m; 1:200 elevation two new strokes `handrail-return-start/end`; 1:500 elevation `ramp` mass up to the kerb top and `post-0..5` from the kerb top.
- Decisions (en, de) of volumes 1:100, 1:200, 1:500 and of the representations 1:100, 1:200, 1:500 rewritten where the form changed.
- Rig, poses, clips, anchors, dimensions, 1:50 lists: unchanged (three fixed/prismatic bones are right for a rigid ramp; the only moving bone is the massless wheelchair carrier, which has no solid).

### Validation

`r3-validate.ts components/wheelchair-ramp --catalog-only --volumes` → `VALID components/wheelchair-ramp (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after `volumes-insert.py`). Rendered and read after the last change: 1:100 body three-quarter, 1:100 structure elevation, 1:200 body elevation, 1:200 thermal sheet, 1:200 2D, 1:500 body sheet, 1:500 structure three-quarter, 1:500 2D, 1:50 clip `wheelchair-ascent` t 5 s. Rendered but not read one by one (the 1:50 lists did not change and no pose moves a solid): poses `crowd-live-load`, `wheelchair-on-run-2`, `handrail-push`, the other five clip instants, 1:100 thermal, 1:100 / 1:200 structure sheets, 1:500 thermal.

Note: the `representations` member of the catalog file was rewritten with the formatter of `volumes-insert.py` (short point lists on one line); content beyond the listed shapes and texts is unchanged.

### Remaining, stated plainly

- 1:50 body: the three movement joints are only seams between the pours; a real joint is a 15-20 mm sealed groove (two features wide at 1:50). Not modelled; it would change pours, thermal skins and their areas.
- 1:200 2D: the height dimension reads 1.45 m (rail axis as a stroke) while 1:50 and 1:100 read 1.47 m (rail top).
- 1:500 front and plan drawings leave the kerb out (0.2 mm squares and strips); only the elevation carries it.
- The sheets listed above as "not read one by one" should be read by the next pass.

### Fundamental issues

- `volumes-silhouette` seems to fill an open, unfilled stroke as a closed polygon: giving the 1:200 elevation stroke `handrail` a 100 mm hook at each end dropped the elevation `iou` from passing to 0.741 (with the old 3D: 0.874), p95 105 mm, largest distance 175.7 mm, which is the area between the polyline and the chord joining its two ends (about 9.3 m x 0.1 m). Two separate two-point strokes for the returns pass. Not investigated in `S/🔨️modules/🗂️catalog/🧊️volumes`.
- `volumes-insert.py` runs `main()` on import, so its `dump` / `member_range` cannot be imported for other root members; the script here executes its source without the last line.
