# 🏢 Curtain Wall Bay (`components/curtain-wall-bay`)

## 2026-11-22 23:00 — coarse scales for their own smallest feature (umbrella rework, report `U/rework-walls.md`)

Findings before (`r3-validate.ts components/curtain-wall-bay --catalog-only --no-print`): 119 `volumes-feature` (1:100: 28 body, 8 structure; 1:200: 30 body, 8 structure; 1:500: 34 body, 11 structure): isolators 14 mm, pressure plates 10 mm, caps 12 mm, gaskets 8 and 10 mm, edge seal 16 mm, packer 22 mm, screws 8 mm, setting blocks 7.5 mm, cradle hook 12 mm, and at 1:500 the 50 mm tubes and 38 mm connectors.

Cause: the 1:100, 1:200 and 1:500 lists carried the parts list of 1:50. Each coarse scale is now modelled for its own smallest feature, following what the 2D drawing of that scale retains, merges and omits.

| Scale | Body | Thermal | Structure |
|---|---|---|---|
| 1:50 | base spigots 42 mm wide instead of 38 mm: a sliding fit with 1 mm play in the 44 mm tube. Before, 3 mm of air left them unconnected (`volumes-connected`, body and structure, measured in the first full run of this pass) | — | — |
| 1:100 (15 mm), 45 → 9 solids | every member one profile of 16 mm cap, 20 mm web and tube with the two glazing rebates (`mullion` twice, `transom-base`, `-mid`, `-head`); `igu` one 28 mm strip ending in the rebates; `spandrel` one 127.5 mm strip between the tubes; slab cleats and hanging bolts kept | plate and cap shells merged into one 12 mm cap shell per member, the rest as before (22 → 18) | six members, three connectors, setting blocks and cradle hook as pads 15 mm thick, bolts and spigots as supports, masses, cleat; screws gone (31 → 20) |
| 1:200 (30 mm), → 6 solids | members as plain bars 50 x 200 and 50 x 150 mm, `igu` 28 mm between the bars, `spandrel` strip | as 1:100 with the coarser gas cells (→ 16) | six members, connectors, hook, four supports as own solids (30 mm bolts, spigots), masses (→ 16) |
| 1:500 (75 mm), 40 → 3 | `mullion-back` twice (75 x 80 mm, the step of the plan), `vision` block 1.4 x 2.45 x 0.12 m, `spandrel` block 1.4 x 0.85 x 0.12 m, as drawn | cap shells as own solids (19 → 15) | frame, vision and spandrel members, hook 75 mm, four supports, mass (28 → 9) |

Sheets: Opened at half size: 1:100 body; 1:200 thermal and structure; 1:500 body. Not opened: 1:100 thermal and structure, 1:200 body, 1:500 thermal and structure, the 2D drawings (unchanged), and 1:50 after the spigot change (proven by the full run only).

Proof (2026-11-22, pinned bun, one process at a time, each run to its end):

- `r3-validate.ts components/curtain-wall-bay --catalog-only`: `VALID components/curtain-wall-bay (catalog file as it is): schema 0, validateAsset 0, checkVolumes not run (--volumes)`
- `r3-validate.ts components/curtain-wall-bay --catalog-only --volumes`: `VALID components/curtain-wall-bay (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`
- check CLI `--asset curtain-wall-bay`: `✓ curtain-wall-bay (1/1)`
- volumes source `volumes/components/curtain-wall-bay.json` equals `asset.volumes` of the catalog file (compared after the splice with `volumes-insert.py`).

Script: `rework-curtain-wall-bay-coarse.py` (states every hand-chosen number; it refuses a second run on the rebuilt source, a rerun needs the source of before this pass). The 2D drawings were not changed: their `decisions` (retained, merged, omitted) per scale were the specification the 3D lists were brought to. No evaluated manifest value changes.

What stays imperfect: At 1:100 the outer rebate of each mullion (15 x 28 mm, towards the neighbouring bay) is open, as at 1:50. The spandrel strip of 1:100 and 1:200 ends at the tube faces, so the 15 mm of enamel glass inside the rebates is not modelled there. At 1:500 the mullion backs are 75 mm wide (true 50 mm), as the plan draws them. No reference table against the real subject was written and no pose or clip was rendered in this pass; no independent review.
