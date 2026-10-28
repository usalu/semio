# Actual Source Video Visual Audit

Latest finding status: the original lamp and sash/occupant findings are corrected and verified in refreshed published source media. A later source 86 readout/material collision is pending correction. This remains a partial curriculum review; later chapter and complete-lesson samples are pending.

This is an incremental review of decoded actual source movies from the full curriculum render, not synthetic timeline renders. No complete curriculum visual pass is claimed. Some initial source provenance was subsequently invalidated by caption/helper corrections; refreshed final media must be rechecked where the content changes.

## Method

The retained source-review.py extracts six deterministic frames per completed source (12%, 30%, 48%, 66%, 84%, 97%). Every contact sheet and individual frame includes a source-content hash in its filename to avoid stale same-path image-tool displays. All selected frames listed below were independently viewed. Contact sheets are temporary generated evidence under generated/source-review; times and findings remain in this report.

## Actionable Findings

### P2: Physics Keeps An Independent Disconnected Pendant Lamp

Source ordinal 4, Beat3_Energieerhaltung, hash e98ec6ac8a9e: at 49.00, 67.33 and 85.83 seconds the room lamp still has an open line-built shade and detached outline bulb. Physics fundamentals helper at lines 662–675 constructs three shade lines and an unfilled bulb whose top is 0.03 drawing units below the shade bottom. This bypasses the corrected connected, filled shared lamp_glyph. Calls at lines 1520 and 1618 retain the old helper. Use the shared lamp helper and verify the refreshed source.

### P2: Projected Sashes Obscure Outer Occupants

Source ordinal 11, Beat1_Lueftungsbedarf, hash 6b150c26d2e0: at 8.17, 20.17, 32.33 seconds and later selected frames, both inward projected sashes overlap the outer occupants' heads/upper bodies. In the 640-pixel image the left pane spans approximately x84–119 while the left head is centered at x116; the right pane spans x278–313 while the right head is centered at x279. The transparent cyan panels visibly tint the heads. Adjust occupant positions to clear the opening swing or dimension the pane depth for this overview. The floor anchoring and human-to-room scale otherwise read plausibly.

## Reviewed Frames

| Ordinal | Source | Media Hash | Selected Seconds | Review |
|---|---|---|---|---|
| 2 | Beat1_EnergieImAlltag | 8b5a1719a5c5 | 21.17, 53.00, 84.67, 116.50, 148.16, 171.16 | Shared house remains connected; later roof/wall/window section and annotation leaders legible. No new architectural issue. |
| 3 | Beat2_Leistung | c9363bf4ea35 | 19.50, 48.50, 77.67, 106.66, 135.66, 156.66 | Graphs, captions and numeric labels remain within frame; no relevant structure obstruction. |
| 4 | Beat3_Energieerhaltung | e98ec6ac8a9e | 12.33, 30.67, 49.00, 67.33, 85.83, 99.00 | Room material coherent, standing figure anchored and scaled; old pendant defect above. |
| 6 | Beat5_Strahlung | 35a87f53a7f1 | 11.83, 29.33, 46.83, 64.50, 82.00, 94.67 | Section apertures, roof/floor material, shading and solar paths legible; no new architectural issue. |
| 7 | Beat6_ThermischeMasse | fcfeb8848f96 | 11.33, 28.17, 45.17, 62.00, 78.83, 91.17 | Canonical mass section, connected room geometry and thermal labels fit; some samples intentionally occur during tile/temperature animation. Opening/closing path already source-audited. No new issue. |
| 8 | Beat7_SensibelLatent | 62b254fdd659 | 19.17, 47.83, 76.50, 105.17, 134.00, 154.67 | Heating/condensation diagrams and section closeup legible, no new architectural clipping. Selected samples do not include the person-emission subsection. |
| 9 | Beat8_Zusammenfassung | afa7af1500db | 9.83, 24.50, 39.00, 53.67, 68.33, 78.83 | Summary and source reference panels fit; no architectural structure involved. |
| 11 | Beat1_Lueftungsbedarf | 6b150c26d2e0 | 8.17, 20.17, 32.33, 44.33, 56.50, 65.17 | Room/person scale and floor anchors coherent; projected sash overlap above. |
| 12 | Beat2_Lueftungsweg | 2263f1663cea | 6.50, 16.33, 26.00, 35.67, 45.50, 52.50 | Closed house sections, opposite seasonal streams and dimension/thermometer labels fit. No new architectural issue. |
| 13 | Beat3_Fahrplan | d917f532afa4 | 4.33, 10.67, 17.00, 23.33, 29.67, 34.17 | Five roadmap panels and captions legible; stylized icon-only cards are intentionally diagrammatic. |

## Remaining Review

Await completed critical ventilation natural/mechanical sections, heating final calculation/AnlagenVerluste, cooling room scenes, and final full-lesson contact sheets from the verification owner. Recheck refreshed hashes after any new source correction. Successful decoding alone does not establish visual acceptance.

## Additional Ventilation Volume Review

All six selected frames for each source below were viewed. No new actionable architectural defect was identified. Source 14 preserves filled structural bands around the net air volume and clear measurement arrows. Source 15 preserves the miniature house shell while volume/air exchange animates, then intentionally morphs usage glyphs into log-scale points. Source 16 keeps the four occupants grounded in the schematic room and separate from the flow/reserve bars. A sampled usage-label morph in source 15 is transient and does not establish a persistent clipping defect.

