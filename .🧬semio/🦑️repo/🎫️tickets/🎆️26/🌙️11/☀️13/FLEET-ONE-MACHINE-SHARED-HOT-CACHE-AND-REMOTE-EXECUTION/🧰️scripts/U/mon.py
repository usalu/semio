import subprocess,sys,time,re,os
root=int(sys.argv[1]); out=sys.argv[2]
def tree(root):
    r=subprocess.run(["ps","-axo","pid=,ppid=,rss="],capture_output=True,text=True).stdout.split("\n")
    ch={};rss={}
    for l in r:
        p=l.split()
        if len(p)==3:
            a,b,c=map(int,p);ch.setdefault(b,[]).append(a);rss[a]=c
    seen=[root];i=0
    while i<len(seen):
        seen+=ch.get(seen[i],[]);i+=1
    return seen,rss
pk={"parent_fp":0,"parent_rss":0,"sum_fp":0,"sum_rss":0,"workers_fp":0,"workers_rss":0}
f=open(out,"w")
while True:
    pids,rss=tree(root)
    if root not in rss: break
    fp={}
    r=subprocess.run(["footprint","-f","bytes","--noCategories"]+[str(p) for p in pids],capture_output=True,text=True).stdout
    cur=None
    for l in r.split("\n"):
        m=re.match(r".*\[(\d+)\]:.*Footprint: (\d+) B",l)
        if m: fp[int(m.group(1))]=int(m.group(2))
    big=max(fp,key=fp.get) if fp else root; pf=fp.get(big,0)/2**20; pr=rss.get(big,0)/1024
    sf=sum(fp.values())/2**20; sr=sum(rss.get(p,0) for p in pids)/1024
    wf=sf-pf; wr=sr-pr
    for k,v in (("parent_fp",pf),("parent_rss",pr),("sum_fp",sf),("sum_rss",sr),("workers_fp",wf),("workers_rss",wr)):
        pk[k]=max(pk[k],v)
    f.write("%d n=%d pfp=%.0f prss=%.0f sumfp=%.0f sumrss=%.0f\n"%(time.time(),len(pids),pf,pr,sf,sr));f.flush()
    time.sleep(0.7)
f.write("PEAK "+" ".join("%s=%.0f"%kv for kv in pk.items())+"\n");f.close()
