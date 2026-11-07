Mount audit 2026-11-05

Static source inspection only; no render or dynamic contact test.

The staffage bridge places the authored origin at the supplied anchor; it does not snap named supports to walls.

Gas boiler: origin is the lowest connection stub and wall-bracket is local (0, 0.64 m). Heating calculation anchors its origin 0.7 m above cellar floor but at x=-2.85, where no wall is depicted; cellar section walls are at x=-6.65 and -1.35. The chimney above is not a wall support.

Cylinder: origin is on floor at tank axis, both feet at y=0, and calculation anchors it on cellar floor y=-1.6; support heights agree.

Radiator: root (0.035, 0.11) rotated 90 degrees maps bracket-top (0.47, 0.035) to x=0 and 0.58 m above floor. Calculation anchors section radiators at x=-6.41/-1.59, approximately 0.128 scene units from inner wall faces at -6.538/-1.462. Neither bracket touches those faces. Keep 0.11 m bottom clearance when correcting.

Thermostat: housing center is 1.5 m above floor origin in each pose; screws are housing-local y=±0.034 m. Calculation floor anchor agrees with poses after the current correction. Its x=-6.42 differs from inner wall face x=-6.538; a face-on device may be on an implied back wall but that wall is not explicitly drawn.

Primary energy: boiler base is passed as origin; first installation line 435 uses G+0.01, so stub bottoms are essentially on floor despite bracket 0.64 m above. Annex has outline at far side and top but no support plane at boiler center. Comparison line 823 also has no explicit support. Front radiator floor-origin anchor has +0.02 scene unit vertical offset.

Sources: energy heating calculation and primary-energy scene; staffage bridge; gas-boiler, hot-water-cylinder, room-thermostat and panel-radiator authored JSON catalog entries. Findings are source-level deductions and source may change concurrently.
