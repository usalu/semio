"""✏ Round 3, furniture/picnic-table: the 2D drawings of every scale as the true orthographic views of that scale's body (ruling E1), drawn from the same hand-stated parts as the solids: end elevation (seen from -Z: the near A-frame with its lapped cleat and stretcher in front of the legs, the bench ends and gusset brackets behind them), front (the long side seen from +X: plank edges, both A-frames edge-on, braces, the near bench on its brackets) and plan. Every part states per view whether it shows and where it lies in the paint order; hidden parts are not drawn.
"""
import importlib.util
import os

from shapely.geometry import Polygon
from shapely.geometry.polygon import orient

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_picnic_table_kit", os.path.join(HERE, "r3-picnic-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)


def n(v):
    """🔢 A coordinate as written in the file."""
    return round(v + 0.0, 4)


def poly(points):
    """🔷 A closed polygon."""
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in points], "closed": True}


def rect(x0, y0, x1, y1):
    """▭ A rectangle between two corners."""
    return {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}


def local_ring(bone, points):
    """⭕ World (X, Y) points in the elevation frame of a bone."""
    return [K.local(bone, x, y)[:2] for x, y in points]


def outline(bone, p):
    """🖊 The elevation geometry of a part in its bone frame."""
    if p["kind"] == "px":
        ys = [y for _, y in p["profile"]]
        return poly(local_ring(bone, K.rect(p["span"][0], min(ys), p["span"][1], max(ys))))
    ring = local_ring(bone, p["profile"])
    if "holes" not in p:
        return poly(ring)
    rings = [list(orient(Polygon(ring), 1.0).exterior.coords)[:-1]] + [local_ring(bone, h) for h in p["holes"]]
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{n(x)} {n(y)}" for x, y in r) + " Z" for r in rings)}


def shifted(p, k):
    """➡ Copy k of a repeated part."""
    if k == 0:
        return p
    dx, dy, dz = (k * s for s in p["repeat"][1])
    q = dict(p)
    if p["kind"] == "pz":
        q["profile"] = [(x + dx, y + dy) for x, y in p["profile"]]
        q["span"] = (p["span"][0] + dz, p["span"][1] + dz)
    else:
        q["origin"] = (p["origin"][0] + dx, p["origin"][1] + dy, p["origin"][2] + dz)
    return q


def common(p, sid, view, scale):
    """🧾 The common members of a shape."""
    region, _ = K.REGION[p["group"]]
    role = "detail" if p["kind"] == "rev" else "body"
    out = {"id": sid, "bone": p["bone"], "role": role, "z": p["views"][view], "filled": True, "tone": p["tone"], "region": region, "material": p["material"]}
    if scale == "50" and p.get("hatch"):
        if view == "e":
            out["hatch"] = p["hatch"] if not sid.startswith("leg-l") else "grain-v-l"
        elif view == "p" and p["kind"] == "pz" and p["bone"] in ("top",):
            out["hatch"] = "grain-l"
    return out


def shapes(scale, view):
    """🗺 The shapes of one view of one scale."""
    out, seen = [], set()
    for p in K.PARTS[scale]():
        if p["views"][view] is None:
            continue
        bone = p["bone"]
        moving = bone.startswith("bench")
        sid0 = p["id"]
        if view == "e" and sid0.endswith("-b"):
            continue
        if view == "f" and not moving and "-l" in sid0:
            continue
        count = p.get("repeat", (1,))[0]
        for k in range(count):
            q = shifted(p, k)
            sid = sid0 if count == 1 else f"{sid0}-{k + 1}"
            if q["kind"] == "rev":
                if view != "e":
                    continue
                u, v, _ = K.local(bone, *q["origin"])
                r = max(b for _, b in q["profile"])
                geometry = {"kind": "ellipse", "cx": n(u), "cy": n(v), "rx": n(r), "ry": n(r)}
                extra = {"penMm": 0.13}
            elif view == "e":
                geometry, extra = outline(bone, q), {}
            elif moving:
                geometry, extra = outline(bone, q), {"solid": {"kind": "extruded", "sweep": "w", "z0": n(q["span"][0]), "z1": n(q["span"][1])}}
            elif q["kind"] == "pz":
                xs, ys = [x for x, _ in q["profile"]], [y for _, y in q["profile"]]
                z0, z1 = q["span"]
                geometry = rect(z0, min(ys), z1, max(ys)) if view == "f" else rect(min(xs), z0, max(xs), z1)
                extra = {}
            else:
                zs = [z for z, _ in q["profile"]]
                geometry = poly(q["profile"]) if view == "f" else rect(q["span"][0], min(zs), q["span"][1], max(zs))
                extra = {}
            key = repr((bone, geometry, extra))
            if key in seen:
                continue
            seen.add(key)
            shape = common(p, sid, view, scale)
            shape.update(geometry)
            shape.update(extra)
            out.append(shape)
    return out


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


