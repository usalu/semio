import json,re,os,sys,collections
root="/Users/ueli/Documents/semio"
m=json.load(open(sys.argv[1]))
files=set(x["path"] for x in m["sources"]); entry=m["entry"]
rx=re.compile(r"""(?:from\s*|import\s*\(?\s*|require\(\s*|import\s+)["']([^"']+)["']""")
g={}
def res(base,spec):
    p=os.path.normpath(os.path.join(os.path.dirname(base),spec))
    for c in (p,p+".ts",p+".mjs",p+".js",p+".json",p+"/🟦️.ts",p+"/🟨️.mjs"):
        if c in files: return c
    return None
for f in files:
    try: s=open(os.path.join(root,f),errors="replace").read()
    except: continue
    g[f]=set(filter(None,(res(f,x) for x in rx.findall(s) if x.startswith("."))))
def reach(n,seen):
    st=[n]
    while st:
        x=st.pop()
        if x in seen: continue
        seen.add(x); st.extend(g.get(x,()))
    return seen
print("entry",entry[-60:],len(reach(entry,set())),"of",len(files))
sizes=[]
for d in sorted(g.get(entry,())):
    sizes.append((len(reach(d,set())),d))
for n,d in sorted(sizes,reverse=True)[:25]: print(n,d[-110:])
