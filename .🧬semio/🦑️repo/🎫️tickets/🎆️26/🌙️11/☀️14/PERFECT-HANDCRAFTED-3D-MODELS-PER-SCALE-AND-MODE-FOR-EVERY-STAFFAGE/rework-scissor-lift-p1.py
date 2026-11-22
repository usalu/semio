"""🪜 Scissor lift, second round pass 1b: steering knuckles with kingpin caps at the front wheels, gate hinges and latch at 1:100, dished wheels and visibly separate inner and outer arms at 1:200, decisions rewritten. Hand-chosen numbers, idempotent. `--catalog` writes the steering-knuckle shapes of the front view into the 2D drawings of the catalog asset."""
import json
import sys

P = "volumes/machinery/scissor-lift.json"
C = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/scissor-lift/🔣️.json"
GEO = ("kind", "size", "transform", "repeat", "cornerRadius", "profile", "axis", "origin", "plane", "from", "to", "holes", "vertices", "faces", "triangles", "tags", "sheet", "slender")
BOX = [0.17, 0.37, 0.43, 0.67, 0.28, 0.33]


def find(lst, i):
    return next((x for x in lst if x["id"] == i), None)


def put(lst, after, new):
    lst[:] = [x for x in lst if x["id"] != new["id"]]
    k = next((n for n, x in enumerate(lst) if x["id"] == after), len(lst) - 1)
    lst.insert(k + 1, new)


def solid(V, scale, sibling, i, geo, region, member=None, bone=None, look=None):
    s = V[scale]
    base = {k: v for k, v in find(s["body"], look or sibling).items() if k not in GEO}
    base.update({"id": i, **geo})
    if bone:
        base["bone"] = bone
    put(s["body"], sibling, base)
    put(s["thermal"], "th-" + sibling, {"id": "th-" + i, "of": i, "region": region})
    if member:
        put(s["structure"], "member-gate-stile", {"id": "member-" + i, "of": i, "member": member, "part": "member"})


def box(size, at, repeat=None, radius=None):
    g = {"kind": "box", "size": size, "transform": {"translation": at}}
    if radius:
        g["cornerRadius"] = radius
    if repeat:
        g["repeat"] = {"count": 2, "translation": repeat}
    return g


