import sys, json
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
slug = sys.argv[1]
new = json.loads(sys.argv[2])
log = []
def fix(a):
    for r in a["thermal"]["regions"]:
        if r["id"] in new and r["areaM2"] != new[r["id"]]:
            log.append((r["id"], r["areaM2"], new[r["id"]]))
            r["areaM2"] = new[r["id"]]
L.edit_asset("🛋️furniture", slug, fix)
print(slug, log)
open("🗑️generated/author-16/areafix-%s.txt" % slug, "w").write(json.dumps(log))
