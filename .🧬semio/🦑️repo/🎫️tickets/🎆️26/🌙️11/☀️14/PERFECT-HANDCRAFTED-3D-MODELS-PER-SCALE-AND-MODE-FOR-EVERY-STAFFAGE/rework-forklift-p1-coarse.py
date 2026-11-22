"""🚜 Forklift 1:100, 1:200 and 1:500, pass 1 (second round): the drive axle as body, the mast on its pivot, steer axle on its pendulum block with stubs into the wheels, carriage on the mast, every carried part joined, each sized for the smallest feature of its scale (15 / 30 / 75 mm).

Run from the ticket folder after rework-forklift-p1-50.py: python3 rework-forklift-p1-coarse.py
"""
import copy, math
exec(open("rework-10-lib.py").read())
S = Source("forklift")
M2 = lambda dz: {"count": 2, "translation": [0, 0, dz]}
FINE = {s["id"]: s for s in S.v["50"]["body"]}


class Scale:
    """🎚️ One scale of the source with its three lists."""

    def __init__(self, scale):
        self.scale = scale
        self.v = S.v[str(scale)]
        self.b = {s["id"]: s for s in self.v["body"]}

    def put(self, solid, region):
        S.put(self.scale, [solid])
        self.b[solid["id"]] = solid
        self.v["thermal"] = [t for t in self.v["thermal"] if t["id"] != solid["id"] and t.get("of") != solid["id"]]
        self.v["thermal"].append({"id": "th-" + solid["id"], "of": solid["id"], "region": region})

    def fine(self, sid, region):
        self.put(copy.deepcopy(FINE[sid]), region)

    def member(self, mid, of, bone, part="member"):
        self.v["structure"] = [m for m in self.v["structure"] if m["id"] != mid]
        self.v["structure"].append({"id": mid, "of": of, "member": bone, "part": part})

    def unmember(self, *ids):
        self.v["structure"] = [m for m in self.v["structure"] if m["id"] not in ids]

    def drive(self, motor_r, gear_r, flange_to):
        own = {t["id"]: t for t in self.v["thermal"] if "of" not in t}
        base = {k: v for k, v in own["motor"].items() if k != "region"}
        motor = {**base, "profile": [[0, motor_r], [0.6, motor_r]], "origin": [0, 0, -0.3]}
        motor.pop("repeat", None)
        gear = {**{k: v for k, v in own["gear-brake"].items() if k != "region"}, "profile": [[0, gear_r], [0.085, gear_r]], "origin": [0, 0, 0.3], "repeat": M2(-0.685)}
        self.put(motor, "drive-motor")
        self.put(gear, "brakes")
        if "terminal-box" in own:
            self.put({k: v for k, v in own["terminal-box"].items() if k != "region"}, "drive-motor")
        self.unmember("drive-axle", "bearing-f")
        self.member("m-motor", "motor", "axle-f")
        self.member("m-gear-brake", "gear-brake", "axle-f")
        if flange_to is not None:
            self.put(cyl("hub-flange-f", "axle-f", "steel", [0, 0, 0.385], [0, 0, 1], 0.07, round(flange_to - 0.385, 4), tone=0.35, role="body", repeat=M2(-round(flange_to + 0.385, 4))), "chassis-paint")
            self.member("bearing-f", "hub-flange-f", "wheel-f", "joint")

    def decide(self, en, de):
        self.v["decisions"] = [d for d in self.v["decisions"] if not d["en"].startswith("Second round")] + [{"en": en, "de": de}]


def chain_profile():
    c = (1.94, -0.035)
    ring = lambda r, angles: [[round(c[0] + r * math.sin(math.radians(a)), 5), round(c[1] - r * math.cos(math.radians(a)), 5)] for a in angles]
    return [[0.44, -0.106], [1.94, -0.106]] + ring(0.071, range(15, 180, 15)) + [[1.94, 0.036], [1.3, 0.036], [1.3, 0.02], [1.94, 0.02]] + ring(0.055, range(165, 0, -15)) + [[1.94, -0.09], [0.44, -0.09]]


