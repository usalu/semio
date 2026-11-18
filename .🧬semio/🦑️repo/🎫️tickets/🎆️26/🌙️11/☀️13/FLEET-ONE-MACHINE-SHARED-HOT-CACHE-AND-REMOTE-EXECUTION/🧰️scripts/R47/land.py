#!/usr/bin/env python3
"""R47 landing record: the edits are made in the live files (anchored, re-read before each edit); this script takes landing.lock, runs the targeted law tests (only files named 🟦️.ts, never the fixture programs 🧬️*.ts that R44's walker mistook for tests), checks the long-running pids survive and appends the entry to landings.md.
usage: land.py [--tests-only]"""
import os,subprocess,hashlib,datetime,sys
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B="🏗️build/"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
FILES=[B+"🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🪢️hard-parts/🟦️.ts",B+"👁️observation/🏚️relocation/🟨️.mjs",B+"👁️observation/🏚️relocation/🟨️.d.mts",B+"👁️observation/🏚️relocation/🧪️tests/🟦️.ts",B+"👁️observation/🟨️.mjs",B+"🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs",B+"🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs",B+"🕸️graph/🕰️generations/🟨️.mjs",B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs",B+"🧷️format-stamps/🧬️schema/🔣️.json",B+"🧷️format-stamps/🟨️.d.mts",B+"🧷️format-stamps/🔄️reexec/🟨️.mjs",B+"🧷️format-stamps/🧪️tests/🟦️.ts"]
TESTS=[B+"🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🪢️hard-parts",B+"🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🧾️vouch",B+"👁️observation/🏚️relocation/🧪️tests",B+"🧷️format-stamps/🧪️tests",B+"🏡️resident/🕰️graph-hooks/🧪️tests",B+"🕸️graph/🕰️generations/🧪️tests/🔎️doubt",B+"🕸️graph/🕰️generations/🧪️tests/🛡️stamp-soundness",B+"🕸️graph/🕰️generations/✂️cutoff/🧪️tests",B+"🕸️graph/🧮️derived-nodes/🧩️closure-kinds/📒️manifest/🧪️tests"]
def sha(p):return hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
def pids():
    out=subprocess.run(["ps","-o","pid=","-p","31945,31946,19234"],capture_output=True,text=True).stdout.split()
    return sorted(out)
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R47-%d\n"%os.getpid())
try:
    before=pids();env={k:v for k,v in os.environ.items() if not k.startswith("SEMIO_")};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    lines=[]
    for t in TESTS:
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=L+t,env=env,capture_output=True,text=True)
        red=[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")]
        lines.append("- test `LIB/%s` exit %d, %d red%s"%(t,r.returncode,len(red),(": "+red[0][:140]) if red else ""));print(lines[-1])
    after=pids()
    if "--tests-only" in sys.argv:sys.exit(0)
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
    entry="\n## Package R47 landing 3 (%s): audit fixes A1-A4 (rebase in sync checks, per-process live roots, order-free chains, JSON-escaped roots) and bounded restart following (B1, B2)\n"%stamp
    entry+="".join("- edit `LIB/%s` after %s\n"%(f,sha(L+f)) for f in FILES)+"\n".join(lines)+"\n- pids 31945,31946,19234 alive before %s after %s\n"%(before,after)
    open(T+"/landings.md","a").write(entry)
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
