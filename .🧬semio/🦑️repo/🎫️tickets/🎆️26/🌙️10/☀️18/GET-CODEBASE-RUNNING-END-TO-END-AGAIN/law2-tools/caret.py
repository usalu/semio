#!/usr/bin/env python3
import re,sys
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
t=open(C+"/🗑️generated/law2-out/last.log",encoding="utf-8").read().splitlines()
for i,l in enumerate(t):
    if re.match(r"^\s+\^\s*$",l) and i>0:
        src=t[i-1]
        m=re.match(r"^(\d+) \| ",src)
        if not m: continue
        prefix=len(m.group(0)); col=l.index("^")
        # bun shows a window of the source; caret col relative to displayed line
        print("L"+m.group(1),"...",src[max(prefix,col-110):col+140].replace("\n"," "))
