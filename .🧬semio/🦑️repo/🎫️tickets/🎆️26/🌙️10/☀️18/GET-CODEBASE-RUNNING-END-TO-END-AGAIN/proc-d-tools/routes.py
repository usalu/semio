import os,re,sys,json
root="/Users/ueli/Documents/semio"
mod=root+"/🧰️framework/🔨️modules/🏃️process"
tests=[l.strip()[2:] for l in open(sys.argv[1],encoding="utf8")]
tests=[t for t in tests if True]
script=open(mod+"/📜️script.ts",encoding="utf8").read()
# source strings in script.ts
cands=set(re.findall(r'"([^"]+\.ts)"',script))
routed={}
def add(p,why):
    p=os.path.normpath(p)
    if os.path.exists(p) and p not in routed: routed[p]=why
for c in cands:
    add(os.path.join(mod,c),"script.ts")
    add(os.path.join(mod,"..",c),"script.ts")
    add(os.path.join(root+"/🧰️framework",c),"script.ts")
# facet loop for native-artifacts
for f in ["🧾️receipt","🎰️jobserver","🎟️admission","📈️telemetry","🗃️stage-cache","🧩️component-pass"]:
    add(mod+f"/📦️artifacts/🏗️native-build/{f}/🧪️tests/🟦️.ts","script.ts")
# imports transitive
imp=re.compile(r'(?:from|import)\s*\(?\s*["\']([^"\']+\.ts)["\']')
queue=list(routed)
while queue:
    f=queue.pop()
    try: s=open(f,encoding="utf8").read()
    except: continue
    for m in imp.findall(s):
        if m.startswith("."):
            p=os.path.normpath(os.path.join(os.path.dirname(f),m))
            if os.path.exists(p) and p not in routed:
                routed[p]="import of "+os.path.relpath(f,mod); queue.append(p)
out=[]
for t in tests:
    p=os.path.normpath(mod+"/"+t)
    out.append((t,routed.get(p)))
un=[t for t,w in out if not w]
print(len(tests),"mention files;",len(un),"unrouted",file=sys.stderr)
for t in un: print(t)
