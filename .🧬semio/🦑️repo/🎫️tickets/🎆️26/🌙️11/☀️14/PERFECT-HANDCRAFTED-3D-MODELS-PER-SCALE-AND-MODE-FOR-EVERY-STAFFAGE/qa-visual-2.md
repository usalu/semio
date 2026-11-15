# Visual QA 2: vehicles, machinery, vegetation

Second independent visual reviewer (prefix `qa2-`). Method: for every spliced asset of the three categories, fresh sheets of every authored scale (`volumes-render.ts`, default direction) plus two further axonometric directions per scale (az -55 el 28 = front-left-top, az 215 el -12 = rear-right-low) for body, thermal and structure, stitched per asset and read with the Read tool; closer single-list renders (`qa2-big.sh`) where a sheet left doubt. Everything under `T/🗑️generated/qa2/<slug>/` (`qa2-<slug>.s<N>.{sheet,flt,rrl,all}.png`, `qa2-<slug>.{A,B}.png` = four scales in one direction, `.body.<az>.png` closeups). Scripts: `T/qa2-run.sh`, `T/qa2-mont.sh`, `T/qa2-montage.ts`, `T/qa2-stitch.ts`, `T/qa2-big.sh`.

Grades: A perfect, B minor (named), C reads wrong (named). Graded from what is seen, not from validator numbers. Order: Ready list first (city-bus, city-car), then the rest. Assets whose source changed within the last 15 minutes while I reviewed are listed under "Pending" and are not graded.

Scale columns: body / thermal / structure, in that order. `-` = scale unavailable.

## Summary of what is wrong most often (read this first)

1. **Wheels are never in arches.** City-car, delivery-van, concrete-mixer-truck, vintage-car (partly), scissor-lift: tyres are free cylinders standing outside the body side wall, bodies have no wheel-arch cut-out, no fender or sill wrap. The 2D overlay is green because the projected silhouette matches; in 3D the wheel is stuck onto the side. The validator cannot see this.
2. **Structure is a copy of the body.** Excavator, forklift, diesel-generator, heat-pump, humidifier, scissor-lift, scroll-compressor, plate/finned HX and the solar collector use the whole enclosure or house as one solid "member". Real load paths (skid beams, engine bed, slew ring, boom box sections, cabinet corner posts, frame rails) are absent; cladding is structure, which the design says is absent.
3. **Hollow things are solid.** Excavator boom and stick are solid plates, concrete-mixer drum is a solid funnel in structure, sorption-wheel housing and compressor shell are solid drums, crane at 1:500 is solid prisms.
4. **Structure members float.** City-car/van roof rails without posts, tram pantograph and columns without roof or side frame, vintage-car body unsupported at 1:100 and coarser, hornbeam stems stop inside the crown, ivy stems hang free, vegetation branch rods unattached to masses.
5. **Mechanisms are dropped at the wrong scale.** Excavator loses all cylinders at 1:100, scissor-lift has no scissor arms at any scale, expansion valve loses its capillary at 1:100 (bulb floats), pumpjack/crane fine.
6. **Coarser scales are the same object with parts deleted** in vehicles (bus, van, car): identical masses and wheels, only glazing and fittings thin out; they never re-abstract (e.g. van 1:200 keeps the lumpy loft and the outside axles).
7. **Vegetation (unspliced previews)**: crown masses are flat pancakes or faceted cones in every species (maple, plane, birch, pine, lime): no species character, no pollard head, no multi-stem, no pine tiers; structure branches do not reach the masses.

## Ready list (validator green) disagreements

- `city-bus`: validator green; I agree on the whole (A-/B). Findings are minor (below).
- `city-car`: validator green, but **body C at 1:50, 1:100, 1:200, 1:500: wheels outside the body, no arches, flat side walls, no tumblehome**; structure C at 1:200/1:500 (roof rails float). Disagree.

