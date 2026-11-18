#!/usr/bin/env python3
"""R59 landing: packed record form (observation rows stored once, closure entries streamed layer by layer, held documents not downloaded again).
Modes: `baseline` runs the test list on the live tree and stores the red tests per directory; `land` takes landing.lock, three-way merges the staged files onto the live ones (a file whose live text equals the staged base is replaced, otherwise diff3 merges the staged change onto it and a conflict aborts), writes the new files, runs the same test list and restores every touched file if a test is red that was green in the baseline; the landing is appended to landings.md. Staged files live outside the worktree (scratch dir given as argument)."""
import os,sys,subprocess,hashlib,datetime,json,shutil,tempfile
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
G=B+"🪪️graph-state/";PR=G+"📦️project-records/";SI=B+"🕸️graph/📥️inference/🧭️script-inputs/"
MOD=[PR+"🧳️portable/🟦️.ts",PR+"🟦️.ts",PR+"🧬️schema/🔣️.json",G+"🛰️exchange/🟦️.ts",G+"🧬️schema/🔣️.json",SI+"🟨️.mjs",SI+"💾️entries/🟨️.mjs",SI+"🟨️.d.mts"]
NEW=[(PR+"🗜️packed-rows/"+r,"stage/packed-rows/"+r) for r in["🟦️.ts","🧬️schema/🔣️.json","🧪️tests/🟦️.ts","🧪️tests/🐍️oracle.py","🧪️tests/🧫️fixtures/🔣️.json"]]+[(PR+"🧪️tests/🗜️packed/🟦️.ts","work-tests/🪪️graph-state/📦️project-records/🧪️tests/🗜️packed/🟦️.ts")]
TESTS=[(PR+"🗜️packed-rows/🧪️tests",[]),(PR+"🧳️portable/🧪️tests",[]),(PR+"🧳️portable/🧪️tests/🕰️stamps",[]),(PR+"🧳️portable/🧪️tests/🌍️real",[]),(PR+"🧪️tests",[]),(PR+"🧪️tests/✂️after-cut",[]),(PR+"🧪️tests/🌍️two-platforms",[]),(PR+"🧪️tests/🚚️closures",[]),(PR+"🧪️tests/🗜️packed",[]),(G+"🧪️tests",[]),(G+"🙋️wanted/🧪️tests",[]),
 (SI+"🧪️tests",["./closure-store-limits.test.ts","./closure-answer-refresh.test.ts"]),(B+"🕸️graph/📥️inference/🧪️tests/🧺️bundle-rows",[]),(B+"🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests",[]),(B+"🌐️remote-cache/🖥️server/🧪️tests/🧭️heads",[]),(B+"🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests",[]),(B+"👁️observation/🧪️tests",[]),
 (SI+"🔌️provider-closures/🧪️tests/🛃️preflight",[]),(SI+"🪆️heavy-questions/🧪️tests",[])]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def run_tests(art):
    out={}
    for rel,files in TESTS:
        path=R+"/"+rel
        if not os.path.isdir(path):out[rel]={"exit":None,"red":[],"skipped":True};print("skip",rel[len(L):]);continue
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        r=subprocess.run([BUN,"test",*(files or ["./🟦️.ts"]),"--timeout","180000"],cwd=path,env=env,capture_output=True,text=True)
        red=sorted({l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")});pass_=[l for l in (r.stdout+r.stderr).splitlines() if l.strip().endswith(" pass")]
        out[rel]={"exit":r.returncode,"red":red,"pass":pass_[-1].strip() if pass_ else ""};print("ok  " if r.returncode==0 else "RED ",rel[len(L):],*red,sep="\n  " if red else " ",flush=True)
    return out
if sys.argv[1]=="baseline":
    art=tempfile.mkdtemp(prefix="r59-art-");res=run_tests(art);json.dump(res,open(sys.argv[2],"w"),ensure_ascii=False,indent=1);shutil.rmtree(art,ignore_errors=True);sys.exit(0)
scratch=sys.argv[2];baseline=json.load(open(sys.argv[3]))
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R59-%d\n"%os.getpid())
backup=tempfile.mkdtemp(prefix="r59-backup-",dir=scratch);touched=[];created=[];restored=False
def restore():
    global restored
    for rel in touched:shutil.copy(backup+"/"+rel.replace("/","__"),R+"/"+rel)
    for p in created:
        if os.path.exists(p):os.remove(p)
    restored=True
try:
    plan=[]
    for rel in MOD:
        live=R+"/"+rel;base=scratch+"/base/"+rel[len(L+"🏗️build/"):];work=scratch+"/work/"+rel[len(L+"🏗️build/"):]
        if sha(live)==sha(base):plan.append((rel,"replace",work,None))
        else:
            r=subprocess.run(["diff3","-m",live,base,work],capture_output=True)
            if r.returncode!=0:sys.exit("conflict while merging "+rel+" (live changed in the same place)")
            plan.append((rel,"merge",None,r.stdout))
    for rel,how,work,merged in plan:
        shutil.copy(R+"/"+rel,backup+"/"+rel.replace("/","__"));touched.append(rel)
        if how=="replace":shutil.copy(work,R+"/"+rel)
        else:open(R+"/"+rel,"wb").write(merged)
        print(how,rel[len(B):])
    for dest,src in NEW:
        p=R+"/"+dest;os.makedirs(os.path.dirname(p),exist_ok=True)
        if os.path.exists(p):shutil.copy(p,backup+"/"+dest.replace("/","__"));touched.append(dest)
        else:created.append(p)
        shutil.copy(scratch+"/"+src,p)
    art=tempfile.mkdtemp(prefix="r59-art-");res=run_tests(art);shutil.rmtree(art,ignore_errors=True)
    newred={rel:[x for x in v["red"] if x not in baseline.get(rel,{}).get("red",[])] for rel,v in res.items()}
    bad={rel:v for rel,v in newred.items() if v}
    broken=[rel for rel,v in res.items() if v["exit"] not in(0,None) and not v["red"] and baseline.get(rel,{}).get("exit")==0]
    if bad or broken:
        print("NEW RED",bad,broken);restore();print("RESTORED the touched files")
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R59 landing (%s): packed record form, observation rows stored once; closure entries streamed layer by layer; documents a taker holds are not downloaded again%s\n"%(datetime.datetime.now().strftime("%Y-%m-%d %H:%M"),"  (RESTORED: red tests, see result-R59.md)" if restored else ""))
        if not restored:
            for rel in MOD:f.write("- edit `LIB/%s` after %s (before %s)\n"%(rel[len(L):],sha(R+"/"+rel),sha(backup+"/"+rel.replace("/","__"))))
            for dest,_ in NEW:f.write("- new `LIB/%s` after %s\n"%(dest[len(L):],sha(R+"/"+dest)))
        for rel,v in res.items():f.write("- test `LIB/%s` exit %s, %d red%s, baseline red %d%s\n"%(rel[len(L):],v["exit"],len(v["red"]),": "+"; ".join(v["red"]) if v["red"] else "",len(baseline.get(rel,{}).get("red",[])),", "+v.get("pass","") if v.get("pass") else ""))
    sys.exit(1 if restored else 0)
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
