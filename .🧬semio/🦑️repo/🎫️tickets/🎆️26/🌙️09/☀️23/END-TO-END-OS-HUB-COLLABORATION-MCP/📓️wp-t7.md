# WP-T7: GIS GeoJSON, generation2d Example Export, Non-io Crate Failures, Raster DWG, DWG Oracle

Slice: T7 (session 10). Captures: `.tmp-ticket/wp-t7/generated/`. Kept inputs: `wp-t7/run-batch.sh`,
`wp-t7/crates-*.txt`. Private target: `wp-t7/target`.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. gismap GeoJSON import/export | landed (native) | stdio-json 86/86 (`stdio-json-1.txt`), gismap 158/158 (`gismap-1.txt`) |
| 2. generation2d export on bundled example | in progress | `generation2d-1.txt` |
| 3a. flow | green without change (256/256, 1 ignored) | `batch-1-summary.txt` |
| 3b. fem-2d timing laws | fixed (tests measure the product law), rerun pending | |
| 3c. draw | fixed one root cause (child restore projection), rerun pending | |
| 3d. raster | green (228/228) incl. the rewritten dwg import | `batch-1-summary.txt` |
| 3e. pptx | fixed (committed fixture), rerun pending | |
| 4. raster dwg import history | repaired | raster 228/228 |
| 5. DWG third-party oracle | pending | |
