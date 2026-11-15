"""Jogger 1:50 hand corrections on top of the kit fit (snapshot 🗑️generated/author-33/jogger.kit.json)."""
from author_33_lib import *
slug = "jogger"
snap = json.load(open(f"{T}/🗑️generated/author-33/jogger.kit.json"))
cur = load(slug)
v = copy.deepcopy(snap["volumes"]["50"])
b = v["body"]
for s in b:
    if s["id"] == "torso":
        for q in s["sections"]:
            q["exponent"] = 3.0
            q["rx"] *= 1.04
            q["ry"] *= 1.04
for side in "lr":
    sk, sh = by(b, f"thigh-skin-{side}"), by(b, f"thigh-{side}")
    for q in sk["sections"]:
        u = q["at"][0]
        if u <= 0.21:
            q["rx"] = min(q["rx"], 0.9 * interp(sh, max(u, 0), "rx"))
            q["ry"] = min(q["ry"], 0.9 * interp(sh, max(u, 0), "ry"))
b[:] = [s for s in b if s["id"] != "shirt-hem"]
p = by(b, "pelvis")
hem = [sec(u, interp(p, u, "rx") + 0.006, interp(p, u, "ry") + 0.006, 2.2) for u in (-0.09, -0.06, -0.03, 0.0, 0.03, 0.06, 0.085)]
hem.append(sec(0.108, hem[-1]["rx"] * 0.9, hem[-1]["ry"] * 0.9, 2.2))
b.append(loft("shirt-hem", "pelvis", "cotton-clothing", "detail", hem, 0.7))
cur["volumes"]["50"]["body"] = b
save(slug, cur)
print(bun("kit-thermal.ts", f"humans/{slug}", "--scale", "50", "--map", "torso-clothing=shoulder-l+shoulder-r", "--write")[:1500])
