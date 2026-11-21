# Vehicles, Machinery and Building Components

The catalog contains 9 vehicles, 16 machines and 18 components. Each has body, thermal and structural geometry at four digital scales. The official document and volume checks use the live catalog, including its original print-availability decisions.

The new digital scales are 1:500 for the glass balustrade, tilt window, panel radiator, evaporative humidifier, finned heat exchanger, plate heat exchanger, expansion valve, scroll compressor, sorption wheel, scissor lift, city bicycle and electric kick scooter, plus 1:200 for the expansion valve. The authoring file is `equipment-digital.py`. The outer dimensions, mounting directions and mechanical pivots come from each asset’s existing measured frames.

The 1:500 body, thermal and structural views were rendered and inspected for all twelve assets. The plate exchanger keeps four open port collars, the valve maintains its sensing bulb and return tube, the scissor stack and the extendible deck retain independent moving frames, and the cycle wheels remain open. The check includes analytical surface-area and support measurements alongside those views.

The expansion valve’s body, head and capillary surface areas are 0.0055, 0.0034 and 0.0074 m². The installation retains the previous heat flux per unit area at the casting and diaphragm. The panel radiator includes both sides of each pressed plate. The room-facing projected areas are 0.60 m² at the rear and 0.20 m² per front band. Rated, comfort and night thermal solves deliver 1210 W, 929 W and 665 W. The outdoor heat pump models exchange through the exterior of its closed compressor compartment. The two cycle frame mass distributions lie on their tubes and retain the aggregate centre of mass.

The uninsulated 1960 wall has twelve courses of 240 × 115 × 71 mm brick. The 1:50 body and structural model contain 168 full and clipped units with 168 bed, collar and head-joint pieces. The section and plan show the alternating bond. A Three.js box intersection and volume oracle verified the 0.4 × 1 × 1 m envelope, no overlapping interiors and a total model volume of 0.3999999854 m³. The independent expected dimensions and checks are in `solid-wall-bond.json` and `solid-wall-bond.ts`.

| Asset | Body solids at 1:50 / 100 / 200 / 500 before repeat expansion | Catalog validation |
| --- | --- | --- |
| cargo-bike | 136 / 78 / 47 / 33 | Passed (Nov 20 03:56) |
| city-bicycle | 114 / 53 / 36 / 34 | Changed since pass (Nov 20 03:57) |
| city-bus | 198 / 123 / 79 / 40 | Passed (Nov 21 02:10) |
| city-car | 166 / 87 / 42 / 14 | Changed since pass (Nov 20 03:55) |
| delivery-van | 160 / 88 / 44 / 33 | Changed since pass (Nov 20 03:57) |
| electric-kick-scooter | 46 / 19 / 7 / 7 | Passed (Nov 21 01:45) |
| motor-scooter | 84 / 42 / 16 / 11 | Changed since pass (Nov 20 04:06) |
| tram | 238 / 143 / 104 / 55 | Passed (Nov 21 03:35) |
| vintage-car-1929 | 220 / 112 / 77 / 21 | Changed since pass (Nov 20 04:02) |
| concrete-mixer-truck | 259 / 157 / 113 / 60 | Changed since findings (Nov 21 03:11) |
| diesel-generator | 61 / 46 / 13 / 13 | Passed (Nov 20 03:47) |
| evaporative-humidifier | 32 / 19 / 9 / 6 | Passed (Nov 21 10:04) |
| excavator | 157 / 63 / 44 / 29 | Passed (Nov 21 02:24) |
| expansion-valve | 19 / 9 / 9 / 8 | Changed since pass (Nov 20 03:53) |
| finned-heat-exchanger | 16 / 12 / 8 / 5 | Passed (Nov 21 10:05) |
| forklift | 156 / 96 / 54 / 34 | Changed since findings (Nov 21 03:04) |
| heat-pump-outdoor-unit | 36 / 27 / 13 / 8 | Changed since pass (Nov 20 03:53) |
| oil-pumpjack | 73 / 62 / 41 / 31 | Changed since pass (Nov 20 03:49) |
| plate-heat-exchanger | 25 / 16 / 14 / 13 | Passed (Nov 21 09:57) |
| scissor-lift | 114 / 85 / 53 / 39 | Changed since pass (Nov 20 03:43) |
| scroll-compressor | 16 / 10 / 7 / 6 | Changed since pass (Nov 20 03:43) |
| solar-thermal-collector | 38 / 14 / 8 / 2 | Changed since pass (Nov 20 03:49) |
| sorption-wheel | 28 / 12 / 10 / 10 | Passed (Nov 20 03:44) |
| tower-crane | 436 / 407 / 361 / 63 | Changed since findings (Nov 21 05:52) |
| wheel-loader | 143 / 108 / 57 / 28 | Changed since findings (Nov 21 02:56) |
| cantilever-balcony | 21 / 12 / 6 / 3 | Changed since pass (Nov 20 04:02) |
| curtain-wall-bay | 44 / 7 / 4 / 4 | Changed since pass (Nov 20 04:02) |
| etics-brick-wall-12 | 61 / 3 / 2 / 1 | Passed (Nov 20 04:02) |
| etics-brick-wall-30 | 61 / 3 / 2 / 1 | Passed (Nov 20 04:02) |
| external-venetian-blind | 106 / 38 / 31 / 15 | Changed since pass (Nov 20 04:05) |
| glass-balustrade | 23 / 10 / 3 / 3 | Changed since pass (Nov 20 03:44) |
| glulam-beam | 42 / 17 / 3 / 1 | Changed since pass (Nov 20 04:03) |
| masonry-wall-with-lintel | 318 / 9 / 4 / 3 | Changed since findings (Nov 21 02:36) |
| panel-radiator | 26 / 11 / 5 / 7 | Changed since pass (Nov 20 03:49) |
| sectional-garage-door | 78 / 42 / 24 / 10 | Changed since findings (Nov 20 04:09) |
| sliding-door | 38 / 8 / 5 / 4 | Changed since pass (Nov 20 04:06) |
| solid-brick-wall-1960 | 57 / 2 / 1 / 1 | Changed since pass (Nov 20 04:06) |
| spiral-stair | 58 / 23 / 23 / 22 | Changed since findings (Nov 20 04:10) |
| stair-flight | 17 / 8 / 6 / 2 | Passed (Nov 20 04:10) |
| steel-column | 41 / 9 / 3 / 3 | Passed (Nov 21 09:50) |
| tied-canopy | 44 / 21 / 7 / 4 | Changed since pass (Nov 20 04:10) |
| tilt-window | 28 / 12 / 5 / 4 | Passed (Nov 21 09:50) |
| wheelchair-ramp | 34 / 12 / 9 / 7 | Passed (Nov 21 09:53) |

