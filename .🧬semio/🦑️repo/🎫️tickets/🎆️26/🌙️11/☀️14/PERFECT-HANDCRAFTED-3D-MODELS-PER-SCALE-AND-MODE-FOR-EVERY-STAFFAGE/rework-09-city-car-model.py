"""🚗 Author 09 rework of vehicles/city-car: seat backs that stand on their cushions, mirror feet and axles at the coarse scales, and a body in white whose members meet (torque boxes, floor rails, strut towers, upper rails, radiator supports, wheel-house arches, tail posts, C-pillar feet, bonnet hinge brackets, knuckles, struts, trailing arms, springs). Reference numbers: `rework/09.md`."""
from rework_09_lib import Source, cap, box, ell, loft, of, both

S = Source("vehicles/city-car")
TRIM = {"material": "interior-trim", "role": "detail", "tone": 0.3}
IRON = {"material": "cast-iron", "role": "detail", "tone": 0.35}
BIW = {"material": "paint", "role": "detail", "tone": 0.5, "member": "chassis", "part": "member"}


def st(bone, part="member"):
    return {"material": "cast-iron", "role": "detail", "tone": 0.5, "member": bone, "part": part}


FRONT_BACK = [(-0.15, 0.56, 0.05, 0.22), (-0.19, 0.76, 0.06, 0.245), (-0.27, 1.0, 0.058, 0.24), (-0.33, 1.14, 0.05, 0.225), (-0.36, 1.21, 0.04, 0.19)]
REAR_BACK = [(-0.94, 0.6, 0.05, 0.61), (-0.99, 0.8, 0.06, 0.63), (-1.08, 1.02, 0.055, 0.62), (-1.13, 1.14, 0.045, 0.6)]


def back(id, rows, z, n, extra):
    return loft(id, "chassis", [([x, y, z], t, w, n) for x, y, t, w in rows], **TRIM, **extra)


for prefix, name, extra in (("", "body", {}), ("th-", "thermal", {"region": "interior", "layer": "foam"})):
    for side, sign in (("l", 1), ("r", -1)):
        S.put(50, name, back(f"{prefix}seat-front-back-{side}", FRONT_BACK, sign * 0.36, 3, extra))
        S.put(50, name, ell(f"{prefix}seat-rear-headrest-{side}", "chassis", [0.045, 0.07, 0.11], [-1.155, 1.2, sign * 0.34], **TRIM, **extra))
    S.put(50, name, back(f"{prefix}seat-rear-back", REAR_BACK, 0, 4, extra))
for prefix, name, extra in (("", "body", {}), ("th-", "thermal", {"region": "interior"})):
    for side, sign in (("l", 1), ("r", -1)):
        S.put(100, name, back(f"{prefix}seat-front-{side}", FRONT_BACK[:3] + [(-0.33, 1.13, 0.052, 0.225), (-0.365, 1.22, 0.045, 0.12), (-0.41, 1.36, 0.045, 0.11)], sign * 0.36, 3, extra))
    S.put(100, name, back(f"{prefix}seat-rear", REAR_BACK[:3] + [(-1.15, 1.2, 0.045, 0.58)], 0, 4, extra))
    S.put(100, name, cap(f"{prefix}steering-column", "chassis", [0.378, 0.992, 0.36], [0.632, 0.873, 0.36], 0.03, 0.022, **TRIM, **extra), after=f"{prefix}steering-wheel")

for prefix, name, extra in (("", "body", {}), ("th-", "thermal", {"region": "interior"})):
    for n, tip in ((1, [0.43, 1.15, 0.36]), (2, [0.325, 0.925, 0.217]), (3, [0.325, 0.925, 0.503])):
        S.put(100, name, cap(f"{prefix}steering-spoke-{n}", "chassis", [0.36, 1, 0.36], tip, 0.012, **TRIM, **extra), after=f"{prefix}steering-column")
for side, sign in (("l", 1), ("r", -1)):
    S.put(100, "body", ell(f"knuckle-f{side}", "axle-f", [0.06, 0.095, 0.035], [0, 0, sign * 0.645], **IRON), after="lower-arm-r")
    S.put(500, "body", cap(f"mirror-foot-{side}", "chassis", [0.775, 1.0, sign * 0.70], [0.768, 1.07, sign * 0.80], 0.04, material="plastic-trim", role="detail", tone=0.3), after=f"mirror-{side}")
S.put(200, "body", cap("axle-beam-front", "axle-f", [0, 0, -0.7], [0, 0, 0.7], 0.03, **IRON))
S.put(500, "body", cap("axle-beam-front", "axle-f", [0, 0, -0.7], [0, 0, 0.7], 0.04, **IRON))
S.put(500, "body", cap("axle-beam-rear", "axle-r", [0, 0, -0.7], [0, 0, 0.7], 0.04, **IRON))

