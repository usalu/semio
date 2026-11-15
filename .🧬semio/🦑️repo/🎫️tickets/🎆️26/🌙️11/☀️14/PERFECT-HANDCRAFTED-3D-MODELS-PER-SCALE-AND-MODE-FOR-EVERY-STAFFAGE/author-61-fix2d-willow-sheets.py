"""Author 61: add the missing elevation outlines of the willow's curtain sheets (see author-61-sheets2d.ts) to the catalog asset, text-level at the end of each scale's elevation array."""
import json, sys
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/weeping-willow/🔣️.json"
G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/author-61/"
t = open(A).read()
total = 0
for scale in sys.argv[1:]:
    shapes = json.load(open(G + "sheets2d.%s.json" % scale))
    i = t.index('"representations"'); i = t.index('"%s": {' % scale, i)
    g = t.index('"geometry"', i); v = t.index('"elevation": [', g); f = t.index('"front": [', v)
    end = t.rfind("}", v, f) + 1
    existing = t[v:end]
    new = [s for s in shapes if '"id": "%s"' % s["id"] not in existing]
    ins = "".join(",\n" + "\n".join("            " + ln for ln in json.dumps(s, ensure_ascii=False, indent=2).split("\n")) for s in new)
    t = t[:end] + ins + t[end:]
    total += len(new)
    print(scale, len(new), "added")
open(A, "w").write(t)
json.loads(t)
print("ok", total)
