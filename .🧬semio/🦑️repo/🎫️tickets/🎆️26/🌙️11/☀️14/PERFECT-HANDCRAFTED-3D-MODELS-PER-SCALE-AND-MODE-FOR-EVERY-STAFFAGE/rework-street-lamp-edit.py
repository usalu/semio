"""🔦 Hand edit of `objects/street-lamp` (second round, pass 1): a real foundation head under the base plate, coarse scales freed of 1:50 hardware, the stay kept at 1:500, drawings and texts to match.

Usage: python3 rework-street-lamp-edit.py
Writes the source `volumes/objects/street-lamp.json` and the root members of the catalog asset (rig, anchor, dimensions, thermal region, structure assumption, 2D shapes); the catalog `volumes` member is published afterwards with `volumes-insert.py`.
"""
import copy
import json
import os
import types

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/objects/street-lamp.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/street-lamp/🔣️.json"
layout = open(os.path.join(T, "volumes-insert.py")).read()
insert = types.SimpleNamespace()
exec("import json\n" + layout[layout.index("def scalar"):layout.index("def member_end")], insert.__dict__)

HEAD = 0.5
SKIN = 0.003
HARDWARE = {
    "100": ["door-lock", "head-latch-pivot", "head-latch", "head-latch-keeper"],
    "200": ["door-lock", "head-latch-pivot", "head-latch", "head-latch-keeper", "head-hatch-joint"],
    "500": ["door-lock", "head-latch-pivot", "head-latch", "head-latch-keeper", "head-hatch-joint", "pole-hatch"],
}
PLATE = {"100": (0.02, False), "200": (0.03, False)}
HEAD_HEIGHT = {"50": 0.1, "100": 0.1, "200": 0.1, "500": 0.12}
MEMBER_FROM = {"50": 0.015, "100": 0.015, "200": 0.03}
ARRIS = {"50": 0.015, "100": 0.015}

