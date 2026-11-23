# ⛱️ Tied Canopy (`components/tied-canopy`)

## 2026-11-22 23:00 — coarse scales for their own smallest feature (umbrella rework, report `U/rework-walls.md`)

Findings before (`r3-validate.ts components/tied-canopy --catalog-only --no-print`): 87 `volumes-feature` (1:100: 14, 1:200: 33, 1:500: 40): nuts 13 mm, clamp plates, tip plate and EPDM bearings 8 mm, clevis cheeks 10 mm, gussets 15 mm, plates 20 mm, joints 16.8 mm, studs 16 mm, and at 1:500 also pins, turnbuckle bodies and the 60.3 mm tube in all three lists.

Cause: the 1:100, 1:200 and 1:500 lists carried the parts list of 1:50. Each coarse scale is now modelled for its own smallest feature, following what the 2D drawing of that scale retains, merges and omits.

| Scale | Body | Thermal | Structure |
|---|---|---|---|
| 1:100 (15 mm), 44 → 32 solids | Nuts, clamp plates, tip plate and bearing strips gone; beam runs 5.5 m; panes, joint gaskets and notch seal start on the top flange (24.8 mm glazing layer = pane + bearing, as drawn); the two 10 mm cheeks of each fork are one 37 mm head; studs end flush with the plates | references to removed parts dropped (34 → 28) | `m-beam` 5.5 m, tip-plate joint dropped, studs flush (22 → 21) |
| 1:200 (30 mm, 24.8 mm drawn), 44 → 9 | `wall-core`, `wall-outer` (wool + render, 200 mm, with pad holes), two pads that take in their plates, beam, one `glass-roof`, `node-beam` and `node-wall` lugs 40 mm wide with the drawn outline, `strut-tube` as one 60.3 mm bar from pin to pin | wall layer shells kept, `th-plate-beam` as own shell, beam, roof, lugs and strut as references (32 → 15) | beam, strut, two lugs, two pads, two stud-group blocks 250 x 162 x 132 mm in the concrete, supports, mass (22 → 11) |
| 1:500 (75 mm), 44 → 4 | `wall` 500 mm, `beam` as plain 82 x 160 mm bar, `glass-roof`, `strut-tube` from the wall face to the roof with its true section and `slender: true` (4.75 m by 60.3 mm; the drawing shows it as a 0.13 mm stroke) | plate shells, render faces, beam, roof, strut (31 → 12) | beam, strut, two supports, mass (22 → 5) |

Sheets: Opened at half size: 1:100 body; 1:200 body, thermal, structure; 1:500 body, thermal, structure; 2D 1:500. Not opened: 1:100 thermal and structure, 2D 1:100 and 1:200.

Proof (2026-11-22, pinned bun, one process at a time, each run to its end):

- `r3-validate.ts components/tied-canopy --catalog-only`: `VALID components/tied-canopy (catalog file as it is): schema 0, validateAsset 0, checkVolumes not run (--volumes)`
- `r3-validate.ts components/tied-canopy --catalog-only --volumes`: `VALID components/tied-canopy (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`
- check CLI `--asset tied-canopy`: `✓ tied-canopy (1/1)`
- volumes source `volumes/components/tied-canopy.json` equals `asset.volumes` of the catalog file (compared after the splice with `volumes-insert.py`).

Script: `rework-tied-canopy-coarse.py` (states every hand-chosen number; it refuses a second run on the rebuilt source, a rerun needs the source of before this pass). The 2D drawings were not changed: their `decisions` (retained, merged, omitted) per scale were the specification the 3D lists were brought to. No evaluated manifest value changes.

What stays imperfect: At 1:500 the beam is a box, so the thermal list measures about 2.7 m² for `canopy-steel` against the declared 3.43 m² of the I-section (inside the 30 % band of coarse scales). The 1:200 stud blocks are an envelope of four studs, not studs. 1:50 was not looked at or reworked in this pass (rebar, nuts, clamps stay as they were). No reference table against the real subject was written and no pose or clip was rendered in this pass; no independent review.