## city-bus (Ready; source was changing at the time of my render, re-checked below)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | A- | B | Reads as a low-floor city bus (side glazing grid, roof pods with louvres, wheel arches with hub and nuts, bumpers, lamps, mirror stubs). Tyres are 12-gon; rear face a blank panel without rear window or lamp panel; mirrors tiny. Thermal: floor/skin/glazing/roof/battery/HVAC/brakes nested and in the right places. Structure: ladder floor + pillars + roof rails + door frames, plausible, axles with joints. |
| 100 | B | B | B | Glazing collapses to one continuous band, doors as thin panels; thermal one grey skin with only battery/HVAC coloured; structure roof is a full flat slab, not a frame. |
| 200 | B- | B | B | Same as 100, wheels are lumps in the sill, windscreen glazing ok. |
| 500 | B | B | B | Solid box with a glazing band, wheels invisible from above (only sub-sill stubs), windscreen slope kept. Acceptable abstraction. |

Instructions: 50 body `tyre-*` use 24 segments (or a revolved tyre with sidewall bulge); add a rear glazing/lamp panel on the rear face solid `rear-*`; mirrors at 1:50 should be 80 x 200 mm arms on the A-pillar. 100/200: keep at least the door positions as dark glazing separate from the band (two door leaves on the right side). 100 structure: replace the roof slab member by two longitudinal rails plus 5 cross ribs.

## city-car (Ready)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | C | A- | B | Hatchback silhouette, glazing, mirrors, lamps read well. But the four wheels are cylinders standing outside the flat side wall (no arch cut, no fender lip), hubcap spokes are loose sticks, side wall flat (no tumblehome), rear a vertical box with a bumper block, exhaust runs the whole sill length below the car. Structure: ladder frame + roll cage with stretched orange joint lens shapes. |
| 100 | C | B | B | Same body; thermal one grey skin; joint ellipsoids stretched. |
| 200 | C | B | C | Roof rails at the top are floating bars not meeting the pillars. |
| 500 | B- | B | C | Roof glazing missing, wheels are stuck-on discs; structure: two long floating bars above a plain frame, no pillars. |

Instructions: body `wheel-*`: move each wheel into a carved arch: add arch openings (subtract via lofted `wall` shell sections with an arch gap, or build the side wall as two lofts with the arch between) and put wheel centre 0.12 m inside the outer side wall plane; fender lip loft of 15 mm at 1:50. `wall`/`side` solids: add tumblehome by narrowing the roof-line loft sections 6 percent. Exhaust: keep it in thermal/structure only (hidden, under the floor between rails), not as a body solid running the sill. Structure 200/500: add the A/B/C pillars and connect `roof-rail-*` endpoints to them; shorten joint ellipsoids to near-spheres.

## tram

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | C | B | C | Articulated two-section tram with pantograph and skirt cut-outs reads well, windows grid ok. But (a) the windscreen plate leans forward over a recessed dark face like a visor, (b) the second car has no bogie, no wheels, its skirt runs to the ground, (c) structure is two sill beams, door portals and one gantry: no roof, no side frame; pantograph floats over nothing. |
| 100 | C | B | C | Same, windows fewer. |
| 200 | C | B | C | Same, bogie gone but wheel cut-outs remain. |
| 500 | B- | B | C | Two glazing panels, wheels sub-sill, structure as above. |

Instructions: windscreen solid (`windscreen`/`front-glazing`): rotate so that the top edge is 150 mm behind the bottom edge; add the nose cheeks. Second car: add the second bogie pair (and wheel cut-outs) under the rear car, or add a Jacobs bogie at the articulation; fix the skirt height of the rear car to match the front car. Structure: add roof beams (2 longitudinal + cross ribs every 3 m), side window pillars, connect pantograph base to a roof beam; floor sills should run through the articulation with the joint solid. Thermal 100-500: separate roof, glazing, doors, skin as regions with their own colour like at 1:50.

