# Audit: full staffage suite (2026-11-02, QA4)

## How it ran
- `repo run @teaching/staffage:test --lang=en` passed the capacity gate (about 10 minutes) and then failed in the repo tooling itself: `OwnedCommandFailure: repository:command failed (1)` after `Current immutable input capture work refused` (dependency capture, `📦️dependencies/💾️storage`). No test ran. Owner: repo framework (not the staffage archive).
- Fallback, one process from the staffage package with the archive config: `vitest run --config ../../🧪️tests/🎚️config/🟦️.ts` (plain `vitest run` finds no files because the include is relative to that config).

## Result
- Test files: 64 passed, 5 failed (69).
- Tests: 1492 passed, 20 failed, 11 skipped (1523). Duration 187 s.

## Failures by group and likely owner

| Group | Failing | Message | Likely owner |
|---|---|---|---|
| three-views, `market-parasol` (1 sweep case, 1 consistency case counted in the 7) | spread guard at every pose and clip instant, 1:50 and 1:100 (and coarser): elevation y [0, 2.46] vs front y [0, 2.96], front x [-1.00, 1.00] vs plan y [-1.18, 1.18], furled elevation x [-0.2, 0.2] vs plan x [-0.11, 0.15] | O1 (object worker, report-O1) with FW for the spread/solid placement |
| three-views, `seated-reader` | 1:500 seated-upright: elevation y 1.342 vs front y 1.296; front height 1.296 vs dimensions 1.340 | P2 (report-P2) |
| three-views, `running-child` | 1:500 standing-ready: elevation x [-0.152, 0.120] vs plan x [-0.148, 0.184]; plan length 0.332 vs 0.281 | P2 |
| three-views, `pregnant-person` | 1:500 hand-on-back: elevation x [-0.353, 0.362] vs plan x [-0.356, 0.438]; plan length 0.793 vs 0.720 | P2 |
| three-views, `person-with-umbrella` | 1:50 umbrella-up-still: front/plan lateral extent 0.584 vs dimensions 1.000; plan x [-0.143, 0.824] vs elevation [-0.225, 0.824] | H2 (report-H2; revolved canopy placement after the single 3D path, FW to confirm) |
| three-views, `goat` | 1:50 standing-alert: front y [-0.003, 1.557] vs elevation [0, 1.092]; front width 0.740 vs depthM 0.340 | A1 (report-A1) |
| export-performance (11 of 30): feature width (10) and exported frames digest (1) | pre-optimisation module vs new module differ (e.g. 2.001 vs 2.000 mm, 0.397 vs 0.500 mm, 9.937 vs 9.920 mm); pinned digest of `walker-line/0000.svg` differs | FW (corrected `featureWidthMm`, see "Feature width measure" in fw-status) and TS (pinned vectors and reference module must be regenerated for the corrected measure) |
| section-views (1 of 18) | marker at the plane of the section, plan 1:50: line offset 0.010 vs tolerance 0.001 | FW (side/plan marker placement) or TS (tolerance), not an asset |
| caption-fit (1 of 3) | text width vs PyMuPDF Helvetica: "Wandhängender Brennwert-Gaskessel" differs by 5.56 vs 0.167; poppler/MuPDF logs `No display font for 'ArialUnicode'` | TS / environment (system font fallback on this Windows host); not an asset |
| print-size (suite does not load, 0 tests) | `Cannot find package 'd3-format'` imported by the test | TS / environment (dev dependency not installed; the same dependency sync is stale in the repo run) |

## Notes
- H1 assets (child-on-kick-scooter, courier-with-hand-truck, cyclist, elderly-with-cane, jogger, kneeling-gardener, leaning-person): no failure in any group (three-views 14 of 14 passed for them, suitability inside `check` ok).
- I changed nothing in this task (test infrastructure and asset folders of others untouched). Raw outputs are deleted.
