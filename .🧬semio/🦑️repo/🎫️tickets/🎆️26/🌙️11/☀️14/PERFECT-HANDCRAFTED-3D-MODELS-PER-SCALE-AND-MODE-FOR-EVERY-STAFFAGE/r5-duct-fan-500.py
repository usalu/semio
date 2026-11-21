"""🌀 Round 5, appliances/duct-fan: the 1:500 model authored anew as a crisp massing (rounded plate with a true round bore, housing tube, motor cross, can and three swept blade planforms); rewrites only `volumes.500` of `volumes/appliances/duct-fan.json`.

Usage: python3 r5-duct-fan-500.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("v", os.path.join(HERE, "r3-duct-fan-volumes.py"))
v = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(v)
k = v.k

SOURCE = os.path.join(HERE, "volumes", "appliances", "duct-fan.json")
CROSS = {"count": 2, "rotationAxis": [0, 0, 1], "rotationRad": round(math.pi / 2, 7)}


def scale500():
    """🔎 1:500: plate 12 mm with a 32-sided round bore, tube, a two-bar motor cross, the 80 mm can and three swept 8 mm blade planforms."""
    bore = 0.096
    body = [
        v.solid("plate", "extrude", "plate", tone=0.88, profile=k.rounded_square(k.PLATE / 2, k.PLATE_R, n=3), holes=[list(reversed(k.circle(bore, n=32)))], plane="xy", **{"from": k.Z_FACE, "to": k.Z_WALL}),
        v.rev("ring", "ring", [(k.Z_WALL, bore), (k.Z_WALL, k.BORE), (k.Z_END, k.BORE), (k.Z_END, bore)]),
        v.solid("spider", "box", "ring", tone=0.6, size=[0.192, 0.016, 0.016], transform={"translation": [0, 0, 0.02]}, repeat=CROSS),
    ] + [v.planform_blade(i, 3, -0.027, -0.019, root=0.036, tip=0.090, n=4) for i in range(1, 4)] + [
        v.rev("hub", "rotor", [(-0.040, 0), (-0.040, 0.032), (-0.034, 0.040), (0.012, 0.040), (0.012, 0)], loop=False, material="motor-housing", tone=0.55),
    ]
    thermal = [v.ref(f"th-{s}", s, region="casing", layer="shell") for s in ("plate", "ring", "spider")]
    thermal += [v.ref(f"th-blade-{i}", f"blade-{i}", region="blades", layer="blade") for i in range(1, 4)]
    thermal += [v.ref("th-hub", "hub", region="hub", layer="can"), v.extract(0.040, bore, -0.0028, -0.0003)]
    structure = [
        v.ref("member-plate", "plate", member="plate", part="member"),
        v.ref("member-ring", "ring", member="ring", part="member"),
        v.ref("member-spider", "spider", member="ring", part="member"),
        v.ref("member-hub", "hub", member="rotor", part="member"),
        v.mass("mass-fan", "ring", 0.014, -0.016),
        v.pin(k.Z_FACE, k.Z_WALL, 0.012, v.CORNERS[0]),
        v.roller(0.012, 0.010, v.CORNERS[1], -0.039),
    ]
    decisions = [
        v.text("Own 1:500 model, a crisp massing of the plate-type wall fan: the 0.22 m plate 12 mm thick with rounded corners and a true round 192 mm bore, the housing tube to the duct end, and the motor held on a cross of two 16 mm bars.",
               "Eigenes 1:500-Modell, eine klare Massenform des Wandventilators: die Platte von 0,22 m, 12 mm stark mit gerundeten Ecken und einer echten runden Bohrung von 192 mm, das Gehäuserohr bis zum Kanalende und der Motor auf einem Kreuz aus zwei Stäben von 16 mm."),
        v.text("The rotor reads as the 80 mm can with three broad swept blades, each its planform 8 mm thick in the blade plane; pitch, twist, nozzle, bead and screw caps are merged away at this scale.",
               "Das Laufrad liest sich als Becher von 80 mm mit drei breiten gepfeilten Flügeln, jeder sein Grundriss 8 mm stark in der Flügelebene; Anstellung, Verwindung, Düse, Wulst und Schraubenkappen gehen in diesem Maßstab auf."),
        v.text("Thermal by reference with the outlet annulus; structure: plate, tube, cross and can as members, one mass of the whole fan at its centre and the two corner screws as pin and roller.",
               "Thermik als Verweis mit dem Auslassring; Tragwerk: Platte, Rohr, Kreuz und Becher als Stäbe, eine Masse des ganzen Ventilators in seinem Schwerpunkt und die zwei Eckschrauben als Fest- und Rollenlager."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    """💾 Replace `volumes.500` in the source."""
    with open(SOURCE, encoding="utf-8") as f:
        doc = json.load(f)
    doc["volumes"]["500"] = scale500()
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("written 1:500", SOURCE)


if __name__ == "__main__":
    main()
