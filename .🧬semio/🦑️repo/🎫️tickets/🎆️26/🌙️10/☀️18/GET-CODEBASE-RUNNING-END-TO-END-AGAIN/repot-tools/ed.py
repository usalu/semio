#!/usr/bin/env python3
# usage: ed.py <file> <<'X'
# OLD<<<
# ...
# NEW<<<
# ...
# X   (FE0F-insensitive match of OLD; NEW inserted verbatim; exactly one match required unless ALL=1)
import sys,re,os
f=sys.argv[1]
spec=sys.stdin.read()
parts=re.split(r"^(?:OLD|NEW)<<<\n",spec,flags=re.M)
old,new=parts[1].rstrip("\n"),parts[2].rstrip("\n")
def fe(x):
    out=[];i=0
    while i<len(x):
        ch=x[i];out.append(ch)
        nxt=x[i+1] if i+1<len(x) else ""
        prev=x[i-1] if i>0 else ""
        if ord(ch)>=0x1F000 and nxt not in("\ufe0f","\u200d") and prev!="\u200d": out.append("\ufe0f")
        i+=1
    return "".join(out)
new=fe(new)
s=open(f,encoding="utf-8").read()
def pat(o):
    out=""
    for ch in o.replace("️",""):
        out+=re.escape(ch)+("️?" if ord(ch)>0x2000 else "")
    return out
ms=list(re.finditer(pat(old),s))
if len(ms)==0: print("NO MATCH"); sys.exit(1)
if len(ms)>1 and not os.environ.get("ALL"): print("MULTI",len(ms)); sys.exit(1)
s2=re.sub(pat(old),lambda m:new,s) if os.environ.get("ALL") else s[:ms[0].start()]+new+s[ms[0].end():]
open(f,"w",encoding="utf-8").write(s2)
print("ok",len(ms))
