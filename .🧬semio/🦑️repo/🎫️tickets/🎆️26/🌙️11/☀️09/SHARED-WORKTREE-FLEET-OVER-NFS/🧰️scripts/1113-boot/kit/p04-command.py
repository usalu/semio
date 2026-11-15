import sys;sys.path.insert(0,__import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from lib import *
C='🔁️replica/⌨️command/🟦️.ts'
SC='🔁️replica/🧬️schema/🔣️.json'
T='🔁️replica/📖️texts/🔣️.json'
sub(C,'import {compactV1,dryOpenStateV1,homedOriginsV1,','import {compactV1,dryOpenReplicaV1,homedOriginsV1,')
sub(C,'storeReachableV1,verifyV1,type ReplicaEventV1,','smokeBootV1,storeReachableV1,verifyV1,type ReplicaEventV1,')
sub(C,'import {selfCheckV1} from "../../🛡️resilience/🟦️.ts";','import {installBootStageV1,readFallbackV1,selfCheckV1} from "../../🛡️resilience/🟦️.ts";')
sub(C,'import {readFile,stat} from "node:fs/promises";','import {readFile,stat} from "node:fs/promises";\nimport {join} from "node:path";')
sub(C,'''  const root=resolve(import.meta.dirname,"..",".."),state=await dryOpenStateV1(context.root),checked0=await selfCheckV1(root,signal),checked=state===null?checked0:{...checked0,ok:false,error:`local state cannot be read: ${state}`},report=''','''  const root=resolve(import.meta.dirname,"..",".."),checked0=await selfCheckV1(root,signal),state=checked0.ok?await dryOpenReplicaV1(context.root):null,smoke=checked0.ok&&state===null?await smokeBootV1(signal):null,checked=state!==null?{...checked0,ok:false,error:`local state cannot be opened: ${state}`}:smoke!==null?{...checked0,ok:false,error:`the daemon does not boot: ${smoke}`}:checked0,report=''')
sub(C,'''  if(error instanceof ReplicaBusyV1){say("error.busy",{pid:error.pid});return {exitCode:1};}
  if(error instanceof OriginTakenV1){say("error.origin.taken",{origin:error.originId});return {exitCode:1};}''','''  if(error instanceof ReplicaBusyV1){say("error.busy",{pid:error.pid});return {exitCode:parsed.operation==="run"?75:1};}
  if(error instanceof OriginTakenV1){say("error.origin.taken",{origin:error.originId});return {exitCode:parsed.operation==="run"?75:1};}''')
sub(C,'  await runV1({...options,watchRoot:context.environment.SEMIO_FLEET_CODE_WATCH},signal,event=>{','  const fallback=readFallbackV1(context.environment);\n  await runV1({...options,watchRoot:context.environment.SEMIO_FLEET_CODE_WATCH,...fallback===undefined?{}:{fallback}},signal,event=>{')
sub(C,'''  say("seed.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  const result=await seedV1(options,signal,sink);''','''  say("seed.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  const result=await seedV1(options,signal,sink);
  await installBootStageV1(resolved.worktree,join(import.meta.dirname,"..","📜️script.ts")).catch(()=>{});''')
sub(C,'''   const result=await joinV1(options,signal,sink);
   say("join.done",''','''   const result=await joinV1(options,signal,sink);
   await installBootStageV1(resolved.worktree,join(import.meta.dirname,"..","📜️script.ts")).catch(()=>{});
   say("join.done",''')
sub(C,'''
 if(report.health!==null){''','''
 if(report.daemon.busySeconds!==undefined)say("status.daemon.busy",{seconds:report.daemon.busySeconds});
 if(report.code!==undefined&&report.code.source!=="worktree")say("status.code.fallback",{source:`@status.code.${report.code.source}`,phase:`@status.code.phase.${report.code.phase}`});
 if(report.health!==null){''')
sub(SC,'''            "pid": { "oneOf": [{ "type": "integer", "minimum": 1 }, { "type": "null" }] }
          }
        },''','''            "pid": { "oneOf": [{ "type": "integer", "minimum": 1 }, { "type": "null" }] },
            "busySeconds": { "type": "integer", "minimum": 5, "description": "Present while the event loop of the running daemon has not answered for at least five seconds; the watchdog thread of the daemon proves that it is alive." }
          }
        },
        "code": {
          "description": "Present while a daemon runs: where its code comes from (the worktree, a sealed snapshot of this machine or a snapshot of the store) and its phase.",
          "type": "object",
          "additionalProperties": false,
          "required": ["source", "phase"],
          "properties": {
            "source": { "enum": ["worktree", "snapshot", "store"] },
            "phase": { "enum": ["boot", "open", "run"] }
          }
        },''')
sub(T,'''    "status.daemon.stopped": {''','''    "status.daemon.busy": {
      "en": "Daemon: busy, the event loop has not answered for {seconds} s",
      "de": "Daemon: beschäftigt, die Ereignisschleife antwortet seit {seconds} s nicht"
    },
    "status.code.fallback": {
      "en": "Code: fallback, running {source} ({phase}) until the code of the worktree starts again",
      "de": "Code: Ersatz, es läuft {source} ({phase}), bis der Code des Arbeitsverzeichnisses wieder startet"
    },
    "status.code.snapshot": {
      "en": "the sealed snapshot of this machine",
      "de": "der versiegelte Snapshot dieser Maschine"
    },
    "status.code.store": {
      "en": "the snapshot of the store",
      "de": "der Snapshot des Stores"
    },
    "status.code.worktree": {
      "en": "the worktree",
      "de": "das Arbeitsverzeichnis"
    },
    "status.code.phase.boot": {
      "en": "loading",
      "de": "wird geladen"
    },
    "status.code.phase.open": {
      "en": "opening its state",
      "de": "öffnet seinen Zustand"
    },
    "status.code.phase.run": {
      "en": "running",
      "de": "läuft"
    },
    "status.daemon.stopped": {''')
