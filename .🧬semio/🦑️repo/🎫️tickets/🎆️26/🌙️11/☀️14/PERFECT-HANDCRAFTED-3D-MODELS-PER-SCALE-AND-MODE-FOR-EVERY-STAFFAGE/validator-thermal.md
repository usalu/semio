# Validator, thermal maintainer (validator3)

Module `S/🔨️modules/🗂️catalog/🧊️volumes` (swap 16 in `schema-api.md`), scripts `validator3-*` here, output `🗑️generated/validator3/`. Tests: new suite `🧪️tests/🧊️volumes-thermal` (12, trimesh oracle `🐍️.py`).

## Done
- **0 Suites**: all green on the live module (23:24): volumes-rules 110, volumes-thermal 12, exposed-normals 7, model-silhouette 16, volume-traps 13, asset-validation 73, volume-schema 99. The previous 96/97 was the C30 test, now passing.
- **Found and fixed a lost swap**: swap 13 (outward normals, `orientationOf`, kernel `isClosed`) had been overwritten by the measures file of swaps 14/15 (built on an older base). Restored; `🧿️exposed-normals` 7/7.
- **1 C29 verified** against trimesh surface samples on ten assets (laptop, stepladder, cantilever-balcony, etics-brick-wall-12, park-bench, sorption-wheel, standing-adult, labrador-dog, city-car, apple-tree): all regions within 1 % at 1:50 after one deviation was fixed: hidden regions are now the full mesh area (city-car interior 4.35 -> 4.75, engine-bay 1.01 -> 1.19). `sectionFaces` on `{ of }` thermal references works and is pinned; the body solid's own marks are deliberately not inherited (author 56). Conditioned regions: no special case; the interior face needs `sectionFaces` (design C25 c, note under R-C in `area-decisions.md`).
- **2 Same-region burial**: area-weighted sampling existed; the seam defect was real: a disc in a ring of one region read +14 % (48 vs 64 segments) to +23 % (32 vs 64). Now `SEAM_FEATURES` = 0.2 smallest features (1.5 mm at 1:50): solids meeting within it touch; within 0.6 % of the hand value for 64/48/32/40; a real 5 mm gap keeps both faces. Cost: bounding-box candidate selection keeps city-car 1:50 thermal at 14.6 s CPU against 13.6 s.
- **C21 refined** (coordinator): tiny regions out of the 1:200/1:500 total on both sides; fixture test with the sample figure.
- Answers appended (`### Answer (thermal maintainer ...)`) to tool-requests 59, 45, 42, 26, 12, 11 and schema-requests 3, 4, 5, 6.

## Not done
- **3 Per-solid projected raster cache**: not built; it is inside the silhouette module the other maintainer edits (`Mask.fill`, measureBanded) and would change shared code without a measured need there yet. Done on the thermal side: tessellation cache (existing, content-keyed), per-mesh orientation and box caches, bounding-box candidates; `schema-catalog-check.ts` no longer computes the 1:50 thermal measure a second time when no 1:50 thermal finding exists.
- **4 Loop**: `validator3-loop.sh` is running in the background: it waits for the previous loop (ends about 00:34), then 6 runs every 30 minutes, after each `validator3-lists.py` writes Trend (with the change against the previous run), Ready, Thermal only and Geometry into `validator.md` (block `lists:begin/end`; it coexists with the old trend block until that loop ends). `catalog-check.md` is rewritten by the check itself. All module caches were invalidated by swap 16, so the first run after it is cold.
- Ticket `ticket_reopen`/`ticket_close` not called (shared ticket of many agents).

## Proposals
- stepladder 1:500 total 0.825: pen-floor rails; thermal rails true-size and `slender: true` work now without `halo` (C29 ignores burial by other regions).
- leaning-person `wall-face` 0.846 at all scales: one-sided area needs `sectionFaces` on the wall-side face.
- Seam tolerance 0.2 feature is a measure tolerance (constant `SEAM_FEATURES`, not in `VOLUME_LIMITS`, no limit weakened); say if you want it in the pinned limits.
