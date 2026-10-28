import sys,re,glob,collections
root=sys.argv[1]
files=sorted(glob.glob(root+"/g3*-*.log")+glob.glob(root+"/g4*-*.log"))
cnt=collections.Counter();ex={};fl={}
for f in files:
    with open(f,errors="replace") as h:
        for ln in h:
            ln=ln.rstrip("\n")
            if len(ln)>400: ln=ln[:400]
            t=re.sub(r"\x1b\[[0-9;]*m","",ln)
            t=re.sub(r"[0-9a-f]{12,}","<H>",t)
            t=re.sub(r"\d+(\.\d+)?","<N>",t)
            t=re.sub(r"/Users/[^ \"']+","<P>",t)
            t=t[:160]
            cnt[t]+=1
            ex.setdefault(t,ln); fl.setdefault(t,f.split("/")[-1])
with open(sys.argv[2],"w") as o:
    for t,c in cnt.most_common():
        o.write(f"{c}\t{fl[t]}\t{ex[t][:300]}\n")
print(len(cnt),"templates")
