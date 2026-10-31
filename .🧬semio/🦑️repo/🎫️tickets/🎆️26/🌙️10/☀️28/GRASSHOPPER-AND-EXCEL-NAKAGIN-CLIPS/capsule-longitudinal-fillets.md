# Native Longitudinal Capsule Fillets

## Current implementation

The source-backed 3D casing is implemented with six flat panels, twelve rolled edges and eight spherical corners. Physical bounds and all 33 poses/ports are unchanged. Inner steel uses radius minus 1.2 mm. Project-owned analytic patches, exact sill clipping and ear-clipped material caps preserve real apertures without CAD/Boolean runtime dependencies. Spherical faces are planar triangles; explicit quarter LODs 1/2/4 produce genuinely different closed meshes.

All 16 casing tests pass in 8.554 s, including 3D SDF/native witnesses, internal component containment, usable fitting bounds and contacts, actual window reveal, conservative collision enclosure, Rhino watertight positive material patches, and 7.77 m² of actual floor-to-steel bearing. Actual steel/lining doorway rays at 0.2001/0.21/0.30/1.10/2.1499 m clear centre/both jambs for all six families; positive outside-jamb controls hit. Only explicitly explained external entrance/bolt hardware is exempt from body containment; circulation retains every physical barrier.

## Positive physical surface evidence

The native Slash TRIANGLES asset contains full three-dimensional rounding, not merely a constant rounded X/Z extrusion. Decode actual POSITION values in place, convert(x,y,z)CAD=(GLBx,−GLBz,GLBy), and evaluate the standard3D rounded-box signed distance with centre(0,0,1.05), inner-box half-extents(1.05,1.85,1.00), radius.25m. A total1283physical vertex entries lie on this outer surface within1e−6m;773of them are genuine longitudinal edge/corner witnesses with1.85<|Y|<2.10,3Dsurface distance approximatelyzero and strict negative distance inside the constant rounded-X/Z extrusion.

Example actual point(−1.04999995,−2.02677679,−.12677670) has3Ddistance+6.74e−8m, while its constant-X/Z profile distance is−.07322330m. Another(−1.04999995,−2.09582639,.00450924) has3Ddistance+4.45e−8m and extruded-X/Z distance−.20450924m. These are actual physical triangles, excluding all line/control geometry; they cannot be explained by inflated CAD bounding boxes.

The flat end panel atY±2.10 lies withinX±1.05 and Z.05..2.05 in native coordinates. The remaining end-edge material rolls into the side/roof/floor across radius.25m. Under the current nominal reference frame this means physicalboundsX±1.30,Y−.10..4.10,Z0..2.50, core-boxX±1.05,Y.15..3.85,Z.25..2.25, with longitudinal and spherical corner transitions.

## Reconstruction implication

The previous constant X/Z-rounded extrusion overfilled native front/back edge rollovers, despite correct overall bounds; this defect is now corrected. A conservative collision envelope may intentionally enclose these rollovers, but the visible casing requires true12edge/8corner rounding to match this recoverable native source. Entry/window openings remain inside the flat panel regions except the lower5cm of the doorway, where a true cutout must continue through the adjacent lower roll. Existing source aperture planes/recess and all33world poses remain valid.

The positive finding was sent to parent and the independent auditor before the authorized topology correction. Final combined model/circulation/visual gates follow the independently owned fixture mounting fixes. Generated numerical witness data are in native-longitudinal-fillets.json.
