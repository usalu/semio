import subprocess,sys,time,os,json,threading
bun="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
here=os.path.dirname(os.path.abspath(__file__))
pre=["--preload",os.path.join(here,"overlay.ts")] if os.environ.get("R103_OVER") else []
p=subprocess.Popen([bun]+pre+[os.path.join(here,sys.argv[1])]+sys.argv[2:],stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,cwd="/tmp")
def rd():
    for l in p.stdout: print(l.rstrip()[:600],flush=True)
th=threading.Thread(target=rd);th.start()
def fp(pid):
    r=subprocess.run(["footprint","-p",str(pid),"--noCategories"],capture_output=True,text=True).stdout
    for l in r.splitlines():
        if "phys_footprint:" in l:
            v=l.split("phys_footprint:")[1].split()
            n=float(v[0]);u=v[1]
            return n*(1024 if u.startswith("GB") else 1 if u.startswith("MB") else 1/1024)
    return None
def kids(root):
    rows={}
    for l in subprocess.run(["ps","-A","-o","pid=,ppid="],capture_output=True,text=True).stdout.splitlines():
        a=l.split()
        if len(a)==2: rows.setdefault(int(a[1]),[]).append(int(a[0]))
    res=[];st=[root]
    while st:
        x=st.pop();res.append(x);st.extend(rows.get(x,[]))
    return res
peak=0;rssp=0;t0=time.time();ser=[];treepeak=0;maxkids=0
while p.poll() is None:
    f=fp(p.pid)
    r=subprocess.run(["ps","-o","rss=","-p",str(p.pid)],capture_output=True,text=True).stdout.strip()
    r=int(r)//1024 if r else 0
    if f: peak=max(peak,f)
    ks=kids(p.pid)
    tf=sum((fp(k) or 0) for k in ks)
    treepeak=max(treepeak,tf);maxkids=max(maxkids,len(ks)-1)
    rssp=max(rssp,r);ser.append((round(time.time()-t0),round(f or 0),round(tf),len(ks)-1))
    time.sleep(1)
th.join()
print("PEAK_TREE_FOOTPRINT_MB",round(treepeak),"MAX_CHILDREN",maxkids,"PEAK_FOOTPRINT_MB",round(peak),"PEAK_RSS_MB",rssp,"WALL",round(time.time()-t0,1))
print("series",ser[::5])
