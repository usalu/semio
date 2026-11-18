import sys,os,json
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import patches as P
FILES=list(P.TARGETS)
MIRROR=os.environ.get("R154_MIRROR")
def loc(p): return p if MIRROR is None else MIRROR+p[len(P.ROOT):]
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    new={p:P.TARGETS[p](P.read(loc(p))) for p in FILES}
    os.makedirs(backup,exist_ok=True);manifest={}
    for i,p in enumerate(FILES):
        old=P.read(loc(p));P.write(backup+"/%d.bak"%i,old);manifest[p]={"i":i,"before":P.sha(old),"after":P.sha(new[p])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for p in FILES: P.write(loc(p),new[p])
    print("applied",len(FILES))
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for p,m in manifest.items():
        now=P.sha(P.read(loc(p)))
        if now==m["after"]: P.write(loc(p),P.read(backup+"/%d.bak"%m["i"]))
        elif now==m["before"]: pass
        else: bad.append(p)
    if bad: print("CONFLICT (peer edit since apply):",bad);sys.exit(4)
    print("restored")
