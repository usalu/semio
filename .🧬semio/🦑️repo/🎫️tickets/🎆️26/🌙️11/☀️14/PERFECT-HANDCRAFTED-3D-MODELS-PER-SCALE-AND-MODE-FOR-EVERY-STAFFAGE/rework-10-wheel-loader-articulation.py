"""🚜 Wheel loader: the articulation joint becomes a real joint (it was fixed and only drawn as a plan symbol) and the two steering cylinders become barrel and rod bones that span their pins when the frames articulate

`--with-pose` adds the pose `steering-left` (35 degrees): the product then demands every 2D shape of the front half as a `solid` (`lateral-ambiguous`, `elevation-lateral`), which is not drawn yet, so the catalog gets the rig without the pose; the pose was used on the candidate to prove that both rods close on their pins (0.0 mm).

Usage: python3 rework-10-wheel-loader-articulation.py [--candidate] [--with-pose]
"""
import sys, copy
exec(open('rework-10-lib.py').read())
CAND = '--candidate' in sys.argv
c, s = Catalog('wheel-loader'), Source('wheel-loader')
Z, REAR_PIN, FRONT_PIN, ROD0, ROD_L, Y, BARREL = 0.56, -1.05, 0.42, 0.57, 0.9, 1.0, 0.95


def steer(theta_deg, side):
    """📐 Plan geometry of one steering cylinder at articulation theta (positive = front frame to the left): abduction of the barrel and rod travel."""
    t = math.radians(theta_deg)
    z = side * Z
    fx, fz = FRONT_PIN * math.cos(t) - z * math.sin(t), FRONT_PIN * math.sin(t) + z * math.cos(t)
    dx, dz = fx - REAR_PIN, fz - z
    return round(math.degrees(math.atan2(dz, dx)), 3), round(math.hypot(dx, dz) - (FRONT_PIN - REAR_PIN), 4)


