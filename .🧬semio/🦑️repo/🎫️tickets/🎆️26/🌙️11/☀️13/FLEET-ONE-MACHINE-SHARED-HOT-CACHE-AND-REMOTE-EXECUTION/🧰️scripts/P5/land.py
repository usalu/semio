#!/usr/bin/env python3
"""P5 landing 2 (persistent worker pools, self-starting workers, farm wait away from the governor slot): lands payload.tar in one burst under T/landing.lock.
usage: land.py --dry-run | --land [--authorized] | --rollback <dir>   (--land needs N/LAND-OK or --authorized, the coordinator's go; it refuses when a live file differs from baseline.json)"""
import os,sys,tarfile,tempfile,subprocess,hashlib,json,time,shutil
HERE=os.path.dirname(os.path.abspath(__file__))
R=next(c for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"] if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"))
L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
F="🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests"
TESTS=["🧠️memory/🧪️tests/🚪️away/🟦️.ts","🧠️memory/🧪️tests/🧹️parent-budget/🟦️.ts","🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🚪️away/🟦️.ts","🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🧹️parent-budget/🟦️.ts",F+"/🟦️.ts",F+"/🔒️farm-close/🟦️.ts",F+"/🪣️declined/🟦️.ts",F+"/🔓️release/🟦️.ts",F+"/🪆️captured/🟦️.ts",F+"/♻️retirement/🟦️.ts",F+"/🌊️wave-tally/🟦️.ts"]
sha=lambda b:hashlib.sha256(b).hexdigest()
rd=lambda p:open(p,"rb").read()
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    base=json.load(open(HERE+"/baseline.json",encoding="utf8"))
    if mode=="--rollback":
        d=sys.argv[2]
        for rel in base:
            b=os.path.join(d,rel)
            if os.path.exists(b):shutil.copy(b,R+"/"+rel)
            elif os.path.exists(R+"/"+rel):os.remove(R+"/"+rel)
        print("rolled back");return
    tmp=tempfile.mkdtemp()
    with tarfile.open(HERE+"/payload.tar") as t:t.extractall(tmp)
    bad=[]
    for rel,b in base.items():
        p=R+"/"+rel;live=sha(rd(p)) if os.path.exists(p) else None
        if live==sha(rd(tmp+"/"+rel)):continue
        if live!=b:bad.append((rel,live,b))
    print("files",len(base),"refused",len(bad))
    for r in bad:print(" refused",r[0][-70:],str(r[1])[:12],str(r[2])[:12])
    if mode=="--dry-run" or bad:sys.exit(1 if bad else 0)
    if not(os.path.exists(N+"/LAND-OK") or "--authorized" in sys.argv):sys.exit("N/LAND-OK missing")
    lock=T+"/landing.lock";os.mkdir(lock);open(lock+"/owner","w").write("P5 landing 2 %s\n"%time.strftime("%H:%M",time.gmtime()))
    stamp=time.strftime("%H%M%S",time.gmtime());back=T+"/🗑️generated/P5-landing-"+stamp
    try:
        for rel in base:
            p=R+"/"+rel
            if os.path.exists(p):os.makedirs(os.path.dirname(back+"/"+rel),exist_ok=True);shutil.copy(p,back+"/"+rel)
        for rel in base:
            p=R+"/"+rel;os.makedirs(os.path.dirname(p),exist_ok=True);shutil.copy(tmp+"/"+rel,p+".p5tmp")
        for rel in base:os.replace(R+"/"+rel+".p5tmp",R+"/"+rel)
        print("landed; originals in",back)
        for rel in TESTS:
            d=os.path.dirname(L+"/"+rel)
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=d,capture_output=True,text=True,env={**{k:v for k,v in os.environ.items() if not k.startswith("SEMIO_FLEET") and not k.startswith("SEMIO_REMOTE_CACHE")},"BUN_RUNTIME_TRANSPILER_CACHE_PATH":"0"})
            tail=[l for l in (r.stdout+r.stderr).splitlines() if l.strip().endswith(("pass","fail")) or l.startswith("(fail)")]
            print(rel[-60:],r.returncode,tail[-6:])
        open(T+"/landings.md","a").write("\n## Package P5 landing 2 (%s UTC)\n%d files, originals in %s\n"%(time.strftime("%H:%M",time.gmtime()),len(base),back))
    finally:shutil.rmtree(lock)
main()