| Ordinal | Source | Media Hash | Selected Seconds |
|---|---|---|---|
| 14 | Beat1_Innenvolumen | 8b03452d71e3 | 9.00, 22.17, 35.50, 48.83, 62.17, 71.67 |
| 15 | Beat2_Luftwechselrate | 21df0d781afa | 10.00, 25.00, 40.00, 54.83, 69.83, 80.67 |
| 16 | Beat3_Volumenstrom | f12fe1756902 | 8.67, 21.33, 34.17, 46.83, 59.67, 68.83 |

Review total so far: 13 actual sources, 78 decoded selected frames. This remains an incremental sample review, not a full curriculum pass.

## Targeted Canonical And Face-Emission Frames

Source 7 thermal-mass additional frames at 31.17, 53.17, 73.17, 81.17, 83.17 and 90.17 seconds were viewed from media fcfeb8848f96 (sample hash 22acac). They cover completed 20-tile mass calculation, phase lag, radiation exchange, summer night purge with bottom-hinged sash, and winter release. Section material remains connected, the open pane has a coherent lower hinge, and explanatory leaders fit around the section. No new defect identified.

Source 8 additional frames at 113.17, 115.17, 117.17, 119.17, 121.17 and 123.17 seconds were viewed from media 62b254fdd659 (sample hash 967789), covering the previously unsampled person subsection. Breath/heat origins now appear at the head/upper body, the connected silhouette reads clearly, and labels/bar chart do not obscure it. No new defect identified.

Source 17 Beat1_Luftwaerme, media a0ebc7a83c65 (sample hash d0b2e3), was viewed at 7.17, 17.83, 28.50, 39.17, 50.00 and 57.67 seconds. The isolated heater/cubic-metre illustration is intentionally diagrammatic; no clipping or structure obstruction identified.

Review tally: 14 distinct actual sources, 96 selected decoded frames. Final curriculum acceptance remains pending later source/chapter review and refreshed media where corrections occur.

## Winter/Summer Heat-Flow Comparison

Source 20 Beat4_WinterUndSommer, media 7f8447bdafbe (sample 033918), was independently viewed at 6.00, 14.83, 23.67, 32.50, 41.33 and 47.83 seconds. Both seasonal miniature house sections stay connected and visible; flow direction annotations, load bars and captions are separate from structural geometry. No new actionable defect was identified. Tally: 15 distinct sources, 102 selected decoded frames.

The coordinator reports both actual-footage findings are now source-corrected: Physics lamp and adjacent everyday helpers delegate to tested shared primitives; ventilation need positions outer occupants from opened-sash bounds with physical clearance. These are not marked footage-resolved until the normal render resume refreshes source 4 and source 11 and their new hashed frames are reviewed.

## Natural Ventilation Window Regulator

Source 21 Beat1_Fensterregeln, media a94fa5cf1530 (sample 222e5a), was viewed at 9.33, 23.17, 36.83, 50.67, 64.50 and 74.50 seconds. The room section remains closed around its aperture, occupant feet meet the floor, and the projected sash stays clear of the occupant. Partial/wide opening states and seasonal controls remain legible. No new actionable issue identified. Tally: 16 distinct sources, 108 selected decoded frames.

## Remaining Natural-Ventilation Samples

All six selected frames in each source below were viewed. Source 23 keeps both apertures/sashes aligned with their wall gaps and the through-flow unobstructed; source 24 shows the occupant anchored to the slab and its shaft height/pressure annotations clear of the section; source 25 keeps the small thermal room section distinct from its graph/legend; source 26 keeps its occupant clear of the sash and filter/heat-loss annotations separate from the structural bands. The ungenutzt label in source 26 sits close to the left edge but a full-resolution individual frame confirmed its characters remain visible, so no clipping defect is asserted. No new actionable architectural issue was identified.

| Ordinal | Source | Media Hash | Sample Hash | Selected Seconds |
|---|---|---|---|---|
| 23 | Beat3_Querlueftung | 3668f1a5d227 | cbfd4b | 9.00, 22.33, 35.83, 49.17, 62.50, 72.17 |
| 24 | Beat4_Auftrieb | 379a688e5811 | 12180e | 11.33, 28.00, 44.83, 61.50, 78.33, 90.50 |
| 25 | Beat5_Nachtlueftung | 2fdc549fdaa1 | 232578 | 9.67, 24.17, 38.67, 53.17, 67.67, 78.17 |
| 26 | Beat6_GrenzenDerFreienLueftung | 864ebc8c5f0e | 93a6f9 | 9.67, 24.17, 38.50, 53.00, 67.50, 77.83 |

Tally: 20 distinct sources, 132 selected decoded frames. Updated footage for the two corrected findings and later mechanical/heating/cooling chapters remains pending.

## Mechanical Ventilation Basic Types

Source 27 Beat1_Grundtypen, media 58ecfbe7316d (sample 563eb8), was independently viewed at 7.00, 17.50, 28.00, 38.33, 48.83 and 56.33 seconds. All three filled room sections retain their intended apertures, fan/path attachment points remain aligned, and pressure annotations sit outside the structure. Comparative highlights do not obscure the room envelopes. No new actionable defect identified. Tally: 21 distinct sources, 138 selected decoded frames.

## Mechanical System Comparison

