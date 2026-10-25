#!/usr/bin/env python3
"""usage: chain3.py ROWS.tsv [only-substring]  Groups the chosen rows by workspace root (lib rows batched as repeated -p, [[test]] rows one invocation per test name),
runs one `bun repo cargo ... test` per group and records `🗑️generated/plugc-d/g3/<n>.raw` + a verdict line in `verdicts3.tsv` (group, packages, passed, failed, error lines)."""
import os,re,sys,subprocess,collections,time
os.chdir("/Users/ueli/Documents/semio")
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/plugc-d/g3"; os.makedirs(G,exist_ok=True)
V=C+"/🗑️generated/plugc-d/verdicts3.tsv"
rows=[l.rstrip("\n").split("\t") for l in open(sys.argv[1]) if l.strip()]
only=sys.argv[2].split(',') if len(sys.argv)>2 else None
groups=collections.OrderedDict()
for r in rows:
    cf=r[0]; t=open(cf).read()
    m=re.search(r'^workspace\s*=\s*"([^"]+)"',t,re.M)
    root=os.path.normpath(os.path.join(os.path.dirname(cf),m.group(1))) if m else os.path.dirname(cf)
    groups.setdefault((root,r[2],r[3] if r[2]=="test" else ""),[]).append(r)
done=set()
if os.path.exists(V):
    for l in open(V): done.add(l.split("\t")[0])
env=dict(os.environ); env.pop("SEMIO_SCRIPT_STALL_MS",None); env["SEMIO_BUILD_TERMINAL"]="false"
for i,((root,kind,name),rs) in enumerate(groups.items()):
    gid="%03d-%s"%(i,os.path.basename(root))
    if only:
        if not any(o in root for o in only): continue
    elif gid in done: continue
    pk=[r[1] for r in rs]
    args=["nice","-n","10","bun","repo","cargo","--store=shared","--configuration=prototype","--","test","--manifest-path",rs[0][0]]
    for p in pk: args+=["-p",p]
    args+=(["--lib"] if kind=="lib" else ["--test",name])
    args+=["sqlite_snapshot","--no-fail-fast","--message-format=short"]
    log=G+"/"+gid+".raw"
    t0=time.time()
    INFRA=("Original Script process terminated","Too many open files","Rust crate Source refused","Authenticated captured build command","LeaseOrderError","SEMIO_LEASE_ORDER","Host capacity wait cancelled","Stale loaded repository Store")
    for attempt in range(60):
        try:
            with open(log,"wb") as f: subprocess.run(args,stdout=f,stderr=subprocess.STDOUT,env=env)
        except OSError:
            time.sleep(30); continue
        try: raw=open(log,errors="replace").read()
        except OSError: time.sleep(30); continue
        if any(m in raw for m in INFRA) and "test result:" not in raw and attempt<3:
            os.replace(log,log+".infra%d"%attempt); time.sleep(60); continue
        break
    txt=""
    for attempt in range(60):
        try: txt=open(log,errors="replace").read(); break
        except OSError: time.sleep(30)
    res=re.findall(r"test result: \w+\. (\d+) passed; (\d+) failed",txt)
    passed=sum(int(a) for a,b in res); failed=sum(int(b) for a,b in res)
    errs=len(re.findall(r": error(?:\[|:)",txt))
    open(V,"a").write("%s\t%s\tpassed=%d\tfailed=%d\terrorLines=%d\tsecs=%d\n"%(gid,",".join(pk),passed,failed,errs,time.time()-t0))
open(V,"a").write("chain3-done\n")
