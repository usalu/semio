import json,os,sys,glob
root=sys.argv[1]
def keyof(x,kind):
    if kind=="inserted": return json.dumps(x.get("row",{}).get("id", x.get("row")),sort_keys=True)+"@%s"%x.get("index") if False else json.dumps((x.get("row") or {}).get("id",x.get("row")))
    return json.dumps(x.get("id"))
def scan(v,path,out):
    if isinstance(v,dict):
        for kind in ("inserted","removed","moved","modified"):
            l=v.get(kind)
            if isinstance(l,list) and l:
                ks=[keyof(x,kind) if isinstance(x,dict) else None for x in l]
                if len(ks)!=len(set(ks)): out.append((path,kind,len(ks),len(set(ks))))
        for k,x in v.items(): scan(x,path+"/"+k,out)
    elif isinstance(v,list):
        for x in v: scan(x,path+"[]",out)
for art in sorted(glob.glob(os.path.join(root,"*"))):
    fx=glob.glob(art+"/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations")
    if not fx: continue
    bad=[]
    for p in glob.glob(fx[0]+"/*/*/🔣️.diff.json"):
        out=[];scan(json.load(open(p,encoding="utf-8")),"",out)
        if out: bad.append((p.split("🧬️mutations/")[1].replace("/🔣️.diff.json",""),out[0]))
    print(os.path.basename(art),len(bad),bad[:2])
