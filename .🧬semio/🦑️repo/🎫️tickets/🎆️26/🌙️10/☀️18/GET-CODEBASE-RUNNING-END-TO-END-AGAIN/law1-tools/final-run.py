#!/usr/bin/env python3
import os,re,subprocess,sys,json
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
rows=[r for r in open(C+"/law1-tools/list-fw-rows.txt",encoding="utf-8").read().split("\n") if r]
out=C+"/🗑️generated/law1-out/final.jsonl"
open(out,"w").close()
env=dict(os.environ,LAW1_MAX_MS="3600000")
for r in rows:
    p=subprocess.run(["python3",C+"/law1-tools/run1.py",r],cwd=ROOT,env=env,capture_output=True,text=True)
    m=re.search(r"pass=(\S+) fail=(\S+)",p.stdout)
    rec={"file":r,"pass":m.group(1) if m else "?","fail":m.group(2) if m else "?"}
    open(out,"a",encoding="utf-8").write(json.dumps(rec,ensure_ascii=False)+"\n")
print("done")
