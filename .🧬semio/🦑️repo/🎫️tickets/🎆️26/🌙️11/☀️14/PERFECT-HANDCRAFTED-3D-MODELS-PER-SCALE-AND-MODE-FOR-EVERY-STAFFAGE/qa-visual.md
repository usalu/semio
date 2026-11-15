# Visual QA of spliced models (appliances, furniture, components, objects)

Reviewer: independent, no edits to models, assets or tools. Method: `qa-render.ts <asset...>` (own copy of the render code, in T) writes for every authored scale a sheet (overlays + body/thermal/structure axonometric) and a views image (front-left-top az 125 el 25, rear-right-low az 215 el -15; body, thermal cutaway, thermal uncut outside, structure). Every image was opened with the Read tool; grades come from what is visible, not from validator numbers. Images: `🗑️generated/qa/<category>/<slug>.s<N>.sheet.png|views.png`. Source tables of the other reviewers: `🗑️generated/qa/part-1..7.md` (their content is repeated below).

## Coverage and totals

- 69 spliced assets in the four categories. Graded: 61 (the 21 on the validator "Ready" list first, then the rest). NOT graded: market-parasol, playground-swing, retractable-bollard, shopping-trolley, stepladder, street-lamp, tealight, waste-bin (all objects): from the second pass on the Read tool of this host failed with "PreToolUse hook did not respond before its timeout" for me and for two reviewer sub-agents (renders exist under `🗑️generated/qa/objects/`; playground-swing changed < 15 min before and must be re-rendered). They are the open work of this review.
- Grades (asset x scale x list, 61 assets): A 49, B 490, C 109. By list: body A 31 / B 171 / C 14; thermal A 15 / B 169 / C 32; structure A 3 / B 150 / C 63.
- By category (all lists): appliances A 3 / B 152 / C 61, furniture A 25 / B 102 / C 17, components A 17 / B 170 / C 20, objects (only 4 graded) A 4 / B 66 / C 11.
- Of the 21 "Ready" assets none is perfect: the validator Ready list is not a statement about the look. Closest to A: folding-deck-chair (50), bookshelf (50/100), standing set of the better appliances (electric-fan-heater body 50). Worst Ready assets: mvhr-unit, split-ac-indoor-unit (thermal and structure C at all scales), cooking-pot (structure C at all scales), hot-water-cylinder (structure C).

## Recurring weaknesses (all categories)

1. **Structure as a filled block.** Members are the body shells relabelled or solid fills (fan heater, MVHR, split AC, hot-water cylinder tank, vacuum canister, single bed mattress, sofa arms/back, panel radiator, ramp wedge, chalkboard, tilt-window jambs, walls). A hollow sheet casing or a frame with plates is the real load path. Single biggest C source (63 of 109).
2. **Masses and supports.** Mass solids float in the void (bar-stool, park bench, tower server and rack), sit on leg ends instead of the centre of mass (cafe-chair), or hide inside opaque member shells; supports are one pad instead of one per foot, invisible, or float (park-bench 100/200, garage door 500, hot-water cylinder).
3. **Slender-exempt legs and rods much thinner than the drawing at 1:200 and 1:500** (bar-stool, cafe-chair, charcoal-grill 500, vacuum wand, spiral-stair balusters 12 mm vs 15 mm, bicycle stand, patio-heater pole). The overlay shows a red halo around every such part; the validator passes it as `slender`.
4. **Coarser scale is a copy of the finer one**, often with only a joint or mass dropped (bookshelf 200/500 equals 100, hood 100 = 50, laptop 200 = 100, rack 200 = 100, pendant 100 = 50, circulation-pump thermal 200 = 100, split-AC thermal 50/100/200 identical, MVHR thermal 100 = 200, stair 500 = 200, walls 200 = 100). Principle of design.md ("new abstraction") not met.
5. **Thermal cutaways omit hidden internals**: heat exchanger core, filters, fan, evaporator coil, bellows, gas bottle (patio-heater), heat exchanger and insulation (gas boiler), motor and bag (vacuum). Regions exist but are empty hulls or a single hatched slab; layer shells (insulation, steel) not nested visibly (hot-water cylinder, etics and brick walls at 1:500 drop the insulation layer).
6. **Faceting** of round parts (12-16 sides: circulation pump, TRV, cooking pot, grill bowl, pendant dome, patio-heater dome, hot-water cylinder flutes) and stair-stepped lofts at seat pads and motor ridges (overlapping revolve rings).
7. **Plain, blank boxes** where the object has detail visible in any drawing (MVHR front, split-AC panel seams, walls without courses, tilt-window sash/handle/hinge invisible, door leaf blocks).

## Per category

- **Appliances** (24 spliced; 16 graded): worst on thermal and structure. Bodies are mostly B (reads right, faceted, simple); thermal C where the real heat path is absent (MVHR, split AC, kettle water drawn as fan wedges, gas boiler empty cavity, vacuum); structure C as filled blocks (fan heater, MVHR, split AC, vacuum, hood) or floating masses (tower server, rack, gas hob trivet mass).
- **Furniture** (12 spliced; all graded except none missing): best category. Bar-stool/cafe-chair/deck-chair/bookshelf read right at 50; failures are at 200/500 (legs thinner than drawn, copies of 100) and structure (sofa blocks, bed slab, park-bench legs stop at the stretcher so supports float).
- **Components** (18 spliced; all graded): bodies often block stand-ins (walls, door, window), structure slabs, many 1:100 copies of 1:50, thermal at 1:500 drops insulation layers (etics 12/30, venetian blind 24 lamellae vs 5 in the body), floating handrails and rails at 200/500 (spiral stair, ramp, balustrade), invisible supports (curtain wall 8 mm).
- **Objects** (15 spliced; 4 graded Ready ones + earlier ones in part-6 NOT graded): cooking-pot structure C at all scales and thermal base open at 100/200; charcoal-grill 500 legs under feature; patio-heater has no gas bottle and a thin 500 pole; crate stack 500 structure hollow where body is solid.

## Instruction format

Each asset below: table asset x scale x list (body / thermal / structure) with grade and one line; then "Instructions" (solid ids or parts, what to change) and whether I disagree with a green validator. A = perfect, B = minor (named), C = reads wrong (named).

---

# Part A: Ready-list assets graded by the lead reviewer
## furniture/bar-stool (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | A | B | Reads as a bar stool (padded seat, curved backrest, 4 splayed legs, foot ring, feet). Seat-pad rim shows stair-stepped facets (loft rings); structure mass ellipsoid floats in the void between legs at ring height. |
| 100 | B | A | B | Same stool, same facets at seat rim; mass again floating mid-air; only 2 of 4 foot supports visible as bigger caps. |
| 200 | B | B | C | Legs are slender yellow rods clearly thinner than the drawn legs (overlay red halo around every leg, 4 red tabs in plan); structure mass is a magenta box floating in the void between legs, outside the body. |
| 500 | C | B | C | Legs wire-thin (well under 75 mm feature, drawn legs 3x wider: red everywhere), backrest a plain block; thermal foot-ring shell offset/gapped against the body ring; structure has a large mass cube in mid-air and a mass plate on the seat. |
Instructions:
- [50,100,200,500 structure, mass-*] put the seat mass at the seat (centre of mass of seat+backrest, inside the seat pad), not at foot-ring height in the void; at 200/500 drop the cube mass in mid-air.
- [50,100 body seat pad solid] raise loft sections (>=24 segments) or use revolve for the round pad so the rim is not stair-stepped.
- [200,500 body legs] model legs at the drawn width (>= feature 30/75 mm) as tapered round/square sections, not slender rods; at 500 the leg set may be four thick tapered posts.
- [500 thermal footrest ring] make the thermal ring the same solid (of) as the body ring; it is offset/cut apart.
- [500 body backrest] give the backrest the curved plate form of finer scales (thin curved wedge) instead of a block.
Disagrees with green validator: yes at 200/500 (legs thinner than drawn, floating masses).

## appliances/circulation-pump (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | B | B | Wet-rotor pump reads right (volute drum, motor drum with window, terminal box with lid, two union stubs with threaded ends). Casing and motor are 12-16 gon prisms, visibly faceted; volute is a plain cylinder, not a spiral; thermal th-motor-plume loft leaves dark curved sliver fins above the motor; terminal box (non-bearing cover) is a blue member. |
| 100 | B | B | B | Terminal box now a plain block, stubs plain cylinders, same facets; structure pipe prism passes through the casing and out of both stubs, plausible. |
| 200 | B | B | B | Body 4 solids, motor shows stair-step shading steps along its top (overlapping revolve bodies), terminal box a cube; thermal still the 7-solid 100 set (same plume fins) rather than a coarser abstraction. |
| 500 | - | - | - | unavailable |
Instructions:
- [50 thermal th-motor-plume] reshape the loft: the sliver fins above the motor are open curved surfaces, make it a closed ellipsoid/capsule plume or remove.
- [50,100,200 body casing/motor-head/body] raise segment counts to >= 24 for the revolves (visible 12-gon); 200: remove the stair step between `body` and `pipe` (make one smooth profile).
- [50 structure member-terminal] terminal box carries no load: replace by a small post/joint or drop to a joint.
- [200 thermal] author a genuinely coarser thermal set (merge th-pipe-l/r into one shell) instead of the 100 set.
Disagrees with green validator: no (it is Ready and mostly fine; faceting is not measured).

## appliances/decentralised-hrv-fan (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | B | B | Reads as a wall sleeve fan: square rounded cover plate, tube, lip ring, ceramic core disc inside; the room-side face is a blank slab (no slot/grille/louvre) and no impeller visible; thermal hub/blades are not visible anywhere (swallowed by core/casing, cannot be verified); structure: tube and plate as members, orange joint ring, mass ellipsoid inside tube, no visible support at the screw points. |
| 100 | B | B | B | Same object, impeller/blades reduced to specks inside the tube, front plate still blank; structure mass almost invisible. |
| 200 | B | B | B | Clean tube+plate; plan/front overlay shows 3D plate offset/thicker than drawn (blue strips at plate edge); no impeller or core readable. |
Instructions:
- [50,100 body cover plate] add the real cover detail: recessed slot/louvre grille or ring bezel on the room-side face (new thin solids), the rear screw heads exist but the front is bare.
- [50,100 thermal hub, blades] the impeller sits inside the core and is hidden: either move it into its own visible axial gap in the tube or state in decisions; a cutaway shows nothing of it.
- [200 body flange] plate thickness in 3D exceeds the drawn plate (blue strips front/plan): thin the plate solid to the drawn thickness.
- [50 structure] add support solids at the four screw fixings of the cover plate (the supports are not visible in any view).
Disagrees with green validator: no.

## appliances/electric-fan-heater (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | A | B | C | Convincing fan heater: rounded casing, top carry handle with recess, louvred grille with angled slats, side control knob and slot, rear intake, four feet. Thermal: hollow casing shell, grille with fins, intake plate, knob; no heating element or fan wheel behind the grille (the emitter is only the grille plate). Structure: one solid rounded block filling the whole casing volume plus four pads. |
| 100 | B | B | C | Louvres reduced to a flat dark panel (fine), handle simplified; same fill-block structure; overlay shows drawn handle ridge/corners missing in 3D (red in elevation). |
| 200 | B | B | C | Still the same 9-solid thermal set as 100 (hollow shell); bottom rail/feet drawn but missing in 3D (red bar along the base); structure again a solid block. |
Instructions:
- [50,100,200 structure member-*, the full-body block] replace the solid block by the real load path: base frame rails/bottom plate, four corner posts or the folded sheet casing as a thin shell member, a top plate under the handle, handle as a member with two joints; feet as supports (they exist). A solid fill of the casing is not a structure.
- [50 thermal] model the hidden heater element and fan wheel behind the grille (hidden region solids) so the cutaway shows where heat is made; the emitter is currently only the grille plate.
- [50 structure joint at knob] the orange joint ring on the side wall sits at the control knob; joints belong at member connections (corner/handle), move or delete.
- [200 body] add the feet/base rail the drawing shows (red strip in all three overlays) as 2 low boxes.
- [200 thermal] merge shells/abstract (it is the 100 set unchanged).
Disagrees with green validator: yes for structure (a green check cannot see that it is a filled block).

## appliances/hot-water-cylinder (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | B | C | Tall insulated cylinder with domed top, three feet, small side stubs and a label plate: reads right; 24-gon facets visible as vertical flutes, no visible jacket/seam or top/bottom insulation cap; thermal: four stratified water zones plus top cap, zones fill the whole radius so no insulation/steel shell layer is visible in the cutaway; structure: tank as four stacked solid cylinders with orange joint rings (a filled block, not a vessel shell) and a single support pad although the body has three feet. |
| 100 | B | B | C | Same; feet merged into a base plate, ports reduced to two studs; structure still solid cylinders + one pad. |
| 200 | B | B | C | Simplified cylinder; thermal base shows a grey ghost slab under zone 4 (zone 4 floats above the base); one support pad only. |
Instructions:
- [50,100,200 structure member-*] model the tank as a thin steel shell (revolve loop, real wall thickness) with domed end caps instead of four solid cylinders; keep the joint rings only at real weld seams.
- [50,100,200 structure supports] one pad per foot (three), under the feet; currently one pad.
- [50 thermal] add the jacket insulation and steel wall as nested layer shells around the zones (thickness per asset layer data) so the cutaway shows layers; zones should be inside them.
- [50 body] jacket: add a seam/edge line or a top insulation cap step; use >= 48 segments for the cylinder to remove the fluting.
- [200 thermal zone-4/base] extend the lowest zone/insulation to the base or add the base shell; it currently ends above the base ghost.
Disagrees with green validator: yes for structure (solid fill).

