"""Kneeling-gardener 1:50: repair of the 07 seed (mouth, clog split into foot and toe lofts), thermal, structure."""
from author_33_lib import *
slug = "kneeling-gardener"
orig = json.load(open(f"{T}/🗑️generated/author-33/gardener.orig07.json"))
cur = load(slug)
v = copy.deepcopy(orig["volumes"]["50"])
b = v["body"]
for s in b:
    if s["id"] == "mouth": s["r0"] = s["r1"] = 0.004
CUT = 0.15
for sd in "lr":
    c = by(b, f"clog-{sd}")
    secs = [q for q in c["sections"] if q["at"][0] < CUT]
    last = copy.deepcopy(secs[-1]); last["at"][0] = CUT
    for k in ("rx", "ry"): last[k] = interp(c, CUT, k)
    c["sections"] = secs + [last]
for q in by(b, "shirt")["sections"]:
    q["exponent"] = 3.0; q["rx"] *= 1.06; q["ry"] *= 1.05
p = by(b, "pelvis")
hem = [sec(u, interp(p, u, "rx") + 0.003, interp(p, u, "ry") + 0.003, 2.4) for u in (-0.065, -0.03, 0.0, 0.03, 0.06, 0.085)]
hem.append(sec(0.094, hem[-1]["rx"] * 0.9, hem[-1]["ry"] * 0.9, 2.4))
b.append(loft("shirt-hem", "pelvis", "linen", "detail", hem, 0.7))
by(b, "trug")["profile"] = [[0, 0], [0, 0.215], [0.21, 0.25], [0.21, 0.235], [0.02, 0.195], [0.02, 0]]
cur["volumes"]["50"]["body"] = b
cur["volumes"]["50"]["thermal"] = []
cur["volumes"]["50"]["structure"] = []
save(slug, cur)
d = load(slug); b = d["volumes"]["50"]["body"]
for sd in "lr":
    t = hull(slug, 50, f"toe-{sd}", f"clog-toe-{sd}", "rubber-clog", 5, ebone=f"foot-{sd}", pick=f"clog-{sd}", range="0,0.0868", exp=2.4, tol=0.003, round=0.6, tone=0.3)
    b.append(t)
save(slug, d)
print(check(slug, 50, "volumes-bones,volumes-silhouette,volumes-feature,volumes-views-disagree"))

print(thermal(slug, 50, "shirt-torso=shirt-hem+shoulder-l+shoulder-r,clogs=clog-toe-l+clog-toe-r")[:1900])
DET = {"eye-l","eye-r","brow-l","brow-r","nose","mouth","ear-l","ear-r","hat-band","knee-patch-l","knee-patch-r","clog-strap-l","clog-strap-r","weed-stalk-0","weed-stalk-1","weed-stalk-2","weed-stalk-3","trug-weeds","shoulder-l","shoulder-r"}
print(prune_and_rebuild_shells(slug, 50, DET, drop_refs=("trug-weeds","weed-stalk-0","weed-stalk-1","weed-stalk-2","weed-stalk-3","clog-strap-l","clog-strap-r")))
d = load(slug)
for t in d["volumes"]["50"]["thermal"]:
    if t["id"] == "th-plume": print("plume", t.get("transform"))
person_structure(slug, 50)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "50", "--write")[-400:])
print(check(slug, 50))
