FINAL (2026-11-27: check compares shape geometry in model space; chairs matrix 252 ok, 0 fail in en and de; suites green; today's file did not reproduce the cell under the old check either)

# CL-CH: `used-bentwood-dining-chairs` 1:1000 forces `language-geometry`

Findings (tools `🧰️matrix/CL-CH/diff.ts`, logs `🗑️generated/matrix/CL-CH/`):
- Re-drawing pose `in-use` at requested 1:1000 in English and German, all 7 modes and 3 views, with the original raw comparison (paper commands per `shape:` id): 0 differences. Today's file does not reproduce the cell; the failing record came from a state of the asset before the MV-L3 write, or is not stable.
- What does differ by language: the sheet (elevation 200.32 vs 205.38 mm wide, front 199.92 vs 204.99, plan 164.57 vs 168.45), because `Canvas.finish` translates all paths so the extent (labels included) starts at the margin. `metadata.origin` is equal here, but whenever a wider German label reaches beyond the shapes on the left or top, every shape path is translated and the raw comparison would fail while the drawing is the same. The engine's own contract (`🎨️view-synchrony`) is shape geometry relative to `metadata.origin`.
- Cure (in the check, the engine is right): `shapeGeometry` in `🧪️tests/🔭️multiview-coverage/🧰️.ts` now compares model-space coordinates (origin-relative, per `mmPerMetre`), with command ops and ids, tolerance 1e-9 m (`sameShapeGeometry`). Not weakened: ids, order, op sequence and every coordinate are still compared; only the layout translation is removed.
- Regression test beside the engine tests: `🧪️tests/🎨️view-synchrony` gets "shares fill geometry between English and German" per viewpoint at 1:500 and 1:50 (+ feature scenario).

Proof: matrix `full.sh --assets used-bentwood-dining-chairs`, en run: 252 ok, 0 fail; de run: 252 ok, 0 fail (through `repo capacity run`).

Suites (through the queue): 🎨️view-synchrony 21 passed (incl. the 8 new language tests), 🔭️multiview-coverage 42 passed, 🧊️three-views 775 passed.

Caveat: the original failure could not be reproduced on the current file even with the old raw comparison, so the root cause of that one record is not proven; the check change closes the demonstrated language-dependent mechanism (sheet translation by label width). Files: 🧪️tests/🔭️multiview-coverage/🧰️.ts, 🧪️tests/🎨️view-synchrony/🟦️.ts and 🥒️.feature, 🧰️matrix/CL-CH/diff.ts.