## vintage-car-1929

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | A- | B | Reads as a 1929 sedan: wire wheels, spare, free-standing lamps, radiator shell, running boards, fenders, bumper bars. Minor: bumper bars a thin ladder, running board straight slab, spare in the wing is a flat disc. Thermal: exhaust runs the full length and engine, tyres coloured well. Structure: ladder frame with axles, floor plate, mass blob. |
| 100 | B | B | C | Same, lamps simplified; structure: two thin rails + axles, floor plate dropped, body unsupported. |
| 200 | B | B | C | Same. |
| 500 | B- | B | C | Wheels plain discs, one glazing band. |

Instructions: structure 100-500: add 3 cross-members (front, mid, rear) and the floor plate member of 1:50 (or a coarser equivalent) so that body mass rests on the frame; connect the mass ellipsoid to the frame with a seat-support member. Body 500: add wheel arches visible as dark rings.

## delivery-van

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | C | B | B | Wheels hang outside the body, visible axle rods stick out beyond the sill, body is a lumpy faceted loft with bulging wheel-arch cheeks, cab shows only a windscreen and a tiny side window, no door lines, no mirror shapes; cargo box plain. Structure: frame, roof cage, door frame; a small floating plate. |
| 100 | C | B | C | Same, roof rails float. |
| 200 | C | B | C | Same. |
| 500 | C | B | B | Lumpy loft kept, no re-abstraction. |

Instructions: rebuild the lower body so that arches are carved into the loft (wheel `r` 0.36 inside the arch, 40 mm gap); remove the long axle capsules from the body list (axles belong to structure only) or hide them inside the sill; replace the lumpy cab loft by 6 clean sections (hood, A-pillar, roof) with side windows as separate glazing solids and add door gap lines; add mirrors. Structure 100/200: connect roof rails to pillars and remove the floating plate or anchor it to a cross-member.

## electric-kick-scooter (500 unavailable)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | A- | B | Deck, fork, stem, hinge block, fender, grips, wheels read as an e-scooter. Missing: headlamp, brake lever, kickstand, motor hub in the rear wheel; deck looks slim. Rear wheel only a ghost in structure. |
| 100 | B | B | B | Same, fewer parts. |
| 200 | B | B | B | Handlebar a bar, wheels discs. |

Instructions: structure: add the wheel rims as members (hub joint to deck at both ends); body 50: headlamp box at the stem top, kickstand capsule.

## excavator (re-rendered after the source went quiet: identical to the first render, grades stand)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | B | C | Reads as a crawler excavator (tracks with rollers and links are good, cab with glazing, counterweight, exhaust, rails). Boom and stick are solid plates (extrusions), no depth taper, no box sections; one boom cylinder lies nearly horizontal; no bucket cylinder or linkage (bucket just touches the stick end). Structure copies the body masses: solid house, solid boom, track frames as blocks, no slew ring. |
| 100 | B- | B | C | Tracks plain, only 1 cylinder. |
| 200 | C | B | C | Tracks as sausages, no cylinders at all, boom/stick unchanged. |
| 500 | C | B | C | Same. |

Instructions: boom/stick: model as hollow box loft (outer + inner offset 30 mm at 1:50) or two side plates + top and bottom plates; boom cylinder: from house front lug at 60 degrees to the boom mid lug; add stick cylinder (exists thin) and bucket cylinder + link (two bars) from stick top to bucket; keep boom and stick cylinders as slender at 1:100. 200/500: cylinders as one capsule each, track sections with 4 bevelled sprocket lugs. Structure: boom and stick as tube members with real sections, house as frame + slew ring, track frames as two beams with rollers.

## forklift

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | A- | B | Counterbalance forklift with overhead guard, steering wheel, seat, mast, tilt cylinders, forks and pallet load: reads. Mast rails only visible on one side from some directions; load box a plain box. |
| 100 | B | B | B | Same, fewer details. |
| 200 | B | B | B | Overhead guard is a flat slab, forks fused to the pallet. |
| 500 | B | B | B | Tyres as sub-body stubs, still reads. |

