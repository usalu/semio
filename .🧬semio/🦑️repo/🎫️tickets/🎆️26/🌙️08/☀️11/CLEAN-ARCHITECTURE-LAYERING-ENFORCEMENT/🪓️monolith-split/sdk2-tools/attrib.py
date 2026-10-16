# -*- coding: utf-8 -*-
import re,collections,json,sys
R="/Users/ueli/Documents/semio/"
G=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
def parse(p):
    s=collections.Counter()
    for l in open(p,encoding="utf8"):
        m=re.match(r'^\s+(error|warn) ([^:]+): (\S+) → (.*)$',l)
        if m: s[tuple(re.sub('[^\\x00-\\x7f]','',x) for x in (m.group(2),m.group(3),m.group(4).strip()))]+=1
    return s
a=parse(G+"/sdk2-before-err.txt"); b=parse(G+"/sdk2-after2-err.txt")
mine=set()
for sn in ("sdk2-snap-layout.json","sdk2-snap-tt.json","sdk2-snap-reads.json"):
    mine|=set(json.load(open(G+"/"+sn)).keys())
extra=open(G+"/sdk2-touched-extra.txt").read().split("\n") if len(sys.argv)>1 else []
mine={re.sub('[^\\x00-\\x7f]','',x) for x in mine|{x for x in extra if x}}
rules=["plugins-framework-sdk-only","no-cross-technology-s-to-framework","framework-no-products","not-to-unlisted","no-circular"]
for r in rules:
    rem=sum(n-b.get(k,0) for k,n in a.items() if k[0].startswith(r) and b.get(k,0)<n)
    addm=[(k,n-a.get(k,0)) for k,n in b.items() if k[0].startswith(r) and a.get(k,0)<n and k[1] in mine]
    addo=sum(n-a.get(k,0) for k,n in b.items() if k[0].startswith(r) and a.get(k,0)<n and k[1] not in mine)
    print(f"{r}: total before={sum(n for k,n in a.items() if k[0].startswith(r))} after={sum(n for k,n in b.items() if k[0].startswith(r))} removed={rem} added-in-my-files={sum(n for _,n in addm)} added-elsewhere={addo}")
    for k,n in addm: print("    +",n,k[1][-100:],"->",k[2][-100:])
