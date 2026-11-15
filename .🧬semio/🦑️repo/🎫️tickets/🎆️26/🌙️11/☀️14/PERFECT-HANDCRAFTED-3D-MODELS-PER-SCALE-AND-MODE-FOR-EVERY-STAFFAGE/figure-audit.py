"""Anthropometric cross-check of the figure kit tables against the catalog's own 28 humans (rig bone lengths, BodyPhysiology areas)."""
import json, glob, os, statistics as st
R = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
rows = []
for f in sorted(glob.glob(R + "/*/🔣️.json")):
    a = json.load(open(f))["asset"]; slug = f.split("/")[-2]
    b = {x["id"]: x for x in a["rig"]["bones"]}
    L = lambda k: (b.get(k) or b.get(k + "-l") or {}).get("lengthM")
    H0 = a["dimensions"]["heightM"]; H = H0; M = a["dimensions"]["massKg"]
    if not all(L(k) for k in ["thigh", "shank", "torso", "head", "arm", "forearm", "hand", "foot"]) if False else False: pass
    d = {k: L(k) for k in ["pelvis", "torso", "neck", "head", "arm", "forearm", "hand", "thigh", "shank", "foot", "toe"]}
    if None in (d["thigh"], d["shank"], d["torso"], d["head"]): continue
    reg = a["thermal"]["regions"]
    body = [r for r in reg if r["model"] == "body"]
    skin = sum(r["areaM2"] for r in body if r["kind"] in ("skin", "hair")) + sum(r["areaM2"] for r in body if r["kind"] == "clothing")
    dub = 0.007184 * M ** 0.425 * (H * 100) ** 0.725
    S = 0.08 + sum(d[k] or 0 for k in ["shank","thigh","pelvis","torso","neck","head"])
    rows.append((slug, S, M, d, sum(r["areaM2"] for r in body if r["kind"] in ("skin","hair")) , dub))
names = ["thigh", "shank", "arm", "forearm", "hand", "foot", "torso", "neck", "head", "pelvis"]
print("n", len(rows))
print("rig stature S = 0.08 ankle + shank + thigh + pelvis + torso + neck + head")
ratio = {k: [] for k in names + ["leg", "L1", "mass"]}
for slug, H, M, d, sk, dub in rows:
    for k in names:
        if d[k]: ratio[k].append(d[k] / H)
    ratio["leg"].append((d["thigh"] + d["shank"]) / H)
print("%-9s %7s %7s %7s %7s | %s" % ("seg/H", "mean", "sd", "min", "max", "Winter/Drillis-Contini (kit table basis)"))
ref = {"thigh": .245, "shank": .246, "arm": .186, "forearm": .146, "hand": .108, "foot": .152, "head": .130, "torso": None, "leg": .491}
for k in names + ["leg"]:
    v = ratio[k]
    if v: print("%-9s %7.4f %7.4f %7.4f %7.4f | %s" % (k, st.mean(v), st.pstdev(v), min(v), max(v), ref.get(k)))
print("\nrig stature S against catalog heightM (upright only)")
for f in sorted(glob.glob(R + "/*/🔣️.json")):
    a = json.load(open(f))["asset"]; slug = f.split("/")[-2]
    r = [x for x in rows if x[0] == slug]
    if r and slug not in ("sleeping-person","kneeling-gardener","seated-guitarist","seated-office-worker","seated-reader","wheelchair-user","cyclist","tour-guide","person-with-umbrella"): print("  %-28s S %.3f  heightM %.3f  ratio %.3f" % (slug, r[0][1], a["dimensions"]["heightM"], r[0][1] / a["dimensions"]["heightM"]))
print("\nper asset H, M, thigh/H, shank/H, arm/H, forearm/H (outliers > 2 sd)")
for slug, H, M, d, sk, dub in rows:
    z = {k: (d[k] / H - st.mean(ratio[k])) / (st.pstdev(ratio[k]) or 1) for k in ["thigh", "shank", "arm", "forearm", "torso", "head"] if d[k]}
    out = [k for k, v in z.items() if abs(v) > 2]
    print("%-28s H %.2f M %5.1f  %s" % (slug, H, M, ("OUT " + ",".join(out)) if out else ""))
