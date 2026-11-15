#!/usr/bin/env python3
"""runs one overlay test file: ovrun.py <overlay dir> <substring of the test path> (regenerates the overlay first)"""
import json,os,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__));D=sys.argv[1];sub=sys.argv[2]
subprocess.run(["rm","-rf",D]);subprocess.run([sys.executable,HERE+"/land.py","--overlay",D],check=True,capture_output=True)
tests=json.load(open(D+"/tests.json"));m=[v for k,v in tests.items() if sub in k]
if len(m)!=1:sys.exit("no unique test for %r: %r"%(sub,list(tests)))
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
r=subprocess.run([BUN,"test","--preload",D+"/preload.ts",m[0],"--timeout","100000"],cwd=os.path.dirname(m[0]),env=env,capture_output=True,text=True)
t=r.stdout+r.stderr
print("\n".join(l[:240] for l in t.splitlines() if l.startswith(("(fail)","error:")) or " pass" in l or " fail" in l))
