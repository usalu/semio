"""Restores volume solids of a humans asset to the text of HEAD (same ids, byte for byte). python3 humansa-restore-solids.py <slug> <solidId,...> [scales]"""
import importlib.util, json, subprocess, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('s', O + 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug, ids = sys.argv[1], set(sys.argv[2].split(","))
scales = sys.argv[3].split(",") if len(sys.argv) > 3 else ["500", "200", "100", "50"]
F = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/" + slug + "/🔣️.json"
head = subprocess.run(["git", "-C", "/Users/ueli/Documents/semio", "show", "HEAD:" + F], capture_output=True, text=True, encoding="utf8").stdout
hroot = s.parse(head)
d = s.Doc("👤️humans/" + slug)
def idof(text, it): return json.loads(text[next(v for k, v in it["pairs"] if k == "id")["s"]:next(v for k, v in it["pairs"] if k == "id")["e"]])
n = 0
for sc in scales:
    htext = {idof(head, it): head[it["s"]:it["e"]] for it in s.walk(hroot, ["asset", "volumes", sc, "body"])["items"]}
    for it in s.walk(d.root, ["asset", "volumes", sc, "body"])["items"]:
        sid = idof(d.t, it)
        if sid in ids and sid in htext: d.rep(it, htext[sid]); n += 1
d.commit()
print(slug, "restored solids", n)
