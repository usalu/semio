"""Reviewer decisions on thermal areaM2 (rulings C1, C15, C16, C18, C19). Key (slug, region) -> (act, value, reason, finding).
act: A accept the author value (value None = the value found in the file, else the value to apply: author's measured value of an unapplied row), RX restore committed + areaBasis exchange,
RF restore committed + model finding, S set a hand-calculated value, SX set + exchange, X mark exchange only (value stays), RB restore committed body region (C18) + finding,
N no change needed (value already right), DEFER no authored 1:50 model in the file yet (C15). The apply script only writes when the file still holds the value this review saw."""
A, RX, RF, S, SX, X, RB, N, DEFER = "A", "RX", "RF", "S", "SX", "X", "RB", "N", "DEFER"
D = {}
def d(slug, region, act, value, reason, finding=""):
    D[(slug, region)] = (act, value, reason, finding)

d("agave", "caudex", S, 0.29, "stem cylinder o0.22 x 0.42 m: pi*0.22*0.42 = 0.29; the model is a rectangular prism (extruded profile), not a cylinder", "agave/caudex: the `caudex` extrusion is a 0.42 x 0.22 x 0.26 m box (rectangular profile and flat extrusion); a revolve or loft of o0.22 would expose 0.29 m2")
d("apple-tree", "fruit", RX, None, "aggregate of ~750 apples o0.08: 750 * 4*pi*0.04^2 = 15 m2 at harvest load; five fruit solids cannot show it")
d("apple-tree", "limb-bark", RF, None, "4 scaffolds 2.5 m o0.15 (4.7) + 8 secondaries 1.5 m o0.08 (3.0) = 7.7 m2; 4+4 limbs of the model give 3.0", "apple-tree/limb-bark: only 4 scaffold and 4 secondary limbs are modelled, a vase crown of a full-standard tree has ~8 secondaries per scaffold plus tertiaries (about 8 m2 of bark)")
d("apple-tree", "spur-twig", A, None, "4 twig bundles of o0.02: no hand estimate better than +-50 % at 0.1-0.25 m2, region is weighting only (passive)")
d("apple-tree", "trunk-bark", RF, None, "stem 0-1.7 m o0.38 avg = 2.0, fork 1.7-2.7 m o0.22 = 0.7 -> 2.7-3.6 m2 with fluting; the model counts 4.98", "apple-tree/trunk-bark: the `trunk` loft starts 0.35 m below ground (buried root flare r0.55) and tapers to a point at 3.64 m, i.e. a central-leader cone through the vase crown that the real tree does not have")
d("bamboo-clump", "culms", RF, None, "6 culms o0.029 tapering to o0.015, mean o0.022 x 3.5 m mean height: 6*pi*0.022*3.5 = 1.45-1.9 m2", "bamboo-clump/culms: every culm is a flat ribbon, the extrusion of a 0.029 m wide profile over z -0.37..0.37 (0.74 m wide), exposing 19 m2; a revolve/loft o0.029 is needed")
d("date-palm", "fruit", SX, 9.0, "8 bunches * 600 dates * 1.9e-3 m2 (prolate spheroid 0.035 x 0.020) = 9.1 m2; individual dates cannot be modelled")
d("date-palm", "stem-bark", RF, None, "stem 8.1 m o0.5 = 12.7 + crown shaft 1.0 + root flare 1.2 = 14-15 m2; the model counts 36.3", "date-palm/stem-bark: the 26 ring and 9 skirt/plug detail extrusions sit on the stem surface and are counted as additional exposed bark (2.5x the stem), they must be flagged `texture` or kept inside the stem envelope")
d("hosta-clump", "petioles", A, None, "~16-22 petioles o0.012 x 0.3 m = 0.18-0.25 m2, covers the model value")
d("ivy-on-trellis", "stems", RF, None, "4 main stems 2 m o0.015 (0.38) + 12 laterals 0.5 m o0.008 (0.15) = 0.53 m2", "ivy-on-trellis/stems: the hidden stems region carries only a fraction of the stems (0.14 m2 against 0.5 m2 of 4 main stems and laterals)")
d("ivy-on-trellis", "trellis-timber", A, None, "posts 2 x 2.1 m x 0.38 (1.6) less shoes + rails 1.0 + battens 1.0 = 3.2-3.5 m2; both faces of a 2.8 m2 lattice is the honest value (old 6.3 treated the lattice as a solid plate)")
d("ivy-on-trellis", "post-shoes", A, None, "2 open U shoes 0.11 x 0.28 x 0.13: 2*(2*(0.0308+0.0143+0.0364)) = 0.33 less the base on the ground = 0.26-0.30")
d("japanese-maple", "stem-bark", DEFER, None, "no volumes in the file at review")
d("japanese-maple", "limb-bark", DEFER, None, "no volumes in the file at review")

d("concrete-mixer-truck", "cab", A, None, "cab 2.5 x 2.0 x 2.2 m box without the underside: 2*(5.0+5.5+4.4)-5 = 25-27 m2")
d("concrete-mixer-truck", "cab-glazing", A, None, "drawn windscreen 2.1 x 0.62 (1.3) + door and quarter glass 1.4 = 2.7 m2 from the drawing")
d("concrete-mixer-truck", "chassis", A, None, "rails 2 x 8.8 m x 0.28 x 2 faces 9.9-10.8 + cross members 2 + tanks 3 + 4 axles with springs 8 + sub-frame deck 8.8 x 0.62 both faces 10.9 + steps 1.5 = 36 m2 (old 12 was the rails only)")
d("concrete-mixer-truck", "chute", A, None, "open trough both faces: main 2.4 m (floor 1.25 + walls 1.44) + extension 1.3 m (0.7 + 0.78) = 4.2 per face, 8.4 both; old 4 was one face")
d("concrete-mixer-truck", "engine-bay", RF, None, "engine tunnel: grille 0.9 + tunnel top and sides 2*0.6*0.8 + belly pan 0.8 = 2.7-3.2 m2; q = 4000 W / 3.2 = 1250 W/m2 stays", "concrete-mixer-truck/engine-bay: the region has the front grille only (0.65 x 1.4); tunnel panels and belly pan of the engine compartment are missing as thermal solids")
d("concrete-mixer-truck", "exhaust-stack", A, None, "vertical stack o0.15 x 2.9 m = 1.37 m2 (pipe o0.127 gives 1.2)")
d("concrete-mixer-truck", "drum", RF, None, "9 m3 drum, 16 m3 geometric volume, o2.4: belly + two cones = 32-40 m2 (pi*2.4*5.05*0.85 = 32 + closures)", "concrete-mixer-truck/drum: the 59-section `drum-flight` loft (spiral blade, inside the drum) is counted as exposed outer surface; 51.8 m2 against ~36")
d("concrete-mixer-truck", "drum-drive", A, None, "gearbox/motor housing 0.8 x 0.8 x 0.6: 2*(0.64+0.48+0.48) less the mount 0.64 = 2.6-3.2 m2")
d("concrete-mixer-truck", "tyres", A, None, "8 tyres 315/80 R22.5 (o1.04 x 0.315): tread 1.03 + 2 sidewalls 0.59 each = 2.2 -> 17.6 m2 (19.2 with the wider drawn tyre)")

d("diesel-generator", "skid-tank", A, None, "exposed sides and ends of the 6.06 x 1.94 x 0.2 tank 2*(6.06+1.94)*0.2 = 3.2 + castings 0.7 = 3.9 m2; old 12 was the footprint 6.06*1.94 = 11.8 (contact plane, C1b)")
d("diesel-generator", "discharge-hood", RF, None, "hood outer shell: pentagon sides 2*0.70 + slope 0.73 + top 1.43 + back 0.62 = 4.2 m2; heat leaves through the outer face (q = heatW/area), inner faces face the hot air", "diesel-generator/discharge-hood: the 40 mm plates (layer says 2 mm) are counted on both faces and the outlet frame, 10.2 m2 against 4.2 m2 of outer shell")
d("diesel-generator", "inlet-louvres", S, 5.8, "60 slats (4 panels x 15) of 2 mm sheet 1.3 x 0.035: both faces 2*1.3*0.035 + edges = 0.096 m2 each = 5.8 m2", "diesel-generator/inlet-louvres: slat boxes are 25 mm thick (layer 2 mm), counting 0.157 m2 per slat instead of 0.096")
d("diesel-generator", "rain-cap", A, None, "plate 0.3 x 0.3 both faces 0.18 + rim 4*0.3*0.02 = 0.2 m2")
d("diesel-generator", "outlet-flap", A, None, "flap 1.7 x 0.63 one face 1.07 + edges 0.28 (other face against the mouth frame) = 1.25-1.35")
d("diesel-generator", "enclosure-walls", A, None, "4 walls 2*(6.06+2.44)*2.4 = 40.8 m2 plus ~8 % corrugation = 44 m2")
d("diesel-generator", "silencer", A, None, "capsule o0.55: cylinder 2*pi*0.275*1.65 = 2.85 + two hemispheres 0.95 = 3.8 m2")
d("diesel-generator", "exhaust-stack", A, None, "stack o0.2 x 0.4 m = 0.25 m2")

