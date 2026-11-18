#!/usr/bin/env python3
"""R51 landing 1: removes the tool row `typescript-package-index` (R29 import-hub row whose closure reaches the generated `@semio-tech/ui-rs:generate` output through `🛂️manifest`) from the provider-closure policy and from its corpus. Takes landing.lock, edits the two live files anchored (re-read right before), runs the provider-closure law tests, appends the landing to landings.md."""
import os,sys,json,subprocess,hashlib,datetime
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
P=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/"
FILES=[P+"🔣️.json",P+"🧪️tests/🔣️.json"]
TESTS=[P+"🧪️tests",P+"🧪️tests/🪡️tool-cover",P+"🧪️tests/🧮️blast-radius"]
KEY='"typescript-package-index": {'
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def cut(text):
    i=text.index(KEY);i=text.rindex("\n",0,i)+1;j=text.index(KEY);depth=0;k=j+len(KEY)-1
    while True:
        c=text[k]
        if c=="{":depth+=1
        elif c=="}":
            depth-=1
            if depth==0:break
        k+=1
    k+=1
    assert text[k]==","; k+=1
    assert text[k]=="\n"; k+=1
    return text[:i]+text[k:]
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R51-%d\n"%os.getpid())
try:
    before={}
    for f in FILES:
        text=open(f,encoding="utf8").read()
        if KEY not in text:print("already absent",f);continue
        before[f]=sha(f);new=cut(text);json.loads(new);assert KEY not in new
        tmp=f+".r51tmp";open(tmp,"w",encoding="utf8").write(new);os.replace(tmp,f)
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    res=[]
    for d in TESTS:
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","240000"],cwd=d,env=env,capture_output=True,text=True)
        red=[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")];res.append((d,r.returncode,red));print("ok  " if r.returncode==0 else "RED ",d[len(P):] or ".",*red)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R51 landing 1 (%s): tool row `typescript-package-index` removed (its closure imports the generated `@semio-tech/ui-rs:generate` output via `🛂️manifest`; the refusal killed the 90-minute group read at 17:11 UTC)\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        for p in FILES:f.write("- edit `%s` %s to %s\n"%(p[len(R)+1:],before.get(p,"-"),sha(p)))
        for d,c,red in res:f.write("- test `%s` exit %d, %d red%s\n"%(d[len(R)+1:],c,len(red),": "+"; ".join(red) if red else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
