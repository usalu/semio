"""🧱 Server rack, author pass 1 of the second round: blanking panels U17 to U39, levelling feet, caster axles, wheel load path.

Hand-chosen numbers only. Frame of every solid: u = up along the bone, v = lateral, w = depth (door at w = -0.5).
The equipment bone starts 0.1 m above the floor; one rack unit is 44.45 mm, a panel is 0.8 mm shorter than its units.
Writes the volumes source; with `--catalog` it also writes the 2D shapes, thermal areas, hinge limits and decisions.
See rework/assets/appliances--server-rack.md.
"""
import copy
import json
import re
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
SOURCE = T / "volumes/appliances/server-rack.json"
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/server-rack/🔣️.json")

U = 0.04445
FIRST_2U = 0.7112 + U
ONE_U = 0.7112 + 22.5 * U
FIELD_FROM, FIELD_TO = 0.7112, 1.7332
FIELD_MID, FIELD_LEN = (FIELD_FROM + FIELD_TO) / 2, FIELD_TO - FIELD_FROM
STACK = {"count": 11, "translation": [2 * U, 0.0, 0.0]}
PAIR = {"count": 2, "translation": [0.0, -0.5, 0.0]}
FASTENER_V = 0.2325
FOOT_W = 0.3
OWN = ("top-ring", "blank-", "th-blank-", "level-foot-", "th-level-foot-", "caster-axle-", "th-caster-axle-", "member-caster-axle-", "member-caster-wheel-")


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
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


def member_range(text, indent, key):
    found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    if not found:
        return None
    return found.start() + 1, member_end(text, found.end())


def r(value):
    return round(value, 6)


def clean(items, keep=()):
    return [item for item in items if item["id"] in keep or not item["id"].startswith(OWN)]


def by_id(items, id):
    return next(item for item in items if item["id"] == id)


def after(items, id, new):
    at = next((k for k, item in enumerate(items) if item["id"] == id), len(items) - 1) + 1
    items[at:at] = new


def like(template, id, **more):
    solid = copy.deepcopy(template)
    solid.pop("repeat", None)
    solid.update({"id": id, **more})
    return solid


def plate(template, id, length, u, thickness, w, repeat=None):
    solid = like(template, id, size=[r(length), 0.4826, thickness], transform={"translation": [r(u), 0.0, r(w)]})
    if repeat:
        solid["repeat"] = repeat
    return solid


def turned(template, id, bone, material, profile, axis, origin, repeat=None, **more):
    solid = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": template["tone"], "profile": profile, "axis": axis, "origin": [r(x) for x in origin], **more}
    if repeat:
        solid["repeat"] = repeat
    return solid


def reference(template, id, of, **more):
    solid = {key: value for key, value in template.items() if key not in ("sectionFaces", "layer")}
    solid.update({"id": id, "of": of, **more})
    return solid


