import json, subprocess, sys, glob, os
here = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
slug = sys.argv[1]
a = json.load(open(glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*appliances/%s/*.json" % slug)[0]))["asset"]
area = {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]}
def run(flag):
    args = [BUN, "volumes-check.ts", "appliances/" + slug, "--rules", "volumes-thermal-coverage", "--json"] + ([flag] if flag == "--product" else [])
    out = subprocess.run(args, cwd=here, capture_output=True, text=True).stdout
    return {(m["name"].split(":")[1], m["scale"]): m["value"] for m in json.loads(out)["measurements"] if m["name"].startswith("areaDeviation:") and (flag == "--product" or m.get("engine") == "o")}
o, p = run("--owned"), run("--product")
for k in sorted(o, key=lambda k: (k[1], k[0])):
    A = area[k[0]]
    print("1:%d" % k[1], k[0].ljust(13), "areaM2", A, "owned dev", round(o[k], 2), "product dev", round(p.get(k, -1), 2))
