
## From author 29 (animals)
- animals/pigeon: wings are flat plates; front toe bars 64 mm wide in the drawing (evaluator derives width from foot length).
- animals/roe-deer: trunks at 1:50/1:100/1:200 visibly tubular; structure members generated, not hand-sized.
- animals/mallard-duck: 1:50 list is the round-1 author's, reviewed and kept.

## From author 38
- objects/playground-swing 1:200 and 1:500: legs, braces and chains were thickened to the drawn stroke widths; under C24 they go back to true-size `slender` solids once the validator implements it (thermal total then drops back).

## From author 32 (vehicles)
- vehicles/delivery-van, vehicles/city-bus: cab and shell have no tumblehome; coarse hulls faceted; bus 1:500 buries tyres, drums and e-axle.
- vehicles/city-car, delivery-van, city-bus: `sectionFaces` was used on the shell underside (free air) — to be removed under C25; car 1:100 tailgate panel inner side to be judged.

## From authors 41, 46, 25 (animals — anatomy)
- animals/domestic-cat: head reads slightly rodent-like.
- animals/hedgehog: legs are stubs (rig pose floats the paws; elevation draws no legs).
- animals/herring-gull: wings read as two long rods; drawn bill, legs and feet much thicker than a real gull's (2D); front IoU 0.88–0.89 at 1:100/1:200.
- animals/goat: 1:500 bar through the head (pen-widened ear stroke); horns and hoof wedges drawn far larger than declared areas.
- animals/pig, animals/sheep: trunks are long uniform barrels.
- animals/pigeon: wings flat plates. animals/roe-deer: trunks tubular.

## From reviewer 2 (areas of machinery, vehicles, components, objects; C29, details in area-decisions.md "Reviewer 2")
- Conditioned layered regions (R-C: sliding-door leaf-frame/track-steel/jamb-steel, tilt-window frame-timber/frame-sill, cantilever-balcony thermal-break, tied-canopy wall-plate, vehicle shells): the INTERIOR-side face of the layered solid must carry `sectionFaces` (conditioned boundary, C25 b), otherwise the model exposes more than the decided one-sided value. tilt-window: the jamb linings (0.3 deep) are bound to `frame-timber` although the region is the sash.
- components/spiral-stair column-steel and cantilever-balcony railing-steel posts: the open bores (1.37 and 0.35 m2) are sealed voids of capped sections; fill the bore with a cavity solid (as the sliding-door does) or mark it, the decided values are outer faces only.
- components/panel-radiator valve: the region title names one thermostatic valve and its supply pipe, the solids bind both valve bodies, both pipes and the lockshield cap (0.078 m2); either split a lockshield region (return end temperature) or rename the region.
- machinery/concrete-mixer-truck engine-bay (tunnel panels and belly pan missing), forklift drive-motor (no motor housing) and mast (rails drawn as filled boxes, not C channels): unchanged defects, the committed values stay.
- machinery/tower-crane trolley: the frame is a filled 1.4 x 0.55 x 2.0 block (9.3 m2); the real trolley is an open frame of box girders (about 10 m2 in all), redraw it open.
- machinery/evaporative-humidifier casing: front and rear panels are 10 mm boxes whose inner faces count; decided value is outer faces (1.5), bury or mark the inner faces.
- machinery/excavator engine-compartment: the drawn engine hood is 2.4 m wide (the first decision assumed 1.0 m); hydraulics model counts the whole rod length, the part inside the barrel is the same region and does not count.
- machinery/solar-thermal-collector: after C25 the glass underside and the absorber underside count; the housing cavity faces count too.
- vehicles/city-car tailgate: 1:50 must carry the same inner-side `sectionFaces +v` as 1:100 to 1:500. vehicles/city-bicycle, cargo-bike and machinery/oil-pumpjack, scissor-lift, wheel-loader have no `volumes` in the file: their areas are hand values until the models land.
- Sweep for the validator: regions whose committed area was an exposed-only value and that no author touched (for example tealight wax, other covered or contact surfaces) are not in the audit; `volumes-thermal-coverage` under C29 will report them.