def volumes(source):
    v50, v100, v200, v500 = (source["volumes"][key] for key in ("50", "100", "200", "500"))
    for scale in (v50, v100, v200, v500):
        for name in ("body", "thermal", "structure"):
            scale[name] = clean(scale[name], keep=() if scale is v50 else ("member-caster-wheel-f", "member-caster-wheel-r"))

    body = v50["body"]
    front = by_id(body, "patch-40-front")
    lock = by_id(body, "door-lock")
    wheel = by_id(body, "caster-wheel-f")
    stud = [[0.0, 0.021], [0.003, 0.025], [0.009, 0.025], [0.012, 0.012], [0.012, 0.006], [0.05, 0.006], [0.05, 0.0095], [0.058, 0.0095]]
    head = [[0.0, 0.005], [0.002, 0.005]]
    after(body, "switch-42-chassis", [
        plate(front, "blank-2u-front", 2 * U - 0.0008, FIRST_2U, 0.008, -0.404, STACK),
        plate(front, "blank-1u-front", U - 0.0008, ONE_U, 0.008, -0.404),
        turned(lock, "blank-2u-fastener-a", "equipment", "blank-steel", head, [0, 0, -1], [FIRST_2U, FASTENER_V, -0.408], STACK, texture=True),
        turned(lock, "blank-2u-fastener-b", "equipment", "blank-steel", head, [0, 0, -1], [FIRST_2U, -FASTENER_V, -0.408], STACK, texture=True),
        turned(lock, "blank-1u-fastener-a", "equipment", "blank-steel", head, [0, 0, -1], [ONE_U, FASTENER_V, -0.408], texture=True),
        turned(lock, "blank-1u-fastener-b", "equipment", "blank-steel", head, [0, 0, -1], [ONE_U, -FASTENER_V, -0.408], texture=True),
    ])
    body += [
        turned(wheel, "caster-axle-f", "feet", "rack-steel", [[0.0, 0.006], [0.036, 0.006]], [0, -1, 0], [0.025, 0.268, -0.45], PAIR),
        turned(wheel, "caster-axle-r", "feet", "rack-steel", [[0.0, 0.006], [0.036, 0.006]], [0, -1, 0], [0.025, 0.268, 0.45], PAIR),
        turned(wheel, "level-foot-f", "feet", "rack-steel", stud, [1, 0, 0], [0.012, 0.25, -FOOT_W], PAIR),
        turned(wheel, "level-foot-r", "feet", "rack-steel", stud, [1, 0, 0], [0.012, 0.25, FOOT_W], PAIR),
    ]
    sheet = by_id(v50["thermal"], "th-patch")
    after(v50["thermal"], "th-server", [
        plate(sheet, "th-blank-2u", 2 * U - 0.0008, FIRST_2U, 0.0015, -0.40875, STACK),
        plate(sheet, "th-blank-1u", U - 0.0008, ONE_U, 0.0015, -0.40875),
    ])
    skin = by_id(v50["thermal"], "th-caster-race-f")
    v50["thermal"] += [reference(skin, f"th-{id}", id) for id in ("caster-axle-f", "caster-axle-r", "level-foot-f", "level-foot-r")]
    bar = by_id(v50["structure"], "member-caster-cheek-b-r")
    after(v50["structure"], "member-caster-cheek-b-r", [reference(bar, f"member-{id}", id) for id in ("caster-axle-f", "caster-wheel-f", "caster-axle-r", "caster-wheel-r")])

    front = by_id(v100["body"], "server-front")
    blank = {"material": "blank-steel", "tone": by_id(v50["body"], "patch-40-front")["tone"]}
    foot = [[0.0, 0.025], [0.015, 0.025], [0.015, 0.0075], [0.058, 0.0075]]
    after(v100["body"], "top-group-chassis", [
        {**plate(front, "blank-2u-front", 2 * U - 0.0008, FIRST_2U, 0.015, -0.4075, STACK), **blank},
        {**plate(front, "blank-1u-front", U - 0.0008, ONE_U, 0.015, -0.4075), **blank},
    ])
    wheel = by_id(v100["body"], "caster-wheel-f")
    v100["body"] += [
        turned(wheel, "level-foot-f", "feet", "rack-steel", foot, [1, 0, 0], [0.012, 0.25, -FOOT_W], PAIR),
        turned(wheel, "level-foot-r", "feet", "rack-steel", foot, [1, 0, 0], [0.012, 0.25, FOOT_W], PAIR),
    ]
    sheet = by_id(v100["thermal"], "th-blanks")
    after(v100["thermal"], "th-servers", [plate(sheet, "th-blank-field", FIELD_LEN, FIELD_MID, 0.0015, -0.41575)])
    skin = by_id(v100["thermal"], "th-caster-head-f")
    v100["thermal"] += [reference(skin, f"th-{id}", id) for id in ("level-foot-f", "level-foot-r")]

    block = by_id(v200["body"], "top-group")
    after(v200["body"], "top-group", [{**plate(block, "blank-field", FIELD_LEN, FIELD_MID, 0.03, -0.415), **blank}])
    caster = by_id(v200["body"], "caster-f")
    plain = [[0.0, 0.025], [0.058, 0.025]]
    v200["body"] += [
        turned(caster, "level-foot-f", "feet", "rack-steel", plain, [1, 0, 0], [0.012, 0.25, -FOOT_W], PAIR),
        turned(caster, "level-foot-r", "feet", "rack-steel", plain, [1, 0, 0], [0.012, 0.25, FOOT_W], PAIR),
    ]
    sheet = by_id(v200["thermal"], "th-blanks")
    after(v200["thermal"], "th-servers", [plate(sheet, "th-blank-field", FIELD_LEN, FIELD_MID, 0.0015, -0.43075)])
    skin = by_id(v200["thermal"], "th-caster-f")
    v200["thermal"] += [reference(skin, f"th-{id}", id) for id in ("level-foot-f", "level-foot-r")]

    block = by_id(v500["body"], "top-group")
    after(v500["body"], "top-group", [{**plate(block, "blank-field", FIELD_LEN, FIELD_MID, 0.075, -0.3705), **blank}])
    roof = by_id(v500["body"], "roof")
    ring = like(roof, "top-ring", holes=[[[-0.225, -0.4235], [-0.225, 0.4235], [0.225, 0.4235], [0.225, -0.4235]]])
    ring["from"], ring["to"] = 1.925, 1.9988
    after(v500["body"], "roof", [ring])
    sheet = by_id(v500["thermal"], "th-blanks")
    after(v500["thermal"], "th-servers", [plate(sheet, "th-blank-field", FIELD_LEN, FIELD_MID, 0.0015, -0.40875)])


