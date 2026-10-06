# Area-Proportional Venn Before-Source Drawing Audit

This bounded investigation used existing compiled artifacts only. No product/fixture source mutation and no compiler job were performed. Both baseline staged math styles are SHA2566BEA147A9DAF1A5356EBE50FDB94453F7CAE1DFE3777C191D06D9F7D751522B2. Existing neutral control math-proportional-computed supplies sets A/38,B/26 and intersection AB/12 to an80×40mm VizFigure. Both baseline PDFs have12pages; the control is on page3. Light PDF SHA256B25D85D8340ACD95E01856C457ADC19373A3DA6C12C1916A4F471D817D50FB78; dark D120DE0BC740A6FCCEE6DC7F2C59EE218A5A1E56F52840BBD8932C9ED1E7F869.

## Actual Rendering Evidence

Both themes native probes report circles[-0.1226354444153996,0,-0.000064025869659] and[0.1226354444153996,0,9.098872110553622], centre distance0.2452708888307992. Actual light PDF.js path decoding finds four four-cubic circle paths: one fill/stroke pair each. First path width0.0003399848937988281pt,height0.0003000000142492354pt; second width51.58456993103027pt,height51.58456039428711pt. Its reconstructed radius9.098944973945617mm matches the probe within PGF quantization. The first circle is actually near-zero in the PDF, so this is a rendering defect rather than solely metadata corruption.

Poppler180dpi both-theme page3 render commands exited0. Both images were actually viewed. They show a near-zero red mark and one blue circle at the canvas lower-left, extending outside the figure frame. Temporary rasters: generated/venn-baseline-visual/light-page3.png and dark-page3.png. Complete operator dump and concise circle summary are temporary generated evidence; scalar measurements are retained here.

## Source Attribution and Oracle Recommendation

Before-source lens-distance helper line1322 writes the overlap-area iteration result into sci_a_fp. Caller set-proportional line1424 passed lazily evaluated expressions containing the same sci_a_fp used for first radius. Every subsequent area evaluation and final first-circle draw therefore reads the overwritten scratch value. The helper is explicitly an equal-circle routine while this control has unequal radii. Its formula also contains an explicit acos result×pi/180 factor; angle-unit correctness must be established in the independent oracle rather than assumed.

Existing third-party d3-scale scaleSqrt([0,38],[0,11]) returns radii11mm and9.098872110553623mm. A separate analytic unequal-circle diagnostic gives target overlap120.04190876348366mm² and distance10.211769047181921mm. The observed distance with those intended radii would yield260.0908023208814mm² (full containment), rather than the requested12/38 first-set fraction. This diagnostic formula/solver is independent of native source but is not claimed to be a third-party overlap oracle. No venn.js dependency was found in the inspected root/print package declarations and Bun lock.

Recommended neutral regression: declared set sizes/intersection, D3 radii, independent true unequal-circle overlap/distance (and containment/disjoint boundary cases), actual PDF.js circle bounds/centres plus figure containment in both themes. Freeze input radii/target values during solving and retain them for drawing; use a solver that preserves the declared unequal-set areas. Keep Native parser ownership and existing native owner; no new runtime module/library is needed.
