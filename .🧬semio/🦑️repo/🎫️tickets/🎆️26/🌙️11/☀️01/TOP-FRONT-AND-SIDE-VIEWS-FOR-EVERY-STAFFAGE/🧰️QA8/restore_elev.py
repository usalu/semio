import json, os, subprocess, sys

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
sys.path.insert(0, A2)
import lib, jsonfmt

# restores the authored elevation outline of every shape that a revolved solid replaced (original from HEAD), the builders decide again
for aid in ("mallard-duck", "pig", "pigeon", "red-fox", "red-squirrel", "roe-deer", "sheep"):
    rel = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/%s/🔣️.json" % aid
    head = json.loads(subprocess.run(["git", "-c", "core.quotepath=false", "show", "HEAD:" + rel], cwd=lib.REPO, capture_output=True).stdout.decode("utf-8"))["asset"]
    path = os.path.join(lib.CAT, aid, "🔣️.json")
    doc = jsonfmt.read(path)
    n = 0
    for sc, rep in doc["asset"]["representations"].items():
        if "geometry" not in rep or sc not in head["representations"] or "geometry" not in head["representations"][sc]:
            continue
        old = {s["id"]: s for s in head["representations"][sc]["geometry"]["elevation"]}
        for i, sh in enumerate(rep["geometry"]["elevation"]):
            if sh.get("solid", {}).get("kind") == "revolved" and sh["id"] in old:
                rep["geometry"]["elevation"][i] = old[sh["id"]]
                n += 1
    jsonfmt.write(path, doc)
    print(aid, "restored", n)