d("evaporative-humidifier", "casing", A, None, "box 0.5 x 0.6 x 0.4 outer 1.48 m2 + sump tray rim; old 0.9 left a pair of faces out")
d("evaporative-humidifier", "trough", S, 0.2, "open trough 0.4 x 0.36: inside floor 0.144 + inner walls 2*(0.4+0.36)*0.04 = 0.06 -> 0.2 m2 (bottom lies on the pad)")
d("evaporative-humidifier", "pad", RX, None, "fluted cellulose pad: wetted flute surface is an exchange area the 0.4 x 0.4 x 0.36 envelope cannot show")
d("evaporative-humidifier", "sump", A, None, "free surface 0.44 x 0.38 = 0.167 + rim and visible sides = 0.2 m2")

d("excavator", "house", RF, None, "upper structure profile perimeter 9.08 m * 2.5 m width less underside 8.4 + 2 side faces 5.2 = 19.5 m2, plus cab (perimeter 6.5 * 1.15 + sides 4) = 11 -> 30 m2", "excavator/house: measured 17.6 m2 for house + cab + boom foot, the profile extrusions alone give 30 m2: parts of the cab and house are buried or the cab overlaps the house")
d("excavator", "counterweight", A, None, "profile perimeter 4.8 m less the front face 1.2 against the house = 3.6 m x 2.4 m = 8.6 + 2 sides 1.8 = 10.4 m2")
d("excavator", "cab-glazing", S, 4.0, "cab 1.15 x 1.5 x 1.7: windscreen 1.23 (drawn) + two side windows 2*0.9 + rear 0.5 + roof 0.5 = 4.0 m2", "excavator/cab-glazing: side, rear and roof windows are slivers (0.2 m2 beyond the 1.23 m2 windscreen)")
d("excavator", "hydraulics", RF, None, "boom cylinders 2 x (barrel o0.14*1.5 + rod o0.08*1.2 = 0.96) + stick 0.73 + bucket 0.55 = 3.2 m2 + hoses", "excavator/hydraulics: only one boom cylinder is modelled (the real excavator has two) and no hoses")
d("excavator", "attachment", A, None, "boom 5.7 m (perimeter 2.2) 12.5 + stick 2.9 m 5.2 + bucket outer 5 + inner 4 = 27-30 m2")
d("excavator", "tracks", A, None, "2 tracks: outer belt 9.3 m x 0.6 = 5.6, both side faces 6.8 = 12.4 each = 24.8 + carbody 3 = 27.8 m2")
d("excavator", "engine-compartment", A, None, "hood panels and engine block surface 2.5-2.6 m2 (box 1.1 x 0.65 x 1.0 less base)")
d("excavator", "exhaust-stack", A, None, "stack o0.12 x 0.5 m = 0.19 m2")

for r, why in (("body", "box 0.064 x 0.05 x 0.06 = 0.0125 m2"), ("head", "revolve o0.05 x 0.03: 0.0047 m2"), ("inlet", "pipe o9.5 mm x 0.1 m = 0.003 m2"), ("outlet", "pipe o9.5 mm x 0.1 m = 0.003 m2"), ("bulb", "bulb o0.014 x 0.05 m = 0.0022-0.0025 m2"), ("capillary", "o1.6 mm x 0.15 m = 0.00075 m2")):
    d("expansion-valve", r, A, None, why)

d("finned-heat-exchanger", "fins", RX, None, "fin pack: 200 fins of 0.205 x 0.52 both faces = 40 m2 of exchange surface; neither 2 m2 (heatW tuned to it) nor the 1.09 m2 envelope shows it")
d("finned-heat-exchanger", "tubes", A, None, "6 tubes o0.018 x 0.48 m = 0.163 m2 plus bends = 0.19; heatW 24 W follow-up: keep q = 300 W/m2 -> 56 W if the coil should stay as warm")
d("finned-heat-exchanger", "headers", A, None, "two 0.15 x 0.5 x 0.02 boxes: edges 0.026 each + top face of the upper one 0.075 + stubs 0.01 = 0.13 m2 (old 0.012 was the stubs only); heatW 2 W follow-up: keep q = 167 W/m2 -> 22 W")

for r, a, why in (("chassis-paint", A, "chassis 1.0 m wide: top 1.9 + sides 1.7 + arches 1.2 = 4.7-6 m2"), ("guard", A, "4 posts o0.06 x 1.2 + rails and slats = 2.5-3 m2"), ("battery-pack", A, "case 1.0 x 0.47 x 0.9: sides 2*0.47*1.0 + ends 2*0.47*0.9 + top 0.9 = 2.7 less embedded = 2.2 m2; heatW 540 W is physical, q rises to 248 W/m2 (case 20-25 K above the air)"), ("counterweight", A, "profile perimeter 3.2 m * 1.1 m + 2 sides 1.7 = 4.3 m2"), ("controller", A, "dash panel 0.6 x 0.7: 0.42 m2"), ("hydraulics", A, "2 tilt cylinders: barrel o0.08*0.45 + rod = 0.16 each = 0.32 m2"), ("fork-gear", A, "carriage plate 0.6 x 0.84 + tines 2*0.5 + backrest = 1.5-1.6 m2"), ("tyres", A, "front o0.78 x 0.2 + rear o0.55 x 0.16: tread + outer sidewall 1.9-2.0 m2"), ("brakes", A, "4 drums o0.3: 0.3 m2"), ("pallet", A, "EUR pallet 1.2 x 0.8: deck top+edges 1.0 + runners 0.8 = 1.78 m2"), ("cargo", A, "goods block 1.16 x 0.9 x 0.76: 4.34 m2 less the face against the backrest 1.04 = 3.3 m2")):
    d("forklift", r, a, None, why)
d("forklift", "drive-motor", RF, None, "traction motor o0.28 x 0.4 (0.35 + ends 0.12) + gear housing 0.25 = 0.7-0.9 m2; 0.192 would be q = 2240 W/m2 for 430 W", "forklift/drive-motor: the region is the flat 0.5 x 0.1 x 0.8 `drive-cover` only; motor and gearbox housing are missing")
d("forklift", "mast", RF, None, "2 C-channel outer rails (perimeter 0.7 x 2.05 = 1.4 each) + 2 inner rails (0.6 x 1.6 = 0.96 each) + cross members, chains, brackets 0.6 = 5.4 m2", "forklift/mast: rails are solid 0.14 x 0.14 and 0.10 x 0.14 boxes; real mast rails are C/I-channels whose inner faces are exposed (5 m2)")

for r, a, why in (("feet", A, "2 feet 0.34 x 0.18 x 0.09 without top and bottom: 2*2*(0.34+0.18)*0.09 = 0.19 m2"), ("fan-guard", A, "blades 5 x 0.06 = 0.3 + hub/motor 0.15 + stay 0.11 + wire grille 0.15 = 0.7-1.1 m2"), ("discharge-vane", A, "vane 0.24 x 0.72 both faces 0.346 + edges 0.096 = 0.44 m2 (free in the air stream)"), ("base-tray", A, "tray 0.45 x 1.1 underside 0.495 + edges 0.22 = 0.715 m2"), ("top-lid", A, "lid top 0.47 x 1.12 = 0.526 + edges 0.127 = 0.65 m2")):
    d("heat-pump-outdoor-unit", r, a, None, why)
