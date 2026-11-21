"""🔥 Round 5 panel radiator: the round-3 volumes of 1:50, 1:100 and 1:200 (`r3-panel-radiator-build.py`) with the emitters as flush outlet patches, and the digital 1:500 handcrafted anew (radiator mass, four bracket blocks to the wall, head, connection column; thermal layers; header-and-post frame structure).

Writes `volumes/components/panel-radiator.json`.

Usage: python3 r5-panel-radiator-build.py

@see ./r5-brief.md
@see ./r3-components-panel-radiator.md
"""
import importlib.util
import math
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("r3build", os.path.join(T, "r3-panel-radiator-build.py"))
B = importlib.util.module_from_spec(spec)
spec.loader.exec_module(B)
K = B.K
span_box, revolve, ellipsoid, ref, text = K.span_box, K.revolve, K.ellipsoid, B.ref, B.text
H, W = K.HALF, K.WALL
F500 = 0.075
PROUD = 0.0015


def emitters(t):
    """💨 The two emitters as outlet patches of the scale's thickness `t`, sunk into the surface the air leaves and standing only 1.5 mm proud: over the grille slots (0.0284 m2, the 0.19 m nozzle) and on the middle of the front panel (the 0.1 m nozzle)."""
    side = 0.0886
    return [
        dict(span_box("th-emitter-convection-outlet", (K.TOP - t + PROUD, K.TOP + PROUD), (-0.095, -0.005), (-0.1575, 0.1575), tone=0.5), region="top-grille", tags=["convection-outlet"]),
        dict(span_box("th-emitter-front-plume", (0.2925 - side / 2, 0.2925 + side / 2), (-0.1 - PROUD, -0.1 - PROUD + t), (-side / 2, side / 2), tone=0.5), region="panel-front-mid", tags=["front-plume"]),
    ]


B.emitters = emitters

BRACKET_U = ((0.4275, 0.5025), (0.0125, 0.0875))
BRACKET_W = ((0.3625, 0.4375), (-0.4375, -0.3625))
COLUMN = {"u": (K.FLOOR_U, 0.0), "v": (-0.0875, -0.0125), "w": (-0.49, -0.39)}


def body500():
    """🏗️ 1:500 body at the 75 mm feature: the radiator as one rounded mass of panels, fins and grille, the four brackets as 75 mm blocks bridging the wall gap, the thermostat head at the 75 mm floor and the H-block with both pipes as one column to the floor."""
    out = [span_box("radiator", (0.0, K.TOP), (-0.1, 0.0), (-H, H), tone=0.65)]
    for i, u in enumerate(BRACKET_U):
        for j, w in enumerate(BRACKET_W):
            out.append(span_box("bracket-" + ("upper" if i == 0 else "lower") + "-" + ("left" if j == 0 else "right"), u, (0.0, W), w, role="detail", tone=0.4))
    out += [
        K.extrude("valve-head", "xy", [(0.0375 * math.cos(i * math.pi / 6), 0.0375 * math.sin(i * math.pi / 6)) for i in range(12)], -0.595, -0.5, bone="valve-head", material=B.PLASTIC, role="detail", tone=0.85),
        span_box("connection-column", COLUMN["u"], COLUMN["v"], COLUMN["w"], material="brass", role="detail", tone=0.3),
    ]
    return out


def thermal500():
    """🌡️ 1:500 thermal: the mass in its layers (rear panel 30 mm, convector core 40 mm with the grille as its top 75 mm, front panel 30 mm in three bands of 0.2 m), the bracket blocks with their wall face as mounting face, the valve as head and column, flush outlet patches."""
    out = [dict(span_box("th-panel-rear", (0.0, K.TOP), (-0.03, 0.0), (-H, H)), region="panel-rear")]
    for k, (name, region) in enumerate((("lower", "panel-front-lower"), ("mid", "panel-front-mid"), ("upper", "panel-front-upper"))):
        out.append(dict(span_box("th-band-" + name, (k * 0.2, (k + 1) * 0.2), (-0.1, -0.07), (-H, H)), region=region))
    out += [
        dict(span_box("th-convector", (0.0, K.TOP - F500), (-0.07, -0.03), (-H, H)), region="convector"),
        dict(span_box("th-top-grille", (K.TOP - F500, K.TOP), (-0.07, -0.03), (-H, H)), region="top-grille"),
    ]
    for i in range(2):
        for j in range(2):
            sid = "bracket-" + ("upper" if i == 0 else "lower") + "-" + ("left" if j == 0 else "right")
            out.append(ref("th-" + sid, sid, region="bracket", sectionFaces=["+v"]))
    out += [ref("th-valve-head", "valve-head", region="valve"), ref("th-connection-column", "connection-column", region="valve")]
    return out + emitters(F500)


