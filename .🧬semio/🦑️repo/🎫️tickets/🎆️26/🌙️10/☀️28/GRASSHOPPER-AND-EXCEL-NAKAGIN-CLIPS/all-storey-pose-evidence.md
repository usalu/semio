# Positive All-Storey Pose Corrections

Actual before-demolition UV triangle contact sheets and signed roof/underside planes corroborate the following24 changes. These replace earlier roof-occlusion-only uncertainty for these cases. XY and yaw select native canonical physical footprints; they are derived registration values, not surveyed centimetre accuracy. Every record retains its authored phase and Z, has positive circular rim/window evidence, and does not duplicate another occupied same-floor/phase body. Counts, IDs and room-program assignment are outside this audit patch scope.

|Tower|Floor|Facade/item|Old→new family|Centre XY m|Yaw°|Phase|Retained score raw²|Evidence alternative sheet/cell|
|---|---:|---|---|---|---:|---:|---:|---|
|A|4|0/1|J→Backslash|[-12.45, 4.85]|0|1|0.001155|gluon-aperture-alternative-1.png/2|
|A|4|3/0|Slash→L|[-15.15, 2.15]|0|1|0.000444|gluon-aperture-alternative-1.png/5|
|A|7|0/1|Backslash→J|[-13.25, 4.05]|90|1|0.000414|gluon-aperture-alternative-1.png/13|
|A|7|3/0|L→Slash|[-15.95, 1.35]|90|1|0.001056|gluon-aperture-alternative-1.png/16|
|A|9|0/1|J→Backslash|[-12.45, 4.85]|0|1|0.002221|gluon-aperture-alternative-2.png/1|
|A|9|3/0|Slash→L|[-15.15, 2.15]|0|1|0.000921|gluon-aperture-alternative-2.png/3|
|A|10|0/1|Backslash→J|[-13.25, 4.05]|90|1|0.001044|gluon-aperture-alternative-2.png/4|
|A|10|3/0|L→Slash|[-15.95, 1.35]|90|1|0.002297|gluon-aperture-alternative-2.png/7|
|A|11|0/1|J→Backslash|[-12.45, 4.85]|0|1|0.002243|gluon-aperture-alternative-2.png/8|
|A|11|3/0|Slash→L|[-15.15, 2.15]|0|1|0.001068|gluon-aperture-alternative-2.png/11|
|A|12|0/1|Backslash→J|[-13.25, 4.05]|90|1|0.001735|gluon-aperture-alternative-2.png/12|
|A|12|3/0|L→Slash|[-15.95, 1.35]|90|1|0.001150|gluon-aperture-alternative-2.png/14|
|A|13|0/1|J→Backslash|[-12.45, 4.85]|0|1|0.004877|gluon-aperture-alternative-2.png/15|
|B|4|0/1|Slash→L|[2.15, 4.05]|-90|1|0.001668|gluon-aperture-alternative-3.png/0|
|B|4|1/1|J→Backslash|[4.85, 1.35]|-90|1|0.000408|gluon-aperture-alternative-3.png/1|
|B|6|0/1|L→Slash|[1.35, 4.85]|0|1|0.002056|gluon-aperture-alternative-3.png/6|
|B|6|1/1|Backslash→J|[4.05, 2.15]|0|1|0.000905|gluon-aperture-alternative-3.png/7|
|B|7|0/1|Slash→L|[2.15, 4.05]|-90|1|0.001557|gluon-aperture-alternative-3.png/9|
|B|7|1/1|J→Backslash|[4.85, 1.35]|-90|1|0.001035|gluon-aperture-alternative-3.png/10|
|B|8|0/1|L→Slash|[1.35, 4.85]|0|1|0.001253|gluon-aperture-alternative-3.png/12|
|B|9|1/1|Backslash→J|[4.05, 2.15]|0|1|0.001167|gluon-aperture-alternative-3.png/16|
|B|11|0/1|Slash→L|[2.15, 4.05]|-90|1|0.004157|gluon-aperture-alternative-4.png/1|
|B|11|1/1|J→Backslash|[4.85, 1.35]|-90|1|0.004988|gluon-aperture-alternative-4.png/2|
|B|11|2/1|S→J|[2.15, -4.05]|-90|2|0.003762|gluon-aperture-alternative-4.png/3|

Phase evidence samples actual signed near-horizontal material in the candidate footprint, with the same conservative ±.005raw height windows and shrunken body bounds as source-reference-recovery.md. The retained phase has the highest combined roof/underside score in all24 cases. B11N1 has only a partial rim capture but a strong phase1 roof score; B11S1 front rim is visually unambiguous even though the coarse automatic gradient-circle fit is weak. Body containment/ports/whole140 collision checks remain implementation gates.