d("heat-pump-outdoor-unit", "casing", RF, None, "outer box 0.72 x 1.1 x 1.45 = 6.9 less coil side 1.6, outlet 0.7, bottom 0.8 = 3.8 m2 (outer faces only; inner faces look into the coil and fan chamber)", "heat-pump-outdoor-unit/casing: 1-2 mm panels are counted on both faces (inner sides included), 6.1 m2 against 3.4-3.8 m2 outer")
d("heat-pump-outdoor-unit", "coil-guard", RF, None, "louvred guard over the coil face 0.86 x 1.06 = 0.91 m2 x louvre factor 1.75 = 1.6 m2 (q = -137 W/m2)", "heat-pump-outdoor-unit/coil-guard: the `coil` box 0.22 x 0.86 x 1.06 is a solid slab; both big faces and edges (2.8 m2) are counted, the back face looks into the fan chamber")

d("plate-heat-exchanger", "frame", A, None, "4 posts 0.05 x 0.56 + base 0.5 x 0.5 + 2 end walls: 1.2 m2")
d("plate-heat-exchanger", "plates", RX, None, "cross-flow plate pack: 60+ plates of 0.44 x 0.4 are an exchange surface (several m2) the 0.44 x 0.4 x 0.54 envelope cannot show")
d("plate-heat-exchanger", "ports", S, 0.22, "4 collars 0.30 x 0.15 x 0.06: outer wall 2*(0.30+0.15)*0.06 = 0.054 each = 0.216 m2 (inner walls look into the duct)")
d("plate-heat-exchanger", "damper", A, 0.368, "flap 0.4 x 0.4 both faces (free in the bypass) 0.32 + edges 0.02 = 0.34 m2")

for r, a, v, why in (("feet", A, None, "base ring plate 0.28 square with o0.19 hole: top 0.049 + bottom 0.049 (on grommets) + edges and hole wall = 0.056-0.105 m2"), ("shell", A, None, "shell o0.28 x 0.25 = 0.22 m2"), ("dome", A, None, "ellipsoidal cap r0.14: 0.06 m2"), ("discharge", A, None, "stub o16 x 70 mm 0.0035 + flange o32 x 4 mm 0.0016 = 0.005-0.007 m2"), ("suction", A, None, "stub o10 x 0.15 m with collar = 0.006 m2"), ("terminal", A, None, "box 0.036 x 0.06 x 0.055: 0.0145 less the mount 0.003 + lid = 0.0138 m2")):
    d("scroll-compressor", r, a, v, why)
d("scroll-compressor", "scroll", RF, None, "the scroll plate o0.09 seen through the sight glass: one face pi*0.045^2 = 0.0064 m2 (old 0.006), q = 3 W/0.006", "scroll-compressor/scroll: the plate-with-spiral solids count both faces, edge and spiral flanks (0.0158 m2) of a part of which only the glass-facing face belongs to the region")

d("solar-thermal-collector", "absorber", RF, None, "selective plate 1.88 x 0.91 = 1.71 m2 (aperture 1.8), risers lie under the plate", "solar-thermal-collector/absorber: the nine risers are counted as exposed surface on the sun side (2.38 m2 against 1.71 m2 plate face)")
d("solar-thermal-collector", "housing", S, 2.6, "tray 2.0 x 1.0 x 0.1: underside 2.0 + long sides 0.4 + ends 0.2 = 2.6 m2 outer shell (cavity faces lie under the glass)")
d("solar-thermal-collector", "stand", A, 0.96, "2 rails 1.5 m x 0.2 perimeter 0.6 + 4 struts ~0.5 m x 0.18 = 0.36 -> 0.96 m2")

d("sorption-wheel", "hub", A, None, "two end faces 2*pi*0.045^2 = 0.0127 less 16 spoke footprints 0.0036 = 0.009 m2")
d("sorption-wheel", "casing", S, 0.68, "outer band 2*pi*0.31*0.22 = 0.43 + two outer flange faces 2*pi*(0.31^2-0.27^2)... = 0.146 + stand block 0.1 = 0.68 m2 (inner band faces the wheel)")
d("sorption-wheel", "disc", X, None, "honeycomb matrix of 20 mm cells: exchange area, the envelope 2 x 0.25 + rim 0.32 cannot show it; keep 0.19")
d("sorption-wheel", "regen", S, 0.12, "two 84 degree seal plates, outer face 0.0596 m2 each = 0.12 m2 (the inner faces are the gap)")

for r in ("counterweight", "house", "winch-house", "cab-glazing"):
    d("tower-crane", r, DEFER, None, "no volumes in the file at review")
for r in ("bike-metal", "tyres", "saddle"):
    d("cyclist", r, DEFER, None, "no volumes in the file at review")

for slug, r, a, v, why in (
    ("parent-with-stroller", "stroller-fabric", A, None, "seat 0.45 x 0.8 both faces + hood + basket 0.38 x 0.4 box = 1.3-1.4 m2"),
    ("parent-with-stroller", "stroller-frame", A, None, "5.7 m of tube o0.024 = 0.43 m2"),
    ("parent-with-stroller", "tyres", A, None, "4 wheels o0.25 x 0.05: 0.1 each less contact = 0.33-0.4 m2"),
    ("person-with-rollator", "frame", A, None, "tube o0.025 x 8.4 m = 0.66 m2"),
    ("person-with-umbrella", "canopy", A, None, "shallow dome r0.52 rise 0.25: pi*(0.27+0.0625) = 1.04 m2 + rim = 1.15"),
    ("person-with-umbrella", "umbrella-frame", A, None, "shaft o0.021 x 0.95 + handle + hub = 0.064 m2"),
    ("person-with-white-cane", "cane-shaft", A, None, "o0.012 x 1.3 m = 0.049 + tip = 0.0588"),
    ("runner-with-blade-prosthesis", "blade", A, None, "J blade 0.07 wide x 0.5 m x 2 faces + edges = 0.076 m2"),
    ("skateboarder", "trucks", A, None, "2 boxes 0.075 x 0.05 x 0.18: 0.105 less mounts = 0.07 m2"),
    ("skateboarder", "wheels", A, None, "4 wheels o0.054 x 0.03: 0.0097 each less bearing = 0.034 m2"),
    ("sleeping-person", "pillow", A, None, "top 0.24 less the head 0.06 + exposed rim 0.1 = 0.2-0.28 m2 (covered faces are buried by head and duvet, C1a)"),
    ("standing-phone-user", "headphones", A, None, "band 0.4 x 0.02 + 2 cups = 0.02-0.03 m2"),
    ("tour-guide", "pointer-metal", A, None, "capsule r5 mm x 0.548 m: 2*pi*0.005*0.548 = 0.0172 m2"),
    ("tour-guide", "pole-metal", A, None, "capsule r12 mm x 1.34 m: 2*pi*0.012*1.34 = 0.101 + caps = 0.117 m2"),
    ("tour-guide", "flag-cloth", A, None, "0.468 x 0.302 both faces = 0.283 m2 + edges"),
    ("walking-commuter", "bag-leather", A, None, "box 0.209 x 0.294 x 0.14: 0.263 less the face on the hip 0.06 = 0.2 m2"),
    ("wheelchair-user", "tyres", A, None, "2 rear tyres o0.6 x 0.03 (0.23 each) + 2 castors 0.04 each = 0.54 m2"),
    ("wheelchair-user", "cushion", A, None, "seat cushion + back sling 0.4-0.45 m2"),
    ("worker-carrying-box", "box-surface", A, None, "box 0.4 x 0.3 x 0.3: 0.66 m2 less the hand-covered 0.15 = 0.51 m2 (C1a)"),
):
    d(slug, r, a, v, why)
d("runner-with-blade-prosthesis", "socket", RB, None, "C18: body-model region (socket worn on the stump), measured 0.068 deviates 29 % from the committed 0.0959; the committed value stays", "runner-with-blade-prosthesis/socket: loft of 9 sections exposes 0.068 m2 against 0.0959 (-29 %), check the section radii of the socket loft")

