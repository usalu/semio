import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
import subprocess
BONE = {k: tuple(v) for k, v in json.loads(subprocess.run(["./vehicles-run.sh", "bones", "cargo-bike", "50"], capture_output=True, text=True).stdout.strip().splitlines()[-1]).items()}
sc = int(sys.argv[1]); views = sys.argv[2].split(",")
tol = {50: "0.004", 100: "0.006", 200: "0.005", 500: "0.012"}[sc]
allspec = json.load(open("vehicles-cb-spec.json"))[str(sc)]
for view in views:
    spec = allspec.get(view, allspec) if "elevation" in allspec or "front" in allspec or "plan" in allspec else allspec
    items = d.at("asset", "representations", str(sc), "geometry", view)["items"]
    have = {}
    for it in items:
        o = json.loads(d.t[it["s"]:it["e"]]); have[o["id"]] = o
    for sid, ids in spec.items():
        if sid not in have: continue
        solid = "solid" in have[sid]
        bone = have[sid].get("bone", "frame")
        if view == "elevation":
            r = d.retrace_each(sc, view, sid, ids.split(","), tol, xf=BONE.get(bone, BONE["frame"]))
        else:
            r = d.retrace_each(sc, view, sid, ids.split(","), tol)
        print(sc, view, sid, "->", r)
d.commit()
