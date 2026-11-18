#!/usr/bin/env python3
"""R86: runs the staged slice-report law in scratch. usage: devC.py SCRATCH new|old"""
import os,re,shutil,subprocess,sys
SC,mode=sys.argv[1:3]
R="/Users/ueli/Documents/semio";SR=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
d=SC+"/slice-report";shutil.rmtree(d,ignore_errors=True);os.makedirs(d+"/🧬️schema");os.makedirs(d+"/🧪️tests")
src=lambda n:(SC+"/stageC/"+n+".new") if mode=="new" else (SC+"/stageC/"+n+".base")
shutil.copy(src("1"),d+"/🟨️.mjs");shutil.copy(src("3"),d+"/🧬️schema/🔣️.json")
t=open(SC+"/stageC/4.new",encoding="utf8").read()
def fix(m):return m.group(1)+'"'+os.path.normpath(os.path.join(SR+"/🧪️tests",m.group(2)))+'"' if "🧬️schema/🔣️.json" not in m.group(2) and m.group(2)!="../🟨️.mjs" else m.group(0)
t=re.sub(r'(from)"(\.{1,2}/[^"]*)"',fix,t)
open(d+"/🧪️tests/🟦️.ts","w",encoding="utf8").write(t)
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
r=subprocess.run([BUN,"test",d+"/🧪️tests/🟦️.ts","--timeout","120000"],cwd=d,env=env);sys.exit(r.returncode)
