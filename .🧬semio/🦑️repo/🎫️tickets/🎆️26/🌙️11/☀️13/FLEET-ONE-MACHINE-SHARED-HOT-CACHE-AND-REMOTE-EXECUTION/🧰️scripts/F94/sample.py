import subprocess,sys,time,os,re
out=sys.argv[1]
p=subprocess.Popen(["repo","test-file",sys.argv[2]],stdout=open(out,"w"),stderr=subprocess.STDOUT)
best=(0,"")
rows=[]
while p.poll() is None:
    ps=subprocess.run(["ps","-axo","pid=,ppid=,rss=,command="],capture_output=True,text=True).stdout.splitlines()
    tbl={}
    for l in ps:
        m=re.match(r"\s*(\d+)\s+(\d+)\s+(\d+)\s+(.*)",l)
        if m: tbl[int(m[1])]=(int(m[2]),int(m[3]),m[4])
    kids={}
    for pid,(pp,r,c) in tbl.items(): kids.setdefault(pp,[]).append(pid)
    tree=[];st=[p.pid]
    while st:
        x=st.pop();tree.append(x);st+=kids.get(x,[])
    tot=sum(tbl[x][1] for x in tree if x in tbl)
    if tot>best[0]:
        best=(tot,"\n".join("%6d MB %s"%(tbl[x][1]//1024,tbl[x][2][:110]) for x in sorted(tree,key=lambda x:-tbl.get(x,(0,0,""))[1])[:12] if x in tbl))
    time.sleep(0.5)
print("peak tree MB",best[0]//1024);print(best[1])
