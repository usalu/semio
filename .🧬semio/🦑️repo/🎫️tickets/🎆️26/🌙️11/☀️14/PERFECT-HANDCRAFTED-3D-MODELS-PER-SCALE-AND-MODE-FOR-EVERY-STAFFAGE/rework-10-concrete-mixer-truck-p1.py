"""🚛 Author pass 1 on machinery/concrete-mixer-truck: chute lift cylinder on two new bones with its hanger, cab hardware (wipers, indicators, plate, beacons, roof hatch), cambered leaf springs at 1:50, and wheels modelled per scale (tyre, rim, disc, hub for 15 / 30 / 75 mm features) with axle tubes that reach their brakes.

Usage: python3 rework-10-concrete-mixer-truck-p1.py candidate|write
`candidate` writes the source and a candidate asset document (catalog members plus the source volumes) for `r3-validate.ts --file`; `write` writes the source and the catalog members (rig, poses, clips, representations), after which `volumes-insert.py` publishes the volumes.
"""
import sys, math, copy
exec(open("rework-10-lib.py").read())

MODE = sys.argv[1]
S = Source("concrete-mixer-truck")
C = Catalog("concrete-mixer-truck")
A = C.asset

SWIVEL = (-3.95, 1.95)
LIFT_FOOT_U = -0.70
ROD_PIN = (0.55, 0.23)
CHUTE_REST = 202.0


def lift_world(d):
    """📐 World angle and length of the lift cylinder from its foot pin to the chute pin with the chute lowered by d degrees."""
    a, n = math.radians(CHUTE_REST + d), math.radians(CHUTE_REST + 90 + d)
    bx = SWIVEL[0] + ROD_PIN[0] * math.cos(a) + ROD_PIN[1] * math.cos(n)
    by = SWIVEL[1] + ROD_PIN[0] * math.sin(a) + ROD_PIN[1] * math.sin(n)
    ax, ay = SWIVEL[0], SWIVEL[1] + LIFT_FOOT_U
    return math.degrees(math.atan2(by - ay, bx - ax)), math.hypot(bx - ax, by - ay)


REST_ANGLE, REST_LENGTH = lift_world(0)


def lift_pose(d):
    """🔩 Pose angles of barrel and rod bones that keep both on the pin line."""
    t, _ = lift_world(d)
    return {"chute-lift": {"angleDeg": round(t - REST_ANGLE, 3)}, "chute-lift-rod": {"angleDeg": round(t - d - REST_ANGLE, 3)}}


def struct(scale, sid, of, member, part="member"):
    lst = S.v[str(scale)]["structure"]
    if not any(s["id"] == sid for s in lst):
        lst.append({"id": sid, "of": of, "member": member, "part": part})


def own(scale, lst, solid):
    target = S.v[str(scale)][lst]
    at = next((i for i, s in enumerate(target) if s["id"] == solid["id"]), None)
    if at is None:
        target.append(solid)
    else:
        target[at] = solid


