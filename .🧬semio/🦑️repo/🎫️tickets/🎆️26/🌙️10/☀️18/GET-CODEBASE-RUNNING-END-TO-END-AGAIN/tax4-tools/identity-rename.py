# 🔧 Renames mutation owner directories whose descriptor semanticKind differs from the directory slug to emoji+semanticKind, moving every same-named leaf directory of the artifact and re-pointing every bounded text reference (incl. the descriptor owner path). usage: python3 identity-rename.py <findings-s.json> [--apply]
import json,os,re,subprocess,sys,collections,unicodedata
rows=[v for v in json.load(open(sys.argv[1],encoding="utf8")) if v["code"]=="mutation-payload-schema-authority-invalid" and "Descriptor version" in v["message"]]
apply="--apply" in sys.argv
def split_emoji(name):
    m=re.match(r"^(.+?️)([a-z].*)$",unicodedata.normalize("NFC",name),re.S)
    return m.groups() if m else (None,None)
plan=[]
for v in rows:
    owner=os.path.dirname(v["path"]); j=json.load(open(v["path"],encoding="utf8"))
    name=os.path.basename(owner); e,slug=split_emoji(name)
    sk=j["semanticKind"]
    if e is None: print("SKIP odd",owner[-80:],name,sk); continue
    art="/".join(owner.split("/")[:owner.split("/").index("🗿️artifacts")+2]) if "🗿️artifacts" in owner else None
    plan.append({"owner":owner,"name":name,"new":f"{e}{sk}","artifact":art,"plugin":"/".join(owner.split("/")[:3])})
byart=collections.defaultdict(list)
for p in plan: byart[p["artifact"] or p["plugin"]].append(p)
files_cache={}
def files_of(scope):
    if scope not in files_cache:
        fs=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",scope],capture_output=True,text=True).stdout.split("\n") if f.endswith((".rs",".ts",".toml",".py",".json",".feature",".md",".proto",".graphql")) and "node_modules" not in f and "/target/" not in f and os.path.isfile(f)]
        files_cache[scope]=fs
    return files_cache[scope]
moves=[];edits={}
for scope,items in byart.items():
    kinds=collections.defaultdict(set)
    for f in files_of(scope):
        if f.endswith("/🔣️.json") and "/🧬️mutations/" in f:
            parts=f.split("/🧬️mutations/")[-1].split("/")
            if len(parts)==2:
                try: kinds[parts[0]].add(json.load(open(f,encoding="utf8")).get("semanticKind"))
                except Exception: pass
    names={}
    for p in items:
        if p["name"] in names and names[p["name"]]!=p["new"]: print("CONFLICT",p["name"]); continue
        names[p["name"]]=p["new"]
    for name,new in names.items():
        ks=kinds.get(name,set())
        if len(ks)>1: print("MULTI-KIND",scope[-60:],name,ks); continue
    pat={n:re.compile(r'(?<![^/"\s(])'+re.escape(n)+r'(?=[/"\s)]|$)') for n in names}
    dirs=set()
    for f in files_of(scope):
        segs=f.split("/")
        for i,s in enumerate(segs[:-1]):
            if s in names and i>0 and segs[i-1]=="🧬️mutations": dirs.add("/".join(segs[:i+1]))
    for d in sorted(dirs): moves.append((d,os.path.join(os.path.dirname(d),names[os.path.basename(d)])))
    for f in files_of(scope):
        try: t=open(f,encoding="utf8").read()
        except Exception: continue
        n=t
        for name,new in names.items():
            if name in n: n=pat[name].sub(new,n)
        if n!=t: edits[f]=n
print("artifacts",len(byart),"renames",len(plan),"dir moves",len(moves),"files to edit",len(edits))
for a,b in moves[:6]: print("  ",a[-90:],"->",os.path.basename(b))
bad=[(a,b) for a,b in moves if os.path.exists(b)]
print("occupied targets",len(bad))
if apply:
    for f,t in edits.items(): open(f,"w",encoding="utf8").write(t)
    moves.sort(key=lambda m:-m[0].count("/"))
    done=0
    for a,b in moves:
        if os.path.isdir(a): os.rename(a,b); done+=1
    print("applied; dirs moved",done)