DECISIONS = {
    "100": [
        {
            "en": "Body at 1:100 (feature 15 mm): the concrete foundation head 0.50 x 0.50 m stands 0.10 m out of the ground and carries the 300 x 300 x 20 mm base plate (nuts and bolt ends are below the feature and omitted); the shroud is a solid flare on four stations, the column a solid cone 170 to 80 mm with its dome merged, the flush inspection door a texture seam; the clamp is one sleeve without its ear, the socket is merged into the 60 mm arm, the stay is kept slender at its true 27 mm, brace band and gusset lug are omitted (8 and 12 mm); the luminaire is cover and tray on nine stations with the spigot collar and the gear hatch seam (texture), without fins, lock and catch.",
            "de": "Körper 1:100 (Merkmal 15 mm): der Fundamentkopf aus Beton 0,50 x 0,50 m steht 0,10 m aus dem Boden und trägt die Fußplatte 300 x 300 x 20 mm (Muttern und Schraubenenden liegen unter dem Merkmal und entfallen); die Manschette ist ein voller Kragen auf vier Stationen, der Mast ein voller Kegel 170 auf 80 mm mit eingeschmolzener Kuppel, die bündige Revisionstür eine Texturfuge; die Klemme ist eine Hülse ohne Lasche, die Muffe geht im Ausleger von 60 mm auf, die Strebe bleibt schlank mit ihren wahren 27 mm, Strebenring und Knotenblech entfallen (8 und 12 mm); die Leuchte ist Haube und Wanne auf neun Stationen mit Zapfenkragen und Klappenfuge (Textur), ohne Rippen, Verschluss und Riegel.",
        },
        {
            "en": "Thermal at 1:100: every region references its body solids; the base plate belongs to the galvanised column region and the foundation head to the concrete region; the lens keeps its inner face as section face; the plume outlet is a curved plate 0.20 x 0.22 m on the crown of the cover (no fins at this scale).",
            "de": "Thermik 1:100: jede Region verweist auf ihre Körper; die Fußplatte gehört zur verzinkten Mastregion und der Fundamentkopf zur Betonregion; die Abdeckung behält ihre Innenseite als Schnittfläche; der Fahnenaustritt ist eine gebogene Platte 0,20 x 0,22 m auf dem Scheitel der Haube (keine Rippen in diesem Maßstab).",
        },
        {
            "en": "Structure at 1:100: the load path runs from the tray as head chassis through the spigot clamp into the 60 x 3 mm arm tube and the slender stay, through the clamp sleeve into the 4 mm conical tube (both tubes stay hollow, their walls are real), through the base weld into the plate and the foundation head, which bears on a 15 mm pad in the ground joint; anchor bolts and brace lugs are below the feature and merged; four masses at the centres of the bones.",
            "de": "Tragwerk 1:100: der Lastpfad läuft von der Wanne als Kopfrahmen über die Zapfenklemme in das Auslegerrohr 60 x 3 mm und die schlanke Strebe, über die Klemmhülse in das 4 mm starke Kegelrohr (beide Rohre bleiben hohl, ihre Wände sind echt), über die Fußnaht in die Platte und den Fundamentkopf, der auf einem Lager von 15 mm in der Bodenfuge ruht; Ankerschrauben und Strebenlaschen liegen unter dem Merkmal und gehen auf; vier Massen an den Schwerpunkten der Knochen.",
        },
    ],
    "200": [
        {
            "en": "Body at 1:200 (feature 30 mm): the foundation head 0.50 x 0.50 x 0.10 m, the base plate thickened to the feature (30 mm) so that it still reads between concrete and steel, the column one solid cone 170 to 80 mm with a rounded top (shroud and clamp sleeve are merged into it), the 60 mm arm from the column axis, the stay slender at its true section, the luminaire one superelliptic loft on six stations (cover and tray merged, flat underside kept by the exponent) with its spigot collar, the lens and the inspection door as texture; lock, catch and hatch seam are omitted.",
            "de": "Körper 1:200 (Merkmal 30 mm): der Fundamentkopf 0,50 x 0,50 x 0,10 m, die Fußplatte auf das Merkmal verstärkt (30 mm), damit sie zwischen Beton und Stahl lesbar bleibt, der Mast ein voller Kegel 170 auf 80 mm mit gerundetem Kopf (Manschette und Klemmhülse gehen in ihm auf), der Ausleger 60 mm ab Mastachse, die Strebe schlank mit wahrem Querschnitt, die Leuchte ein superelliptischer Loft auf sechs Stationen (Haube und Wanne vereint, die flache Unterseite durch den Exponenten) mit Zapfenkragen, Abdeckung und Revisionstür als Textur; Verschluss, Riegel und Klappenfuge entfallen.",
        },
        {
            "en": "Thermal at 1:200: every region references its body solid, the plate counts to the column region; only the total area is compared at this scale.",
            "de": "Thermik 1:200: jede Region verweist auf ihren Körper, die Platte zählt zur Mastregion; in diesem Maßstab wird nur die Gesamtfläche verglichen.",
        },
        {
            "en": "Structure at 1:200: foundation head with the plate, conical tube and arm tube hollow (cavities of 72 to 162 mm and 54 mm exceed the feature), the stay slender, the head chassis a flat loft inside the housing; joints as solid blocks at the base, the clamp and the spigot; the support is a 30 mm pad under the head; four masses.",
            "de": "Tragwerk 1:200: Fundamentkopf mit Platte, Kegelrohr und Auslegerrohr hohl (Hohlräume von 72 bis 162 mm und 54 mm über dem Merkmal), die Strebe schlank, der Kopfrahmen ein flacher Loft im Gehäuse; Verbindungen als volle Körper an Fuß, Klemme und Zapfen; das Auflager ist ein Lager von 30 mm unter dem Kopf; vier Massen.",
        },
    ],
    "500": [
        {
            "en": "Body at 1:500 (feature 75 mm): five parts carry the silhouette: the foundation head 0.50 x 0.50 m with the plate merged into its 0.12 m, the column as a plain cone 170 to 80 mm, the 60 mm arm and the diagonal stay as slender members at their true sections (the triangle of arm, stay and column is what identifies the bracket from afar), the luminaire as one loft on five stations with its spigot collar; door, lock, catch and seams are omitted.",
            "de": "Körper 1:500 (Merkmal 75 mm): fünf Teile tragen die Silhouette: der Fundamentkopf 0,50 x 0,50 m mit der in seinen 0,12 m aufgegangenen Platte, der Mast als schlichter Kegel 170 auf 80 mm, der Ausleger 60 mm und die schräge Strebe als schlanke Stäbe mit wahrem Querschnitt (das Dreieck aus Ausleger, Strebe und Mast macht die Konsole von weitem kenntlich), die Leuchte als ein Loft auf fünf Stationen mit Zapfenkragen; Tür, Verschluss, Riegel und Fugen entfallen.",
        },
        {
            "en": "Thermal at 1:500: the body solids carry their regions; the lens is a thin plate under the head (texture, inner face as section face) so that the region keeps its solid; only the total is compared.",
            "de": "Thermik 1:500: die Körper tragen ihre Regionen; die Abdeckung ist eine dünne Platte unter dem Kopf (Textur, Innenseite als Schnittfläche), damit die Region ihren Körper behält; verglichen wird nur die Summe.",
        },
        {
            "en": "Structure at 1:500: foundation head, column, hollow arm, stay and a chassis loft in the head, joined by solid blocks at the base and the clamp and by the spigot collar; the masses sit at the centres of the bones and the head bears on the ground pad.",
            "de": "Tragwerk 1:500: Fundamentkopf, Mast, hohler Ausleger, Strebe und ein Rahmenloft im Kopf, verbunden durch volle Körper an Fuß und Klemme und durch den Zapfenkragen; die Massen sitzen an den Schwerpunkten der Knochen und der Kopf ruht auf dem Bodenlager.",
        },
    ],
}


