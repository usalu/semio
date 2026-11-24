"""🏗️ Writes the parametric repair ops of `tower-crane` of batch R4-X (ops/tower-crane.json; applied after the ops of SAM, R2-M and R3-M; make/tower-crane.sh adds the outlines). Authored against the catalog file as it stood before this batch (`ASSET_FILE`); the ops go by id, a second run of the ops leaves the file unchanged.

`bars`: a scale models no bar below its smallest feature (7.5, 15, 30, 75 mm), and a bar of at least that size is a body of the scale, not a `slender` line. Measured cause (`misfit.ts`): the guard compares a slender solid along a centre line taken from its mesh; for the 39 m mast chords (160 mm square) that gave a band of 223 mm, for the 54 m jib chord and rails a band twice their section (the IoU failures of 1:500 and of the 1:200 front), and a 14 mm rope on a raster of 18.75 mm is there or not by the luck of a cell centre (front 1:500: 415 mm). So every straight bar loses `slender` and is compared as its own mesh; bars thinner than the smallest feature are typified to it (1:500: ropes, 50 mm pendants, 60 and 70 mm lacing to 75 mm; 1:200: ropes to 30 mm; 1:100: ropes to 15 mm); the capsules of batch R3-M (pendants, hoist rope runs) are tubes of revolution again (the capsule was the work-around for the centre line, its round tips are not the bar). The drawn `member` shapes and rope strips get the same sections.
`rails`: the jib rails are 140 mm high and 120 mm wide in the body and were drawn 120 mm high and 140 mm wide.
`parts`: parts that are no bars are drawn as the body has them, each as a prism, a disc or a body of revolution of its bone so that it is right in every view and in the slewed pose (a flat outline is only right in the plan and on the fixed base): the winch house stands on its two skids (the front view filled the space between them), its roof grille is the plate of the body (it was drawn as a block inside the house), the side glass of the cab at 1:200, the hook block as two cheek plates with sheave and pin (at 1:50 with 0.13 mm outlines: the 20 mm of play between sheave and cheek is 0.4 mm of paper, and between two 0.25 and 0.35 mm outlines only 0.1 mm of it would stay white, below the smallest gap that prints), the lugs of the skip, the trolley axles, the two plates of the heel sheave hanger, the rim of the skip, the round bow of the hook and the stepped slew motors at 1:50, the sheave block of the trolley at 1:500.
`ropes`: the two falls of the hoist rope are drawn where their solids hang (0.204 m off the bone, 4 mm further than drawn) and as long as they are; the front view draws one of the two (they lie behind each other, and two rings of one swept shape that cover each other cancel)."""
import json, math, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
VIEWS = ("elevation", "front", "plan")
asset = json.load(open(os.environ.get("ASSET_FILE") or CATALOG))["asset"]
ops = []
r5 = lambda v: round(v, 5)


def bar(solid, feature):
    """🪵️ A slender straight bar as a body of its scale: no `slender`, at least one smallest feature thick, a capsule as a tube of revolution between its end points."""
    out = {k: v for k, v in solid.items() if k != "slender"}
    if solid["kind"] == "capsule":
        p0, p1, r = solid["p0"], solid["p1"], max(solid["r0"], feature / 2)
        length = math.dist(p0, p1)
        out = {k: v for k, v in out.items() if k not in ("p0", "p1", "r0", "r1")}
        out.update(kind="revolve", profile=[[0, r5(r)], [r5(length), r5(r)]], axis=[r5((b - a) / length) for a, b in zip(p0, p1)], origin=[r5(v) for v in p0])
    elif solid["kind"] == "revolve":
        out["profile"] = [[x, r5(max(r, feature / 2))] for x, r in solid["profile"]]
    return out