## appliances/mvhr-unit (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | C | C | Wall-hung box with four duct collars on top, round fan window, a slot and a LED on the front. Reads as a ventilation box but is a blank slab: no door seam/handle, no filter cover, no display. Thermal: hollow thick-walled casing with one impeller in the porthole and the four collars; the heat-recovery core, filters, supply and extract fans and bypass of an MVHR are absent, so the cutaway shows an empty box. Structure: member-shell/back read as one solid block; wall hooks not visible in any view. |
| 100 | B | C | C | Same, impeller reduced to three blades, window dark disc; collars plain cylinders. |
| 200 | B | C | C | Thermal identical to 100 (8 solids) instead of a coarser abstraction; red slivers at collar tops in front overlay (drawn collars taller/ wider than 3D). |
Instructions:
- [50 thermal, new hidden solids] add the counter-flow heat exchanger core (hexagonal/diamond block, large area, inside casing), two filter slabs and two fan scrolls as hidden thermal solids in the existing regions, or name the region they belong to; connect them to the four collars with short duct stubs.
- [50,100,200 structure member-shell, member-back] make the casing a thin sheet shell (loop revolve/extrude with a real wall) plus a rear mounting rail member; the wall hooks (support-wall-*) must be visible at the rear edge.
- [50,100,200 body casing] add the front cover seam and handle; at 200 raise collar solids to the drawn height (red tops in front overlay).
- [200 thermal] merge the two duct shells and drop the impeller blades; it is a copy of the 100 set.
Disagrees with green validator: yes (thermal content missing, structure filled).

## appliances/split-ac-indoor-unit (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | C | C | Reads as a wall split unit: rounded front, top intake grille, display tile, bottom discharge slot with vane. Front is a smooth extrusion (no front panel joint, no filter frame, grille is one flat plate, vane is a flat strip); rear is a flat blank slab. Thermal: only 4 solids, a hollow casing with a thick hatched cut, grille and vane; the evaporator coil and cross-flow fan (the actual heat exchange) are missing. Structure: one solid block with an ellipsoid mass; a thin vane shaft rod below it. |
| 100 | B | C | C | Same, thermal and structure identical in kind to 50. |
| 200 | B | C | C | Thermal set identical to 100; overlay shows the drawn lower outlet bulge missing in 3D (red blob at the bottom front of the elevation); vane shaft sticks out under the body. |
Instructions:
- [50,100,200 thermal, hidden] add a slanted evaporator coil block (A-shape of two slabs) behind the front grille and a cross-flow fan cylinder above the discharge slot as hidden solids in the existing regions (or add hidden region in schema if allowed); casing thickness should be sheet-thin, not the thick wall visible in the cutaway.
- [50,100,200 structure member-*] replace the solid block with a rear mounting plate member + two end plates + the fan shaft member and a front panel frame; supports at the two rear hooks (support-* currently one small green pad on top).
- [50 body] add the filter frame, panel seam lines and make the vane a curved wing; [200] add the lower outlet bulge drawn in the elevation.
Disagrees with green validator: yes (missing interior, solid-block structure).

## appliances/thermostatic-radiator-valve (Ready; authored at 50 and 100 only)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | C | C | Head cylinder with knob on a stepped valve body: reads as a TRV head, but all round parts are visibly 12-gons, the valve body is a plain stack of cylinders (no hex nut, union nuts, spindle bonnet) and the plan overlay shows the drawn side tabs (inlet/outlet connections) red = missing in 3D. Thermal: head and knob shells only; the thermostatic bellows/sensor inside the head is missing; valve body appears only as one red cylinder. Structure: rod member in the head, mass blob in the valve, and two large green support boxes stacked under the spigot, wider than the valve body and not attached to a pipe. |
| 100 | B | C | C | 2 body solids; overlay: 3D knob protrudes left beyond the drawing (blue strip in front) and 3D has blue corners in plan while the drawn narrow spigot is red on both sides in elevation; thermal head shows a hatched cut disc but no bellows. Structure unchanged: supports boxes dangling below the body. |
Instructions:
- [50 body] add the drawn side connections (union nuts/inlet) that the plan overlay shows red; add a hex nut and spindle ring at the head-to-valve joint; use >= 24 segments for head and knob.
- [100 body] pull the knob solid in to the drawn extent (3D knob sticks out left) and widen the lower spigot to the drawn width (red edges).
- [50,100 thermal] add the hidden thermostatic bellows/sensor (small capsule) inside the head shell and show the valve body wall as a shell with the water inside.
- [50,100 structure support-*] supports must sit at the pipe connection: two short pipe stubs entering the valve body side or the single bottom spigot as a pad that is no wider than the valve body; delete the two stacked boxes.
Disagrees with green validator: partly (overlay deviations at 100 are within limits but visible).

## furniture/bookshelf (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | A | A | B | Convincing open bookcase: two side panels, top, five shelves, back panel, plinth; thermal carcass/shelves/back/plinth nested correctly and legible; structure shows the same boards as members. Supports (one green wedge in the corner) are not visible at 50/100 and the single mass block floats on one shelf. |
| 100 | A | A | B | Identical in look to 50 (16 solids vs 26: shelf pins gone) - fine. |
| 200 | B | B | B | Visually the 100 model again (16/16/24 solids); not a new abstraction; 3D back panel shows a different tone only. |
| 500 | B | B | B | Same 16-solid model with fattened shelves (red ticks at shelf ends in elevation/front overlay: drawn shelf edges stick out beyond the sides, 3D does not); same 24 structure solids; support is one green wedge at the lower right corner only. |
Instructions:
- [200,500 body/thermal/structure] these scales must be new abstractions: at 200 merge the five shelves + back into one slab-with-grooves or drop shelf-end detail; at 500 reduce to carcass box + 5 shelf lines (about 8 solids) and say so in decisions.
- [500 body shelves] extend shelf ends as drawn (red ticks) or shorten the drawn lines; today the overlays disagree at every shelf.
- [50-500 structure support-*] one foot support per plinth corner (four) or a plinth-wide strip, visible below the plinth; currently one wedge at one corner (200/500) and none visible (50/100).
- [50-500 structure mass-*] place books mass inside the carcass volume at mid-height centre (currently a small block sitting on one shelf edge).
Disagrees with green validator: no.

## furniture/cafe-chair (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | B | B | B | Reads as a bistro chair: seat slab with apron, two back rails, rear posts with caps, four splayed legs with stretchers and feet. Legs/stretchers are very thin wires; seat plate perfectly flat (no dishing/edge radius); thermal seat/back/tube shells fine; structure: small magenta mass blocks sit on two legs (not at the centre of mass of the chair). |
| 100 | B | B | B | Same; legs and rails drawn as 'slender' rods thinner than the drawn legs (red halo in all overlays). |
| 200 | C | B | B | Legs/stretchers are thin yellow rods while the drawing carries legs at least 30 mm wide (red halo around every leg, stretcher and post); back reduced to one slab; mass blocks on legs plus a seat plate. |
| 500 | C | B | C | Legs are wire-thin vs 75 mm drawn width (large red halos), back a thick block, seat a heavy box; structure masses are cubes dangling on two legs and a plate on the seat, support pucks larger than the legs. |
Instructions:
- [200,500 body legs, stretchers, posts] give legs the drawn width (>= 30/75 mm as tapered square or round sections) instead of slender rods; at 500 legs and posts must be visible posts, stretchers merged into one low rail frame.
- [50-500 structure mass-*] put one mass at the chair's centre of mass (seat centre, 0.3 m above floor) inside the seat volume and one for the back; remove masses sitting on legs.
- [500 structure support-*] support pads no wider than the leg foot.
- [50,100 body seat] add the seat edge radius/dish (a loft with rounded corners) so it is not a flat slab.
Disagrees with green validator: yes at 200/500 (halo = under-thick legs accepted via slender).

## furniture/folding-deck-chair (Ready)
| scale | body | thermal | structure | what I saw |
|---|---|---|---|---|
| 50 | A | A | A | Very good: square-section timber frame (two A-frames, back frame, seat rail), canvas sling draped as curved seat and back, joints and masses on the members, supports at the four feet. Reads as a deck chair from every side; only the armrest/cross-rail pegs of a real chair are simplified. |
| 100 | B | A | B | Frame sections turned from square to round dowels (not the same object detailing as 50); plan overlay: 3D canvas sticks out past the drawn left edge (blue strip). |
| 200 | B | B | B | Back and seat canvas become flat slabs (the seat a thick plate), frame plain bars; canvas protrudes past the drawn edge (blue strip in plan and front bottom). |
| 500 | B | B | C | Canvas a lumpy block, frame bars; structure joints and masses become large cubes at every node and foot (orange/magenta/green blocks bigger than the legs). |
Instructions:
- [100 body frame] keep the square timber section of 50 (or state why dowels), the drawn legs are rectangular.
- [100-500 body canvas] trim the canvas to the drawn width (blue strips at the left of the plan, bottom of the front); at 200/500 keep a curved/folded sheet instead of a thick plate/block.
- [500 structure joint-*, mass-*, support-*] shrink node cubes to the member section (about 1.3x); masses should sit on the canvas/seat, not on the leg ends.
Disagrees with green validator: no.


---

# Part B: remaining Ready-list assets (reviewer part 7)

# QA part 7 (visual review, strict)

Rendered with qa-render.ts, all authored scales, every sheet and views image opened. etics-brick-wall-30.json was last edited 22:35 (about 14 min before review); it was re-rendered at 22:49 and judged on the fresh render. cooking-pot has no 1:500 volumes (volumes.500 is empty): not graded, listed as missing.

Note on the structure renders: masses (magenta) and supports (green) are very often invisible (buried inside the opaque member shell or thinner than the plate they sit under). Every "mass not visible" below is a visual finding even where the JSON contains the solid.

## components/etics-brick-wall-12
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | 1 m panel, 28 solids but the outside is one flat slab: bed joints (10 mm) are coplanar so courses, render texture, dowels are invisible; thermal layers nest correctly but all four are near-identical reds; structure = 13 courses stacked into a filled slab, 7.5 mm base support not visible |
| 100 | B | B | B | plain 3-layer block, same look as 1:50 minus the courses; structure one solid block, support slab invisible |
| 200 | B | B | B | near copy of 1:100 (insulation+render merged); structure identical to 1:100 |
| 500 | B | C | B | single grey block; thermal is ONE solid although the 120 mm insulation is larger than the 75 mm feature size, so the layer that defines this assembly is gone |
Instructions:
- [50, body, brick-course-00..12 / bed-joint-01..12 / render] make bed joints recessed (joint solids 5 mm behind the brick face) or give the render a visible reveal at the edge, otherwise the body is a smooth box; add the thin mesh/dowel heads on the render face or state they are omitted by design.
- [50..200, thermal, th-*] give each layer a clearly different hue (plaster, brick, insulation, render) so the cutaway shows the nesting.
- [100/200/500, structure, member-brick-leaf, support-base-fixed] raise the support slab so it is visible (it is flush with the wall foot, no green anywhere); at 100/200 show 2 solids max but keep a visible green pad of 15/30/75 mm.
- [500, thermal, only solid] split into brick core + 120 mm insulation + render (3 layers) or at least core + insulation.
- [200, body/thermal] differentiate from 1:100 (currently the same two-three blocks); merge render into insulation explicitly and drop the 20 mm render below the 30 mm feature.
Disagrees with green validator: yes, thermal 1:500 (insulation layer dropped although larger than the smallest feature).

## components/etics-brick-wall-30
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | same as -12 with 300 mm insulation; the thick EPS reads as a pale block behind the brick leaf, no visible layer difference on the face; structure is only the brick leaf (blue) with the insulation as ghost, correct load path but support invisible |
| 100 | B | B | B | 3 plain blocks, insulation 0.30 plausible |
| 200 | B | B | B | 2 blocks (brick+insulation), render merged; fine abstraction but same look as 100 |
| 500 | B | C | B | one 0.7 m block; thermal single solid: 300 mm insulation (4 times the feature size) lost, the cutaway cannot show it |
Instructions:
- [50, body, insulation] add a visible change of material/tone at the brick/insulation boundary on the side face; courses invisible as in -12 (see there).
- [500, thermal, wall] split into brick leaf + 300 mm insulation (+ render merged), colour them differently.
- [all, structure, support-base-fixed] make the support pad visible (see -12).
- [200, thermal] keep, but make the render a separate thin skin only if > 30 mm, otherwise merge as done.
Disagrees with green validator: yes, thermal 1:500 (same reason as -12).

