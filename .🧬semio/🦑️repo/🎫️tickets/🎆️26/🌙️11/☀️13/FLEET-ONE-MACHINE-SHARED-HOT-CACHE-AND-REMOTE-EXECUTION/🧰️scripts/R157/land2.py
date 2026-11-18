import sys,os,json,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
MIRROR=os.environ.get("R157_MIRROR")
def loc(p): return p if MIRROR is None else MIRROR+"/"+os.path.basename(os.path.dirname(p))+"-"+os.path.basename(p)
FILES={"lab":L+"/🟦️.ts"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:80])
    return text.replace(old,new)
def lab(t):
    t=one(t,'sliceLabSeedHomeV1,sliceLabShimsV1,','sliceLabSeedHomeV1,sliceLabSeedToolsV1,sliceLabShimsV1,')
    t=one(t,'sliceLabSeedHomeV1(sliceLabHomeSeedsV1(tools,liveHome,plan.home))','sliceLabSeedHomeV1(sliceLabHomeSeedsV1(sliceLabSeedToolsV1(tools,options.stages),liveHome,plan.home))')
    return t
PATCH={"lab":lab}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(loc(p)))
    for i,(k,p) in enumerate(FILES.items()):
        old=read(loc(p));write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,p in FILES.items(): write(loc(p),new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(loc(p)))
        if now==m["after"]: write(loc(p),read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
