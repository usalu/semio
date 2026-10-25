import os,re,sys,json
sys.path.insert(0,os.path.dirname(__file__))
IMPORT = re.compile(r"""(?:\bimport\s*(?:type\s+)?(?:[\w$*{}\s,]+?\s*from\s*)?|\bexport\s*(?:type\s+)?(?:\*|\{[^}]*\})\s*from\s*|\bimport\s*\(\s*)(["'])(\.{1,2}/[^"'\\\r\n]+)\1""")
LITERAL = re.compile(r"""(["'])(\.{1,2}/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1""")
DECL=re.compile(r"\.d\.[cm]?ts$")
def ok(p): return os.path.isfile(p) and "node_modules" not in p and "/.cache/" not in p
def schemas(p):
    f=os.path.join(os.path.dirname(p),"🧬️schema")
    return [os.path.join(f,n) for n in sorted(os.listdir(f)) if n.endswith(".json")] if os.path.isdir(f) else []
def run(entry,imports=True,literals=True,sib=True,admitted=None):
    seen=set();pending=[os.path.normpath(entry)];parent={}
    while pending:
        p=pending.pop()
        if p in seen or not ok(p): continue
        if admitted is not None and p not in admitted: continue
        seen.add(p)
        if p.endswith(".json"): continue
        t=open(p,encoding="utf-8").read()
        cands=[]
        if sib: cands+=[(s,"schema") for s in schemas(p)]
        pats=([IMPORT] if imports else [])+([LITERAL] if literals else [])
        for pat in pats:
            for m in pat.finditer(t):
                sp=m.group(2)
                if DECL.search(sp): continue
                cands.append((os.path.normpath(os.path.join(os.path.dirname(p),sp)),"lit" if pat is LITERAL else "imp"))
        for c,k in cands:
            if os.path.isfile(c) and c not in seen:
                parent.setdefault(c,(p,k)); pending.append(c)
    return seen,parent
if __name__=="__main__":
    e=sys.argv[1]
    for name,kw in [("imports",dict(literals=False,sib=False)),("imports+lit",dict(sib=False)),("imports+sib",dict(literals=False)),("all",{})]:
        s,_=run(e,**kw);print(name,len(s))