def structure500():
    """🦴 1:500 structure: the water panels idealised as their stiff ring (top and bottom header as 75 mm bars, the two end channels as 75 mm posts), the four wall fixings as 75 mm supports reaching from the frame to the wall, the valve spindle joint, the radiator mass at its centre."""
    v = (-0.0875, -0.0125)
    out = []
    for sid, u, w in (
        ("member-header-top", (K.TOP - F500, K.TOP), (-H, H)),
        ("member-header-bottom", (0.0, F500), (-H, H)),
        ("member-post-left", (F500, K.TOP - F500), (H - F500, H)),
        ("member-post-right", (F500, K.TOP - F500), (-H, -H + F500)),
    ):
        s = span_box(sid, u, v, w, tone=0.6)
        s.update(member="radiator", part="member")
        out.append(s)
    for name, w in (("left", BRACKET_W[0]), ("right", BRACKET_W[1])):
        s = span_box("support-hook-" + name, BRACKET_U[0], (W - F500, W), w, tone=0.4)
        s.update(member="radiator", part="support", anchor="hook-" + name)
        p = span_box("support-rest-" + name, BRACKET_U[1], (W - F500, W), w, tone=0.4)
        p.update(member="radiator", part="support", anchor="rest-" + name)
        out += [s, p]
    j = span_box("joint-valve-head", (K.HEAD_U - F500 / 2, K.HEAD_U + F500 / 2), (K.HEAD_V - F500 / 2, K.HEAD_V + F500 / 2), (-0.5 - F500, -0.5), tone=0.5)
    j.update(member="valve-head", part="joint")
    m = ellipsoid("mass-radiator", (0.3, -0.05, 0), (0.0375, 0.0375, 0.0375), tone=0.5)
    m.update(member="radiator", part="mass")
    return out + [j, m]


DECISIONS500 = [
    text("Body (digital only, no drawing at this scale): the panels, fin packs and grille are one crisp mass of 0.6 x 0.1 x 1.0 m (its 8 mm edge radius is below the feature); the four brackets are 75 mm blocks bridging the 35 mm wall gap, so the radiator visibly hangs on the wall; the thermostat head is raised to the 75 mm floor (real 52 mm); the H-block and both pipes are one 100 x 75 mm column from the valve end to the floor.", "Körper (nur digital, keine Zeichnung in diesem Maßstab): Platten, Lamellenpakete und Gitter sind eine klare Masse von 0,6 x 0,1 x 1,0 m (ihr Kantenradius von 8 mm liegt unter dem Merkmal); die vier Konsolen sind 75-mm-Blöcke über dem Wandabstand von 35 mm, so dass der Heizkörper sichtbar an der Wand hängt; der Thermostatkopf ist auf die Untergrenze von 75 mm angehoben (real 52 mm); Hahnblock und beide Rohre sind eine Säule von 100 x 75 mm vom Ventilende bis zum Boden."),
    text("Thermal: the mass is layered as the radiator is built: rear panel 30 mm, convector core 40 mm with the top grille as its upper 75 mm, front panel 30 mm in three bands of 0.2 m; the bracket blocks with their wall face as mounting face; valve = head and column; the two outlet patches sit flush in the grille and the front face.", "Thermisch: die Masse ist geschichtet wie der Heizkörper gebaut ist: Rückplatte 30 mm, Konvektorkern 40 mm mit dem Abdeckgitter als seinen oberen 75 mm, Frontplatte 30 mm in drei Bändern von 0,2 m; die Konsolblöcke mit ihrer Wandseite als Montagefläche; Ventil = Kopf und Säule; die beiden Auslassflächen liegen bündig im Gitter und in der Front."),
    text("Minimal: the thermal list is already as simple as the regions allow (one solid per region, the four bracket blocks the radiator hangs on, the two outlet patches), so it keeps the count of 1:200 while body and structure drop.", "Minimal: die thermische Liste ist bereits so einfach, wie die Regionen es zulassen (ein Körper je Region, die vier Konsolblöcke, an denen der Heizkörper hängt, die beiden Auslassflächen), daher behält sie die Anzahl von 1:200, während Körper und Tragwerk abnehmen."),
    text("Structure: the two water panels act as one stiff ring, idealised as the top and bottom headers (75 mm bars) and the two end channels (75 mm posts); the ring hangs on the two upper wall fixings and is held off the wall by the two lower ones (75 mm supports from the frame to the wall plane); the valve spindle joint; the radiator mass with water at its centre.", "Tragwerk: die beiden Wasserplatten wirken als ein steifer Ring, idealisiert als oberer und unterer Sammelkanal (75-mm-Stäbe) und die beiden Endkanäle (75-mm-Pfosten); der Ring hängt an den beiden oberen Wandbefestigungen und wird von den beiden unteren auf Abstand gehalten (75-mm-Auflager vom Rahmen bis zur Wandebene); das Gelenk der Ventilspindel; die Heizkörpermasse mit Wasser in ihrem Schwerpunkt."),
]


