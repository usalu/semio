#!/usr/bin/env python3
"""R49 landing record. The files below were edited in place in the live tree (each re-read before its edit, every edit an insertion or a new file), so this script does not replace anything: it takes landing.lock, runs the test files of every touched member and its direct dependants, and appends the landing to landings.md with the sha256 of every touched file. No originals exist for the edited files (not captured before the edit); new files are listed as new."""
import os,sys,subprocess,hashlib,datetime
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";I=B+"🕸️graph/📥️inference/📦️project-nodes/";G=B+"🪪️graph-state/";S=B+"🌐️remote-execution/🛰️sidecar/"
NEW=[I+"🌐️owner-pool/🟦️.ts",I+"🌐️owner-pool/🧬️schema/🔣️.json",I+"🌐️owner-pool/📣️publisher/🟦️.ts",I+"🌐️owner-pool/📊️status/🟦️.ts",I+"🌐️owner-pool/🔒️guard/🟦️.ts",I+"🌐️owner-pool/🏃️runner/🟨️.mjs",I+"🌐️owner-pool/🧪️tests/🟦️.ts",I+"🌐️owner-pool/🧪️tests/🧫️fixtures/🔣️.json",I+"🌐️owner-pool/🧪️tests/🧫️fixtures/🐍️oracle.py",S+"🏊️pool/🟦️.ts",S+"🧪️tests/🏊️pool/🟦️.ts",B+"🌐️remote-cache/🖥️server/🧪️tests/🧭️heads/🟦️.ts"]
EDITED=[G+"🛰️exchange/📡️client/🟦️.ts",G+"🙋️wanted/🟦️.ts",G+"🙋️wanted/💰️cost/🟦️.ts",B+"🌐️remote-cache/🖥️server/🟦️.ts",I+"🌐️owner-pool/🟦️.ts",G+"🛰️exchange/🟦️.ts",G+"📦️project-records/🟦️.ts",I+"🟨️.mjs",I+"🏊️project-farm/👷️service/🟨️.mjs",B+"👁️observation/🟨️.mjs",B+"👁️observation/🟨️.d.mts",B+"👁️observation/🧪️tests/🧫️fixtures/🔣️.json",S+"🟦️.ts",S+"🧬️schema/🔣️.json",S+"🧰️shared/🟦️.ts",S+"👷️agent/🟦️.ts"]
TESTS=[B+"🌐️remote-cache/🖥️server/🧪️tests/🧭️heads",G+"🙋️wanted/💰️cost/🧪️tests",I+"🌐️owner-pool/🧪️tests",G+"📦️project-records/🧪️tests",G+"🧪️tests",G+"🙋️wanted/🧪️tests",B+"👁️observation/🧪️tests",I+"🏊️project-farm/🧪️tests",I+"🧪️tests/🐌️slow-last",I+"🧪️tests/📈️inferred",I+"🧪️tests/🚪️away",S+"🧪️tests",S+"🧪️tests/🔁️restart",S+"🧪️tests/🕸️graph",S+"🧪️tests/🧮️closure",S+"🧪️tests/🏊️pool",B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
owner="R49-%d"%os.getpid();open(lock+"/owner","w").write(owner+"\n")
try:
    art=os.path.join(os.environ.get("TMPDIR","/tmp"),"r49-art");os.makedirs(art,exist_ok=True)
    res=[]
    for rel in TESTS:
        env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env["SEMIO_TEST_ARTIFACT_DIR"]=art
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
        failed=[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")];res.append((rel,r.returncode,failed));print("ok   " if not failed and r.returncode==0 else "RED  ",rel[len(L):],*failed,sep="\n  " if failed else " ")
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R49 landing (%s): pool process hygiene: one process per machine, memory grant with recycle, yield to the machine's own work, footprint in pool.json, supervisor kills the old child\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        for p in NEW:f.write("- new `%s` after %s\n"%(p[len(L):],sha(R+"/"+p)))
        for p in EDITED:f.write("- edit `%s` after %s (before not captured, edited in place)\n"%(p[len(L):],sha(R+"/"+p)))
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