ap = (
    ("ceiling-diffuser", "plate", RF, "flange ring 0.152 + 3 cone risers 0.09 + annuli 0.176 = 0.42 m2 (square 0.595 diffuser)", "ceiling-diffuser/plate: the stepped cones are revolved round (o0.595) instead of square, exposing 0.24 m2 against 0.42 m2"),
    ("circulation-pump", "casing", A, "volute o0.115 x 0.06 + unions = 0.03 m2", ""),
    ("circulation-pump", "motor", A, "motor head o0.09 x 0.08 = 0.024 m2", ""),
    ("circulation-pump", "terminal-box", A, "box 0.06 x 0.04 x 0.03 = 0.0128 m2", ""),
    ("decentralised-hrv-fan", "casing", A, "open box 0.26 x 0.26 x 0.24 without the wall side: 0.32-0.39 m2", ""),
    ("decentralised-hrv-fan", "blades", A, "5 blades 0.045 x 0.06 both faces = 0.027 m2", ""),
    ("decentralised-hrv-fan", "hub", A, "hub o0.056 x 0.03: 0.0075 m2", ""),
    ("desktop-monitor", "housing", RF, "rear shell 24-inch: back 0.3 x 0.5 = 0.15 + rim 1.66*0.03 = 0.2 m2", "desktop-monitor/housing: the 6-section loft exposes 0.124 m2 against ~0.2 m2 of back shell and rim"),
    ("desktop-monitor", "stand", A, "base plate + neck + feet = 0.11 m2", ""),
    ("desktop-monitor", "panel-front", A, "panel 0.299 x 0.53 = 0.159 + bezel edges = 0.2 m2", ""),
    ("duct-fan", "casing", S, "plate front 0.028 (0.22 square less the o0.16 opening) + ring inner wall 0.03 + sides 0.018 + lip 0.01 = 0.09-0.1 m2", "duct-fan/casing: plate/ring/lip extrusions expose 0.2 m2, twice the hand value; check that ring and lip are annuli, not discs"),
    ("duct-fan", "blades", A, "5 blades both faces 0.0285 m2", ""),
    ("duct-fan", "hub", A, "hub o0.068 x 0.03: 2 faces 0.0072 + side 0.0064 = 0.0136 m2", ""),
    ("electric-fan-heater", "feet", A, "4 feet o0.02 x 0.01: 0.0013 each", ""),
    ("electric-fan-heater", "grille", A, "perforated plate 0.17 x 0.21 both faces 0.036 + hole walls = 0.04-0.05 m2", ""),
    ("electric-fan-heater", "intake", A, "plate 0.16 x 0.21 0.034 + slats 0.012 + edges = 0.05 m2", ""),
    ("electric-fan-heater", "control", A, "dial o0.04 x 0.015: 0.0032 m2", ""),
    ("electric-fan-heater", "top-cover", A, "top 0.17 x 0.21 = 0.036 + rim 0.015 = 0.051 m2", ""),
    ("electric-fan-heater", "casing", A, "housing 0.16-0.17 m2", ""),
    ("electric-kettle", "plate", A, "power base o0.18: side 0.011 + visible top ring 0.008 = 0.019-0.021 m2; heatW 40 W gives q = 1900 W/m2 instead of 800 W/m2, follow-up: ~17 W keeps the old flux", ""),
    ("electric-kettle", "handle", A, "handle path 0.2 m x (2*0.03+2*0.02) = 0.02 m2", ""),
    ("electric-kettle", "lid", RF, "dome o0.14: outer face 0.017 + button 0.003 = 0.02 m2; inner face is the steam side (operating region, heat leaves through the outer face)", "electric-kettle/lid: lid dome counted on both faces, inner face is the steam side"),
    ("electric-kettle", "water", A, "free surface pi*0.07^2 = 0.0154 m2", ""),
    ("electric-kettle", "shell", A, "o0.15 x 0.2 = 0.094 m2", ""),
    ("electricity-meter", "plate", S, "name plate 0.13 x 0.04 x 0.0075: front 0.0052 + rim 0.0026 = 0.0078 m2 (back face on the housing)", "electricity-meter/plate: measured 0.0135 counts the back face on the housing"),
    ("electricity-meter", "window", RF, "window ring and glass 0.0055 + counter 0.0023 + rim 0.003 = 0.011 m2; inner faces look into the meter", "electricity-meter/window: glass inner faces are counted, 0.0245 m2 against 0.011 m2 room-facing"),
    ("electricity-meter", "disc", A, "disc o0.084 in the gap: two faces 2*pi*0.042^2 = 0.0111 m2", ""),
    ("electricity-meter", "casing", A, "box 0.17 x 0.2425 x 0.093 + back plate + terminals = 0.2 m2", ""),
    ("gas-boiler", "sight-glass", A, "window o0.08 less rim: 0.0042 m2", ""),
    ("gas-boiler", "controls", N, "panel 0.2 x 0.12 = 0.024 + knobs = 0.03 m2 (committed); the model region holds display, knobs and gauge but not the panel plate", "gas-boiler/controls: the panel plate between display and knobs is in `casing` solids; the region exposes 0.015 m2 of 0.03 m2"),
    ("gas-boiler", "flue-collar", A, "pipe o0.08 x 0.12 = 0.03 + collar 0.012 = 0.042; model 0.032 (-25 %)", ""),
    ("gas-boiler", "connections", N, "5 stubs o0.022 x 0.06 (0.0041) + nuts (0.0045) = 0.043 m2 (committed 0.04)", "gas-boiler/connections: stubs and nuts expose 0.025 m2 against 0.043 m2 of five o22 mm stubs with union nuts"),
    ("gas-hob", "trivet", A, "pan support rails + legs 0.043 m2", ""),
    ("gas-hob", "knob", A, "knob o22 x 9 mm: front 0.00038 + side 0.00062 = 0.001 m2", ""),
    ("gas-hob", "burner", N, "cap o0.06 x 0.012 (0.0057) + crown ring o0.1 (0.0088) = 0.0145 m2; 0.006 would make q = 20 kW/m2 (T > 600 K)", "gas-hob/burner: the exposed measure drops the part under the flame seat (0.006 m2); a brass burner at 350 C needs ~10 kW/m2 of 120 W"),
    ("gas-meter", "index", A, "index frame + base 0.19 x 0.098 + dial = 0.029 m2", ""),
    ("gas-meter", "pipes", A, "2 stubs o0.028 x 0.07 with unions = 0.017 m2", ""),
    ("gas-meter", "shell", A, "box 0.24 x 0.27 x 0.17 0.30 less back 0.065, index and pipe footprints 0.02 = 0.21; model 0.165 (-21 %)", ""),
    ("hot-water-cylinder", "connections", A, "5 stubs with nuts = 0.021 m2", ""),
    ("kitchen-extractor-hood", "chimney", A, "chimney section 0.3 x 0.2: perimeter 1.0 x 0.55 less back 0.165 = 0.39-0.55 m2", ""),
    ("kitchen-extractor-hood", "canopy", A, "slope 0.9 x 0.5 = 0.45 + sides 0.15 = 0.6 m2 outer prism without the back on the wall; q = 120/0.54 = 222 W/m2", ""),
    ("kitchen-extractor-hood", "filter", A, "plate 0.86 x 0.46 = 0.396 + edges 0.03 + mesh = 0.45 m2", ""),
    ("mvhr-unit", "ducts", A, "4 spigots o0.098 x 0.05: 0.062 + beads 0.012 = 0.075-0.12 m2", ""),
    ("pendant-lamp", "lampholder", A, "o0.05 x 0.03 = 0.0042 m2", ""),
    ("room-thermostat", "led", A, "half oblate dome a=3.5 mm c=2 mm: pi*a^2*(1+(1-e^2)/e*atanh e) = 5.6e-5 m2", ""),
    ("room-thermostat", "dial", A, "dial o0.042 x 0.006: 0.0014 front + rim 0.0008 = 0.0024 m2", ""),
    ("server-rack", "feet", RF, "4 castors o0.07 x 0.03: tread 0.0066 + sides 0.0077 = 0.014 each = 0.057 m2", "server-rack/feet: the foot solids expose 0.025 m2 against 0.057 m2 of four o70 mm castors"),
    ("shower-head", "head", A, "loft o0.137 x 0.0147: both faces 0.029 + rim 0.0072 + rounding = 0.037-0.045 m2", ""),
    ("shower-head", "valve", A, "o0.04 body + stubs = 0.018 m2", ""),
    ("split-ac-indoor-unit", "casing", A, "front 0.2465 + top 0.18 + ends 0.122 less the grille 0.083 = 0.47 m2 (back on the wall, bottom is the outlet)", ""),
    ("split-ac-indoor-unit", "discharge-vane", A, "vane 0.7 x 0.08 both faces + edges = 0.12 m2", ""),
    ("split-ac-indoor-unit", "intake-grille", A, "grille 0.7 x 0.12 = 0.083 m2", ""),
    ("thermostatic-radiator-valve", "head", A, "collar o0.06 x 0.068 = 0.0128 + end 0.0028 = 0.0156 m2", ""),
    ("thermostatic-radiator-valve", "body", A, "body o0.038 x 0.042 + stub o0.017 x 0.023 + nut = 0.0076-0.01 m2", ""),
    ("thermostatic-radiator-valve", "knob", A, "knob o0.04 x 0.02: 0.0027 + 0.0013 = 0.004 m2", ""),
    ("toaster", "feet", A, "4 feet o0.025 x 0.01 side = 0.0031 m2", ""),
    ("toaster", "top-plate", A, "top 0.3 x 0.17 = 0.051 + edges = 0.055 m2", ""),
    ("tower-server", "side-panel", RF, "two 0.413 x 0.438 panels, outer faces 2*0.181 = 0.36 m2", "tower-server/side-panel: the two panel boxes give 0.36 m2 but 0.2715 m2 is measured; check what removes 25 % (sectionFaces on the +w/-w faces of the panels or a buried portion)"),
    ("tower-server", "rear-panel", A, "rear 0.438 x 0.2 = 0.0876 m2", ""),
    ("tower-server", "psu-vent", A, "rear plate 0.108 x 0.17 + top 0.09 x 0.17 = 0.034 m2", ""),
    ("city-car", "roof", A, "roof 1.1 x 1.3 x curvature 1.2 = 1.7-1.8 m2", ""),
    ("city-car", "bonnet", A, "bonnet 0.9 x 1.3 x 1.2 curvature = 1.4 m2", ""),
    ("city-car", "tailgate", A, "hatch panel 0.8 x 0.95 = 0.75 m2", ""),
    ("city-car", "glazing", A, "screen 1.05 + rear 0.9 + 4 side lights 1.2 + 2 quarter lights = 4.0-4.3 m2", ""),
    ("city-car", "exhaust-system", A, "pipe o30 x 1.45 m 0.137 + silencer 0.43 + ends 0.06 + tail 0.1 = 0.72 m2 (applied 0.675)", ""),
)
for slug, r, a, why, finding in ap:
    v = None
    d(slug, r, a, v, why, finding)

