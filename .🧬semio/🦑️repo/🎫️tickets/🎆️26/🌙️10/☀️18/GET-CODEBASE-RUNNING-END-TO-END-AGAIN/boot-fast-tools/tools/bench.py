#!/usr/bin/env python3
"""bench.py --root DIR --runs N [--timeout S] [--out FILE] LABEL=ENTRY::ARGS ... : interleaved wall-clock bench of light commands (nice 10, SIGINT then SIGKILL on timeout, refuses to start above load 45). ENTRY is a script path relative to DIR (or 'bun-e' for the runtime floor); ARGS are space separated."""
import argparse,json,os,signal,statistics,subprocess,sys,time
ap=argparse.ArgumentParser();ap.add_argument("--root",required=True);ap.add_argument("--runs",type=int,default=7);ap.add_argument("--timeout",type=float,default=30);ap.add_argument("--out");ap.add_argument("--max-load",type=float,default=45);ap.add_argument("specs",nargs="+")
a=ap.parse_args();confs=[]
for s in a.specs:
    label,rest=s.split("=",1);entry,args=rest.split("::",1);confs.append((label,entry,args.split() if args else []))
def once(entry,args):
    cmd=["nice","-n","10","bun","-e","1"] if entry=="bun-e" else ["nice","-n","10","bun",entry]+args
    t=time.perf_counter();p=subprocess.Popen(cmd,cwd=a.root,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,start_new_session=True)
    try:rc=p.wait(timeout=a.timeout)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid,signal.SIGINT)
        try:p.wait(timeout=5)
        except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait()
        rc="timeout"
    return (time.perf_counter()-t)*1000,rc
res={l:[] for l,_,_ in confs};loads=[]
for l,e,ar in confs:once(e,ar)
for r in range(a.runs):
    la=os.getloadavg()[0]
    if la>=a.max_load:print(f"skip round {r}: load {la:.1f}");continue
    loads.append(la)
    for l,e,ar in confs:
        ms,rc=once(e,ar);res[l].append((ms,rc))
def q(v,p):
    v=sorted(v);k=(len(v)-1)*p;f=int(k);c=min(f+1,len(v)-1);return v[f]+(v[c]-v[f])*(k-f)
rows=[]
for l,_,_ in confs:
    ok=[m for m,rc in res[l] if rc!="timeout"];bad=len(res[l])-len(ok)
    if ok:rows.append({"label":l,"n":len(ok),"failed":bad,"min":min(ok),"p50":statistics.median(ok),"p90":q(ok,.9),"max":max(ok)})
    else:rows.append({"label":l,"n":0,"failed":bad})
print(f"load1 mean {statistics.mean(loads) if loads else float('nan'):.1f} over {len(loads)} rounds")
print(f"{'label':28} {'n':>3} {'fail':>4} {'min':>7} {'p50':>7} {'p90':>7} {'max':>7}")
for r in rows:print(f"{r['label']:28} {r['n']:3d} {r['failed']:4d} "+(f"{r['min']:7.0f} {r['p50']:7.0f} {r['p90']:7.0f} {r['max']:7.0f}" if r["n"] else ""))
if a.out:json.dump({"load":loads,"rows":rows,"samples":{l:[m for m,_ in v] for l,v in res.items()}},open(a.out,"w"),indent=1)
