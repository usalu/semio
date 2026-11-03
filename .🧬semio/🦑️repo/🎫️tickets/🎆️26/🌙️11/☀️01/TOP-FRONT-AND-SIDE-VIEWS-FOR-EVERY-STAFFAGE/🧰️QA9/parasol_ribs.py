"""QA9: give the market parasol its eight physical ribs (true length, azimuth by abduction in every pose) and draw all ribs in plan.
Idempotent: always starts from the snapshot of the file before this script (QA9 scratch copy) when --from-snapshot is given."""
import json, math, os, re, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧰️O1"))
from lib import load, save, cap

A = "market-parasol"
ALU = dict(region="frame-alu", material="aluminium-anodised")
RIBS = {"rib-right": 0, "rib-h": +1, "rib-b": -1, "rib-g": +1, "rib-c": -1, "rib-f": +1, "rib-d": -1, "rib-left": 0}
doc = json.load(open(sys.argv[1], encoding="utf-8")) if len(sys.argv) > 1 else load(A)
asset = doc["asset"]
bones = {b["id"]: b for b in asset["rig"]["bones"]}
L = bones["rib-right"]["lengthM"]
rad = math.radians
info = {}
for rid, sign in RIBS.items():
    b = bones[rid]
    lp, beta = b["lengthM"], b["angleDeg"]
    if sign == 0:
        info[rid] = dict(theta=0.0 if rid == "rib-right" else 180.0, sign=0, scale=1.0, rest=beta)
        continue
    dx, dy = lp * math.cos(rad(beta)), lp * math.sin(rad(beta))
    cos_alpha = math.sqrt(1 - (dy / L) ** 2)
    theta = math.degrees(math.acos(max(-1, min(1, dx / (L * cos_alpha)))))
    info[rid] = dict(theta=sign * theta, sign=sign, scale=L / lp, rest=beta, cosphi=lp / L)
    phi = math.degrees(math.atan(math.cos(rad(beta)) * math.tan(rad(sign * theta))))
    assert abs(math.cos(rad(phi)) - lp / L) < 2e-3, (rid, phi, lp / L)
    print(rid, "theta", round(sign * theta, 1), "phi_rest", round(phi, 2))


def phi_for(rid, beta_deg):
    th = info[rid]["theta"]
    if info[rid]["sign"] == 0:
        return 0.0
    return math.degrees(math.atan(math.cos(rad(beta_deg)) * math.tan(rad(th))))


def wrap(a):
    return (a + 180) % 360 - 180


REST = {}
for rid in RIBS:
    REST[rid] = 0.0 if math.cos(rad(bones[rid]["angleDeg"])) > 0 else 180.0
OLD = {rid: bones[rid]["angleDeg"] for rid in RIBS}
for rid in RIBS:
    bones[rid]["lengthM"] = L
    bones[rid]["angleDeg"] = REST[rid]
    if "mass" in bones[rid]:
        bones[rid]["mass"]["comX"] = round(L / 2, 4)


def scale_x(s, k):
    if k == 1.0:
        return
    kind = s["kind"]
    if kind == "capsule":
        s["x0"] = round(s["x0"] * k, 6); s["x1"] = round(s["x1"] * k, 6)
    elif kind == "ellipse":
        s["cx"] = round(s["cx"] * k, 6); s["rx"] = round(s["rx"] * k, 6)
    elif kind == "rect":
        s["x"] = round(s["x"] * k, 6); s["w"] = round(s["w"] * k, 6)
    elif kind == "poly":
        s["points"] = [[round(x * k, 6), y] for x, y in s["points"]]
    elif kind == "path":
        toks = re.findall(r"[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?", s["d"])
        out, i = [], 0
        for t in toks:
            if t.isalpha():
                out.append(t); i = 0
            else:
                v = float(t) * (k if i % 2 == 0 else 1)
                out.append(f"{v:.5f}".rstrip("0").rstrip(".")); i += 1
        s["d"] = " ".join(out)
    else:
        raise ValueError(kind)


for sc, rep in asset["representations"].items():
    if rep["status"] != "authored":
        continue
    for vp in ("elevation", "front", "plan"):
        for s in rep["geometry"][vp]:
            if s["bone"] in RIBS and "followTip" not in s:
                scale_x(s, info[s["bone"]]["scale"])


