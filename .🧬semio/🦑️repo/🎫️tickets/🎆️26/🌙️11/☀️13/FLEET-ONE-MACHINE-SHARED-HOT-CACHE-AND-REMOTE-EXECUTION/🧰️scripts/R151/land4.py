import sys,os,json,shutil
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
import patches4 as P
FILES=[P.NODES]
MIRROR=os.environ.get("R151_MIRROR")
def loc(p): return p if MIRROR is None else MIRROR+p[len(ROOT):]
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    new={p:P.TARGETS4[p](read(loc(p))) for p in FILES}
    os.makedirs(backup,exist_ok=True);manifest={}
    for i,p in enumerate(FILES):
        old=read(loc(p));write(backup+"/%d.bak"%i,old);manifest[p]={"i":i,"before":sha(old),"after":sha(new[p])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for p in FILES: write(loc(p),new[p])
    print("applied",len(FILES))
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for p,m in manifest.items():
        now=sha(read(loc(p)))
        if now==m["after"]: write(loc(p),read(backup+"/%d.bak"%m["i"]))
        elif now==m["before"]: pass
        else: bad.append(p)
    if bad: print("CONFLICT (peer edit since apply):",bad);sys.exit(4)
    print("restored")
