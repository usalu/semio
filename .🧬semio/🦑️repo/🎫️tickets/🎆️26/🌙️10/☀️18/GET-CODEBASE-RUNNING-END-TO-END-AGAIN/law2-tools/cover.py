#!/usr/bin/env python3
# usage: cover.py <idx> <test-name-prefix> [scope-glob]  -> files covering the positive needles of a test
import re,sys,json,subprocess,collections
R="/Users/ueli/Documents/semio/"
C=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
idx=int(sys.argv[1]); name=sys.argv[2]; scope=sys.argv[3] if len(sys.argv)>3 else "🧰️framework/*.rs"
f=open(C+"/law2-tools/list-mine.txt",encoding="utf-8").read().split("\n")[idx]
s=open(R+f,encoding="utf-8").read()
i=s.find(name); j=s.find("\ntest(",i+10); j=len(s) if j<0 else j
p=s[i:j]
reads=re.findall(r'(?:readModuleTreeSource|readSourceFamilySource|readFileSync|read|readFile|Bun\.file|sliceModuleTreeSource|readStoreSourceText)\(\s*(?:new URL\()?\s*("[^"]*\.rs"|"[^"]*/")',p)
print("reads:",[r[-90:] for r in dict.fromkeys(reads)])
pos=[];neg=[]
for q in re.finditer(r'(not\.)?toContain\(\s*("(?:[^"\\]|\\.)*"|`(?:[^`\\]|\\.)*`)\s*\)|\.includes\(\s*("(?:[^"\\]|\\.)*")\s*\)\)\.toBe\((true|false)\)',p):
    raw=q.group(2) or q.group(3)
    try: v=json.loads(raw) if raw[0]=='"' else raw[1:-1]
    except Exception: v=raw[1:-1]
    isneg=bool(q.group(1)) or q.group(4)=="false"
    (neg if isneg else pos).append(v)
cov=collections.defaultdict(list)
for v in pos:
    r=subprocess.run(["git","grep","-l","-F","-e",v,"--",scope],cwd=R,capture_output=True,text=True)
    fs=[x for x in r.stdout.split("\n") if x and "🗑️generated" not in x and "/.🧬semio/" not in x]
    if not fs: print("  MISSING-EVERYWHERE:",v[:110])
    for x in fs: cov[x].append(v)
best=sorted(cov.items(),key=lambda kv:-len(kv[1]))[:4]
for fp,vs in best: print("  cover %d/%d"%(len(vs),len(pos)),fp.replace("🧰️framework/🛍️products/💻️os/🔨️modules/","OSM/")[-170:])
if best:
    print("  not in best:",[v[:70] for v in pos if v not in best[0][1]])
for v in neg:
    r=subprocess.run(["git","grep","-l","-F","-e",v,"--",scope],cwd=R,capture_output=True,text=True)
    n=len([x for x in r.stdout.split("\n") if x and "🗑️generated" not in x and "/.🧬semio/" not in x])
    if n: print("  neg present in",n,"files:",v[:80])