## Area review (reviewer)
- agave/caudex: the `caudex` extrusion is a 0.42 x 0.22 x 0.26 m box (rectangular profile and flat extrusion); a revolve or loft of o0.22 would expose 0.29 m2
- apple-tree/limb-bark: only 4 scaffold and 4 secondary limbs are modelled, a vase crown of a full-standard tree has ~8 secondaries per scaffold plus tertiaries (about 8 m2 of bark)
- apple-tree/trunk-bark: the `trunk` loft starts 0.35 m below ground (buried root flare r0.55) and tapers to a point at 3.64 m, i.e. a central-leader cone through the vase crown that the real tree does not have
- bamboo-clump/culms: every culm is a flat ribbon, the extrusion of a 0.029 m wide profile over z -0.37..0.37 (0.74 m wide), exposing 19 m2; a revolve/loft o0.029 is needed
- date-palm/stem-bark: the 26 ring and 9 skirt/plug detail extrusions sit on the stem surface and are counted as additional exposed bark (2.5x the stem), they must be flagged `texture` or kept inside the stem envelope
- ivy-on-trellis/stems: the hidden stems region carries only a fraction of the stems (0.14 m2 against 0.5 m2 of 4 main stems and laterals)
- concrete-mixer-truck/drum: the 59-section `drum-flight` loft (spiral blade, inside the drum) is counted as exposed outer surface; 51.8 m2 against ~36
- concrete-mixer-truck/engine-bay: the region has the front grille only (0.65 x 1.4); tunnel panels and belly pan of the engine compartment are missing as thermal solids
- diesel-generator/discharge-hood: the 40 mm plates (layer says 2 mm) are counted on both faces and the outlet frame, 10.2 m2 against 4.2 m2 of outer shell
- diesel-generator/inlet-louvres: slat boxes are 25 mm thick (layer 2 mm), counting 0.157 m2 per slat instead of 0.096
- excavator/cab-glazing: side, rear and roof windows are slivers (0.2 m2 beyond the 1.23 m2 windscreen)
- excavator/house: measured 17.6 m2 for house + cab + boom foot, the profile extrusions alone give 30 m2: parts of the cab and house are buried or the cab overlaps the house
- excavator/hydraulics: only one boom cylinder is modelled (the real excavator has two) and no hoses
- forklift/drive-motor: the region is the flat 0.5 x 0.1 x 0.8 `drive-cover` only; motor and gearbox housing are missing
- forklift/mast: rails are solid 0.14 x 0.14 and 0.10 x 0.14 boxes; real mast rails are C/I-channels whose inner faces are exposed (5 m2)
- heat-pump-outdoor-unit/casing: 1-2 mm panels are counted on both faces (inner sides included), 6.1 m2 against 3.4-3.8 m2 outer
- heat-pump-outdoor-unit/coil-guard: the `coil` box 0.22 x 0.86 x 1.06 is a solid slab; both big faces and edges (2.8 m2) are counted, the back face looks into the fan chamber
- scroll-compressor/scroll: the plate-with-spiral solids count both faces, edge and spiral flanks (0.0158 m2) of a part of which only the glass-facing face belongs to the region
- solar-thermal-collector/absorber: the nine risers are counted as exposed surface on the sun side (2.38 m2 against 1.71 m2 plate face)
- runner-with-blade-prosthesis/socket: loft of 9 sections exposes 0.068 m2 against 0.0959 (-29 %), check the section radii of the socket loft
- ceiling-diffuser/plate: the stepped cones are revolved round (o0.595) instead of square, exposing 0.24 m2 against 0.42 m2
- desktop-monitor/housing: the 6-section loft exposes 0.124 m2 against ~0.2 m2 of back shell and rim
- electric-kettle/lid: lid dome counted on both faces, inner face is the steam side
- electricity-meter/window: glass inner faces are counted, 0.0245 m2 against 0.011 m2 room-facing
- gas-boiler/connections: stubs and nuts expose 0.025 m2 against 0.043 m2 of five o22 mm stubs with union nuts
- gas-boiler/controls: the panel plate between display and knobs is in `casing` solids; the region exposes 0.015 m2 of 0.03 m2
- gas-hob/burner: the exposed measure drops the part under the flame seat (0.006 m2); a brass burner at 350 C needs ~10 kW/m2 of 120 W
- server-rack/feet: the foot solids expose 0.025 m2 against 0.057 m2 of four o70 mm castors
- tower-server/side-panel: the two panel boxes give 0.36 m2 but 0.2715 m2 is measured; check what removes 25 % (sectionFaces on the +w/-w faces of the panels or a buried portion)
- vacuum-cleaner/body-shell: measured exposed 0.31 m2 leaves out faces covered by hose and wand sockets and the floor side (C29: total counts them)
- city-bus/brakes: the brake solids expose 0.8 m2 against 1.2 m2 of four o0.43 discs with calipers
- delivery-van/exhaust-system: hidden region without any solid in thermal (0.8 m2 declared, none modelled)
- bar-stool/backrest: the pad loft exposes 0.021 m2, one third of the two faces and rim (0.07 m2)
- park-bench/back-timber: measured 1.26 = faces only (3 x 0.42), slat edges missing
- shopping-trolley/gate-wire: the gate is four 25 mm bars (capsule r 0.0125) instead of a o4 mm wire grid