def rev(sid, bone, material, profile, axis, origin, tone, role="detail", **more):
    return {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": profile, "axis": axis, "origin": origin, **more}


def loop(points):
    return points + [points[0]]


# ---------------------------------------------------------------- chute lift

HANGER = [[0.02, -0.24], [-0.66, -0.24], [-0.76, -0.10], [-0.76, 0.05], [-0.64, 0.05], [-0.64, -0.08], [-0.60, -0.17], [0.02, -0.17]]
HANGER_500 = [[0.02, -0.25], [-0.68, -0.25], [-0.78, -0.12], [-0.78, 0.05], [-0.64, 0.05], [-0.60, -0.17], [0.02, -0.17]]
HANGER_COARSE = [[0.02, -0.24], [-0.68, -0.24], [-0.76, -0.12], [-0.76, 0.05], [-0.64, 0.05], [-0.60, -0.17], [0.02, -0.17]]


def lift(scale):
    G = "galvanised-steel"
    if scale == 50:
        parts = [
            ext("chute-lift-hanger-l", "chute-swivel", G, HANGER, "xy", 0.022, 0.05, tone=0.42),
            ext("chute-lift-hanger-r", "chute-swivel", G, HANGER, "xy", -0.05, -0.022, tone=0.42),
            rev("chute-lift-foot-pin", "chute-swivel", "cast-iron", [[0, 0], [0, 0.012], [0.13, 0.012], [0.13, 0]], [0, 0, 1], [LIFT_FOOT_U, 0, -0.065], 0.3),
            rev("chute-lift-barrel", "chute-lift", "paint-steel", [[-0.03, 0], [-0.03, 0.022], [0.03, 0.022], [0.03, 0.035], [0.285, 0.035], [0.285, 0.041], [0.31, 0.041], [0.31, 0]], [1, 0, 0], [0, 0, 0], 0.35, role="body"),
            rev("chute-lift-barrel-eye", "chute-lift", "paint-steel", [[0, 0], [0, 0.03], [0.04, 0.03], [0.04, 0]], [0, 0, 1], [0, 0, -0.02], 0.35),
            rev("chute-lift-rod", "chute-lift-rod", "cast-iron", [[0.02, 0], [0.02, 0.018], [0.33, 0.018], [0.33, 0]], [1, 0, 0], [0, 0, 0], 0.7, role="body"),
            rev("chute-lift-rod-eye", "chute-lift-rod", "cast-iron", [[0, 0], [0, 0.028], [0.036, 0.028], [0.036, 0]], [0, 0, 1], [0, 0, -0.018], 0.4),
            box("chute-lift-lug-l", "chute", G, [0.08, 0.075, 0.012], [ROD_PIN[0], 0.2275, 0.026], tone=0.42),
            box("chute-lift-lug-r", "chute", G, [0.08, 0.075, 0.012], [ROD_PIN[0], 0.2275, -0.026], tone=0.42),
            rev("chute-lift-rod-pin", "chute", "cast-iron", [[0, 0], [0, 0.011], [0.09, 0.011], [0.09, 0]], [0, 0, 1], [ROD_PIN[0], ROD_PIN[1], -0.045], 0.3),
        ]
    else:
        rod = {100: 0.018, 200: 0.022, 500: 0.038}[scale]
        barrel = 0.04 if scale == 500 else 0.035
        hanger = {100: HANGER, 200: HANGER_COARSE, 500: HANGER_500}[scale]
        parts = [
            ext("chute-lift-hanger", "chute-swivel", G, hanger, "xy", -0.05, 0.05, tone=0.42),
            rev("chute-lift-barrel", "chute-lift", "paint-steel", [[-0.03, 0], [-0.03, barrel], [0.31, barrel], [0.31, 0]], [1, 0, 0], [0, 0, 0], 0.35, role="body"),
            rev("chute-lift-rod", "chute-lift-rod", "cast-iron", [[-0.03, 0], [-0.03, rod], [0.33, rod], [0.33, 0]], [1, 0, 0], [0, 0, 0], 0.7, role="body"),
            box("chute-lift-lug", "chute", G, [0.08, 0.075, 0.08 if scale == 500 else 0.064], [ROD_PIN[0], 0.2275, 0], tone=0.42),
        ]
    S.put(scale, parts, region="chute")
    for p in parts:
        member = {"chute-swivel": "chute-swivel", "chute-lift": "chute-lift", "chute-lift-rod": "chute-lift-rod", "chute": "chute"}[p["bone"]]
        part = "joint" if p["id"].endswith("-pin") else "member"
        struct(scale, "st-" + p["id"], p["id"], member, part)
    if scale != 50:
        own(scale, "structure", {**rev("joint-chute-lift-foot", "chute-lift", "cast-iron", [[0, 0], [0, 0.04], [0.12, 0.04], [0.12, 0]], [0, 0, 1], [0, 0, -0.06], 0.3), "member": "chute-lift", "part": "joint"})
        own(scale, "structure", {**rev("joint-chute-lift-rod", "chute-lift-rod", "cast-iron", [[0, 0], [0, 0.04], [0.1, 0.04], [0.1, 0]], [0, 0, 1], [0, 0, -0.05], 0.3), "member": "chute-lift-rod", "part": "joint"})


for sc in (50, 100, 200, 500):
    lift(sc)

# ---------------------------------------------------------------- cab hardware

GLASS_X = lambda y: 4.429 + (2.46 - y) * 0.0908 + 0.006


def cab(scale):
    P = "paint-steel"
    parts = []
    if scale == 50:
        for side, pivot, tip in (("l", 0.72, 0.31), ("r", -0.23, -0.64)):
            parts.append({"id": f"wiper-arm-{side}", "kind": "capsule", "bone": "chassis", "material": "rubber", "role": "detail", "tone": 0.15, "p0": [GLASS_X(2.045) + 0.012, 2.045, pivot], "p1": [GLASS_X(2.11) + 0.02, 2.11, tip], "r0": 0.009, "r1": 0.008})
            parts.append({"id": f"wiper-blade-{side}", "kind": "capsule", "bone": "chassis", "material": "rubber", "role": "detail", "tone": 0.12, "p0": [GLASS_X(2.11) + 0.009, 2.11, tip - 0.29], "p1": [GLASS_X(2.11) + 0.009, 2.11, tip + 0.29], "r0": 0.009, "r1": 0.009})
    if scale in (50, 100):
        t = 0.03 if scale == 50 else 0.04
        for side, w in (("l", 0.94), ("r", -0.94)):
            parts.append(box(f"indicator-{side}", "chassis", "glass", [t, 0.1, 0.2], [4.478, 1.08, w], tone=0.8, cornerRadius=0.01))
        parts.append(box("number-plate", "chassis", P, [0.012 if scale == 50 else 0.016, 0.11, 0.52], [4.505, 0.6, 0], tone=0.92))
    dome = {50: [[0, 0], [0, 0.075], [0.025, 0.075], [0.03, 0.068], [0.1, 0.068], [0.125, 0.05], [0.138, 0.025], [0.14, 0]], 100: [[0, 0], [0, 0.075], [0.1, 0.07], [0.13, 0.04], [0.14, 0]], 200: [[0, 0], [0, 0.075], [0.11, 0.07], [0.14, 0]], 500: [[0, 0], [0, 0.075], [0.14, 0.075], [0.14, 0]]}[scale]
    for side, w in (("l", 0.5), ("r", -0.5)) if scale != 500 else ():
        parts.append(rev(f"beacon-{side}", "chassis", "glass", dome, [0, 1, 0], [3.2, 3.1, w], 0.75))
    if scale != 500:
        parts.append(box("roof-hatch", "chassis", P, [0.5, 0.03, 0.6], [3.62, 3.115, 0], tone=0.42, cornerRadius=0.012 if scale == 50 else 0))
    S.put(scale, parts, region="cab")


S.drop(500, ["beacon-l", "beacon-r"])
for sc in (50, 100, 200, 500):
    cab(sc)

# ---------------------------------------------------------------- leaf springs, 1:50


def leaf(x0):
    c = lambda dx: 0.035 * (dx / 0.74) ** 2
    top = [[x0 + dx, 0.665 + c(dx)] for dx in (-0.74, -0.5, -0.25, 0, 0.25, 0.5, 0.74)]
    half = [[0.74, 0.64 + c(0.74)], [0.56, 0.64 + c(0.56)], [0.56, 0.615 + c(0.56)], [0.38, 0.615 + c(0.38)], [0.38, 0.59 + c(0.38)], [0.2, 0.59 + c(0.2)], [0.2, 0.565]]
    bottom = [[x0 + dx, y] for dx, y in half] + [[x0 - dx, y] for dx, y in half[::-1]]
    return top + bottom


for n, x0 in ((1, 3.025), (2, 1.23)):
    for side, w in (("l", 0.34), ("r", -0.34)):
        S.put(50, [ext(f"spring-{n}-{side}", "chassis", "paint-steel", leaf(x0), "xy", w - 0.045, w + 0.045, tone=0.28, role="detail")])

# ---------------------------------------------------------------- wheels per scale

TYRE = {
    100: [[-0.128, 0.288], [-0.15, 0.33], [-0.1575, 0.41], [-0.152, 0.47], [-0.135, 0.508], [-0.118, 0.52], [0.118, 0.52], [0.135, 0.508], [0.152, 0.47], [0.1575, 0.41], [0.15, 0.33], [0.128, 0.288]],
    200: [[-0.13, 0.288], [-0.1575, 0.36], [-0.1575, 0.46], [-0.125, 0.52], [0.125, 0.52], [0.1575, 0.46], [0.1575, 0.36], [0.13, 0.288]],
    500: [[-0.1575, 0.29], [-0.1575, 0.45], [-0.1, 0.52], [0.1, 0.52], [0.1575, 0.45], [0.1575, 0.29]],
}
RIM = {
    100: [[-0.13, 0.275], [-0.13, 0.31], [-0.115, 0.31], [-0.115, 0.293], [0.115, 0.293], [0.115, 0.31], [0.13, 0.31], [0.13, 0.275]],
    200: [[-0.13, 0.26], [-0.13, 0.31], [0.13, 0.31], [0.13, 0.26]],
}
FACE = {
    200: {"front-disc": [[0.096, 0], [0.096, 0.27], [0.126, 0.27], [0.126, 0]], "front-hub": [[0.098, 0], [0.098, 0.14], [0.135, 0.14], [0.21, 0.09], [0.223, 0.06], [0.223, 0]], "outer-disc": [[-0.183, 0], [-0.183, 0.27], [-0.153, 0.27], [-0.153, 0]], "outer-hub": [[-0.181, 0], [-0.181, 0.15], [-0.145, 0.15], [0.03, 0.115], [0.054, 0.08], [0.054, 0]], "inner-disc": [[0.153, 0], [0.153, 0.27], [0.183, 0.27], [0.183, 0]]},
    500: {"front-disc": [[0.045, 0], [0.045, 0.29], [0.12, 0.29], [0.12, 0.14], [0.205, 0.09], [0.223, 0.06], [0.223, 0]], "outer-disc": [[-0.21, 0], [-0.21, 0.29], [-0.135, 0.29], [-0.135, 0.15], [0.03, 0.115], [0.054, 0.08], [0.054, 0]], "inner-disc": [[0.1, 0], [0.1, 0.29], [0.175, 0.29], [0.175, 0]]},
}
WHEEL_PREFIX = ("tyre-", "rim-", "wheel-disc-", "hub-", "wheel-nuts-")


def wheels(scale):
    vol = S.v[str(scale)]
    if scale == 100:
        for s in vol["body"]:
            if s["id"].startswith("tyre-"):
                s["profile"] = loop(copy.deepcopy(TYRE[100]))
            elif s["id"].startswith("rim-") and not s["id"].startswith("rim-dish-"):
                s["profile"] = loop(copy.deepcopy(RIM[100]))
            elif s["id"].startswith("wheel-disc-"):
                mid = {"1": 1.131, "2": 1.131}.get(s["id"].split("-")[2], 0.927 if s["id"].endswith("outer") else 0.913)
                sign = 1 if "-l" in s["id"][10:] else -1
                lo, hi = sorted((sign * (mid - 0.008), sign * (mid + 0.008)))
                s["from"], s["to"] = round(lo, 4), round(hi, 4)
        return
    S.drop(scale, [s["id"] for s in vol["body"] if s["id"].startswith(WHEEL_PREFIX)])
    face = FACE[scale]
    for n in (1, 2, 3, 4):
        bone = f"wheel-{n}"
        for side, sign in (("l", 1), ("r", -1)):
            axis = [0, 0, sign]
            sets = [("", 1.02, "front")] if n < 3 else [("-outer", 1.095, "outer"), ("-inner", 0.745, "inner")]
            for suffix, centre, kind in sets:
                origin = [0, 0, sign * centre]
                tyre = rev(f"tyre-{n}-{side}{suffix}", bone, "rubber", loop(copy.deepcopy(TYRE[scale])), axis, origin, 0.12, role="body")
                S.put(scale, [tyre], region="tyres")
                made = [tyre]
                if scale == 200:
                    rim = rev(f"rim-{n}-{side}{suffix}", bone, "paint-steel", loop(copy.deepcopy(RIM[200])), axis, origin, 0.6)
                    S.put(scale, [rim], region="running-gear")
                    made.append(rim)
                disc = rev(f"wheel-disc-{n}-{side}{suffix}", bone, "paint-steel", copy.deepcopy(face[kind + "-disc"]), axis, origin, 0.6)
                S.put(scale, [disc], region="running-gear")
                made.append(disc)
                if kind + "-hub" in face:
                    hub = rev(f"hub-{n}-{side}{suffix}", bone, "cast-iron", copy.deepcopy(face[kind + "-hub"]), axis, origin, 0.3)
                    S.put(scale, [hub], region="brakes")
                    made.append(hub)
                for m in made:
                    struct(scale, "wheel-member-" + m["id"], m["id"], bone)
    if scale == 500:
        for n, half in ((1, 0.92), (2, 0.92), (3, 0.66), (4, 0.66)):
            axle = S.get(500, f"axle-{n}")
            axle["profile"], axle["origin"] = [[0, 0.08], [2 * half, 0.08]], [0, 0, -half]


for sc in (100, 200, 500):
    wheels(sc)

# ---------------------------------------------------------------- decisions


def decide_once(scale, en, de):
    if not any(d["en"] == en for d in S.v[str(scale)]["decisions"]):
        S.decide(scale, en, de)


def undecide(scale, start):
    S.v[str(scale)]["decisions"] = [d for d in S.v[str(scale)]["decisions"] if not d["en"].startswith(start)]


decide_once(50, "The swivel chute is carried, as on the real machine, by a single-acting lift cylinder: two hanger cheeks drop from the swivel ring on the side away from the concrete flow to a foot pin 0.70 m below the chute hinge, the 70 mm barrel turns on that pin and its 36 mm rod on a lug pair under the trough 0.55 m from the hinge, each on a bone of its own so that both stay on the pin line while the chute is lowered. The cab carries two parked wipers, corner indicators, the number plate, two rotating beacons and the roof hatch, and the front axles hang on cambered five-leaf springs seated on the axle beams.", "Die Schwenkschurre wird wie an der wirklichen Maschine von einem einfach wirkenden Hubzylinder getragen: Zwei Hängelaschen führen vom Drehkranz auf der dem Betonstrom abgewandten Seite zu einem Fußbolzen 0,70 m unter dem Schurrengelenk, das 70-mm-Zylinderrohr dreht auf diesem Bolzen und seine 36-mm-Kolbenstange an einem Laschenpaar unter dem Trog 0,55 m vom Gelenk, jedes an einem eigenen Knochen, damit beide beim Absenken der Schurre auf der Bolzenlinie bleiben. Das Fahrerhaus trägt zwei geparkte Wischer, Eckblinker, das Kennzeichen, zwei Rundumleuchten und die Dachluke, und die Vorderachsen hängen an gesprengten Fünfblattfedern, die auf den Achskörpern sitzen.")
decide_once(100, "At 1:100 the wheels are modelled for a 15 mm feature: smooth tyre shoulders without the tread grooves, rims with 15 mm flanges, hand-hole discs 16 mm thick, hubs, nuts and brake drums. The chute lift cylinder is a hanger plate, barrel and rod on their own bones; the cab keeps indicators, plate, beacons and roof hatch, the wipers are left out.", "Bei 1:100 sind die Räder für 15 mm Detailgröße modelliert: glatte Reifenschultern ohne Profilrillen, Felgen mit 15-mm-Hörnern, 16 mm starke Radschüsseln mit Handlöchern, Naben, Muttern und Bremstrommeln. Der Schurren-Hubzylinder besteht aus Hängelasche, Zylinderrohr und Kolbenstange an eigenen Knochen; das Fahrerhaus behält Blinker, Kennzeichen, Rundumleuchten und Dachluke, die Wischer entfallen.")
undecide(200, "The steering axles carry singles")
decide_once(200, "At 1:200 every wheel is an eight-point tyre section on a plain rim band with a closed 30 mm disc and a turned hub; the driven axles keep separate inner and outer tyres with the 35 mm gap between them, the outer disc dished inwards. The chute lift cylinder stays as hanger plate, barrel and a 44 mm rod; the cab keeps beacons and roof hatch.", "Bei 1:200 ist jedes Rad ein Reifenquerschnitt aus acht Punkten auf glattem Felgenband mit geschlossener 30-mm-Schüssel und gedrehter Nabe; die Antriebsachsen behalten getrennte innere und äußere Reifen mit 35 mm Abstand, die äußere Schüssel nach innen gekröpft. Der Schurren-Hubzylinder bleibt als Hängelasche, Zylinderrohr und 44-mm-Kolbenstange; das Fahrerhaus behält Rundumleuchten und Dachluke.")
undecide(500, "The steering axles carry singles")
undecide(500, "At 1:500 every tyre")
decide_once(500, "At 1:500 every tyre is a six-point section and each wheel face one turned solid of a 75 mm disc with the hub boss; the inner twin tyres sit on a plain web disc, the axle tubes run through to the brake drums so that tyre, disc, drum and tube are one connected load path. The lift cylinder of the chute is an 80 mm strut of barrel and rod on their own bones under a hanger plate; wipers, indicators, plate, beacons and roof hatch are below the feature size and left out.", "Bei 1:500 ist jeder Reifen ein Querschnitt aus sechs Punkten und jede Radfront ein Drehkörper aus 75-mm-Schüssel mit Nabe; die inneren Zwillingsreifen sitzen auf einer glatten Scheibe, die Achsrohre laufen bis in die Bremstrommeln, sodass Reifen, Schüssel, Trommel und Rohr ein zusammenhängender Lastpfad sind. Der Hubzylinder der Schurre ist eine 80-mm-Strebe aus Zylinderrohr und Kolbenstange an eigenen Knochen unter einer Hängelasche; Wischer, Blinker, Kennzeichen, Rundumleuchten und Dachluke liegen unter der Detailgröße und entfallen.")

S.save()

# ---------------------------------------------------------------- catalog: rig, poses, clips, 2D

bones = A["rig"]["bones"]
if not any(b["id"] == "chute-lift" for b in bones):
    at = next(i for i, b in enumerate(bones) if b["id"] == "chute-ext") + 1
    bones[at:at] = [
        {"id": "chute-lift", "parent": "chute-swivel", "title": {"en": "Chute lift cylinder barrel", "de": "Zylinderrohr des Schurren-Hubzylinders"}, "x": LIFT_FOOT_U, "y": 0, "angleDeg": round(REST_ANGLE - 90, 3), "lengthM": 0.31, "joint": {"type": "revolute", "minDeg": -12, "maxDeg": 12}},
        {"id": "chute-lift-rod", "parent": "chute", "title": {"en": "Chute lift cylinder rod", "de": "Kolbenstange des Schurren-Hubzylinders"}, "x": ROD_PIN[0], "y": ROD_PIN[1], "angleDeg": round(REST_ANGLE + 180 - CHUTE_REST, 3), "lengthM": 0.33, "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 25}},
    ]
next(b for b in bones if b["id"] == "chute")["joint"].update({"minDeg": -8, "maxDeg": 15})
for pose in A["poses"]:
    d = pose.get("bones", {}).get("chute", {}).get("angleDeg")
    if d is not None:
        pose["bones"].update(lift_pose(d))
for clip in A["clips"]:
    for key in clip["keys"]:
        d = key.get("bones", {}).get("chute", {}).get("angleDeg")
        if d is not None:
            key["bones"].update(lift_pose(d))


def shape(sid, bone, z, tone, region, material, role="detail", **geometry):
    return {"id": sid, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, **geometry, "region": region, "material": material}


def poly(points):
    return {"kind": "poly", "points": points, "closed": True}


def revolved(bone):
    return {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}


def draw(scale, view, shapes):
    target = A["representations"][str(scale)]["geometry"][view]
    for s in shapes:
        at = next((i for i, t in enumerate(target) if t["id"] == s["id"]), None)
        if at is None:
            target.append(s)
        else:
            target[at] = s


for sc in (50, 100, 200):
    rod = {50: 0.018, 100: 0.018, 200: 0.022}[sc]
    for view, z in (("elevation", 3.8), ("plan", 3.8)):
        draw(sc, view, [
            shape("chute-lift-hanger", "chute-swivel", z - 0.02, 0.42, "chute", "galvanised-steel", **poly(HANGER if sc != 200 else HANGER_COARSE), solid={"kind": "extruded", "sweep": "w", "z0": -0.05, "z1": 0.05}),
            shape("chute-lift-rod", "chute-lift-rod", z, 0.7, "chute", "cast-iron", **poly([[-0.03, 0], [0.33, 0], [0.33, rod], [-0.03, rod]]), solid=revolved("chute-lift-rod")),
            shape("chute-lift-barrel", "chute-lift", z + 0.01, 0.35, "chute", "paint-steel", **poly([[-0.03, 0], [0.31, 0], [0.31, 0.035], [-0.03, 0.035]]), solid=revolved("chute-lift")),
        ])
    dome = [[3.125, 3.1], [3.275, 3.1], [3.268, 3.2], [3.25, 3.225], [3.2, 3.24], [3.15, 3.225], [3.132, 3.2]]
    draw(sc, "elevation", [shape("beacon-left", "chassis", 2.05, 0.75, "cab", "glass", **poly(dome)), shape("roof-hatch", "chassis", 2.04, 0.42, "cab", "paint-steel", **poly([[3.37, 3.1], [3.87, 3.1], [3.87, 3.13], [3.37, 3.13]]))])
    front = [shape(f"beacon-{name}", "chassis", 3.6, 0.75, "cab", "glass", **poly([[w - 0.075, 3.1], [w + 0.075, 3.1], [w + 0.068, 3.2], [w + 0.05, 3.225], [w, 3.24], [w - 0.05, 3.225], [w - 0.068, 3.2]])) for name, w in (("left", 0.5), ("right", -0.5))]
    plan = [shape(f"beacon-{name}", "chassis", 2.12, 0.75, "cab", "glass", kind="ellipse", cx=3.2, cy=w, rx=0.075, ry=0.075) for name, w in (("left", 0.5), ("right", -0.5))]
    plan.append(shape("roof-hatch", "chassis", 2.1, 0.42, "cab", "paint-steel", kind="rect", x=3.37, y=-0.3, w=0.5, h=0.6, r=0.012))
    if sc != 200:
        front.append(shape("roof-hatch", "chassis", 3.58, 0.42, "cab", "paint-steel", kind="rect", x=-0.3, y=3.1, w=0.6, h=0.03))
        front.append(shape("number-plate", "chassis", 3.7, 0.92, "cab", "paint-steel", kind="rect", x=-0.26, y=0.545, w=0.52, h=0.11))
        front += [shape(f"indicator-{name}", "chassis", 3.7, 0.8, "cab", "glass", kind="rect", x=w - 0.1, y=1.03, w=0.2, h=0.1) for name, w in (("left", 0.94), ("right", -0.94))]
    if sc == 50:
        front += [shape(f"wiper-blade-{name}", "chassis", 3.72, 0.12, "cab", "rubber", kind="rect", x=tip - 0.29, y=2.101, w=0.58, h=0.018) for name, tip in (("left", 0.31), ("right", -0.64))]
        front += [shape(f"wiper-arm-{name}", "chassis", 3.71, 0.15, "cab", "rubber", **poly([[pivot, 2.036], [pivot, 2.054], [tip, 2.118], [tip, 2.1]])) for name, pivot, tip in (("left", 0.72, 0.31), ("right", -0.23, -0.64))]
    draw(sc, "front", front)
    draw(sc, "plan", plan)


C.touch("rig", "poses", "clips", "representations")
print("[DEBUG] lift rest angle", round(REST_ANGLE, 3), "length", round(REST_LENGTH, 4), "8 deg", lift_world(8), "15", lift_world(15), "-8", lift_world(-8), lift_pose(8))
if MODE == "candidate":
    A["volumes"] = S.v
    C.touch("volumes")
    print(C.write(candidate=True))
else:
    print(C.write())
