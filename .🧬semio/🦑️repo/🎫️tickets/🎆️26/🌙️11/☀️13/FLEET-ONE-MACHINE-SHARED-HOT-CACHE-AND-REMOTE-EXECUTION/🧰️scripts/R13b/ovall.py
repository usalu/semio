#!/usr/bin/env python3
"""runs test files under the overlay (or live with --live): ovall.py <overlay dir>|--live <root-relative test paths...>; prints one line per file"""
import json,os,subprocess,sys,time
R="/Users/Shared/semio"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
live=sys.argv[1]=="--live";D=None if live else sys.argv[1]
tests={} if live else json.load(open(D+"/tests.json"))
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
for rel in sys.argv[2:]:
    path=tests.get(rel,R+"/"+rel);cmd=[BUN,"test"]+([] if live else ["--preload",D+"/preload.ts"])+[path,"--timeout","170000"]
    t0=time.time();r=subprocess.run(cmd,cwd=os.path.dirname(path),env=env,capture_output=True,text=True);t=r.stdout+r.stderr
    fails=[l[:150] for l in t.splitlines() if l.startswith("(fail)")];summary=[l.strip() for l in t.splitlines() if l.strip().endswith((" pass"," fail")) ]
    print(("OK  " if r.returncode==0 else "RED ")+rel.split("🏗️build/")[-1][:90],"|",",".join(summary),"|%ds"%(time.time()-t0),*fails,sep="\n   " if fails else " ",flush=True)
