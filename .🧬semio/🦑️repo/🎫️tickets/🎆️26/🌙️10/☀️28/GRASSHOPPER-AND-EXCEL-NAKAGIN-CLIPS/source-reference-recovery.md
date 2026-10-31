# Current Evidence Update

All140 visible window-bearing orientations are now positively covered (107 unchanged plus33 corrections); A4/A5/A13 S0 omissions are derived by documented count7 plus seven distinct positive physical capsules. Earlier occlusion-only/16-positive observations below are historical intermediate methods, superseded by all140-window-coverage.md and all-storey-pose-evidence.md. Hidden handing and metric uncertainty remain separate.

# Source Reference Recovery

Read-only reference analysis,2026-10-29. No production geometry or source copies. Temporary reference/analysis outputs stay in the ticket generated folder and are removed at cleanup. This report is saved early; GLUON feature registration remains in progress.

## Local Thesis

The two supplied bachelor-thesis PDFs are byte-identical:6,346,331bytes,65 pages,SHA25683ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3. Text was extracted from every page and the full figure list reviewed. No Nakagin/Kurokawa reference or architectural capsule drawing appears. The work concerns SemIO language design, planning/model concepts, implementation and a syntax example; figures are framework/concept diagrams and software statistics. A capsule substring false hit is the programming term encapsulation. Therefore these PDFs contain no recoverable original Nakagin section/dimension figure.

Inspected original paths:

- ♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎓️bachelor-thesis-ueli-saluz.pdf
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf

## GLUON Raw Evidence

