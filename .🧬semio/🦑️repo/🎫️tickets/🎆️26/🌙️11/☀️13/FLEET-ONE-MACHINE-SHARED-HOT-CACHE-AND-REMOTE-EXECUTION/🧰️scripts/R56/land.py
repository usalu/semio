#!/usr/bin/env python3
"""R56 landing. Usage: land.py --stage claims|nxplatform|provenance [--dry-run]. Takes landing.lock, applies anchored edits (each old text must occur exactly once; an already applied edit is skipped), runs the standing preflight tests and the tests of the touched members, records sha256 in landings.md. On a red test the edited files are restored from the backup folder printed at the start."""
import os,sys,subprocess,hashlib,datetime,shutil,json
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";C=L+"⚡️caching/";FF=C+"🚀️bootstrap/🛰️fleet-fetch/"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import edits
STAGES=edits.stages(L,B,C,FF)
stage=sys.argv[sys.argv.index("--stage")+1];S=STAGES[stage]
PREFLIGHT=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
dry="--dry-run" in sys.argv
lock=T+"/landing.lock"
def tests(dirs):
    res=[]
    for rel in dirs:
        d=R+"/"+rel
        if not os.path.isdir(d):res.append((rel,-1,["missing dir"]));print("MISS",rel[len(L):]);continue
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=d,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;failed=[l[:160] for l in out.splitlines() if l.startswith("(fail)")]
        res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
    return res
def readers():
    try:
        r=json.load(open(R+"/.cache/f/graph-server/read-record.json"));out=[]
        for k in("runner","read"):
            try:os.kill(r[k],0);out.append(str(r[k]))
            except Exception:pass
        return out
    except Exception:return []
plan=[]
for rel,pairs in S["edits"].items():
    p=R+"/"+rel;text=open(p,encoding="utf8").read();new=text
    for old,rep in pairs:
        if rep in new and old not in new:continue
        if new.count(old)!=1:sys.exit("anchor not unique/absent in %s: %s"%(rel,old[:70]))
        new=new.replace(old,rep)
    plan.append((rel,p,text,new))
if dry:
    print("dry run ok",[(rel[len(L):],t!=n) for rel,p,t,n in plan]);sys.exit(0)
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R56-%s-%d %s\n"%(stage,os.getpid(),datetime.datetime.now().isoformat()))
alive0=readers()
backup=T+"/🗑️generated/R56-landing-%s-%s"%(stage,datetime.datetime.now().strftime("%H%M%S"))
try:
    os.makedirs(backup);print("backup",backup)
    mine=[os.path.dirname(n) for n in S.get("new",[])]
    foreign=[d for d in S["tests"] if not any(m==R and False or (L+m[len(L):]).startswith(d) for m in mine)]
    base={rel:(code,failed) for rel,code,failed in tests(foreign)} if foreign else {}
    print("baseline red:",{k[len(L):]:v[1] for k,v in base.items() if v[0]!=0})
    before={}
    for rel,p,text,new in plan:
        before[rel]=sha(p)
        with open(backup+"/"+hashlib.sha256(rel.encode()).hexdigest()[:16],"w",encoding="utf8") as f:f.write(text)
        if new!=text:open(p,"w",encoding="utf8").write(new)
    json.dump({hashlib.sha256(rel.encode()).hexdigest()[:16]:rel for rel,*_ in plan},open(backup+"/index.json","w"))
    res=tests(PREFLIGHT+S["tests"])
    red=[x for x in res if x[1]!=0 and not (x[0] in base and base[x[0]][0]!=0 and set(x[2])<=set(base[x[0]][1]))]
    if red and "--keep" not in sys.argv:
        for rel,p,text,new in plan:open(p,"w",encoding="utf8").write(text)
        print("RED: edits restored");sys.exit(1)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R56 landing `%s` (%s): %s\n"%(stage,datetime.datetime.now().strftime("%Y-%m-%d %H:%M"),S["title"]))
        for n in S.get("new",[]):f.write("- new `LIB/%s`\n"%n[len(L):])
        for rel,p,text,new in plan:f.write("- edit `%s` before %s after %s\n"%(rel[len(L):],before[rel],sha(p)))
        f.write("- pids %s alive before %s after %s\n"%("graph-read",alive0,readers()))
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