for scale, feature in FEATURE.items():
    body = asset["volumes"][scale]["body"]
    for solid in body:
        if solid.get("slender") is True:
            ops.append({"op": "solid", "scale": scale, "id": solid["id"], "solid": bar(solid, feature)})
    for view in VIEWS:
        for shape in asset["representations"][scale]["geometry"][view]:
            new = json.loads(json.dumps(shape))
            if shape["kind"] == "member" and shape["section"]["kind"] == "circle" and shape["section"]["d"] < feature:
                new["section"]["d"] = feature
            if shape["kind"] == "member" and shape["id"] in ("jib-rail-l", "jib-rail-r"):
                new["section"] = {"kind": "rect", "w": 0.14, "h": 0.12}
            rope = re.fullmatch(r"(rope-\d)-line", shape["id"])
            if rope and shape["kind"] == "path":
                falls = [s for s in body if s["bone"] == rope[1] and s["kind"] == "revolve" and s["axis"] == [0, -1, 0]]
                r = max(max(s["profile"][0][1] for s in falls), feature / 2)
                rect = lambda s: f"M {r5(s['origin'][0] - r)},{r5(s['origin'][1] - s['profile'][1][0])} L {r5(s['origin'][0] + r)},{r5(s['origin'][1] - s['profile'][1][0])} L {r5(s['origin'][0] + r)},{r5(s['origin'][1])} L {r5(s['origin'][0] - r)},{r5(s['origin'][1])} Z"
                new["d"] = " ".join(rect(s) for s in (falls[:1] if view == "front" else falls))
                new["solid"] = {**shape["solid"], "z0": r5(-r), "z1": r5(r)}
            if new != shape:
                ops.append({"op": "shape", "scale": scale, "view": view, "id": shape["id"], "shape": new})



def solid_of(scale, ident):
    """🔎️ One body solid of a scale, None when the scale has none of that id."""
    return next((s for s in asset["volumes"][scale]["body"] if s["id"] == ident), None)


def shape_of(scale, view, ident):
    """🔎️ One drawn shape of a view, None when the view has none of that id."""
    return next((s for s in asset["representations"][scale]["geometry"][view] if s["id"] == ident), None)


GEOMETRY = ("kind", "x", "y", "w", "h", "points", "closed", "d", "cx", "cy", "rx", "ry", "from", "to", "section", "solid", "penMm")


def put(scale, view, ident, geometry, like, after=None, **style):
    """✍️ Replaces the shape of an id by a new geometry in the style it had, or adds it after `after` in the style of `like`."""
    old = shape_of(scale, view, ident)
    base = old or shape_of(scale, view, like) or next(s for v in VIEWS for s in [shape_of(scale, v, like)] if s)
    new = {**geometry, "id": ident, **{k: v for k, v in base.items() if k not in GEOMETRY and k != "id" and not (k == "regions" and "region" in style)}, **style}
    ops.append({"op": "shape", "scale": scale, "view": view, "id": ident, "shape": new} if old else {"op": "shape-add", "scale": scale, "view": view, "after": after or like, "shape": new})


