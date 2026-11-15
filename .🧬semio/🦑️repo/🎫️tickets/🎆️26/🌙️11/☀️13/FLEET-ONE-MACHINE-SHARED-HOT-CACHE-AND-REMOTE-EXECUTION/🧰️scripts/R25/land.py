#!/usr/bin/env python3
"""R25 landing: the staged dependency funding primitives (schema constants and pointer schema, the funding window functions in the funding module, two bilingual texts, the window test).
usage: land.py --dry-run | --land | --rollback <backup-dir>   [--root <scratch root>]   (--root rehearses the whole landing in a scratch copy: lock, tests, record and rollback; the files are served to bun through overlay-preload.ts)
  Edits are the anchored edits of edits.py (an edit counts as applied when its new text is present), re-read from the live file right before they are written; nothing is merged blindly: an anchor that is not found exactly once refuses the landing.
  --land needs N/LAND-OK, takes T/landing.lock, refuses while a `tools setup all` runs (it loads the funding module), runs every regression file once before and once after (verdict `ok`, `same as before` or `NEW RED`), load-checks the touched sources and records the landing in T/landings.md.
  No graph command, no `repo tools setup` and no build is run here."""
import os,sys,subprocess,hashlib,time,shutil,datetime,json,re
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
LIVE="/Users/Shared/semio"
R=sys.argv[sys.argv.index("--root")+1] if "--root" in sys.argv else LIVE
REHEARSAL=R!=LIVE
TK="/.🧬semio/🦑️repo/🎫️tickets/🎆️26"
T=R+TK+"/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+TK+"/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
MOD="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
PUB=MOD+"/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication"
NEWTEST=PUB+"/⚙️setup/🧪️tests/🪟️publication-window/🟦️.ts"
TESTS=[PUB+"/⚙️setup/🧪️tests/🟦️.ts",PUB+"/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts",PUB+"/⚙️setup/🧪️tests/🌊️moving-tree/🟦️.ts",PUB+"/⚙️setup/♻️retirement/🧪️tests/🟦️.ts",PUB+"/⚙️setup/♻️retirement/🧪️tests/🔁️busy-driver/🟦️.ts",MOD+"/📖️texts/🧪️tests/🟦️.ts",PUB+"/🧪️tests/♻️phase/🟦️.ts",PUB+"/🧪️tests/📋️roles/🟦️.ts",MOD+"/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/📄️generation/🧪️tests/🟦️.ts","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📸️observation/🧪️tests/🟦️.ts",NEWTEST]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p,default=None):
    try:
        with open(p,"rb") as f:return f.read()
    except FileNotFoundError:return default
def plan():
    out={};refused=[];by={}
    for rel,old,new,count in edits.E:by.setdefault(rel,[]).append((old,new,count))
    for rel,rows in by.items():
        live=rd(R+"/"+rel)
        if live is None:refused.append((rel,"live file is gone"));continue
        text=live.decode("utf8")
        for old,new,count in rows:
            if new in text:continue
            n=text.count(old)
            if n!=count:refused.append((rel,"anchor found %d times (wanted %d): %r"%(n,count,old[:70])));break
            text=text.replace(old,new)
        else:out[rel]=("same" if text.encode("utf8")==live else "edit",text.encode("utf8"))
    for d,_,fs in os.walk(HERE+"/new"):
        for f in fs:
            rel=os.path.relpath(os.path.join(d,f),HERE+"/new");mine=rd(os.path.join(d,f));live=rd(R+"/"+rel)
            out[rel]=("same" if live==mine else "new" if live is None else "replace",mine)
    return out,refused
def parses(rel,data):
    if rel.endswith(".json"):
        try:json.loads(data);return True,""
        except ValueError as e:return False,str(e)
    import tempfile;fd,tmp=tempfile.mkstemp(suffix=".ts",prefix="r25-parse-");os.write(fd,data);os.close(fd)
    r=subprocess.run([BUN,"-e","new Bun.Transpiler({loader:'ts'}).transformSync(await Bun.file(process.argv[1]).text())",tmp],capture_output=True,text=True)
    os.remove(tmp);return r.returncode==0,r.stderr[-200:]
