import re,sys,json,os
T=os.path.dirname(os.path.abspath(__file__))
ns={"__file__":T+"/scan4.py"}
exec(open(T+"/scan4.py").read().split("for x in d:")[0],ns)
T1=ns["T1"];d=ns["d"]
HDR=re.compile(r"\b(?:test|it|describe)(?:\.[A-Za-z]+(?:\([^)]*\))?)*\(\s*[\"'`]([^\"'`]{0,90})")
want=sys.argv[1:]
for x in d:
    if x["id"] not in want: continue
    print("##",x["table"],x["id"])
    for f in x["files"]:
        p=os.path.join(x["root"],f)
        L=open(p,encoding="utf8",errors="replace").read().split("\n")
        hdr=None;shown=set()
        for i,l in enumerate(L,1):
            m=list(HDR.finditer(l))
            if m: hdr=(i,m[-1].group(0)[:120])
            for k,rx in T1.items():
                if rx.search(l):
                    key=(hdr[0] if hdr else 0,k)
                    if key in shown: continue
                    shown.add(key)
                    print(f"  {f.split('/')[-3] if f.count('/')>2 else f}:{i} [{k}] in {hdr[1] if hdr else '-'!r} :: {l.strip()[:100]}")
