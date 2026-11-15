import os,sys,subprocess,time
root,out=sys.argv[1],sys.argv[2];only=sys.argv[3:] 
BUN="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
M=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules"
files=[]
for base in [M+"/🤝️shared-worktree"]+([M+"/📚️library/🏗️build/"+x for x in("🌐️remote-cache","🌐️remote-execution","🚢️fleet-status","🪪️graph-state")] if os.environ.get("WITHBUILD") else []):
    for d,_,fs in os.walk(base):
        if "node_modules" in d or "/🧪️tests" not in d+"/":continue
        files+=[os.path.join(d,f) for f in sorted(fs) if f.endswith(".ts")]
files=sorted(files)
if only:files=[f for f in files if any(o in f for o in only)]
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_GRAPH_SERVER"]="off"
with open(out,"a") as o:
  for f in files:
    t=time.time()
    try:r=subprocess.run([BUN,"test","./"+os.path.basename(f),"--timeout","180000"],cwd=os.path.dirname(f),env=env,capture_output=True,text=True,timeout=900)
    except subprocess.TimeoutExpired:o.write("TIMEOUT %s\n"%f[len(M)+1:]);o.flush();continue
    txt=r.stdout+r.stderr;fails=[l for l in txt.splitlines() if l.startswith("(fail)")]
    summ=[l.strip() for l in txt.splitlines() if l.strip().startswith(("pass","fail")) and len(l.split())<=2 or "pass" in l and l.strip().startswith(("pass",))]
    o.write("%s rc=%d %.0fs %s\n"%(f[len(M)+1:],r.returncode,time.time()-t," | ".join(summ[:2])))
    for l in fails:o.write("   "+l+"\n")
    o.flush()
