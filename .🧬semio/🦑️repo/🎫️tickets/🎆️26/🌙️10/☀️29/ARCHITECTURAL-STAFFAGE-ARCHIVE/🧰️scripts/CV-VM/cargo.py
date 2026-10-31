import sys
from build import *

REL = "🚜️vehicles/cargo-bike"
KEYS = [
    opkey("assist", "Motor assist", "Motorunterstützung", [(0, "Off", "Aus"), (0.3, "Eco", "Eco"), (0.6, "Tour", "Tour"), (1, "Turbo (250 W)", "Turbo (250 W)")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.5, "About 12 km/h", "Etwa 12 km/h"), (1, "About 25 km/h", "Etwa 25 km/h")]),
    opkey("braking", "Braking", "Bremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Light braking", "Leichte Bremsung"), (1, "Full stop from 25 km/h", "Vollbremsung aus 25 km/h")]),
]
tube = lambda: layer("tube", "Aluminium tube 2 mm", "Aluminiumrohr 2 mm", "metal", "aluminium", 0.002, 0.002 / 160, 3000, source="literature", ref="incropera-2011")
REGIONS = [
    region("frame", "Aluminium frame tubes", "Aluminium-Rahmenrohre", "surface", "passive", "aluminium", 0.9, 0.03, "cylinder", 0.6, 3000, [tube()]),
    region("box-wood", "Plywood cargo box", "Sperrholz-Transportkasten", "surface", "passive", "plywood", 3.2, 0.8, "vertical-plate", 0.5, 14000,
           [layer("ply", "Birch plywood 12 mm", "Birkensperrholz 12 mm", "timber", "plywood", 0.012, 0.012 / 0.14, 14000, source="literature", ref="incropera-2011")]),
    region("cargo", "Plastic crates", "Kunststoffkisten", "surface", "passive", "plastic", 1.2, 0.5, "vertical-plate", 0.5, 5400,
           [layer("wall", "Polypropylene wall 3 mm", "Polypropylenwand 3 mm", "polymer", "plastic", 0.003, 0.003 / 0.2, 5400, source="literature", ref="incropera-2011")]),
    region("fenders", "Mudguards", "Schutzbleche", "surface", "passive", "plastic", 0.6, 0.6, "cylinder", 0.6, 5400, []),
    region("touch-parts", "Saddle and grips", "Sattel und Griffe", "surface", "passive", "plastic", 0.25, 0.15, "cylinder", 0.6, 5000, []),
    region("battery-pack", "Frame battery case", "Rahmen-Akkugehäuse", "surface", "operating", "battery-case", 0.15, 0.4, "vertical-plate", 0.5, 10000, [], ("assist", 9)),
    region("drive-motor", "Mid-drive motor housing", "Mittelmotor-Gehäuse", "engine", "operating", "motor-case", 0.12, 0.14, "cylinder", 0.4, 11000, [], ("assist", 30), ref="en-15194"),
    region("tyres", "Tyre treads and sidewalls", "Reifenlaufflächen und -flanken", "tyre", "operating", "rubber", 0.9, 0.5, "cylinder", 0.5, 7000,
           [layer("tread", "Rubber tread and casing", "Gummilauffläche und Karkasse", "polymer", "rubber", 0.004, 0.004 / 0.2, 7000, source="authored-estimate", ref="wong-2008")], ("rolling", 86), ref="wong-2008"),
    region("brakes", "Brake rotors", "Bremsscheiben", "surface", "operating", "steel-disc", 0.04, 0.16, "vertical-plate", 0.4, 3450, [], ("braking", 1700)),
]
DEFAULTS = {"frame": (1, 0.5), "box-wood": (1, 0.6), "cargo": (1, 0.6), "fenders": (1, 0.5), "touch-parts": (1, 0.7), "battery-pack": (1, 0.3), "drive-motor": (0.8, 0.2), "tyres": (1, 0.1), "brakes": (0.5, 0)}
CONTACTS = {"tyres": (0.02, 30)}
P = lambda a, r, b: {"operating": {"assist": a, "rolling": r, "braking": b}, "emitters": {}}
POSES = {"parked-on-stand": P(0, 0, 0), "loaded-on-stand": P(0, 0, 0), "rolling-loaded": P(0, 0.4, 0), "ride-start": P(0, 0.4, 0), "ride-end": P(0, 0.4, 0)}
fix_clips = lambda clips: None