## Additional asset corrections

The rebuilt motor scooter has a hinged seat, a single-sided front suspension, a swinging engine unit, a protected silencer and a raked steering column. The first published model had 79, 40, 16 and 10 body parts. The current revision has 84, 42, 16 and 11. Its three orthographic drawings, symbol, poses, physical regions and structural members use the same geometry. All four print-suitability checks passed, and the final GLB reopened with twelve scenes.

The wheel loader's radiator and exhaust outlets now have distinct geometric and airflow-glyph identifiers. Its 4,756,520-byte GLB reopened with twelve scenes. The tram door calculation uses the opaque faces, edges and glazing reveals of its four leaves. The blind retains all 17 thin heat-exchanging slats in its coarsest thermal view, while the facade render shows its exterior face. The sectional door retains its rear crossbar in plan and front view.

## November 21 verification

The four 1:500 digital models for the tilt window, evaporative humidifier and both heat exchangers were restored after an overnight source rebuild. Each recovered level, its authoring function and its ticket source compare equal to the catalog JSON. The active builders now call those same handcrafted definitions. The 1:200 drawings keep the humidifier water level, finned-coil header and plate exchanger’s damper, rear base rail and bypass opening legible with individual pen weights. All three drawings passed both range endpoints, including 1:350.

The masonry wall with lintel has 248 mm blocks on a 250 mm module. The sliding door’s two track brushes leave a 1 mm working slit. The sectional door’s spring turns use 7.8 mm axial wire sections on an 8 mm pitch. All three GLBs round-tripped twelve scenes with zero geometric topology findings. The GLB sizes were 2,795,480, 1,488,036 and 12,559,244 bytes respectively.

The spiral stair’s 43 balusters, two end posts, post flanges and helical rail yield 2.592 m² of exposed steel from their lengths and diameters. The prior declaration of 1.1917 m² omitted part of the rail assembly. A new complete run is required for that change. The steel column, tilt window, wheelchair ramp, evaporative humidifier and both heat exchangers passed current full catalog validation.

The table uses the latest completed full product check for each asset and marks a file edited since that check. It also accepts fleet merged-source checks where all four ticket volumes exactly match the catalog. Older runs used earlier validator versions. All 172 equipment volumes now match their author sources after resolving the city bicycle brake cables and the exposed steel/glass balustrade handrail. The catalog now includes the delivery van’s lofted mirror housings, proud bumper skin and three pressed arch lips. The sliding-door track occupies the fourth arch. The vintage saloon’s volumes now match the revised wheelbase, suspension and hand-drawn period equipment, including its steering dampers, bonnet mascot and vacuum wiper. For these, only a catalog-only result counts as product evidence.

The tower crane’s 1:50 elevation draws the true cross-sections of the mast, boom and counter-jib chords and places two 72 mm diameter axial spigots inside the heel chords. Its elevation measured 1.875 mm p95 and 0.99825 IoU against the candidate body. The two joint pins at each of the three finer scales lie entirely inside their 120 mm chords. The 1:20 lifting-load print check passed. The live model and author source match. A complete product run is still pending.

The later per-document schema checks, scoped alternative resolution, wheel-loader cab, mixer barrel and analytical shaft evidence are recorded in [Equipment revisions and scoped catalog validation](equipment-scoped.md). The dedicated physiological outlet rule and oracle are recorded in [Breath-aperture validation](breath-apertures.md).
