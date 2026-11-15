import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🔁️replica/🟦️.ts",[
 ('''  if(folded.manifest.get(path)!==undefined||indexRealmV1(path)&&manifest.operationAt(id)==="delete")continue;
  manifest.clearId(id);replica.changed.add(filesKey(id));replica.dirty=true;
  if(replica.files.live(id)&&!queue.hasId(id))enqueueId(replica,id);
  count++;''','''  if(folded.manifest.get(path)!==undefined)continue;
  manifest.clearId(id);queue.removeId(id);forget(replica,path);replica.changed.add(queueKey(id));replica.dirty=true;
  count++;'''),
 ('''     if(await repairAsked.get()){replica.say({event:"repaired",count:await repairV1(replica,signal)});repairAsked.expire();full=true;}''','''     const asked=await repairAsked.get(),apart=asked?[]:await drifted(signal);
     if(asked||apart.length>0){
      if(apart.length>0)replica.say({event:"drift",reason:apart.join(","),count:drift.attempts});
      replica.say({event:"repaired",count:await repairV1(replica,signal),...apart.length>0?{reason:"drift"}:{}});repairAsked.expire();full=true;
     }'''),
 (' replica.say({event:"started",originId:settings.originId,groupId:settings.groupId});',' options.opened?.(replica);\n replica.say({event:"started",originId:settings.originId,groupId:settings.groupId});'),
])
edit(w+"/🔁️replica/🧬️schema/🔣️.json",[('"superseded", "stale-republish", "error"]','"superseded", "stale-republish", "drift", "error"]')])
edit(w+"/🔁️replica/⌨️command/🟦️.ts",[('   if(event.event==="stale-republish")say("event.stale-republish",{path:event.path??""});','   if(event.event==="stale-republish")say("event.stale-republish",{path:event.path??""});\n   if(event.event==="drift")say("event.drift",{fields:event.reason??"-",attempt:event.count??0});')])
edit(w+"/🔁️replica/📖️texts/🔣️.json",[('''    "status.git.shared": {''','''    "event.drift": {
      "en": "This machine differs from the settled fleet in {fields} (check {attempt}); it repairs its state from the store.",
      "de": "Diese Maschine weicht vom abgeglichenen Fleet in {fields} ab (Prüfung {attempt}); sie repariert ihren Zustand aus dem Store."
    },
    "status.git.shared": {''')])
