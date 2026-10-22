#!/usr/bin/env python3
"""ttfo.py --root DIR --runs N LABEL=ENTRY::ARGS ... : interleaved time to first stdout byte and wall time to exit (nice 10, SIGINT then SIGKILL after 30 s)."""
import argparse,os,select,signal,statistics,subprocess,time
ap=argparse.ArgumentParser();ap.add_argument("--root",required=True);ap.add_argument("--runs",type=int,default=9);ap.add_argument("--out");ap.add_argument("specs",nargs="+");a=ap.parse_args()
confs=[]
for s in a.specs:
    l,r=s.split("=",1);e,ar=r.split("::",1);confs.append((l,e,ar.split() if ar else []))
def once(e,ar):
    t=time.perf_counter();p=subprocess.Popen(["nice","-n","10","bun",e]+ar,cwd=a.root,stdout=subprocess.PIPE,stderr=subprocess.DEVNULL,start_new_session=True)
    first=None
    try:
        while True:
            r,_,_=select.select([p.stdout],[],[],30)
            if not r:raise TimeoutError
            b=os.read(p.stdout.fileno(),65536)
            if first is None and b:first=(time.perf_counter()-t)*1000
            if not b:break
        p.wait(timeout=30)
    except Exception:
        os.killpg(p.pid,signal.SIGINT)
        try:p.wait(timeout=5)
        except Exception:os.killpg(p.pid,signal.SIGKILL);p.wait()
    return first,(time.perf_counter()-t)*1000
res={l:[] for l,_,_ in confs}
for l,e,ar in confs:once(e,ar)
for _ in range(a.runs):
    for l,e,ar in confs:res[l].append(once(e,ar))
print(f"load1 {os.getloadavg()[0]:.1f}\n{'label':26} {'first p50':>9} {'first min':>9} {'exit p50':>9} {'exit min':>8} {'exit p90':>8}")
rows={}
for l,_,_ in confs:
    f=[x for x,_ in res[l] if x is not None];w=[y for _,y in res[l]];w2=sorted(w)
    rows[l]={"firstP50":statistics.median(f) if f else None,"firstMin":min(f) if f else None,"exitP50":statistics.median(w),"exitMin":min(w),"exitP90":w2[int(len(w2)*0.9)-1]}
    r=rows[l];print(f"{l:26} {r['firstP50'] or 0:9.0f} {r['firstMin'] or 0:9.0f} {r['exitP50']:9.0f} {r['exitMin']:8.0f} {r['exitP90']:8.0f}")
if a.out:
    import json;json.dump({"rows":rows,"samples":{l:v for l,v in res.items()}},open(a.out,"w"),indent=1)
