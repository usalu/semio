"""🗄️ Tower server, second round pass 1: hinged side cover (bone `cover`), pose `service-open`, interior at 1:50, 1:200 and 1:500 bodies re-modelled. Hand-chosen numbers; writes the candidate document, `--publish` writes source and catalog."""
import json, sys, copy, os
T = os.path.dirname(os.path.abspath(__file__))
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/tower-server/🔣️.json"
SRC = T + "/volumes/appliances/tower-server.json"
OUT = T + "/🗑️generated/r3-tower-server/🔣️.json"
HX, HY, HZ = 0.012, 0.213, -0.095
OPEN = 35
r = lambda v: round(v, 5)

def to_cover(s):
    s = dict(s); s["bone"] = "cover"
    if s.get("kind") == "extrude":
        assert s["plane"] == "xy"
        f = lambda p: [r(HY - p[1]), r(p[0] - HX)]
        s["profile"] = [f(p) for p in s["profile"]]
        if "holes" in s: s["holes"] = [[f(p) for p in h] for h in s["holes"]]
        s["from"], s["to"] = r(s["from"] - HZ), r(s["to"] - HZ)
    elif s.get("kind") == "box":
        sx, sy, sz = s["size"]; tx, ty, tz = s["transform"]["translation"]
        assert "rotation" not in s["transform"]
        s["size"] = [sy, sx, sz]; s["transform"] = {"translation": [r(HY - ty), r(tx - HX), r(tz - HZ)]}
    else: raise SystemExit("kind " + s["id"])
    return s

def rebind(lst, ids):
    for i, s in enumerate(lst):
        if s["id"] in ids and s.get("bone") == "case": lst[i] = to_cover(s)

def after(lst, anchor, new):
    i = next(k for k, s in enumerate(lst) if s["id"] == anchor) + 1
    lst[i:i] = new

doc = json.load(open(A)); a = doc["asset"]
V = json.load(open(SRC))["volumes"]
if any(b["id"] == "cover" for b in a["rig"]["bones"]): raise SystemExit("already applied")

a["rig"]["bones"].append({"id": "cover", "parent": "case", "title": {"en": "Side cover, tilting out on its bottom edge", "de": "Seitendeckel, kippt um seine Unterkante nach außen"}, "x": HX, "y": HY, "z": HZ, "angleDeg": -90, "lengthM": 0.413, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 90}})
off = next(p for p in a["poses"] if p["id"] == "off")
so = copy.deepcopy(off); so["id"] = "service-open"
so["title"] = {"en": "Open for service", "de": "Für die Wartung geöffnet"}
so["description"] = {"en": "Switched off, latch released: the side cover is tilted 35 degrees out on its bottom edge before it is lifted away, drive cage, memory, cooler and power supply lie open.", "de": "Ausgeschaltet, Verriegelung gelöst: der Seitendeckel ist um seine Unterkante 35 Grad nach außen gekippt, bevor er abgehoben wird, Laufwerkskäfig, Speicher, Kühler und Netzteil liegen offen."}
so["bones"] = {"cover": {"twistDeg": OPEN}}
a["poses"].append(so)

for sc in ("50", "100", "200", "500"):
    rebind(V[sc]["thermal"], {"th-cover-skin", "th-cpu-grille"})
rebind(V["50"]["body"], {"cover", "cpu-grille", "latch-well", "latch-lever"})
rebind(V["100"]["body"], {"cover", "cpu-grille", "latch-well"})

