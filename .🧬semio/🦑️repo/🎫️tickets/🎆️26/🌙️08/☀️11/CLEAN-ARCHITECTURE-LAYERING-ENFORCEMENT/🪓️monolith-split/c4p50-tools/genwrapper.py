"""📦️ usage: genwrapper.py <exportsmap.json> <wrapper.ts> <header-docstring-line>: rewrites a package entry as explicit per-owner export lists (no export *)."""
import json,sys,os
m=json.load(open(sys.argv[1],encoding="utf8"))
wp=sys.argv[2]; head=open(wp,encoding="utf8").read().split("\n")[0]
out=[head]
def rel(f): return os.path.relpath(f,os.path.dirname(wp)).replace(os.sep,"/")
for e in m:
    r=rel(e["file"])
    if e["values"]: out.append("export { %s } from \"%s\";"%(", ".join(e["values"]),r))
for e in m:
    r=rel(e["file"])
    if e["types"]: out.append("export type { %s } from \"%s\";"%(", ".join(e["types"]),r))
open(wp,"w",encoding="utf8").write("\n".join(out)+"\n")
print("wrote",wp.split("/")[-4:], len(out))
