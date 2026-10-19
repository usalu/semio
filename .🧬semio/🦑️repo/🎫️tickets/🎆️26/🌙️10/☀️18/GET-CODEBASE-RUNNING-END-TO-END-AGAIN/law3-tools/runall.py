#!/usr/bin/env python3
import os,re,subprocess,sys
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
rows=[r for r in open(sys.argv[1],encoding="utf-8").read().split("\n") if r]
env=dict(os.environ,LAW3_MAX_MS="3600000")
for i,r in enumerate(rows):
    p=subprocess.run(["python3",C+"/law3-tools/run1.py",r],cwd=ROOT,env=env,capture_output=True,text=True)
    print(i+55 if len(sys.argv)<3 else i, p.stdout.strip().replace("\n","\n    ")[:3000])