def column200(v):
    """🔩 1:200: the H-block with its unions and both pipes as one 100 x 36 mm connection column from the floor to the radiator, as the 1:200 drawing draws it (the pipes alone fall below the printable feature at 1:350)."""
    s = v["200"]
    s["body"] = [b for b in s["body"] if b["id"] not in ("connection-block", "pipes")] + [span_box("connection-column", (K.FLOOR_U, 0.0), K.BLOCK["v"], K.BLOCK["w"], material="brass", role="detail", tone=0.3)]
    s["thermal"] = [t for t in s["thermal"] if t["id"] not in ("th-connection-block", "th-pipes")] + [ref("th-connection-column", "connection-column", region="valve")]
    s["decisions"][0] = text(
        "Body: the panels, fin packs and grille are one rounded mass of 0.6 x 0.1 x 1.0 m, as the drawing of this scale fills it (the 56 mm gap between the panels is 0.28 mm on paper); the upper and lower brackets of each side are 30 mm blocks in the wall gap; the head stays at its real 52 mm; the H-block with its unions and both pipes are one 100 x 36 mm connection column from the floor to the radiator, as the drawing of this scale draws it.",
        "Körper: Platten, Lamellenpakete und Gitter sind eine gerundete Masse von 0,6 x 0,1 x 1,0 m, wie die Zeichnung dieses Maßstabs sie füllt (der Spalt von 56 mm zwischen den Platten misst auf Papier 0,28 mm); die oberen und unteren Konsolen jeder Seite sind 30-mm-Blöcke im Wandabstand; der Kopf bleibt bei seinen echten 52 mm; Hahnblock mit Verschraubungen und beide Rohre sind eine Anschlusssäule von 100 x 36 mm vom Boden bis zum Heizkörper, wie die Zeichnung dieses Maßstabs sie zeichnet.",
    )
    s["decisions"][1] = text(
        "Thermal: the merged mass is split into the regions it holds: rear panel 30 mm, convector core 40 mm, three front bands of 0.2 m and 30 mm, the grille as the top 30 mm of the core; brackets, valve as head and connection column; outlet patches flush with the grille and the front face.",
        "Thermisch: die zusammengefasste Masse ist in ihre Regionen geteilt: Rückplatte 30 mm, Konvektorkern 40 mm, drei Frontbänder von 0,2 m und 30 mm, das Gitter als die oberen 30 mm des Kerns; Konsolen, Ventil als Kopf und Anschlusssäule; Auslassflächen bündig in Gitter und Front.",
    )


def channels100(v):
    """〰️ 1:100: the pressed channels of the front panel as texture strips at every second seam (66.7 mm), between the header lines, as the 1:100 front draws them."""
    seam = span_box("panel-channels", (0.04, 0.53), (-0.1012, -0.0996), (-0.5 + 2 / 30 - 0.002, -0.5 + 2 / 30 + 0.002), role="texture", tone=0.45, texture=True, repeat={"count": 14, "translation": [0, 0, K.r5(2 / 30)]})
    v["100"]["body"].insert(2, seam)
    d = v["100"]["decisions"][0]
    d["en"] += " The pressed channels of the front panel show as texture strips at every second seam, as the 1:100 front draws them."
    d["de"] += " Die gepressten Kanäle der Frontplatte erscheinen als Texturstreifen an jeder zweiten Naht, wie die Vorderansicht 1:100 sie zeichnet."


def volumes():
    """📚 The `volumes` of every scale."""
    v = B.volumes()
    column200(v)
    channels100(v)
    for key, old_en, old_de in (("50", "outlet patches 7.5 mm proud", "Auslassflächen 7,5 mm über der Oberfläche"), ("100", "outlet patches 15 mm proud", "Auslassflächen 15 mm über der Oberfläche")):
        d = v[key]["decisions"][1]
        assert old_en in d["en"] and old_de in d["de"], key
        d["en"] = d["en"].replace(old_en, "outlet patches sunk into the surface and standing 1.5 mm proud")
        d["de"] = d["de"].replace(old_de, "Auslassflächen in die Oberfläche eingelassen, 1,5 mm vorstehend")
    v["500"] = {"status": "authored", "denominator": 500, "body": body500(), "thermal": thermal500(), "structure": structure500(), "decisions": DECISIONS500}
    return v


if __name__ == "__main__":
    v = volumes()
    with open(B.SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": v}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("written", B.SOURCE, {s: {k: len(v[s].get(k, [])) for k in ("body", "thermal", "structure")} for s in v})
