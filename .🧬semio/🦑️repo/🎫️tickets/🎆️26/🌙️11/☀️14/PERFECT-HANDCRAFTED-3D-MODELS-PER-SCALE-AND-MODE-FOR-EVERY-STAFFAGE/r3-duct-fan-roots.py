"""🌱 Round 3, appliances/duct-fan: root fixes (anchors at the real screws, rig mass split, casing area, texts) and the elevation drawings of 1:50, 1:100 and 1:200 redrawn as the true view of each scale's body (ruling E1); writes the candidate `🗑️generated/r3-duct-fan/🔣️.json` with the volumes source merged in.

Usage: python3 r3-duct-fan-roots.py
"""
import copy
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/duct-fan/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "duct-fan.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-duct-fan", "🔣️.json")
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-duct-fan-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

CASING_AREA = 0.18
HUB_AREA = 0.0165


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def f(v):
    """🔢 A coordinate for a path string."""
    s = f"{round(v, 5):.5f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def circle_d(r, cx=0.0, cy=0.0):
    """⭕ A full circle as two arcs."""
    return f"M {f(cx + r)} {f(cy)} A {f(r)} {f(r)} 0 0 1 {f(cx - r)} {f(cy)} A {f(r)} {f(r)} 0 0 1 {f(cx + r)} {f(cy)} Z"


def poly_d(points):
    """✏️ A closed polyline subpath."""
    return "M " + " L ".join(f"{f(x)} {f(y)}" for x, y in points) + " Z"


def plate_d(bore):
    """⬜ The rounded square plate with its bore."""
    return poly_d(k.rounded_square(k.PLATE / 2, k.PLATE_R, 4)) + " " + circle_d(bore)


def blade_outline(sections):
    """🪭 The plan-form of a lofted blade seen along the axis: leading-edge points out, the straight tip, trailing-edge points back."""
    le, te = [], []
    for s in sections:
        qx, qy, qz, qw = s["rotation"]
        xa = (1 - 2 * (qy * qy + qz * qz), 2 * (qx * qy + qz * qw))
        le.append((s["at"][0] + s["rx"] * xa[0], s["at"][1] + s["rx"] * xa[1]))
        te.append((s["at"][0] - s["rx"] * xa[0], s["at"][1] - s["rx"] * xa[1]))
    return le + list(reversed(te))


def shape(base, **geometry):
    """🧩 A shape keeping the common members of `base` and replacing its geometry."""
    out = {key: base[key] for key in ("id", "bone", "role", "z", "filled", "tone")}
    out.update(geometry)
    out.update({key: base[key] for key in ("region", "regions", "material", "hatch", "penMm") if key in base})
    return out


def struts(width, r0, r1, base):
    """✚ The four motor-spider arms seen from the room."""
    arms = []
    for i, (cx, cy, w, h) in enumerate(((r0, -width / 2, r1 - r0, width), (-width / 2, r0, width, r1 - r0), (-r1, -width / 2, r1 - r0, width), (-width / 2, -r1, width, r1 - r0))):
        arms.append(dict(base, id=f"strut-{i + 1}", kind="rect", x=round(cx, 5), y=round(cy, 5), w=round(w, 5), h=round(h, 5)))
    return arms


