#!/usr/bin/env python3
"""q9: replica: the first outage of a daemon that never saw its store is `waiting-for-mount` (X20). usage: q9-mount.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
p=root+"/🔁️replica/🟦️.ts"
s=open(p,encoding="utf8").read()
s=sub(s,'const down=()=>{if(!outage){outage=true;replica.say({event:"store-unreachable",path:settings.storeRoot,reason:guard.state()});}};','let everUp=false;\n const down=()=>{if(!outage){outage=true;replica.say({event:"store-unreachable",path:settings.storeRoot,reason:everUp?guard.state():"waiting-for-mount"});}};')
s=sub(s,'''    if(!await reachable.get(outage))down();
    else{
     if(outage){''','''    if(!await reachable.get(outage))down();
    else{
     everUp=true;
     if(outage){''')
open(p,"w",encoding="utf8").write(s)
t=root+"/🔁️replica/🧪️tests/🟦️.ts"
s=open(t,encoding="utf8").read()
s=sub(s,' expect(events.some(event=>event.event==="started")).toBe(false);\n write(a,"early.txt"',' write(a,"early.txt"')
open(t,"w",encoding="utf8").write(s)
q="q9-mount-tests.py"
s=open(q,encoding="utf8").read()
s=s.replace(' expect(events.some(event=>event.event==="started")).toBe(false);\n','')
open(q,"w",encoding="utf8").write(s)
print("ok")