Instructions: structure chassis: replace the solid block by frame rails + counterweight solid + mast channels; add the second mast channel (both sides) in body 100+; 200: overhead guard as 4 posts + slats (3) not a slab.

## tower-crane

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | B | A- | Lattice mast, slewing head, jib with trolley, counter-jib with counterweights, cab, tie rods read as a crane. Hook block floats: the hoist rope is not visible at 1:50-1:200 (hairline). |
| 100 | B | B | B | Fewer lattice members, fine. |
| 200 | B | B | B | Same. |
| 500 | B | B | B | Solid tower and jib prisms: acceptable coarse abstraction; rope visible. |

Instructions: make the hoist rope at 1:50-1:200 at least one feature wide (7.5 mm at 1:50) so that the hook block is connected; thermal region colours inside the lattice are almost invisible; fine.

## concrete-mixer-truck

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | A- | B | Mixer truck reads (inclined drum, charge hopper, ladder, rear support frame, 4 axles). Cab is a flat-faced box with no hood/fender; wheels plain cylinders outside the frame without fenders; drum smooth with no spiral. |
| 100 | B | A- | B | Same. |
| 200 | B- | B | B | Cab box, wheel stubs. |
| 500 | B- | B | B | Same. |

Instructions: cab: add a sloped windscreen and side window loft, lower step; add fender solids over the wheels; drum body: one shallow spiral rib at 1:50; structure: cab absent: add cab floor/pillar members; drum member should be a shell (tube) not a solid funnel.

## diesel-generator

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | B | C | Canopy with louvres, silencer, exhaust stack with rain cap, discharge hood, skid tank, fuel fill: reads. No engine or alternator solids exist at all (hollow canopy). Structure is the whole enclosure `wall` as one solid member (cladding as structure) with skid box. |
| 100 | B | B | C | Louvres fewer. |
| 200 | B- | B | C | Louvres as flat dark panels. |
| 500 | C | B | C | Blank box, cannot be told from a cabinet. |

Instructions: add engine + alternator solids in thermal (hidden region) and body-hidden if the asset declares them; structure: skid beams (two I sections), engine bed, 4 lifting corners, mounting joints; remove `member-wall`. 500: keep the louvre panel darker and the silencer/hood on top (they exist) but add skid and the corner castings so the box reads as a genset.

## oil-pumpjack

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | A- | A- | B | Horsehead, walking beam, Samson post legs with braces, pitman and crank weights, gearbox, motor, polished rod, base: reads perfectly. Pitman arm visible on one side only. |
| 100 | B | B | B | Detail kept, fine. |
| 200 | B | B | B | Same. |
| 500 | B | B | B | Still full detail at 1:500 (not an abstraction), but recognisable. |

Instructions: structure: base slab solid; use two skid beams; add the second pitman arm.

## scissor-lift

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B- | B | C | Platform with guardrails, chassis and big wheels read; the scissor pack is a solid grey block with slivers: arms, pivots and lift cylinder are not modelled. |
| 100 | C | B | C | Same, wheels fewer. |
| 200 | C | B | C | Same. |
| 500 | - | - | - | not authored (3 scales) |

Instructions: stowed scissor: 6-8 nested arm bars (two pairs, each 50 mm) with end pivot pins visible from the sides at 1:50; at 1:100 two crossed pairs; structure: arms as members with pivot joints, platform frame as members (the guardrails are ghost only), chassis rails.

## heat-pump-outdoor-unit (first judged from the wrong side; corrected after a front-direction render, `qa2-heat-pump-outdoor-unit.front.png`)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B- | B | C | Box with circular wire fan grille, hub, blades, side coil louvre recess, feet: reads as a heat-pump outdoor unit from the front. But a horizontal slab (flap/deflector) lies across the middle of the fan and stands proud of the face: reads as a tray or shelf, not as an air outlet; louvre side is a plain recess. Structure is a solid copy of the box with the slab as a member. |
| 100 | B- | B | C | Fan reduced to a flat dark disc, same slab. |
| 200 | B- | B | C | Same. |
| 500 | B- | B | C | Same. |