## Visual QA 2 (vehicles, machinery, vegetation; details and per-scale grades in qa-visual-2.md)
- city-car: wheels are cylinders outside the flat side wall at every scale (no arches cut, no fender lip, no tumblehome); the 2D overlay is green, the validator cannot see it. Roof rails float at 1:200/1:500 structure. Grade C.
- delivery-van: wheels and axle rods outside the body, lumpy faceted loft with bulging arch cheeks kept at all scales, cab has windscreen and a tiny side window only, roof rails float at 1:100/1:200. Grade C.
- tram: windscreen plate leans forward over a recessed face; the second car has no bogie and its skirt reaches the ground; structure is only sills, door portals and a gantry (no roof, no side frame, pantograph floats). Grade C.
- vintage-car-1929: 1:100 to 1:500 structure is two thin rails plus axles, body unsupported (floor plate and cross members of 1:50 dropped). Grade C structure.
- excavator: boom and stick are solid plates (no box sections), one nearly horizontal boom cylinder, no bucket cylinder or linkage, no cylinders at 1:200/1:500; structure copies the body masses (solid house, solid boom, blocks for track frames, no slew ring). Grade C structure, B/C body.
- forklift: structure chassis is one solid body block, mast channels visible on one side only. Grade B.
- concrete-mixer-truck: cab is a flat-faced box, no fenders over the wheels, drum smooth, structure drum is a solid funnel and the cab has no structure. Grade B.
- diesel-generator: no engine or alternator solid anywhere (hollow canopy); structure is the whole enclosure `wall` as a solid member (cladding counted as structure), no skid beams/engine bed; 1:500 body is a blank box. Grade C structure.
- scissor-lift: the scissor pack is a solid block with slivers (no arms, pivots or cylinder) at all scales; platform rails are ghost-only in structure. Grade C.
- heat-pump-outdoor-unit: a horizontal slab lies across the middle of the fan grille and stands proud of the face at every scale; structure is a solid copy of the box. Grade B-/C.
- scroll-compressor, sorption-wheel, evaporative-humidifier, plate-heat-exchanger, finned-heat-exchanger, solar-thermal-collector: structure is a solid copy of the housing, shell or collector box (hollow things solid). Grade B-/C. Humidifier body does not read as a humidifier (no pad, tank, fan visible).
- expansion-valve 1:100: capillary dropped, the sensing bulb floats unconnected beside the valve.
- tower-crane 1:50 to 1:200: the hoist rope is a hairline (invisible), the hook block appears to float.
- city-bus/city-car (Ready): bus A-/B (tyre 12-gons, blank rear face, tiny mirrors, 1:100 roof structure a flat slab); city-car C in body and at coarse structure although the validator is green.
- hornbeam-hedge: structure stems stop inside the crown (no laterals to the crown lumps); 1:500 body is a rounded sausage (hedges are flat-topped).
- ivy-on-trellis: ivy is faceted pillow clumps with loose vertical stems hanging over the lattice; structure lacks the trellis lattice and the stems float.
- date-palm: sparse stiff spike fronds read like an agave; a dark slab hangs under the crown at 1:50; 1:500 crown is a folded-fan plate on a pole. Grade B-/C.
- Pre-splice previews (unspliced, for the authors): japanese-maple, london-plane-pollarded and multi-stem-birch are C (stacked pancake or cone crowns, no pollard head, no multi-stem, branch rods floating); scots-pine B-/C (crown too small and flat); small-leaved-lime 1:500 body is a trunk stick only (crown missing in body).
- Method note: sheets and the two extra directions can miss a face (heat-pump front fan was only seen after a front render); always add a direction facing the asset's front.

## Visual QA 3 (humans and animals; details in `qa-visual-3.md`)

Incomplete: 21 of 42 assets graded (image viewer hook failed), 21 rendered but not graded (see the report).
- Humans (10 graded) are one construction family (box torso, tube limbs, ball shoulders, blade hands, wedge shoes) of uneven quality; from 1:100 thermal collapses to one grey region and at 1:500 structure is a solid body copy; child proportions missing (child-on-kick-scooter, running-child); running-child and runner-with-blade-prosthesis do not show their pose.
- Animals (11 graded): segmented-cylinder barrels, stick legs, cone necks; detached ears (red-squirrel, roe-deer), detached wings (mallard-duck, herring-gull, pigeon), floating thermal discs above the back (sheep, goat, deer, fox, cow), cow rump half-disc, duck vertical plate, worker 1:500 floating thermal cube, hedgehog forelegs as barrels.
- Validator-green but wrong: hedgehog (Ready) body C-; every overlay green on assets that read wrong.

