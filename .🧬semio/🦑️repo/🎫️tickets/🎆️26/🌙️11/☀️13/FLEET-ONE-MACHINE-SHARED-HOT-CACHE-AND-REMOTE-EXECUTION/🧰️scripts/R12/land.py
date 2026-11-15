#!/usr/bin/env python3
"""R12 landing (fleet across networks: identity instead of CIDR, topology facts instead of literals, read-through between groups). Payload: R12-bundle.json beside this script (unified diffs against the files the work was made from, new files in full).
usage: land.py --dry-run | --land [--authorized] [--no-tests] | --rollback <backup-dir>      (--root <dir> uses another tree, for rehearsals)
  --dry-run  writes nothing: per file `copy` (live equals the baseline, the diff gives exactly the tested result), `merge` (live changed elsewhere, the diff still applies by context), `same` (already landed), REFUSED (does not apply).
  --land     needs N/LAND-OK-R12 (or --authorized), takes landing.lock of the build ticket AND 1113-landing.lock of the shared-worktree ticket (the topology member is part of the replica code), keeps the originals in T/🗑️generated/R12-landing-<time>/, replaces all files back to back, runs the parse proof and every test file of the touched members and of the whole shared-worktree module, and records the landing in landings.md and 1113-landings.md.
  It never removes a lock it does not own. The `repo` build command of the landing rules and the replica rollout (see result-R12.md) are not run by this script."""
import os,sys,json,subprocess,hashlib,time,shutil,datetime,tempfile,ast
HERE=os.path.dirname(os.path.abspath(__file__))
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
def root():
    if arg("--root"):return os.path.abspath(arg("--root"))
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root()
TICKETS=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11"
T=TICKETS+"/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=TICKETS+"/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
SW_TICKET=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
MODULES="🧰️framework/🛍️products/🦑️repo/🔨️modules"
B=MODULES+"/📚️library/🏗️build"
SW=MODULES+"/🤝️shared-worktree"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
TESTS=[B+"/"+t for t in [
 "🌐️remote-cache/🧭️peers/🧪️tests/🟦️.ts","🌐️remote-cache/🛰️fleet/🪪️identity/🧪️tests/🟦️.ts","🌐️remote-cache/🛰️fleet/🧪️tests/🟦️.ts",
 "🌐️remote-cache/🖥️server/🧪️tests/🟦️.ts","🌐️remote-cache/🖥️server/🧪️tests/🔐️access/🟦️.ts","🌐️remote-cache/🧪️tests/🟦️.ts",
 "🌐️remote-execution/🛰️sidecar/🧪️tests/🟦️.ts","🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🟦️.ts","🌐️remote-execution/⌨️command/🧪️tests/🟦️.ts",
 "🌐️remote-execution/🧪️tests/🌐️fleet/🟦️.ts","🌐️remote-execution/🚀️offload/🧪️tests/🌐️cluster/🔏️signed-fleet/🟦️.ts","🚢️fleet-status/🧪️tests/🟦️.ts","🪪️graph-state/🙋️wanted/🧪️tests/🟦️.ts"]]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def module_tests():
    found=[]
    for d,_,fs in os.walk(R+"/"+SW):
        if "node_modules" in d:continue
        if os.path.basename(d)=="🧪️tests" or "/🧪️tests/" in d:
            found+= [os.path.relpath(os.path.join(d,f),R) for f in sorted(fs) if f.endswith(".ts")]
    return sorted(found)
def apply_patch(live_path,diff):
    with tempfile.TemporaryDirectory() as d:
        p=d+"/d.patch";open(p,"w",encoding="utf8").write(diff)
        r=subprocess.run(["patch","-s","-F3","--no-backup-if-mismatch","-o",d+"/out",live_path,p],capture_output=True,text=True)
        if r.returncode!=0 or not os.path.exists(d+"/out"):return None,(r.stdout+r.stderr).strip()[:200]
        return rd(d+"/out"),""
def plan(bundle):
    out={};refused=[]
    for rel,diff in sorted(bundle["patches"].items()):
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p)
        if sha(raw)==bundle["result"][rel]:out[rel]=("same",raw);continue
        data,why=apply_patch(p,diff)
        if data is None:refused.append((rel,"does not apply: "+why));continue
        exact=sha(raw)==bundle["baseline"][rel]
        out[rel]=("copy" if exact and sha(data)==bundle["result"][rel] else "merge",data)
    for rel,text in sorted(bundle["new"].items()):
        p=R+"/"+rel;data=text.encode("utf8")
        if os.path.exists(p):
            if rd(p)==data:out[rel]=("same",data)
            else:refused.append((rel,"new file exists with other content"))
        else:out[rel]=("new",data)
    return out,refused
