FINAL (2026-11-26 04:35: 1 of 3 clean with all proofs (`hawthorn`); `black-alder` and `field-maple` OPEN in their proven MV-1 state, depth plates still in them: their loft copies pass gate, validator and sweep but fail the silhouette guard, numbers and request in section 4)

# 🍃️ Multiview Fixer Report: Round MV-L1 (Drawing Quality of Three Trees in Wind Poses)

Assets: `black-alder`, `field-maple`, `hawthorn` (`S/🗂️catalog/🌳️vegetation`). Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder, `W` = `U/🧰️matrix/MV-L1`, `M` = `U/🧰️matrix/MR`, `GEN` = `U/🗑️generated/matrix/MV-L1`. State at the start: the three catalog files were the proven state of batch MV-1 (backups in `GEN/backup`, sha-256 `12ad6a769bd9c29d…`, `d13f2e89c9f1d9cc…`, `99395b1f7299ea65…`).

## 1. What was wrong and what replaces it

MV-1 mended the extents of the foliage with depth plates (`mvc-*`, slices of a mass 0.004 mm thick) below the flat front and plan outline, and renamed the outline to `-mvs`. In a wind pose the plates show beside the outline: straight cuts, notches and slivers (`GEN/sheets/z.*.before.png`).

In `hawthorn` every foliage mass is now ONE `loft` shape per view (side elevation, front, plan) whose body is the body the mass already has: the ellipsoids of its bone in `volumes.<scale>.body`. The engine turns that body, so every view is its true projection in every pose, clip instant and transition, as one outline. The tool is `W/loft.ts` (its docstring is the full description). In short:

- **Bead.** An ellipsoid is a bead of parallel elliptic sections across the viewing direction: its contour (the great circle across A⁻¹n for the axes A and the viewing direction n), one section on either side at the latitude the contour reaches at the largest turn of the bone over every subject of the matrix (plus 3 degrees, at least 4, a further section every 16 degrees), and a point at each pole. That is two to four convex hulls per ellipsoid for the engine.
- **Thread.** The beads of a mass hang on one thread of 0.6 micrometres, which has no area in the union of the engine, so a mass is always one shape. A second loft of the same mass draws its own outline inside the first.
- **Hidden ellipsoids** (inside the others at every turn, 0.04 mm of paper allowed) are left out.
- **Holes.** A gap the ellipsoids leave inside the silhouette at some turn is closed by a filling ellipsoid that is never seen beyond the silhouette of the mass, never stands free in a hole (a circle in an opening) and, at rest, never covers an opening the authored outline draws itself: the default pose keeps its drawing, and an authored opening closes when the wind turns the mass instead of becoming a crescent with tails. Dots (holes below 1 mm of paper, where three ellipsoids just meet) are closed by a pass with the engine over every subject, with balls of at least 0.5 mm of paper.
- **Untouched:** leaf strokes (they keep the suffix `-mvs`, so the engine carries them at the depth of the elevation silhouette of their bone), limbs with their plates (`mvc-limb-*`: thin, not foliage, and they carry hard findings of MV-1), 1:500 (no foliage bones are drawn there).

Tried by the predecessors and not used: rigid cards with lobes (`body.ts`), one loft of stacked sections per mass (`slice.ts`: loses the lobes of the outline).

Four errors of the loft tool, found and repaired in this takeover:

1. The contour of an ellipsoid was taken across Aᵀn instead of A⁻¹n. Eleven sections per ellipsoid hid it; with three sections an oblique ellipsoid was drawn up to 216 mm short (`hawthorn` 1:200 `fol-5`). I first blamed the union of the engine; MX-3 measured the union against shapely and showed the loss was in the bodies. The run now prints `LOST` for any mass the engine draws smaller than its ellipsoids reach (it must not appear).
2. Eleven sections per ellipsoid cost 8 to 22 s per evaluation at 1:50; now 0.2 to 0.35 s.
3. Ellipsoids that touch no other were a second loft, which drew circles and dots inside the mass.
4. The suffix `-mvs` of the leaf strokes was stripped with the plates; without it the strokes of `field-maple` stood 355 mm outside their mass in `storm` at 1:50 (4 hard findings `sync-forward`, 217 mm).