steel = {"bone": "case", "material": "chassis-steel", "role": "body"}
after(V["50"]["body"], "mainboard", [
    {"id": "drive-cage", "kind": "extrude", **steel, "tone": 0.5, "profile": [[-0.055, 0.13], [-0.055, 0.0128], [0.055, 0.0128], [0.055, 0.13], [0.0542, 0.13], [0.0542, 0.0136], [-0.0542, 0.0136], [-0.0542, 0.13]], "plane": "zx", "from": -0.19, "to": -0.03},
    {"id": "disk-drives", "kind": "box", **steel, "tone": 0.35, "size": [0.026, 0.147, 0.1084], "transform": {"translation": [0.0338, -0.1135, 0.0]}, "repeat": {"count": 2, "translation": [0.036, 0.0, 0.0]}},
    {"id": "memory-modules", "kind": "box", "bone": "case", "material": "bezel-abs", "role": "detail", "tone": 0.3, "size": [0.133, 0.004, 0.03], "transform": {"translation": [0.2765, 0.0, 0.0761]}, "texture": True, "repeat": {"count": 4, "translation": [0.0, 0.0095, 0.0]}},
])

b50 = {s["id"]: s for s in V["50"]["body"]}
b50["cpu-cooler-base"]["size"] = [0.068, 0.04, 0.0201]
b50["psu-core"]["size"], b50["psu-core"]["transform"] = [0.0772, 0.125, 0.13], {"translation": [0.3914, 0.1525, 0.0]}
after(V["50"]["body"], "mainboard", [{"id": "board-standoffs-" + n, "kind": "revolve", "bone": "case", "material": "chassis-steel", "role": "detail", "tone": 0.6, "texture": True, "profile": [[0.0, 0.0], [0.0, 0.003], [0.0065, 0.003], [0.0065, 0.0]], "axis": [0, 0, 1], "origin": [x, -0.02, 0.0927], "repeat": {"count": 3, "translation": [0.0, 0.11, 0.0]}} for n, x in (("low", 0.06), ("mid", 0.2), ("high", 0.335))])
next(s for s in V["100"]["body"] if s["id"] == "rear-fan-hub")["profile"] = [[-0.0255, 0.0], [-0.0255, 0.021], [0.01, 0.021], [0.01, 0.0]]
b2 = V["200"]["body"]
c2 = next(s for s in b2 if s["id"] == "cover")
c2["holes"].append([[0.415, -0.05], [0.415, -0.12], [0.385, -0.12], [0.385, -0.05]])
c2["from"], c2["to"] = -0.1, -0.07
rebind(b2, {"cover"})
ch = next(s for s in b2 if s["id"] == "chassis")
ch["profile"] = [[-0.07, 0.012], [0.1, 0.012], [0.1, 0.43], [0.08, 0.43], [0.08, 0.032], [-0.07, 0.032]]
co = next(s for s in b2 if s["id"] == "core")
co["size"], co["transform"] = [0.398, 0.4, 0.15], {"translation": [0.231, 0.005, 0.005]}
s2 = {s["id"]: s for s in V["200"]["structure"]}
s2["member-case-floor"]["size"], s2["member-case-floor"]["transform"] = [0.02, 0.4, 0.17], {"translation": [0.022, 0.005, 0.015]}
s2["member-case-posts"]["transform"]["translation"] = [0.221, 0.195, -0.06]
s2["member-case-top-frame"]["profile"] = [[0.205, -0.07], [-0.195, -0.07], [-0.195, 0.1], [0.205, 0.1]]
s2["member-case-top-frame"]["holes"] = [[[0.185, 0.08], [-0.175, 0.08], [-0.175, -0.05], [0.185, -0.05]]]
b2 += [
    {"id": "optical-drive", "kind": "box", "bone": "case", "material": "bezel-abs", "role": "body", "tone": 0.16, "size": [0.042, 0.19, 0.148], "transform": {"translation": [0.406, -0.125, 0.0]}},
    {"id": "rear-fan-hub", "kind": "revolve", "bone": "case", "material": "bezel-abs", "role": "detail", "tone": 0.2, "profile": [[-0.015, 0.0], [-0.015, 0.021], [0.015, 0.021], [0.015, 0.0]], "axis": [0, -1, 0], "origin": [0.285, 0.21, -0.025]},
]

