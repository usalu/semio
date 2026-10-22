import subprocess,sys,time,re,json
# usage: treecpu.py <root-pid> <out.jsonl> [interval]  ; samples cumulative CPU seconds + RSS MB of the process tree below root-pid
root=int(sys.argv[1]);out=open(sys.argv[2],"a");iv=float(sys.argv[3]) if len(sys.argv)>3 else 1.0
def cpu(s):
    p=re.match(r"(?:(\d+)-)?(?:(\d+):)?(\d+):([\d.]+)$",s.strip())
    if not p:return 0.0
    d,h,m,sec=p.groups();return (int(d or 0)*86400+int(h or 0)*3600+int(m)*60+float(sec))
t0=time.time();dead={}
while True:
    rows=subprocess.run(["ps","-axo","pid=,ppid=,rss=,time="],capture_output=True,text=True).stdout.split("\n")
    info={};kids={}
    for r in rows:
        f=r.split()
        if len(f)<4:continue
        pid,ppid,rss=int(f[0]),int(f[1]),int(f[2]);info[pid]=(rss,cpu(f[3]));kids.setdefault(ppid,[]).append(pid)
    if root not in info:break
    tree=[];stack=[root]
    while stack:
        p=stack.pop();tree.append(p);stack+=kids.get(p,[])
    for p in tree:dead[p]=info[p][1]
    out.write(json.dumps({"t":round(time.time()-t0,1),"cpu":round(sum(dead.values()),2),"rssMB":round(sum(info[p][0] for p in tree)/1024),"procs":len(tree)})+"\n");out.flush()
    time.sleep(iv)
