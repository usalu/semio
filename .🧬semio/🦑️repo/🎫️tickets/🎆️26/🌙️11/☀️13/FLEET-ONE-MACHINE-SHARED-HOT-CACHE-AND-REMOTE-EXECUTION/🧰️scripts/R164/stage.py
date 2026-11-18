import os,sys,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import patches as X
OUT=os.environ["R164_SCRATCH"]+"/stage"
def absolutize(text,origin_dir):
    def r(m):
        s=m.group(2)
        return m.group(1)+os.path.normpath(os.path.join(origin_dir,s))+m.group(3) if s.startswith("../") or s.startswith("./") else m.group(0)
    return re.sub(r'''((?:from\s*|import\s*\(\s*|import\s*)["'])(\.{1,2}/[^"']+)(["'])''',r,text)
KEYS=["nodes","pool","schema","status","runner","child","driver","publisher"]
def staged_path(k): return OUT+"/"+k+"/"+os.path.basename(X.FILES[k])
swap={X.FILES[k]:staged_path(k) for k in KEYS if k!="schema"}
swap[X.P+"/🧬️schema/🔣️.json"]=staged_path("schema")
for k in KEYS:
    t=X.PATCH[k](X.read(X.FILES[k]))
    if k!="schema":
        t=absolutize(t,os.path.dirname(X.FILES[k]))
        d=os.path.dirname(X.FILES[k])
        t=re.sub(r'new URL\("(\.{1,2}/[^"]+)", import\.meta\.url\)',lambda m:'new URL("file://'+os.path.normpath(os.path.join(d,m.group(1)))+'")',t)
    X.write(staged_path(k),t)
for k in KEYS:
    if k=="schema":continue
    t=X.read(staged_path(k))
    for a,b in swap.items(): t=t.replace('"'+a+'"','"'+b+'"').replace("'"+a+"'","'"+b+"'").replace('"file://'+a+'"','"file://'+b+'"')
    X.write(staged_path(k),t)
TESTS={"admission":X.N+"/🧪️tests/🤝️pool-admission/🟦️.ts","turn":X.P+"/🧪️tests/🧭️turn/🟦️.ts","selected":X.N+"/🧪️tests/🤝️selected-fleet/🟦️.ts"}
for k,f in TESTS.items():
    t=X.read(f)
    if k=="selected": t=X.PATCH["selected"](t)
    t=absolutize(t,os.path.dirname(f))
    for a,b in swap.items(): t=t.replace('"'+a+'"','"'+b+'"')
    if k=="selected": t=t.replace('new URL("../../🟨️.mjs",import.meta.url)','new URL("file://'+staged_path("nodes")+'")')
    if k=="turn": t=t.replace('join(import.meta.dir,"..","🧫️fixtures","🧮️driver.mjs")','"'+X.P+'/🧪️tests/🧫️fixtures/🧮️driver.mjs"')
    X.write(OUT+"/"+k+".test.ts",t)
print("staged")

import json
EXISTING=json.load(open(os.path.dirname(os.path.abspath(__file__))+"/tests.json"))
for i,f in enumerate(EXISTING):
    path=X.LIVE+"/"+f if not f.startswith("/") else f
    d=os.path.dirname(path);tx=X.read(path)
    tx=tx.replace("import.meta.dir",json.dumps(d))
    tx=absolutize(tx,d)
    tx=re.sub(r'new URL\("(\.{1,2}/[^"]+)",\s*import\.meta\.url\)',lambda m:'new URL("file://'+os.path.normpath(os.path.join(d,m.group(1)))+('/' if os.path.isdir(os.path.normpath(os.path.join(d,m.group(1)))) else '')+'")',tx)
    if "📥️take" in path: tx=tx.replace("import.meta.url",json.dumps("file://"+d+"/test.ts"))
    for a,b in swap.items(): tx=tx.replace('"'+a+'"','"'+b+'"').replace('"file://'+a+'"','"file://'+b+'"')
    if path.endswith("🤝️selected-fleet/🟦️.ts"): tx=X.PATCH["selected"](tx) if False else tx
    X.write(OUT+"/ex/%02d.test.ts"%i,tx)
print("existing staged",len(EXISTING))
