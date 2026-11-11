# BIM 114 clearance formula and equation audit

The clearance equation now groups the support half-widths: s − (r₁+r₂) = w. The prior displayed s − r₁+r₂ gave the wrong result under standard precedence, although its numeric row correctly showed 4.75 − 0.30 = 4.45 m. The original production partial 40 demonstrated the issue and is stale after this correction.

The complete information-scene equation and unit audit checked the authored values and frozen model: the clearance and required-width equations use metres, the fit comparison matches the model clearance calculation, and the current grid is four 4.75 m bays over 19 m, leaving zero symmetric remainder. The assumed vehicle envelope is 1.75 m wide, with 0.5 m per-side margin; 2.75 m is less than 4.45 m clear width. No further grouping or unit error was found.

The language-neutral schema and fixture exercise unequal support radii. The original source failed against independent Shapely segment lengths (4.85 vs 4.45 and 4.88 vs 4.66); the corrected actual-source expression passes. A second permanent test checks the native clearance function, required spacing, fit, grid, and units against independent geometry. Both tests passed in the Python 3.12 production environment. The BIM test aggregator registers the suite once.

Fresh Cairo captures of the actual-source row fit within the authored panel in English and German; respective gaps are 0.0498 and 0.1373 scene units. An apparent missing-glyph observation in the inline German preview was disproved by the current PNGs: all five symbolic token boxes have identical nonblack counts in both languages (87, 18, 353, 50, 85), with alpha 255. No source defect was indicated.

The earlier BIM 114–116 render and unfiltered full suite were deliberately stopped before the edit and are incomplete, not passes. Production must rerender the new source; final visual and freshness review and the full suite remain required. The 19 frozen model/cache inputs were rehashed with zero mismatches. The active 117–119 segment dependency closures exclude this source and its test aggregator.

Changed or created files:

- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py — SHA-256 f97f5c35cf8835918fe8d7cc0329e47dc062fef265ae7e49e490f2bd7293709c.
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py — SHA-256 16d3b9275bfdd1cdafd2106cc3bb05ab9df6c8f874f40b46bacf863926db75b5.
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🧪️tests/📏️clearance/🐍️.py — SHA-256 84da83231baa44b22a824e988c7cbc426f5a44f8f454464fc99d8deb821a4e0a.
- Same test directory 🔣️.json — SHA-256 28d73b167b3dc74b87bbf2e70327e134fb6a096468619ce39fdd239ef3a0e558; 🔣️.schema.json — SHA-256 48915d653021437de5f200922f71dc0627992d4f5019b897c7313ac1da71b454.

Retained ticket inputs: bim114-formula-cases.json, bim114-formula.schema.json, bim114-formula-test.py, bim114-formula-layout.py. Generated captures and logs are under 🗑️generated.
