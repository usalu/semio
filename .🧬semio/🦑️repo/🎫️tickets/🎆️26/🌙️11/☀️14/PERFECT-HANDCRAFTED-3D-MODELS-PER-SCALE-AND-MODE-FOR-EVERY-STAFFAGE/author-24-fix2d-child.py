"""Root repair of running-child 1:500 elevation: the open shoe stroke becomes the closed filled outline of the 1:200 shoe."""
import re
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/running-child/🔣️.json"
t = open(P).read()
pts = [[-0.0245, -0.0109], [-0.0163, -0.0409], [-0.0033, -0.0462], [0.1036, 0.0], [0.109, 0.0136], [0.0872, 0.0327], [0.0163, 0.0256], [-0.0163, 0.0065]]
body = ",\n".join("                [\n                  %s,\n                  %s\n                ]" % (x, y) for x, y in pts)
n = 0
for side in ("lr" if '"sil-shoe-l"' in t and '"filled": false' in t.split('"sil-shoe-l"')[1][:200] else ""):
    pat = re.compile(r'("id": "sil-shoe-%s".*?"filled": )false(,\s*"tone": 0.85,\s*"kind": "poly",\s*"points": \[).*?(\],\s*"closed": )false' % side, re.S)
    t, k = pat.subn(lambda m: m.group(1) + "true" + m.group(2) + "\n" + body + "\n              " + m.group(3) + "true", t, count=1)
    n += k
print("replaced", n)
open(P, "w").write(t)

import json
d = json.load(open(P))
plan = d["asset"]["representations"]["500"]["geometry"]["plan"]
for side in "lr":
    heel = next(s for s in plan if s["id"] == f"shoe-heel-{side}")
    heel["x1"], heel["r1"] = 0.1085, 0.0256
    plan[:] = [s for s in plan if s["id"] != f"shoe-toe-{side}"]
json.dump(d, open(P, "w"), indent=2, ensure_ascii=False)
open(P, "a").write("\n")
print("plan shoe toe merged into heel capsule at 1:500")