def elevation(old, scale, volume):
    """🖼️ The elevation of one scale redrawn from that scale's body."""
    by = {s["id"]: s for s in old}
    body = {s["id"]: s for s in volume["body"]}
    bore = 0.096 if scale == "200" else k.BORE
    throat = 0.096 if scale == "200" else k.THROAT
    out = [shape(by["plate"], kind="path", d=plate_d(bore))]
    if scale == "50":
        out += [dict(by[f"screw-{i}"], rx=0.005, ry=0.005) for i in range(1, 5)]
    if "ring" in by:
        out.append(shape(by["ring"], kind="path", d=circle_d(k.BORE) + " " + circle_d(throat)))
    opening = shape(by["opening"], kind="ellipse", cx=0, cy=0, rx=throat, ry=throat)
    opening["role"] = "texture"
    out.append(opening)
    arm = {"bone": "ring", "role": "body", "z": 3.15, "filled": True, "tone": 0.6, "region": "casing", "material": "abs-plastic"}
    out += struts({"50": 0.008, "100": 0.015, "200": 0.030}[scale], {"50": 0.026, "100": 0.0, "200": 0.0}[scale], {"50": 0.0975, "100": 0.0975, "200": 0.096}[scale], arm)
    for i in range(1, 6 if scale != "200" else 4):
        b = body[f"blade-{i}"]
        pts = blade_outline(b["sections"]) if b["kind"] == "loft" else [tuple(p) for p in b["profile"]]
        out.append(shape(by[f"blade-{i}"], kind="path", d=poly_d(pts)))
    hub_r = {"50": k.HUB_R, "100": 0.036, "200": 0.044}[scale]
    out.append(shape(by["hub"], kind="ellipse", cx=0, cy=0, rx=hub_r, ry=hub_r))
    if scale == "50":
        out.append(shape(by["hub-cap"], kind="ellipse", cx=0, cy=0, rx=0.012, ry=0.012))
    return out


def side(old, scale):
    """📏 Front and plan: the 1:50 bead is 7.5 mm deep (the modelled bead), the 1:200 T mark becomes plate and spigot as two masses (the joint between the two parts, as at 1:100); the rest is already the true side view."""
    out = copy.deepcopy(old)
    if scale == "200" and "points" in out[0]:
        base = {key: out[0][key] for key in ("role", "z", "filled", "tone", "region", "material")}
        lateral = out[0]["points"][0][0] == -0.045
        rects = [("plate", "plate", -0.045, 0.015, 0.11), ("ring", "ring", -0.030, 0.075, 0.10)]
        out = [dict(base, id=i, bone=b, kind="rect", **({"x": d0, "y": -h, "w": dd, "h": 2 * h} if lateral else {"x": -h, "y": d0, "w": 2 * h, "h": dd})) for i, b, d0, dd, h in rects]
        out[1]["z"] = 3.05
    if scale == "50":
        for s in out:
            if s["id"] == "lip":
                if "w" in s and s["w"] < 0.01:
                    s["x"], s["w"] = 0.0375, 0.0075
                else:
                    s["y"], s["h"] = 0.0375, 0.0075
    return out