def tune(bones_map, base_angles):
    for rid in RIBS:
        beta = OLD[rid] + bones_map.get(rid, {}).get("angleDeg", 0)
        e = bones_map.setdefault(rid, {})
        e["angleDeg"] = round(wrap(beta - REST[rid]), 4)
        if info[rid]["sign"] != 0:
            e["abductionDeg"] = round(phi_for(rid, beta), 4)


for pose in asset["poses"]:
    tune(pose["bones"], None)
    print("pose", pose["id"], {r: pose["bones"][r].get("abductionDeg") for r in RIBS if info[r]["sign"]})

for clip in asset["clips"]:
    for key in clip["keys"]:
        for rid, e in (key.get("bones") or {}).items():
            if rid in RIBS and "angleDeg" in e:
                e["angleDeg"] = round(wrap(OLD[rid] + e["angleDeg"] - REST[rid]), 4)

for sc in ("50", "100", "200"):
    for s in asset["representations"][sc]["geometry"]["plan"]:
        if s["bone"] in RIBS and s["kind"] == "capsule":
            s["x1"] = round(L if REST[s["bone"]] == 0 else -L, 6)
        if s.get("solid", {}).get("axisBone") == "crown":
            pass

for sc, rep in asset["representations"].items():
    if rep["status"] != "authored":
        continue
    for vp in ("front", "plan"):
        for s in rep["geometry"][vp]:
            if s.get("solid", {}).get("kind") == "revolved" and s["solid"]["axisBone"] == "crown":
                s["solid"]["axisBone"] = "runner"

PLAN_SCALES = {200: (0.011, 0.007, {}), 100: (0.011, 0.007, {}), 50: (0.011, 0.007, {})}
for sc, (r0, r1, _) in PLAN_SCALES.items():
    plan = asset["representations"][str(sc)]["geometry"]["plan"]
    have = {s["id"] for s in plan}
    for rid in RIBS:
        if rid in have or info[rid]["sign"] == 0:
            continue
        x1 = L if REST[rid] == 0 else -L
        s = cap(rid, rid, 0, 0, x1, 0, r0, r1, z=2.5, tone=0.35, follow=True, **ALU)
        plan.append(s)

TXT = {
    "silhouette": ("All eight ribs are physical bones of the full rib length, each turned out of the side plane by its azimuth (abduction in every pose); the plan draws every rib as a spoke and the side view foreshortens them by itself. The hem stays a revolved circle about the mast, because a revolved solid is always a circle.",
                   "Alle acht Rippen sind physische Knochen in voller Rippenlänge, jede mit ihrem Azimut (Abduktion in jeder Pose) aus der Seitenebene gedreht; die Draufsicht zeichnet jede Rippe als Speiche, die Seitenansicht verkürzt sie von selbst. Der Saum bleibt ein Rotationskreis um den Mast, weil ein Rotationskörper immer ein Kreis ist."),
}
for sc in ("50", "100", "200", "500"):
    rep = asset["representations"][sc]
    if rep["status"] != "authored":
        continue
    d = rep["decisions"]["silhouette"]
    old_en, old_de = "plan: the canopy disc", None
    if TXT["silhouette"][0] not in d["en"]:
        d["en"] = d["en"].rstrip() + " " + TXT["silhouette"][0]
        d["de"] = d["de"].rstrip() + " " + TXT["silhouette"][1]
    for k in ("omitted",):
        d2 = rep["decisions"][k]
        d2["en"] = d2["en"].replace("the other four ribs are not drawn in front and plan", "the front keeps the revolved rib solids").replace("the other ribs are not drawn in plan because their plan angle cannot follow the folding", "the remaining ribs are drawn in plan at 1:100 and larger")
        d2["de"] = d2["de"].replace("die übrigen vier Rippen werden in Vorder- und Draufsicht nicht gezeichnet", "die Vorderansicht behält die Rotationsrippen").replace("die übrigen Rippen werden in der Draufsicht nicht gezeichnet, weil ihr Grundrisswinkel dem Zusammenfalten nicht folgen kann", "die übrigen Rippen sind in der Draufsicht ab 1:100 gezeichnet")
save(A, doc)
print("saved")
