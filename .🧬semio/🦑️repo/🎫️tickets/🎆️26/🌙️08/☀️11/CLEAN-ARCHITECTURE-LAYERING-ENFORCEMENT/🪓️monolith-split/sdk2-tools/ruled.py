# -*- coding: utf-8 -*-
import re,collections,sys
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
def parse(p):
    s=collections.Counter()
    for l in open(p,encoding="utf8"):
        m=re.match(r'^\s+(error|warn) ([^:]+): (\S+) → (.*)$',l)
        if m: s[(m.group(2),m.group(3),m.group(4).strip())]+=1
    return s
a=parse(G+"/sdk2-before-err.txt"); b=parse(G+"/sdk2-after-err.txt")
want=sys.argv[1]
rem=[(k,n-b.get(k,0)) for k,n in a.items() if k[0].startswith(want) and b.get(k,0)<n]
add=[(k,n-a.get(k,0)) for k,n in b.items() if k[0].startswith(want) and a.get(k,0)<n]
print(want,"removed",sum(n for _,n in rem),"added",sum(n for _,n in add))
if len(sys.argv)>2:
    for k,n in add: print("  +",n,k[1][-110:],"->",k[2][-110:])
if len(sys.argv)>3:
    for k,n in rem: print("  -",n,k[1][-110:],"->",k[2][-110:])
