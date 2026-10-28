# Shared Worked Example Ledger

The worked example and exercise use the same two column ledger renderer, arrow identifiers, 64.6 mm writing rules, and 6.3 mm row pitch. The example shows six filled values outside its graph; its graph shares the exercise arrow renderer.

The language agnostic fixture defines all six directed edges, the three row ledger, and the argument mapping. The verification is expected to fail before implementation and pass afterward. The production test owner will independently validate the fixture against a third party PDF parser after compilation.

The preimplementation test failed on the old two argument ledger entry, as intended. After implementation, all neutral geometry contracts pass. The shared callback uses an empty third argument for the twenty exercise entries and six concrete values for the worked example. Argument order remains 12, 21, 23, 32, 13, 31; visual order is clockwise in the left column and reverse in the right column.

K16 selected edges and complete graph code were left byte for byte unchanged. The worked table also retains its eight rows, sixteen millimetre header, half scale shapes and absent header identifiers.

Native DE and EN compilation exposed one normal space before the full width ledger (2.13867 pt overfull). Moving noindent after setup keeps row count assignment whitespace in vertical mode. Value baselines are lifted 0.8 mm above the original rules to clear typographic descenders; arrow identifiers, rule positions and dimensions remain unchanged.

The native PDF exposed TeX math font fallback from text 8.5 pt to math 8 pt. Filled values now use standard 9 pt for both text and math, meeting the minimum 8.5 pt requirement without changing row positions, baselines, rules, identifiers or graph geometry. All native variants require recompilation after this font correction.
