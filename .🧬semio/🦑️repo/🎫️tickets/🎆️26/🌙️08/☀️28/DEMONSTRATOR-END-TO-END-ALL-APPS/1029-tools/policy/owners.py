import os,re,subprocess,sys,json
R="/Users/ueli/Documents/semio"
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
IX=json.load(open(os.path.join(R,C,"1029-tools/policy/crateindex.json")))
def libpaths():
    out={}
    for name,d in IX.items():
        t=open(os.path.join(d,"Cargo.toml"),encoding="utf-8").read()
        for m in re.finditer(r'^\[(lib|\[test\]|\[bin\]|\[bench\]|\[example\])\][^\[]*?^path\s*=\s*"([^"]+)"',t,re.M|re.S):
            p=os.path.normpath(os.path.join(d,m.group(2)))
            out.setdefault(p,set()).add(name)
        for m in re.finditer(r'^\[\[(test|bin|bench|example)\]\]\n((?:[^\[\n].*\n)*)',t,re.M):
            pm=re.search(r'^path\s*=\s*"([^"]+)"',m.group(2),re.M)
            if pm:
                p=os.path.normpath(os.path.join(d,pm.group(1)))
                out.setdefault(p,set()).add(name)
    return out
LIBS=None
ROOTS=os.environ.get("OWNER_ROOTS","🧰️framework ✏️s 🌎️hub").split()
def includers(f):
    base=os.path.basename(f); parent=os.path.basename(os.path.dirname(f))
    pat=(parent+"/"+base) if base=="🦀️.rs" else base
    r=subprocess.run(["rg","-l","-F","--glob","*.rs","--glob","!.🧬semio/**",pat,*ROOTS],cwd=R,capture_output=True,text=True)
    res=set()
    for g in r.stdout.split("\n"):
        g=g.strip()
        if not g: continue
        gp=os.path.join(R,g)
        if gp==f: continue
        try: t=open(gp,encoding="utf-8").read()
        except: continue
        for m in re.finditer(r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]|include!\(\s*"([^"]+)"\s*\)',t):
            rel=m.group(1) or m.group(2)
            cand=os.path.normpath(os.path.join(os.path.dirname(gp),rel))
            if cand==f: res.add(gp)
            elif parent and os.path.basename(os.path.dirname(gp))!="" and False: pass
    return res
def owners(f,seen=None):
    global LIBS
    if LIBS is None: LIBS=libpaths()
    seen=seen if seen is not None else set()
    f=os.path.normpath(f)
    if f in seen: return set()
    seen.add(f)
    out=set(LIBS.get(f,()))
    for g in includers(f):
        out|=owners(g,seen)
    return out
if __name__=="__main__":
    for f in sys.argv[1:]:
        f=os.path.join(R,f) if not f.startswith("/") else f
        print(f[-70:],sorted(owners(f)))
