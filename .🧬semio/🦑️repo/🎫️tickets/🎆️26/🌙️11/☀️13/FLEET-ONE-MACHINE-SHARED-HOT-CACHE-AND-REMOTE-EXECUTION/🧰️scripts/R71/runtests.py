import os,sys,subprocess,tempfile,shutil,time
D=sys.argv[1];dirs=sys.argv[2:]
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
for rel in dirs:
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";a=os.path.realpath(tempfile.mkdtemp());env["SEMIO_TEST_ARTIFACT_DIR"]=a
    t=time.time();r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=D+"/"+rel,env=env,capture_output=True,text=True);shutil.rmtree(a,ignore_errors=True)
    out=r.stdout+r.stderr;fails=[l[:200] for l in out.splitlines() if l.startswith("(fail)")];summ=[l for l in out.splitlines() if l.strip().startswith(("Ran ","  ")) and (" pass" in l or " fail" in l)]
    print("ok " if r.returncode==0 else "RED",rel.split("🏗️build/")[-1],round(time.time()-t),"s",summ[-2:],fails,flush=True)
    if r.returncode!=0:
        i=out.find("error:");print(out[max(0,i-800):i+1200])