The side elevation became a loft only after MX-3 made a loft of the elevation the depth twin of its bone (D66, request `## multiview MV-L1 — depth of flat front and plan shapes …`, answered 23:25): before that the flat strokes were drawn as shards.

## 2. Recipe (for MV-2 and MV-3: a plant whose foliage masses carry depth plates; proven on `hawthorn`, 2026-11-26 03:03; it did NOT carry `black-alder` and `field-maple` past the silhouette guard, so expect step 3 to stop some plants)

Preconditions, check them first (`python3` over the asset JSON is enough): the foliage bones have `ellipsoid` solids in `volumes.<scale>.body` (no `weights`), and each foliage bone draws exactly one closed flat `role: "body"` outline per view. A mass without ellipsoids (grass plumes drawn from lofts, say) is not converted by this tool: `loft.ts` skips it and its plates stay; say so.

All through the queue, one reservation of 800 MB for the trial steps (`W/work.sh 800`, jobs dropped into `GEN/jobs`), `source W/env.sh` with your `BATCH`:

1. Back the document up (`GEN/backup/<id>.json`).
2. Search run on a copy (2 to 6 minutes):

   ```sh
   cd "$T"; "$B" "$W/loft.ts" "$GEN/backup/<id>.json" --save "$W/ops/<id>.json" --out "$GEN/try/<id>.json" > "$GEN/log/<id>-loft.log"
   grep -E '^1:|ISSUE|LOST' "$GEN/log/<id>-loft.log"
   ```

   Must hold: no `ISSUE` (a `loft-fold` is a defect of the thread), no `LOST`, `extent of the body against the authored outline` within a few millimetres (it was 3 to 10 mm on the three trees; more means the authored outline is not the silhouette of the body at that scale: look at it), evaluation time per subject below a second.
3. **Light proof on the copy, before any write (binding):** `W/light.sh <id>` inside a reservation of 1100 MB (`measure.ts` peaks at 833 MB) runs the gate over every subject, the gate over the poses (the scan line; the official scan reads the catalog and runs after the write), the silhouette guard with `validateAsset` and the sweep, and the check CLI with `--file`, and prints `LIGHT PROOF <id>: PASS|FAIL`. No PASS, no write: `black-alder` was written on a green gate alone and failed the guard by 1207 mm (section 4).
4. Open the panels of the strongest wind pose at every scale and view, and the default pose (`W/zoom.ts <json> <scale> <pose> <elevation|front|plan> <png> 1300`, `DENSITY=500` keeps it below 600 MB). Look for: cuts, notches, slivers, dots, circles inside a mass, strokes outside their mass.
5. Write and prove: `W/batch.sh a <id>…` (1600 MB: `make.sh` twice, gate, scan, guard, check CLI, matrix) and `W/batch.sh b <id>…` (2500 MB: the six suites). `make.sh` applies `ops/<id>.json` with `loft.ts --ops --write`; the second run must leave the digest unchanged.

What goes wrong, and the answer:

| Symptom | Cause | Answer |
|---|---|---|
| `LOST` or a mass smaller than its authored outline | sections do not reach the contour | a defect of the tool, not of the asset: stop and report |
| circles or dots inside a mass | a second loft of the mass, or a hole below the pen | one loft per mass (the tool does it); `--speck` larger |
| crescents with tails where the default pose has a small opening | an authored opening seen at a turn | the fillings close it in the wind and keep it at rest; `at most N holes at one turn` larger than `authored openings` in the log names the mass |
| leaf strokes far outside their mass in the wind | the stroke lost its suffix `-mvs` | never strip the suffix of strokes |
| strokes drawn as shards | elevation loft before D66 (engine of 2026-11-25 23:15) | needs that engine |
| gate `sync-bone-*` on a limb | the plates of a limb were removed | plates of non-foliage bones stay (`strip` is per bone) |
| gate `anchor-landmark` | an outlet bound to the place of a plate | `../MV-1/anchors.ts <copy> --save ops/<id>-anchors.json`, `make.sh` applies it |
| guard: a view far beyond its limit (hundreds of millimetres) although `LOST` is silent and the extents agree | not found (section 4): on `black-alder` three masses round the one authored see-through of the crown each produce it alone | do not write; bisect with `--only <bone> --views <view> --scales <scale> --no-fill --speck 0` and `measure.ts --scales <scale>`, add the numbers to the request `## multiview MV-L1 — silhouette guard reads 1207 mm …` |