for slug, r, a, v, why, finding in (
    ("bar-stool", "seat-plate", A, None, "plywood disc o0.36: underside 0.102 + rim 0.0136 = 0.116 m2", ""),
    ("bar-stool", "backrest", RF, None, "low backrest pad 0.25 x 0.1 both faces 0.05 + rim 0.02 = 0.07 m2", "bar-stool/backrest: the pad loft exposes 0.021 m2, one third of the two faces and rim (0.07 m2)"),
    ("bookshelf", "carcass", A, None, "2 sides (inner and outer faces) 2.1 + top and bottom both faces 0.92 -> 2.95 m2", ""),
    ("bookshelf", "shelves", A, None, "5 shelves 0.78 x 0.277 both faces 0.43 + edges 0.04 = 0.47 -> 2.3 m2", ""),
    ("bookshelf", "back-panel", A, None, "0.764 x 1.712 both faces = 2.62 m2 (free-standing)", ""),
    ("bookshelf", "plinth", A, None, "0.772 x 0.075 faces + top = 0.15 m2", ""),
    ("cafe-chair", "seat-beech", A, None, "seat 0.42 x 0.42 both faces 0.35 + aprons 0.1 + edge 0.05 = 0.5 m2", ""),
    ("cafe-chair", "back-beech", A, None, "panel 0.17 x 0.37 both faces 0.126 + posts 0.056 + rail 0.044 + finials = 0.25 m2", ""),
    ("cafe-chair", "steel-tube", A, None, "4 legs o0.022 x 0.42 + stretchers + feet = 0.15 m2", ""),
    ("dining-table", "apron", A, None, "aprons 4.2 m x 0.08 x 2 faces 0.67 + lower edge 0.1 = 0.78 m2", ""),
    ("folding-deck-chair", "canvas-back", A, None, "sling 0.6 x 0.4 both faces = 0.48 m2", ""),
    ("folding-deck-chair", "canvas-seat", A, None, "sling 0.45 x 0.4 both faces = 0.36 m2", ""),
    ("height-adjustable-desk", "steel-desk", A, None, "2 T-feet 0.28 + 2 x 2 telescope stages 0.7 + cross rail 0.28 + underframe and brackets 0.3-0.5 = 1.6-2.2 m2", ""),
    ("office-swivel-chair", "chair-plastic", A, None, "seat-shell underside 0.18 + backrest shell rear 0.18 + 5 base arms 0.2 + arms/hub 0.1 = 0.7 m2", ""),
    ("office-swivel-chair", "chair-fabric", A, None, "seat cushion + back pad exposed faces = 0.5 m2", ""),
    ("office-swivel-chair", "chair-steel", A, None, "column + stems + mechanism = 0.11 m2", ""),
    ("office-swivel-chair", "castor-rubber", A, None, "5 castor wheels o0.05 x 0.02 = 0.018 m2", ""),
    ("operating-table", "table-column", A, None, "telescopic column 0.21 x 0.34 box + lower loft + head = 0.7 m2", ""),
    ("operating-table", "stand", A, None, "base + column + arm + knob = 1.1 m2", ""),
    ("operating-table", "table-frame", A, None, "top carrier 1.96 x 0.4: underside 0.78 + edges 0.19 = 0.97 (0.81 measured, -17 %)", ""),
    ("operating-table", "light-housing", A, None, "dome o0.5 hemisphere 2*pi*0.25^2 = 0.39 m2", ""),
    ("operating-table", "table-base", A, None, "plate loft + 2 castors = 0.76 m2", ""),
    ("operating-table", "table-pad", S, 1.4, "4 pad sections 1.988 x 0.5: top 0.994 + long sides 2*1.988*0.04... = 0.318 + ends 0.08 = 1.39 m2 (underside on the carrier)", "operating-table/table-pad: measured 0.815 is below the top face alone (0.994); the tilted pad sections overlap each other at the hinges"),
    ("park-bench", "steel-frame", A, None, "two cast-style end frames: feet 0.2 + legs 0.5 + brace/bearer/arm 0.4 = 1.07 m2", ""),
    ("park-bench", "seat-timber", S, 2.33, "4 slats 1.8 x 0.11875 x 0.045: top+bottom 0.4275 + edges 0.162 + ends 0.011 = 0.6 each = 2.4 less bearer contacts 0.07 = 2.33 m2", "park-bench/seat-timber: measured 1.711 = top and bottom only (4 x 0.4275), the four 45 mm edges per slat in the gaps are missing (probably sectionFaces on the slat edges)"),
    ("park-bench", "back-timber", S, 1.7, "3 raked slats 1.8 x 0.1 x 0.04: top+bottom 0.42 + edges 0.144 each = 1.7 m2", "park-bench/back-timber: measured 1.26 = faces only (3 x 0.42), slat edges missing"),
    ("picnic-table", "hinges", A, None, "4 blocks 0.08 x 0.1 x 0.11: 0.056 each less mounts = 0.16 m2", ""),
    ("picnic-table", "bench-seats", A, None, "2 boards 0.36 x 1.7: both faces 1.22 + edges 0.17 = 2.7-2.8 m2", ""),
    ("picnic-table", "a-frame", A, None, "4 legs 0.85 x 0.055 board 0.175 each = 0.7 + beams 0.56 = 1.15-1.26 m2 (old 2.4 counted plate envelopes)", ""),
    ("picnic-table", "table-top", A, None, "5 planks 0.138 x 1.8: 0.497 + edges 0.155 = 0.65 each = 3.25 m2 (2.96 in the gaps)", ""),
    ("single-bed", "bed-frame", A, None, "head 0.95 x 0.9 + rails 2.04 x 0.16 x 4 faces + posts = 3.4 m2", ""),
    ("three-seat-sofa", "base-rail", A, None, "skirt 1.7 x 0.86 x 0.14: sides 0.72 + open underside 1.46 = 2.2 m2 (1.92 measured)", ""),
    ("three-seat-sofa", "back-cushions", A, None, "3 cushions 0.6 x 0.55 x 0.15: front 0.33 + top and sides 0.255 + back = 1.75-3.0 m2", ""),
    ("three-seat-sofa", "feet", A, None, "4 turned feet = 0.09 m2", ""),
    ("three-seat-sofa", "seat-cushions", A, None, "3 boxes 0.54 x 0.62 x 0.23: top 0.34 + front 0.12 + sides = 0.54 each = 1.63 m2", ""),
    ("three-seat-sofa", "arm-pads", A, None, "2 pads 0.2 x 0.52 x 0.9 = 2.7 m2", ""),
    ("sectional-garage-door", "spring-assembly", A, None, "122 turns o8 mm on a o75 coil: 0.72 m2 wire surface (open turns) + shaft 0.18 + drums 0.2 = 0.9 m2", ""),
    ("sectional-garage-door", "track-steel", A, 3.18, "9.8 m of 2 mm C-section (strip 0.12 m, both faces 0.235 m2/m) 2.3 + hangers 0.36 + tie bar 0.3 + flags 0.2 = 3.16 m2", ""),
    ("spiral-stair", "column-steel", A, None, "CHS 168.3 x 6.3 x 2.8 m: outer 1.48 + bore 1.37 (open) + base plate 0.1 = 2.95 m2", ""),
    ("spiral-stair", "treads", A, 6.67, "15 treads: plan 0.1247 (22.5 degree sector R0.8) top + underside 0.249 + 55 mm folded flange 2*1.8*0.0525 = 0.19 -> 0.44 each = 6.6 m2", ""),
    ("sliding-door", "leaf-frame", A, 1.21, "4 hollow 50 x 50 members, 5.6 m x 0.2 perimeter = 1.12 m2 + joints", ""),
    ("sliding-door", "track-steel", A, 1.4, "100 x 84 housing 3.1 m x 0.36 perimeter = 1.12 + end caps and slot walls = 1.4 m2", ""),
    ("sliding-door", "jamb-steel", A, 1.74, "2 posts 2.05 m x 0.4 perimeter minus rebate = 1.64-1.74 m2", ""),
    ("chalkboard", "frame", A, None, "frame ring 4.0 x 1.2: outer 0.47 + inner 0.47 + half front rim 0.2 = 1.15 m2", ""),
    ("chalkboard", "tray", A, 1.34, "4.0 m aluminium U trough, 0.26 m2/m (underside, front wall both faces, floor, inner back) = 1.03 + chalk, ruler, sponge 0.14 + ends = 1.2-1.34 m2", ""),
    ("charcoal-grill", "handle-wood", A, None, "handle o0.03 x 0.35 m = 0.033-0.045 m2", ""),
    ("charcoal-grill", "ash-catcher", A, None, "catcher disc 0.25: underside + rim 0.14 m2", ""),
    ("charcoal-grill", "bowl-shell", A, None, "spherical cap r0.29 depth 0.25: pi*(0.0855+0.0625) = 0.465 less vents and legs = 0.4 m2", ""),
    ("charcoal-grill", "vents-alu", A, 0.0172, "dial o80 x 35: side 0.0088 + top 0.0048 + 3 holes o12 walls 0.0040 = 0.0176 m2", ""),
    ("retractable-bollard", "beacon-led", A, None, "ring lens o0.2 x 0.05 = 0.031 + cap = 0.0375 m2", ""),
    ("retractable-bollard", "shaft-steel", A, None, "raised shaft o0.2 x 0.65 = 0.408 less reflector band 0.063 and beacon 0.03 = 0.31 m2", ""),
    ("retractable-bollard", "cover-frame", A, None, "cast frame 0.4 square ring = 0.093 m2", ""),
    ("retractable-bollard", "hydraulic-unit", A, None, "power unit box 0.45 m2", ""),
    ("retractable-bollard", "sleeve-steel", N, None, "1.2 -> 1.252 is within 5 %, no change", ""),
    ("shopping-trolley", "handle-grip", A, None, "capsule r20 mm x 0.49 m: 2*pi*0.02*0.49 = 0.0616 m2", ""),
    ("shopping-trolley", "frame-tube", A, None, "4.5 m of tube o14 mm (chassis 2.2 + forks 1.4 + legs) = 0.197 m2", ""),
    ("shopping-trolley", "wheels", A, None, "4 castor wheels o0.125: 0.058-0.14 m2 depending on the sides, no better estimate", ""),
    ("shopping-trolley", "gate-wire", RF, None, "gate grid of o4 mm wire: ~9.6 m of wire * pi*0.004 = 0.12 m2", "shopping-trolley/gate-wire: the gate is four 25 mm bars (capsule r 0.0125) instead of a o4 mm wire grid"),
    ("cooking-pot", "base", X, None, "heated bottom disc pi*0.086^2 = 0.0232 m2 is the heat transfer area (contact plane with the hob); 0.0052 would make q = 2300 W/m2 for 12 W", ""),
    ("cooking-pot", "handle", A, 0.00496, "two ear handles 20 x 14 mm with rivets: 2*0.0025 = 0.005 m2", ""),
    ("kitchen-extractor-hood", "chimney", A, None, "see appliances", ""),
):
    if (slug, r) not in D:
        d(slug, r, a, v, why, finding)

