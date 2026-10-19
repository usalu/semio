#!/usr/bin/env python3
# usage: body.py <idx> <test-name-substring> [maxchars]
import sys
R="/Users/ueli/Documents/semio/"
C=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
f=open(C+"/law2-tools/list-mine.txt",encoding="utf-8").read().split("\n")[int(sys.argv[1])]
s=open(R+f,encoding="utf-8").read()
i=s.find(sys.argv[2])
if i<0: print("NOT FOUND"); sys.exit()
j=s.find("\ntest(",i+10)
j2=s.find("\nfor(",i+10)
ends=[x for x in (j,j2) if x>0]
j=min(ends) if ends else len(s)
print(s[i-6:j][:int(sys.argv[3]) if len(sys.argv)>3 else 3500])
