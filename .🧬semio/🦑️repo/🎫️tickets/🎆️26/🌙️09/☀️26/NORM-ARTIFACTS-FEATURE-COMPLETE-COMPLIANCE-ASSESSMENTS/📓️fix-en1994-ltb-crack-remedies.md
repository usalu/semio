# Fix — EN 1994 LTB + crack remedies (girder-G1)

## Blockers
- en1994.6.4.ltb.girder-G1: ltbLengthM binary search was inverted (failed mid raised lo instead of lowering hi), so l_req stayed at current 8 m (no-op). Even with a correct search, chi_LT to 1 cannot clear hogging M_Ed above M_pl,Rd. Remeding steel.wPlYM3 alone is overwritten by SteelSection::resolve() for catalogue designations (HEB400).
- en1994.7.4.crack.girder-G1: barSpacingM remedy set s_req = s_max/scale, then util was recomputed as (s/s_max)*scale, which floated just above 1.0 and Fail under utilization <= 1.0.

## Fix
1. LTB: correct max-L search when min L clears; else OneOf steel.designation to heavier HEB (catalogue HEB450-HEB700) filtered so option 0 clears; qAreaPa fallback. chi_lt honors L down to 0.05 m.
2. Crack: util = max(as_limit/As, s/s_limit); remedy spacing to 0.98 * s_limit.
3. Test bridge_girder_ltb_and_crack_remedy0_clear.

## nx
Summary [   0.887s] 76 tests run: 76 passed, 0 skipped