## components/external-venetian-blind
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | lintel/parapet stubs, glazing, headbox, 16 slats read well; guide rails exist in thermal and structure but are missing in the body; wall stubs and glazing show no visible nested layers in the cutaway |
| 100 | B | B | B | body now has yellow (slender) rails, good; same stub/glazing issue; structure identical to 1:50 (23 solids, same ids) |
| 200 | B | B | B | body and thermal are visually the same as 1:100 (24 lamellae kept although a 30 mm feature cannot show 5 mm slats); structure reduced to 6, good |
| 500 | B | C | B | body is a real new abstraction (5 fat slats between rails); thermal still has 24 lamellae (dense hatch noise, contradicts body); overlay red blocks beside the slat column = drawn slat depth larger than the 3D slats |
Instructions:
- [50, body] add the two guide rails (use the box-and-rails solids of the thermal list) and the bottom rail end caps; give the lamellae the slight tilt that the thermal list has.
- [50/100, thermal, wall stubs facade-wall/*, glazing-behind/*] split the stubs into visible block/EPS/render/plaster shells and the glazing into pane-in/cavity/pane-out with distinct hues; today the section reads as one colour.
- [200, body+thermal] build a 1:200 abstraction: 10-12 slat bands at 60-100 mm pitch, rails at >= 30 mm, not the 1:100 list.
- [500, thermal, lamellae 24 solids] replace by the same 5 slat bands as the body (one region "lamellae", 75 mm thick bands), keep box-and-rails as one solid.
- [500, body, slats] extend slat depth to the drawn depth (red blocks in elevation overlay).
- [100/200, body, glazing] blue hairline in the elevation overlay = 3D-only pane edge; align pane thickness with the drawing.
- [structure, all] lamellae are only a ghost; add a bottom-rail member + 2 hanging-cord members so the suspended load path (headbox to rails to bottom rail) reads; add a joint headbox-lintel.
Disagrees with green validator: yes, thermal 1:500 and body/thermal 1:200 are copies of the finer scale.

## components/solid-brick-wall-1960
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | 1 m panel, plaster/brick/render; same invisible-courses issue as etics-12 |
| 100 | B | B | B | two blocks, smooth |
| 200 | B | B | B | body same ids and same look as 1:100 (a copy), thermal differs only by merging |
| 500 | A | A | B | one honest block, thermal single material is correct for homogeneous brick; structure support invisible |
Instructions:
- [50, body] as etics-12: recess bed joints or show courses.
- [200, body] differentiate from 1:100 (merge plaster and render into one 30 mm skin, 1:100 keeps two).
- [100..500, structure, support-base-fixed] make the support pad visible.
Disagrees with green validator: no (data matches what is seen); note copy 100 to 200.

## components/stair-flight
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | steel mono-stringer with 12 floating-looking plate treads (grip holes, nosing, fine); stringer reads as a thin ribbon/flat sheet below the treads, no end plates, no tread brackets, joints (orange) tiny |
| 100 | B | B | B | same ids/look as 1:50 with details removed, i.e. a stripped copy, not a new abstraction |
| 200 | B | B | B | stepped slab look with thick treads, stringer a flat bar; ok abstraction, overlay red at tread ends in plan = drawn tread edge sticks out |
| 500 | B | B | B | same ids and the same look as 1:200 (copy); top landing nosing red in elevation (drawn only) |
Instructions:
- [50, body, stringer-steel solids] give the stringer a real C/I section (web + flanges) with end plates; add tread brackets/welds where the treads meet it (joints orange are < 10 mm and cannot be seen).
- [100, body] merge treads into one row of 6-8 solids or use fewer details, so it is a distinct abstraction from 1:50.
- [500, all lists] re-author: treads fused into one sawtooth solid (about 75 mm steps), stringer 75 mm, not the 1:200 solids.
- [50/100 structure, supports] green supports at both ends are small blobs; make a visible bottom base plate and top landing bearing, and show the mass sphere inside the stringer/tread at mid-flight (it sits on the surface, partly outside).
Disagrees with green validator: no, but 1:500 is a copy of 1:200 (identical ids).

## components/steel-column
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | A | B | I-section column with base plate, 4 anchor bolts, cap plate; plan overlay red corners = drawn plate chamfers missing in 3D; structure support not visible (hidden below the orange base plate), bracket joint with mass at mid height plausible |
| 100 | B | B | B | same, anchor bolts reduced to one visible; thermal merged parts |
| 200 | B | B | B | boxy I section with grooves, caps; structure 3 solids, cap ghosted, base plate green: clean |
| 500 | B | B | B | elevation overlay blue ticks at cap and base = 3D plates wider than drawn; thermal shows web as two hatched strips |
Instructions:
- [50/100, body+structure, base-plate, cap-plate] chamfer the plate corners to match the drawing (red triangles in the plan overlay).
- [100, body] bring back all 4 anchor bolts (feature 15 mm is fine for a 20 mm bolt) or state a rule.
- [50/100, structure, support] add a green support under the base plate (a visible 10-20 mm pad or the bolts).
- [500, body, cap/base plates] narrow the plates to the drawn width.
- [500, thermal, web region] draw the web as one thin solid inside the flanges, not two stripes.
Disagrees with green validator: no.

## objects/charcoal-grill
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | A | B | kettle bowl, lid, vent, handles, ash catcher, 3 legs, ring brace, 2 wheels: reads perfectly as a kettle grill; but the bowl/lid show visible vertical gore facets and the handles are boxy blocks; thermal cutaway has hatched grate, charcoal bed, ash catcher; structure legs/brace fine, no joints at the leg-bowl brackets, pink ring-mass sits partly outside the bowl |
| 100 | B | A | B | same, wheels in structure become fat orange discs; wheel brace dropped; supports small |
| 200 | B | B | B | legs only (4 poles, no cross brace, no wheels) = fair abstraction; grate drawn as thin green line; faceting still visible |
| 500 | C | B | C | overlay: legs have a thick red halo = drawn legs about 75 mm wide, 3D legs far thinner (36 slender solids); structure support cubes are 75 mm blocks bigger than the legs they carry |
Instructions:
- [50..500, body, bowl-shell / lid-shell revolve] raise the segment count (at least 48 around, rounder rim profile) so facets disappear; replace the handle boxes with tapered lofts.
- [500, body, legs 36 slender solids] thicken the legs to the 75 mm drawn width (use one tapered leg solid each), then drop the "slender" flag.
- [500, structure, legs, support-*] same leg width; make the support pads about 1.3 x leg width, not 3 x.
- [50..200, structure] add leg-to-bowl bracket joints (orange) at the 3 leg tops, and a hinge/handle joint on the lid.
- [50, structure, mass ring] move/resize the magenta mass to the charcoal bed inside the bowl (centre of mass), it now reads as a ring around the ash catcher.
- [200, body] add the lower brace ring as one 30 mm hoop to avoid the 4 free-standing sticks.
Disagrees with green validator: yes, 1:500 legs/supports (halo visible in overlay).

## objects/cooking-pot
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | C | pot with rolled rim, two torpedo-shaped handles, rivets, water with bubbles; the 16-sided wall is clearly faceted; thermal wall/water/base shells nest, steam omitted visually; structure: only one opaque blue shell, no visible mass, no visible support at the base |
| 100 | B | C | C | handles become pointed cones; thermal base layer is only a ring: the water (yellow) is exposed on the underside; structure no mass/support visible |
| 200 | C | C | C | handles are tall flat boxes standing ABOVE the rim (overlay: red band along the pot top = drawn rim higher than the 3D wall; blue block on top = 3D handle); faceted wall; thermal base missing again (yellow disc on the underside); structure no mass/support visible |
| 500 | n/a | n/a | n/a | no volumes authored for 1:500 |
Instructions:
- [50..200, body, wall / base revolve] use at least 48 segments; the 16-gon is the first thing one sees.
- [50, body, handle-left/right] replace the bullet loft by a flat strap handle (rectangular section, tapered) mounted under the rim.
- [200, body, handle-left/right boxes size 0.043x0.04x0.024] lower them to 5-10 mm below the rim and flatten (about 15 mm high), they stick up like ears; raise wall top to the drawn rim line (red band).
- [100/200, thermal, th-base, th-water] the water solid starts at y=0 and pokes through the base ring; start water above the base plate and make th-base a full disc that closes the underside.
- [50..200, structure, mass-pot, mass-water, support-base-contact] the masses are inside the opaque member shell and the support sits at y=0.075 (near the rim) so none of them is visible; show members as ghost or cut-open and move the support to the base ring at y=0.
- [500] author a 1:500 pot (cylinder + two 75 mm handle stubs) or declare it out of scope.
Disagrees with green validator: yes, the thermal base at 1:100 and 1:200 and the support/mass placement look wrong although the lists are valid.

## objects/patio-heater
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | square base housing on plinth, pole, domed reflector with emitter head, control knob and hose connector: reads right; housing is a plain box without vent louvres; thermal housing is an EMPTY shell: the gas bottle and burner pipe are missing; structure has a cylinder (bottle) in the ghost housing |
| 100 | B | B | B | same without the connector |
| 200 | B | B | B | faceted dome (visible rings), box + pole; structure pole noticeably thinner than the body pole |
| 500 | B | B | C | very faceted dome; thermal bottle still missing; structure pole is a thin line next to a fat 75 mm body pole |
Instructions:
- [50..500, thermal] add a region for the gas bottle inside th-base-housing (hidden, cooler) and a thin gas pipe inside the pole; the cutaway currently looks like a hollow can.
- [200/500, structure, pole-tube] use the same section as the body pole (>= 30 mm at 1:200, 75 mm at 1:500).
- [50, body, base housing] add 2 rows of vent slots (8 mm) and the door panel line.
- [200/500, body, reflector dome] raise segment count; the dome facets are the most visible faceting in this asset.
- [50..500, structure, mass-base, mass-bottle] the two magenta masses are inside the opaque bottle shell and never visible; show the bottle member as ghost.
Disagrees with green validator: no (numbers fine), but hidden-internal omission is not caught.

## objects/storage-crate-stack
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | A | B | five nested tapered crates with rim lips and handle/vent slots; slots are flat dark patches, no feet/underside ribs, bottom is plain; thermal cutaway = five coloured hollow shells with hatched sections, good; structure = shells, only the bottom pad as support |
| 100 | B | B | B | same without slots |
| 200 | B | B | B | tapered shells fine, but plan overlay shows blue 3D-only slivers left/right and red along top/bottom: 3D footprint is about 30 mm wider and shorter than drawn; structure masses show only as magenta slivers at the rims |
| 500 | B | B | C | body is one solid tapered-less prism (fine at 75 mm); thermal five closed slabs, no cavity; structure shows an open top crate with thick rim whereas body is a closed block |
Instructions:
- [200, body, crate-* ] widen footprint by about 30 mm in Z and narrow in X to the drawing (plan overlay).
- [50, body] model the slots as real recesses and add feet/ribs under the lowest crate.
- [500, structure, member-shell-1..5] either close the top (match the body) or open the body top; keep one rule for both lists.
- [200/500, thermal] keep a visible cavity (empty air region) in at least the top crate; now it reads as five solid blocks.
- [50..500, structure, mass-1..5] masses are at y=0.15 inside each crate but hardly visible; show them as ghost-visible blobs sized to the contents.
Disagrees with green validator: no.

## Summary of weaknesses seen across part 7
- Walls (etics-12/-30, solid-brick-1960): block stand-ins at all scales, courses/joints invisible, structure = filled slab, support slab invisible; thermal 1:500 drops the insulation layer (etics).
- Copy of finer scale: venetian blind 1:200 body+thermal = 1:100, blind thermal 1:500 keeps 24 lamellae, stair 1:500 = 1:200, solid-brick 1:200 body = 1:100, etics 1:200 = 1:100.
- Faceting of round parts: cooking pot (16-gon), charcoal-grill bowl/lid, patio-heater dome, all scales.
- Thin legs/rods: charcoal-grill 1:500 (red halo), patio-heater structure pole at 1:200/1:500.
- Thermal hidden internals: patio-heater (no bottle), cooking-pot base (water exposed).
- Masses/supports invisible inside opaque member shells (pot, patio-heater, crates, columns, walls).


---

# Part C1: other assets (reviewer part 1)

# Visual QA part 1 (appliances)

All eight assets have scales 50, 100, 200 (no 500). All images (sheet and views) of every scale were opened. ceiling-diffuser.json was touched 12 minutes before the first pass, so it was re-rendered and reviewed last (>15 min old by then).

## appliances/ceiling-diffuser
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Square flange with 3 stepped cones below and a collar above reads right; cones are plain faceted steps, jets sit as two stubs at the flange rim, structure is one flat slab |
| 100 | B | B | B | Cones merged to a shallow stepped tray on the underside, collar ring plain; same rim stubs for jets |
| 200 | B | B | B | Underside is a shallow tray, flange visibly thicker than drawn (red band in elevation); mass is a square block lying on the slab |
Instructions:
- [50/100/200, thermal, jet-left jet-right] The jets sit as small cylinders at the outer flange edge under the plate. A 4-way diffuser throws air out of the slot between the cone steps and along the ceiling. Move the emitters to the slot lips (between cone-1/cone-2/cone-3 at 50, cone-1/cone-2 at 100, at the cone lower rim at 200) and give them a throw direction along the ceiling.
- [50, structure, member-body] A single 595 mm slab is carrying everything. Add the cone assembly as members (cone-1..3 sheet shells with a thin section, joined to flange by joints) and a collar-to-flange joint; hangers (support-ceiling-hanger-*) are 20 mm blobs on one edge, make both visible and give them a real section.
- [200, body, flange cone] Elevation overlay shows red along the flange underside: the 3D flange is thinner than the drawn one. Thicken flange to the drawn thickness or correct the 2D.
- [200, structure, mass-body] The mass is a flat square block, 40 percent of the plate wide, on the top face. Make it an ellipsoid/cylinder inside the plate+cone envelope under the collar (centre of mass of plate+cones), not a block lying on the plate.
- [50/100, structure, mass-body] Disc lies on the plate top at the collar, acceptable, but sink it into the plate/cone envelope so it is inside the body.
- [50, thermal, th-collar] Collar is a thin green half-ring only; add the supply air plenum volume inside the collar (hidden region) so the cutaway shows where the air enters.
Disagrees with green validator: n/a (no validator data given).

## appliances/desktop-monitor
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Reads as a monitor on a flat foot with a neck and a rear hump; the hump is stepped/faceted, foot is a plain plate, hinge not visible |
| 100 | B | B | B | Same, hump as smooth lens, neck is a simple tapered post |
| 200 | C | B | B | Panel turned into a large-radius squircle blob (red corners in front overlay, 3D smaller than drawn), foot warped into a curved wedge |
Instructions:
- [50, body, housing] Hump shows visible faceting in rings (stepped). Smooth the profile or add segments; hump outline in the az215 view is an odd blob, give it the real rear-cover shape with a flatter outline and a vent grille step.
- [50, body, hinge] hinge exists in the list but cannot be seen from any direction; expose the hinge at the neck/panel joint (small cylinder on the neck top) or drop it.
- [200, body, panel] Panel is rounded to a squircle with radii of about 60 mm: the drawn rectangle corners (red in front overlay) are missing. Use a rectangle with at most the smallest-feature corner radius (30 mm) so the silhouette matches the 2D.
- [200, body, stand-base] Base is a bowed, tapered wedge (curved sides in az125/az215). Replace with a flat slab with a slight chamfer, as drawn (blue slivers in elevation/front show 3D-only wedge).
- [200, thermal, th-front-slab th-hump] Front slab is a plain full box; hump is a shell with a rectangular green sheet. Make the hump the same outline as body and keep the nested panel/air layer.
- [50/100/200, thermal, th-housing] Housing is one green shell; add the power supply as a hidden warm region inside the hump (the catalog says the rear housing with PSU is warmest).
- [50/100, structure, member-panel] Panel is one solid slab: model as frame (perimeter members) plus rear plate; the 3D mass ellipsoid (mass-panel) should sit inside the hump and not look like a plate-sized disc on the back (50, az215).
- [200, structure, support-base-contact] Green support is a wedge-shaped patch on a warped base; flatten with the base.
Disagrees with green validator: n/a.

## appliances/duct-fan
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | A | B | Wall plate with 4 screws, round duct ring, struts, hub and 5 blades read correctly; cylinder wall shows ring facets, blades look pale |
| 100 | B | B | B | Struts/blades kept, screws merged; same facet banding |
| 200 | B | B | B | Three blades and a hub, plate; supports are oversized stubs that poke outside the plate edge |
Instructions:
- [50/100/200, body, ring plate] Cylinder bands are visible in all views (about 24 segments, azimuthal shading steps). Raise segment count so shading is smooth.
- [50, body, blade-1..5] Blades render almost transparent against the ring; give them the same tone as hub and thicker leading edges (7.5 mm min).
- [100/200, thermal, th-airstream] Airstream is not visible in the cutaway at 100 and 200 (present at 50 as sheet); ensure it is a visible emitter volume in front of the blades.
- [200, structure, support-wall-screw-top support-wall-screw-bottom] Plan/front overlay shows blue 3D-only blocks at the plate edges: the supports are about 22 mm chunks protruding behind and beside the plate. Put them inside the plate area (at the 4 screw positions or the top/bottom pair at the drawn positions) and cut them to the plate thickness.
- [50/100/200, structure, member-rotor mass-plate] Rotor member is a thin bar across the hub, no motor body; add a motor cylinder in the hub as member+mass (mass-plate is tiny at 200, move the mass to the motor).
Disagrees with green validator: n/a.

## appliances/electric-kettle
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | C | B | Reads as a jug/percolator on a base, handle ok, spout a small cone that looks stuck on; 16-sided body is visibly faceted |
| 100 | B | C | B | Same, lid knob and gauge removed |
| 200 | B | C | B | Same, chunkier facets, handle a boxy block |
Instructions:
- [50/100/200, body, shell spout] Spout is a short cone attached at the rim; make it a lipped spout flowing out of the shell wall (taper from wall at mid-height to the rim). Elevation overlay shows blue sliver at the spout.
- [50/100/200, body, shell base] Facets (about 16 sides) are very visible in all axonometrics. Increase the segment count on shell, base, lid.
- [50, body, gauge] Gauge is a flat pink rectangle that reads as a decal; make it a recessed window with a frame or remove.
- [50/100/200, thermal, th-water th-shell] The cutaway shows the water as radial fan wedges (cap triangles) instead of a nested volume inside the shell; the shell is a single wall. Model water as a closed cylinder (lower half cut), shell wall with real thickness around it, and an air gap below the lid.
- [50/100/200, thermal, th-steam th-steam-plume] Steam is two thin floating rings at the spout tip. Make a steam plume volume that starts at the spout opening (emitter) and rises, not hoops.
- [50, thermal, th-lid th-lid-button] Lid is a half disc only; add the lid underside/air layer.
- [50/100/200, structure, member-body] Spout has no structural member (shown as ghost white cone). Add the spout as a member joined to the shell; water mass/member is hidden inside an opaque shell so the load path cannot be read: show shell as a wall (hollow) so the water mass is visible.
Disagrees with green validator: n/a.

## appliances/electricity-meter
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Boxy meter with back plate, terminal block, round window with disc, counter slot; reads as a meter but cover is a plain box |
| 100 | B | B | B | Same with sharper edges, window ring and counter as plates |
| 200 | B | B | B | A plain box with a terminal block, a ring and a small block; readable but crude |
Instructions:
- [50/100/200, body, body] Cover is a sharp box with a front step; a real meter has a rounded dome cover over the window. Round the cover edges (a 15 to 30 mm radius per scale) and curve the window area.
- [100, body, terminals] Blue strip in plan: 3D terminal block is deeper than the drawn one; match the drawn depth.
- [200, body, boss terminals] Plan overlay has blue blocks at both corners: 3D has terminal corners that the 2D does not draw; trim.
- [200, thermal, th-plate] Plate (yellow) sits in the terminal zone at the bottom, at 50/100 it sits above the terminals. Move it back.
- [50, thermal, th-disc th-window-glass] In the cutaway only the window ring is green and the disc is missing; make the disc a visible hidden region behind the glass.
- [50/100/200, structure, member-body mass-body] Body is a solid opaque block, so masses/joint-disc cannot be judged; the joint-disc shows as an orange dot on the outside of the case. Make the cover a hollow member and place the disc joint inside behind the window; show mass-body inside.
- [50/100/200, structure, support-wall-screw-top support-wall-screw-bottom] Supports are tiny dots; give them real heads/sections.
Disagrees with green validator: n/a.

## appliances/gas-boiler
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | C | B | Wall boiler: casing, door, control panel with knobs and display, 5 stubs and top flue collar are all there; the collar facets and the interior is empty |
| 100 | B | C | B | Controls merged to a panel and one knob; recessed lower panel; nicely abstracted |
| 200 | B | C | B | Plain box with flue stub, a square recess for the sight glass and a skirt; stubs became one bar |
Instructions:
- [50/100/200, thermal, th-casing] The cutaway shows an almost empty dark cavity with a small burner, flame, sight glass and flue collar floating. Add the heat exchanger, fan, pump, gas valve, expansion vessel (hidden regions) as volumes and an insulation layer inside the casing with thickness (nested shells). At 200 add one merged hot-core volume.
- [50/100, thermal, th-burner-tube th-burner-bar th-flame] Burner floats at mid-height with nothing under or around it; attach to a combustion chamber (th-casing inner box) and place the flame inside.
- [50, thermal, th-flue-gas] The flue gas emitter is the small cyan part inside the top; extend it up through the collar and out of the top as the outlet.
- [50, body, collar-base collar-pipe] Collar is faceted; increase segments.
- [200, body, panel] The skirt under the front is a plain wedge; bring it back to the drawn control-panel shape.
- [50/100/200, structure, mass-casing] Mass is a dome clipped flat against the back wall; centre it in the casing volume (about 30 percent of depth from the wall) so it is fully inside.
- [50/100/200, structure, member-casing-shell] Shell shows an open front box; add the front door/plate as a member so the box closes (or state it in decisions).
Disagrees with green validator: n/a.

## appliances/gas-hob
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | C | Slab with a rimmed plate, one burner crown and a long rectangular trivet; knob at the side; spiky flame crown |
| 100 | B | B | C | Flame replaced by a cone; trivet frame remains |
| 200 | B | B | B | Trivet became an open rectangular tray on the plate; recognisable but crude |
Instructions:
- [50/100, structure, mass-trivet] The trivet mass is a thin ellipsoid floating in the air at the trivet rail height, not on any member. Replace by lumps at the rail/leg members (cast iron) located on the rails.
- [50/100/200, body, trivet-rail trivet-legs-front trivet-legs-back] Trivet is a 460 x 130 rail frame over a single burner, proportion reads off (wide narrow bar, burner is a small dome in the middle). Check the drawn trivet and make it a pan support of proportional width with 4 legs on the plate.
- [200, body, trivet] The 3D trivet is a rectangular box ring (walls) instead of rails on legs: the elevation drawing shows a gap between plate and slab. Model as top frame on legs (merged), keep the air gap.
- [50, body, flame-ring flame-centre] Flame crown spikes are sharp triangles; soften, keep within 7.5 mm feature size.
- [50/100/200, thermal, th-plate] Hatched housing is a solid block; add the gas pipe/injector and the hollow air space under the plate as hidden regions; flame plume emitter should start at the burner crown.
- [50/100/200, body, housing] Underside is a flat slab; add feet or a base lip.
Disagrees with green validator: n/a.

## appliances/gas-meter
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | C | Best of the set: two-shell tin case with seam bead, two pipe stubs with unions, index with counter and dial, rating plate; small drawn-only strip under the elevation |
| 100 | B | B | C | Case with two stubs, a window panel with a dial disc and a plate; seam gone |
| 200 | B | B | C | Rounded box with 2 stubs, a dark index slot and a dial; blue 3D-only corners in plan |
Instructions:
- [50, body, base faces] Elevation overlay has a red drawn-only strip along the bottom (about 100 mm wide): add the drawn base/foot to the 3D underside or remove from 2D.
- [100/200, body, housing] Plan overlay shows blue 3D-only blocks at the front corners at 200 and a blue strip at 100; trim the 3D index box to the drawn footprint.
- [50/100/200, structure, member-shell] The shell member is an open-top, open-bottom box: no top or bottom plate, pipe stubs and a green support hover inside the opening, the mass blob sits against the back wall. Close the case (top and bottom plates as thin members) and connect the stubs to the top plate with joints.
- [50/100/200, structure, mass-body] Move the mass to the centre of the case volume and inside the closed shell.
- [50/100/200, thermal, th-housing] Add the hidden bellows/diaphragm chambers inside the lower and upper shells; at 200 merge them as one volume.
Disagrees with green validator: n/a.

---
Totals (72 cells): A 2 (duct-fan 50 thermal, gas-meter 50 body), B 58, C 12 (desktop-monitor 200 body; electric-kettle thermal x3; gas-boiler thermal x3; gas-hob structure 50 and 100; gas-meter structure x3).


---

# Part C2: other assets (reviewer part 2)

# QA part 2 (appliances), visual review

Scope: 8 assets, 24 asset-scale pairs (all sheets and views images opened). No source file had changed in the last 15 minutes. Grades are from what is visible in the renders.

## appliances/kitchen-extractor-hood
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | C | Canopy plus telescoping chimney reads right; collar hole on top faceted; structure is the body shells re-labelled |
| 100 | B | B | C | Near copy of 1:50 (same 9 body solids, same 8 thermal regions, only one joint fewer) |
| 200 | B | B | C | Reduced to 5 solids, plain box chimney; filter and lamp-r dropped; structure is again the body shells |
Instructions:
- [50/100/200, structure, member-canopy-shell, member-canopy-end-l/r, member-chimney-lower, member-chimney-upper] These are `of:` references to the body solids, so the structure view is a solid filled canopy wedge and chimney box, not a load path. Author real members: 1 mm folded sheet shells shown as walls, an internal chimney bracket frame, the canopy-to-chimney stiffening flange, and the screw-on wall bracket as a visible member tying the chimney to the wall.
- [50/100/200, structure, support-wall-hook, support-wall-rest] Boxes 0.3 m tall sit at the back plane and are invisible in every render (hidden inside the shell). Put the two wall hooks at the chimney back plate and the canopy back flange where they touch the wall, outside the body ghost at the wall side so they read as supports.
- [50/100/200, structure, mass-canopy, mass-chimney, mass-chimney-top] The masses are not visible in any view (hidden inside the solid shells). Move the canopy mass to the motor/filter block (centre of the canopy, low) and show the chimney masses inside the hollow chimney; they must be visible against the ghosted body.
- [100, all lists] 1:100 is the 1:50 model with one joint removed. Re-abstract: merge lamp-l/lamp-r into the filter plate, merge collar into chimney-upper, drop the top-plate step and the faceted hole (round hole to 30 mm polygon-free cut or a flat disc).
- [50/100, body, collar] Round hole on the top plate is visibly polygonal; raise the segment count or make it a smooth lathe.
- [200, body, chimney-lower/upper] The chimney is a plain prism with no telescopic step at 1:200 while the drawing keeps the step; keep the 20 mm step or note the drawing is simplified (overlay is green, so only stylistic).
- [50/100/200, thermal] Cutaway removes the canopy front so the filter and lamp hatches are partly visible but the motor/fan/extract duct is not modelled; add an inner duct shell inside th-chimney-lower with th-extract as its core and a visible grease-filter layer under th-canopy.
Disagrees with green validator: no (overlay is all green; the weakness is semantic, not geometric).

## appliances/laptop
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | C | B | Keyboard, trackpad, hinge shoulder, feet and lid present; thermal cut shows one hatched slab and no hidden internals |
| 100 | B | C | B | Two slabs only (base, lid); keys, hinge and feet gone; lid drawn thicker than modelled (red halo) |
| 200 | B | C | B | Identical in look to 1:100; lid slightly wider than drawn (blue edge) |
Instructions:
- [50, thermal, th-deck, th-underside] The cutaway shows the deck as a single filled red slab with a dome cut. Add hidden regions inside the base: battery pack (front half, under the palm rest), SoC + heat pipe + fan block (rear centre), SSD, and an air gap shell between deck and underside; emit the plume at the hinge-side exhaust slot.
- [100/200, thermal, th-deck, th-underside] Same: add at least battery and SoC/fan regions as nested solids inside the base; currently nothing is hidden.
- [100, body, base, lid-shell] 100 is two slabs. Add the hinge shoulder strip (>= 15 mm) and the keyboard well as a recessed panel on the base so it is not just a folded plate.
- [100/200, body] 1:200 is a visual copy of 1:100; at 1:200 merge lid-shell and display and drop the keyboard well so the abstraction visibly differs.
- [100, body, lid-shell] Red halo along the lid in elevation: drawn lid is thicker than the 3D lid; thicken the lid to the drawn width (minimum 15 mm at this scale).
- [200, body, lid] Blue edge on the lid sides in the front overlay: 3D lid is wider than drawn; trim 0.5 percent per side.
- [50/100/200, structure, mass-base] The base mass is hardly visible at 1:50 and 1:100 (inside the slab) and at 1:200 sits as a flat disc on the top face; place it inside the base near the battery and make its footprint match the battery.
- [50, structure, joint-hinge] The hinge is a thin orange stripe floating on the lid face; run it along the real hinge axis at the base/lid junction and give it hinge-barrel thickness.
Disagrees with green validator: no (overlay mostly green; the red lid halo is the only mismatch).

## appliances/pendant-lamp
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Rose, cord, holder, faceted dome shade and bulb read correctly; dome and bulb visibly faceted |
| 100 | B | B | B | Near copy of 1:50; red ring in the plan overlay (shade drawn larger than 3D) |
| 200 | B | B | B | Cord thinner than drawn (red halo in elevation); ceiling hook capsule bigger than the rose |
Instructions:
- [50/100/200, body, shade, bulb-glass, bulb] Visible 16-segment facets on the dome and on the bulb. Raise to at least 32 segments around and 12 rings (or use a lathe profile).
- [100, body, shade] Plan overlay shows a red ring around the whole shade: scale the 3D shade radius up by about 1 percent to match the drawn rim.
- [200, body, cord] Red halo along the cord in the 1:200 elevation: the 3D cord is thinner than the drawn width; widen it to 30 mm or draw it as the minimum feature.
- [50/100, structure, mass-cord] The magenta ellipsoid sits on the cord and is wider than the cord (reads as a floating bead). Reduce its radius to the cord diameter or fold it into the cord member.
- [50, structure, mass-rose] The rose mass pokes out of the rose top; keep it inside the rose volume.
- [200, structure, support-ceiling-hook] The green capsule is tilted and sticks out beyond the rose ghost; make it vertical and contained in the rose cylinder.
- [100, thermal, th-shade-plume] The plume is not visible in any of the cut views; confirm it exists as a solid above the shade and the bulb (emitter at the bulb, rising into the shade top).
- [100/200, scale] 1:100 and 1:50 differ only by two joints and several masses; at 1:200 the collar merges into the shade. Good for 200, but 100 should merge the collar into the shade too.
Disagrees with green validator: partially: overlays are mostly green but the 200 cord is visibly thinner than drawn (red halo) and the 100 shade is smaller than drawn.

## appliances/room-thermostat
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | C | B | Rounded plate with display, LED, dial and pointer is right; dial is a 12-gon; thermal cut shows only an empty shell |
| 100 | C | C | B | Body is two solids and loses display and LED; thermal cut leaves display, LED and dial floating with no casing |
Instructions:
- [50/100, body, dial] The dial is a visible 12-gon. Use at least 32 segments.
- [100, body, housing, dial] Body drops the display recess and LED; at 15 mm minimum feature the display (about 35 x 20 mm) must stay. Add a display panel solid on the housing front.
- [100, thermal, th-casing] In the cut view the casing is missing entirely and display, LED and dial float in space. Make th-casing a nested shell (outer plastic, 2 mm wall, air cavity) so the cut leaves a half shell visible, and add PCB and battery as hidden regions inside.
- [50, thermal, th-casing] The cut shows an empty dark red box: add PCB, sensor and battery hidden regions inside the cavity.
- [100, structure, support-wall-screw-top, support-wall-screw-bottom] The two green blocks stick out above and below the housing; keep them inside the housing contour (screw bosses) or give them mounting-ear members that are part of the housing.
- [50/100, structure, member-dial, mass-dial] The dial mass is a faceted pink blob right on the housing face; shift it inside the dial and smooth it.
- [50, overlay] Housing elevation has a thin red margin and the plan a blue wall bump: the 3D housing is slightly smaller than drawn in elevation and the wall-contact solid protrudes in plan. Match the drawn outline.
Disagrees with green validator: partially (the validator is green, but the floating thermal pieces at 1:100 are visible only in the cutaway).

## appliances/server-rack
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Vents, cable holes on roof, casters, door present; door reads as a flat perforated panel; floating mass ellipsoid |
| 100 | B | B | B | Glass door with frame, casters; roof detail gone; same frame |
| 200 | B | B | B | 12 solids, plain box with door frame; near copy of 1:100 |
| 500 | C | B | C | Plain box, door area shows an odd open pocket; red band at the bottom (feet missing); posts of the frame not connected |
Instructions:
- [50, body, door-field, door-header, door-handle] The door is flat; add the visible glass panel and equipment behind it so the rack reads as a rack, as at 1:100.
- [50/100/200, structure, mass-equipment] The magenta ellipsoid fills the middle of the cabinet with no rails or trays touching it (floating in the void). Replace by 4 to 6 slab masses sitting on the equipment rails at each server height, inside the equipment-rail envelope.
- [50, structure, member-equipment-rail-0..3] Rails are hairlines, thinner than the posts; give them the drawn 1U angle section (15 x 15 mm) or merge them into the posts.
- [100/200, scale] 1:200 is a copy of 1:100 minus the roof lines; at 1:200 merge casters into a single plinth and drop the door-handle and the door frame so the abstraction differs.
- [100/200, structure, member-frame-ring] Only a top ring exists; add the bottom ring tying the four posts so posts do not float above the support ring.
- [500, body, cabinet, server-block, top-group] The front shows a recessed pocket in the lower right (a partly open door) and the red band in the elevation and front overlays says the 3D stops about 100 mm above the drawn base. Extend the cabinet down to the floor, close the front pocket with a plain door face, and keep one solid recess line only.
- [500, structure, member-frame-post-0..3, member-equipment-rail-0..1, member-door] Posts are four free-standing sticks with no ring, different heights, and a door-frame loop hanging beside them: it does not read as a load path. Add a bottom and top frame ring (75 mm minimum section) connecting all four posts and the support base, and make the posts the same height.
- [500, thermal, th-feet] The feet block is hatched but no foot exists in the body; either add the plinth to the body or drop th-feet.
Disagrees with green validator: yes at 1:500: overlay shows a full-width red band at the bottom of elevation and front (drawn base missing in 3D), which a pass/fail count did not flag.

## appliances/shower-head
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | C | B | Rail with holder, bracket, hand shower and hose loop reads right; the thermal cut leaves only head and hose |
| 100 | B | C | B | Same; hose partly yellow (slender flag), fewer segments |
| 200 | C | C | B | Rail is a big square bar, hose is an angular polyline loop, lower bracket gone, loop differs from drawing (blue/red in plan) |
Instructions:
- [50/100/200, thermal, th-tube, th-bracket-low, th-valve, th-handle] The rail, bracket and valve vanish in the cutaway and only head and hose remain; in the outside view they are plain red/green slabs. Make the rail a nested tube (steel wall, water channel inside) so the cut shows a section, and put hot-water core inside valve and hose.
- [50/100/200, thermal, th-spray, th-steam-plume] The magenta spray ellipse and sphere float in front of the head face detached from the head. Anchor them at the head face outlets and let the spray cone start at the face plane.
- [200, body, hose-00..07, hose-c0..c2] The hose loop is a coarse polyline with visible corners and a different loop from the drawing (blue/red in the plan overlay). Use at least 24 segments along the loop and match the drawn centreline.
- [200, body, tube] The rail is far wider than at 1:50/1:100 (a block); keep the drawn 30 mm width and re-add the bracket-low so the hose has an anchor.
- [50, structure, support-wall-top, support-wall-low] The green wall supports are small tilted capsules offset from the rail brackets (floating); make them boxes touching the bracket back faces.
- [50/100/200, structure, mass-shower] The pink dot sits next to the head, outside the head body at the elbow; move it inside the head volume.
- [100/200, body, hose] Hose segments are flagged slender (yellow): widen to the minimum feature or turn them into a tube sweep.
Disagrees with green validator: yes at 1:200: the plan overlay shows large red (head) and blue (hose) areas that a green status would hide.

## appliances/toaster
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Rounded casing with lever slots, dial, buttons, feet, crumb pan reads as a toaster; rounded corner shows stepped bands |
| 100 | B | B | B | Same with fewer details; thermal top plate slab stands proud of the casing |
| 200 | B | C | B | Four solids, feet and dial gone, structure keeps the feet; thermal top-plate is a big green slab above the casing |
Instructions:
- [50/100/200, body, casing] Corner roundings show visible banding; use a smooth fillet (at least 8 steps per quarter).
- [200, body, casing] Feet are missing (red patches at the bottom in the elevation overlay) while structure support-foot-left/right hang below the body. Add two foot pads to the body.
- [100/200, thermal, th-top-plate] The green top plate protrudes above the casing and is wider than the recessed plate in the body (visible in the az215 outside view). Match its size and height to body top-plate.
- [50/100/200, thermal] Add the heating elements (nichrome mesh on both sides of each slot, hot orange) as hidden regions inside the cavity; the slice is shown but the heater is not.
- [50/100/200, structure, member-base-frame] The chassis is a thick walled open box (reads as a solid block). Model as 4 corner posts plus base plate plus heater frame so the load path to the feet is visible.
- [100, structure, member-base-frame] Blue strip at the bottom in elevation: 3D base frame extends below the body outline; lift it to the body bottom.
Disagrees with green validator: partially: top-plate mismatch is clear in render but the overlay is mostly green.

## appliances/tower-server
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Case, bezel, rear vents and fan grille, feet are present; front intake, drive bay, power button are not visible |
| 100 | B | B | C | Case with bezel panel and feet; structure has four posts and a base plate only, mass ellipsoid floats |
| 200 | C | B | C | Two plain boxes, no plinth visible except a dark strip; posts unconnected at the top and ellipsoid mass floats |
Instructions:
- [50, body, drive-bay, intake-mesh, power-button, front-bezel] Listed solids do not show in the front view; confirm they sit on the front face (or cut the bezel and place the drive bay in front).
- [100/200, body, case-side, front-bezel] 1:200 is a plain box; keep at least the bezel step and the rear vent block (30 mm features) so it reads as a case.
- [100/200, structure, member-case-post-fl/fr/rl/rr] Four free posts with no top frame; the case top is not carried. Add a top ring (as at 1:50) and a bottom tray.
- [50/100/200, structure, mass-case] The big faceted ellipsoid hangs in the void with no tray, rails or contact; replace by a motherboard slab and PSU box as masses attached to the tray and the rear wall.
- [200, structure, support-feet-contact] The green support is a ring beneath the case with an empty middle; make it four foot pads under the posts.
- [50/100/200, thermal, th-side-panel, th-case-plume] Thermal regions are only thin panels and vents; add inner shells for PSU (rear top), motherboard and CPU heatsink, drive bay and fans so the cutaway shows hidden internals.
Disagrees with green validator: no (overlays are green; weaknesses are in the structure and thermal lists).


---

# Part C3: other assets (reviewer part 3)

# QA part 3 (visual review, all sheets and views of all four scales read)

Ids are not quoted where I could not verify them with a query; instructions name the part as seen (author maps to ids in T/volumes/<category>/<slug>.json). No asset source changed in the last 15 minutes.

## appliances/vacuum-cleaner
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | C | C | Body reads as a cylinder canister with ribbed hose, wand, floor nozzle and wheels; canister is stepped/faceted (visible horizontal banding) and the rear/underside is plain. Thermal is a red shell, green lid and one yellow disc: no motor, dust bag/filter or fan chamber although the cutaway removes the near half. Structure canister is one solid blue slab with a magenta disc inside, green supports are thin plates next to the wheels |
| 100 | B | C | C | Same object with fewer hose rings (good abstraction). Thermal is identical in content to 1:50 (22 solids both), structure is the 1:50 canister slab again |
| 200 | B | C | C | Body 16 solids, wand shown as a thin yellow slender (below 30 mm) fused to the hose at the elbow; thermal still 20 solids with the same shell/lid/wheel set (a copy of the finer one); structure canister solid block |
| 500 | B | C | C | Body is a clean block abstraction with the wand merged into the hose arch (blue overlay band = wand/hose silhouette thicker in 3D than drawn). Thermal is broken: hose is striped teal/white (gaps), wand is magenta only half-way and a white gap runs to the nozzle, nozzle block floats detached from the wand; structure canister solid slab with supports as big blocks |

Instructions:
- [50 and 100, thermal, canister body-shell/lid/wheels] Add the hidden internals under the lid: dust bag/filter, motor block with fan, cable reel, with their own nested layers inside the shell; at 1:100 merge them into one motor+bag volume instead of copying the 1:50 list.
- [50 and 100, structure, canister member] Replace the single filled slab by a load path: chassis tray (floor plate), two side ribs/axle bracket to the wheel axle, front caster fork; place the motor mass on the tray and the bag mass above it; keep the magenta mass inside the shell volume only.
- [200 and 500, thermal, hose + wand + nozzle] Make hose and wand shells continuous (remove the white gaps at 500), reconnect the nozzle to the wand end, and drop the 50/100 solids you only copied (e.g. brush strip, small joints) to produce a genuine coarser abstraction.
- [200, body, wand] The wand is a slender below 30 mm: thicken to the drawn width (>= 30 mm) or fold it into the hose arch as done at 500, to remove the red/blue halo.
- [500, body/structure, hose+wand] The 3D arch is wider than the drawn silhouette (blue outline in elevation/front/plan); thin the hose/wand to the drawn width.
- [50, body, canister shell] Round the canister shell with more segments (facet banding visible in all views) and the wheel with a proper tyre.
Disagrees with green validator: n/a (no validator data read); visually the thermal lists at 100/200 do not differ enough from 50.

## furniture/dining-table
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Tapered legs, apron, top with leaf and slide rails underneath; thermal cut hides leaf/rails well but top is one red slab; structure legs/apron/top blue, slab drawn as two tones only |
| 100 | A | B | B | Legs less tapered, otherwise the 1:50 object with fewer parts; thermal near copy (8 solids) |
| 200 | B | B | B | Legs straight, apron simplified: elevation shows red blocks at the top ends (drawn apron/breadboard ends missing in 3D) and blue slivers along the lower legs (3D leg thicker than the drawn taper) |
| 500 | B | B | B | Top thickens into a block, apron absorbed; supports as green foot blocks look large against 75 mm legs |

Instructions:
- [200, body, tabletop ends/apron] Add the apron end pieces that the drawing shows at the top corners (red), or remove them from the drawing; make the leg width equal to the drawn lower leg width (blue slivers).
- [50 to 500, thermal, table-top] The top is one solid slab; for 1:50 and 1:100 nest veneer/core/underside shells to show layers.
- [50 to 500, structure, table top] Show the top as joined slab on aprons with corner blocks; structure slab is currently two-tone filled with no rail/joint detail except at 50/100 orange tags.
- [500, structure, supports] Shrink the green foot supports to leg section.
Disagrees with green validator: n/a.

## furniture/height-adjustable-desk
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Desk with telescopic columns, feet, crossbar, drive boxes, monitor and laptop reads well; thermal cut shows only a salmon slab, green frame, a purple drive and the laptop lid; structure shows slab, columns and feet but no crossbar and no monitor/laptop mass |
| 100 | A | B | B | Thermal turns grey slab and is otherwise the 1:50 list (22 solids both) |
| 200 | B | B | B | Columns/feet blocky; crossbeam reduced to a bar; thermal still same |
| 500 | B | B | B | Monitor is a thick slab fused with the top in elevation; structure joints are oversized orange blobs on each column stage |

Instructions:
- [all, structure, top-board and crossbar] Add the crossbar/frame rails that tie both columns (visible in body) as members with joints at the column heads; add the monitor and laptop masses on the board (currently none visible).
- [all, thermal, top-board, drive, monitor-housing] Show hidden internals: motor and gearbox inside each column, controller box under the top, panel/backlight inside the monitor housing; cutaway currently hides nothing.
- [100 to 500, thermal] 100 and 200 are copies of the 50 colours/solids; reduce differently (merge columns stages, drop drive-right).
- [500, structure, joints] Scale the orange joint blocks down to the telescope stage section.
Disagrees with green validator: n/a.

## furniture/office-swivel-chair
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Five-star base, gas column, seat, tall back, armrests, castors read right; back/seat surfaces nicely rounded. Thermal has fabric/foam layers and a visible purple mechanism; structure backrest hangs on one thin post, armrest pads ghosted with posts ending in nothing |
| 100 | A | B | B | Fewer parts, same form; thermal same layers |
| 200 | B | B | B | Boxy armrests, seat pan flat; structure armrest now full members (good) but the backrest link is a thin rod |
| 500 | B | B | C | Armrests gone (drawn red in plan), column and seat are boxes, star arms thin rods; structure green supports are cubes much larger than the star arms they carry and the backrest touches the seat only at its edge |

Instructions:
- [50 and 100, structure, armrests] Include the armrest pad member or end the post at a joint; the post currently ends at a ghost.
- [50 to 200, structure, backrest] Add a backrest spine/bracket from seat plate to back so the backrest has a load path (thin rod only).
- [500, structure, caster supports (green cubes)] Reduce the cubes to 75 mm and size star arms to the same width as the body arms (they are far thinner than the drawn width).
- [500, body, base] The column and star are only 8-sided; raise segment count or accept boxes consistently.
- [500, structure, backrest] Connect the backrest to the seat (shared edge member).
Disagrees with green validator: n/a.

## furniture/operating-table
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Segmented padded table on column and base plus separate light stand with lamp reads right; column is a square box with a stepped plinth |
| 100 | A | B | B | Same, one fewer cushion detail; thermal column shows hollow notch |
| 200 | B | B | B | Table top is a plain slab, column conical in drawing but boxy in 3D (blue slivers beside the column) |
| 500 | B | B | B | Boxes; stand arm still resolved; light as faceted disc |

Instructions:
- [all, structure, table column and base] Column and base are filled blocks; use a hollow column section (telescopic) with a base frame and add the patient mass inside the pad volume.
- [all, thermal, light housing] Cutaway shows only a magenta hat with a lens; add LED ring/driver as hidden internals.
- [200, body, column] Make the column taper like the drawing (or fix the drawing); remove the blue slivers.
- [all, body, lamp disc] Raise facet count of the lamp dome.
Disagrees with green validator: n/a.

## furniture/park-bench
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Slatted seat and back with cast-iron end frames and armrests read right; structure legs reach the ground here but the magenta mass lenses sit in the void between the legs |
| 100 | A | B | C | Back is a solid board (fine abstraction); structure end frames end half way: lower legs and stretchers are ghosted white and the green supports float under the ghost |
| 200 | B | B | C | Same break in the load path; masses float in the leg gap |
| 500 | B | B | B | Slab-shaped legs, backrest as a tilted plate, structure continuous to the feet (good) |

Instructions:
- [100 and 200, structure, end frames (legs/stretchers)] Add the lower leg members down to the support so the green plates are connected; at present legs stop at the stretcher.
- [50 to 200, structure, masses (magenta lenses)] Move the masses into the seat/back timber volume (centre of seat), not the void between the frame legs.
- [50 to 200, thermal, steel-frame] Frame is hollow-looking in cutaway; add cast iron thickness shells or accept at one scale only.
- [200, body, armrests] Arms are thin bars (<30 mm); thicken to 30 mm.
Disagrees with green validator: probably yes: structure of 100/200 passes presumably though legs are visibly broken.

## furniture/picnic-table
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Planked top, benches, A-frames, hinge plates read right; benches are single slabs |
| 100 | A | B | B | Top planks fused to one slab (good), structure 19 solids again identical to 50 |
| 200 | B | B | B | Slabs and bars; fine |
| 500 | B | B | B | Big masses ellipses lie on top of the slabs (visible, probably at the surface rather than inside) |

Instructions:
- [500, structure, mass solids] Sink the three masses into the top/bench volumes (currently visible on the surface).
- [50 and 100, structure] 100 structure is a copy of 50; merge plank members at 100 into one slab as the body does.
- [50 to 200, thermal, A-frames] Leg sections 50 mm shown in cut; fine; add bolt/hinge hidden internals optionally.
Disagrees with green validator: n/a.

## furniture/single-bed
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | C | Head/foot boards, rails, quilted three-part mattress, centre leg read right; thermal shows hidden slats (good); structure is a solid slab-like mattress block with headboard, no frame/slat members visible |
| 100 | A | B | C | Slats collapse to a green plane; a small purple leg floats in the cutaway (the centre support whose rail was cut away); structure same filled blocks |
| 200 | B | B | C | Mattress one block; same floating leg; structure still three mattress blocks and ghosted footboard |
| 500 | B | B | C | Footboard is a full panel; thermal/structure have 3D-only blue bands along both long sides in the plan |

Instructions:
- [all, structure, mattress blocks] Remove the mattress from structure or carry it as a mass; show the load path: rails, slat bearers, headboard/footboard posts, centre leg with joints at rail ends.
- [100 to 500, thermal, centre leg] The leg hangs in the void when the near rail is cut; either keep the cross rail it hangs from or place the cut so it stays attached.
- [500, body, rails] Plan shows rails wider than drawn (blue bands): narrow the side rails to the drawn width.
- [200 and 500, thermal] Copy of 100 with fewer slats; give a genuine abstraction.
Disagrees with green validator: n/a.


---

# Part C4: other assets (reviewer part 4)

# QA part 4 (visual review)

Method: rendered with qa-render.ts, read every sheet and views image of every authored scale. No volume source was modified in the last 15 minutes for these eight assets. glass-balustrade and panel-radiator have no 1:500 (not authored, skipped).
Thermal row 1 is a cutaway, row 2 the uncut outside.

## furniture/three-seat-sofa
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | C | Reads as a sofa (rounded arms, three seat and back cushions, tapered feet). Back is one flat slab, structure is filled blocks, legs only ghosted |
| 100 | B | B | C | Visually identical to 50 (same 11 body solids, same bevels); structure same filled blocks |
| 200 | B | B | C | Simplified (back-2 plus end pieces, square feet) but seat and arms keep bevels; feet at right differ from drawing (red/blue at right foot) |
| 500 | A | B | B | Clean block abstraction, seat/back/arms/feet readable; drawn-only red slivers at both feet and at plan front edge |

Instructions:
- [50, body, back-1 back-2 back-3] Back cushions read as one continuous slab from the front and rear; give each cushion its own bevelled/rounded top edge and a visible gap, plus a slight backward lean and a thinner top than bottom. Rear shows only a flat panel: add a back frame/dust-cover shape (skirt line, top rail).
- [50, structure, member-arm-l member-arm-r member-back-1..3 member-seat-1..3] Arms and back are drawn as solid filled blocks and cushions as members: that is not a load path. Replace with the real frame: arm posts/panels (about 18-25 mm timber panel, hollow inside), back rail(s) between the arms, seat rails front/back/ends, deck (member-deck) on the rails, cushions only as masses. Add the four legs as members (currently ghost grey, only a green pad under them), so rails to legs to supports reads continuous.
- [50, structure, joint-seat-1..3 joint-back-1..3] Joints are tiny orange ticks floating under the cushions; place them at rail-to-post, post-to-rail and leg-to-rail connections instead.
- [50, thermal, th-back-1..3 th-back-1-core..] In the cutaway the back cushions are not visible at all (only on the outside view, green). Show the back cushion cut (fabric shell around foam core) so the back is readable in row 1.
- [100, all lists] 100 is a copy of 50 (identical silhouette, bevels, feet). Coarsen: merge seats into one cushion row with only shallow bevels, drop tapered-foot detail to simple prisms, reduce thermal shells to fabric+foam per element.
- [100/200, structure] Same filled-block problem as 50 for member-arm-l/r and member-back-*; at 200 reduce to arm panel, one back rail, deck plus legs, not three back blocks.
- [200, body, arm-foot-l arm-foot-r] The right foot is drawn at a different x than the 3D foot (blue/red pair at right foot in elevation overlay). Align the foot positions with the drawing.
- [500, thermal, th-seat-block th-back-block] Cutaway shows seat and arms as plain slabs; acceptable, but add a hint of the two-layer build-up (fabric/foam) as one thin shell.
- [500, structure, support-foot-*] Green feet become large cubes; keep legs at their drawn 75 mm minimum but align with the drawn feet (red slivers at both feet in the overlay mean the drawn feet are narrower and double).
Disagrees with green validator: no data on validator; visually the 100 copy and the solid structure blocks are things a numeric check would not flag.

## components/cantilever-balcony
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | A | B | Slab with wall segment, thermal break module, post+handrail+mid rail with base plate and bolts. No balusters/infill and no slab falls; rebar floats as thin vertical lines in ghost wall |
| 100 | B | A | B | Nearly the 50 model with base plate and bolts removed; rebar still drawn as lines in the ghost wall |
| 200 | B | B | B | Slab, wall, two posts and one handrail; thermal keeps break module and layers; structure only slab + posts |
| 500 | B | B | B | Block slab with two posts and a top rail; 3D rail narrower than drawn (red strips at post tops and at slab plan corner) |

Instructions:
- [50, body, railing-post railing-handrail railing-mid-rail] The balustrade is only two posts, a handrail and a single mid rail, wide-open, so it does not read as a safe balcony railing: add vertical balusters (or infill) at about 100 mm gaps at the 7.5 mm feature limit, or a glass infill; also a third post at mid span.
- [50, structure, rebar-top rebar-bottom wall-rebar-inner wall-rebar-outer rebar-top-hook] Rebar appears as vertical lines standing in the ghost wall and as small staples at the slab edge: top bars of a cantilever must run continuously through the slab into the wall slab with hooks, bottom bars shorter. Move the bars into the slab depth, extend them across the break module, and show the load-carrier joint at the break.
- [50/100, body, wall-core-band wall-core-upper wall-insulation wall-render] The wall is a very thin cut-out; add the hatch faces consistently (hatch appears on left only) or give the wall an end return so it does not read as a plane.
- [100, all lists] 100 is nearly identical to 50 (same silhouette and layers). Merge the three wall layers to wall-core plus one insulation/render pair, remove the rebar stubs outside the slab, simplify the break module to one solid.
- [200, structure, slab-member] Only a solid slab block is left: acceptable for 1:200 but add the break module line and the wall support block visibly (green strip only) so the load path (slab, break, wall) reads.
- [500, body/structure, railing-handrail railing-post] Red (drawn-only) at post tops and slab corners: 3D rail and posts stop short of the drawn 75 mm minimum; enlarge posts and rail to the drawn width/height.
Disagrees with green validator: n/a.

## components/curtain-wall-bay
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Frame with two mullions, three transoms, glass and spandrel panel, all nested; thermal cutaway layers visible (pane, cavity, spacer, break); supports practically invisible |
| 100 | B | B | B | Same silhouette as 50; thermal reads as hatch only (all solids cut by the plane, colours lost) |
| 200 | B | B | B | Frame/glass/spandrel as three solids; plan overlay shows blue 3D-only bands (frame/glass thicker than drawn) |
| 500 | A | B | B | Just glass, spandrel and head rail: right for 1:500; thermal shows nothing but cut hatch |

Instructions:
- [50, structure, support-base-guide-roller support-head-anchor-pin] The supports are 8 mm rollers/pins and are not visible in the structure render, so the load path (IGU on cradle, frame to head anchor and base guide) cannot be read. Enlarge to a readable section (at least 7.5 mm real feature, about 25-30 mm anchors) and add the setting blocks under the IGU.
- [50, structure, member-igu member-spandrel] Glass and spandrel are shown as full blue slabs; show only the setting-block bearing and the pressure plates, keep IGU as a ghost/mass rather than a member.
- [100, thermal, th-glass-centre th-glass-edge th-spandrel th-mullion th-transom-*] The cutaway shows only hatch/grey: the single-layer regions sit exactly on the cut plane. Give glazing at least pane/cavity/pane layers as nested shells or move the cut plane so glass, spandrel and frame colours read.
- [100, all] 100 is a copy of 50 (same frame, glass, spandrel). Reduce to frame ring plus two panels at 1:100 (already 7 body solids, so thin the mullion/transom caps and drop the spacer details).
- [200/500, thermal, th-glass-centre th-spandrel th-mullion th-transom] Whole bay is hatch only; split into glass (low U), spandrel (insulated), frame (thermally-broken) shells with distinct colours.
- [200, structure, member-frame] Plan overlay shows blue bands: 3D frame/glass depth is larger than the drawn section. Match member-frame depth to the drawn.
Disagrees with green validator: yes (likely): the thermal lists at 100/200/500 are valid but unreadable on the cutaway; the invisible supports at 50 would pass numeric checks.

## components/glass-balustrade
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Glass panel in a base shoe with handrail and gaskets; shoe is a solid box, glass floats above the plate in the structure |
| 100 | B | B | B | Same as 50 with gaskets/bolts removed: shoe plus glass plus handrail |
| 200 | B | B | B | Identical to 100 (copy), red line at handrail top in front section |
(no 1:500 authored)

Instructions:
- [50, body, shoe] The shoe is a closed solid box; model it as a U channel (two side walls plus base) with the glass seated between, gaskets (gasket-a, gasket-b) at the sides and wedge bolts (clamp-bolts) at the face.
- [50, thermal, th-clamp-shoe th-glass] The glass is one flat red slab: laminated glass should show 2 glass layers with an interlayer, and the shoe interior (gasket, glass seat) is hidden in a solid green box. Add the hidden internals in the cutaway.
- [50, structure, support-base-clamp-block clamp-bolts-joint glass-member] The support is a thin flat plate while the body shoe is a tall box, so the glass hovers above the support and the orange bolt ticks lie on the surface. Make the clamp block the full shoe section (base plus side walls) and put the bolts through the side wall into the glass edge zone.
- [100/200, all] 100 and 200 are copies of one another (same three solids, same silhouette). At 200 merge to panel plus top rail only; at 100 keep shoe separate from glass.
- [100, structure] Structure has no joints: add the clamp bolt joint at the shoe-to-glass line.
- [200, body, handrail] Red line at handrail top in the front section: drawn handrail is taller than the 3D handrail; raise it to the drawn height or draw to 3D.
Disagrees with green validator: n/a.

## components/glulam-beam
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | 15 lamella lines read as glulam, bearing plates at the ends; structure is one solid block with tiny supports |
| 100 | B | B | B | Plain box with bearing plates; thermal identical to 50 |
| 200 | B | B | B | Box with plates; red strip at beam soffit in front section (drawn-only) |
| 500 | A | B | B | Single box is right for 1:500 |

Instructions:
- [50, thermal, th-side-l th-side-r th-top th-soffit] The cutaway shows a full red hatched slab with hairline skins: the thermal layers (char/coating, wood core, moisture-buffer) are not nested shells of believable thickness. Give an outer coating/skin 3-5 mm and the wood core, and show the bearing plate gap/steel as a separate shell.
- [50/100/200/500, thermal, th-top th-soffit th-side-l th-side-r] The same six solids are repeated unchanged at all four scales: coarsen at 200/500 (core plus one skin) and keep the full set only at 50.
- [50/100, structure, support-bearing-left-pin support-bearing-right-roller joint-load-carrier] Supports are barely visible green dots at the beam ends and there are no bearing plates in structure: make bearing plates members and the pin/roller supports readable (min 7.5 mm at 50, 15 mm at 100).
- [200, body, beam] Front section shows a red band at the bottom: the drawn beam extends below the 3D beam (bearing plate zone). Make plates or beam match the drawing.
- [100, body, beam] Lamellae lost completely: add one or two groove lines or accept as is; tone is flat.
Disagrees with green validator: n/a.

## components/masonry-wall-with-lintel
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | A | B | Layered wall with window opening, lintel, sill; layers and hidden lintel build-up visible in cutaway; structure wall is one solid leaf with tiny pads |
| 100 | B | B | B | Same silhouette and openings; thermal collapses to hatch (4 solids) |
| 200 | B | B | B | Wall with opening, dark lintel strip and sill; thermal hatch/dark with no layers |
| 500 | B | B | B | Wall with opening only, no lintel/sill visible; thermal and structure identical to 200 |

Instructions:
- [50, structure, member-wall-leaf member-lintel-inner member-lintel-shell] The wall is one solid block and the lintel sits inside it: show the lintel as a distinct beam with bearing length of at least 150-200 mm on each side and the wall leaf as masonry above and below, with load-spreading on the lintel ends; the three small green foundation pads are the only support, so add a continuous or at least wider wall footing.
- [100, thermal, th-field th-field-leaf th-lintel th-sill] The cutaway renders hatch only. Reduce to nested field shells (plaster, masonry, insulation) so layers are visible.
- [100/200, body, lintel sill] The lintel/sill are slabs stuck on the face and the plan overlay shows blue bars (3D only) in the opening: either extend the drawn lintel/sill or keep them in the wall depth.
- [200/500, thermal, th-field th-lintel th-sill] Identical at both scales and a flat dark colour: 500 should be one shell per element, 200 should keep two layers.
- [500, body, lintel sill] Lintel and sill are gone from the 3D body at 500 although the asset is named for the lintel: keep a visible lintel strip (at least 75 mm) or note that it is merged; the structure list still has member-lintel and member-sill at 500, which cannot be seen in body.
- [200/500, structure, member-wall member-lintel member-sill] 200 and 500 are the same six solids: 500 should drop the sill member and merge lintel into the wall.
Disagrees with green validator: yes (likely): 500 structure lists member-lintel/member-sill that the body no longer shows; a pass on counts hides the copy.

## components/panel-radiator
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | C | Panel with three front sections, top grille, end caps, brackets and bottom valve/pipes; structure is a solid slab block |
| 100 | B | B | C | Same as 50 with convector reduced to a flat sheet; thermal front bands read as slabs |
| 200 | B | B | C | Block panel with bottom valves; valve stubs offset vs drawing (red/blue) |
(no 1:500 authored)

Instructions:
- [50, structure, member-panel] The structure is a solid blue slab; a panel radiator is two thin pressed steel sheets welded at the edges, with a water channel and welded convector fins. Show member-panel as the rear and front sheet(s) (1.25 mm sheet at the 7.5 mm minimum is too thin: use a 7.5 mm hollow shell) plus the fin row, not a block; add the water mass (magenta) at the panel centre of gravity.
- [50, structure, member-bracket member-bracket-far support-bracket-top-block] A long green rail runs along the back at the top while the body shows four small separate brackets: align them. Show the four bracket members as the actual load path to the wall (screw/hanger) and put the support at the wall plane.
- [50, thermal, th-panel-front-lower th-panel-front-mid th-panel-front-upper th-convector-front th-convector-rear] The front is cut into three horizontal solid colour bands, each a full-depth slab: no water channel and no air gap between sheet and fins; the convector appears as a zig-zag only at one end. Make the front panel a thin shell with the water layer (inner) and convector fins nested behind it.
- [50, body, top-grille] Top grille is a solid cap: add slots (at 7.5 mm minimum) and the radiator's side end caps rounded.
- [100, all] 100 is nearly a copy of 50: merge the three front sections into one front plate with two groove lines, replace convector with one block, drop bracket-far.
- [100, thermal, th-convector] The convector is reduced to a thin sheet beside the rear panel: it must sit between the panels and be thicker than the drawn gap.
- [200, body, valve-body valve-body-right pipe] The red/blue ticks at the valve stubs show the 3D valves stand at different x than the drawn ones: move valve-body and valve-body-right to the drawn positions.
- [200, structure, member-panel] Same solid slab problem: reduce to a shell block or declare a single member of the panel, with a visible bracket rail.
Disagrees with green validator: no data; visually the solid structure slab and missing water/convector in the thermal cutaway are not caught by counts.

## components/sectional-garage-door
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Door panel with four sections, hinges, rollers, bottom seal, curved tracks, hangers, torsion shaft with drums and spring; panel faces are flat with hardly visible joints, core not visible in the cutaway |
| 100 | B | B | B | Close copy of 50 (rollers and hinges still individual) |
| 200 | B | B | B | Panel as one slab, tracks, shaft; hinges become thick green strips across the panel |
| 500 | B | B | C | One slab panel, tracks and shaft; structure hangers float detached from the tracks |

Instructions:
- [50, body, panel-1 panel-2 panel-3 panel-4 panel-rib-1..4] Joints between sections are hairline and ribs are tiny: add the 10-15 mm panel joint groove with the typical horizontal rib profile, and a visible top and bottom weather seal.
- [50/100, thermal, th-hinge-leaf-up-* th-roller-*] The cutaway shows one red slab: the panel core (PU) and outer/inner skin layers are not visible. Show the section as three nested layers (outer skin, PU core, inner skin) so the cut face reads.
- [50, structure, m-panel-1..4] Each panel is a solid blue block: show the panel as a skin shell plus end stiles/struts and the hinge reinforcement, with the mass as the spring.
- [50, structure, support-hanger-1 support-hanger-2 m-hanger-l m-hanger-r] Hangers end free in the void (vertical stubs from the horizontal track): add the ceiling bracket end or a ceiling mass reference so the track load path ends at a support.
- [100, all lists] 100 duplicates 50 (29 body solids vs 30): drop individual rollers to roller pairs, merge hinges into strips, remove the track-end bar.
- [200, thermal, th-hw-strip-1..4] Hinge strips run across the full door width while the body shows only small hinges: make them the same short hinge blocks as in the body or reduce the strip to what is drawn.
- [500, structure, support-hanger-1 support-hanger-2] The hanger supports float as four separate green blobs near the track ends and there is no hanger member in the 500 body or structure (m-hanger-l/r are missing from 500); add the hangers as members attached to the track or remove the supports and keep jamb fixings.
- [500, body, rail-up-l rail-arc-l rail-back-l rail-up-r rail-arc-r rail-back-r shaft] Rails and shaft are yellow slender (below 75 mm at 1:500) and the elevation overlay has red at the track ends; thicken the tracks and shaft to the drawn minimum or merge into a thicker rail box.
- [200/500, thermal, th-panel-*] At 500 the cut shows only hatch: add at least a skin plus core split or a single colour with legend.
Disagrees with green validator: yes: the floating hanger supports at 500 and the slender rails pass numeric checks but read wrong on screen.

---
Summary tally: 8 assets, 30 authored scale rows, 90 grades: A 9, B 74, C 7.


---

# Part C5: other assets (reviewer part 5)

# QA part 5 (visual review of sheets and views; sources not modified in the last 15 min)

## components/sliding-door
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Portal head track, two jambs, hung glass leaf with frame, two hangers, handle, seal: reads right. Thermal jamb/track show as single solid boxes although regions have skin/cavity layers; no cavity visible. Structure: leaf frame hung on two orange carriage joints, supports at jamb feet, but no floor guide and no visible masses |
| 100 | B | B | B | Nearly identical to 1:50 (same hangers, handle, seal); reads as the fine model with a few parts dropped, not re-abstracted |
| 200 | B | B | B | Leaf frame and glass merged, hangers gone, leaf now touches the head track; sensible, but thermal is the same flat boxes and the structure joint at the carriage is not visible |
| 500 | B | B | C | Body fine (portal plus leaf slab). Thermal: leaf frame only a green cap strip on the head, glass hatch smaller than the grey body ghost (red edge). Structure: leaf is one dark filled slab hanging on a joint that is not visible = block, not a load path |

Instructions:
- [50, thermal, th-jamb-*/th-track-*] make skin-b / cavity / skin-a read as nested shells (visible hollow cavity in track and jamb), not one solid box each.
- [50, structure, m-leaf-*] add a floor guide (support) and show the two carriage joints bigger; add a mass for the leaf at its centre.
- [100, body, hanger-a/b, handle-*, bottom-seal] abstract: merge frame stiles/rails into one frame ring and drop the sub-15 mm parts instead of keeping the 50 list.
- [200, structure, j-carriage] make the joint visible at the head (enlarge to minimum feature) and add supports at jamb feet that read.
- [500, thermal, th-leaf-frame/th-glass] make glass hatch fill the leaf outline (it is smaller than the body leaf); give leaf frame a full ring or none.
- [500, structure, m-leaf] replace the filled slab by a frame-ring member plus joint at the head; show j-carriage.
Disagrees with green validator: n/a (no data); 500 leaf glass visibly smaller than body leaf.

