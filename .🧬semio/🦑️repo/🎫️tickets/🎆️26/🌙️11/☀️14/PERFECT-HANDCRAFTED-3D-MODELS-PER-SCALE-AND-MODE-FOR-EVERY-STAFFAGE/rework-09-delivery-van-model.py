"""🚐 Author 09 rework of vehicles/delivery-van: outriggers that carry the body-side cage on the chassis rails, knuckles between lower arms, drive shafts and hubs, strut towers, rail kick-ups and a bumper beam at 1:200, and a steering wheel on its column. Reference numbers: `rework/09.md`."""
from rework_09_lib import Source, cap, box, ell

S = Source("vehicles/delivery-van")
BIW = {"material": "paint", "role": "detail", "tone": 0.5, "member": "chassis", "part": "member"}
IRON = {"material": "cast-iron", "role": "detail", "tone": 0.35}
TRIM = {"material": "plastic-trim", "role": "detail", "tone": 0.25}

for scale in (50, 100, 200):
    r_out = 0.035
    for side, sign in (("l", 1), ("r", -1)):
        z = lambda v: sign * v
        for n, x in ((1, 1.1), (2, 0.3), (3, -0.84)):
            S.put(scale, "structure", cap(f"outrigger-{n}-{side}", "chassis", [x, 0.36, z(0.5)], [x, 0.37, z(0.93)], r_out, **BIW))
        S.put(scale, "structure", cap(f"outrigger-4-{side}", "chassis", [-2.47, 0.37, z(0.5)], [-2.57, 0.5, z(0.925)], r_out, **BIW))
        if scale in (50, 100):
            S.put(scale, "structure", cap(f"axle-f-knuckle-{side}", "axle-f", [0, -0.085, z(0.72)], [-0.01, 0.1, z(0.72)], 0.025, material="cast-iron", role="detail", tone=0.5, member="axle-f", part="member"))
        if scale == 200:
            S.put(scale, "structure", cap(f"strut-tower-{side}", "chassis", [1.66, 0.52, z(0.52)], [1.64, 0.98, z(0.64)], 0.055, **BIW))
            S.put(scale, "structure", cap(f"front-rail-kick-{side}", "chassis", [1.3, 0.48, z(0.46)], [1.1, 0.38, z(0.5)], 0.045, **BIW))
            S.put(scale, "structure", cap(f"upper-rail-{side}", "chassis", [1.25, 1.05, z(0.92)], [1.66, 1.0, z(0.7)], 0.035, **BIW))
    if scale == 200:
        S.put(scale, "structure", box("bumper-beam-front", "chassis", [0.08, 0.12, 1.3], [2.58, 0.52, 0], **BIW))
        S.put(scale, "structure", box("bumper-beam-rear", "chassis", [0.08, 0.1, 1.5], [-2.58, 0.44, 0], **BIW))

for side, sign in (("l", 1), ("r", -1)):
    S.put(100, "body", ell(f"knuckle-f{side}", "axle-f", [0.07, 0.11, 0.05], [0, 0, sign * 0.745], **IRON), after="drive-shaft-r")
S.put(100, "body", cap("steering-column", "chassis", [1.02, 1.21, 0.5], [1.5, 0.835, 0.5], 0.02, **TRIM), after="steering-wheel")
for n, tip in ((1, [1.137, 1.36, 0.5]), (2, [0.9615, 1.135, 0.6645]), (3, [0.9615, 1.135, 0.3355])):
    S.put(100, "body", cap(f"steering-spoke-{n}", "chassis", [1.02, 1.21, 0.5], tip, 0.012, **TRIM), after="steering-column")

S.decide(50, "Outriggers under the hinge pillar, the B- and C-pillars and the rear corner carry the body-side cage on the chassis rails, and a knuckle on each side joins lower arm, stub axle and strut.", "Ausleger unter Scharniersäule, B- und C-Säule und der hinteren Ecke tragen den Seitenwandkäfig auf den Rahmenlängsträgern, und je ein Achsschenkel verbindet Querlenker, Achsstummel und Federbein.")
S.decide(100, "The steering wheel has three spokes and a column to the firewall, a knuckle joins each front drive shaft to its brake and hub, and outriggers tie the body-side cage to the chassis rails.", "Das Lenkrad hat drei Speichen und eine Lenksäule bis zur Stirnwand, je ein Achsschenkel verbindet Antriebswelle, Bremse und Nabe, und Ausleger binden den Seitenwandkäfig an die Rahmenlängsträger.")
S.decide(200, "Outriggers, rail kick-ups, strut towers under short upper rails and both bumper beams make underframe, front suspension and body-side cage one connected frame.", "Ausleger, Längsträgerkröpfungen, Federbeindome unter kurzen oberen Längsträgern und beide Stoßfängerträger machen Bodengruppe, Vorderachse und Seitenwandkäfig zu einem zusammenhängenden Rahmen.")
S.save()
print("written")
