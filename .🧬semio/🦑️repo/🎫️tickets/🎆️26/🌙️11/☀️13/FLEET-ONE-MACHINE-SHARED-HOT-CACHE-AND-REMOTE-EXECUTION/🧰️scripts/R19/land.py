#!/usr/bin/env python3
"""R19 replica change PREPARED, NOT LANDED: the executable bit survives a content rewrite by an editor that recreates the file without it (publish: bit = disk bit OR recorded bit when the content changed; a same-content bit change still follows the disk), and the publishing machine chmods the file to the recorded bit.
usage: land.py --dry-run | --land
  --dry-run  checks that every anchor is found exactly once in the live files (or the patch is already applied), takes no lock, writes nothing.
  --land     needs N/LAND-OK-R19 (the coordinator batches it with R12: a replica digest change restarts every daemon); anchored read-modify-write, takes T/landing.lock, runs the replica test files."""
import os,sys,subprocess,time
HERE=os.path.dirname(os.path.abspath(__file__))
R="/Users/Shared/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
RP=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
REPLICA=RP+"/🟦️.ts";GITTEST=RP+"/🧪️tests/🌿️git.ts"
PATCH=[(REPLICA,'''  const executable=replica.settings.modeless?(current?.operation==="put"?current.executable!:stagedExecutable(replica,entry.path)):entry.executable;
  if(current?.operation==="put"&&current.bytes===entry.bytes){
   const hash=await hashFileV1(target);
   if(!hash.stable)return unstable();
   if(current.sha256===hash.sha256&&current.executable===executable)''','''  const recorded=current?.operation==="put"?current.executable!:undefined,decide=(sha256:string):boolean=>replica.settings.modeless?(recorded??stagedExecutable(replica,entry.path)):entry.executable||(recorded===true&&current!.sha256!==sha256);
  if(current?.operation==="put"&&current.bytes===entry.bytes){
   const hash=await hashFileV1(target);
   if(!hash.stable)return unstable();
   const executable=decide(hash.sha256);
   if(current.sha256===hash.sha256&&current.executable===executable)'''),
(REPLICA,'''  return stored.stable?{job,sha256:stored.sha256,bytes:stored.bytes,executable,touch:false}:unstable();''','''  return stored.stable?{job,sha256:stored.sha256,bytes:stored.bytes,executable:decide(stored.sha256),touch:false}:unstable();'''),
(REPLICA,'''  const events:FleetEventV1[]=[],updates:[string,StateEntry,boolean][]=[];
  const packsHere''','''  const events:FleetEventV1[]=[],updates:[string,StateEntry,boolean][]=[],chmods:string[]=[];
  const packsHere'''),
(REPLICA,'''    events.push({version:1,hlc,operation:"put",path:entry.path,sha256:item.sha256!,bytes:item.bytes!,executable:item.executable!,base});''','''    if(!replica.settings.modeless&&item.executable===true&&!entry.executable)chmods.push(entry.path);
    events.push({version:1,hlc,operation:"put",path:entry.path,sha256:item.sha256!,bytes:item.bytes!,executable:item.executable!,base});'''),
(REPLICA,'''   setFile(replica,path,state);if(fresh)replica.manifest.set(path,plain(state));
  }
  activityV1("persist");''','''   setFile(replica,path,state);if(fresh)replica.manifest.set(path,plain(state));
  }
  for(const path of chmods)await chmod(absolute(replica.settings.worktree,path),0o755).catch(error=>{if(!gone(error))throw error;});
  activityV1("persist");'''),
(GITTEST,"statSync,symlinkSync,utimesSync,writeFileSync} from \"node:fs\";","statSync,symlinkSync,utimesSync,writeFileSync} from \"node:fs\";")]
def read(p):return open(p,encoding="utf8").read()
def status():
    out=[]
    for p,a,b in PATCH:
        s=read(p);out.append((p,"pending" if s.count(a)==1 and a!=b else "applied" if s.count(b)>=1 else "ANCHOR LOST" if a!=b else "n/a"))
    test=open(HERE+"/new-test.ts",encoding="utf8").read();t=read(GITTEST)
    out.append((GITTEST,"test applied" if test.strip() in t else "test pending"))
    return out
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run";st=status()
    for p,w in st:print(w.ljust(12),os.path.relpath(p,R)[-90:])
    bad=[x for x in st if x[1]=="ANCHOR LOST"]
    if mode!="--land":print("dry run: nothing written;",len(bad),"refused;","git.ts needs the renameSync import too" );sys.exit(1 if bad else 0)
    if bad:sys.exit("refused: anchor lost")
    if not os.path.exists(N+"/LAND-OK-R19"):sys.exit("LAND-OK-R19 does not exist: prepared, not landed")
    lock=T+"/landing.lock";owner="R19-%d-%d"%(os.getpid(),int(time.time()))
    try:os.mkdir(lock)
    except FileExistsError:sys.exit(lock+" is held: wait, never remove it")
    open(lock+"/owner","w").write(owner+"\n")
    try:
        for p,a,b in PATCH:
            s=read(p)
            if a!=b and s.count(a)==1:open(p,"w",encoding="utf8").write(s.replace(a,b))
        t=read(GITTEST)
        if "renameSync" not in t.split("\n",3)[1]:t=t.replace("mkdtempSync,readFileSync,rmSync,","mkdtempSync,readFileSync,renameSync,rmSync,",1)
        test=open(HERE+"/new-test.ts",encoding="utf8").read()
        if test.strip() not in t:t=t.rstrip("\n")+"\n\n"+test
        open(GITTEST,"w",encoding="utf8").write(t)
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        for f in ["🌿️git.ts","🟦️.ts","🩹️heal.ts","🧭️agree.ts","🏁️race.ts","💾️store.ts","🛟️boot.ts"]:
            r=subprocess.run([BUN,"test","./"+f,"--timeout","180000"],cwd=RP+"/🧪️tests",env=env,capture_output=True,text=True)
            print(f,r.returncode,[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")])
    finally:
        if os.path.exists(lock+"/owner") and read(lock+"/owner").strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
main()