## components/spiral-stair
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Tube column, base plate, 15 wedge treads with lip, helical handrail with balusters: reads very well. Handrail ends float above the last tread with no newel; thermal has only three flat regions (treads, column, balustrade), column cavity not shown; structure has no weld joints between treads and column sleeve |
| 100 | B | B | B | Same picture as 1:50 (34 vs 38 solids, same balusters, same helix): a copy. Balusters are r 6 mm = 12 mm wide at 1:100 (min 15 mm) |
| 200 | C | B | C | Balusters removed from the body: handrail is a faceted helix floating in the air, touching nothing (no posts, no tread contact). Thermal still draws 15 balusters that the body no longer has (inconsistent). Structure: handrail member floats, not connected to treads or column |
| 500 | C | B | C | Same floating faceted handrail as 1:200; polyline facets visible on a 65 mm rail. Structure handrail unconnected; mass/support invisible |

Instructions:
- [50 and 100, body, baluster, handrail] keep the 100 rod diameter >= 15 mm at 1:100 (r0/r1 0.006 -> 0.0075) or merge balusters into a panel; at 100 collapse stairs to fewer fine parts so it is a real abstraction of 50.
- [50, structure, st-handrail, st-baluster] add joints at baluster bottoms into tread-lip and a handrail end support; add weld joints tread-to-sleeve.
- [50/100, thermal, th-column] show hollow tube (air core) as an inner region; add paint/zinc shell on treads if relevant.
- [200 and 500, body/structure, handrail] connect the handrail: add posts (one every 3rd tread, >= 30 mm at 1:200, >= 75 mm at 1:500) or a baluster band, so the rail is carried by treads; smooth the helix (more loft sections) to remove faceting.
- [200, thermal, th-baluster] remove th-baluster (not present in body) or add it back to the body.
- [200 and 500, structure, st-handrail, mass-balustrade] add supports/joints at the first and last tread; keep the mass inside the body.
Disagrees with green validator: yes at 200/500 - floating handrail passes, visibly unsupported; at 100 baluster 12 mm below the 15 mm feature limit.

