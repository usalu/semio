"""🌱 Round 3, furniture/dining-table: the root corrections outside `volumes` and the 2D shapes: anchors in the frame of the moved half bones, the leg foot 45 mm square, the mass from the modelled parts, references and notes, the structural assumption of the slide and the leaf, and the true-extent symbol.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_dining_table_kit", os.path.join(HERE, "r3-dining-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

REFERENCES = [
    {"id": "hoeffner-oak-160x90", "citation": "Höffner product listings of solid-wood dining tables 160 x 90 cm (overall height 76 cm, tops 22-27 mm, underside heights 60-75 cm, extension leaves of 50-60 cm), hoeffner.de articles 40305726, 13302906 and 50308493, read 2026-11-19."},
    {"id": "haefele-table-slides", "citation": "Häfele table fittings: ball-bearing guides for one insert leaf, synchronous or asynchronous, for frame tables (hwt-pro.com article 5765, read 2026-11-19), and the DABAG table slide data sheet 68_1242_30 (steel outer profile with inner profile, 850 mm for a 500 mm leaf), read 2026-11-19."},
]

NOTES = {
    "en": "Solid oak table 1.60 x 0.90 x 0.75 m closed, 2.20 m extended. Each half is a 30 mm top (15 mm underside bevel on the free edges) over a 30 mm shadow gap, a frame of 25 x 80 mm aprons tenoned into two legs (70 mm square under the apron, tapered on all faces to 45 mm at the floor, knee clearance 0.61 m), held to the top by oak blocks in the gap. The halves run on a synchronised telescopic steel slide (tube in tube): a 25 x 12 mm tube of 2 mm sheet on each long apron and a stationary 21 x 8 mm centre bar per side running in the tubes of both halves. The 0.60 m leaf is stored in the shadow gap on the aprons and runners and rises 30 mm into the opening, dowelled to both halves. Masses from the modelled volumes x 700 kg/m3 (oak) and 7800 kg/m3 (steel).",
    "de": "Massiver Eichentisch 1,60 x 0,90 x 0,75 m geschlossen, 2,20 m ausgezogen. Jede Hälfte ist eine 30-mm-Platte (15 mm Unterfase an den freien Kanten) über einer 30-mm-Schattenfuge und ein Rahmen aus Zargen 25 x 80 mm, in zwei Beine gezapft (unter der Zarge 70 mm im Quadrat, allseitig auf 45 mm am Boden verjüngt, Beinfreiheit 0,61 m), mit Eichenklötzen in der Fuge an der Platte befestigt. Die Hälften laufen auf einem synchronen Teleskopauszug aus Stahl (Rohr in Rohr): je Längszarge ein Rohr 25 x 12 mm aus 2 mm Blech und je Seite ein feststehender Mittelstab 21 x 8 mm, der in den Rohren beider Hälften läuft. Die 0,60-m-Einlage liegt verstaut in der Schattenfuge auf Zargen und Schienen und hebt sich 30 mm in die Öffnung, mit beiden Hälften verdübelt. Massen aus Modellvolumen x 700 kg/m3 (Eiche) und 7800 kg/m3 (Stahl).",
}

ASSUMPTION = {
    "en": "Each table half is a rigid body with its two legs on the floor; the four legs are four contacts (x = 0.135 m from each end, z = +-0.375 m); the halves slide 0.30 m each on the synchronised slide (prismatic joints in the plane of the top), whose centre bars stay in place; the leaf rises 0.03 m from its store in the shadow gap into the opening and is dowelled to both halves.",
    "de": "Jede Tischhälfte ist ein starrer Körper mit ihren zwei Beinen am Boden; die vier Beine sind vier Kontakte (x = 0,135 m von jedem Ende, z = +-0,375 m); die Hälften gleiten je 0,30 m auf dem Gleichlaufauszug (Schiebegelenke in der Plattenebene), dessen Mittelstäbe stehen bleiben; die Einlage hebt sich 0,03 m aus ihrem Platz in der Schattenfuge in die Öffnung und wird mit beiden Hälften verdübelt.",
}


def anchors(old):
    """⚓ Leg contacts at the floor and the edge test point in the frame of the half bones (y 0.735 m), feet 45 x 45 mm."""
    out = []
    for anchor in old:
        anchor = dict(anchor)
        if anchor["id"] == "edge-right":
            anchor["y"] = K.r6(K.TOP_Y1 - K.CREASE_Y)
        if anchor.get("kind") == "contact":
            anchor["y"] = K.r6(-K.CREASE_Y)
            anchor["footprintLengthM"] = 0.045
            anchor["footprintWidthM"] = 0.045
        out.append(anchor)
    return out


def symbol(old):
    """🔣 The true-extent symbol of the closed table, redrawn to the construction."""
    sym = dict(old)
    g = lambda sid, role, z, filled, tone, geom: dict({"id": sid, "bone": "ground", "role": role, "z": z, "filled": filled, "tone": tone}, **geom)
    rect = lambda x, y, w, h: {"kind": "rect", "x": x, "y": y, "w": w, "h": h}
    line = lambda pts, closed=False: {"kind": "poly", "points": pts, "closed": closed}
    sym["geometry"] = {
        "elevation": [
            g("top", "body", 3, True, 0.85, rect(0, 0.72, 1.6, 0.03)),
            g("apron", "detail", 2, False, 0.5, rect(0.17, 0.61, 1.26, 0.08)),
            g("leg-l", "body", 2.5, True, 0.85, line([[0.1125, 0], [0.1575, 0], [0.17, 0.58], [0.17, 0.72], [0.1, 0.72], [0.1, 0.58]], True)),
            g("leg-r", "body", 2.5, True, 0.85, line([[1.4425, 0], [1.4875, 0], [1.5, 0.58], [1.5, 0.72], [1.43, 0.72], [1.43, 0.58]], True)),
            g("rail", "detail", 2, False, 0.5, line([[0.3, 0.6775], [1.3, 0.6775]])),
            g("leaf", "detail", 3.5, False, 0.5, line([[0.5, 0.72], [0.5, 0.69], [1.1, 0.69], [1.1, 0.72]])),
        ],
        "front": [
            g("top", "body", 3, True, 0.85, rect(-0.45, 0.72, 0.9, 0.03)),
            g("apron", "detail", 2, False, 0.5, rect(-0.34, 0.61, 0.68, 0.08)),
            g("leg-left", "body", 2.5, True, 0.85, line([[0.3525, 0], [0.3975, 0], [0.41, 0.58], [0.41, 0.72], [0.34, 0.72], [0.34, 0.58]], True)),
            g("leg-right", "body", 2.5, True, 0.85, line([[-0.3975, 0], [-0.3525, 0], [-0.34, 0.58], [-0.34, 0.72], [-0.41, 0.72], [-0.41, 0.58]], True)),
        ],
        "plan": old["geometry"]["plan"],
    }
    sym["description"] = {
        "en": "The extending dining table closed in all three views: top 1.60 x 0.90 m at 0.75 m over a 30 mm shadow gap, aprons, four tapered legs, the slide and the 0.60 m leaf outline.",
        "de": "Der Ausziehtisch geschlossen in allen drei Ansichten: Platte 1,60 x 0,90 m auf 0,75 m über einer 30-mm-Schattenfuge, Zargen, vier verjüngte Beine, der Auszug und der Umriss der 0,60-m-Einlegeplatte.",
    }
    return sym


def apply(asset):
    """🌱 Writes the root corrections into the asset in memory."""
    m = K.masses()
    asset["dimensions"] = dict(asset["dimensions"], massKg=round(2 * m["half"][0] + m["leaf"][0] + m["frame"][0], 1))
    asset["anchors"] = anchors(asset["anchors"])
    prov = asset["provenance"]
    known = {r["id"] for r in prov["references"]}
    prov["references"] = prov["references"] + [r for r in REFERENCES if r["id"] not in known]
    prov["notes"] = NOTES
    asset["structure"]["assumptions"][0] = ASSUMPTION
    for region in asset["thermal"]["regions"]:
        if region["id"] == "slide-rails":
            region["areaM2"] = 0.2
    asset["symbols"] = [symbol(asset["symbols"][0])] + asset["symbols"][1:]
