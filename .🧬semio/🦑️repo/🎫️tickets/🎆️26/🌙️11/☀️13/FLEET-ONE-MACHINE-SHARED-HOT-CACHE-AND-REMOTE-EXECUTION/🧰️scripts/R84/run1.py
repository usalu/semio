#!/usr/bin/env python3
"""R84: run one test file through the genuine-birth harness on the live tree (no edits). usage: run1.py <path relative to root> [setup|setup-sdk] [pattern] [art]"""
import os,sys,subprocess,shutil,re,hashlib
R="/Users/ueli/Documents/semio"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
GB=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
rel=sys.argv[1];birth=sys.argv[2] if len(sys.argv)>2 else "setup";pat=sys.argv[3] if len(sys.argv)>3 and sys.argv[3] else None;art=len(sys.argv)>4 and sys.argv[4]=="art"
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";d=None
if art:d=R+"/.cache/test-sandbox/r84-art-%d"%os.getpid();os.makedirs(d,exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=d
cmd=[BUN,GB,"--file="+(rel if rel.startswith("/") else R+"/"+rel),"--birth="+birth,"--root="+R]+(["--pattern="+pat] if pat else [])
try:r=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=int(os.environ.get("R84_TIMEOUT","1500")));out=r.stdout+r.stderr;code=r.returncode
except subprocess.TimeoutExpired:out="TIMEOUT";code=124
finally:
    if d:shutil.rmtree(d,ignore_errors=True)
o="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R84/out/"+hashlib.md5(rel.encode()).hexdigest()[:8]+".out";open(o,"w").write(out)
print("exit",code,"pass",re.findall(r"^\s*(\d+) pass",out,re.M),"fail",re.findall(r"^\s*(\d+) fail",out,re.M),"out",o)
for l in out.splitlines():
    if l.startswith("(fail)"):print(l[:220])
