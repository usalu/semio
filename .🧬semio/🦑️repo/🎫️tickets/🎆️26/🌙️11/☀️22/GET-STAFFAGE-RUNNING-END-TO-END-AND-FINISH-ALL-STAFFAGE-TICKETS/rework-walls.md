# 🧱 Rework report: walls, curtain wall and canopy (six invalid original components)

Session 2026-11-22 23:02 to 23:30. `K` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components`. Pinned bun, one tool process at a time, every proof line below run to its end in this session.

**Result: all six assets are `VALID`** in the cheap validation and in the full measured proof, and `✓` in the product check CLI. No evaluated manifest value changed, so there is nothing for the umbrella coordinator to edit in `🔣️.json` of the catalog.

## The modelling decision

All six failed for one reason: the 1:100, 1:200 and 1:500 lists were the parts list of 1:50. The 2D drawings of these assets were already drawn per scale, and their `decisions` (retained, merged, omitted) say what each scale shows. The 3D lists were brought to exactly that, by hand per asset and scale:

| Smallest feature | Masonry | Insulation and coats | Frames and fittings |
|---|---|---|---|
| 1:50, 7.5 mm | bricks and joints (unchanged) | every coat and board | every profile part |
| 1:100, 15 mm | course bands of 83 mm with their bed joint in them; plaster in the leaf where the drawing merges it | boards with the adhesive in them, the render coats as one 20 mm coat | one cap-web-tube profile per member; fork cheeks as one head; nuts, clamps, screws gone |
| 1:200, 30 mm | bond bands of 167 mm or one wall mass | one outer layer | plain bars, node lugs, one glazing layer |
| 1:500, 75 mm | one homogeneous wall | in the wall | blocks; a long tube keeps its true section as a `slender` line |

Thermal lists keep every declared layer as a layer shell (they are exempt from the feature rule by design); where a shell referenced a body solid that no longer exists it became a solid of its own. Structure lists were simplified with the body; bearing pads and markers are one feature thick, as the supports of these assets already were.

No rule, threshold or tool was touched. `slender: true` was set on one part only (the 4.75 m tube strut of the canopy at 1:500, 79 times as long as thick, drawn as a stroke). No `envelope` or `sheet` flag was added.

## Per asset

| Asset | Findings before | Remodelled (body solids 1:100 / 1:200 / 1:500, before → after) | Final proof | Sheets opened (half size) |
|---|---|---|---|---|
| `tied-canopy` | 87 `volumes-feature` | 44 → 32 / 9 / 4: fittings merged into heads and lugs, glazing with its bearing, wall layers merged step by step, strut pin to pin, then wall to roof | cheap `VALID … schema 0, validateAsset 0`; full `VALID … checkVolumes 0`; `✓ tied-canopy (1/1)` | 1:100 body; 1:200 body, thermal, structure; 1:500 body, thermal, structure; 2D 1:500 |
| `solid-brick-wall-1960` | 112 | 57 → 3 / 1 / 1: two alternating course bands and render, six bond bands, one wall | cheap `VALID`; full `VALID … checkVolumes 0`; `✓ solid-brick-wall-1960 (1/1)` | 1:100 body, thermal; 1:200 body, structure |
| `etics-brick-wall-12` | 122 | 66 → 6, 61 → 3, 60 → 1: course bands, boards 130 mm with adhesive, one render coat; bond bands and two ETICS courses 150 mm; one wall 530 mm | cheap `VALID`; full `VALID … checkVolumes 0`; `✓ etics-brick-wall-12 (1/1)` | 1:100 body; 1:200 thermal |
| `etics-brick-wall-30` | 122 | same decision with its own numbers: boards 310 mm, ETICS courses 330 mm, wall 710 mm | cheap `VALID`; full `VALID … checkVolumes 0`; `✓ etics-brick-wall-30 (1/1)` | 1:200 body; 2D 1:100 |
| `masonry-wall-with-lintel` | 525 | 318 → 64 / 4 / 3: 53 facing course bands, whole inner leaf, plain lintel, one sill profile; then wall mass, lintel band, sill, parapet cap; then wall, lintel, sill band | cheap `VALID`; full `VALID … checkVolumes 0`; `✓ masonry-wall-with-lintel (1/1)` | 1:100 body, structure; 1:200 body; 1:500 structure |
| `curtain-wall-bay` | 119 | 45 → 9, → 6, 40 → 3: profile members with rebates, glazing strip, spandrel strip; plain bars; vision and spandrel blocks between mullion backs | cheap `VALID`; full `VALID … checkVolumes 0`; `✓ curtain-wall-bay (1/1)` | 1:100 body; 1:200 thermal, structure; 1:500 body |

The full lines read `VALID components/<slug> (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. After the last publication the cheap validation of all six was run once more (23:26, all `VALID`) and every volumes source was compared with `asset.volumes` of its catalog file (all equal). Per scale and list, with solid counts and what was merged: `K/rework/assets/components--<slug>.md`.

## Found on the way

- `curtain-wall-bay` 1:50 was measurably invalid beyond the feature rule: the first full run reported `volumes-connected` for body and structure (base spigots 3.0 mm from the mullion tube, tolerance 1.5 mm). Fixed in the asset: spigots 42 mm wide, a sliding fit with 1 mm play in the 44 mm tube. This is the only change to a 1:50 list in this pass.
- `volumes-monotonic` counts every copy of a `repeat` as a solid and has no exception besides a decision that starts with `Minimal:`. The ETICS walls carry such a decision at 1:100 and 1:500, because their structure list (leaf, support, mass, four anchors, then leaf, support, mass) cannot get simpler at those steps. Body and thermal lists simplify at every step.

## What stays imperfect

- Sheets: 24 of the 72 changed sheets and drawings (6 assets × 3 scales × body, thermal, structure, 2D) were opened, at half size; the others were rendered and not opened. Listed per asset in the per-asset reports. The 2D drawings were not changed by this pass.
- 1:50 of these assets was not judged or reworked (apart from the spigots): no reference table against the real subject, no poses or clips rendered. These six still count as without a full handcrafting pass in the sense of `K/rework-brief.md`, and none is independently reviewed.
- Bond bands of 1:200 (solid brick and ETICS walls) are copies of one solid and show no seams in the sheet.
- Bearing pads, setting blocks and the cradle hook in the coarse structure lists are one feature thick, not their true 7.5 to 12.3 mm.
- `tied-canopy` 1:500: the beam is a plain bar, so its thermal surface measures about 2.7 m² against the declared 3.43 m² (inside the 30 % band).
- The scripts refuse a second run on the rebuilt sources; a rerun needs the volumes source of before this pass (the before-snapshots were tool output and are deleted).

## Files (repo-relative)

Created:
- `K/rework-tied-canopy-coarse.py`, `K/rework-solid-brick-wall-1960-coarse.py`, `K/rework-etics-brick-wall-12-coarse.py`, `K/rework-etics-brick-wall-30-coarse.py`, `K/rework-masonry-wall-with-lintel-coarse.py`, `K/rework-curtain-wall-bay-coarse.py` (the authoritative scripts)
- `K/rework-walls-dump.py`, `K/rework-walls-asset.py`, `K/rework-walls-findings.py`, `K/rework-walls-look.sh`, `K/rework-walls-small.sh`, `K/rework-walls-publish.sh` (listing, rendering, publishing with the three proofs)
- `K/rework/assets/components--tied-canopy.md`, `components--solid-brick-wall-1960.md`, `components--etics-brick-wall-12.md`, `components--etics-brick-wall-30.md`, `components--masonry-wall-with-lintel.md`, `components--curtain-wall-bay.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-walls.md`

Updated:
- `K/volumes/components/tied-canopy.json`, `solid-brick-wall-1960.json`, `etics-brick-wall-12.json`, `etics-brick-wall-30.json`, `masonry-wall-with-lintel.json`, `curtain-wall-bay.json`
- `C/tied-canopy/🔣️.json`, `C/solid-brick-wall-1960/🔣️.json`, `C/etics-brick-wall-12/🔣️.json`, `C/etics-brick-wall-30/🔣️.json`, `C/masonry-wall-with-lintel/🔣️.json`, `C/curtain-wall-bay/🔣️.json` (member `volumes` only, spliced with `K/volumes-insert.py`)

Removed: `K/🗑️generated/rework-walls` (before-snapshots, half-size copies) and this pass's sheets of the six assets under `K/🗑️generated/r3-sheet/components` and `K/🗑️generated/r3-2d/components`.

Not touched: the manifest, shared engine, schema, validator, tests, tools, README, site code; no ticket opened or closed.