Inputs: scan-contact-sheets.py, scan-alternative-sheets.py, scan-confirm-poses.py. Generated JSON stores every exact XYZ/yaw/phase, three signed scores, image/cell and duplicate check. Seven current-pose and four alternative-pose contact sheets texture-map actual source triangles with barycentric UV interpolation; no generated building geometry or copyrighted mesh is reused at runtime.

The original16-positive dark-pixel threshold was incomplete and used an incorrect X-reflection normal sign for ±90° front walls. The analysis filter is corrected; its threshold now yields29 positives, but full textured rim inspection gives much more positive evidence and is authoritative for the24 corrections. Roof-phase54/86 coverage was unaffected. Negative/blank contact patches alone are never treated as absent capsules.

Initially unresolved white front panels on B south phase0 capsules/B10S1 were subsequently resolved by positive right-wall rims, documented below. Blank front material alone was not used to choose the alternate direction. Entries with indistinguishable exterior bodies but different hidden entry handing remain bounded by authored/native evidence.

## Nine Additional Positive Side-Window Corrections

The formerly unresolved B south front panels are resolved by inspecting both actual long walls. The right wall has an actual circular rim in all nine records below; the opposite wall does not show a rim. These change the aperture wall/family while retaining the body XYZ/yaw and phase.

|Tower|Floor|Facade/item|Old→new|Phase|Strongest signed phase|UV side sheet cell|
|---|---:|---|---|---:|---:|---:|
|B|4|2/0|Slash→P|0|0|0|
|B|5|2/0|Slash→P|0|0|2|
|B|6|2/0|Slash→P|0|0|4|
|B|7|2/0|Slash→P|0|0|6|
|B|8|2/0|Slash→P|0|0|8|
|B|9|2/0|Slash→P|0|0|10|
|B|10|2/0|Slash→P|0|0|12|
|B|10|2/1|J→S|2|2|14|
|B|11|2/0|Slash→P|0|0|16|

Evidence: gluon-unresolved-sides-1.png and generated/gluon-additional-window-candidates.json. All nine retained phases have the strongest signed material score; absolute side-circle height residuals against current outerfloorZ+1.25 are−.108..+.025m. Together with the24 above,33 corrected candidate wall orientations are positively observed. The remaining107 current expected window-bearing walls were inspected in the seven actual UV sheets and show an identifiable circular rim, including damaged/partly captured apertures. This establishes visible aperture-facing evidence for all140 current-or-corrected records, not exact historical hidden entry handing, sanitary fitout or surveyed window centres. Production changes and collision/port checks remain execution-owned.

## Correct Window Height Convention

The first circle-fit metadata incorrectly compared height above a roof-minus-nominal2.5m baseline with1.05m. Current model centre is1.25m above outerfloor, equivalent to1.05m above the+.20m finished floor. The analysis now records both baselines.23 usable fits in the original24 candidates give absolute registered centre residuals−.097..+.054m, median−.0084m against actual authored outerfloorZ+1.25. B11S1 has a weak automatic fit and is supported by its manually visible rim and phase2 planes. The apparent1.29–1.38m local heights are not a systematic.3m model defect. No vertical window-centre change is supported by these residuals. Corrected normal and height filtering produces34 conservative dark-disc/light-surround positives; actual full-rim UV evidence supersedes that incomplete dark-region threshold.

## Physical Casing Evidence

Native actual physical triangle bounds are2.60×4.20×2.50m; these differ from nominal analytical2.5×4 dimensions. Native/authored centre positions4.85/4.05 minus physical half-length2.10/half-width1.30 exactly meet the5.5m core half-side2.75; nominal4/2.5 requires unsupported.10/.05m collars. The independent untruncated scan roof long spans give4.184–4.357m (median4.245) under the A53.50m height calibration. Opposing normal-gated flat walls across36 captures give width median2.654m at the former46.25 horizontal registration (2.686 under independent A-height scale),10–90%2.645–2.706 before that rescaling;12 end-plane pairs give length median4.310m,10–90%4.148–4.369. Photogrammetric roughness/core-plane contamination prevents survey precision, but supports the native physical casing over nominal2.5×4.

