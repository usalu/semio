#!/usr/bin/env python3
import re
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
t=open(C+"/🗑️generated/law2-out/last.log",encoding="utf-8").read().splitlines()
seen=set()
for l in t:
    m=re.search(r"at <anonymous> \((/[^)]+🟦️[^)]*\.ts):(\d+):(\d+)\)",l) or re.search(r"at <anonymous> \((/[^)]+\.ts):(\d+):(\d+)\)",l)
    if m and (m.group(1),m.group(2),m.group(3)) not in seen:
        seen.add((m.group(1),m.group(2),m.group(3)))
        L=open(m.group(1),encoding="utf-8").read().split("\n")
        line=L[int(m.group(2))-1]; col=int(m.group(3))
        print("L%s:%s"%(m.group(2),m.group(3)),line[max(0,col-150):col+120])