CANDIDATE = T / "🗑️generated/r3-server-rack/🔣️.json"
BLANK = {"bone": "equipment", "role": "body", "z": 3.2, "filled": True, "tone": 0.5, "kind": "rect", "y": -0.2413, "h": 0.4826, "region": "blanks", "material": "blank-steel"}
HEAD = {"bone": "equipment", "role": "detail", "z": 3.3, "filled": False, "tone": 0.2, "kind": "ellipse", "rx": 0.005, "ry": 0.005}
FOOT = {"bone": "feet", "role": "body", "z": 3.05, "filled": True, "tone": 0.3, "kind": "poly", "closed": True, "region": "feet", "material": "rack-steel"}
AREAS = {"blanks": 0.545, "feet": 0.095}
SAID = {
    "50": {
        "en": " The 23 empty units U17 to U39 are closed by eleven 2U blanking panels and one 1U panel with their quarter-turn fasteners, so cold air cannot bypass the servers; four levelling feet with 50 mm pads on M12 studs stand screwed up 12 mm beside the casters.",
        "de": " Die 23 leeren Einheiten HE17 bis HE39 sind mit elf 2-HE-Blindplatten und einer 1-HE-Platte samt Schnellverschlüssen geschlossen, damit keine Kaltluft an den Servern vorbeiströmt; vier Nivellierfüße mit 50-mm-Tellern an M12-Bolzen stehen 12 mm hochgedreht neben den Rollen.",
    },
    "100": {
        "en": " The empty units U17 to U39 are closed by eleven 2U blanking panels and one 1U panel; the levelling feet are a 50 mm pad on a 15 mm stud.",
        "de": " Die leeren Einheiten HE17 bis HE39 sind mit elf 2-HE-Blindplatten und einer 1-HE-Platte geschlossen; die Nivellierfüße sind ein 50-mm-Teller an einem 15-mm-Bolzen.",
    },
    "200": {
        "en": " The blanking panels of the empty units U17 to U39 are one closed field between server block and top group; each levelling foot is one 50 mm cylinder.",
        "de": " Die Blindplatten der leeren Einheiten HE17 bis HE39 sind ein geschlossenes Feld zwischen Serverblock und Obergruppe; jeder Nivellierfuß ist ein 50-mm-Zylinder.",
    },
    "500": {
        "en": " The blanking panels of the empty units U17 to U39 are one closed field between server block and top group; the levelling feet are below the feature and left out.",
        "de": " Die Blindplatten der leeren Einheiten HE17 bis HE39 sind ein geschlossenes Feld zwischen Serverblock und Obergruppe; die Nivellierfüße liegen unter dem kleinsten Merkmal und entfallen.",
    },
}
DOOR = {
    "en": (("the door with 20 mm holes at 28 mm pitch", "the door with 52 by 57 mm apertures between 10 mm webs in 30 rows of 8"), ("is generalised to 20 mm holes with 8 mm webs", "is generalised to 52 by 57 mm apertures with 10 mm webs, as in the 3D door sheet")),
    "de": (("die Tür mit 20-mm-Löchern im Raster 28 mm", "die Tür mit Öffnungen von 52 mal 57 mm zwischen 10-mm-Stegen in 30 Reihen zu 8"), ("ist auf 20-mm-Löcher mit 8-mm-Stegen verallgemeinert", "ist auf Öffnungen von 52 mal 57 mm mit 10-mm-Stegen verallgemeinert, wie im 3D-Türblech")),
}
GONE = {
    "en": ("The levelling feet (screwed up behind the base frame), cabling", "Cabling"),
    "de": ("Die Nivellierfüße (hinter dem Sockelrahmen hochgedreht), die Verkabelung", "Die Verkabelung"),
}


def foot(id, w, pad, top, stud):
    return {**FOOT, "id": id, "points": [[r(w - 0.025), 0.012], [r(w + 0.025), 0.012], [r(w + 0.025), top], [r(w + stud), top], [r(w + stud), 0.07], [r(w - stud), 0.07], [r(w - stud), top], [r(w - 0.025), top]]} if pad else {**FOOT, "id": id, "points": [[r(w - 0.025), 0.012], [r(w + 0.025), 0.012], [r(w + 0.025), 0.07], [r(w - 0.025), 0.07]]}


