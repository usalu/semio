"""🔧 Round 3 steel column root fixes: the 2D drawings redrawn as the true views of the handcrafted body (ruling E1).

- The splice at 1.5 m is removed at 1:50 and 1:100: a 3.0 m column of 184 kg is one rolled piece.
- Gussets: one real plate (50 mm wide, top at 0.11 m, nose, snipe at 1:50) instead of two different sizes at 1:50 and 1:100.
- Anchors are M16 (EN ISO 4032 nut 24 mm across flats, ISO 7089 washer 30 mm): rod 16 mm, washer outline in plan, nut across corners in the front view; drawn as studs at 1:100.
- Fillet lines at the true tangent of the 18 mm root radius (67 mm from the axis) over the whole shaft.
- Head plate holes for the beam bolts at 1:50 (plan).
- 1:500: plates at the 75 mm feature of the model instead of 20 mm (0.04 mm on paper).

Usage: python3 r3-steel-column-roots.py [--write] [--area REGION=VALUE ...]
Without --write the candidate goes to 🗑️generated/r3-steel-column/🔣️.json; with --write the catalog file is re-read and only the changed members are replaced in its text.

See: r3-components-steel-column.md, r3-steel-column-volumes.py, round3-brief.md (ruling E1)
"""

import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/steel-column/🔣️.json"
OUT = os.path.join(HERE, "🗑️generated", "r3-steel-column", "🔣️.json")
ANCHORS = {"00": (-0.125, -0.1), "01": (-0.125, 0.1), "10": (0.125, -0.1), "11": (0.125, 0.1)}
CAP_HOLES = {"00": (-0.05, -0.06), "01": (-0.05, 0.06), "10": (0.05, -0.06), "11": (0.05, 0.06)}


