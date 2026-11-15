#!/usr/bin/env python3
"""R1 landing (a dependency store wait names its holder and never waits for an ancestor; a holder whose process is gone is reclaimed and reported): applies the anchored edits of payload.py (every old text exactly once, an edit whose new text is already present is skipped) in one burst under T/landing.lock.
usage: land.py --dry-run | --land [--lock-held] [--no-tests] | --rollback <backup-dir>      (--root <dir> uses another tree, for rehearsals)
  --dry-run applies the edits in memory, parses every edited TypeScript file with the bun transpiler, checks that every relative import of the new text resolves and prints the plan; it writes nothing into the tree.
  --land takes T/landing.lock (mkdir + owner file; a lock older than 40 minutes is taken over; with --lock-held the caller holds it), re-reads every live file, refuses when an old text does not occur exactly once, writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/R1-landing-<time>/ and runs the test files of payload.TESTS.
  The real `repo` build command of the landing rules is not run by this script."""
import os,sys,subprocess,hashlib,time,shutil,datetime,re
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
import payload
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md"):return c
    sys.exit("no worktree root found")
R=root()
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def plan():
    out={};refused=[]
    for rel in dict.fromkeys(e[0] for e in payload.EDITS):
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);text=raw.decode("utf8")
        for path,old,new in payload.EDITS:
            if path!=rel:continue
            if old!=new and new in text and text.count(old)==0:continue
            n=text.count(old)
            if n!=1:refused.append((rel,"old text occurs %d times, expected 1: %r"%(n,old[:70])));break
            text=text.replace(old,new)
        else:
            out[rel]=("same" if text.encode("utf8")==raw else "edit",raw,text.encode("utf8"))
    return out,refused
def check(out):
    bad=[]
    for rel,(how,raw,data) in out.items():
        if how=="same" or not rel.endswith(".ts"):continue
        tmp=os.path.join(HERE,"syntax-check.ts");open(tmp,"wb").write(data)
        r=subprocess.run([BUN,"build","--no-bundle","--target=bun",tmp],capture_output=True)
        if r.returncode!=0:bad.append((rel,(r.stdout+r.stderr).decode("utf8","replace")[:600]))
        for m in re.finditer(r'from"(\.[^"]+)"',data.decode("utf8")):
            target=os.path.normpath(os.path.join(os.path.dirname(R+"/"+rel),m.group(1)))
            if not os.path.exists(target):bad.append((rel,"import does not resolve: "+m.group(1)[-90:]))
    return bad
def main():
    if "--rollback" in sys.argv:
        b=arg("--rollback")
        for dp,_,fs in os.walk(b):
            for f in fs:
                src=os.path.join(dp,f);rel=os.path.relpath(src,b)
                if rel.endswith(".absent"):
                    t=R+"/"+rel[:-7]
                    if os.path.exists(t):os.remove(t)
                else:shutil.copyfile(src,R+"/"+rel)
        print("rolled back from",b);return
    out,refused=plan()
    for rel,(how,raw,data) in out.items():print(how.ljust(5),rel[-100:],sha(raw)[:12],"->",sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel[-90:],why)
    bad=check(out)
    for rel,why in bad:print("INVALID",rel[-90:],why)
    if "--dry-run" in sys.argv or "--land" not in sys.argv:
        print("dry run:",len(out),"files,",len(refused),"refused,",len(bad),"invalid");return
    if refused or bad:sys.exit("refused")
    lock=T+"/landing.lock";held="--lock-held" in sys.argv;owner="R1-%d-%d"%(os.getpid(),int(time.time()))
    if not held:
        for _ in range(240):
            try:
                os.mkdir(lock);break
            except FileExistsError:
                age=time.time()-os.path.getmtime(lock)
                if age>40*60:
                    shutil.rmtree(lock,ignore_errors=True);continue
                time.sleep(5)
        else:sys.exit("landing lock busy")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R1-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        for rel,(how,raw,data) in out.items():
            if how=="same":continue
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True);shutil.copyfile(R+"/"+rel,backup+"/"+rel)
            open(R+"/"+rel+".r1tmp","wb").write(data)
        for rel,(how,raw,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r1tmp",R+"/"+rel)
        print("landed",sum(1 for v in out.values() if v[0]!="same"),"files; originals in",backup)
        res=[]
        for rel in ([] if "--no-tests" in sys.argv else payload.TESTS):
            cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            verdict="as expected" if r.returncode==0 and not failed else "UNEXPECTED"
            res.append((rel,r.returncode,len(failed),verdict,text[-1500:]));print(verdict.ljust(11),rel[-80:],"fail",len(failed),*failed,sep="\n  " if failed else " ")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R1 landing (%s): the dependency store wait names its holder (pid, command, start), a holder that is an ancestor of the waiter is refused after a grace instead of waited for, an entry of a holder whose process is gone is reclaimed with a log line\n"%stamp)
            for rel,(how,raw,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,rel[len("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"):] if rel.startswith("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/") else rel,sha(raw)[:12],sha(data)[:12]))
            for rel,code,failed,verdict,_ in res:f.write("- test `%s` exit %d, %d red: %s\n"%(rel[-70:],code,failed,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        for rel,code,failed,verdict,text in res:
            if verdict=="UNEXPECTED":print("UNEXPECTED test result: inspect, then --rollback "+backup);print(text)
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
