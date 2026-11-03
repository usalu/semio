@capability-colour-maps
@oracle-python-cie-wcag-machado
@comparison-floating-point-v1
Feature: Colour maps are accessible by construction
  Ramp samples, WCAG contrast and CVD distinguishability must equal an independent Python implementation of the published formulas, and satisfy the accessibility thresholds in every theme variant.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Ramp samples and metrics agree with the oracle: Given every ramp in light, dark and high-contrast variants When the Rust samples, contrast ratios and colour differences under three deficiencies are computed Then they equal the Python results
  - Contrast against the background: Given each ramp and its variant background When contrast is measured at nine samples Then it is at least 3 to 1 in light and dark and 7 to 1 in high contrast, ignoring the neutral centre of diverging ramps
  - Distinguishable under colour-vision deficiency: Given sequential, diverging and failure ramps When protan, deutan and tritan vision are simulated Then neighbouring samples differ by at least 2 and the end points by at least 30 in CIE76
  - Utilisation break at one: Given utilisation ratios just below and above 1 When they are coloured Then the failure side uses the failure ramp, differs by at least 15 from the safe end under every deficiency and carries diagonal hatching
  - Colour is never the only channel: Given pass, warning, fail, not-applicable and each of them with an unverified basis When their cues are built Then glyph and hatch pairs are pairwise distinct and the labels exist in English and German
  - CIEDE2000 equals the published reference: Given the Sharma, Wu and Dalal CIEDE2000 test pairs When the colour difference is computed in Rust and in the independent Python oracle Then both equal the published values to four decimals
  - Status colours under protan, deutan and tritan simulation: Given the pass, warning, fail and not-applicable colours of each theme variant When protan, deutan and tritan vision are simulated Then every pair differs by at least 5 in CIEDE2000, pass and warning against fail by at least 20 and 16, and the Python oracle agrees