def text(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


def poly(shape, points):
    """✏️ The shape with new points, members kept in order."""
    return {**shape, "points": points}


def without(shapes, ids):
    """🧹 The shapes without the named ones."""
    return [s for s in shapes if s["id"] not in ids]


def by_id(shapes):
    """🗂️ Shapes by id."""
    return {s["id"]: s for s in shapes}


def fix_50(rep):
    """🔍 1:50."""
    g = rep["geometry"]
    gusset = [[0.02, 0.115], [0.02, 0.15], [0.045, 0.15], [0.11, 0.11], [0.11, 0.1], [0.035, 0.1]]
    new = {
        "fillet-a": [[0.02, 0.067], [2.98, 0.067]],
        "fillet-b": [[0.02, -0.067], [2.98, -0.067]],
        "stiffener-a": gusset,
        "stiffener-b": [[u, -v] for u, v in gusset],
    }
    g["elevation"] = [poly(s, new[s["id"]]) if s["id"] in new else s for s in without(g["elevation"], ["splice-line"])]
    front = []
    for s in without(g["front"], ["splice-line"]):
        if s["id"] == "anchor-nut-a":
            s = {**s, "x": -0.1139, "w": 0.0277}
        if s["id"] == "anchor-nut-b":
            s = {**s, "x": 0.0861, "w": 0.0277}
        front.append(s)
    g["front"] = front
    plan = []
    for s in g["plan"]:
        if s["id"].startswith("anchor-nut-"):
            s = {**s, "rx": 0.015, "ry": 0.015}
        if s["id"].startswith("anchor-bolt-"):
            s = {**s, "rx": 0.008, "ry": 0.008}
        plan.append(s)
    for key, (x, y) in CAP_HOLES.items():
        plan.append({"id": f"cap-hole-{key}", "bone": "column", "role": "detail", "z": 4.1, "filled": False, "tone": 0.35, "region": "cap-plate", "kind": "ellipse", "cx": x, "cy": y, "rx": 0.009, "ry": 0.009})
    g["plan"] = plan
    d = rep["decisions"]
    d["silhouette"] = text(
        "The H-section seen onto its web: flange edges, web with fillet lines, base plate with two stiffener gussets, anchor nuts and bolts and the head plate; the column reads as one rolled steel member. Front and plan: the front is the flange face with the near gusset edge-on and the two near anchor nuts and bolts; the plan is the top view with the head plate (20 mm) and its four bolt holes over the base plate, two gussets and four anchor washers with bolt ends; the far nuts are hidden behind the near ones in the front.",
        "Das H-Profil mit Blick auf den Steg: Flanschkanten, Steg mit Hohlkehlenlinien, Fußplatte mit zwei Rippen, Ankermuttern und -bolzen und die Kopfplatte; die Stütze liest sich als ein gewalztes Stahlprofil. Vorder- und Draufsicht: Die Vorderansicht ist die Flanschfläche mit der vorderen Rippe in Kantenansicht und den zwei vorderen Ankermuttern und -bolzen; die Draufsicht ist die Sicht von oben mit der Kopfplatte (20 mm) und ihren vier Schraubenlöchern über der Fußplatte, zwei Rippen und vier Ankerscheiben mit Bolzenenden; die hinteren Muttern sind in der Vorderansicht hinter den vorderen verdeckt.",
    )
    d["retained"] = text(
        "Flange thickness 15 mm (0.3 mm), fillet lines at the tangent of the 18 mm root radius, 20 mm plates at true thickness (0.4 mm), gussets 0.05 x 0.09 m with a 25 mm nose, a sloped edge up to a 10 mm top and a 15 mm snipe at the flange weld, M16 anchors (washer and nut 18 mm high, nut 24 mm across flats, 27.7 mm across corners in the front view, washer 30 mm in plan), bolt ends and the four 18 mm holes of the head plate.",
        "Flanschdicke 15 mm (0,3 mm), Hohlkehlenlinien an der Tangente des 18-mm-Ausrundungsradius, 20-mm-Platten in wahrer Dicke (0,4 mm), Rippen 0,05 x 0,09 m mit 25-mm-Nase, schräger Kante bis zu einer 10-mm-Oberkante und 15-mm-Eckausschnitt an der Flanschnaht, M16-Anker (Scheibe und Mutter 18 mm hoch, Mutter mit Schlüsselweite 24 mm, 27,7 mm über Eck in der Vorderansicht, Scheibe 30 mm in der Draufsicht), Bolzenenden und die vier 18-mm-Löcher der Kopfplatte.",
    )
    d["omitted"] = text(
        "Weld seams (8.5 mm legs, 0.17 mm), the anchor bolt thread, the 3 mm step of the washer under the nut and the grout joint below the plate.",
        "Schweißnähte (8,5 mm Schenkel, 0,17 mm), das Ankerbolzengewinde, der 3-mm-Absatz der Scheibe unter der Mutter und die Mörtelfuge unter der Platte.",
    )
    d["thinElements"] = text(
        "The 9 mm web thickness is not drawn (view onto the web); bolt shanks are 0.1 mm pen lines in the elevations (16 mm real, 0.32 mm) and 16 mm circles in plan.",
        "Die Stegdicke 9 mm wird nicht gezeichnet (Blick auf den Steg); Bolzenschäfte sind in den Ansichten 0,1-mm-Linien (16 mm real, 0,32 mm) und in der Draufsicht 16-mm-Kreise.",
    )


def fix_100(rep):
    """🔎 1:100."""
    g = rep["geometry"]
    gusset = [[0.035, 0.1], [0.035, 0.15], [0.055, 0.15], [0.11, 0.115], [0.11, 0.1]]
    new = {"stiffener-left": gusset, "stiffener-right": [[u, -v] for u, v in gusset]}
    elevation = [poly(s, new[s["id"]]) if s["id"] in new else s for s in without(g["elevation"], ["splice-joint"])]
    stud = {"bone": "column", "role": "detail", "z": 2.5, "filled": True, "tone": 0.1, "region": "base-plate", "material": "steel"}
    for key, side in (("a", 1), ("b", -1)):
        elevation.append({"id": f"anchor-{key}", **stud, "kind": "poly", "points": [[0.035, side * 0.113], [0.035, side * 0.137], [0.075, side * 0.137], [0.075, side * 0.113]], "closed": True})
    g["elevation"] = elevation
    front = []
    for s in without(g["front"], ["splice-joint"]):
        if s["id"] == "stiffener-edge":
            s = {**s, "h": 0.075}
        front.append(s)
    for key, x in (("a", -0.112), ("b", 0.088)):
        front.append({"id": f"anchor-{key}", **stud, "z": 3.5, "tone": 0.35, "kind": "rect", "x": x, "y": 0.035, "w": 0.024, "h": 0.04})
    g["front"] = front
    plan = []
    for s in g["plan"]:
        if s["id"] == "cap-plate-block":
            for key, (x, y) in ANCHORS.items():
                plan.append({"id": f"anchor-{key}", **stud, "tone": 0.35, "kind": "ellipse", "cx": x, "cy": y, "rx": 0.012, "ry": 0.012})
        plan.append(s)
    g["plan"] = plan
    d = rep["decisions"]
    d["silhouette"] = text(
        "A 2 mm wide column bar, a base plate with two stiffener gussets and four anchor studs, and a head plate; the steel column reads as an H-section seen onto its web. Front and plan: the front shows the flange face, the edge of the near gusset (20 mm) and the two near studs; the plan shows the head plate over the base plate with the two gussets sticking out of the head plate on the web axis and the four studs at the corners of the base plate.",
        "Ein 2 mm breiter Stützenbalken, Fußplatte mit zwei Rippen und vier Ankerbolzen und Kopfplatte; die Stahlstütze liest sich als H-Profil mit Blick auf den Steg. Vorder- und Draufsicht: Die Vorderansicht zeigt die Flanschfläche, die Kante der vorderen Rippe (20 mm) und die zwei vorderen Bolzen; die Draufsicht zeigt die Kopfplatte über der Fußplatte, die beiden Rippen ragen auf der Stegachse unter der Kopfplatte hervor, die vier Bolzen sitzen in den Ecken der Fußplatte.",
    )
    d["retained"] = text(
        "Plates, two gussets of the true 50 mm width up to the true height of 0.11 m (0.5 x 0.75 mm above the plate) and four anchor studs of 24 x 40 mm (0.24 x 0.4 mm).",
        "Platten, zwei Rippen der wahren Breite von 50 mm bis zur wahren Höhe von 0,11 m (0,5 x 0,75 mm über der Platte) und vier Ankerbolzen von 24 x 40 mm (0,24 x 0,4 mm).",
    )
    d["omitted"] = text(
        "Flange edge lines, fillet lines, the snipe of the gussets, the holes of the head plate (0.18 mm) and the step between washer, nut and bolt end.",
        "Flanschkantenlinien, Hohlkehlenlinien, der Eckausschnitt der Rippen, die Löcher der Kopfplatte (0,18 mm) und der Absatz zwischen Scheibe, Mutter und Bolzenende.",
    )
    d["thinElements"] = text(
        "The 20 mm plates are drawn 35 mm thick (0.35 mm, above the 0.2 mm feature minimum); the bar is shortened accordingly. The gussets are drawn with a 20 mm nose and a 15 mm flat top, so no filled tip is thinner than the printable feature; washer, nut and bolt end of each anchor are one 24 mm stud.",
        "Die 20-mm-Platten sind 35 mm dick gezeichnet (0,35 mm, über dem Mindestmerkmal von 0,2 mm); der Balken ist entsprechend gekürzt. Die Rippen sind mit 20-mm-Nase und 15 mm breiter Oberkante gezeichnet, sodass keine gefüllte Spitze dünner ist als das druckbare Merkmal; Scheibe, Mutter und Bolzenende jedes Ankers sind ein 24-mm-Bolzen.",
    )


def fix_500(rep):
    """🛰️ 1:500."""
    g = rep["geometry"]
    t = 0.075
    g["elevation"] = [poly(s, [[0, 0.15], [0, -0.15], [t, -0.15], [t, -0.1], [3 - t, -0.1], [3 - t, -0.12], [3, -0.12], [3, 0.12], [3 - t, 0.12], [3 - t, 0.1], [t, 0.1], [t, 0.15]]) if s["id"] == "column" else s for s in g["elevation"]]
    g["front"] = [poly(s, [[-0.13, 0], [0.13, 0], [0.13, t], [0.1, t], [0.1, 3 - t], [0.11, 3 - t], [0.11, 3], [-0.11, 3], [-0.11, 3 - t], [-0.1, 3 - t], [-0.1, t], [-0.13, t]]) if s["id"] == "column" else s for s in g["front"]]
    d = rep["decisions"]
    d["silhouette"] = text(
        "One composed vertical bar 0.20 m wide and 3.00 m high (0.4 x 6 mm) with the base and head plates as 75 mm step-outs of the outline. Front and plan: the front is the flange face with the 0.26 m base plate and the 0.22 m head plate as one outline; the plan is the 0.30 x 0.26 m base plate under the 0.24 x 0.22 m head plate, the roof of the column seen from above.",
        "Ein zusammengesetzter senkrechter Balken, 0,20 m breit und 3,00 m hoch (0,4 x 6 mm), mit Fuß- und Kopfplatte als 75-mm-Absätzen der Kontur. Vorder- und Draufsicht: Die Vorderansicht ist die Flanschfläche mit der 0,26 m breiten Fußplatte und der 0,22 m breiten Kopfplatte in einem Umriss; die Draufsicht zeigt die Fußplatte 0,30 x 0,26 m unter der Kopfplatte 0,24 x 0,22 m, das Dach der Stütze von oben.",
    )
    d["thinElements"] = text(
        "The 20 mm plates are drawn 75 mm thick (0.15 mm, the smallest printable feature) as step-outs of the outline; the stiffeners (0.1 x 0.18 mm) are below printable size and left out.",
        "Die 20-mm-Platten sind 75 mm dick gezeichnet (0,15 mm, das kleinste druckbare Merkmal) als Absätze der Kontur; die Rippen (0,1 x 0,18 mm) liegen unter der druckbaren Größe und entfallen.",
    )


def fix(doc, areas):
    """🔧 Apply every root fix to the asset document in place; the names of the changed asset members."""
    a = doc["asset"]
    r = a["representations"]
    fix_50(r["50"])
    fix_100(r["100"])
    fix_500(r["500"])
    changed = ["representations"]
    for region in a["thermal"]["regions"]:
        if region["id"] in areas:
            region["areaM2"] = areas[region["id"]]
            changed = ["representations", "thermal"]
    return changed


def member_end(source, start):
    """🧵 The end of the JSON value that starts at `start`."""
    depth, index, quoted = 0, start, False
    while index < len(source):
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced asset file")


def splice(source, key, value):
    """🪡 The file text with one asset member replaced, in the file's own two-space style."""
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    end = member_end(source, found.end())
    return source[: found.end()] + json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n    ") + source[end:]


def main():
    """🚀 Build the candidate, or rewrite the changed members of the catalog file with --write."""
    areas = {}
    for k, arg in enumerate(sys.argv):
        if arg == "--area":
            region, value = sys.argv[k + 1].split("=")
            areas[region] = float(value)
    with open(CATALOG, encoding="utf-8") as f:
        source = f.read()
    doc = json.loads(source)
    before = json.loads(source)
    changed = fix(doc, areas)
    result = source
    for key in changed:
        result = splice(result, key, doc["asset"][key])
    check = json.loads(result)
    if check != doc or [k for k in check["asset"]] != [k for k in before["asset"]]:
        raise SystemExit("splice did not round-trip")
    if {k: v for k, v in check["asset"].items() if k not in changed} != {k: v for k, v in before["asset"].items() if k not in changed}:
        raise SystemExit("splice changed other members")
    write = "--write" in sys.argv
    if write and open(CATALOG, encoding="utf-8").read() != source:
        raise SystemExit("asset changed while editing; retry")
    path = CATALOG if write else OUT
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(result)
    print(path, changed)


if __name__ == "__main__":
    main()
