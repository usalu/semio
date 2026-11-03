# r2 vf3: staffage TypeScript suites

Before (first run): lib vitest rc=1 (3 failed, 1100 passed, 11 skipped; all in export-performance), React rc=0 (107 passed, 1 skipped), catalog check ran clean through 45/147 before the process vanished (no fail lines; the earlier "width is not defined" in round-1 is gone).

After:
- Catalog check (full, 147 casts + manifest): rc=0, no failures. oil-pumpjack, hot-water-cylinder and mvhr-unit all pass; the manifest/asset mismatches g4 reported no longer reproduce (fixed by pa/g4 before my run; I made no asset edits).
- Export-performance suite: re-run alone 28/28 passed (it had been edited by pf between runs; the three first failures were stale in-flight state).
- Full lib vitest, second run: 2 failed, 1103 passed, 11 skipped.
  - thermal-span test (`🧪️tests/🌡️thermal-span/🟦️.ts:121`): course manifest has spans [25, 78]; `physics-fundamentals-kettle` in the visuals manifest (line 60) uses 78. Squarely agent ts (thermalSpanK) in-flight, not touched.
  - cast-export "text-free" test (`root is not defined`, line 252): fixed by another agent during my run (test now only checks --lang refusal, line 258).
- React vitest: 107 passed, rc=0.
- `📐️measure` syntax error: not present (catalog and vitest import it fine).
