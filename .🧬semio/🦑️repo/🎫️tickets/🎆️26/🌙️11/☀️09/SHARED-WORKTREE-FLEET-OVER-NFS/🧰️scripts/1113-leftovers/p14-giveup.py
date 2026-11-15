import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🔁️replica/🟦️.ts",[
 ('''  drift.attempts=fields.length===0?0:drift.attempts+1;
  drift.wait=fields.length===0?baseDrift:Math.min(baseDrift*2**drift.attempts,3_600_000);
  drift.at=now+drift.wait;
  return fields;''','''  if(fields.length>0&&drift.attempts>=3){drift.at=now+3_600_000;return [];}
  drift.attempts=fields.length===0?0:drift.attempts+1;
  drift.wait=fields.length===0?baseDrift:Math.min(baseDrift*2**drift.attempts,3_600_000);
  drift.at=now+drift.wait;
  return fields;'''),
])
edit(w+"/README.md",[
 ('Repeated findings back off from two minutes to one hour. Differences in code, git state or view are never repaired automatically.','Repeated findings back off from two minutes to one hour, and after three repairs that did not end the difference (a machine that ignores a path the others share differs by design) the daemon stops repairing until the difference has gone, so a by-design divergence never costs a store fold every hour. Differences in code, git state or view are never repaired automatically.'),
])
