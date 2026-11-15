"""Reviewer 3 decisions (humans, animals, vegetation, appliances, furniture; ruling C29 and the rule at the top of the Reviewer 2 section of area-decisions.md).
Key (slug, region) -> (act, value, reason). act: A accept the value in the file, S set value, X mark areaBasis exchange (value stays), SX set + exchange, D defer.
BODY: C27 redistributions, (slug, region, new value, old value) of living subjects; applied by areareview3-apply.py like S rows. FIND: (slug, text) findings for model-findings.md."""
A, S, X, SX, D = "A", "S", "X", "SX", "D"
R = {}
FIND = []
BODY = []
POSES = []
def d(slug, region, act, value, why):
    R[(slug, region)] = (act, value, why)

d("japanese-maple", "stem-bark", A, None, "3 stems o0.12 tapering to o0.07 over 2.3 m: 3 x pi x 0.095 x 2.3 = 2.0 + root collar 0.3 = 2.3; model 2.6 (13 %), within the 15 % of the validator")
d("japanese-maple", "limb-bark", S, 1.55, "6 limbs 1.2 m, o0.09 to o0.04: 6 x pi x 0.065 x 1.2 = 1.47; model 1.72 (analytic 1.71); 1.55 is within 15 % of both")
d("london-plane-pollarded", "pollard-bark", S, 7.2, "pollard head o1.1 x 1.2 m (4.1) + 5 knuckles o0.35 x 0.6 (3.3) = 7.4; the committed 4.13 counted the head without the knuckles; model 7.15")
d("london-plane-pollarded", "trunk-bark", S, 11.2, "clear stem 4.2 m, mean o0.9 with the flare: pi x 0.9 x 4.2 = 11.9, 10.5-11.9; model 11.24")
d("london-plane-pollarded", "shoot-bark", A, None, "shoot crown of ~60 water shoots o0.04 x 0.5 m = 3.8; model 4.4 (20 %), a weighting-only passive region")
d("multi-stem-birch", "stem-bark", A, None, "3 stems o0.28 tapering to o0.12, mean o0.2 x 9 m = 5.6 each = 17 + stool 0.8 = 17.8; model 18.8")
d("multi-stem-birch", "branch-bark", A, None, "6 branches o0.06 x 2.4 m mean = 0.45 each = 2.7; model 2.62")
d("scots-pine", "bark-lower", S, 15.5, "trunk 0-8 m o0.55 tapering to o0.5: pi x 0.525 x 8 = 13.2 + root flare 1.3 + stubs 0.5 = 15.0; model 16.8-17.2 (the 3-section loft bulges); the author raised 14.42 to 17.2")
d("scots-pine", "bark-upper", S, 12.0, "trunk 8-18 m o0.4 to o0.2: pi x 0.3 x 10 = 9.4 + leader 2.0 = 11.4; model 11.7-13.5; the author raised 11.38 to 13.5")
d("scots-pine", "limb-bark", S, 13.0, "6 limbs: 2 x (o0.4 mean 0.2 x 4.5 m = 2.8) + 2 x (0.15 x 4 m = 1.9) + 2 x (0.1 x 4 m = 1.3) = 11.9, with the base swelling 13; model 15.1 (limbs drawn o0.25 at the base)")
d("miscanthus-grass", "culms", A, None, "~45 culms o0.004 x 1.0 m mean = 0.57 + tuft base 0.2 = 0.6-0.7; model 0.70 (16 %)")
d("english-oak", "trunk-bark", S, 25.0, "stem 0-4.5 m mean o1.1 = 15.5 + flare 2.5 + upper trunk 4.5-7 m o0.7 = 5.5 + leader 2 = 25.5; committed 16 omitted the upper trunk and the leader; model analytic 25.0")
d("english-oak", "limb-bark", A, None, "limb wood 3 m3 at mean r0.17 gives 2/r x V = 35-42 m2; the model limbs (12 solids, 66 m2) are drawn up to r0.37 over 5.7 m, 1.6x heavier than the mass and crown allow: model defect, committed 42 stays")
d("hornbeam-hedge", "stem-bark", S, 1.0, "hedge 0.9 m x 2.2 m, 9 stems per 3 m: 27 stems o0.025 x 1.0 m = 2.1 under the foliage in the model, 0.8 committed; 1.0 is the midpoint of hand 0.8 and model 1.19")
d("kneeling-gardener", "pad", S, 0.70, "foam pad 0.76 x 0.4 x 0.04: 2 x (0.304 + 0.0304 + 0.016) = 0.70 (both faces count, the ground face is contact, C29)")
d("kneeling-gardener", "trowel", S, 0.041, "blade 0.16 x 0.07 tapered 0.0085 x 2 faces = 0.017 + shank 0.006 + handle o0.03 x 0.11 = 0.0104 + ferrule = 0.039-0.045; model 0.0448")
d("kneeling-gardener", "trug", S, 0.86, "boat basket o0.5 x 0.21 deep, open: outside 0.43 + inside 0.43 = 0.86 (both faces of a free shell)")
d("kneeling-gardener", "hat", A, None, "C18: clothing of a living subject. One-sided sky face: brim annulus (r0.235, r0.09) 0.148 + crown 0.061 + band 0.009 = 0.213 = 0.226 (6 %); the real hat has 0.38 with the brim underside; the underside faces the shaded hair pack and counts as the head side, so the physiological value stays, the model measures 0.398 (finding)")
d("child-on-kick-scooter", "deck", A, None, "deck 0.59 x 0.13 x 0.02 plate: 2 x 0.077 + edges 0.029 = 0.18 + fender 0.048 + brake pad 0.011 + stem foot = 0.262; model 0.2624")
d("child-on-kick-scooter", "grips", A, None, "2 grips o0.03 x 0.087: 2 x (0.0082 + 0.0014 caps) = 0.0192, with the bar ends 0.022; model 0.0216")
d("cyclist", "bike-metal", A, None, "frame tubes 3.1 m o0.034 = 0.33, fork/stays 0.2, bars and grips 0.1, fenders 0.29, rims 0.147, chain and rings 0.1, cranks and pedals 0.1 = 1.27; model 1.19 (13 %), committed 1.05 within 20 %")
d("cyclist", "tyres", A, None, "2 tyres 700 x 38: torus (2 pi x 0.33)(2 pi x 0.019) = 0.25 each = 0.5-0.6; model 0.6005")
d("cyclist", "saddle", A, None, "saddle 0.27 x 0.15 x 0.05 closed shell = 0.07; model 0.072")
d("parent-with-stroller", "stroller-fabric", A, None, "seat sling 0.47 x 0.85 both faces 0.8 + hood 0.45 + basket 0.38 x 0.4 x 0.1 both faces 0.46 = 1.7-1.9; the author's 2.0 stands within 5 % of the model 1.91")
d("parent-with-stroller", "stroller-frame", A, None, "4.3 m of tube o0.025 = 0.34 + footrest 0.1 + hangers 0.05 = 0.5; the author's 0.5 stands")
d("person-on-crutches", "crutch-cuffs", S, 0.10, "2 open cuffs o0.1 x 0.1 high, 75 % closed, both faces: 2 x 2 x 0.0236 = 0.094 + edges 0.01 = 0.10; model 0.109 (the 0.06 counted one face)")
d("person-on-crutches", "crutch-tips", S, 0.014, "2 rubber tips o0.03 x 0.05: side 0.0047 + ends 0.0014 = 0.0061 each = 0.012-0.014; model 0.018")
d("person-with-rollator", "tyres", S, 0.21, "4 wheels o0.2 x 0.03: tread 0.019 + 2 sidewall annuli pi x (0.1^2 - 0.07^2) = 0.016 each = 0.051 per wheel = 0.205 (the 0.029 per wheel counted a narrow sidewall ring)")
d("person-with-umbrella", "umbrella-frame", S, 0.14, "shaft o0.0176 x 0.95 = 0.0525 + J handle o0.033 x 0.2 = 0.021 + hub 0.013 + 8 ribs o0.005 x 0.56 = 0.075 + ferrule 0.004 = 0.165; the model draws two ribs (analytic 0.138): 0.14 is the model value, hand 0.165")
d("person-with-white-cane", "sunglasses", S, 0.0098, "two lenses 0.05 x 0.04 elliptical both faces 0.0063 + edges 0.0013 + bridge and arm stubs 0.0022 = 0.0098; the committed 0.0046 was the outer lens faces only")
d("seated-office-worker", "castor-rubber", S, 0.036, "5 castor wheels o0.05 x 0.02: tread 0.0031 + 2 sides 0.0039 = 0.007 each = 0.035")
d("seated-office-worker", "chair-steel", S, 0.2, "column sleeve o0.068 x 0.28 0.06 + column 0.02 + collar and cone 0.024 + mechanism box 0.2 x 0.042 x 0.14 0.085 + backrest strut 0.03 = 0.22; model measured 0.19")
d("seated-office-worker", "chair-plastic", S, 0.85, "backrest shell 0.4 x 0.45 both faces 0.4 + 5 star arms o0.03 x 0.28 0.13 + hub 0.03 + 2 armrests (pad 0.25 x 0.06 both faces + post) 0.12 = 0.7-0.9; model 0.90; the 0.35 was one face of the backrest and the arms")
d("seated-office-worker", "laptop", S, 0.39, "15 inch laptop open: base 0.355 x 0.25 x 0.019 closed 0.20 + lid 0.355 x 0.25 x 0.009 closed 0.19 = 0.39 (the underside on the table counts, C29); heatW 60 W stays, q = 154 W/m2 (committed 0.233 gave 257)")
d("kitchen-extractor-hood", "filter", S, 0.84, "grease filter 0.86 x 0.46 x 0.02: 2 x 0.396 + edges 0.05 = 0.84 (the upper face inside the hood counts, C29); reviewer 1's own hand figure of one face 0.45 was kept at 0.2")
d("kitchen-extractor-hood", "chimney", S, 0.78, "chimney 0.3 x 0.2 telescopic, 0.7 m: 1.0 x 0.7 = 0.70 + collar 0.064 = 0.76 (the wall side counts); model 0.846 with the extract duct")
d("ceiling-diffuser", "plate", A, None, "R-B: room-facing face 0.595^2 - 0.01 = 0.35 + cones 0.07 + edges 0.03 = 0.45; the back flange face lies against the ceiling void (C25 b): the model's 0.757 counts it because the flange box has no `sectionFaces`: finding")
d("electric-kettle", "plate", S, 0.065, "power base o0.18 x 0.03: side 0.017 + top 0.0254 + underside 0.0254 = 0.068; model 0.0648 (the top under the kettle counts, C29)")
d("electric-kettle", "shell", S, 0.15, "body o0.16 x 0.2: lateral 0.10 + bottom 0.02 + spout 0.01 + handle and rim 0.02 = 0.15; model 0.166")
d("electric-kettle", "water", SX, 0.0155, "evaporating free surface pi x 0.0705^2 = 0.0156: the boil flux acts there, the water body is enclosed (its 0.088 is no outer face): exchange area (C19)")
d("electric-kettle", "lid", A, None, "dome o0.14 outer face 0.0185 + button 0.004 + rim 0.003 = 0.026; the inner face is enclosed (R-A); the model's closed solid counts it (0.034), finding stays")
d("electricity-meter", "window", A, None, "window and counter glass: pane o0.092 outer face 0.0066 + counter 0.0023 + rim 0.003 = 0.012; the inner faces face the sealed meter; the model closed solid counts both faces (0.026)")
d("gas-hob", "knob", A, None, "two knobs (0.3 m hob) o0.022 x 0.009: 2 x (0.00038 + 0.00062) = 0.002-0.0028; the model draws one knob (0.0014 incl. back face)")
d("gas-meter", "index", S, 0.05, "index plate 0.19 x 0.098 x 0.008 closed 0.042 (its back on the meter body counts) + drums 0.008 = 0.05")
d("gas-meter", "pipes", S, 0.02, "2 stubs o0.028 x 0.07 0.012 + 2 unions o0.04 x 0.018 0.0045 + ends = 0.02")
d("hot-water-cylinder", "zone-1", A, None, "side of a 0.33 m slice of o0.6: pi x 0.6 x 0.33 = 0.62; the end faces between the stacked slices are interfaces of one cylinder (C25 a), the model counts them (1.19): finding")
d("hot-water-cylinder", "zone-2", A, None, "side of a 0.33 m slice of o0.6 = 0.62 (interfaces are no outer faces)")
d("hot-water-cylinder", "zone-3", A, None, "side of a 0.33 m slice of o0.6 = 0.62 (interfaces are no outer faces)")
d("hot-water-cylinder", "zone-4", A, None, "side of a 0.33 m slice of o0.6 = 0.62 (interfaces are no outer faces)")
d("hot-water-cylinder", "top-cap", A, None, "dome o0.6 outer face 0.283 x 1.1 + rim = 0.31; the face to zone-4 is an interface (finding)")
d("hot-water-cylinder", "connections", S, 0.03, "5 stubs o0.022 x 0.05 0.017 + 4 hex nuts 0.0029 each + ends = 0.03")
d("mvhr-unit", "casing", S, 1.16, "box 0.6 x 0.665 x 0.3: 2 x (0.18 + 0.2 + 0.2) = 1.16 (the wall face counts, heat crosses every face of the casing)")
d("mvhr-unit", "ducts", S, 0.10, "4 open spigots o0.098 x 0.062 outer 0.019 + bead 0.006 = 0.025 each = 0.10; the model closed solids (0.167) count the open ends")
d("room-thermostat", "led", A, None, "half dome a3.5 mm c2 mm 5.6e-5 m2; heat crosses the dome, the base is the source side")
d("room-thermostat", "dial", A, None, "knob o0.042 x 0.01: side 0.0013 + front 0.0014 = 0.0027; model 0.0016-0.0027")
d("server-rack", "feet", A, None, "4 castors o0.07 x 0.03 wheel: tread 0.0066 + sides 0.0077 = 0.014 each = 0.058; the model draws them as o0.032 posts (0.0246): model defect")

