"""Removes a shape from a view of a humans asset (array item and its comma). python3 humansa-remove-shape.py <slug> <scale:view:id,...>"""
import importlib.util, json, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
d = s.Doc("👤️humans/" + sys.argv[1])
for item in sys.argv[2].split(","):
    sc, view, sid = item.split(":")
    arr = s.walk(d.root, ["asset", "representations", sc, "geometry", view])
    items = arr["items"]
    for k, it in enumerate(items):
        if json.loads(d.t[next(v for kk, v in it["pairs"] if kk == "id")["s"]:next(v for kk, v in it["pairs"] if kk == "id")["e"]]) != sid: continue
        if k > 0: d.edits.append((items[k - 1]["e"], it["e"], ""))
        else: d.edits.append((it["s"], items[k + 1]["s"], ""))
        print("removed", item); break
d.commit()