Source 29 Beat3_Anlagenvergleich, media 44deca2f9703 (sample 81bdde), was viewed at 7.33, 18.17, 29.00, 39.83, 50.67 and 58.50 seconds. All three miniature room structures remain legible within their cards, standing people retain coherent floor/ceiling proportions, sash/fan positions do not overlap occupants, and airflow/efficiency labels remain outside the structural bands. No new actionable issue identified. Tally: 22 distinct sources, 144 selected decoded frames.

## Mechanical Duct And Ventilation Summary

Source 30 Beat4_Kanalquerschnitt, media 64363ac45edf (sample e729b7), was viewed at 9.83, 24.50, 39.00, 53.67, 68.33 and 78.83 seconds. Duct outline/circular cross-section stay connected and dimension/formula callouts remain separate from the material silhouette. Source 31 Beat1_Zusammenfassung, media e3c832e36b41 (sample de654c), was viewed at 12.50, 31.17, 49.83, 68.50, 87.16 and 100.66 seconds. Its seven quantity cards and seasonal calculation columns stay legible within the frame; no architectural structure is involved. No new actionable defect identified in either source. Tally: 24 distinct sources, 156 selected decoded frames.

## Refreshed Physics Lamp And Heat Pump

Source 4 was refreshed by the normal renderer and independently re-viewed at 12.33, 30.67, 49.00, 67.33, 85.83 and 99.00 seconds from new media 164cca405f7e (sample 8627cf). The shared filled shade and bulb now meet visibly in all selected lamp states. The old pendant finding is footage-resolved. Room sections and occupant remain coherent.

New source 5 Beat4_Waermepumpe, media a0ef7ec987b9 (sample 7e0fc1), was viewed at 16.83, 42.00, 67.17, 92.33, 117.33 and 135.50 seconds. Heat-pump equipment is intentionally a system closeup; the later building section retains joined envelope material and sensible small emitter scale. Diagram labels and comparison bars fit their regions. No new actionable defect identified.

Tally: 25 distinct sources, 168 selected decoded frames, including six refreshed frames that resolve the lamp defect. Source 11 sash/occupant correction still awaits refreshed-footage review.

## Published Window Comparison, Heat Recovery And Heating Fundamentals

From this batch onward, the extractor requires each source .render.json before decoding. All selected frames below were viewed. Source 22 shows two dimensioned room/window/radiator sections with both wide casement and tilt states coherently attached to the right-hand apertures; energy bars and timing readouts remain separate. Source 28 heat-recovery airflow, exchanger and formula/load panels remain legible without geometric disconnection. Source 34 heating fundamentals shows a connected occupant, radiator and dimensionally credible exterior wall; floor/ceiling bands are visibly bounded and load-flow annotations stay separate. No new actionable issue identified.

| Ordinal | Source | Media Hash | Sample Hash | Selected Seconds |
|---|---|---|---|---|
| 22 | Beat2_StossOderKipp | 571ced93dd05 | 3e823d | 14.00, 34.67, 55.50, 76.17, 97.00, 112.00 |
| 28 | Beat2_Waermerueckgewinnung | dd650b119926 | 59ca3d | 11.83, 29.33, 47.00, 64.50, 82.17, 94.83 |
| 34 | Beat1_DreiWegeDerWaerme | 463d9b6b0467 | 912437 | 8.83, 22.00, 35.00, 48.17, 61.33, 70.67 |

Tally: 28 distinct sources, 186 selected decoded frames. The report remains an incremental visual audit; corrected source 11 and later heating/cooling chapters still await review.

## Published Ventilation Loads And Heating Transfer Mechanisms

All six selected frames in every source below were independently viewed. Sources 18 and 19 retain coherent shared house sections and unobstructed flow/temperature/humidity panels. Sources 35–37 preserve the grounded person and radiator in a credible thin exterior-wall section; the intentional wall magnifier and radiation rays explain rather than accidentally obscure the structure. Sources 39, 40 and 42 show legible material-layer, thickness/resistance and wall-area diagrams with separate dimension arrows and formula regions. No new actionable defect identified.

| Ordinal | Source | Media/Sample Hash | Selected Seconds |
|---|---|---|---|
| 18 | Beat2_SensibleWaerme | 87a8181a519d-a35595 | 9.83, 24.33, 39.00, 53.50, 68.17, 78.67 |
| 19 | Beat3_LatenteLast | 9ed3a16f1403-72395e | 9.33, 23.17, 37.17, 51.00, 64.83, 75.00 |
| 35 | Beat2_Waermeleitung | 3fc901cf6c39-4939b9 | 6.33, 15.83, 25.33, 34.67, 44.17, 51.00 |
| 36 | Beat3_Konvektion | 5de9220fc084-e9c800 | 6.50, 16.00, 25.50, 35.00, 44.67, 51.50 |
| 37 | Beat4_Strahlung | 5e3997026d3a-5e0511 | 5.67, 14.00, 22.33, 30.83, 39.17, 45.17 |
| 39 | Beat6_VonWegenZuZahlen | 07b9b406292c-5e708a | 5.67, 14.17, 22.50, 30.83, 39.33, 45.33 |
| 40 | Beat7_Waermedurchlasswiderstand | 0e875ebe8dc1-a4f2f0 | 8.33, 20.83, 33.33, 45.83, 58.33, 67.33 |
| 42 | Beat9_WaermestromFormel | 86fbc8eec515-e94292 | 8.67, 21.50, 34.33, 47.33, 60.17, 69.50 |

Tally: 36 distinct actual sources, 234 selected decoded source frames.

## Complete Physics Lesson Contact Review

