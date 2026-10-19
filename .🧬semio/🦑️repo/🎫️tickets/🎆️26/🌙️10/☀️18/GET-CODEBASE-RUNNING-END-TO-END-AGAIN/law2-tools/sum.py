import json,re,sys
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
idx=[int(x) for x in sys.argv[1:]]
recs=[json.loads(l) for l in open(C+"/🗑️generated/law2-out/before.jsonl",encoding="utf-8")]
for i in idx:
    r=recs[i]
    print("=====",i,r["file"][-110:])
    t=open(r["log"],encoding="utf-8",errors="replace").read().splitlines()
    n=0
    for j,l in enumerate(t):
        if l.startswith("error:") or l.startswith("(fail)") or l.startswith("Expected") or l.startswith("Received") or "Error:" in l[:40]:
            print("  ",l[:260]); n+=1
            if n>24: break
