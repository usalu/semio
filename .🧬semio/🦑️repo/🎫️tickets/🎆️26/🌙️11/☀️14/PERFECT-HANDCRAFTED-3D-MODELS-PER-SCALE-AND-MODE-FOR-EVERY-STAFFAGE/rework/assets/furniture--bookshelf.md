# furniture/bookshelf — rework report

## Pass 1 (second round), 2026-11-22 about 15:50–16:01, author cut off without a report; proven by the umbrella finisher 22:32

State found by the finisher: `rework/14.md` section 8 "judged and measured, no defect found, not changed" (night). In the afternoon an author wrote and applied `rework-a1-bookshelf-scales.py` (source 16:00, catalog 16:01, `volumes` source = catalog) and left no report: shelf masses at 1:50; 1:100, 1:200 and 1:500 modelled for their own smallest feature (no 5 mm shelf pins, shelves from side to side, typified bearings in the structure list); the pin strokes removed from the 2D elevations of the coarse scales.

Proof by the finisher: `r3-validate.ts furniture/bookshelf --catalog-only --volumes` → `VALID furniture/bookshelf (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ bookshelf (1/1)`, four scales suitable.

Sheets read at full size: 1:50 body (30 solids) and 1:500 body (8 solids), six views each. Verdict: carcass of two sides, top, plinth and back panel with five shelves on their own bones; 1:500 is the same shelf in eight solids. Rendered, not read: 1:100 body and structure, 2D 1:200.

Remains imperfect: no reference table was written; at 1:50 the shelf pins and the back-panel groove cannot be seen at sheet size and were not checked in close-up; the 1:500 sheet looks identical to 1:50 at this zoom, so whether the sides were thickened or declared as sheets was taken from the validator, not from the eye; poses (loaded shelves) were not rendered.