A = Scale(100)
A.drive(0.13, 0.105, 0.5225)
A.put(cyl("wheel-dish", "chassis", "plastic" if False else A.b["steering-wheel"]["material"], [1.686, 1.278, 0], [-0.55493, 0.8319, 0], 0.14, 0.016, tone=0.2, role="body"), "chassis-paint")
A.put(cyl("wheel-hub", "chassis", A.b["column"]["material"], [1.701, 1.255, 0], [-0.5547, 0.83205, 0], 0.035, 0.045, tone=0.2, role="body"), "chassis-paint")
A.put(box("armrest-arm", "chassis", "steel", [0.04, 0.03, 0.15], [1.136, 1.03, -0.245], tone=0.25, role="body"), "chassis-paint")
A.put(box("steer-cyl-lug", "axle-r", "steel", [0.05, 0.05, 0.03], [-0.09, 0.0074, 0.2], tone=0.35, role="body", repeat=M2(-0.4)), "chassis-paint")
A.fine("kingpin", "chassis-paint")
A.fine("stub-axle", "chassis-paint")
A.put(box("steer-arm", "axle-r", "steel", [0.15, 0.03, 0.05], [-0.06, 0.0074, 0.35], tone=0.35, role="body", repeat=M2(-0.7)), "chassis-paint")
A.put(box("axle-pivot-block", "axle-r", "cast-iron", [0.2, 0.225, 0.12], [0, 0.1725, 0], tone=0.3, role="body"), "chassis-paint")
A.b["post-rear"]["size"] = [0.06, 1.175, 0.05]
A.b["post-rear"]["transform"]["translation"] = [0.556, 1.5925, 0.5]
A.fine("pivot-bracket", "chassis-paint")
A.fine("pivot-pin", "mast")
A.b["sheave"]["repeat"] = M2(-0.2)
A.put(cyl("sheave-axle", "mast-inner", "steel", [1.94, -0.035, -0.125], [0, 0, 1], 0.015, 0.25, tone=0.35, role="body"), "mast")
A.put(box("sheave-hanger", "mast-inner", "steel", [0.105, 0.03, 0.05], [1.9575, -0.035, 0], tone=0.4, role="body"), "mast")
chain = A.b["chain"]
for key in ("size", "transform", "slender"):
    chain.pop(key, None)
chain.update({"kind": "extrude", "profile": chain_profile(), "plane": "xy", "from": 0.08, "to": 0.12, "repeat": M2(-0.2)})
A.fine("chain-anchor", "fork-gear")
A.put(box("chain-anchor-rear", "mast-inner", "steel", [0.04, 0.056, 0.05], [1.3, 0.028, 0.1], tone=0.35, role="body", repeat=M2(-0.2)), "mast")
A.put(box("cyl-seat", "mast-outer", "steel", [0.03, 0.09, 0.07], [-0.115, 0.075, 0.317], tone=0.35, role="body", repeat=M2(-0.634)), "mast")
for rid in ("carriage-roller-1", "carriage-roller-2"):
    A.fine(rid, "fork-gear")
A.fine("inner-roller", "mast")
A.unmember("stub-axles", "pin-pivot")
A.member("j-kingpin", "kingpin", "axle-r", "joint")
A.member("j-stub-axle", "stub-axle", "axle-r", "joint")
A.member("m-axle-pivot", "axle-pivot-block", "axle-r")
A.member("m-fender-deck", "fender-deck", "chassis")
A.member("m-floor-plate", "floor-plate", "chassis")
A.member("m-tilt-lug-base", "tilt-lug-base", "chassis")
A.member("m-tilt-lug", "tilt-lug", "mast-outer")
A.member("m-pivot-bracket", "pivot-bracket", "chassis")
A.member("j-pivot-pin", "pivot-pin", "mast-outer", "joint")
A.member("m-cyl-seat", "cyl-seat", "mast-outer")
A.member("j-carriage-roller-1", "carriage-roller-1", "carriage", "joint")
A.member("j-carriage-roller-2", "carriage-roller-2", "carriage", "joint")
A.member("j-inner-roller", "inner-roller", "mast-inner", "joint")
A.decide(
    "Second round: at 15 mm the drive unit is one motor barrel with its terminal boxes, the two gear and brake housings and the hub flanges into the rim discs; the mast stands on its pivot brackets and pins; the steer axle hangs on a pendulum block under the counterweight and reaches the wheels through king pins and stub axles, the steering cylinder sits on two lugs with steering arms to the king pins; the steering wheel is a ring on a dished plate with its hub on the column; sheaves on one axle under a hanger of the inner head, chains wrapped over them between carriage and rear anchors; lift cylinders on seats, carriage and mast rollers in their channels. The structure takes the same path through fender decks, floor plate, pivot and rollers.",
    "Zweite Runde: Bei 15 mm ist die Antriebseinheit ein Motorzylinder mit Klemmenkästen, zwei Getriebe- und Bremsgehäusen und den Nabenflanschen in die Radschüsseln; der Mast steht auf seinen Lagerböcken mit Bolzen; die Lenkachse hängt an einem Pendellagerbock unter dem Gegengewicht und erreicht die Räder über Achsschenkelbolzen und Achsstummel, der Lenkzylinder sitzt auf zwei Laschen mit Lenkhebeln zu den Achsschenkelbolzen; das Lenkrad ist ein Kranz auf einer Schüssel mit Nabe auf der Lenksäule; Kettenrollen auf einer Achse unter einem Halter des Innenmastkopfs, die Ketten laufen darüber zwischen Gabelträger- und hinterem Anker; Hubzylinder auf Konsolen, Gabelträger- und Mastrollen in ihren Profilen. Das Tragwerk nimmt denselben Weg über Kotflügeldecks, Bodenplatte, Mastlager und Rollen.")

