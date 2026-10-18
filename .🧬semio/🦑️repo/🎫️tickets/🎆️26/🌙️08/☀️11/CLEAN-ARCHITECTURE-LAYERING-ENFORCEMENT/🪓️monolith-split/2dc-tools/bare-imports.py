"""🔗️ Adds the direct canonical EngineHandles import to child modules that relied on `use super::*` for it."""
import json, sys
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
files = json.load(open(F + "/2dc-tools/bare-files.json"))
LINE = "use semio_framework_2d_compute::EngineHandles;"
done = []
for p in files:
    t = open("/Users/ueli/Documents/semio/" + p, encoding="utf8").read()
    if LINE in t: print("HAS", p[-60:]); continue
    lines = t.split("\n")
    idx = [i for i, l in enumerate(lines) if l == "use super::*;"]
    if not idx: print("NO-SUPER-GLOB", p); continue
    lines.insert(idx[0] + 1, LINE)
    open("/Users/ueli/Documents/semio/" + p, "w", encoding="utf8").write("\n".join(lines))
    done.append(p)
json.dump(done, open(F + "/2dc-tools/journal-bare.json", "w"), ensure_ascii=False)
print("edited", len(done))