[Public before-demolition whole-building GLB](https://gluon.tokyo/nakagin/models/NakaginCapsuleTower.glb), retrieved successfully:16,527,260bytes. Cinema4D official glTF exporter metadata. One merged mesh,329,124 position vertices,one embedded JPEG texture. Four named hierarchy nodes, no node transforms, no metric unit/scale metadata or semantic room identities. The mesh name identifies a merge of Drone/Roof/3F/Exterior-East/Pilotis captures.

Raw bounds are X[−.3849164,.3138013],Y[−.0101108,1.1445266],Z[−.2718509,.1558762]. Y is vertical. Total raw height1.1546374 is normalized, so geometry cannot be treated as metres directly. This does not prevent recovery: scale/orientation must be estimated from identifiable capsule features and checked against independent primary dimensions. Analysis is proceeding with actual triangle/planar-feature distributions rather than declaring merged geometry unusable.

## Recoverable Scan Registration and Floor-3 Findings

Analysis inputs retained in ticket root: scan-reference.py,scan-features.py,scan-slices.py,scan-lowest.py,scan-lowest-quantify.py,scan-phases.py,scan-B-phases.py. Raw GLB,JSON distributions and reference projection images remain temporary generated outputs. No scan mesh is used as a production asset.

Normal-weighted plan axes are about.25 degrees from raw X/Z. Raw core centres lie near X±.12,Z0. A (tall) is positiveX. A working horizontal registration of existing authored metre coordinates is x_scan=−x_model/46.25−.12,z_scan=y_model/46.25. Its scale uses the current11.1m core spacing, so this is inferred registration, not independent proof of that metric spacing. Existing four physical pose footprints align with observed boxes to roughly.003–.005 raw units (about.14–.23m under this working scale), allowing for scan noise and hardware-inclusive extents. No geographic north is asserted.

Signed horizontal normals are essential: many strong lowest planes around rawY.223 are DOWNWARD undersides of the first full row, not roofs of floor3. Initial unsigned histograms are not used as final phase evidence.

| Observed lowest box | Approximate raw plan bounds | Signed surface evidence | Existing physical pose / correction |
| --- | --- | --- | --- |
| A scan-east | X.18–.28,Z0–.06 | Downward peak.18075; upward roof.23875 | A W/Slash, phase1 |
| A scan-north | X.12–.215,Z.06–.12 | Downward.17875 (rough)/.18325; upward roof.23825 | A N/J, phase1; replaces unsupported floor3 W/S pose |
| B scan-west/north | X−.245–−.18,Z0–.10 | Downward.18125; upward roof.23775 | B E/J, phase1 |
| B scan-west/south | X−.28–−.18,Z−.06–0 | Strong downward.20275; roof is occluded in merged scan | B E/Slash, phase2 |

The two A roofs agree within.0005 raw; their undersides are within the rough scan-floor spread. The B underside separation is.0215 raw, approximately one-third of the repeated pitch. These factual relative positions/heights supersede the previous assumed same-facade A pair and the initial flat floor3 assignment. Exact metre coordinates and room suffixes remain unproven; the topology/relative phase is directly recoverable.

## B Normal-Storey Handedness, Not Sparse-Only Exception

The scan confirms that the native B phase1/2 schedule is reversed globally. It is not enough to flip only floor3. Fixed registered footprints were tested with actual signed upward horizontal surface area on three separate normal storeys:

| B footprint | Floor4 roof rawY | Floor6 roof rawY | Floor11 roof rawY | Recovered phase |
| --- | --- | --- | --- | --- |
| N/Slash |.29775 |.41675 |.71425 |1 |
| S/J |.31675 |.43575 |.73325 |2 |
| E/J (north position) |.29775 |.41575 |.71425 |1 |
| E/Slash (south position) |.31625 |.43575 |.73475 |2 |

Paired separations are about.019–.0205 raw, one-third pitch. Independent downward evidence includes B N/Slash.36225/.48125 and B E/J.36125/.42175/.54075/.66075, matching phase1; B S/J has a strong first-row underside.26225, matching phase2 rather than1. At top floor11 the broad exposed roofs make the order especially clear. Lower-row tiny exposed roof fragments are corroborated by the underside data; no claim rests on a missing fragment alone.

Therefore B should mirror A in X (reversing circulation winding), rather than merely rotate A180 degrees while preserving winding. B nonzero sublevels must swap1↔2 across normal floors: N/Slash and E/J become1, S/J and E/Slash become2. Phase0 remains0. This is a source-level correction required before rebuilding stairs around an incorrect authored schedule.

## Raw Pitch and Height Constraint

A signed-underside phase1 sequence at rawY.24325,.30375,.36375,.42325,.60225,.72075 has gap-normalized pitch values.0605,.0600,.0595,.0596667,.05925. Their median is.0596667; least-squares pitch is.0596584507. B E/J upward roofs.29775,.35725,.41575,.47575,.53525,.59525,.65475,.71425 give adjacent median.0595, with individual observed increments.0585–.0600. Thus the robust raw pitch is approximately.0595–.0597, not a metre value.

Raw maxima: A1.1445266008; B1.0121959448. With representative ground mode.0025, B/A height ratio=.8841264678. If A53.50m is used as an external calibration, B total is47.300766m. This is a derived scan ratio, not a recovered printed B-head digit. It robustly constrains the short tower to roughly47.3m under that calibration, subject to ground/top registration and export normalization uncertainty.

Capsule roof patches are recoverable but their raw bounding lengths include scan/hardware edges, while4×2.5m is a nominal envelope. Typical patches span about.058–.063 by.088–.094 raw. Blindly setting4m equal to the full longer extent yields scale42.6–45.5m/raw, whereas hardware-inclusive4.2–4.246m gives44.7–48.3. This spread straddles the scale needed to interpret pitch as2.75 or2.90. A precise as-designed versus as-built pitch conclusion therefore still needs an identifiable shell-only edge or independent metric registration; normalized GLB alone does not justify exact metre claims. Its unlabelled merge does not prevent the strong relative-phase and position recovery above.

## A Seven-Unit Floors4,5,13

The formerly omitted E1/L footprint is positively present in the scan on all three floors. A fixed front-plane probe at rawZ−.09,X0–.06, with only the appropriate body-height band and actual near-vertical triangles, finds surface areas.0023219,.0022633,.0019739raw² on floors4,5,13 (73,65,204 triangles). Thus the existing E1 omission is contradicted by positive geometry, not merely by a null sample elsewhere.

Top floor13 has seven directly exposed upward roof surfaces distributed as follows:

| Component / supported footprint(s) | Mean normalY | Actual roof area raw² | Raw bounds |
| --- | --- | --- | --- |
|3319,E0/J +N0/Slash |+.995 |.010047 |X−.000235–.119391,Y.812429–.815916,Z−.000168–.149400 |
|2997,N1/J +W0/Slash |+.994 |.009984 |X.121889–.272836,Y.832720–.836227,Z−.000068–.149493 |
|3372,E1/L |+.995 |.005007 |X.000575–.058751,Y.811840–.815620,Z−.091577–.001493 |
|5046,S1/P |+.993 |.005027 |X.122171–.180787,Y.852596–.855278,Z−.149533–−.058867 |
|3007,W1/S |+.995 |.004755 |X.180490–.241286,Y.852662–.855559,Z−.090813–−.001441 |

The two connected double-roof components each occupy the union of two touching nominal footprints; their area is about twice a single roof. All seven canonical positions except S0/Backslash are positively covered. S0 target front-plane at rawZ−.15,X.06–.12 has zero surface support at its expected floor13 body height. Together with the documented count7 and visible top-roof coverage, this strongly supports S0 as the top-floor omission rather than E1.

On floors4/5 S0 also has zero sampled front support, but W0/N1 have missing/weak front samples too: some lower facade surfaces are occluded or absent from this merged capture. Therefore lower negative samples alone are not proof of missing capsules. E1 restoration is directly supported; S0 is the strongest lower omission candidate, reinforced by top-floor pattern, but must retain inferred provenance unless additional visible wall/plan evidence closes the coverage gap. No automatic deletion is justified solely by a zero face-area probe.

## Independent Metric Attempts and Their Limits

### Full roof components

The horizontal components were extracted from actual topology after exact-position welding, without fitting a model ROI to truncate their bounding rectangles. Five isolated intact roof patches have long spans.0893727,.0902310,.0906658,.0928383,.0930701raw (median.0906658) and short spans around.058–.061raw. They are overall scanned roof/envelope bounds, not automatically the nominal4.00m shell-only edge.

Conditional scale and pitch calculations:

~~~text
S_nominal =4.00/.0906658 ≈44.118m/raw
P_nominal =.05965845*S_nominal ≈2.632m
A_height_nominal =(1.1445266−.0025)*S_nominal ≈50.38m

S_overall =4.20/.0906658 ≈46.324m/raw
P_overall =.05965845*S_overall ≈2.764m
A_height_overall =(1.1445266−.0025)*S_overall ≈52.90m
~~~

The4.20m value is the longest M+ A806 overall dimension, interpreted as the obvious long module axis; this inference is explicit because the catalogue does not label axes. At S_overall a.0583raw short span is2.70m, compatible with another published overall dimension. The source-native capsule GLB also has hardware-inclusive length about4.246m, but that kit is not an independent as-built survey.

Thus a literal4.00m calibration of full scanned bounds is inconsistent with the sourced overall height. A4.20m overall-envelope interpretation conditionally favours about2.75m as-built pitch, while a shell-only4m edge has not been cleanly isolated. This is a real possible design/as-built or extent-convention discrepancy with the clear2.90 printed drawing; it is not resolved by assuming11.1m core spacing. Metric confidence remains weaker than relative phase confidence.

### Several porthole dark-disc fits

A barycentric UV rasterizer projects the actual textured +X facade into raw Z/Y coordinates at10,000pixels/raw. This avoids centroid-colour block artefacts and preserves the mesh UV correspondence. Six window regions were inspected; least-squares circles were fitted to connected inner dark regions over multiple intensity thresholds, with radial intensity profiles as a separate boundary check.

Several intact circles fit geometrically well (roughly1–3pixel RMS), but their photometric dark-disc radii are not stable: top about110–112pixels, next127–129,middle135–138,lower134–140; broken and lowest windows are unstable. The top aperture visibly has a broader pale/reveal band, and blinds/reflections/damage alter the dark region. Fitting every dark patch to1.30m would imply incompatible scales from about46 to59m/raw despite excellent circle residuals. A low residual does not establish that the fitted dark boundary is the physical1.30m mouth.

Middle/lower fits alone would suggest about2.85–2.90m pitch, but selecting only those would be circular cherry-picking. The multi-window check therefore rejects photometric black-disc radius as a sufficiently uniform calibration feature in this reference. A physically identified aperture edge/convention, or independent measured registration, is still needed to settle exact metric pitch. Outer frame must not silently be assigned the1.30m clear diameter either.

## Scope of What Is Now Recovered

Merged/unlabelled geometry did NOT make factual recovery impossible. The analysis recovered four sparse-floor3 pose/phase relationships, B global phase handedness over three normal storeys, positive contradiction of the A E1 omissions and strong top13 S0 omission, plus robust raw pitch and a short/tall height ratio. Remaining limitations concern exact metric convention, lower occluded-mask identity and unresolved printed glyphs; they must not be conflated with the now-recoverable topology.

## Registered Ground, Roof And Podium Datum (Latest Evidence)

This supersedes the earlier confident “2.90” transcription. Focused rereading favors **2.80 m**, independently supported by the before-demolition mesh. Cauchy-weighted ground-plane fitting over 3,029 nearly horizontal triangles gives raw Y = .002517006 X + .001686168 Z + .001471174, weighted RMS .000988455. Supported A apex is Y1.144526601; local ground at the A core is .001773215; the span is1.142753386 raw units. Scaling that span to the original section's42.14+11.36=53.50m gives46.81675036m/raw. Multi-floor pitch becomes2.79301479m (least-squares A),2.79339944m (A median),2.78559665m (B median). These observations support the2.80 reading within reconstruction/scan error; they do not independently prove every raster glyph.

The same registration gives B apex height47.3333m. The degraded B head reading10.83 is plausible, not a recovered exact measurement. Ground4.?0 remains ambiguous. The section's clearly legible3.72 and5.00 remain separate from these ambiguous endpoints.

The exposed podium flat roof is near raw Y.1665, about **7.71–7.72m** above the fitted ground. Its raised perimeter/coping is near .1746, about8.09m. The latter must not silently become the capsule-support datum. Registered first A phase-1 roof: (.23875−.001773215)×46.81675036=11.0940m. A7.72 datum +2.80/3 phase +nominal2.50 shell gives11.1533m, residual+.0593m; an8.12 datum gives+.4593m. Thus7.72 is the better supported flat-roof datum, with an approximately.40m raised edge modeled separately.

The signed lowest underside near .18075 gives8.3791m and an observed roof-to-underside span2.715m. Current capsule_assembly nominal shell spans Z0..2.5; its service rails are above zero, and this source read does **not** establish .2m lower hardware. The difference is an unresolved physical-envelope/finish convention, not permission to invent a hidden .2m datum correction. Flat-roof/slab-top and shell-floor/finished-floor values must remain explicit.

## All-Storey Positive-Evidence Coverage

Retained input scan-all-capsules.py writes generated/gluon-all-capsule-coverage.json with the exact model JSON hash, all140 current physical centers/rotations/families/phases, signed upward/downward material areas at the authored and both alternative phases, and expected window-facing material evidence. Registration uses physical XY coordinates, not item indices; item labels vary between storeys. Conservative footprint interiors exclude neighboring edge strips. This is positive material evidence, not a complete capture or a surveyed metric fit.

| Tower | Floor | Authored count | Positive roof/underside | Sparse/occluded |
|---|---:|---:|---:|---:|
| A |3|2|2|0|
| A |4|7|5|2|
| A |5|7|1|6|
| A |6|7|3|4|
| A |7|8|1|7|
| A |8|8|3|5|
| A |9|7|0|7|
| A |10|8|1|7|
| A |11|8|1|7|
| A |12|7|2|5|
| A |13|7|7|0|
| B |3|2|2|0|
| B |4|8|6|2|
| B |5|8|3|5|
| B |6|7|0|7|
| B |7|8|3|5|
| B |8|8|3|5|
| B |9|7|1|6|
| B |10|8|2|6|
| B |11|8|8|0|

Total54 positive and86 sparse/occluded. The conservative alternative-phase threshold found no new strong contradiction candidates. A missing face is not a missing capsule. Facade material at an expected window-bearing plane supports the shell-facing orientation only; it does not certify a circular aperture, hidden entry hand or interior fitout. Earlier explicit B multi-floor signed phase ordering and A floor3/top13 findings remain stronger than these automated sparse-case classifications.

## Actual Bridge Section Evidence

scan-bridges.py intersects every source triangle with raw X0, rather than using bounding rectangles. Three bridge walking surfaces occur around raw Y.3495,.5272,.7058. Their separation agrees with three regular storeys. The top bridge's near-flat contiguous section extends approximately Z−.05425..−.00714:2.21m under A-height calibration, centered near−1.44m. First bridge positive flat samples already span1.54m. The present1.25m reconstruction is too narrow against this positive evidence; native2.30m is compatible in order and requires coordinated endpoint/door reconciliation. A central section does not establish the native L-return; its actual endpoint footprint remains to be inspected. Rough rails, edge capture and scan distortions prevent claiming millimetre-accurate widths.

## Window-Facing Contrast And Bridge Return, Latest Read

The aperture-facing pass samples the actual embedded UV texture at triangle centroids with material-area weighting. Its conservative positive criterion is dark fraction>.55 inside a.46m radius, dark fraction<.40 in the.73–.93m annulus, contrast>.25, and both sampled areas>.00008raw². The expected window-bearing plane must be within.28m and its signed outward normal dot>.65. It finds **16 positive dark-disc/light-surround orientations**, not124 proven wrong orientations: A6 facade1/item0 J; A7 facade1/items0 L and1 J, facade2/items0 Backslash and1 P; A10 facade1/items0 L and1 J; A11 facade1/item1 J; A12 facade1/item0 J; A13 facade0/item0 Slash; B5,6,8,9,10 facade3/item0 L; B7 facade3/item1 J. Raw per-capsule areas/fractions remain in the generated coverage data. This photometric contrast supports aperture-facing orientation, but is not a fitted circular rim or evidence for concealed entry handedness. Blinds, damage, occlusion, capture holes and metric registration can suppress it.

The all140 signed-surface test uses shrunken nominal local bounds |x|<1.10m and |y|<1.84m, |normalY|>.90, and roof/underside height windows±.005raw. Underside is sampled.058raw below the measured roof; this is an observed envelope offset, not a product shell parameter. Positive score threshold is.00025raw². Conflict candidates require own score<.00012raw² and an alternative phase>.0007raw². No candidate met that conservative contradiction rule. Absence under these thresholds does not certify absence.

The texture-colored actual bridge roof-plane projection (generated/gluon-bridge-top-planes.png) resolves the earlier return uncertainty: **all three decks have an L-return at the scan-positive-X/A end toward negative scanZ/authoredY**. Approximate raw return bounds X+.03..+.06,Z−.11..−.05 are compatible with the native1.25×2.5m form. They are rough capture bounds, not survey coordinates. The main top bridge central section spans≈2.21m, supporting the native2.30m order. These are actionable footprint facts for the coordinator's reconstruction, not automatically final door endpoints.