## Area review 3 (reviewer 3)
- hot-water-cylinder/zone-1..4, top-cap: the end faces between the stacked slices of one cylinder are interfaces, not outer faces: mark them `sectionFaces` (C25 a); measured 1.18 per zone against the side 0.62, top-cap 0.62 against 0.31
- ceiling-diffuser/plate: the back face of the flange box lies against the plenum (C25 b) and carries no `sectionFaces`: measured 0.757 against the room-facing 0.45
- bar-stool/seat-plate: `+u` on the plate hides the face against the pad; the pad is another region, so under C29 the face counts: remove the marking (0.124 measured against 0.216)
- enclosed inner faces (electric-kettle/lid, electricity-meter/window, toaster/top-plate, electric-fan-heater/top-cover, grille, intake, circulation-pump/rotor-window, desktop-monitor/panel-front, operating-table/light-lens, seated-guitarist/guitar, person-cooking/pot, kitchen-extractor-hood/filter): the solids are closed, so the face that looks into an enclosed cavity (a sealed casing, the heater interior, the sound box, the pot charge) is counted; R-A and R-B count the outward face only. Either model such a part as a hollow shell (C30) or the coordinator extends C25 to the inner face of an enclosed cavity; until then these regions fail `volumes-thermal-coverage` by 30-90 %
- leaning-person/wall-face: conditioned wall: the interior-side face has no `sectionFaces` (R-C), measured 6.46 against the outward 3.5
- toaster/feet: feet drawn o0.05 (real o0.025): 4x the area (0.020 against 0.007)
- tower-server/feet: `-v` marks the ground faces of the pads; C29 counts the ground contact (0.010 measured, hand 0.016)
- server-rack/feet: castors modelled as o0.032 posts instead of o0.07 wheels (0.0246 against 0.058)
- bookshelf/plinth: plinth drawn as a solid 0.25 m deep block (0.539 m2) instead of a 18 mm kick board (0.147 m2)
- operating-table/light-housing: `th-light-plume` ellipsoid (0.26 m2) is a convection plume, not a surface, and counts in the region
- english-oak/limb-bark: limbs drawn up to r0.37 over 5.7 m: 66 m2 measured against 35-42 m2 from the wood volume of the 5971 kg tree
- scots-pine/limb-bark: limbs drawn o0.25 at the base: 15.1 m2 against hand 11.9
- person-with-umbrella/umbrella-frame: two of eight ribs modelled
- gas-hob/knob: one of two knobs modelled
- kneeling-gardener/hat: validator measures 0.398 against the physiological 0.226 (brim underside counted); C18 keeps the value, the coordinator decides whether the validator compares clothing of living subjects with the sky-facing surface
- humans, face and head skin (about 30 assets): the face solid covers the whole head including the scalp under the hair (hair is exchange, not measured): measured 1.2-2.2x the face value; compare face+hair
- humans, skin under garments (jogger, person-on-crutches, runner, sleeping-person, ...): the skin solid runs under the garment solid and counts whole (C29: covered faces count) while the declared skin value is the bare part: the subject total measures 0.74-0.98 of the declared total; shorts drawn loose (person-on-crutches 0.567 against 0.197), helmets and caps drawn 1.7-2.5x
- animals, small skin parts: every hoof, horn, beak, cere, toe, web and paw of cow, goat, sheep, pig, roe-deer, pigeon, gull, mallard, squirrel, cat and hedgehog is drawn 2-12x the real part (numbers in the C27 table of the Reviewer 3 section); dairy-cow udder 0.355 against 0.45 is the one undersized part

## Visual QA

Report `qa-visual.md` (2026-11-14/15, images judged by eye, validator numbers ignored). 61 of 69 spliced assets in appliances, furniture, components, objects graded: A 49, B 490, C 109 (structure C 63, thermal C 32, body C 14). None of the 21 validator "Ready" assets is perfect. Recurring: structure as a filled block; floating masses and single/invisible supports; slender legs/rods thinner than the drawing at 1:200/1:500 (accepted by `slender`); coarser scales that are copies of the finer scale; thermal cutaways without hidden internals (heat-exchanger, coil, bottle, insulation layers); faceting (12-16 sides); blank box bodies. Worst Ready assets: mvhr-unit, split-ac-indoor-unit, cooking-pot, hot-water-cylinder. Not graded (tooling failure): market-parasol, playground-swing, retractable-bollard, shopping-trolley, stepladder, street-lamp, tealight, waste-bin. Per-asset instructions with solid ids: `qa-visual.md`.
