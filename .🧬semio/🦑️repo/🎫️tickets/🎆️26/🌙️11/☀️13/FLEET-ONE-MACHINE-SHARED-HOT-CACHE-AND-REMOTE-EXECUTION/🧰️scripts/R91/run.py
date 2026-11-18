import subprocess,sys,time,os,json
bun="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
here=os.path.dirname(os.path.abspath(__file__))
env=dict(os.environ,R91_STAGING="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R91/stg",R91_WAVE="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R80/wave")
extra=sys.argv[1].split(",") if sys.argv[1] else []
for kv in extra:
    k,v=kv.split("=");env[k]=v
args=sys.argv[2:]
pre=["--preload",os.environ.get("R91_PRELOAD_FILE")] if os.environ.get("R91_PRELOAD_FILE") else []
p=subprocess.Popen([bun]+pre+[os.path.join(here,"harness.ts")]+args,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,cwd="/tmp")
import threading
def rd():
    for l in p.stdout: print(l.rstrip()[:400],flush=True)
th=threading.Thread(target=rd);th.start()
peak=0;t0=time.time();series=[];treepeak=0;maxkids=0
def table():
    out=subprocess.run(["ps","-A","-o","pid=,ppid=,rss="],capture_output=True,text=True).stdout.split("\n")
    rows={}
    for l in out:
        f=l.split()
        if len(f)==3: rows[int(f[0])]=(int(f[1]),int(f[2])//1024)
    return rows
def tree(rows,root):
    kids={};
    for pid,(pp,_) in rows.items(): kids.setdefault(pp,[]).append(pid)
    res=[];st=[root]
    while st:
        x=st.pop();res.append(x);st.extend(kids.get(x,[]))
    return res
while p.poll() is None:
    try:
        rows=table()
        if p.pid in rows:
            t=tree(rows,p.pid);self_=rows[p.pid][1];tot=sum(rows[x][1] for x in t if x in rows)
            peak=max(peak,self_);treepeak=max(treepeak,tot);maxkids=max(maxkids,len(t)-1)
            series.append((round(time.time()-t0,2),self_,tot,len(t)-1))
    except Exception as e: pass
    time.sleep(0.25)
th.join()
print("PEAK_SELF_RSS_MB",peak,"PEAK_TREE_RSS_MB",treepeak,"MAX_CHILDREN",maxkids,"WALL",round(time.time()-t0,1))
print("series (t,self,tree,kids) every 8:",series[::8][:80])
