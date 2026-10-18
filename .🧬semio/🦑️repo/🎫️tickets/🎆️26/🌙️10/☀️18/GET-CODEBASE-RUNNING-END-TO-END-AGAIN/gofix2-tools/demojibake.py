#!/usr/bin/env python3
import sys,re
cp850=set(bytes(range(0x80,0x100)).decode("cp850"))
def fix(s):
    out=[];i=0;changes=[]
    n=len(s)
    while i<n:
        if s[i] in cp850:
            j=i
            while j<n and s[j] in cp850: j+=1
            run=s[i:j]
            try:
                dec=run.encode("cp850").decode("utf-8")
                if any(ord(c)>0xFF for c in dec):
                    out.append(dec); changes.append((run,dec)); i=j; continue
            except Exception: pass
            out.append(run); i=j; continue
        out.append(s[i]); i+=1
    return "".join(out),changes
if __name__=="__main__":
    dry="--dry" in sys.argv
    for p in [a for a in sys.argv[1:] if not a.startswith("--")]:
        s=open(p,encoding="utf-8").read()
        t,ch=fix(s)
        print(p,len(ch),"runs fixed")
        for a,b in ch[:40]: print("   ",repr(a),"->",b)
        if not dry and ch: open(p,"w",encoding="utf-8").write(t)