The physical mesh must remain centered on the authored centre: extend.1m each end relative to the current nominal back frame, or derive a physical back origin centre−2.1. A naive4.2m extrusion from the old nominal origin shifts the whole body. The physical casing schema/API rewrite is coordinator-owned; nominal worksheet dimensions may remain explicit analytical inputs.

## Side-Window Metric Conflict, Independently Rechecked

Native actual P/S glazing disks independently fit offset.950000008m, radius.650000008m and localZ1.050000007m above native body origin (physical bottom−.20). This is genuine precise native evidence, not an assumed value. Actual pre-demolition UV rims conflict with its longitudinal placement.20 A P/S projections give local offsets.452–.588m (median.543) under canonical registration; nine B side views give.588–.724m (median.633). These preliminary registrations use46.25 and can be biased by body-centre placement, so they do not alone establish a precise as-built parameter.

The independent raw-plane check removes that registration: A13P uses entire connected roof component5046 rawZ ends[−.1495325416,−.0588667057], midpoint−.1041996237; its visible actual rim centre is rawZ−.1181954877. A-height-only scale46.81675036 gives.65524m offset from the physical roof midpoint. A13S component3007 ends[−.0908132270,−.0014405270], midpoint−.0461268770, visible rim centre−.0576227410 gives.53820m. Actual outer +X faces are sampled near rawX.1807/.2413 respectively, without a model XY centre or nominal4m ROI. Both source images were opened at full scale. The resulting spread≈.12m and UV damage preclude surveyed centimetre precision, but the observed centres are materially closer to.6m than.95m. This remains an explicit native-kit versus pre-demolition-source conflict for the coordinator to resolve; no production offset was changed by the audit.

The raw P image exposes only part of its long wall because the same-phase adjacent capsule covers the rest; its body midpoint comes from the full untruncated roof component, not that clipped visible wall. The S wall shows the full long face. Raw circle scores/radii are corroborative texture fits, not a claim that warped/damaged dark glazing has a perfect physical circle. Native and scan observations must both remain in provenance.

## Final Side-Window Recommendation

A repeat on the second tower confirms the discrepancy without assuming a tower centre or the former46.25 horizontal registration. B11 south phase0 is a newly identified P. Its roof merges into an L-shaped neighboring roof, so the complete component bbox is unsuitable. A transverse interior X strip through47 actual horizontal triangles preserves the entire longitudinal Z range: ends[−.1494297385,−.0592538714], span4.221741m using only the A53.50 height scale. Its actual side rim centre rawZ−.1153376690 gives offset.514791m from the end midpoint.

B10S roof is physically occluded by the next capsule: no suitable roof triangles were found, and no roof midpoint was invented. Independent opposing vertical end-plane fits instead give rawX ends[−.2153235039,−.1225603454], midpoint−.1689419246. Its visible actual side rim centre is rawX−.1814958640, giving offset.587735m using the same A-height scale. Both B raw UV images were opened and inspected. Data: generated/gluon-raw-B-side-centres.json; input scan-raw-B-side-centres.py. The B10 opposite end planes come from actual signed flat wall patches, not nominal4.2 endpoints.

| Actual capture | Body-centre anchor | Derived offset m |
|---|---|---:|
| A13P | Entire untruncated connected roof |.65524|
| A13S | Entire untruncated connected roof |.53820|
| B11P | Full longitudinal roof sample inside transverse strip |.51479|
| B10S | Opposing actual vertical end planes; roof unavailable |.58773|

Recommended authored reconstruction quantity: **one common0.60m side-window offset**, explicitly derived from the pre-demolition capture, with roughly±.10–.15m reconstruction uncertainty. These four independent-body-relative values span.515–.655m (median≈.563m). Separate P.65/S.55 constants are not supported: P alone varies.14m, comparable to the proposed family difference. Preserve native.950000008m as a precise conflicting kit-reference quantity, with its geometry-derived provenance. Do not call the new0.60m a surveyed exact value.

The native source remains useful for exact physical casing/profile and fitting relationships; it does not override positively observed as-built window placement. Conversely, scan UV damage/warping and finite circular-rim capture mean these fits should not be rounded to fictitious centimetre precision. Final source/tests/images must verify the coordinator's chosen physical casing and placement convention.

## A4/A5 Membership By Exhaustion

The full wall/rim audit now identifies seven distinct physical capsules on A4 and seven on A5 at the appropriate floor/phases, equal each documented count7. Consequently S0 cannot be present as an eighth capsule. Its omission is derived membership by exhaustion, matching the A13 proof, and no longer an inferred absence from an occluded sample. This does not certify hidden entry hand or centimetre-accurate registered XY.
