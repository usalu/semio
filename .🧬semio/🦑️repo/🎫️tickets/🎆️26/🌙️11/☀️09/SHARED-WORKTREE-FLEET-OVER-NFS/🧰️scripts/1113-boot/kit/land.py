#!/usr/bin/env python3
"""Lands the tested candidate tree onto the live module: every file that differs is replaced atomically, in dependency order, but only if the live file is still byte-identical to the base this work started from (nobody landed in between). usage: land.py <candidate-module> <base-module> <live-module> [--revert]"""
import os,sys,hashlib,json
cand,base,live=sys.argv[1:4]
revert='--revert' in sys.argv
def sha(p):
    return hashlib.sha256(open(p,'rb').read()).hexdigest() if os.path.isfile(p) else None
def files(root):
    out=set()
    for d,ds,fs in os.walk(root):
        ds[:]=[x for x in ds if x!='node_modules']
        for f in fs: out.add(os.path.relpath(os.path.join(d,f),root))
    return out
first=['🛡️resilience/🧬️schema/🔣️.json','🩺️attest/🧬️schema/🔣️.json','🔁️replica/🧬️schema/🔣️.json','🔁️replica/📖️texts/🔣️.json','🛡️resilience/🧪️tests/🧫️fixtures/🔣️.json']
ts=['🛡️resilience/🟦️.ts','📒️journal/🟦️.ts','🩺️attest/🟦️.ts','🔁️replica/🟦️.ts','🔁️replica/⌨️command/🟦️.ts','🔁️replica/📜️script.ts']
src,dst=(base,live) if revert else (cand,live)
want=files(src)|files(live) if revert else files(src)
changed=[r for r in sorted(want) if sha(os.path.join(src,r))!=sha(os.path.join(live,r)) and (os.path.exists(os.path.join(src,r)) )]
if revert:
    changed=[r for r in sorted(files(base)|files(cand)) if sha(os.path.join(base,r))!=sha(os.path.join(live,r))]
order=[r for r in first+ts if r in changed]+[r for r in changed if r not in first+ts]
for r in order:
    live_sha=sha(os.path.join(live,r)); known=sha(os.path.join(base,r))
    if not revert and live_sha!=known:
        print("ABORT: live file changed since the base:",r); sys.exit(2)
    if revert and live_sha not in (sha(os.path.join(cand,r)),known):
        print("ABORT: live file is neither the landed nor the base version:",r); sys.exit(2)
rows=[]
for r in order:
    t=os.path.join(live,r)
    before=sha(t)
    s=os.path.join(src,r)
    if os.path.exists(s):
        os.makedirs(os.path.dirname(t),exist_ok=True)
        tmp=t+'.landing.tmp'
        open(tmp,'wb').write(open(s,'rb').read())
        os.replace(tmp,t)
    else:
        os.remove(t)
    rows.append((r,before,sha(t)))
for r,b,a in rows: print("| ./%s | %s | %s |"%(r,(b or '-')[:16],(a or '-')[:16]))
print(len(rows),"files")