DECISIONS = {
    "50": {
        "silhouette": text("A 0.22 m square wall fan, 4.4 mm on paper, seen from the room: a rounded plate with four screw caps and a 0.2 m bore, the bell-mouth nozzle as a ring round the 176 mm throat, five swept and pitched blades on the 68 mm motor can with its spinner cap, and the four arms of the motor spider behind them. Front and plan, 1.8 mm by 4.4 mm, show the fan side-on: the 12 mm plate on the room side, the spigot behind it with the 7.5 mm sealing bead and the seam between nozzle and spigot sleeve.",
                           "Ein quadratischer Wandventilator von 0,22 m, auf Papier 4,4 mm, vom Raum aus gesehen: eine Platte mit gerundeten Ecken, vier Schraubenkappen und einer Bohrung von 0,2 m, die Einlaufdüse als Ring um den Hals von 176 mm, fünf gepfeilte und angestellte Flügel am Motorbecher von 68 mm mit seiner Nabenkappe und dahinter die vier Arme des Motorsterns. Vorderansicht und Grundriss, 1,8 mm auf 4,4 mm, zeigen den Ventilator von der Seite: die Platte von 12 mm auf der Raumseite, dahinter der Stutzen mit dem Dichtwulst von 7,5 mm und der Fuge zwischen Düse und Stutzenhülse."),
        "retained": text("Plate of 0.22 m with the 0.2 m bore, nozzle ring to the 176 mm throat, five blades as their true plan-form (the straight tip of the planar tip cut), the 68 mm hub and the 24 mm spinner cap, four 10 mm screw caps, four 8 mm spider arms.",
                         "Platte von 0,22 m mit der Bohrung von 0,2 m, Düsenring zum Hals von 176 mm, fünf Flügel in ihrem wahren Grundriss (die gerade Spitze des ebenen Spitzenschnitts), die Nabe von 68 mm und die Nabenkappe von 24 mm, vier Schraubenkappen von 10 mm, vier Sternarme von 8 mm."),
        "merged": text("The stator flange lies behind the hub and is not seen; the duct beyond the spigot is the dark tone of the opening.",
                       "Der Statorflansch liegt hinter der Nabe und ist nicht zu sehen; der Kanal hinter dem Stutzen ist der dunkle Ton der Öffnung."),
        "omitted": text("The cable gland and the label. The 4 mm proud screw caps in front and plan (0.08 mm on paper); screws, blades, hub and spider are hidden there.",
                        "Die Kabeldurchführung und das Typenschild. Die 4 mm vorstehenden Schraubenkappen in Vorderansicht und Grundriss (0,08 mm auf Papier); Schrauben, Flügel, Nabe und Stern sind dort verdeckt."),
        "thinElements": text("The blades are 31 mm wide at the hub (0.63 mm) and 57 mm at the tip as seen from the room; the spider arms are 8 mm (0.16 mm) and the cap is drawn with the 0.13 mm detail pen.",
                             "Die Flügel sind vom Raum aus gesehen an der Nabe 31 mm (0,63 mm) und an der Spitze 57 mm breit; die Sternarme messen 8 mm (0,16 mm), die Kappe wird mit dem 0,13-mm-Detailstift gezeichnet."),
        "hatching": text("None; the opening is a dark texture tone with a 0.1 mm texture pen: it is the duct seen through the fan, not a part, so it does not count as body.",
                         "Keine; die Öffnung ist ein dunkler Texturton mit einem Texturstift von 0,1 mm: sie ist der Kanal, durch den Ventilator gesehen, kein Bauteil, und zählt daher nicht als Körper."),
        "styles": text("Plate 0.88, nozzle ring 0.7, opening 0.12, spider 0.6, blades 0.82, hub 0.55, cap 0.9; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
                       "Platte 0,88, Düsenring 0,7, Öffnung 0,12, Stern 0,6, Flügel 0,82, Nabe 0,55, Kappe 0,9; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": text("Casing, blades and hub are three regions: in the thermal view the hub (motor) warms and the blades stay cool; the blades turn with the rotor bone.",
                         "Gehäuse, Flügel und Nabe sind drei Regionen: in der Thermalansicht erwärmt sich die Nabe (Motor), die Flügel bleiben kühl; die Flügel drehen sich mit dem Rotorknochen."),
    },
    "100": {
        "silhouette": text("A 2.2 mm square with a ring round the bore, a five-bladed pinwheel and the spider cross behind it. Front and plan, 0.9 mm by 2.2 mm: plate and spigot as two masses.",
                           "Ein Quadrat von 2,2 mm mit einem Ring um die Bohrung, einem fünfflügeligen Windrad und dem Sternkreuz dahinter. Vorderansicht und Grundriss, 0,9 mm auf 2,2 mm: Platte und Stutzen als zwei Massen."),
        "retained": text("Plate with the bore, nozzle ring, opening, five swept blades as the plan-form of the 1:100 model, the 72 mm hub and the 15 mm spider arms.",
                         "Platte mit Bohrung, Düsenring, Öffnung, fünf gepfeilte Flügel im Grundriss des 1:100-Modells, die Nabe von 72 mm und die Sternarme von 15 mm."),
        "merged": text("Screw caps and spinner cap are dropped; the hub carries the cap.",
                       "Schrauben- und Nabenkappen entfallen; die Nabe trägt die Kappe."),
        "omitted": text("Four screw caps, the hub cap and the stator flange. Bead and seam line in front and plan.",
                        "Vier Schraubenkappen, die Nabenkappe und der Statorflansch. Wulst und Fugenlinie in Vorderansicht und Grundriss."),
        "thinElements": text("The blades are 30 mm wide at the hub (0.3 mm) and 58 mm at the tip; the spider arms are drawn at the 15 mm printable width.",
                             "Die Flügel sind an der Nabe 30 mm (0,3 mm) und an der Spitze 58 mm breit; die Sternarme werden mit der druckbaren Breite von 15 mm gezeichnet."),
        "hatching": text("None; the opening is a dark texture tone (0.1 mm texture pen), the duct seen through the fan.",
                         "Keine; die Öffnung ist ein dunkler Texturton (Texturstift 0,1 mm), der Kanal durch den Ventilator gesehen."),
        "styles": text("Plate 0.88, ring 0.7, opening 0.12, spider 0.6, blades 0.82, hub 0.55; one body pen of 0.18 mm.",
                       "Platte 0,88, Ring 0,7, Öffnung 0,12, Stern 0,6, Flügel 0,82, Nabe 0,55; ein Körperstift von 0,18 mm."),
        "overlays": text("Casing, blades and hub stay three regions, the blades turn with the rotor.",
                         "Gehäuse, Flügel und Nabe bleiben drei Regionen, die Flügel drehen sich mit dem Rotor."),
    },
    "200": {
        "silhouette": text("A 1.1 mm square with a dark 192 mm bore, a three-bladed pinwheel and the spider cross. Front and plan, 0.45 mm by 1.1 mm: the 15 mm plate and the spigot as two masses.",
                           "Ein Quadrat von 1,1 mm mit einer dunklen Bohrung von 192 mm, einem dreiflügeligen Windrad und dem Sternkreuz. Vorderansicht und Grundriss, 0,45 mm auf 1,1 mm: die Platte von 15 mm und der Stutzen als zwei Massen."),
        "retained": text("Plate with the bore, dark opening, three broad swept blades, the hub and the four 30 mm spider arms that hold it.",
                         "Platte mit Bohrung, dunkle Öffnung, drei breite gepfeilte Flügel, die Nabe und die vier Sternarme von 30 mm, die sie halten."),
        "merged": text("The thin housing tube lies behind the plate edge of the bore; five blades become three.",
                       "Das dünne Gehäuserohr liegt hinter dem Bohrungsrand der Platte; aus fünf Flügeln werden drei."),
        "omitted": text("Screws, hub cap, nozzle ring. Everything but plate and spigot in front and plan.",
                        "Schrauben, Nabenkappe, Düsenring. In Vorderansicht und Grundriss alles außer Platte und Stutzen."),
        "thinElements": text("The three blades are 50 to 85 mm wide (0.25 to 0.43 mm), the hub 88 mm and the spider arms 30 mm, so that they stay above the feature width and read as a pinwheel when they turn.",
                             "Die drei Flügel sind 50 bis 85 mm (0,25 bis 0,43 mm), die Nabe 88 mm und die Sternarme 30 mm breit, damit sie über der Mindestbreite bleiben und sich beim Drehen als Windrad lesen."),
        "hatching": text("None; the opening is a dark texture tone (0.1 mm texture pen), the duct seen through the fan.",
                         "Keine; die Öffnung ist ein dunkler Texturton (Texturstift 0,1 mm), der Kanal durch den Ventilator gesehen."),
        "styles": text("Plate 0.88, opening 0.12, spider 0.6, blades 0.82, hub 0.55; one body pen of 0.13 mm.",
                       "Platte 0,88, Öffnung 0,12, Stern 0,6, Flügel 0,82, Nabe 0,55; ein Körperstift von 0,13 mm."),
        "overlays": text("The plate and spider carry the casing region; blades and hub keep their regions.",
                         "Platte und Stern tragen die Gehäuseregion; Flügel und Nabe behalten ihre Regionen."),
    },
}


def main():
    """💾 Build and write the candidate."""
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    vol = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    a["volumes"] = vol
    a["summary"] = text("A 0.22 m square wall-mounted axial extract fan seen from the room: a rounded plate with a bell-mouth nozzle, five swept and pitched blades on an external-rotor motor held by a four-arm spider, extracting 180 m3/h at 28 W.",
                        "Ein quadratischer Axial-Wandventilator von 0,22 m zur Abluft, vom Raum aus gesehen: eine Platte mit Einlaufdüse und fünf gepfeilten, angestellten Flügeln an einem Außenläufermotor, den ein vierarmiger Stern hält; er saugt 180 m3/h bei 28 W ab.")
    refs = [r for r in a["provenance"]["references"] if r["id"] not in ("vents-150-m", "maico-ezq-ezs")]
    refs += [
        {"id": "vents-150-m", "citation": "VENTS 150 M and 150 VKO axial fans, product data (ventilation-system.com, read 2026-11-20): plastic casing and impeller, spigot 150 mm (VKO flange 154 mm), front 206 x 182 mm, 106 to 108 mm deep, 24 W, 295 to 298 m3/h at 2400 rpm, 0.8 to 0.89 kg."},
        {"id": "maico-ezq-ezs", "citation": "Maico EZQ and EZS axial wall fans, product pages (maico-ventilatoren.com, read 2026-11-20): wall fans of nominal size 200 built either on a square wall plate (EZQ) or a steel wall ring (EZS), with the impeller in the nozzle and the motor held behind it."},
    ]
    a["provenance"]["references"] = refs
    a["provenance"]["notes"] = text(
        "A bathroom-class axial wall fan seen face-on from the room, built like a plate-type wall-ring fan: a moulded ABS plate of 0.22 m by 0.22 m and 12 mm with rounded corners and four screw caps, a 0.2 m bore with a bell-mouth inlet nozzle into a 176 mm throat, the spigot sleeve behind it (0.2 m outside, 195 mm bore, a 208 mm sealing bead at its end) and a four-arm motor spider carrying an external-rotor motor: the 68 mm steel can is the hub and carries five swept, pitched and twisted ABS blades (chord 40 to 62 mm, pitch 38 to 24 degrees) that run 1 mm inside the throat. It stands for the circle-with-blades symbol of the ventilation clips. The origin lies on the fan axis at the exit of the rotor stage; the plate face is at w = -0.045 m (room) and its back lies on the wall at w = -0.033 m, the spigot reaches w = 0.045 m (depthM 0.09). Dimensions and the mass of 0.9 kg are rounded from this product class (Vents 150 M: 206 x 182 x 106 mm, 0.89 kg, 24 W); the mass is split into 0.2 kg plate, 0.45 kg housing with spider and stator and 0.25 kg rotor with blades (authored estimate). At level 1 the fan extracts 0.05 m3/s (180 m3/h) with a motor power of 28 W (6 W electronics and housing, 22 W motor), at level 0.5 half the flow and 14 W, which also follows a typical axial fan: power grows faster than the flow, so the linear model is a slight overestimate of the motor heat at the low stage. The rotor turns around the axis normal to the drawing; the clip shows the real 1400 rpm slowed to 180 and 360 degrees per second.",
        "Ein Axial-Wandventilator der Badklasse, frontal vom Raum aus gesehen, gebaut wie ein Wandringventilator mit Platte: eine gespritzte ABS-Platte von 0,22 m mal 0,22 m und 12 mm mit gerundeten Ecken und vier Schraubenkappen, eine Bohrung von 0,2 m mit einer Einlaufdüse in einen Hals von 176 mm, dahinter die Stutzenhülse (0,2 m außen, Bohrung 195 mm, ein Dichtwulst von 208 mm am Ende) und ein vierarmiger Motorstern, der einen Außenläufermotor trägt: der Stahlbecher von 68 mm ist die Nabe und trägt fünf gepfeilte, angestellte und verwundene ABS-Flügel (Sehne 40 bis 62 mm, Anstellung 38 bis 24 Grad), die 1 mm innerhalb des Halses laufen. Er steht für das Symbol aus Kreis mit Flügeln der Lüftungsclips. Der Ursprung liegt auf der Ventilatorachse am Ende der Laufradstufe; die Plattenfront liegt bei w = -0,045 m (Raum), ihre Rückseite bei w = -0,033 m an der Wand, der Stutzen reicht bis w = 0,045 m (depthM 0,09). Maße und die Masse von 0,9 kg sind aus dieser Produktklasse gerundet (Vents 150 M: 206 x 182 x 106 mm, 0,89 kg, 24 W); die Masse teilt sich in 0,2 kg Platte, 0,45 kg Gehäuse mit Stern und Stator und 0,25 kg Laufrad mit Flügeln (Schätzwert). Auf Stufe 1 saugt er 0,05 m3/s (180 m3/h) mit einer Motorleistung von 28 W ab (6 W Elektronik und Gehäuse, 22 W Motor), bei Stufe 0,5 den halben Strom und 14 W; das folgt auch einem üblichen Axialventilator: die Leistung wächst stärker als der Volumenstrom, das lineare Modell überschätzt die Motorwärme auf der kleinen Stufe also leicht. Das Laufrad dreht sich um die Achse senkrecht zur Zeichnung; der Clip zeigt die echten 1400 U/min verlangsamt auf 180 und 360 Grad je Sekunde.")
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["plate"]["mass"]["kg"] = 0.2
    bones["ring"]["mass"]["kg"] = 0.45
    bones["rotor"]["mass"]["kg"] = 0.25
    bones["ring"]["title"] = text("Housing with nozzle, spigot, motor spider and stator", "Gehäuse mit Düse, Stutzen, Motorstern und Stator")
    bones["rotor"]["title"] = text("External rotor with blades", "Außenläufer mit Flügeln")
    anchors = {x["id"]: x for x in a["anchors"]}
    anchors["wall-screw-top"].update(x=-0.092, y=0.092)
    anchors["wall-screw-top"]["title"] = text("Upper left screw in the wall", "Obere linke Schraube in der Wand")
    anchors["wall-screw-bottom"].update(x=0.092, y=-0.092)
    anchors["wall-screw-bottom"]["title"] = text("Lower right screw in the wall", "Untere rechte Schraube in der Wand")
    regions = {r["id"]: r for r in a["thermal"]["regions"]}
    regions["casing"]["areaM2"] = CASING_AREA
    regions["hub"]["areaM2"] = HUB_AREA
    regions["casing"]["title"] = text("Plate, nozzle, spigot and motor spider", "Platte, Düse, Stutzen und Motorstern")
    a["structure"]["assumptions"] = [text(
        "The 0.9 kg fan is held by four screws in the plate corners; the idealisation takes the upper left screw as the pin that carries the weight and the lower right one as the roller against the wall, so the pair carries the weight and the small overturning moment of the 90 mm deep housing. The motor hangs on the four spider arms in the spigot, the rotor on its shaft bearing.",
        "Der 0,9 kg schwere Ventilator hängt an vier Schrauben in den Plattenecken; die Idealisierung nimmt die Schraube oben links als Festlager, das das Gewicht trägt, und die unten rechts als Rollenlager an der Wand, so trägt das Paar das Gewicht und das kleine Kippmoment des 90 mm tiefen Gehäuses. Der Motor hängt an den vier Sternarmen im Stutzen, das Laufrad an seinem Wellenlager.")]
    reps = a["representations"]
    for scale in ("50", "100", "200"):
        geo = reps[scale]["geometry"]
        geo["elevation"] = elevation(geo["elevation"], scale, vol[scale])
        geo["front"] = side(geo["front"], scale)
        geo["plan"] = side(geo["plan"], scale)
        reps[scale]["decisions"] = DECISIONS[scale]
        reps[scale]["pens"]["texture"] = 0.1
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, indent=2, ensure_ascii=False)
        fh.write("\n")
    print("written", OUT)


if __name__ == "__main__":
    main()
