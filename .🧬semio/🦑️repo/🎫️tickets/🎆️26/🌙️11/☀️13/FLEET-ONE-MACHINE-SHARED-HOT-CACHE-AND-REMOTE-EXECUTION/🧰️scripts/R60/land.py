#!/usr/bin/env python3
"""R60 landing: finished owner records outlive the accelerator packs under optional-cache pressure (two-tier least-recently-written retirement), the owners of a wave are counted as present from one listing and ordered present first, and the laws of a stopped read (SIGINT, SIGKILL, edit, restamp, half-written record, record identity) become tests.
Modes: `baseline <out.json>` runs the test list on the live tree and stores the red tests per directory; `land <scratch> <baseline.json>` takes landing.lock, three-way merges the staged files (scratch/base = text at staging time, scratch/work = staged text) onto the live ones, adds the new files, runs the test list and restores everything when a test is red that was green in the baseline."""
import re,os,sys,subprocess,hashlib,datetime,json,shutil,tempfile
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
MB=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/";PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
TESTS=[(MB+"🧪️tests/♻️lru",[]),(MB+"🧪️tests",[]),(PN+"📋️present/🧪️tests",[]),(PN+"🧪️tests/🐌️slow-last",[]),(PN+"🧪️tests/📈️inferred",[]),(PN+"🧪️tests/🚦️governed-dispatch",[]),(PN+"🧪️tests/🚪️away",[]),(PN+"🌐️owner-pool/🧪️tests",[]),
 (B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",[]),(B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests",[]),
 (B+"🧪️tests/🕸️graph/🛟️records-first",[]),(B+"🧪️tests/🕸️graph/🧊️optional-cache",[]),(PN+"🧪️tests/🛟️stopped-wave",[])]
def run_tests(art,only=None):
    out={}
    for at,(rel,files) in enumerate(TESTS):
        if only is not None and at not in only:continue
        path=R+"/"+rel
        if not os.path.isdir(path):out[rel]={"exit":None,"red":[],"skipped":True};print("skip",rel[len(L):],flush=True);continue
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        r=subprocess.run([BUN,"test",*(files or ["./🟦️.ts"]),"--timeout","600000"],cwd=path,env=env,capture_output=True,text=True)
        text=r.stdout+r.stderr
        red=sorted({re.sub(r" \[[0-9.]+ms\]$","",l) for l in text.splitlines() if l.startswith("(fail)")});pass_=[l for l in text.splitlines() if l.strip().endswith(" pass")]
        out[rel]={"exit":r.returncode,"red":red,"pass":pass_[-1].strip() if pass_ else ""};print("ok  " if r.returncode==0 else "RED ",rel[len(L):],*red,sep="\n  " if red else " ",flush=True)
    return out
if sys.argv[1]=="baseline":
    only=None if len(sys.argv)<4 else set(range(int(sys.argv[3].split("-")[0]),int(sys.argv[3].split("-")[1])+1))
    art=os.path.realpath(tempfile.mkdtemp(prefix="r60-art-"));res=run_tests(art,only)
    old=json.load(open(sys.argv[2])) if os.path.exists(sys.argv[2]) else {}
    old.update(res);json.dump(old,open(sys.argv[2],"w"),ensure_ascii=False,indent=1);shutil.rmtree(art,ignore_errors=True);sys.exit(0)
scratch=sys.argv[2];baseline=json.load(open(sys.argv[3]))
mods=[];news=[]
for dp,dn,fn in os.walk(scratch+"/work"):
    for f in fn:
        rel=os.path.relpath(os.path.join(dp,f),scratch+"/work")
        (mods if os.path.exists(scratch+"/base/"+rel) else news).append(L+rel)
mods.sort();news.sort()
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R60-%d\n"%os.getpid())
backup=tempfile.mkdtemp(prefix="r60-backup-",dir=scratch);touched=[];created=[];restored=False
def restore():
    global restored
    for rel in touched:shutil.copy(backup+"/"+rel.replace("/","__"),R+"/"+rel)
    for p in created:
        if os.path.exists(p):os.remove(p)
    restored=True
try:
    plan=[]
    for rel in mods:
        live=R+"/"+rel;key=rel[len(L):];base=scratch+"/base/"+key;work=scratch+"/work/"+key
        if sha(live)==sha(base):plan.append((rel,"replace",work,None))
        else:
            r=subprocess.run(["diff3","-m",live,base,work],capture_output=True)
            if r.returncode!=0:sys.exit("conflict while merging "+rel+" (live changed in the same place)")
            plan.append((rel,"merge",None,r.stdout))
    for rel,how,work,merged in plan:
        shutil.copy(R+"/"+rel,backup+"/"+rel.replace("/","__"));touched.append(rel)
        if how=="replace":shutil.copy(work,R+"/"+rel)
        else:open(R+"/"+rel,"wb").write(merged)
        print(how,rel[len(L):],flush=True)
    for dest in news:
        p=R+"/"+dest;os.makedirs(os.path.dirname(p),exist_ok=True)
        if os.path.exists(p):shutil.copy(p,backup+"/"+dest.replace("/","__"));touched.append(dest)
        else:created.append(p)
        shutil.copy(scratch+"/work/"+dest[len(L):],p);print("new",dest[len(L):],flush=True)
    art=os.path.realpath(tempfile.mkdtemp(prefix="r60-art-"));res=run_tests(art);shutil.rmtree(art,ignore_errors=True)
    newred={rel:[x for x in v["red"] if x not in baseline.get(rel,{}).get("red",[])] for rel,v in res.items()}
    bad={rel:v for rel,v in newred.items() if v}
    broken=[rel for rel,v in res.items() if v["exit"] not in(0,None) and not v["red"] and baseline.get(rel,{}).get("exit")==0]
    if bad or broken:
        print("NEW RED",bad,broken);restore();print("RESTORED the touched files")
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R60 landing (%s): finished owner records outlive the accelerator packs under optional-cache pressure (two-tier retirement); owner records counted as present from one listing and started first; laws of a stopped read%s\n"%(datetime.datetime.now().strftime("%Y-%m-%d %H:%M"),"  (RESTORED: red tests, see result-R60.md)" if restored else ""))
        if not restored:
            for rel in mods:f.write("- edit `LIB/%s` after %s (before %s)\n"%(rel[len(L):],sha(R+"/"+rel),sha(backup+"/"+rel.replace("/","__"))))
            for dest in news:f.write("- new `LIB/%s` after %s\n"%(dest[len(L):],sha(R+"/"+dest)))
        for rel,v in res.items():f.write("- test `LIB/%s` exit %s, %d red%s, baseline red %d%s\n"%(rel[len(L):],v["exit"],len(v["red"]),": "+"; ".join(v["red"]) if v["red"] else "",len(baseline.get(rel,{}).get("red",[])),", "+v.get("pass","") if v.get("pass") else ""))
    sys.exit(1 if restored else 0)
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