for key, v in {("duct-fan", "casing"): 0.1, ("electricity-meter", "plate"): 0.0078}.items():
    a, _, w, f = D[key]
    D[key] = (a, v, w, f)

for slug, regions in (("city-bus", ("body-panels", "brakes", "doors", "e-axle", "glazing", "tyres")), ("kneeling-gardener", ("pad", "trowel")), ("london-plane-pollarded", ("trunk-bark",)), ("multi-stem-birch", ("stem-bark",)), ("playground-swing", ("chain-steel",)), ("seated-reader", ("bench", "notebook"))):
    for r in regions:
        d(slug, r, DEFER, None, "no volumes in the file at review")
for slug, r, why in (
    ("delivery-van", "body-panels", "sides 2*5.34*1.1 less slider 2.4 = 9.3 + front 2.0 + wheel arches 2.0 + bumper 1.0 = 14.3 m2 without the underbody (8 m2 more only if the loft closes its bottom)"),
    ("delivery-van", "brakes", "4 discs and calipers 0.18 each = 0.72 m2"),
    ("delivery-van", "cab-glazing", "windscreen 1.5*0.9 = 1.35 + 2 door glasses 1.2 + quarter lights 0.5 = 3.1 m2"),
    ("delivery-van", "rear-doors", "two leaves 1.35 x 0.9 outer face 2.43 + edges 0.15 = 2.6 m2"),
    ("delivery-van", "roof", "(5.34 - 1.4 bonnet - 0.2) * 1.95 = 7.2 m2"),
    ("delivery-van", "slider-door", "panel 1.05 x 1.444 = 1.52 + edges 0.2 + tracks 0.3 = 2.0-2.3 m2"),
    ("delivery-van", "tyres", "4 tyres 235/65 R16 o0.69 x 0.235: tread 0.51 + sidewalls 0.5 = 1.0 each = 4.0 m2"),
    ("drinking-fountain", "bubbler-brass", "bubbler head o0.06 x 0.1 + neck = 0.045 m2"),
    ("drinking-fountain", "water-jet", "jet o0.012 x 0.18 arc = 0.007 m2"),
    ("market-parasol", "base-concrete", "slab 0.5 x 0.5 x 0.06 without the bottom: 0.25 + 4*0.5*0.06 = 0.37 less the pole socket; model 0.274 (-26 %)"),
    ("patio-heater", "base-housing", "housing o0.3 x 0.5 + base disc = 0.9-1.0 m2"),
    ("patio-heater", "control", "knob and panel 0.005 m2"),
    ("patio-heater", "reflector", "dish o0.8: pi*0.4^2 = 0.5 x 1.2 curvature = 0.6-0.68 m2"),
    ("person-on-crutches", "crutch-cuffs", "2 cuffs o0.1 x 0.1 high = 0.03 each = 0.06 m2"),
    ("person-on-crutches", "crutch-tips", "2 rubber tips o0.03 x 0.05 = 0.0045 each = 0.009 m2"),
    ("seated-office-worker", "castor-rubber", "5 castor wheels o0.05 x 0.02 = 0.025 m2"),
    ("seated-office-worker", "chair-steel", "column, stems and mechanism 0.11-0.13 m2"),
    ("seated-office-worker", "laptop", "lid 0.33 x 0.23 = 0.076 + base faces and edges = 0.23-0.3 m2"),
    ("steel-column", "cap-plate", "plate 0.3 x 0.26 top 0.078 + edges 0.03 + bolts = 0.12 m2"),
    ("stepladder", "feet", "4 rubber feet = 0.03 m2"),
    ("stepladder", "platform", "top platform 0.3 x 0.3 minus frame = 0.2 m2"),
    ("stepladder", "rear-frame", "2 rails 1.5 x 0.06 x 4 faces + rungs = 0.5 m2"),
    ("stepladder", "spreaders", "2 spreader bars both faces = 0.074 m2"),
    ("stepladder", "treads", "3 treads 0.4 x 0.12 both faces + edges = 0.39 m2"),
    ("tied-canopy", "canopy-steel", "sheet 2.5 x 1.2 both faces 2.97 m2"),
    ("tied-canopy", "rod-steel", "tie rods o0.012 x 29 m = 1.09 m2"),
):
    d(slug, r, A, None, why)