## 3. Per tree

### `hawthorn` (clean, all proofs; written 2026-11-26 02:43)

- Before (state of MV-1): 55 + 31 + 29 depth plates in the front and 23 + 40 + 20 in the plan at 1:50, 1:100, 1:200. The plates of the seven foliage masses are gone now; the plates of `limb-1`, `limb-3`, `limb-4` stay (25 plates and 25 `-mvs` shapes are left in the file: limbs and leaf strokes).
- Now: 63 lofts (7 masses × 3 views × 3 scales), one per mass and view. Extent of the body against the authored outline in the default pose: at most 3 mm (1:50), 4 mm (1:100), 4 mm (1:200). Evaluation 0.16 s per subject at 1:50; the gate over 134 subjects and four scales takes 73 to 105 s.
- The top mass `fol-7` has two authored openings in the front and the elevation and the open crown in the plan. At rest they are drawn as authored; in the wind the two front openings close (40 fillings), the crown opening of the plan stays open.
- Opened: `gale` at 1:50 in elevation, front and plan, `gale` front and plan at 1:100 and 1:200, `summer-calm` front at 1:50 (`GEN/sheets/z.hawthorn.*.try.png` of 02:42, the copy the ops were saved from): no cut, notch, sliver, dot or circle. In the plan the branches under the open crown are cut by the foliage in `gale` as before this round (flat limb shapes, not foliage).
- Known rest: the dot pass stopped at its cap of 40 balls for `fol-7` at 1:50 (89 holes below 1 mm of paper, counted over 134 subjects, got no ball: at single clip instants a pen-width dot can show where three ellipsoids just meet); 6 holes of 56 mm at 1:100 lie in authored openings and stay.

```
second run: hawthorn catalog file dd8790c29ea7c4b6 -> dd8790c29ea7c4b6
gate exit 0 hard findings: 0
scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
hawthorn silhouette: OK
hawthorn validate: 0 issues
hawthorn three-views sweep: 0 findings
check: exit 0 ✓ hawthorn (1/1); 4 suitability lines
matrix exit 0 [matrix] 252 cells: 84 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (736 s) status {'ok': 84, 'group-drift': 168}
🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 75 skipped (85);
⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
```

### `black-alder` (OPEN, catalog file in its MV-1 state, digest `ee9dcf31fb5b2ca4`)

- Copy with lofts (14 masses, 126 lofts): gate 0 hard over 126 subjects (109 to 186 s), `validateAsset` 0, sweep 0, extents against the authored outline 3, 5 and 8 mm, no `LOST`; `gale` panels of the front at 1:50 and 1:200 clean, the plan at 1:50 clean except two pen-width dots in the top mass.
- **Silhouette guard on the copy FAILS:** 1:50 elevation 58.7 mm and front 30.9 mm (limit 15), 1:200 front 1207.5 mm (limit 60); 1:100 and 1:500 pass. The MV-1 state reads 13.1, 13.1 and 16.8 mm there.
- Mistake of this lane, stated plainly: the catalog file was written at 03:22 on a green gate without the guard on the copy, the proof chain then failed the guard, and the file stood in the catalog in that state until 03:38, when it was put back byte-identically (the MV-1 bytes with the `tags` another lane changed in the three files between 19:05 and 01:55). Since then `light.sh` runs before every write.

### `field-maple` (OPEN, catalog file untouched, digest `bff851e37a2c48a0`)