Independently viewed all eight frames in full-review/physical_fundamentals-5564a953ba58.png: approximately 3.0, 82.0, 205.0, 358.8, 512.5, 666.2, 820.0 and 973.7 seconds. The eight sequence samples retain expected chapter order and coherent scene geometry; the energy-conservation room now has its connected filled shared pendant. No new clipping or structural defect identified in the complete-lesson samples. The verification owner separately decoded all 6,149 frames of the 1024.821-second 640 by 360 movie at approximately 6 fps; expected silent authored corpus has no audio stream. Decoding attribution belongs to the verification owner; visual inspection of all eight selected frames was performed by this auditor.

The other four complete lessons and refreshed source 11 remain pending. No five-lesson visual pass is claimed.

## Published Handoff, Wall Conductance, Envelope And Internal Gains

All six selected frames from each source below were viewed. Source 32 shared house sections and seasonal leaders remain coherent. Sources 41, 43 and 45 present bounded material/lattice and wall-area diagrams with legible dimensions. Source 47 correctly colors the complete roof/wall/floor section groups and door/window elements while load leaders stay outside the material. Source 50 has floor-anchored human, equipment and shared connected lamp at plausible storey scale. No new actionable architectural defect identified.

| Ordinal | Source | Media/Sample Hash | Selected Seconds |
|---|---|---|---|
| 32 | Beat2_UebergabeHeizungKuehlung | 86041f03a7b5-bc74b6 | 8.67, 21.67, 34.50, 47.33, 60.33, 69.67 |
| 41 | Beat8_UWert | f59ed543be67-23eedc | 8.00, 19.83, 31.83, 43.67, 55.50, 64.17 |
| 43 | Beat1_MakroUndMikro | 3e2ce2840e24-51fd06 | 6.50, 16.00, 25.50, 35.00, 44.67, 51.50 |
| 45 | Beat3_UWertUndGradient | 56f8df3b5ad4-730995 | 8.67, 21.50, 34.50, 47.33, 60.33, 69.67 |
| 47 | Beat5_Gebaeudehuelle | e885648f770f-047e7d | 7.17, 17.83, 28.50, 39.17, 49.83, 57.50 |
| 50 | Beat1_WinterInterneGewinne | d64c823591c5-8662e6 | 3.50, 8.67, 13.83, 18.83, 24.00, 27.67 |

Tally: 42 distinct source clips, 270 selected decoded source frames, plus eight independently viewed complete-Physics frames. Source 11 repaired-footage publication and later heating final calculation/cooling still await visual review.

## Repaired Occupant Clearance And Heating Internal/Solar Gains

All six selected frames in every source below were independently viewed. Refreshed source 11 visibly clears all four heads/upper bodies from the projected sashes, keeps their feet on the slab, and places moisture clusters around the moved people. The original sash-overlap finding is footage-resolved. Sources 51–53 preserve connected seated bodies, desk/chair anchors, and filled shared pendant. Sources 55–59 show coherent house/room sections, recessed glazing, shading and solar paths with explanatory labels separated from their structural geometry. No new actionable defect identified.

| Ordinal | Source | Media/Sample Hash | Selected Seconds |
|---|---|---|---|
| 11 | Beat1_Lueftungsbedarf | d05e54583432-49b52f | 8.17, 20.17, 32.33, 44.33, 56.50, 65.17 |
| 51 | Beat2_PersonenPhiP | 974bddb1906e-083e3f | 2.83, 7.17, 11.33, 15.67, 19.83, 22.83 |
| 52 | Beat3_GeraetePhiE | 2aa30511c65e-0e2747 | 2.50, 6.00, 9.67, 13.33, 16.83, 19.50 |
| 53 | Beat4_BeleuchtungPhiL | 3e5fd1bc4c49-0d8a31 | 2.83, 6.83, 10.83, 14.83, 18.83, 21.67 |
| 55 | Beat1_VerlustZuGewinn | 70ac20f3591c-b7445c | 2.00, 5.00, 8.00, 10.83, 13.83, 16.00 |
| 56 | Beat2_BestrahlungUndFlaeche | 3472b8597e37-c1d80d | 3.67, 9.17, 14.50, 19.83, 25.33, 29.17 |
| 57 | Beat3_GWert | d70d28abe35a-86a32d | 5.00, 12.50, 20.00, 27.33, 34.83, 40.17 |
| 58 | Beat4_SaisonaleWinkel | 2f3386b8f5a1-0592c1 | 4.33, 10.50, 16.83, 23.00, 29.33, 33.83 |
| 59 | Beat5_Verschattung | 532fce1be5e3-03d014 | 6.00, 15.00, 24.00, 33.17, 42.17, 48.67 |

Tally: 50 distinct source clips, 324 selected decoded source frames, plus eight complete-Physics samples. Both actual-footage findings are corrected and independently verified in refreshed published media. Final heating calculation/AnlagenVerluste, cooling and remaining four complete lessons remain pending.

## Heating Final Calculation And Cooling Architecture

All six selected frames from each source below were independently viewed. Sources 63–67 preserve miniature house/room sections, grounded occupants/emitters/shared lamps, and legible final energy formulas. Sources 70–75 and 77 preserve credible overview human/storey scale and seated desk/chair anchors; the office room, cooling plant and lighting remain coherent. Source 78 opaque-envelope coloring follows complete material groups with joined roof/wall geometry. Source 82 solar section and subsequent orientation diagram fit the frame. Source 86 has the readout collision below; its room/window/ray geometry otherwise remains coherent.

