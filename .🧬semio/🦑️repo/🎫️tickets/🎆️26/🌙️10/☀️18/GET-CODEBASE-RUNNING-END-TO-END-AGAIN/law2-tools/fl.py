#!/usr/bin/env python3
# prints failing tests with first assertion lines from last.log
import re,sys
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
t=open(C+"/🗑️generated/law2-out/last.log",encoding="utf-8").read().splitlines()
W=int(sys.argv[1]) if len(sys.argv)>1 else 230
for i,l in enumerate(t):
    if l.startswith(("error:","TypeError","ENOENT","ReferenceError","SyntaxError","AssertionError")):
        nm=[x for x in t[i:i+60] if x.startswith("(fail)")]
        det=[x for x in t[i+1:i+8] if x.startswith(("Expected","Received","Expected to"))]
        print((nm[0][7:80] if nm else "?"),"|",l[:W],"|"," ".join(d[:W] for d in det[:2]))
