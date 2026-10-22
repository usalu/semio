import glob,hashlib,json,os,shutil,subprocess,sys,time
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OUT=C+"/🗑️generated/bzl-snapshot"
target=sys.argv[1];config=sys.argv[2];tag=sys.argv[3];attempts=int(sys.argv[4]);prefix=sys.argv[5]  # prefix: repo path prefix whose files get disturbed
log=OUT+"/"+tag+".log";report=OUT+"/"+tag+".report.json"
def manifests():
    return {f:os.stat(f).st_mtime for f in glob.glob(ROOT+"/.cache/b/p/m/bazel/workspaces/*/*/snapshot.json")}
def sha(p):
    return hashlib.sha256(open(p,"rb").read()).hexdigest()
result={"attempts":[]}
for attempt in range(1,attempts+1):
    before=manifests();start=time.time()
    logf=open(log,"ab",buffering=0)
    proc=subprocess.Popen(["nice","-n","10","bun","repo","--tool=bazel","run",target,"--configuration="+config],cwd=ROOT,stdout=logf,stderr=subprocess.STDOUT,stdin=subprocess.DEVNULL)
    row={"attempt":attempt,"pid":proc.pid,"started":time.strftime("%H:%M:%S")}
    disturbed=[];saved={};planned=None
    try:
        while proc.poll() is None:
            now=manifests()
            fresh=[f for f,m in now.items() if m>start and (f not in before or m>before[f])]
            if fresh and planned is None:
                time.sleep(1.5)
                manifest=sorted(fresh,key=lambda f:-now[f])[0]
                rows=json.load(open(manifest))["rows"]
                planned={"manifest":manifest,"files":len(rows)}
                chosen=[r for r in rows if r["path"].startswith(prefix)][:3]+[r for r in rows if r["path"].endswith(".rs") and r["bytes"]<4000][:3]
                for r in chosen:
                    p=ROOT+"/"+r["path"]
                    if not os.path.isfile(p): continue
                    data=open(p,"rb").read()
                    if hashlib.sha256(data).hexdigest()!=r["sha256"]:
                        disturbed.append({"path":r["path"],"note":"already differs from planned bytes at disturbance time"})
                        continue
                    saved[p]=data
                    tmp=p+".snapshot-probe"
                    with open(tmp,"wb") as h: h.write(data)
                    os.replace(tmp,p)
                    disturbed.append({"path":r["path"],"how":"atomic replace with identical bytes (new inode, new mtime)"})
                edit=[r for r in rows if r["path"].endswith(".rs") and r["bytes"]<4000 and ROOT+"/"+r["path"] in saved][:1]
                for r in edit:
                    p=ROOT+"/"+r["path"]
                    with open(p,"ab") as h: h.write(b"\n")
                    disturbed.append({"path":r["path"],"how":"appended a newline (restored after the run)","plannedSha":r["sha256"]})
            time.sleep(1)
    finally:
        for p,data in saved.items():
            if open(p,"rb").read()!=data:
                tmp=p+".snapshot-probe"
                with open(tmp,"wb") as h: h.write(data)
                os.replace(tmp,p)
    row.update({"exit":proc.returncode,"planned":planned,"disturbed":disturbed,"seconds":round(time.time()-start)})
    text=open(log,"rb").read().decode("utf8","replace")[-20000:]
    row["loaderFault"]=("refused" in text and "captured" in text.lower()) and planned is None
    result["attempts"].append(row)
    json.dump(result,open(report,"w"),indent=1,ensure_ascii=False)
    if planned is not None or proc.returncode==0: break
    time.sleep(240)
print(json.dumps(result,ensure_ascii=False))