for scale, fine in ((50, True), (100, True), (200, False), (500, False)):
    rr = {50: 1.0, 100: 1.0, 200: 1.0, 500: 1.3}[scale]
    for side, sign in (("l", 1), ("r", -1)):
        z = lambda v: sign * v
        S.put(scale, "structure", cap(f"c-pillar-foot-{side}", "chassis", [-1.0, 0.72, z(0.74)], [-0.89, 0.3, z(0.745)], 0.04, **BIW), after=f"c-pillar-{side}")
        S.put(scale, "structure", cap(f"tail-post-{side}", "chassis", [-1.74, 0.9, z(0.7)], [-1.85, 0.53, z(0.56)], 0.035 * rr, **BIW), after=f"d-pillar-{side}")
        if scale != 500:
            S.put(scale, "structure", cap(f"wheelhouse-rear-{side}", "chassis", [-1.0, 0.72, z(0.72)], [-1.33, 0.8, z(0.7)], 0.03, **BIW), after=f"tail-post-{side}")
            S.put(scale, "structure", cap(f"wheelhouse-rear-aft-{side}", "chassis", [-1.33, 0.8, z(0.7)], [-1.74, 0.88, z(0.7)], 0.03, **BIW), after=f"wheelhouse-rear-{side}")
        S.put(scale, "structure", cap(f"upper-rail-{side}", "chassis", [0.8, 0.9, z(0.72)], [1.6, 0.74, z(0.66)], 0.03 * rr, **BIW), after=f"a-pillar-{side}")
        S.put(scale, "structure", cap(f"radiator-support-{side}", "chassis", [1.6, 0.74, z(0.66)], [1.83, 0.5, z(0.46)], 0.025 * rr * (1.2 if scale >= 200 else 1), **BIW), after=f"upper-rail-{side}")
        S.put(scale, "structure", cap(f"bonnet-hinge-bracket-{side}", "chassis", [0.82, 0.98, z(0.6 if scale != 500 else 0.58)], [0.955, 0.962, z(0.6 if scale != 500 else 0.58)], {50: 0.018, 100: 0.018, 200: 0.02, 500: 0.038}[scale], **BIW), after=f"radiator-support-{side}")
        if scale != 500:
            S.put(scale, "structure", cap(f"strut-tower-{side}", "chassis", [1.11, 0.47, z(0.47)], [1.11, 0.74, z(0.585)], 0.05, **BIW), after=f"upper-rail-{side}")
            S.put(scale, "structure", cap(f"torque-box-{side}", "chassis", [0.62, 0.28, z(0.5)], [0.76, 0.3, z(0.72)], 0.04, **BIW), after=f"hinge-pillar-{side}")
        if scale in (100, 200):
            S.put(scale, "structure", box(f"floor-rail-{side}", "chassis", [1.5, 0.06, 0.08], [-0.13, 0.25, z(0.5)], **BIW), after=f"torque-box-{side}")
        if scale in (50, 100):
            S.put(scale, "structure", of(f"st-knuckle-f{side}", f"knuckle-f{side}", member="axle-f", part="member"))
        if scale == 200:
            S.put(scale, "structure", cap(f"strut-{side}", "axle-f", [0, 0, z(0.62)], [-0.04, 0.42, z(0.585)], 0.03, **st("axle-f")))
            S.put(scale, "structure", cap(f"spring-{side}", "axle-r", [0.2, 0, z(0.5)], [0.12, 0.2, z(0.5)], 0.04, **st("axle-r")))
        if scale == 500:
            S.put(scale, "structure", cap(f"strut-{side}", "axle-f", [0, 0, z(0.62)], [-0.04, 0.52, z(0.66)], 0.04, **st("axle-f")))
            S.put(scale, "structure", cap(f"trailing-arm-{side}", "axle-r", [0, 0, z(0.6)], [0.42, 0.02, z(0.6)], 0.04, **st("axle-r")))
            S.put(scale, "structure", cap(f"spring-{side}", "axle-r", [0.06, 0, z(0.5)], [0.06, 0.2, z(0.5)], 0.045, **st("axle-r")))
for scale, extra in ((50, {"region": "interior", "layer": "foam"}), (100, {"region": "interior"})):
    for side, sign in (("l", 1), ("r", -1)):
        S.put(scale, "thermal", box(f"th-seat-front-base-{side}", "chassis", [0.3, 0.15, 0.3], [0.06, 0.4, sign * 0.36], **TRIM, **extra), after=f"th-seat-front-cushion-{side}")
    S.drop(scale, "thermal", "th-seat-rear-base")
