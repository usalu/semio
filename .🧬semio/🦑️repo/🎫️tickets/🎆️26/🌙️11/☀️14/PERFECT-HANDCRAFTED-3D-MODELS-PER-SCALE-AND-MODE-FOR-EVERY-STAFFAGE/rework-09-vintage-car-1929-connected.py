"""🚙 Author 09, second pass on vehicles/vintage-car-1929 for the product rules `volumes-connected` and `volumes-feature`: support pads under the tyre patches, hubs that reach their brake drums and stub axles, engine ancillaries seated on the block, bearers on the rails, bumper clamps, lamp posts and spring shackles at 1:100, coarse-scale wheels and body frame drawn for their smallest feature. Run after `rework-09-vintage-car-1929-model.py`."""
from rework_09_lib import Source, cap, rev, of, pads, rename

S = Source("vehicles/vintage-car-1929")
CONTACTS = [("contact-f-l", "axle-f", 0, -0.37, 0.7), ("contact-f-r", "axle-f", 0, -0.37, -0.7), ("contact-r-l", "axle-r", 0, -0.37, 0.7), ("contact-r-r", "axle-r", 0, -0.37, -0.7)]
WHEELS = (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1))
STEEL = {"material": "exhaust-steel", "role": "detail", "tone": 0.25}
NICKEL = {"material": "nickel-plate", "role": "detail", "tone": 0.8}

for scale in (50, 100, 200, 500):
    pads(S, scale, CONTACTS, 0.1, 0.105)
    for w, bone, sign in WHEELS:
        S.get(scale, "body", f"hub-{w}")["profile"] = [[-0.055, 0], [-0.055, 0.05], [-0.005, 0.068], [0.03, 0.068], [0.038, 0.055], [0.038, 0]]

S.get(50, "body", "mirror")["origin"][1] = 1.557
dynamo = S.get(50, "body", "dynamo")
dynamo["p0"][2] = dynamo["p1"][2] = 0.152
manifold = S.get(50, "body", "inlet-manifold")
manifold["p0"][2] = manifold["p1"][2] = 0.13
for n in (1, 2):
    for side in ("l", "r"):
        bearer = S.get(50, "body", f"engine-bearer-{n}-{side}")
        bearer["size"][1] = 0.06
        bearer["transform"]["translation"][1] = 0.54

for w, bone, sign in WHEELS:
    for name in ("body", "thermal", "structure"):
        S.v["100"][name] = [s for s in S.v["100"][name] if s["id"] != f"valve-{w}" and s.get("of") != f"valve-{w}"]
for side, sign in (("l", 1), ("r", -1)):
    z = lambda v: sign * v
    S.put(100, "body", cap(f"lamp-post-{side}", "chassis", [1.83, 0.862, z(0.36)], [1.83, 0.5, z(0.31)], 0.0125, **NICKEL), after="lamp-bar")
    for scale, x, r in ((100, -2.208, 0.012), (200, -2.2, 0.015)):
        S.put(scale, "body", cap(f"bumper-clamp-{side}", "chassis", [x, 0.475, z(0.42)], [x, 0.565, z(0.42)], r, **NICKEL), after=f"bracket-rear-{side}")
    for id, p0, p1, r in ((f"shackle-front-eye-{side}", [2.07, 0.455, z(0.34)], [2.06, 0.495, z(0.31)], 0.0125), (f"shackle-front-{side}", [1.31, 0.405, z(0.34)], [1.3, 0.475, z(0.31)], 0.0125), (f"spring-hanger-{side}", [-0.95, 0.46, z(0.47)], [-0.95, 0.5, z(0.33)], 0.014), (f"shackle-rear-{side}", [-1.96, 0.48, z(0.47)], [-1.955, 0.555, z(0.33)], 0.0125)):
        S.put(100, "body", cap(id, "chassis", p0, p1, r, **STEEL))
        S.put(100, "structure", of(f"st-{id}", id, member="chassis", part="joint"))

for side, sign in (("l", 1), ("r", -1)):
    S.put(100, "body", cap(f"half-shaft-r{side}", "axle-r", [0, 0, sign * 0.57], [0, 0, sign * 0.66], 0.02, material="cast-iron", role="detail", tone=0.3), after="axle-tube")
for w, bone, sign in WHEELS:
    for section in S.get(200, "body", f"spokes-{w}")["sections"]:
        section["rx"], section["ry"] = max(section["rx"], 0.016), max(section["ry"], 0.016)
    if S.get(500, "body", f"spokes-{w}") is not None:
        rename(S, 500, f"spokes-{w}", f"wheel-disc-{w}", rev(f"wheel-disc-{w}", bone, [[-0.03, 0.06], [-0.03, 0.245], [0.045, 0.245], [0.045, 0.06], [-0.03, 0.06]], [0, 0, sign], [0, 0, sign * 0.7], material="steel-rim", role="detail", tone=0.7))
    for name in ("body", "thermal", "structure"):
        S.v["500"][name] = [s for s in S.v["500"][name] if s["id"] != f"hub-cap-{w}" and s.get("of") != f"hub-cap-{w}"]
    S.get(500, "body", f"drum-{w}")["profile"] = [[-0.125, 0], [-0.125, 0.145], [-0.12, 0.16], [-0.055, 0.16], [-0.05, 0.15], [-0.05, 0]]
for s in S.v["500"]["structure"]:
    if s.get("kind") == "box" and s["id"].split("-")[0] in ("body", "cant", "floor", "roof"):
        s["size"] = [max(c, 0.075) for c in s["size"]]
    if s.get("kind") == "loft" and s["id"].startswith("pillar-"):
        for section in s["sections"]:
            section["rx"] = section["ry"] = 0.038
S.decide(500, "At 1:500 each artillery wheel is a tyre on a rim with a 75 mm wheel disc, a hub that reaches the 75 mm brake drum and no hub cap; sills, pillars, cant rails, floor bearers and roof bows of the body frame are 75 mm members, and a pad under every tyre patch is the support of the structure.", "Im Maßstab 1:500 ist jedes Artillerierad ein Reifen auf einer Felge mit 75 mm starker Radscheibe, einer Nabe bis zur 75 mm breiten Bremstrommel und ohne Nabenkappe; Schweller, Säulen, Dachlängsholme, Bodenträger und Dachspriegel des Aufbaugerippes sind 75-mm-Stäbe, und eine Platte unter jeder Reifenaufstandsfläche ist das Auflager des Tragwerks.")
S.save()
print("written")
