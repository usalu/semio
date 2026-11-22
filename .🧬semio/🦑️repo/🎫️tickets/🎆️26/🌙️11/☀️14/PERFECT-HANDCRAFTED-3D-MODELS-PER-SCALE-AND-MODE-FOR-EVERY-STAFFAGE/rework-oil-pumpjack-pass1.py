"""🛢️ Hand edit of machinery/oil-pumpjack, second round pass 1: post ladder, brake (drum, band, lever, pull rod), closed structure load path with the four slab bearings. Idempotent: every solid it writes is removed first."""
import json
import pathlib

SOURCE = pathlib.Path(__file__).parent / "volumes" / "machinery" / "oil-pumpjack.json"
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
LEG_V0, LEG_SLOPE, LEG_U0 = 1.52922, -0.39887, -0.07326
LADDER_HALF_WIDTH = 0.22
CAP_UNDERSIDE = {"50": 3.56, "100": 3.55, "200": 3.55, "500": 3.525}
STRINGER_THICKNESS = {"50": 0.01, "100": 0.015, "200": 0.03, "500": 0.075}
STRINGER_DEPTH = {"50": 0.06, "100": 0.06, "200": 0.06, "500": 0.075}
RUNG_PITCH = 0.28
LOAD_PATH = [
    ("skid-deck", "skid"), ("motor-feet", "motor"), ("motor-foot-1", "motor"), ("motor-foot-2", "motor"),
    ("bearing-cap-l", "gearbox"), ("bearing-cap-r", "gearbox"), ("crank-shaft", "crank"),
    ("saddle-l", "post"), ("saddle-r", "post"),
    ("tail-pin-l", "beam"), ("tail-pin-r", "beam"), ("tail-plate", "beam"), ("tail-pin", "beam"),
    ("pitman-eye-l", "pitman"), ("pitman-eye-r", "pitman"), ("equalizer", "pitman"),
    ("wellhead-base", "base"), ("wellhead-valve", "base"), ("wellhead-top", "base"), ("wellhead", "base"), ("stuffing-box", "base"),
    ("rod-clamp", "rod"), ("horsehead-face", "horsehead"),
    ("ladder-stringer-l", "post"), ("ladder-stringer-r", "post"),
] + [(f"ladder-rung-{index}", "post") for index in range(1, 14)]
BEARINGS = [("contact-a-l", -1.4, 0.6), ("contact-a-r", -1.4, -0.6), ("contact-b-l", 5.4, 0.6), ("contact-b-r", 5.4, -0.6)]
OWN_BODY = {"ladder-stringer-l", "ladder-stringer-r", "brake-shaft", "brake-drum", "brake-band", "brake-stand", "brake-lever", "brake-rod"} | {f"ladder-rung-{i}" for i in range(1, 20)}


DECISIONS = {
    "50": ("Access ladder and brake at 1:50: two 60 x 10 mm flat stringers 440 mm apart with thirteen 20 mm rungs at 280 mm stand on the slab in front of the well-side legs and end under the cap plate; a 380 x 110 mm brake drum sits outboard of the crank on the extended input shaft, with its band block, a 1 m hand lever on a stand at the motor end of the skid and a 16 mm pull rod. The structure names every part that passes load on (deck, motor feet, bearing caps, saddle blocks, tail hangers, pitman eyes, wellhead stack, rod clamp, horsehead face, ladder) and ends in the four slab bearings.",
           "Aufstiegsleiter und Bremse bei 1:50: zwei Flachstahlholme 60 x 10 mm im Abstand von 440 mm mit dreizehn 20-mm-Sprossen im Raster von 280 mm stehen vor den bohrlochseitigen Beinen auf der Platte und enden unter der Kopfplatte; eine Bremstrommel 380 x 110 mm sitzt außerhalb der Kurbel auf der verlängerten Antriebswelle, mit Bandklotz, einem 1 m langen Handhebel auf einem Bock am Motorende des Rahmens und einer 16-mm-Zugstange. Das Tragwerk nennt jedes Teil, das Last weitergibt (Deckblech, Motorfüße, Lagerdeckel, Sattelböcke, Hängelaschen, Schubstangenaugen, Bohrlochkopf, Stangenklemme, Pferdekopfstirn, Leiter), und endet in den vier Plattenauflagern."),
    "100": ("Access ladder and brake at 1:100 (smallest feature 15 mm): stringers 60 x 15 mm, the thirteen 20 mm rungs, brake drum, band block, lever and 16 mm pull rod are kept at their true sections; the structure ends in the four slab bearings.",
            "Aufstiegsleiter und Bremse bei 1:100 (kleinstes Detail 15 mm): Holme 60 x 15 mm, die dreizehn 20-mm-Sprossen, Bremstrommel, Bandklotz, Hebel und 16-mm-Zugstange behalten ihre wahren Querschnitte; das Tragwerk endet in den vier Plattenauflagern."),
    "200": ("Access ladder and brake at 1:200 (smallest feature 30 mm): stringers 60 x 30 mm with the rungs as true 20 mm rods, brake drum on its shaft and the hand lever; band block and pull rod are left out because nothing of them prints; the structure ends in the four slab bearings.",
            "Aufstiegsleiter und Bremse bei 1:200 (kleinstes Detail 30 mm): Holme 60 x 30 mm mit den Sprossen als wahre 20-mm-Stäbe, Bremstrommel auf ihrer Welle und der Handhebel; Bandklotz und Zugstange entfallen, weil nichts davon druckbar ist; das Tragwerk endet in den vier Plattenauflagern."),
    "500": ("Access ladder and brake at 1:500 (smallest feature 75 mm): the ladder is two 75 mm stringers without rungs, the brake is the drum on its shaft; lever, band block and pull rod are left out; the structure ends in the four slab bearings.",
            "Aufstiegsleiter und Bremse bei 1:500 (kleinstes Detail 75 mm): die Leiter besteht aus zwei 75-mm-Holmen ohne Sprossen, die Bremse aus der Trommel auf ihrer Welle; Hebel, Bandklotz und Zugstange entfallen; das Tragwerk endet in den vier Plattenauflagern."),
}


