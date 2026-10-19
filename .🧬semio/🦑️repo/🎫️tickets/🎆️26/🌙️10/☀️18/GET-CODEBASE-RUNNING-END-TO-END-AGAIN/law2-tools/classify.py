#!/usr/bin/env python3
# usage: classify.py <idx> <family-dir(abs or repo-relative)> [name-prefix ...]
# For every test (or named tests) report which toContain/includes needles are absent from the concatenated non-test .rs family.
import re,sys,os,json
R="/Users/ueli/Documents/semio/"
C=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
idx=int(sys.argv[1]); fam=sys.argv[2]; names=sys.argv[3:]
f=open(C+"/law2-tools/list-mine.txt",encoding="utf-8").read().split("\n")[idx]
s=open(R+f,encoding="utf-8").read()
famdir=fam if fam.startswith("/") else R+fam
text=[]
for r,ds,fs in os.walk(famdir):
    ds[:]=[d for d in ds if d not in(("node_modules","target","dist","🗑️generated") if os.environ.get("INCLUDE_TESTS") else ("🧪️tests","🧪️testing","node_modules","target","dist","🗑️generated"))]
    for x in fs:
        if x.endswith(".rs"): text.append(open(os.path.join(r,x),encoding="utf-8",errors="replace").read())
T="\n".join(text)
Tn=re.sub(r"\s+","",T)
parts=re.split(r'(?=\ntest\()',s)
for p in parts:
    m=re.match(r'\ntest\("([^"]+)"',p)
    if not m: continue
    if names and not any(m.group(1).startswith(n) for n in names): continue
    ne=[]
    for q in re.finditer(r'(not\.)?(?:toContain|toMatch)\(\s*("(?:[^"\\]|\\.)*"|`(?:[^`\\]|\\.)*`|\'(?:[^\'\\]|\\.)*\')\s*\)|\.includes\(\s*("(?:[^"\\]|\\.)*")\s*\)',p):
        raw=q.group(2) or q.group(3)
        try: v=json.loads(raw) if raw[0]=='"' else raw[1:-1]
        except Exception: v=raw[1:-1]
        neg=bool(q.group(1))
        present=v in T or re.sub(r"\s+","",v) in Tn
        ne.append((neg,v,present))
    bad=[(n,v) for n,v,pr in ne if (pr if n else not pr)]
    print(("OK   " if not bad else "BAD  ")+m.group(1)[:80],len(ne),"needles", [("NOT " if n else "")+v[:60] for n,v in bad][:6])
