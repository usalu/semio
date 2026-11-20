# Vehicles, Machinery and Building Components

The catalog contains 9 vehicles, 16 machines and 18 components. Each has body, thermal and structural geometry at four digital scales. The official document and volume checks use the live catalog, including its original print-availability decisions.

The new digital scales are 1:500 for the glass balustrade, tilt window, panel radiator, evaporative humidifier, finned heat exchanger, plate heat exchanger, expansion valve, scroll compressor, sorption wheel, scissor lift, city bicycle and electric kick scooter, plus 1:200 for the expansion valve. The authoring file is `equipment-digital.py`. The outer dimensions, mounting directions and mechanical pivots come from each asset’s existing measured frames.

The 1:500 body, thermal and structural views were rendered and inspected for all twelve assets. The plate exchanger keeps four open port collars, the valve maintains its sensing bulb and return tube, the scissor stack and the extendible deck retain independent moving frames, and the cycle wheels remain open. The check includes analytical surface-area and support measurements alongside those views.

The expansion valve’s body, head and capillary surface areas are 0.0055, 0.0034 and 0.0074 m². The installation retains the previous heat flux per unit area at the casting and diaphragm. The panel radiator includes both sides of each pressed plate. The room-facing projected areas are 0.60 m² at the rear and 0.20 m² per front band. Rated, comfort and night thermal solves deliver 1210 W, 929 W and 665 W. The outdoor heat pump models exchange through the exterior of its closed compressor compartment. The two cycle frame mass distributions lie on their tubes and retain the aggregate centre of mass.

The uninsulated 1960 wall has twelve courses of 240 × 115 × 71 mm brick. The 1:50 body and structural model contain 168 full and clipped units with 168 bed, collar and head-joint pieces. The section and plan show the alternating bond. A Three.js box intersection and volume oracle verified the 0.4 × 1 × 1 m envelope, no overlapping interiors and a total model volume of 0.3999999854 m³. The independent expected dimensions and checks are in `solid-wall-bond.json` and `solid-wall-bond.ts`.

| Asset | Body solids at 1:50 / 100 / 200 / 500 before repeat expansion | Catalog validation |
| --- | --- | --- |
| cargo-bike | 136 / 78 / 47 / 33 | Passed |
| city-bicycle | 111 / 53 / 36 / 34 | Passed |
| city-bus | 198 / 123 / 79 / 40 | Passed |
| city-car | 137 / 67 / 32 / 14 | Passed |
| delivery-van | 157 / 88 / 44 / 33 | Passed |
| electric-kick-scooter | 46 / 19 / 7 / 7 | Passed |
| motor-scooter | 79 / 40 / 16 / 10 | Passed |
| tram | 238 / 143 / 104 / 55 | Passed |
| vintage-car-1929 | 213 / 112 / 77 / 21 | Passed |
| concrete-mixer-truck | 295 / 156 / 112 / 58 | Passed |
| diesel-generator | 61 / 46 / 13 / 13 | Passed |
| evaporative-humidifier | 32 / 19 / 9 / 6 | Passed |
| excavator | 157 / 63 / 44 / 29 | Passed |
| expansion-valve | 19 / 9 / 9 / 8 | Passed |
| finned-heat-exchanger | 16 / 12 / 8 / 5 | Passed |
| forklift | 156 / 96 / 54 / 34 | Passed |
| heat-pump-outdoor-unit | 36 / 27 / 11 / 8 | Passed |
| oil-pumpjack | 73 / 62 / 41 / 31 | Passed |
| plate-heat-exchanger | 25 / 16 / 14 / 13 | Passed |
| scissor-lift | 114 / 85 / 53 / 39 | Passed |
| scroll-compressor | 16 / 10 / 7 / 6 | Passed |
| solar-thermal-collector | 38 / 14 / 8 / 2 | Passed |
| sorption-wheel | 28 / 12 / 10 / 10 | Passed |
| tower-crane | 436 / 407 / 361 / 63 | Pending |
| wheel-loader | 138 / 105 / 57 / 24 | Passed |
| cantilever-balcony | 21 / 12 / 6 / 3 | Passed |
| curtain-wall-bay | 44 / 7 / 4 / 3 | Passed |
| etics-brick-wall-12 | 61 / 3 / 2 / 1 | Passed |
| etics-brick-wall-30 | 61 / 3 / 2 / 1 | Passed |
| external-venetian-blind | 32 / 24 / 24 / 12 | Passed |
| glass-balustrade | 7 / 3 / 3 / 3 | Passed |
| glulam-beam | 42 / 17 / 3 / 1 | Passed |
| masonry-wall-with-lintel | 318 / 9 / 4 / 3 | Passed |
| panel-radiator | 26 / 10 / 6 / 3 | Passed |
| sectional-garage-door | 44 / 30 / 14 / 13 | Open findings |
| sliding-door | 38 / 8 / 5 / 4 | Passed |
| solid-brick-wall-1960 | 57 / 2 / 2 / 1 | Passed |
| spiral-stair | 58 / 23 / 23 / 22 | Pending |
| stair-flight | 17 / 8 / 6 / 2 | Pending |
| steel-column | 15 / 7 / 3 / 3 | Pending |
| tied-canopy | 44 / 21 / 7 / 4 | Pending |
| tilt-window | 28 / 12 / 4 / 3 | Passed |
| wheelchair-ramp | 25 / 14 / 12 / 4 | Pending |

## Open measured findings

- sectional-garage-door — VOLUMES volumes-cross-scale 1:500 volumes-cross-scale volumes.500.body: The body of 1:500 disagrees with 1:200 in the plan: IoU 0.892 (at least 0.261), boundary distance p95 675.0 mm (at most 543.8 mm).

## Additional asset corrections

The rebuilt motor scooter has a hinged seat, a single-sided front suspension, a swinging engine unit, a protected silencer and a raked steering column. The body has 79, 40, 16 and 10 parts at the four scales. Its three orthographic drawings, symbol, poses, physical regions and structural members use the same geometry. All four print-suitability checks passed, and the final GLB reopened with twelve scenes.

The wheel loader's radiator and exhaust outlets now have distinct geometric and airflow-glyph identifiers. Its 4,756,520-byte GLB reopened with twelve scenes. The tram door calculation uses the opaque faces, edges and glazing reveals of its four leaves. The blind retains all 17 thin heat-exchanging slats in its coarsest thermal view, while the facade render shows its exterior face. The sectional door retains the rear crossbar in plan and front view and assigns the sandwich skin's bonded face to its insulating core.
