#!/usr/bin/env python3
"""usage: owners.py OUT.tsv  Maps every file defining a `fn sqlite_snapshot*` test to the crate target (lib or [[test]]) whose #[path] module tree mounts it.
Row: manifest TAB package TAB kind(lib|test) TAB targetName TAB testCount. Reuses plug-mz-c's modscan2 walker."""
import os,re,sys,subprocess
os.chdir("/Users/ueli/Documents/semio")
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
src=open(C+"/plug-mz-c-tools/modscan2.py").read().split("tot=0")[0]
sys.argv=[sys.argv[0],"/dev/null"]
exec(src)
out=sys.argv[0]
ROOTS=sys.stdin.read().split() if False else ["✏️s/🔌️plugins"]
tests=set()
cnt={}
for root in ROOTS:
    for f in subprocess.run(["rg","-l","fn sqlite_snapshot","--glob","*.rs",root],capture_output=True,text=True).stdout.split("\n"):
        if f:
            tests.add(os.path.normpath(f))
            cnt[os.path.normpath(f)]=len(re.findall(r'fn sqlite_snapshot',open(f,encoding="utf-8").read()))
rows=[]
for root in ROOTS:
    for cf in subprocess.run(["rg","--files","-g","Cargo.toml",root],capture_output=True,text=True).stdout.split("\n"):
        if not cf: continue
        try: t=open(cf,encoding="utf-8").read()
        except: continue
        if "[package]" not in t: continue
        nm=re.search(r'^name\s*=\s*"([^"]+)"',t,re.M).group(1)
        for kind,tp in targets(cf,t):
            if kind not in("lib","test"): continue
            seen=set(); walk(tp,seen)
            hit=[f for f in seen if f in tests]
            if hit:
                tn=""
                if kind=="test":
                    for blk in re.finditer(r'\[\[test\]\](.*?)(?=\n\[|\Z)',t,re.S):
                        p=re.search(r'path\s*=\s*"([^"]+)"',blk.group(1)); n=re.search(r'name\s*=\s*"([^"]+)"',blk.group(1))
                        if p and n and os.path.normpath(os.path.join(os.path.dirname(cf),p.group(1)))==os.path.normpath(tp): tn=n.group(1)
                rows.append((cf,nm,kind,tn,sum(cnt[f] for f in hit),len(seen),";".join(sorted(hit))))
open(sys.argv[0] if False else "/dev/stdout","w")
import json
print("\n".join("\t".join(map(str,r)) for r in rows))