C = Scale(200)
C.drive(0.13, 0.11, 0.495)
C.b["column"]["profile"] = [[0, 0.035], [0.26, 0.035]]
C.b["armrest"]["transform"]["translation"] = [1.256, 1.225, -0.28]
C.b["steer-cylinder"]["origin"] = [-0.1, 0.0074, -0.33]
C.put(cyl("stub-axle", "axle-r", "steel", [0, 0, 0.33], [0, 0, 1], 0.045, 0.155, tone=0.35, role="body", repeat=M2(-0.815)), "chassis-paint")
C.put(box("axle-pivot-block", "axle-r", "cast-iron", [0.2, 0.225, 0.12], [0, 0.1725, 0], tone=0.3, role="body"), "chassis-paint")
C.put(box("pivot-bracket", "chassis", "steel", [0.15, 0.1, 0.03], [2.141, 0.3, 0.36], tone=0.35, role="body", repeat=M2(-0.72)), "chassis-paint")
C.put(box("carriage-plate", "carriage", "steel", [0.43, 0.225, 0.03], [0.215, 0.0825, 0.203], tone=0.35, role="body", repeat=M2(-0.406)), "fork-gear")
C.unmember("pin-pivot")
C.member("j-stub-axle", "stub-axle", "axle-r", "joint")
C.member("m-axle-pivot", "axle-pivot-block", "axle-r")
C.member("pin-pivot", "pivot-bracket", "mast-outer", "joint")
C.member("m-carriage-plate", "carriage-plate", "carriage")
C.member("m-fender", "fender", "chassis")
C.member("m-floor-plate", "floor-plate", "chassis")
C.member("m-tilt-lug", "tilt-lug", "mast-outer")
C.decide(
    "Second round: at 30 mm the drive axle is body (motor barrel, two gear and brake housings, hub flanges into the wheel dishes), the mast stands on two pivot brackets beside the front frame, the carriage runs on two side plates in the inner rails, the steer axle hangs on its pendulum block and carries the wheels on stub axles with the steering cylinder bolted to the beam; column into the steering wheel, armrest on the seat side.",
    "Zweite Runde: Bei 30 mm ist die Antriebsachse Körper (Motorzylinder, zwei Getriebe- und Bremsgehäuse, Nabenflansche in die Radschüsseln), der Mast steht auf zwei Lagerböcken am Vorderrahmen, der Gabelträger läuft mit zwei Seitenblechen in den Innenprofilen, die Lenkachse hängt am Pendellagerbock und trägt die Räder auf Achsstummeln, der Lenkzylinder ist an den Achskörper geschraubt; Lenksäule bis ins Lenkrad, Armlehne an der Sitzseite.")

D = Scale(500)
D.drive(0.13, 0.11, None)
D.put(box("mast-foot", "chassis", "steel", [0.12, 0.15, 0.714], [2.106, 0.3, 0], tone=0.35, role="body"), "chassis-paint")
D.put(box("tilt-lug-base", "chassis", "steel", [0.09, 0.125, 0.09], [1.796, 0.7275, 0.42], tone=0.35, role="body", repeat=M2(-0.84)), "chassis-paint")
D.put(box("tilt-lug", "mast-outer", "steel", [0.1, 0.075, 0.11], [0.65, -0.0275, 0.412], tone=0.35, role="body", repeat=M2(-0.824)), "mast")
D.put(box("axle-pivot-block", "axle-r", "cast-iron", [0.15, 0.23, 0.15], [0, 0.177, 0], tone=0.3, role="body"), "chassis-paint")
D.unmember("cw-beam", "pallet-base", "pallet-column", "pallet-stringer")
D.member("m-cw-block", "cw-block", "counterweight")
D.member("m-pallet", "pallet", "pallet")
D.member("m-fender", "fender", "chassis")
D.member("m-mast-foot", "mast-foot", "chassis")
D.member("m-tilt-lug-base", "tilt-lug-base", "chassis")
D.member("m-tilt-lug", "tilt-lug", "mast-outer")
D.member("m-axle-pivot", "axle-pivot-block", "axle-r")
D.decide(
    "Second round: at 75 mm the drive axle is one barrel with its two gear housings between the wheels, the mast stands on a foot block on the axle line, the tilt cylinders run from a lug base on the fender to a lug on the mast rail, and the steer axle hangs on one block under the counterweight; the structure carries counterweight, pallet, fenders and these blocks as members so that every path ends in a tyre patch.",
    "Zweite Runde: Bei 75 mm ist die Antriebsachse ein Zylinder mit zwei Getriebegehäusen zwischen den Rädern, der Mast steht auf einem Fußblock in der Achslinie, die Neigezylinder laufen von einem Laschensockel auf dem Kotflügel zu einer Lasche am Mastprofil, die Lenkachse hängt an einem Block unter dem Gegengewicht; das Tragwerk führt Gegengewicht, Palette, Kotflügel und diese Blöcke als Stäbe, sodass jeder Lastpfad in einer Reifenaufstandsfläche endet.")
S.save()
for sc in (100, 200, 500):
    v = S.v[str(sc)]
    print("[DEBUG]", sc, len(v["body"]), len(v["thermal"]), len(v["structure"]))