d("toaster", "top-plate", A, None, "R-B: top 0.3 x 0.17 less slots = 0.051 + edges = 0.055; the underside faces the hot interior (source side); the model's closed solid counts both faces (0.079)")
d("toaster", "control", S, 0.0044, "lever 0.03 x 0.024 x 0.008 closed 0.0023 + dial o0.03 x 0.008 0.0021 = 0.0044 (the backs on the housing count, C29)")
d("toaster", "feet", A, None, "4 rubber feet o0.025 x 0.01: side 0.0008 + 2 x 0.00049 = 0.0018 each = 0.007; the model draws them o0.05 (4x the area, 0.020): model defect")
d("tower-server", "feet", S, 0.016, "4 pads 0.04 x 0.03 x 0.012 closed 0.0041 each = 0.0164 (the ground face counts, C29); the model's `-v` marking removes the ground faces (0.010)")
d("vacuum-cleaner", "wheels", S, 0.068, "2 wheels o0.12 x 0.03: tread 0.0113 + 2 sides 0.0113 each = 0.034 each = 0.068")
d("vacuum-cleaner", "wand", S, 0.088, "tube o0.036 x 0.78 = 0.088 + ends 0.002; the 0.066 was the shorter retracted tube")
d("vacuum-cleaner", "grip", S, 0.025, "sleeve o0.035 x 0.15 = 0.0165 + ends and bore faces 0.008; model 0.0297")
d("bar-stool", "seat-pad", S, 0.25, "pad o0.36 x 0.04: top 0.102 + side 0.045 + underside on the plate 0.102 = 0.249 (C29: covered faces count)")
d("bar-stool", "seat-plate", A, None, "plywood disc o0.34 x 0.032: 2 x 0.091 + rim 0.034 = 0.216 (reviewer 1 stands); the model's `+u` on the top hides the face against the pad, which is another region and counts (C29): finding")
d("bookshelf", "plinth", A, None, "kick board 0.772 x 0.075 x 0.018: 2 x 0.058 + top and bottom 0.028 + ends 0.003 = 0.147; the model is a solid 0.25 m deep block (0.539): model defect")
d("dining-table", "leaf-top", S, 1.14, "stored leaf 0.6 x 0.88 x 0.03: 2 x 0.528 + edges 0.09 = 1.14 (both faces count, the pose carries the stowing: exposedFraction)")
d("dining-table", "apron", S, 0.92, "aprons 2 x (1.4 + 0.7) = 4.2 m x 0.08 high x 2 faces 0.67 + lower edge 0.084 + upper edge on the top 0.084 + corner blocks 0.08 = 0.92; model 1.01")
d("dining-table", "table-top", A, None, "1.6 x 0.9 both faces 2.88 + edges 0.15 = 3.03 (reviewer 1 stands, model 3.03)")
d("height-adjustable-desk", "steel-desk", A, None, "2 foot plates 2 x 0.6 + 2 telescope stages 0.40 + 2 sleeves 0.38 + 2 columns 0.29 + top rail 0.41 + cable tray 0.18 + brackets 0.06 = 3.0 solid sum; interfaces between sleeves, stages and columns bury 0.8 of it: 2.15-2.4 (reviewer 1 stands)")
d("height-adjustable-desk", "drive-left", S, 0.075, "drive housing 0.1 x 0.148 x 0.1 closed 0.079; heatW 35 W gives q = 470 W/m2 (committed 0.05 gave 700)")
d("height-adjustable-desk", "drive-right", S, 0.075, "drive housing 0.1 x 0.148 x 0.1 closed 0.079")
d("office-swivel-chair", "chair-steel", S, 0.22, "sleeve 0.059 + column 0.02 + collar and cone 0.025 + mechanism box 0.2 x 0.2 x 0.042 0.114 + strut 0.03 = 0.25; model 0.22")
d("office-swivel-chair", "castor-rubber", S, 0.035, "5 wheels o0.05 x 0.025: tread 0.0039 + 2 sides 0.0039 = 0.0079 each = 0.039; model 0.033")
d("operating-table", "table-column", S, 0.9, "column cover 0.21 x 0.21 x 0.34 sides 0.29 + lower section 0.25 + head plate 0.32 x 0.4 x 0.04 closed 0.31 = 0.85-0.9; model 0.913")
d("operating-table", "light-housing", S, 0.43, "dome o0.5 outer face 2 pi 0.25^2 = 0.39 + rim 0.04; the `th-light-plume` ellipsoid (0.26) is a convection plume, not a surface: finding")
d("operating-table", "light-lens", A, None, "lens disc o0.48 room face 0.181 + rim 0.023 = 0.2; the inner face looks into the housing (R-A enclosed)")
d("park-bench", "seat-timber", A, None, "4 slats 1.8 x 0.11875 x 0.045 both faces + edges 0.6 each = 2.4; model 2.40")
d("park-bench", "back-timber", S, 1.3, "3 raked slats 1.8 x (0.08 x 0.05 chamfered, perimeter 0.229) = 0.412 + ends 0.008 = 0.42 each = 1.26 + fixings 0.04")
d("single-bed", "mattress", S, 4.5, "2.0 x 0.9 x 0.21 closed 2 x (1.8 + 0.42 + 0.19) = 4.8 (the model 1.43 x 0.82 core gives 4.3): 4.5; the bottom on the slats counts (C29)")
d("single-bed", "slats", S, 3.0, "21 slats 0.06 x 0.022 x 0.86 closed 0.1436 each = 3.0 (both faces, the rail contact counts)")
d("single-bed", "bed-frame", S, 4.7, "head board 0.95 x 0.9 both faces + edges 1.93 + foot board 0.5 x 0.9 1.0 + 2 rails 2.04 x 0.16 closed 0.83 each = 4.6")
d("electric-fan-heater", "casing", S, 0.33, "housing 0.196 x 0.31 x 0.24 closed 0.365 less the grille and intake openings 0.071 + handle 0.05 = 0.34")
d("electric-fan-heater", "top-cover", A, None, "R-B: top 0.17 x 0.21 = 0.036 + rim 0.015 = 0.051; the underside faces the hot interior; the model's closed solid counts it (0.102)")
d("electric-fan-heater", "grille", A, None, "R-B: perforated plate 0.17 x 0.21 outer face 0.036 + hole walls 0.014 = 0.05; the rear face is the interior")
d("electric-fan-heater", "intake", A, None, "R-A enclosed: plate 0.16 x 0.21 outer face 0.034 + slats 0.012 + edges = 0.05; the inner face looks into the casing")
d("electric-fan-heater", "control", S, 0.0045, "dial o0.04 x 0.015: side 0.0019 + front 0.0013 + back on the casing 0.0013 = 0.0045")
d("electric-fan-heater", "feet", A, None, "4 feet o0.03 x 0.01: side 0.00094 + 2 x 0.0007 = 0.0023 each = 0.009; the model feet are larger (0.0137)")
d("circulation-pump", "casing", S, 0.048, "volute o0.115 x 0.06: side 0.0217 + 2 faces 0.0208 + unions 0.0063 + stubs 0.003 = 0.052; model 0.0466")
d("circulation-pump", "motor", S, 0.034, "motor head o0.09 x 0.075: side 0.0212 + end 0.0064 + back on the casing 0.0064 = 0.034")
d("circulation-pump", "rotor-window", A, None, "pane o0.052: outer face 0.0021 + edge 0.0003 = 0.0024; the inner face looks into the wet rotor chamber (enclosed)")
d("desktop-monitor", "panel-front", A, None, "R-B: front face 0.299 x 0.53 = 0.159 + bezel and edges 0.04 = 0.2; the back faces the interior; the model counts both faces (0.37)")
d("desktop-monitor", "stand", S, 0.16, "base plate 0.25 x 0.21 x 0.01 closed 0.114 (the underside on the desk counts) + neck 0.0375 + hinge 0.006 + feet 0.003 = 0.16")
d("duct-fan", "casing", S, 0.14, "plate 0.22 x 0.22 x 0.012: front less bore 0.017 + back on the wall 0.048 + edges 0.01 + ring wall outer 0.046 + lip 0.02 = 0.14; the model ring is a solid block (0.207)")
d("picnic-table", "a-frame", S, 1.2, "4 leg boards 0.09 x 0.045 x 0.88: both faces 0.158 + edges 0.079 = 0.24 each = 0.95 + cross beams 0.96 and 0.6 m (perimeter 0.18) 0.28 = 1.2-1.5; model 1.18; reviewer 1's 0.998 was the low end")
d("seated-reader", "cup", A, None, "cup body o0.08 x 0.105 side 0.023 + base 0.004 + lid o0.074 top 0.0043 + side 0.0035 (sleeve overlaps the body, same region) = 0.035; model 0.0376")
d("gas-boiler", "casing", A, None, "within 3 % of the model (0.927): box 0.4 x 0.7 x 0.3 outer less the controls panel")
d("pendant-lamp", "shade", A, None, "shade dome o0.3 outer face 0.11, model 0.105")