b5 = V["500"]["body"]
plinth = next(s for s in b5 if s["id"] == "plinth")
bez = next(s for s in b5 if s["id"] == "front-bezel")
octagon = [[0.033, 0.285], [0.01601, 0.32601], [-0.025, 0.343], [-0.06601, 0.32601], [-0.083, 0.285], [-0.06601, 0.24399], [-0.025, 0.227], [0.01601, 0.24399]]
V["500"]["body"] = [
    plinth,
    {"id": "case", "kind": "box", **steel, "tone": 0.55, "size": [0.4372, 0.3, 0.1992], "transform": {"translation": [0.2306, 0.0, 0.0004]}},
    {"id": "rear-panel", "kind": "extrude", **steel, "tone": 0.5, "profile": [[-0.0992, 0.012], [0.1, 0.012], [0.1, 0.4492], [-0.0992, 0.4492]], "plane": "zx", "from": 0.15, "to": 0.225, "holes": [[[-0.075, 0.438], [0.075, 0.438], [0.075, 0.352], [-0.075, 0.352]], octagon[::-1]]},
    {"id": "front-bezel", "kind": "extrude", "bone": "case", "material": bez["material"], "role": "body", "tone": bez["tone"], "profile": [[-0.1, 0.012], [0.1, 0.012], [0.1, 0.45], [-0.1, 0.45]], "plane": "zx", "from": -0.225, "to": -0.15, "holes": [[[-0.08, 0.35], [0.08, 0.35], [0.08, 0.05], [-0.08, 0.05]]]},
    {"id": "core", "kind": "box", **steel, "tone": 0.14, "size": [0.42, 0.425, 0.18], "transform": {"translation": [0.23, 0.0025, 0.0]}},
    {"id": "top-skin", "kind": "extrude", **steel, "tone": 0.58, "sheet": True, "profile": [[0.225, -0.0992], [-0.15, -0.0992], [-0.15, 0.1], [0.225, 0.1]], "plane": "yz", "from": 0.4492, "to": 0.45, "holes": [[[0.205, 0.085], [0.105, 0.085], [0.105, -0.085], [0.205, -0.085]]]},
    {"id": "psu-top-grille", "kind": "extrude", "bone": "case", "material": "chassis-steel", "role": "detail", "tone": 0.4, "sheet": True, "profile": [[0.205, -0.085], [0.105, -0.085], [0.105, 0.085], [0.205, 0.085]], "plane": "yz", "from": 0.4492, "to": 0.45},
    to_cover({"id": "cover", "kind": "extrude", **steel, "tone": 0.55, "sheet": True, "profile": [[0.012, 0.225], [0.012, -0.15], [0.45, -0.15], [0.45, 0.225]], "plane": "xy", "from": -0.1, "to": -0.0992, "holes": [[[0.34, 0.195], [0.34, 0.035], [0.21, 0.035], [0.21, 0.195]]]}),
    to_cover({"id": "cpu-grille", "kind": "extrude", "bone": "case", "material": "chassis-steel", "role": "detail", "tone": 0.4, "sheet": True, "profile": [[0.21, 0.195], [0.21, 0.035], [0.34, 0.035], [0.34, 0.195]], "plane": "xy", "from": -0.1, "to": -0.0992}),
]

for sc in ("50", "100", "200", "500"):
    after(V[sc]["structure"], "member-case-posts" if sc != "50" else "member-case-post-rear", [{"id": "member-cover-panel", "of": "cover", "member": "cover", "part": "member"}])
a["volumes"] = V
os.makedirs(os.path.dirname(OUT), exist_ok=True)
json.dump(doc, open(OUT, "w"), ensure_ascii=False, indent=2)
if "--publish" in sys.argv:
    json.dump({"volumes": V}, open(SRC, "w"), ensure_ascii=False, indent=2)
    cur = json.load(open(A))
    cur["asset"]["rig"], cur["asset"]["poses"] = a["rig"], a["poses"]
    json.dump(cur, open(A, "w"), ensure_ascii=False, indent=2)
print("written", OUT)
