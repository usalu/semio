"""tc2: every moved test file is reached by exactly one crate (module-tree walk from every crate lib root of the four artifact workspaces)."""
import os,sys,json,glob,collections
sys.path.insert(0,os.path.dirname(__file__)); sys.path.insert(0,os.path.dirname(__file__)+"/ttc")
import modtree
from ttclib import *
R=ROOT+"/"
arts=["✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing","✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout","✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d","✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d"]
reach=collections.defaultdict(set)
libs=0
for a in arts:
    ws=parse_toml(open(R+a+"/Cargo.toml").read())
    m=ws.get("workspace",{}).get("members","")
    import re
    for mem in re.findall(r'"([^"]+)"',m):
        mp=R+a+"/"+mem+"/Cargo.toml"
        if not os.path.exists(mp): print("MISSING MEMBER",a,mem); continue
        sec=parse_toml(open(mp).read())
        lp=sec.get("lib",{}).get("path","").strip('"')
        lib=os.path.normpath(os.path.dirname(mp)+"/"+lp) if lp else None
        if not lib or not os.path.exists(lib): continue
        libs+=1
        for md in modtree.build(lib):
            if not md.inline and md.exists: reach[os.path.normpath(md.file)].add(sec["package"]["name"].strip('"'))
moved=[]
for f in glob.glob(os.path.dirname(__file__)+"/specs/*.json"):
    s=json.load(open(f)); moved+=[(R+x,s["name"]) for x in s["files"]]
bad=0
for p,comp in moved:
    crates=reach.get(os.path.normpath(p),set())
    if len(crates)!=1 or comp not in crates:
        bad+=1; print("DUP/WRONG",p[-70:],crates)
multi=[(p,c) for p,c in reach.items() if len(c)>1 and ("🧪️tests" in p)]
print("lib roots",libs,"moved files",len(moved),"bad",bad,"test files reached by >1 crate (all)",len(multi))
for p,c in multi[:10]: print("  ",p.split("subsets/")[-1][-70:],sorted(c))