| Ordinal | Source | Media/Sample Hash | Selected Seconds |
|---|---|---|---|
| 63 | ReviewingHeatLosses | 48a64eda6ff2-f34a22 | 5.33, 13.00, 20.83, 28.50, 36.33, 42.00 |
| 64 | Scene2 | 1f70178f9dc5-8f7b77 | 6.33, 15.83, 25.33, 34.83, 44.33, 51.17 |
| 65 | ReviewingHeatGains | de7760f64400-f79329 | 6.00, 14.83, 23.67, 32.50, 41.17, 47.67 |
| 66 | Scene4 | 73d9bda28054-951cf5 | 9.00, 22.33, 35.67, 49.00, 62.17, 71.83 |
| 67 | UltimateEnergyBalance | 3a3798cf2020-d2605f | 3.83, 9.33, 14.83, 20.50, 26.00, 30.00 |
| 70 | Beat1_WinterGains | 359a64e43c36-afc47b | 4.67, 11.67, 18.67, 25.67, 32.67, 37.83 |
| 71 | Beat2_SummerOverheat | 2da0286f739f-c93b0f | 4.33, 10.67, 17.17, 23.50, 29.83, 34.50 |
| 72 | Beat3_CoolingSystem | 9be2ec01a3e0-ce8e00 | 6.50, 16.00, 25.67, 35.17, 44.67, 51.67 |
| 73 | Beat1_OfficeRoom | 1dd1a89f7748-4d7ef2 | 4.33, 10.50, 16.83, 23.00, 29.33, 33.83 |
| 74 | Beat2_HumanFactor | 55ed9b37378b-cd59d6 | 11.00, 27.50, 44.00, 60.33, 76.83, 88.67 |
| 75 | Beat3_DevicesLighting | b564c50b988a-3a2e2c | 10.83, 26.83, 43.00, 59.00, 75.17, 86.67 |
| 77 | Beat5_Mitigation | bfeacded6c53-cccd7e | 3.83, 9.67, 15.33, 21.17, 26.83, 31.00 |
| 78 | Beat1_TransmissionOpaque | 7393a4ef47bf-789d84 | 6.83, 16.83, 27.00, 37.00, 47.17, 54.33 |
| 82 | Beat1_SolarIrradiance | fe86b686efba-f52ca7 | 4.17, 10.50, 16.67, 23.00, 29.17, 33.67 |
| 86 | Beat5_SolarCoolingLoad | 3a0750605184-8d42cf | 3.17, 7.67, 12.17, 16.67, 21.17, 24.50 |

### P2: Solar Cooling Readouts Collide With Section Material

Source 86 Beat5_SolarCoolingLoad, hash 3a0750605184: at 16.67, 21.17 and 24.50 seconds the final W in the load result overlaps the right white wall band. The area annotation A = 1.92 m² also sits across the ceiling band. Individual original-size frame 86-3a0750605184-8d42cf-4.png confirms the collision. In cooling solar-radiation Python lines 468, 488 and 520, fixed read_at = [3.9, 1.2] left-anchors the result beyond the inner right bound x_r = 5.5 and places area y = 1.8 inside the ceiling material y = 1.6 to 1.86. Anchor the readouts using actual text bounds and inner wall margin, and place the area wholly above the roof band or inside the air region. Reported to coordinator; corrected footage pending.

## Complete Ventilation Lesson Contact Review

Independently viewed all eight selected frames in full-review/ventilation-6e6e3314a79f.png: approximately 3.0, 132.7, 331.5, 579.8, 828.3, 1076.8, 1325.3 and 1573.8 seconds. Shared house/room material, comparison sections, recovery diagrams and final seasonal balance panels remain visible and readable. No additional actionable defect identified in these complete-lesson samples. The verification owner decoded all 9,940 frames over 1656.647 seconds at 640 by 360 and approximately 6 fps; full-frame decoding is attributed to that owner.

Tally: 65 distinct source clips, 414 selected decoded source frames, plus sixteen complete-lesson samples across Physics and Ventilation. Canonical AnlagenVerluste is prioritized as soon as its published sidecar appears; source 86 correction and later cooling/demand/remaining complete lessons remain pending.

## Published Cooling Strategy And Energy-Balance Architecture

All six selected frames from each source below were viewed. Source 87 room grilles and section material remain joined; source 88 equations/duct sizing diagrams fit their panels. Source 89 keeps the occupant grounded in the shared house as insulation/shading/control annotations are added. Source 90 is an intentionally schematic sorption equipment diagram; source 91 control flowchart retains visible branch labels. Source 93 energy-balance house, occupant, lamp and emitter fit their storeys; source 96 maintains the shared envelope for seasonal balance; source 97 standards-method panels have no material/label collision. No new actionable architectural issue identified.