- Copy with lofts (11 masses, 99 lofts): gate 0 hard over 106 subjects (113 to 143 s; with the stroke suffixes kept), `validateAsset` 0, sweep 0, extents 8, 8 and 10 mm, no `LOST`; `storm` panels at 1:50 in elevation, front and plan read cleanly, the hatched see-throughs of the crown as authored.
- **Silhouette guard on the copy FAILS:** 1:50 elevation 22.6 mm (limit 15), 1:100 front 67.1 mm (limit 30); the other ten views pass.

## 4. Open: the silhouette guard on `black-alder` and `field-maple`

Measured on copies (03:51 to 04:30), every run to its end:

| Variant of the `black-alder` copy | Guard, front 1:200 |
|---|---|
| MV-1 state | 16.8 mm |
| every mass a loft, with fillings and dot balls | 1207.5 mm |
| every mass a loft, no filling, no ball | 1207.5 mm |
| a rest stage that closes every unauthored hole of the rest pose | 1207.5 mm |
| sections enlarged to stand around the ellipse (1 / cos(pi / 32)) | 1204.9 mm, and 362.9 mm in the elevation |
| ONE mass a loft in the front only: `fol-8`, or `fol-9`, or `fol-10` | 1207.5 mm each |
| one mass a loft: each of the other eleven | 16.8 to 22.5 mm |

So neither the fillings, nor a hole at rest, nor a hairline between masses, nor a lost ellipsoid is the cause; three explanations I gave on the way were wrong and are withdrawn. The filled shapes of the copy with `fol-8` alone converted have exactly the openings of the authored file in the default pose (eight see-throughs of the crown and the authored opening of `fol-10`, 555 x 232 mm). The three masses that trigger it surround that one opening. The guard gives the number, not the place; its reference follows the drawn shapes (`drawnFor`), which I did not get to the bottom of. Request filed: `requests.md`, `## multiview MV-L1 — silhouette guard reads 1207 mm for a foliage loft whose filled drawing has the same openings as before`.

Consequence: the wind-pose drawings of `black-alder` and `field-maple` are as the coordinator rejected them (294 and 170 depth plates). The lofts are four minutes of search run away once the guard question is answered (`loft.ts <id> --save ops/<id>.json --out <copy>`, then `light.sh`).

Also open, small: the dot pass is capped at 40 balls per mass and view; `hawthorn` `fol-7` at 1:50 reached it (section 3).

## 5. Requests filed in `requests.md`

- `## multiview MV-L1 — depth of flat front and plan shapes when the elevation of their bone is a loft`: answered and repaired by MX-3 (D66).
- `## multiview MV-L1 — recipe ready` (03:17).
- `## multiview MV-L1 — silhouette guard reads 1207 mm for a foliage loft whose filled drawing has the same openings as before` (04:30): open.

## 6. Files

Updated (catalog): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/hawthorn/🔣️.json`. Written and put back unchanged: `…/black-alder/🔣️.json`. Not touched: `…/field-maple/🔣️.json`, every engine, suite, fixture, README and manifest file.

In `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`:

- Updated: `multiview-MV-L1.md`, `requests.md` (three sections appended), `🧰️matrix/MV-L1/loft.ts`, `make.sh`, `prove.sh` (matrix at 1600 MB, `ONLY`), `zoom.ts` (`DENSITY`).
- Created: `🧰️matrix/MV-L1/light.sh`, `batch.sh`, `ops/hawthorn.json`.
- Removed: `🧰️matrix/MV-L1/ops/black-alder.json`, `ops/field-maple.json` (ops of failed trials), `ops/black-alder-anchors.json` (empty ops of a predecessor trial), `🗑️generated/matrix/MV-L1` (tool output and backups).
- Of the predecessors, kept and not part of the chain: `body.ts`, `slice.ts`, `solids.ts`, `drift.ts`, `pair.ts`, `first.sh`, `look.sh`, `round.sh`, `trial.sh`, `finish.sh`, `pairs.sh`, `sheets.sh`, `zooms.sh`, the list files.

The chain of a plant: `../MV-1/make.sh <id>`, then `make.sh <id>` (`loft.ts --ops ops/<id>.json --write`). The tool of this state reproduces `ops/hawthorn.json` byte for byte from the MV-1 document, and a further apply leaves the catalog file unchanged (run of 04:31).
