"""🏥️ Author 14, operating table with surgical light: parts that did not meet.

Body and thermal, 1:100: the tilt trunnion between column head and saddle cheeks and the four hinge knuckles between back, seat and leg sections were missing, so the top lay in five loose pieces 5 to 10 mm apart; the bearing between the yoke and the light head was missing, so the head hung 23 mm beside the yoke. 1:200: the same bearing was missing. 1:500: the arm ended 1 mm short of the column tube.
Structure: the ballast block that carries the column on the stand spine was no member at 1:50, 1:100 and 1:200 (the column stood 55 mm over the spine); at 1:100 the hinges were no joints, so back and leg beams hung free; at 1:50 the gas spring of the arm ended 6 mm short of the root joint and 3 mm off the link it presses on, and the sterile handle, which carries nothing, hung as a member under the head; at 1:500 legs and spine of the stand ended 15 mm under the tube and the head spar 25 mm under the yoke drop.

Bone frames: `top` u = x, v = y - 0.740, w = z; `light-head` u = x - 0.370, v = y - 1.690; `stand-arm` u = 1.170 - x, v = 1.920 - y; `stand-base` u = x - 1.170, v = y.
Run from the ticket folder: python3 rework-14-operating-table-joints.py
"""
import json

PATH = "volumes/furniture/operating-table.json"
document = json.load(open(PATH))
volumes = document["volumes"]


def put(scale, name, after, entry):
    entries = [e for e in volumes[scale][name] if e["id"] != entry["id"]]
    index = next(i for i, e in enumerate(entries) if e["id"] == after) + 1
    entries.insert(index, entry)
    volumes[scale][name] = entries


def find(scale, name, ident):
    return next(e for e in volumes[scale][name] if e["id"] == ident)


def pin(ident, radius, length, origin, bone="top", material="stainless-steel", axis=(0, 0, 1), role="detail", tone=0.4):
    return {"id": ident, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[0.0, 0.0], [0.0, radius], [length, radius], [length, 0.0]], "axis": list(axis), "origin": list(origin)}


HINGES = (("hinge-back-r", [-0.22, 0.06, -0.25]), ("hinge-back-l", [-0.22, 0.06, 0.2]), ("hinge-leg-r", [0.26, 0.06, -0.25]), ("hinge-leg-l", [0.26, 0.06, 0.2]))

put("100", "body", "column-head", pin("tilt-pin", 0.015, 0.2, [0.0, 0.0, -0.1]))
put("100", "thermal", "th-column-head", {"id": "th-tilt-pin", "of": "tilt-pin", "region": "table-frame"})
after_body, after_thermal, after_structure = "frame-leg-l", "th-frame-leg-l", "member-top-cross-4"
for ident, origin in HINGES:
    put("100", "body", after_body, pin(ident, 0.022, 0.05, origin))
    put("100", "thermal", after_thermal, {"id": f"th-{ident}", "of": ident, "region": "table-frame"})
    put("100", "structure", after_structure, {"id": f"joint-{ident}", "of": ident, "member": "top", "part": "joint"})
    after_body, after_thermal, after_structure = ident, f"th-{ident}", f"joint-{ident}"

put("100", "body", "yoke-drop", pin("head-bearing", 0.019, 0.057, [0.24, 0.0, 0.0], bone="light-head", material="light-aluminium", axis=(1, 0, 0), role="body"))
put("100", "thermal", "th-yoke-drop", {"id": "th-head-bearing", "of": "head-bearing", "region": "light-housing"})
put("100", "structure", "member-head-cross", {"id": "joint-head-bearing", "of": "head-bearing", "member": "light-head", "part": "joint"})
put("200", "body", "yoke-drop", pin("head-bearing", 0.02, 0.06, [0.24, 0.0, 0.0], bone="light-head", material="light-aluminium", axis=(1, 0, 0), role="body"))
put("200", "thermal", "th-yoke-drop", {"id": "th-head-bearing", "of": "head-bearing", "region": "light-housing"})
put("200", "structure", "member-head-cross", {"id": "joint-head-bearing", "of": "head-bearing", "member": "light-head", "part": "joint"})

arm = find("500", "body", "arm-body")
arm["size"], arm["transform"]["translation"] = [0.83, 0.06, 0.06], [0.415, 0.0, 0.0]

for scale in ("50", "100", "200"):
    put(scale, "structure", "member-stand-spine", {"id": "member-stand-ballast", "of": "stand-ballast", "member": "stand-base", "part": "member"})

spring = find("50", "structure", "member-arm-spring")
spring["p0"], spring["p1"] = [0.02, -0.008, 0.0], [0.55, 0.016, 0.0]
volumes["50"]["structure"] = [e for e in volumes["50"]["structure"] if e["id"] != "member-head-handle"]

for ident, x in (("member-stand-leg-near", -0.24), ("member-stand-leg-far", 0.24)):
    leg = find("500", "structure", ident)
    leg["size"], leg["transform"]["translation"] = [0.08, 0.115, 0.48], [x, 0.0575, 0.0]
spine = find("500", "structure", "member-stand-spine")
spine["size"], spine["transform"]["translation"] = [0.44, 0.095, 0.075], [0.0, 0.0675, 0.0]
spar = find("500", "structure", "member-head-spar")
spar["size"], spar["transform"]["translation"] = [0.36, 0.1, 0.075], [0.0, 0.02, 0.0]

SENTENCES = {
    "50": (1, " In the structure the ballast block carries the column on the stand spine, the gas spring of the arm runs from the root joint onto the lower link, and the sterile handle, which carries nothing, is no member.", " Im Tragwerk trägt der Ballastblock die Säule auf dem Stativholm, die Gasfeder des Arms läuft vom Wurzelgelenk auf den unteren Lenker, und der Sterilgriff, der nichts trägt, ist kein Stab."),
    "100": (0, " The tilt trunnion through column head and saddle cheeks, the four hinge knuckles between back, seat and leg sections and the bearing between yoke and light head are kept as 30 to 44 mm pins, because without them the top lies in loose pieces and the head hangs beside its yoke; they are joints of the structure, where the ballast block also carries the column.", " Der Kippzapfen durch Säulenkopf und Sattelwangen, die vier Gelenkaugen zwischen Rücken-, Sitz- und Beinteil und das Lager zwischen Bügel und Leuchtenkopf bleiben als Bolzen von 30 bis 44 mm, weil die Platte sonst in losen Stücken läge und der Kopf neben seinem Bügel hinge; im Tragwerk sind sie Gelenke, dort trägt auch der Ballastblock die Säule."),
    "200": (0, " The bearing between the yoke drop and the light head is kept as a 40 mm pin so that the head hangs in its yoke; in the structure it is the joint of the head, and the ballast block carries the column.", " Das Lager zwischen Bügelschenkel und Leuchtenkopf bleibt als 40-mm-Bolzen, damit der Kopf in seinem Bügel hängt; im Tragwerk ist es das Gelenk des Kopfes, und der Ballastblock trägt die Säule."),
    "500": (0, " The arm runs to the axis of the column tube; in the structure the legs and the spine of the stand reach the foot of the tube and the head spar reaches the yoke drop.", " Der Arm läuft bis zur Achse des Säulenrohrs; im Tragwerk reichen Beine und Holm des Stativs bis an den Rohrfuß und der Kopfholm bis an den Bügelschenkel."),
}
for scale, (index, en, de) in SENTENCES.items():
    decision = volumes[scale]["decisions"][index]
    if en not in decision["en"]:
        decision["en"] += en
        decision["de"] += de
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