d("person-cooking", "hob", S, 0.6, "52 x 52 cm hob plate: top 0.27 + underside in the worktop 0.27 + edges 0.027 + burner rings 0.03 = 0.6 (the committed 0.3 was the top face)")
d("person-cooking", "pot", S, 0.2, "R-B: outer wall pi x 0.24 x 0.141 = 0.106 + base 0.045 + lid top 0.045 + handles 0.01 = 0.2; the inside faces the charge (source side); model 0.275 closed solid")
d("person-cooking", "spoon", S, 0.026, "wooden spoon 0.3 m: handle o0.02 x 0.25 0.0157 + bowl 0.012 less the joint = 0.026")
d("person-with-white-cane", "bag", S, 0.17, "shoulder bag 0.25 x 0.2 x 0.08 closed 0.172 (strap folded in the model); model 0.1703")
d("person-with-white-cane", "cane-grip", S, 0.022, "rubber grip o0.025 x 0.2 m 0.0157 + wrist strap 2 x 0.015 x 0.2 = 0.006 = 0.022")
d("running-child", "backpack", S, 0.18, "child pack 0.2 x 0.26 x 0.09 closed 0.19 less the strap overlap; model 0.162")
d("seated-guitarist", "guitar", A, None, "acoustic guitar: top 0.16 + back 0.16 + ribs 1.5 m x 0.095 = 0.14 + neck and head 0.1 = 0.56-0.62; the sound box is enclosed (R-A), the model counts the inner faces of its plates (1.03): finding")
d("wheelchair-user", "frame-metal", S, 1.3, "frame tubes 7 m o0.025 0.55 + 2 rims (o0.6 x 0.07 tube) 0.26 + footplate 0.25 + spokes 0.1 + push rims 0.1 = 1.26; model 1.31; the 0.55 was the frame tubes alone")
d("shower-head", "rail", S, 0.083, "tube o0.022 x 0.69 0.0477 + caps + holder box 0.02 + brackets 0.0125 = 0.083")
d("shower-head", "handle", S, 0.022, "handle o0.035 x 0.18 = 0.0198 + ends 0.002 = 0.022")
d("thermostatic-radiator-valve", "body", S, 0.0095, "body o0.038 x 0.042 0.005 + stub o0.017 x 0.023 0.0012 + nut and ends 0.003 = 0.0095")
d("thermostatic-radiator-valve", "knob", S, 0.005, "knob o0.04 x 0.02: side 0.0025 + front 0.0013 + back on the body 0.0013 = 0.005")
d("leaning-person", "phone-surface", S, 0.025, "phone 0.15 x 0.07 x 0.008: 2 x 0.0105 + edges 0.0035 = 0.0245")
d("leaning-person", "wall-face", A, None, "R-C: conditioned wall sample 2.5 x 1.3 = 3.25 + edges 0.25 = 3.5, outward faces only; the model counts the interior-side face (6.46): its `sectionFaces` is missing: finding")
d("apple-tree", "spur-twig", S, 0.185, "midpoint of the committed 0.12-0.23 hand figure and the model 0.1465: a weighting-only passive region")
d("agave", "caudex", A, None, "cylinder o0.22 x 0.42 = 0.29 (reviewer 1 stands); the extrusion in the model is a rectangular prism (0.38): finding")

