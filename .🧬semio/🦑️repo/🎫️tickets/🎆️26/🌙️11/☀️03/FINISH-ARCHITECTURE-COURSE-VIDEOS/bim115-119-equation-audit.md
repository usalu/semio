# BIM 115–119 bounded equation and unit audit

This read-only source audit checked authored display equalities, quantity units and their native source relationships in objects (115), representations (116), models (117), consistency (118), and exchange (119). It did not compute the full native geometry, inspect every rendered frame, or certify the final lesson outputs.

One concrete issue was found: representations source line 928 displays the W-S wall physical thickness and its 1:20 paper length separated by an equals sign. The underlying native LS-FACADE-29 layer widths sum to 0.29 m, so the paper image is 14.5 mm, not a physically equal length. The current integer formatting also drops the half millimetre. The adjacent 1:200 examples already use an arrow. A source-local arrow and one decimal digit are pending; no source has been edited yet.

Other inspected relationships include source 115 tributary load density times area producing force; source 116 wall gross area minus opening, glass width times height, area share, and volume subtraction; source 117 slab gross minus void, midsurface offset, load density and tributary force; source 118 its explicit 17 cm structural reference and 9 cm reserve, matching the frozen model layer values; and source 119 model-derived dimensions and schema quantities. No other concrete mismatch was established from these bounded checks. Intermediate rounded values and full production frames remain subject to actual-render review.

The runtime segment-state closure check found zero references to the 116 representations source in ordinals 114 and 115, and one in 116. It does not establish master freshness or completion.

## Corrected paper-scale label

A permanent language-neutral schema and conversion cases first failed against the actual source equality and then passed after the source-local arrow and one-decimal change. Independent Shapely line lengths verify 0.29 m at 1:20 maps to 14.5 mm on paper. Fresh native Cairo output of the actual source label reads “0.29 m → 14.5 mm” in English and “0,29 m → 14,5 mm” in German; text bounds were 2.236 × 0.186 and 2.231 × 0.205 scene units, respectively. These isolated captures do not substitute for the full production-frame layout review.

The permanent test is registered once in the BIM aggregator. All 19 frozen model/cache inputs still hash identically.

Changed/created files:
- 👁️representations/🐍️.py: SHA-256 10ef5aa1d2fca55f953cae5da3e0abdf0c670539400487c049f85c071281fdcb.
- 🧪️tests/🐍️.py: SHA-256 645f033f0c571f33fab8b4a18410b3db7dffa173c1208b37cec5d971b3c90cf9.
- 👁️representations/🧪️tests/📐️paper/🐍️.py: SHA-256 a7a2e4e45b715755108ca9c0ff896668a59ecd4b2a0bda2090260512dd70966b.
- Same test directory 🔣️.json: b203e54843181561024c2bde528f7b70cb9051329d431e5acdc099405de479ca; 🔣️.schema.json: 64fe5912b85d064738d7886280ec4c4fc0ae5d96cbcbbf74b948c009ff446d5c.
- Retained ticket input bim116-paper-layout.py; generated Cairo captures under 🗑️generated.
