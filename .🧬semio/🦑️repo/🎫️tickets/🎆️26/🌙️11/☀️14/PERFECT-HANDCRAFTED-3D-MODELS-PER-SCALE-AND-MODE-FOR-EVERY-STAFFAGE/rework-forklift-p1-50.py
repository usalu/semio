"""🚜 Forklift 1:50, pass 1 (second round): the drive axle unit as body, every carried part joined to what carries it (armrest arm, steering linkage, pendulum axle pivot, sheave yokes, chain over the sheave, cylinder seats, rollers that fill their channels), the overhead guard and tilt lugs in the load path.

Run from the ticket folder: python3 rework-forklift-p1-50.py
"""
import math
exec(open("rework-10-lib.py").read())
S = Source("forklift")
V = S.v["50"]
B = {s["id"]: s for s in V["body"]}
M2 = lambda dz: {"count": 2, "translation": [0, 0, dz]}


def thermal(sid, region):
    V["thermal"] = [t for t in V["thermal"] if t["id"] != sid and t.get("of") != sid]
    V["thermal"].append({"id": "th-" + sid, "of": sid, "region": region})


def member(mid, of, bone, part="member"):
    V["structure"] = [m for m in V["structure"] if m["id"] != mid]
    V["structure"].append({"id": mid, "of": of, "member": bone, "part": part})


drive = [t for t in V["thermal"] if t["id"] in ("motor-mount", "terminal-box", "motor", "gear-brake")]
regions = {t["id"]: t["region"] for t in drive}
for t in drive:
    solid = {k: v for k, v in t.items() if k != "region"}
    if solid["id"] == "motor":
        solid["repeat"] = M2(-0.34)
    S.put(50, [solid])
    thermal(solid["id"], regions[solid["id"]])
S.put(50, [cyl("hub-flange-f", "axle-f", "steel", [0, 0, 0.385], [0, 0, 1], 0.07, 0.1375, tone=0.35, role="body", repeat=M2(-0.9075))])
thermal("hub-flange-f", "chassis-paint")
V["structure"] = [m for m in V["structure"] if m["id"] not in ("drive-axle", "bearing-f")]
member("m-motor-mount", "motor-mount", "axle-f")
member("m-motor", "motor", "axle-f")
member("m-gear-brake", "gear-brake", "axle-f")
member("bearing-f", "hub-flange-f", "wheel-f", "joint")

S.put(50, [box("armrest-arm", "chassis", "steel", [0.04, 0.03, 0.15], [1.136, 1.03, -0.245], tone=0.25, role="body", cornerRadius=0.006)])
thermal("armrest-arm", "chassis-paint")

S.put(50, [
    box("steer-cyl-lug", "axle-r", "steel", [0.05, 0.05, 0.03], [-0.09, 0.0074, 0.2], tone=0.35, role="body", repeat=M2(-0.4)),
    box("steer-arm", "axle-r", "steel", [0.15, 0.03, 0.05], [-0.06, 0.0074, 0.35], tone=0.35, role="body", repeat=M2(-0.7)),
    box("axle-pivot-block", "axle-r", "cast-iron", [0.2, 0.225, 0.12], [0, 0.1725, 0], tone=0.3, role="body", cornerRadius=0.01),
    cyl("axle-pivot-pin", "axle-r", "steel", [-0.13, 0.0924, 0], [1, 0, 0], 0.03, 0.26, tone=0.35, role="body"),
])
for sid in ("steer-cyl-lug", "steer-arm", "axle-pivot-block", "axle-pivot-pin"):
    thermal(sid, "chassis-paint")
B["stub-axle"]["profile"] = [[0, 0.045], [0.175, 0.045]]
member("m-axle-pivot", "axle-pivot-block", "axle-r")
member("j-axle-pivot-pin", "axle-pivot-pin", "axle-r", "joint")

B["post-rear"]["size"] = [0.06, 1.175, 0.05]
B["post-rear"]["transform"]["translation"] = [0.556, 1.5925, 0.5]
member("m-fender-deck", "fender-deck", "chassis")
member("m-floor-plate", "floor-plate", "chassis")
member("m-tilt-lug-base", "tilt-lug-base", "chassis")
member("m-tilt-lug", "tilt-lug", "mast-outer")
member("m-pivot-bracket", "pivot-bracket", "chassis")
B["pivot-pin"]["profile"] = [[0, 0.03], [0.072, 0.03]]