wheel = S.get(50, "thermal", "th-steering-wheel")
extra = {k: wheel[k] for k in ("region", "layer") if k in wheel}
S.put(50, "thermal", cap("th-steering-column", "chassis", [0.378, 0.992, 0.36], [0.632, 0.873, 0.36], 0.03, 0.022, **TRIM, **extra), after="th-steering-wheel")
for n, tip in ((1, [0.43, 1.15, 0.36]), (2, [0.325, 0.925, 0.217]), (3, [0.325, 0.925, 0.503])):
    S.put(50, "thermal", cap(f"th-steering-spoke-{n}", "chassis", [0.36, 1, 0.36], tip, 0.012, **TRIM, **extra), after="th-steering-column")
S.put(200, "thermal", box("th-front-seat-bases", "chassis", [0.38, 0.15, 1.1], [0.06, 0.4, 0], **TRIM, region="interior"), after="th-front-cushions")
S.put(500, "structure", box("heel-kick", "chassis", [0.1, 0.1, 1.5], [-0.86, 0.32, 0], corner=0.02, **BIW))
S.put(500, "structure", box("bonnet-frame-front", "bonnet", [0.08, 0.076, 1.24], [0.88, -0.02, 0], corner=0.02, **st("bonnet")), after="bonnet-frame-r")
S.put(500, "structure", box("bonnet-frame-rear", "bonnet", [0.08, 0.076, 1.24], [0.05, -0.02, 0], corner=0.02, **st("bonnet")), after="bonnet-frame-front")

S.decide(50, "Seat backs rise from the recliner at the rear edge of their cushions and the rear headrests sit on the bench back; in the structure the wheel-house arch runs from the C-pillar foot to the tail post, the upper rails end on radiator supports, the bonnet frame hangs on hinge brackets of the cowl and the knuckles join lower arms, struts and stub axles.", "Die Sitzlehnen steigen vom Lehnengelenk an der Hinterkante ihrer Polster auf, die hinteren Kopfstützen sitzen auf der Banklehne; im Tragwerk läuft der Radhausbogen vom Fuß der C-Säule zum Heckpfosten, die oberen Längsträger enden auf den Kühlerträgern, der Haubenrahmen hängt an Scharnierkonsolen des Windlaufs, und die Achsschenkel verbinden Querlenker, Federbeine und Achsstummel.")
S.decide(100, "Seat backs stand on their cushions, the steering wheel sits on its column and knuckles join the front lower arms to the drive shafts; the body in white has torque boxes, floor rails, strut towers, upper rails, radiator supports, wheel-house arches, tail posts and C-pillar feet, so no member ends in the air.", "Die Sitzlehnen stehen auf ihren Polstern, das Lenkrad sitzt auf seiner Lenksäule, und Achsschenkel verbinden die vorderen Querlenker mit den Antriebswellen; die Rohkarosserie hat Drehmomentkästen, Bodenlängsträger, Federbeindome, obere Längsträger, Kühlerträger, Radhausbögen, Heckpfosten und C-Säulenfüße, sodass kein Stab frei endet.")
S.decide(200, "The front axle reaches both hubs; in the structure struts, rear springs, strut towers, upper rails, torque boxes, floor rails, wheel-house arches and tail posts tie axles, underframe and cabin cage into one connected frame.", "Die Vorderachse reicht bis in beide Naben; im Tragwerk verbinden Federbeine, hintere Federn, Federbeindome, obere Längsträger, Drehmomentkästen, Bodenlängsträger, Radhausbögen und Heckpfosten Achsen, Bodengruppe und Kabinenkäfig zu einem zusammenhängenden Rahmen.")
S.decide(500, "Both axles are solid beams from hub to hub under the body and each door mirror stands on a foot at the belt in front of the door glass; in the structure struts, trailing arms, springs, upper rails with radiator supports, C-pillar feet, tail posts and a full-width heel-kick member join wheels, underframe and cage, and the bonnet frame is a closed rectangle on hinge brackets.", "Beide Achsen sind durchgehende Träger von Nabe zu Nabe unter dem Aufbau, und jeder Außenspiegel steht auf einem Fuß an der Brüstung vor der Türscheibe; im Tragwerk verbinden Federbeine, Längslenker, Federn, obere Längsträger mit Kühlerträgern, C-Säulenfüße, Heckpfosten und ein durchgehender Fersenblechträger Räder, Bodengruppe und Käfig, und der Haubenrahmen ist ein geschlossenes Rechteck an Scharnierkonsolen.")
S.save()
print("written", S.path)