## components/tied-canopy
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | A | Wall with render/insulation/concrete, I-beam cantilever, four glass panes, diagonal tube strut, cheek plates, pins: very credible. Thermal wall layers are hard to tell apart in the cutaway (concrete vs wool vs render); the rear of the wall only shows plain colour. Structure: beam, strut, cheek plates, pin joints, wall anchor tails: believable load path |
| 100 | A | B | A | Slightly simplified; wall now two layers. Thermal wall shows only the hatched slab (no visible layers) |
| 200 | B | B | B | Panes merged to one sheet, plates gone. Thermal wall loses wool/render layers while body keeps them; rod top end has no joint/support at the wall top (hangs on wall face) |
| 500 | B | B | B | Wall single block, roof sheet, beam, rod: good abstraction. Thermal wall one hatched block; rod top end unsupported again |

Instructions:
- [50, thermal, th-wall-concrete/th-wall-wool/th-wall-render] use more distinct region colours or an explicit section hatch so the three layers read in the cutaway.
- [100/200, thermal, th-wall-core, th-wall-insulation, th-wall-outer] render the layers as separate shells (they exist in the body) instead of one hatched block.
- [200 and 500, structure, support-tie-rod, support-wall-pin] add a support at the rod head on the wall (support-wall-pin sits only at the beam); add a joint at the rod/beam connection.
Disagrees with green validator: no.

