# Asset agent A6 report

Assets (both handcrafted, generators in `T/🧰️a6/`: `car-shapes.mjs`, `vintage-car.mjs`, `split-ac.mjs`, `build.mjs`, `cast.sh`, `view.py`):

## vintage-car-1929 (`🚜️vehicles`)
- Voisin C7/C11 class saloon: 4.38 m by 1.70 m by 1.70 m, 1300 kg, wheelbase 3.0 m, track 1.4 m, wheels r 0.37 m (5.00-19). Separate crescent fenders, running boards, side spare wheel, louvred bonnet, nickel radiator and headlamps, fabric roof slab, three side windows, 12-spoke wheels with valve (rotation legible).
- Rig: chassis, fixed axles (carry the tyre contacts), rolling wheel bones. Poses `parked`, `driving`, `braking` (+ key poses). Clips `drive-by` (theta = -x/r, as city-car) and `arrive-and-park`.
- Plan mechanism: there is no separate plan asset type. Plan geometry is `representations[scale].geometry.plan` (x = length, y = width, bound to the root bone, shape follows the bone only by x travel), and the manifest/presentation list `viewpoints: [elevation, plan]`; the cast uses `viewpoint: plan`. Authored at all four scales (the plan has no wheels: fenders cover them). Same mechanism as agave, bamboo, hosta, ivy and spiral-stair. A5's city-car/humans plan work was not yet present in the files when read.
- Thermal: engine louvres, radiator, exhaust, bonnet, tyres, drum brakes, tailpipe emitter; all authored-estimate; thermal cast shows warm bonnet, radiator, exhaust, tyres.

## split-ac-indoor-unit (`🔌️appliances`)
- 0.85 m wide (depthM), 0.29 m high, 0.21 m deep (widthM), 9 kg. Origin at the casing bottom on the wall plane; wall hook (pin) and rest (roller). Bones unit and a rotating discharge vane (0 to -55 deg).
- Poses `off`, `cooling` (vane -35), `cooling-low`; clips `start-cooling`, `vane-swing` (loop). Heat binding: operating keys `cooling` (0, 0.4, 1) and `fan`; 2.5 kW cooling = 2.0 sensible + 0.5 latent, 0.7 kW electric (EER 3.6), documented in the assumptions. Cold air jet is an emitter `cold-jet`: 0.125 m3/s, -13.3 K (the schema forbids negative vapour change, so the latent share is noted, not modelled). Visible surfaces cooled slightly (-15 W casing, -8 W vane).
- Plan view authored; 1:500 unavailable (alternatives 200, 100).

## Verification
- Manifest rows added (vehicles after city-car; appliances before toaster).
- Catalog check per asset passes (office-laser suitable at all authored scales). The full-manifest check currently fails only on other agents' in-flight rows (person-cooking, laptop, desktop-monitor, pendant-lamp analytics).
- Casts exported with the cast exporter (dark-line, dark-solid, thermal, plan; car clip) and inspected as PNG; iterated (seats, steering, roof slab, radiator).
- Staffage test run: see the final status line below.
- The staffage vitest run was still running when this report was written; output goes to T/🗑️generated/a6/test.txt.
