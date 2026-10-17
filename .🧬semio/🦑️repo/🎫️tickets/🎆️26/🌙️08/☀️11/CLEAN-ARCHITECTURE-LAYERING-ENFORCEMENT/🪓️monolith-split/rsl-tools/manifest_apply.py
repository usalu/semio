"""📦️ F-04(a) manifest edits: adds `<crate> = { path = "<relative>" }` to the dependency table of the anchor (the facade, plugin or host dependency line), right after that line, for every owner manifest of rsl-manifest-plan.json. Dry run unless --write. Writes rsl-edges.json (consumer package, dependency package, dev flag) for the lock propagation."""
import json, os, re, sys, shutil
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from facade_lib import *
PRE="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rsl-pre/"
write="--write" in sys.argv
mp=json.load(open(G+"/rsl-manifest-plan.json"))["manifests"]
edges=[];edited=0
for m,row in sorted(mp.items()):
    text=open(m,encoding="utf8").read()
    lines=text.split("\n")
    ents=dep_entries(text)
    pname=re.search(r'^name\s*=\s*"([^"]+)"',text,re.M).group(1)
    inserts=[]
    for pkg,info in sorted(row.items()):
        secs={a[0] for a in info["anchors"]}
        sec="dependencies" if "dependencies" in secs else sorted(secs)[0]
        prefs=info["prefs"]
        apk={"semio_framework":"semio-framework","semio_framework_plugin":"semio-framework-plugin","semio_framework_os_host":"semio-framework-os-host"}
        cand=[e for e in ents if e["section"]==sec and e["pkg"] in [apk[p] for p in prefs]]
        if not cand: print("NO ANCHOR LINE",m,pkg); sys.exit(2)
        if any(e["pkg"]==pkg for e in ents if e["section"]==sec) : print("ALREADY",m,pkg); continue
        anchor=cand[0]
        rel=os.path.relpath(PKGDIR[pkg],os.path.dirname(m))
        assert os.path.exists(os.path.join(os.path.dirname(m),rel,"Cargo.toml")),(m,rel)
        inserts.append((anchor["line"],anchor["indent"]+f'{pkg} = {{ path = "{rel}" }}'))
        edges.append(dict(consumer=pname,dep=pkg,dev=(sec=="dev-dependencies"),manifest=m))
    if not inserts: continue
    for ln,new in sorted(inserts,key=lambda x:(-x[0],x[1]),reverse=False)[::-1]:
        pass
    byline={}
    for ln,new in inserts: byline.setdefault(ln,[]).append(new)
    for ln in sorted(byline,reverse=True):
        lines[ln+1:ln+1]=sorted(byline[ln])
    out="\n".join(lines)
    edited+=1
    if write:
        cur=open(m,encoding="utf8").read(); assert cur==text,"concurrent edit "+m
        dest=PRE+m
        if not os.path.exists(dest): os.makedirs(os.path.dirname(dest),exist_ok=True); shutil.copy2(m,dest)
        open(m,"w",encoding="utf8").write(out)
json.dump(edges,open(G+"/rsl-edges.json","w"),ensure_ascii=False)
print(("WROTE" if write else "DRY"),edited,"manifests",len(edges),"edges")