def drawings(asset):
    for scale, representation in asset["representations"].items():
        geometry = representation["geometry"]
        for view in geometry:
            geometry[view] = [shape for shape in geometry[view] if not shape["id"].startswith(("blank-", "level-foot-"))]
        if scale in ("50", "100"):
            shapes = []
            for k in range(11):
                u = FIRST_2U + k * 2 * U
                shapes.append({**BLANK, "id": f"blank-{17 + 2 * k}", "x": r(u - U + 0.0004), "w": 0.0881})
                if scale == "50":
                    shapes += [{**HEAD, "id": f"blank-{17 + 2 * k}-fastener-{side}", "cx": r(u), "cy": v} for side, v in (("a", FASTENER_V), ("b", -FASTENER_V))]
            shapes.append({**BLANK, "id": "blank-39", "x": r(ONE_U - U / 2 + 0.0004), "w": 0.04365})
            if scale == "50":
                shapes += [{**HEAD, "id": f"blank-39-fastener-{side}", "cx": r(ONE_U), "cy": v} for side, v in (("a", FASTENER_V), ("b", -FASTENER_V))]
        else:
            shapes = [{**BLANK, "id": "blank-field", "x": FIELD_FROM, "w": r(FIELD_LEN), "penMm": 0.2}]
        elevation = geometry["elevation"]
        at = next(k for k, shape in enumerate(elevation) if shape["id"] in ("patch-40", "top-group"))
        elevation[at:at] = shapes
        if scale != "500":
            pad, top, stud = {"50": (True, 0.024, 0.006), "100": (True, 0.027, 0.0075), "200": (False, 0.07, 0.025)}[scale]
            geometry["front"] += [foot("level-foot-front", -FOOT_W, pad, top, stud), foot("level-foot-rear", FOOT_W, pad, top, stud)]
        decisions = representation["decisions"]
        for language in ("en", "de"):
            old, new = GONE[language]
            decisions["omitted"][language] = decisions["omitted"][language].replace(old, new)
            for key, (was, now) in zip(("retained", "merged"), DOOR[language]):
                decisions[key][language] = decisions[key][language].replace(was, now)
            if SAID[scale][language].strip() not in decisions["retained"][language]:
                decisions["retained"][language] += SAID[scale][language]


def facts(asset):
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "blanks":
            region["title"] = {"en": "Patch panel, cable brush and blanking panels", "de": "Patchfeld, Kabelbürste und Blindplatten"}
        if region["id"] == "feet":
            region["title"] = {"en": "Casters and levelling feet", "de": "Rollen und Nivellierfüße"}
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "door-hinge":
            bone["joint"]["twistMinDeg"] = -130
        if bone["id"] == "rear-door-a":
            bone["joint"]["twistMaxDeg"] = 130
        if bone["id"] == "rear-door-b":
            bone["joint"]["twistMinDeg"] = -130


def said(source):
    for scale, lists in source["volumes"].items():
        for language in ("en", "de"):
            if SAID[scale][language].strip() not in lists["decisions"][0][language]:
                lists["decisions"][0][language] += SAID[scale][language]


def splice(text, asset, keys):
    for key in keys:
        start, end = member_range(text, "    ", key)
        text = text[:start] + "    " + json.dumps(key) + ": " + dump(asset[key], 2) + text[end:]
    return text


def catalog(source, publish):
    text = CATALOG.read_text()
    document = json.loads(text)
    asset = document["asset"]
    drawings(asset)
    facts(asset)
    result = splice(text, asset, ("rig", "representations", "thermal"))
    assert json.loads(result) == document, "splice did not round-trip"
    if publish:
        assert CATALOG.read_text() == text, "catalog changed while splicing; retry"
        CATALOG.write_text(result)
        print("wrote catalog")
    else:
        asset["volumes"] = source["volumes"]
        CANDIDATE.parent.mkdir(parents=True, exist_ok=True)
        CANDIDATE.write_text(splice(result, asset, ("volumes",)))
        print("wrote", CANDIDATE)


def main():
    source = json.loads(SOURCE.read_text())
    volumes(source)
    said(source)
    SOURCE.write_text(json.dumps(source, ensure_ascii=False, indent=2) + "\n")
    print("wrote", SOURCE.name, {scale: {name: len(items) for name, items in lists.items() if isinstance(items, list)} for scale, lists in source["volumes"].items()})


    if "--candidate" in sys.argv or "--catalog" in sys.argv:
        catalog(source, "--catalog" in sys.argv)


if __name__ == "__main__":
    main()