## components/tilt-window
Scales authored: 50, 100, 200 (500 not authored).

| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | C | Reads as a window in a deep timber reveal with glass, but the sash is not distinguishable from the frame (stiles/rails hide in the reveal), no handle, hinge barrel invisible, no tilt gap. Thermal: glass inner/cavity/outer and edge seal show well, sill yellow; the frame is one solid purple block. Structure: reveal/jambs are filled blocks, no hinge joint or support visible anywhere |
| 100 | B | B | C | Pixel-identical to 1:50 in the body, glass cavity collapsed to a grey slab in thermal; copy rather than abstraction |
| 200 | B | B | C | Again identical body; sash-frame is a ring but nothing separates it from jambs. Structure has support-hinge as a 40 mm box, not visible |

Instructions:
- [50, body, stile-left, stile-right, rail-bottom, rail-top] set the sash proud of the jambs (or add a visible sash/frame gap) and add an operating handle plus a visible hinge; in the closed pose the sash ring must read.
- [50, structure, m-jamb-left, m-jamb-right, m-sill-plate, j-hinge-barrel, support-hinge] jamb/sill blocks are deep solid U boxes: reduce to real frame sections (50 x 70 mm) and show hinge barrel and support where they connect; hinge-barrel is hidden inside the sill.
- [100, body] drop the 50 sash details to a single sash ring (as 200 does) and keep pane; do not ship it as a copy of 50.
- [100/200, thermal, th-pane-centre, th-pane-edge, th-pane] keep glass/cavity/glass layers at 1:100 (24 mm IGU) and show the edge seal.
- [200, structure, support-hinge] make the support visible and located on the sash edge.
Disagrees with green validator: n/a.