def prism(solid):
    """🧱️ A box or an extrusion of the (x, y) plane as its side profile swept laterally: the same part in every view and pose."""
    if solid["kind"] == "box":
        (sx, sy, sz), (tx, ty, tz) = solid["size"], solid.get("transform", {}).get("translation", [0, 0, 0])
        return {"kind": "rect", "x": r5(tx - sx / 2), "y": r5(ty - sy / 2), "w": sx, "h": sy, "solid": {"kind": "extruded", "sweep": "w", "z0": r5(tz - sz / 2), "z1": r5(tz + sz / 2)}}
    assert solid["kind"] == "extrude" and solid["plane"] == "xy"
    return {"kind": "poly", "points": solid["profile"], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": solid["from"], "z1": solid["to"]}}


def disc(solid):
    """🥏️ A cylinder about the lateral axis as its circle swept laterally."""
    (l0, r), (l1, _) = solid["profile"]
    z = sorted(solid["origin"][2] + solid["axis"][2] * l for l in (l0, l1))
    return {"kind": "ellipse", "cx": solid["origin"][0], "cy": solid["origin"][1], "rx": r, "ry": r, "solid": {"kind": "extruded", "sweep": "w", "z0": r5(z[0]), "z1": r5(z[1])}}


def beam(solid, scale, filled):
    """🪵️ A box along x as a `member` of its section, filled or as the stroke of its height."""
    (sx, sy, sz), (tx, ty, tz) = solid["size"], solid["transform"]["translation"]
    out = {"kind": "member", "from": {"x": r5(tx - sx / 2), "y": ty, "z": tz}, "to": {"x": r5(tx + sx / 2), "y": ty, "z": tz}, "section": {"kind": "rect", "w": sy, "h": sz}}
    return out if filled else {**out, "penMm": max(0.13, round(sy * 1000 / int(scale), 3))}


for scale in FEATURE:
    paper = lambda metres: metres * 1000 / int(scale)
    for view in VIEWS:
        house = shape_of(scale, view, "winch-house")
        if house and house["kind"] == "rect" and house["y"] != 0:
            ops.append({"op": "shape", "scale": scale, "view": view, "id": "winch-house", "shape": {**house, "y": 0, "h": 1.8}})
        if shape_of(scale, view, "winch-skids"):
            ops.append({"op": "shape-remove", "scale": scale, "view": view, "id": "winch-skids"})
    for view in VIEWS:
        for side in ("r", "l"):
            skid = solid_of(scale, f"winch-skid-{side}")
            filled = paper(skid["size"][1]) >= 0.25
            put(scale, view, f"winch-skid-{side}", beam(skid, scale, filled), "winch-house", role="detail", z=3.2, tone=0.4, filled=filled, region="winch-house", material=skid["material"])
    for view in ("elevation", "front"):
        grille = solid_of(scale, "winch-motor-grille")
        if grille:
            put(scale, view, "winch-motor-grille", beam(grille, scale, False), "winch-vent-stack", role="detail", filled=False, material=grille["material"])
    sheave = solid_of(scale, "trolley-sheave-block")
    if scale == "500":
        for view in VIEWS:
            put(scale, view, "trolley-sheave", prism(sheave), "trolley-sheave")
    if scale == "200":
        glass = shape_of(scale, "elevation", "cab-glass")
        for view in ("front", "plan"):
            ops.append({"op": "shape-add", "scale": scale, "view": view, "after": "cab-glass-front", "shape": glass})
    if scale in ("100", "50"):
        thin = {"penMm": 0.13} if scale == "50" else {}
        put(scale, "front", "block-body", {**prism(solid_of(scale, "block-plate-r")), **thin}, "block-body")
        put(scale, "front", "block-plate-l", {**prism(solid_of(scale, "block-plate-l")), **thin}, "block-body")
        put(scale, "front", "block-sheave", {**disc(solid_of(scale, "block-sheave")), **thin}, "block-body", "block-plate-l", role="detail")
        put(scale, "front", "block-sheave-pin", {**disc(solid_of(scale, "block-sheave-pin")), **thin}, "block-body", "block-sheave", role="detail")
        for side in ("r", "l"):
            put(scale, "front", f"skip-lug-{side}", prism(solid_of(scale, f"skip-lug-{side}")), "skip-gate", "bail-leg-l")
            for n in (1, 2):
                put(scale, "front", f"trolley-wheel-{side}-{n}-axle", disc(solid_of(scale, f"trolley-wheel-{side}-{n}-axle")), f"trolley-wheels-{side}")
    if scale == "50":
        put(scale, "front", "hoist-heel-hanger", prism(solid_of(scale, "hoist-heel-hanger-r")), "hoist-heel-hanger")
        put(scale, "front", "hoist-heel-hanger-l", prism(solid_of(scale, "hoist-heel-hanger-l")), "hoist-heel-hanger")
        rim = solid_of(scale, "skip-rim")
        (h0, r_out) = rim["profile"][0][0], max(p[1] for p in rim["profile"])
        h1 = max(p[0] for p in rim["profile"])
        y0 = rim["origin"][1]
        for view in ("elevation", "front"):
            body = shape_of(scale, view, "skip-body")
            put(scale, view, "skip-rim", {"kind": "poly", "points": [[r_out, r5(y0 + h1)], [r_out, r5(y0 + h0)], [-r_out, r5(y0 + h0)], [-r_out, r5(y0 + h1)]], "closed": True, "solid": body["solid"]}, "skip-body", role="detail", z=4.45)
        hook = shape_of(scale, "front", "hook")
        low = min(p[1] for p in shape_of(scale, "elevation", "hook")["points"])
        flat = sorted(p[0] for p in shape_of(scale, "elevation", "hook")["points"] if abs(p[1] - low) < 1e-6)
        radius = solid_of(scale, "hook")["sections"][0]["rx"]
        ops.append({"op": "shape", "scale": scale, "view": "front", "id": "hook", "shape": {**hook, "points": [[x, max(y, r5(low + radius))] for x, y in hook["points"]]}})
        put(scale, "front", "hook-bow", {"kind": "member", "from": {"x": flat[0], "y": r5(low + radius), "z": 0}, "to": {"x": flat[-1], "y": r5(low + radius), "z": 0}, "section": {"kind": "circle", "d": 2 * radius}}, "hook")
        for side in ("l", "r"):
            motor = solid_of(scale, f"slew-motor-{side}")
            right = [[r5(motor["origin"][0] + r), r5(motor["origin"][1] + l)] for l, r in motor["profile"] if r > 0]
            for view in ("elevation", "front"):
                old = shape_of(scale, view, f"slew-motor-{side}")
                put(scale, view, f"slew-motor-{side}", {"kind": "poly", "points": right + [[r5(2 * motor["origin"][0] - x), y] for x, y in reversed(right)], "closed": True, "solid": old["solid"]}, f"slew-motor-{side}")

TYPIFIED = {
    "500": ("the hoist rope with its eight pieces, the 50 mm pendants and the 60 and 70 mm lacing bars are 75 mm round bars", "das Hubseil mit seinen acht Stücken, die 50-mm-Abspannstangen und die 60- und 70-mm-Füllstäbe sind 75-mm-Rundstäbe"),
    "200": ("the rope falls and the hoist rope run are 30 mm round bars (real 14 mm)", "die Seilstränge und der Hubseillauf sind 30-mm-Rundstäbe (real 14 mm)"),
    "100": ("the rope falls and the hoist rope run are 15 mm round bars (real 14 mm)", "die Seilstränge und der Hubseillauf sind 15-mm-Rundstäbe (real 14 mm)"),
    "50": ("nothing is thinner than 7.5 mm here, the 14 mm rope keeps its section", "hier ist nichts dünner als 7,5 mm, das 14-mm-Seil behält seinen Querschnitt"),
}
REPLACED = {
    "500": [
        (("50 mm pendant tie bars;", "pendant tie bars (50 mm, typified to 75 mm);"), ("50-mm-Zugstangen;", "Zugstangen (50 mm, auf 75 mm typisiert);")),
        (("the hoist rope run at its true 14 mm as a slender line.", "the hoist rope run as a 75 mm bar."), ("der Hubseillauf mit seinen wahren 14 mm als schlanke Linie.", "der Hubseillauf als 75-mm-Stab.")),
        (("The lattice bars keep their true sections as slender members at this scale on purpose:", "The lattice bars of 75 mm and more keep their true sections at this scale on purpose:"), ("Die Gitterstäbe behalten in diesem Maßstab bewusst ihre wahren Querschnitte als schlanke Stäbe:", "Die Gitterstäbe ab 75 mm behalten in diesem Maßstab bewusst ihre wahren Querschnitte:")),
    ],
}
for scale, (en, de) in TYPIFIED.items():
    text = dict(asset["volumes"][scale]["decisions"][0])
    for (old_en, new_en), (old_de, new_de) in REPLACED.get(scale, []):
        assert old_en in text["en"] and old_de in text["de"], (scale, old_en)
        text = {"en": text["en"].replace(old_en, new_en), "de": text["de"].replace(old_de, new_de)}
    text["en"] += f" Fourth round: a scale models no bar below its smallest feature, and a bar of that size or more is a body, not a slender line: {en}; no bar is declared slender, each is compared as its own mesh. The jib rails are 140 mm high and 120 mm wide."
    text["de"] += f" Vierte Runde: Ein Maßstab modelliert keinen Stab unter seinem kleinsten Merkmal, und ein Stab ab dieser Größe ist ein Körper, keine schlanke Linie: {de}; kein Stab ist mehr als schlank deklariert, jeder wird als sein eigenes Netz verglichen. Die Auslegerschienen sind 140 mm hoch und 120 mm breit."
    ops.append({"op": "set", "path": ["volumes", scale, "decisions", 0], "value": text})
    drawn = dict(asset["representations"][scale]["decisions"]["thinElements"])
    drawn["en"] += f" Fourth round: {en}, drawn as they are modelled; every bar of the body is drawn in every view (both diagonal directions of the tower head, the jib tip axle), the winch house stands on its two skids, its roof grille is the plate of the body and its vent stack sits on its roof in the plan."
    drawn["de"] += f" Vierte Runde: {de}, gezeichnet wie modelliert; jeder Stab des Körpers ist in jeder Ansicht gezeichnet (beide Diagonalrichtungen der Turmspitze, die Achse an der Auslegerspitze), das Windenhaus steht auf seinen zwei Kufen, sein Dachgitter ist die Platte des Körpers und sein Abluftrohr sitzt im Grundriss auf seinem Dach."
    ops.append({"op": "set", "path": ["representations", scale, "decisions", "thinElements"], "value": drawn})

with open(os.path.join(HERE, "..", "ops", "tower-crane.json"), "w") as f:
    f.write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
print(f"tower-crane: {len(ops)} ops")
