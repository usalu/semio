# 🪟 Masonry Wall with Lintel (`components/masonry-wall-with-lintel`)

## 2026-11-22 23:00 — coarse scales for their own smallest feature (umbrella rework, report `U/rework-walls.md`)

Findings before (`r3-validate.ts components/masonry-wall-with-lintel --catalog-only --no-print`): 525 `volumes-feature`: 118 bed and head joints in the 1:100 body, 122 in the 1:200 body (joints and the 15 mm plasters), 267 in the 1:500 body (every brick, joint, plaster, sill end), and six bearing joints of 7.5 and 12.3 mm in each coarse structure list.

Cause: the 1:100, 1:200 and 1:500 lists carried the parts list of 1:50. Each coarse scale is now modelled for its own smallest feature, following what the 2D drawing of that scale retains, merges and omits.

| Scale | Body | Thermal | Structure |
|---|---|---|---|
| 1:100 (15 mm), 318 → 64 solids | facing as 53 course bands of 83 mm (33 courses cut by opening, lintel and sill), joints in them; `inner-leaf` as one 175 mm leaf around opening and lintel pocket; cavity wool to the lintel underside, wool between the lintels 0.25 m high; `lintel-outer` as plain bar with its bed in it; `sill` as one weathered profile over 1.4 m without drip groove and raised ends; plaster, reveal and soffit plaster, inner lintel, sill wool kept | 53 facing strips, lintel shell, one sill (57 → 55) | two wall members, two lintels, sill, six bearing pads 15 mm thick, supports, masses; 25 wall ties gone (47 → 20) |
| 1:200 (30 mm), 318 → 4 | `wall` (445 mm mass around opening, lintel and sill pocket), `lintel` band 1.75 x 0.25 x 0.445 m, `sill` bar with its 50 mm nose, `parapet-cap` behind it | 21 facing strips, lintel shell as own solid, sill (25 → 23) | two wall members, lintel, sill, lintel pads left and right, sill bed, beam bearing (30 mm), supports, masses (47 → 17) |
| 1:500 (75 mm), 318 → 3 | `wall`, `lintel`, `sill-band` through the wall with the nose | 8 facing zones, lintel shell and sill as own shells (12 → 10) | wall, lintel, sill band, lintel pads and beam bearing (75 mm), supports, masses (47 → 15) |

Sheets: Opened at half size: 1:100 body and structure; 1:200 body; 1:500 structure. Not opened: all thermal sheets, 1:200 structure, 1:500 body, the 2D drawings (unchanged).

Proof (2026-11-22, pinned bun, one process at a time, each run to its end):

- `r3-validate.ts components/masonry-wall-with-lintel --catalog-only`: `VALID components/masonry-wall-with-lintel (catalog file as it is): schema 0, validateAsset 0, checkVolumes not run (--volumes)`
- `r3-validate.ts components/masonry-wall-with-lintel --catalog-only --volumes`: `VALID components/masonry-wall-with-lintel (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`
- check CLI `--asset masonry-wall-with-lintel`: `✓ masonry-wall-with-lintel (1/1)`
- volumes source `volumes/components/masonry-wall-with-lintel.json` equals `asset.volumes` of the catalog file (compared after the splice with `volumes-insert.py`).

Script: `rework-masonry-wall-with-lintel-coarse.py` (states every hand-chosen number; it refuses a second run on the rebuilt source, a rerun needs the source of before this pass). The 2D drawings were not changed: their `decisions` (retained, merged, omitted) per scale were the specification the 3D lists were brought to. No evaluated manifest value changes.

What stays imperfect: Bearing pads of the structure list are drawn one feature thick at every coarse scale (15 / 30 / 75 mm), like the foundation supports the asset already had; their true thickness is 7.5 and 12.3 mm. At 1:200 the leaves are one mass in the body while the structure list still tells the bearing leaf from the facing. The glazing rebates, wall ties and block joints exist at 1:50 only. No reference table against the real subject was written and no pose or clip was rendered in this pass; no independent review.