def source():
    doc = json.load(open(P))
    V = doc["volumes"]
    for scale in V:
        for name in ("body", "thermal", "structure"):
            V[scale][name][:] = [x for x in V[scale][name] if "manual-box" not in x["id"]]
    for scale in ("50", "100", "200"):
        solid(V, scale, "axle-f", "steer-yoke", box([0.07, 0.17, 0.05], [0.66, 0.1525, 0.265], [0, 0, -0.53]), "chassis")
    for scale, r, h in (("50", 0.02, 0.012), ("100", 0.022, 0.015)):
        solid(V, scale, "steer-yoke", "kingpin-cap", {"kind": "revolve", "profile": [[0, r], [h, r]], "axis": [0, 1, 0], "origin": [0.66, 0.2375, 0.265], "repeat": {"count": 2, "translation": [0, 0, -0.53]}}, "chassis")
    solid(V, "100", "gate-stile", "gate-hinge", box([0.025, 0.05, 0.02], [-0.07, 0.6, 0.3275], [0, -0.3, 0]), "rails", "platform")
    solid(V, "100", "gate-hinge", "gate-latch", box([0.03, 0.025, 0.02], [-0.07, 0.6, -0.3275]), "rails", "platform")
    for w in ("fl", "fr", "rl", "rr"):
        find(V["200"]["body"], "tyre-" + w)["profile"] = [[-0.055, 0], [-0.055, 0.1525], [0.055, 0.1525], [0.055, 0.105], [0.025, 0.105], [0.025, 0]]
    for side, z0, z1, step in (("a", 0.27, 0.32, -0.59), ("b", 0.215, 0.265, -0.48)):
        for n in "1234":
            arm = find(V["200"]["body"], "arm-" + side + n)
            arm.update({"kind": "extrude", "plane": "yz", "from": -0.045, "to": 1.505, "profile": [[-0.045, z0], [0.045, z0], [0.045, z1], [-0.045, z1]], "holes": [[[-0.039, z0 + 0.006], [0.039, z0 + 0.006], [0.039, z1 - 0.006], [-0.039, z1 - 0.006]]], "repeat": {"count": 2, "translation": [0, 0, step]}})
    D = {
        "50": ({"en": "Second round, pass 1: every part is joined to what carries it (pivot bearers under the base lugs, full-width roller axle, rail posts down to the deck channels, gate hinges and latch, controller bracket). New: the two steering knuckles between tub and front wheels with their kingpin caps. The front wheels roll but do not steer in the rig yet.", "de": "Zweite Runde, Durchgang 1: Jedes Teil ist mit seinem Träger verbunden (Lagerböcke unter den Fußlaschen, durchgehende Rollenachse, Geländerpfosten bis auf die Bühnenträger, Torbänder und Riegel, Halter der Steuerung). Neu: die beiden Achsschenkel zwischen Wanne und Vorderrädern mit ihren Bolzenkappen. Die Vorderräder rollen, lenken im Rig aber noch nicht."},),
        "100": ({"en": "Second round, pass 1: steering knuckles with 44 mm kingpin caps, two 20 mm gate hinges and the latch join the gate to the rear posts; ladder brackets, track and pivot bearers close the load path to the chassis.", "de": "Zweite Runde, Durchgang 1: Achsschenkel mit 44-mm-Bolzenkappen, zwei 20-mm-Torbänder und der Riegel verbinden das Tor mit den hinteren Pfosten; Leiterhalter, Laufschiene und Lagerböcke schließen den Lastpfad zum Fahrgestell."},),
        "200": ({"en": "Second round, pass 1: each wheel is a tyre with a 30 mm dished wheel face instead of a plain disc; the arms are the true 90 x 50 mm rectangular tubes with their 6 mm wall and open ends, so the thermal view counts the inner surface as the declared total area does; steering knuckles as blocks; track, base lugs and bearers carry the stack.", "de": "Zweite Runde, Durchgang 1: Jedes Rad ist ein Reifen mit 30 mm eingezogener Radschüssel statt einer glatten Scheibe; die Arme sind die wahren Rechteckrohre 90 x 50 mm mit 6 mm Wand und offenen Enden, sodass die Thermik die Innenfläche mitzählt, wie es die deklarierte Gesamtfläche tut; Achsschenkel als Klötze; Laufschiene, Fußlaschen und Lagerböcke tragen das Paket."},),
    }
    for scale, extra in D.items():
        d = V[scale]["decisions"]
        d[:] = [x for x in d if not x["en"].startswith("Second round, pass 1")] + list(extra)
    V["500"]["decisions"] = [
        {"en": "Body for a 75 mm feature: the chassis is the tub with one flush pod per side, stub axles and four plain wheel discs; the scissors are eight bars 90 x 75 mm on four stages with the three cross tubes of every inner arm at 75 mm, no pins or bosses, so the stowed stack reads as one layered block and the raised lift as crossing bars; cylinder and rod are two rounds; the platform is one slab with posts, top and mid rails, gate bar and toe boards at 75 mm in place of the real 40 mm tube and 8 mm board; the slide-out deck is a 75 mm plate with its own rail frame inside the main rails; the controller is a box. Ladder, doors, handles and steering knuckles are left out.", "de": "Körper für 75 mm kleinstes Merkmal: Das Fahrgestell ist die Wanne mit je einem bündigen Seitenkasten, Achsstummeln und vier glatten Radscheiben; die Schere besteht aus acht Stäben 90 x 75 mm in vier Stufen mit den drei Querrohren jedes Innenarms in 75 mm, ohne Bolzen und Augen, sodass das eingefahrene Paket als geschichteter Block und die gehobene Bühne als kreuzende Stäbe lesbar ist; Zylinder und Stange sind zwei Rundstäbe; die Bühne ist eine Platte mit Pfosten, Hand- und Knieleisten, Torholm und Fußleisten in 75 mm anstelle des realen 40-mm-Rohrs und 8-mm-Blechs; die Ausschubbühne ist eine 75-mm-Platte mit eigenem Geländerrahmen innerhalb des Hauptgeländers; die Steuerung ist ein Kasten. Leiter, Türen, Griffe und Achsschenkel entfallen."},
        {"en": "Thermal: the seven regions on these masses; battery, pump and controller covers and the two outlets (motor vent at the front end, battery plume on the right pod) are thermal-only patches because their body parts are below the feature. Structure: two frame rails and the stack rail with one cross member, pivot bearers, axle bars on four tyre contact supports, the eight arms as members with one joint disc per outer arm, deck beam with end member and the extension runner; battery and platform masses.", "de": "Thermik: die sieben Regionen auf diesen Massen; Batterie-, Pumpen- und Steuerungsdeckel und die beiden Auslässe (Motorlüftung am vorderen Ende, Batteriefahne am rechten Kasten) sind rein thermische Flächen, weil ihre Körperteile unter dem Merkmal liegen. Tragwerk: zwei Rahmenlängsträger und die Paketschiene mit einem Querträger, Lagerböcke, Achsstäbe auf vier Reifenaufstandsflächen, die acht Arme als Stäbe mit je einer Gelenkscheibe pro Außenarm, Bühnenträger mit Endträger und die Ausschubschiene; Massen für Batterie und Bühne."},
    ]
    json.dump(doc, open(P, "w"), ensure_ascii=False, indent=1)
    print("written", {k: [len(V[k][l]) for l in ("body", "thermal", "structure")] for k in V})


