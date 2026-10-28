import re,sys,json,os,collections
T=os.path.dirname(os.path.abspath(__file__))
ns={"__file__":T+"/scan4.py"}
exec(open(T+"/scan4.py").read().split("for x in d:")[0],ns)
T1=ns["T1"];LINEGATE=ns["LINEGATE"];d=ns["d"]
HDR=re.compile(r"(?<![\w.])(?:test|it|describe)((?:\.[A-Za-z]+(?:\([^)]*\))?)*)\s*\(")
COND=re.compile(r"win32|process\.platform|\bposix\b|platform\s*[!=]==|isWindows|Windows|available\b")
def analyze(x):
    un=collections.Counter();co=collections.Counter();gated=collections.Counter();exun=[];exco=[]
    for f in x["files"]:
        try: L=open(os.path.join(x["root"],f),encoding="utf8",errors="replace").read().split("\n")
        except: continue
        hdrs=[]  # (line, gated)
        for i,l in enumerate(L):
            ms=list(HDR.finditer(l))
            if ms: hdrs.append((i,bool(re.search(r"skipIf|\.skip\b|todo",ms[-1].group(1)))))
        hl=[h[0] for h in hdrs]
        import bisect
        short=f.split("/")[-3] if f.count("/")>2 else f
        for i,l in enumerate(L):
            for k,rx in T1.items():
                if not rx.search(l): continue
                if LINEGATE.search(l) and k in("symlink","chmod","pgroup"): continue
                j=bisect.bisect_right(hl,i)-1
                if j>=0 and hdrs[j][1]:
                    gated[k]+=1; continue
                lo=hl[j] if j>=0 else max(0,i-15); hi=hl[j+1] if j+1<len(hl) else len(L)
                text="\n".join(L[lo:hi])+"\n"+"\n".join(L[max(0,i-3):i+1])
                if COND.search(text):
                    co[k]+=1
                    if len(exco)<4: exco.append((k,short,i+1,l.strip()[:100]))
                else:
                    un[k]+=1
                    if len(exun)<6: exun.append((k,short,i+1,l.strip()[:100]))
    x["un"]=dict(un);x["co"]=dict(co);x["ga"]=dict(gated);x["exun"]=exun;x["exco"]=exco
for x in d: analyze(x)
json.dump(d,open(T+"/scan6.json","w"),ensure_ascii=False)
if __name__=="__main__":
    c=collections.Counter()
    for x in d: c["clean" if not x["un"] and not x["co"] and not x["ga"] else ("unconditioned" if x["un"] else "conditioned-or-gated-only")]+=1
    print(c)
