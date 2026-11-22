"""🚗 Author 09, second pass on vehicles/city-car for the product rules `volumes-connected` and `volumes-feature`: support pads under the four tyre patches, reflectors seated in the bumper, rear stub axles at 1:100, and coarse-scale wheels and cage members drawn for their smallest feature (wheel disc instead of spokes at 1:500). Run after `rework-09-city-car-model.py`."""
from rework_09_lib import Source, cap, box, rev, pads, rename

S = Source("vehicles/city-car")
CONTACTS = [("contact-f-l", "axle-f", 0, -0.3, 0.72), ("contact-f-r", "axle-f", 0, -0.3, -0.72), ("contact-r-l", "axle-r", 0, -0.3, 0.72), ("contact-r-r", "axle-r", 0, -0.3, -0.72)]
for scale in (50, 100, 200, 500):
    pads(S, scale, CONTACTS, 0.12, 0.185)

for side in ("l", "r"):
    S.get(50, "body", f"reflector-rear-{side}")["transform"]["translation"][0] = -1.957
for side, sign in (("l", 1), ("r", -1)):
    S.put(100, "body", cap(f"stub-axle-r{side}", "axle-r", [0, 0, sign * 0.6], [0, 0, sign * 0.745], 0.022, material="cast-iron", role="detail", tone=0.35), after="trailing-arm-r")

WHEELS = (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1))
for w, bone, sign in WHEELS:
    spoke = S.get(200, "body", f"{w}-spoke")
    for section in spoke["sections"]:
        section["rx"], section["ry"] = max(section["rx"], 0.016), max(section["ry"], 0.016)
    if w[0] == "f":
        S.get(200, "body", f"{w}-disc")["profile"] = [[-0.02, 0.15], [0.01, 0.15], [0.01, 0.08], [-0.02, 0.08], [-0.02, 0.15]]
        S.get(500, "body", f"{w}-disc")["profile"] = [[-0.012, 0.15], [0, 0.15], [0, 0.075], [-0.012, 0.075], [-0.012, 0.15]]
    S.get(500, "body", f"{w}-hub")["profile"] = [[0.0, 0], [0.0, 0.062], [0.064, 0.062], [0.074, 0.045], [0.078, 0]]
    S.get(500, "body", f"{w}-disc-hat")["profile"] = [[-0.045, 0], [-0.045, 0.082], [0.03, 0.074], [0.03, 0]]
    if S.get(500, "body", f"{w}-spoke") is not None:
        rename(S, 500, f"{w}-spoke", f"{w}-wheel-disc", rev(f"{w}-wheel-disc", bone, [[0.0, 0.055], [0.0, 0.19], [0.075, 0.19], [0.075, 0.055], [0.0, 0.055]], [0, 0, sign], [0, 0, sign * 0.72], material="alloy", role="detail", tone=0.8))
for id in ("a-pillar-l", "a-pillar-r", "roof-rail-l", "roof-rail-r", "cowl-crossmember", "windscreen-header", "roof-bow-b", "roof-bow-c", "rear-header"):
    member = S.get(500, "structure", id)
    member["r0"] = member["r1"] = 0.038
S.v["500"]["decisions"][0] = {"en": "At 1:500 each wheel is a tyre on a rim with a 75 mm wheel disc, a hub and the brake behind it; spokes are below the smallest feature and left out. A continuous steel roof rail ties the four pillars on each side to the headers.", "de": "Im Maßstab 1:500 ist jedes Rad ein Reifen auf einer Felge mit 75 mm starker Radscheibe, Nabe und der Bremse dahinter; Speichen liegen unter dem kleinsten Merkmal und entfallen. Durchgehende Dachholme verbinden die vier Säulen jeder Seite mit den Dachquerträgern."}
S.save()
print("written")
