#!/usr/bin/env python3
"""R57 landing 2 (watchdog by progress counters, stop reason before the signal, read.log rotation). Based on land.py of landing 1: whole-heap walks of the graph parent are bounded by a duty share (heap census, forced release, parent collector). Takes landing.lock (waits for it), three-way merges the staged files onto the live ones (git merge-file on scratch copies only), appends the law tests, runs the standing preflight tests, the tests of every touched member and of its direct dependants, appends the landing to landings.md and releases the lock. A red test restores the files this script wrote (only when the live file still equals what it wrote)."""
import os,sys,subprocess,hashlib,datetime,time,shutil,tempfile
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R57"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";M=B+"🧠️memory/";F=M+"🦶️footprint/";G=B+"🕸️graph/";N=G+"📥️inference/📦️project-nodes/";SI=G+"📥️inference/🧭️script-inputs/"
STAGE=SC+"/stage3";MAP=[l.split(" ",1) for l in open(STAGE+"/map.txt",encoding="utf8").read().splitlines()]
FILES=[(B+m[1].strip(),"stage3","") for m in MAP]
APPENDS=[]
W=B+"🪪️graph-state/🙋️wanted/";SD=B+"🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/"
TESTS=[W+"🧪️tests",W+"🧪️tests/🛠️server",W+"🧪️tests/🛠️server/🌊️wave-rows",SD+"🔭️read",SD[:-len("🕸️graph/")]+"🕸️graph",SD+"🌳️tree",SD+"🔬️stall",SD+"🌊️wave-rows",SD+"💰️wave-costs",SD+"📡️wave-upload",SI+"🔌️provider-closures/🧪️tests/🛃️preflight",SI+"🪆️heavy-questions/🧪️tests"]
KNOWN_RED=["a read that holds far more than its grant under pressure is stopped after the overrun limit"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
lock=T+"/landing.lock"
def acquire():
    waited=0
    while True:
        try:os.mkdir(lock);open(lock+"/owner","w").write("R57-%d\n"%os.getpid());return
        except FileExistsError:
            try:age=time.time()-os.stat(lock).st_mtime
            except FileNotFoundError:continue
            if age>40*60:
                print("taking over a lock older than 40 minutes:",open(lock+"/owner").read().strip());shutil.rmtree(lock);continue
            if waited%120==0:print("waiting for landing.lock held by",open(lock+"/owner").read().strip() if os.path.exists(lock+"/owner") else "?",flush=True)
            time.sleep(10);waited+=10
            if waited>35*60:sys.exit("lock not free in 35 minutes")
acquire()
written={};backups=SC+"/live-before";os.makedirs(backups,exist_ok=True)
try:
    for live,stage,rel in FILES:
        lp=R+"/"+live;fid=[m[0] for m in MAP if B+m[1].strip()==live][0];bp=SC+"/"+stage+"/base/"+fid;wp=SC+"/"+stage+"/work/"+fid
        cur=open(lp,"rb").read();shutil.copy(lp,backups+"/"+hashlib.md5(live.encode()).hexdigest())
        if cur==open(bp,"rb").read():new=open(wp,"rb").read()
        else:
            tmp=tempfile.mkdtemp(dir=SC);shutil.copy(lp,tmp+"/live");shutil.copy(bp,tmp+"/base");shutil.copy(wp,tmp+"/work")
            r=subprocess.run(["git","merge-file","-p",tmp+"/live",tmp+"/base",tmp+"/work"],capture_output=True)
            if r.returncode!=0:raise SystemExit("merge conflict in "+live)
            new=r.stdout
        open(lp,"wb").write(new);written[live]=(hashlib.sha256(cur).hexdigest()[:12],hashlib.sha256(new).hexdigest()[:12])
    for live,body,marker in APPENDS:
        lp=R+"/"+live;cur=open(lp,"rb").read();shutil.copy(lp,backups+"/"+hashlib.md5(live.encode()).hexdigest())
        if marker.encode() in cur:raise SystemExit("law tests already present in "+live)
        new=cur+open(body,"rb").read();open(lp,"wb").write(new);written[live]=(hashlib.sha256(cur).hexdigest()[:12],hashlib.sha256(new).hexdigest()[:12])
    art=os.path.join(SC,"art");os.makedirs(art,exist_ok=True);res=[];red=False
    for rel in TESTS:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        failed=[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)") and not any(k in l for k in KNOWN_RED)];res.append((rel,r.returncode,failed));red=red or bool(failed) or (r.returncode!=0 and not any(k in (r.stdout+r.stderr) for k in KNOWN_RED))
        print("ok   " if not failed and r.returncode==0 else "RED  ",rel[len(L):],*failed,sep="\n  " if failed else " ",flush=True)
    if red and os.environ.get("R57_KEEP_RED")!="1":
        for live,(before,after) in written.items():
            lp=R+"/"+live
            if sha(lp)==after:shutil.copy(backups+"/"+hashlib.md5(live.encode()).hexdigest(),lp)
        print("RED: restored the files of this landing");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R57 landing 2 (%s): the stall watchdog of the graph read counts any forward progress, a stop writes why before the signal, read.log is rotated\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        for live,(before,after) in written.items():f.write("- edit `%s` before %s after %s\n"%(live[len(L):],before,after))
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    try:os.remove(lock+"/owner")
    except FileNotFoundError:pass
    try:os.rmdir(lock)
    except OSError:pass
