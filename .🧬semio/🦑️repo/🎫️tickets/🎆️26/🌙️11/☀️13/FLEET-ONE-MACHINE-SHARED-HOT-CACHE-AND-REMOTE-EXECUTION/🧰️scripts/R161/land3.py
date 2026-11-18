import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
FILES={"hs":L+"/🏠️home/🧬️schema/🔣️.json","ht":L+"/🏠️home/🧪️tests/🟦️.ts"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
def hs(t): return one(t,'            {"id": "bun", "path": "bun", "tools": ["bun"]},','            {"id": "repo-watch", "path": "repo-watch", "tools": []},\n            {"id": "bun", "path": "bun", "tools": ["bun"]},')
def ht(t):
    t=one(t,'.toEqual(["downloads","bun","go"]);','.toEqual(["downloads","repo-watch","bun","go"]);')
    return one(t,'[["downloads","skipped: no-source"],["bun","skipped: no-source"]]','[["downloads","skipped: no-source"],["repo-watch","skipped: no-source"],["bun","skipped: no-source"]]')
PATCH={"hs":hs,"ht":ht}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(p))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(p);write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(p,new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(p))
        if now==m["after"]: write(p,read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