def reps(a, old):
    R = {}
    st = lambda id, bone, d, regs, pen, z=1, tone=0.4: path(id, bone, "detail", z, d, tone, False, regions=regs, material="aluminium", pen=pen)
    wheelr = lambda sfx, bone, r, regs: ell("tyre-" + sfx, bone, "body", 4, 0, 0, r, tone=0.3, regions=regs, material="rubber")
    # 500
    fr = lambda id, bone, d, regs: path(id, bone, "body", 2, d, 0.4, False, regions=regs, material="aluminium", pen=0.13)
    e500 = [
        fr("frame-lines", "frame", "M 0,0.33 L 0.55,0.32 L 0.3,0.98 L 0.285,1.07 M 0,0.33 L 0.34,0.86 L 0.3,0.98 M 0.55,0.32 L 1.78,0.4 L 1.85,0.62", ["frame", "touch-parts", "battery-pack", "drive-motor"]),
        fr("bar-lines", "bar", "M 0,0 L 0,0.43 L -0.14,0.5", ["frame", "touch-parts"]),
        fr("fork-line", "fork", "M 0,0 L 0.25,-0.365", ["frame", "fenders"]),
        path("steer-line", "fork", "body", 3.7, "M -1.11,0 L 0,0", 0.4, False, region="frame", material="aluminium", pen=0.13),
        fr("stand-line", "stand", "M 0,0 L 0.3433,0", ["frame"]),
        path("box-shell", "box", "body", 3, "M 0,0 L 0.83,0 C 0.85,0.09 0.91,0.15 1.0,0.17 L 1.0,0.65 L 0,0.65 Z", 0.6, True, region="box-wood", material="plywood"),
        rect("crate", "crate", "body", 3.5, 0, 0, 0.5, 0.56, tone=0.45, region="cargo", material="plastic"),
        wheelr("r", "wheel-r", 0.33, ["tyres", "brakes"]), wheelr("f", "wheel-f", 0.255, ["tyres", "brakes"]),
    ]
    R["500"] = rep(500, *RANGES[500], PRINT_PENS[500], [], e500, decisions(
        tx("Two wheel discs, the plywood box with its crate and a frame drawn as centre-lines; the rider-less long-john reads as box and two circles.", "Zwei Radscheiben, der Sperrholzkasten mit Kiste und ein als Mittellinien gezeichneter Rahmen; das fahrerlose Long John liest sich als Kasten und zwei Kreise."),
        tx("Box, crate, both wheels as discs, frame, fork, handlebar stem and the stand as lines (stand still moves).", "Kasten, Kiste, beide Räder als Scheiben, Rahmen, Gabel, Lenkerschaft und Stütze als Linien (die Stütze bewegt sich weiter)."),
        tx("Saddle, battery, motor and mudguards are merged into the frame and wheel regions (area-weighted overlay).", "Sattel, Akku, Motor und Schutzbleche gehen in den Rahmen- und Radregionen auf (flächengewichtete Überlagerung)."),
        tx("Crank, pedals, rack, lamp, fork spokes, box planks, crate slots, hub and rim.", "Kurbel, Pedale, Gepäckträger, Lampe, Speichen, Kastenbretter, Kistenschlitze, Nabe und Felge."),
        tx("Tubes of 0.022 to 0.06 m would be 0.04 to 0.12 mm: they are drawn as centre-line strokes of 0.13 mm (0.065 m), thicker than real; this exceeds the outline tolerance and is stated here. Wheels are solid discs instead of spoked wire wheels.", "Rohre von 0,022 bis 0,06 m wären 0,04 bis 0,12 mm: sie sind als Mittellinien mit 0,13 mm (0,065 m) gezeichnet, dicker als real; das überschreitet die Konturtoleranz und wird hier ausgewiesen. Räder sind Vollscheiben statt Speichenräder."),
        tx("None.", "Keine."),
        tx("Box mid-tone, crate lighter, tyres dark grey, frame lines dark.", "Kasten mittlerer Ton, Kiste heller, Reifen dunkelgrau, Rahmenlinien dunkel."),
        tx("Frame, box, crate and wheels are separate overlay shapes; motor, battery, saddle and mudguards share the colour of the frame and wheel shapes that carry them.", "Rahmen, Kasten, Kiste und Räder sind getrennte Überlagerungsformen; Motor, Akku, Sattel und Schutzbleche teilen die Farbe der Formen, die sie tragen.")))
    # 200 / 100
    s200 = lambda: [
        st("chainstay", "frame", "M 0,0.33 L 0.55,0.32", ["frame"], 0.13), st("seatstay", "frame", "M 0,0.33 L 0.34,0.86", ["frame"], 0.13),
        st("seat-tube", "frame", "M 0.55,0.32 L 0.3,0.98 L 0.285,1.04", ["frame"], 0.15), st("main-beam", "frame", "M 0.55,0.32 L 1.78,0.4", ["frame"], 0.3),
        st("head-riser", "frame", "M 1.78,0.4 L 1.85,0.62", ["frame"], 0.25),
        st("bar-stem", "bar", "M 0,0 L 0,0.43", ["frame"], 0.13), st("fork-leg", "fork", "M 0,0 L 0.25,-0.365", ["frame"], 0.15),
        st("steer-rod", "fork", "M -1.11,0 L 0,0", ["frame"], 0.13, z=3.7),
        st("stand-leg", "stand", "M 0,0 L 0.3433,0", ["frame"], 0.13),
        path("saddle", "frame", "body", 2, "M 0.18,1.065 L 0.4,1.075", 0.2, False, region="touch-parts", material="plastic", pen=0.3),
        path("handlebar", "bar", "body", 2, "M 0,0.43 L -0.14,0.5", 0.2, False, region="touch-parts", material="plastic", pen=0.2),
        path("box-shell", "box", "body", 3, "M 0,0 L 0.83,0 C 0.85,0.09 0.91,0.15 1.0,0.17 L 1.0,0.65 L 0,0.65 Z", 0.6, True, region="box-wood", material="plywood"),
        path("box-planks", "box", "detail", 3.1, "M 0,0.2 L 0.87,0.2 M 0,0.4 L 1.0,0.4", 0.5, False),
        rect("crate-lower", "crate", "body", 3.5, 0, 0, 0.5, 0.28, 0.015, tone=0.45, region="cargo", material="plastic"),
        rect("crate-upper", "crate", "body", 3.6, 0.02, 0.28, 0.46, 0.28, 0.015, tone=0.5, region="cargo", material="plastic"),
        rect("battery-case", "battery", "detail", 2.5, -0.03, -0.19, 0.06, 0.38, tone=0.25, region="battery-pack", material="battery-case"),
        rect("motor-housing", "motor", "detail", 2.5, -0.07, -0.07, 0.14, 0.14, 0.03, tone=0.3, region="drive-motor", material="motor-case"),
        path("rear-fender", "frame", "detail", 1.5, "M -0.307,0.507 C -0.241,0.622 -0.117,0.690 0.015,0.685 C 0.148,0.679 0.266,0.600 0.322,0.480", 0.4, False, region="fenders", material="plastic", pen=0.2),
        path("front-fender", "fork", "detail", 1.5, "M 0.003,-0.223 C 0.054,-0.134 0.148,-0.080 0.250,-0.080 C 0.352,-0.080 0.446,-0.134 0.497,-0.223", 0.4, False, region="fenders", material="plastic", pen=0.2),
        ell("tyre-r", "wheel-r", "body", 4, 0, 0, 0.33, tone=0.3, region="tyres", material="rubber"), ell("rim-r", "wheel-r", "detail", 4.2, 0, 0, 0.28, tone=0.6, filled=False, material="alloy-rim"),
        ell("disc-r", "wheel-r", "detail", 4.4, 0, 0, 0.08, tone=0.4, region="brakes", material="steel-disc"),
        ell("tyre-f", "wheel-f", "body", 4, 0, 0, 0.255, tone=0.3, region="tyres", material="rubber"), ell("rim-f", "wheel-f", "detail", 4.2, 0, 0, 0.203, tone=0.6, filled=False, material="alloy-rim"),
        ell("disc-f", "wheel-f", "detail", 4.4, 0, 0, 0.08, tone=0.4, region="brakes", material="steel-disc"),
    ]
    e200 = s200()
    R["200"] = rep(200, *RANGES[200], PRINT_PENS[200], [], e200, decisions(
        tx("Long-john with box and two crates, spoke-free wheels shown as tyre disc, rim ring and brake disc, frame as tube strokes.", "Long John mit Kasten und zwei Kisten, Räder als Reifenscheibe mit Felgenring und Bremsscheibe, Rahmen als Rohrstriche."),
        tx("Frame tubes, fork, stem, handlebar, saddle, stand, box with plank lines, two crates, battery, motor, mudguards, rims and discs.", "Rahmenrohre, Gabel, Schaft, Lenker, Sattel, Stütze, Kasten mit Bretterlinien, zwei Kisten, Akku, Motor, Schutzbleche, Felgen und Scheiben."),
        tx("Hub merges into the brake disc; the crank and pedals are omitted.", "Nabe geht in der Bremsscheibe auf; Kurbel und Pedale entfallen."),
        tx("Rack, lamp, crate slots, brake lever, chain ring (below 0.08 m).", "Gepäckträger, Lampe, Kistenschlitze, Bremshebel, Kettenblatt (unter 0,08 m)."),
        tx("Tubes (0.022 to 0.06 m) are strokes of 0.13 to 0.3 mm (0.026 to 0.06 m), close to real width; the wire spokes are not drawn.", "Rohre (0,022 bis 0,06 m) sind Striche von 0,13 bis 0,3 mm (0,026 bis 0,06 m), nahe der realen Breite; die Drahtspeichen entfallen."),
        tx("None.", "Keine."), tx("Box and crates mid-tone, tyres dark grey, frame strokes dark.", "Kasten und Kisten mittlerer Ton, Reifen dunkelgrau, Rahmenstriche dunkel."),
        tx("Each region has its own shape; thin tube regions show as stroke colour.", "Jede Region hat eine eigene Form; dünne Rohrregionen zeigen sich als Strichfarbe.")))
    e100 = s200()
    e100 += [cap("crank-arms", "crank", "detail", 2.6, -0.17, 0, 0.17, 0, 0.014, tone=0.6, material="alloy-rim"), rect("pedal-a", "crank", "detail", 2.7, 0.12, -0.012, 0.1, 0.024, tone=0.3, material="plastic"), rect("pedal-b", "crank", "detail", 2.7, -0.22, -0.012, 0.1, 0.024, tone=0.3, material="plastic"),
             rect("rear-light", "frame", "detail", 2.5, -0.3, 0.78, 0.04, 0.06, 0.01, tone=0.3, material="plastic"), ell("head-lamp", "fork", "detail", 2.5, 0.1, 0.0, 0.04, 0.03, tone=0.8, material="plastic"),
             path("rear-rack", "frame", "detail", 1.6, "M -0.27,0.78 L 0.3,0.86 M -0.02,0.33 L -0.22,0.79", 0.4, False, material="aluminium"),
             path("box-rim", "box", "detail", 3.1, "M 0,0.6 L 1.0,0.6", 0.5, False), ell("hub-r", "wheel-r", "detail", 4.6, 0, 0, 0.03, tone=0.7, material="alloy-rim"), ell("hub-f", "wheel-f", "detail", 4.6, 0, 0, 0.03, tone=0.7, material="alloy-rim")]
    for s in e100:
        if s["id"] in ("chainstay", "seatstay", "seat-tube", "main-beam", "head-riser", "fork-leg"):
            pass
    R["100"] = rep(100, *RANGES[100], PRINT_PENS[100], [], e100, decisions(
        tx("The 1:200 drawing plus crank, pedals, lamps, rack and box rim; frame tubes at stroke widths of 0.13 to 0.3 mm.", "Die 1:200-Zeichnung mit Kurbel, Pedalen, Leuchten, Gepäckträger und Kastenrand; Rahmenrohre mit Strichstärken von 0,13 bis 0,3 mm."),
        tx("Everything of 1:200 plus crank arms, pedals, rear light, head lamp, rack, box rim, hubs.", "Alles aus 1:200 sowie Kurbelarme, Pedale, Rücklicht, Frontlampe, Gepäckträger, Kastenrand, Naben."),
        tx("Chain ring merges into the crank arms.", "Das Kettenblatt geht in den Kurbelarmen auf."), tx("Brake lever, crate slots, corner posts, wire spokes.", "Bremshebel, Kistenschlitze, Eckpfosten, Drahtspeichen."),
        tx("Tubes 0.022 to 0.06 m = 0.2 to 0.6 mm: drawn as strokes at about real width.", "Rohre 0,022 bis 0,06 m = 0,2 bis 0,6 mm: als Striche in etwa realer Breite gezeichnet."),
        tx("None.", "Keine."), tx("As 1:200 with lighter rims; pens 0.25 / 0.18 mm.", "Wie 1:200 mit helleren Felgen; Stifte 0,25 / 0,18 mm."), tx("As 1:200.", "Wie 1:200.")))
    e50 = convert_v1_shapes(old)
    R["50"] = rep(50, *RANGES[50], PRINT_PENS[50], [], e50, decisions(
        tx("Full elevation of the first version: tubes as capsules, box with planks and corner posts, crates with slots, wheels with rims and discs.", "Vollständige Ansicht der ersten Fassung: Rohre als Kapseln, Kasten mit Brettern und Eckpfosten, Kisten mit Schlitzen, Räder mit Felgen und Scheiben."),
        tx("Everything including crank, pedals, rack, lamps and brake lever.", "Alles einschließlich Kurbel, Pedalen, Gepäckträger, Leuchten und Bremshebel."),
        tx("Nothing merged.", "Nichts zusammengefasst."), tx("Wire spokes (not modelled).", "Drahtspeichen (nicht modelliert)."),
        tx("Seat stay r = 0.011 m is 0.44 mm wide, above the minimum.", "Sattelstrebe r = 0,011 m ist 0,44 mm breit, über dem Minimum."),
        tx("None.", "Keine."), tx("Three line weights 0.35 / 0.25 / 0.13 mm.", "Drei Linienstärken 0,35 / 0,25 / 0,13 mm."), tx("All regions at real extent.", "Alle Regionen in realer Ausdehnung.")))
    return R


