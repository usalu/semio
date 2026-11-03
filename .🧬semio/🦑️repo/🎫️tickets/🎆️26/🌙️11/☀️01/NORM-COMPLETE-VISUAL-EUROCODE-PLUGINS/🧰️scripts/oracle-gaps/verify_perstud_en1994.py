"""🔩 Hand check of `en1994.6.6.2.perstud` against the oracle: analytic Voronoi cells (no sampling), P_Rd with k_t per station from 6.6.3.1 and 6.6.4.2, and the old aggregate (mean P_Rd x studs per half) for contrast. Usage: verify_perstud_en1994.py <fixture-dir-name>..."""
import json, math, subprocess, sys, pathlib
ART = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any")
ORACLE = ART / "🔮️oracles/🚦️check-outcome/🐍️.py"
for name in sys.argv[1:]:
    path = ART / "🧪️tests/🧫️fixtures/📄️oracle-documents" / name / "🔣️.json"
    doc = json.load(open(path, encoding="utf8"))
    out = {c["id"]: c["utilization"] for c in json.loads(subprocess.run(["nice", "-n", "19", sys.executable, "-I", str(ORACLE), str(path)], capture_output=True, text=True, check=True).stdout)["checks"]}
    b = doc["beams"][0]
    L, fck, ecm, sh = b["spanM"], b["concreteFCkPa"], b["concreteECmPa"], b["sheeting"]
    xs = sorted(((s["xM"], s) for s in doc["studs"] if s["beamId"] == b["id"]), key=lambda t: (t[0], t[1]["id"]))
    inside = [(x, s) for x, s in xs if -1e-6 <= x <= L + 1e-6]
    stations = []
    for x, s in inside:
        if stations and x - stations[-1][0][0] <= 1e-3:
            stations[-1].append((x, s))
        else:
            stations.append([(x, s)])
    def prd(s, nr):
        d, h = s["diameterM"], s["heightM"]
        a = 1.0 if h / d > 4 else 0.2 * (h / d + 1)
        p = min(0.8 * s["fUPa"] * math.pi * d * d / 4, 0.29 * a * d * d * math.sqrt(fck * ecm)) / 1.25
        hp = sh["heightM"]
        g = sh["ribWidthM"] / hp * (min(h, hp + 0.075) / hp - 1)
        thin, weld = sh["thicknessM"] <= 1e-3, d <= 0.020
        kmax = {(True, 1, True): .85, (True, 1, False): .75, (True, 2, True): .7, (True, 2, False): .6, (False, 1, True): 1., (False, 1, False): .75, (False, 2, True): .8, (False, 2, False): .6}[(thin, nr, weld)]
        return p * min(0.7 / math.sqrt(nr) * g, kmax)
    n_req = out["en1994.6.6.3.1.prd.beam-B1"] * min(sum(prd(s, len(st)) * (0.5 if abs(x - L / 2) <= 1e-3 else 1) for st in stations for x, s in st if x < L / 2 + 1e-3), sum(prd(s, len(st)) * (0.5 if abs(x - L / 2) <= 1e-3 else 1) for st in stations for x, s in st if x > L / 2 - 1e-3))
    flow = n_req / (L / 2)
    force, worst = {}, 0.0
    for lo, hi, left in ((0, L / 2, True), (L / 2, L, False)):
        sts = [st for st in stations if any((x < L / 2 - 1e-3 if left else x > L / 2 + 1e-3) or abs(x - L / 2) <= 1e-3 for x, _ in st)]
        pos = [st[0][0] for st in sts]
        for k, st in enumerate(sts):
            a = lo if k == 0 else (pos[k - 1] + pos[k]) / 2
            c = hi if k == len(sts) - 1 else (pos[k] + pos[k + 1]) / 2
            w = lambda x: 0.5 if abs(x - L / 2) <= 1e-3 else 1.0
            tot = sum(w(x) * prd(s, len(st)) for x, s in st)
            for x, s in st:
                f = flow * (c - a) * w(x) * prd(s, len(st)) / tot
                worst = max(worst, f / (w(x) * prd(s, len(st))))
    mean = sum(prd(s, len(st)) for st in stations for _, s in st) / len(inside)
    aggregate = n_req / (len(inside) / 2 * mean)
    print(name, json.dumps({"oracle": out["en1994.6.6.2.perstud.beam-B1"], "analytic": worst, "aggregateMean": aggregate, "prd": out["en1994.6.6.3.1.prd.beam-B1"]}))