d("courier-with-hand-truck", "tyres", A, None, "2 hand-truck wheels o0.26 x 0.05: tread 0.041 + 2 sidewall annuli 0.038 each = 0.12 per wheel = 0.25; model 0.2615")
d("courier-with-hand-truck", "parcels", A, None, "3 parcels closed 2.3 + tape 0.04, less the stacked contact faces 0.35 (faces between solids of one region are not outer) = 2.0; the author re-set the value to 1.79 after this review: within 10 % of the hand figure and of the model 1.805, accepted")
d("person-cooking", "counter", A, None, "cabinet 0.6 x 0.76 x 0.62 sides and fronts 1.85 + worktop top 0.40 and edges 0.15 + plinth 0.49 + handle 0.02 = 2.9; model 2.96")
d("seated-guitarist", "stool", A, None, "seat o0.41 x 0.04: 2 x 0.132 + 0.0515 = 0.316 + 3 legs o0.042 x 0.51 0.22 + rails 0.1 = 0.64; model 0.646")
d("parent-with-stroller", "tyres", A, None, "2 rear o0.32 x 0.04 (0.116 each) + 2 front o0.2 x 0.04 (0.067 each) = 0.36 rubber + hub faces 0.12 modelled in the region = 0.48; model 0.493")
d("weeping-willow", "trunk-bark", S, 14.5, "stem 0-3 m o1.0 = 9.4 + root flare 2 + 2 forks o0.5 x 1.5 m = 4.7 = 16; model 15.05, 13.5 (author) was 10 % under both")
d("weeping-willow", "limb-bark", A, None, "4 main limbs o0.35 x 5 m 5.5 + 4 secondary o0.2 x 5 m 3.1 + 100 pendulous branches o0.04 x 1.4 m 17.6 = 26 less overlap; committed 22 stands within 5 % of the model 22.97")
d("seated-office-worker", "desk", A, None, "top 0.78 x 0.6 x 0.025 closed 1.005 + 2 posts 0.04 x 0.04 x 0.695 0.229 + apron 0.126 + feet 0.029 = 1.39 less the post-to-top contacts 0.04 = 1.35 (author re-set 1.1 to 1.35; model 1.346)")

