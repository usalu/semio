import os,subprocess,statistics,sys,time
L="/private/tmp/boot-fast/repo-launcher";env={**os.environ,"SEMIO_RUNTIME_CACHE":"/private/tmp/boot-fast/rcache","SEMIO_LANGUAGE":"en"}
S="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
cases=[("launcher --help","ov3",[L,"--help"]),("launcher run --help (door)","ov3",[L,"run","--help"]),("launcher run --help (old script)","ovref",[L,"run","--help"]),("launcher leases status (door)","ov3",[L,"leases","status"]),("launcher leases status (old script)","ovref",[L,"leases","status"]),("launcher tools status (door refusal)","ov3",[L,"tools","status"]),("bun door run --help (no launcher)","ov3",["bun",S+"🚪️door/📜️script.ts","repo","run","--help"]),("bun -e 1 (floor)","ov3",["bun","-e","1"])]
res={c[0]:[] for c in cases}
def once(root,cmd):
    t=time.perf_counter();subprocess.run(["nice","-n","10"]+cmd,cwd="/private/tmp/boot-fast/"+root,env={**env,"SEMIO_WORKSPACE_ROOT":"/private/tmp/boot-fast/"+root},stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,timeout=60);return (time.perf_counter()-t)*1000
for n,r,c in cases:once(r,c)
for _ in range(11):
    for n,r,c in cases:res[n].append(once(r,c))
print("load1 %.1f"%os.getloadavg()[0])
for n,v in res.items():print(f"{n:40} p50 {statistics.median(v):6.0f} min {min(v):6.0f}")
