import re,sys,json,os,collections
T=os.path.dirname(os.path.abspath(__file__))
src=open(T+"/scan4.py").read()
ns={"__file__":T+"/scan4.py"}
exec(src.split("for x in d:")[0],ns)
T1=ns["T1"];TEST=ns["TEST"];GATE=ns["GATE"];LINEGATE=ns["LINEGATE"];d=ns["d"]
COND=re.compile(r"win32|process\.platform|\bposix\b|platform\s*[!=]==|isWindows|windows|Windows")
for x in d:
    kinds_un=collections.Counter();kinds_cond=collections.Counter();exun=[];exco=[]
    for f in x["files"]:
        p=os.path.join(x["root"],f)
        try: lines=open(p,encoding="utf8",errors="replace").read().split("\n")
        except: continue
        starts=[i for i,l in enumerate(lines) if TEST.match(l)]+[len(lines)]
        def block(i):
            prev=[s for s in starts if s<=i and s<len(lines)]
            b0=prev[-1] if prev else None
            nxt=[s for s in starts if s>i]
            b1=nxt[0] if nxt else len(lines)
            return b0,b1
        for i,l in enumerate(lines):
            for k,rx in T1.items():
                if not rx.search(l): continue
                b0,b1=block(i)
                hdr=lines[b0] if b0 is not None else ""
                if hdr and GATE.search(hdr): continue
                if LINEGATE.search(l) and k in("symlink","chmod","pgroup"): continue
                if b0 is None: text="\n".join(lines[max(0,i-15):i+4])
                else: text="\n".join(lines[b0:b1])+"\n"+"\n".join(lines[max(0,i-3):i+1])
                short=f.split("/")[-3] if f.count("/")>2 else f
                if COND.search(text):
                    kinds_cond[k]+=1
                    if len(exco)<3: exco.append((k,short,i+1,l.strip()[:110]))
                else:
                    kinds_un[k]+=1
                    if len(exun)<3: exun.append((k,short,i+1,l.strip()[:110]))
    x["un"]=dict(kinds_un);x["co"]=dict(kinds_cond);x["exun"]=exun;x["exco"]=exco
json.dump(d,open(T+"/scan5.json","w"),ensure_ascii=False)
if __name__=="__main__":
    c=collections.Counter()
    for x in d: c["clean" if not x["un"] and not x["co"] else ("unconditioned" if x["un"] else "conditioned-only")]+=1
    print(c)
    for x in d:
        if x["co"] and not x["un"]: print(" co-only",x["table"][0]+"/"+x["id"],x["co"])
