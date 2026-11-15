#!/usr/bin/env python3
"""q6: replica: a follower that rejoins from a snapshot deletes the live entries older than the tombstone horizon that the snapshot lacks. usage: q6-tombstone.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
p=root+"/🔁️replica/🟦️.ts"
s=open(p,encoding="utf8").read()
s=sub(s,'import {appendEventsV1,repairBlobV1,blobPathV1,','import {appendEventsV1,expiredHorizonV1,repairBlobV1,blobPathV1,')
s=sub(s,''' for(const [other,cursor] of Object.entries(snapshot.cursors))if(other!==originId&&cursorAheadV1(cursor,replica.cursors[other]))replica.cursors[other]=cursor;
 replica.dirty=true;
 replica.say({event:"progress",stage:origin===undefined?"jump":"rejoin"''',''' for(const id of replica.manifest.ids()){
  if(staged.live(id)||replica.manifest.operationAt(id)==="delete"||!expiredHorizonV1(replica.manifest.hlcAt(id),snapshot.hlc))continue;
  replica.manifest.write(id,{hlc:snapshot.hlc,operation:"delete"});
  if(!replica.files.live(id)||!replica.files.sameHlcAs(replica.manifest,id))enqueueId(replica,id);
 }
 for(const [other,cursor] of Object.entries(snapshot.cursors))if(other!==originId&&cursorAheadV1(cursor,replica.cursors[other]))replica.cursors[other]=cursor;
 replica.dirty=true;
 replica.say({event:"progress",stage:origin===undefined?"jump":"rejoin"''')
open(p,"w",encoding="utf8").write(s)
print("ok")