## components/wheelchair-ramp
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | C | Two-run ramp with a landing, kerbs, double handrails with extensions, posts with base plates: reads right. Thermal: red top skin, hatched green sides, purple kerb OK but the core is a hatch only. Structure: the whole ramp is one filled blue wedge, supports only a green block at the top landing |
| 100 | B | B | C | Identical to 1:50 with a few parts fewer; structure unchanged filled block |
| 200 | B | B | C | Slab body and 27 slender posts/rails (below 30 mm?) give red/blue slivers on the rail in the overlay; structure unchanged |
| 500 | C | B | C | Handrails float in the air above the slab, no posts, nothing touches the ramp. Overlay shows posts drawn (red) but missing in 3D. Structure rails also float; green supports appear only as stripes under the slab |

Instructions:
- [50/100/200, structure, m-pour-run-1, m-pour-landing-middle, m-pour-run-2, m-pour-landing-top, m-slab] replace the filled wedge by a believable load path: slab (thickness), retaining side walls or fill, strip footing, with supports along the whole underside (support-ground-* exist only at the top landing).
- [200, body, post-0..5, rail-*] make posts >= 30 mm wide at 1:200 (visible red halo now).
- [500, body, rail-run-1, rail-landing, rail-run-2] add posts (>= 75 mm) or merge the rail into a balustrade panel on the slab so the rail touches the ramp; same for th-rail-* and m-rail-*.
- [500, structure, support-ground-0..3] align supports so the slab sits on them clearly; add joints where rails meet posts.
Disagrees with green validator: yes at 500 (floating rails pass), partly at 200 (slender posts).