def leg_v(u):
    return LEG_V0 + LEG_SLOPE * (u - LEG_U0)


def steel(identity, bone, tone, **geometry):
    return {"id": identity, "bone": bone, "material": "painted-steel", "role": "body", "tone": tone, **geometry}


def ladder(scale):
    top, depth, thickness = CAP_UNDERSIDE[scale], STRINGER_DEPTH[scale], STRINGER_THICKNESS[scale]
    near, far = 0.11, 0.11 + depth
    profile = [[-0.25, leg_v(-0.25) + near], [top, leg_v(top) + near], [top, leg_v(top) + far], [-0.25, leg_v(-0.25) + far]]
    profile = [[round(u, 5), round(v, 5)] for u, v in profile]
    solids = [
        steel("ladder-stringer-l", "post", 0.5, kind="extrude", profile=profile, plane="xy", **{"from": LADDER_HALF_WIDTH - thickness, "to": LADDER_HALF_WIDTH}),
        steel("ladder-stringer-r", "post", 0.5, kind="extrude", profile=profile, plane="xy", **{"from": -LADDER_HALF_WIDTH, "to": -LADDER_HALF_WIDTH + thickness}),
    ]
    if scale == "500":
        return solids
    for index in range(13):
        u = 0.05 + RUNG_PITCH * index
        v = round(leg_v(u) + 0.14, 5)
        solids.append(steel(f"ladder-rung-{index + 1}", "post", 0.45, kind="capsule", p0=[round(u, 5), v, -LADDER_HALF_WIDTH + thickness / 2], p1=[round(u, 5), v, LADDER_HALF_WIDTH - thickness / 2], r0=0.01, r1=0.01, slender=True))
    return solids


def brake(scale):
    solids = [
        {**steel("brake-shaft", "gearbox", 0.3, kind="revolve", profile=[[0.0, 0.05], [0.39, 0.05]], axis=[0.0, 0.0, 1.0], origin=[1.25, -0.55, 0.42]), "material": "cast-iron"},
        {**steel("brake-drum", "gearbox", 0.35, kind="revolve", profile=[[0.0, 0.19], [0.11, 0.19]], axis=[0.0, 0.0, 1.0], origin=[1.25, -0.55, 0.80]), "material": "cast-iron"},
    ]
    if scale == "500":
        return solids
    solids.append(steel("brake-stand", "skid", 0.4, kind="box", size=[0.10, 0.30, 0.06], transform={"translation": [6.95, 0.39, 0.97]}))
    solids.append(steel("brake-lever", "skid", 0.3, kind="capsule", p0=[6.95, 0.50, 0.97], p1=[7.10, 1.45, 0.97], r0=0.016, r1=0.016, slender=True))
    if scale == "200":
        return solids
    solids.append(steel("brake-band", "gearbox", 0.28, kind="box", size=[0.14, 0.08, 0.10], transform={"translation": [1.25, -0.32, 0.855]}))
    solids.append(steel("brake-rod", "skid", 0.35, kind="capsule", p0=[6.99, 0.78, 0.97], p1=[5.60, 1.13, 0.88], r0=0.008, r1=0.008, slender=True))
    return solids


def edit(scale, lists):
    body = [solid for solid in lists["body"] if solid["id"] not in OWN_BODY]
    added = ladder(scale) + brake(scale)
    lists["body"] = body + added
    thermal = [solid for solid in lists["thermal"] if solid.get("of") not in OWN_BODY]
    for solid in added:
        thermal.append({"id": "th-" + solid["id"], "of": solid["id"], "region": "gearbox" if solid["id"] in ("brake-shaft", "brake-drum") else "steel"})
    lists["thermal"] = thermal
    present = {solid["id"] for solid in lists["body"]}
    path_ids = {"member-" + identity for identity, _ in LOAD_PATH}
    structure = [solid for solid in lists["structure"] if solid["id"] not in path_ids and solid.get("part") != "support"]
    referenced = {solid.get("of") for solid in structure}
    for identity, bone in LOAD_PATH:
        if identity in present and identity not in referenced:
            structure.append({"id": "member-" + identity, "of": identity, "member": bone, "part": "member"})
    for anchor, x, z in BEARINGS:
        structure.append({"id": "support-" + anchor, "kind": "box", "bone": "base", "material": "concrete", "role": "ground", "tone": 0.2, "member": "base", "part": "support", "anchor": anchor, "size": [1.0, 0.01, 1.2], "transform": {"translation": [x, 0.005, z]}})
    lists["structure"] = structure
    english, german = DECISIONS[scale]
    lists["decisions"] = [entry for entry in lists["decisions"] if not entry["en"].startswith("Access ladder and brake")] + [{"en": english, "de": german}]


document = json.loads(SOURCE.read_text())
for scale, lists in document["volumes"].items():
    edit(scale, lists)
SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print({scale: {name: len(lists[name]) for name in ("body", "thermal", "structure")} for scale, lists in document["volumes"].items()})
