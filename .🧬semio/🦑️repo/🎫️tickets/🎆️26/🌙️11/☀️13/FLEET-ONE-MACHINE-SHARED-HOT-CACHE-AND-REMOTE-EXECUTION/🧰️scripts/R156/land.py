import sys,os,json,hashlib
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import patch_load
ROOT="/Users/ueli/Documents/semio"
B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
HERE=os.path.dirname(os.path.abspath(__file__))
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R156"
MIRROR=os.environ.get("R156_MIRROR")
def loc(p): return p if MIRROR is None or not p.startswith(ROOT) else MIRROR+p[len(ROOT):]
def read(p): return open(loc(p),encoding="utf8").read()
def write(p,t):
    os.makedirs(os.path.dirname(loc(p)),exist_ok=True);open(loc(p),"w",encoding="utf8").write(t)
def exists(p): return os.path.exists(loc(p))
def sha(t): return hashlib.sha256(t.encode("utf8")).hexdigest()
G=B+"/🕸️graph"
ROWS=[
 ("load",G+"/📥️load/🟦️.ts",None),
 ("graph-schema",G+"/🧬️schema/🔣️.json","graph-schema.json"),
 ("authority",B+"/🧬️schema/📥️authority.json","authority.json"),
 ("retry",G+"/🔁retry/🟨️.mjs","retry.mjs"),
 ("retry-d",G+"/🔁retry/🟨️.d.mts","retry.d.mts"),
 ("retry-schema",G+"/🔁retry/🧬️schema/🔣️.json","retry-schema.json"),
 ("repair-rounds",B+"/🧪️tests/🕸️graph/🩹️repair-rounds/🟦️.ts","repair-rounds.ts"),
 ("stale-facts",B+"/🧪️tests/🕸️graph/🧊️stale-facts/🟦️.ts","stale-facts.ts"),
 ("churn",B+"/🧪️tests/🕸️graph/🌊️seal-under-churn/🟦️.ts","churn.ts"),
 ("tooling-edit",G+"/📥️inference/🔖️stamps/🧪️tests/🔧️tooling-edit/🟦️.ts","tooling-edit.ts"),
]
EXPECT=json.load(open(SC+"/expect.json")) if os.path.exists(SC+"/expect.json") else {}
def target(name,live,new):
    if name=="load": return patch_load.patch(live)
    return open(HERE+"/new/"+new,encoding="utf8").read()
def main():
    cmd,backup=sys.argv[1],sys.argv[2]
    if cmd=="apply":
        os.makedirs(backup,exist_ok=True);manifest={};plan={}
        for i,(name,path,new) in enumerate(ROWS):
            old=read(path) if exists(path) else None
            if old is not None and name not in("load","stale-facts","repair-rounds") and EXPECT.get(name) and sha(old)!=EXPECT[name]: raise SystemExit("live file changed since staging: "+name)
            if old is not None and name in("stale-facts","repair-rounds") and EXPECT.get(name) and sha(old)!=EXPECT[name]: raise SystemExit("live test changed since staging: "+name)
            plan[name]=target(name,old,new)
        for i,(name,path,new) in enumerate(ROWS):
            old=read(path) if exists(path) else None
            manifest[name]={"i":i,"path":path,"existed":old is not None,"before":sha(old) if old is not None else None,"after":sha(plan[name])}
            if old is not None: write(backup+"/%d.bak"%i,old)
        json.dump(manifest,open(backup+"/manifest.json","w"))
        for name,path,new in ROWS: write(path,plan[name])
        print("applied",len(ROWS))
    elif cmd=="restore":
        manifest=json.load(open(backup+"/manifest.json"));bad=[]
        for name,m in manifest.items():
            path=m["path"];now=sha(read(path)) if exists(path) else None
            if now==m["after"]:
                if m["existed"]: write(path,read(backup+"/%d.bak"%m["i"]))
                else: os.remove(loc(path))
            elif now==m["before"]: pass
            else: bad.append(name)
        for d in (B+"/🧪️tests/🕸️graph/🌊️seal-under-churn",G+"/📥️inference/🔖️stamps/🧪️tests/🔧️tooling-edit"):
            try: os.rmdir(loc(d))
            except OSError: pass
        if bad: print("CONFLICT (peer edit since apply):",bad);sys.exit(4)
        print("restored")

if __name__=="__main__": main()