| Ordinal | Source | Media/Sample Hash | Selected Seconds |
|---|---|---|---|
| 87 | Beat2_VolumeFlowEquation | c8583c1039e3-89e5a5 | 6.67, 16.50, 26.33, 36.17, 46.00, 53.17 |
| 88 | Beat3_IsolateAirflow | 33ccba0d4ac1-e39a1e | 15.17, 37.83, 60.33, 83.00, 105.66, 122.00 |
| 89 | Beat1_LastZuerstSenken | 2c12d1a3ca5a-7515ff | 5.50, 13.50, 21.67, 29.67, 37.67, 43.50 |
| 90 | Beat2_SorptionsKuehlung | 7853e1c4da3a-563eb8 | 7.00, 17.50, 28.00, 38.33, 48.83, 56.33 |
| 91 | Beat3_KomfortStrategie | ba5c6ef1ee33-ae1cef | 6.83, 16.83, 26.83, 36.83, 46.83, 54.00 |
| 93 | Beat1_Bilanzgrenze | c7801b15366c-f7af12 | 8.50, 21.00, 33.67, 46.17, 58.67, 67.83 |
| 96 | Beat4_WinterUndSommer | 6aaee8a876ff-9301cf | 9.17, 22.83, 36.50, 50.17, 63.83, 73.67 |
| 97 | Beat1_GEGundNormen | 25765ec5992a-8805f5 | 7.00, 17.17, 27.50, 37.83, 48.17, 55.50 |

The coordinator reports source 86 readout placement is now corrected and its production-Manim boundary regression passes; this audit awaits the refreshed normal-render hash before resolving the footage finding.

Tally: 73 distinct sources, 462 selected decoded source frames, plus sixteen complete-lesson samples. AnlagenVerluste remains first priority on publication.

## Additional Demand and Plant Source Frames

Independently reviewed all six sampled frames from each published clip below. These add 42 frames, bringing the selected source review to 80 distinct clips and 504 source frames, plus 16 complete-lesson samples. No additional actionable geometry or label defect was visible in these samples. Source 86 remains pending refreshed footage; source 68 remains pending publication.

| Source | Scene | Media hash | Frame times (seconds) |
|---|---|---|---|
| 101 | Beat1_LeistungUndEnergie | 8cf8a7d52c17-602729 | 9.33, 23.17, 37.00, 50.83, 64.83, 74.83 |
| 102 | Beat2_Normheizlast | 73a7104d8b1c-d1fbf8 | 10.50, 26.00, 41.67, 57.17, 72.66, 84.00 |
| 104 | Beat4_RichtigAuslegen | f79a60606d68-01bd23 | 11.50, 28.50, 45.50, 62.50, 79.67, 91.83 |
| 106 | Beat2_UebergabeVerteilungSpeicherung | 1e4ea7f512ed-9a13ce | 7.33, 18.33, 29.33, 40.17, 51.17, 59.17 |
| 107 | Beat3_Erzeugung | 98ed263b4a30-039c3d | 8.67, 21.67, 34.67, 47.67, 60.67, 70.17 |
| 108 | Beat4_Anlagenaufwandszahl | d70411242c6a-9ac0f2 | 8.17, 20.33, 32.33, 44.50, 56.67, 65.33 |
| 109 | Beat1_DreiBilanzgrenzen | 667267fe8211-b7348a | 9.50, 23.67, 38.00, 52.17, 66.33, 76.50 |

The demand house retains floor-anchored occupants and correctly scaled emitters. The plant and storage samples retain contained fill and connected schematic paths. The final loss-factor and boundary diagrams do not obscure their architectural context.

## Published AnlagenVerluste Animation Acceptance

Source 68 media SHA prefix 7c42ea5e3536 was reviewed at 11.33, 20.00, 28.17, 36.00, 45.00, 53.00, 61.83, 70.00, 78.67, 86.00, 90.83 and 93.33 seconds, including its final decoded frame. Evidence sheets: generated/source-review/68-7c42ea5e3536-963f52-contact.png and 68-7c42ea5e3536-3e9e00-contact.png under the ticket trash output directory.

No additional actionable visual defect was visible. The occupant stands on the occupied-storey floor; radiators fit beneath the windows at a plausible scale. The storage fill stays inside its shell through the middle and late states. Hydraulic risers terminate at the radiators and join the cellar loop and equipment; the boiler flue remains continuous through floor and roof. Final explanatory columns and equation stay clear of the house. Temporary numerical values during counter animation are expected transition states.

Coverage is now 81 distinct source clips and 516 selected source frames, plus 16 complete-lesson samples. Corrected source 86 and the remaining three complete-lesson contacts remain pending; this is not a five-lesson final pass.

## Corrected Solar Cooling Readout Footage

Source 86 is now independently re-reviewed with current published media hash 970db95817a1 (sample set d3716b), replacing historical 3a0750605184. All six actual frames at 3.17, 7.67, 12.17, 16.67, 21.17 and 24.50 seconds were inspected. The area and shading-factor annotations now stay below the ceiling and inside the right wall; the final 96.8 W result also stays inside the room with clear right margin. The historical ceiling and wattage collisions are resolved in footage. Evidence: generated/source-review/86-970db95817a1-d3716b-contact.png under the ticket trash output directory. No additional actionable defect was visible.

Coverage remains 81 distinct sources, now 522 selected source frames and 16 complete-lesson samples. All concrete findings raised by this audit have final footage or scope-resolution evidence. Remaining final-movie samples are still pending; no five-lesson visual completion claim is made yet.

## Final Complete-Lesson Sample Acceptance

Independently inspected all eight sampled frames from each of the remaining three completed movies, using the unique media-hashed contact sheets below. No additional actionable architectural geometry, scale, clipping or annotation obstruction was visible. A few samples capture ordinary fade or build transitions; those do not establish missing geometry.

| Lesson | Media hash | Decoded frames | Duration (seconds) | Contact sheet |
|---|---|---|---|---|
| heating | 366b243ca0f1 | 10608 | 1767.987 | heating-366b243ca0f1.png |
| cooling | ae7ac0d86f65 | 6284 | 1047.323 | cooling-ae7ac0d86f65.png |
| energy_balance | d774dc9dc35b | 13206 | 2200.972 | energy_balance-d774dc9dc35b.png |

