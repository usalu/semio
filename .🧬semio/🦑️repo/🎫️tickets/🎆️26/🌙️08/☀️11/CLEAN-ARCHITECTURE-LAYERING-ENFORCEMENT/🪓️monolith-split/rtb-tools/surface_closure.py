"""🧰️ Closure of the shared surface helpers inside the engine-surfaces law file. usage: surface_closure.py"""
import re, glob
from lib import *
p = glob.glob(EL + "/⚙️EngineCanvas/🧪️tests/*wgpu-engine-surfaces/🦀️.rs")[0]
text = read(p); cl = blank(text)
lines, cll = text.split("\n"), cl.split("\n")
items, depth, i, n = [], 0, 0, len(cll)
while i < n:
    l = cll[i]
    if depth == 0 and l and not l[0].isspace() and l[0] not in "}#/":
        s = i
        while s > 0 and lines[s - 1].lstrip().startswith(("#[", "///", "//")):
            s -= 1
        d, seen, j = 0, False, i
        while j < n:
            d += cll[j].count("{") - cll[j].count("}")
            seen = seen or "{" in cll[j]
            if (seen and d <= 0) or (not seen and cll[j].rstrip().endswith(";")):
                break
            j += 1
        m = re.match(r"(?:pub(?:\([^)]*\))? )?(?:unsafe )?(?:fn|struct|enum|const|static|type|trait|impl)\b[ <]*(\w+)?", l.strip())
        name = None
        mm = re.match(r"(?:pub(?:\([^)]*\))? )?(?:const |async |unsafe )*(?:fn|struct|enum|const|static|type) (\w+)", l.strip())
        if mm: name = mm.group(1)
        items.append(dict(s=s, e=j, head=l.strip()[:80], name=name, use=l.startswith("use "), tok=set(re.findall(r"[A-Za-z_]\w*", "\n".join(cll[s:j + 1])))))
        i = j + 1
        continue
    depth += l.count("{") - l.count("}")
    i += 1
by = {it["name"]: it for it in items if it["name"]}
seed = ["close_retained_surface_fixture", "paint_retained_surface_scene", "paint_retained_surface_in_window", "RetainedSurfacePaint"]
need = set(seed)
ch = True
while ch:
    ch = False
    for nm in list(need):
        for t in by[nm]["tok"]:
            if t in by and t not in need and not by[t]["head"].startswith("fn ") or (t in by and t not in need):
                need.add(t); ch = True
print("closure:", sorted(need))
for nm in sorted(need):
    it = by[nm]; print(f"  {nm}: lines {it['s']+1}-{it['e']+1} {it['head'][:60]}")
users = {nm: sorted(x["name"] for x in items if x["name"] and x["name"] not in need and nm in x["tok"]) for nm in need}
print({k: v for k, v in users.items() if v})
