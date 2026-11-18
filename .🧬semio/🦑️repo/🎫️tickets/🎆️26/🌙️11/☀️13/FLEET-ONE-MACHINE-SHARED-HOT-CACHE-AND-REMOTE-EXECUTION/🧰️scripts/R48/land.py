#!/usr/bin/env python3
"""R48 landing: a slice load that grows past the plan memory is cut smaller (fewer projects at once, smaller waves) instead of refused.
The hunks are already in the live tree (this package was developed in the live tree, not in a copy: recorded in result-R48.md). This script takes the landing lock, verifies every hunk and new file against edits.py and new/, runs the test files, records the landing in landings.md and checks that the graph read pids survived.
usage: land.py --verify | --land [--lock-held] | --rollback
"""
import os,sys,subprocess,hashlib,time,datetime
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
B=edits.L;PIDS=["31945","31946","32013"]
TESTS=[B+"🧠️memory/✂️cut/🧪️tests",B+"🧠️memory/🪜️plan-stage/🧪️tests",B+"🕸️graph/🧪️tests/🪜️plan-stages"]+[B+"🏡️resident/🧪️tests/"+d for d in ("✂️cut-load","🛡️plan-guard","🪓️plan-limit","🐧️leaf-ceiling","🛟️reservation","🪜️plan-refusal","🔁️restart","📐️booking","🧷️history","🛑️cancel","📨️protocol")]
def rd(p):
    with open(p,"rb") as f:return f.read()
def sha(b):return hashlib.sha256(b).hexdigest()
def alive():
    out=subprocess.run(["ps","-axo","pid"],capture_output=True,text=True).stdout.split()
    return [p for p in PIDS if p in out]
def newfiles():
    out=[]
    for d,_,fs in os.walk(HERE+"/new"):
        for f in fs:out.append(os.path.relpath(os.path.join(d,f),HERE+"/new"))
    return sorted(out)
def verify():
    bad=[]
    for f,o,n,c in edits.E:
        if open(R+"/"+f,encoding="utf8").read().count(n)!=c:bad.append(f+": hunk missing "+n[:50])
    for f,key,c in edits.SCHEMA_KEYS:
        if open(R+"/"+f,encoding="utf8").read().count(key)!=c:bad.append(f+": key "+key)
    for rel in newfiles():
        p=R+"/"+rel
        if not os.path.exists(p) or rd(p)!=rd(HERE+"/new/"+rel):bad.append(rel+": differs from new/")
    return bad
def rollback():
    for f,o,n,c in reversed(edits.E):
        p=R+"/"+f;t=open(p,encoding="utf8").read()
        if t.count(n)==c:open(p,"w",encoding="utf8").write(t.replace(n,o))
    for rel in newfiles():
        if os.path.exists(R+"/"+rel):os.remove(R+"/"+rel)
def main():
    mode=next((m for m in ("--rollback","--land","--verify") if m in sys.argv),"--verify")
    if mode=="--rollback":rollback();print("rolled back (schema keys host-slice-owners-cut stay: remove by hand if wanted)");return
    bad=verify()
    for b in bad:print("BAD",b)
    if mode=="--verify":print("verified" if not bad else "not verified");return
    if bad:sys.exit("not landing")
    owner="R48-%d-%d"%(os.getpid(),int(time.time()));held="--lock-held" in sys.argv;lock=T+"/landing.lock"
    if not held:
        try:os.mkdir(lock)
        except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
        open(lock+"/owner","w").write(owner+"\n")
    before=alive();res=[]
    try:
        for rel in TESTS:
            env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
            r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
            text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
            res.append((rel,r.returncode,failed));print("ok   " if not failed and r.returncode==0 else "RED  ",rel[len(B):],*failed)
        after=alive();stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R48 landing (%s): a slice load past the plan memory is cut smaller and continues instead of being refused\n"%stamp)
            for e in edits.E:f.write("- edit `LIB/🏗️build/%s` hunk %s\n"%(e[0][len(B):],sha(e[2].encode())[:12]))
            for rel in newfiles():f.write("- new `LIB/🏗️build/%s` %s\n"%(rel[len(B):],sha(rd(R+"/"+rel))[:12]))
            for rel,code,failed in res:f.write("- test `LIB/🏗️build/%s` exit %d, %d red%s\n"%(rel[len(B):],code,len(failed),": "+"; ".join(failed) if failed else ""))
            f.write("- pids %s alive before %s after %s\n"%(",".join(PIDS),before,after))
    finally:
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
main()