d("delivery-van", "exhaust-system", RF, None, "pipe 3.5 m o0.06 (0.66) + DPF can o0.18 x 0.45 (0.3) + silencer (0.3) = 1.26 m2", "delivery-van/exhaust-system: hidden region without any solid in thermal (0.8 m2 declared, none modelled)")

for slug, regions in (("city-bicycle", ("saddle", "steer-assembly")), ("scots-pine", ("bark-upper", "limb-bark")), ("tower-crane", ("ballast", "skip"))):
    for r in regions:
        d(slug, r, DEFER, None, "no volumes in the file at review")

d("scots-pine", "bark-lower", DEFER, None, "no volumes in the file at review")
d("street-lamp", "arm-alu", A, None, "arm o0.06 x 1.2 m = 0.226 m2")
d("street-lamp", "lamp-housing", A, None, "head 0.4 x 0.2 x 0.06: top 0.08 + sides 0.07 = 0.15-0.2 m2; q 40 W/0.19 = 210 W/m2")
d("street-lamp", "lamp-lens", A, None, "lens 0.27 x 0.2 plan x 1.1 curvature = 0.06 m2")
d("office-swivel-chair", "castor-rubber", A, 0.023, "5 castor wheels o0.05: 0.007 each, 0.02-0.035 m2")
d("office-swivel-chair", "chair-steel", A, 0.145, "column, 5 stems, mechanism, struts, hubs: 0.11-0.15 m2")

d("tealight", "cup", A, None, "aluminium cup o0.038 x 0.016: side 0.0019 + rim = 0.002 m2 (base on the surface excluded)")

for slug, r, why in (
    ("panel-radiator", "top-grille", "grille 1.0 x 0.135 = 0.135 m2"),
    ("vintage-car-1929", "body-panels", "tub sides 2.6 + rear 0.8 + cabin sides 2.2 + front 0.7 + cabin rear 1.0 + scuttle 0.15 = 7.5-8.2 m2"),
    ("vintage-car-1929", "engine-bay", "bonnet sides and top of the engine compartment 1.1 m2"),
    ("vintage-car-1929", "glazing", "windscreen 0.9 + rear 0.3 + side lights 0.8 = 2.0 m2"),
    ("vintage-car-1929", "roof", "fabric roof 1.5 x 1.4 x 1.2 curvature = 2.5-3.1 m2"),
    ("vintage-car-1929", "tyres", "5 tyres (4 + spare) o0.76 x 0.15: 0.6 each = 3.0 m2"),
):
    d(slug, r, A, None, why)

d("city-car", "body-panels", S, 13.0, "C25: underside no longer marked; sides 5.5 + ends 2.2 (the declared 8.2 is these) + underbody 3.9*1.55*0.8 = 4.8 -> 13.0 m2")
d("delivery-van", "body-panels", S, 22.0, "C25: underside no longer marked; sides+front+arches+bumper 14.3 + underbody 5.34*1.8*0.85 = 8.2 -> 22 m2 (the committed value)")
d("height-adjustable-desk", "top-board", S, 2.68, "C25: underside no longer marked; board 1.6 x 0.8 both faces 2.56 + edges 0.12 = 2.68 m2 (solver is one face per region at faces=1, both faces give the right Watts; geometry plate-up is a note: the underside convects as plate-down)")
d("dining-table", "table-top", S, 3.0, "C25: underside no longer marked; top 1.6 x 0.9 both faces 2.88 + edges 0.15 = 3.0 m2")
d("person-with-umbrella", "canopy", S, 2.1, "C25: underside no longer marked; dome r0.52 rise 0.25 both faces 2*pi*(0.27+0.0625)... = 2*1.04 + rim = 2.1 m2")

d("city-bus", "body-panels", S, 62.0, "C25: underside no longer marked; sides 2*12*2.7 = 64.8 less glazing 33.2 and doors 5.3 = 26.3 + ends 13.8 less glass 4 = 9.8 + underbody 12*2.55*0.85 = 26 -> 62 m2")
d("city-bus", "glazing", A, None, "side glazing 2*12*1.2*0.8 = 23 + windscreen 3.5 + rear 1.5 + door glass 5 = 33 m2")
d("city-bus", "doors", A, None, "two double doors, 4 leaves 2.0 x 1.0 less glass = 5.3 m2")
d("city-bus", "tyres", A, None, "6 tyres 275/70 R22.5 o0.96 x 0.275: tread 0.83 + sidewalls 0.9 less contact = 8.3-10 m2")
d("city-bus", "brakes", RF, None, "4 discs o0.43 both faces 0.29 = 1.2 m2", "city-bus/brakes: the brake solids expose 0.8 m2 against 1.2 m2 of four o0.43 discs with calipers")
d("city-bus", "e-axle", A, None, "axle housing o0.4 x 1.3 + motor housings = 2-3 m2")
d("playground-swing", "chain-steel", A, None, "2 chains o0.011 x 1.83 + shackles = 0.13 m2")
d("seated-reader", "notebook", A, None, "closed notebook 0.2 x 0.15 x 0.03: 2*(0.03+0.006+0.0045) = 0.08 m2")
d("seated-reader", "bench", A, None, "seat + back + 2 legs 1.1 m2")
d("tower-crane", "ballast", A, None, "2x2 blocks 3.5 x 3.5 x 2.0: sides 28 + top 12 = 40 less joints = 36.5 m2")
d("tower-crane", "counterweight", A, None, "3 slabs stacked 2.4 x 2.2 x 2.4: sides 21 + top 5.8 + underside 5.8 = 32.6 m2")
d("tower-crane", "house", A, None, "slew platform 5.8 x 0.7 x 2.4: top 13.9 + sides 8.1 + ends 3.4 + open underside 13.9 less tower foot = 32 m2")
d("tower-crane", "winch-house", S, 31.5, "box 4 x 1.8 x 1.385 outer faces 30.5 + vent stack 1.0 (heat leaves through the outer face; the model counts the 7.5 mm plates on both faces, 53 m2)")
d("tower-crane", "skip", A, None, "skip body 2*0.8 + block 1.2 + hook + 4 slings = 8-9.4 m2")
d("city-bicycle", "mudguards", DEFER, None, "no volumes in the file at review")
d("city-bicycle", "saddle", DEFER, None, "no volumes in the file at review")
d("multi-stem-birch", "branch-bark", DEFER, None, "no volumes in the file at review")

d("waste-bin", "bin-hdpe", A, None, "240 l bin: 4 sides 4*0.5*0.9 less the base = 1.7 m2")
d("waste-bin", "lid-hdpe", A, None, "lid 0.58 x 0.58 both faces edges = 0.38 m2")

d("glulam-beam", "bearing-steel", A, None, "steel bearing plates, exposed faces 0.06 m2 (plates 0.2 x 0.15 both faces and edges, one per end)")
d("waste-bin", "liner-film", A, None, "liner film 0.111 m2: folded film 0.3 x 0.18 both faces plus the collar")

