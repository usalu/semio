"""🔎️ Prints, per pose of person-sweeping, the grips against the handle line, the bristle tips against the floor and the trunk lean (from kit.ts frames)."""
import json, math, sys
from lib import GEN, HUMANS, bun
SLUG = "person-sweeping"
path = f"{HUMANS}/{SLUG}/🔣️.json"
asset = json.load(open(path))["asset"]
ids = ",".join(p["id"] for p in asset["poses"])
bun("kit.ts", "frames", path, ids, f"{GEN}/sweeping-frames.json")
fr = json.load(open(f"{GEN}/sweeping-frames.json"))
sub = lambda a, b: [a[i] - b[i] for i in range(3)]
dot = lambda a, b: sum(a[i] * b[i] for i in range(3))
for pid, s in fr.items():
    f, an = s["frames"], s["anchors"]
    b = f["broom"]
    u = b["u"]
    def off(p):
        r = sub(p, b["o"]); al = dot(r, u); perp = [r[i] - al * u[i] for i in range(3)]
        return al, math.sqrt(dot(perp, perp))
    hw = lambda fr_, p: [fr_["o"][k] + p[0]*fr_["u"][k] + p[1]*fr_["v"][k] + p[2]*fr_["w"][k] for k in range(3)]
    gl = off(hw(f["hand-l"], [0.09, 0, -0.031]))
    an["grip-r"] = hw(f["hand-r"], [0.09, 0, 0.031])
    hl = f["hand-l"]["v"]
    print(f"{pid:18s} tip {[round(x,3) for x in an['broom-tip']]}  grip-r {[round(x,3) for x in an['grip-r']]}  grip-l along {gl[0]:.3f} off {gl[1]*1000:.0f} mm  handVl·up {-dot(hl,u):.2f}  heels {an['heel-l'][1]:.3f},{an['heel-r'][1]:.3f} balls {an['ball-l'][1]:.3f},{an['ball-r'][1]:.3f}  head-u {[round(x,2) for x in f['broom-head']['u']]}  elev {math.degrees(math.asin(-u[1])):.0f}")
