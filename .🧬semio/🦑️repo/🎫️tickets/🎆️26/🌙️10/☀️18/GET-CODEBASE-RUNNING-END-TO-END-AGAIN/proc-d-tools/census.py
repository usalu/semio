"""One `repo exec` envelope: runs every listed test file in its own `repo tools exec bun -- test` process; usage census.py <listfile> <outdir> [<wallSeconds>]"""
import os,subprocess,sys,time,signal,re,shutil
root="/Users/ueli/Documents/semio"; mod=root+"/🧰️framework/🔨️modules/🏃️process/"
lst,out=sys.argv[1],sys.argv[2]; wall=int(sys.argv[3]) if len(sys.argv)>3 else 420
os.makedirs(out,exist_ok=True)
files=[l.strip() for l in open(lst,encoding="utf8") if l.strip()]
summ=open(out+"/summary.txt","a",encoding="utf8")
done=set(l.split("\t")[0] for l in open(out+"/summary.txt",encoding="utf8")) if os.path.exists(out+"/summary.txt") else set()
for n,f in enumerate(files):
    if f in done: continue
    art="/private/tmp/claude-501/pd/a%d"%n; shutil.rmtree(art,ignore_errors=True); os.makedirs(art)
    env=dict(os.environ,SEMIO_TEST_ARTIFACT_DIR=art)
    for k in("SEMIO_SCRIPT_STALL_MS",):env.pop(k,None)
    log=open("%s/f%03d.log"%(out,n),"wb")
    t=time.time()
    p=subprocess.Popen((["repo","tools","exec","bun","--","test"] if os.environ.get("CENSUS_ENVELOPE") else ["nice","-n","10","bun","test"])+["--timeout","120000",mod+f],cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT,stdin=subprocess.DEVNULL,start_new_session=True)
    try: rc=p.wait(timeout=wall)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid,signal.SIGTERM); time.sleep(3)
        try: os.killpg(p.pid,signal.SIGKILL)
        except Exception: pass
        rc="WALL"
    log.close()
    txt=open("%s/f%03d.log"%(out,n),encoding="utf8",errors="replace").read()
    m=re.findall(r"^\s*(\d+) (pass|fail)$",txt,re.M)
    cnt=" ".join(f"{b}={a}" for a,b in m)
    summ.write(f"{f}\trc={rc}\t{cnt}\t{int(time.time()-t)}s\tlog=f{n:03d}\n"); summ.flush()
    shutil.rmtree(art,ignore_errors=True)
summ.write("DONE\n")
