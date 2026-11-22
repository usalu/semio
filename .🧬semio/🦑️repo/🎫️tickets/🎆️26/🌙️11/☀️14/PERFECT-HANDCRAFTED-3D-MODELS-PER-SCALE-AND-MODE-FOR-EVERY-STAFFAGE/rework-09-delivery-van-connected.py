"""🚐 Author 09, second pass on vehicles/delivery-van for the product rules `volumes-connected` and `volumes-feature`: support pads under the tyre patches, brake hats seated on the hubs, the rear axle beam run into the brakes, tail pipe hung under the bumper, mirror and plate seated, headrest stems at 1:100, coarse-scale wheels and cage members drawn for their smallest feature. Run after `rework-09-delivery-van-model.py`."""
from rework_09_lib import Source, cap, rev, pads

S = Source("vehicles/delivery-van")
CONTACTS = [("contact-f-l", "axle-f", 0, -0.356, 0.84), ("contact-f-r", "axle-f", 0, -0.356, -0.84), ("contact-r-l", "axle-r", 0, -0.356, 0.84), ("contact-r-r", "axle-r", 0, -0.356, -0.84)]
WHEELS = (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1))
BIW = {"material": "paint", "role": "detail", "tone": 0.5, "member": "chassis", "part": "member"}

for scale in (50, 100, 200, 500):
    pads(S, scale, CONTACTS, 0.14, 0.235)
    beam = S.get(scale, "body", "rear-axle-beam")
    beam["p0"][2], beam["p1"][2] = -0.8, 0.8
    for w, bone, sign in WHEELS:
        hat = S.get(scale, "body", f"{w}-brake-hat")
        hat["profile"] = [[-0.075 if scale == 500 else -0.045, 0], [-0.075 if scale == 500 else -0.045, 0.09], [0.002, 0.082], [0.002, 0]]
    if scale != 500:
        pipe = S.get(scale, "body", "tail-pipe")
        pipe["p0"][1] = pipe["p1"][1] = 0.316
S.get(50, "body", "interior-mirror")["transform"]["translation"][1] = 1.773
S.get(50, "body", "plate-front")["transform"]["translation"][0] = 2.674
for name, z in (("driver", 0.5), ("bench-outer", -0.59), ("bench-inner", -0.15)):
    S.put(100, "body", cap(f"headrest-stem-{name}", "chassis", [0.397, 1.53, z], [0.385, 1.6, z], 0.0075, material="cast-iron", role="detail", tone=0.4), after=f"headrest-{name}")

for scale in (200, 500):
    for w, bone, sign in WHEELS:
        for name in ("body", "thermal", "structure"):
            S.v[str(scale)][name] = [s for s in S.v[str(scale)][name] if s["id"] != f"{w}-nut" and s.get("of") != f"{w}-nut"]
        if w[0] == "f":
            if scale == 200:
                S.get(200, "body", f"{w}-brake-disc")["profile"] = [[-0.061, 0.15], [-0.031, 0.15], [-0.031, 0.085], [-0.061, 0.085], [-0.061, 0.15]]
            else:
                for name in ("body", "thermal", "structure"):
                    S.v["500"][name] = [s for s in S.v["500"][name] if s["id"] != f"{w}-brake-disc" and s.get("of") != f"{w}-brake-disc"]
for w, bone, sign in WHEELS:
    old = S.get(500, "body", f"{w}-disc-wheel")
    S.put(500, "body", rev(f"{w}-disc-wheel", bone, [[0.0, 0.05], [0.0, 0.2], [0.075, 0.2], [0.075, 0.05], [0.0, 0.05]], [0, 0, sign], [0, 0, sign * 0.84], material=old["material"], role=old["role"], tone=old["tone"]))
for id in ("a-pillar-l", "a-pillar-r", "cant-rail-l", "cant-rail-r", "cowl-crossmember", "windscreen-header", "roof-bow-b", "roof-bow-2", "roof-bow-c", "roof-bow-4", "rear-header"):
    member = S.get(500, "structure", id)
    member["r0"] = member["r1"] = 0.038
for side, sign in (("l", 1), ("r", -1)):
    S.put(500, "structure", cap(f"outrigger-4-{side}", "chassis", [-2.5, 0.42, sign * 0.5], [-2.57, 0.52, sign * 0.925], 0.04, **BIW))
S.decide(500, "At 1:500 each wheel is a tyre on a rim with a 75 mm wheel disc, a hub and a brake drum-sized hat behind it; wheel nuts and the vented front rotor are below the smallest feature and left out. The rear axle beam runs into both brakes and a pad under every tyre patch is the support of the structure.", "Im Maßstab 1:500 ist jedes Rad ein Reifen auf einer Felge mit 75 mm starker Radscheibe, Nabe und einem Bremstopf dahinter; Radmuttern und die belüftete vordere Bremsscheibe liegen unter dem kleinsten Merkmal und entfallen. Der Hinterachskörper läuft bis in beide Bremsen, und eine Platte unter jeder Reifenaufstandsfläche ist das Auflager des Tragwerks.")
S.save()
print("written")
