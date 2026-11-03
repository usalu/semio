import json
import sys

sys.stdout.reconfigure(encoding="utf8")
a = json.load(open(sys.argv[1], encoding="utf8"))["asset"]
print("keys", list(a))
print("dims", a["dimensions"])
print("rig", a["rig"]["kind"], a["rig"]["root"], len(a["rig"]["bones"]))
print("bones", [(b["id"], b["parent"], b["joint"]) for b in a["rig"]["bones"]])
print("anchors", [(x["id"], x["bone"], x["kind"]) for x in a["anchors"]])
print("poses", [(p["id"], p.get("key", False)) for p in a["poses"]])
print("clips", [(c["id"], c["durationS"], c["loop"], c["rootMotion"]) for c in a["clips"]])
print("pres", a["presentation"])
print("rep keys", {k: (v.get("status"), list(v)) for k, v in a["representations"].items()})
r = a["representations"]["100"]
for k, v in r.items():
    if isinstance(v, (dict, list)):
        print(" ", k, type(v).__name__, (list(v) if isinstance(v, dict) else len(v)))
th = a["thermal"]
print("thermal", list(th))
print("emitters", [(e["id"], e["kind"], e.get("anchor")) for e in th["emitters"]])
print("resp", {k: v for k, v in th["respiration"].items() if k != "patterns"})
print("regions", [(x["id"]) for x in th["regions"]])
print("symbols", [s.get("id") for s in a["symbols"]])
