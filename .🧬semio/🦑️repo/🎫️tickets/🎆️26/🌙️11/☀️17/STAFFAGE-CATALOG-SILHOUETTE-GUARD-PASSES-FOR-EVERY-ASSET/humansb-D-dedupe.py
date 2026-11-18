"""🧹 Removes later duplicates of a shape id in one view list (text span from the end of the previous item to the end of the duplicate). python3 humansb-D-dedupe.py <slug> <scale> <view> <id>"""
import importlib.util, json, sys
spec = importlib.util.spec_from_file_location('s', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug, scale, view, sid = sys.argv[1:5]
d = s.Doc(slug)
items = s.walk(d.root, ["asset", "representations", scale, "geometry", view])["items"]
hits = [i for i, it in enumerate(items) if any(k == "id" and json.loads(d.t[v["s"]:v["e"]]) == sid for k, v in it["pairs"])]
for i in hits[1:][::-1]:
    d.edits.append((items[i - 1]["e"], items[i]["e"], ""))
d.commit()
print("removed", len(hits) - 1)
