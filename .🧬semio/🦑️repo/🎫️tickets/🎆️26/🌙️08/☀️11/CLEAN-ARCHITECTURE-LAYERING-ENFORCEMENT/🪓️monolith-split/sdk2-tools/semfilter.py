# -*- coding: utf-8 -*-
import re,json,sys,collections
R="/Users/ueli/Documents/semio/"
G=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
touched=set(l.strip() for l in open(G+"/sdk2-tsc-list-after.txt") if l.strip())
for sn in ("sdk2-snap-layout.json","sdk2-snap-tt.json","sdk2-snap-reads.json"): touched|=set(json.load(open(G+"/"+sn)).keys())
rows=[l.rstrip("\n") for l in open(G+"/sdk2-tsc-sem-after.txt",encoding="utf8") if l.strip()]
mine=[l for l in rows if l.split("(")[0] in touched]
print("total",len(rows),"in touched files",len(mine), "touched files",len(touched))
by=collections.Counter(l.split("(")[0] for l in mine)
for f,n in by.most_common(): print(n,f.replace("✏️s/🔌️plugins/","P/")[-120:])
open(G+"/sdk2-tsc-sem-mine.txt","w").write("\n".join(mine)+"\n")