bones = c.asset['rig']['bones']
if not any(b['id'] == 'articulation' for b in bones):
    bones.insert(1, {"id": "articulation", "parent": "rear-frame", "title": {"en": "Articulation joint (vertical axis through the two hitch pins)", "de": "Knickgelenk (senkrechte Achse durch die beiden Gelenkbolzen)"}, "x": 0.0, "y": 0.59, "angleDeg": 90.0, "lengthM": 0.9, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -40, "twistMaxDeg": 40}})
    front = next(b for b in bones if b['id'] == 'front-frame')
    front.update(parent='articulation', x=-0.59, y=0.0, angleDeg=-90.0)
    lo = min(steer(40, 1)[1], steer(-40, 1)[1])
    hi = max(steer(40, 1)[1], steer(-40, 1)[1])
    for side, tag, en, de in ((1, 'l', 'left', 'links'), (-1, 'r', 'right', 'rechts')):
        bones.append({"id": f"steer-cyl-{tag}", "parent": "rear-frame", "title": {"en": f"Steering cylinder {en}, barrel", "de": f"Lenkzylinder {de}, Zylinderrohr"}, "x": REAR_PIN, "y": Y, "z": side * Z, "angleDeg": 0.0, "lengthM": BARREL, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0}, "mass": {"kg": 45, "comX": 0.47, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
        bones.append({"id": f"steer-rod-{tag}", "parent": f"steer-cyl-{tag}", "title": {"en": f"Steering cylinder {en}, rod", "de": f"Lenkzylinder {de}, Kolbenstange"}, "x": ROD0, "y": 0.0, "angleDeg": 0.0, "lengthM": ROD_L, "joint": {"type": "prismatic", "axisDeg": 0, "minM": round(lo - 0.01, 3), "maxM": round(hi + 0.01, 3)}, "mass": {"kg": 25, "comX": 0.45, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    rear = next(b for b in bones if b['id'] == 'rear-frame')
    rear['mass']['kg'] -= 140

poses = c.asset['poses']
if '--with-pose' in sys.argv and not any(p['id'] == 'steering-left' for p in poses):
    pose = copy.deepcopy(next(p for p in poses if p['id'] == 'carry'))
    pose.update(id='steering-left', title={"en": "Carry position, steering 35° to the left", "de": "Transportstellung, 35° nach links eingelenkt"}, description={"en": "Travelling with a full bucket held low while the front frame is articulated 35° to the left about the hitch pins: the left steering cylinder is retracted, the right one extended.", "de": "Fahrt mit voller, tief gehaltener Schaufel, der Vorderwagen ist um die Gelenkbolzen 35° nach links eingeknickt: der linke Lenkzylinder ist eingefahren, der rechte ausgefahren."})
    pose['bones']['articulation'] = {"twistDeg": 35}
    for side, tag in ((1, 'l'), (-1, 'r')):
        a, d = steer(35, side)
        pose['bones'][f'steer-cyl-{tag}'] = {"abductionDeg": a}
        pose['bones'][f'steer-rod-{tag}'] = {"dx": d}
    poses.insert(1, pose)

for rep in c.asset['representations'].values():
    for view, shapes in rep.get('geometry', {}).items():
        for sh in shapes:
            base = sh['id'].replace('-edge', '')
            if base not in ('steer-cyl-l', 'steer-cyl-r', 'steer-rod-l', 'steer-rod-r') or sh['bone'] == base:
                continue
            side = 1 if base.endswith('-l') else -1
            rod = 'rod' in base
            sh['bone'] = base
            if view == 'elevation':
                h = 0.03 if rod else 0.055
                length = ROD_L if rod else BARREL
                sh['points'] = [[0, 0], [0, h], [length, h], [length, 0], [length, -h], [0, -h]]
            elif view == 'front':
                sh['cx'] = side * Z
            else:
                h = 0.03 if rod else 0.055
                x0, x1 = (REAR_PIN + ROD0, FRONT_PIN) if rod else (REAR_PIN, REAR_PIN + BARREL)
                z = side * Z
                sh['points'] = [[x0, z], [x0, z + h], [x1, z + h], [x1, z], [x1, z - h], [x0, z - h]]

c.asset['summary']['en'] = c.asset['summary']['en'].replace("Steering articulation is shown in a plan symbol.", "The frames steer about the articulation joint (40° each way) with two steering cylinders.")
c.asset['summary']['de'] = c.asset['summary']['de'].replace("Die Knicklenkung zeigt ein Grundrisszeichen.", "Die Rahmen lenken um das Knickgelenk (40° je Seite) mit zwei Lenkzylindern.")

PS = 'paint-steel'
for scale in (50, 100):
    for side, tag in ((1, 'l'), (-1, 'r')):
        s.put(scale, [
            cyl(f'steer-cyl-{tag}', f'steer-cyl-{tag}', PS, [0, 0, 0], [1, 0, 0], 0.055, BARREL, tone=0.5),
            cyl(f'steer-rod-{tag}', f'steer-rod-{tag}', 'chrome', [0, 0, 0], [1, 0, 0], 0.03, ROD_L, tone=0.8),
        ])
        s.put(scale, [
            cyl(f'steer-cyl-eye-{tag}', f'steer-cyl-{tag}', PS, [0, 0, -0.035], [0, 0, 1], 0.06, 0.07, tone=0.45),
            cyl(f'steer-rod-eye-{tag}', f'steer-rod-{tag}', PS, [ROD_L, 0, -0.035], [0, 0, 1], 0.05, 0.07, tone=0.45),
        ], region='hydraulics')
        s.put(scale, [
            box(f'steer-lug-rear-{tag}', 'rear-frame', PS, [0.16, 0.13, 0.065], [REAR_PIN, Y, side * 0.4925], tone=0.45, role='body'),
            box(f'steer-lug-front-{tag}', 'front-frame', PS, [0.16, 0.13, 0.085], [FRONT_PIN, Y, side * 0.4825], tone=0.45, role='body'),
        ], region='front-frame')
for scale, rod_r in ((200, 0.03), (500, 0.0375)):
    for side, tag in ((1, 'l'), (-1, 'r')):
        s.put(scale, [
            cyl(f'steer-cyl-{tag}', f'steer-cyl-{tag}', PS, [0, 0, 0], [1, 0, 0], 0.055, BARREL, tone=0.5),
            cyl(f'steer-rod-{tag}', f'steer-rod-{tag}', 'chrome', [0, 0, 0], [1, 0, 0], rod_r, ROD_L, tone=0.8),
        ], region='hydraulics')
for scale in (50, 100, 200, 500):
    members = [dict(cyl('member-articulation', 'articulation', PS, [0, 0, 0], [1, 0, 0], 0.055, 0.9), member='articulation', part='member')]
    members += [{"id": f"member-{part}-{tag}", "of": f"{part}-{tag}", "member": f"{part}-{tag}", "part": "member"} for tag in ('l', 'r') for part in ('steer-cyl', 'steer-rod')]
    s.put(scale, members, lst='structure')
    s.put(scale, [dict(cyl('joint-articulation', 'articulation', PS, [0.3, 0, 0], [1, 0, 0], 0.09, 0.3), member='articulation', part='joint')], lst='structure')
stack = s.get(100, 'exhaust-stack')
stack['profile'] = [[a, 0.05 if abs(r - 0.057) < 1e-9 else r] for a, r in stack['profile']]
s.decide(50, "Rework 10: the steering cylinders sit on their own barrel and rod bones 0.56 m off the centre line (clear of the rear frame rails they used to cut through), with cap and rod eyes and lugs on both frames; the articulation is a real joint.", "Überarbeitung 10: die Lenkzylinder sitzen auf eigenen Rohr- und Stangenknochen 0,56 m neben der Mittellinie (frei von den Hinterrahmenholmen, die sie bisher durchdrangen), mit Boden- und Stangenaugen und Laschen an beiden Rahmen; das Knickgelenk ist ein echtes Gelenk.")
for scale in (200, 500):
    s.decide(scale, "Rework 10: the two steering cylinders (110 mm barrels) are modelled on their own bones so that the articulation joint works at this scale too.", "Überarbeitung 10: die beiden Lenkzylinder (110-mm-Rohre) sind auf eigenen Knochen modelliert, damit das Knickgelenk auch in diesem Maßstab arbeitet.")
s.decide(100, "Rework 10: steering cylinders, eyes and lugs as at 1:50 on the articulation rig; the exhaust stack wall is 15 mm for this scale.", "Überarbeitung 10: Lenkzylinder, Augen und Laschen wie bei 1:50 am Knickgelenk-Skelett; die Auspuffrohrwand ist für diesen Maßstab 15 mm dick.")

c.touch('rig', 'poses', 'representations', 'summary')
if CAND:
    c.asset['volumes'] = s.v
    c.text = c.text  # volumes are embedded through the member writer below
    c.touch('volumes')
    print(c.write(candidate=True))
    print('35 deg:', steer(35, 1), steer(35, -1), '40 deg:', steer(40, 1), steer(-40, 1))
else:
    s.save()
    print(c.write())