All five complete-lesson contact sheets have now been independently inspected: 40 final-movie samples in addition to 522 selected source frames from 81 distinct sources. The three concrete actual-footage findings (lamp continuity, sash/occupant overlap, solar annotation collision) have been rechecked against refreshed published media and are resolved. No open actionable finding remains from this review. This is sampled visual acceptance, not inspection of every frame; reuse-agent full-frame decoding provides the separate technical playback evidence. Exact full SHA256 values and original-size sampled frames are recorded in generated/full-review/metadata.json under the ticket trash output directory.

## Late Activity-Glyph Consistency Finding

The earlier no-open-finding statement is superseded by one late cross-review finding. Cooling final sheet cooling-ae7ac0d86f65.png, frame 03 at approximately 209.5 seconds, shows the seated person as a filled shared silhouette while walking/sprinting people retain hollow heads and thin line bodies. Independently confirmed after reuse-agent notification. Source: energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py lines 99–139, particularly hollow head constructor at 111–112 and walk/sprint branches at 123–139. Use the shared filled person construction for all activity poses so the comparison remains visible and consistent at final 640×360 playback. Parent disposition or refreshed footage is pending; architectural geometry findings remain resolved.

## Bounded Manifest Human-Construction Sweep

Read-only sweep of all 32 unique Python script paths referenced by the five-video manifest, including entrypoints and intro. Reviewed AST helper names containing person, human, activity, walk, run, sleep, seated, occupant or figure; additionally reviewed all Circle constructor lines, local head/torso/limb assignments, and every _stand/_furnish helper. No extra render was performed.

The only concrete residual hollow-human assembly is the already reported cooling internal-gains _activity_figure sleep/walk/sprint branches (lines 111–139). All other actual people route to shared person_glyph or seated_person_glyph, including ventilation occupants, demand standing/furnishing helpers and Physics _person alias at line 491. Candidate Circle/Line matches elsewhere were legitimate thermostat dials, shower controls/rose, fans, pumps, markers, weather/activity rings, lens diagrams or plotted particles. Helpers named running/run_hour update numbers rather than construct people. No further residual hollow-human instance was found within the manifest source inventory.

## Remaining Source Coverage: Audit Partition

The prior 81-source selection prioritized architectural consumers; the other 29 had not individually been sampled and were not all non-architectural. Extracted six frames from each already-published media file without timeline rendering. Independently inspected the 15 sources below (90 frames). No new concrete architectural, scale, clipping or persistent label-obstruction defect was found. The other 14 sources are assigned to the final-inventory reviewer; completion is not claimed until its evidence arrives.

| Source | Scene | Hash and sample set | Frame times (seconds) |
|---|---|---|---|
| 100 | Beat4_Gradtagszahl | 83cf06841dbf-46a0a3 | 8.67, 21.50, 34.33, 47.17, 60.00, 69.33 |
| 103 | Beat3_Jahresdauerlinie | f2a5153ecf7d-f12fd5 | 9.50, 23.67, 37.83, 52.00, 66.17, 76.33 |
| 110 | Beat2_Primaerenergiefaktor | 481ee5909886-579d53 | 9.00, 22.50, 36.00, 49.33, 62.83, 72.50 |
| 111 | Beat3_Rechenbeispiel | b86e9945b1d7-62f7d9 | 8.33, 20.83, 33.17, 45.67, 58.00, 67.00 |
| 112 | Beat4_Referenzgebaeude | 3570c02e5c0e-892c44 | 7.67, 19.17, 30.67, 42.17, 53.50, 61.83 |
| 113 | Beat1_BedarfVsVerbrauch | 827c76df0031-3c06bb | 8.33, 20.67, 33.00, 45.33, 57.83, 66.67 |
| 114 | Beat2_DieSkala | b657c643d939-08f67c | 8.17, 20.33, 32.50, 44.67, 57.00, 65.67 |
| 115 | Beat3_PflichtangabenUndFolgen | 10118d716177-40aea8 | 7.17, 17.83, 28.33, 39.00, 49.67, 57.33 |
| 116 | Beat1_BerechnungVsAbrechnung | c81efd327082-a954bc | 14.50, 36.33, 58.00, 79.83, 101.67, 117.33 |
| 117 | Beat2_HydraulischerAbgleich | 0ca281b35775-0636c3 | 11.17, 27.67, 44.33, 60.83, 77.50, 89.50 |
| 118 | Beat3_SerienAbschluss | 287976586925-6f9411 | 14.33, 35.83, 57.33, 78.83, 100.33, 115.83 |
| 94 | Beat2_Verlustseite | 3cfa2d5d4091-1570fb | 23.17, 57.67, 92.16, 126.83, 161.33, 186.33 |
| 95 | Beat3_Gewinnseite | 8e422157624b-daf777 | 15.33, 38.00, 60.83, 83.50, 106.33, 122.83 |
| 98 | Beat2_Monatsbilanz | e3c76266a10d-f78dfb | 12.33, 30.83, 49.17, 67.67, 86.00, 99.33 |
| 99 | Beat3_Klimadaten | 4d55e43ba8b7-610aeb | 8.33, 20.67, 33.17, 45.50, 57.83, 66.83 |

## Remaining Source Coverage: Final-Inventory Partition