def shape_text(shape):
    pad, inner = " " * 10, " " * 12
    rows = []
    for key, value in shape.items():
        if isinstance(value, dict):
            body = ",\n".join(inner + "  " + json.dumps(k) + ": " + json.dumps(v) for k, v in value.items())
            rows.append(inner + json.dumps(key) + ": {\n" + body + "\n" + inner + "}")
        else:
            rows.append(inner + json.dumps(key) + ": " + json.dumps(value))
    return pad + "{\n" + ",\n".join(rows) + "\n" + pad + "}"


def object_end(text, start):
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index + 1
    raise SystemExit("unbalanced")


def catalog():
    text = open(C).read()
    if '"id": "steer-yoke-l"' in text:
        raise SystemExit("steer-yoke is already drawn")
    doc = json.loads(text)
    x0, y0, x1, y1, z0, z1 = BOX
    for scale in ("50", "100", "200"):
        for view in ():
            lst = doc["asset"]["representations"][scale]["geometry"][view]
            after = {"elevation": "control-box", "front": None, "plan": "deck"}[view]
            k = next((n for n, s in enumerate(lst) if s["id"] == after), -1)
            z = (lst[k]["z"] + lst[k + 1]["z"]) / 2 if k >= 0 else lst[0]["z"] - 0.01
            shape = {"id": "manual-box", "bone": "platform", "role": "detail", "z": round(z, 4), "filled": True, "tone": 0.15, "region": "rails", "material": "chassis-paint", "kind": "poly", "points": [[x0, y0], [x1, y0], [x1, y1], [x0, y1]], "closed": True}
            if view != "elevation":
                shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
            at = text.index('"' + view + '": [', text.index('"' + scale + '": {', text.index('"representations"')))
            if after:
                end = object_end(text, text.rindex("{", 0, text.index('"id": "' + after + '"', at)))
                text = text[:end] + ",\n" + shape_text(shape) + text[end:]
            else:
                end = text.index("[", at) + 1
                text = text[:end] + "\n" + shape_text(shape) + "," + text[end:]
        lst = doc["asset"]["representations"][scale]["geometry"]["front"]
        tyre = next(s for s in lst if s["id"].startswith("tyre-"))
        at = text.index('"front": [', text.index('"' + scale + '": {', text.index('"representations"')))
        start = text.rindex("{", 0, text.index('"id": "' + tyre["id"] + '"', at)) - 10
        for i, a, b in (("steer-yoke-l", 0.24, 0.29), ("steer-yoke-r", -0.29, -0.24)):
            shape = {"id": i, "bone": "chassis", "role": "body", "z": round(tyre["z"] - (0.004 if a > 0 else 0.002), 4), "filled": True, "tone": 0.35, "region": "chassis", "material": "chassis-paint", "kind": "poly", "points": [[0.625, 0.0675], [0.695, 0.0675], [0.695, 0.2375], [0.625, 0.2375]], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": a, "z1": b}}
            text = text[:start] + shape_text(shape) + ",\n" + text[start:]
    json.loads(text)
    target = sys.argv[2] if len(sys.argv) > 2 else C
    open(target, "w").write(text)
    print("2D written", target)


if __name__ == "__main__":
    catalog() if "--catalog" in sys.argv else source()
