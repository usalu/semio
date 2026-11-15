#!/usr/bin/env python3
"""R5b read-only: for the files each landing of T/landings.md touched, how many projects of a fresh sample hold a command closure that contains the file (so a membership or content change of it moves their task key, and today also their node), before and after the scheduling domains."""
import json,os,re,sys,collections
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import boundary as B,ports as P,domains as D
R=B.R;LIB=B.LIB
T=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/"
want=sys.argv[1:] or ["P14","R2","R1b","R4","R6","R7"]
lines=open(T+"landings.md",encoding="utf8").read().split("\n")
idx=[i for i,l in enumerate(lines) if l.startswith("## ")]
landed={};kind={}
for a,b in zip(idx,idx[1:]+[len(lines)]):
    m=re.match(r"## Package (\w+) landing",lines[a])
    if m and m.group(1) in want:
        for l in lines[a+1:b]:
            m2=re.match(r"- (new|copy|merge|same|edit\w*) `([^`]+)`",l)
            if m2 and m2.group(1)!="same":landed.setdefault(m.group(1),set()).add(m2.group(2));kind.setdefault(m.group(1),{})[m2.group(2)]=m2.group(1)
held,n=B.fresh_files()
everything=[]
for base in (LIB+"🏗️build",LIB+"⚡️caching","🧰️framework/🔨️modules/🏃️process"):
    for d,ds,fs in os.walk(R+base):
        ds[:]=[x for x in ds if x!="node_modules"]
        for f in fs:everything.append(os.path.relpath(os.path.join(d,f),R))
def resolve(text):
    hits=[f for f in everything if f.endswith("/"+text) or f==text]
    return hits
domains=[p for did,pr,_,_ in D.DOMAINS if did!="cache-collection" for p in pr]
old=json.load(open(R+LIB+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json",encoding="utf8"))
oldp=[p for d in old["domains"] if d["id"] not in [x[0] for x in D.DOMAINS] for p in d["prefixes"]]
allp=oldp+domains
print("sample of fresh projects:",n)
for name in want:
    fs=set()
    for t in landed.get(name,()):
        r=resolve(t)
        fs.update(r)
    inF=[f for f in fs if f in held]
    holders_before=set().union(*[held[f] for f in inF if not any(f.startswith(p) for p in oldp)]) if inF else set()
    holders_after=set().union(*[held[f] for f in inF if not any(f.startswith(p) for p in allp)]) if inF else set()
    newer=set()
    for t,k in kind.get(name,{}).items():
        if k=="new":newer.update(resolve(t))
    newF=[f for f in newer if f in held]
    newHold=set().union(*[held[f] for f in newF]) if newF else set()
    newHoldAfter=set().union(*[held[f] for f in newF if not any(f.startswith(p) for p in allp)]) if newF else set()
    tests=[f for f in fs if "/🧪️tests/" in f]
    print(f"{name}: files {len(fs)} (tests {len(tests)}), in a fresh closure {len(inF)}, holders before the domains {len(holders_before)}/{n}, after the domains {len(holders_after)}/{n}; NEW files in a closure {len(newF)}: node-changing holders today {len(newHold)}, with the domains {len(newHoldAfter)}, with tool records 0")
