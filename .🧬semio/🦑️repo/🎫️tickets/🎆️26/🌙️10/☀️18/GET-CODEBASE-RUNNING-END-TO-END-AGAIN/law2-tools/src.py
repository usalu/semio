#!/usr/bin/env python3
# usage: src.py <idx> <test-prefix> [max]  -> only the source-reading tail of a test body
import sys,re
R="/Users/ueli/Documents/semio/"
C=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
f=open(C+"/law2-tools/list-mine.txt",encoding="utf-8").read().split("\n")[int(sys.argv[1])]
s=open(R+f,encoding="utf-8").read()
i=s.find(sys.argv[2])
if i<0: print("NOT FOUND"); sys.exit()
j=s.find("\ntest(",i+10); j=len(s) if j<0 else j
b=s[i:j]
k=min([x for x in (b.find("readModuleTreeSource"),b.find("readStoreSourceText"),b.find("readFile"),b.find("Bun.file"),b.find("readFileSync"),b.find("sourceTokens")) if x>=0] or [0])
print(b[:90].replace("\n"," ")); print("...."); print(b[k:k+int(sys.argv[3]) if len(sys.argv)>3 else k+3200])
