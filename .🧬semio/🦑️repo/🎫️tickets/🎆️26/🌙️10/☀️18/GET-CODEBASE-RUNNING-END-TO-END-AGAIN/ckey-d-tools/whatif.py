import json,base64,re,os,sys,collections
SNAP=".cache/b/p/m/bazel/workspaces/s2/784c1946a50a20ac/implementation.json"
snap=json.load(open(SNAP));root=snap["root"];files={f["path"]:f for f in snap["files"]}
LIT=re.compile(r"""(["'])(\.{1,2}/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1""")
DECL=re.compile(r"\.d\.[cm]?ts$")
TYPEONLY=re.compile(r"""(?:^|[;\n])\s*(?:import|export)\s+type\b[^;\n]*?from\s*(["'])\.{1,2}/[^"'\n]+\1|typeof\s+import\(\s*(["'])\.{1,2}/[^"'\n]+\2\s*\)|import\(\s*(["'])\.{1,2}/[^"'\n]+\3\s*\)\s*\.\s*[A-Z]\w+(?=[\s,;>)\]])""")
schemas=collections.defaultdict(list)
for p in files:
    parts=p.split("/")
    if "🧬️schema" in parts and p.endswith(".json"):
        i=parts.index("🧬️schema");schemas["/".join(parts[:i])].append(p)
text={p:base64.b64decode(f["base64"]).decode("utf-8","replace") for p,f in files.items() if not p.endswith(".json")}
def N(seeds,strip_types=False,cut=()):
    seen=set();pending=list(seeds)
    while pending:
        p=pending.pop()
        if p in seen or p not in files:continue
        seen.add(p)
        if p.endswith(".json"):continue
        def add(t):
            if t in files and t not in seen and not any(c in p and d in t for c,d in cut):pending.append(t)
        for s in schemas.get(os.path.dirname(p),[]):add(s)
        for spec,t in files[p]["imports"].items():
            if not t.startswith("installed:"):add(t)
        t=text[p]
        if strip_types:t=TYPEONLY.sub(" ",t)
        for m in LIT.finditer(t):
            if DECL.search(m.group(2)):continue
            add(os.path.relpath(os.path.normpath(os.path.join(root,os.path.dirname(p),m.group(2))),root))
    return seen
V=[os.path.relpath(l,root) for l in open("/private/tmp/claude-501/bun.txt").read().split("\n") if l and "🎬️executor" not in l]
seeds=[s for s in V if s in files]
print("base",len(N(seeds)))
print("strip type-only/typeof-import literals",len(N(seeds,True)))
cuts=[("📥️caller/🟦️.ts","🕸️graph"),("📥️caller/🟦️.ts","🧭️"),("📥️caller/🟦️.ts","🏃️process/🦀️cargo")]
print("+caller dynamic imports cut",len(N(seeds,True,cuts)))
cuts2=cuts+[("🧾️closures/🟦️.ts",""),("📥️inputs/💾️storage/🟨️.mjs","")]
print("+closures/inputs storage url lists cut",len(N(seeds,True,cuts2)))
cuts3=cuts2+[("📥️caller/📤️orchestration/🟨️.mjs","🔌️backends")]
n3=N(seeds,True,cuts3);print("+orchestration backends cut",len(n3))
rest=[p for p in sorted(n3) if p not in set(seeds)]
print(len(rest),"beyond value closure; sample:",[r[-70:] for r in rest[:12]])
def N2(seeds,literals=False,siblings=True):
    seen=set();pending=list(seeds)
    while pending:
        p=pending.pop()
        if p in seen or p not in files:continue
        seen.add(p)
        if p.endswith(".json"):continue
        if siblings:
            for s in schemas.get(os.path.dirname(p),[]):
                if s not in seen:pending.append(s)
        for spec,t in files[p]["imports"].items():
            if not t.startswith("installed:") and t not in seen:pending.append(t)
        if literals:
            for m in LIT.finditer(text[p]):
                if DECL.search(m.group(2)):continue
                t=os.path.relpath(os.path.normpath(os.path.join(root,os.path.dirname(p),m.group(2))),root)
                if t in files and t not in seen:pending.append(t)
    return seen
print("no literals, siblings",len(N2(seeds)),"no literals no siblings",len(N2(seeds,False,False)),"seeds",len(seeds))
print("--- first-level map edges leaving the static value closure")
Vs=set(seeds)
out=collections.Counter()
ex={}
for p in seeds:
    if p.endswith(".json") or p not in files:continue
    for spec,t in files[p]["imports"].items():
        if not t.startswith("installed:") and t not in Vs:
            out[p[-70:]]+=1;ex.setdefault(p[-70:],(spec[-50:],t[-60:]))
for p,v in out.most_common(30):print(v,p,ex[p])
print("--- progressive cuts of map edges (no literals)")
def N3(seeds,cut):
    seen=set();pending=list(seeds)
    while pending:
        p=pending.pop()
        if p in seen or p not in files:continue
        seen.add(p)
        if p.endswith(".json"):continue
        for s in schemas.get(os.path.dirname(p),[]):
            if s not in seen:pending.append(s)
        for spec,t in files[p]["imports"].items():
            if t.startswith("installed:") or t in seen:continue
            if any(a in p and b in t for a,b in cut):continue
            pending.append(t)
    return seen
cuts=[("📥️caller/🟦️.ts","🕸️graph"),("📥️caller/🟦️.ts","🧭️")]
print("cut caller->graph/invocation/selection",len(N3(seeds,cuts)))
c2=cuts+[("🧹️","")] 
c2=cuts+[("📥️inputs/💾️storage/🟨️.mjs","🧹️cache-gc")]
print("+ inputs storage -> cache-gc",len(N3(seeds,c2)))
c3=c2+[("📥️caller/🛰️service","📤️child")]
print("+ service->child",len(N3(seeds,c3)))
c4=c3+[("🧾️closures/🟦️.ts","👷️read.mjs")]
print("+ closures->read.mjs",len(N3(seeds,c4)))
c5=c4+[("⚙️runtime/🧭️selection","🗣️messages")]
print("+ runtime selection->messages",len(N3(seeds,c5)))
c6=c5+[("📥️opening/🟦️.ts","♻️reset")]
print("+ opening->recovery",len(N3(seeds,c6)))
c7=c6+[("🎫️gate/🟦️.ts","🤝️handover")]
print("+ gate->handover",len(N3(seeds,c7)))
c8=c7+[("🔒️leases/🟦️.ts","📦️metadata")]
n=N3(seeds,c8);print("+ leases->metadata",len(n))
print("--- debug leftovers after all cuts")
def N4(seeds,cut):
    seen=set();pending=[(s,None) for s in seeds];why={}
    while pending:
        p,par=pending.pop()
        if p in seen or p not in files:continue
        seen.add(p);why[p]=par
        if p.endswith(".json"):continue
        for s in schemas.get(os.path.dirname(p),[]):
            if s not in seen:pending.append((s,p))
        for spec,t in files[p]["imports"].items():
            if t.startswith("installed:") or t in seen:continue
            if any(a in p and b in t for a,b in cut):continue
            pending.append((t,p))
    return seen,why
seen,why=N4(seeds,c8)
sib=set(s for p in seeds for s in schemas.get(os.path.dirname(p),[]))
left=[p for p in seen if p not in set(seeds) and p not in sib]
print(len(left))
cnt=collections.Counter(why[p][-75:] for p in left if why[p])
for k,v in cnt.most_common(8):print(v,k)