B["sheave"]["repeat"] = M2(-0.2)
S.put(50, [
    box("sheave-yoke-l", "mast-inner", "steel", [0.105, 0.03, 0.008], [1.9575, -0.035, 0.071], tone=0.4, role="body", repeat=M2(0.058)),
    box("sheave-yoke-r", "mast-inner", "steel", [0.105, 0.03, 0.008], [1.9575, -0.035, -0.129], tone=0.4, role="body", repeat=M2(0.058)),
    box("cyl-seat", "mast-outer", "steel", [0.03, 0.09, 0.07], [-0.115, 0.075, 0.317], tone=0.35, role="body", repeat=M2(-0.634)),
])
thermal("sheave-yoke-l", "mast")
thermal("sheave-yoke-r", "mast")
thermal("cyl-seat", "mast")
member("m-cyl-seat", "cyl-seat", "mast-outer")
c = (1.94, -0.035)
outer = [[0.44, -0.106], [1.94, -0.106]] + [[round(c[0] + 0.071 * math.sin(math.radians(a)), 5), round(c[1] - 0.071 * math.cos(math.radians(a)), 5)] for a in range(15, 180, 15)] + [[1.94, 0.036], [1.3, 0.036]]
inner = [[1.3, 0.02], [1.94, 0.02]] + [[round(c[0] + 0.055 * math.sin(math.radians(a)), 5), round(c[1] - 0.055 * math.cos(math.radians(a)), 5)] for a in range(165, 0, -15)] + [[1.94, -0.09], [0.44, -0.09]]
chain = B["chain"]
for key in ("size", "transform", "slender"):
    chain.pop(key, None)
chain.update({"kind": "extrude", "profile": outer + inner, "plane": "xy", "from": 0.08, "to": 0.12, "repeat": M2(-0.2)})
S.put(50, [box("chain-anchor-rear", "mast-inner", "steel", [0.04, 0.056, 0.05], [1.3, 0.028, 0.1], tone=0.35, role="body", repeat=M2(-0.2))])
thermal("chain-anchor-rear", "mast")

for rid in ("carriage-roller-1", "carriage-roller-2"):
    B[rid]["profile"] = [[0, 0.042], [0.06, 0.042]]
B["inner-roller"]["profile"] = [[0, 0.0425], [0.028, 0.0425]]

V["decisions"] = V["decisions"][:2] + [{
    "en": "Second round: the drive unit under the floor plate is body as well (twin motors on a common mount, terminal boxes, gear and brake housings through the frame plates, hub flanges into the rim discs), so the drive wheels hang on a real axle. Every carried part meets what carries it: the armrest on its arm from the seat base, the steering cylinder on two lugs with the rod linked to the king pins by steering arms, the steer axle on its pendulum pivot block and pin under the counterweight, the sheaves in yokes under the inner head with the lift chains wrapped over them from the carriage anchors to the rear anchors, the lift cylinders on their seats on the lower tie, mast and carriage rollers filling their channels, the rear guard posts let into the counterweight. The structure follows the same path: guard posts and tilt lugs through fender decks and floor plate into the front frame plates, wheels through hub flanges and the axle unit.",
    "de": "Zweite Runde: Die Antriebseinheit unter der Bodenplatte ist ebenfalls Körper (zwei Fahrmotoren auf gemeinsamem Träger, Klemmenkästen, Getriebe- und Bremsgehäuse durch die Rahmenbleche, Nabenflansche in die Radschüsseln), die Antriebsräder sitzen damit auf einer echten Achse. Jedes getragene Teil trifft sein Tragteil: die Armlehne auf ihrem Arm am Sitzunterbau, der Lenkzylinder auf zwei Laschen, die Stange über Lenkhebel an den Achsschenkelbolzen, die Lenkachse am Pendellagerbock mit Bolzen unter dem Gegengewicht, die Kettenrollen in Gabeln unter dem Innenmastkopf mit den Hubketten vom Gabelträgeranker über die Rollen zum hinteren Anker, die Hubzylinder auf ihren Konsolen an der unteren Traverse, Mast- und Gabelträgerrollen füllen ihre Profile, die hinteren Schutzdachpfosten sind in das Gegengewicht eingelassen. Das Tragwerk folgt demselben Weg: Dachpfosten und Neigelaschen über Kotflügeldecks und Bodenplatte in die vorderen Rahmenbleche, Räder über Nabenflansche und Achseinheit.",
}]
S.save()
print("[DEBUG] 1:50 body", len(V["body"]), "thermal", len(V["thermal"]), "structure", len(V["structure"]))
