#!/usr/bin/env python3
"""R49 divergence landing: the neutral record carries no machine stamps; the cache counts divergences per kind.
usage: land.py [--root DIR] --dry-run | --land [--merge] [--no-tests] [--lock-held] [--replace-new] | --rollback BACKUPDIR
--merge: every live file is read again right before it is written, the anchored edits are applied to THAT text (other agents, for example R36, change the same modules), the result must differ from the live text exactly in this package's hunks, and a file that changed between the read and the replace aborts the landing and restores what was already replaced; an anchor that is gone or twice there refuses the whole landing."""
import os,sys,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import importlib;edits=importlib.import_module(os.environ.get("R49_EDITS","edits_div"))
def arg(name):return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
LB=edits.B
TESTS=[LB+"🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🕰️stamps",LB+"🌐️remote-cache/🧾️provenance/🧪️tests/🧮️by-class",LB+"🪪️graph-state/📦️project-records/🧳️portable/🧪️tests",LB+"🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🌍️real",LB+"🪪️graph-state/📦️project-records/🧪️tests",LB+"🪪️graph-state/📦️project-records/🧪️tests/🚚️closures",LB+"🪪️graph-state/📦️project-records/🧪️tests/🌍️two-platforms",LB+"🪪️graph-state/🧪️tests",LB+"🌐️remote-cache/🧾️provenance/🧪️tests",LB+"🌐️remote-cache/🔬️action-result/🧪️tests",LB+"🌐️remote-cache/🖥️server/🧪️tests",LB+"🌐️remote-cache/🖥️server/🧪️tests/🔐️access",LB+"🌐️remote-cache/🧪️tests",LB+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",LB+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def short(rel):return rel[len(edits.L):] if rel.startswith(edits.L) else rel
def newfiles():
    out=[]
    for d,_,fs in os.walk(HERE+"/new_div"):
        for f in fs:out.append(os.path.relpath(os.path.join(d,f),HERE+"/new_div"))
    return sorted(out)
def apply(rel,text):
    edited=text
    for q,old,new,count in edits.E:
        if q!=rel:continue
        if new!=old and edited.count(new)>=count and count>0:continue
        n=edited.count(old)
        if n!=count:return None,"anchor occurs %d times, expected %d: %r"%(n,count,old[:70])
        edited=edited.replace(old,new)
    back=edited
    for q,old,new,count in reversed(edits.E):
        if q==rel and new!=old:back=back.replace(new,old)
    if back!=text and edited!=text:
        return None,"the edit is not a pure insertion on the live text (a hunk overlaps another change)"
    return edited,None
def plan():
    out={};refused=[]
    for rel in sorted({e[0] for e in edits.E}):
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);edited,why=apply(rel,raw.decode("utf8"))
        if why:refused.append((rel,why));continue
        out[rel]=("same" if edited.encode("utf8")==raw else "edit",edited.encode("utf8"),raw)
    for rel in newfiles():
        data=rd(HERE+"/new_div/"+rel);p=R+"/"+rel
        if os.path.exists(p):
            if rd(p)==data:out[rel]=("same",data,data)
            elif "--replace-new" in sys.argv:out[rel]=("edit",data,rd(p))
            else:refused.append((rel,"new file exists with other content"))
        else:out[rel]=("new",data,b"")
    return out,refused
def rollback(backup):
    for d,_,fs in os.walk(backup):
        for f in fs:
            rel=os.path.relpath(os.path.join(d,f),backup)
            if f.endswith(".absent"):
                p=R+"/"+rel[:-7]
                if os.path.exists(p):os.remove(p)
                continue
            p=R+"/"+rel;open(p+".r49dtmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r49dtmp",p)
def lock_take(owner):
    lock=T+"/landing.lock"
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
    open(lock+"/owner","w").write(owner+"\n");return lock
def lock_drop(lock,owner):
    if os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
def red():
    for rel in newfiles():
        if "/🧪️tests/" not in rel:continue
        p=R+"/"+rel
        if not os.path.exists(p):os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"wb").write(rd(HERE+"/new_div/"+rel))
    for rel in TESTS[:2]:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True);text=r.stdout+r.stderr
        print("RED?",short(rel),"exit",r.returncode,[l for l in text.splitlines() if l.startswith("(fail)")])
def main():
    if "--red" in sys.argv:red();return
    mode=next((m for m in ("--rollback","--land","--dry-run") if m in sys.argv),"--dry-run")
    if mode=="--rollback":
        backup=arg("--rollback");owner="R49d-rollback-%d"%os.getpid();lock=lock_take(owner)
        try:rollback(backup);print("restored from",backup)
        finally:lock_drop(lock,owner)
        return
    out,refused=plan()
    for rel,(how,data,_) in out.items():print(how.ljust(5),short(rel),sha(data)[:12])
    for rel,why in refused:print("REFUSED",short(rel),why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else ""));return
    if refused:sys.exit("refused")
    owner="R49d-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv
    if held:
        if not os.path.isdir(T+"/landing.lock"):sys.exit("--lock-held, but T/landing.lock does not exist")
        lock=T+"/landing.lock"
    else:lock=lock_take(owner)
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R49d-landing-"+datetime.datetime.now().strftime("%H%M%S")
    replaced=[]
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(short(r)+": "+w for r,w in refused))
        before={}
        for rel,(how,data,raw) in out.items():
            if how=="same":continue
            p=R+"/"+rel;before[rel]=sha(raw) if how=="edit" else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if how=="edit":shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r49dtmp","wb").write(data)
        try:
            for rel,(how,data,raw) in out.items():
                if how=="same":continue
                p=R+"/"+rel
                live=rd(p) if os.path.exists(p) else b""
                if sha(live)!=sha(raw):
                    if "--merge" not in sys.argv:raise RuntimeError("live file changed after it was read: "+short(rel))
                    edited,why=apply(rel,live.decode("utf8")) if how=="edit" and rel in {e[0] for e in edits.E} else (None,"new file changed")
                    if why:raise RuntimeError("merge refused for "+short(rel)+": "+why)
                    open(p+".r49dtmp","wb").write(edited.encode("utf8"));data=edited.encode("utf8");before[rel]=sha(live)
                    out[rel]=(how,data,live)
                os.replace(p+".r49dtmp",p);replaced.append(rel)
        except Exception as error:
            for rel in replaced:
                b=backup+"/"+rel
                if os.path.exists(b+".absent"):os.remove(R+"/"+rel)
                elif os.path.exists(b):shutil.copyfile(b,R+"/"+rel)
            for rel in out:
                if os.path.exists(R+"/"+rel+".r49dtmp"):os.remove(R+"/"+rel+".r49dtmp")
            sys.exit("landing aborted and rolled back: "+str(error))
        print("landed",len(replaced),"files; originals in",backup)
        res=[]
        for rel in ([] if "--no-tests" in sys.argv else TESTS):
            cwd=R+"/"+rel;env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R49/art"
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            res.append((rel,r.returncode,failed));print("ok   " if not failed and r.returncode==0 else "RED  ",short(rel),*failed,sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R49d landing (%s): a project record is a function of its inputs (no machine stamps in the neutral bytes), divergences counted per kind in /status\n"%stamp)
            for rel,(how,data,raw) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,short(rel),before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(short(rel),code,len(failed),": "+"; ".join(failed) if failed else ""))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if any(r[1]!=0 for r in res):print("RED test files: compare with the reds recorded in result-R49d.md (derived-nodes, graph-release and generation-protocol were red before), then --rollback "+backup+" if new")
    finally:
        if not held:lock_drop(lock,owner)
main()