Instructions: body 50/100/200/500: replace the horizontal slab by the real outlet louvre (vertical or horizontal blades inside the grille ring, 3 to 5 blades at 1:50 flush with the ring) or remove it; at 1:100 keep ring + two crossing spokes + hub instead of a flat disc, 1:200/500 a darker disc with a ring is right; side louvre: 8 slats at 1:50. Structure: corner posts, base rails, fan motor bracket (spoke ring) instead of the solid box. Note for the review method: the default sheet and my two directions never showed the front face; always add a front direction (az 110 for this asset).

## scroll-compressor (500 unavailable)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | B | B- | Hermetic shell with dome, suction stub, terminal boss, base plate: reads; plain. |
| 100 | B- | B | B- | Same. |
| 200 | B- | B | B- | Same. |

Instructions: structure: shell as a tube (hollow), add the scroll frame/bearing member inside (the thermal hidden scroll exists), mounting feet joints.

## solar-thermal-collector

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | A- | B | B | Frame, glass, stand legs and base rails read as a collector. Thermal has a hatched half-disc at the top right of the absorber (a cut face of unknown meaning). Structure: collector box as one solid slab. |
| 100 | B | B | B | Same. |
| 200 | B | B | B | Same. |
| 500 | B | B | B | Slab, fine. |

Instructions: thermal: explain or remove the half-disc cut face (look at `sectionFaces` there); structure: frame as 4 rails + back plate instead of a slab; stand: add the diagonal brace.

## sorption-wheel

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | A- | A- | B- | Housing ring, media face with purge sector and six amber spokes, base: reads. |
| 100 | B | A- | B- | Two spokes only. |
| 200 | B | B | B- | Media face plain. |
| 500 | B | B | B- | Disc with sector. |

Instructions: structure: housing as ring (hollow) + hub + 6 spokes; currently a solid drum.

## evaporative-humidifier

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B- | B | C | Box with front window and open top: does not read as a humidifier (no tank, pad ribs, fan, outlet). |
| 100 | B- | B | C | Same. |
| 200 | B- | B | C | Same. |

Instructions: add the media pad (ribbed), water tray, fan and outlet grille visibly in body; structure: frame posts + tray, not a solid box.

## expansion-valve (50, 100 only)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | B | B | Valve body, dome, inlet/outlet stubs, capillary and sensing bulb read as a TXV. |
| 100 | B- | B | B | Capillary dropped: the bulb floats unconnected next to the valve. |

Instruction: 100: keep the capillary as one slender capsule or move the bulb onto the stub (pipe clip).

## finned-heat-exchanger

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | B+ | B- | Solid grey block with tube end holes; fin pack not shown (fin pitch below the feature size: acceptable) but the block face is a flat plate. |
| 100 | B | B+ | B- | Same. |
| 200 | B | B | B- | Block. |

Instructions: add 3-4 fin-pack groove lines on the face at 1:50; structure: end plates + tube sheets as members, fins not structural.

## plate-heat-exchanger

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | B+ | B- | Port frames as recessed rectangles, not round nozzles; plate stack visible on one side. |
| 100 | B | B | B- | Same. |
| 200 | B- | B | B- | Block with ports. |

Instructions: ports as short pipe stubs (revolve) with flange; structure: front/back cover plates + 4 tie bolts, not a solid block.

## hornbeam-hedge

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B | A- | B | Clipped hedge crown with lumpy top on six visible stems and a planter plate: reads as a pleached hedge. |
| 100 | B | A- | B | Stems hidden, plinth, six humps. |
| 200 | B | B | B | Simplified. |
| 500 | B- | B | B | Single rounded sausage on a plinth: hedges are flat topped; the rounded ends look like a bolster. |

