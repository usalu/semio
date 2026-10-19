#!/usr/bin/env python3
import sys,glob,urllib.parse
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L=C+"/🗑️generated/law3-out/"
w=int(sys.argv[2]) if len(sys.argv)>2 else 500
n=int(sys.argv[3]) if len(sys.argv)>3 else 120
t=urllib.parse.unquote(open((glob.glob(L+sys.argv[1]+"-*.log")+glob.glob(L+sys.argv[1]+"*.log"))[0],encoding="utf-8",errors="replace").read())
out=[]
for l in t.split("\n"):
    if l.startswith("Internal error: directory"): continue
    out.append(l[:w])
print("\n".join(out[:n]))