HATCHES = [
    {"id": "grain-h", "title": T("Grain horizontal", "Maserung waagerecht"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13},
    {"id": "grain-v", "title": T("Grain along the right leg", "Maserung entlang des rechten Beins"), "pattern": "lines", "angleDeg": 120, "spacingMm": 0.6, "penMm": 0.13},
    {"id": "grain-v-l", "title": T("Grain along the left leg", "Maserung entlang des linken Beins"), "pattern": "lines", "angleDeg": 60, "spacingMm": 0.6, "penMm": 0.13},
    {"id": "grain-l", "title": T("Grain along the planks in plan", "Maserung entlang der Bohlen im Grundriss"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.13},
]

DECISIONS = {
    "50": {
        "silhouette": T("Picnic table 30 x 15 mm with its whole construction. End elevation: five plank ends with eased edges and drainage gaps, the near A-frame with bevelled cleat and stretcher lapped over the two raked legs, bolt heads in the laps, the brace end-on between stretcher and cleat, the bench plank ends with the gusset brackets behind the legs. Long side: plank edge, both A-frames edge-on in their two layers, the braces rising to the centre cleat, bearing plates, the bench between the frames on its gussets and flanges, the batten. Plan: five planks, the legs and stretchers reaching out under them, two planks per bench, the bracket webs between plank ends and legs.",
                        "Picknicktisch 30 x 15 mm mit seiner ganzen Konstruktion. Stirnansicht: fünf Bohlenenden mit gebrochenen Kanten und Wasserfugen, der vordere A-Bock mit gefastem Riegel und Zange über den zwei geneigten Beinen aufgeblattet, Schraubenköpfe in den Blättern, die Strebe stirnseitig zwischen Zange und Riegel, die Bankbohlenenden mit den Knotenblechkonsolen hinter den Beinen. Längsseite: Bohlenkante, beide A-Böcke hochkant in ihren zwei Lagen, die Streben zum Mittelriegel ansteigend, Lagerplatten, die Bank zwischen den Böcken auf Knotenblechen und Flanschen, die Leiste. Grundriss: fünf Bohlen, darunter hervortretend Beine und Zangen, zwei Bohlen je Bank, die Konsolstege zwischen Bohlenenden und Beinen."),
        "retained": T("Every plank with gap and eased edges, both timber layers of the A-frames, cleat bevels, braces, centre cleat, gusset web and flange, bearing plate with its stow hole, bolt heads and nuts of the laps.",
                      "Jede Bohle mit Fuge und gebrochenen Kanten, beide Holzlagen der A-Böcke, Riegelfasen, Streben, Mittelriegel, Knotenblechsteg und -flansch, Lagerplatte mit Rastloch, Schraubenköpfe und Muttern der Blätter."),
        "merged": T("Near and far A-frame coincide in the end elevation; left and right legs coincide edge-on in the long side.",
                    "Vorderer und hinterer A-Bock fallen in der Stirnansicht zusammen; linke und rechte Beine fallen in der Längsansicht hochkant zusammen."),
        "omitted": T("Plank screws (5 mm heads), pivot bolt heads and plungers (behind the legs in the end elevation, 8 mm proud in the long side), cleats and braces under the top in plan.",
                     "Bohlenschrauben (5-mm-Köpfe), Drehbolzenköpfe und Rastbolzen (in der Stirnansicht hinter den Beinen, in der Längsansicht 8 mm vorstehend), Riegel und Streben unter der Platte im Grundriss."),
        "thinElements": T("The 10 mm gaps are 0.2 mm on paper and stay open between the plank outlines; the 8 mm steel plates are 0.16 mm and drawn true.",
                          "Die 10-mm-Fugen sind auf Papier 0,2 mm und bleiben zwischen den Bohlenumrissen offen; die 8-mm-Stahlbleche sind 0,16 mm und wahr gezeichnet."),
        "hatching": T("Grain as lines every 0.6 mm: along the legs and horizontally on cleat, stretcher and plank ends in the end elevation, along the top planks in plan; steel plain tone.",
                      "Maserung als Linien alle 0,6 mm: entlang der Beine und waagerecht auf Riegel, Zange und Bohlenenden in der Stirnansicht, entlang der Tischbohlen im Grundriss; Stahl nur Tonfläche."),
        "styles": T("Planks light (0.7), legs 0.55, cleats and stretchers 0.5, braces 0.46, steel dark (0.22-0.25), bolts near black.",
                    "Bohlen hell (0,7), Beine 0,55, Riegel und Zangen 0,5, Streben 0,46, Stahl dunkel (0,22-0,25), Schrauben fast schwarz."),
        "overlays": T("Every shape carries one region (table-top, bench-seats, a-frame, hinges).", "Jede Form trägt genau eine Region (table-top, bench-seats, a-frame, hinges)."),
    },
    "100": {
        "silhouette": T("Picnic table 15.2 x 7.5 mm: square plank ends with their gaps, the A-frame with cleat and stretcher over the legs, the brace end-on, bench planks and one bracket plate behind each leg. Long side: plank edge, the A-frames in two layers, braces, the bench on its bracket plates with the batten. Plan: planks, legs, stretchers, bench planks, bracket plates.",
                        "Picknicktisch 15,2 x 7,5 mm: kantige Bohlenenden mit ihren Fugen, der A-Bock mit Riegel und Zange über den Beinen, die Strebe stirnseitig, Bankbohlen und je ein Konsolblech hinter jedem Bein. Längsseite: Bohlenkante, die A-Böcke in zwei Lagen, Streben, die Bank auf ihren Konsolblechen mit der Leiste. Grundriss: Bohlen, Beine, Zangen, Bankbohlen, Konsolbleche."),
        "retained": T("Every plank and gap, both layers of the frames, cleat bevels, braces, centre cleat, batten.", "Jede Bohle und Fuge, beide Lagen der Böcke, Riegelfasen, Streben, Mittelriegel, Leiste."),
        "merged": T("Gusset web and bearing plate into one 15 mm bracket web; the flange at the 15 mm floor.", "Knotenblechsteg und Lagerplatte zu einem 15-mm-Konsolsteg; der Flansch im 15-mm-Mindestmaß."),
        "omitted": T("Bolts, nuts, plungers, eased edges, grain.", "Schrauben, Muttern, Rastbolzen, gebrochene Kanten, Maserung."),
        "thinElements": T("The 10 mm gaps are drawn at the 0.15 mm print minimum (0.1 mm true).", "Die 10-mm-Fugen sind im Druckminimum 0,15 mm gezeichnet (wahr 0,1 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Planks 0.7, legs 0.55, cleats and stretchers 0.5, braces 0.46, steel 0.22.", "Bohlen 0,7, Beine 0,55, Riegel und Zangen 0,5, Streben 0,46, Stahl 0,22."),
        "overlays": T("One region per shape.", "Eine Region je Form."),
    },
    "200": {
        "silhouette": T("Picnic table 7.6 x 3.8 mm: top slab, the A-frame with cleat, stretcher and legs, the brace, one board per bench with its bracket block. Long side: slab, the two A-frames, the braces, the bench board between them. Plan: slab, legs and stretchers, two bench boards.",
                        "Picknicktisch 7,6 x 3,8 mm: Tischscheibe, der A-Bock mit Riegel, Zange und Beinen, die Strebe, je ein Brett pro Bank mit seinem Konsolblock. Längsseite: Scheibe, die zwei A-Böcke, die Streben, das Bankbrett dazwischen. Grundriss: Scheibe, Beine und Zangen, zwei Bankbretter."),
        "retained": T("Leg rake, the two layers of the frames, braces, the bench hanging between the frames.", "Beinneigung, die zwei Lagen der Böcke, Streben, die zwischen den Böcken hängende Bank."),
        "merged": T("Planks into one slab and one board per bench; centre cleat into the slab; bracket into a 30 mm block.", "Bohlen zu einer Scheibe und einem Brett je Bank; Mittelriegel in die Scheibe; Konsole zu einem 30-mm-Block."),
        "omitted": T("Gaps, bevels, batten, bolts.", "Fugen, Fasen, Leiste, Schrauben."),
        "thinElements": T("Slab and boards 40 mm (0.2 mm), drawn true.", "Scheibe und Bretter 40 mm (0,2 mm), wahr gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Slab and boards 0.7, legs 0.55, cleats and stretchers 0.5, steel 0.22.", "Scheibe und Bretter 0,7, Beine 0,55, Riegel und Zangen 0,5, Stahl 0,22."),
        "overlays": T("One region per shape.", "Eine Region je Form."),
    },
    "500": {
        "silhouette": T("Picnic table sign 3.0 x 1.5 mm: the top slab, the A of both legs with the stretcher, the two bench boards with their bracket blocks. Long side: slab, two A-frame plates, the bench board. Plan: slab, the legs reaching out, two bench boards.",
                        "Picknicktischzeichen 3,0 x 1,5 mm: die Tischscheibe, das A aus beiden Beinen mit der Zange, die zwei Bankbretter mit ihren Konsolblöcken. Längsseite: Scheibe, zwei A-Bock-Scheiben, das Bankbrett. Grundriss: Scheibe, die ausgreifenden Beine, zwei Bankbretter."),
        "retained": T("Overall size, table and seat height, the A of the legs, the benches between the frames.", "Gesamtgröße, Tisch- und Sitzhöhe, das A der Beine, die Bänke zwischen den Böcken."),
        "merged": T("Top, cleats and planks into a 75 mm slab; legs and stretcher of each frame into one 90 mm plate; brackets into 75 mm blocks.", "Platte, Riegel und Bohlen zu einer 75-mm-Scheibe; Beine und Zange jedes Bocks zu einer 90-mm-Scheibe; Konsolen zu 75-mm-Blöcken."),
        "omitted": T("Braces (45 mm), gaps, bolts.", "Streben (45 mm), Fugen, Schrauben."),
        "thinElements": T("Every member at least 75 mm (0.15 mm print minimum).", "Jedes Glied mindestens 75 mm (Druckminimum 0,15 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Slab and boards 0.7, frames 0.5, steel 0.22.", "Scheibe und Bretter 0,7, Böcke 0,5, Stahl 0,22."),
        "overlays": T("One region per shape.", "Eine Region je Form."),
    },
}


def apply(asset):
    """🖊 Replaces the geometry, hatches and decisions of every scale's representation."""
    for scale in ("50", "100", "200", "500"):
        rep = asset["representations"][scale]
        rep["geometry"] = {"elevation": shapes(scale, "e"), "front": shapes(scale, "f"), "plan": shapes(scale, "p")}
        rep["decisions"] = DECISIONS[scale]
        if scale == "50":
            rep["hatches"] = HATCHES
