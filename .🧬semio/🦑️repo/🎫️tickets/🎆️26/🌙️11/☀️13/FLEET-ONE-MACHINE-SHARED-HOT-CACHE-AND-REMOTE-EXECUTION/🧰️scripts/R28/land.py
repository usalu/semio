#!/usr/bin/env python3
"""R28 landing: the closure manifest is stored in hash-chained shards (the single record outgrew the 64 MiB read limit and was never readable), saved while a settle runs, and closures are vouched by the stamps of the dirty files they read. Anchored edits (edits.py), one replaced module (replace/) and new files (new/). usage: land.py [--dry-run|--land|--rollback <dir>] [--root <root>] [--no-tests] [--lock-held]"""
import os,sys,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
def arg(name):return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
B=edits.B
G=edits.G
TESTS=[G+"🧮️derived-nodes/🧩️closure-kinds/📒️manifest/🧪️tests",G+"🕰️generations/✂️cutoff/🧪️tests",G+"🕰️generations/✂️cutoff/🧪️tests/🧾️vouch",G+"🧮️derived-nodes/🧪️tests",G+"🕰️generations/🧪️tests",G+"🕰️generations/🔌️load/🧪️tests"]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def newfiles():
    out=[]
    for d,_,fs in os.walk(HERE+"/new"):
        for f in fs:out.append(os.path.relpath(os.path.join(d,f),HERE+"/new"))
    return sorted(out)
def plan():
    out={};refused=[]
    for rel in sorted({e[0] for e in edits.E}):
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);text=raw.decode("utf8");edited=text;ok=True
        for q,old,new,count in edits.E:
            if q!=rel:continue
            if new in edited and count>0:continue
            n=edited.count(old)
            if n!=count:refused.append((rel,"anchor occurs %d times, expected %d: %r"%(n,count,old[:70])));ok=False;break
            edited=edited.replace(old,new)
        if not ok:continue
        out[rel]=("same" if edited==text else "edit",edited.encode("utf8"),raw)
    for rel,src in edits.REPLACE:
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);data=rd(HERE+"/"+src)
        if raw==data:out[rel]=("same",data,raw)
        elif sha(raw)==edits.MANIFEST_BEFORE_SHA:out[rel]=("edit",data,raw)
        else:refused.append((rel,"live module is not the one replace/ was built from (sha %s): re-merge with make_manifest.py"%sha(raw)[:12]))
    for rel in newfiles():
        data=rd(HERE+"/new/"+rel);p=R+"/"+rel
        if os.path.exists(p):
            if rd(p)==data:out[rel]=("same",data,data)
            else:refused.append((rel,"new file exists with other content"))
        else:out[rel]=("new",data,b"")
    return out,refused
def main():
    mode=sys.argv[1] if len(sys.argv)>1 and sys.argv[1] in("--dry-run","--land","--rollback") else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];os.makedirs(T,exist_ok=True);lock=T+"/landing.lock";owner="R28-rollback-%d"%os.getpid()
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
        open(lock+"/owner","w").write(owner+"\n")
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup)
                    if f.endswith(".absent"):
                        p=R+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    p=R+"/"+rel;open(p+".r28tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r28tmp",p)
            print("restored from",backup)
        finally:
            if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
        return
    out,refused=plan()
    for rel,(how,data,_) in out.items():print(how.ljust(5),rel[len(edits.L):] if rel.startswith(edits.L) else rel,sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel[len(edits.L):] if rel.startswith(edits.L) else rel,why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused")
    os.makedirs(T,exist_ok=True)
    lock=T+"/landing.lock";owner="R28-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    if held:
        if not os.path.isdir(lock):sys.exit("--lock-held, but T/landing.lock does not exist")
    else:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R28-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        before={}
        for rel,(how,data,raw) in out.items():
            if how=="same":continue
            p=R+"/"+rel;before[rel]=sha(raw) if how=="edit" else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if how=="edit":shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r28tmp","wb").write(data)
        for rel,(how,data,raw) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r28tmp",R+"/"+rel)
        print("landed",sum(1 for v in out.values() if v[0]!="same"),"files; originals in",backup)
        res=[]
        for rel in ([] if "--no-tests" in sys.argv else TESTS):
            cwd=R+"/"+rel;env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            res.append((rel,r.returncode,failed));print("ok   " if not failed and r.returncode==0 else "RED  ",rel[len(edits.L):],*failed,sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R28b landing (%s): the start of a read says nothing when no hold blocks it\n"%stamp)
            for rel,(how,data,raw) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,rel[len(edits.L):] if rel.startswith(edits.L) else rel,before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(edits.L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[1]!=0 for r in res):print("RED test files: inspect, then --rollback "+backup)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