def parse_proof(files):
    bad=[]
    for rel in files:
        p=R+"/"+rel
        if rel.endswith(".ts"):
            r=subprocess.run([BUN,"build","--no-bundle",p],capture_output=True,text=True)
            if r.returncode!=0:bad.append((rel,r.stderr[:200]))
        elif rel.endswith(".json"):
            try:json.load(open(p,encoding="utf8"))
            except Exception as e:bad.append((rel,str(e)))
        elif rel.endswith(".py"):
            try:ast.parse(open(p,encoding="utf8").read())
            except Exception as e:bad.append((rel,str(e)))
    return bad
def take(lock,owner):
    try:os.mkdir(lock)
    except FileExistsError:
        try:who=open(lock+"/owner").read().strip()
        except OSError:who="unknown"
        return who
    open(lock+"/owner","w").write(owner+"\n")
    return None
def give(lock,owner):
    try:
        if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
    except OSError:pass
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    if mode=="--rollback":
        backup=sys.argv[2];lock=T+"/landing.lock";owner="R12-rollback-%d"%os.getpid()
        who=take(lock,owner)
        if who:sys.exit("landing.lock is held by "+who)
        try:
            for d,_,fs in os.walk(backup):
                for f in fs:
                    rel=os.path.relpath(os.path.join(d,f),backup)
                    if f.endswith(".absent"):
                        p=R+"/"+rel[:-7]
                        if os.path.exists(p):os.remove(p)
                        continue
                    p=R+"/"+rel;open(p+".r12tmp","wb").write(rd(os.path.join(d,f)));os.replace(p+".r12tmp",p)
            print("restored from",backup)
        finally:give(lock,owner)
        return
    bundle=json.load(open(HERE+"/R12-bundle.json",encoding="utf8"))
    out,refused=plan(bundle)
    for rel,(how,data) in out.items():print(how.ljust(5),rel[len(MODULES)+1:] if rel.startswith(MODULES) else rel,sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel[len(MODULES)+1:],why)
    if mode!="--land":print("dry run: nothing written"+("; %d refused"%len(refused) if refused else "; clean"));return
    if refused:sys.exit("refused: a live file changed in the lines of the diff, rebuild the payload")
    if not os.path.exists(N+"/LAND-OK-R12") and "--authorized" not in sys.argv:sys.exit("LAND-OK-R12 does not exist: the owner lands this package (it restarts the group services and moves the replica code digest)")
    owner="R12-%d-%d"%(os.getpid(),int(time.time()));locks=[T+"/landing.lock",SW_TICKET+"/1113-landing.lock"];held=[]
    try:
        for lock in locks:
            who=take(lock,owner)
            if who:sys.exit(lock+" is held by "+who+": wait, never remove it")
            held.append(lock)
        out,refused=plan(bundle)
        if refused:sys.exit("refused after taking the locks: "+"; ".join(r for r,_ in refused))
        stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R12-landing-"+datetime.datetime.now().strftime("%H%M%S");before={}
        for rel,(how,data) in out.items():
            if how=="same":continue
            p=R+"/"+rel;before[rel]=sha(rd(p)) if os.path.exists(p) else "-"
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if os.path.exists(p):shutil.copyfile(p,backup+"/"+rel)
            else:open(backup+"/"+rel+".absent","w").close()
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".r12tmp","wb").write(data)
        for rel,(how,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r12tmp",R+"/"+rel)
        print("landed",sum(1 for h,_ in out.values() if h!="same"),"files; originals in",backup)
        bad=parse_proof(list(out))
        for rel,why in bad:print("PARSE FAIL",rel,why)
        res=[]
        tests=[] if "--no-tests" in sys.argv else TESTS+[t for t in module_tests() if t not in TESTS]
        for rel in tests:
            cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            failed=[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")]
            res.append((rel,r.returncode,failed));print(("ok  " if r.returncode==0 else "RED ")+rel[len(MODULES)+1:],*failed,sep="\n  " if failed else " ")
        for name,where in(("landings.md",T),("1113-landings.md",SW_TICKET)):
            with open(where+"/"+name,"a",encoding="utf8") as f:
                f.write("\n## Package R12 landing (%s): the fleet across networks (signed identity instead of CIDR, ports and hosts from the topology, read-through between groups)\n"%stamp)
                for rel,(how,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,rel[len(MODULES)+1:] if rel.startswith(MODULES) else rel,before.get(rel,"-")[:12],sha(data)[:12]))
                for rel,code,failed in res:f.write("- test `%s` exit %d, %d red\n"%(rel[len(MODULES)+1:],code,len(failed)))
                f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        if bad or any(code!=0 for _,code,_ in res):print("RED result: inspect, then --rollback "+backup)
    finally:
        for lock in reversed(held):give(lock,owner)
main()
