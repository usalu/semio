# Thermal legend suite, 14 of 14

Cause: the case "weights the merged regions ... 1:500 silhouette" read `shape:sil-torso` of `walking-commuter`; that asset no longer has a merged form at 1:500 (loft rewrite, D54).

Fix (test side only; no catalog asset or engine touched):
- New test-owned selector `S/🧫️fixtures/🎨️thermal-legend/🟦️.ts` (`mergedSilhouette()`): the first human asset (ids sorted) whose authored 1:500 elevation has a shape listing several thermal regions that all carry area; returns asset, first pose, shape id and region ids. It throws when none exists, so the case cannot pass vacuously.
- `S/🧪️tests/🎨️thermal-legend/🟦️.ts`: the case takes subject, pose, shape id and region ids from the selector, asserts at least two regions, and keeps the unchanged area x exposed-fraction weighted-mean assertion (9 digits) and the "area-weighted" label.

Run: `🧰️matrix/TL/run.sh` (repo capacity run, 1950 MB, pinned bun): 14 passed (14). Log in `🗑️generated/matrix/TL/` deleted afterwards.
