"""🌱 Round 3, appliances/decentralised-hrv-fan: root fixes (EPP material, anchors at the real corner screws, rig mass split and lengths, areas, texts) and the drawings of 1:50, 1:100 and 1:200 redrawn as the true views of each scale's body (ruling E1); writes the candidate `🗑️generated/r3-decentralised-hrv-fan/🔣️.json` with the volumes source merged in.

Usage: python3 r3-decentralised-hrv-fan-roots.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/decentralised-hrv-fan/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "decentralised-hrv-fan.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-decentralised-hrv-fan", "🔣️.json")
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-decentralised-hrv-fan-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

CASING_AREA = 0.39
HUB_AREA = 0.01
BLADES_AREA = 0.028


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def f(v):
    """🔢 A coordinate for a path string."""
    s = f"{round(v, 5):.5f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def poly_d(points):
    """✏️ A closed polyline subpath."""
    return "M " + " L ".join(f"{f(x)} {f(y)}" for x, y in points) + " Z"


def blade_outline(sections):
    """🪭 The plan-form of a lofted blade seen along the axis: one edge out from the hub surface, round the tip arc, the other edge back."""
    lead, trail = [], []
    for s in sections[1:]:
        qx, qy, qz, qw = s["rotation"]
        xa = (1 - 2 * (qy * qy + qz * qz), 2 * (qx * qy + qz * qw))
        lead.append((s["at"][0] + s["rx"] * xa[0], s["at"][1] + s["rx"] * xa[1]))
        trail.append((s["at"][0] - s["rx"] * xa[0], s["at"][1] - s["rx"] * xa[1]))
    keep = (0, len(lead) - 3, len(lead) - 2, len(lead) - 1)
    return [lead[i] for i in keep] + [trail[i] for i in reversed(keep)]


def shape(base, **geometry):
    """🧩 A shape keeping the common members of `base` and replacing its geometry."""
    out = {key: base[key] for key in ("id", "bone", "role", "z", "filled", "tone")}
    out.update(geometry)
    out.update({key: base[key] for key in ("region", "regions", "material", "hatch", "penMm") if key in base})
    return out


def struts(r0, r1, width):
    """➖ The three struts of the motor mount seen from the room."""
    return [{"id": f"strut-{i + 1}", "bone": "ring", "role": "body", "z": 3.27, "filled": True, "tone": 0.6, "kind": "poly", "points": k.strut_outline(a, r0, r1, width), "closed": True, "region": "casing", "material": "abs-plastic"} for i, a in enumerate(k.STRUTS)]


def cells_d(pitch, rings):
    """🍯 The channel walls of the honeycomb: one hexagon per channel, neighbours sharing their edges."""
    return " ".join(poly_d(k.hexagon(cx, cy, pitch / math.sqrt(3))) for cx, cy in k.cell_centres(pitch, rings))


def elevation(old, scale, volume):
    """🖼️ The elevation of one scale redrawn from that scale's body."""
    by = {s["id"]: s for s in old}
    body = {s["id"]: s for s in volume["body"]}
    out = [by["plate"]]
    if scale == "50":
        out += [by[f"screw-{i}"] for i in range(1, 5)]
        out.append({"id": "terminal", "bone": "plate", "role": "detail", "z": 3.02, "filled": True, "tone": 0.8, "kind": "rect", "x": -0.03, "y": -0.1225, "w": 0.06, "h": 0.02, "r": 0.002, "region": "casing", "material": "abs-plastic"})
    if scale == "200":
        opening = shape(by["opening"], kind="ellipse", cx=0, cy=0, rx=k.SLEEVE_OUT, ry=k.SLEEVE_OUT)
    else:
        out.append(by["ring"])
        opening = shape(by["opening"], kind="ellipse", cx=0, cy=0, rx=k.THROAT, ry=k.THROAT)
        opening["material"] = "epp-foam"
    out.append(opening)
    out.append(by["core-disc"])
    if scale == "50":
        out.append(by["core-cells"])
    if scale == "100":
        out.append(shape(by["core-cells"], kind="path", d=cells_d(k.CELL_100, 2)))
    if scale == "200":
        out.append({"id": "strut-1", "bone": "ring", "role": "body", "z": 3.27, "filled": True, "tone": 0.6, "kind": "rect", "x": -0.015, "y": -0.082, "w": 0.03, "h": 0.164, "region": "casing", "material": "abs-plastic"})
    else:
        out += struts({"50": 0.02, "100": 0.0}[scale], k.THROAT, {"50": 0.008, "100": 0.015}[scale])
    for i in range(1, 6 if scale != "200" else 4):
        b = body[f"blade-{i}"]
        pts = blade_outline(b["sections"]) if b["kind"] == "loft" else [tuple(p) for p in b["profile"]]
        out.append(shape(by[f"blade-{i}"], kind="path", d=poly_d(pts)))
    hub_r = {"50": k.HUB_R, "100": 0.03, "200": 0.04}[scale]
    out.append(shape(by["hub"], kind="ellipse", cx=0, cy=0, rx=hub_r, ry=hub_r))
    if scale == "50":
        out.append(shape(by["hub-cap"], kind="ellipse", cx=0, cy=0, rx=0.01, ry=0.01))
    return out