d("miscanthus-grass", "culms", DEFER, None, "no volumes in the file at review")
d("tram", "body-panels", A, None, "sides 2*15.45*2.6 = 80 less glazing 33 and doors 9 = 38 + cab front/ends 8 + underbody 15*2.3*0.8 = 27.6 + bellows 5 = 72-80 m2 (underside counted, C25)")
d("tram", "glazing", A, None, "side glazing 2*15*1.0*0.9 = 27 + windscreen 3.2 + end glass 2.6 = 32.8 m2")
d("tram", "doors", A, None, "plug-sliding door leaves outer faces 8.9 m2")
d("tram", "brake-discs", A, None, "disc brakes 8 discs o0.6: 0.24 each = 1.9 m2")
d("tram", "hvac-unit", A, None, "roof unit 2.8 x 1.2 x 0.6: top 3.4 + sides 4.4 + ends 1.4 = 9 m2")

d("cantilever-balcony", "thermal-break", A, None, "break plate strip 0.17 m2 (width x length, both faces within 15 % of the declared 0.2)")
d("oil-pumpjack", "gearbox", A, None, "gearbox 1.5 x 1.2 x 1.2: 2*(1.8+1.8+1.44) = 10 less the base 1.8 = 8.2-8.7 m2")
d("oil-pumpjack", "steel", A, None, "walking beam, samson post, pitman arms, cranks: the model 44 m2 is within 16 % of the committed 38")

d("person-with-white-cane", "sunglasses", A, None, "two lenses 0.05 x 0.04 outer faces 0.004 + frame = 0.0046 m2")

d("toaster", "bread", A, None, "two bread slices 0.1 x 0.09 visible faces and crust 0.054 m2")

for slug, r, v, why in (
    ("bar-stool", "seat-plate", 0.216, "C29 total: plywood disc o0.36 both faces 2*0.102 + rim 0.0136 (the top is under the pad of another region, still counted)"),
    ("duct-fan", "casing", 0.12, "C29 total: front 0.028 + back 0.028 (on the wall) + ring inner wall 0.03 + sides 0.018 + lip 0.01"),
    ("electricity-meter", "plate", 0.0135, "C29 total: name plate 0.13 x 0.04 x 0.0075 both faces 2*0.0052 + rim 0.0026 = 0.013"),
    ("gas-meter", "shell", 0.3, "C29 total: box 0.24 x 0.27 x 0.17 all faces = 0.30 (back, index and pipe footprints included)"),
    ("ivy-on-trellis", "post-shoes", 0.33, "C29 total: 2 shoes 0.11 x 0.28 x 0.13 all faces = 0.33"),
    ("ivy-on-trellis", "trellis-timber", 3.6, "C29 total: posts 2*0.38*2.1 = 1.6 (including the part in the shoes) + rails 1.0 + battens 1.0"),
    ("office-swivel-chair", "chair-fabric", 1.0, "C29 total: seat cushion box 2*(0.223+0.03+0.03) = 0.57 + back pad both faces 0.45"),
    ("office-swivel-chair", "chair-plastic", 1.1, "C29 total: seat shell both faces 0.36 + backrest shell both faces 0.36 + 5 base arms 0.2 + arms and hub 0.2"),
    ("operating-table", "table-frame", 1.76, "C29 total: carrier 1.96 x 0.4 both faces 1.57 + edges 0.19"),
    ("operating-table", "table-pad", 2.4, "C29 total: 4 pad sections 1.988 x 0.5 top 0.994 + underside 0.994 + sides 0.318 + ends 0.08"),
    ("parent-with-stroller", "tyres", 0.4, "C29 total: 4 wheels o0.25 x 0.05, 0.1 each incl. the contact patch"),
    ("park-bench", "seat-timber", 2.4, "C29 total: 4 slats 1.8 x 0.11875 x 0.045: 0.6 each incl. bearer contacts"),
    ("picnic-table", "hinges", 0.22, "C29 total: 4 blocks 0.08 x 0.1 x 0.11 all faces 0.056 each"),
    ("skateboarder", "trucks", 0.105, "C29 total: 2 boxes 0.075 x 0.05 x 0.18 all faces"),
    ("skateboarder", "wheels", 0.039, "C29 total: 4 wheels o0.054 x 0.03 0.0097 each"),
    ("sleeping-person", "pillow", 0.58, "C29 total: pillow 0.6 x 0.4 top 0.24 + underside 0.24 + rim 0.1 (head and duvet cover it, the pose exposedFraction 0.7 handles that)"),
    ("split-ac-indoor-unit", "casing", 0.73, "C29 total: 0.85 x 0.29 x 0.21 box all faces 0.73 (back on the wall, grille and outlet overlap included)"),
    ("three-seat-sofa", "back-cushions", 3.0, "C29 total: 3 cushions 0.6 x 0.55 x 0.15 all faces 1.0 each"),
    ("three-seat-sofa", "base-rail", 3.65, "C29 total: skirt 1.7 x 0.86 x 0.14 top 1.46 + underside 1.46 + sides 0.72"),
    ("three-seat-sofa", "seat-cushions", 3.6, "C29 total: 3 boxes 0.5433 x 0.23 x 0.62 all faces 1.21 each"),
    ("three-seat-sofa", "arm-pads", 3.0, "C29 total: 2 pads 0.2 x 0.52 x 0.9 all faces 1.5 each"),
    ("walking-commuter", "bag-leather", 0.263, "C29 total: box 0.209 x 0.294 x 0.14 all faces"),
    ("worker-carrying-box", "box-surface", 0.66, "C29 total: box 0.4 x 0.3 x 0.3 all faces (the hands cover part, the pose contact handles it)"),
    ("wheelchair-user", "cushion", 0.63, "C29 total: seat cushion both faces 0.32 + rim 0.06 + back sling both faces 0.25"),
    ("electric-kettle", "plate", 0.044, "C29 total: power base o0.18: side 0.011 + top ring 0.0078 + underside 0.0254 (heatW 40 W stays: q = 909 W/m2)"),
    ("toaster", "feet", 0.007, "C29 total: 4 feet o0.025 x 0.01: side 0.0031 + bottom 0.002 + top 0.002"),
    ("electric-fan-heater", "feet", 0.0085, "C29 total: 4 feet with bottom and top faces, 1.6 x the exposed 0.0053"),
    ("shower-head", "valve", 0.025, "C29 total: body 0.062 x 0.082 x 0.062 all faces incl. the wall face + stubs"),
):
    d(slug, r, S, v, why)

d("child-on-kick-scooter", "backpack", S, 0.28, "C29 total: pack 0.25 x 0.3 x 0.12 all faces 2*(0.075+0.03+0.036) = 0.28 (the back is on the child, still counted)")
d("child-on-kick-scooter", "stem", A, None, "stem tube o0.03 x 0.8 + handlebar + grips = 0.15 m2")
d("child-on-kick-scooter", "wheels", A, None, "2 wheels o0.12 x 0.03: tread 0.011 + sides 0.02 = 0.03 each = 0.06 m2")
d("person-with-rollator", "bag", S, 0.33, "C29 total: tote 0.3 x 0.32 x 0.1 all faces = 0.32 m2")
d("person-with-rollator", "seat-basket", A, None, "seat plate 0.4 x 0.3 both faces 0.24 + wire basket 0.4 x 0.3 x 0.25 all faces 0.6 + frame = 0.85-1.05 m2")

d("person-with-rollator", "tyres", A, None, "C29 total: 4 wheels o0.2 x 0.03: tread 0.019 + sides 0.01 = 0.029 each = 0.116 m2")
d("vacuum-cleaner", "body-shell", RF, None, "C29 total: canister 0.3 x 0.25 x 0.3 all faces 2*(0.075+0.09+0.075) = 0.48 incl. the floor and hose faces; the committed 0.42", "vacuum-cleaner/body-shell: measured exposed 0.31 m2 leaves out faces covered by hose and wand sockets and the floor side (C29: total counts them)")
d("vacuum-cleaner", "hose", A, None, "C29 total: hose o0.032 x 1.2 m = 0.12 m2")
d("vacuum-cleaner", "wand", A, None, "C29 total: tube o0.03 x 0.6 m + nozzle head = 0.066-0.09 m2")

d("child-on-kick-scooter", "wheels", A, 0.082, "C29 total: 2 wheels o0.12 x 0.03 with the tread and both sides incl. the hub caps = 0.07-0.09 m2")
d("courier-with-hand-truck", "hand-truck", A, None, "C29 total: frame tubes 2 x 1.1 m o0.03 + toe plate 0.3 x 0.2 both faces + wheels = 0.6-0.7 m2")