## objects/bicycle-stand
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Smooth tube hoop with a round bend and crossbar: reads as a stand. No ground collars/flanges visible in the body although they exist in the id list; thermal tube is a thin red shell with open ends and no air core; structure: legs, bends, weld joints, green foot blocks sit under the tube ends |
| 100 | B | B | B | Hoop bends are faceted polylines (three segments per bend); red/blue slivers in the elevation show hoop vs structure mismatch; plan shows red squares = collars/feet drawn but missing |
| 200 | B | B | B | Coarser faceted bends, red squares in plan at the feet (collars missing), structure foot blocks are larger than the legs and protrude below |
| 500 | C | B | B | Body is a flat extruded A-shaped plate (100 mm wide strips, 30 mm thick), not a tube; thermal/structure show a thin tube floating inside the grey ghost slab, so body and other lists disagree. Structure foot blocks plunge beneath the body |

Instructions:
- [50, body, ground-collar-l, ground-collar-r, weld-collar-l, weld-collar-r] collars exist in id list but are not visible: make them visible and sit them over ground.
- [100/200, body, hoop] increase bend resolution (e.g. >= 8 segments per 90 degree) so bends do not facet; make structure bend members match.
- [100/200, body] add foot flange/collar solids (red squares in plan show they are drawn).
- [500, body, hoop] model as a tube (round or octagonal section 75 mm) following the hoop path, not an extruded plate; align th-hoop and structure hoop-member with the same width.
- [500, structure, support-leg-left-block, support-leg-right-block, root-l, root-r] keep supports flush under the feet, smaller than the legs' width plus 50 mm.
Disagrees with green validator: yes at 500 (the body slab is much wider than the thermal tube shell yet passes).

## objects/chalkboard
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | A | B | B | Two panels in a frame with centre mullion, sliding leaf, tray with chalk and sponge: very good. Thermal enamel/core layers good (red left, violet right), frame green, tray yellow; structure is an all-blue filled slab with little detail (bars not visible, joints tiny) |
| 100 | B | B | B | Same as 1:50 minus tray items; thermal now flat grey panels (core layers lost); structure same filled slab with orange strip |
| 200 | B | B | B | Right panel is a brown opaque slab (leaf in front?) while 50 shows two dark panels; thermal panels grey; structure filled slab, green hook pin sits on the front face |
| 500 | B | B | B | Body is a plain slab; the tray (red strip in elevation) is missing in 3D, thermal hatches only; structure supports (green blocks) sit on the front face, not behind the board |

Instructions:
- [50 to 500, structure, frame-*, panel-left-member, panel-right-member, leaf-panel-member] show the board as frame bars plus panel members (hollow in between) and not one solid colour block; move joints to hanger positions.
- [100/200, thermal, th-panel-left, th-panel-right, th-leaf-slab] keep core/enamel layering (two shells) at 1:100 where it exists at 50.
- [200, body, leaf-slab, panel-right] make the leaf cover the right panel consistently with the pose or render both panels the same.
- [500, body, tray] add tray (>= 75 mm) at the bottom edge as drawn.
- [200/500, structure, support-wall-hook-pin, support-wall-rest-roller] put the supports at the rear/top edge, not on the visible front face.
Disagrees with green validator: partly at 500 (tray drawn but not in 3D).

## objects/drinking-fountain
| scale | body | thermal | structure | one line what you saw |
|---|---|---|---|---|
| 50 | B | B | B | Cast pedestal with collars, flange bolts, bowl, rim bead, swan-neck bubbler, jet, drops: reads very well. Visible facets on the revolved pedestal and bowl (shaded stripes); thermal shows a hollow pedestal shell, basin, bubbler, jet but no supply pipe or drain inside; structure all blue, hidden anchor bolts not shown |
| 100 | B | B | B | Collars/bolts/drops dropped, facets still visible; thermal and structure about the same as 50 (nearly a copy minus parts) |
| 200 | B | B | B | Bubbler is an angular polyline spout, bowl faceted, pedestal 16-gon visible; structure support slab green, mass invisible |
| 500 | B | B | B | Bubbler is a large box on the rim plus slanted stub: a block stand-in, too big; support missing in structure; joint absent |

Instructions:
- [50/100/200, body, pedestal, basin] raise radial segment count (>= 48) to remove visible faceting on the revolved bodies.
- [50, thermal, th-pedestal] add the hidden supply pipe and drain core inside the hollow pedestal; add water in basin.
- [100, body, thermal] differentiate more from 50 (single collar ring, no bolts) so it is a new abstraction.
- [200, body, bubbler] smooth the spout path (more segments).
- [500, body, bubbler, button] replace the box with a tapered tube/neck of >= 75 mm; shrink the head.
- [500, structure, support-base-fixed] add base support and a basin joint; show mass inside the pedestal.
Disagrees with green validator: no.


---

# Not graded (open)

market-parasol, playground-swing (re-render, source changed), retractable-bollard, shopping-trolley, stepladder, street-lamp, tealight, waste-bin: renders ready in 🗑️generated/qa/objects/, review blocked by the failing Read tool of the host.
