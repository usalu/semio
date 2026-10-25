#!/usr/bin/env python3
"""usage: status.py [--all]  Prints each unseen verdict row of verdicts3.tsv; rows with errors get their grouped error summary (sum.py)."""
import os,subprocess,sys
os.chdir("/Users/ueli/Documents/semio")
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
V=C+"/🗑️generated/plugc-d/verdicts3.tsv"; S=C+"/🗑️generated/plugc-d/status.seen"
seen=set(open(S).read().split("\n")) if os.path.exists(S) else set()
for l in open(V):
    k=l.split("\t")[0]
    if k in seen and "--all" not in sys.argv: continue
    f=l.rstrip("\n").split("\t")
    print(f[0],"|",f[1][:140],"|",*f[2:])
    if "errorLines=0" not in l and k!="chain3-done":
        print(subprocess.run(["python3",C+"/plugc-d-tools/sum.py",C+"/🗑️generated/plugc-d/g3/"+k+".raw","260"],capture_output=True,text=True).stdout)
    open(S,"a").write(k+"\n")