Final-inventory reviewer independently inspected all 14 assigned sheets (84 frames) and reported no concrete visual defect. Its reviewed evidence confirms intact room/house material, floor-anchored occupants, glazing perspective, separated calculation labels and final intro titles. Ordinary write/fade transitions and animated counter values were distinguished from persistent defects.

| Source | Scene | Hash and sample set | Frame times (seconds) |
|---|---|---|---|
| 1 | Demo_Intro_PhysikalischeGrundlagen | fca5daab43ed-781c52 | 1.67, 4.17, 6.50, 9.00, 11.50, 13.17 |
| 10 | Demo_Intro_Lueftung | 6f78f87dcd08-fff6bd | 1.67, 4.17, 6.50, 9.00, 11.50, 13.17 |
| 33 | Demo_Intro_Heizlast | 744b1c33d58d-4a5220 | 1.67, 4.17, 6.50, 9.00, 11.50, 13.17 |
| 38 | Beat5_Zusammenfassung | 114f89e1febf-53cb1a | 8.00, 19.67, 31.50, 43.17, 55.00, 63.50 |
| 44 | Beat2_RWert | 9dd47ba34882-728c98 | 6.17, 15.17, 24.17, 33.33, 42.33, 48.83 |
| 46 | Beat4_WattVergleich | 16c6f6dd0b72-23c0da | 7.83, 19.33, 30.83, 42.33, 53.83, 62.17 |
| 48 | Beat6_Waermebruecken | 3284ea19868d-27e96a | 12.17, 30.33, 48.33, 66.50, 84.67, 97.67 |
| 49 | Beat1_LueftungImWinter | 44ae391dd859-027e95 | 8.33, 20.67, 33.00, 45.33, 57.83, 66.67 |
| 54 | Beat5_SummeUndDichte | b3f2ad2fdc42-ce3982 | 3.83, 9.33, 14.83, 20.33, 25.83, 29.83 |
| 62 | Beat8_Hauptgleichung | ca2e4e75ef3c-d5270b | 7.17, 17.67, 28.17, 38.83, 49.33, 57.00 |
| 69 | Demo_Intro_Kuehllast | deeb1752e6fe-4a5220 | 1.67, 4.17, 6.50, 9.00, 11.50, 13.17 |
| 76 | Beat4_CumulativeLoad | 783c48d8c9a8-a45202 | 6.33, 15.50, 24.83, 34.00, 43.33, 50.00 |
| 81 | Beat3_LueftungslastImSommer | 13e9bf3af80c-bf26c9 | 7.33, 18.17, 29.00, 39.83, 50.67, 58.50 |
| 92 | Demo_Intro_Energiebilanz | e34146297a6a-4a5220 | 1.67, 4.17, 6.50, 9.00, 11.50, 13.17 |

All 110 distinct published manifest sources now have selected visual-frame coverage: 696 source-frame reviews, including historical and refreshed comparisons, plus 40 completed-movie samples. All architecture and annotation findings are resolved; the late activity-pose style finding remains pending refreshed footage. The complete-source coverage is sampled, not every-frame visual inspection.

## Filled Activity Preview Acceptance

Independently inspected the new 1600×900 activity-preview/images/activity-preview/ActivityPreview_ManimCE_v0.22.0.png under generated output. Sleep rests on the bed as a continuous filled profile. Walking and sprinting have solid connected heads, torsos and limbs, with recognizable distinct poses; seated pose remains recognizable and filled. Wattage and activity labels remain clear. Preview accepted; canonical refreshed source/movie evidence is still pending before the late finding is closed.

## Canonical Filled-Activity Footage Acceptance

Refreshed published source 74 Beat2_HumanFactor, media hash 4c45a259a069, independently reviewed at 11, 27.5, 44, 60.33, 76.83 and 88.67 seconds (sample cd59d6), plus targeted activity comparison at 30, 32, 34, 36, 38 and 40 seconds (sample bedff5). Unique contact sheets 74-4c45a259a069-cd59d6-contact.png and 74-4c45a259a069-bedff5-contact.png are under generated/source-review.

Actual 640×360 footage confirms solid connected sleep/walk/sprint profiles matching the seated silhouette, distinct recognizable poses, contained captions and clear wattage labels. Comparison holds clearly from 34 through 40 seconds; initial partial figures and in-flight counters at 30–32 seconds are expected build animation. Late hollow-activity finding is resolved in canonical source footage. No actionable visual finding remains. Source review coverage is all 110 unique sources, now 708 selected source-frame reviews plus 40 complete-movie samples; updated assembled Cooling evidence remains the separate final verification step.

## Final Updated Cooling Assembly Acceptance

Independently inspected all eight frames in cooling-22b50ab73f2e.png. Full movie SHA256 is 22b50ab73f2e75bf4c5f58ef54c9f3df09182ef4c649e65d48a2d9a9d1ab3f95. Activity comparison at 209.5 seconds now visibly contains the filled sleep/walk/sprint profiles; the remaining sampled scenes remain coherent. Reuse verifier decoded all 6284 Cooling frames (1047.323 seconds), confirmed all 110 current source/provenance records, and confirmed the other four movie hashes unchanged.

Final sampled visual acceptance: all 110 unique sources covered, 708 selected source-frame reviews and 48 movie-sample reviews including the superseded Cooling comparison. No actionable finding remains. This report claims sampled visual review plus separately reported complete decode/provenance verification, not manual examination of every movie frame.