NOTES = tx("Long-john cargo bike, riderless: the assist, rolling and braking keys describe the machine alone (no rider heat). Motor and battery losses are small (30 W and 9 W at level 1: 12 percent of a 250 W motor and a few percent of the battery draw), so the model shows a warm motor housing rather than a plume. Masses and dimensions are authored typical values (authored-estimate).",
           "Long-John-Lastenrad ohne Fahrer: die Schlüssel Unterstützung, Rollen und Bremsen beschreiben nur die Maschine (keine Fahrerwärme). Motor- und Akkuverluste sind klein (30 W und 9 W bei Stufe 1: 12 Prozent eines 250-W-Motors und wenige Prozent der Akkuentnahme), daher zeigt das Modell ein warmes Motorgehäuse statt einer Abgasfahne. Massen und Abmessungen sind selbst gesetzte typische Werte (authored-estimate).")
ASSUMPTIONS = [tx("Heat release at level 1: motor housing 30 W (about +25 K), battery case 9 W (+6 K), tyres 86 W (about 25 km/h), rotors 1.7 kW for a stop from 25 km/h (3.3 kJ in 2 s) on 0.3 kg of steel (3.45 kJ/(m2 K) on 0.04 m2).",
                    "Wärmefreisetzung bei Stufe 1: Motorgehäuse 30 W (etwa +25 K), Akkugehäuse 9 W (+6 K), Reifen 86 W (etwa 25 km/h), Scheiben 1,7 kW für einen Halt aus 25 km/h (3,3 kJ in 2 s) auf 0,3 kg Stahl (3,45 kJ/(m2 K) auf 0,04 m2)."),
               tx("No emitters: the machine has no exhaust or fan; thin tubes (0.03 m) respond within seconds (3 kJ/(m2 K)).", "Keine Emitter: die Maschine hat weder Abgas noch Lüfter; dünne Rohre (0,03 m) reagieren innerhalb von Sekunden (3 kJ/(m2 K)).")]
SPEC = dict(sky=8, ground=28, operating=KEYS, regions=REGIONS, emitters=[], assumptions=ASSUMPTIONS, defaults=DEFAULTS, contacts=CONTACTS, poses=POSES, fix_clips=fix_clips, reps=reps, notes=NOTES)
if __name__ == "__main__":
    a = assemble("cargo-bike", REL, SPEC)
    if len(sys.argv) > 1:
        previews(a, "cargo", poses=sys.argv[2:] or None, dens=[int(x) for x in sys.argv[1].split(",")], zoom=(4,))
