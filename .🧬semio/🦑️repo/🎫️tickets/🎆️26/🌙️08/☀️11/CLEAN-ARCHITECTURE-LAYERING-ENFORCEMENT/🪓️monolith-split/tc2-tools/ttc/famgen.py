"""tc2: builds the oracle families: parent + companions + the first-party crates of the same artifact workspace they depend on (transitively, normal edges for dependencies, all edges for the roots)."""
import json,os,sys,glob,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ttclib import *
mans=Manifests(Source(None)); mans.build_index(force=True)
src=Source(None)
SNAP="snap:/Users/ueli/.cache/tc2/tc2-snap/base"
P="semio-s-artifact-"
def ws_of(pkg):
    m=mans.manifest(pkg,src)
    sec=m["sections"].get("package",{})
    return os.path.normpath(os.path.join(ROOT+"/"+m["dir"],sec.get("workspace",'"."').strip().strip('"')))
def deps(pkg,roots=False):
    m=mans.manifest(pkg,src)
    out=[]
    for kind,name,pk,feats,opt,ren in mans.deps(m):
        if kind=="dependencies" or roots:
            out.append(pk)
    return out
def closure(roots,ws,cap=40):
    seen=[];stack=list(roots)
    first=True
    while stack and len(seen)<cap:
        p=stack.pop(0)
        if p in seen: continue
        if not p.startswith("semio-s-artifact") : continue
        m=mans.manifest(p,src)
        if m is None: continue
        if ws_of(p)!=ws: continue
        seen.append(p)
        stack+=deps(p,roots=(p in roots))
    return seen
specs={}
for f in sorted(glob.glob(os.path.dirname(os.path.abspath(__file__))+"/../specs/*.json")):
    s=json.load(open(f)); specs.setdefault(s["parent"],[]).append(s["name"])
fams={"transform":"draw-drawing-transform","style":"draw-drawing-style","structure":"draw-drawing-structure","model":"draw-drawing-model","scene":"draw-drawing-scene","edcmd":"draw-drawing-editor-commands","edint":"draw-drawing-editor-interaction","editor":"draw-drawing-editor","host":"draw-drawing-host","lcmd":"layout-layout-editor-commands","lframes":"layout-layout-mutations-frames","lcontent":"layout-layout-mutations-content","b2":"block-2d-editor","b5":"block-5d-editor"}
need=set()
for fname,short in fams.items():
    parent=P+short
    comps=specs[parent]
    ws=ws_of(parent)
    cl=closure([parent]+comps,ws)
    non=[c for c in cl if c not in comps]
    d=dict(name=fname,crates=cl,libNoTest=non,base=dict(rev=SNAP,crates=non))
    json.dump(d,open(os.path.dirname(os.path.abspath(__file__))+f"/families/{fname}.json","w"),ensure_ascii=False)
    need|=set(non)
    print(fname,len(cl),"crates")
open("/Users/ueli/.cache/tc2/need.txt","w").write(" ".join(sorted(need)))
