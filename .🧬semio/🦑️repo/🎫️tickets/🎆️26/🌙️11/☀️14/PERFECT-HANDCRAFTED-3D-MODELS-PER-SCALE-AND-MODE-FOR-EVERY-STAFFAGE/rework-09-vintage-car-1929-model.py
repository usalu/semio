"""🚙 Author 09 rework of vehicles/vintage-car-1929: leaf springs on brackets at the coarse scales so both axles hang on the ladder frame, wheel bearings that start on the axle ends, engine bearers resting on the frame rails, a fan on its shaft, bench seats that stand on the floor and a steering column down to the steering box. Reference numbers: `rework/09.md`."""
import copy
from rework_09_lib import Source, cap, box, rev, of

S = Source("vehicles/vintage-car-1929")
STEEL = {"material": "exhaust-steel", "role": "detail", "tone": 0.25}
SEAT = {"material": "upholstery", "role": "detail", "tone": 0.3}

S.put(50, "body", cap("fan-shaft", "chassis", [1.56, 0.8, 0], [1.625, 0.8, 0], 0.02, material="engine-metal", role="detail", tone=0.3), after="fan")
for n in (1, 2):
    for side, sign in (("l", 1), ("r", -1)):
        bearer = S.get(50, "body", f"engine-bearer-{n}-{side}")
        bearer["transform"]["translation"][1] = 0.548

for scale, r_bearing in ((50, 0.03), (100, 0.03), (200, 0.035), (500, 0.04)):
    for axle, bone in (("f", "wheel-f"), ("r", "wheel-r")):
        for side, sign in (("l", 1), ("r", -1)):
            old = S.get(scale, "structure", f"wheel-bearing-{axle}{side}")
            fields = {k: old[k] for k in ("material", "role", "tone", "member", "part")}
            S.put(scale, "structure", rev(f"wheel-bearing-{axle}{side}", bone, [[0, 0], [0, r_bearing], [0.15, r_bearing], [0.15, 0]], [0, 0, sign], [0, 0, sign * 0.575], **fields))

for scale, half_w, t_eye, t_mid, r_bracket in ((200, 0.03, 0.016, 0.032, 0.02), (500, 0.04, 0.04, 0.045, 0.04)):
    for side, sign in (("l", 1), ("r", -1)):
        for end in ("front", "rear"):
            spring = copy.deepcopy(S.get(100, "body", f"spring-{end}-{side}"))
            for section, t in zip(spring["sections"], (t_eye, t_mid, t_eye)):
                section["rx"], section["ry"] = half_w, t
            S.put(scale, "body", spring)
            S.put(scale, "structure", of(f"st-spring-{end}-{side}", f"spring-{end}-{side}", member="chassis", part="member"))
        for id, p0, p1 in ((f"spring-hanger-{side}", [-0.95, 0.5, sign * 0.33], [-0.95, 0.46, sign * 0.47]), (f"shackle-rear-{side}", [-1.955, 0.555, sign * 0.33], [-1.96, 0.48, sign * 0.47])):
            S.put(scale, "body", cap(id, "chassis", p0, p1, r_bracket, **STEEL))
            S.put(scale, "structure", of(f"st-{id}", id, member="chassis", part="joint"))

for scale in (100, 200):
    for prefix, name, extra in (("", "body", {}), ("th-", "thermal", {"region": "interior"})):
        S.put(scale, name, box(f"{prefix}seat-front", "chassis", [0.5, 0.32, 1.1], [0, 0.71, 0], corner=0.04, **SEAT, **extra))
        S.put(scale, name, box(f"{prefix}seat-front-back", "chassis", [0.13, 0.6, 1.1], [-0.25, 1.08, 0], corner=0.04, rotation=[0, 0, 0.069756, 0.997564], **SEAT, **extra), after=f"{prefix}seat-front")
        S.put(scale, name, box(f"{prefix}seat-rear", "chassis", [0.56, 0.67, 1.1], [-1.42, 0.885, 0], corner=0.04, **SEAT, **extra))
for prefix, name, extra in (("", "body", {}), ("th-", "thermal", {"region": "interior"})):
    S.put(100, name, cap(f"{prefix}steering-column", "chassis", [0.24, 1.22, -0.3], [1.02, 0.539, -0.3], 0.015, **{**STEEL, "tone": 0.3}, **extra))
    for n, tip in ((1, [0.24, 1.22, -0.11]), (2, [0.24, 1.22, -0.49]), (3, [0.365, 1.363, -0.3]), (4, [0.115, 1.077, -0.3])):
        S.put(100, name, cap(f"{prefix}steering-spoke-{n}", "chassis", [0.24, 1.22, -0.3], tip, 0.0085, **{**STEEL, "tone": 0.3}, **extra), after=f"{prefix}steering-column")

S.decide(50, "The fan sits on a shaft from the block, the engine bearers rest on the frame rails and every wheel bearing starts on its axle end, so engine, axles and wheels hang on the ladder frame without a gap.", "Der Lüfter sitzt auf einer Welle am Motorblock, die Motorträger liegen auf den Rahmenlängsträgern auf, und jedes Radlager beginnt auf seinem Achsende, sodass Motor, Achsen und Räder lückenlos am Leiterrahmen hängen.")
S.decide(100, "The front bench is a cushion on the floor with a raked back, the rear bench reaches the seat pan, the steering column runs down to the steering box on the frame and carries four spokes; the wheel bearings start on the axle ends.", "Die vordere Bank ist ein Polster auf dem Boden mit geneigter Lehne, die hintere Bank reicht bis zur Sitzwanne, die Lenksäule läuft bis zum Lenkgetriebe am Rahmen und trägt vier Speichen; die Radlager beginnen auf den Achsenden.")
S.decide(200, "Semi-elliptic leaf springs, the rear pair outboard of the rails on a hanger and a shackle bracket, carry both axles on the ladder frame; the benches stand on the floor.", "Halbelliptische Blattfedern, das hintere Paar außerhalb der Längsträger an Federbock und Laschenkonsole, tragen beide Achsen am Leiterrahmen; die Sitzbänke stehen auf dem Boden.")
S.decide(500, "Four leaf springs drawn as 80 mm packs tie the two axle beams to the frame rails, the rear pair on outboard brackets, so the wheels no longer stand free under the body.", "Vier Blattfedern als 80-mm-Pakete binden die beiden Achsträger an die Rahmenlängsträger, das hintere Paar an außenliegenden Konsolen, sodass die Räder nicht mehr frei unter dem Aufbau stehen.")
S.save()
print("written")
