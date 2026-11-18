#!/usr/bin/env python3
"""R86: runs the staged module against the law in scratch. usage: dev.py SCRATCH new|old   (new: staged closure-kinds; old: live one)"""
import os,subprocess,sys,re
SC,mode=sys.argv[1],sys.argv[2]
R="/Users/ueli/Documents/semio";L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
here=os.path.dirname(os.path.abspath(__file__))
subprocess.run([sys.executable,here+"/absolutize.py",SC+"/stage/1.new",L+"🧩️closure-kinds",SC+"/ck.mjs"],check=True)
ck=open(SC+"/ck.mjs",encoding="utf8").read().replace("CUTOFF_POLICY_V1.partRowBatchRows","4096").replace("CUTOFF_POLICY_V1.partProgressMilliseconds","5000")
open(SC+"/ck.mjs","w",encoding="utf8").write(ck)
t=open(here+"/part-ids.test.ts",encoding="utf8").read()
t=t.replace('"../../🟨️.mjs"','"'+(SC+"/ck.mjs" if mode=="new" else L+"🧩️closure-kinds/🟨️.mjs")+'"').replace('"../../../💾️store/🟨️.mjs"','"'+L+'💾️store/🟨️.mjs"')
open(SC+"/part-ids.test.ts","w",encoding="utf8").write(t)
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["SEMIO_GRAPH_WALK_FARM"]="off"
r=subprocess.run([BUN,"test",SC+"/part-ids.test.ts","--timeout","180000"],cwd=SC,env=env)
sys.exit(r.returncode)