def layout_root(value, depth, key=None, flat=False):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(name, ensure_ascii=False) + ": " + layout_root(item, depth + 1, name, flat or name in ("representations", "volumes")) for name, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if all(isinstance(item, list) and all(not isinstance(leaf, (dict, list)) for leaf in item) for item in value):
            return json.dumps(value, ensure_ascii=False)
        if flat and all(not isinstance(item, (dict, list)) for item in value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + layout_root(item, depth + 1, None, flat) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def find(rows, identity):
    return next((row for row in rows if row.get("id") == identity), None)


def box(identity, material, tone, size, y, extra=None):
    solid = {"id": identity, "kind": "box", "bone": "foundation", "material": material, "role": "body", "tone": tone, "size": size, "transform": {"translation": [0, y, 0]}}
    solid.update(extra or {})
    return solid


source = json.load(open(SOURCE))
volumes = source["volumes"]
brace = copy.deepcopy(find(volumes["200"]["body"], "arm-brace"))
for scale, model in volumes.items():
    body, thermal, structure = model["body"], model["thermal"], model["structure"]
    height = HEAD_HEIGHT[scale]
    plinth = find(body, "plinth")
    plinth["size"] = [HEAD, height, HEAD]
    plinth["transform"] = {"translation": [0, height / 2, 0]}
    plinth.pop("cornerRadius", None)
    if scale in ARRIS:
        plinth["cornerRadius"] = ARRIS[scale]
    if scale in PLATE and not find(body, "base-plate"):
        thickness, _ = PLATE[scale]
        body.insert(1, box("base-plate", "galv-steel", 0.35, [0.3, thickness, 0.3], 0.1 + thickness / 2))
        thermal.insert(1, {"id": "th-base-plate", "of": "base-plate", "region": "pole-galv"})
    member = find(structure, "member-plinth") or find(structure, "member-foundation")
    if scale in MEMBER_FROM:
        low = MEMBER_FROM[scale]
        fresh = box(member["id"], "concrete", 0.65, [HEAD, 0.1 - low, HEAD], (0.1 + low) / 2, {"member": "foundation", "part": "member"})
        if scale in ARRIS:
            fresh["cornerRadius"] = ARRIS[scale]
        structure[structure.index(member)] = fresh
        if not find(structure, "member-base-plate"):
            structure.insert(structure.index(fresh) + 1, {"id": "member-base-plate", "of": "base-plate", "member": "foundation", "part": "member"})
    support = find(structure, "support-foundation-support")
    support["size"] = [HEAD, support["size"][1], HEAD]
    for identity in HARDWARE.get(scale, []):
        for rows in (body, thermal, structure):
            rows[:] = [row for row in rows if row.get("id") != identity and row.get("of") != identity]
    if scale == "500" and not find(body, "arm-brace"):
        body.insert(body.index(find(body, "arm")) + 1, brace)
        thermal.insert(thermal.index(find(thermal, "th-arm")) + 1, {"id": "th-arm-brace", "of": "arm-brace", "region": "arm-alu"})
        structure.insert(structure.index(find(structure, "member-arm")) + 1, {"id": "member-arm-brace", "of": "arm-brace", "member": "arm", "part": "member"})
    covered = find(thermal, "th-plinth")
    if scale == "500":
        thermal[thermal.index(covered)] = {"id": "th-plinth", "of": "plinth", "region": "plinth-concrete", "sectionFaces": ["-v"]}
    else:
        thermal[thermal.index(covered)] = {"id": "th-plinth", "of": "plinth", "region": "plinth-concrete", "sectionFaces": ["-v", "+v"]}
        if not find(thermal, "th-plinth-top"):
            edge = HEAD / 2 - ARRIS.get(scale, 0)
            skin = {"id": "th-plinth-top", "kind": "extrude", "bone": "foundation", "material": "concrete", "role": "body", "tone": 0.65, "profile": [[edge, edge], [-edge, edge], [-edge, -edge], [edge, -edge]], "holes": [[[0.15, 0.15], [0.15, -0.15], [-0.15, -0.15], [-0.15, 0.15]]], "plane": "zx", "from": 0.1 - SKIN, "to": 0.1, "region": "plinth-concrete", "sectionFaces": ["-v"], "sheet": True}
            thermal.insert(1, skin)
    if scale == "50":
        structure[:] = [row for row in structure if row["id"] not in ("joint-head-latch-pivot", "joint-head-latch", "joint-head-keeper", "joint-door-lock")]
    if scale == "500":
        find(body, "arm")["slender"] = True
        find(structure, "member-arm")["slender"] = True
    if scale in DECISIONS:
        model["decisions"] = DECISIONS[scale]
    else:
        for decision in model["decisions"]:
            decision["en"] = decision["en"].replace("concrete plinth 0.30 x 0.30 x 0.10 m with rounded arrises", "the head of the concrete foundation block, 0.50 x 0.50 m and 0.10 m out of the ground with eased arrises (the block continues below ground and is cut there; the anchor bolts keep 130 mm to its edge)").replace("down into the concrete plinth", "down into the concrete foundation head").replace("under the plinth", "under the head")
            decision["de"] = decision["de"].replace("Betonsockel 0,30 x 0,30 x 0,10 m mit gerundeten Kanten", "der Kopf des Betonfundaments, 0,50 x 0,50 m und 0,10 m über dem Boden mit gebrochenen Kanten (der Block reicht in den Boden und ist dort geschnitten; die Ankerschrauben halten 130 mm Randabstand)").replace("in den Betonsockel", "in den Fundamentkopf").replace("unter dem Sockel", "unter dem Kopf")
open(SOURCE, "w").write(insert.dump(source, 0) + "\n")

text = open(CATALOG).read()
document = json.loads(text)
if layout_root(document, 0) + "\n" != text:
    import difflib
    print("\n".join(line[:160] for line in list(difflib.unified_diff(text.splitlines(), layout_root(document, 0).splitlines(), lineterm="", n=0))[:30]))
    raise SystemExit("catalog is not in the expected layout")
asset = document["asset"]
asset["dimensions"]["widthM"] = 1.475
asset["dimensions"]["depthM"] = HEAD
foundation = find(asset["rig"]["bones"], "foundation")
foundation["lengthM"] = HEAD / 2
foundation["title"] = {"en": "Foundation head and base plate", "de": "Fundamentkopf und Fußplatte"}
anchor = find(asset["anchors"], "foundation-support")
anchor["footprintLengthM"] = HEAD
anchor["footprintWidthM"] = HEAD
region = find(asset["thermal"]["regions"], "plinth-concrete")
region["title"] = {"en": "Concrete foundation head", "de": "Fundamentkopf aus Beton"}
region["areaM2"] = 0.34
region["characteristicLengthM"] = HEAD
assumption = {
    "en": "The concrete foundation block continues below ground and is cut at the ground plane; only its head, 0.50 x 0.50 m and 0.10 m high, is modelled, and the fixed support acts in that plane.",
    "de": "Der Fundamentblock aus Beton reicht in den Boden und ist in der Geländeebene geschnitten; modelliert ist nur sein Kopf, 0,50 x 0,50 m und 0,10 m hoch, und das feste Auflager wirkt in dieser Ebene.",
}
if assumption not in asset["structure"]["assumptions"]:
    asset["structure"]["assumptions"].append(assumption)
plate = {view: find(asset["representations"]["50"]["geometry"][view], "base-plate") for view in ("elevation", "front", "plan")}
for scale, representation in asset["representations"].items():
    geometry = representation["geometry"]
    for view in ("elevation", "front", "plan"):
        shapes = geometry[view]
        plinth = find(shapes, "plinth")
        plinth.update(x=-HEAD / 2, w=HEAD)
        if view == "plan":
            plinth.update(y=-HEAD / 2, h=HEAD)
        else:
            plinth.update(y=0, h=HEAD_HEIGHT[scale])
        if scale in PLATE and not find(shapes, "base-plate"):
            shape = copy.deepcopy(plate[view])
            shape.pop("r", None)
            if view != "plan":
                shape["h"] = PLATE[scale][0]
            shapes.insert(shapes.index(plinth) + 1, shape)
        if scale == "500" and view != "plan" and not find(shapes, "arm-brace"):
            shape = copy.deepcopy(find(asset["representations"]["200"]["geometry"][view], "arm-brace"))
            shape["role"] = "body"
            shapes.insert(shapes.index(find(shapes, "arm") or find(shapes, "pole")) + 1, shape)
        if scale == "500" and find(shapes, "arm-brace"):
            find(shapes, "arm-brace")["role"] = "body"
fresh = open(CATALOG).read()
assert fresh == text, "catalog changed while editing, run again"
open(CATALOG, "w").write(layout_root(document, 0) + "\n")
print("edited")
