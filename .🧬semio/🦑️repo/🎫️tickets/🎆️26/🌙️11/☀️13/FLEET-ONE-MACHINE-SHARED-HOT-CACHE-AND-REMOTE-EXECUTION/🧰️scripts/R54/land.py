#!/usr/bin/env python3
"""R54 landing record: the files were edited in place (insertions and new files, each re-read before its edit; the sidecar role list, the agent supervision line and the shared role type are the only edits of existing build code, the fleet table the only edit of the replica module). This takes landing.lock, runs the test files of every touched member, and appends the landing with the sha256 of every touched file to landings.md."""
import os,sys,subprocess,hashlib,datetime
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
F="🧰️framework/🛍️products/🦑️repo/🔨️modules/";L=F+"📚️library/";B=L+"🏗️build/";S=B+"🌐️remote-execution/🛰️sidecar/";W=F+"🤝️shared-worktree/"
NEW=[S+"🚦️ready/🟦️.ts",S+"🚦️ready/🏃️wrapper/🟦️.ts",S+"🚦️ready/🧬️schema/🔣️.json",S+"🚦️ready/🧪️tests/🟦️.ts",S+"🚦️ready/🧪️tests/🧫️fixtures/🔣️.json",S+"🚦️ready/🧪️tests/🧫️fixtures/🐍️oracle.py",W+"🩺️attest/🚦️readiness/🟦️.ts"]
EDITED=[S+"🟦️.ts",S+"🧰️shared/🟦️.ts",S+"👷️agent/🟦️.ts",S+"🧪️tests/🔁️restart/🟦️.ts",W+"🔁️replica/⌨️command/🟦️.ts",W+"🔁️replica/📖️texts/🔣️.json",W+"🔁️replica/🧪️tests/🧭️operator.ts",W+"README.md"]
TESTS=[(S+"🚦️ready/🧪️tests","🟦️.ts"),(S+"🧪️tests","🟦️.ts"),(S+"🧪️tests/🔁️restart","🟦️.ts"),(S+"🧪️tests/🧮️closure","🟦️.ts"),(W+"🔁️replica/🧪️tests","🧭️operator.ts")]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R54-%d\n"%os.getpid())
try:
    res=[]
    for rel,name in TESTS:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        r=subprocess.run([BUN,"test","./"+name,"--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;failed=[l for l in out.splitlines() if l.startswith("(fail)")]
        summary=[l.strip() for l in out.splitlines() if l.strip().endswith(("pass","fail"))]
        res.append((rel,name,r.returncode,failed,summary));print("ok   " if not failed and r.returncode==0 else "RED  ",rel,name,*summary,*failed)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R54 landing, seventh part: independent semantic stage digests, reason in ready.json, UTF-8 locale, mirror label (%s): workers make themselves ready (sidecar role `ready`, `ready.json`, readiness table of `fleet`)\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        f.write("Deviation: edited in place before the lock was taken (insertions and new files only, each file re-read before its edit); no before hashes.\n")
        for p in NEW:f.write("- new `%s` after %s\n"%(p,sha(R+"/"+p)))
        for p in EDITED:f.write("- edit `%s` after %s\n"%(p,sha(R+"/"+p)))
        for rel,name,code,failed,summary in res:f.write("- test `%s/%s` exit %d, %s%s\n"%(rel,name,code,", ".join(summary),"; RED: "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
