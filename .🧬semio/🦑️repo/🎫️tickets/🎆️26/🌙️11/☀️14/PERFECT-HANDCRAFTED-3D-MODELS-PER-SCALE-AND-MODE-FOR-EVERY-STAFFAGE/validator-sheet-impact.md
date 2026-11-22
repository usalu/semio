# Impact of the Declared Thin Sheet on the Catalog

`volumes-feature` lets a thin sheet pass below the smallest feature only when it is declared: `sheet: true` on the solid, or a thermal layer gauge (`thicknessM`) for its bone and material. The solids below are sheet-shaped (middle extent at least the limit and at least 20 times the thickness) but undeclared: they passed before and fail now. A true sheet (vane, leaf blade, fabric, membrane, foil, sheet metal, thin board or panel) gets `sheet: true`; anything else is rebuilt for its scale.

Totals: 152 assets read, 21 assets affected, 111 solids (counted per scale and list), 0 assets that could not be read or judged.

## Solids per Asset, Scale and List

- 🏗️machinery/diesel-generator: 1:200 body hood-side-l, hood-side-r, hood-top, hood-back, hood-frame
- 🏗️machinery/diesel-generator: 1:500 body hood-side-l, hood-side-r, hood-top, hood-back, hood-frame
- 🏗️machinery/heat-pump-outdoor-unit: 1:100 body fan-blades
- 🏗️machinery/heat-pump-outdoor-unit: 1:200 body fan-blades
- 🏗️machinery/heat-pump-outdoor-unit: 1:500 body fan-blades
- 🏗️machinery/scroll-compressor: 1:100 body base-plate, oldham-ring
- 🏗️machinery/scroll-compressor: 1:100 thermal th-oldham-ring
- 🏗️machinery/scroll-compressor: 1:200 body base-plate
- 🏗️machinery/solar-thermal-collector: 1:100 body absorber
- 🏗️machinery/solar-thermal-collector: 1:200 body absorber
- 🏗️machinery/solar-thermal-collector: 1:500 body absorber
- 🐕️animals/herring-gull: 1:100 body near-primary-1, near-primary-2, near-primary-3, near-primary-4, near-primary-5, near-primary-6, near-primary-7, far-primary-1, far-primary-2, far-primary-3, far-primary-4, far-primary-5, far-primary-6, far-primary-7
- 🐕️animals/herring-gull: 1:200 body near-primary-1, near-primary-2, near-primary-3, near-primary-4, near-primary-5, near-primary-6, near-primary-7, far-primary-1, far-primary-2, far-primary-3, far-primary-4, far-primary-5, far-primary-6, far-primary-7
- 🐕️animals/labrador-dog: 1:500 structure rib-3
- 👤️humans/courier-with-hand-truck: 1:100 body snap-ring
- 👤️humans/cyclist: 1:200 body rear-rim, front-rim
- 👤️humans/cyclist: 1:500 body rear-rim, front-rim
- 👤️humans/wheelchair-user: 1:200 body rear-rim-l, hand-rim-l, rear-rim-r, hand-rim-r
- 👤️humans/wheelchair-user: 1:500 body rear-rim-l, hand-rim-l, rear-rim-r, hand-rim-r
- 🚜️vehicles/city-bicycle: 1:200 body rim-r, rim-f
- 🚜️vehicles/city-car: 1:100 body rl-disc, rr-disc
- 🚜️vehicles/city-car: 1:200 body rl-disc, rr-disc
- 🚜️vehicles/city-car: 1:500 body fl-disc, fr-disc, rl-disc, rr-disc
- 🚜️vehicles/delivery-van: 1:100 body rl-brake-disc, rr-brake-disc
- 🚜️vehicles/delivery-van: 1:200 body rl-brake-disc, rr-brake-disc
- 🚜️vehicles/delivery-van: 1:500 body rl-brake-disc, rr-brake-disc
- 🧱️components/curtain-wall-bay: 1:200 body igu-edge-seal, spandrel-packer
- 🧱️components/curtain-wall-bay: 1:500 body igu-edge-seal, spandrel-packer
- 🧱️components/etics-brick-wall-12: 1:100 body bed-joint
- 🧱️components/etics-brick-wall-12: 1:200 body bed-joint
- 🧱️components/etics-brick-wall-12: 1:500 body bed-joint
- 🧱️components/etics-brick-wall-30: 1:100 body bed-joint
- 🧱️components/etics-brick-wall-30: 1:200 body bed-joint
- 🧱️components/etics-brick-wall-30: 1:500 body bed-joint
- 🧱️components/external-venetian-blind: 1:200 body edge-spacer
- 🧱️components/external-venetian-blind: 1:500 body edge-spacer
- 🧱️components/glass-balustrade: 1:200 body base-plate
- 🧱️components/masonry-wall-with-lintel: 1:100 structure joint-lintel-inner-left, joint-lintel-inner-right, joint-beam-bearing
- 🧱️components/masonry-wall-with-lintel: 1:200 body plaster
- 🧱️components/masonry-wall-with-lintel: 1:200 structure joint-lintel-inner-left, joint-lintel-inner-right, joint-beam-bearing
- 🧱️components/masonry-wall-with-lintel: 1:500 body plaster
- 🧱️components/masonry-wall-with-lintel: 1:500 structure joint-lintel-inner-left, joint-lintel-inner-right, joint-beam-bearing
- 🧱️components/panel-radiator: 1:200 body panel-rear, panel-front
- 🧱️components/solid-brick-wall-1960: 1:100 body bed-joint
- 🧱️components/solid-brick-wall-1960: 1:200 body bed-joint
- 🧱️components/solid-brick-wall-1960: 1:500 body bed-joint
- 🪑️objects/charcoal-grill: 1:100 body damper-plate
- 🪑️objects/charcoal-grill: 1:200 body damper-plate
- 🪑️objects/charcoal-grill: 1:500 body damper-plate, vent-dial