Instructions: structure: stems must run into the crown and carry masses (they stop inside the crown in 50: add the lateral branches to each crown lump); body 500: flat top, vertical ends.

## ivy-on-trellis

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B- | B | B- | Trellis frame with lattice reads well; the ivy is faceted pillow clumps (no leaf reading, no climbing stems), stems hang vertically below the mass over the lattice. |
| 100 | B- | B | B- | Same. |
| 200 | B- | B | B- | Same. |
| 500 | B | B | B- | Slab. |

Instructions: body 50: ivy as overlapping leaf cluster shells following the trellis (denser at the lattice); stems as sinuous capsules following the trellis rails; structure: add trellis lattice members and attach the stems to the lattice.

## date-palm (source quiet for 17 minutes at render time)

| scale | body | thermal | structure | note |
|---|---|---|---|---|
| 50 | B- | B | B | Straight tapered trunk with base flare, boot/leaf-base cluster, a crown of about ten stiff pointed fronds: reads as a palm but a sparse one. A dark rectangular slab hangs on the trunk below the crown (looks like a plank, not leaf bases). Fronds are short stiff spikes like an agave, not long arching pinnate fronds; no fruit bunches; no trunk scars. Structure: trunk tube segments with frond stubs, fine. |
| 100 | B- | B | B | Same, serrated frond edges. |
| 200 | B- | B | B | Same, fewer. |
| 500 | C | B | B | Crown is a folded-fan slab/umbrella of four plates: reads as a plate on a pole. |

Instructions: body 50: remove or rebuild the dark slab under the crown as 3 overlapping leaf-base sheaths wrapping the trunk (revolve segments), fronds as long arched lofts (2 m at 1:50 scale) with 3 to 5 sections drooping at the tips and a rachis capsule, outer fronds hanging below horizontal, add a date bunch ellipsoid under the crown; 500: crown as a low dome of 6 to 8 radial arched blades (not a fan slab), at least one blade pointing down.

## Pending (source still changing while I reviewed, not graded)

At the end of this review the following spliced assets had been edited within the last 15 minutes (age in minutes at the time of the last look): agave 0, apple-tree 2, bamboo-clump 1, english-oak 3, hosta-clump 0, weeping-willow 0 (source not yet spliced), city-bus 0 (graded from the render of 22:36, re-check once the source is quiet). Their sheets exist under `T/🗑️generated/qa2/<slug>/` from earlier snapshots and are stale; they need a new pass (`./qa2-run.sh vegetation/<slug>; ./qa2-mont.sh <slug>`).

## Extras: unspliced assets previewed (not part of the Ready/splice list; findings for the authors)

| asset | body (50/100/200/500) | structure | one-line |
|---|---|---|---|
| japanese-maple | C/C/C/C | C | Crown is a stack of flat pancakes; no branching visible; branch rods float; not a maple. |
| lombardy-poplar | B-/B-/B-/B | B | Column and tiers read; no pointed apex; twigs absent. |
| london-plane-pollarded | C/C/C/C | C | Lollipop: bare trunk and a ball of cone lobes; no pollard head or knuckles; twigs float. |
| multi-stem-birch | C/C/C/B- | C | Single trunk, flat tiers: not multi-stem, not airy. |
| scots-pine | B-/B-/B-/C | C | Bare trunk is right; crown is a few flat pancakes, too small; branches do not reach crown masses. |
| miscanthus-grass | B+/B-/B-/B- | B | Fan of blades and plumes reads as a tussock at 1:50; coarser: flat crossed quads, cone. |
| small-leaved-lime | B-/B-/B-/C | B- | Faceted lobes read as a generic tree; 1:500 body is a trunk stick only (crown missing in body). |
| cargo-bike, city-bicycle, motor-scooter, wheel-loader | not reviewed | | not spliced and source recently changing |