def side(old, scale, lateral):
    """📏 Front and plan: plate, sleeve and outer collar are already the true side view; the cover joint and the wall line stood for nothing the unit has (the wall face is the back of the plate) and go. At 1:200 the T mark becomes plate and tube as two masses, as at 1:100."""
    if scale == "200":
        base = {"role": "body", "z": 3, "filled": True, "tone": 0.8, "region": "casing", "material": "abs-plastic"}
        rects = [("plate", "plate", k.Z_FACE, 0.02, 0.13, 3), ("ring", "ring", k.Z_WALL, 0.22, k.SLEEVE_OUT, 3.05)]
        return [dict(base, id=i, bone=b, z=z, kind="rect", **({"x": d0, "y": -h, "w": dd, "h": 2 * h} if lateral else {"x": -h, "y": d0, "w": 2 * h, "h": dd})) for i, b, d0, dd, h, z in rects]
    return [s for s in old if s["id"] not in ("cover-joint", "wall-line")]


DECISIONS = {
    "50": {
        "silhouette": text("A 0.26 m square base of the inner cover, 5.2 mm on paper, seen from the room with cover and filter taken off: the collar of the fan housing as a ring, the black funnel of the EPP jacket round the ceramic honeycomb, and in front of it five radial fan blades on the motor can, held by three struts. Front and plan, 4.8 mm by 5.2 mm on paper, show the unit side-on as a T: the 20 mm tray on the room face of the wall and the round sleeve that crosses 0.22 m of wall, with the 208 mm collar for the outer hood at its end. Fan, ceramic core and jacket are inside the sleeve and not visible from outside.",
                           "Ein quadratischer Sockel der Innenblende von 0,26 m, auf Papier 5,2 mm, vom Raum aus gesehen bei abgenommener Blende und ohne Filter: der Kragen des Ventilatorgehäuses als Ring, der schwarze Trichter des EPP-Mantels um die Keramikwabe und davor fünf radiale Flügel am Motorbecher, den drei Streben halten. Vorderansicht und Grundriss, 4,8 mm auf 5,2 mm, zeigen das Gerät von der Seite als T: die 20 mm tiefe Wanne auf der Raumseite der Wand und die runde Hülse, die 0,22 m Wand durchquert, mit dem Kragen von 208 mm für die Außenhaube am Ende. Ventilator, Keramikkern und Mantel liegen in der Hülse und sind von außen nicht zu sehen."),
        "retained": text("Tray of 0.26 m with its rounded corners, four 14 mm screw bosses and the terminal box of 60 mm by 20 mm, housing collar of 196 mm, the jacket funnel from 176 mm to 160 mm, ceramic disc of 160 mm with 37 hexagonal channels, three 8 mm struts, five blades as the true plan-form of the model (25 to 49 mm wide) and the 56 mm hub with its cap.",
                         "Wanne von 0,26 m mit gerundeten Ecken, vier Schraubdome von 14 mm und der Klemmkasten von 60 mm mal 20 mm, Gehäusekragen von 196 mm, der Manteltrichter von 176 mm auf 160 mm, Keramikscheibe von 160 mm mit 37 sechseckigen Kanälen, drei Streben von 8 mm, fünf Flügel im wahren Grundriss des Modells (25 bis 49 mm breit) und die Nabe von 56 mm mit ihrer Kappe."),
        "merged": text("The real 3 mm cells of the ceramic are generalised to 37 channels of 22 mm, one line for each wall; the 2.5 mm rim of the tray falls into its outline, the stator boss lies behind the hub.",
                       "Die echten Zellen der Keramik von 3 mm sind zu 37 Kanälen von 22 mm verallgemeinert, eine Linie je Wand; der 2,5 mm starke Rand der Wanne fällt in ihre Kontur, der Statorzapfen liegt hinter der Nabe."),
        "omitted": text("The clip-on cover, the filter, the outer hood, the cable and the 8 mm screw heads. In front and plan everything inside tray and sleeve.",
                        "Die aufgesteckte Blende, der Filter, die Außenhaube, das Kabel und die Schraubenköpfe von 8 mm. In Vorderansicht und Grundriss alles, was in Wanne und Hülse liegt."),
        "thinElements": text("The struts are 8 mm (0.16 mm on paper) and drawn as filled bars; the cell walls and the cap use the 0.13 mm detail pen.",
                             "Die Streben messen 8 mm (0,16 mm auf Papier) und sind als gefüllte Stäbe gezeichnet; Zellwände und Kappe nutzen den Detailstift von 0,13 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Tray 0.88, bosses 0.45, terminal box 0.8, housing collar 0.7, EPP funnel 0.12, ceramic 0.62, struts 0.6, blades 0.82, hub 0.55, cap 0.9; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
                       "Wanne 0,88, Dome 0,45, Klemmkasten 0,8, Gehäusekragen 0,7, EPP-Trichter 0,12, Keramik 0,62, Streben 0,6, Flügel 0,82, Nabe 0,55, Kappe 0,9; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": text("Casing, core, blades and hub are four regions: in the thermal view the core changes between the two phases, the hub (motor) warms and the blades stay near the air temperature; the blades turn with the rotor bone in both directions.",
                         "Gehäuse, Kern, Flügel und Nabe sind vier Regionen: in der Thermalansicht wechselt der Kern zwischen den zwei Phasen, die Nabe (Motor) erwärmt sich und die Flügel bleiben nahe der Lufttemperatur; die Flügel drehen sich mit dem Rotorknochen in beide Richtungen."),
    },
    "100": {
        "silhouette": text("A 2.6 mm square with the housing ring, a black liner ring, a light disc with a coarse cell pattern and a five-bladed radial impeller on three struts. Front and plan, 2.4 mm by 2.6 mm: plate, sleeve and outer collar as three masses.",
                           "Ein Quadrat von 2,6 mm mit dem Gehäusering, einem schwarzen Mantelring, einer hellen Scheibe mit grobem Zellmuster und einem fünfflügeligen radialen Laufrad auf drei Streben. Vorderansicht und Grundriss, 2,4 mm auf 2,6 mm: Platte, Hülse und Außenkragen als drei Massen."),
        "retained": text("Plate, housing ring of 196 mm, EPP liner from 176 mm to 160 mm, ceramic disc with 19 channels of 30 mm, three 15 mm struts, five blades as the plan-form of the 1:100 model and the 60 mm hub.",
                         "Platte, Gehäusering von 196 mm, EPP-Mantel von 176 mm auf 160 mm, Keramikscheibe mit 19 Kanälen von 30 mm, drei Streben von 15 mm, fünf Flügel im Grundriss des 1:100-Modells und die Nabe von 60 mm."),
        "merged": text("The channels are merged to 19 coarse hexagons that stay inside the disc; bosses, terminal box and hub cap are dropped; the struts meet behind the hub.",
                       "Die Kanäle sind zu 19 groben Sechsecken zusammengefasst, die in der Scheibe bleiben; Dome, Klemmkasten und Nabenkappe entfallen; die Streben treffen sich hinter der Nabe."),
        "omitted": text("Four screw bosses, the terminal box, the hub cap and the fine cell structure.",
                        "Vier Schraubdome, der Klemmkasten, die Nabenkappe und die feine Zellstruktur."),
        "thinElements": text("The blades are 26 mm wide at the hub (0.26 mm) and 51 mm at the tip; the struts are drawn at the 15 mm printable width; the cell outlines stay 0.13 mm.",
                             "Die Flügel sind an der Nabe 26 mm (0,26 mm) und an der Spitze 51 mm breit; die Streben werden mit der druckbaren Breite von 15 mm gezeichnet; die Zellkonturen bleiben 0,13 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Plate 0.88, ring 0.7, EPP liner 0.12, ceramic 0.62, struts 0.6, blades 0.82, hub 0.55; body pen 0.18 mm, detail 0.13 mm.",
                       "Platte 0,88, Ring 0,7, EPP-Mantel 0,12, Keramik 0,62, Streben 0,6, Flügel 0,82, Nabe 0,55; Körperstift 0,18 mm, Detail 0,13 mm."),
        "overlays": text("Casing, core, blades and hub stay four regions; the blades turn with the rotor.",
                         "Gehäuse, Kern, Flügel und Nabe bleiben vier Regionen; die Flügel drehen sich mit dem Rotor."),
    },
    "200": {
        "silhouette": text("A 1.3 mm square with a dark ring of 196 mm, a light ceramic disc and a three-bladed impeller in front of one upright bridge bar. Front and plan, 1.2 mm by 1.3 mm: plate and sleeve as two masses.",
                           "Ein Quadrat von 1,3 mm mit einem dunklen Ring von 196 mm, einer hellen Keramikscheibe und einem dreiflügeligen Laufrad vor einem stehenden Brückensteg. Vorderansicht und Grundriss, 1,2 mm auf 1,3 mm: Platte und Hülse als zwei Massen."),
        "retained": text("Plate, the dark end of the thick tube, ceramic disc, one 30 mm bridge bar, three broad radial blades and the 80 mm hub.",
                         "Platte, das dunkle Ende des dicken Rohrs, Keramikscheibe, ein Brückensteg von 30 mm, drei breite radiale Flügel und die Nabe von 80 mm."),
        "merged": text("Fan housing, sleeve and EPP liner are one tube whose end is the dark ring; the cell pattern is merged into the tone of the disc; five blades become three and the three struts one bridge bar.",
                       "Ventilatorgehäuse, Hülse und EPP-Mantel sind ein Rohr, dessen Ende der dunkle Ring ist; das Zellmuster geht im Ton der Scheibe auf; aus fünf Flügeln werden drei und aus den drei Streben ein Brückensteg."),
        "omitted": text("Screw bosses, terminal box, hub cap and cells. The collar at the outer end in front and plan.",
                        "Schraubdome, Klemmkasten, Nabenkappe und Zellen. Der Kragen am Außenende in Vorderansicht und Grundriss."),
        "thinElements": text("The blades are 45 to 85 mm wide (0.23 to 0.43 mm), the bridge bar 30 mm and the ceramic disc 164 mm (0.82 mm), so that the light disc reads between blades and bar.",
                             "Die Flügel sind 45 bis 85 mm (0,23 bis 0,43 mm), der Brückensteg 30 mm breit und die Keramikscheibe misst 164 mm (0,82 mm), damit die helle Scheibe zwischen Flügeln und Steg lesbar bleibt."),
        "hatching": text("None.", "Keine."),
        "styles": text("Plate 0.88, tube end 0.12, ceramic 0.62, bridge bar 0.6, blades 0.82, hub 0.55; one body pen of 0.13 mm.",
                       "Platte 0,88, Rohrende 0,12, Keramik 0,62, Brückensteg 0,6, Flügel 0,82, Nabe 0,55; ein Körperstift von 0,13 mm."),
        "overlays": text("Plate, tube and bridge bar carry the casing region; core, blades and hub keep theirs.",
                         "Platte, Rohr und Brückensteg tragen die Gehäuseregion; Kern, Flügel und Nabe behalten ihre."),
    },
}

REFERENCES = [
    {"id": "lunos-e2-60", "citation": "LUNOS e2 60, technical information E348 02.20 (read 2026-11-20): decentralised push-pull unit for a round duct of 160 mm, 5 to 60 m3/h, plug-in module of 154 mm diameter and 243 mm length (short version 160 mm long for walls from 200 mm), ceramic heat store in an EPP foam housing."},
    {"id": "ambientika-200", "citation": "Ambientika wall unit for 200 mm and 160 mm pipes, technical sheet (ambientika.eu, read 2026-11-20): ceramic heat exchanger of 170 mm diameter and 150 mm length with hexagonal cells in a wall pipe of 200 mm."},
    {"id": "blauberg-vento-expert-a50", "citation": "Blauberg Vento Expert A50-1 W, data sheets V.2 and V.3 (blaubergventilatoren.net, read 2026-11-20): reversible fan and ceramic regenerator in a 160 mm duct, minimum wall thickness 240 mm, 5 to 8.5 W over three speeds, cycle of about 70 s (DIBt Z-51.3-399)."},
    {"id": "helios-ecovent-verso", "citation": "Helios KWL EcoVent Verso, product description (read 2026-11-20): wall sleeve, fan casing of EPP, protection grille, flow straighteners and ceramic heat accumulator behind a removable inner cover."},
]

NOTES = text(
    "A decentralised push-pull unit of the wall-pipe class of 200 mm (Ambientika: ceramic of 170 mm by 150 mm in a 200 mm pipe) seen from the room with the clip-on inner cover and the filter taken off. Built as these units are: the base of the inner cover is a moulded ABS tray of 0.26 m by 0.26 m and 20 mm with rounded corners, screwed to the wall through four bosses in its corners, with the terminal box at the bottom; the fan housing stands in it with a 196 mm collar and a 176 mm throat and plugs into the wall sleeve of 196 mm with 3 mm wall, which is cut to the wall (0.22 m here, at the short end of the products: Lunos e2 short from 200 mm, Blauberg from 240 mm) and ends in a 208 mm collar for the outer hood, which is not shown. A reversible axial fan of 172 mm with five radial, symmetric blades on a 56 mm external-rotor can, held by three struts, runs in front of the ceramic honeycomb of 0.16 m diameter and 0.1 m depth, which sits in a black EPP jacket. The real honeycomb has cells of about 3 mm; drawings and models show them generalised. The axis is w, the room at -w: tray rim and housing lip at w = -0.12 m, the wall face (back of the tray) at w = -0.10 m, the outer end of the sleeve at w = 0.12 m (depthM 0.24); the origin lies on the axis. Dimensions and the mass of 2.65 kg are an authored estimate of this class: 1.6 kg ceramic (0.002 m3 at a bulk density of about 800 kg/m3), 0.7 kg sleeve, fan housing, stator and jacket, 0.25 kg tray with terminal box and 0.1 kg rotor. The fan blows room air outwards for 70 seconds (the warm air heats the ceramic, the air leaves at about 0 degC) and then reverses and draws outdoor air inwards for 70 seconds (the ceramic warms it to about 16 degC): the heat recovery efficiency is about 80 percent, at 35 m3/h per unit and 4 W of electric power (motor and electronics) in the working stage. The clip shows the real 1500 rpm slowed to two revolutions per 70 s phase, with a short standstill at each reversal.",
    "Ein dezentrales Pendellüftungsgerät der Klasse für Wandrohre von 200 mm (Ambientika: Keramik von 170 mm mal 150 mm im Rohr von 200 mm), vom Raum aus gesehen bei abgenommener Steckblende und ohne Filter. Gebaut wie diese Geräte: der Sockel der Innenblende ist eine gespritzte ABS-Wanne von 0,26 m mal 0,26 m und 20 mm mit gerundeten Ecken, durch vier Dome in den Ecken an die Wand geschraubt, mit dem Klemmkasten unten; darin steht das Ventilatorgehäuse mit einem Kragen von 196 mm und einem Hals von 176 mm und steckt in der Wandhülse von 196 mm mit 3 mm Wand, die auf die Wand abgelängt ist (hier 0,22 m, am kurzen Ende der Produkte: Lunos e2 kurz ab 200 mm, Blauberg ab 240 mm) und in einem Kragen von 208 mm für die nicht gezeigte Außenhaube endet. Ein umkehrbarer Axialventilator von 172 mm mit fünf radialen, symmetrischen Flügeln an einem Außenläuferbecher von 56 mm, gehalten von drei Streben, läuft vor der Keramikwabe von 0,16 m Durchmesser und 0,1 m Tiefe, die in einem schwarzen EPP-Mantel sitzt. Die echte Wabe hat Zellen von etwa 3 mm; Zeichnungen und Modelle zeigen sie verallgemeinert. Die Achse ist w, der Raum bei -w: Wannenrand und Gehäuselippe bei w = -0,12 m, die Wandfläche (Rückseite der Wanne) bei w = -0,10 m, das Außenende der Hülse bei w = 0,12 m (depthM 0,24); der Ursprung liegt auf der Achse. Maße und die Masse von 2,65 kg sind Schätzwerte dieser Klasse: 1,6 kg Keramik (0,002 m3 bei einer Rohdichte von etwa 800 kg/m3), 0,7 kg Hülse, Ventilatorgehäuse, Stator und Mantel, 0,25 kg Wanne mit Klemmkasten und 0,1 kg Laufrad. Der Ventilator bläst 70 Sekunden lang Raumluft nach außen (die warme Luft erwärmt die Keramik, die Luft verlässt das Haus mit etwa 0 degC) und kehrt dann um und saugt 70 Sekunden lang Außenluft nach innen (die Keramik erwärmt sie auf etwa 16 degC): der Wärmebereitstellungsgrad beträgt etwa 80 Prozent, bei 35 m3/h je Gerät und 4 W elektrischer Leistung (Motor und Elektronik) in der Arbeitsstufe. Der Clip zeigt die echten 1500 U/min verlangsamt auf zwei Umdrehungen je 70-s-Phase mit kurzem Stillstand bei jeder Umkehr.")


def main():
    """💾 Build and write the candidate."""
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    vol = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    a["volumes"] = vol
    a["summary"] = text(
        "A 0.26 m square decentralised push-pull ventilation unit with its inner cover off: in the base tray a reversible fan with five radial blades turns in front of a ceramic honeycomb core that stores the heat of the extract air for 70 seconds and gives it back to the supply air for the next 70; a 196 mm sleeve carries both through the wall.",
        "Ein quadratisches dezentrales Pendellüftungsgerät von 0,26 m bei abgenommener Innenblende: in der Sockelwanne dreht sich ein umkehrbarer Ventilator mit fünf radialen Flügeln vor einem Keramik-Wabenkern, der die Wärme der Abluft 70 Sekunden speichert und in den nächsten 70 Sekunden an die Zuluft zurückgibt; eine Hülse von 196 mm führt beide durch die Wand.")
    ids = {r["id"] for r in REFERENCES}
    a["provenance"]["references"] = [r for r in a["provenance"]["references"] if r["id"] not in ids] + REFERENCES
    a["provenance"]["notes"] = NOTES
    a["materials"] = [m for m in a["materials"] if m["id"] != "epp-foam"] + [{
        "id": "epp-foam", "title": text("Black expanded polypropylene", "Schwarzes expandiertes Polypropylen"), "emissivity": 0.92, "solarAbsorptivity": 0.9,
        "densityKgM3": 60, "conductivityWmK": 0.039, "specificHeatJKgK": 1700, "source": "authored-estimate", "reference": "iso-10456-2007"}]
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["plate"]["title"] = text("Base tray of the inner cover", "Sockelwanne der Innenblende")
    bones["plate"]["mass"]["kg"] = 0.25
    bones["ring"]["title"] = text("Wall sleeve with fan housing, stator and EPP jacket", "Wandhülse mit Ventilatorgehäuse, Stator und EPP-Mantel")
    bones["ring"]["mass"]["kg"] = 0.7
    bones["ring"]["lengthM"] = 0.196
    bones["rotor"]["title"] = text("Reversible external rotor with blades", "Umkehrbarer Außenläufer mit Flügeln")
    bones["rotor"]["lengthM"] = 0.086
    anchors = {x["id"]: x for x in a["anchors"]}
    anchors["wall-screw-top"].update(x=-k.SCREW, y=k.SCREW, z=k.Z_WALL)
    anchors["wall-screw-top"]["title"] = text("Upper left screw in the wall", "Obere linke Schraube in der Wand")
    anchors["wall-screw-bottom"].update(x=k.SCREW, y=-k.SCREW, z=k.Z_WALL)
    anchors["wall-screw-bottom"]["title"] = text("Lower right screw in the wall", "Untere rechte Schraube in der Wand")
    anchors["outlet"]["title"] = text("Axis of the ceramic core", "Achse des Keramikkerns")
    regions = {r["id"]: r for r in a["thermal"]["regions"]}
    regions["casing"]["areaM2"] = CASING_AREA
    regions["casing"]["title"] = text("Base tray, fan housing and wall sleeve", "Sockelwanne, Ventilatorgehäuse und Wandhülse")
    regions["hub"]["areaM2"] = HUB_AREA
    regions["blades"]["areaM2"] = BLADES_AREA
    a["structure"]["assumptions"] = [text(
        "The 2.65 kg unit is idealised as hanging on two of the four corner screws of the base tray: the upper left one is the pin, the lower right one the roller against the wall. In the building the sleeve is bedded in the wall and carries the ceramic core and the fan directly, so the screws see less than this; the idealisation is the case of a sleeve that lies loose in its core hole. The core sits in the jacket in the sleeve, the motor hangs on three struts in the fan housing, the rotor on its shaft bearing.",
        "Das 2,65 kg schwere Gerät ist so idealisiert, dass es an zwei der vier Eckschrauben der Sockelwanne hängt: die obere linke ist das Festlager, die untere rechte das Rollenlager an der Wand. Im Bau liegt die Hülse satt in der Wand und trägt Keramikkern und Ventilator direkt, die Schrauben sehen also weniger; die Idealisierung ist der Fall einer Hülse, die lose in ihrer Kernbohrung liegt. Der Kern sitzt im Mantel in der Hülse, der Motor hängt an drei Streben im Ventilatorgehäuse, das Laufrad an seinem Wellenlager.")]
    reps = a["representations"]
    for scale in ("50", "100", "200"):
        geo = reps[scale]["geometry"]
        geo["elevation"] = elevation(geo["elevation"], scale, vol[scale])
        geo["front"] = side(geo["front"], scale, True)
        geo["plan"] = side(geo["plan"], scale, False)
        reps[scale]["decisions"] = DECISIONS[scale]
    reps["500"]["limitingFeature"] = text("Screw bosses of 14 mm", "Schraubdome von 14 mm")
    reps["500"]["reason"] = text(
        "At 1:500 the fan unit is 0.52 mm by 0.52 mm on paper, well under the 3 mm a recognisable drawing needs, and its smallest drawn part, the screw bosses of 14 mm, is 0.03 mm, below the 0.2 mm office-laser minimum feature; draw it at 1:200 or larger.",
        "Bei 1:500 ist das Lüftergerät 0,52 mm mal 0,52 mm groß auf Papier, deutlich unter den 3 mm, die eine erkennbare Zeichnung braucht, und sein kleinstes gezeichnetes Teil, die Schraubdome von 14 mm, misst 0,03 mm, unter dem Mindestmerkmal von 0,2 mm des Büro-Laserdrucks; bei 1:200 oder größer zeichnen.")
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, indent=2, ensure_ascii=False)
        fh.write("\n")
    print("written", OUT)


if __name__ == "__main__":
    main()
