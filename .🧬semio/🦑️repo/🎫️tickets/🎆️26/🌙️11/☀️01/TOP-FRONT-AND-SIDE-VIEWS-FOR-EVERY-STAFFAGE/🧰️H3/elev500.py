"""The 1:500 elevation carries the same limb thickness as the 1:500 front and plan: the cross-section (local y) of every limb silhouette is scaled about its bone axis up to the radius the three views share at this scale."""
import sys, re
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *

ARMS = ("arm", "forearm", "upper-arm")
LEGS = ("thigh", "shank")
TARGET = {"arm": .09, "leg": .08}
OVERRIDE = {"running-child": {"arm": .055, "leg": .05}}


def family(bone):
    base = bone.rsplit("-", 1)[0] if bone[-2:] in ("-l", "-r", "-n", "-f") else bone
    if base in ARMS: return "arm"
    if base in LEGS: return "leg"
    return None


def scale_path(d, k):
    tok = re.findall(r"[A-Za-z]|-?\d+\.?\d*(?:e-?\d+)?", d)
    assert not set(t for t in tok if t.isalpha()) - set("MLCQZ"), set(t for t in tok if t.isalpha())
    out, i = [], 0
    nums = []
    for t in tok:
        if t.isalpha():
            out.append(t); nums = []
        else:
            nums.append(t)
            out.append(t if len(nums) % 2 else f"{float(t) * k:.5f}")
    return " ".join(out)


def maxabs_y(s):
    b = pts_bbox(s); return max(abs(b[1]), abs(b[3]))


for aid in sys.argv[1:]:
    doc = load(aid); a = doc["asset"]
    rep = a["representations"]["500"]
    n = 0
    for s in rep["geometry"]["elevation"]:
        f = family(s["bone"])
        if f is None or s["id"].startswith(("blade",)): continue
        cur = maxabs_y(s)
        if cur < 1e-6: print("skip", s["id"], s["kind"]); continue
        tgt = OVERRIDE.get(aid, TARGET)[f]
        k = tgt / cur
        if (k <= 1.001 and aid not in OVERRIDE) or abs(k - 1) < .001: continue
        if s["kind"] == "path": s["d"] = scale_path(s["d"], k)
        elif s["kind"] == "poly": s["points"] = [[x, round(y * k, 5)] for x, y in s["points"]]
        elif s["kind"] == "capsule": s.update(y0=round(s["y0"] * k, 5), y1=round(s["y1"] * k, 5), r0=round(s["r0"] * k, 5), r1=round(s["r1"] * k, 5))
        else: continue
        n += 1
    save(aid, doc)
    print(aid, "thickened", n)