def env():
    e={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    e["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    if REHEARSAL:e.update(R5_OVERLAY=R,R5_LIVE=LIVE,R5_PRELOAD=HERE+"/overlay-preload.ts")
    return e
def run_test(rel):
    path=R+"/"+rel
    if not os.path.exists(path):return None
    cmd=[BUN,"test"]+(["--preload",HERE+"/overlay-preload.ts"] if REHEARSAL else [])+["./"+os.path.basename(path),"--timeout","180000"]
    r=subprocess.run(cmd,cwd=os.path.dirname(path),env=env(),capture_output=True,text=True);t=r.stdout+r.stderr
    return r.returncode,[l for l in t.splitlines() if l.startswith("(fail)")]
def lock(path,owner):
    try:os.mkdir(path)
    except FileExistsError:
        age=time.time()-os.path.getmtime(path)
        if age<2400:sys.exit(path+" is held by "+(rd(path+"/owner",b"?").decode().strip())+": wait, never remove it")
        shutil.rmtree(path)
        os.mkdir(path)
    open(path+"/owner","w").write(owner+"\n")
def unlock(path,owner):
    if (rd(path+"/owner",b"").decode().strip())==owner:os.remove(path+"/owner");os.rmdir(path)
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];owner="R25-rollback-%d"%os.getpid();lock(T+"/landing.lock",owner)
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup);p=R+"/"+rel
                    if f.endswith(".absent"):
                        p=R+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    open(p+".r25tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r25tmp",p)
            print("restored from",backup)
        finally:unlock(T+"/landing.lock",owner)
        return
    if mode=="--land" and not REHEARSAL:
        busy=subprocess.run(["ps","-eo","pid,command"],capture_output=True,text=True).stdout
        running=[l for l in busy.splitlines() if "tools setup all" in l and "land.py" not in l]
        if running:sys.exit("a `tools setup all` run is live (it loads the funding module): wait for it\n"+"\n".join(l[:80] for l in running))
    out,refused=plan()
    for rel,(how,data) in out.items():
        ok,why=parses(rel,data) if how!="same" else (True,"");print(how.ljust(7),"parse "+("ok" if ok else "FAIL "+why),rel[-100:],sha(data)[:12])
        if not ok:refused.append((rel,"does not parse"))
    for rel,why in refused:print("REFUSED",rel[-100:],why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else "; 0 refused"));return
    if refused:sys.exit("refused")
    if not os.path.exists(N+"/LAND-OK"):sys.exit("LAND-OK does not exist")
    owner="R25-%d-%d"%(os.getpid(),int(time.time()));stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R25-landing-"+datetime.datetime.now().strftime("%H%M%S")
    lock(T+"/landing.lock",owner)
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        baseline={rel:run_test(rel) for rel in TESTS}
        before={}
        for rel,(how,data) in out.items():
            if how=="same":continue
            p=R+"/"+rel;before[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r25tmp","wb").write(rd(os.path.join(HERE,"new",rel)) if how in("new","replace") else data)
        for rel,(how,_) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r25tmp",R+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup)
        res=[]
        for rel in TESTS:
            now=run_test(rel);was=baseline.get(rel)
            if now is None:continue
            code,failed=now
            norm=lambda rows:{re.sub(r"\s*\[[0-9.]+m?s\]$","",l) for l in rows};verdict="ok" if not failed and code==0 else ("same as before" if was is not None and norm(failed)<=norm(was[1]) and (code!=0)==(was[0]!=0) else "NEW RED")
            res.append((rel,code,failed,was,verdict));print(verdict.ljust(14),rel[-100:],"fail",len(failed),"before","-" if was is None else len(was[1]))
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R25 landing (%s): the staged dependency funding primitives: build beside the store with no hold, a bounded exclusive publish reported as `exclusive for N ms`, pinned snapshots for readers, retirement by pins, recovery after kill -9\n"%stamp)
            for rel,(how,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,rel,before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed,was,verdict in res:f.write("- test `%s` exit %d, %d red (before %s): %s\n"%(rel,code,len(failed),"-" if was is None else len(was[1]),verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(v=="NEW RED" for *_,v in res):print("UNEXPECTED result: inspect, then --rollback "+backup)
    finally:unlock(T+"/landing.lock",owner)
main()
