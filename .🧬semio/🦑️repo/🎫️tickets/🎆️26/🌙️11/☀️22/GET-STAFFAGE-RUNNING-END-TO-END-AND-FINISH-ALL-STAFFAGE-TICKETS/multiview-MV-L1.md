IN WORK (2026-11-26 02:00: `hawthorn` catalog file WRITTEN at 01:55 and NOT proven: its ops of 01:50 leave two dot fillings standing as circles in the opening of the plan in `gale` at 1:50, corrected ops are being made and the file is rewritten and proven next; `black-alder` and `field-maple` still the proven state of MV-1 plus the `tags` another lane changed; the Recipe below is a draft until the first tree is proven)

# 🍃️ Multiview Fixer Report: Round MV-L1 (Drawing Quality of Three Trees in Wind Poses)

Assets: `black-alder`, `field-maple`, `hawthorn` (`S/🗂️catalog/🌳️vegetation`). Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder, `W` = `U/🧰️matrix/MV-L1`, `M` = `U/🧰️matrix/MR`, `GEN` = `U/🗑️generated/matrix/MV-L1`. State at the start: the three catalog files were the proven state of batch MV-1 (backups in `GEN/backup`, sha-256 `12ad6a769bd9c29d…`, `d13f2e89c9f1d9cc…`, `99395b1f7299ea65…`).

## 1. What was wrong and what replaces it

MV-1 mended the extents of the foliage with depth plates (`mvc-*`, slices of a mass 0.004 mm thick) below the flat front and plan outline, and renamed the outline to `-mvs`. In a wind pose the plates show beside the outline: straight cuts, notches and slivers (`GEN/sheets/z.*.before.png`).

Now every foliage mass is ONE `loft` shape per view (side elevation, front, plan) whose body is the body the mass already has: the ellipsoids of its bone in `volumes.<scale>.body`. The engine turns that body, so every view is its true projection in every pose, clip instant and transition, as one outline. The tool is `W/loft.ts` (its docstring is the full description). In short:

- **Bead.** An ellipsoid is a bead of parallel elliptic sections across the viewing direction: its contour (the great circle across A⁻¹n for the axes A and the viewing direction n), one section on either side at the latitude the contour reaches at the largest turn of the bone over every subject of the matrix (plus 3 degrees, at least 4, a further section every 16 degrees), and a point at each pole. That is two to four convex hulls per ellipsoid for the engine.
- **Thread.** The beads of a mass hang on one thread of 0.6 micrometres, which has no area in the union of the engine, so a mass is always one shape. A second loft of the same mass draws its own outline inside the first.
- **Hidden ellipsoids** (inside the others at every turn, 0.04 mm of paper allowed) are left out.
- **Holes.** A gap the ellipsoids leave inside the silhouette at some turn is closed by a filling ellipsoid that is never seen beyond the silhouette of the mass and, at rest, never covers an opening the authored outline draws itself. Dots (holes below 1 mm of paper) are closed by a pass with the engine over every subject.
- **Untouched:** leaf strokes (they keep the suffix `-mvs`, so the engine carries them at the depth of the elevation silhouette of their bone), limbs with their plates (`mvc-limb-*`: thin, not foliage, and they carry hard findings of MV-1), 1:500 (no foliage bones are drawn there).

Tried by the predecessors and not used: rigid cards with lobes (`body.ts`), one loft of stacked sections per mass (`slice.ts`: loses the lobes of the outline).

Three errors of the first loft tool, found and repaired in this takeover:

1. The contour of an ellipsoid was taken across Aᵀn instead of A⁻¹n. Eleven sections per ellipsoid hid it; with three sections an oblique ellipsoid was drawn up to 216 mm short (`hawthorn` 1:200 `fol-5`). I first blamed the union of the engine; MX-3 measured the union against shapely and showed the loss was in the bodies. The run now prints `LOST` for any mass the engine draws smaller than its ellipsoids reach (it must not appear).
2. Eleven sections per ellipsoid cost 8 to 22 s per evaluation at 1:50; now 0.2 to 0.35 s.
3. Ellipsoids that touch no other were a second loft, which drew circles and dots inside the mass.

## Recipe (for MV-2 and MV-3: a plant whose foliage masses carry depth plates)

Preconditions, check them first (`python3` over the asset JSON is enough): the foliage bones have `ellipsoid` solids in `volumes.<scale>.body` (no `weights`), and each foliage bone draws exactly one closed flat `role: "body"` outline per view. A mass without ellipsoids (grass plumes drawn from lofts, say) is not converted by this tool: `loft.ts` skips it and its plates stay; say so.

All through the queue, one reservation of 800 MB for the trial steps (`W/work.sh 800`, jobs dropped into `GEN/jobs`), `source W/env.sh` with your `BATCH`:

1. Back the document up (`GEN/backup/<id>.json`).
2. Search run on a copy (2 to 6 minutes):

   ```sh
   cd "$T"; "$B" "$W/loft.ts" "$GEN/backup/<id>.json" --save "$W/ops/<id>.json" --out "$GEN/try/<id>.json" > "$GEN/log/<id>-loft.log"
   grep -E '^1:|ISSUE|LOST' "$GEN/log/<id>-loft.log"
   ```

   Must hold: no `ISSUE` (a `loft-fold` is a defect of the thread), no `LOST`, `extent of the body against the authored outline` within a few millimetres (it was 3 to 10 mm on the three trees; more means the authored outline is not the silhouette of the body at that scale: look at it), evaluation time per subject below a second.
3. Gate on the copy: `"$B" "$M/gate.ts" "$GEN/try/<id>.json" --soft` → `hard findings: 0`. Then guard, validator and sweep: `cd "$S/📦️packages/🟦️typescript"; "$B" "$M/measure.ts" "$GEN/backup/<id>.json" "$GEN/try/<id>.json"` (1100 MB: it peaks at 833 MB).
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