import json as _json, os as _os
_c27 = _os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "🗑️generated", "areareview3", "c27.json")
if _os.path.exists(_c27):
    for _slug, _v in _json.load(open(_c27)).items():
        for _id, _old, _new, _meas, _floor in _v["rows"]:
            if _new != _old:
                BODY.append((_slug, _id, _new, _old))

BODY += [("horse", "hooves", 0.195, 0.12, 0.03353), ("horse", "coat-trunk", 3.975, 4.05, 0.68321)]

FIND += [
 ("hot-water-cylinder/zone-1..4, top-cap", "the end faces between the stacked slices of one cylinder are interfaces, not outer faces: mark them `sectionFaces` (C25 a); measured 1.18 per zone against the side 0.62, top-cap 0.62 against 0.31"),
 ("ceiling-diffuser/plate", "the back face of the flange box lies against the plenum (C25 b) and carries no `sectionFaces`: measured 0.757 against the room-facing 0.45"),
 ("bar-stool/seat-plate", "`+u` on the plate hides the face against the pad; the pad is another region, so under C29 the face counts: remove the marking (0.124 measured against 0.216)"),
 ("enclosed inner faces (electric-kettle/lid, electricity-meter/window, toaster/top-plate, electric-fan-heater/top-cover, grille, intake, circulation-pump/rotor-window, desktop-monitor/panel-front, operating-table/light-lens, seated-guitarist/guitar, person-cooking/pot, kitchen-extractor-hood/filter)", "the solids are closed, so the face that looks into an enclosed cavity (a sealed casing, the heater interior, the sound box, the pot charge) is counted; R-A and R-B count the outward face only. Either model such a part as a hollow shell (C30) or the coordinator extends C25 to the inner face of an enclosed cavity; until then these regions fail `volumes-thermal-coverage` by 30-90 %"),
 ("leaning-person/wall-face", "conditioned wall: the interior-side face has no `sectionFaces` (R-C), measured 6.46 against the outward 3.5"),
 ("toaster/feet", "feet drawn o0.05 (real o0.025): 4x the area (0.020 against 0.007)"),
 ("tower-server/feet", "`-v` marks the ground faces of the pads; C29 counts the ground contact (0.010 measured, hand 0.016)"),
 ("server-rack/feet", "castors modelled as o0.032 posts instead of o0.07 wheels (0.0246 against 0.058)"),
 ("bookshelf/plinth", "plinth drawn as a solid 0.25 m deep block (0.539 m2) instead of a 18 mm kick board (0.147 m2)"),
 ("operating-table/light-housing", "`th-light-plume` ellipsoid (0.26 m2) is a convection plume, not a surface, and counts in the region"),
 ("english-oak/limb-bark", "limbs drawn up to r0.37 over 5.7 m: 66 m2 measured against 35-42 m2 from the wood volume of the 5971 kg tree"),
 ("scots-pine/limb-bark", "limbs drawn o0.25 at the base: 15.1 m2 against hand 11.9"),
 ("person-with-umbrella/umbrella-frame", "two of eight ribs modelled"),
 ("gas-hob/knob", "one of two knobs modelled"),
 ("kneeling-gardener/hat", "validator measures 0.398 against the physiological 0.226 (brim underside counted); C18 keeps the value, the coordinator decides whether the validator compares clothing of living subjects with the sky-facing surface"),
 ("humans, face and head skin (about 30 assets)", "the face solid covers the whole head including the scalp under the hair (hair is exchange, not measured): measured 1.2-2.2x the face value; compare face+hair"),
 ("humans, skin under garments (jogger, person-on-crutches, runner, sleeping-person, ...)", "the skin solid runs under the garment solid and counts whole (C29: covered faces count) while the declared skin value is the bare part: the subject total measures 0.74-0.98 of the declared total; shorts drawn loose (person-on-crutches 0.567 against 0.197), helmets and caps drawn 1.7-2.5x"),
 ("animals, small skin parts", "every hoof, horn, beak, cere, toe, web and paw of cow, goat, sheep, pig, roe-deer, pigeon, gull, mallard, squirrel, cat and hedgehog is drawn 2-12x the real part (numbers in the C27 table of the Reviewer 3 section); dairy-cow udder 0.355 against 0.45 is the one undersized part"),
]
