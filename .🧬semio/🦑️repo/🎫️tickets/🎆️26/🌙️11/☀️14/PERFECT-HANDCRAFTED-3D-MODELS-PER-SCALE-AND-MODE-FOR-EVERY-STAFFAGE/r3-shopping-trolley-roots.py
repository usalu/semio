"""🦴 Round 3 shopping trolley roots: rig on the real construction (chassis on the rail end, basket on the floor rear edge, handle on the grip axis, gate hanging from the rim bar), anchors at the real wheel contacts, masses and centres from the parts, push load on the grip, region titles and areas, provenance."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("trolley_kit", os.path.join(T, "r3-shopping-trolley-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)


def tx(en, de):
    return {"en": en, "de": de}


MASS = {"chassis": 6.7, "basket": 5.5, "gate": 1.0, "handle": 0.8}
COM_WORLD = {"chassis": (-0.048, 0.256), "basket": (0.04, 0.69)}
COM_LOCAL = {"gate": (0.165, 0.0), "handle": (0.012, -0.004)}


def com_local(bone):
    if bone in COM_LOCAL:
        return COM_LOCAL[bone]
    return tuple(k.local(bone, COM_WORLD[bone]))


def rig():
    def bone(bid, parent, title, length, joint, ref="trolley-class"):
        o = k.BONES[bid]["origin"]
        if parent is None:
            x, y = o
        else:
            po = k.BONES[parent]["origin"]
            x, y = o[0] - po[0], o[1] - po[1]
        cx, cy = com_local(bid)
        return {"id": bid, "parent": parent, "title": title, "x": round(x, 6), "y": round(y, 6), "angleDeg": k.BONES[bid]["angle"] - (k.BONES[parent]["angle"] if parent else 0.0), "lengthM": length, "joint": joint, "mass": {"kg": MASS[bid], "comX": round(cx, 4), "comY": round(cy, 4), "distribution": "point", "source": "authored-estimate", "reference": ref}}

    return {
        "kind": "mechanical",
        "root": "chassis",
        "bones": [
            bone("chassis", None, tx("Chassis: bent rails and push-frame uprights with castors and crate tray", "Fahrgestell: gebogene Holme und Schiebebügel mit Lenkrollen und Kistenablage"), 0.625, {"type": "fixed"}),
            bone("basket", "chassis", tx("Wire basket", "Drahtkorb"), 0.62, {"type": "fixed"}),
            bone("handle", "chassis", tx("Push handle", "Schiebegriff"), 0.055, {"type": "fixed"}),
            bone("gate", "basket", tx("Rear nesting gate", "Hintere Nesting-Klappe"), k.GATE_LEN, {"type": "revolute", "minDeg": 0, "maxDeg": 80}),
        ],
    }


def anchors(old):
    by = {a["id"]: a for a in old}
    out = []
    for pos, lr, s in (("rear", "left", 1), ("rear", "right", -1), ("front", "left", 1), ("front", "right", -1)):
        c = k.CASTORS[pos]
        x, y = k.local("chassis", (c["xs"] - k.TRAIL, 0.0))
        a = dict(by["wheel-%s-%s" % (pos, lr)])
        a.update({"x": round(x, 6), "y": round(y, 6), "z": round(s * c["zs"], 6)})
        out.append(a)
    a = dict(by["basket-centre"])
    a.update({"x": 0.31, "y": round(k.floor_y(0.31) + 0.003, 4)})
    out.append(a)
    return out


PROVENANCE_REF = {"id": "wanzl-catalogue-2025", "citation": "Wanzl Metallwarenfabrik GmbH (2025). Shopping trolleys, catalogue 1193 EN: order sections EL, ELX, Light, DRC, ELT series (EL 101 1002 x 600 x 978 mm, ELX 101 1035 x 590 x 978 mm, 100-101 l baskets, four swivel castors o125 mm with rubber tread, nesting depth 202-230 mm, crate holder 20 kg), accessories (handle caps, coin deposit systems)."}

NOTES = tx(
    "Side elevation, push direction to the right, handle at the left. Dimensions and the 14 kg mass are class values inside the band of the Wanzl EL/ELX 101 data (authored estimates). The construction is the real one of the class: two o25 tubes bent from the lower rails up into the push-frame uprights, rails converging to the front and open at the rear bottom so the next trolley nests, front cross tube and struts under the basket, crate tray on the rails, four swivel castors o125 trailing 35 mm, welded o4.5 wire basket with o8 rim, floor rising 25 mm to the front, rear gate hanging from the rim bar. The nested state is the gate swung up, as when another trolley nose pushes in; the second trolley is not drawn. Castors are drawn static: a rolling clip is not authored because the planted root solver is not used for caster swivel.",
    "Seitenansicht, Schieberichtung nach rechts, Griff links. Maße und die Masse von 14 kg sind Klassenwerte im Band der Daten von Wanzl EL/ELX 101 (Schätzwerte). Die Konstruktion ist die reale der Klasse: zwei Rohre o25, von den unteren Holmen in die Schiebebügel hochgebogen, Holme nach vorn zusammenlaufend und hinten unten offen, damit der nächste Wagen einfährt, vorderes Querrohr und Stützen unter dem Korb, Kistenablage auf den Holmen, vier Lenkrollen o125 mit 35 mm Nachlauf, geschweißter Drahtkorb o4,5 mit Randdraht o8, Boden nach vorn um 25 mm steigend, hintere Klappe am Randstab hängend. Der Nesting-Zustand ist die hochgeschwenkte Klappe, wie wenn die Spitze eines anderen Wagens hineingeschoben wird; der zweite Wagen wird nicht gezeichnet. Die Rollen sind statisch gezeichnet: ein Rollclip ist nicht verfasst, weil der Wurzellöser für das Schwenken der Lenkrollen nicht genutzt wird.",
)

STRUCTURE_ASSUMPTIONS = [
    tx("A rigid body on four wheel contacts (rear wheels at x = -0.295 m, z = +-0.25 m; front wheels at x = +0.265 m, z = +-0.198 m; all castors trailing 35 mm behind their swivel axes); the gate is a revolute joint posed, not simulated.", "Ein starrer Körper auf vier Radkontakten (hintere Räder bei x = -0,295 m, z = +-0,25 m; vordere Räder bei x = +0,265 m, z = +-0,198 m; alle Rollen 35 mm hinter ihrer Schwenkachse nachlaufend); die Klappe ist ein Drehgelenk, das gestellt, nicht simuliert wird."),
    tx("Goods mass of 30 kg and the 40 N push are authored estimates.", "Warenmasse von 30 kg und Schub von 40 N sind Schätzwerte."),
]


def apply(asset, areas=None):
    prov = asset["provenance"]
    if not any(r["id"] == PROVENANCE_REF["id"] for r in prov["references"]):
        prov["references"].append(PROVENANCE_REF)
    prov["notes"] = NOTES
    asset["rig"] = rig()
    asset["anchors"] = anchors(asset["anchors"])
    for pose in asset["poses"]:
        for load in pose["loads"]:
            if load["id"] == "push":
                load.update({"bone": "handle", "x": 0.0, "y": 0.0})
    asset["structure"]["assumptions"] = STRUCTURE_ASSUMPTIONS
    for region in asset["thermal"]["regions"]:
        if region["id"] == "frame-tube":
            region["title"] = tx("Chassis and handle tubes, castor housings", "Fahrgestell- und Griffrohre, Rollengehäuse")
        if region["id"] == "wheels":
            region["title"] = tx("Castor wheels", "Lenkrollenräder")
        if areas and region["id"] in areas:
            region["areaM2"] = areas[region["id"]]
    return asset


def masses():
    out = {}
    for bone in MASS:
        cx, cy = com_local(bone)
        r = {"chassis": 0.02, "basket": 0.02, "gate": 0.006, "handle": 0.01}[bone]
        out[bone] = ([cx, cy, 0.0], r)
    return out
